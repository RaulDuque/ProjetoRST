"""N4-1 (C1): modular layers of the run frames #0, #2 and #4.

Same guarantees as the idle (build_idle_layers.py):
1. every opaque texel of the approved frame goes to exactly ONE layer
   (nearest limb or region from run_poses.py, material from the palette key
   and the position along the limb); no texel is changed;
2. hidden anatomy (agent's interpretation, guided by the owner's N4-B
   reference) is painted per pose, limb by limb along the pose's bones, only
   where a layer ABOVE covers it in the original;
3. the frames are written as 3-column strips (336x128) per layer, recomposed
   in draw order and compared texel by texel with the originals.

Usage (project root): ``python3 -B art-src/n4-sprite-machine/scripts/build_run_layers.py``
"""

from __future__ import annotations

import json
import math
import sys
from pathlib import Path

from PIL import Image, ImageDraw

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(ROOT / "art-src/n3-4-original-protagonist/scripts"))

import run_poses as R  # noqa: E402
from n34_palette import PALETTE  # noqa: E402

SOURCE = ROOT / "art-src/n3-6-animation-proof/source"
OUT = ROOT / "art-src/n4-sprite-machine/stage/parts/run_c1"
EVIDENCE = ROOT / "docs/development/n4/evidence/n4c"
W, H = 112, 128
FRAMES = [0, 2, 4]
# Back to front; the idle layers keep their relative order (stage/protagonist.ron).
ORDER = [
    "body_back", "trousers_back", "boots_back", "sleeve_back", "bracers_back",
    "body", "trousers", "boots", "shirt_torso", "arm_front", "shirt", "bracers",
    "hair_back", "hair_front",
]
PIECE_OF = {
    "body_back": "body", "body": "body", "arm_front": "body",
    "trousers_back": "trousers", "trousers": "trousers",
    "boots_back": "boots", "boots": "boots",
    "sleeve_back": "shirt", "shirt_torso": "shirt", "shirt": "shirt",
    "bracers_back": "bracers", "bracers": "bracers",
    "hair_back": "hair", "hair_front": "hair",
}
HAIR = set("WwhH")
SKIN = set("Ssz")
LIGHT = (0.6, -0.8)  # from the upper front (image y grows down)


# --- geometry ---------------------------------------------------------------------------


def seg_dist(p, a, b) -> tuple[float, float, float]:
    """Distance from p to segment ab, the parameter t along it, and the
    signed offset (positive towards LIGHT)."""
    ax, ay = a
    bx, by = b
    dx, dy = bx - ax, by - ay
    ll = dx * dx + dy * dy or 1e-9
    t = max(0.0, min(1.0, ((p[0] - ax) * dx + (p[1] - ay) * dy) / ll))
    cx, cy = ax + t * dx, ay + t * dy
    ox, oy = p[0] - cx, p[1] - cy
    d = math.hypot(ox, oy)
    nx, ny = -dy, dx
    nl = math.hypot(nx, ny) or 1.0
    nx, ny = nx / nl, ny / nl
    if nx * LIGHT[0] + ny * LIGHT[1] < 0:
        nx, ny = -nx, -ny
    return d, t, ox * nx + oy * ny


def inside_poly(p, poly) -> bool:
    x, y = p
    inside = False
    j = len(poly) - 1
    for i in range(len(poly)):
        xi, yi = poly[i]
        xj, yj = poly[j]
        if (yi > y) != (yj > y) and x < (xj - xi) * (y - yi) / (yj - yi + 1e-9) + xi:
            inside = not inside
        j = i
    return inside


def limb_segments(f: dict, side: str) -> dict[str, tuple]:
    arm = f[f"{side}_arm"]
    leg = f[f"{side}_leg"]
    r = R.RADII
    return {
        f"{side}_upper_arm": (arm[0], arm[1], r["upper_arm"]),
        f"{side}_forearm": (arm[1], arm[2], r["forearm"]),
        f"{side}_fist": (arm[2], arm[3], r["fist"]),
        f"{side}_thigh": (leg[0], leg[1], r["thigh"]),
        f"{side}_shin": (leg[1], leg[2], r["shin"]),
        f"{side}_foot": (f[f"{side}_heel"], leg[3], r["foot"]),
    }


