#!/usr/bin/env python3
import subprocess, sys
from pathlib import Path
S = Path(__file__).resolve().parent

def run(n):
    print(f"\n=== {n} ===")
    r = subprocess.run([sys.executable, str(S / n)])
    if r.returncode:
        raise SystemExit(r.returncode)

if __name__ == "__main__":
    run("build_d10.py")
    run("test_d10.py")
    print("\nD10 finished.")
