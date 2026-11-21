"""Etapa 03D: take02 sheet asked by Prompt 03D take02 (after import_external.py,
take_report.py, restore_hair.py and compare_takes.py).

Puts side by side, at the game's real size (80x128, pivot 40,118):
  approved N3.5 | take01 final (N4 hair) | take02 raw read-back | take02 + N4 hair
at 1x, 2x and 8x (torso crop), plus a dark-ground preview, and the pixel difference of
every pair. Only reads existing run outputs; draws nothing new.

    python3 -I -B evidence/erro_corretion/etapa03d/scripts/compare_take02_sheet.py
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from compare_takes import DARK, IDLE, STAGE, TORSO, rgba, row, stack, zoom  # noqa: E402

RUNS = STAGE / "runs"
OUT = RUNS / "take02"


def differing(a, b) -> int:
    count = 0
    for y in range(a.height):
        for x in range(a.width):
            p, q = a.getpixel((x, y)), b.getpixel((x, y))
            count += not (p == q or (not p[3] and not q[3]))
    return count


def main() -> None:
    sprites = [
        ("aprovado N3.5", rgba(IDLE)),
        ("take01 final (cabelo N4)", rgba(RUNS / "take01/hair_restore/armour_hair_restored_80x128.png")),
        ("take02 bruto", rgba(RUNS / "take02/armour_on_canvas_80x128.png")),
        ("take02 + cabelo N4", rgba(RUNS / "take02/hair_restore/armour_hair_restored_80x128.png")),
    ]
    pairs = {}
    for i, (ta, a) in enumerate(sprites):
        for tb, b in sprites[i + 1:]:
            pairs[f"{ta} x {tb}"] = differing(a, b)
    short = ["N3.5", "take01", "t02 bruto", "t02+N4"]  # 1x tiles are 80 px wide
    rows = [row([(zoom(s, 1), short[i]) for i, (_, s) in enumerate(sprites)]),
            row([(zoom(s, 2), f"{t} 2x") for t, s in sprites]),
            row([(zoom(s.crop(TORSO), 8), f"{t} tronco 8x") for t, s in sprites])]
    notes = [f"{k}: {v} px diferentes" for k, v in pairs.items()]
    stack(rows, "Etapa 03D - take01 x take02 (80x128, pivo 40,118)", notes).save(
        OUT / "takes_01_vs_02_comparison.png")
    dark = [row([(zoom(s, 1, DARK), short[i]) for i, (_, s) in enumerate(sprites)]),
            row([(zoom(s, 3, DARK), f"{t} 3x") for t, s in sprites])]
    stack(dark, "Etapa 03D - take01 x take02 sobre fundo escuro", []).save(OUT / "takes_dark_preview.png")
    (OUT / "takes_01_vs_02_pairs.json").write_text(json.dumps(pairs, indent=1, ensure_ascii=False))
    print(json.dumps(pairs, indent=1, ensure_ascii=False))


if __name__ == "__main__":
    main()
