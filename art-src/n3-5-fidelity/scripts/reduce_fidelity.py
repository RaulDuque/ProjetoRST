"""N3.5: reduce the owner's right-side view with the same method at every
candidate resolution, so the resolution hypotheses differ only in size.

Method (identical for each height), chosen because it is the method of the
`reducao` panel the owner picked (the N3.4 draft), minus its light halo:
1. Crop the right-side view of `Ficha_Modular_do_Protagonista_Gotico.png`
   (read only) and flood-fill the flat panel from the border.
2. Erode the figure mask by one source pixel, so the anti-aliased rim that
   mixes figure and panel does not become a light halo in the sprite.
3. Box-filter (premultiplied alpha) to the target height; alpha >= 50 % is
   opaque, the rest transparent.
4. Map each pixel to the nearest colour of the N3.4 palette, without the
   hand-only accents (metal `M`, eye `e`).

Two rejected palette variants of the 106 px reduction are also written for
the evidence: a 32-colour median cut and a 24-colour median cut plus the
N3.4 skin and hair ramps.

Writes `generated/reduction_<height>.png`, `generated/variant_*.png` and the
editable text grid `source/reduction_106.txt` (N3.4 palette keys).

    python3 art-src/n3-5-fidelity/scripts/reduce_fidelity.py
"""

from __future__ import annotations

import string
import sys
from dataclasses import dataclass
from pathlib import Path

from PIL import Image, ImageFilter

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "art-src/n3-4-original-protagonist/scripts"))
from n34_palette import PALETTE, TRANSPARENT  # noqa: E402

SHEET = ROOT / "Md-usario/assets-user/prota/base/Ficha_Modular_do_Protagonista_Gotico.png"
HERE = ROOT / "art-src/n3-5-fidelity"
VIEW_BOX = (680, 80, 775, 418)
SOLE_ROW = 337
BACKGROUND_TOLERANCE = 22
HAND_ONLY = {"M", "e"}
CANDIDATE = "106"
SKIN_AND_HAIR = "SszWwhH"


@dataclass(frozen=True)
class Target:
    """One resolution hypothesis: canvas, figure height and pivot."""

    name: str
    canvas: tuple[int, int]
    height: int
    pivot: tuple[int, int]


# A: N3.4 contract. B: larger source. C: the height that maps one texel to
# one screen pixel under the current 405 camera at the same world height
# (53 world units * 720/405 = 94.2 screen pixels).
TARGETS = [
    Target("106", (80, 128), 106, (40, 118)),
    Target("132", (96, 160), 132, (48, 150)),
    Target("94", (72, 112), 94, (36, 104)),
]


def distance(a: tuple[int, ...], b: tuple[int, ...]) -> int:
    return sum(abs(x - y) for x, y in zip(a[:3], b[:3], strict=True))


def figure_mask(view: Image.Image) -> Image.Image:
    px = view.load()
    width, height = view.size
    background = px[2, 2]
    mask = Image.new("L", view.size, 255)
    out = mask.load()
    stack = [(x, 0) for x in range(width)] + [(0, y) for y in range(height)]
    stack += [(width - 1, y) for y in range(height)]
    while stack:
        x, y = stack.pop()
        if not (0 <= x < width and 0 <= y < height) or out[x, y] == 0:
            continue
        if distance(px[x, y], background) > BACKGROUND_TOLERANCE:
            continue
        out[x, y] = 0
        stack += [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
    for y in range(SOLE_ROW + 1, height):
        for x in range(width):
            out[x, y] = 0
    return mask.filter(ImageFilter.MinFilter(3))


def reduced(height: int) -> tuple[Image.Image, Image.Image]:
    """Box-filtered figure (RGB) and its binary alpha at `height` px."""
    view = Image.open(SHEET).convert("RGB").crop(VIEW_BOX)
    mask = figure_mask(view)
    box = mask.getbbox()
    if box is None:
        raise SystemExit("no figure in the view box")
    figure = view.crop(box).convert("RGBA")
    figure.putalpha(mask.crop(box))
    width = max(1, round(figure.width * height / figure.height))
    small = figure.resize((width, height), Image.Resampling.BOX)
    alpha = small.getchannel("A").point(lambda a: 255 if a >= 128 else 0)
    return small.convert("RGB"), alpha


def snap(rgb: Image.Image, alpha: Image.Image, colours: dict[str, tuple[int, int, int]]) -> list[str]:
    rows = []
    for y in range(rgb.height):
        row = []
        for x in range(rgb.width):
            if not alpha.getpixel((x, y)):
                row.append(TRANSPARENT)
                continue
            colour = rgb.getpixel((x, y))
            row.append(min(colours, key=lambda key: distance(colours[key], colour)))
        rows.append("".join(row))
    return rows


def place(rows: list[str], target: Target) -> list[str]:
    """Put the figure on the canvas: last row on the sole row, feet on the pivot."""
    sole = [x for x, key in enumerate(rows[-1]) if key != TRANSPARENT]
    feet = (sole[0] + sole[-1]) // 2 if sole else len(rows[0]) // 2
    left = target.pivot[0] - feet
    top = target.pivot[1] - target.height + 1
    grid = [[TRANSPARENT] * target.canvas[0] for _ in range(target.canvas[1])]
    for y, row in enumerate(rows):
        for x, key in enumerate(row):
            if key != TRANSPARENT and 0 <= left + x < target.canvas[0]:
                grid[top + y][left + x] = key
    return ["".join(row) for row in grid]


def render(rows: list[str], colours: dict[str, tuple[int, int, int]]) -> Image.Image:
    image = Image.new("RGBA", (len(rows[0]), len(rows)), (0, 0, 0, 0))
    for y, row in enumerate(rows):
        for x, key in enumerate(row):
            if key != TRANSPARENT:
                image.putpixel((x, y), (*colours[key], 255))
    return image


def adaptive(rgb: Image.Image, count: int) -> dict[str, tuple[int, int, int]]:
    indexed = rgb.quantize(colors=count, method=Image.Quantize.MEDIANCUT, dither=Image.Dither.NONE)
    flat = indexed.getpalette() or []
    # One character per colour, never one of the N3.4 skin and hair keys.
    keys = [c for c in string.digits + string.ascii_letters if c not in SKIN_AND_HAIR]
    return {keys[i]: tuple(flat[3 * i : 3 * i + 3]) for i in range(count)}


def main() -> None:
    (HERE / "generated").mkdir(parents=True, exist_ok=True)
    (HERE / "source").mkdir(parents=True, exist_ok=True)
    colours = {key: value for key, value in PALETTE.items() if key not in HAND_ONLY}
    for target in TARGETS:
        rgb, alpha = reduced(target.height)
        grid = place(snap(rgb, alpha, colours), target)
        render(grid, PALETTE).save(HERE / f"generated/reduction_{target.name}.png")
        if target.name == CANDIDATE:
            (HERE / "source/reduction_106.txt").write_text("\n".join(grid) + "\n", encoding="utf-8")
            variants = {
                "adaptive32": adaptive(rgb, 32),
                "adaptive24_skin_hair": adaptive(rgb, 24)
                | {key: PALETTE[key] for key in SKIN_AND_HAIR},
            }
            for name, palette in variants.items():
                render(place(snap(rgb, alpha, palette), target), palette).save(
                    HERE / f"generated/variant_{name}.png"
                )
        print(f"{target.name}: canvas {target.canvas}, figure {target.height} px, pivot {target.pivot}")


if __name__ == "__main__":
    main()
