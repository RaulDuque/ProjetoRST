"""Tests of publish_guard.py (V1.5 pipeline safety).

Every test builds a small fake project (game constants, skill RON files and an
``assets/`` tree) in a temporary directory: the real assets are never written.
The only test that runs a real post script checks that it refuses ``assets/``
as output before reading or writing anything.

Run from the project root:
    python3 -m unittest discover -s art-src/player/scripts/post/tests -v
"""

from __future__ import annotations

import hashlib
import json
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

from PIL import Image

POST = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(POST))

import publish_guard as G  # noqa: E402

PROJECT = POST.parents[3]
GROUPS = {  # density 1: contract cells and pivots, columns
    "locomotion": ((64, 64), (32.0, 63.0), 8),
    "combat": ((128, 96), (40.0, 95.0), 7),
}
# (clip, group, frames, mode, durations) - the V1.4 set, with the game timings.
ACTION = {"attack_basic": 0.28, "dodge": 0.28, "hurt": 0.35, "slash_crescent": 0.40, "slash_rising": 0.45,
          "cast_forward": 0.30, "cast_up": 0.60}
V14_CLIPS = [("idle", "locomotion", 4), ("run", "locomotion", 4), ("dodge", "locomotion", 4),
             ("hurt", "locomotion", 4), ("death", "locomotion", 3), ("jump_rise", "locomotion", 2),
             ("fall", "locomotion", 2), ("land", "locomotion", 2), ("crouch", "locomotion", 2),
             ("guard_start", "locomotion", 2), ("attack_basic", "combat", 3), ("slash_crescent", "combat", 3),
             ("slash_rising", "combat", 3), ("cast_forward", "combat", 3), ("cast_up", "combat", 3)]


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def fake_project(root: Path) -> Path:
    """Constants and skill files copied from the real project (read-only)."""
    for rel in ("src/core/constants.rs", *G.SKILL_FILES):
        (root / rel).parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(PROJECT / rel, root / rel)
    return root


def clip_line(key: str, group: str, first: int, count: int, durations: list[float] | None = None,
              pivot: tuple[float, float] | None = None) -> str:
    if key in ACTION or key.endswith("_air"):
        total = ACTION[key.removesuffix("_air")]
        mode, durs = "FitToAction", durations or [total / count] * count
    elif key in ("idle", "run", "fall"):
        mode, durs = "Loop(fps: 12.0)", durations or [1 / 12] * count
    else:
        mode, durs = "Once(fps: 12.0)", durations or [1 / 12] * count
    px, py = pivot or GROUPS[group][1]
    fallback = "None" if key == "idle" else 'Some("idle")'
    d = ", ".join(f"{v:.6f}" for v in durs)
    return (f'        "{key}": (atlas: "{group}", first: {first}, count: {count}, mode: {mode}, '
            f"pivot: ({px}, {py}), fallback: {fallback}, durations: [{d}], speed_reference: None),")


