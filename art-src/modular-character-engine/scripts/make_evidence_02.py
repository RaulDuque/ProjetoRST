"""Build the prototype 02 evidence from the Bevy captures (offline tool).

Inputs (written by `cargo run --example modular_character_engine`,
`cargo test --test modular_character_engine_tests -- --nocapture` saved in
`test_metrics.log`, and `scripts/run_external_tools.py`):
`target/modular_character_engine/`. Output:
`evidence/modular_character_engine/02_equipamentos/`. Every character image is
a crop of what Bevy rendered (captions are added around it); the isolated
breastplates are the rasters the lab drew (`raw/`).

    ~/.local/share/dc-art-tools/asf-venv/bin/python -B \
        art-src/modular-character-engine/scripts/make_evidence_02.py
"""

from __future__ import annotations

import json
import re
import shutil
from pathlib import Path

from PIL import Image, ImageChops, ImageDraw

ROOT = Path(__file__).resolve().parents[3]
CAP = ROOT / "target/modular_character_engine"
TOOLS = CAP / "tools"
OUT = ROOT / "evidence/modular_character_engine/02_equipamentos"
EV01 = ROOT / "evidence/modular_character_engine/01_prototipo"
STAGE = ROOT / "art-src/modular-character-engine/stage"
FACTORY = ROOT / "art-src/n4-sprite-machine/stage/factory/isolated"
RUN6 = ROOT / "art-src/n3-8-production-run/stage/sprites/player/n36_run6.png"
IDLE = ROOT / "art-src/n3-8-production-run/stage/sprites/player/n35_idle.png"

PANELS = {"original": 220, "hybrid": 640, "articulated": 1060}
HALF_W = 190
TOP, BOTTOM = 190, 640
RUN_MS = 67
BG = (48, 43, 56)
INK = (240, 205, 120)


def load(name: str) -> Image.Image:
    path = CAP / f"{name}.png"
    if not path.exists():
        raise SystemExit(f"missing capture {path} (run the example first)")
    return Image.open(path).convert("RGB")


def crop(image: Image.Image, panel: str = "hybrid") -> Image.Image:
    x = PANELS[panel]
    return image.crop((x - HALF_W, TOP, x + HALF_W, BOTTOM))


def caption(image: Image.Image, text: str) -> Image.Image:
    out = Image.new("RGB", (image.width, image.height + 22), BG)
    out.paste(image, (0, 22))
    ImageDraw.Draw(out).text((6, 5), text, fill=INK)
    return out


def row(images: list[Image.Image], gap: int = 6) -> Image.Image:
    w = sum(i.width for i in images) + gap * (len(images) - 1)
    out = Image.new("RGB", (w, max(i.height for i in images)), BG)
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


def gif(frames: list[Image.Image], name: str, durations: list[int] | int) -> None:
    frames[0].save(OUT / name, save_all=True, append_images=frames[1:], duration=durations, loop=0, disposal=1)


def log_lines() -> dict[str, str]:
    out = {}
    for line in (CAP / "run.log").read_text().splitlines():
        m = re.match(r"\[modular_character_engine\] (\S+) clip=", line)
        if m:
            out[m.group(1)] = line
    return out


def label(log: dict[str, str], name: str) -> str:
    line = log.get(name, "")
    items = re.findall(r'"([a-z_]+)"', line)
    flip = "left" if "flip=true" in line else "right"
    f = re.search(r"frame=(\d+)", line)
    return f"frame {f.group(1) if f else '?'}  facing {flip}  worn: {', '.join(items) or 'nothing'}"


def joints_of(path: Path, clip: str) -> list[dict[str, tuple[float, float]]]:
    text = path.read_text()
    block = text[text.index(f'"{clip}": ('):]
    block = block[: block.index("frames:")]
    out = []
    for src in re.findall(r"\{ pelvis:.*?\}", block, re.S):
        out.append({k: (float(a), float(b)) for k, a, b in re.findall(r"(\w+): \(([\d.]+), ([\d.]+)\)", src)})
    return out


BONES = [("pelvis", "neck"), ("neck", "head"), ("shoulder_near", "elbow_near"), ("elbow_near", "hand_near"),
         ("shoulder_far", "elbow_far"), ("elbow_far", "hand_far"), ("pelvis", "knee_a"), ("knee_a", "ankle_a"),
         ("ankle_a", "toe_a"), ("pelvis", "knee_b"), ("knee_b", "ankle_b"), ("ankle_b", "toe_b")]


