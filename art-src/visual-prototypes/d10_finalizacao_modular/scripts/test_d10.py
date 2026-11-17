#!/usr/bin/env python3
"""D10 technical tests."""
from __future__ import annotations

import json
import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(Path(__file__).resolve().parent))
from build_d10 import CLASSIC, ComposeError, compose_loadout, find_piece  # noqa: E402
from d10_common import CELL, GAME_H, PIVOT, opaque_count  # noqa: E402


def fail(m):
    raise AssertionError(m)


def test_antifake_cape_gloves():
    data = json.loads((ROOT / "testes" / "antifake_report.json").read_text())
    for key in ("cape", "gloves", "coat", "weapon", "boots"):
        if key not in data:
            fail(f"missing {key}")
        if data[key]["status"] == "fail":
            fail(f"{key} failed antifake: {data[key]}")
        if key in ("cape", "gloves") and data[key]["status"] != "pass":
            fail(f"{key} must pass in D10, got {data[key]}")


def test_no_full_ref():
    # compose path never loads full_classic_ref filename
    pose = Image.open(ROOT / "loadouts/classico/pose.png").convert("RGBA")
    ref = ROOT / "fontes" / "d8_full_classic_ref.png"
    if ref.exists() and pose.tobytes() == Image.open(ref).convert("RGBA").tobytes():
        fail("identical to full classic ref")


def test_errors():
    try:
        find_piece("nope_xyz")
        fail("expected error")
    except ComposeError:
        pass


def test_dims():
    for p in (ROOT / "loadouts/classico/pose.png", ROOT / "corpo/body_base.png"):
        im = Image.open(p).convert("RGBA")
        if im.size != CELL:
            fail(str(p))
    if opaque_count(Image.open(ROOT / "corpo/body_base.png").convert("RGBA")) < 100:
        fail("body empty")


def test_anim():
    if len(list((ROOT / "animacoes/idle").glob("classico_f*.png"))) != 6:
        fail("idle")
    if len(list((ROOT / "animacoes/run").glob("classico_f*.png"))) != 8:
        fail("run")
    for g in ("idle_classico.gif", "run_classico.gif", "after_image_A.gif", "troca_dinamica.gif"):
        if not (ROOT / "previews" / g).exists():
            fail(g)


def test_boards():
    for n in ("03_capa_independente.png", "04_luvas_independentes.png", "14_resultado_final.png"):
        if not (ROOT / "comparacoes" / n).exists():
            fail(n)


def test_feet():
    im = Image.open(ROOT / "loadouts/classico/pose.png").convert("RGBA")
    box = im.split()[-1].getbbox()
    assert box
    if abs(box[3] - 1 - PIVOT[1]) > 6:
        fail("feet")


def test_game():
    im = Image.open(ROOT / "loadouts/classico/pose_game.png")
    if im.height != GAME_H:
        fail(f"h={im.height}")


def main():
    failed = 0
    for t in (
        test_antifake_cape_gloves,
        test_no_full_ref,
        test_errors,
        test_dims,
        test_anim,
        test_boards,
        test_feet,
        test_game,
    ):
        try:
            t()
            print(f"OK  {t.__name__}")
        except Exception as e:
            failed += 1
            print(f"FAIL {t.__name__}: {e}")
    if failed:
        sys.exit(1)
    print("\nAll D10 tests passed.")


if __name__ == "__main__":
    main()
