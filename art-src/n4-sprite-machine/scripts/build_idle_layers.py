"""N4-A: build the modular idle layers, check the recomposition, make evidence.

1. partition.py splits every opaque texel of the approved N3.5 idle into one
   layer (no texel changed);
2. reconstruct.py adds the hidden anatomy (agent's interpretation) to the
   body and arm layers, only where a layer above covers it;
3. each layer is written as a palette-key grid (``.txt``) and an RGBA PNG on
   the same 80x128 canvas and pivot (40,118) as the original;
4. the layers are recomposed in draw order and compared with the original,
   texel by texel; any difference is an error;
5. evidence sheets and ``metrics.json`` go to docs/development/n4/evidence/.

Usage (project root): ``python3 -B art-src/n4-sprite-machine/scripts/build_idle_layers.py``
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

from PIL import Image, ImageDraw

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(ROOT / "art-src/n3-4-original-protagonist/scripts"))

import partition  # noqa: E402
import reconstruct  # noqa: E402
from n34_palette import PALETTE  # noqa: E402

ORIGINAL_PNG = ROOT / "art-src/n3-5-fidelity/generated/candidate_106.png"
STAGE = ROOT / "art-src/n4-sprite-machine/stage"
PARTS = STAGE / "parts/idle"
EVIDENCE = ROOT / "docs/development/n4/evidence"
W, H = 80, 128
# Back to front. Must match the `layers` list of stage/protagonist.ron
# (tests/n4_sprite_machine_tests.rs recomposes the grids in the RON order).
ORDER = ["body", "trousers", "boots", "arm_front", "shirt", "bracers", "hair_back", "hair_front"]
# Which removable piece owns each layer (body and arm_front are the base).
PIECE_OF = {
    "body": "body", "arm_front": "body", "trousers": "trousers", "boots": "boots",
    "shirt": "shirt", "bracers": "bracers", "hair_back": "hair", "hair_front": "hair",
}


def rgba(key: str) -> tuple[int, int, int, int]:
    r, g, b = PALETTE[key][:3]
    return (r, g, b, 255)


def build() -> tuple[dict[str, dict], dict]:
    rows = partition.read_grid()
    layers = partition.partition(rows)
    for name in ORDER:
        layers.setdefault(name, {})
    z = {name: i for i, name in enumerate(ORDER)}
    owner = {p: name for name, texels in layers.items() for p in texels}
    report = {"original_opaque": len(owner), "painted": {}, "clipped": {}}

    def add_painted(layer: str, paint: dict) -> None:
        kept, clipped = {}, 0
        for p, key in paint.items():
            if p in layers[layer]:
                continue  # never repaint an original texel
            above = owner.get(p)
            if above is None or z[above] <= z[layer]:
                clipped += 1  # would show over the approved art
                continue
            kept[p] = key
        reconstruct.outline(kept, set(layers[layer]))
        layers[layer].update(kept)
        report["painted"][layer] = len(kept)
        report["clipped"][layer] = clipped

    add_painted("body", reconstruct.body_paint())
    add_painted("arm_front", reconstruct.arm_paint())
    return layers, report


def grid_text(texels: dict) -> str:
    rows = [["."] * W for _ in range(H)]
    for (x, y), key in texels.items():
        rows[y][x] = key
    return "\n".join("".join(r) for r in rows) + "\n"


def image(texels: dict) -> Image.Image:
    im = Image.new("RGBA", (W, H), (0, 0, 0, 0))
    for (x, y), key in texels.items():
        im.putpixel((x, y), rgba(key))
    return im


def compose(layers: dict, names: list[str]) -> Image.Image:
    out = Image.new("RGBA", (W, H), (0, 0, 0, 0))
    for name in ORDER:
        if name in names:
            out.alpha_composite(image(layers[name]))
    return out


def on_bg(im: Image.Image, bg=(48, 44, 56)) -> Image.Image:
    base = Image.new("RGBA", im.size, bg + (255,))
    base.alpha_composite(im)
    return base


def sheet(items: list[tuple[str, Image.Image]], path: Path, scale: int = 4, bg=(48, 44, 56)) -> None:
    crop = (20, 8, 60, 122)
    cw, ch = (crop[2] - crop[0]) * scale, (crop[3] - crop[1]) * scale
    out = Image.new("RGB", (len(items) * (cw + 6), ch + 16), (16, 14, 20))
    d = ImageDraw.Draw(out)
    for i, (label, im) in enumerate(items):
        tile = on_bg(im, bg).crop(crop).resize((cw, ch), Image.NEAREST)
        out.paste(tile.convert("RGB"), (i * (cw + 6), 16))
        d.text((i * (cw + 6) + 2, 2), label, fill=(235, 200, 110))
    out.save(path)


def main() -> None:
    layers, report = build()
    PARTS.mkdir(parents=True, exist_ok=True)
    EVIDENCE.mkdir(parents=True, exist_ok=True)
    for name in ORDER:
        (PARTS / f"{name}.txt").write_text(grid_text(layers[name]))
        image(layers[name]).save(PARTS / f"{name}.png")

    original = Image.open(ORIGINAL_PNG).convert("RGBA")
    full = compose(layers, ORDER)
    diff_count, diff_map = 0, Image.new("RGB", (W, H), (0, 0, 0))
    for y in range(H):
        for x in range(W):
            a, b = original.getpixel((x, y)), full.getpixel((x, y))
            same = (a[3] == 0 and b[3] == 0) or a == b
            if not same:
                diff_count += 1
                diff_map.putpixel((x, y), (255, 0, 64))
            elif a[3]:
                diff_map.putpixel((x, y), (40, 60, 40))
    report["recomposition_differences"] = diff_count
    report["layers"] = {n: len(layers[n]) for n in ORDER}

    every = list(ORDER)
    without = lambda piece: [n for n in ORDER if PIECE_OF[n] != piece]  # noqa: E731
    base = [n for n in ORDER if PIECE_OF[n] == "body"]
    # Holes: after removing a piece, transparent texels inside the row
    # extent of the base body (body + arm_front) are holes where anatomy
    # should be. The base body itself is measured too.
    base_im = compose(layers, base)
    extent = {}
    for y in range(H):
        xs = [x for x in range(W) if base_im.getpixel((x, y))[3]]
        if xs:
            extent[y] = (min(xs), max(xs))

    def holes_in(im: Image.Image) -> int:
        return sum(1 for y, (a, b) in extent.items() for x in range(a, b + 1) if im.getpixel((x, y))[3] == 0)

    report["holes_inside_body_after_removal"] = {
        piece: holes_in(compose(layers, without(piece)))
        for piece in ["hair", "shirt", "trousers", "boots", "bracers"]
    }
    report["holes_inside_base_body"] = holes_in(base_im)
    sheet([("N3.5 original", original), ("modular (all layers)", full),
           ("diff (red = different)", diff_map.convert("RGBA"))], EVIDENCE / "n4a_recomposition.png", scale=5)
    diff_map.resize((W * 6, H * 6), Image.NEAREST).save(EVIDENCE / "n4a_diff_map.png")
    sheet([(n, image(layers[n])) for n in ORDER], EVIDENCE / "n4a_layers.png")
    sheet([
        ("original outfit", full),
        ("no hair", compose(layers, without("hair"))),
        ("no shirt", compose(layers, without("shirt"))),
        ("no trousers", compose(layers, without("trousers"))),
        ("no boots", compose(layers, without("boots"))),
        ("no bracers", compose(layers, without("bracers"))),
        ("base body", compose(layers, base)),
    ], EVIDENCE / "n4a_removal.png")
    sheet([
        ("original outfit", full),
        ("no hair", compose(layers, without("hair"))),
        ("no shirt", compose(layers, without("shirt"))),
        ("no trousers", compose(layers, without("trousers"))),
        ("no boots", compose(layers, without("boots"))),
        ("base body", compose(layers, base)),
    ], EVIDENCE / "n4a_removal_light.png", bg=(214, 210, 200))
    pieces = {
        "body (base)": compose(layers, base),
        "hair (back + front)": compose(layers, ["hair_back", "hair_front"]),
        "shirt": compose(layers, ["shirt"]),
        "trousers": compose(layers, ["trousers"]),
        "boots": compose(layers, ["boots"]),
        "bracers": compose(layers, ["bracers"]),
    }
    sheet(list(pieces.items()), EVIDENCE / "n4a_pieces.png")
    (EVIDENCE / "metrics.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))
    if diff_count:
        sys.exit(f"recomposition differs from the original in {diff_count} texels")


if __name__ == "__main__":
    main()
