"""Model sheet of one Blender model version on a neutral background.

Row 1: front, side and back sprite masters (same orthographic scale, same
crop, so proportions compare directly) and the 3/4 perspective view.
Row 2: the detail renders (collar, coat, half-cape, sword).
The masters come from ``bpy/previews.py`` and the other views from
``bpy/presentation.py``; nothing is retouched.

``--layout v003`` (V1.2-C) adds a third row (boots, sword, flat "toon"
render, readable-sword variant) and close-ups of face and hair;
``--layout v004`` (V1.2-D) adds face front/profile and torso close-ups;
``--background dark`` uses the room colour instead of the neutral grey.

Usage:
    python3 art-src/player/scripts/post/model_sheet.py --version v002 --out SHEET.png
    python3 art-src/player/scripts/post/model_sheet.py --version v003 --layout v003 \
        --renders art-src/player/renders/v003 --out SHEET.png [--background dark]
"""

from __future__ import annotations

import argparse
from pathlib import Path

from PIL import Image, ImageDraw

from labels import font
from palette import CHARACTER, CHARACTER_RGB, SCENE_BACKGROUND_FALLBACK

PLAYER_DIR = Path(__file__).resolve().parents[2]
RENDERS = PLAYER_DIR / "renders"
MODEL_NAME = "swordsman_base"

BACKGROUNDS = {
    "neutral": (112, 112, 118),  # neutral mid grey: neither the room nor white
    "dark": SCENE_BACKGROUND_FALLBACK,  # the room colour of the game
}
BACKGROUND = BACKGROUNDS["neutral"]
CELL = 420
PAD = 16
LABEL_H = 26
TEXT = (240, 240, 244)
MUTED = (210, 210, 216)

ROW_TURNAROUND = (
    ("front_master", "Frontal (ortográfica)"),
    ("side_master", "Lateral (ortográfica, vista do jogo)"),
    ("back_master", "Traseira (ortográfica)"),
    ("three_quarter", "Três quartos (perspectiva 85 mm)"),
)
ROW_DETAILS = (
    ("detail_collar", "Detalhe: cabeça, gola e forro"),
    ("detail_coat", "Detalhe: casaca aberta, cinto e fivela"),
    ("detail_cape", "Detalhe: meia-capa assimétrica"),
    ("detail_sword", "Detalhe: espada (guarda e curvatura)"),
)


ROWS_V003 = (
    (("front_master", "Frontal (ortográfica)"), ("side_master", "Lateral (ortográfica, vista do jogo)"),
     ("back_master", "Traseira (ortográfica)"), ("three_quarter", "Três quartos (perspectiva 85 mm)")),
    (("close_face", "Close: rosto"), ("close_hair", "Close: cabelo (costas, 3/4)"),
     ("close_coat", "Close: casaca, forro, cinto"), ("close_cape", "Close: meia-capa e fecho")),
    (("close_boots", "Close: botas"), ("close_sword", "Close: espada e mão"),
     ("side_master_toon", "Lateral, cores chapadas (teste C)"),
     ("side_master_readable", "Lateral, lâmina ×1,7 (variante)")),
)
ROWS_V004 = (
    ROWS_V003[0],
    (("close_face", "Close: rosto (3/4)"), ("close_face_front", "Close: rosto (frente)"),
     ("close_face_profile", "Close: rosto (perfil)"), ("close_hair", "Close: cabelo (costas, 3/4)")),
    (("close_torso", "Close: tronco, colete e cinto"), ("close_coat", "Close: casaca, forro, cinto"),
     ("close_cape", "Close: meia-capa e fecho"), ("close_boots", "Close: botas")),
    (("close_sword", "Close: espada e mão"), ("side_master_toon", "Lateral, cores chapadas (teste C)"),
     ("side_master_readable", "Lateral, lâmina ×1,7 (variante)"),
     ("side_master_readable_toon", "Lateral, lâmina ×1,7, cores chapadas")),
)
LAYOUTS = {
    "v002": ((ROW_TURNAROUND, ROW_DETAILS),
             "Experimental, feito de primitivas, sem rig. Não é arte final. "
             "Mesma luz em todas as vistas; fundo neutro adicionado depois do render."),
    "v003": (ROWS_V003,
             "Experimental: malhas procedurais (seções, tubos ao longo de curvas, tecido com espessura), "
             "sem rig. Não é arte final. Luz relativa à câmera em todas as vistas."),
    "v004": (ROWS_V004,
             "Experimental: refinamento localizado do v003 (rosto, cabelo, corpo, vestuário, materiais), "
             "sem rig. Não é arte final. Mesmas câmeras e luz do v003."),
}


def on_background(path: Path, size: int, background: tuple[int, int, int] = BACKGROUND) -> Image.Image:
    image = Image.open(path).convert("RGBA")
    base = Image.new("RGBA", image.size, (*background, 255))
    base.alpha_composite(image)
    return base.resize((size, size), Image.Resampling.LANCZOS)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--version", required=True, help="model version, e.g. v002")
    parser.add_argument("--renders", type=Path, default=RENDERS)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--layout", choices=sorted(LAYOUTS), default="v002")
    parser.add_argument("--background", choices=sorted(BACKGROUNDS), default="neutral")
    args = parser.parse_args()

    rows, subtitle = LAYOUTS[args.layout]
    background = BACKGROUNDS[args.background]
    title_h = 64
    palette_h = 60
    width = PAD + len(ROW_TURNAROUND) * (CELL + PAD)
    height = title_h + len(rows) * (LABEL_H + CELL + PAD) + palette_h
    sheet = Image.new("RGBA", (width, height), (*background, 255))
    draw = ImageDraw.Draw(sheet)
    draw.text((PAD, 10), f"Espadachim (conceito B): modelo-base {args.version}, Blender 5.2.4, EEVEE",
              fill=TEXT, font=font(20))
    draw.text((PAD, 38), subtitle, fill=MUTED, font=font(14))
    y = title_h
    for row in rows:
        for i, (view, label) in enumerate(row):
            x = PAD + i * (CELL + PAD)
            draw.text((x, y + 4), label, fill=TEXT, font=font(14))
            path = args.renders / f"{MODEL_NAME}_{args.version}_{view}.png"
            sheet.alpha_composite(on_background(path, CELL, background), (x, y + LABEL_H))
        y += LABEL_H + CELL + PAD
    swatch = (width - 2 * PAD) // len(CHARACTER)
    for i, name in enumerate(CHARACTER):
        x = PAD + i * swatch
        draw.rectangle((x, y, x + swatch - 3, y + 24), fill=CHARACTER_RGB[name])
    draw.text((PAD, y + 30), "Paleta preliminar (palette.py)", fill=MUTED, font=font(13))

    args.out.parent.mkdir(parents=True, exist_ok=True)
    sheet.convert("RGB").save(args.out)
    print(f"[model_sheet] {args.out} {sheet.size[0]}x{sheet.size[1]}")


if __name__ == "__main__":
    main()
