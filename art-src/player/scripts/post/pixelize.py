"""Turns a high-resolution RGBA render into the two candidate finishes.

Works on any RGBA master whose side is a multiple of the 64x64 frame: the
vector stand-in today, a Blender render (``render.py``) once Blender exists.

- Finish A (pixel art): each block of the master becomes one pixel with the
  dominant palette colour of its opaque area; alpha is binary; an optional
  1 px outline is added. The result still needs manual cleanup by an artist
  (details smaller than a block, such as the eye, may vanish or flicker
  between frames).
- Finish B (high resolution): Lanczos downscale to 2x the frame density
  (128x128), soft alpha, optional soft rim.

Usage:
    python3 art-src/player/scripts/post/pixelize.py MASTER.png --out-dir DIR
"""

from __future__ import annotations

import argparse
from collections import Counter
from pathlib import Path

from PIL import Image, ImageFilter

from contract import FRAME, HD_DENSITY
from palette import CHARACTER_RGB
COVERAGE_THRESHOLD = 0.4  # opaque fraction of a block needed to emit a pixel
ALPHA_OPAQUE = 128


def nearest_palette(rgb: tuple[int, int, int]) -> tuple[int, int, int]:
    return min(
        CHARACTER_RGB.values(),
        key=lambda p: (p[0] - rgb[0]) ** 2 + (p[1] - rgb[1]) ** 2 + (p[2] - rgb[2]) ** 2,
    )


def pixel_art(master: Image.Image, frame: int = FRAME) -> Image.Image:
    """Finish A without outline: dominant colour per block, binary alpha.

    Each opaque pixel is first snapped to the palette and then counted, so
    shaded, anti-aliased renders (every pixel unique) still yield the block's
    dominant palette colour instead of an arbitrary pixel.
    """
    master = master.convert("RGBA")
    block = master.size[0] // frame
    if master.size != (frame * block, frame * block):
        raise ValueError(f"master must be a square multiple of {frame}, got {master.size}")
    pixels = master.load()
    out = Image.new("RGBA", (frame, frame), (0, 0, 0, 0))
    out_px = out.load()
    cache: dict[tuple[int, int, int], tuple[int, int, int]] = {}
    area = block * block
    for fy in range(frame):
        for fx in range(frame):
            counts: Counter[tuple[int, int, int]] = Counter()
            opaque = 0
            for y in range(fy * block, (fy + 1) * block):
                for x in range(fx * block, (fx + 1) * block):
                    r, g, b, a = pixels[x, y]
                    if a >= ALPHA_OPAQUE:
                        opaque += 1
                        rgb = (r, g, b)
                        if rgb not in cache:
                            cache[rgb] = nearest_palette(rgb)
                        counts[cache[rgb]] += 1
            if opaque / area < COVERAGE_THRESHOLD:
                continue
            out_px[fx, fy] = (*counts.most_common(1)[0][0], 255)
    return out


def add_outline(sprite: Image.Image, color: tuple[int, int, int]) -> Image.Image:
    """1 px outline on transparent pixels touching the silhouette (4-neighbour)."""
    src = sprite.load()
    out = sprite.copy()
    dst = out.load()
    w, h = sprite.size
    for y in range(h):
        for x in range(w):
            if src[x, y][3] != 0:
                continue
            for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                nx, ny = x + dx, y + dy
                if 0 <= nx < w and 0 <= ny < h and src[nx, ny][3] != 0:
                    dst[x, y] = (*color, 255)
                    break
    return out


def high_res(master: Image.Image, frame: int = FRAME) -> Image.Image:
    """Finish B without rim: Lanczos downscale to HD_DENSITY x the frame."""
    size = frame * HD_DENSITY
    return master.convert("RGBA").resize((size, size), Image.Resampling.LANCZOS)


def add_soft_rim(sprite: Image.Image, color: tuple[int, int, int]) -> Image.Image:
    """Soft rim behind the sprite: dilated, blurred alpha in the rim colour.

    Known limitation: the sole sits 2 texels above the cell's bottom edge, so
    the rim is cut flat under the feet (see docs/ASSET_CONTRACT.md, section 5).
    """
    alpha = sprite.getchannel("A").filter(ImageFilter.MaxFilter(5))
    alpha = alpha.filter(ImageFilter.GaussianBlur(1.2))
    rim = Image.new("RGBA", sprite.size, (*color, 0))
    rim.putalpha(alpha)
    rim.alpha_composite(sprite)
    return rim


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("master", type=Path)
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument("--name", default="swordsman_idle_side")
    args = parser.parse_args()

    master = Image.open(args.master)
    rim = CHARACTER_RGB["rim"]
    args.out_dir.mkdir(parents=True, exist_ok=True)
    outputs = {
        f"{args.name}_A_pixel_raw.png": pixel_art(master),
        f"{args.name}_B_hd_raw.png": high_res(master),
    }
    outputs[f"{args.name}_A_pixel_rim.png"] = add_outline(outputs[f"{args.name}_A_pixel_raw.png"], rim)
    outputs[f"{args.name}_B_hd_rim.png"] = add_soft_rim(outputs[f"{args.name}_B_hd_raw.png"], rim)
    for name, image in outputs.items():
        path = args.out_dir / name
        image.save(path)
        print(f"[pixelize] {path} {image.size[0]}x{image.size[1]}")


if __name__ == "__main__":
    main()