def write_set(root: Path, clips: list[tuple[str, str, int]], version: str | None = "1.5",
              overrides: dict[str, dict] | None = None, colour: int = 200) -> Path:
    """Writes atlases + manifest (+ publish.json when ``version``) under ``root``."""
    overrides = overrides or {}
    firsts: dict[str, int] = {g: 0 for g in GROUPS}
    lines_clips = []
    for key, group, count in clips:
        lines_clips.append(clip_line(key, group, firsts[group], count, **overrides.get(key, {})))
        firsts[group] += count
    atlas_lines = []
    for group, ((cw, ch), _, cols) in GROUPS.items():
        n = max(firsts[group], 1)
        rows = (n + cols - 1) // cols
        size = (cols * cw + (cols - 1) * 2, rows * ch + (rows - 1) * 2)
        img = Image.new("RGBA", size, (0, 0, 0, 0))
        for k in range(n):
            x, y = (k % cols) * (cw + 2), (k // cols) * (ch + 2)
            img.paste((colour, 40, 40, 255), (x + 10, y + 10, x + 30, y + 40))
        path = root / f"sprites/player/player_{group}.png"
        path.parent.mkdir(parents=True, exist_ok=True)
        img.save(path)
        atlas_lines.append(f'        "{group}": (texture: "sprites/player/player_{group}.png", cell: ({cw}, {ch}), '
                           f"columns: {cols}, rows: {rows}, padding: 2, density: 1),")
    ron = ["// test manifest", "(", "    version: 1,", "    facing: Right,", "    atlases: {", *atlas_lines,
           "    },", "    clips: {", *lines_clips, "    },", ")", ""]
    (root / G.MANIFEST).parent.mkdir(parents=True, exist_ok=True)
    (root / G.MANIFEST).write_text("\n".join(ron), encoding="utf-8")
    if version is not None:
        (root / G.STAGE_INFO).write_text(json.dumps({"generator": f"test_v{version}", "generator_version": version}))
    return root


class GuardCase(unittest.TestCase):
    def setUp(self) -> None:
        self._tmp = tempfile.TemporaryDirectory(prefix="publish_guard_")
        self.tmp = Path(self._tmp.name)
        self.project = fake_project(self.tmp / "project")
        self.assets = self.project / "assets"
        self.backups = self.tmp / "backups"
        write_set(self.assets, V14_CLIPS, version=None, colour=100)
        (self.assets / G.STAGE_INFO).unlink(missing_ok=True)
        G.init_lock(self.assets, "1.4", "test_v1.4")
        self.before = self.snapshot()

    def tearDown(self) -> None:
        self._tmp.cleanup()

    def snapshot(self) -> dict[str, str]:
        return {str(p.relative_to(self.assets)): sha(p) for p in sorted(self.assets.rglob("*")) if p.is_file()}

    def stage(self, clips=None, version: str | None = "1.5", **kw) -> Path:
        return write_set(self.tmp / "stage", V14_CLIPS if clips is None else clips, version, **kw)

    def current(self) -> str:
        return G.lock_state(self.assets)[1] or ""

    def publish(self, stage: Path, confirm: str | None = "auto", **kw) -> G.PublishResult:
        return G.publish(stage, self.assets, self.current() if confirm == "auto" else confirm, self.backups,
                         self.project, **kw)

    def assert_refused(self, stage: Path, text: str, **kw) -> None:
        with self.assertRaises(G.PublishError) as ctx:
            self.publish(stage, **kw)
        self.assertIn(text, str(ctx.exception))
        self.assertEqual(self.snapshot(), self.before, "assets changed after a refusal")


class SafetyTests(GuardCase):
    def test_01_old_script_with_incomplete_atlas_is_refused(self) -> None:
        v13 = [c for c in V14_CLIPS if c[0] in ("idle", "run", "dodge", "attack_basic")]
        stage = self.stage(v13, version="1.3")
        with self.assertRaises(G.PublishError) as ctx:
            self.publish(stage)
        message = str(ctx.exception)
        self.assertIn("historical scripts cannot publish", message)
        self.assertIn("drops published clips", message)
        self.assertEqual(self.snapshot(), self.before)

    def test_02_manifest_without_a_required_clip_is_refused(self) -> None:
        stage = self.stage([c for c in V14_CLIPS if c[0] != "fall"])
        self.assert_refused(stage, "required clip 'fall' missing")

    def test_03_invalid_png_is_refused(self) -> None:
        stage = self.stage()
        (stage / "sprites/player/player_combat.png").write_bytes(b"not a png at all")
        self.assert_refused(stage, "is not a readable PNG")

    def test_03b_empty_frame_is_refused(self) -> None:
        stage = self.stage()
        path = stage / "sprites/player/player_locomotion.png"
        with Image.open(path) as img:
            img = img.copy()
        img.paste((0, 0, 0, 0), (0, 0, 64, 64))  # frame 0 of idle
        img.save(path)
        self.assert_refused(stage, "frame 0 is empty")

    def test_04_mismatched_dimensions_are_refused(self) -> None:
        stage = self.stage()
        path = stage / "sprites/player/player_locomotion.png"
        with Image.open(path) as img:
            img.crop((0, 0, img.size[0] - 2, img.size[1])).save(path)
        self.assert_refused(stage, "grid")

    def test_05_invalid_pivot_is_refused(self) -> None:
        self.assert_refused(self.stage(overrides={"run": {"pivot": (32.0, 70.0)}}), "pivot")
        self.assert_refused(self.stage(overrides={"run": {"pivot": (30.0, 63.0)}}), "differ from the contract")

    def test_06_invalid_duration_is_refused(self) -> None:
        self.assert_refused(self.stage(overrides={"hurt": {"durations": [0.075] * 4}}), "the game action lasts 0.35")
        self.assert_refused(self.stage(overrides={"idle": {"durations": [0.1, 0.1, -0.1, 0.1]}}), "not positive")
        self.assert_refused(self.stage(overrides={"idle": {"durations": [0.1, 0.1]}}), "2 durations for 4 frames")

    def test_07_incompatible_version_is_refused(self) -> None:
        stage = self.stage()
        self.publish(stage)  # the lock is now 1.5
        self.before = self.snapshot()
        self.assert_refused(self.stage(version="1.4", colour=50), "older than the published 1.5")
        self.assert_refused(self.stage(version="v1.x", colour=50), "invalid version")

    def test_08_protected_destination_is_refused(self) -> None:
        inside = write_set(self.assets / "staging", V14_CLIPS)
        self.before = self.snapshot()
        self.assert_refused(inside, "staging directory")
        with self.assertRaises(G.PublishError):
            G.refuse_protected_output(self.assets / "sprites", self.assets)

    def test_08b_real_old_scripts_refuse_the_real_assets(self) -> None:
        real = {p: sha(p) for p in sorted((PROJECT / "assets").rglob("*")) if p.is_file()}
        for script in ("game_sprites_v1_3.py", "game_sprites_v1_4.py"):
            run = subprocess.run([sys.executable, str(POST / script), "--assets-dir", str(PROJECT / "assets"),
                                  "--sheet-dir", str(self.tmp / "sheets")],
                                 cwd=POST, capture_output=True, text=True, check=False)
            self.assertNotEqual(run.returncode, 0, script)
            self.assertIn("publish with publish_guard.py", run.stderr, script)
        after = {p: sha(p) for p in sorted((PROJECT / "assets").rglob("*")) if p.is_file()}
        self.assertEqual(after, real)
        self.assertFalse((self.tmp / "sheets").exists(), "the refusal happens before any output")

    def test_09_failed_generation_is_refused(self) -> None:
        stage = self.stage(version=None)  # the generator stopped before publish.json
        self.assert_refused(stage, "not a staging directory")
        stage = self.stage()
        (stage / "sprites/player/player_combat.png").unlink()  # stopped before the second atlas
        self.assert_refused(stage, "staged file sprites/player/player_combat.png missing")
        stage = self.stage()
        text = (stage / G.MANIFEST).read_text()
        (stage / G.MANIFEST).write_text(text[: len(text) // 2])  # truncated manifest
        self.assert_refused(stage, "manifest")

    def test_10_failure_during_publication_rolls_back(self) -> None:
        stage = self.stage(colour=30)

        def crash(n: int, rel: str) -> None:
            if n == 1:
                raise OSError("disk full (simulated)")

        self.assert_refused(stage, "rolled back", before_replace=crash)
        self.assertFalse((self.assets / G.JOURNAL).exists())
        self.assertEqual(G.lock_state(self.assets)[2], [])

    def test_11_repeated_run_is_idempotent_and_needs_confirmation(self) -> None:
        stage = self.stage(colour=60)
        self.assert_refused(stage, "needs --confirm", confirm=None)
        self.assert_refused(stage, "needs --confirm", confirm="000000000000")
        first = self.publish(stage)
        self.assertEqual(first.status, "published")
        self.assertTrue(first.backup and (first.backup / G.MANIFEST).is_file())
        published = self.snapshot()
        second = self.publish(stage, confirm=None)
        self.assertEqual(second.status, "unchanged")
        self.assertEqual(self.snapshot(), published)
        lock = G.read_lock(self.assets) or {}
        self.assertEqual(lock["pipeline_version"], "1.5")
        self.assertEqual(set(lock["clips"]), {c[0] for c in V14_CLIPS})

    def test_11b_published_files_changed_outside_the_guard_block_publication(self) -> None:
        (self.assets / "sprites/player/player_combat.png").write_bytes(b"edited by hand")
        self.before = self.snapshot()
        self.assert_refused(self.stage(), "changed outside the guard")

    def test_12_recovery_after_an_interrupted_publication(self) -> None:
        stage = self.stage(colour=80)
        code = (f"import sys, os; sys.path.insert(0, {str(POST)!r}); import publish_guard as G; from pathlib import Path\n"
                f"def die(n, rel):\n    if n == 2: os._exit(3)\n"
                f"G.publish(Path({str(stage)!r}), Path({str(self.assets)!r}), {self.current()!r}, "
                f"Path({str(self.backups)!r}), Path({str(self.project)!r}), before_replace=die)\n")
        run = subprocess.run([sys.executable, "-I", "-c", code], capture_output=True, text=True, check=False)
        self.assertEqual(run.returncode, 3, run.stderr)
        self.assertTrue((self.assets / G.JOURNAL).exists(), "the crash leaves the journal")
        self.assertNotEqual(G.lock_state(self.assets)[2], [], "one file was already replaced")
        with self.assertRaises(G.PublishError) as ctx:
            self.publish(stage)
        self.assertIn("run recover first", str(ctx.exception))
        self.assertTrue(G.recover(self.assets))
        after = self.snapshot()
        self.assertEqual(after, self.before)
        self.assertFalse(G.recover(self.assets), "nothing left to recover")
        self.assertEqual(self.publish(stage).status, "published")

    def test_variant_clip_follows_the_base_action_duration(self) -> None:
        clips = [*V14_CLIPS, ("attack_basic_air", "combat", 3)]
        self.assertEqual(self.publish(self.stage(clips)).status, "published")
        bad = self.stage(clips, overrides={"attack_basic_air": {"durations": [0.1, 0.1, 0.1]}}, colour=20)
        self.before = self.snapshot()
        self.assert_refused(bad, "attack_basic_air")


class WeaponlessSocketTests(GuardCase):
    """V2.2: a manifest with ``weapon_in_body: false`` publishes the socket
    table with it, and refuses a missing or incomplete one."""

    def weaponless(self, frames: dict[str, int] | None = None, sockets: bool = True) -> Path:
        stage = self.stage(colour=70)
        ron = stage / G.MANIFEST
        ron.write_text(ron.read_text(encoding="utf-8").replace(
            "    facing: Right,", "    facing: Right,\n    weapon_in_body: false,", 1), encoding="utf-8")
        if sockets:
            frame = ("(weapon: (x: 1.0, y: 2.0, angle: 3.0, front: true), hand_l: (x: 0.0, y: 1.0, front: false), "
                     "head: (x: 0.0, y: 2.0), chest: (x: 0.0, y: 1.0), back: (x: 0.0, y: 1.0))")
            counts = {key: count for key, _, count in V14_CLIPS}
            counts.update(frames or {})
            lines = ["(", "    version: 1,", "    clips: {"]
            lines += [f'        "{k}": [' + ", ".join([frame] * n) + "]," for k, n in counts.items()]
            lines += ["    },", ")", ""]
            (stage / G.SOCKETS).write_text("\n".join(lines), encoding="utf-8")
        return stage

    def test_the_optional_header_line_parses_and_defaults_to_baked(self) -> None:
        baked = G.parse_manifest((self.assets / G.MANIFEST).read_text(encoding="utf-8"))
        self.assertTrue(baked.weapon_in_body)
        self.assertNotIn(G.SOCKETS, G.manifest_files(baked))
        stage = self.weaponless()
        manifest = G.parse_manifest((stage / G.MANIFEST).read_text(encoding="utf-8"))
        self.assertFalse(manifest.weapon_in_body)
        self.assertIn(G.SOCKETS, G.manifest_files(manifest))

    def test_sockets_are_published_with_the_weaponless_atlases(self) -> None:
        result = self.publish(self.weaponless())
        self.assertEqual(result.status, "published")
        self.assertTrue((self.assets / G.SOCKETS).is_file())
        lock = G.read_lock(self.assets) or {}
        self.assertIn(G.SOCKETS, lock["files"])

    def test_missing_or_incomplete_sockets_are_refused(self) -> None:
        self.assert_refused(self.weaponless(sockets=False), "player_sockets.ron missing")
        first = V14_CLIPS[0]
        self.assert_refused(self.weaponless({first[0]: first[2] + 1}), f"clip {first[0]} has")

    def test_a_rollback_removes_the_new_socket_file(self) -> None:
        def crash(n: int, rel: str) -> None:
            if rel == G.SOCKETS:
                raise OSError("disk full (simulated)")

        self.assert_refused(self.weaponless(), "rolled back", before_replace=crash)
        self.assertFalse((self.assets / G.SOCKETS).exists())


class RealAssetsTests(unittest.TestCase):
    def test_shipped_assets_match_their_lock(self) -> None:
        assets = PROJECT / "assets"
        lock, _, drift = G.lock_state(assets)
        self.assertIsNotNone(lock, "assets/animations/player.publish.json missing")
        self.assertEqual(drift, [])
        manifest = G.parse_manifest((assets / G.MANIFEST).read_text(encoding="utf-8"))
        self.assertEqual(G.validate_manifest(manifest) + G.image_failures(assets, manifest)
                         + G.duration_failures(manifest, G.game_durations()), [])
        self.assertEqual(sorted(lock["clips"]), sorted(manifest.clips))
        self.assertFalse((assets / G.JOURNAL).exists())


if __name__ == "__main__":
    unittest.main()
