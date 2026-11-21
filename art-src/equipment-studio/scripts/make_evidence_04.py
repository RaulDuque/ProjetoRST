"""Build the Engine 04 (Equipment Studio V2) evidence (offline tool).

Inputs (all produced by the studio itself, nothing is drawn here):
- `target/equipment_studio/v2/*.png`: real window screenshots of
  `cargo run --example equipment_studio -- --tour` and `-- --tour-reload`
  (the editor driven through its panels, keys and a synthetic mouse);
- `target/equipment_studio/v2/*_WxH.rgba`: the final composite of each
  captured step (the one image every view of the studio shows);
- `target/equipment_studio/v2/tour.log` and `tour_reload.log`;
- the Engine 03 evidence (for the before / after sheet).

Output: `evidence/modular_character_engine/04_studio_v2/`. Also runs Agent
Sprite Forge `pixel_qa.py` on the composites (MIT, Copyright (c) 2026
0x0funky): binary alpha and the run loop seam (`--onion`).

    ~/.local/share/dc-art-tools/asf-venv/bin/python -B \
        art-src/equipment-studio/scripts/make_evidence_04.py
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
CAP = ROOT / "target/equipment_studio/v2"
OUT = ROOT / "evidence/modular_character_engine/04_studio_v2"
E03 = ROOT / "evidence/modular_character_engine/03_equipment_studio"
ASF_QA = Path.home() / ".claude/plugins/cache/agent-sprite-forge/agent-sprite-forge/0.6.0/skills/codeart2d/scripts/pixel_qa.py"

# Canvas area of the studio window (1280x720, panels excluded).
CANVAS = (250, 30, 980, 666)
BG = (36, 34, 44)
INK = (240, 205, 120)
POSES = ["06_p_idle_0", "06_p_idle_1"] + [f"06_p_run_{i}" for i in range(6)]
POSE_MS = [400, 400] + [67] * 6


def shot(name: str) -> Image.Image:
    p = CAP / f"{name}.png"
    if not p.exists():
        raise SystemExit(f"missing capture {p}")
    return Image.open(p).convert("RGB")


def raw(name: str) -> Image.Image:
    hits = sorted(CAP.glob(f"{name}_*x*.rgba"))
    if not hits:
        raise SystemExit(f"missing composite {name}")
    w, h = map(int, re.search(r"_(\d+)x(\d+)\.rgba$", hits[0].name).groups())
    return Image.frombytes("RGBA", (w, h), hits[0].read_bytes())


def on_bg(img: Image.Image, scale: int = 1) -> Image.Image:
    img = img.resize((img.width * scale, img.height * scale), Image.NEAREST)
    out = Image.new("RGB", img.size, BG)
    out.paste(img, (0, 0), img)
    return out


def caption(img: Image.Image, text: str) -> Image.Image:
    out = Image.new("RGB", (img.width, img.height + 20), BG)
    out.paste(img, (0, 20))
    ImageDraw.Draw(out).text((6, 4), text, fill=INK)
    return out


def row(images: list[Image.Image], gap: int = 8) -> Image.Image:
    w = sum(i.width for i in images) + gap * (len(images) - 1)
    h = max(i.height for i in images)
    out = Image.new("RGB", (w, h), BG)
    x = 0
    for i in images:
        out.paste(i, (x, 0))
        x += i.width + gap
    return out


def column(images: list[Image.Image], gap: int = 8) -> Image.Image:
    w = max(i.width for i in images)
    h = sum(i.height for i in images) + gap * (len(images) - 1)
    out = Image.new("RGB", (w, h), BG)
    y = 0
    for i in images:
        out.paste(i, (0, y))
        y += i.height + gap
    return out


def gif(path: Path, frames: list[Image.Image], ms: int | list[int]) -> None:
    w = max(f.width for f in frames)
    h = max(f.height for f in frames)
    norm = []
    for f in frames:
        c = Image.new("RGB", (w, h), BG)
        c.paste(f, (0, 0))
        norm.append(c.quantize(colors=255, method=Image.Quantize.MEDIANCUT))
    norm[0].save(path, save_all=True, append_images=norm[1:], duration=ms, loop=0, disposal=1)


def window_gif(name: str, steps: list[str], ms: int = 900) -> None:
    gif(OUT / name, [caption(shot(s), s) for s in steps], ms)


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(CAP / "01_interface.png", OUT / "01_editor_interface.png")
    window_gif("02_hierarchy_demo.gif", [
        "02_h0_breastplate", "02_h1_rotate", "02_h2_move", "02_h3_undo",
        "02_s0_sword", "02_s1_parent", "02_s3_rotate", "02_s4_rotate"])
    window_gif("03_undo_redo.gif", [
        "03_u0_before", "03_u1_undo", "03_u2_undo", "03_u3_undo_all", "03_u4_redo", "03_u5_redo"])
    window_gif("04_material_editor.gif", [
        "04_m0_select", "04_m1_new_material", "04_m2_base", "04_m3_light", "04_m4_shadow",
        "04_m5_finish", "04_m6_region", "04_m7_rivet", "04_m8_revert_warns"])
    window_gif("08_mouse_editing.gif", [
        "08_k0_select", "08_k1_drag", "08_k2_rotate", "08_k3_vertex_mode", "08_k4_vertex",
        "08_k5_vertex", "08_k6_pixels", "08_k7_brush", "08_k8_fill", "08_k9_undo_all"])
    window_gif("07_equipment_mix.gif", [
        "07_x0_play", "07_x1_jerkin", "07_x2", "07_x3_no_pauldron", "07_x4",
        "07_x5_cuirass_pauldron", "07_x6", "07_x7_no_bracers", "07_x8_no_faulds", "07_x9_all"], 700)

    # 05: every component isolated, the bare body and the whole set (run #2).
    parts = [("05_c_unequipped", "bare (approved art)"),
             ("05_c_isolated_cuirass", "cuirass"),
             ("05_c_isolated_pauldron", "pauldron"),
             ("05_c_isolated_bracers", "bracers"),
             ("05_c_isolated_faulds", "faulds"),
             ("06_p_run_2", "all worn")]
    row([caption(on_bg(raw(n), 4), t) for n, t in parts]).save(OUT / "05_armor_components.png")

    # 06: idle + the 6 run poses, the composite at 4x with its 1x beside it.
    frames = []
    for n in POSES:
        img = raw(n)
        big = on_bg(img, 4)
        frames.append(caption(row([big, on_bg(img)]), f"{n} (4x and 1x, no guides)"))
    gif(OUT / "06_full_armor_idle_run.gif", frames, POSE_MS)
    row([caption(on_bg(raw(n), 2), n.replace("06_p_", "")) for n in POSES]).save(OUT / "06_full_armor_idle_run_frames.png")

    # 09: saved, saved as, reopened by a second process.
    sheet = [caption(shot(s).resize((640, 360)), s) for s in
             ("09_v1_saved", "09_v2_pauldron", "09_v3_save_as", "09_v4_new_item", "09_r0_reopened", "09_r1_material")]
    column([row(sheet[0:2]), row(sheet[2:4]), row(sheet[4:6])]).save(OUT / "09_save_reload.png")

    # 10: Engine 03 (cuirass painted over the arm, guides on) vs Engine 04.
    e03 = Image.open(E03 / "05_six_poses_a_b.png").convert("RGB")
    cell_w = e03.width // 6
    before = e03.crop((cell_w * 2, 0, cell_w * 3, e03.height // 2))
    after = on_bg(raw("06_p_run_2"), 4)
    before = before.resize((round(before.width * after.height / before.height), after.height))
    editors = row([caption(Image.open(E03 / "01_editor_capture.png").convert("RGB").resize((640, 360)), "Engine 03 editor"),
                   caption(shot("01_interface").resize((640, 360)), "Engine 04 editor")])
    poses = row([caption(before, "Engine 03, run #2: one cuirass layer, guides on"),
                 caption(after, "Engine 04, run #2: regions, sockets and depths")])
    column([editors, poses]).save(OUT / "10_before_after_engine03_engine04.png")

    # 11: native resolution, no scaling (bare row, equipped row).
    bare = raw("05_c_unequipped")
    natives = [on_bg(raw(n)) for n in POSES]
    column([caption(row(natives, 4), "1x native: idle 0-1, run 0-5 (equipped)"),
            caption(row([on_bg(bare)], 4), "1x native: run 2 bare")]).save(OUT / "11_native_resolution.png")

    # ASF pixel_qa on the 1x composites.
    work = CAP / "qa"
    work.mkdir(exist_ok=True)
    pngs = []
    for n in POSES[2:]:
        p = work / f"{n}.png"
        raw(n).save(p)
        pngs.append(str(p))
    tools = OUT / "tools"
    tools.mkdir(exist_ok=True)
    report = tools / "asf_pixel_qa_run_composites.json"
    cmd = [sys.executable, "-B", str(ASF_QA), "--input", *pngs, "--onion",
           "--review", str(tools / "asf_pixel_qa_run_review.png"), "--scales", "1,4", "--report", str(report)]
    res = subprocess.run(cmd, cwd=ROOT, capture_output=True, text=True)
    metrics: dict = {"asf_pixel_qa_exit": res.returncode}
    if report.exists():
        data = json.loads(report.read_text())
        metrics["asf_pixel_qa_checks"] = data.get("checks")
    logs = []
    for name in ("tour.log", "tour_reload.log"):
        p = CAP / name
        if p.exists():
            logs += [l for l in p.read_text().splitlines() if "RESULT" in l or "FAIL" in l or "body" in l]
    metrics["tours"] = logs
    (OUT / "metrics.json").write_text(json.dumps(metrics, indent=2) + "\n")
    print(json.dumps(metrics, indent=1)[:2500])


if __name__ == "__main__":
    main()
