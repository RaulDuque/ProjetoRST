"""Derive the prototype pauldrons from the N41-B factory art (offline tool).

The factory pauldrons (`art-src/n4-sprite-machine/stage/factory/isolated/
pauldron_*.png`, 28x30 opaque texels) are about twice the protagonist's
shoulder. This script crops their opaque box and reduces it 2:1: each output
texel takes the most common opaque colour of its 2x2 block (transparent when
fewer than two of the four are opaque). No new colour is created. The
result is DERIVED and PROVISIONAL art, not approved.

    ~/.local/share/dc-art-tools/asf-venv/bin/python -B \
        art-src/modular-character-engine/scripts/derive_pauldrons.py
"""

from __future__ import annotations

from collections import Counter
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parents[3]
SOURCE = ROOT / "art-src/n4-sprite-machine/stage/factory/isolated"
OUT = ROOT / "art-src/modular-character-engine/stage/equipment"
ITEMS = ("pauldron_steel", "pauldron_bronze")


def reduce_half(image: Image.Image) -> Image.Image:
    box = image.getchannel("A").getbbox()
    if box is None:
        raise SystemExit("empty source image")
    image = image.crop(box)
    w, h = image.size
    out = Image.new("RGBA", (w // 2, h // 2), (0, 0, 0, 0))
    src = image.load()
    for y in range(h // 2):
        for x in range(w // 2):
            block = [src[2 * x + i, 2 * y + j] for i in (0, 1) for j in (0, 1)]
            opaque = [p for p in block if p[3] > 0]
            if len(opaque) >= 2:
                out.putpixel((x, y), Counter(opaque).most_common(1)[0][0])
    return out


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    for item in ITEMS:
        source = Image.open(SOURCE / f"{item}.png").convert("RGBA")
        small = reduce_half(source)
        small.save(OUT / f"{item}_half.png")
        src_colours = {c for _, c in source.getcolors(1 << 16) or [] if c[3] > 0}
        out_colours = {c for _, c in small.getcolors(1 << 16) or [] if c[3] > 0}
        assert out_colours <= src_colours, "a colour was invented"
        print(f"{item}_half.png {small.size}")


if __name__ == "__main__":
    main()
