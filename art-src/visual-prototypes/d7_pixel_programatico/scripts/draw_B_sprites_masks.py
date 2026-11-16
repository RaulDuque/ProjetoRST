#!/usr/bin/env python3
"""D7-B: compose from 2D source parts + masks (D2 idle + D4 layers).

Honesty: pixels originate from existing D2/D4 art. This script cuts, masks,
registers to the sole pivot, and restacks — it does not invent D2 detail.
"""
from __future__ import annotations

from pathlib import Path

from PIL import Image

from d7_common import (
    CELL,
    D2,
    D4,
    PIVOT,
    ROOT,
    board,
    canvas,
    ensure_dirs,
    feet_place,
    load_d2_idle,
    nearest_up,
    scale_game,
    scale_to_height,
    trim,
)

OUT = ROOT / "estrategias" / "B_sprites_masks"
SOURCE_NOTE = OUT / "FONTES.md"


def _cut_region(src: Image.Image, box: tuple[int, int, int, int]) -> Image.Image:
    return src.crop(box).convert("RGBA")


def _mask_by_predicate(src: Image.Image, pred) -> Image.Image:
    src = src.convert("RGBA")
    out = Image.new("RGBA", src.size, (0, 0, 0, 0))
    s, d = src.load(), out.load()
    for y in range(src.height):
        for x in range(src.width):
            px = s[x, y]
            if px[3] < 16:
                continue
            if pred(x, y, px):
                d[x, y] = px
    return out


def extract_d2_parts(idle: Image.Image) -> dict[str, Image.Image]:
    """Semantic masks from D2 idle_f00 native resolution."""
    idle = idle.convert("RGBA")
    box = idle.split()[-1].getbbox()
    assert box is not None
    x0, y0, x1, y1 = box
    h = y1 - y0
    w = x1 - x0

    def head_pred(x, y, px):
        return y < y0 + int(h * 0.28)

    def boots_pred(x, y, px):
        return y > y0 + int(h * 0.82)

    def sword_pred(x, y, px):
        r, g, b, a = px
        # bright steel / light pixels toward front-right of bbox
        bright = (r + g + b) / 3 > 140 and abs(r - g) < 50
        return bright and x > x0 + int(w * 0.45) and y > y0 + int(h * 0.35)

    def wine_pred(x, y, px):
        r, g, b, a = px
        return r > g + 15 and r > b and r > 30 and g < 60 and y > y0 + int(h * 0.2)

    def body_pred(x, y, px):
        return (
            not head_pred(x, y, px)
            and not boots_pred(x, y, px)
            and not sword_pred(x, y, px)
        )

    parts = {
        "head": _mask_by_predicate(idle, head_pred),
        "cape_wine": _mask_by_predicate(idle, wine_pred),
        "body": _mask_by_predicate(idle, body_pred),
        "boots": _mask_by_predicate(idle, boots_pred),
        "sword": _mask_by_predicate(idle, sword_pred),
        "full_source": idle,
    }
    return parts


def stack_parts(parts: dict[str, Image.Image], order: list[str]) -> Image.Image:
    """Stack masked parts in draw order (already same canvas as source)."""
    base = parts["full_source"]
    out = Image.new("RGBA", base.size, (0, 0, 0, 0))
    for key in order:
        out.alpha_composite(parts[key])
    return out


def compose_from_d2_masks() -> Image.Image:
    idle = load_d2_idle()
    parts = extract_d2_parts(idle)
    # Restack: cape wine under body, sword on top — proves mask pipeline
    rebuilt = stack_parts(
        parts,
        ["cape_wine", "body", "boots", "head", "sword"],
    )
    # If sword mask too thin, keep body pixels (already in body). Fill gaps from source.
    # Prefer rebuilt where opaque, else source (preserves silhouette)
    src = idle
    s, r = src.load(), rebuilt.load()
    for y in range(src.height):
        for x in range(src.width):
            if r[x, y][3] < 16 and s[x, y][3] >= 16:
                r[x, y] = s[x, y]
    placed = feet_place(scale_to_height(rebuilt, 110))
    return placed, parts


def compose_from_d4_layers() -> Image.Image:
    """Independent 2D parts from D4 camadas, stacked by code."""
    order = [
        ("01_cape_back.png", "cape"),
        ("00_body.png", "body"),
        ("02_light_coat.png", "coat"),
        ("07_sword.png", "sword"),
    ]
    layers = []
    for fname, _name in order:
        p = D4 / "camadas" / fname
        if p.exists():
            layers.append(Image.open(p).convert("RGBA"))
    if not layers:
        return canvas()
    # normalize each to ~110h then feet-place and composite
    out = canvas()
    for lay in layers:
        sp = feet_place(scale_to_height(lay, 110))
        out.alpha_composite(sp)
    return out


def main() -> None:
    ensure_dirs()
    OUT.mkdir(parents=True, exist_ok=True)

    d2_pose, parts = compose_from_d2_masks()
    d2_pose.save(OUT / "pose_estatica.png")
    nearest_up(d2_pose, 3).save(OUT / "pose_estatica_3x.png")
    scale_game(d2_pose).save(OUT / "pose_escala_jogo.png")

    for name, im in parts.items():
        if name == "full_source":
            continue
        trim(im).save(OUT / f"mask_{name}.png")

    d4_pose = compose_from_d4_layers()
    d4_pose.save(OUT / "pose_d4_stack.png")
    nearest_up(d4_pose, 3).save(OUT / "pose_d4_stack_3x.png")

    # Primary candidate = D2 mask rebuild (highest fidelity to approved identity)
    board(
        [
            ("D2 masks", nearest_up(d2_pose, 2)),
            ("D4 stack", nearest_up(d4_pose, 2)),
            ("game", nearest_up(scale_game(d2_pose), 3)),
        ],
        OUT / "preview_board.png",
        "D7-B sprites+masks (fonte D2/D4)",
    )

    SOURCE_NOTE.write_text(
        "# Fontes D7-B\n\n"
        "- `d2_protagonista_B/animacoes/idle/frames/idle_f00.png` — máscaras semânticas "
        "(head/body/wine/boots/sword) e restack.\n"
        "- `d4_modular_d2/camadas/*.png` — prova de composição por peças 2D independentes.\n\n"
        "Este candidato **não inventa** o acabamento D2; o código corta, mascara e "
        "recompõe arte-fonte já aprovada.\n",
        encoding="utf-8",
    )
    print(f"D7-B wrote {OUT}")


if __name__ == "__main__":
    main()
