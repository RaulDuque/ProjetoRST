"""Build the Engine 05 evidence (offline tool; nothing is drawn here).

Sources:
- `cargo run --example equipment_preview` (called here): the settled 1x
  composites of any item list on idle 0-1 and run 0-5 (the same pure
  functions the studio shows);
- `cargo run --example equipment_studio -- --tour`: window screenshots and
  raw composites in `target/equipment_studio/v2/` (per step) and the
  per-frame recordings in `v2/rec/` (60 Hz fixed clock, `_m` = mirrored);
- `target/equipment_studio/tools05/` (`run_tools_05.py`).

Output: `evidence/modular_character_engine/05_art_and_flexible_equipment/`.
Every enlargement is an integer nearest-neighbour scale; no guide, skeleton
or mark is drawn on the art images (only the editor capture shows guides).

    python3 -B art-src/equipment-studio/scripts/make_evidence_05.py
"""

from __future__ import annotations

import json
import os
import re
import shutil
import subprocess
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[3]
PREVIEW = ROOT / "target/equipment_preview"
CAP = ROOT / "target/equipment_studio/v2"
REC = CAP / "rec"
TOOLS = ROOT / "target/equipment_studio/tools05"
OUT = ROOT / "evidence/modular_character_engine/05_art_and_flexible_equipment"
BIN = ROOT / "target/debug/examples/equipment_preview"
BG = (36, 34, 44)
INK = (240, 205, 120)
POSES = [("idle", 0), ("idle", 1)] + [("run", i) for i in range(6)]
RUN = [("run", i) for i in range(6)]
E04 = ["gothic_set_cuirass", "gothic_set_pauldron", "gothic_set_bracers", "gothic_set_faulds"]
V2 = ["gothic_v2_cuirass", "gothic_v2_pauldron", "gothic_v2_bracers", "gothic_v2_faulds"]


def preview(name: str, *items: str, isolated: bool = False) -> None:
    env = dict(os.environ, CARGO_MANIFEST_DIR=str(ROOT))
    args = [str(BIN), name, *items] + (["--isolated"] if isolated else [])
    res = subprocess.run(args, cwd=ROOT, env=env, capture_output=True, text=True)
    if res.returncode != 0:
        raise SystemExit(res.stderr)


def raw(path: Path) -> Image.Image:
    w, h = map(int, re.search(r"_(\d+)x(\d+)\.rgba$", path.name).groups())
    return Image.frombytes("RGBA", (w, h), path.read_bytes())


def pv(name: str, clip: str, f: int) -> Image.Image:
    return raw(sorted(PREVIEW.glob(f"{name}_{clip}_f{f}_*x*.rgba"))[0])


def cap(step: str) -> Image.Image:
    return raw(sorted(CAP.glob(f"{step}_*x*.rgba"))[0])


def rec(name: str) -> list[Image.Image]:
    out = []
    for p in sorted(REC.glob(f"{name}_*x*.rgba")):
        img = raw(p)
        if "_m_" in p.name:
            img = img.transpose(Image.FLIP_LEFT_RIGHT)
        out.append(img)
    if not out:
        raise SystemExit(f"missing recording {name}")
    return out


def on_bg(img: Image.Image, scale: int = 1, crop: tuple[int, int, int, int] | None = None) -> Image.Image:
    if crop:
        img = img.crop(crop)
    img = img.resize((img.width * scale, img.height * scale), Image.NEAREST)
    out = Image.new("RGB", img.size, BG)
    out.paste(img, (0, 0), img)
    return out


def caption(img: Image.Image, text: str) -> Image.Image:
    out = Image.new("RGB", (img.width, img.height + 18), BG)
    out.paste(img, (0, 18))
    ImageDraw.Draw(out).text((4, 3), text, fill=INK)
    return out


def row(images: list[Image.Image], gap: int = 6) -> Image.Image:
    w = sum(i.width for i in images) + gap * (len(images) - 1)
    h = max(i.height for i in images)
    out = Image.new("RGB", (w, h), BG)
    x = 0
    for i in images:
        out.paste(i, (x, h - i.height))
        x += i.width + gap
    return out


def column(images: list[Image.Image], gap: int = 10) -> Image.Image:
    w = max(i.width for i in images)
    out = Image.new("RGB", (w, sum(i.height for i in images) + gap * (len(images) - 1)), BG)
    y = 0
    for i in images:
        out.paste(i, (0, y))
        y += i.height + gap
    return out


