"""Etapa 02 (Erro-Corretion): evidence sheets and measurements of the rig fit.

Inputs (read only): out/fit_results.json and out/guide_*.png from fit_rig.py, the
approved sprites and the N4 layers. Outputs: out/fit_<pose>.png (side by side +
overlay), out/occlusion_<pose>.png, out/summary.png, out/measurements.json.

    python3 -I evidence/erro_corretion/etapa02/scripts/analyze.py
"""

from __future__ import annotations

import json
import math
from collections import Counter
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[4]
OUT = ROOT / "evidence/erro_corretion/etapa02/out"
LAYERS = ROOT / "art-src/n4-sprite-machine/stage/parts"
SUPER = 8
ZOOM = 5
GRID = 8
BG = (58, 58, 66, 255)
LABEL_COLOURS = {
    (255, 0, 0): "torso", (255, 255, 0): "head", (0, 255, 0): "near_arm",
    (0, 0, 255): "far_arm", (255, 0, 255): "near_leg", (0, 255, 255): "far_leg",
}
SHOW = {"torso": (230, 70, 70), "head": (230, 210, 60), "near_arm": (70, 220, 90),
        "far_arm": (80, 120, 240), "near_leg": (220, 80, 220), "far_leg": (60, 200, 210)}
SPRITE_PT = (60, 255, 120)
RIG_PT = (255, 70, 70)


