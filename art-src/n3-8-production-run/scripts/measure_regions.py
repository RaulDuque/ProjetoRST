"""N3.8 study for N4: where the body regions and hands are in the six approved
run frames and in the N3.5 idle (read-only; estimates, not sockets).

Reads the palette-key grids (one character per texel) of the approved art:
``art-src/n3-6-animation-proof/source/video_0NN.txt`` (112x128, pivot 56,118)
and ``art-src/n3-5-fidelity/source/candidate_106.txt`` (80x128, pivot 40,118).
Writes ``art-src/n3-8-production-run/analysis/regions.json``.

Units: world units from the sole-line pivot (x forward, y up), 1 u = 2 texels.
Hands: the sleeves are rolled up, so a skin component below the face is a
forearm plus its fist. The fist estimate is the mean of the texels within
4 texels of the component's point farthest from the shoulder (top of the
shirt). Which arm is in front of the body is NOT known from the pixels
(``depth: "unknown"``): every value is an estimate for N4, not a socket.
"""

from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
SOURCES = [("idle_n35", ROOT / "art-src/n3-5-fidelity/source/candidate_106.txt", (40, 118))] + [
    (f"run_{i}_video_{v}", ROOT / f"art-src/n3-6-animation-proof/source/video_{v:03d}.txt", (56, 118))
    for i, v in enumerate([48, 50, 52, 54, 56, 58])
]
GROUPS = {
    "hair": set("WwhH"),
    "skin": set("Ssz"),
    "shirt": set("Ggq"),
    "trousers": set("Ppo"),
    "leather": set("Lld"),
    "metal": set("M"),
    "outline": set("K"),
}


def components(cells: set[tuple[int, int]]) -> list[list[tuple[int, int]]]:
    left, out = set(cells), []
    while left:
        stack, comp = [left.pop()], []
        while stack:
            x, y = stack.pop()
            comp.append((x, y))
            for n in ((x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)):
                if n in left:
                    left.remove(n)
                    stack.append(n)
        out.append(comp)
    return out


def world(px: float, py: float, pivot: tuple[int, int]) -> dict:
    return {"x": round((px - pivot[0]) / 2, 1), "y": round((pivot[1] - py) / 2, 1)}


def centre(cells) -> tuple[float, float]:
    return sum(c[0] for c in cells) / len(cells), sum(c[1] for c in cells) / len(cells)


def measure(path: Path, pivot: tuple[int, int]) -> dict:
    rows = path.read_text().splitlines()
    by = {g: set() for g in GROUPS}
    for y, row in enumerate(rows):
        for x, ch in enumerate(row):
            for g, keys in GROUPS.items():
                if ch in keys:
                    by[g].add((x, y))
    skin = sorted(components(by["skin"]), key=lambda c: -len(c))
    skin = [c for c in skin if len(c) >= 6]
    face = min(skin, key=lambda c: centre(c)[1]) if skin else []
    hands = [c for c in skin if c is not face and centre(c)[1] > centre(face)[1] + 10] if face else []
    shirt_top = min(c[1] for c in by["shirt"]) if by["shirt"] else 0
    shoulder = (centre(by["shirt"])[0] if by["shirt"] else pivot[0], shirt_top + 4)

    def fist(comp):
        far = max(comp, key=lambda c: (c[0] - shoulder[0]) ** 2 + (c[1] - shoulder[1]) ** 2)
        near = [c for c in comp if abs(c[0] - far[0]) <= 4 and abs(c[1] - far[1]) <= 4]
        return centre(near)

    opaque = {(x, y) for y, row in enumerate(rows) for x, ch in enumerate(row) if ch != "."}
    ys = [c[1] for c in opaque]
    result = {
        "texels": {g: len(c) for g, c in by.items()},
        "top_y": world(0, min(ys), pivot)["y"],
        "lowest_y": world(0, max(ys), pivot)["y"],
        "head_face": world(*centre(face), pivot) if face else None,
        "chest_shirt_centre": world(*centre(by["shirt"]), pivot) if by["shirt"] else None,
        "hair_centre": world(*centre(by["hair"]), pivot) if by["hair"] else None,
        "hair_back_extent_x": world(min(c[0] for c in by["hair"]), 0, pivot)["x"] if by["hair"] else None,
        "hands": [
            dict(world(*fist(c), pivot), texels=len(c), depth="unknown")
            for c in sorted(hands, key=lambda c: -centre(c)[0])
        ],
    }
    return result


def main() -> None:
    out = {name: measure(path, pivot) for name, path, pivot in SOURCES}
    dest = ROOT / "art-src/n3-8-production-run/analysis/regions.json"
    dest.write_text(json.dumps(out, indent=1) + "\n")
    for name, r in out.items():
        print(name, "hands:", [(h["x"], h["y"], h["texels"]) for h in r["hands"]], "head:", r["head_face"],
              "chest:", r["chest_shirt_centre"], "hair:", r["texels"]["hair"], "skin:", r["texels"]["skin"],
              "low:", r["lowest_y"], "top:", r["top_y"], "hair_back_x:", r["hair_back_extent_x"])


if __name__ == "__main__":
    main()
