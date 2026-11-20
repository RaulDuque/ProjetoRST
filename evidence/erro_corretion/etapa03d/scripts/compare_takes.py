"""Etapa 03D: compare imported takes at the game's real size (after import_external.py,
take_report.py and restore_hair.py for each take).

Uses each take's hair-restored 80x128 sprite (pivot 40,118). Writes to runs/<out>/:
  takes_comparison.png   approved | takes, at 1x, 2x and 8x (torso crop)
  takes_dark_preview.png the sprites on a dark gothic ground at 1x and 3x
  takes_metrics.json     per take: changed texels, silhouette growth, colours, zone identity

    python3 -I -B evidence/erro_corretion/etapa03d/scripts/compare_takes.py --takes take01 take02 --out take02
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[4]
STAGE = ROOT / "evidence/erro_corretion/etapa03d"
IDLE = ROOT / "art-src/n3-5-fidelity/generated/candidate_106.png"
HAIR = [ROOT / "art-src/n4-sprite-machine/stage/parts/idle" / f"{n}.png" for n in ("hair_back", "hair_front")]
HEAD_END, LEGS_START = 27, 72
TORSO = (22, 22, 58, 78)
BG = (30, 26, 38, 255)
DARK = (14, 11, 20, 255)
SHEET_BG = (20, 18, 26, 255)
WHITE = (240, 240, 240)


def rgba(path: Path) -> Image.Image:
    return Image.open(path).convert("RGBA")


def zoom(image: Image.Image, k: int, bg=BG) -> Image.Image:
    big = image.resize((image.width * k, image.height * k), Image.Resampling.NEAREST)
    base = Image.new("RGBA", big.size, bg)
    base.alpha_composite(big)
    return base


def row(items: list[tuple[Image.Image, str]], gap: int = 14) -> Image.Image:
    out = Image.new("RGBA", (sum(i.width for i, _ in items) + gap * (len(items) - 1),
                             max(i.height for i, _ in items) + 20), SHEET_BG)
    draw = ImageDraw.Draw(out)
    x = 0
    for image, title in items:
        out.alpha_composite(image, (x, 20))
        draw.text((x + 2, 3), title, fill=WHITE)
        x += image.width + gap
    return out


def stack(rows: list[Image.Image], title: str, notes: list[str]) -> Image.Image:
    width = max(r.width for r in rows) + 16
    out = Image.new("RGBA", (width, 34 + sum(r.height + 12 for r in rows) + 16 * len(notes) + 8), SHEET_BG)
    draw = ImageDraw.Draw(out)
    draw.text((8, 8), title, fill=WHITE)
    y = 34
    for r in rows:
        out.alpha_composite(r, (8, y))
        y += r.height + 12
    for line in notes:
        draw.text((8, y), line, fill=(220, 220, 220))
        y += 16
    return out


def measure(sprite: Image.Image, original: Image.Image, hair: set) -> dict:
    zones = {"head": [0, 0], "legs": [0, 0], "hair_n4": [0, 0], "torso_band": [0, 0]}
    changed = grown = 0
    for y in range(original.height):
        for x in range(original.width):
            p, q = sprite.getpixel((x, y)), original.getpixel((x, y))
            same = p == q or (not p[3] and not q[3])
            changed += not same
            grown += bool(p[3] and not q[3])
            if q[3]:
                names = ["head" if y < HEAD_END else "legs" if y >= LEGS_START else "torso_band"]
                if (x, y) in hair:
                    names.append("hair_n4")
                for n in names:
                    zones[n][0] += 1
                    zones[n][1] += same
    colours = {c[:3] for c in sprite.get_flattened_data() if c[3]}
    base = {c[:3] for c in original.get_flattened_data() if c[3]}
    return {"texels_changed": changed, "silhouette_added": grown, "colours": len(colours),
            "new_colours": len(colours - base), "canvas": sprite.size,
            "alpha_binary": {c[3] for c in sprite.get_flattened_data()} <= {0, 255},
            "zones_identical": {k: f"{v[1]}/{v[0]}" for k, v in zones.items()}}


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--takes", nargs="+", required=True)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()
    out = STAGE / "runs" / args.out
    out.mkdir(parents=True, exist_ok=True)
    original = rgba(IDLE)
    hair_img = Image.new("RGBA", original.size, (0, 0, 0, 0))
    for layer in HAIR:
        hair_img.alpha_composite(rgba(layer))
    hair = {(x, y) for y in range(128) for x in range(80) if hair_img.getpixel((x, y))[3]}
    sprites = [("aprovado N3.5", original)]
    metrics = {}
    for take in args.takes:
        path = STAGE / "runs" / take / "hair_restore/armour_hair_restored_80x128.png"
        sprite = rgba(path)
        sprites.append((f"{take} (cabelo N4)", sprite))
        metrics[take] = measure(sprite, original, hair)
    rows = [row([(zoom(s, 1), t) for t, s in sprites]), row([(zoom(s, 2), f"{t} 2x") for t, s in sprites]),
            row([(zoom(s.crop(TORSO), 8), f"{t} tronco 8x") for t, s in sprites])]
    notes = [f"{t}: alterados {m['texels_changed']} | silhueta +{m['silhouette_added']} | cores {m['colours']} "
             f"(+{m['new_colours']}) | identicos: {m['zones_identical']}" for t, m in metrics.items()]
    stack(rows, "Etapa 03D - takes na resolucao real (80x128, pivo 40,118)", notes).save(out / "takes_comparison.png")
    dark = [row([(zoom(s, 1, DARK), t) for t, s in sprites]), row([(zoom(s, 3, DARK), f"{t} 3x") for t, s in sprites])]
    stack(dark, "Etapa 03D - previa sobre fundo escuro", []).save(out / "takes_dark_preview.png")
    (out / "takes_metrics.json").write_text(json.dumps(metrics, indent=1, default=list))
    print(json.dumps(metrics, indent=1, default=list))


if __name__ == "__main__":
    main()
