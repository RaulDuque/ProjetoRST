#!/usr/bin/env python3
"""D7-C: hybrid — Blender/D5–D6 silhouette as anatomy mask + D2 palette + stamped details.

Does NOT ship EEVEE/toon finish as final look. Silhouette guides placement;
colors and details come from D2 palette + cropped D2 head stamp.
"""
from __future__ import annotations

import math
from pathlib import Path

from PIL import Image, ImageDraw

from d7_common import (
    PAL,
    PIVOT,
    ROOT,
    board,
    canvas,
    ensure_dirs,
    feet_place,
    load_d2_idle,
    nearest_up,
    quantize_to_d2,
    scale_game,
    scale_to_height,
    trim,
)

OUT = ROOT / "estrategias" / "C_hibrido"
LAB_H = 100


def load_anatomy_silhouette() -> Image.Image:
    for rel in (
        ROOT / "referencias" / "d6_toon.png",
        ROOT / "referencias" / "d5_toon_f00.png",
    ):
        if rel.exists():
            return Image.open(rel).convert("RGBA")
    return load_d2_idle()


def silhouette_mask(src: Image.Image) -> Image.Image:
    """Alpha-only silhouette; drop bright steel (weapon) so we redraw the saber."""
    src = trim(src.convert("RGBA"), pad=1)
    out = Image.new("RGBA", src.size, (0, 0, 0, 0))
    s, d = src.load(), out.load()
    for y in range(src.height):
        for x in range(src.width):
            r, g, b, a = s[x, y]
            if a < 24:
                continue
            # skip bright blade pixels
            if (r + g + b) / 3 > 160 and abs(r - g) < 40:
                continue
            d[x, y] = (255, 255, 255, 255)
    return scale_to_height(out, LAB_H)


def fill_mask_with_palette(mask: Image.Image) -> Image.Image:
    box = mask.split()[-1].getbbox()
    if not box:
        return canvas()
    mx0, my0, mx1, my1 = box
    h = max(1, my1 - my0)
    w = max(1, mx1 - mx0)
    out = Image.new("RGBA", mask.size, (0, 0, 0, 0))
    s, d = mask.load(), out.load()
    for y in range(mask.height):
        for x in range(mask.width):
            if s[x, y][3] < 24:
                continue
            rel = (y - my0) / h
            across = (x - mx0) / w
            if rel < 0.16:
                col = PAL["hair"]
            elif rel < 0.22 and across > 0.35:
                col = PAL["skin"]
            elif rel > 0.88:
                col = PAL["boot"]
            elif across < 0.38 and rel > 0.28:
                col = PAL["wine"] if rel > 0.45 else PAL["coat"]
            elif 0.22 <= rel <= 0.35 and across > 0.55:
                col = PAL["wine_hi"]  # lapel hint
            else:
                col = PAL["coat_hi"] if across > 0.55 else PAL["coat"]
            d[x, y] = col
    return out


def stamp_d2_head(base: Image.Image) -> Image.Image:
    idle = load_d2_idle()
    box = idle.split()[-1].getbbox()
    if not box:
        return base
    x0, y0, x1, y1 = box
    head_h = max(6, int((y1 - y0) * 0.30))
    head = idle.crop((x0, y0, x1, y0 + head_h))
    # Match ~18% of lab character height
    target = max(12, int(LAB_H * 0.20))
    head = scale_to_height(head, target)
    out = base.copy()
    bb = out.split()[-1].getbbox()
    if not bb:
        return out
    # Replace top hair band: composite head centered on shoulders
    hx = bb[0] + (bb[2] - bb[0]) // 2 - head.width // 2 + 1
    hy = bb[1] - 1
    # clear a small head rectangle so flat hair band does not clash
    clear = Image.new("RGBA", out.size, (0, 0, 0, 0))
    cpx = clear.load()
    opx = out.load()
    for y in range(bb[1], min(bb[1] + target + 2, out.height)):
        for x in range(bb[0], bb[2]):
            if opx[x, y][3] > 0 and (y - bb[1]) < target + 1:
                # keep lower neck
                if (y - bb[1]) < int(target * 0.85):
                    opx[x, y] = (0, 0, 0, 0)
    out.alpha_composite(head, (max(0, hx), max(0, hy)))
    return out


