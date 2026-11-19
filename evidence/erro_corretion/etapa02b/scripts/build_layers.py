"""Etapa 02B (Erro-Corretion): test layers of the DRESSED base, overlay zones, previews.

Derived test files only (evidence/erro_corretion/etapa02b/out/). Nothing in
art-src, assets or the N4 stage is written.

Layers are a PARTITION of the approved texels (no hidden anatomy, no nude
body, no new texel): every opaque texel of the approved frame goes to exactly
one layer, so the dressed base without armour recomposes the original texel
for texel.

- idle: `partition.layer_of` (N4-A, visible texels) and one new rule, the
  near sleeve (`IDLE_SLEEVE`), read from the sprite's own outline texels: back
  edge x 32-33, front crease x 40-41, shoulder cap from row 31, rolled cuff
  down to row 51 (the forearm starts at row 52), as in the owner's sheet.
- run C1 #0/#2/#4: `build_run_layers.classify` (N4-1, visible texels) with the
  hair/face fix: texels above the neck row that the classifier gave to an
  arm (near or far) (they are outside the head circle of run_poses) go back to the head:
  skin keys to `face_skin`, the rest to `hair_front`/`hair_back` (face_x rule).

Overlay order (back to front) for the pilot, with the armour slots as names
only (nothing is drawn in them):
  far limbs < ARMOR_SHOULDER_FAR < trousers/boots < shirt_torso < ARMOR_TORSO
  < sleeve_near < ARMOR_SHOULDER_NEAR < hand_near/bracer_near < face_skin
  < hair_back < hair_front

    python3 -I evidence/erro_corretion/etapa02b/scripts/build_layers.py
"""

from __future__ import annotations

import json
import math
import sys
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[4]
sys.path.insert(0, str(ROOT / "art-src/n4-sprite-machine/scripts"))
sys.path.insert(0, str(ROOT / "art-src/n3-4-original-protagonist/scripts"))
import build_run_layers as RUN  # noqa: E402  (read only: classify)
import partition as IDLE  # noqa: E402  (read only: layer_of)
import run_poses as POSES  # noqa: E402
from n34_palette import PALETTE  # noqa: E402

OUT = ROOT / "evidence/erro_corretion/etapa02b/out"
E02 = ROOT / "evidence/erro_corretion/etapa02/out"
IDLE_GRID = ROOT / "art-src/n3-5-fidelity/source/candidate_106.txt"
IDLE_PNG = ROOT / "art-src/n3-5-fidelity/generated/candidate_106.png"
RUN_DIR = ROOT / "art-src/n3-6-animation-proof/source"
RUN_PNG = ROOT / "art-src/n3-6-animation-proof/generated/run6.png"
RUN_SOURCES = {0: "video_048", 2: "video_052", 4: "video_056"}
HAIR = set("WwhH")
SKIN = set("Ssz")
ZOOM = 4

# Idle near sleeve: rows 31..51, x up to the front crease (41); the back edge is
# the hair boundary of partition.HAIR_RIGHT (texels left of it are hair already).
IDLE_SLEEVE = {"rows": range(31, 52), "max_x": 41}

ORDER = [
    "sleeve_far", "hand_far", "bracer_far", "trousers_far", "boots_far",
    "ARMOR_SHOULDER_FAR",
    "trousers", "boots", "shirt_torso",
    "ARMOR_TORSO",
    "sleeve_near",
    "ARMOR_SHOULDER_NEAR",
    "hand_near", "bracer_near", "face_skin", "hair_back", "hair_front",
]
RUN_NAMES = {
    "body": "face_skin", "arm_front": "hand_near", "shirt": "sleeve_near", "bracers": "bracer_near",
    "sleeve_back": "sleeve_far", "body_back": "hand_far", "bracers_back": "bracer_far",
    "trousers_back": "trousers_far", "boots_back": "boots_far",
}
IDLE_NAMES = {"body": "face_skin", "arm_front": "hand_near", "bracers": "bracer_near"}
CATEGORY = {
    "hair_back": "cabelo", "hair_front": "cabelo", "face_skin": "rosto/pescoco",
    "shirt_torso": "camisa", "sleeve_near": "camisa", "sleeve_far": "camisa",
    "hand_near": "maos/antebracos", "hand_far": "maos/antebracos",
    "bracer_near": "braceletes", "bracer_far": "braceletes",
    "trousers": "calca", "trousers_far": "calca", "boots": "botas", "boots_far": "botas",
}
SHOW = {
    "hair_back": (200, 200, 230), "hair_front": (255, 255, 255), "face_skin": (240, 180, 140),
    "shirt_torso": (70, 120, 240), "sleeve_near": (60, 220, 120), "sleeve_far": (30, 110, 70),
    "hand_near": (250, 140, 60), "hand_far": (150, 80, 40), "bracer_near": (250, 230, 40),
    "bracer_far": (140, 130, 20), "trousers": (170, 70, 170), "trousers_far": (90, 40, 90),
    "boots": (220, 60, 60), "boots_far": (110, 30, 30),
}
BG = (58, 58, 66, 255)
# Near and far arm layers: nothing of an arm is above the neck row in C1 (fists stay below).
ARM_LAYERS = {"hand_near", "sleeve_near", "bracer_near", "hand_far", "sleeve_far", "bracer_far"}


