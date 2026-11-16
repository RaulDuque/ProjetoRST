#!/usr/bin/env python3
"""Automated technical tests for D7 lab (not artistic approval)."""
from __future__ import annotations

import hashlib
import sys
import tempfile
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
SCRIPTS = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPTS))

from d7_common import CELL, GAME_H, PIVOT, scale_game  # noqa: E402


def _fail(msg: str) -> None:
    raise AssertionError(msg)


def test_pngs_exist() -> None:
    required = [
        ROOT / "estrategias" / "A_primitivas" / "pose_estatica.png",
        ROOT / "estrategias" / "B_sprites_masks" / "pose_estatica.png",
        ROOT / "estrategias" / "C_hibrido" / "pose_estatica.png",
        ROOT / "comparacoes" / "02_D7_tres_estrategias.png",
        ROOT / "comparacoes" / "03_D7_identidade_escala_real.png",
    ]
    for p in required:
        if not p.exists():
            _fail(f"missing {p}")


def test_dimensions_and_alpha() -> None:
    for letter, folder in (
        ("A", "A_primitivas"),
        ("B", "B_sprites_masks"),
        ("C", "C_hibrido"),
    ):
        p = ROOT / "estrategias" / folder / "pose_estatica.png"
        im = Image.open(p).convert("RGBA")
        if im.size != CELL:
            _fail(f"{letter} cell size {im.size} != {CELL}")
        alpha = im.split()[-1]
        if alpha.getbbox() is None:
            _fail(f"{letter} fully transparent")
        # no solid magenta background flood
        px = im.load()
        mag = 0
        for y in range(im.height):
            for x in range(im.width):
                r, g, b, a = px[x, y]
                if a > 200 and r > 200 and g < 40 and b > 150:
                    mag += 1
        if mag > 40:
            _fail(f"{letter} looks like magenta matte ({mag} px)")


def test_game_scale_height() -> None:
    for folder in ("A_primitivas", "B_sprites_masks", "C_hibrido"):
        p = ROOT / "estrategias" / folder / "pose_escala_jogo.png"
        im = Image.open(p).convert("RGBA")
        box = im.split()[-1].getbbox()
        if box is None:
            _fail(f"{folder} game scale empty")
        h = box[3] - box[1]
        if abs(h - GAME_H) > 2 and im.height != GAME_H:
            # file may be tightly cropped to GAME_H
            if im.height != GAME_H:
                _fail(f"{folder} game h={im.height}/{h} expected ~{GAME_H}")


def test_feet_near_pivot() -> None:
    for folder in ("A_primitivas", "B_sprites_masks", "C_hibrido"):
        im = Image.open(ROOT / "estrategias" / folder / "pose_estatica.png").convert("RGBA")
        box = im.split()[-1].getbbox()
        assert box
        bottom = box[3] - 1
        if abs(bottom - PIVOT[1]) > 3:
            _fail(f"{folder} feet y={bottom} pivot={PIVOT[1]}")


def test_no_empty_boards() -> None:
    for name in (
        "01_D2_D5_D6_comparacao.png",
        "02_D7_tres_estrategias.png",
        "03_D7_identidade_escala_real.png",
        "10_D7_resultado_final.png",
    ):
        p = ROOT / "comparacoes" / name
        im = Image.open(p)
        if im.width < 40 or im.height < 40:
            _fail(f"tiny board {name}")


def test_deterministic_A() -> None:
    """Regenerate A into temp and compare hash of pose."""
    import draw_A_primitivas as A

    a1 = A.draw_pose()
    a2 = A.draw_pose()
    if hashlib.md5(a1.tobytes()).hexdigest() != hashlib.md5(a2.tobytes()).hexdigest():
        _fail("A not deterministic")


def test_marco_b_if_present() -> None:
    leve = ROOT / "loadouts" / "leve" / "pose.png"
    pes = ROOT / "loadouts" / "pesado" / "pose.png"
    if not leve.exists():
        print("SKIP marco B loadouts (not generated)")
        return
    im_l = Image.open(leve).convert("RGBA")
    im_p = Image.open(pes).convert("RGBA")
    if im_l.size != CELL or im_p.size != CELL:
        _fail("loadout cell mismatch")
    # silhouettes should differ
    if hashlib.md5(im_l.tobytes()).hexdigest() == hashlib.md5(im_p.tobytes()).hexdigest():
        _fail("leve and pesado identical")
    manifest = ROOT / "personagem" / "layers_manifest.json"
    if not manifest.exists():
        _fail("missing layers_manifest.json")
    # run frames
    run0 = ROOT / "animacoes" / "run" / "leve_f00.png"
    if not run0.exists():
        _fail("missing run pilot frame")


def main() -> None:
    tests = [
        test_pngs_exist,
        test_dimensions_and_alpha,
        test_game_scale_height,
        test_feet_near_pivot,
        test_no_empty_boards,
        test_deterministic_A,
        test_marco_b_if_present,
    ]
    failed = 0
    for t in tests:
        try:
            t()
            print(f"OK  {t.__name__}")
        except Exception as e:
            failed += 1
            print(f"FAIL {t.__name__}: {e}")
    if failed:
        print(f"\n{failed} failed")
        sys.exit(1)
    print("\nAll D7 technical tests passed (art not judged).")


if __name__ == "__main__":
    main()
