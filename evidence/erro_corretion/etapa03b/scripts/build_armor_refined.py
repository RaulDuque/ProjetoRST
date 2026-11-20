"""Etapa 03B (Erro-Corretion): refined gothic armour (purple + gold), pixel-art oriented.

Runs ONLY on a work copy of the Etapa 02B blend (never on the original rig nor on the
Etapa 03 files, which stay as approved):

    blender --background --factory-startup evidence/erro_corretion/etapa03b/work/armor_refined_work.blend \
        --python evidence/erro_corretion/etapa03b/scripts/build_armor_refined.py -- \
        --targets evidence/erro_corretion/etapa02/out/targets.json \
        --fit evidence/erro_corretion/etapa02/out/fit_results.json \
        --anchors evidence/erro_corretion/etapa02b/out/layers_report.json \
        --layers evidence/erro_corretion/etapa02b/out/layers \
        --out evidence/erro_corretion/etapa03b/out \
        --save evidence/erro_corretion/etapa03b/work/armor_refined_v002.blend

Same pipeline as Etapa 03 (`etapa03/scripts/build_armor.py`), with these changes:

1. C1 #4 mannequin (action FIT03B_run_c1_4): the torso was ~2.5 px behind the
   sprite chest. spine_01 and spine_02 are leaned (world Y rotation about each bone
   head) by the pair of angles that best covers the approved `shirt_torso` texels
   with the torso armour (projected vertex extent per row) while bringing the neck
   to the sprite neck; then the clavicles are re-solved onto the sprite shoulders
   and the arms re-aimed at the Etapa 02 targets (02B method). Legs, root, camera,
   scale and pivot are untouched.
2. Pauldrons: faceted (flat-shaded) cop instead of a smooth dome, flatter, with a
   medial ridge, a pointed lower edge and a gold crest fin; the 3 lames step
   outwards (bell) and alternate shade.
3. Gold: flat bands cut from the plate surface (2-3 cm wide = 1.2-1.8 texels)
   instead of thin tubes; keel band on a sharper central ridge; fewer, larger
   rivets; only the lowest lame/tasset of each stack keeps a gold edge.
4. Materials: painted/enamelled metal (low metallic, mid roughness, low specular);
   controlled key/fill/rim suns, darker world.
5. Gorget: taller and more flared at the front so it reads under the jaw; layer
   order unchanged (still under hair and face).

Writes per pose the whole armour, one PNG per overlay group and a gorget-only mask,
the model sheet, `armor_build.json` (parameters, lean search, intersections) and
the saved model.
"""

from __future__ import annotations

import json
import math
import sys
from pathlib import Path

import bmesh
import bpy
from mathutils import Matrix, Vector
from mathutils.bvhtree import BVHTree

ARGS = sys.argv[sys.argv.index("--") + 1:]
OPTS = dict(zip(ARGS[::2], ARGS[1::2], strict=True))
TARGETS = json.loads(Path(OPTS["--targets"]).read_text())
FIT = json.loads(Path(OPTS["--fit"]).read_text())
ANCHORS = json.loads(Path(OPTS["--anchors"]).read_text())
LAYERS = Path(OPTS["--layers"])
OUT = Path(OPTS["--out"])
SAVE = Path(OPTS["--save"])
SUPER = 8
SOLE_EDGE = 118.5
LEAN_POSES = {"run_c1_4"}
LEAN_RANGE = [i * 0.5 for i in range(-24, 25)]  # degrees, +-12
CLAVICLE_SEARCH = [i * 0.25 for i in range(-180, 181)]

if "etapa03b" not in bpy.data.filepath:
    raise SystemExit("refusing to run outside the etapa03b work copy")

SCENE = bpy.context.scene
ARM = bpy.data.objects["rig_v007"]
PB = ARM.pose.bones
ORIGINAL_MESHES = [o for o in bpy.data.objects if o.type == "MESH"]

# Mannequin torso cross-sections (z, back x, front x, half width y), as in Etapa 03.
TORSO = [
    (0.96, -0.097, 0.092, 0.141), (1.02, -0.091, 0.090, 0.133), (1.08, -0.081, 0.093, 0.126),
    (1.14, -0.079, 0.097, 0.124), (1.20, -0.076, 0.106, 0.136), (1.26, -0.079, 0.115, 0.148),
    (1.32, -0.082, 0.117, 0.157), (1.38, -0.084, 0.108, 0.164), (1.44, -0.085, 0.085, 0.167),
]
# One texel = 1 / 58.53 m = 17 mm. Anything meant to survive the reduction is >= ~20 mm.
P = {
    "segments": 72,
    "plate_standoff": 0.056, "plate_side": 0.032, "plate_thickness": 0.014,
    "keel": 0.034, "keel_power": 24,           # sharper central ridge (two planes)
    "plate_top": 1.465, "neck_dip": 0.012, "armhole": 0.070,
    "plate_bottom": 1.130, "plackart_point": 0.055,
    "lames": 3, "lame_step": 0.034, "lame_height": 0.046, "lame_grow": 0.005,
    "tasset_top": 1.030, "tasset_flare": 0.022, "tasset_height": 0.072, "tasset_step": 0.058,
    "gorget_bottom": 1.425, "gorget_back": 1.550, "gorget_front": 1.545, "gorget_flare": 0.050,
    "pauldron_radius": 0.110, "pauldron_flatten": 0.34, "pauldron_segments": 12, "pauldron_rows": 4,
    "pauldron_ridge": 0.016, "pauldron_point_deg": 28, "pauldron_lames": 3, "pauldron_thickness": 0.012,
    "lame_out_step": 0.010, "crest_height": 0.026, "rim_inner_cut": 0.30,
    "band_main": 0.030, "band_minor": 0.020, "band_thickness": 0.005, "rivet": 0.012,
    "sharp_deg": 24,
}
GROUPS = ("far_low", "pauldron_far", "torso", "pauldron_near")
TORSO_PARTS = {"breastplate", "gorget", "abdomen"}


