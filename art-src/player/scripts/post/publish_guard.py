"""Safe publication of the player's game sprites (V1.5).

The game loads ``assets/animations/player.ron`` and the atlases it names
(``assets/sprites/player/player_<group>.png``). Before V1.5 every post script
(game_sprites_v1_3.py, game_sprites_v1_4.py) wrote those files directly, so
running an older script replaced the current atlases with fewer clips. Now:

1. post scripts only write to a **staging** directory (``stage_dir``); a
   staging directory inside ``assets/`` is refused (``refuse_protected_output``);
2. ``publish`` is the only writer of ``assets/``. It validates the staged PNGs
   and manifest (structure, sizes, pivots, durations, non-empty frames),
   checks the generator version against the published one, requires every
   clip already published, compares hashes, backs up the current files,
   replaces them through temporary files with a journal, re-validates and
   rolls back on any failure;
3. ``assets/animations/player.publish.json`` (the **lock**) records what is
   published: pipeline version, generator, clips and the SHA-256 of every
   file. A data test (tests/data_tests.rs) checks the shipped files against it,
   so a write that bypasses the guard is caught too.

Replacing needs ``--confirm`` with the state id of what is published now
(``status`` prints it): proof that the caller saw what will be replaced, not a
generic "overwrite" switch, and no option skips a validation.

Usage (from the project root):
    python3 art-src/player/scripts/post/publish_guard.py status
    python3 art-src/player/scripts/post/publish_guard.py publish --stage DIR --confirm STATE_ID
    python3 art-src/player/scripts/post/publish_guard.py recover
    python3 art-src/player/scripts/post/publish_guard.py init-lock --version 1.4 --generator NAME
"""

from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import os
import re
import shutil
from dataclasses import dataclass, field
from pathlib import Path
from typing import Callable

from PIL import Image

from contract import COMBAT_FRAME, COMBAT_PIVOT, FRAME, PIVOT

PLAYER_DIR = Path(__file__).resolve().parents[2]
PROJECT = PLAYER_DIR.parents[1]
ASSETS = PROJECT / "assets"
BACKUPS = PLAYER_DIR / "renders" / "publish_backups"  # ignored by Git, local rollback copies
MANIFEST = "animations/player.ron"
LOCK = "animations/player.publish.json"
JOURNAL = "animations/.player.publish.journal.json"
STAGE_INFO = "publish.json"  # written by the post script next to the staged files
LOCK_SCHEMA = 1
MANIFEST_VERSION = 1
MAX_SIDE = 2048
# Clips the game has shown since V1.4: a manifest without any of them is never published.
REQUIRED_CLIPS = ("idle", "run", "dodge", "attack_basic", "hurt", "death", "jump_rise", "fall", "land",
                  "crouch", "guard_start", "slash_crescent", "slash_rising", "cast_forward", "cast_up")
# Contract pivots (texels at density 1) of the character groups; ``effects`` cells
# only need the pivot inside the cell.
GROUP_PIVOTS = {"locomotion": ((FRAME, FRAME), PIVOT), "combat": (COMBAT_FRAME, COMBAT_PIVOT)}
SKILL_FILES = ("assets/data/skills/sword_techniques.ron", "assets/data/spells/elemental.ron")
DURATION_TOLERANCE_S = 1e-4


class PublishError(Exception):
    """The publication was refused or failed; ``assets/`` is unchanged (or restored)."""


# --- versions, hashes, paths ------------------------------------------------------------


def parse_version(text: str) -> tuple[int, ...]:
    if not re.fullmatch(r"\d+(\.\d+)*", text or ""):
        raise PublishError(f"invalid version {text!r} (expected e.g. 1.5)")
    return tuple(int(p) for p in text.split("."))


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def state_id(files: dict[str, str]) -> str:
    """Short id of a set of published files (relative path -> sha256)."""
    joined = "\n".join(f"{k} {v}" for k, v in sorted(files.items()))
    return hashlib.sha256(joined.encode()).hexdigest()[:12]


def is_inside(path: Path, root: Path) -> bool:
    try:
        path.resolve().relative_to(root.resolve())
        return True
    except ValueError:
        return False


def refuse_protected_output(path: Path, assets_dir: Path = ASSETS) -> Path:
    """Post scripts call this on their output directory: only staging is allowed."""
    if is_inside(path, assets_dir):
        raise PublishError(f"{path} is inside {assets_dir}: post scripts write to a staging directory; "
                           "publish with publish_guard.py")
    return path


