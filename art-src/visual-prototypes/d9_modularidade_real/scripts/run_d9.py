#!/usr/bin/env python3
"""Run D9 lab pipeline."""
from __future__ import annotations

import subprocess
import sys
from pathlib import Path

SCRIPTS = Path(__file__).resolve().parent


def run(name: str) -> None:
    print(f"\n=== {name} ===")
    r = subprocess.run([sys.executable, str(SCRIPTS / name)], check=False)
    if r.returncode != 0:
        raise SystemExit(f"{name} failed ({r.returncode})")


def main() -> None:
    run("build_layers.py")
    run("compose_d9.py")
    run("test_d9.py")
    print("\nD9 pipeline finished.")


if __name__ == "__main__":
    main()