def gif(path: Path, frames: list[Image.Image], ms: int | list[int]) -> None:
    w = max(f.width for f in frames)
    h = max(f.height for f in frames)
    pal = []
    for f in frames:
        c = Image.new("RGB", (w, h), BG)
        c.paste(f, (0, 0))
        pal.append(c.quantize(colors=255, method=Image.Quantize.MEDIANCUT))
    pal[0].save(path, save_all=True, append_images=pal[1:], duration=ms, loop=0, disposal=1)


def rec_gif(name: str, out: str, scale: int = 4, crop=(0, 8, 112, 124), label: str = "") -> None:
    frames = rec(name)[::2]  # 60 Hz recording -> 30 fps
    imgs = []
    for i, f in enumerate(frames):
        big = on_bg(f, scale, crop)
        small = on_bg(f, 1, crop)
        imgs.append(caption(row([big, small]), f"{label}  {i * 2 / 60:.2f} s  ({scale}x and 1x)"))
    gif(OUT / out, imgs, [33, 34] * (len(imgs) // 2) + [33] * (len(imgs) % 2))


def pose_row(name: str, poses, scale: int, crop=None) -> Image.Image:
    return row([on_bg(pv(name, c, f), scale, crop) for c, f in poses])


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    preview("e05_bare")
    preview("e05_e04", *E04)
    preview("e05_v2", *V2)
    preview("e05_v2_sword", *V2, "arming_sword")
    preview("e05_full", *V2, "gothic_cape", "arming_sword")
    preview("e05_cape", "gothic_cape")
    preview("e05_cape_iso", "gothic_cape", isolated=True)
    preview("e05_capeb", "gothic_cape_short")
    preview("e05_capeb_iso", "gothic_cape_short", isolated=True)
    preview("e05_leather_mix", "leather_v2_jerkin", "gothic_v2_pauldron", "gothic_v2_bracers", "gothic_v2_faulds", "gothic_cape")
    for item in V2:
        preview(f"e05_one_{item}", item)
        preview(f"e05_iso_{item}", item, isolated=True)

    torso = (24, 14, 96, 100)
    # 01 before / after (no sword, so the chest is visible).
    b = [("idle", 0), ("run", 2), ("run", 4)]
    rows = [caption(pose_row("e05_e04", b, 4, torso), "Engine 04 gothic set (4x)"),
            caption(pose_row("e05_v2", b, 4, torso), "Engine 05 refined set (4x)"),
            caption(row([pose_row("e05_e04", POSES, 1), pose_row("e05_v2", POSES, 1)], 20), "1x: Engine 04 (left), Engine 05 (right)")]
    column(rows).save(OUT / "01_armor_before_after.png")

    # 02 components: isolated and worn alone (run #2 and run #4), plus the capes.
    cells = []
    for item in V2:
        short = item.replace("gothic_v2_", "")
        cells.append(caption(row([on_bg(pv(f"e05_iso_{item}", "run", 2), 3, torso),
                                  on_bg(pv(f"e05_one_{item}", "run", 2), 3, torso),
                                  on_bg(pv(f"e05_one_{item}", "run", 4), 3, torso)]), f"{short}: isolated, worn alone (run 2, run 4), 3x"))
    cells.append(caption(row([on_bg(pv("e05_cape_iso", "run", 2), 3, (0, 14, 90, 100)),
                              on_bg(pv("e05_cape", "run", 2), 3, (0, 14, 90, 100)),
                              on_bg(pv("e05_cape", "idle", 0), 3, (0, 14, 90, 100))]), "cape: isolated, worn alone (run 2, idle), 3x"))
    column([row(cells[0:2], 16), row(cells[2:4], 16), cells[4]]).save(OUT / "02_equipment_components.png")

    # 03 idle (with and without the sword), 4x, 2x and 1x.
    idle = [("idle", 0), ("idle", 1)]
    crop_idle = (10, 10, 70, 125)
    column([caption(row([pose_row("e05_v2", idle, 4, crop_idle), pose_row("e05_full", idle, 4, crop_idle)], 20),
                    "idle 0-1 (breath): set without the sword | set with cape and sword (4x)"),
            caption(row([pose_row("e05_v2", idle, 2), pose_row("e05_full", idle, 2), pose_row("e05_v2", idle, 1), pose_row("e05_full", idle, 1)], 16), "2x and 1x")
            ]).save(OUT / "03_armor_idle.png")

    # 04 run frames.
    column([caption(pose_row("e05_v2", RUN, 3, (14, 8, 100, 124)), "run 0-5, refined set without the sword (3x)"),
            caption(pose_row("e05_full", RUN, 3, (0, 8, 112, 124)), "run 0-5, set + cape + sword (3x)"),
            caption(row([pose_row("e05_v2", RUN, 1), pose_row("e05_full", RUN, 1)], 20), "1x"),
            ]).save(OUT / "04_armor_run_frames.png")

    # 05 run gif (settled loop, 6 x 1/15 s).
    frames = [caption(row([on_bg(pv("e05_v2", c, f), 4, (14, 8, 100, 124)), on_bg(pv("e05_v2", c, f), 1, (14, 8, 100, 124))]), f"run {f} (4x, 1x)") for c, f in RUN]
    gif(OUT / "05_armor_run.gif", frames, 67)

    # 06 cape isolated.
    column([caption(pose_row("e05_cape_iso", POSES, 2), "gothic cape alone (no body): idle 0-1, run 0-5 (2x)"),
            caption(pose_row("e05_cape", POSES, 2), "worn: the body stays in front of the cloth (2x)"),
            caption(pose_row("e05_cape", POSES, 1), "1x")]).save(OUT / "06_cape_isolated.png")

    rec_gif("cape_idle", "07_cape_idle.gif", label="cape, idle with breath (studio, live)")
    rec_gif("cape_run", "08_cape_run.gif", label="cape, run (studio, live)")
    rec_gif("start_stop", "09_cape_start_stop.gif", label="idle -> run -> idle (studio, live)")
    rec_gif("turnaround", "10_cape_turnaround.gif", label="turnarounds while running (studio, live)")
    full = rec("full_idle")[::2] + rec("full_run")[::2] * 2
    gif(OUT / "11_complete_equipment_set.gif",
        [caption(row([on_bg(f, 4, (0, 8, 112, 124)), on_bg(f, 1, (0, 8, 112, 124))]), "complete set with cape and sword (studio, live)") for f in full], 33)
    rec_gif("swap_run", "12b_swaps_during_the_run.gif", label="pauldron, bracers and cape swapped while running")

    # 12 combinations (studio captures, raw composites of each step).
    combos = [("e05_r1_default_clothes", "default clothes"), ("e05_r2_cuirass", "cuirass"), ("e05_r3_pauldron", "pauldron"),
              ("e05_r4_bracers", "bracers"), ("e05_r5_faulds", "waist"), ("e05_r6_cape", "cape"),
              ("e05_r7_full_set", "full set + sword"), ("e05_r8_leather_mix", "leather + gothic + cape"),
              ("e05_r10_short_cape", "short cape (data only)")]
    cells = [caption(on_bg(cap(s), 3, (0, 8, 112, 124)), t) for s, t in combos]
    column([row(cells[0:5], 8), row(cells[5:9], 8)]).save(OUT / "12_equipment_combinations.png")

    # 13 the two capes (same code, other data).
    column([caption(pose_row("e05_cape", POSES, 2), "gothic_cape: 8 segments, 50 texels, ripple 2.5 Hz (2x)"),
            caption(pose_row("e05_capeb", POSES, 2), "gothic_cape_short: 6 segments, 32 texels, mantle + ragged hem, 5 Hz (2x, data only)"),
            caption(row([pose_row("e05_cape_iso", RUN, 2), pose_row("e05_capeb_iso", RUN, 2)], 16), "isolated run 0-5 (2x)")]).save(OUT / "13_cape_variants.png")

    # 14 native resolution.
    column([caption(pose_row("e05_bare", POSES, 1), "1x bare (approved art)"),
            caption(pose_row("e05_v2", POSES, 1), "1x refined set"),
            caption(pose_row("e05_full", POSES, 1), "1x set + cape + sword"),
            caption(pose_row("e05_leather_mix", POSES, 1), "1x leather + gothic pieces + cape"),
            caption(pose_row("e05_full", POSES, 2), "2x set + cape + sword")]).save(OUT / "14_native_resolution.png")

    shutil.copyfile(CAP / "e05_15_editor.png", OUT / "15_editor_capture.png")
    shutil.copyfile(CAP / "e05_cloth_edit.png", OUT / "15b_editor_cloth_parameter.png")

    tools = OUT / "tools"
    tools.mkdir(exist_ok=True)
    for name in ("tools05.json", "qa_full_run_review.png"):
        if (TOOLS / name).exists():
            shutil.copyfile(TOOLS / name, tools / name)
    logs = []
    for name in ("tour.log", "tour_reload.log"):
        p = CAP / name
        if p.exists():
            logs += [l for l in p.read_text().splitlines() if "RESULT" in l or "FAIL" in l or "body" in l]
    metrics = {"tours": logs}
    if (TOOLS / "tools05.json").exists():
        metrics["tools05"] = json.loads((TOOLS / "tools05.json").read_text())["spring"]
    (OUT / "metrics.json").write_text(json.dumps(metrics, indent=2) + "\n")
    print("\n".join(sorted(p.name for p in OUT.iterdir())))


if __name__ == "__main__":
    main()