# --- manifest (the RON subset the post scripts generate) --------------------------------


@dataclass
class Atlas:
    texture: str
    cell: tuple[int, int]
    columns: int
    rows: int
    padding: int
    density: int

    def image_size(self) -> tuple[int, int]:
        return (self.columns * self.cell[0] + (self.columns - 1) * self.padding,
                self.rows * self.cell[1] + (self.rows - 1) * self.padding)


@dataclass
class Clip:
    atlas: str
    first: int
    count: int
    mode: str  # "Loop", "Once" or "FitToAction"
    fps: float | None
    pivot: tuple[float, float]
    fallback: str | None
    durations: list[float]
    speed_reference: float | None


@dataclass
class Manifest:
    version: int
    facing: str
    atlases: dict[str, Atlas] = field(default_factory=dict)
    clips: dict[str, Clip] = field(default_factory=dict)


_NUM = r"-?\d+(?:\.\d+)?"
_ATLAS = re.compile(rf'^"(?P<id>[^"]+)": \(texture: "(?P<texture>[^"]+)", cell: \((?P<cw>\d+), (?P<ch>\d+)\), '
                    rf"columns: (?P<cols>\d+), rows: (?P<rows>\d+), padding: (?P<pad>\d+), "
                    rf"density: (?P<den>\d+)\),$")
_CLIP = re.compile(rf'^"(?P<id>[^"]+)": \(atlas: "(?P<atlas>[^"]+)", first: (?P<first>\d+), count: (?P<count>\d+), '
                   rf"mode: (?P<mode>Loop\(fps: (?P<lfps>{_NUM})\)|Once\(fps: (?P<ofps>{_NUM})\)|FitToAction), "
                   rf"pivot: \((?P<px>{_NUM}), (?P<py>{_NUM})\), "
                   rf'fallback: (?P<fallback>None|Some\("[^"]+"\)), durations: \[(?P<durations>[^\]]*)\], '
                   rf"speed_reference: (?P<speed>None|Some\({_NUM}\))\),$")


def parse_manifest(text: str) -> Manifest:
    """Parses the generated RON. Any line outside the known format is an error
    (fail closed): the guard never guesses what a hand-edited file means."""
    lines = [ln.strip() for ln in text.splitlines()]
    lines = [ln for ln in lines if ln and not ln.startswith("//")]
    expected_open = ["(", None, None, "atlases: {"]
    if len(lines) < 6 or lines[0] != expected_open[0] or lines[3] != expected_open[3]:
        raise PublishError("manifest: unexpected header")
    v = re.fullmatch(r"version: (\d+),", lines[1])
    f = re.fullmatch(r"facing: (\w+),", lines[2])
    if not v or not f:
        raise PublishError("manifest: missing version or facing")
    manifest = Manifest(int(v.group(1)), f.group(1))
    i = 4
    while i < len(lines) and lines[i] != "},":
        m = _ATLAS.match(lines[i])
        if not m:
            raise PublishError(f"manifest: unreadable atlas line {lines[i][:80]!r}")
        manifest.atlases[m["id"]] = Atlas(m["texture"], (int(m["cw"]), int(m["ch"])), int(m["cols"]),
                                          int(m["rows"]), int(m["pad"]), int(m["den"]))
        i += 1
    if i + 1 >= len(lines) or lines[i + 1] != "clips: {":
        raise PublishError("manifest: missing clips")
    i += 2
    while i < len(lines) and lines[i] != "},":
        m = _CLIP.match(lines[i])
        if not m:
            raise PublishError(f"manifest: unreadable clip line {lines[i][:80]!r}")
        mode = m["mode"].split("(")[0]
        fps = m["lfps"] or m["ofps"]
        fallback = None if m["fallback"] == "None" else m["fallback"][6:-2]
        speed = None if m["speed"] == "None" else float(m["speed"][5:-1])
        try:
            durations = [float(d) for d in m["durations"].split(",") if d.strip()]
        except ValueError as error:
            raise PublishError(f"manifest: clip {m['id']}: bad durations") from error
        if m["id"] in manifest.clips:
            raise PublishError(f"manifest: clip {m['id']} defined twice")
        manifest.clips[m["id"]] = Clip(m["atlas"], int(m["first"]), int(m["count"]), mode,
                                       float(fps) if fps else None, (float(m["px"]), float(m["py"])),
                                       fallback, durations, speed)
        i += 1
    if lines[i:] != ["},", ")"]:
        raise PublishError("manifest: unexpected trailer")
    return manifest


