#!/usr/bin/env python3
"""Marco A boards: D2 vs D5/D6 vs D7-A/B/C at cell and gameplay scale."""
from __future__ import annotations

from pathlib import Path

from PIL import Image, ImageDraw

from d7_common import (
    D5,
    D6,
    ROOT,
    VIS,
    board,
    ensure_dirs,
    feet_place,
    load_causeway,
    load_d2_idle,
    nearest_up,
    scale_game,
    scale_to_height,
)

CMP = ROOT / "comparacoes"


def _load(path: Path) -> Image.Image | None:
    return Image.open(path).convert("RGBA") if path.exists() else None


def _ref_d5() -> Image.Image:
    p = ROOT / "referencias" / "d5_toon_f00.png"
    if p.exists():
        return feet_place(scale_to_height(_load(p), 110))
    return feet_place(scale_to_height(load_d2_idle(), 110))


def _ref_d6() -> Image.Image:
    p = ROOT / "referencias" / "d6_toon.png"
    if p.exists():
        return feet_place(scale_to_height(_load(p), 110))
    return _ref_d5()


def _d7(letter: str) -> Image.Image:
    folder = {
        "A": ROOT / "estrategias" / "A_primitivas" / "pose_estatica.png",
        "B": ROOT / "estrategias" / "B_sprites_masks" / "pose_estatica.png",
        "C": ROOT / "estrategias" / "C_hibrido" / "pose_estatica.png",
    }[letter]
    im = _load(folder)
    if im is None:
        raise FileNotFoundError(folder)
    return im


def causeway_strip(sprites: list[tuple[str, Image.Image]], path: Path) -> None:
    bg = load_causeway()
    games = [(lab, scale_game(im)) for lab, im in sprites]
    if bg is None:
        board([(lab, nearest_up(im, 3)) for lab, im in games], path, "causeway fallback", bg=(20, 18, 28))
        return
    bg = bg.convert("RGBA")
    # tile characters along ground line
    gap = 8
    total_w = gap + sum(im.width + gap for _, im in games)
    # crop a horizontal slice of causeway
    ch = max(80, max(im.height for _, im in games) + 40)
    if bg.height > ch:
        y0 = max(0, bg.height - ch - 20)
        strip = bg.crop((0, y0, min(bg.width, total_w + 40), y0 + ch))
    else:
        strip = bg
    if strip.width < total_w + 20:
        # extend
        ext = Image.new("RGBA", (total_w + 40, strip.height), (20, 18, 28, 255))
        ext.alpha_composite(strip, (0, 0))
        strip = ext
    out = strip.copy()
    d = ImageDraw.Draw(out)
    x = 12
    ground = out.height - 12
    for lab, im in games:
        y = ground - im.height
        out.alpha_composite(im, (x, max(0, y)))
        d.text((x, 4), lab, fill=(230, 210, 170, 255))
        x += im.width + gap
    path.parent.mkdir(parents=True, exist_ok=True)
    out.convert("RGB").save(path)


def main() -> None:
    ensure_dirs()
    CMP.mkdir(parents=True, exist_ok=True)

    d2 = feet_place(scale_to_height(load_d2_idle(), 110))
    d5 = _ref_d5()
    d6 = _ref_d6()
    a, b, c = _d7("A"), _d7("B"), _d7("C")

    board(
        [
            ("D2", nearest_up(d2, 2)),
            ("D5", nearest_up(d5, 2)),
            ("D6", nearest_up(d6, 2)),
        ],
        CMP / "01_D2_D5_D6_comparacao.png",
        "D2 vs D5 vs D6 (acabamento anterior)",
    )

    board(
        [
            ("D7-A", nearest_up(a, 2)),
            ("D7-B", nearest_up(b, 2)),
            ("D7-C", nearest_up(c, 2)),
        ],
        CMP / "02_D7_tres_estrategias.png",
        "D7 tres estrategias (Marco A)",
    )

    board(
        [
            ("D2 game", nearest_up(scale_game(d2), 3)),
            ("A game", nearest_up(scale_game(a), 3)),
            ("B game", nearest_up(scale_game(b), 3)),
            ("C game", nearest_up(scale_game(c), 3)),
        ],
        CMP / "03_D7_identidade_escala_real.png",
        "Escala real ~52px (nearest 3x)",
    )

    board(
        [
            ("D2 3x", nearest_up(d2, 3)),
            ("B 3x", nearest_up(b, 3)),
            ("A 3x", nearest_up(a, 3)),
            ("C 3x", nearest_up(c, 3)),
        ],
        CMP / "04_D7_detalhes_personagem.png",
        "Detalhe nearest 3x",
        bg=(8, 8, 12),
    )

    # light bg readability
    board(
        [
            ("D2", nearest_up(d2, 2)),
            ("B", nearest_up(b, 2)),
            ("A", nearest_up(a, 2)),
            ("C", nearest_up(c, 2)),
        ],
        CMP / "04b_fundo_claro.png",
        "Fundo claro",
        bg=(210, 205, 195),
    )

    causeway_strip(
        [("D2", d2), ("D5", d5), ("D6", d6), ("A", a), ("B", b), ("C", c)],
        CMP / "08_D7_causeway.png",
    )

    # problems board: A geometric, C flat bands, B source-dependent
    board(
        [
            ("A geometric", nearest_up(a, 2)),
            ("C bands", nearest_up(c, 2)),
            ("B needs D2 art", nearest_up(b, 2)),
        ],
        CMP / "09_D7_problemas.png",
        "Problemas honestos Marco A",
    )

    board(
        [
            ("D2 ref", nearest_up(d2, 2)),
            ("selecionado B", nearest_up(b, 2)),
        ],
        CMP / "10_D7_resultado_final.png",
        "Marco A — candidato selecionado (B)",
    )

    print(f"Marco A boards -> {CMP}")


if __name__ == "__main__":
    main()
