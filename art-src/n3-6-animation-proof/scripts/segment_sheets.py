"""N3.6: find and number every pose on the owner's reference sheets.

Reads (never writes) the sheets in `Md-usario/assets-user/prota/base/`,
separates poses from the flat background by connected components on a
quarter-size mask and numbers them in reading order (rows top to bottom,
then left to right). Writes `analysis/poses.json` with the sheet-pixel box of
each pose; the analysis documents cite poses as `<sheet>#<number>`.

    python3 art-src/n3-6-animation-proof/scripts/segment_sheets.py
"""

from __future__ import annotations

import json
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parents[3]
BASE = ROOT / "Md-usario/assets-user/prota/base"
OUT = ROOT / "art-src/n3-6-animation-proof/analysis"
SHEETS = {
    "run_a": "Gemini_Generated_Image_15q6g515q6g515q6.jpg",
    "run_jump": "Gemini_Generated_Image_gm471hgm471hgm47.jpg",
    "attack": "Gemini_Generated_Image_p5njkap5njkap5nj.jpg",
    "idle_run_jump": "Gemini_Generated_Image_xwfavvxwfavvxwfa.jpg",
    "fight_left": "al.jpg",
}
STEP = 4
# Background distance (sum of channel differences) above which a pixel is art.
THRESHOLD = 90
# Components smaller than this many mask cells are noise or pivot markers.
MIN_CELLS = 400
MARGIN = 6


def background(image: Image.Image) -> tuple[int, int, int]:
    corners = [image.getpixel((5, 5)), image.getpixel((image.width - 6, 5)),
               image.getpixel((5, image.height - 6)), image.getpixel((image.width - 6, image.height - 6))]
    return tuple(sorted(c[i] for c in corners)[1] for i in range(3))


def is_art(pixel: tuple[int, int, int], bg: tuple[int, int, int], green_bg: bool) -> bool:
    if green_bg:
        r, g, b = pixel
        # The `al.jpg` pivot markers are saturated green lines: not art.
        if g > r + 60 and g > b + 60:
            return False
    return sum(abs(a - b) for a, b in zip(pixel, bg, strict=True)) > THRESHOLD


def components(image: Image.Image, green_bg: bool) -> list[tuple[int, int, int, int]]:
    small = image.resize((image.width // STEP, image.height // STEP), Image.Resampling.BOX)
    bg = background(small)
    width, height = small.size
    px = small.load()
    mask = [[is_art(px[x, y], bg, green_bg) for x in range(width)] for y in range(height)]
    seen = [[False] * width for _ in range(height)]
    boxes = []
    for y0 in range(height):
        for x0 in range(width):
            if not mask[y0][x0] or seen[y0][x0]:
                continue
            stack, cells = [(x0, y0)], 0
            seen[y0][x0] = True
            x_min = x_max = x0
            y_min = y_max = y0
            while stack:
                x, y = stack.pop()
                cells += 1
                x_min, x_max, y_min, y_max = min(x_min, x), max(x_max, x), min(y_min, y), max(y_max, y)
                for dx in (-2, -1, 0, 1, 2):
                    for dy in (-2, -1, 0, 1, 2):
                        nx, ny = x + dx, y + dy
                        if 0 <= nx < width and 0 <= ny < height and mask[ny][nx] and not seen[ny][nx]:
                            seen[ny][nx] = True
                            stack.append((nx, ny))
            if cells >= MIN_CELLS:
                boxes.append((max(0, x_min * STEP - MARGIN), max(0, y_min * STEP - MARGIN),
                              min(image.width, (x_max + 1) * STEP + MARGIN),
                              min(image.height, (y_max + 1) * STEP + MARGIN)))
    return boxes


def reading_order(boxes: list[tuple[int, int, int, int]]) -> list[tuple[int, int, int, int]]:
    """Group boxes into rows by vertical overlap, then sort each row by x."""
    rows: list[list[tuple[int, int, int, int]]] = []
    for box in sorted(boxes, key=lambda b: (b[1] + b[3]) / 2):
        centre = (box[1] + box[3]) / 2
        for row in rows:
            top = min(b[1] for b in row)
            bottom = max(b[3] for b in row)
            if top <= centre <= bottom:
                row.append(box)
                break
        else:
            rows.append([box])
    return [b for row in rows for b in sorted(row, key=lambda b: b[0])]


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    result = {}
    for name, file in SHEETS.items():
        image = Image.open(BASE / file).convert("RGB")
        boxes = reading_order(components(image, green_bg=name == "fight_left"))
        result[name] = {"file": file, "size": list(image.size),
                        "poses": [{"id": i + 1, "box": list(b)} for i, b in enumerate(boxes)]}
        print(f"{name}: {len(boxes)} poses")
    (OUT / "poses.json").write_text(json.dumps(result, indent=1) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