def _identifier(text: str) -> bool:
    return re.fullmatch(r"[a-z][a-z0-9_]*", text) is not None


def validate_manifest(manifest: Manifest) -> list[str]:
    """Same structural rules as ``AnimationManifest::validate`` in Rust, plus the contract pivots."""
    fails = []
    if manifest.version != MANIFEST_VERSION:
        fails.append(f"manifest version {manifest.version}, expected {MANIFEST_VERSION}")
    if manifest.facing != "Right":
        fails.append(f"facing {manifest.facing}, expected Right")
    for aid, atlas in manifest.atlases.items():
        if not _identifier(aid):
            fails.append(f"atlas id {aid!r} is not snake_case")
        if atlas.texture != f"sprites/player/player_{aid}.png":
            fails.append(f"atlas {aid}: texture {atlas.texture!r}, expected sprites/player/player_{aid}.png")
        if min(atlas.cell) <= 0 or atlas.columns <= 0 or atlas.rows <= 0 or atlas.density <= 0:
            fails.append(f"atlas {aid}: zero cell, grid or density")
    for cid, clip in manifest.clips.items():
        atlas = manifest.atlases.get(clip.atlas)
        if not _identifier(cid):
            fails.append(f"clip id {cid!r} is not snake_case")
        if atlas is None:
            fails.append(f"clip {cid}: unknown atlas {clip.atlas!r}")
            continue
        if clip.count <= 0 or clip.first + clip.count > atlas.columns * atlas.rows:
            fails.append(f"clip {cid}: frames {clip.first}..{clip.first + clip.count} outside the atlas")
        px, py = clip.pivot
        if not (0.0 <= px <= atlas.cell[0] and 0.0 <= py <= atlas.cell[1]):
            fails.append(f"clip {cid}: pivot {clip.pivot} outside the cell {atlas.cell}")
        contract = GROUP_PIVOTS.get(clip.atlas)
        if contract is not None:
            (cw, ch), (cpx, cpy) = contract
            d = atlas.density
            if atlas.cell != (cw * d, ch * d) or (px, py) != (cpx * d, cpy * d):
                fails.append(f"clip {cid}: cell {atlas.cell} / pivot {clip.pivot} differ from the contract "
                             f"({(cw * d, ch * d)} / {(cpx * d, cpy * d)})")
        if len(clip.durations) != clip.count:
            fails.append(f"clip {cid}: {len(clip.durations)} durations for {clip.count} frames")
        if any(not (d > 0.0) or d != d or d == float("inf") for d in clip.durations):
            fails.append(f"clip {cid}: a duration is not positive")
        if clip.mode in ("Loop", "Once") and not (clip.fps and clip.fps > 0.0):
            fails.append(f"clip {cid}: fps not positive")
        if clip.speed_reference is not None and not clip.speed_reference > 0.0:
            fails.append(f"clip {cid}: speed reference not positive")
        if clip.fallback is not None and clip.fallback not in manifest.clips:
            fails.append(f"clip {cid}: fallback {clip.fallback!r} does not exist")
    if "idle" not in manifest.clips:
        fails.append("required clip 'idle' missing")
    for cid in manifest.clips:
        seen, current = set(), cid
        while current is not None and current in manifest.clips:
            if current in seen:
                fails.append(f"clip {cid}: fallback cycle")
                break
            seen.add(current)
            current = manifest.clips[current].fallback
    return fails


# --- game durations (read-only) ------------------------------------------------------------


def game_durations(project: Path = PROJECT) -> dict[str, float]:
    """Seconds of every FitToAction clip, read from the game (never written)."""
    constants = (project / "src" / "core" / "constants.rs").read_text(encoding="utf-8")

    def const(name: str) -> float:
        m = re.search(rf"pub const {name}: f32 = ([0-9.]+);", constants)
        if not m:
            raise PublishError(f"constant {name} not found in src/core/constants.rs")
        return float(m.group(1))

    seconds = {"attack_basic": const("BASIC_ATTACK_TIME"), "dodge": const("PLAYER_DODGE_TIME"),
               "hurt": const("PLAYER_HURT_TIME")}
    for name in SKILL_FILES:
        text = (project / name).read_text(encoding="utf-8")
        for block in re.split(r"\n\s*\(\n", text):
            anim = re.search(r'animation: "([a-z_]+)"', block)
            exe = re.search(r"execution_time: ([0-9.]+)", block)
            if anim and exe:
                seconds[anim.group(1)] = float(exe.group(1))
    # A variant (``<clip>_air``) follows the same gameplay action as its base clip.
    return seconds


