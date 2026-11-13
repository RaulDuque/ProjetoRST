"""Side-by-side model comparison: two versions, same views, scale and light.

Both rows use sprite masters at the contract scale (same orthographic
camera, sole on the pivot line), the same crop and the same background. For
comparable lighting, the older version is the relit render produced by
``bpy/build_swordsman_v003.py --relight-v002`` (v002 geometry, v003 lights).
A last column shows each side master reduced to the 1:1 game size (~52 px)
and to x2, on the room colour.

Usage:
    python3 art-src/player/scripts/post/version_compare.py --out SHEET.png
"""

from __future__ import annotations

import argparse
from pathlib import Path

from PIL import Image, ImageDraw

from contract import FRAME, HD_DENSITY
from labels import font
from palette import SCENE_BACKGROUND_FALLBACK

PLAYER_DIR = Path(__file__).resolve().parents[2]
RENDERS = PLAYER_DIR / "renders"
OLD = ("v002 (primitivas; luz da v003)", RENDERS / "v003" / "relit_v002" / "swordsman_base_v002_relit_{view}_master.png")
NEW = ("v003 (malhas procedurais)", RENDERS / "v003" / "swordsman_base_v003_{view}_master.png")
VIEWS = (("front", "Frontal"), ("side", "Lateral"), ("back", "Traseira"))
CROP = (232, 120, 792, 1024)  # same window on every 1024 master: whole figure and sword
CELL_H = 520
BACKGROUND = (112, 112, 118)
TEXT = (240, 240, 244)
MUTED = (210, 210, 216)
PAD = 16


def framed(path: Path) -> Image.Image:
    image = Image.open(path).convert("RGBA")
    base = Image.new("RGBA", image.size, (*BACKGROUND, 255))
    base.alpha_composite(image)
    crop = base.crop(CROP)
    width = round(crop.size[0] * CELL_H / crop.size[1])
    return crop.resize((width, CELL_H), Image.Resampling.LANCZOS)


def game_scale(path: Path) -> Image.Image:
    """Side master at 1:1 (64 px cell) and x2 (128 px), Lanczos, on the room colour."""
    image = Image.open(path).convert("RGBA")
    tile = Image.new("RGBA", (FRAME + FRAME * HD_DENSITY + 3 * PAD, FRAME * HD_DENSITY + 2 * PAD),
                     (*SCENE_BACKGROUND_FALLBACK, 255))
    small = image.resize((FRAME, FRAME), Image.Resampling.LANCZOS)
    big = image.resize((FRAME * HD_DENSITY, FRAME * HD_DENSITY), Image.Resampling.LANCZOS)
    tile.alpha_composite(small, (PAD, PAD + FRAME * HD_DENSITY - FRAME))
    tile.alpha_composite(big, (2 * PAD + FRAME, PAD))
    return tile


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    rows = (OLD, NEW)
    sample = framed(Path(str(NEW[1]).format(view="side")))
    cell_w = sample.size[0]
    scale_w = game_scale(Path(str(NEW[1]).format(view="side"))).size[0]
    title_h, label_h = 70, 28
    width = PAD + len(VIEWS) * (cell_w + PAD) + scale_w + PAD
    height = title_h + len(rows) * (label_h + CELL_H + PAD)
    sheet = Image.new("RGBA", (width, height), (*BACKGROUND, 255))
    draw = ImageDraw.Draw(sheet)
    draw.text((PAD, 10), "Espadachim: v002 × v003, mesmas vistas, escala, luz e fundo", fill=TEXT, font=font(20))
    draw.text((PAD, 40), "Masters ortográficos na escala do contrato (sola na linha do pivô). "
              "Última coluna: lateral reduzida a 1:1 (~52 px) e ×2, no fundo da sala.",
              fill=MUTED, font=font(14))
    y = title_h
    for label, pattern in rows:
        draw.text((PAD, y + 4), label, fill=TEXT, font=font(16))
        for i, (view, name) in enumerate(VIEWS):
            x = PAD + i * (cell_w + PAD)
            draw.text((x + cell_w - 80, y + 6), name, fill=MUTED, font=font(13))
            sheet.alpha_composite(framed(Path(str(pattern).format(view=view))), (x, y + label_h))
        x = PAD + len(VIEWS) * (cell_w + PAD)
        sheet.alpha_composite(game_scale(Path(str(pattern).format(view="side"))), (x, y + label_h))
        y += label_h + CELL_H + PAD
    args.out.parent.mkdir(parents=True, exist_ok=True)
    sheet.convert("RGB").save(args.out)
    print(f"[version_compare] {args.out} {sheet.size[0]}x{sheet.size[1]}")


if __name__ == "__main__":
    main()
