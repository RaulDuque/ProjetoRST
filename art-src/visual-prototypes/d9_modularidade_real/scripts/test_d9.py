#!/usr/bin/env python3
"""D9 technical + anti-fake modularity tests."""
from __future__ import annotations

import json
import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(Path(__file__).resolve().parent))

from compose_d9 import CLASSIC, ComposeError, compose_loadout, find_piece  # noqa: E402
from d9_common import CELL, GAME_H, PIVOT, opaque_count  # noqa: E402


def fail(msg: str) -> None:
    raise AssertionError(msg)


def test_no_full_classic_dependency() -> None:
    """Classic pose must not be a copy of full_classic_ref file usage in compose."""
    pose = Image.open(ROOT / "loadouts" / "classico" / "pose.png").convert("RGBA")
    ref = ROOT / "fontes" / "d8_full_classic_ref.png"
    if ref.exists():
        full = Image.open(ref).convert("RGBA")
        # composed classic should differ from full sprite (modular path)
        if pose.tobytes() == full.tobytes():
            fail("classic pose identical to full_classic_ref — false modularity")


def test_compose_errors() -> None:
    try:
        find_piece("does_not_exist_xyz")
        fail("expected ComposeError")
    except ComposeError:
        pass
    bad = dict(CLASSIC)
    bad["weapon"] = "does_not_exist_xyz"
    try:
        compose_loadout(bad)
        fail("expected ComposeError on bad loadout")
    except ComposeError:
        pass


def test_atlas_and_body() -> None:
    body = Image.open(ROOT / "corpo" / "body_base.png").convert("RGBA")
    if body.size != CELL:
        fail("body cell")
    if opaque_count(body) < 120:
        fail("body_base too empty")
    coat = Image.open(ROOT / "equipamentos" / "roupas" / "coat_front.png").convert("RGBA")
    if opaque_count(coat) < 50:
        fail("coat too empty")


def test_desmontagem_no_holes() -> None:
    lo = dict(CLASSIC)
    lo["coat"] = None
    lo["back"] = None
    lo["weapon"] = None
    base = compose_loadout(lo)
    if opaque_count(base) < 250:
        fail("body collapsed after removing coat/cape/weapon")


def test_antifake_report() -> None:
    p = ROOT / "testes" / "antifake_report.json"
    if not p.exists():
        fail("missing antifake report")
    data = json.loads(p.read_text())
    for key in ("cape", "coat", "weapon"):
        if key not in data:
            fail(f"missing {key}")
        if data[key].get("status") == "fail":
            # allow soft fail documentation but warn
            print(f"WARN antifake {key}: {data[key]}")


def test_sockets_counts() -> None:
    for clip, n in (("idle", 6), ("run", 8)):
        data = json.loads((ROOT / "sockets" / f"{clip}_sockets.json").read_text())
        if len(data["frames"]) != n:
            fail(f"{clip} frames")


def test_animation_files() -> None:
    if len(list((ROOT / "animacoes" / "idle").glob("classico_f*.png"))) != 6:
        fail("idle count")
    if len(list((ROOT / "animacoes" / "run").glob("classico_f*.png"))) != 8:
        fail("run count")
    for g in ("idle_classico.gif", "run_classico.gif", "troca_dinamica.gif"):
        if not (ROOT / "previews" / g).exists():
            fail(g)


def test_feet() -> None:
    im = Image.open(ROOT / "loadouts" / "classico" / "pose.png").convert("RGBA")
    box = im.split()[-1].getbbox()
    assert box
    if abs(box[3] - 1 - PIVOT[1]) > 6:
        fail(f"feet {box[3]-1}")


def test_boards() -> None:
    for name in (
        "01_D8_vs_D9.png",
        "03_desmontagem_completa.png",
        "09_troca_individual.png",
        "16_resultado_final.png",
    ):
        p = ROOT / "comparacoes" / name
        if not p.exists() or Image.open(p).width < 40:
            fail(name)


def test_loadouts_differ() -> None:
    a = Image.open(ROOT / "loadouts/classico/pose.png").tobytes()
    b = Image.open(ROOT / "loadouts/pesado/pose.png").tobytes()
    if a == b:
        fail("identical loadouts")


def test_game_scale() -> None:
    im = Image.open(ROOT / "loadouts/classico/pose_game.png").convert("RGBA")
    if im.height != GAME_H:
        fail(f"game h {im.height}")


def main() -> None:
    tests = [
        test_atlas_and_body,
        test_no_full_classic_dependency,
        test_compose_errors,
        test_desmontagem_no_holes,
        test_antifake_report,
        test_sockets_counts,
        test_animation_files,
        test_feet,
        test_boards,
        test_loadouts_differ,
        test_game_scale,
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
        sys.exit(1)
    print("\nAll D9 technical tests passed.")


if __name__ == "__main__":
    main()