def torso_section(z: float) -> tuple[float, float, float]:
    z = max(TORSO[0][0], min(TORSO[-1][0], z))
    for (z0, b0, f0, w0), (z1, b1, f1, w1) in zip(TORSO, TORSO[1:]):
        if z0 <= z <= z1:
            t = (z - z0) / (z1 - z0)
            return b0 + (b1 - b0) * t, f0 + (f1 - f0) * t, w0 + (w1 - w0) * t
    return TORSO[-1][1:]


def superellipse(angle: float, n: float = 2.6) -> tuple[float, float]:
    c, s = math.cos(angle), math.sin(angle)
    return (math.copysign(abs(c) ** (2 / n), c), math.copysign(abs(s) ** (2 / n), s))


def torso_point(z: float, angle: float, front_off: float, side_off: float, keel: float = 0.0) -> Vector:
    back, front, half = torso_section(z)
    centre, depth = (front + back) / 2, (front - back) / 2
    sx, sy = superellipse(angle)
    facing = max(0.0, math.cos(angle))
    x = centre + (depth + front_off) * sx + keel * facing ** P["keel_power"]
    return Vector((x, (half + side_off) * sy, z))


# --- materials: enamelled / painted metal ------------------------------------------------------


def material(name: str, colour, metallic: float, roughness: float, specular: float):
    mat = bpy.data.materials.get(name) or bpy.data.materials.new(name)
    mat.use_nodes = True
    bsdf = next(n for n in mat.node_tree.nodes if n.type == "BSDF_PRINCIPLED")
    bsdf.inputs["Base Color"].default_value = (*colour, 1.0)
    bsdf.inputs["Metallic"].default_value = metallic
    bsdf.inputs["Roughness"].default_value = roughness
    for key in ("Specular IOR Level", "Specular"):
        if key in bsdf.inputs:
            bsdf.inputs[key].default_value = specular
            break
    return mat


MAT = {}


def make_materials() -> None:
    MAT["purple"] = material("armor03b_purple", (0.120, 0.036, 0.190), 0.30, 0.52, 0.30)
    MAT["purple_dark"] = material("armor03b_purple_dark", (0.058, 0.017, 0.098), 0.30, 0.56, 0.25)
    MAT["gold"] = material("armor03b_gold", (0.80, 0.55, 0.18), 1.0, 0.40, 0.50)


# --- geometry helpers ----------------------------------------------------------------------------

COLLECTION = None


def link(obj, group: str, bone: str, mat, part: str) -> None:
    COLLECTION.objects.link(obj)
    obj["armor_group"] = group
    obj["armor_part"] = part
    if mat is not None:
        obj.data.materials.append(mat)
    bone_data = ARM.data.bones[bone]
    rest = ARM.matrix_world @ bone_data.matrix_local @ Matrix.Translation((0.0, bone_data.length, 0.0))
    obj.parent = ARM
    obj.parent_type = "BONE"
    obj.parent_bone = bone
    obj.matrix_parent_inverse = rest.inverted()


def mark_sharp(mesh, degrees: float) -> None:
    bm = bmesh.new()
    bm.from_mesh(mesh)
    limit = math.radians(degrees)
    for edge in bm.edges:
        if len(edge.link_faces) == 2 and edge.calc_face_angle(0.0) > limit:
            edge.smooth = False
    bm.to_mesh(mesh)
    bm.free()


def loft(name: str, rings: list[list[Vector]], group: str, bone: str, mat, thickness: float, part: str,
         cyclic: bool = True, smooth: bool = True, sharp: float | None = None, merge: bool = False):
    verts = [v for ring in rings for v in ring]
    count = len(rings[0])
    faces = []
    for i in range(len(rings) - 1):
        for j in range(count if cyclic else count - 1):
            a, b = i * count + j, i * count + (j + 1) % count
            faces.append((a, b, (i + 1) * count + (j + 1) % count, (i + 1) * count + j))
    mesh = bpy.data.meshes.new(name)
    mesh.from_pydata(verts, [], faces)
    if merge:  # closes a lofted apex (first ring collapsed to one point)
        bm = bmesh.new()
        bm.from_mesh(mesh)
        bmesh.ops.remove_doubles(bm, verts=bm.verts, dist=1e-6)
        bm.to_mesh(mesh)
        bm.free()
    mesh.update()
    for poly in mesh.polygons:
        poly.use_smooth = smooth
    if smooth and sharp is not None:
        mark_sharp(mesh, sharp)
    obj = bpy.data.objects.new(name, mesh)
    solid = obj.modifiers.new("thickness", "SOLIDIFY")
    solid.thickness = thickness
    solid.offset = 0.0
    solid.use_even_offset = True
    link(obj, group, bone, mat, part)
    return obj


def radial(centre_x: float):
    def normal(p: Vector) -> Vector:
        d = Vector((p.x - centre_x, p.y, 0.0))
        return d.normalized() if d.length > 1e-6 else Vector((1.0, 0.0, 0.0))
    return normal