def duration_failures(manifest: Manifest, seconds: dict[str, float]) -> list[str]:
    fails = []
    for cid, clip in manifest.clips.items():
        base = cid[:-4] if cid.endswith("_air") else cid
        if base not in seconds:
            continue
        total = sum(clip.durations)
        if clip.mode != "FitToAction" or abs(total - seconds[base]) > DURATION_TOLERANCE_S:
            fails.append(f"clip {cid}: {clip.mode} {total:.4f} s, the game action lasts {seconds[base]} s")
    return fails


# --- staged files ----------------------------------------------------------------------------


def manifest_files(manifest: Manifest) -> list[str]:
    """Paths (relative to assets/) of everything a manifest publishes."""
    return [*(a.texture for a in manifest.atlases.values()), MANIFEST]


def image_failures(root: Path, manifest: Manifest) -> list[str]:
    fails = []
    for aid, atlas in manifest.atlases.items():
        path = root / atlas.texture
        try:
            with Image.open(path) as img:
                img.load()
                mode, size, fmt = img.mode, img.size, img.format
                rgba = img.convert("RGBA") if mode == "RGBA" else None
        except (OSError, SyntaxError, ValueError) as error:
            fails.append(f"atlas {aid}: {atlas.texture} is not a readable PNG ({error})")
            continue
        if fmt != "PNG" or mode != "RGBA":
            fails.append(f"atlas {aid}: {fmt} {mode}, expected PNG RGBA")
        if size != atlas.image_size():
            fails.append(f"atlas {aid}: image {size}, grid {atlas.image_size()}")
        if max(size) > MAX_SIDE:
            fails.append(f"atlas {aid}: {size} above {MAX_SIDE} px")
        if rgba is None or size != atlas.image_size():
            continue
        alpha = rgba.getchannel("A")
        for cid, clip in manifest.clips.items():
            if clip.atlas != aid:
                continue
            for k in range(clip.first, clip.first + clip.count):
                col, row = k % atlas.columns, k // atlas.columns
                x, y = col * (atlas.cell[0] + atlas.padding), row * (atlas.cell[1] + atlas.padding)
                if alpha.crop((x, y, x + atlas.cell[0], y + atlas.cell[1])).getbbox() is None:
                    fails.append(f"clip {cid}: frame {k - clip.first} is empty")
    return fails


@dataclass
class Staged:
    root: Path
    manifest: Manifest
    generator: str
    version: tuple[int, ...]
    files: dict[str, str]  # relative path -> sha256


def read_stage(stage: Path) -> Staged:
    info_path = stage / STAGE_INFO
    if not info_path.is_file():
        raise PublishError(f"{info_path} missing: not a staging directory of a post script")
    info = json.loads(info_path.read_text(encoding="utf-8"))
    version = parse_version(str(info.get("generator_version", "")))
    ron = stage / MANIFEST
    if not ron.is_file():
        raise PublishError(f"{ron} missing")
    manifest = parse_manifest(ron.read_text(encoding="utf-8"))
    files = {}
    for rel in manifest_files(manifest):
        path = stage / rel
        if not path.is_file():
            raise PublishError(f"staged file {rel} missing")
        files[rel] = sha256(path)
    return Staged(stage, manifest, str(info.get("generator", "?")), version, files)


def validate_stage(staged: Staged, project: Path = PROJECT) -> list[str]:
    fails = validate_manifest(staged.manifest)
    fails += [f"required clip {c!r} missing" for c in REQUIRED_CLIPS if c not in staged.manifest.clips]
    fails += duration_failures(staged.manifest, game_durations(project))
    fails += image_failures(staged.root, staged.manifest)
    return fails


# --- lock (what is published) ----------------------------------------------------------------


def read_lock(assets_dir: Path) -> dict | None:
    path = assets_dir / LOCK
    if not path.is_file():
        return None
    lock = json.loads(path.read_text(encoding="utf-8"))
    if lock.get("schema") != LOCK_SCHEMA:
        raise PublishError(f"{path}: unknown schema {lock.get('schema')!r}")
    return lock