# --- partition ----------------------------------------------------------------------------


def classify(f: dict, p, key: str) -> tuple[str, str]:
    """(layer, group) of an original texel. The group is the body part the
    texel dresses: head, hair, torso, near_arm, far_arm, near_leg, far_leg."""
    hx, hy = f["head"]
    in_head = math.hypot(p[0] - hx, p[1] - hy) <= f["head_r"]
    in_torso = inside_poly(p, f["torso"])
    near = limb_segments(f, "near")
    far = limb_segments(f, "far")
    near_arm_hit = min(
        (seg_dist(p, a, b)[0] - r, name) for name, (a, b, r) in near.items() if "arm" in name or "fist" in name
    )
    # Head, face and hair.
    if in_head:
        if key in HAIR or (p[0] < f["face_x"] and key not in SKIN):
            return ("hair_front" if p[0] >= f["face_x"] else "hair_back"), "hair"
        return "body", "head"
    if p[1] <= 42 and not in_torso and near_arm_hit[0] > 1.0 and p[0] < f["head"][0] + 4:
        return "hair_back", "hair"
    if in_torso and p[1] < f["torso"][0][1] + 3 and key in HAIR:
        return "hair_back", "hair"
    # The near arm is drawn over the torso.
    if near_arm_hit[0] <= 1.0:
        return arm_material(f, "near", near_arm_hit[1], p, key), "near_arm"
    if in_torso:
        if p[1] >= f["belt_y"]:
            return "trousers", "torso"
        return ("body" if key in SKIN and p[1] < f["neck"][1] + 6 else "shirt_torso"), "torso"
    # Legs and the far arm: nearest surface.
    best = min(
        (seg_dist(p, a, b)[0] - r, name, side)
        for side, segs in (("near", near), ("far", far))
        for name, (a, b, r) in segs.items()
    )
    _, name, side = best
    if "arm" in name or "fist" in name:
        return arm_material(f, side, name, p, key), f"{side}_arm"
    return leg_material(f, side, name, p), f"{side}_leg"


def arm_material(f: dict, side: str, name: str, p, key: str) -> str:
    wrist = f[f"{side}_arm"][2]
    near = side == "near"
    if key in SKIN:
        return "arm_front" if near else "body_back"
    # The bracer band is checked before the fist (its dark texels are next
    # to the fist, and they belong to the bracer, not to the hand).
    if math.hypot(p[0] - wrist[0], p[1] - wrist[1]) <= 2.6:
        return "bracers" if near else "bracers_back"
    if name.endswith("fist"):
        return "arm_front" if near else "body_back"
    return "shirt" if near else "sleeve_back"


def leg_material(f: dict, side: str, name: str, p) -> str:
    near = side == "near"
    boot = name.endswith("foot")
    if name.endswith("shin"):
        a, b = f[f"{side}_leg"][1], f[f"{side}_leg"][2]
        boot = seg_dist(p, a, b)[1] >= f[f"boot_t_{side}"]
    if boot:
        return "boots" if near else "boots_back"
    return "trousers" if near else "trousers_back"


# --- anatomy ------------------------------------------------------------------------------


def skin_key(t: float) -> str:
    if t < 0.18:
        return "z"
    if t < 0.4:
        return "s"
    if t < 0.86:
        return "S"
    return "s"


def capsule_cells(a, b, r: float) -> set[tuple[int, int]]:
    out = set()
    x0, x1 = int(min(a[0], b[0]) - r - 1), int(max(a[0], b[0]) + r + 2)
    y0, y1 = int(min(a[1], b[1]) - r - 1), int(max(a[1], b[1]) + r + 2)
    for y in range(max(0, y0), min(H, y1)):
        for x in range(max(0, x0), min(W, x1)):
            if seg_dist((x, y), a, b)[0] <= r:
                out.add((x, y))
    return out


