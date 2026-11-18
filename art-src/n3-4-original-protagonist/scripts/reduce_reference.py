"""N3.4 step 1: reduce the owner's right-side view to a 106 px draft grid.

Reads (never writes) the right-side view in
`Md-usario/assets-user/prota/base/Ficha_Modular_do_Protagonista_Gotico.png`,
separates the figure from the flat grey panel, box-filters it to 106 px of
opaque height and maps every pixel to the nearest colour of `n34_palette.py`.

Output: `source/base_idle_draft.txt`, an 80x128 text grid. This draft is
*not* the sprite. The sprite is `source/base_idle.txt`, a copy of the draft
corrected pixel by pixel by hand (face, hair, hands, boots, outline). Keeping
both files lets anyone diff what the reduction gave and what was edited.

    python3 art-src/n3-4-original-protagonist/scripts/reduce_reference.py
"""

from __future__ import annotations

import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "art-src/player/scripts/post"))
sys.path.insert(0, str(Path(__file__).resolve().parent))
from n34_palette import PALETTE, TRANSPARENT  # noqa: E402
from resample import resize  # noqa: E402

SHEET = ROOT / "Md-usario/assets-user/prota/base/Ficha_Modular_do_Protagonista_Gotico.png"
OUT = ROOT / "art-src/n3-4-original-protagonist/source/base_idle_draft.txt"

# Right-side view on the modular sheet (sheet pixels).
VIEW_BOX = (680, 80, 775, 418)
# Last row of the boot soles inside VIEW_BOX; rows below are the floor shadow.
SOLE_ROW = 337
# Contract inherited from N3.3.
CANVAS = (80, 128)
TOP_Y = 13
SOLE_Y = 118
PIVOT_X = 40
BACKGROUND_TOLERANCE = 22
ALPHA_CUT = 0.5
# Accents that the reduction would misuse (grey hair read as metal, any dark
# pixel read as an eye). They are placed by hand only.
HAND_ONLY = {"M", "e"}


def distance(a: tuple[int, ...], b: tuple[int, ...]) -> int:
    return sum(abs(x - y) for x, y in zip(a[:3], b[:3], strict=True))


def figure_mask(view: Image.Image) -> Image.Image:
    """Flood-fill the flat panel from the border; everything else is figure."""
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
    return mask


def nearest(colour: tuple[int, int, int]) -> str:
    keys = [key for key in PALETTE if key not in HAND_ONLY]
    return min(keys, key=lambda key: distance(PALETTE[key], colour))


def main() -> None:
    view = Image.open(SHEET).convert("RGB").crop(VIEW_BOX)
    mask = figure_mask(view)
    box = mask.getbbox()
    if box is None:
        raise SystemExit("no figure found in the view box")
    figure = view.crop(box)
    alpha = mask.crop(box)
    target_h = SOLE_Y - TOP_Y + 1
    target_w = max(1, round(figure.width * target_h / figure.height))
    rgba = figure.copy()
    rgba.putalpha(alpha)
    small = resize(rgba, (target_w, target_h), Image.Resampling.BOX)
    # Feet centre on the sole row decides the horizontal placement.
    sole = [x for x in range(target_w) if small.getpixel((x, target_h - 1))[3] > 0]
    feet_centre = (sole[0] + sole[-1]) // 2 if sole else target_w // 2
    left = PIVOT_X - feet_centre
    grid = [[TRANSPARENT] * CANVAS[0] for _ in range(CANVAS[1])]
    for y in range(target_h):
        for x in range(target_w):
            r, g, b, a = small.getpixel((x, y))
            if a / 255 < ALPHA_CUT:
                continue
            gx, gy = left + x, TOP_Y + y
            if 0 <= gx < CANVAS[0] and 0 <= gy < CANVAS[1]:
                grid[gy][gx] = nearest((r, g, b))
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text("\n".join("".join(row) for row in grid) + "\n", encoding="utf-8")
    print(f"figure {figure.size} -> {target_w}x{target_h}, left={left}, wrote {OUT.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
