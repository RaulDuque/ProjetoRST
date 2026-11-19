"""N4-1 (C1): the hand-marked skeletons of run_poses.py drawn over the
approved frames (evidence/n4c/n4c_skeletons.png). Red: near arm, orange:
far arm, light blue: near leg, blue: far leg, green: torso and head,
magenta: heels.

Usage (project root): ``python3 -B art-src/n4-sprite-machine/scripts/draw_skeletons.py``
"""

from __future__ import annotations

import sys
from pathlib import Path

from PIL import Image, ImageDraw

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
sys.path.insert(0, str(HERE))
import run_poses as R  # noqa: E402

K = 5


def main() -> None:
    run6 = Image.open(ROOT / "art-src/n3-6-animation-proof/generated/run6.png").convert("RGBA")
    tiles = []
    for i, f in R.FRAMES.items():
        fr = run6.crop((i * 112, 0, (i + 1) * 112, 128))
        bg = Image.new("RGBA", fr.size, (210, 210, 220, 255))
        bg.alpha_composite(fr)
        big = bg.resize((112 * K, 128 * K), Image.NEAREST).convert("RGB")
        d = ImageDraw.Draw(big)

        def p(q):
            return (q[0] * K + K // 2, q[1] * K + K // 2)

        for name, col in (("near_arm", (255, 40, 40)), ("far_arm", (255, 170, 40)),
                          ("near_leg", (40, 200, 255)), ("far_leg", (40, 90, 255))):
            d.line([p(q) for q in f[name]], fill=col, width=3)
            for q in f[name]:
                x, y = p(q)
                d.ellipse([x - 4, y - 4, x + 4, y + 4], outline=col, width=2)
        d.polygon([p(q) for q in f["torso"]], outline=(0, 200, 0))
        hx, hy = p(f["head"])
        r = f["head_r"] * K
        d.ellipse([hx - r, hy - r, hx + r, hy + r], outline=(0, 200, 0), width=2)
        for h in ("near_heel", "far_heel"):
            x, y = p(f[h])
            d.ellipse([x - 3, y - 3, x + 3, y + 3], fill=(255, 0, 255))
        d.text((15 * K + 4, 10 * K + 4), f"#{i} ({f['source']})", fill=(0, 0, 0))
        tiles.append(big.crop((15 * K, 10 * K, 100 * K, 122 * K)))
    out = Image.new("RGB", (sum(t.width for t in tiles), tiles[0].height), (255, 255, 255))
    x = 0
    for t in tiles:
        out.paste(t, (x, 0))
        x += t.width
    out.save(ROOT / "docs/development/n4/evidence/n4c/n4c_skeletons.png")


if __name__ == "__main__":
    main()