def skeleton_compare() -> Image.Image:
    """01 (red) vs 02 (cyan) marks over the approved run frames, 4x."""
    j01 = joints_of(STAGE / "history/rig_lab_01.ron", "run")
    j02 = joints_of(STAGE / "rig_lab.ron", "run")
    strip = Image.open(RUN6).convert("RGBA")
    tiles = []
    k = 4
    for i in range(6):
        cell = strip.crop((i * 112, 0, i * 112 + 112, 128))
        big = Image.new("RGBA", (112 * k, 128 * k), BG + (255,))
        big.alpha_composite(cell.resize((112 * k, 128 * k), Image.NEAREST))
        d = ImageDraw.Draw(big)
        for joints, colour in ((j01[i], (255, 70, 70)), (j02[i], (60, 230, 255))):
            for a, b in BONES:
                pa, pb = joints[a], joints[b]
                d.line([((pa[0] + 0.5) * k, (pa[1] + 0.5) * k), ((pb[0] + 0.5) * k, (pb[1] + 0.5) * k)], fill=colour, width=2)
            for name in ("shoulder_near", "hand_near"):
                p = joints[name]
                d.ellipse([(p[0] + 0.5) * k - 4, (p[1] + 0.5) * k - 4, (p[0] + 0.5) * k + 4, (p[1] + 0.5) * k + 4], outline=colour, width=2)
        tiles.append(caption(big.convert("RGB").crop((0, 40, 112 * k, 128 * k)), f"run #{i}: red = 01, cyan = 02"))
    return column([row(tiles[:3]), row(tiles[3:])])


def raw_png(name: str) -> Image.Image | None:
    for f in (CAP / "raw").glob(f"{name}_*.rgba"):
        m = re.search(r"_(\d+)x(\d+)\.rgba$", f.name)
        return Image.frombytes("RGBA", (int(m.group(1)), int(m.group(2))), f.read_bytes())
    return None


def items_sheet() -> Image.Image:
    k = 4
    tiles = []
    for title, path, pivot in (
        ("sword_arming_cross (N41-B)", FACTORY / "sword_arming_cross.png", (10, 53.5)),
        ("sword_estoc_ring (N41-B)", FACTORY / "sword_estoc_ring.png", (10, 53.5)),
        ("pauldron_steel (derived 2:1)", STAGE / "equipment/pauldron_steel_half.png", (7, 5)),
        ("pauldron_bronze (derived 2:1)", STAGE / "equipment/pauldron_bronze_half.png", (7, 5)),
    ):
        img = Image.open(path).convert("RGBA")
        big = img.resize((img.width * k, img.height * k), Image.NEAREST)
        tile = Image.new("RGB", (max(big.width, 150) + 16, 64 * k + 40), BG)
        tile.paste(big, (8, 34), big)
        d = ImageDraw.Draw(tile)
        cx, cy = 8 + pivot[0] * k, 34 + pivot[1] * k
        d.line((cx - 8, cy, cx + 8, cy), fill=(0, 255, 255))
        d.line((cx, cy - 8, cx, cy + 8), fill=(0, 255, 255))
        d.text((4, 4), title, fill=INK)
        tiles.append(tile)
    plates = []
    for clip, frames in (("idle", 2), ("run", 6)):
        for f in range(frames):
            img = raw_png(f"chest_projected_{clip}_f{f}")
            if img is None:
                continue
            box = img.getchannel("A").getbbox()
            img = img.crop((box[0] - 2, box[1] - 2, box[2] + 2, box[3] + 2))
            big = img.resize((img.width * k, img.height * k), Image.NEAREST)
            t = Image.new("RGB", (big.width + 12, big.height + 30), BG)
            t.paste(big, (6, 24), big)
            ImageDraw.Draw(t).text((4, 4), f"{clip} #{f}", fill=INK)
            plates.append(t)
    head = Image.new("RGB", (900, 22), BG)
    ImageDraw.Draw(head).text((6, 5), "breastplate: ONE vector model (rig_lab.ron, plate_cuirass) drawn on each pose's torso cage, isolated (4x)", fill=INK)
    return column([row(tiles, 10), head, row(plates, 8)])