def erode(cells: set, keep_bottom: bool = False) -> set:
    """One texel inside a dressed silhouette (clothes have a little room).
    With ``keep_bottom`` the lowest texels stay, so feet keep the ground."""
    out = set()
    for (x, y) in cells:
        n = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        if all(q in cells for q in n) or (keep_bottom and (x, y + 1) not in cells and all(q in cells for q in n[:2])):
            out.add((x, y))
    return out


def shade_along(cells: set, segs: list) -> dict[tuple[int, int], str]:
    """Shade each texel across the bone it is nearest to (back dark, lit
    middle, slightly darker front), like the approved forearm and face."""
    out = {}
    for p in cells:
        d, a, b, r = min((seg_dist(p, a, b)[0], a, b, r) for a, b, r in segs)
        off = seg_dist(p, a, b)[2]
        out[p] = skin_key(max(0.0, min(1.0, (off / (r + 1.5) + 1) / 2)))
    return out


def outline(shape: dict, keep_from: set) -> None:
    """Edges away from the light become K, edges towards it z; edges against
    ``keep_from`` (original skin of the same part) stay as shaded."""
    s = set(shape) | keep_from
    for (x, y) in list(shape):
        if (x - 1, y) not in s or (x, y + 1) not in s:
            shape[(x, y)] = "K"
        elif (x + 1, y) not in s or (x, y - 1) not in s:
            shape[(x, y)] = "z"


def group_cells(groups: dict, name: str) -> set:
    return {p for p, g in groups.items() if g == name}


# Half-thickness limits (texels): the bone core always drawn, and the most
# a limb may take from the dressed silhouette.
FAR_TONE = {"S": "s", "s": "z", "z": "K"}
CORE_R = {"upper_arm": 1.6, "forearm": 1.4, "fist": 1.6, "thigh": 2.8, "shin": 2.0, "foot": 1.4}
MAX_R = {"upper_arm": 3.0, "forearm": 2.6, "fist": 2.8, "thigh": 4.6, "shin": 3.6, "foot": 3.2}


def limb_shape(f: dict, side: str, kind: str, groups: dict, skin: set) -> dict:
    """A limb as the eroded dressed silhouette of that limb plus a bone core
    (so it continues behind other limbs), shaded across its bones."""
    segs = limb_segments(f, side)
    names = ["upper_arm", "forearm", "fist"] if kind == "arm" else ["thigh", "shin", "foot"]
    bones = [segs[f"{side}_{n}"] for n in names]
    dressed = group_cells(groups, f"{side}_{kind}")
    cells = erode(dressed, keep_bottom=(kind == "leg"))
    # Never thicker than a limb: where both legs (or limb and torso) merge
    # in the dressed silhouette, the eroded mass is capped along the bones.
    cap = set()
    for n, (a, b, _) in zip(names, bones):
        cap |= capsule_cells(a, b, MAX_R[n])
    cells &= cap
    for n, (a, b, _) in zip(names, bones):
        cells |= capsule_cells(a, b, CORE_R[n])
    cells -= skin
    shape = shade_along(cells, bones)
    if kind == "leg":
        # N4-B reference: a shadow along the back of the thigh and the calf,
        # light on the front of the shin, kneecap highlight.
        thigh, shin = bones[0], bones[1]
        for p in list(shape):
            d, t, off = seg_dist(p, thigh[0], thigh[1])
            if d <= thigh[2] + 1 and 0.3 < t < 0.85 and -0.7 * thigh[2] < off < -0.35 * thigh[2]:
                shape[p] = "z"
            d, t, off = seg_dist(p, shin[0], shin[1])
            if d <= shin[2] + 1 and 0.05 < t < 0.45 and off < -0.35 * shin[2]:
                shape[p] = "z"
        knee = f[f"{side}_leg"][1]
        if knee in shape:
            shape[knee] = "S"
    outline(shape, skin)
    if side == "far":
        # The far limbs are in shadow in the approved frames: one tone darker.
        shape = {p: FAR_TONE.get(k, k) for p, k in shape.items()}
    return shape


