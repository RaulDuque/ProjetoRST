#!/usr/bin/env python3
"""Automated checks for D12 lab outputs."""
from __future__ import annotations

import json
import sys
from pathlib import Path

from PIL import Image

REPO = Path(__file__).resolve().parents[4]
LAB = Path(__file__).resolve().parents[1]
ASSETS = REPO / "assets/experimental/d12"
CMP = LAB / "comparacoes"


def fail(msg: str) -> None:
    print("FAIL", msg)
    sys.exit(1)


def main() -> None:
    contract = json.loads((ASSETS / "contract.json").read_text(encoding="utf-8"))
    assert contract["world_height"] == 64.0, contract
    assert contract["cell_px"] == [128, 160]
    assert contract["pivot_sole_px"] == [64, 156]

    required = [
        "01_D11_diagnostico.png",
        "02_escala_e_pivos.png",
        "03_silhueta.png",
        "04_contraste_A_B_C.png",
        "05_espada_e_maos.png",
        "06_alfa_e_contornos.png",
        "09_pipeline_A_B_C.png",
        "10_classico_refinado.png",
        "11_pesado_refinado.png",
        "12_sombra_contato.png",
        "13_after_image_B.png",
        "14_idle_run.png",
        "15_causeway_escala_real.png",
        "16_resultado_final.png",
    ]
    for name in required:
        p = CMP / name
        if not p.exists():
            fail(f"missing evidence {name}")

    for loadout, clip, n in (
        ("classico", "idle", 6),
        ("classico", "run", 8),
        ("pesado", "idle", 6),
        ("pesado", "run", 8),
    ):
        for i in range(n):
            path = ASSETS / "frames" / loadout / clip / f"f{i:02d}.png"
            if not path.exists():
                fail(f"missing frame {path}")
            im = Image.open(path)
            if im.size != (128, 160):
                fail(f"bad size {path} {im.size}")

    # Default visual untouched
    player_ron = REPO / "assets/animations/player.ron"
    if not player_ron.exists():
        fail("player.ron missing")

    results = LAB / "testes/test_results.json"
    if results.exists():
        data = json.loads(results.read_text(encoding="utf-8"))
        if not data.get("ok"):
            fail(f"pipeline failures {data.get('failures')}")

    print("OK  test_d12")


if __name__ == "__main__":
    main()
