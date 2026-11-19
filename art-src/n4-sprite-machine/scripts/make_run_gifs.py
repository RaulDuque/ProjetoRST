"""N4-1 (C1): GIFs of the modular run, made from the project's real files
(the layer strips in stage/parts/run_c1/ and the approved run6.png).

C1 has the frames #0, #2 and #4: each is shown for 2/15 s so the cycle stays
0.4 s, while the original plays its 6 frames at 15 fps. GIF delays are whole
centiseconds: 7/7/6 cs per original frame (0.20 s per pair = 15 fps), and
13/13/14 cs per C1 frame (0.40 s per cycle).

Usage (project root): ``python3 -B art-src/n4-sprite-machine/scripts/make_run_gifs.py``
"""

from __future__ import annotations

import re
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[3]
STAGE = ROOT / "art-src/n4-sprite-machine/stage"
RUN6 = ROOT / "art-src/n3-6-animation-proof/generated/run6.png"
OUT = ROOT / "docs/development/n4/evidence/n4c"
W, H = 112, 128
CROP = (12, 8, 100, 122)
SCALE = 3
BG = (48, 44, 56, 255)
PIECES = ["hair", "shirt", "trousers", "boots", "bracers"]


def composition() -> tuple[list[str], dict[str, list[tuple[str, str]]], list[str]]:
    """Layer order, the run_c1 (layer, texture) pieces of every part, and the
    default outfit, read from protagonist.ron (no RON parser in the art
    scripts: a small, checked regex over the known layout)."""
    text = (STAGE / "protagonist.ron").read_text()
    layers = re.findall(r'"([a-z_]+)"', re.search(r"layers: \[(.*?)\]", text, re.S).group(1))
    parts: dict[str, list[tuple[str, str]]] = {}
    for m in re.finditer(r'"([a-z_]+)": \(region: \w+.*?\]\),', text, re.S):
        pieces = re.findall(r'layer: "([a-z_]+)".*?"run_c1": \(texture: "([^"]+)"', m.group(0), re.S)
        parts[m.group(1)] = pieces
    outfit = re.findall(r'"([a-z_]+)"', re.search(r"default_outfit: \[(.*?)\]", text, re.S).group(1))
    assert len(layers) == 14 and sum(len(p) for p in parts.values()) == 14, (layers, parts)
    return layers, parts, outfit


def frame_of(strip: Image.Image, k: int) -> Image.Image:
    return strip.crop((k * W, 0, (k + 1) * W, H))


def compose(layers, parts, worn: list[str], k: int, strips) -> Image.Image:
    z = {n: i for i, n in enumerate(layers)}
    pieces = sorted((z[layer], tex) for part in worn for layer, tex in parts[part])
    out = Image.new("RGBA", (W, H), (0, 0, 0, 0))
    for _, tex in pieces:
        out.alpha_composite(frame_of(strips[tex], k))
    return out


def tile(im: Image.Image, label: str) -> Image.Image:
    base = Image.new("RGBA", (W, H), BG)
    base.alpha_composite(im)
    t = base.crop(CROP).resize(((CROP[2] - CROP[0]) * SCALE, (CROP[3] - CROP[1]) * SCALE), Image.NEAREST)
    out = Image.new("RGB", (t.width, t.height + 14), (16, 14, 20))
    out.paste(t.convert("RGB"), (0, 14))
    ImageDraw.Draw(out).text((2, 1), label, fill=(235, 200, 110))
    return out


def row(tiles: list[Image.Image]) -> Image.Image:
    out = Image.new("RGB", (sum(t.width for t in tiles) + 4 * (len(tiles) - 1), tiles[0].height), (0, 0, 0))
    x = 0
    for t in tiles:
        out.paste(t, (x, 0))
        x += t.width + 4
    return out


def save(frames: list[Image.Image], delays_cs: list[int], name: str) -> None:
    pal = [f.convert("P", palette=Image.ADAPTIVE, colors=96) for f in frames]
    pal[0].save(OUT / name, save_all=True, append_images=pal[1:], duration=[d * 10 for d in delays_cs], loop=0, disposal=1)


def main() -> None:
    layers, parts, outfit = composition()
    strips = {tex: Image.open(STAGE / tex).convert("RGBA") for p in parts.values() for _, tex in p}
    run6 = Image.open(RUN6).convert("RGBA")
    base = ["body"]
    full = ["body"] + outfit
    looks = [("dressed", full)] + [(f"no {p}", [x for x in full if x != p]) for p in PIECES] + [("base body", base)]
    c1_delays = [13, 13, 14]
    save([tile(compose(layers, parts, full, k, strips), f"modular C1 #{2 * k}") for k in range(3)], c1_delays, "n4c_run_modular.gif")
    save([tile(compose(layers, parts, base, k, strips), f"base body #{2 * k}") for k in range(3)], c1_delays, "n4c_run_base_body.gif")
    save([row([tile(compose(layers, parts, names, k, strips), label) for label, names in looks]) for k in range(3)],
         c1_delays, "n4c_run_removal.gif")
    # Original 6 frames at 15 fps next to C1 (each C1 frame held for two).
    frames, delays = [], []
    for i in range(6):
        frames.append(row([tile(frame_of(run6, i), f"approved run6 #{i} (15 fps)"),
                           tile(compose(layers, parts, full, i // 2, strips), f"modular C1 #{2 * (i // 2)}"),
                           tile(compose(layers, parts, base, i // 2, strips), "C1 base body")]))
        delays.append([7, 7, 6][i % 3])
    save(frames, delays, "n4c_original_vs_modular.gif")
    print("written:", sorted(p.name for p in OUT.glob("*.gif")))


if __name__ == "__main__":
    main()