def torso_shape(f: dict, groups: dict, skin: set) -> dict:
    """Torso, neck and skull: the eroded dressed torso and a rounded skull
    under the hair, shaded per row (back dark, chest lit)."""
    # The pose's torso polygon (the dressed torso texels are split between
    # the torso, the near arm drawn over it and the hair on the back).
    poly = f["torso"]
    dressed = {(x, y) for y in range(H) for x in range(W)
               if inside_poly((x, y), poly) and y < f["belt_y"]}
    cells = set(dressed)
    hx, hy = f["head"]
    skull = {(x, y) for y in range(H) for x in range(W)
             if math.hypot((x - hx + 1) / (f["head_r"] + 0.5), (y - hy) / (f["head_r"] + 0.5)) <= 1}
    neck = capsule_cells(f["neck"], ((f["shoulder_near"][0] + f["shoulder_far"][0]) / 2, f["neck"][1] + 5), 2.6)
    cells |= skull | neck
    cells -= skin
    shape = {}
    rows = {}
    for (x, y) in cells:
        rows.setdefault(y, []).append(x)
    for (x, y) in cells:
        a, b = min(rows[y]), max(rows[y])
        shape[(x, y)] = skin_key((x - a) / max(1, b - a))
    # Chest and abdomen (N4-B reference): pectoral fold, abdominal line.
    top = min(p[1] for p in dressed) if dressed else hy
    for (x, y) in list(shape):
        if y == top + 10 and x > max(rows[y]) - 6:
            shape[(x, y)] = "z"
        if top + 12 <= y < f["belt_y"] - 1 and x == max(rows[y]) - 3 and y % 3:
            shape[(x, y)] = "s"
    outline(shape, skin)
    return shape


def underwear(f: dict, groups: dict) -> dict[tuple[int, int], str]:
    """Neutral dark-grey undergarment over the hips (owner's decision pending)."""
    hx, hy = f["hip"]
    dressed = group_cells(groups, "torso") | group_cells(groups, "near_leg") | group_cells(groups, "far_leg")
    out = {}
    for (x, y) in dressed:
        if f["belt_y"] - 1 <= y <= hy + 5 and abs(x - hx) <= 9:
            out[(x, y)] = "q"
    out = {p: k for p, k in out.items() if p in erode(set(out)) or p[1] == f["belt_y"] - 1}
    for (x, y) in list(out):
        if y == f["belt_y"] - 1:
            out[(x, y)] = "l"
        elif (x, y + 1) not in out or (x - 1, y) not in out or (x + 1, y) not in out:
            out[(x, y)] = "K"
        elif (x + y) % 5 == 0:
            out[(x, y)] = "g"
    return out


def shirt_under_hair(f: dict, layers: dict) -> dict[tuple[int, int], str]:
    """The shirt continues under the hair over the back and shoulders: the
    nearest visible shirt texel of the row gives the colour."""
    out = {}
    shirt = layers["shirt_torso"]
    for (x, y) in layers["hair_back"]:
        if inside_poly((x, y), f["torso"]) and y > f["neck"][1]:
            row = [(abs(sx - x), k) for (sx, sy), k in shirt.items() if sy == y]
            if row:
                out[(x, y)] = min(row)[1]
    return out