def from_point(centre: Vector):
    def normal(p: Vector) -> Vector:
        return (p - centre).normalized()
    return normal


def lerp_ring(a: list[Vector], b: list[Vector], t: float) -> list[Vector]:
    return [x.lerp(y, t) for x, y in zip(a, b, strict=True)]


def band_inner(rings: list[list[Vector]], edge: int, step: int, width: float) -> list[Vector]:
    """Ring at `width` (mean distance along the surface) from rings[edge], walking by `step`."""
    walked, k = 0.0, edge
    while 0 <= k + step < len(rings):
        seg = sum((p - q).length for p, q in zip(rings[k], rings[k + step], strict=True)) / len(rings[k])
        if walked + seg >= width:
            return lerp_ring(rings[k], rings[k + step], (width - walked) / max(seg, 1e-9))
        walked += seg
        k += step
    return rings[k]


def gold_band(name: str, rings, edge: int, step: int, width: float, normal_of, lift: float, group: str,
              bone: str, part: str, cyclic: bool = True) -> None:
    """Flat gold band cut from the plate surface between its edge and `width` inwards."""
    outer = rings[edge]
    inner = band_inner(rings, edge, step, width)
    outer = [o + (o - i) * 0.12 for o, i in zip(outer, inner, strict=True)]  # caps the plate edge
    lifted = [[p + normal_of(p) * lift for p in ring] for ring in (inner, outer)]
    loft(name, lifted, group, bone, MAT["gold"], P["band_thickness"], part, cyclic=cyclic)


def ribbon(name: str, path: list[Vector], normal_of, width: float, lift: float, group: str, bone: str,
           part: str) -> None:
    lefts, rights = [], []
    for i, p in enumerate(path):
        a, b = path[max(i - 1, 0)], path[min(i + 1, len(path) - 1)]
        n = normal_of(p)
        side = (b - a).cross(n).normalized()
        c = p + n * lift
        lefts.append(c + side * width / 2)
        rights.append(c - side * width / 2)
    loft(name, [lefts, rights], group, bone, MAT["gold"], P["band_thickness"], part, cyclic=False)


def rivet(name: str, at: Vector, group: str, bone: str, part: str) -> None:
    mesh = bpy.data.meshes.new(name)
    obj = bpy.data.objects.new(name, mesh)
    bm = bmesh.new()
    bmesh.ops.create_uvsphere(bm, u_segments=8, v_segments=5, radius=P["rivet"])
    bm.to_mesh(mesh)
    bm.free()
    for poly in mesh.polygons:
        poly.use_smooth = True
    obj.location = at
    link(obj, group, bone, MAT["gold"], part)


def band_lift(plate_thickness: float) -> float:
    return plate_thickness / 2 + P["band_thickness"] / 2 + 0.0015


# --- armour pieces ---------------------------------------------------------------------------------


def breastplate() -> None:
    n = P["segments"]
    angles = [2 * math.pi * j / n for j in range(n)]

    def top(a):
        f = max(0.0, math.cos(a))
        return P["plate_top"] - P["neck_dip"] * f ** 3 - P["armhole"] * abs(math.sin(a)) ** 6

    def bottom(a):
        return P["plate_bottom"] - P["plackart_point"] * max(0.0, math.cos(a)) ** 6

    rows = 36
    rings = [[torso_point(bottom(a) + (top(a) - bottom(a)) * k / rows, a, P["plate_standoff"], P["plate_side"],
                          P["keel"]) for a in angles] for k in range(rows + 1)]
    loft("armor_breastplate", rings, "torso", "chest", MAT["purple"], P["plate_thickness"], "breastplate",
         sharp=P["sharp_deg"])
    back, front, _ = torso_section(1.3)
    normal = radial((front + back) / 2)
    lift = band_lift(P["plate_thickness"])
    gold_band("armor_breastplate_band_top", rings, rows, -1, P["band_main"], normal, lift, "torso", "chest",
              "breastplate")
    gold_band("armor_breastplate_band_bottom", rings, 0, 1, P["band_main"], normal, lift, "torso", "chest",
              "breastplate")
    # Keel: a band two columns wide on the ridge, from just above the plackart point to the neckline.
    keel_rows = [[ring[-1], ring[0], ring[1]] for ring in rings[3:-3]]
    loft("armor_breastplate_keel", [[p + normal(p) * lift for p in row] for row in keel_rows], "torso", "chest",
         MAT["gold"], P["band_thickness"] * 1.4, "breastplate", cyclic=False)
    # Two gothic arch ribs, spread wide so they do not read as parallel wires next to the keel.
    for side in (-1, 1):
        rib = []
        for k in range(4, rows - 7):
            a = side * (0.10 + 0.78 * (k / rows) ** 1.6)
            z = bottom(a) + (top(a) - bottom(a)) * (k / rows)
            rib.append(torso_point(z, a, P["plate_standoff"], P["plate_side"], P["keel"]))
        ribbon(f"armor_breastplate_rib_{'near' if side < 0 else 'far'}", rib, normal, P["band_minor"], lift,
               "torso", "chest", "breastplate")


