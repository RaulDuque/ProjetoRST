"""Build the prototype 01 evidence from the Bevy captures (offline tool).

Input: the raw window captures of `cargo run --example
modular_character_engine` in `target/modular_character_engine/` (1280x720,
three panels at 3 screen pixels per texel). Output:
`evidence/modular_character_engine/01_prototipo/` (crops, sheets, GIFs and
`metrics.json`). Nothing here draws the character: every image is a crop of
what Bevy rendered (captions and frames are added around it).

    ~/.local/share/dc-art-tools/asf-venv/bin/python -B \
        art-src/modular-character-engine/scripts/make_evidence.py
"""

from __future__ import annotations

import json
import re
from pathlib import Path

from PIL import Image, ImageChops, ImageDraw

ROOT = Path(__file__).resolve().parents[3]
CAPTURES = ROOT / "target/modular_character_engine"
OUT = ROOT / "evidence/modular_character_engine/01_prototipo"
RON = ROOT / "art-src/modular-character-engine/stage/rig_lab.ron"
FACTORY = ROOT / "art-src/n4-sprite-machine/stage/factory/isolated"
STAGE = ROOT / "art-src/modular-character-engine/stage"

# Panel centres on screen (1280 wide window, panels at -420, 0, 420).
PANELS = {"original": 220, "hybrid": 640, "articulated": 1060}
HALF_W = 190
TOP, BOTTOM = 190, 640
RUN_MS = 67  # 1/15 s
BG = (48, 43, 56)
INK = (240, 205, 120)


def load(name: str) -> Image.Image:
    path = CAPTURES / f"{name}.png"
    if not path.exists():
        raise SystemExit(f"missing capture {path} (run the example first)")
    return Image.open(path).convert("RGB")


def crop(image: Image.Image, panel: str) -> Image.Image:
    x = PANELS[panel]
    return image.crop((x - HALF_W, TOP, x + HALF_W, BOTTOM))


def caption(image: Image.Image, text: str) -> Image.Image:
    out = Image.new("RGB", (image.width, image.height + 22), BG)
    out.paste(image, (0, 22))
    ImageDraw.Draw(out).text((6, 5), text, fill=INK)
    return out


def row(images: list[Image.Image], gap: int = 6) -> Image.Image:
    w = sum(i.width for i in images) + gap * (len(images) - 1)
    h = max(i.height for i in images)
    out = Image.new("RGB", (w, h), BG)
    x = 0
    for i in images:
        out.paste(i, (x, 0))
        x += i.width + gap
    return out


def column(images: list[Image.Image], gap: int = 6) -> Image.Image:
    w = max(i.width for i in images)
    h = sum(i.height for i in images) + gap * (len(images) - 1)
    out = Image.new("RGB", (w, h), BG)
    y = 0
    for i in images:
        out.paste(i, (0, y))
        y += i.height + gap
    return out


def gif(frames: list[Image.Image], name: str, durations: list[int] | int) -> None:
    frames[0].save(
        OUT / name,
        save_all=True,
        append_images=frames[1:],
        duration=durations,
        loop=0,
        disposal=1,
    )


def worn_label(log: dict[str, str], name: str) -> str:
    line = log.get(name, "")
    items = re.findall(r'"([a-z_]+)"', line)
    flip = "left" if "flip=true" in line else "right"
    frame = re.search(r"frame=(\d+)", line)
    f = frame.group(1) if frame else "?"
    return f"frame {f}  facing {flip}  worn: {', '.join(items) or 'nothing'}"


def read_log() -> dict[str, str]:
    path = CAPTURES / "run.log"
    out: dict[str, str] = {}
    if path.exists():
        for line in path.read_text().splitlines():
            m = re.match(r"\[modular_character_engine\] (\S+) clip=", line)
            if m:
                out[m.group(1)] = line
    return out


