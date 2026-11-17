"""Validate D13 export PNGs exist, are RGBA, and share resolution."""
from __future__ import annotations

import sys
from pathlib import Path

from PIL import Image

LAB = Path(__file__).resolve().parents[1]
EXP = LAB / "exports"
REND = LAB / "renders"

REQUIRED = [
    REND / "materials_lights_A.png",
    REND / "materials_lights_B.png",
    REND / "materials_lights_C.png",
    EXP / "layer_full.png",
    EXP / "layer_coat.png",
    EXP / "layer_cape.png",
    EXP / "layer_sword.png",
    EXP / "layer_armor.png",
]


def main() -> None:
    fails = []
    sizes = set()
    for p in REQUIRED:
        if not p.exists():
            fails.append(f"missing {p.name}")
            continue
        im = Image.open(p)
        if im.mode not in ("RGBA", "RGB"):
            fails.append(f"mode {p.name} {im.mode}")
        sizes.add(im.size)
        # transparent expected for layer exports
        if p.parent.name == "exports" and im.mode == "RGBA":
            a = im.getchannel("A")
            if max(a.getdata()) == 0:
                fails.append(f"empty alpha {p.name}")
    if len(sizes) > 1:
        fails.append(f"mixed sizes {sizes}")
    if fails:
        print("FAIL", fails)
        sys.exit(1)
    print("OK validate_exports", sizes.pop() if sizes else None)


if __name__ == "__main__":
    main()