def abdomen_lames() -> None:
    n = P["segments"]
    angles = [2 * math.pi * j / n for j in range(n)]
    for i in range(P["lames"]):
        z_top = P["plate_bottom"] - 0.012 - i * P["lame_step"]
        z_bot = z_top - P["lame_height"]
        off = P["plate_standoff"] + 0.006 + i * P["lame_grow"]
        side = P["plate_side"] + 0.006 + i * P["lame_grow"]
        rings = []
        for k in range(7):
            z = z_bot + (z_top - z_bot) * k / 6
            flare = 0.010 * (1 - k / 6)
            rings.append([torso_point(z, a, off + flare, side + flare, P["keel"] * 0.5) for a in angles])
        bone = "spine_01" if i < P["lames"] - 1 else "pelvis"
        loft(f"armor_lame_{i}", rings, "torso", bone, MAT["purple_dark"] if i % 2 else MAT["purple"],
             P["plate_thickness"] * 0.9, "abdomen", sharp=P["sharp_deg"])
        back, front, _ = torso_section(z_bot)
        normal = radial((front + back) / 2)
        lift = band_lift(P["plate_thickness"] * 0.9)
        if i == P["lames"] - 1:
            gold_band(f"armor_lame_{i}_band", rings, 0, 1, P["band_minor"], normal, lift, "torso", bone, "abdomen")
        if i == 1:
            for j in (0, 7, n - 7):  # three large rivets at the front only
                p = rings[3][j]
                rivet(f"armor_lame_{i}_rivet_{j}", p + normal(p) * lift, "torso", bone, "abdomen")


def tassets() -> None:
    spans = {"front": (8, 68), "side": (72, 128)}
    for side_name, sign, bone, group in (("near", -1, "thigh.R", "torso"), ("far", 1, "thigh.L", "far_low")):
        for piece, (a0, a1) in spans.items():
            for layer in range(2):
                z_top = P["tasset_top"] - layer * P["tasset_step"]
                z_bot = z_top - P["tasset_height"]
                angles = [sign * math.radians(a0 + (a1 - a0) * j / 16) for j in range(17)]
                rings = []
                for k in range(9):
                    t = k / 8
                    z = z_bot + (z_top - z_bot) * t
                    flare = P["tasset_flare"] * (1 - t) + layer * 0.010
                    rings.append([torso_point(max(z, 0.96), a, P["plate_standoff"] + 0.012 + flare,
                                              P["plate_side"] + 0.012 + flare) + Vector((0, 0, z - max(z, 0.96)))
                                  for a in angles])
                name = f"armor_tasset_{side_name}_{piece}_{layer}"
                loft(name, rings, group, bone, MAT["purple"] if layer == 0 else MAT["purple_dark"],
                     P["plate_thickness"] * 0.9, "tasset", cyclic=False, sharp=P["sharp_deg"])
                if layer == 1:
                    back, front, _ = torso_section(0.96)
                    gold_band(name + "_band", rings, 0, 1, P["band_minor"], radial((front + back) / 2),
                              band_lift(P["plate_thickness"] * 0.9), group, bone, "tasset", cyclic=False)


def gorget() -> None:
    n = P["segments"]
    cx = -0.007
    angles = [2 * math.pi * j / n for j in range(n)]
    rings = []
    for k in range(11):
        t = k / 10
        ring = []
        for a in angles:
            front = max(0.0, math.cos(a))
            top = P["gorget_back"] - (P["gorget_back"] - P["gorget_front"]) * front ** 1.5
            z = P["gorget_bottom"] + (top - P["gorget_bottom"]) * t
            r_x = 0.084 + P["gorget_flare"] * t ** 1.6 + (0.035 + 0.015 * front) * (1 - t) + 0.012 * front * t
            r_y = 0.086 + P["gorget_flare"] * t ** 1.6 + 0.065 * (1 - t)
            ring.append(Vector((cx + r_x * math.cos(a), r_y * math.sin(a), z)))
        rings.append(ring)
    loft("armor_gorget", rings, "torso", "chest", MAT["purple_dark"], P["plate_thickness"], "gorget",
         sharp=P["sharp_deg"])
    gold_band("armor_gorget_band", rings, len(rings) - 1, -1, P["band_main"], radial(cx),
              band_lift(P["plate_thickness"]), "torso", "chest", "gorget")


def frame(axis: Vector) -> tuple[Vector, Vector]:
    helper = Vector((1, 0, 0)) if abs(axis.x) < 0.9 else Vector((0, 1, 0))
    e1 = axis.cross(helper).normalized()
    return e1, axis.cross(e1).normalized()


