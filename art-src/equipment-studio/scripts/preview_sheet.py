"""Sheet of `cargo run --example equipment_preview -- NAME ...` (offline tool).

    python3 -B art-src/equipment-studio/scripts/preview_sheet.py NAME [NAME ...] [--scale 3] [--out file.png]

One row per NAME: idle 0-1 and run 0-5 at the given integer scale (nearest,
no smoothing), plus a 1x strip under each row. Writes
`target/equipment_preview/<first NAME>_sheet.png` unless `--out` is given.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[3]
SRC = ROOT / "target/equipment_preview"
BG = (36, 34, 44)
INK = (240, 205, 120)
ORDER = [("idle", 0), ("idle", 1)] + [("run", i) for i in range(6)]


def frame(name: str, clip: str, f: int) -> Image.Image:
    hits = sorted(SRC.glob(f"{name}_{clip}_f{f}_*x*.rgba"))
    if not hits:
        raise SystemExit(f"missing {name} {clip} #{f}")
    w, h = map(int, re.search(r"_(\d+)x(\d+)\.rgba$", hits[0].name).groups())
    return Image.frombytes("RGBA", (w, h), hits[0].read_bytes())


def on_bg(img: Image.Image, scale: int) -> Image.Image:
    img = img.resize((img.width * scale, img.height * scale), Image.NEAREST)
    out = Image.new("RGB", img.size, BG)
    out.paste(img, (0, 0), img)
    return out


def row(names_frames: list[Image.Image], gap: int = 4) -> Image.Image:
    w = sum(i.width for i in names_frames) + gap * (len(names_frames) - 1)
    h = max(i.height for i in names_frames)
    out = Image.new("RGB", (w, h), BG)
    x = 0
    for i in names_frames:
        out.paste(i, (x, h - i.height))
        x += i.width + gap
    return out


def sheet(names: list[str], scale: int) -> Image.Image:
    rows = []
    for n in names:
        frames = [frame(n, c, f) for c, f in ORDER]
        big = row([on_bg(f, scale) for f in frames])
        small = row([on_bg(f, 1) for f in frames], 8)
        block = Image.new("RGB", (max(big.width, small.width), big.height + small.height + 26), BG)
        ImageDraw.Draw(block).text((4, 2), n, fill=INK)
        block.paste(big, (0, 16))
        block.paste(small, (0, 22 + big.height))
        rows.append(block)
    w = max(r.width for r in rows)
    out = Image.new("RGB", (w, sum(r.height for r in rows)), BG)
    y = 0
    for r in rows:
        out.paste(r, (0, y))
        y += r.height
    return out


def main() -> None:
    args = sys.argv[1:]
    scale = 3
    out = None
    if "--scale" in args:
        i = args.index("--scale")
        scale = int(args[i + 1])
        del args[i:i + 2]
    if "--out" in args:
        i = args.index("--out")
        out = Path(args[i + 1])
        del args[i:i + 2]
    if not args:
        raise SystemExit(__doc__)
    img = sheet(args, scale)
    path = out or SRC / f"{args[0]}_sheet.png"
    img.save(path)
    print(path)


if __name__ == "__main__":
    main()
