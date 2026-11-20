"""Etapa 03B (Erro-Corretion): EXPERIMENTAL pixel-art reduction, readability proof only.

This is NOT Prompt 04: nothing here is integrated in the game, nothing is hand-edited,
and the result is not a sprite candidate. It only checks whether the refined armour
shapes survive the reduction to the approved ~106 px character.

Method (N3.5 reference, `art-src/n3-5-fidelity/scripts/reduce_fidelity.py`):
1. Each armour overlay group (8x supersampled render on the sprite grid, Etapa 02 side
   camera) is box-filtered to 1x (Pillow premultiplies alpha); alpha >= 50 % is
   opaque, the rest transparent. Same BOX + 50 % rule as N3.5.
2. Colour, three variants:
   A. nearest colour of the N3.4 palette without the hand-only accents (exactly the
      N3.5 rule): shows that N3.4 has no purple/gold.
   B. N3.4 + a complementary armour palette (5 purples, 3 golds): a texel is gold
      when its hue is 15-75 deg and saturation >= 0.18, purple otherwise; inside the
      ramp the nearest luma wins (ramp mapping, no dithering).
   C. B + the N3.4 outline `K` on armour texels that touch the transparent
      background (the approved sprite has a dark outline; the armour has none).
3. The dressed layers of 02B are composed at 1x in the approved order (exact texels).

Writes `pixel_readability_test.png` and `pixel_palette_03b.json`.

    python3 -I -B evidence/erro_corretion/etapa03b/scripts/pixel_readability.py
"""

from __future__ import annotations

import colorsys
import json
import sys
from pathlib import Path

from PIL import Image, ImageDraw

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import compose_refined as cr  # noqa: E402

ROOT = cr.ROOT
sys.path.insert(0, str(ROOT / "art-src/n3-4-original-protagonist/scripts"))
from n34_palette import PALETTE  # noqa: E402

N34 = {k: v for k, v in PALETTE.items() if k not in {"M", "e"}}
OUTLINE = PALETTE["K"]
# Complementary armour palette (proposal for Prompt 04, not approved).
ARMOUR_PURPLE = {
    "V1": (150, 108, 178),  # highlight
    "V2": (114, 74, 146),
    "V3": (84, 50, 114),    # base
    "V4": (58, 32, 82),
    "V5": (38, 20, 54),     # deep shadow
}
ARMOUR_GOLD = {
    "Y1": (238, 208, 122),  # highlight
    "Y2": (196, 148, 66),   # base
    "Y3": (124, 84, 40),    # shadow
}
ZOOM = 4


def luma(c) -> float:
    return 0.299 * c[0] + 0.587 * c[1] + 0.114 * c[2]


def nearest(c, colours: dict) -> tuple[int, int, int]:
    return min(colours.values(), key=lambda p: sum(abs(a - b) for a, b in zip(p, c[:3], strict=True)))


def ramp(c) -> tuple[int, int, int]:
    h, s, _ = colorsys.rgb_to_hsv(*(v / 255 for v in c[:3]))
    family = ARMOUR_GOLD if 15 / 360 <= h <= 75 / 360 and s >= 0.18 else ARMOUR_PURPLE
    # Rendered colours are darker than the ramp: stretch luma to the ramp range first.
    lo, hi = (40, 175) if family is ARMOUR_GOLD else (25, 120)
    target_lo, target_hi = luma(min(family.values(), key=luma)), luma(max(family.values(), key=luma))
    t = (min(max(luma(c), lo), hi) - lo) / (hi - lo)
    want = target_lo + t * (target_hi - target_lo)
    return min(family.values(), key=lambda p: abs(luma(p) - want))


