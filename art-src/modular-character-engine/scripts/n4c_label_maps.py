"""Label maps of run frames #0, #2, #4 from the N4-C layers (offline tool).

N4-C (`art-src/n4-sprite-machine/stage/parts/run_c1/*.txt`) split the
approved run frames #0, #2 and #4 into 14 layers, measured on the
high-resolution video. Here every opaque texel takes the TOPMOST N4-C layer
that has it (the dressed recomposition), mapped to the five rig layers of
the Modular Character Engine:

    arm_front, shirt (near sleeve), bracers           -> N (near arm)
    sleeve_back, bracers_back, body_back above hips   -> F (far arm)
    hair_back, hair_front                             -> H (hair)
    shirt_torso, body above hips                      -> T (torso)
    trousers*, boots*, body*/body_back below hips     -> L (legs)

Output: `art-src/modular-character-engine/stage/layers/run_{0,2,4}.txt`
(112x128, `.` transparent). Nothing of the art changes; the Rust test
`modular_character_engine_tests` checks each map against the palette grid.

    python3 -B art-src/modular-character-engine/scripts/n4c_label_maps.py
"""

from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
PARTS = ROOT / "art-src/n4-sprite-machine/stage/parts/run_c1"
OUT = ROOT / "art-src/modular-character-engine/stage/layers"
W, H = 112, 128
# Back to front, as in art-src/n4-sprite-machine/stage/protagonist.ron.
ORDER = [
    "body_back", "trousers_back", "boots_back", "sleeve_back", "bracers_back",
    "body", "trousers", "boots", "shirt_torso",
    "arm_front", "shirt", "bracers", "hair_back", "hair_front",
]
# Row of the hips (pelvis y = 62 in every run frame): above it is torso.
PELVIS_Y = 62
# N4-C head centres (run_poses.py), radius widened to 9: some face texels of the N4-C arm layers
# fall inside them; inside the head only hair stays hair, the rest is torso.
HEADS = {0: ((66, 26), 9.0), 2: ((66, 25), 9.0), 4: ((67, 24), 9.0)}


def code(layer: str, y: int) -> str:
    if layer in ("arm_front", "shirt", "bracers"):
        return "N"
    if layer in ("sleeve_back", "bracers_back"):
        return "F"
    if layer in ("hair_back", "hair_front"):
        return "H"
    if layer == "shirt_torso":
        return "T"
    if layer == "body_back":
        return "F" if y < PELVIS_Y else "L"
    if layer == "body":
        return "T" if y < PELVIS_Y else "L"
    return "L"


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    grids = {}
    for layer in ORDER:
        rows = (PARTS / f"{layer}.txt").read_text().splitlines()
        assert len(rows) == H and all(len(r) == 3 * W for r in rows), layer
        grids[layer] = rows
    for k in range(3):
        out = []
        for y in range(H):
            row = []
            for x in range(W):
                label = "."
                for layer in ORDER:
                    if grids[layer][y][k * W + x] != ".":
                        label = code(layer, y)
                (hx, hy), hr = HEADS[2 * k]
                if label in ("N", "F") and (x + 0.5 - hx) ** 2 + (y + 0.5 - hy) ** 2 <= hr * hr:
                    label = "T"
                row.append(label)
            out.append("".join(row))
        path = OUT / f"run_{2 * k}.txt"
        path.write_text("\n".join(out) + "\n")
        print(path.relative_to(ROOT), sum(r.count(".") != W for r in out), "rows with texels")


if __name__ == "__main__":
    main()