def current_files(assets_dir: Path, lock: dict) -> dict[str, str]:
    out = {}
    for rel in lock["files"]:
        path = assets_dir / rel
        out[rel] = sha256(path) if path.is_file() else "missing"
    return out


def lock_state(assets_dir: Path) -> tuple[dict | None, str | None, list[str]]:
    """(lock, state id of the files on disk, drift from the lock)."""
    lock = read_lock(assets_dir)
    if lock is None:
        return None, None, []
    files = current_files(assets_dir, lock)
    drift = [f"{rel}: on disk {h[:12]}, lock {lock['files'][rel][:12]}"
             for rel, h in files.items() if h != lock["files"][rel]]
    return lock, state_id(files), drift


def write_lock(assets_dir: Path, version: str, generator: str, manifest: Manifest, files: dict[str, str]) -> None:
    lock = {
        "schema": LOCK_SCHEMA,
        "pipeline_version": version,
        "generator": generator,
        "published_at": dt.datetime.now(dt.timezone.utc).isoformat(timespec="seconds"),
        "clips": {cid: clip.atlas for cid, clip in sorted(manifest.clips.items())},
        "files": dict(sorted(files.items())),
        "state_id": state_id(files),
    }
    _atomic_write(assets_dir / LOCK, (json.dumps(lock, indent=2) + "\n").encode())


def init_lock(assets_dir: Path, version: str, generator: str) -> dict:
    """Registers the files already in ``assets/`` (first use of the guard)."""
    if read_lock(assets_dir) is not None:
        raise PublishError("a lock already exists; publish through the guard instead")
    parse_version(version)
    manifest = parse_manifest((assets_dir / MANIFEST).read_text(encoding="utf-8"))
    fails = validate_manifest(manifest) + image_failures(assets_dir, manifest)
    if fails:
        raise PublishError(f"the current assets are not valid: {fails}")
    files = {rel: sha256(assets_dir / rel) for rel in manifest_files(manifest)}
    write_lock(assets_dir, version, generator, manifest, files)
    return read_lock(assets_dir) or {}


# --- publication -----------------------------------------------------------------------------


def _atomic_write(path: Path, data: bytes) -> None:
    tmp = path.with_name(f".{path.name}.tmp")
    with open(tmp, "wb") as fh:
        fh.write(data)
        fh.flush()
        os.fsync(fh.fileno())
    os.replace(tmp, path)


@dataclass
class PublishResult:
    status: str  # "published" or "unchanged"
    state_id: str
    backup: Path | None = None


def check_publishable(staged: Staged, assets_dir: Path, project: Path = PROJECT) -> tuple[dict | None, str | None]:
    """Every refusal reason, without touching assets/. Returns (lock, current state id)."""
    fails = validate_stage(staged, project)
    lock, current, drift = lock_state(assets_dir)
    if lock is None:
        if (assets_dir / MANIFEST).exists():
            fails.append("assets/ has a manifest but no lock: run init-lock first")
    else:
        if drift:
            fails.append(f"the published files changed outside the guard: {drift}")
        published = parse_version(lock["pipeline_version"])
        if staged.version < published:
            fails.append(f"generator {staged.generator} is version {'.'.join(map(str, staged.version))}, "
                         f"older than the published {lock['pipeline_version']}: historical scripts cannot publish")
        missing = sorted(set(lock["clips"]) - set(staged.manifest.clips))
        if missing:
            fails.append(f"the staged manifest drops published clips {missing}")
    if fails:
        raise PublishError("publication refused:\n  - " + "\n  - ".join(fails))
    return lock, current


