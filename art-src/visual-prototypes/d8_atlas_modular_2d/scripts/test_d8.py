#!/usr/bin/env python3
"""Technical tests for D8 lab (not artistic approval)."""
from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(Path(__file__).resolve().parent))
from d8_common import CELL, GAME_H, PIVOT  # noqa: E402


def fail(msg: str) -> None:
    raise AssertionError(msg)


def test_atlas_layers() -> None:
    required = [
        "body",
        "hair_front",
        "face",
        "shirt",
        "coat_front",
        "pants",
        "boots",
        "cape_back",
        "weapon",
        "full_classic_ref",
    ]
    for name in required:
        p = ROOT / "atlas" / f"{name}.png"
        if not p.exists():
            fail(f"missing atlas layer {name}")
        im = Image.open(p).convert("RGBA")
        if im.size != CELL:
            fail(f"{name} size {im.size}")


def test_no_magenta_flood() -> None:
    for folder in (ROOT / "atlas", ROOT / "equipamentos", ROOT / "loadouts" / "classico"):
        for p in folder.glob("*.png"):
            im = Image.open(p).convert("RGBA")
            n = 0
            for r, g, b, a in im.getdata():
                if a > 200 and r > 200 and g < 40 and b > 150:
                    n += 1
            if n > 80:
                fail(f"magenta flood in {p.name}: {n}")


def test_sockets() -> None:
    p = ROOT / "sockets" / "sockets_lab.json"
    if not p.exists():
        fail("missing sockets_lab.json")
    data = json.loads(p.read_text())
    for clip in ("idle", "run"):
        frames = data["clips"].get(clip, [])
        expect = 6 if clip == "idle" else 8
        if len(frames) != expect:
            fail(f"{clip} sockets frames {len(frames)} != {expect}")
        s0 = frames[0]["sockets"]
        for key in ("head", "hand_front", "foot_front", "back", "weapon"):
            if key not in s0:
                fail(f"missing socket {key}")


def test_loadouts() -> None:
    for name in ("classico", "pesado"):
        p = ROOT / "loadouts" / name / "pose.png"
        im = Image.open(p).convert("RGBA")
        if im.size != CELL:
            fail(f"{name} cell")
        if im.split()[-1].getbbox() is None:
            fail(f"{name} empty")
    h1 = hashlib.md5(Image.open(ROOT / "loadouts/classico/pose.png").tobytes()).hexdigest()
    h2 = hashlib.md5(Image.open(ROOT / "loadouts/pesado/pose.png").tobytes()).hexdigest()
    if h1 == h2:
        fail("loadouts identical")


def test_animation_counts() -> None:
    idle = list((ROOT / "animacoes" / "idle").glob("classico_f*.png"))
    run = list((ROOT / "animacoes" / "run").glob("classico_f*.png"))
    if len(idle) != 6:
        fail(f"idle count {len(idle)}")
    if len(run) != 8:
        fail(f"run count {len(run)}")


def test_feet_pivot() -> None:
    im = Image.open(ROOT / "loadouts" / "classico" / "pose.png").convert("RGBA")
    box = im.split()[-1].getbbox()
    assert box
    if abs(box[3] - 1 - PIVOT[1]) > 4:
        fail(f"feet {box[3]-1} vs pivot {PIVOT[1]}")


def test_game_scale() -> None:
    p = ROOT / "loadouts" / "classico" / "pose_game.png"
    im = Image.open(p).convert("RGBA")
    if im.height != GAME_H and (im.split()[-1].getbbox() or (0, 0, 0, 0))[3] - (
        im.split()[-1].getbbox() or (0, 0, 0, 0)
    )[1] not in range(GAME_H - 2, GAME_H + 3):
        if im.height != GAME_H:
            fail(f"game scale h={im.height}")


def test_boards_and_gifs() -> None:
    for name in (
        "01_D7_vs_D8.png",
        "02_atlas_limpo.png",
        "04_sockets.png",
        "05_loadout_classico.png",
        "06_loadout_pesado.png",
        "12_resultado_final.png",
    ):
        p = ROOT / "comparacoes" / name
        if not p.exists() or Image.open(p).width < 40:
            fail(f"board {name}")
    for name in ("idle_classico.gif", "run_classico.gif", "run_pesado.gif"):
        if not (ROOT / "previews" / name).exists():
            fail(f"gif {name}")


def test_manifest() -> None:
    p = ROOT / "atlas" / "atlas_manifest.json"
    data = json.loads(p.read_text())
    if not data.get("layers"):
        fail("empty manifest")


def main() -> None:
    tests = [
        test_atlas_layers,
        test_no_magenta_flood,
        test_sockets,
        test_loadouts,
        test_animation_counts,
        test_feet_pivot,
        test_game_scale,
        test_boards_and_gifs,
        test_manifest,
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
    print("\nAll D8 technical tests passed.")


if __name__ == "__main__":
    main()
