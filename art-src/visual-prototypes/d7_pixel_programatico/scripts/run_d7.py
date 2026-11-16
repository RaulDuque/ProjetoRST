#!/usr/bin/env python3
"""Run full D7 lab: A/B/C → Marco A boards → Marco B → tests."""
from __future__ import annotations

import subprocess
import sys
from pathlib import Path

SCRIPTS = Path(__file__).resolve().parent


def run(name: str) -> None:
    print(f"\n=== {name} ===")
    r = subprocess.run([sys.executable, str(SCRIPTS / name)], check=False)
    if r.returncode != 0:
        raise SystemExit(f"{name} failed with {r.returncode}")


def main() -> None:
    run("draw_A_primitivas.py")
    run("draw_B_sprites_masks.py")
    run("draw_C_hibrido.py")
    run("compose_marco_a.py")
    # Marco B: B candidate is visually nearest to D2 (source masks)
    run("compose_marco_b.py")
    run("test_d7.py")
    print("\nD7 pipeline finished.")


if __name__ == "__main__":
    main()