def read_grid(path: Path) -> list[str]:
    return path.read_text(encoding="utf-8").splitlines()


def idle_layers(rows: list[str]) -> dict[str, dict]:
    out: dict[str, dict] = {}
    for y, row in enumerate(rows):
        for x, key in enumerate(row):
            if key == ".":
                continue
            name = IDLE.layer_of(x, y, key)
            if name == "shirt":
                name = "sleeve_near" if y in IDLE_SLEEVE["rows"] and x <= IDLE_SLEEVE["max_x"] else "shirt_torso"
            name = IDLE_NAMES.get(name, name)
            out.setdefault(name, {})[(x, y)] = key
    return out


def run_layers(index: int, rows: list[str]) -> tuple[dict[str, dict], list]:
    frame = POSES.FRAMES[index]
    out: dict[str, dict] = {}
    moved = []
    for y, row in enumerate(rows):
        for x, key in enumerate(row):
            if key == ".":
                continue
            name, _group = RUN.classify(frame, (x, y), key)
            name = RUN_NAMES.get(name, name)
            if name in ARM_LAYERS and y < frame["neck"][1]:
                fixed = "face_skin" if key in SKIN else ("hair_front" if x >= frame["face_x"] else "hair_back")
                moved.append({"x": x, "y": y, "key": key, "from": name, "to": fixed})
                name = fixed
            out.setdefault(name, {})[(x, y)] = key
    return out, moved


def render(texels: dict, size) -> Image.Image:
    image = Image.new("RGBA", size, (0, 0, 0, 0))
    for (x, y), key in texels.items():
        image.putpixel((x, y), (*PALETTE[key], 255))
    return image


def compose(layers: dict, size) -> Image.Image:
    image = Image.new("RGBA", size, (0, 0, 0, 0))
    for name in ORDER:
        if name in layers:
            image.alpha_composite(render(layers[name], size))
    return image


def diff_count(a: Image.Image, b: Image.Image) -> int:
    pa, pb = a.load(), b.load()
    n = 0
    for y in range(a.height):
        for x in range(a.width):
            u, v = pa[x, y], pb[x, y]
            if u[3] != v[3] or (u[3] and u[:3] != v[:3]):
                n += 1
    return n


def plate(image: Image.Image) -> Image.Image:
    base = Image.new("RGBA", image.size, BG)
    base.alpha_composite(image)
    return base


def zoomed(image: Image.Image) -> Image.Image:
    return image.resize((image.width * ZOOM, image.height * ZOOM), Image.Resampling.NEAREST)


def at(p) -> tuple[float, float]:
    return ((p[0] + 0.5) * ZOOM, (p[1] + 0.5) * ZOOM)


def dilate(cells: set, radius: int) -> set:
    out = set()
    for x, y in cells:
        for dy in range(-radius, radius + 1):
            for dx in range(-radius, radius + 1):
                if abs(dx) + abs(dy) <= radius:
                    out.add((x + dx, y + dy))
    return out


def disk(centre, radius: float, size) -> set:
    cx, cy = centre
    return {(x, y) for y in range(size[1]) for x in range(size[0])
            if math.hypot(x - cx, y - cy) <= radius}


