"""Etapa 03D: put the approved hair back over an imported armour take, from the N4 layers only.

The N4-A partition of the approved idle (`art-src/n4-sprite-machine/stage/parts/idle/`
`hair_back.png` and `hair_front.png`, 80x128, pivot 40,118) holds every hair texel of
the approved sprite, texel for texel (checked here). In the approved layer order both hair
layers sit above the torso armour, so the restoration is a plain composite of those two
layers over the take: every restored texel is an original pixel, nothing is redrawn.

Writes, in runs/<take>/hair_restore/:
  armour_hair_restored_80x128.png, hair_restore_comparison.png, hair_restore.json

    python3 -I -B evidence/erro_corretion/etapa03d/scripts/restore_hair.py --run take01
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[4]
STAGE = ROOT / "evidence/erro_corretion/etapa03d"
IDLE = ROOT / "art-src/n3-5-fidelity/generated/candidate_106.png"
GRID = ROOT / "art-src/n3-5-fidelity/source/candidate_106.txt"
LAYERS = [ROOT / "art-src/n4-sprite-machine/stage/parts/idle" / f"{name}.png" for name in ("hair_back", "hair_front")]
HAIR_KEYS = set("WwhH")
BG = (30, 26, 38, 255)
SHEET_BG = (20, 18, 26, 255)
WHITE = (240, 240, 240)


def rgba(path: Path) -> Image.Image:
    return Image.open(path).convert("RGBA")


def opaque(image: Image.Image) -> set[tuple[int, int]]:
    a = image.getchannel("A").load()
    return {(x, y) for y in range(image.height) for x in range(image.width) if a[x, y]}


def zoom(image: Image.Image, k: int) -> Image.Image:
    big = image.resize((image.width * k, image.height * k), Image.Resampling.NEAREST)
    base = Image.new("RGBA", big.size, BG)
    base.alpha_composite(big)
    return base


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--run", required=True)
    args = parser.parse_args()
    run = STAGE / "runs" / args.run
    out = run / "hair_restore"
    out.mkdir(exist_ok=True)
    original = rgba(IDLE)
    take = rgba(run / "armour_on_canvas_80x128.png")
    hair = Image.new("RGBA", original.size, (0, 0, 0, 0))
    for layer in LAYERS:
        hair.alpha_composite(rgba(layer))
    cells = opaque(hair)
    mismatched = [c for c in cells if hair.getpixel(c) != original.getpixel(c)]
    if mismatched:
        raise SystemExit(f"N4 hair layers differ from the approved sprite at {len(mismatched)} texels")
    restored = take.copy()
    restored.alpha_composite(hair)
    restored.save(out / "armour_hair_restored_80x128.png")
    grid = GRID.read_text(encoding="utf-8").splitlines()
    before = [c for c in cells if take.getpixel(c) != original.getpixel(c)]
    after = [c for c in cells if restored.getpixel(c) != original.getpixel(c)]
    key_hair = [(x, y) for y in range(128) for x in range(80) if grid[y][x] in HAIR_KEYS]
    key_changed = [c for c in key_hair if take.getpixel(c) != original.getpixel(c)]
    not_layer = [c for c in key_changed if c not in cells]
    armour_lost = [c for c in cells if take.getpixel(c) != original.getpixel(c)]
    report = {
        "n4_hair_texels": len(cells), "n4_hair_equal_to_approved": len(cells) - len(mismatched),
        "hair_texels_changed_in_take": len(before), "hair_texels_changed_after_restore": len(after),
        "armour_texels_given_back_to_hair": len(armour_lost),
        "take01_report_note": {
            "hair_key_texels_changed": len(key_changed),
            "of_which_hair_by_n4_layers": len(key_changed) - len(not_layer),
            "of_which_shirt_shading_with_hair_keys": len(not_layer),
        },
    }
    (out / "hair_restore.json").write_text(json.dumps(report, indent=1))
    mark = Image.new("RGBA", original.size, (0, 0, 0, 0))
    for c in opaque(take):
        mark.putpixel(c, (80, 80, 92, 255))
    for c in before:
        mark.putpixel(c, (255, 60, 60, 255))
    panels = [(original, "aprovado"), (take, f"{args.run} (pose)"), (mark, "vermelho: cabelo trocado"),
              (restored, "cabelo N4 restaurado")]
    crop = (22, 10, 58, 60)
    rows = [[zoom(img, 3) for img, _ in panels], [zoom(img.crop(crop), 8) for img, _ in panels]]
    width = sum(i.width for i in rows[1]) + 12 * 3 + 16
    height = 30 + rows[0][0].height + rows[1][0].height + 70
    sheet = Image.new("RGBA", (width, height), SHEET_BG)
    draw = ImageDraw.Draw(sheet)
    draw.text((8, 6), f"Etapa 03D {args.run} - cabelo restaurado com as camadas N4 aprovadas (hair_back + hair_front)",
              fill=WHITE)
    y = 28
    for row_images in rows:
        x = 8
        for image, (_, title) in zip(row_images, panels, strict=True):
            sheet.alpha_composite(image, (x, y + 16))
            draw.text((x, y), title, fill=WHITE)
            x += (rows[1][0].width if row_images is rows[0] else image.width) + 12
        y += row_images[0].height + 22
    draw.text((8, y), f"cabelo N4: {len(cells)} texels (todos = aprovado) | trocados no take: {len(before)} -> "
                      f"depois: {len(after)}", fill=WHITE)
    sheet.save(out / "hair_restore_comparison.png")
    print(json.dumps(report, indent=1))


if __name__ == "__main__":
    main()