def pauldron(side_name: str, sign: int, bone: str, group: str) -> None:
    part = f"pauldron_{side_name}"
    bone_data = ARM.data.bones[bone]
    shoulder = Vector(bone_data.head_local)
    down = (Vector(bone_data.tail_local) - shoulder).normalized()
    out = Vector((0.0, sign, 0.0))
    out = (out - down * out.dot(down)).normalized()
    fwd = down.cross(out).normalized()
    if fwd.x < 0:
        fwd = -fwd
    radius, flatten = P["pauldron_radius"], P["pauldron_flatten"]
    centre = shoulder + out * 0.014 + down * 0.012
    axis = (-down * 0.80 + out * 0.60).normalized()
    ridge_plane = axis.cross(fwd).normalized()  # normal of the plane holding the medial ridge
    e1, e2 = frame(axis)

    def surface(direction: Vector, extra: float = 0.0) -> Vector:
        # Shield outline seen from the side: longer downwards (pointed), broad front-back.
        stretch = 1 + 0.28 * max(0.0, direction.dot(down)) + 0.10 * abs(direction.dot(fwd))
        ridge = P["pauldron_ridge"] * max(0.0, 1 - abs(direction.dot(ridge_plane)) / 0.30)
        v = direction * (radius * stretch + ridge + extra)
        return centre + v - axis * v.dot(axis) * flatten

    segments, rows = P["pauldron_segments"], P["pauldron_rows"]
    rings = []
    for k in range(rows + 1):
        ring = []
        for j in range(segments):
            psi = 2 * math.pi * j / segments
            rim = e1 * math.cos(psi) + e2 * math.sin(psi)
            limit = math.radians(100 - 40 * max(0.0, rim.dot(-out)) + 10 * max(0.0, rim.dot(down))
                                 + P["pauldron_point_deg"] * max(0.0, rim.dot(down)) ** 4)
            phi = limit * k / rows
            ring.append(surface(axis * math.cos(phi) + rim * math.sin(phi)))
        rings.append(ring)
    loft(f"armor_pauldron_{side_name}_cop", rings, group, bone, MAT["purple"], P["pauldron_thickness"], part,
         smooth=False, merge=True)
    normal = from_point(centre)
    lift = band_lift(P["pauldron_thickness"])
    # Gold edge only on the outer/lower rim: the inner rim (towards the neck) stays plain, otherwise
    # it reads as a hoop floating over the breastplate in the side view.
    inner = [(e1 * math.cos(2 * math.pi * j / segments) + e2 * math.sin(2 * math.pi * j / segments)).dot(-out)
             for j in range(segments)]
    start = max(range(segments), key=lambda j: inner[j])
    keep = [(start + d) % segments for d in range(segments) if inner[(start + d) % segments] < P["rim_inner_cut"]]
    gold_band(f"armor_pauldron_{side_name}_rim", [[ring[j] for j in keep] for ring in rings], len(rings) - 1, -1,
              P["band_main"], normal, lift, group, bone, part, cyclic=False)
    # Short crest fin on the top of the medial ridge (silhouette accent), tapering at both ends.
    base, tip = [], []
    for k in range(-6, 7):
        phi = math.radians(40 * k / 6)
        direction = axis * math.cos(phi) + fwd * math.sin(phi)
        p = surface(direction, P["pauldron_thickness"] * 0.5)
        h = P["crest_height"] * min(1.0, (6 - abs(k)) / 3 + 0.2)
        base.append(p)
        tip.append(p + normal(p) * h)
    loft(f"armor_pauldron_{side_name}_crest", [base, tip], group, bone, MAT["gold"], 0.007, part, cyclic=False)
    for i in range(P["pauldron_lames"]):
        s0 = 0.095 + 0.044 * i
        ring_r = radius * 0.86 + P["lame_out_step"] * i
        angles = [math.radians(-118 + 236 * j / 11) for j in range(12)]
        lame_rings = []
        for k in range(4):
            t = k / 3
            along = s0 + 0.058 * t
            r = ring_r + 0.020 * t
            lame_rings.append([centre + down * along + (out * math.cos(a) + fwd * math.sin(a)) * r for a in angles])
        loft(f"armor_pauldron_{side_name}_lame_{i}", lame_rings, group, bone,
             MAT["purple_dark"] if i % 2 == 0 else MAT["purple"], P["pauldron_thickness"] * 0.9, part,
             cyclic=False, smooth=False)
        axis_point = centre + down * (s0 + 0.03)

        def lame_normal(p, c=axis_point):
            d = p - c
            d -= down * d.dot(down)
            return d.normalized()

        lame_lift = band_lift(P["pauldron_thickness"] * 0.9)
        if i == P["pauldron_lames"] - 1:
            gold_band(f"armor_pauldron_{side_name}_lame_{i}_band", lame_rings, len(lame_rings) - 1, -1,
                      P["band_minor"], lame_normal, lame_lift, group, bone, part, cyclic=False)
        p = lame_rings[1][6]
        rivet(f"armor_pauldron_{side_name}_lame_{i}_rivet", p + lame_normal(p) * lame_lift, group, bone, part)


# --- poses -------------------------------------------------------------------------------------------


def update() -> None:
    bpy.context.view_layer.update()


def head(name: str) -> Vector:
    return ARM.matrix_world @ PB[name].head


def tail(name: str) -> Vector:
    return ARM.matrix_world @ PB[name].tail


def mid(name: str) -> Vector:
    return (head(name) + tail(name)) / 2


def rotate_about(name: str, pivot: Vector, rotation: Matrix) -> None:
    PB[name].matrix = Matrix.Translation(pivot) @ rotation.to_4x4() @ Matrix.Translation(-pivot) @ PB[name].matrix
    update()


def aim(name: str, tip, target: Vector) -> None:
    update()
    pivot = head(name)
    point = tip()
    now = math.atan2(point.z - pivot.z, point.x - pivot.x)
    want = math.atan2(target.z - pivot.z, target.x - pivot.x)
    rotate_about(name, pivot, Matrix.Rotation(-(want - now), 3, "Y"))


def load_action(name: str) -> None:
    action = bpy.data.actions[name]
    ARM.animation_data.action = action
    if len(action.slots):
        ARM.animation_data.action_slot = action.slots[0]
    SCENE.frame_set(1)
    update()
    basis = {b.name: b.matrix_basis.copy() for b in PB}
    ARM.animation_data.action = None
    for b in PB:
        b.matrix_basis = basis[b.name]
    update()