def paint(f: dict, groups: dict, layers: dict) -> dict[str, dict]:
    skin_body = set(layers["body"])
    skin_near = set(layers["arm_front"])
    skin_far = set(layers["body_back"])
    body = torso_shape(f, groups, skin_body)
    body.update(underwear(f, groups))
    body.update(limb_shape(f, "near", "leg", groups, skin_body))
    far = limb_shape(f, "far", "leg", groups, skin_far)
    far.update(limb_shape(f, "far", "arm", groups, skin_far))
    return {
        "body_back": far,
        "body": body,
        "arm_front": limb_shape(f, "near", "arm", groups, skin_near),
        "shirt_torso": shirt_under_hair(f, layers),
    }


# --- build --------------------------------------------------------------------------------


def build_frame(i: int) -> tuple[dict[str, dict], dict]:
    f = R.FRAMES[i]
    rows = (SOURCE / f"{f['source']}.txt").read_text().splitlines()
    assert len(rows) == H and all(len(r) == W for r in rows)
    layers = {name: {} for name in ORDER}
    groups = {}
    for y, row in enumerate(rows):
        for x, key in enumerate(row):
            if key != ".":
                layer, group = classify(f, (x, y), key)
                layers[layer][(x, y)] = key
                groups[(x, y)] = group
    z = {n: k for k, n in enumerate(ORDER)}
    owner = {p: n for n, t in layers.items() for p in t}
    report = {"opaque": len(owner), "original_per_layer": {n: len(layers[n]) for n in ORDER}, "painted": {}, "clipped": {}}
    for layer, shape in paint(f, groups, layers).items():
        kept, clipped = {}, 0
        for p, k in shape.items():
            if p in layers[layer]:
                continue
            above = owner.get(p)
            if above is None or z[above] <= z[layer]:
                clipped += 1
                continue
            kept[p] = k
        layers[layer].update(kept)
        report["painted"][layer] = len(kept)
        report["clipped"][layer] = clipped
    report["closed_cavities"] = close_cavities(layers, owner, z)
    return layers, report


def close_cavities(layers: dict, owner: dict, z: dict) -> int:
    """Fill the small cavities a removal opens inside the body (the crotch
    between the undergarment and the thighs, a seam between arm and torso):
    every enclosed transparent texel the dressed original does not have gets
    the nearest body tone, in the body layer if the original piece there is
    above it, else in the far-limb layer. Repeated until nothing changes."""
    base = enclosed(compose(layers, ORDER))
    filled = 0
    for _ in range(4):
        changed = 0
        looks = [BASE] + [without(piece) for piece in ("hair", "shirt", "trousers", "boots", "bracers")]
        for names in looks:
            for p in enclosed(compose(layers, names)) - base:
                above = owner.get(p)
                if above is None:
                    continue
                target = "body" if z[above] > z["body"] else "body_back"
                if z[above] <= z[target]:
                    continue
                pool = layers[target]
                near = [(abs(q[0] - p[0]) + abs(q[1] - p[1]), k) for q, k in pool.items()
                        if abs(q[0] - p[0]) <= 3 and abs(q[1] - p[1]) <= 3 and k != "K"]
                layers[target][p] = min(near)[1] if near else "s"
                changed += 1
        filled += changed
        if not changed:
            break
    return filled


def rgba(key: str):
    r, g, b = PALETTE[key][:3]
    return (r, g, b, 255)


def image(texels: dict) -> Image.Image:
    im = Image.new("RGBA", (W, H), (0, 0, 0, 0))
    for (x, y), k in texels.items():
        im.putpixel((x, y), rgba(k))
    return im


def compose(layers: dict, names) -> Image.Image:
    out = Image.new("RGBA", (W, H), (0, 0, 0, 0))
    for n in ORDER:
        if n in names:
            out.alpha_composite(image(layers[n]))
    return out


def without(piece: str) -> list[str]:
    return [n for n in ORDER if PIECE_OF[n] != piece]


BASE = [n for n in ORDER if PIECE_OF[n] == "body"]