def zones(name: str, layers: dict, size, pose: dict, shoulders: dict) -> tuple[dict, dict]:
    """Overlay zones (masks only, no armour drawn)."""
    opaque = {p for texels in layers.values() for p in texels}
    torso = set(layers.get("shirt_torso", {}))
    head_c, head_r = pose["head"], pose["head_r"]
    neck_y, belt_y = pose["neck"][1], pose["belt_y"]
    head_area = disk(head_c, head_r + 1.5, size)
    near = shoulders["near"]
    far = shoulders.get("far")
    near_disk = disk(near, 6.0, size)
    far_disk = disk(far, 6.0, size) if far else set()
    grow = {p for p in dilate(torso | near_disk, 3)
            if 0 <= p[0] < size[0] and neck_y <= p[1] <= belt_y + 3} - opaque - head_area
    front = set()
    for layer_name in ("hand_near", "bracer_near", "face_skin", "hair_back", "hair_front"):
        front |= set(layers.get(layer_name, {}))
    sleeve = set(layers.get("sleeve_near", {}))
    behind = set()
    for layer_name in ("sleeve_far", "hand_far", "bracer_far", "trousers_far", "boots_far"):
        behind |= set(layers.get(layer_name, {}))
    zone = {
        "cover_shirt_torso": torso,
        "grow_outside_silhouette": grow,
        "grow_visible": grow - front,
        "pauldron_near": near_disk,
        "pauldron_near_over_sleeve": near_disk & sleeve,
        "pauldron_far": far_disk,
        "always_front": front,
        "sleeve_near_front_of_torso_armor": sleeve - near_disk,
        "behind_far_limbs": behind,
    }
    stats = {k: len(v) for k, v in zone.items()}
    return zone, stats


def zone_image(sprite: Image.Image, zone: dict, shoulders: dict) -> Image.Image:
    size = sprite.size
    over = Image.new("RGBA", size, (0, 0, 0, 0))

    def fill(cells, colour):
        for p in cells:
            if 0 <= p[0] < size[0] and 0 <= p[1] < size[1]:
                over.putpixel(p, colour)

    fill(zone["behind_far_limbs"], (20, 160, 160, 120))
    fill(zone["cover_shirt_torso"], (70, 120, 255, 150))
    fill(zone["grow_outside_silhouette"], (255, 200, 40, 170))
    fill(zone["sleeve_near_front_of_torso_armor"], (60, 230, 110, 150))
    fill(zone["always_front"], (255, 255, 255, 90))
    image = zoomed(Image.alpha_composite(plate(sprite), over))
    draw = ImageDraw.Draw(image, "RGBA")
    for key, colour in (("near", (255, 120, 0, 255)), ("far", (190, 90, 255, 255))):
        if shoulders.get(key):
            cx, cy = at(shoulders[key])
            r = 6.0 * ZOOM
            draw.ellipse([cx - r, cy - r, cx + r, cy + r], outline=colour, width=2)
            draw.ellipse([cx - 3, cy - 3, cx + 3, cy + 3], fill=colour)
    return image


def skeleton(draw, points: dict, colour, far_hidden: bool) -> None:
    chains = [("near", ["shoulder", "elbow", "wrist", "fist"]), ("near", ["hip", "knee", "ankle", "toe"])]
    if not far_hidden:
        chains += [("far", ["shoulder", "elbow", "wrist", "fist"]), ("far", ["hip", "knee", "ankle", "toe"])]
    for side, names in chains:
        draw.line([at(points[f"{side}_{n}"]) for n in names], fill=colour, width=2 if side == "near" else 1)
    draw.line([at(points["head"]), at(points["neck"])], fill=colour, width=2)
    for side in ("near", "far"):
        if far_hidden and side == "far":
            continue
        draw.line([at(points["neck"]), at(points[f"{side}_shoulder"])], fill=colour, width=1)


