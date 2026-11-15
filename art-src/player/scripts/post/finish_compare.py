"""Comparison sheet: finish A (pixel art) x finish B (high resolution).

Same character, pose and framing in every cell; only the finish changes.
Rows show the sprite as it would appear on screen with the current 1:1
camera, with a x2 zoom, and magnified x4 for inspection. The background is
the real room colour so contrast can be judged.

Usage:
    python3 art-src/player/scripts/post/finish_compare.py
"""

from __future__ import annotations

import argparse
from pathlib import Path

from PIL import Image, ImageDraw

from labels import font
from palette import CHARACTER, CHARACTER_RGB, SCENE_BACKGROUND_FALLBACK
from resample import resize

PLAYER_DIR = Path(__file__).resolve().parents[2]
RENDERS = PLAYER_DIR / "renders"
OUT = PLAYER_DIR / "comparisons" / "finish_A_pixel_vs_B_hd.png"
DEFAULT_NAME = "swordsman_idle_side"
DEFAULT_SUBTITLE = "Figura vetorial provisória (Blender indisponível). Experimental, não é arte final."

CELL_W = 290
LABEL_H = 40
PAD = 14
TEXT = (225, 220, 230)
MUTED = (150, 145, 160)


def show(sprite: Image.Image, target: int, smooth: bool) -> Image.Image:
    if sprite.size[0] == target:
        return sprite
    if smooth:
        return sprite.resize((target, target), Image.Resampling.BILINEAR)
    return resize(sprite, (target, target), Image.Resampling.NEAREST)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--renders", type=Path, default=RENDERS)
    parser.add_argument("--out", type=Path, default=OUT)
    parser.add_argument("--name", default=DEFAULT_NAME, help="sprite set produced by pixelize.py")
    parser.add_argument("--subtitle", default=DEFAULT_SUBTITLE, help="second title line (source of the sprite)")
    args = parser.parse_args()

    sprites = {
        key: Image.open(args.renders / f"{args.name}_{key}.png").convert("RGBA")
        for key in ("A_pixel_raw", "A_pixel_rim", "B_hd_raw", "B_hd_rim")
    }
    columns = [
        ("A_pixel_raw", "A  pixel art", "sem contorno", False),
        ("A_pixel_rim", "A  pixel art", "contorno 1 px (rim)", False),
        ("B_hd_raw", "B  alta resolução", "sem contorno", True),
        ("B_hd_rim", "B  alta resolução", "contorno suave (rim)", True),
    ]
    rows = [
        ("Câmera 1:1 (tamanho real na tela)", 64),
        ("Câmera com zoom x2", 128),
        ("Ampliado x4 para inspeção (não é tamanho de jogo)", 256),
    ]
    title_h = 70
    palette_h = 70
    height = title_h + sum(LABEL_H + size + PAD for _, size in rows) + palette_h + PAD
    width = CELL_W * len(columns) + PAD
    sheet = Image.new("RGBA", (width, height), (*SCENE_BACKGROUND_FALLBACK, 255))
    draw = ImageDraw.Draw(sheet)
    draw.text((PAD, 10), "Espadachim (conceito B): acabamento A x B, mesmo personagem e pose",
              fill=TEXT, font=font(20))
    draw.text((PAD, 38), args.subtitle, fill=MUTED, font=font(14))
    for i, (_, title, subtitle, _) in enumerate(columns):
        x = PAD + i * CELL_W
        draw.text((x, title_h - 16), f"{title}  ({subtitle})", fill=TEXT, font=font(13))

    y = title_h + 6
    for row_title, size in rows:
        draw.text((PAD, y), row_title, fill=MUTED, font=font(14))
        y += LABEL_H - 16
        for i, (key, _, _, smooth) in enumerate(columns):
            x = PAD + i * CELL_W
            sheet.alpha_composite(show(sprites[key], size, smooth), (x, y))
        y += size + PAD + 16

    draw.text((PAD, y), "Paleta preliminar", fill=MUTED, font=font(14))
    y += 22
    swatch = (width - 2 * PAD) // len(CHARACTER)
    for i, name in enumerate(CHARACTER):
        x = PAD + i * swatch
        draw.rectangle((x, y, x + swatch - 3, y + 26), fill=CHARACTER_RGB[name])

    args.out.parent.mkdir(parents=True, exist_ok=True)
    sheet.convert("RGB").save(args.out)
    print(f"[finish_compare] {args.out} {sheet.size[0]}x{sheet.size[1]}")


if __name__ == "__main__":
    main()
