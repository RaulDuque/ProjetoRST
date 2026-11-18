"""N3.7: dark-background contrast study of the approved run6.

The approved atlas is only read. Every variant is a NEW file:

- A original: the N3.6 run6, untouched.
- B rim: on the lower body (from the hip row down), a dark texel whose back
  neighbour (left, the body faces right) is transparent becomes the
  palette's cool grey `q`. One texel wide, only on exposed back edges.
- C boots: from the boot row down, one palette step lighter
  (K -> d, d -> l, o -> p). The rest of the body is untouched.
- D scene: the sprite untouched; the near-black background gets a soft
  cool back-light behind the character (a scene change, not a sprite one).

Backgrounds: light, grey, near black and a crop of the real castle scene
(`target/world_visual_showcase/after_square.png`, a capture of the game).
A simple edge metric is recorded: the mean luminance difference between the
lower-body silhouette texels and the background right behind them.

Writes `generated/run6_rim.png`, `generated/run6_boots.png`, `generated/backlight.png`,
`analysis/contrast_metrics.json` and the evidence `dark_original.png`,
`dark_rim_light.png`, `dark_comparison.png`, `dark_comparison.gif`.

    python3 -B art-src/n3-7-run-sync/scripts/contrast_study.py
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "art-src/n3-4-original-protagonist/scripts"))
sys.path.insert(0, str(ROOT / "art-src/player/scripts/post"))
from n34_palette import PALETTE  # noqa: E402
from resample import resize  # noqa: E402

HERE = ROOT / "art-src/n3-7-run-sync"
RUN6 = ROOT / "art-src/n3-6-animation-proof/generated/run6.png"
SCENE = ROOT / "target/world_visual_showcase/after_square.png"
EVIDENCE = ROOT / "docs/development/n3_7/evidence"
CANVAS = (112, 128)
COUNT = 6
HIP_ROW = 62
BOOT_ROW = 92
RIM_KEY = "q"
BOOT_STEP = {"K": "d", "d": "l", "o": "p"}
DARK_LIMIT = 60
LIGHT = (226, 224, 228, 255)
GREY = (150, 150, 156, 255)
NEAR_BLACK = (18, 14, 22, 255)
INK = (40, 32, 40)
NEAREST = Image.Resampling.NEAREST
BY_COLOUR = {v: k for k, v in PALETTE.items()}


def zoom(image: Image.Image, factor: int) -> Image.Image:
    return resize(image, (image.width * factor, image.height * factor), NEAREST)


def luminance(rgb) -> float:
    return 0.299 * rgb[0] + 0.587 * rgb[1] + 0.114 * rgb[2]


def rim(image: Image.Image) -> tuple[Image.Image, int]:
    out = image.copy()
    changed = 0
    for y in range(HIP_ROW, image.height):
        for x in range(image.width):
            r, g, b, a = image.getpixel((x, y))
            if not a or luminance((r, g, b)) > DARK_LIMIT:
                continue
            if x == 0 or not image.getpixel((x - 1, y))[3]:
                out.putpixel((x, y), (*PALETTE[RIM_KEY], 255))
                changed += 1
    return out, changed


def boots(image: Image.Image) -> tuple[Image.Image, int]:
    out = image.copy()
    changed = 0
    for y in range(BOOT_ROW, image.height):
        for x in range(image.width):
            r, g, b, a = image.getpixel((x, y))
            k = BY_COLOUR.get((r, g, b)) if a else None
            if k in BOOT_STEP:
                out.putpixel((x, y), (*PALETTE[BOOT_STEP[k]], 255))
                changed += 1
    return out, changed


def backlit(size: tuple[int, int]) -> Image.Image:
    """Near black with a soft cool light behind the character (variant D)."""
    out = Image.new("RGBA", size, NEAR_BLACK)
    cx, cy = size[0] / 2, size[1] * 0.68
    for y in range(size[1]):
        for x in range(size[0]):
            d = ((x - cx) ** 2 / (size[0] * 0.55) ** 2 + (y - cy) ** 2 / (size[1] * 0.6) ** 2) ** 0.5
            k = max(0.0, 1.0 - d)
            out.putpixel((x, y), (int(18 + 56 * k), int(14 + 58 * k), int(22 + 74 * k), 255))
    return out


def scene_plate() -> Image.Image | None:
    if not SCENE.is_file():
        return None
    return Image.open(SCENE).convert("RGBA").crop((330, 380, 330 + CANVAS[0], 380 + CANVAS[1]))


def plate(frame: Image.Image, background) -> Image.Image:
    base = background.copy() if isinstance(background, Image.Image) else Image.new("RGBA", frame.size, background)
    base.alpha_composite(frame)
    return base


def edge_contrast(frame: Image.Image, background) -> float:
    """Mean |luminance(texel) - luminance(background)| over lower-body silhouette texels."""
    back = background if isinstance(background, Image.Image) else Image.new("RGBA", frame.size, background)
    values = []
    for y in range(HIP_ROW, frame.height):
        for x in range(frame.width):
            a = frame.getpixel((x, y))[3]
            if not a:
                continue
            edge = any(not (0 <= x + dx < frame.width and 0 <= y + dy < frame.height)
                       or not frame.getpixel((x + dx, y + dy))[3]
                       for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)))
            if edge:
                values.append(abs(luminance(frame.getpixel((x, y))) - luminance(back.getpixel((x, y)))))
    return round(sum(values) / len(values), 1)


def main() -> None:
    EVIDENCE.mkdir(parents=True, exist_ok=True)
    atlas = Image.open(RUN6).convert("RGBA")
    frames = [atlas.crop((i * CANVAS[0], 0, (i + 1) * CANVAS[0], CANVAS[1])) for i in range(COUNT)]
    rims, boot_frames = [], []
    rim_changed = boot_changed = 0
    for frame in frames:
        r, n = rim(frame)
        rims.append(r)
        rim_changed += n
        b, m = boots(frame)
        boot_frames.append(b)
        boot_changed += m
    for name, items in (("run6_rim", rims), ("run6_boots", boot_frames)):
        out = Image.new("RGBA", atlas.size, (0, 0, 0, 0))
        for i, image in enumerate(items):
            out.alpha_composite(image, (i * CANVAS[0], 0))
        out.save(HERE / "generated" / f"{name}.png")

    scene = scene_plate()
    lit = backlit(CANVAS)
    # Variant D for the isolated Bevy example: the light plate alone.
    lit.save(HERE / "generated" / "backlight.png")
    backgrounds = [("claro", LIGHT), ("cinza", GREY), ("quase preto", NEAR_BLACK)]
    if scene is not None:
        backgrounds.append(("castelo (captura do jogo)", scene))
    variants = [("A original", frames, None), ("B borda de luz", rims, None),
                ("C botas +1 tom", boot_frames, None), ("D cena com contraluz", frames, lit)]
    shown = 2  # run6 frame 2 (air, legs spread)

    def panel(items, override, background) -> Image.Image:
        back = override if override is not None and background == NEAR_BLACK else background
        return zoom(plate(items[shown], back), 3)

    def grid(rows, title) -> Image.Image:
        gap, label = 10, 40
        pw, ph = CANVAS[0] * 3, CANVAS[1] * 3
        cols = len(backgrounds)
        out = Image.new("RGBA", (gap + cols * (pw + gap) + 150, label + len(rows) * (ph + gap) + gap + 20),
                        (226, 224, 228, 255))
        draw = ImageDraw.Draw(out)
        draw.text((gap, 4), title, fill=INK)
        for c, (name, _) in enumerate(backgrounds):
            draw.text((150 + gap + c * (pw + gap), label - 12), name, fill=INK)
        for r, (name, items, override) in enumerate(rows):
            y = label + r * (ph + gap)
            draw.text((gap, y + ph // 2), name, fill=INK)
            for c, (_, background) in enumerate(backgrounds):
                out.alpha_composite(panel(items, override, background), (150 + gap + c * (pw + gap), y))
        return out

    grid([variants[0]], "A - arte aprovada N3.6, sem alteracao (quadro run6 #2)").save(
        EVIDENCE / "dark_original.png")
    grid([variants[1]], "B - borda de luz fria de 1 texel nas bordas escuras de tras, do quadril para baixo "
         f"(cor da paleta '{RIM_KEY}'), experimental").save(EVIDENCE / "dark_rim_light.png")
    grid(variants, "A original / B borda de luz / C botas um tom mais claras / D contraluz no cenario "
         "(sprite intacto; so muda no quase preto)").save(EVIDENCE / "dark_comparison.png")

    # Motion: the four variants side by side on near black, 12 fps.
    gif = []
    for i in range(COUNT):
        row = Image.new("RGBA", (4 * (CANVAS[0] * 3 + 8), CANVAS[1] * 3 + 20), (226, 224, 228, 255))
        draw = ImageDraw.Draw(row)
        for c, (name, items, override) in enumerate(variants):
            back = override if override is not None else NEAR_BLACK
            row.alpha_composite(zoom(plate(items[i], back), 3), (c * (CANVAS[0] * 3 + 8), 20))
            draw.text((c * (CANVAS[0] * 3 + 8) + 4, 4), name, fill=INK)
        gif.append(row.convert("P", palette=Image.Palette.ADAPTIVE))
    gif[0].save(EVIDENCE / "dark_comparison.gif", save_all=True, append_images=gif[1:], loop=0,
                duration=83, disposal=2)

    metrics = {"method": "mean |luminance(silhouette texel) - luminance(background)| on lower-body "
                         f"edge texels (rows >= {HIP_ROW}), mean of the 6 run6 frames",
               "rim_texels_changed": rim_changed, "boot_texels_changed": boot_changed,
               "hip_row": HIP_ROW, "boot_row": BOOT_ROW, "rim_key": RIM_KEY, "boot_step": BOOT_STEP,
               "edge_contrast": {}}
    for name, items, override in variants:
        metrics["edge_contrast"][name] = {}
        for bname, background in backgrounds:
            back = override if override is not None and background == NEAR_BLACK else background
            values = [edge_contrast(items[i], back) for i in range(COUNT)]
            metrics["edge_contrast"][name][bname] = round(sum(values) / len(values), 1)
    (HERE / "analysis/contrast_metrics.json").write_text(json.dumps(metrics, indent=1) + "\n",
                                                         encoding="utf-8")
    print(json.dumps(metrics, indent=1))


if __name__ == "__main__":
    main()