def publish(stage: Path, assets_dir: Path = ASSETS, confirm: str | None = None, backup_root: Path = BACKUPS,
            project: Path = PROJECT, before_replace: Callable[[int, str], None] | None = None) -> PublishResult:
    """Validates ``stage`` and replaces the published files, or raises PublishError
    leaving ``assets_dir`` as it was. ``before_replace`` is a test hook."""
    refuse_protected_output(stage, assets_dir)
    if (assets_dir / JOURNAL).exists():
        raise PublishError("an interrupted publication is pending: run recover first")
    staged = read_stage(stage)
    lock, current = check_publishable(staged, assets_dir, project)
    new_id = state_id(staged.files)
    if lock is not None and current == new_id:
        return PublishResult("unchanged", new_id)  # repeated run: nothing to do
    if lock is not None and confirm != current:
        raise PublishError(f"replacing the published state {current} needs --confirm {current} "
                           f"(new state {new_id})")
    stamp = dt.datetime.now().strftime("%Y%m%dT%H%M%S%f")
    backup = backup_root / f"{stamp}_{current or 'empty'}"
    targets = sorted(set(staged.files) | set(lock["files"] if lock else ()) | {LOCK})
    saved = {}
    for rel in targets:
        src = assets_dir / rel
        if src.is_file():
            (backup / rel).parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(src, backup / rel)
            saved[rel] = sha256(backup / rel)
            if saved[rel] != sha256(src):
                raise PublishError(f"backup of {rel} does not match")
    journal = {"backup": str(backup), "saved": saved, "targets": targets, "new_state": new_id}
    (assets_dir / JOURNAL).parent.mkdir(parents=True, exist_ok=True)
    _atomic_write(assets_dir / JOURNAL, json.dumps(journal, indent=2).encode())
    try:
        for n, rel in enumerate(sorted(staged.files)):
            if before_replace is not None:
                before_replace(n, rel)
            dst = assets_dir / rel
            dst.parent.mkdir(parents=True, exist_ok=True)
            _atomic_write(dst, (stage / rel).read_bytes())
        write_lock(assets_dir, ".".join(map(str, staged.version)), staged.generator, staged.manifest,
                   staged.files)
        after, after_id, drift = lock_state(assets_dir)
        if drift or after_id != new_id:
            raise PublishError(f"verification after publishing failed: {drift}")
    except BaseException as error:
        recover(assets_dir)
        raise PublishError(f"publication failed and was rolled back: {error}") from error
    (assets_dir / JOURNAL).unlink()
    return PublishResult("published", new_id, backup)


def recover(assets_dir: Path = ASSETS) -> bool:
    """Restores the backup named by a pending journal. Returns whether it did."""
    path = assets_dir / JOURNAL
    if not path.exists():
        return False
    journal = json.loads(path.read_text(encoding="utf-8"))
    backup = Path(journal["backup"])
    for rel in journal["targets"]:
        dst = assets_dir / rel
        if rel in journal["saved"]:
            _atomic_write(dst, (backup / rel).read_bytes())
            if sha256(dst) != journal["saved"][rel]:
                raise PublishError(f"recovery of {rel} failed: hash differs from the backup")
        elif dst.exists():
            dst.unlink()  # created by the interrupted publication
    path.unlink()
    return True


# --- command line ----------------------------------------------------------------------------


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--assets-dir", type=Path, default=ASSETS)
    sub = parser.add_subparsers(dest="cmd", required=True)
    sub.add_parser("status")
    p = sub.add_parser("check", help="validate a staging directory without publishing")
    p.add_argument("--stage", type=Path, required=True)
    p = sub.add_parser("publish")
    p.add_argument("--stage", type=Path, required=True)
    p.add_argument("--confirm", help="state id of the files being replaced (see status)")
    sub.add_parser("recover")
    p = sub.add_parser("init-lock")
    p.add_argument("--version", required=True)
    p.add_argument("--generator", required=True)
    args = parser.parse_args()
    try:
        if args.cmd == "status":
            lock, current, drift = lock_state(args.assets_dir)
            if lock is None:
                print("no lock: assets/ is not registered (init-lock)")
                return 1
            print(f"pipeline {lock['pipeline_version']} by {lock['generator']}, {len(lock['clips'])} clips, "
                  f"state {current}")
            for line in drift:
                print(f"DRIFT {line}")
            if (args.assets_dir / JOURNAL).exists():
                print("PENDING an interrupted publication (run recover)")
            return 1 if drift else 0
        if args.cmd == "check":
            staged = read_stage(args.stage)
            check_publishable(staged, args.assets_dir)
            print(f"publishable: {len(staged.manifest.clips)} clips, new state {state_id(staged.files)}")
            return 0
        if args.cmd == "publish":
            result = publish(args.stage, args.assets_dir, args.confirm)
            print(f"{result.status}: state {result.state_id}" + (f", backup {result.backup}" if result.backup else ""))
            return 0
        if args.cmd == "recover":
            print("restored the backup" if recover(args.assets_dir) else "nothing to recover")
            return 0
        lock = init_lock(args.assets_dir, args.version, args.generator)
        print(f"lock written: {len(lock['clips'])} clips, state {lock['state_id']}")
        return 0
    except PublishError as error:
        print(f"[publish_guard] {error}")
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