def stamp_sword(base: Image.Image) -> Image.Image:
    out = base.copy()
    scale = LAB_H / 52.0
    wx = PIVOT[0] + int(8.28 * scale * 0.5)
    wy = PIVOT[1] - int(27.83 * scale * 0.5)
    ang = math.radians(-66.0)
    for i in range(18):
        t = i / 17
        bend = math.sin(t * math.pi) * 2.5
        dx = math.cos(ang) * (t * 26) + math.cos(ang + math.pi / 2) * bend
        dy = -math.sin(ang) * (t * 26) + math.sin(ang + math.pi / 2) * bend
        x, y = int(wx + dx), int(wy + dy)
        if 0 <= x < out.width and 0 <= y < out.height:
            out.putpixel((x, y), PAL["steel_hi"] if i % 4 == 0 else PAL["steel"])
            if y + 1 < out.height:
                out.putpixel((x, y + 1), PAL["steel_dk"])
    d = ImageDraw.Draw(out)
    d.line([(wx - 2, wy), (wx + 3, wy + 2)], fill=PAL["gold"], width=2)
    return out


def add_outline(im: Image.Image) -> Image.Image:
    out = im.copy()
    s = im.load()
    d = out.load()
    w, h = im.size
    for y in range(h):
        for x in range(w):
            if s[x, y][3] < 128:
                continue
            for nx, ny in ((x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)):
                if not (0 <= nx < w and 0 <= ny < h) or s[nx, ny][3] < 128:
                    r, g, b, a = s[x, y]
                    d[x, y] = (max(0, r // 4), max(0, g // 4), max(0, b // 4), a)
                    break
    return out


def gold_accents(im: Image.Image) -> Image.Image:
    out = im.copy()
    bb = out.split()[-1].getbbox()
    if not bb:
        return out
    cx = (bb[0] + bb[2]) // 2
    for dy in (34, 42, 50):
        y = bb[1] + dy
        x = cx + 3
        if 0 <= x < out.width and 0 <= y < out.height and out.getpixel((x, y))[3] > 0:
            out.putpixel((x, y), PAL["gold"])
    return out


def build_hybrid() -> Image.Image:
    sil = load_anatomy_silhouette()
    mask = silhouette_mask(sil)
    painted = fill_mask_with_palette(mask)
    painted = stamp_d2_head(painted)
    painted = gold_accents(painted)
    painted = add_outline(painted)
    placed = feet_place(painted)
    placed = stamp_sword(placed)
    return placed


def build_quantized_blender_control() -> Image.Image:
    sil = load_anatomy_silhouette()
    q = quantize_to_d2(scale_to_height(sil, LAB_H))
    return feet_place(q)


def main() -> None:
    ensure_dirs()
    OUT.mkdir(parents=True, exist_ok=True)

    hybrid = build_hybrid()
    hybrid.save(OUT / "pose_estatica.png")
    nearest_up(hybrid, 3).save(OUT / "pose_estatica_3x.png")
    scale_game(hybrid).save(OUT / "pose_escala_jogo.png")

    control = build_quantized_blender_control()
    control.save(OUT / "controle_quantize_d6.png")

    (OUT / "FONTES.md").write_text(
        "# Fontes D7-C\n\n"
        "- Silhueta/anatomia: `referencias/d6_toon.png` (ou D5) — **apenas máscara**.\n"
        "- Cabeça: recorte de `d2_protagonista_B/.../idle_f00.png`.\n"
        "- Corpo: preenchimento programático com paleta D2.\n"
        "- Espada: traçado por código (ângulo do socket idle).\n"
        "- `controle_quantize_d6.png`: recoloração direta do toon — controle negativo.\n",
        encoding="utf-8",
    )

    board(
        [
            ("hybrid", nearest_up(hybrid, 2)),
            ("quantize D6", nearest_up(control, 2)),
            ("game", nearest_up(scale_game(hybrid), 3)),
        ],
        OUT / "preview_board.png",
        "D7-C hibrido (mascara Blender + pintura 2D)",
    )
    print(f"D7-C wrote {OUT}")


if __name__ == "__main__":
    main()
