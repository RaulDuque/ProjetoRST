"""Build the Engine 03 (Equipment Studio) evidence (offline tool).

Inputs:
- `target/equipment_studio/*.png`: real screenshots of
  `cargo run --example equipment_studio -- --tour` and `-- --tour-reload`
  (the editor driven through its own keys), with `tour.log` /
  `tour_reload.log`;
- `target/equipment_studio/preview/*.rgba`: the 1x rasters the generic
  painter produced (`STUDIO_DUMP=1 cargo test --test equipment_studio_tests`);
- `target/equipment_studio/asf/`: the Agent Sprite Forge renders of
  `scripts/build_imports.py`.

Output: `evidence/modular_character_engine/03_equipment_studio/`. Also runs
ASF `pixel_qa.py` on the painted rasters (MIT, Copyright (c) 2026 0x0funky).

    ~/.local/share/dc-art-tools/asf-venv/bin/python -B \
        art-src/equipment-studio/scripts/make_evidence_03.py
"""

from __future__ import annotations

import json
import re
import shutil
import subprocess
import sys
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[3]
CAP = ROOT / "target/equipment_studio"
PREVIEW = CAP / "preview"
OUT = ROOT / "evidence/modular_character_engine/03_equipment_studio"
STUDIO = ROOT / "art-src/equipment-studio"
RUN6 = ROOT / "art-src/n3-8-production-run/stage/sprites/player/n36_run6.png"
IDLE = ROOT / "art-src/n3-8-production-run/stage/sprites/player/n35_idle.png"
ASF_QA = Path.home() / ".claude/plugins/cache/agent-sprite-forge/agent-sprite-forge/0.6.0/skills/codeart2d/scripts/pixel_qa.py"

# Screen regions of the studio window (1280x720).
MAIN = (600, 120, 1060, 640)
X2 = (1046, 360, 1276, 640)
X1 = (1140, 108, 1260, 250)
BG = (40, 38, 48)
INK = (240, 205, 120)
RUN_MS = 67


def load(name: str) -> Image.Image:
    p = CAP / f"{name}.png"
    if not p.exists():
        raise SystemExit(f"missing capture {p}")
    return Image.open(p).convert("RGB")


def caption(img: Image.Image, text: str) -> Image.Image:
    out = Image.new("RGB", (img.width, img.height + 22), BG)
    out.paste(img, (0, 22))
    ImageDraw.Draw(out).text((6, 5), text, fill=INK)
    return out


def row(images: list[Image.Image], gap: int = 6) -> Image.Image:
    out = Image.new("RGB", (sum(i.width for i in images) + gap * (len(images) - 1), max(i.height for i in images)), BG)
    x = 0
    for i in images:
        out.paste(i, (x, 0))
        x += i.width + gap
    return out


def column(images: list[Image.Image], gap: int = 6) -> Image.Image:
    out = Image.new("RGB", (max(i.width for i in images), sum(i.height for i in images) + gap * (len(images) - 1)), BG)
    y = 0
    for i in images:
        out.paste(i, (0, y))
        y += i.height + gap
    return out


def gif(frames: list[Image.Image], name: str, ms) -> None:
    frames[0].save(OUT / name, save_all=True, append_images=frames[1:], duration=ms, loop=0, disposal=1)


def raw(name: str) -> Image.Image:
    files = list(PREVIEW.glob(f"{name}_*.rgba"))
    if not files:
        raise SystemExit(f"missing raster {name} (run the test with STUDIO_DUMP=1)")
    w, h = map(int, re.search(r"_(\d+)x(\d+)\.rgba$", files[0].name).groups())
    return Image.frombytes("RGBA", (w, h), files[0].read_bytes())


def body(clip: str, frame: int) -> Image.Image:
    if clip == "idle":
        return Image.open(IDLE).convert("RGBA")
    strip = Image.open(RUN6).convert("RGBA")
    return strip.crop((frame * 112, 0, frame * 112 + 112, 128))