def snapshot() -> dict:
    return {b.name: b.matrix_basis.copy() for b in PB}


def restore(basis: dict) -> None:
    for b in PB:
        b.matrix_basis = basis[b.name]
    update()


def rest_pose() -> None:
    if ARM.animation_data:
        ARM.animation_data.action = None
    restore({b.name: Matrix.Identity(4) for b in PB})


def keyframe(action_name: str) -> None:
    action = bpy.data.actions.get(action_name) or bpy.data.actions.new(action_name)
    action.use_fake_user = True
    ARM.animation_data.action = action
    for bone in PB:
        bone.keyframe_insert("location", frame=1)
        mode = "rotation_quaternion" if bone.rotation_mode == "QUATERNION" else "rotation_euler"
        bone.keyframe_insert(mode, frame=1)
    ARM.animation_data.action = None


def mapping(scale: float, pivot_x: float):
    def world(p) -> Vector:
        return Vector(((p[0] - pivot_x) / scale, 0.0, (SOLE_EDGE - p[1]) / scale))

    def texel(v: Vector) -> list[float]:
        return [round(v.x * scale + pivot_x, 3), round(SOLE_EDGE - v.z * scale, 3)]
    return world, texel


def solve_clavicle(side: str, target: Vector, search=CLAVICLE_SEARCH) -> tuple[float, float, float]:
    pivot = head(f"clavicle.{side}")
    point = head(f"upper_arm.{side}") - pivot
    best = (1e9, 0.0, 0.0)
    for ax in search:
        rx = Matrix.Rotation(math.radians(ax), 3, "X")
        for az in search[::2]:
            p = Matrix.Rotation(math.radians(az), 3, "Z") @ (rx @ point) + pivot
            score = math.hypot(p.x - target.x, p.z - target.z) + 1e-5 * (abs(ax) + abs(az))
            if score < best[0]:
                best = (score, ax, az)
    _, ax, az = best
    rotate_about(f"clavicle.{side}", pivot,
                 Matrix.Rotation(math.radians(az), 3, "Z") @ Matrix.Rotation(math.radians(ax), 3, "X"))
    end = head(f"upper_arm.{side}")
    return ax, az, math.hypot(end.x - target.x, end.z - target.z)


def reaim_arm(side: str, arm, world) -> None:
    aim(f"upper_arm.{side}", lambda: head(f"forearm.{side}"), world(arm[1]))
    aim(f"forearm.{side}", lambda: head(f"hand.{side}"), world(arm[2]))
    aim(f"hand.{side}", lambda: tail(f"hand.{side}"), world(arm[3]))


def idle_shoulders(scale: float, pivot_x: float) -> dict:
    """Idle: same correction as Etapa 03 (shoulder on the visible sleeve cap of 02B)."""
    world, texel = mapping(scale, pivot_x)
    anchor = ANCHORS["poses"]["idle"]["anchors"]["near"]
    goal = world((anchor[0], anchor[1] - 1.0))
    report = {}
    search = [i * 0.5 for i in range(-90, 91)]
    for side in ("R", "L"):
        ax, az, _ = solve_clavicle(side, goal, search)
        reaim_arm(side, TARGETS["idle"]["near_arm"], world)
        report[side] = {"rot_x_deg": ax, "rot_z_deg": az, "shoulder_px": texel(head(f"upper_arm.{side}"))}
    return report


def shirt_cells(pose_name: str) -> set[tuple[int, int]]:
    image = bpy.data.images.load(str(LAYERS / pose_name / "shirt_torso.png"), check_existing=True)
    width, height = image.size
    alpha = list(image.pixels)[3::4]
    cells = {(x, height - 1 - r) for r in range(height) for x in range(width) if alpha[r * width + x] > 0}
    bpy.data.images.remove(image)
    return cells


def torso_rows(texel) -> dict[int, tuple[float, float]]:
    """Projected x extent per texel row of the torso armour (breastplate, gorget, lames)."""
    depsgraph = bpy.context.evaluated_depsgraph_get()
    rows: dict[int, list[float]] = {}
    for obj in COLLECTION.objects:
        if obj.get("armor_part") not in TORSO_PARTS or obj.type != "MESH":
            continue
        evaluated = obj.evaluated_get(depsgraph)
        mesh = evaluated.to_mesh()
        matrix = evaluated.matrix_world
        for v in mesh.vertices:
            x, y = texel(matrix @ v.co)
            row = math.floor(y)
            lo_hi = rows.setdefault(row, [1e9, -1e9])
            lo_hi[0], lo_hi[1] = min(lo_hi[0], x), max(lo_hi[1], x)
        evaluated.to_mesh_clear()
    return {k: (v[0], v[1]) for k, v in rows.items()}


def exposed_proxy(cells, texel) -> int:
    rows = torso_rows(texel)
    return sum(1 for x, y in cells if not (y in rows and rows[y][0] <= x + 0.5 <= rows[y][1]))