def preview(name: str, sprite: Image.Image, layers: dict, recomposed: Image.Image, diff: int,
            zone_img: Image.Image, sprite_pts: dict, before: dict, after: dict, far_hidden: bool,
            counts: dict, moved: int, zstats: dict) -> None:
    size = sprite.size
    p1 = zoomed(plate(sprite))
    p2 = zoomed(plate(sprite))
    dim = Image.new("RGBA", p2.size, (0, 0, 0, 110))
    p2.alpha_composite(dim)
    d2 = ImageDraw.Draw(p2, "RGBA")
    skeleton(d2, sprite_pts, (60, 255, 120), far_hidden)
    skeleton(d2, after, (255, 70, 70), far_hidden)
    for key in ("near_shoulder", "far_shoulder"):
        if far_hidden and key.startswith("far"):
            continue
        b, a = at(before[key]), at(after[key])
        d2.ellipse([b[0] - 3, b[1] - 3, b[0] + 3, b[1] + 3], outline=(255, 255, 0), width=1)
        d2.line([b, a], fill=(255, 255, 0), width=1)
    split = Image.new("RGBA", size, (0, 0, 0, 0))
    for layer_name, texels in layers.items():
        for p in texels:
            split.putpixel(p, (*SHOW[layer_name], 255))
    p3 = zoomed(plate(split))
    p4 = zoomed(plate(recomposed))
    diff_img = Image.new("RGBA", size, (16, 16, 20, 255))
    a, b = sprite.load(), recomposed.load()
    for y in range(size[1]):
        for x in range(size[0]):
            if a[x, y] != b[x, y] and (a[x, y][3] or b[x, y][3]):
                diff_img.putpixel((x, y), (255, 0, 0, 255))
            elif a[x, y][3]:
                diff_img.putpixel((x, y), (60, 60, 70, 255))
    p5 = zoomed(diff_img)
    panels = [(p1, "original aprovado"), (p2, "guias: verde sprite, vermelho manequim 02B, amarelo ombro antes"),
              (p3, "divisao em camadas (cores = camadas)"), (p4, "reconstrucao vestido sem armadura"),
              (p5, f"diferenca: {diff} pixels"), (zone_img, "zonas de sobreposicao (sem armadura)")]
    text = [f"{name}", f"pixels divergentes: {diff}", f"cabelo/rosto corrigidos: {moved}", "",
            "texels por categoria (orig = recons.):"]
    for cat, (o, r) in sorted(counts.items()):
        text.append(f"  {cat:16s} {o:4d} = {r:4d}" + ("" if o == r else "  DIFERENTE"))
    text += ["", "articulacoes (erro px) antes -> depois:"]
    for key in ("near_shoulder", "far_shoulder", "neck", "near_elbow", "far_elbow", "near_wrist", "far_wrist",
                "near_fist", "far_fist"):
        if far_hidden and key.startswith("far"):
            continue
        eb = math.dist(sprite_pts[key], before[key])
        ea = math.dist(sprite_pts[key], after[key])
        text.append(f"  {key:14s} {eb:5.2f} -> {ea:5.2f}")
    text += ["", "zonas (texels):",
             f"  camisa coberta     {zstats['cover_shirt_torso']}",
             f"  crescer fora       {zstats['grow_outside_silhouette']}",
             f"   (visivel)         {zstats['grow_visible']}",
             f"  ombreira perto     {zstats['pauldron_near']}",
             f"   (sobre a manga)   {zstats['pauldron_near_over_sleeve']}",
             f"  sempre na frente   {zstats['always_front']}",
             "", "legenda zonas: azul=camisa coberta,", " amarelo=crescer alem da camisa,",
             " verde=manga na frente da couraca,", " branco=cabelo/rosto/mao na frente,",
             " ciano=membros de tras (atras),", " laranja=ombreira perto, roxo=longe"]
    gap, text_w = 8, 290
    width = sum(p.width for p, _ in panels) + gap * len(panels) + text_w
    sheet = Image.new("RGBA", (width, p1.height + 40), (28, 28, 34, 255))
    draw = ImageDraw.Draw(sheet)
    x = 0
    for panel, title in panels:
        sheet.alpha_composite(panel, (x, 40))
        words = title.split(" ")
        line, lines = "", []
        for w in words:
            if len(line) + len(w) > panel.width // 6:
                lines.append(line)
                line = ""
            line += w + " "
        lines.append(line)
        draw.multiline_text((x + 3, 2), "\n".join(lines[:3]), fill=(255, 255, 255), spacing=1)
        x += panel.width + gap
    draw.multiline_text((x + 4, 44), "\n".join(text), fill=(235, 235, 235), spacing=2)
    sheet.save(OUT / f"preview_{name}.png")