def piece_sheet() -> Image.Image:
    """The independent pieces as authored (4x) with their pivots."""
    text = RON.read_text()
    tiles = []
    for m in re.finditer(
        r'"(\w+)": \(\s*slot: (\w+), texture: "(\w+)://([\w/]+\.png)",\s*size: \((\d+), (\d+)\), pivot: \(([\d.]+), ([\d.]+)\),\s*bone: (\w+)',
        text,
    ):
        ident, slot, source, rel, _, _, px, py, bone = m.groups()
        base = FACTORY if source == "factory" else STAGE
        img = Image.open(base / rel).convert("RGBA")
        k = 4
        big = img.resize((img.width * k, img.height * k), Image.NEAREST)
        tile = Image.new("RGB", (max(big.width, 120) + 20, big.height + 60), BG)
        tile.paste(big, (10, 40), big)
        d = ImageDraw.Draw(tile)
        cx, cy = 10 + float(px) * k, 40 + float(py) * k
        d.line((cx - 8, cy, cx + 8, cy), fill=(0, 255, 255))
        d.line((cx, cy - 8, cx, cy + 8), fill=(0, 255, 255))
        d.text((4, 4), ident, fill=INK)
        d.text((4, 18), f"{slot} -> {bone}", fill=(200, 200, 200))
        tiles.append(tile)
    return row(tiles, 10)


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    log = read_log()
    metrics: dict[str, object] = {}

    # 1. Original vs hybrid without equipment: must be identical.
    diffs = {}
    for name in [f"bare_idle_f{f}" for f in range(2)] + [f"bare_run_f{f}" for f in range(6)]:
        cap = load(name)
        a, b = crop(cap, "original"), crop(cap, "hybrid")
        bbox = ImageChops.difference(a, b).getbbox()
        diff = 0
        if bbox is not None:
            d = ImageChops.difference(a, b).convert("L").point(lambda v: 255 if v else 0)
            diff = d.histogram()[255]
        diffs[name] = diff
    metrics["original_vs_hybrid_bare_differing_screen_pixels"] = diffs
    metrics["note"] = (
        "bare_idle_f1 is the breath frame: the hybrid lowers the upper body one texel "
        "while the single approved idle has no such frame, so it differs on purpose"
    )

    # 2. Skeleton overlay (still) and in motion.
    stills = [caption(crop(load("skeleton_idle_f0"), "original"), "idle (approved) + skeleton")]
    stills += [
        caption(crop(load(f"skeleton_run_f{f}"), "original"), f"run #{f} (approved) + skeleton")
        for f in range(6)
    ]
    column([row(stills[:4]), row(stills[4:])]).save(OUT / "01_skeleton_overlay.png")
    frames = [
        caption(
            row([crop(load(f"skeleton_run_f{f}"), "original"), crop(load(f"skeleton_run_f{f}"), "articulated")]),
            f"skeleton on the approved run #{f}  |  same skeleton driving the idle pieces",
        )
        for f in range(6)
    ]
    gif(frames, "02_skeleton_motion.gif", RUN_MS)

    # 3. Running with equipment (hybrid), and the idle breath.
    frames = [
        caption(crop(load(f"equipped_run_f{f}"), "hybrid"), f"hybrid run #{f}: sword + pauldron")
        for f in range(6)
    ]
    gif(frames, "03_run_equipped.gif", RUN_MS)
    row(frames).save(OUT / "03_run_equipped_frames.png")
    idle = [
        caption(crop(load(f"equipped_idle_f{f}"), "hybrid"), f"idle frame {f} (breath {f} texel)")
        for f in range(2)
    ]
    gif(idle, "03b_idle_equipped.gif", [700, 500])

    # 4. Swapping during the run (clock never resets).
    frames = [
        caption(crop(load(f"swap_s{s:02d}"), "hybrid"), worn_label(log, f"swap_s{s:02d}"))
        for s in range(24)
    ]
    gif(frames, "04_swap_during_run.gif", RUN_MS * 2)
    row(frames[2:8]).save(OUT / "04_swap_during_run_frames.png")

    # 5. Turning around mid-run.
    frames = [
        caption(crop(load(f"turn_t{s:02d}"), "hybrid"), worn_label(log, f"turn_t{s:02d}"))
        for s in range(18)
    ]
    gif(frames, "05_turn_around.gif", RUN_MS * 2)
    row([frames[4], frames[5], frames[11], frames[12]]).save(OUT / "05_turn_around_frames.png")
    caption(crop(load("mirror_run_f2"), "hybrid"), "mirrored run #2, variant B items + skeleton").save(
        OUT / "05b_mirror_still.png"
    )

    # 6. Comparison: original | hybrid | articulated.
    frames = []
    for f in range(6):
        cap = load(f"equipped_run_f{f}")
        bare = load(f"bare_run_f{f}")
        frames.append(
            caption(
                row([crop(bare, "original"), crop(cap, "hybrid"), crop(cap, "articulated")]),
                f"run #{f}   original (approved)   |   hybrid (approved + items)   |   articulated (idle pieces + FK/IK + items)",
            )
        )
    gif(frames, "06_comparison_original_hybrid_articulated.gif", RUN_MS * 2)
    column(frames[:3]).save(OUT / "06_comparison_frames.png")

    # 7. Independent pieces, sockets, depth.
    piece_sheet().save(OUT / "07_pieces_authored.png")
    socket_shots = [
        caption(crop(load("sockets_idle_f0"), "hybrid"), "idle: sockets (cyan) + item up axis"),
        caption(crop(load("sockets_run_f1"), "hybrid"), "run #1: same pieces, other pose"),
        caption(crop(load("sockets_run_f4"), "hybrid"), "run #4"),
        caption(row([crop(load("exploded_idle_f0"), "hybrid"), crop(load("exploded_idle_f0"), "articulated")]),
                "exploded: items alone with pivots | articulated pieces"),
    ]
    column([row(socket_shots[:3]), socket_shots[3]]).save(OUT / "07_pieces_and_sockets.png")
    depth = [
        caption(crop(load(f"depth_run_f{f}"), "hybrid"), f"depth layers #{f}")
        for f in range(6)
    ]
    row(depth).save(OUT / "08_depth_layers.png")
    gif(depth, "08_depth_layers.gif", RUN_MS * 2)

    partitions = {}
    if (CAPTURES / "run.log").exists():
        for line in (CAPTURES / "run.log").read_text().splitlines():
            m = re.search(r"partition (\w+) #(\d+): (\d+) texels differ", line)
            if m:
                partitions[f"{m.group(1)}#{m.group(2)}"] = int(m.group(3))
        result = [line for line in (CAPTURES / "run.log").read_text().splitlines() if "RESULT" in line]
        metrics["bevy_tour_result"] = result[-1] if result else "not recorded"
    metrics["partition_recomposition_differing_texels"] = partitions
    (OUT / "metrics.json").write_text(json.dumps(metrics, indent=2) + "\n")
    print(json.dumps(metrics, indent=2))


if __name__ == "__main__":
    main()