def reduce_group(image: Image.Image, variant: str) -> Image.Image:
    small = image.resize((image.width // cr.SUPER, image.height // cr.SUPER), Image.Resampling.BOX)
    out = Image.new("RGBA", small.size, (0, 0, 0, 0))
    src, dst = small.load(), out.load()
    for y in range(small.height):
        for x in range(small.width):
            c = src[x, y]
            if c[3] < 128:
                continue
            colour = nearest(c, N34) if variant == "A" else ramp(c)
            dst[x, y] = (*colour, 255)
    return out


def compose_1x(name: str, size, armour: dict[str, Image.Image] | None) -> Image.Image:
    canvas = Image.new("RGBA", size, (0, 0, 0, 0))
    for item in cr.ORDER:
        if item.startswith("@"):
            if armour is not None:
                canvas.alpha_composite(armour[item[1:]])
            continue
        path = cr.layer_path(name, item)
        if path.exists():
            canvas.alpha_composite(Image.open(path).convert("RGBA"))
    return canvas


def outline(image: Image.Image, armour_cells: set) -> Image.Image:
    out = image.copy()
    px, dst = image.load(), out.load()
    for x, y in armour_cells:
        if not px[x, y][3]:
            continue
        for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
            nx, ny = x + dx, y + dy
            if not (0 <= nx < image.width and 0 <= ny < image.height) or not px[nx, ny][3]:
                dst[x, y] = (*OUTLINE, 255)
                break
    return out


def reduced_pose(folder: Path, name: str, size, variant: str) -> Image.Image:
    groups = {g: Image.open(folder / f"armor_{name}_{g}.png").convert("RGBA") for g in cr.GROUP_NAMES}
    small = {g: reduce_group(img, "A" if variant == "A" else "B") for g, img in groups.items()}
    image = compose_1x(name, size, small)
    if variant == "C":
        cells = set()
        for img in small.values():
            a = img.getchannel("A").load()
            cells |= {(x, y) for y in range(img.height) for x in range(img.width) if a[x, y]}
        image = outline(image, cells)
    return image


def tile(image: Image.Image, zoom: int) -> Image.Image:
    big = image.resize((image.width * zoom, image.height * zoom), Image.Resampling.NEAREST)
    base = Image.new("RGBA", big.size, cr.BG)
    base.alpha_composite(big)
    return base


def main() -> None:
    targets = json.loads(cr.TARGETS.read_text())
    columns = [("original aprovado", None, None), ("ANTES 03, paleta B", cr.BEFORE, "B"),
               ("03B, paleta N3.4 (A)", cr.OUT, "A"), ("03B, N3.4 + compl. (B)", cr.OUT, "B"),
               ("03B, B + contorno K (C)", cr.OUT, "C")]
    rows = []
    natives = []
    for name, pose in targets.items():
        size = tuple(pose["canvas"])
        images = []
        for _, folder, variant in columns:
            images.append(cr.sprite_of(name, pose) if folder is None else reduced_pose(folder, name, size, variant))
        rows.append((name, images))
        natives.append((name, images[-1], images[0]))
    cell_w = 112 * ZOOM
    cell_h = 128 * ZOOM
    head, gap = 22, 6
    width = len(columns) * (cell_w + gap)
    native_h = 3 * 128 + 60
    sheet = Image.new("RGBA", (width, 30 + len(rows) * (cell_h + head) + native_h + 140), cr.SHEET_BG)
    draw = ImageDraw.Draw(sheet)
    draw.text((6, 6), "Etapa 03B - prova EXPERIMENTAL de reducao (nao e o Prompt 04, nao integrada). "
                      "BOX 8x -> 1x, alfa >= 50 %; A = paleta N3.4; B = N3.4 + paleta complementar roxo/dourado; "
                      "C = B + contorno K. Ampliado 4x (nearest).", fill=cr.WHITE)
    y = 30
    for name, images in rows:
        for i, (img, (title, _, _)) in enumerate(zip(images, columns, strict=True)):
            x = i * (cell_w + gap)
            sheet.alpha_composite(tile(img, ZOOM), (x + (cell_w - img.width * ZOOM) // 2, y + head))
            draw.text((x + 4, y + 4), f"{name} - {title}", fill=cr.WHITE)
        y += cell_h + head
    # Native size (1x) and 2x, variant C next to the original: how it reads in game.
    draw.text((6, y + 6), "Tamanho real (1x) e 2x: original | 03B variante C", fill=cr.WHITE)
    y += 26
    for zoom, top in ((1, y), (2, y + 128 + 10)):
        x = 6
        for _, reduced, original in natives:
            for img in (original, reduced):
                sheet.alpha_composite(tile(img, zoom), (x, top))
                x += img.width * zoom + 4
            x += 12
    y += 128 * 3 + 20
    # Palette swatches.
    draw.text((6, y), "Paleta complementar proposta (B/C):", fill=cr.WHITE)
    for i, (key, colour) in enumerate({**ARMOUR_PURPLE, **ARMOUR_GOLD}.items()):
        draw.rectangle([6 + i * 110, y + 18, 6 + i * 110 + 60, y + 58], fill=colour)
        draw.text((6 + i * 110, y + 62), f"{key} {colour}", fill=(220, 220, 220))
    sheet.save(cr.OUT / "pixel_readability_test.png")
    (cr.OUT / "pixel_palette_03b.json").write_text(json.dumps(
        {"status": "experimental proposal, not approved", "purple": ARMOUR_PURPLE, "gold": ARMOUR_GOLD,
         "outline": OUTLINE, "base_palette": "N3.4 (art-src/n3-4-original-protagonist/scripts/n34_palette.py)"},
        indent=1))
    print("saved", cr.OUT / "pixel_readability_test.png")


if __name__ == "__main__":
    main()