def tools_evidence(metrics: dict) -> None:
    dst = OUT / "tools"
    dst.mkdir(exist_ok=True)
    copies = {
        TOOLS / "asf_rig/render/review/run.png": "asf_rig_animate_review.png",
        TOOLS / "asf_rig/render/review/run-onion.png": "asf_rig_animate_onion.png",
        TOOLS / "asf_rig/skeleton.rig.svg": "asf_skeleton.rig.svg",
        TOOLS / "asf_rig/skeleton.anim.json": "asf_skeleton.anim.json",
        TOOLS / "motion_approved_run/legs.png": "asf_pixel_motion_legs_approved.png",
        TOOLS / "motion_hybrid_run_equipped/legs.png": "asf_pixel_motion_legs_hybrid_equipped.png",
        TOOLS / "clips_hybrid_run_equipped/contact-sheet.png": "asf_build_animation_clips_contact.png",
        TOOLS / "qa_chest/review_run.png": "asf_pixel_qa_breastplate_run.png",
        TOOLS / "qa_chest/review_idle.png": "asf_pixel_qa_breastplate_idle.png",
        TOOLS / "pixelfx_act/sword-slash-act.png": "pixelfx_act_sword_slash.png",
        TOOLS / "external_tools.json": "external_tools.json",
        TOOLS / "asf_rig_crosscheck.json": "asf_rig_crosscheck.json",
    }
    turn = sorted((TOOLS / "clips_hybrid_run_equipped/review").glob("*turn*.png"))
    if turn:
        copies[turn[0]] = "asf_build_animation_clips_turn_test.png"
    copied = []
    for src, name in copies.items():
        if src.exists():
            shutil.copyfile(src, dst / name)
            copied.append(name)
    metrics["tool_evidence_files"] = copied
    # Our FK joints over the ASF rig_animate frames.
    export = json.loads((CAP / "rig_export.json").read_text())
    frames = sorted((TOOLS / "asf_rig/render/frames").glob("run-*.png"))
    over = []
    for i, f in enumerate(frames):
        img = Image.open(f).convert("RGBA")
        bg = Image.new("RGBA", img.size, BG + (255,))
        bg.alpha_composite(img)
        d = ImageDraw.Draw(bg)
        z = img.width / export["canvas"][0]
        for seg in export["frames"][i]["fk"].values():
            (ax, ay), (bx, by) = seg
            d.line([(ax * z, ay * z), (bx * z, by * z)], fill=(255, 255, 255), width=1)
            d.ellipse([bx * z - 3, by * z - 3, bx * z + 3, by * z + 3], outline=(255, 255, 255))
        over.append(caption(bg.convert("RGB"), f"ASF rig_animate frame {i} + our Rust FK (white)"))
    if over:
        gif(over, "tools/asf_rig_animate_vs_rust_fk.gif", RUN_MS * 3)
        row(over[:3]).save(dst / "asf_rig_animate_vs_rust_fk.png")


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    log = log_lines()
    metrics: dict[str, object] = {}

    # Bare reconstruction stays identical.
    diffs = {}
    for name in [f"bare_idle_f{f}" for f in range(2)] + [f"bare_run_f{f}" for f in range(6)]:
        cap = load(name)
        d = ImageChops.difference(crop(cap, "original"), crop(cap, "hybrid")).convert("L").point(lambda v: 255 if v else 0)
        diffs[name] = d.histogram()[255]
    metrics["original_vs_hybrid_bare_differing_screen_pixels"] = diffs
    metrics["note_bare"] = "bare_idle_f1 is the experimental breath frame (upper body one texel lower): it differs on purpose"

    # 1. Run with the corrected grip; 2. idle with the sword.
    run = [caption(crop(load(f"sword_run_f{f}")), f"run #{f}: grip profile sword_run") for f in range(6)]
    gif(run, "01_run_sword_grip.gif", RUN_MS * 2)
    row(run).save(OUT / "01_run_sword_grip_frames.png")
    idle = [caption(crop(load(f"sword_idle_f{f}")), f"idle #{f}: grip profile sword_idle") for f in range(2)]
    gif(idle, "02_idle_sword.gif", [700, 500])
    row(idle).save(OUT / "02_idle_sword_frames.png")

    # 3. Breastplate in every pose, three approaches.
    proj = [caption(crop(load(f"chest_projected_{c}_f{f}")), f"breastplate (projected) {c} #{f}")
            for c, n in (("idle", 2), ("run", 6)) for f in range(n)]
    gif(proj, "03_breastplate_all_poses.gif", [700, 500] + [RUN_MS * 2] * 6)
    rows = []
    for mode in ("rigid", "warp", "projected"):
        tiles = [caption(crop(load(f"chest_{mode}_{c}_f{f}")), f"{mode} {c} #{f}") for c, f in
                 (("idle", 0), ("run", 0), ("run", 1), ("run", 2), ("run", 3), ("run", 4), ("run", 5))]
        rows.append(row(tiles))
    column(rows).save(OUT / "03_breastplate_approaches.png")

    # 4. Swap during the run; 5. combination, turn, mirror.
    swap = [caption(crop(load(f"swap_s{s:02d}")), label(log, f"swap_s{s:02d}")) for s in range(24)]
    gif(swap, "04_swap_during_run.gif", RUN_MS * 2)
    row(swap[3:11]).save(OUT / "04_swap_during_run_frames.png")
    combo = [caption(crop(load(f"all_run_f{f}")), f"sword + pauldron + breastplate, run #{f}") for f in range(6)]
    gif(combo, "05_combo_sword_pauldron_breastplate.gif", RUN_MS * 2)
    row(combo).save(OUT / "05_combo_frames.png")
    turn = [caption(crop(load(f"turn_t{s:02d}")), label(log, f"turn_t{s:02d}")) for s in range(18)]
    gif(turn, "05b_turn_around.gif", RUN_MS * 2)
    caption(crop(load("mirror_run_f4")), "mirrored run #4: skeleton, sockets, same grip").save(OUT / "05c_mirror.png")
    depth = [caption(crop(load(f"depth_run_f{f}")), f"depth layers #{f} (items cyan)") for f in range(6)]
    row(depth).save(OUT / "05d_depth_layers.png")
    views = ["all layers", "far arm", "legs", "torso", "hair", "near arm", "equipment only"]
    row([caption(crop(load(f"layers_run_f2_{i}")), views[i]) for i in range(7)]).save(OUT / "05e_layers_one_by_one.png")

    # 6. Isolated equipment.
    items_sheet().save(OUT / "06_equipment_isolated.png")
    caption(row([crop(load("exploded_idle_f0")), crop(load("exploded_idle_f0"), "articulated")]),
            "exploded: items alone with their pivots | articulated pieces").save(OUT / "06b_exploded.png")

    # 7. Comparison with prototype 01.
    old = Image.open(EV01 / "03_run_equipped.gif")
    pairs = []
    for f in range(6):
        old.seek(f)
        a = old.convert("RGB")
        b = caption(crop(load(f"sp_run_f{f}")), f"02 run #{f}: refined marks + grip profile")
        a = a.resize((b.width, int(a.height * b.width / a.width)), Image.NEAREST) if a.width != b.width else a
        pairs.append(row([a, b]))
    gif(pairs, "07_compare_01_vs_02.gif", RUN_MS * 3)
    column(pairs[:3]).save(OUT / "07_compare_01_vs_02.png")
    skeleton_compare().save(OUT / "07b_skeleton_01_vs_02.png")
    sk = [caption(crop(load(f"skeleton_run_f{f}")), f"02 skeleton + torso box + sockets #{f}") for f in range(6)]
    gif(sk, "07c_skeleton_motion.gif", RUN_MS * 2)

    # 8. Real captures of the interactive lab (keys pressed through ButtonInput).
    ui = sorted(CAP.glob("ui_*.png"))
    lab = OUT / "lab"
    lab.mkdir(exist_ok=True)
    thumbs = []
    for f in ui:
        shutil.copyfile(f, lab / f.name)
        im = Image.open(f).convert("RGB").resize((640, 360))
        thumbs.append(caption(im, f.stem))
    column([row(thumbs[i:i + 3]) for i in range(0, len(thumbs), 3)]).save(OUT / "08_lab_interactive.png")

    # 9. Stability and partition metrics; 10. external tools.
    tm = (CAP / "test_metrics.log").read_text()
    m = re.search(r"spread 01 ([\d.]+) -> 02 ([\d.]+) texels, jerk 01 ([\d.]+) -> 02 ([\d.]+) texels; sword angle jerk: 01 ([\d.]+) -> 02 ([\d.]+)", tm)
    if m:
        v = [float(x) for x in m.groups()]
        metrics["stability"] = {
            "shoulder_spread_in_spine_frame_texels": {"01": v[0], "02": v[1]},
            "shoulder_jerk_in_spine_frame_texels": {"01": v[2], "02": v[3]},
            "sword_angle_jerk_degrees": {"01": v[4], "02": v[5]},
            "meaning": "spread = how far the socket travels (02 follows the arm swing on purpose); jerk = mean second difference around the loop (jitter)",
        }
    iou = re.search(r"IoU\): (\[.*\])", tm)
    metrics["rule_partition_vs_n4c_iou"] = iou.group(1) if iou else "not recorded"
    cl = re.search(r"rest leg\): (\[.*\])", tm)
    metrics["articulated_clamped_ankles"] = cl.group(1) if cl else "not recorded"
    parts = [l for l in (CAP / "run.log").read_text().splitlines() if "partition" in l or "RESULT" in l]
    metrics["bevy_lab"] = parts
    tools_evidence(metrics)
    (OUT / "metrics.json").write_text(json.dumps(metrics, indent=2) + "\n")
    print(json.dumps({k: metrics[k] for k in ("stability", "original_vs_hybrid_bare_differing_screen_pixels") if k in metrics}, indent=1))


if __name__ == "__main__":
    main()