def holes(layers: dict, names) -> int:
    """Enclosed holes: transparent texels with no transparent path to the
    canvas border (the air between open limbs reaches the border; a hole
    left inside the body by a removed piece does not)."""
    im = compose(layers, names)
    clear = {(x, y) for y in range(H) for x in range(W) if im.getpixel((x, y))[3] == 0}
    stack = [p for p in clear if p[0] in (0, W - 1) or p[1] in (0, H - 1)]
    seen = set(stack)
    while stack:
        x, y = stack.pop()
        for q in ((x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)):
            if q in clear and q not in seen:
                seen.add(q)
                stack.append(q)
    return len(clear - seen)


def enclosed(im: Image.Image) -> set:
    clear = {(x, y) for y in range(H) for x in range(W) if im.getpixel((x, y))[3] == 0}
    stack = [p for p in clear if p[0] in (0, W - 1) or p[1] in (0, H - 1)]
    seen = set(stack)
    while stack:
        x, y = stack.pop()
        for q in ((x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)):
            if q in clear and q not in seen:
                seen.add(q)
                stack.append(q)
    return clear - seen


def new_holes_image(layers: dict, names) -> Image.Image:
    """The composition with the enclosed holes that the original does not
    have painted red."""
    base = enclosed(compose(layers, ORDER))
    im = compose(layers, names)
    for p in enclosed(im) - base:
        im.putpixel(p, (255, 0, 64, 255))
    return im