def lean_fix(pose_name: str, pose: dict, scale: float) -> dict:
    """C1 #4: lean spine_01/spine_02 so the torso armour covers the shirt, then re-fit the arms."""
    world, texel = mapping(scale, pose["pivot"][0])
    cells = shirt_cells(pose_name)
    neck_goal = world(pose["neck"])
    start = snapshot()
    before = {"exposed_proxy": exposed_proxy(cells, texel), "neck_px": texel(mid("neck"))}
    best = None
    for a1 in LEAN_RANGE[::2]:
        for a2 in LEAN_RANGE[::2]:
            restore(start)
            rotate_about("spine_01", head("spine_01"), Matrix.Rotation(math.radians(a1), 3, "Y"))
            rotate_about("spine_02", head("spine_02"), Matrix.Rotation(math.radians(a2), 3, "Y"))
            neck = mid("neck")
            neck_err = math.hypot(neck.x - neck_goal.x, neck.z - neck_goal.z) * scale
            exposed = exposed_proxy(cells, texel)
            score = exposed + 2.0 * neck_err + 0.15 * (abs(a1) + abs(a2))
            if best is None or score < best[0]:
                best = (score, a1, a2, exposed, neck_err)
    _, a1, a2, exposed, neck_err = best
    restore(start)
    rotate_about("spine_01", head("spine_01"), Matrix.Rotation(math.radians(a1), 3, "Y"))
    rotate_about("spine_02", head("spine_02"), Matrix.Rotation(math.radians(a2), 3, "Y"))
    clavicles = {}
    for side, key in (("R", "near"), ("L", "far")):
        arm = pose[f"{key}_arm"]
        ax, az, residual = solve_clavicle(side, world(arm[0]))
        reaim_arm(side, arm, world)
        clavicles[side] = {"rot_x_deg": ax, "rot_z_deg": az, "residual_px": round(residual * scale, 3)}
    landmarks = {"neck": texel(mid("neck")), "near_shoulder": texel(head("upper_arm.R")),
                 "far_shoulder": texel(head("upper_arm.L")), "near_elbow": texel(head("forearm.R")),
                 "near_wrist": texel(head("hand.R")), "far_elbow": texel(head("forearm.L")),
                 "far_wrist": texel(head("hand.L")), "waist": texel(head("spine_01"))}
    return {"spine_01_deg": a1, "spine_02_deg": a2, "before": before,
            "after": {"exposed_proxy": exposed_proxy(cells, texel), "neck_err_px": round(neck_err, 3),
                      "search_exposed_proxy": exposed},
            "sprite_neck": pose["neck"], "clavicles": clavicles, "landmarks_after": landmarks}


# --- intersections -----------------------------------------------------------------------------------


def world_bvh(objects) -> BVHTree | None:
    depsgraph = bpy.context.evaluated_depsgraph_get()
    verts, polys = [], []
    for obj in objects:
        evaluated = obj.evaluated_get(depsgraph)
        mesh = evaluated.to_mesh()
        offset = len(verts)
        verts += [evaluated.matrix_world @ v.co for v in mesh.vertices]
        polys += [[offset + i for i in p.vertices] for p in mesh.polygons]
        evaluated.to_mesh_clear()
    return BVHTree.FromPolygons(verts, polys) if polys else None


def intersections() -> dict[str, int]:
    """Triangle pairs that overlap between pieces that should not touch."""
    def part(*names):
        return [o for o in COLLECTION.objects if o.type == "MESH" and o.get("armor_part") in names]

    pairs = {
        "pauldron_near_vs_torso": (part("pauldron_near"), part(*TORSO_PARTS)),
        "pauldron_far_vs_torso": (part("pauldron_far"), part(*TORSO_PARTS)),
        "breastplate_vs_abdomen": (part("breastplate"), part("abdomen")),
        "tassets_vs_abdomen": (part("tasset"), part("abdomen")),
    }
    out = {}
    for name, (a, b) in pairs.items():
        ta, tb = world_bvh(a), world_bvh(b)
        out[name] = len(ta.overlap(tb)) if ta and tb else 0
    return out


# --- camera, light, render -----------------------------------------------------------------------------


def sun(name: str, source: Vector, energy: float) -> None:
    obj = bpy.data.objects.get(name)
    if obj is None:
        obj = bpy.data.objects.new(name, bpy.data.lights.new(name, "SUN"))
        SCENE.collection.objects.link(obj)
    obj.data.energy = energy
    obj.data.angle = math.radians(8)
    obj.rotation_euler = (-source).to_track_quat("-Z", "Y").to_euler()
    obj.hide_render = False


def setup_render_engine() -> str:
    for engine in ("BLENDER_EEVEE", "BLENDER_EEVEE_NEXT", "CYCLES"):
        try:
            SCENE.render.engine = engine
            break
        except TypeError:
            continue
    if SCENE.render.engine == "CYCLES":
        SCENE.cycles.samples = 48
    else:
        try:
            SCENE.eevee.taa_render_samples = 32
        except AttributeError:
            pass
    SCENE.render.film_transparent = True
    SCENE.view_settings.view_transform = "Standard"
    SCENE.view_settings.look = "None"
    SCENE.render.image_settings.file_format = "PNG"
    SCENE.render.image_settings.color_mode = "RGBA"
    world = SCENE.world or bpy.data.worlds.new("World")
    SCENE.world = world
    world.use_nodes = True
    background = next(n for n in world.node_tree.nodes if n.type == "BACKGROUND")
    background.inputs["Color"].default_value = (0.20, 0.19, 0.24, 1.0)
    background.inputs["Strength"].default_value = 0.45
    # Original v3 suns off; controlled key (upper front, camera side), fill (back, low), rim.
    for obj in bpy.data.objects:
        if obj.type == "LIGHT" and obj.name.startswith("v3_"):
            obj.hide_render = True
    sun("armor03b_key", Vector((0.55, -0.60, 0.75)), 3.4)
    sun("armor03b_fill", Vector((-0.55, -0.70, 0.05)), 0.7)
    sun("armor03b_rim", Vector((-0.70, 0.35, 0.60)), 1.6)
    return SCENE.render.engine