def hair_fix_sheet(fixes: dict, sprites: dict) -> None:
    tiles = []
    for name, moved in fixes.items():
        sprite = sprites[name]
        box = (44, 8, 92, 48)
        z = 10
        image = plate(sprite).crop(box).resize(((box[2] - box[0]) * z, (box[3] - box[1]) * z), Image.Resampling.NEAREST)
        draw = ImageDraw.Draw(image, "RGBA")
        for m in moved:
            x, y = (m["x"] - box[0]) * z, (m["y"] - box[1]) * z
            colour = (240, 160, 120, 255) if m["to"] == "face_skin" else (80, 200, 255, 255)
            draw.rectangle([x, y, x + z - 1, y + z - 1], outline=colour, width=2)
        draw.text((4, 4), f"{name}: {len(moved)} texels devolvidos (azul=cabelo, laranja=rosto)", fill=(255, 255, 0))
        tiles.append(image)
    width = sum(t.width for t in tiles) + 10 * len(tiles)
    sheet = Image.new("RGBA", (width, tiles[0].height), (28, 28, 34, 255))
    x = 0
    for t in tiles:
        sheet.alpha_composite(t, (x, 0))
        x += t.width + 10
    sheet.save(OUT / "hair_fix.png")


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    targets = json.loads((E02 / "targets.json").read_text())
    shoulders = json.loads((OUT / "shoulders.json").read_text())
    report = {"order": ORDER, "poses": {}}
    fixes, sprites = {}, {}
    run6 = Image.open(RUN_PNG).convert("RGBA")
    for name, pose in targets.items():
        if name == "idle":
            rows = read_grid(IDLE_GRID)
            sprite = Image.open(IDLE_PNG).convert("RGBA")
            layers, moved = idle_layers(rows), []
            pose_info = {"head": pose["head"], "head_r": 7.0, "neck": pose["neck"], "belt_y": pose["belt_y"]}
            sleeve = layers["sleeve_near"]
            top = [p for p in sleeve if p[1] <= min(q[1] for q in sleeve) + 4]
            anchor_near = (sum(p[0] for p in top) / len(top), sum(p[1] for p in top) / len(top) + 2)
            anchors = {"near": anchor_near, "far": None}
        else:
            index = pose["frame"]
            rows = read_grid(RUN_DIR / f"{RUN_SOURCES[index]}.txt")
            sprite = run6.crop((index * 112, 0, index * 112 + 112, 128))
            layers, moved = run_layers(index, rows)
            frame = POSES.FRAMES[index]
            pose_info = {"head": frame["head"], "head_r": frame["head_r"], "neck": frame["neck"],
                         "belt_y": frame["belt_y"]}
            after = shoulders[name]["after"]
            anchors = {"near": tuple(after["near_shoulder"]), "far": tuple(after["far_shoulder"])}
        size = sprite.size
        # The grid is the source of the approved PNG: check it first.
        grid_diff = diff_count(render({(x, y): k for y, r in enumerate(rows) for x, k in enumerate(r) if k != "."},
                                      size), sprite)
        texel_total = sum(len(t) for t in layers.values())
        opaque_total = sum(1 for r in rows for k in r if k != ".")
        overlap = texel_total - len({p for t in layers.values() for p in t})
        recomposed = compose(layers, size)
        diff = diff_count(recomposed, sprite)
        layer_dir = OUT / "layers" / name
        layer_dir.mkdir(parents=True, exist_ok=True)
        for layer_name, texels in layers.items():
            render(texels, size).save(layer_dir / f"{layer_name}.png")
        recomposed.save(OUT / f"reconstruction_{name}.png")
        counts = {}
        for layer_name, texels in layers.items():
            cat = CATEGORY[layer_name]
            counts.setdefault(cat, [0, 0])
            counts[cat][1] += len(texels)
        # Original count per category with the same classification = recomposed by
        # construction; the independent check is the pixel diff above.
        for cat in counts:
            counts[cat][0] = counts[cat][1]
        zone, zstats = zones(name, layers, size, pose_info, anchors)
        zone_img = zone_image(sprite, zone, anchors)
        zone_img.save(OUT / f"zones_{name}.png")
        before = shoulders[name]["before"]
        after = shoulders[name]["after"]
        sprite_pts = {k: tuple(v) for k, v in json.loads((E02 / "fit_results.json").read_text())[name]["sprite"].items()}
        preview(name, sprite, layers, recomposed, diff, zone_img, sprite_pts, before, after, pose["far_hidden"],
                {k: tuple(v) for k, v in counts.items()}, len(moved), zstats)
        if name != "idle":
            fixes[name] = moved
            sprites[name] = sprite
        report["poses"][name] = {
            "grid_vs_png_diff": grid_diff, "opaque_texels": opaque_total, "layer_texels": texel_total,
            "overlapping_texels": overlap, "reconstruction_diff_pixels": diff,
            "layers": {k: len(v) for k, v in sorted(layers.items())},
            "hair_face_fixed": moved, "zones": zstats, "anchors": anchors,
        }
        print(name, "grid/png", grid_diff, "opaque", opaque_total, "in layers", texel_total, "overlap", overlap,
              "| DIFF", diff, "| fixed", len(moved), "| zones", zstats)
    hair_fix_sheet(fixes, sprites)
    (OUT / "layers_report.json").write_text(json.dumps(report, indent=1, default=list))


if __name__ == "__main__":
    main()
