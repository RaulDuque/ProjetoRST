"""Game-scale comparison of two model versions: finishes A/B/C, two blades, three zooms.

Each row is one sprite set produced by ``pixelize.py`` (side view, ~52 px
tall at 1:1) on the room colour; columns are the zooms 1:1 (the provisional
game camera), x1.5 and x2, plus a x4 nearest-neighbour inspection of the 1:1
pixels. A finish: nearest scaling of the 64 px sprite (x1.5 is shown only to
compare; pixel art needs integer zooms). B: the 128 px (density 2) sprite
scaled with a linear filter. C: the flat two-band render through the same
two paths (rows "C/A" and "C/B").

It also prints and draws how many 1:1 pixels differ between the versions,
so close-up refinements are not mistaken for gameplay-scale improvements.

Usage:
    python3 art-src/player/scripts/post/scale_compare.py --old v003 --new v004 --out SHEET.png
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path

from PIL import Image, ImageChops, ImageDraw

from contract import FRAME
from labels import font
from palette import SCENE_BACKGROUND_FALLBACK
from resample import resize

PLAYER_DIR = Path(__file__).resolve().parents[2]
RENDERS = PLAYER_DIR / "renders"
MODEL_NAME = "swordsman_base"
ZOOMS = (1.0, 1.5, 2.0)
INSPECT = 4
PAD = 12
LABEL_W = 250
TEXT = (225, 220, 230)
MUTED = (150, 145, 160)
SHEET_BG = (40, 38, 46)

# (row label, variant name used by pixelize.py, finish path: "A" or "B")
ROWS = (
    ("A pixel art, lâmina normal", "side", "A"),
    ("B alta resolução, lâmina normal", "side", "B"),
    ("C chapada → A, lâmina normal", "side_toon", "A"),
    ("C chapada → B, lâmina normal", "side_toon", "B"),
    ("A pixel art, lâmina ×1,7", "side_readable", "A"),
    ("B alta resolução, lâmina ×1,7", "side_readable", "B"),
    ("C chapada → A, lâmina ×1,7", "side_readable_toon", "A"),
)


def sprite_path(version: str, variant: str, finish: str) -> Path:
    key = "A_pixel_rim" if finish == "A" else "B_hd_rim"
    return RENDERS / version / f"{MODEL_NAME}_{version}_{variant}_{key}.png"


def on_screen(sprite: Image.Image, finish: str, zoom: float) -> Image.Image:
    """The sprite as the game camera would show it: FRAME * zoom screen pixels."""
    size = round(FRAME * zoom)
    method = Image.Resampling.NEAREST if finish == "A" else Image.Resampling.BILINEAR
    tile = Image.new("RGBA", (size, size), (*SCENE_BACKGROUND_FALLBACK, 255))
    tile.alpha_composite(resize(sprite, (size, size), method))
    return tile


def one_to_one(sprite: Image.Image, finish: str) -> Image.Image:
    return on_screen(sprite, finish, 1.0).convert("RGB")


def difference(old: Image.Image, new: Image.Image) -> tuple[int, int]:
    """(pixels that differ, pixels covered by either figure) at 1:1."""
    def mask(image: Image.Image, threshold: int) -> Image.Image:
        return image.convert("L").point(lambda v: 255 if v > threshold else 0)

    changed = mask(ImageChops.difference(old, new), 12).histogram()[255]
    bg = Image.new("RGB", old.size, SCENE_BACKGROUND_FALLBACK)
    either = ImageChops.lighter(mask(ImageChops.difference(old, bg), 0), mask(ImageChops.difference(new, bg), 0))
    return changed, either.histogram()[255]


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--old", required=True, help="older version, e.g. v003")
    parser.add_argument("--new", required=True, help="newer version, e.g. v004")
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--metrics", type=Path, help="optional JSON with the 1:1 differences")
    args = parser.parse_args()

    zoom_w = [round(FRAME * z) for z in ZOOMS]
    cell_w = sum(zoom_w) + FRAME * INSPECT + PAD * (len(ZOOMS) + 1)
    row_h = FRAME * INSPECT + PAD
    title_h, header_h = 64, 26
    entries = []
    for label, variant, finish in ROWS:
        paths = {v: sprite_path(v, variant, finish) for v in (args.old, args.new)}
        entries.append((label, variant, finish, {v: p for v, p in paths.items() if p.exists()}))
    width = LABEL_W + 2 * cell_w + 3 * PAD
    height = title_h + header_h + len(entries) * (row_h + PAD) + PAD
    sheet = Image.new("RGBA", (width, height), (*SHEET_BG, 255))
    draw = ImageDraw.Draw(sheet)
    draw.text((PAD, 8), f"Escala de jogo: {args.old} × {args.new} (lateral, ~52 px em 1:1, fundo da sala)",
              fill=TEXT, font=font(18))
    draw.text((PAD, 34), "Colunas por versão: 1:1 (câmera provisória), ×1,5, ×2 e o 1:1 ampliado ×4 "
              "para inspeção. Δ = pixels 1:1 que mudam entre as versões.", fill=MUTED, font=font(13))
    for k, version in enumerate((args.old, args.new)):
        x = LABEL_W + PAD + k * (cell_w + PAD)
        draw.text((x, title_h), f"{version}:  1:1   ×1,5   ×2   (1:1 ×4)", fill=TEXT, font=font(14))
    metrics = {}
    y = title_h + header_h
    for label, variant, finish, paths in entries:
        draw.text((PAD, y + 6), label, fill=TEXT, font=font(13))
        if len(paths) == 2:
            old_img, new_img = (one_to_one(Image.open(paths[v]).convert("RGBA"), finish)
                                for v in (args.old, args.new))
            changed, covered = difference(old_img, new_img)
            share = changed / covered if covered else 0.0
            metrics[f"{variant}_{finish}"] = {"changed_px": changed, "figure_px": covered,
                                              "share": round(share, 3)}
            draw.text((PAD, y + 26), f"Δ 1:1: {changed} de {covered} px ({share:.0%})", fill=MUTED, font=font(12))
        for k, version in enumerate((args.old, args.new)):
            x = LABEL_W + PAD + k * (cell_w + PAD)
            if version not in paths:
                draw.text((x, y + 6), f"{version}: não gerado nesta versão", fill=MUTED, font=font(12))
                continue
            sprite = Image.open(paths[version]).convert("RGBA")
            for zoom, w in zip(ZOOMS, zoom_w):
                tile = on_screen(sprite, finish, zoom)
                sheet.alpha_composite(tile, (x, y + row_h - PAD - tile.size[1]))
                x += w + PAD
            big = resize(one_to_one(sprite, finish), (FRAME * INSPECT,) * 2, Image.Resampling.NEAREST)
            sheet.paste(big, (x, y))
        y += row_h + PAD
    args.out.parent.mkdir(parents=True, exist_ok=True)
    sheet.convert("RGB").save(args.out)
    print(f"[scale_compare] {args.out} {sheet.size[0]}x{sheet.size[1]}")
    for key, value in metrics.items():
        print(f"[scale_compare] {key}: {value}")
    if args.metrics:
        args.metrics.write_text(json.dumps(metrics, indent=2) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
