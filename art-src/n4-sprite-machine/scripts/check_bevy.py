"""N4-A: compare, in the real Bevy captures, the original sprite (left) and
the composed layers (right). Same scale (4 px per texel), same pivot line,
same background, nearest sampling: any differing pixel is a composition
error. Writes ``bevy_pixel_check`` into docs/development/n4/evidence/metrics.json.

Usage: ``python3 -B art-src/n4-sprite-machine/scripts/check_bevy.py`` after
``cargo run --example n4_sprite_machine`` (and ``-- --run-c1`` for N4-1 C1,
written to docs/development/n4/evidence/n4c/metrics.json).
"""

from __future__ import annotations

import json
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parents[3]
EVIDENCE = ROOT / "docs/development/n4/evidence"
# Canvas on screen (1280x720, camera centred), 4 px per texel.
IDLE = ((260, 118, 580, 630), (700, 118, 1020, 630))  # 80x128, centres -220 / +220
RUN = ((176, 118, 624, 630), (656, 118, 1104, 630))  # 112x128 (N4-1 C1), centres -240 / +240


def check(name: str, boxes=IDLE, folder: Path = EVIDENCE) -> dict:
    im = Image.open(folder / f"{name}.png").convert("RGB")
    a, b = im.crop(boxes[0]), im.crop(boxes[1])
    pa, pb = a.load(), b.load()
    coords = [(x, y) for y in range(a.height) for x in range(a.width)]
    diff = sum(1 for c in coords if pa[c] != pb[c])
    bg = pa[0, 0]
    figure = sum(1 for c in coords if pa[c] != bg)
    return {"pixels": a.width * a.height, "figure_pixels": figure, "different": diff}


def main() -> None:
    metrics_path = EVIDENCE / "metrics.json"
    metrics = json.loads(metrics_path.read_text())
    metrics["bevy_pixel_check"] = {
        n: check(n)
        for n in ["bevy_n4a_original_vs_modular", "bevy_n4a_light_background", "bevy_n4a_mirrored"]
    }
    metrics_path.write_text(json.dumps(metrics, indent=2) + "\n")
    print(json.dumps(metrics["bevy_pixel_check"], indent=2))
    # N4-1 C1 (``-- --run-c1``): approved run6 frame 2k vs modular frame k.
    run_dir = EVIDENCE / "n4c"
    if (run_dir / "bevy_n4c_f0_dressed.png").exists():
        run_path = run_dir / "metrics.json"
        run_metrics = json.loads(run_path.read_text())
        run_metrics["bevy_pixel_check"] = {
            n: check(n, RUN, run_dir)
            for n in ["bevy_n4c_f0_dressed", "bevy_n4c_f2_dressed", "bevy_n4c_f4_dressed", "bevy_n4c_f2_mirrored"]
        }
        run_path.write_text(json.dumps(run_metrics, indent=2) + "\n")
        print(json.dumps(run_metrics["bevy_pixel_check"], indent=2))


if __name__ == "__main__":
    main()
