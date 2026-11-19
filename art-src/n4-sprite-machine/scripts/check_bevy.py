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
import re
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
    eq_dir = EVIDENCE / "n41a"
    if any(eq_dir.glob("bevy_n41a_*.png")):
        cov_path = eq_dir / "coverage.json"
        cov = json.loads(cov_path.read_text()) if cov_path.exists() else {}
        cov["bevy_pixel_check"] = check_equipment()
        cov_path.write_text(json.dumps(cov, indent=2) + "\n")
        print(json.dumps(cov["bevy_pixel_check"], indent=2))


def check_equipment() -> dict:
    """N4-1-A (``-- --equipment [--run-c1]``): the right canvas of every
    dark-background capture against the composition computed offline from
    the same data (build_equipment.compose: base layers + item, with the
    item's HIDE and the family order), scaled 4x, mirrored when needed."""
    import sys

    sys.path.insert(0, str(Path(__file__).resolve().parent))
    import build_equipment as eq  # noqa: E402

    out = {}
    folder = EVIDENCE / "n41a"
    for path in sorted(folder.glob("bevy_n41a_*.png")):
        name = path.stem
        if "_grid_" in name:
            continue  # light background and gizmos: not comparable
        run = name.startswith("bevy_n41a_run")
        clip = "run_c1" if run else "idle"
        frame = 0
        m = re.search(r"_f(\d)_", name)
        if m:
            frame = int(m.group(1)) // 2
        elif "mirrored" in name and run:
            frame = 2
        item_id = next((i for i in ("iron_cuirass", "blackened_breastplate") if name.endswith(i)), None)
        base = eq.base_layers(clip)
        if item_id:
            fam = json.loads((eq.AUTHORING / "families/torso_plate.json").read_text())
            recipe = json.loads((eq.AUTHORING / f"recipes/{item_id}.json").read_text())
            cells, region, neck = eq.family_masks(fam)[clip][frame]
            ic, ir = eq.item_cells(cells, region, neck, recipe)
            item = eq.paint(ic, recipe, ir)
            hides = eq.item_hides(item_id, "torso_plate").get(clip, [])
            expected = eq.compose(clip, frame, base, item, recipe, hides)
        else:
            expected = eq.compose(clip, frame, base, None, None, [])
        if "mirrored" in name:
            expected = expected.transpose(Image.FLIP_LEFT_RIGHT)
        bg = Image.new("RGBA", expected.size, (0, 0, 0, 255))
        im = Image.open(path).convert("RGB")
        box = RUN[1] if run else IDLE[1]
        shot = im.crop(box)
        dark = shot.getpixel((0, 0))
        bg = Image.new("RGBA", expected.size, dark + (255,))
        bg.alpha_composite(expected)
        want = bg.convert("RGB").resize(shot.size, Image.NEAREST)
        pa, pb = shot.load(), want.load()
        diff = sum(1 for y in range(shot.height) for x in range(shot.width) if pa[x, y] != pb[x, y])
        out[name] = {"pixels": shot.width * shot.height, "different": diff}
    return out


if __name__ == "__main__":
    main()
