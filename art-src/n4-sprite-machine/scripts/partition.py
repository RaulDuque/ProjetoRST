"""N4-A: partition of the approved N3.5 idle into layers (no pixel changed).

Every opaque texel of ``art-src/n3-5-fidelity/source/candidate_106.txt``
(palette keys, 80x128, pivot 40,118) is given to exactly ONE layer by the
region rules below. Because the layers are a partition of the original,
recomposing all of them gives the original texel for texel; the rules only
decide how clean each separated piece is.

The rules are hand-authored from the enlarged grid (rows y, columns x, in
canvas texels). They are data for the art, not game logic.
"""

from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
SOURCE = ROOT / "art-src/n3-5-fidelity/source/candidate_106.txt"

HAIR_KEYS = set("WwhH")
SKIN_KEYS = set("Ssz")

# Back hair over the shirt: x <= HAIR_RIGHT[y] is hair (rows 28..47).
HAIR_RIGHT = {
    28: 35, 29: 34, 30: 33, 31: 32, 32: 32, 33: 31, 34: 31, 35: 31, 36: 31, 37: 30,
    38: 30, 39: 30, 40: 31, 41: 31, 42: 31, 43: 31, 44: 32, 45: 32, 46: 32, 47: 31,
}
HEAD_ROWS = range(13, 28)      # skull, face and the hair over them
FACE_FROM_ROW = 18             # above it, everything in the head rows is hair
FACE_FROM_X = 41               # in head rows, non-hair texels at x >= 41 are the face
HAIR_FRONT_FROM_X = 40         # hair texels at x >= 40 cross the face: front hair
FOREARM = (range(52, 59), range(36, 42))
BRACER = (range(59, 64), range(38, 44))
HAND = (range(64, 72), range(40, 46))
TROUSERS_FROM_ROW = 51         # belt and trousers
BOOTS_FROM_ROW = 90            # boot shafts (estimated cuff line) and feet


def read_grid(path: Path = SOURCE) -> list[str]:
    rows = path.read_text().splitlines()
    assert len(rows) == 128 and all(len(r) == 80 for r in rows), "N3.5 grid must be 80x128"
    return rows


def inside(box, x: int, y: int) -> bool:
    rows, cols = box
    return y in rows and x in cols


def layer_of(x: int, y: int, key: str) -> str:
    """The layer an opaque original texel belongs to."""
    if inside(FOREARM, x, y) or inside(HAND, x, y):
        return "arm_front"
    if inside(BRACER, x, y):
        return "bracers"
    if y in HEAD_ROWS:
        if y < FACE_FROM_ROW or key in HAIR_KEYS or (x < FACE_FROM_X and key not in SKIN_KEYS):
            if x >= HAIR_FRONT_FROM_X and y >= 17:
                return "hair_front"
            return "hair_back"
        return "body"
    if y in HAIR_RIGHT and x <= HAIR_RIGHT[y]:
        return "hair_back"
    if y >= BOOTS_FROM_ROW:
        return "boots"
    if y >= TROUSERS_FROM_ROW:
        return "trousers"
    return "shirt"


def partition(rows: list[str]) -> dict[str, dict[tuple[int, int], str]]:
    out: dict[str, dict[tuple[int, int], str]] = {}
    for y, row in enumerate(rows):
        for x, key in enumerate(row):
            if key != ".":
                out.setdefault(layer_of(x, y, key), {})[(x, y)] = key
    return out
