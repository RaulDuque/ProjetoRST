"""N3.6: reduce the keyed video frames into the run proof and its evidence.

Inputs: `frames/video_NNN.png` (from `extract_video.py`), the N3.4 palette
and the N3.5 candidate (for the identity comparison).

Method (the N3.5 reduction, unchanged): box filter with premultiplied alpha
at ONE scale for every frame, alpha >= 50 % opaque, nearest colour of the
N3.4 palette without the hand-only accents. The scale is fixed by frame 0
(standing): its figure becomes 106 px tall, like the N3.5 base.

Alignment: the video runs in place with a fixed camera, so every frame
keeps the video's own coordinates. One global offset puts frame 0's hip on
the pivot column and the lowest sole of the cycle on the pivot row; no
per-frame shift is applied, so the vertical bob is the video's.

Outputs:
- `source/video_NNN.txt`: one 112x128 text grid per frame (N3.4 keys).
- `generated/run6.png`, `generated/run10.png`, `generated/stand.png`,
  `generated/run.ron`, `generated/boxes.ron`, `generated/metrics.json`.
- `docs/development/n3_6/evidence/run_*.png` and `run_*.gif`.

    python3 art-src/n3-6-animation-proof/scripts/build_run.py
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "art-src/n3-4-original-protagonist/scripts"))
sys.path.insert(0, str(ROOT / "art-src/player/scripts/post"))
from n34_palette import PALETTE, TRANSPARENT  # noqa: E402
from resample import resize  # noqa: E402

HERE = ROOT / "art-src/n3-6-animation-proof"
FRAMES = HERE / "frames"
OUT = HERE / "generated"
SOURCE = HERE / "source"
EVIDENCE = ROOT / "docs/development/n3_6/evidence"
N35 = ROOT / "art-src/n3-5-fidelity/generated/candidate_106.png"
SCENE = ROOT / "target/world_visual_showcase/after_square.png"
HAND_ONLY = {"M", "e"}
CANVAS = (112, 128)
PIVOT = (56, 118)
BODY_TEXELS = 106
DENSITY = 2
VIDEO_FPS = 24
STAND = 0
CYCLE = list(range(48, 60))
# Frames 49 and 55 carry interpolation ghosts (doubled legs) in the video.
GHOSTED = {49, 55}
RUN6 = [48, 50, 52, 54, 56, 58]
RUN10 = [f for f in CYCLE if f not in GHOSTED]
RUN6_FPS = 12.0
NEUTRAL = (150, 150, 156, 255)
LIGHT = (226, 224, 228, 255)
DARK = (18, 14, 22, 255)
INK = (40, 32, 40)
NEAREST = Image.Resampling.NEAREST
WINDOW_HEIGHT = 720


def distance(a: tuple[int, ...], b: tuple[int, ...]) -> int:
    return sum(abs(x - y) for x, y in zip(a[:3], b[:3], strict=True))


def zoom(image: Image.Image, factor: int) -> Image.Image:
    return resize(image, (image.width * factor, image.height * factor), NEAREST)


def on(image: Image.Image, background: tuple[int, int, int, int]) -> Image.Image:
    plate = Image.new("RGBA", image.size, background)
    plate.alpha_composite(image.convert("RGBA"))
    return plate


def sheet(panels: list[tuple[str, Image.Image]], background=LIGHT) -> Image.Image:
    gap, label = 12, 28
    width = sum(p.width for _, p in panels) + gap * (len(panels) + 1)
    height = max(p.height for _, p in panels) + label + gap * 2
    out = Image.new("RGBA", (width, height), background)
    draw = ImageDraw.Draw(out)
    x = gap
    for title, panel in panels:
        for line, text in enumerate(title.split("\n")):
            draw.text((x, 3 + 11 * line), text, fill=INK)
        out.alpha_composite(panel.convert("RGBA"), (x, gap + label))
        x += panel.width + gap
    return out


def stack(rows: list[Image.Image], background=LIGHT) -> Image.Image:
    out = Image.new("RGBA", (max(r.width for r in rows), sum(r.height for r in rows)), background)
    y = 0
    for row in rows:
        out.alpha_composite(row, (0, y))
        y += row.height
    return out


def reduce(frame: Image.Image, scale: float) -> list[str]:
    """Box-filter at `scale`, then snap to the N3.4 palette (no accents)."""
    size = (round(frame.width * scale), round(frame.height * scale))
    small = frame.resize(size, Image.Resampling.BOX)
    colours = {k: v for k, v in PALETTE.items() if k not in HAND_ONLY}
    rows = []
    for y in range(small.height):
        row = []
        for x in range(small.width):
            r, g, b, a = small.getpixel((x, y))
            if a < 128:
                row.append(TRANSPARENT)
            else:
                row.append(min(colours, key=lambda k: distance(colours[k], (r, g, b))))
        rows.append("".join(row))
    return rows


def place(rows: list[str], offset: tuple[int, int]) -> list[str]:
    grid = [[TRANSPARENT] * CANVAS[0] for _ in range(CANVAS[1])]
    for y, row in enumerate(rows):
        for x, key in enumerate(row):
            if key == TRANSPARENT:
                continue
            cx, cy = x + offset[0], y + offset[1]
            if not (0 <= cx < CANVAS[0] and 0 <= cy < CANVAS[1]):
                raise SystemExit(f"pixel {x},{y} falls outside the {CANVAS} canvas")
            grid[cy][cx] = key
    return ["".join(r) for r in grid]


def render(rows: list[str]) -> Image.Image:
    image = Image.new("RGBA", CANVAS, (0, 0, 0, 0))
    for y, row in enumerate(rows):
        for x, key in enumerate(row):
            if key != TRANSPARENT:
                image.putpixel((x, y), (*PALETTE[key], 255))
    return image


def opaque_box(rows: list[str]) -> tuple[int, int, int, int]:
    xs = [x for row in rows for x, k in enumerate(row) if k != TRANSPARENT]
    ys = [y for y, row in enumerate(rows) if any(k != TRANSPARENT for k in row)]
    return min(xs), min(ys), max(xs), max(ys)


def atlas(images: list[Image.Image]) -> Image.Image:
    out = Image.new("RGBA", (CANVAS[0] * len(images), CANVAS[1]), (0, 0, 0, 0))
    for i, image in enumerate(images):
        out.alpha_composite(image, (i * CANVAS[0], 0))
    return out


def run10_durations() -> list[float]:
    """Real video timing: each kept frame lasts until the next kept one."""
    out = []
    for i, frame in enumerate(RUN10):
        nxt = RUN10[i + 1] if i + 1 < len(RUN10) else CYCLE[0] + len(CYCLE)
        out.append((nxt - frame) / VIDEO_FPS)
    return out


def manifest_text(durations10: list[float]) -> str:
    def clip(name: str, count: int, fps: float, durations: list[float]) -> str:
        values = ", ".join(f"{d:.5f}" for d in durations)
        return (f'        "{name}": (atlas: "{name}", first: 0, count: {count}, mode: Loop(fps: {fps:.1f}),\n'
                f"            pivot: ({PIVOT[0]:.1f}, {PIVOT[1]:.1f}), fallback: None,\n"
                f"            durations: [{values}], speed_reference: None),\n")

    def atlas_line(name: str, columns: int) -> str:
        return (f'        "{name}": (texture: "{name}.png", cell: ({CANVAS[0]}, {CANVAS[1]}), '
                f"columns: {columns}, rows: 1, padding: 0, density: {DENSITY}),\n")

    return ("(\n    version: 1,\n    facing: Right,\n    weapon_in_body: false,\n    atlases: {\n"
            + atlas_line("stand", 1) + atlas_line("run6", len(RUN6)) + atlas_line("run10", len(RUN10))
            + "    },\n    clips: {\n"
            + clip("idle", 1, 1.0, [1.0]).replace('atlas: "idle"', 'atlas: "stand"')
            + clip("run6", len(RUN6), RUN6_FPS, [1 / RUN6_FPS] * len(RUN6))
            + clip("run10", len(RUN10), float(VIDEO_FPS), durations10)
            + "    },\n)\n")


def gif(frames: list[Image.Image], durations: list[float], path: Path, background) -> None:
    shown = [on(zoom(f, 3), background).convert("P", palette=Image.Palette.ADAPTIVE) for f in frames]
    shown[0].save(path, save_all=True, append_images=shown[1:], loop=0,
                  duration=[round(d * 1000) for d in durations], disposal=2)


def rim_test(image: Image.Image) -> Image.Image:
    """Evidence only: a 1 px lighter rim on dark pixels facing the back (left)."""
    out = image.copy()
    for y in range(image.height):
        for x in range(image.width):
            r, g, b, a = image.getpixel((x, y))
            if not a or (r + g + b) / 3 > 80:
                continue
            behind = image.getpixel((x - 1, y))[3] if x > 0 else 0
            if not behind:
                out.putpixel((x, y), (min(255, r + 38), min(255, g + 34), min(255, b + 44), 255))
    return out


def planted_speed(grids: dict[int, list[str]]) -> float:
    """World units per second at which the planted sole slides back in the cycle."""
    shifts = []
    for a, b in zip(CYCLE, CYCLE[1:], strict=False):
        rows_a, rows_b = grids[a], grids[b]
        sole_a = [x for x, k in enumerate(rows_a[PIVOT[1]]) if k != TRANSPARENT]
        sole_b = [x for x, k in enumerate(rows_b[PIVOT[1]]) if k != TRANSPARENT]
        if sole_a and sole_b and abs(sole_a[0] - sole_b[0]) <= 8:
            shifts.append(sole_a[0] - sole_b[0])
    per_frame = sum(shifts) / len(shifts) if shifts else 0.0
    return per_frame * VIDEO_FPS / DENSITY


def main() -> None:
    for folder in (OUT, SOURCE, EVIDENCE):
        folder.mkdir(parents=True, exist_ok=True)
    stand_full = Image.open(FRAMES / f"video_{STAND:03d}.png").convert("RGBA")
    box = stand_full.getchannel("A").getbbox()
    if box is None:
        raise SystemExit("frame 0 has no figure")
    scale = BODY_TEXELS / (box[3] - box[1])
    # Hip column of frame 0: centre of the figure at 45 % of its height.
    hip_row = box[1] + round(0.45 * (box[3] - box[1]))
    hip_xs = [x for x in range(stand_full.width) if stand_full.getpixel((x, hip_row))[3]]
    hip_x = (hip_xs[0] + hip_xs[-1]) / 2 * scale

    reduced = {f: reduce(Image.open(FRAMES / f"video_{f:03d}.png").convert("RGBA"), scale)
               for f in [STAND, *CYCLE]}
    lowest = max(max(y for y, row in enumerate(reduced[f]) if row.strip(TRANSPARENT))
                 for f in [STAND, *RUN10])
    offset = (PIVOT[0] - round(hip_x), PIVOT[1] - lowest)
    grids = {f: place(rows, offset) for f, rows in reduced.items()}
    for f, rows in grids.items():
        (SOURCE / f"video_{f:03d}.txt").write_text("\n".join(rows) + "\n", encoding="utf-8")
    images = {f: render(rows) for f, rows in grids.items()}

    atlas([images[STAND]]).save(OUT / "stand.png")
    atlas([images[f] for f in RUN6]).save(OUT / "run6.png")
    atlas([images[f] for f in RUN10]).save(OUT / "run10.png")
    durations10 = run10_durations()
    (OUT / "run.ron").write_text(manifest_text(durations10), encoding="utf-8")
    boxes = {f: opaque_box(grids[f]) for f in [STAND, *CYCLE]}
    metrics = {
        "canvas": list(CANVAS),
        "pivot": list(PIVOT),
        "density": DENSITY,
        "video_scale": round(scale, 5),
        "stand_figure_video_px": box[3] - box[1],
        "offset": list(offset),
        "frames": {str(f): {"box": list(boxes[f]), "height": boxes[f][3] - boxes[f][1] + 1,
                            "width": boxes[f][2] - boxes[f][0] + 1, "ghosted": f in GHOSTED}
                   for f in [STAND, *CYCLE]},
        "run6": RUN6,
        "run6_fps": RUN6_FPS,
        "run10": RUN10,
        "run10_durations": [round(d, 5) for d in durations10],
        "cycle_seconds": len(CYCLE) / VIDEO_FPS,
        "planted_sole_speed_units_per_s": round(planted_speed(grids), 2),
        "hand_edits": 0,
        "gui_paint": False,
        "ai_generation_by_agent": False,
    }
    (OUT / "metrics.json").write_text(json.dumps(metrics, indent=1) + "\n", encoding="utf-8")
    # Opaque box of every clip frame, for the overlay of the isolated proof.
    clips = {"idle": [STAND], "run6": RUN6, "run10": RUN10}
    (OUT / "boxes.ron").write_text(
        "{\n" + "".join(
            f'    "{name}": [{", ".join(str(tuple(boxes[f])) for f in frames)}],\n'
            for name, frames in clips.items()) + "}\n",
        encoding="utf-8",
    )

    # --- evidence -------------------------------------------------------
    n35 = Image.open(N35).convert("RGBA")
    run6 = [images[f] for f in RUN6]
    run10 = [images[f] for f in RUN10]

    # Selected frames, with ids and durations; ghosted frames shown and marked.
    sheet([(f"video {f}" + (" FANTASMA" if f in GHOSTED else "") +
            ("\nrun6" if f in RUN6 else "") + ("  run10" if f in RUN10 else ""),
            on(zoom(images[f], 2), NEUTRAL)) for f in CYCLE]).save(EVIDENCE / "run_selected_frames.png")

    # Alignment: onion skin of the 10 kept frames, per-frame boxes, pivot cross.
    onion = Image.new("RGBA", CANVAS, (0, 0, 0, 0))
    for image in run10:
        faded = image.copy()
        faded.putalpha(image.getchannel("A").point(lambda a: 70 if a else 0))
        onion.alpha_composite(faded)
    big = on(zoom(onion, 4), NEUTRAL)
    draw = ImageDraw.Draw(big)
    for f in RUN10:
        x0, y0, x1, y1 = boxes[f]
        draw.rectangle([x0 * 4, y0 * 4, x1 * 4 + 3, y1 * 4 + 3], outline=(60, 120, 220))
    px, py = PIVOT[0] * 4, PIVOT[1] * 4
    draw.line([(px - 16, py), (px + 16, py)], fill=(220, 30, 40), width=2)
    draw.line([(px, py - 16), (px, py + 16)], fill=(220, 30, 40), width=2)
    draw.line([(0, (PIVOT[1] + 1) * 4), (big.width, (PIVOT[1] + 1) * 4)], fill=(220, 30, 40))
    sheet([("10 quadros sobrepostos\ncaixas por quadro (azul), pivo (vermelho)", big)]).save(
        EVIDENCE / "run_alignment.png")

    # Cycle strip at 4x, closing with the first frame again.
    sheet([(f"{f}" + (" -> volta ao 48" if i == len(RUN6) - 1 else ""), on(zoom(images[f], 3), NEUTRAL))
           for i, f in enumerate(RUN6)] + [("48 (repete)", on(zoom(images[48], 3), NEUTRAL))]).save(
        EVIDENCE / "run_cycle.png")

    # Anatomy guides: head top, hip row of frame 0 and pivot row on every frame.
    guide_rows = [boxes[STAND][1], offset[1] + round(0.45 * BODY_TEXELS), PIVOT[1]]
    panels = []
    for f in [STAND, *RUN6]:
        panel = on(zoom(images[f], 3), NEUTRAL)
        draw = ImageDraw.Draw(panel)
        for row in guide_rows:
            draw.line([(0, row * 3), (panel.width, row * 3)], fill=(170, 30, 50))
        panels.append((f"{'parado 0' if f == STAND else f}\nalt. {boxes[f][3] - boxes[f][1] + 1}", panel))
    n35_panel = Image.new("RGBA", (CANVAS[0], CANVAS[1]), (0, 0, 0, 0))
    n35_panel.alpha_composite(n35, ((CANVAS[0] - n35.width) // 2, 0))
    sheet([("N3.5 candidato", on(zoom(n35_panel, 3), NEUTRAL))] + panels).save(
        EVIDENCE / "run_anatomy_comparison.png")

    # Hair: head and back crop of every kept frame, 6x.
    hair_box = (8, 8, 80, 60)
    sheet([(f"{f}", on(zoom(images[f].crop(hair_box), 4), DARK)) for f in RUN6]).save(
        EVIDENCE / "run_hair_comparison.png")

    # Backgrounds and rim-light test (evidence only, not in the clip).
    frame = images[52]
    scene_panel = None
    if SCENE.is_file():
        scene = Image.open(SCENE).convert("RGBA").crop((300, 380, 520, 652))
        sprite = resize(frame, (round(frame.width * 0.889), round(frame.height * 0.889)), NEAREST)
        scene.alpha_composite(sprite, (60, scene.height - 2 - round(PIVOT[1] * 0.889)))
        scene_panel = zoom(scene, 2)
    rows = [sheet([("claro", on(zoom(frame, 3), LIGHT)), ("cinza", on(zoom(frame, 3), NEUTRAL)),
                   ("quase preto", on(zoom(frame, 3), DARK)),
                   ("quase preto + borda de luz\n(teste, nao aplicado)", on(zoom(rim_test(frame), 3), DARK))])]
    if scene_panel is not None:
        rows.append(sheet([("cena real do jogo (after_square.png), camera 405, 2x", scene_panel)]))
    stack(rows).save(EVIDENCE / "run_dark_background.png")

    # Animated GIFs: the real sequences, 3x, on neutral grey.
    for fps in (8, 10, 12):
        gif(run6, [1 / fps] * len(run6), EVIDENCE / f"run6_{fps}fps.gif", NEUTRAL)
    gif(run10, durations10, EVIDENCE / "run10_video_timing.gif", NEUTRAL)
    gif(run6, [1 / RUN6_FPS] * len(run6), EVIDENCE / "run6_12fps_dark.gif", DARK)

    # Camera sheets from the Bevy captures, when present.
    for camera in (405, 360):
        capture = EVIDENCE / f"bevy_n36_{camera}.png"
        if capture.is_file():
            shot = Image.open(capture).convert("RGBA").crop((570, 230, 710, 370))
            sheet([(f"captura Bevy camera {camera}, 1x", shot), ("4x (pixels da captura)", zoom(shot, 4))],
                  ).save(EVIDENCE / f"run_{camera}.png")

    # Frame-by-frame Bevy captures (integer 360 view), cut into one sheet.
    shots = [ROOT / f"target/n3_6_bevy_frames/run6_{i}.png" for i in range(len(RUN6))]
    if all(path.is_file() for path in shots):
        panels = []
        for i, path in enumerate(shots):
            crop = Image.open(path).convert("RGBA").crop((570, 230, 710, 370))
            panels.append((f"Bevy quadro {i} (video {RUN6[i]})", zoom(crop, 2)))
        sheet(panels).save(EVIDENCE / "run_bevy_frames.png")

    print(json.dumps({k: metrics[k] for k in ("video_scale", "offset", "run10_durations",
                                              "planted_sole_speed_units_per_s")}, indent=1))
    for f in [STAND, *CYCLE]:
        print(f, boxes[f])


if __name__ == "__main__":
    main()