def isolated(design: str, clip: str, frame: int, k: int) -> Image.Image:
    img = raw(f"{design}_{clip}_f{frame}")
    box = img.getchannel("A").getbbox() or (0, 0, 8, 8)
    img = img.crop((box[0] - 2, box[1] - 2, box[2] + 2, box[3] + 2))
    big = Image.new("RGBA", (img.width * k, img.height * k), BG + (255,))
    big.alpha_composite(img.resize((img.width * k, img.height * k), Image.NEAREST))
    return big.convert("RGB")


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    (OUT / "editor").mkdir(exist_ok=True)
    metrics: dict[str, object] = {}

    # Real editor captures (full window).
    for name in ("t00_start", "edit_select_part", "new_belt_saved", "reload_selected", "final_all"):
        shutil.copyfile(CAP / f"{name}.png", OUT / "editor" / f"{name}.png")
    load("edit_select_part").save(OUT / "01_editor_capture.png")

    # GIF: a visual change of one part (vertex then colour), full window.
    names = ["edit_select_part", "edit_vertex_1", "edit_vertex_2", "edit_vertex_3", "edit_vertex_4",
             "edit_colour_mode", "edit_colour_1", "edit_colour_2", "edit_colour_3", "edit_reload_discard"]
    frames = [load(n).resize((960, 540), Image.NEAREST) for n in names]
    gif(frames, "02_edit_part_visual.gif", 500)
    row([caption(load(n).crop(MAIN), n) for n in names[:5]]).save(OUT / "02_edit_part_visual_frames.png")

    # GIF: pivot and socket (mount) of the imported sword.
    names = ["pivot_select_sword", "pivot_move_1", "pivot_move_2", "pivot_rotate_back", "pivot_rotate_more",
             "mount_mode", "mount_move", "mount_rotate", "mount_reload"]
    frames = [load(n).resize((960, 540), Image.NEAREST) for n in names]
    gif(frames, "03_pivot_and_socket.gif", 600)
    row([caption(load(n).crop(MAIN), n) for n in names[:8]]).save(OUT / "03_pivot_and_socket_frames.png")

    # GIF: creating the third item and swapping during the run.
    names = ["new_design", "new_slot_belt", "new_belt_shape", "new_belt_colour", "new_belt_buckle",
             "new_belt_buckle_place", "new_belt_saved", "swap_play", "swap_1_jerkin", "swap_2", "swap_3_gothic",
             "swap_4", "swap_5_sword_off", "swap_6", "swap_7_sword_on", "swap_8_belt_off", "swap_9_belt_on"]
    frames = [load(n).resize((960, 540), Image.NEAREST) for n in names]
    gif(frames, "04_create_and_swap.gif", [700] * 7 + [250] * 10)
    swap = [caption(load(n).crop(MAIN), n) for n in names[7:]]
    gif(swap, "04b_swap_during_run.gif", 200)

    # Both breastplates, six run poses each (real captures) + isolated rasters.
    a = [caption(load(f"pose_a_{f}").crop(MAIN), f"A gothic, run #{f}") for f in range(6)]
    b = [caption(load(f"pose_b_{f}").crop(MAIN), f"B leather, run #{f}") for f in range(6)]
    gif(a, "05_six_poses_model_a.gif", RUN_MS * 2)
    gif(b, "05_six_poses_model_b.gif", RUN_MS * 2)
    column([row(a), row(b)]).save(OUT / "05_six_poses_a_b.png")
    comp = []
    for d, label in (("gothic_cuirass", "A: gothic (9 parts)"), ("leather_jerkin", "B: leather (8 parts)")):
        comp.append(caption(row([isolated(d, "idle", 0, 6)] + [isolated(d, "run", f, 6) for f in (0, 2, 4)]), label))
    column(comp).save(OUT / "06_breastplates_compared_isolated.png")

    # Isolated equipment (every design, run #2 and idle, 6x).
    tiles = []
    for d in ("gothic_cuirass", "leather_jerkin", "arming_sword", "studio_item_1", "plate_cuirass_02"):
        tiles.append(caption(row([isolated(d, "idle", 0, 6), isolated(d, "run", 2, 6)]), d))
    column(tiles).save(OUT / "07_equipment_isolated.png")
    row([caption(load(n).crop(MAIN), n) for n in ("idle_equipped", "idle_unequipped", "idle_isolated", "run_isolated")]).save(OUT / "07b_equipped_unequipped_isolated.png")

    # 1x, 2x and enlarged (the real preview views and nearest enlargements).
    cap = load("pose_a_2")
    x1 = cap.crop(X1)
    x2 = cap.crop(X2)
    main = cap.crop(MAIN)
    comp1 = Image.new("RGBA", (112, 128), BG + (255,))
    comp1.alpha_composite(body("run", 2))
    comp1.alpha_composite(raw("gothic_cuirass_run_f2"))
    big = comp1.resize((112 * 8, 128 * 8), Image.NEAREST).convert("RGB").crop((300, 120, 760, 620))
    column([row([caption(x1, "1x (studio preview)"), caption(x2, "2x (studio preview)"), caption(main, "4x (studio main view)")]),
            caption(big, "8x nearest enlargement of the 1x composite (approved frame + painted item; near arm not re-layered)")]).save(OUT / "08_resolution_1x_2x_enlarged.png")

    # Save, close, reload (two processes).
    row([caption(load("new_belt_saved").resize((640, 360)), "process 1: Ctrl+S saved studio_item_1.ron"),
         caption(load("reload_selected").resize((640, 360)), "process 2 (--tour-reload): reopened and selected")]).save(OUT / "09_save_and_reload.png")
    shutil.copyfile(STUDIO / "items/studio_item_1.ron", OUT / "09_studio_item_1.ron")

    # ASF imports and pixel_qa of the painted rasters.
    tools = OUT / "tools"
    tools.mkdir(exist_ok=True)
    for src, name in ((CAP / "asf/buckle/preview-x8.png", "asf_render_pixelspec_buckle_x8.png"),
                      (CAP / "asf/emblem/base/emblem@8x.png", "asf_svg_render_emblem_x8.png"),
                      (CAP / "asf/build_imports.json", "asf_build_imports.json")):
        if src.exists():
            shutil.copyfile(src, tools / name)
    qa = {}
    work = CAP / "qa"
    work.mkdir(exist_ok=True)
    for d in ("gothic_cuirass", "leather_jerkin", "arming_sword", "studio_item_1"):
        pngs = []
        for f in range(6):
            p = work / f"{d}_run_f{f}.png"
            raw(f"{d}_run_f{f}").save(p)
            pngs.append(str(p))
        rep = work / f"{d}_qa.json"
        r = subprocess.run([sys.executable, "-B", str(ASF_QA), "--input", *pngs, "--report", str(rep), "--once"],
                           capture_output=True, text=True)
        qa[d] = {"returncode": r.returncode,
                 "checks": json.loads(rep.read_text()).get("checks") if rep.exists() else r.stderr[-400:]}
    metrics["asf_pixel_qa_painted_run"] = qa
    log = (CAP / "tour.log").read_text().splitlines() + (CAP / "tour_reload.log").read_text().splitlines()
    metrics["tour"] = [l for l in log if "RESULT" in l or "FAIL" in l or "body" in l]
    (OUT / "metrics.json").write_text(json.dumps(metrics, indent=2) + "\n")
    print(json.dumps({k: v["checks"] if isinstance(v, dict) else v for k, v in qa.items()}, indent=1)[:1500])


if __name__ == "__main__":
    main()