def reduce_labels(path: Path) -> dict[tuple[int, int], str]:
    """8x guide -> 1x: opaque when >= 50 % of the block is covered, majority label."""
    big = Image.open(path).convert("RGBA")
    px = big.load()
    out = {}
    for y in range(big.height // SUPER):
        for x in range(big.width // SUPER):
            votes = Counter()
            for dy in range(SUPER):
                for dx in range(SUPER):
                    r, g, b, a = px[x * SUPER + dx, y * SUPER + dy]
                    if a >= 128:
                        key = min(LABEL_COLOURS, key=lambda c: abs(c[0] - r) + abs(c[1] - g) + abs(c[2] - b))
                        votes[LABEL_COLOURS[key]] += 1
            if sum(votes.values()) * 2 >= SUPER * SUPER:
                out[(x, y)] = votes.most_common(1)[0][0]
    return out


def sprite_of(pose: dict) -> Image.Image:
    image = Image.open(ROOT / pose["sprite_path"]).convert("RGBA")
    if pose["canvas"][0] == 112:
        cell = pose["sprite_cell"]
        image = image.crop((cell * 112, 0, cell * 112 + 112, 128))
    return image


def layer(name: str, pose: dict) -> set[tuple[int, int]]:
    if pose["canvas"][0] == 80:
        path = LAYERS / "idle" / f"{name}.png"
        if not path.exists():
            return set()
        image = Image.open(path).convert("RGBA")
    else:
        path = LAYERS / "run_c1" / f"{name}.png"
        if not path.exists():
            return set()
        column = pose["frame"] // 2
        image = Image.open(path).convert("RGBA").crop((column * 112, 0, column * 112 + 112, 128))
    alpha = image.getchannel("A").load()
    return {(x, y) for y in range(image.height) for x in range(image.width) if alpha[x, y]}


def plate(image: Image.Image) -> Image.Image:
    base = Image.new("RGBA", image.size, BG)
    base.alpha_composite(image)
    return base


def zoomed(image: Image.Image) -> Image.Image:
    return image.resize((image.width * ZOOM, image.height * ZOOM), Image.Resampling.NEAREST)


def grid(draw: ImageDraw.ImageDraw, size: tuple[int, int], pivot: tuple[int, int]) -> None:
    w, h = size
    for x in range(0, w + 1, GRID):
        draw.line([(x * ZOOM, 0), (x * ZOOM, h * ZOOM)], fill=(255, 255, 255, 40))
    for y in range(0, h + 1, GRID):
        draw.line([(0, y * ZOOM), (w * ZOOM, y * ZOOM)], fill=(255, 255, 255, 40))
    px, py = (pivot[0] + 0.5) * ZOOM, (pivot[1] + 1) * ZOOM
    draw.line([(0, py), (w * ZOOM, py)], fill=(255, 220, 0, 200))
    draw.line([(px, py - 12), (px, py + 12)], fill=(255, 220, 0, 255), width=2)
    draw.line([(px - 12, py), (px + 12, py)], fill=(255, 220, 0, 255), width=2)


def at(point) -> tuple[float, float]:
    return ((point[0] + 0.5) * ZOOM, (point[1] + 0.5) * ZOOM)


def labels_image(labels: dict, size, alpha: int = 255) -> Image.Image:
    image = Image.new("RGBA", size, (0, 0, 0, 0))
    for (x, y), name in labels.items():
        image.putpixel((x, y), (*SHOW[name], alpha))
    return image


CHAINS = [("near", ["shoulder", "elbow", "wrist", "fist"]), ("far", ["shoulder", "elbow", "wrist", "fist"]),
          ("near", ["hip", "knee", "ankle", "toe"]), ("far", ["hip", "knee", "ankle", "toe"])]
SHORT = {"head": "cab", "neck": "pesc", "shoulder": "omb", "elbow": "cot", "wrist": "pulso",
         "fist": "mao", "hip": "quad", "knee": "joel", "ankle": "torn", "toe": "pe"}


def skeleton(draw, points: dict, colour, far_hidden: bool) -> None:
    for side, names in CHAINS:
        if far_hidden and side == "far":
            continue
        width = 2 if side == "near" else 1
        draw.line([at(points[f"{side}_{n}"]) for n in names], fill=colour, width=width)
    draw.line([at(points["head"]), at(points["neck"])], fill=colour, width=2)


def fit_sheet(name: str, result: dict, pose: dict, labels: dict) -> dict:
    size = tuple(pose["canvas"])
    sprite = sprite_of(pose)
    guide = labels_image(labels, size)
    hidden = result["far_hidden"]
    errors = {}
    for key, target in result["sprite"].items():
        if hidden and key.startswith("far_"):
            continue
        rig = result["rig"][key]
        errors[key] = {"sprite": target, "rig": rig, "dx": round(rig[0] - target[0], 2),
                       "dy": round(rig[1] - target[1], 2), "err": round(math.dist(target, rig), 2)}
    errors["waist"] = {"sprite_y": result["belt_y"], "rig_y": result["rig"]["waist"][1],
                       "dy": round(result["rig"]["waist"][1] - result["belt_y"], 2)}

    panels = []
    a = zoomed(plate(sprite)); da = ImageDraw.Draw(a, "RGBA"); grid(da, size, pose["pivot"])
    skeleton(da, result["sprite"], SPRITE_PT, hidden)
    panels.append((a, "sprite aprovado + esqueleto (verde)"))
    b = zoomed(plate(guide)); db = ImageDraw.Draw(b, "RGBA"); grid(db, size, pose["pivot"])
    skeleton(db, result["rig"], RIG_PT, hidden)
    panels.append((b, "guia do rig 1x (BOX, alpha>=50%) + esqueleto (vermelho)"))
    c = plate(sprite)
    c.alpha_composite(labels_image(labels, size, 110))
    c = zoomed(c); dc = ImageDraw.Draw(c, "RGBA"); grid(dc, size, pose["pivot"])
    skeleton(dc, result["sprite"], SPRITE_PT, hidden)
    skeleton(dc, result["rig"], RIG_PT, hidden)
    for key, e in errors.items():
        if key == "waist":
            continue
        s, r = at(e["sprite"]), at(e["rig"])
        dc.line([s, r], fill=(255, 255, 255, 230), width=1)
        dc.ellipse([s[0] - 3, s[1] - 3, s[0] + 3, s[1] + 3], fill=SPRITE_PT)
        dc.ellipse([r[0] - 3, r[1] - 3, r[0] + 3, r[1] + 3], outline=RIG_PT, width=2)
        if e["err"] >= 2.0:
            part = key.split("_", 1)[-1]
            tag = ("p:" if key.startswith("near") else "l:" if key.startswith("far") else "") + SHORT[part]
            dc.text((r[0] + 5, r[1] - 5), f"{tag} {e['err']:.1f}", fill=(255, 255, 255, 255))
    wy = (result["belt_y"] + 0.5) * ZOOM
    dc.line([(0, wy), (c.width, wy)], fill=(60, 255, 120, 120))
    ry = (result["rig"]["waist"][1] + 0.5) * ZOOM
    dc.line([(0, ry), (c.width, ry)], fill=(255, 70, 70, 120))
    panels.append((c, "sobreposicao: verde=sprite, vermelho=rig, branco=erro (px)"))

    table = sorted(((k, v) for k, v in errors.items() if k != "waist"), key=lambda kv: -kv[1]["err"])
    lines = [f"{name}  escala {result['scale_px_per_m']:.2f} px/m  pivo {tuple(pose['pivot'])}",
             f"torcao peito {result['chest_twist_deg']} graus, pelve {result['pelvis_twist_deg']} graus",
             f"desloc. raiz vs quadril travado {result['root_offset_from_hip_lock_px']} px",
             f"cintura: sprite y={result['belt_y']} rig y={result['rig']['waist'][1]:.1f}",
             "", "marco            dx     dy    erro"]
    for key, e in table:
        lines.append(f"{key:15s} {e['dx']:+5.1f} {e['dy']:+6.1f} {e['err']:6.2f}")
    values = [e["err"] for k, e in errors.items() if k != "waist"]
    lines += ["", f"media {sum(values) / len(values):.2f} px  max {max(values):.2f} px",
              f"<=2 px: {sum(v <= 2 for v in values)}/{len(values)}  <=4 px: {sum(v <= 4 for v in values)}/{len(values)}"]
    if hidden:
        lines.append("idle: membros de tras ocultos (nao medidos)")
    text_w = 330
    width = sum(p.width for p, _ in panels) + 10 * len(panels) + text_w
    sheet = Image.new("RGBA", (width, panels[0][0].height + 26), (28, 28, 34, 255))
    draw = ImageDraw.Draw(sheet)
    x = 0
    for panel, title in panels:
        sheet.alpha_composite(panel, (x, 26))
        draw.text((x + 4, 7), title, fill=(255, 255, 255))
        x += panel.width + 10
    draw.multiline_text((x + 6, 30), "\n".join(lines), fill=(235, 235, 235), spacing=3)
    sheet.save(OUT / f"fit_{name}.png")
    return {"errors": errors, "mean": round(sum(values) / len(values), 2), "max": round(max(values), 2),
            "within_2px": sum(v <= 2 for v in values), "within_4px": sum(v <= 4 for v in values),
            "count": len(values)}


def dilate(cells: set, radius: int) -> set:
    out = set()
    for x, y in cells:
        for dy in range(-radius, radius + 1):
            for dx in range(-radius, radius + 1):
                if abs(dx) + abs(dy) <= radius:
                    out.add((x + dx, y + dy))
    return out


def torso_area(pose: dict) -> set[tuple[int, int]]:
    """Filled torso of the approved pose: run_poses.FRAMES[i]["torso"] polygon
    (run) or reconstruct.TORSO rows (idle). Used to count arm texels over the chest."""
    import importlib.util
    scripts = ROOT / "art-src/n4-sprite-machine/scripts"
    if pose["canvas"][0] == 80:
        spec = importlib.util.spec_from_file_location("rc", scripts / "reconstruct.py")
        rc = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(rc)
        return {(x, y) for y, (a, b) in rc.TORSO.items() for x in range(a, b + 1)}
    spec = importlib.util.spec_from_file_location("rp", scripts / "run_poses.py")
    rp = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(rp)
    mask = Image.new("L", tuple(pose["canvas"]), 0)
    ImageDraw.Draw(mask).polygon(rp.FRAMES[pose["frame"]]["torso"], fill=255)
    px = mask.load()
    return {(x, y) for y in range(mask.height) for x in range(mask.width) if px[x, y]}


def occlusion_sheet(name: str, pose: dict, labels: dict, torso_labels: dict, arm_labels: dict) -> dict:
    size = tuple(pose["canvas"])
    idle = size[0] == 80
    sprite = sprite_of(pose)
    if idle:
        # Idle: no separate sleeve. The near-arm occluder is the interpreted arm
        # (arm_front, partly under the shirt) + bracer; the torso is the shirt minus it.
        arm = layer("arm_front", pose) | layer("bracers", pose)
        torso = layer("shirt", pose) - arm
        front = arm | layer("hair_front", pose) | layer("hair_back", pose)
    else:
        arm = layer("arm_front", pose) | layer("shirt", pose) | layer("bracers", pose)
        torso = layer("shirt_torso", pose)
        front = arm | layer("hair_front", pose) | layer("hair_back", pose)
    rig_arm_visible = {p for p, n in labels.items() if n == "near_arm"}
    rig_arm_full = set(arm_labels)
    rig_torso = set(torso_labels)
    # N4-C1 partition artifact: hair texels above the neck filed under the near-arm
    # layers. Counted, then left out of the arm comparison.
    stray = {p for p in arm if p[1] < pose["neck"][1]}
    arm_clean = arm - stray
    inter = len(arm_clean & rig_arm_visible)
    union = len(arm_clean | rig_arm_visible) or 1
    # Arm crossing the chest: near-arm texels drawn over the torso area.
    sprite_cross = arm & torso_area(pose)
    rig_cross = rig_arm_full & rig_torso
    # External volume test (mask geometry only, not armour): torso grown by 2 texels
    # outside the shirt contour, minus everything that must stay in front.
    grown = dilate(torso, 2)
    body = {(x, y) for y in range(size[1]) for x in range(size[0]) if sprite.getpixel((x, y))[3]}
    outside = grown - body
    visible_plate = grown - front
    hidden_plate = grown & front
    rig_beyond = rig_torso - body
    stats = {
        "sprite_near_arm_texels": len(arm), "rig_near_arm_visible_texels": len(rig_arm_visible),
        "near_arm_iou": round(inter / union, 3), "stray_hair_in_arm_layers": len(stray),
        "rig_arm_inside_sprite_arm": round(inter / (len(rig_arm_visible) or 1), 3),
        "sprite_arm_covered_by_rig": round(inter / (len(arm_clean) or 1), 3),
        "sprite_arm_over_torso_texels": len(sprite_cross), "rig_arm_over_torso_texels": len(rig_cross),
        "torso_texels": len(torso), "grown2_texels": len(grown), "grown2_outside_body": len(outside),
        "grown2_visible_after_occluders": len(visible_plate), "grown2_hidden_by_occluders": len(hidden_plate),
        "rig_torso_texels": len(rig_torso), "rig_torso_outside_sprite_body": len(rig_beyond),
    }

    def paint(cells: dict[tuple[int, int], tuple[int, int, int, int]]) -> Image.Image:
        image = plate(sprite)
        over = Image.new("RGBA", size, (0, 0, 0, 0))
        for p, c in cells.items():
            if 0 <= p[0] < size[0] and 0 <= p[1] < size[1]:
                over.putpixel(p, c)
        image.alpha_composite(over)
        return zoomed(image)

    p1 = paint({**{p: (70, 220, 90, 150) for p in arm}, **{p: (255, 60, 60, 200) for p in rig_arm_visible - arm},
                **{p: (255, 255, 255, 170) for p in rig_arm_visible & arm}})
    p2 = paint({**{p: (230, 70, 70, 120) for p in rig_torso}, **{p: (60, 140, 255, 140) for p in torso}})
    p3 = paint({**{p: (255, 200, 40, 200) for p in visible_plate}, **{p: (150, 60, 200, 200) for p in hidden_plate},
                **{p: (70, 220, 90, 110) for p in arm - grown}})
    titles = ["braco perto: verde=camadas do sprite, branco=rig coincide, vermelho=rig fora",
              "tronco: azul=tronco do sprite, vermelho=volume do tronco do rig",
              "teste +2 texels fora da camisa: amarelo visivel, roxo oculto pelo braco/cabelo"]
    width = sum(p.width for p in (p1, p2, p3)) + 20
    sheet = Image.new("RGBA", (width, p1.height + 40), (28, 28, 34, 255))
    draw = ImageDraw.Draw(sheet)
    x = 0
    for panel, title in zip((p1, p2, p3), titles, strict=True):
        sheet.alpha_composite(panel, (x, 40))
        draw.text((x + 4, 4), title[:62], fill=(255, 255, 255))
        draw.text((x + 4, 18), title[62:], fill=(255, 255, 255))
        x += panel.width + 10
    sheet.save(OUT / f"occlusion_{name}.png")
    return stats


def main() -> None:
    results = json.loads((OUT / "fit_results.json").read_text())
    targets = json.loads((OUT / "targets.json").read_text())
    measurements = {}
    for name, result in results.items():
        pose = {**targets[name], "sprite_path": targets[name]["sprite"]}
        labels = reduce_labels(OUT / f"guide_{name}_all.png")
        torso = reduce_labels(OUT / f"guide_{name}_torso.png")
        arm = reduce_labels(OUT / f"guide_{name}_near_arm.png")
        fit = fit_sheet(name, result, pose, labels)
        occ = occlusion_sheet(name, pose, labels, torso, arm)
        rows = [y for _, y in labels]
        fit["guide_rows"] = [min(rows), max(rows)] if rows else None
        sprite = sprite_of(pose)
        fit["sprite_box"] = sprite.getbbox()
        measurements[name] = {"fit": fit, "occlusion": occ}
        print(name, "mean", fit["mean"], "max", fit["max"], "<=2", fit["within_2px"], "<=4", fit["within_4px"],
              "/", fit["count"], "| iou", occ["near_arm_iou"], "| guide rows", fit["guide_rows"],
              "sprite box", fit["sprite_box"])
    # Segment lengths: rig (constant) vs sprite (projected, per pose).
    first = next(iter(results.values()))
    bones = first["bone_lengths_px"]
    rig_len = {"upper_arm": bones["upper_arm.R"], "forearm": bones["forearm.R"], "thigh": bones["thigh.R"],
               "shin": bones["shin.R"]}
    seg = {"upper_arm": ("shoulder", "elbow"), "forearm": ("elbow", "wrist"), "thigh": ("hip", "knee"),
           "shin": ("knee", "ankle")}
    lengths = {}
    for name, result in results.items():
        for part, (a, b) in seg.items():
            for side in ("near", "far"):
                if result["far_hidden"] and side == "far":
                    continue
                s = result["sprite"]
                lengths.setdefault(part, []).append(round(math.dist(s[f"{side}_{a}"], s[f"{side}_{b}"]), 1))
    measurements["segments"] = {p: {"rig_px": round(rig_len[p], 1), "sprite_projected_px": v,
                                    "sprite_max_px": max(v)} for p, v in lengths.items()}
    for p, v in measurements["segments"].items():
        print("segment", p, v)
    (OUT / "measurements.json").write_text(json.dumps(measurements, indent=1))
    # Summary: the four overlays side by side.
    tiles = [Image.open(OUT / f"fit_{n}.png") for n in results]
    widths = [targets[n]["canvas"][0] * ZOOM for n in results]
    crops = [t.crop((2 * (w + 10), 0, 2 * (w + 10) + w, t.height)) for t, w in zip(tiles, widths, strict=True)]
    width = sum(c.width for c in crops) + 10 * len(crops)
    summary = Image.new("RGBA", (width, max(c.height for c in crops) + 24), (28, 28, 34, 255))
    draw = ImageDraw.Draw(summary)
    x = 0
    for name, crop in zip(results, crops, strict=True):
        summary.alpha_composite(crop, (x, 24))
        fit = measurements[name]["fit"]
        draw.text((x + 4, 4), f"{name}: media {fit['mean']} px, max {fit['max']} px", fill=(255, 255, 255))
        x += crop.width + 10
    summary.save(OUT / "summary.png")


if __name__ == "__main__":
    main()