def exposed(layers: dict, names, original: Image.Image) -> int:
    """Texels opaque in the dressed original that become transparent (the
    clothes are bigger than the body under them; informative, not a defect)."""
    im = compose(layers, names)
    return sum(1 for y in range(H) for x in range(W) if original.getpixel((x, y))[3] and im.getpixel((x, y))[3] == 0)


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    EVIDENCE.mkdir(parents=True, exist_ok=True)
    run6 = Image.open(ROOT / "art-src/n3-6-animation-proof/generated/run6.png").convert("RGBA")
    frames = {}
    report = {}
    for col, i in enumerate(FRAMES):
        layers, rep = build_frame(i)
        frames[i] = layers
        original = run6.crop((i * W, 0, (i + 1) * W, H))
        full = compose(layers, ORDER)
        diff = [(x, y) for y in range(H) for x in range(W)
                if not ((original.getpixel((x, y))[3] == 0 and full.getpixel((x, y))[3] == 0)
                        or original.getpixel((x, y)) == full.getpixel((x, y)))]
        rep["recomposition_differences"] = len(diff)
        pieces = ["hair", "shirt", "trousers", "boots", "bracers"]
        base_holes = holes(layers, ORDER)
        rep["enclosed_in_original"] = base_holes
        rep["new_enclosed_after_removal"] = {p: holes(layers, without(p)) - base_holes for p in pieces}
        rep["new_enclosed_in_base_body"] = holes(layers, BASE) - base_holes
        rep["exposed_after_removal"] = {p: exposed(layers, without(p), original) for p in pieces}
        rep["exposed_in_base_body"] = exposed(layers, BASE, original)
        sil_o = {(x, y) for y in range(H) for x in range(W) if original.getpixel((x, y))[3]}
        sil_m = {(x, y) for y in range(H) for x in range(W) if full.getpixel((x, y))[3]}
        rep["silhouette_differences"] = len(sil_o ^ sil_m)
        rep["lowest_row_original"] = max(y for _, y in sil_o)
        rep["lowest_row_modular"] = max(y for _, y in sil_m)
        report[f"run{i}"] = rep
    # Strips: one 336x128 PNG (3 frames) and one palette grid per layer.
    for name in ORDER:
        strip = Image.new("RGBA", (W * len(FRAMES), H), (0, 0, 0, 0))
        rows = [["."] * (W * len(FRAMES)) for _ in range(H)]
        for col, i in enumerate(FRAMES):
            strip.alpha_composite(image(frames[i][name]), (col * W, 0))
            for (x, y), k in frames[i][name].items():
                rows[y][col * W + x] = k
        strip.save(OUT / f"{name}.png")
        (OUT / f"{name}.txt").write_text("\n".join("".join(r) for r in rows) + "\n")
    evidence(frames, run6)
    (EVIDENCE / "metrics.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({k: {kk: v[kk] for kk in ("recomposition_differences", "enclosed_in_original", "new_enclosed_after_removal", "new_enclosed_in_base_body")} for k, v in report.items()}, indent=1))
    bad = [k for k, v in report.items() if v["recomposition_differences"]]
    if bad:
        sys.exit(f"recomposition differs in {bad}")


def on_bg(im: Image.Image, bg=(48, 44, 56)) -> Image.Image:
    base = Image.new("RGBA", im.size, bg + (255,))
    base.alpha_composite(im)
    return base


def sheet(rows_of_tiles, path: Path, scale=3, bg=(48, 44, 56)) -> None:
    crop = (12, 8, 100, 122)
    cw, ch = (crop[2] - crop[0]) * scale, (crop[3] - crop[1]) * scale
    cols = max(len(r[1]) for r in rows_of_tiles)
    out = Image.new("RGB", (cols * (cw + 4) + 60, len(rows_of_tiles) * (ch + 16)), (16, 14, 20))
    d = ImageDraw.Draw(out)
    for r, (row_label, tiles) in enumerate(rows_of_tiles):
        y = r * (ch + 16)
        d.text((2, y + ch // 2), row_label, fill=(235, 200, 110))
        for c, (label, im) in enumerate(tiles):
            x = 60 + c * (cw + 4)
            out.paste(on_bg(im, bg).crop(crop).resize((cw, ch), Image.NEAREST).convert("RGB"), (x, y + 14))
            d.text((x + 2, y + 1), label, fill=(235, 200, 110))
    out.save(path)


def evidence(frames: dict, run6: Image.Image) -> None:
    looks = [
        ("original", None), ("modular", ORDER), ("no hair", without("hair")), ("no shirt", without("shirt")),
        ("no trousers", without("trousers")), ("no boots", without("boots")), ("no bracers", without("bracers")),
        ("base body", BASE),
    ]
    rows = []
    for i in FRAMES:
        tiles = []
        for label, names in looks:
            im = run6.crop((i * W, 0, (i + 1) * W, H)) if names is None else compose(frames[i], names)
            tiles.append((label, im))
        rows.append((f"#{i}", tiles))
    sheet(rows, EVIDENCE / "n4c_removal.png")
    hole_rows = []
    for i in FRAMES:
        hole_rows.append((f"#{i}", [(label, new_holes_image(frames[i], names)) for label, names in looks[2:]]))
    sheet(hole_rows, EVIDENCE / "n4c_new_holes.png")
    sheet(rows, EVIDENCE / "n4c_removal_light.png", bg=(214, 210, 200))
    # Layers of each frame.
    sheet([(f"#{i}", [(n, image(frames[i][n])) for n in ORDER]) for i in FRAMES], EVIDENCE / "n4c_layers.png", scale=2)
    # Diff maps.
    tiles = []
    for i in FRAMES:
        o = run6.crop((i * W, 0, (i + 1) * W, H))
        m = compose(frames[i], ORDER)
        dm = Image.new("RGBA", (W, H), (0, 0, 0, 255))
        for y in range(H):
            for x in range(W):
                a, b = o.getpixel((x, y)), m.getpixel((x, y))
                if a != b and not (a[3] == 0 and b[3] == 0):
                    dm.putpixel((x, y), (255, 0, 64, 255))
                elif a[3]:
                    dm.putpixel((x, y), (40, 60, 40, 255))
        tiles += [(f"#{i} original", o), (f"#{i} modular", m), (f"#{i} diff", dm)]
    sheet([("", tiles)], EVIDENCE / "n4c_recomposition.png", scale=3)


if __name__ == "__main__":
    main()
