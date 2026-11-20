"""Etapa 03D: comparison sheet and zone metrics of one imported take (after import_external.py).

Zones on the 80x128 canvas of the approved N3.5 sprite (pivot 40,118):
- head: rows 0-26 (hair top and face; the armour must not touch it);
- legs: rows 72-127 (trousers below the tassets and boots);
- hair texels anywhere (N3.4 keys W w h H), including strands over the shoulder/back;
- torso band: rows 27-71, where the armour was asked for.
Also: alpha values, colours, outline cells of the new silhouette.

    python3 -I -B evidence/erro_corretion/etapa03d/scripts/take_report.py --run take01
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
HEAD_END, LEGS_START = 27, 72
HAIR = set("WwhH")
OUTLINE = (26, 20, 26)
BG = (30, 26, 38, 255)
SHEET_BG = (20, 18, 26, 255)
WHITE = (240, 240, 240)


def zoom(image: Image.Image, k: int, bg=BG) -> Image.Image:
    big = image.resize((image.width * k, image.height * k), Image.Resampling.NEAREST)
    base = Image.new("RGBA", big.size, bg)
    base.alpha_composite(big)
    return base


def panel(images: list[tuple[Image.Image, str]], gap: int = 14) -> Image.Image:
    head = 24
    out = Image.new("RGBA", (sum(i.width for i, _ in images) + gap * (len(images) - 1),
                             max(i.height for i, _ in images) + head), SHEET_BG)
    draw = ImageDraw.Draw(out)
    x = 0
    for image, title in images:
        out.alpha_composite(image, (x, head))
        draw.text((x + 2, 5), title, fill=WHITE)
        x += image.width + gap
    return out


def stack(rows: list[Image.Image], title: str, notes: list[str]) -> Image.Image:
    width = max(r.width for r in rows) + 16
    height = 34 + sum(r.height + 12 for r in rows) + 16 * len(notes) + 10
    out = Image.new("RGBA", (width, height), SHEET_BG)
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


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--run", required=True)
    args = parser.parse_args()
    run = STAGE / "runs" / args.run
    original = Image.open(IDLE).convert("RGBA")
    armour = Image.open(run / "armour_on_canvas_80x128.png").convert("RGBA")
    source = Image.open(run / "source.png").convert("RGBA")
    grid = GRID.read_text(encoding="utf-8").splitlines()
    zones = {z: {"texels": 0, "identical": 0} for z in ("head", "legs", "hair_anywhere", "torso_band")}
    diff = Image.new("RGBA", original.size, (0, 0, 0, 0))
    changed = grown = removed = 0
    for y in range(original.height):
        for x in range(original.width):
            p, q = armour.getpixel((x, y)), original.getpixel((x, y))
            same = p == q or (not p[3] and not q[3])
            if q[3]:
                names = ["head" if y < HEAD_END else "legs" if y >= LEGS_START else "torso_band"]
                if grid[y][x] in HAIR:
                    names.append("hair_anywhere")
                for name in names:
                    zones[name]["texels"] += 1
                    zones[name]["identical"] += same
            if not same:
                changed += 1
                if q[3] and p[3]:
                    hair = grid[y][x] in HAIR
                    diff.putpixel((x, y), (255, 60, 60, 255) if hair else (255, 200, 60, 255))
                elif p[3]:
                    grown += 1
                    diff.putpixel((x, y), (60, 200, 255, 255))
                else:
                    removed += 1
                    diff.putpixel((x, y), (200, 60, 255, 255))
            elif q[3]:
                diff.putpixel((x, y), (80, 80, 92, 255))
    for z in zones.values():
        z["changed"] = z["texels"] - z["identical"]
        z["identical_pct"] = round(100 * z["identical"] / z["texels"], 2) if z["texels"] else None
    edge_cells = edge_outline = 0
    for y in range(armour.height):
        for x in range(armour.width):
            if armour.getpixel((x, y))[3] and not original.getpixel((x, y))[3] or (
                    armour.getpixel((x, y))[3] and armour.getpixel((x, y)) != original.getpixel((x, y))):
                touches = any(not (0 <= x + dx < armour.width and 0 <= y + dy < armour.height)
                              or not armour.getpixel((x + dx, y + dy))[3]
                              for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)))
                if touches:
                    edge_cells += 1
                    edge_outline += armour.getpixel((x, y))[:3] == OUTLINE
    colours_o = {c[:3] for c in original.get_flattened_data() if c[3]}
    colours_a = {c[:3] for c in armour.get_flattened_data() if c[3]}
    metrics = {
        "canvas": armour.size, "pivot": [40, 118], "texels_changed": changed, "silhouette_added": grown,
        "silhouette_removed": removed, "zones": zones,
        "alpha_values": sorted({c[3] for c in armour.get_flattened_data()}),
        "colours": {"original": len(colours_o), "result": len(colours_a), "new": len(colours_a - colours_o),
                    "new_rgb": sorted(colours_a - colours_o)},
        "changed_edge_cells": edge_cells, "changed_edge_cells_with_outline_K": edge_outline,
    }
    (run / "zone_metrics.json").write_text(json.dumps(metrics, indent=1))
    big = source.resize((original.height * 4 * source.width // source.height, original.height * 4))
    row1 = panel([(zoom(original, 4), "1. aprovado N3.5 (4x nearest)"), (big, "2. imagem do ChatGPT"),
                  (zoom(armour, 4), "3. Agent Sprite Forge (pixel_look pose), 80x128"),
                  (zoom(diff, 4), "4. diferencas")])
    sizes = []
    for k in (1, 2, 3):
        sizes += [(zoom(original, k), f"aprovado {k}x"), (zoom(armour, k), f"armadura {k}x")]
    row2 = panel(sizes)
    torso = (24, 24, 56, 76)
    row3 = panel([(zoom(original.crop(torso), 8), "torso aprovado 8x"), (zoom(armour.crop(torso), 8), "torso armadura 8x"),
                  (zoom(diff.crop(torso), 8), "diferencas 8x")])
    z = zones
    notes = [
        f"texels alterados: {changed} | silhueta nova: {grown} | removidos: {removed} | alfa: {metrics['alpha_values']}",
        f"cabeca (linhas 0-26): {z['head']['identical']}/{z['head']['texels']} identicos | "
        f"pernas (72-127): {z['legs']['identical']}/{z['legs']['texels']} | "
        f"cabelo em qualquer lugar: {z['hair_anywhere']['identical']}/{z['hair_anywhere']['texels']}",
        f"faixa do tronco (27-71): {z['torso_band']['identical']}/{z['torso_band']['texels']} identicos | cores: "
        f"{len(colours_o)} -> {len(colours_a)} (+{len(colours_a - colours_o)})",
        "diferencas: cinza = igual | amarelo = mudou | vermelho = mudou sobre cabelo | azul = silhueta nova | "
        "roxo = removido",
    ]
    stack([row1, row2, row3], f"Etapa 03D {args.run} - imagem externa (ChatGPT) lida pelo Agent Sprite Forge",
          notes).save(run / f"{args.run}_comparison_sheet.png")
    # The torso as drawn in the picture next to the 1x read-back, at the same size, to see
    # what the reduction to the game's lattice keeps (picture cell size from the pose record).
    record = json.loads((run / "pose/armour.json").read_text())
    request = json.loads((STAGE / "request_pose/request.json").read_text())
    cell = record["lattice"]["cell"]
    box = original.getbbox() or (0, 0, 0, 0)
    mx, my = request["masterAt"][0] - box[0], request["masterAt"][1] - box[1]
    crop = source.crop(tuple(int((v + m) * cell) for v, m in zip(torso, (mx, my, mx, my), strict=True)))
    size = ((torso[2] - torso[0]) * 8, (torso[3] - torso[1]) * 8)
    panel([(crop.resize(size, Image.Resampling.LANCZOS), "tronco na imagem do ChatGPT"),
           (zoom(armour.crop(torso), 8), "tronco lido em 1x (8x nearest)")]).save(run / f"{args.run}_torso_source_vs_1x.png")
    print(json.dumps({k: v for k, v in metrics.items() if k != "colours"}, indent=1))
    print("colours", {k: v for k, v in metrics["colours"].items() if k != "new_rgb"})


if __name__ == "__main__":
    main()