def side_camera(pose: dict, scale: float) -> None:
    camera = bpy.data.objects.get("armor_cam")
    if camera is None:
        camera = bpy.data.objects.new("armor_cam", bpy.data.cameras.new("armor_cam"))
        SCENE.collection.objects.link(camera)
    width, height = pose["canvas"]
    camera.data.type = "ORTHO"
    camera.data.sensor_fit = "VERTICAL"
    camera.data.ortho_scale = height / scale
    camera.data.clip_start, camera.data.clip_end = 0.1, 40.0
    camera.location = ((width / 2 - 0.5 - pose["pivot"][0]) / scale, -10.0, (SOLE_EDGE - (height / 2 - 0.5)) / scale)
    camera.rotation_euler = (math.radians(90), 0.0, 0.0)
    SCENE.camera = camera
    SCENE.render.resolution_x = width * SUPER
    SCENE.render.resolution_y = height * SUPER
    SCENE.render.resolution_percentage = 100


def render(path: Path, groups, parts=None) -> None:
    for obj in COLLECTION.objects:
        obj.hide_render = obj.get("armor_group") not in groups or (parts is not None and obj.get("armor_part") not in parts)
    SCENE.render.filepath = str(path)
    bpy.ops.render.render(write_still=True)


def model_sheet() -> None:
    camera = bpy.data.objects.get("sheet_cam")
    if camera is None:
        camera = bpy.data.objects.new("sheet_cam", bpy.data.cameras.new("sheet_cam"))
        SCENE.collection.objects.link(camera)
    SCENE.camera = camera
    SCENE.render.resolution_x = 900
    SCENE.render.resolution_y = 1000
    target = Vector((0.02, 0.0, 1.22))
    views = {
        "front": (Vector((6.0, 0.0, 0.0)), "ORTHO"),
        "side_near": (Vector((0.0, -6.0, 0.0)), "ORTHO"),
        "back": (Vector((-6.0, 0.0, 0.0)), "ORTHO"),
        "three_quarter": (Vector((2.2, -2.0, 0.55)), "PERSP"),
    }
    for name, (offset, kind) in views.items():
        camera.data.type = kind
        camera.data.ortho_scale = 0.95
        camera.data.lens = 70
        camera.data.sensor_fit = "VERTICAL"
        camera.data.clip_start, camera.data.clip_end = 0.05, 40.0
        camera.location = target + offset
        camera.rotation_euler = (target - camera.location).to_track_quat("-Z", "Y").to_euler()
        render(OUT / f"model_{name}.png", GROUPS)


def main() -> None:
    global COLLECTION
    OUT.mkdir(parents=True, exist_ok=True)
    for obj in ORIGINAL_MESHES:
        obj.hide_render = True
    COLLECTION = bpy.data.collections.new("armor_refined_03b")
    SCENE.collection.children.link(COLLECTION)
    if ARM.animation_data is None:
        ARM.animation_data_create()
    rest_pose()
    make_materials()
    breastplate()
    abdomen_lames()
    tassets()
    gorget()
    pauldron("near", -1, "upper_arm.R", "pauldron_near")
    pauldron("far", 1, "upper_arm.L", "pauldron_far")
    engine = setup_render_engine()
    print("ENGINE", engine, "PIECES", len(COLLECTION.objects))
    report = {"engine": engine, "params": P, "torso_sections": TORSO,
              "pieces": sorted((o.name, o.get("armor_group"), o.get("armor_part"), o.parent_bone)
                               for o in COLLECTION.objects)}
    poses, overlaps = {}, {}
    for pose_name, pose in TARGETS.items():
        scale = FIT[pose_name]["scale_px_per_m"]
        load_action(f"FIT02B_{pose_name}")
        action = f"FIT02B_{pose_name}"
        if pose_name == "idle":
            report["idle_shoulders"] = idle_shoulders(scale, pose["pivot"][0])
            action = "FIT03B_idle"
            keyframe(action)
        elif pose_name in LEAN_POSES:
            overlaps[f"{pose_name}_before_lean"] = intersections()
            report[f"{pose_name}_lean"] = lean_fix(pose_name, pose, scale)
            action = f"FIT03B_{pose_name}"
            keyframe(action)
            print("LEAN", report[f"{pose_name}_lean"])
        overlaps[pose_name] = intersections()
        side_camera(pose, scale)
        render(OUT / f"armor_{pose_name}_all.png", GROUPS)
        for group in GROUPS:
            render(OUT / f"armor_{pose_name}_{group}.png", (group,))
        render(OUT / f"armor_{pose_name}_gorget.png", GROUPS, {"gorget"})
        poses[pose_name] = {"action": action, "scale_px_per_m": scale, "canvas": pose["canvas"],
                            "pivot": pose["pivot"]}
        print("POSE", pose_name, overlaps[pose_name])
    report["poses"] = poses
    report["intersections_triangle_pairs"] = overlaps
    load_action("FIT03B_idle")
    model_sheet()
    (OUT / "armor_build.json").write_text(json.dumps(report, indent=1, default=str))
    bpy.ops.wm.save_as_mainfile(filepath=str(SAVE), copy=True)
    print("SAVED", SAVE)


main()
