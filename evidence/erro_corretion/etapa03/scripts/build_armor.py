"""Etapa 03 (Erro-Corretion): pilot gothic armour (purple + gold) on the invisible mannequin.

Runs ONLY on a work copy of the Etapa 02B blend (never on the original rig):

    blender --background --factory-startup evidence/erro_corretion/etapa03/work/armor_pilot_work.blend \
        --python evidence/erro_corretion/etapa03/scripts/build_armor.py -- \
        --targets evidence/erro_corretion/etapa02/out/targets.json \
        --fit evidence/erro_corretion/etapa02/out/fit_results.json \
        --anchors evidence/erro_corretion/etapa02b/out/layers_report.json \
        --out evidence/erro_corretion/etapa03/out \
        --save evidence/erro_corretion/etapa03/work/armor_pilot_v001.blend

What it does
1. Hides every original mesh of the rig from render (body, clothes, coat, cape,
   sabre, hair): the mannequin is invisible.
2. Idle only: moves the clavicles so the shoulder lands on the visible sleeve cap
   measured in Etapa 02B (layers_report.json anchors), then re-aims the arm at the
   Etapa 02 elbow/wrist/fist targets. Action FIT03_idle. Run poses use FIT02B_*.
3. Builds the armour in the rest pose as independent meshes (own geometry,
   Solidify thickness, subdivision) and curves (gold trims), each rigidly
   parented to one bone:
     breastplate + gorget -> chest; abdomen lames -> spine_01 / pelvis;
     tassets -> thigh.R / thigh.L; pauldrons (dome, 3 lames, crest, rivets) ->
     upper_arm.R / upper_arm.L.
   Sizes come from the mannequin's torso cross-sections plus a stand-off, so the
   plate is bigger than the shirt (real volume), not a mask.
4. Renders, per pose, with the Etapa 02 side camera mapping (scale, pivot, sole
   edge), 8x supersampled, transparent film: the whole armour and one PNG per
   overlay group (far_low, pauldron_far, torso, pauldron_near) for the approved
   layer order. Also a model sheet (front, side, back, 3/4) in the idle pose.
"""

from __future__ import annotations

import json
import math
import sys
from pathlib import Path

import bpy
from mathutils import Matrix, Vector

ARGS = sys.argv[sys.argv.index("--") + 1:]
OPTS = dict(zip(ARGS[::2], ARGS[1::2], strict=True))
TARGETS = json.loads(Path(OPTS["--targets"]).read_text())
FIT = json.loads(Path(OPTS["--fit"]).read_text())
ANCHORS = json.loads(Path(OPTS["--anchors"]).read_text())
OUT = Path(OPTS["--out"])
SAVE = Path(OPTS["--save"])
SUPER = 8
SOLE_EDGE = 118.5

if "etapa03" not in bpy.data.filepath:
    raise SystemExit("refusing to run outside the etapa03 work copy")

SCENE = bpy.context.scene
ARM = bpy.data.objects["rig_v007"]
PB = ARM.pose.bones
ORIGINAL_MESHES = [o for o in bpy.data.objects if o.type == "MESH"]

# --- parameters (metres, rest pose; the model faces +X, near side is -Y) -----------------
# Mannequin torso cross-sections measured on torso_vest/collar/torso_trousers/belt:
# (z, back x, front x, half width y).
TORSO = [
    (0.96, -0.097, 0.092, 0.141), (1.02, -0.091, 0.090, 0.133), (1.08, -0.081, 0.093, 0.126),
    (1.14, -0.079, 0.097, 0.124), (1.20, -0.076, 0.106, 0.136), (1.26, -0.079, 0.115, 0.148),
    (1.32, -0.082, 0.117, 0.157), (1.38, -0.084, 0.108, 0.164), (1.44, -0.085, 0.085, 0.167),
]
P = {
    "segments": 72,            # around the torso
    "plate_standoff": 0.054,   # breastplate distance from the torso, front/back
    "plate_side": 0.032,       # breastplate distance from the torso, sides
    "plate_thickness": 0.012,
    "keel": 0.024,             # gothic central ridge
    "flutes": 0.004,           # fluting depth on the front
    "plate_top": 1.465, "neck_dip": 0.022, "armhole": 0.070,
    "plate_bottom": 1.130, "plackart_point": 0.050,
    "lames": 3, "lame_step": 0.034, "lame_height": 0.046, "lame_grow": 0.005,
    "tasset_top": 1.030, "tasset_flare": 0.022, "tasset_height": 0.072, "tasset_step": 0.058,
    "gorget_bottom": 1.430, "gorget_back": 1.525, "gorget_front": 1.485, "gorget_flare": 0.034,
    "pauldron_radius": 0.118, "pauldron_flatten": 0.22, "pauldron_lames": 3, "pauldron_thickness": 0.012,
    "trim": 0.0075, "trim_thin": 0.0050, "rivet": 0.0095,
}
GROUPS = ("far_low", "pauldron_far", "torso", "pauldron_near")


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


def torso_point(z: float, angle: float, front_off: float, side_off: float, keel: float = 0.0,
                flutes: float = 0.0) -> Vector:
    back, front, half = torso_section(z)
    centre, depth = (front + back) / 2, (front - back) / 2
    sx, sy = superellipse(angle)
    facing = max(0.0, math.cos(angle))
    x = centre + (depth + front_off) * sx + keel * facing ** 10 + flutes * facing ** 3 * math.cos(12 * angle)
    return Vector((x, (half + side_off) * sy, z))


# --- materials -------------------------------------------------------------------------------


def material(name: str, colour, metallic: float, roughness: float):
    mat = bpy.data.materials.get(name) or bpy.data.materials.new(name)
    mat.use_nodes = True
    bsdf = next(n for n in mat.node_tree.nodes if n.type == "BSDF_PRINCIPLED")
    bsdf.inputs["Base Color"].default_value = (*colour, 1.0)
    bsdf.inputs["Metallic"].default_value = metallic
    bsdf.inputs["Roughness"].default_value = roughness
    return mat


MAT = {}


def make_materials() -> None:
    MAT["purple"] = material("armor_purple", (0.105, 0.032, 0.165), 0.72, 0.36)
    MAT["purple_dark"] = material("armor_purple_dark", (0.060, 0.018, 0.100), 0.72, 0.42)
    MAT["gold"] = material("armor_gold", (0.78, 0.52, 0.16), 1.0, 0.30)


# --- geometry helpers -------------------------------------------------------------------------

COLLECTION = None


def link(obj, group: str, bone: str, mat) -> None:
    COLLECTION.objects.link(obj)
    obj["armor_group"] = group
    if mat is not None:
        obj.data.materials.append(mat)
    bone_data = ARM.data.bones[bone]
    rest = ARM.matrix_world @ bone_data.matrix_local @ Matrix.Translation((0.0, bone_data.length, 0.0))
    obj.parent = ARM
    obj.parent_type = "BONE"
    obj.parent_bone = bone
    obj.matrix_parent_inverse = rest.inverted()


def loft(name: str, rings: list[list[Vector]], group: str, bone: str, mat, thickness: float,
         cyclic: bool = True, subdivide: int = 1):
    verts = [v for ring in rings for v in ring]
    count = len(rings[0])
    faces = []
    for i in range(len(rings) - 1):
        for j in range(count if cyclic else count - 1):
            a, b = i * count + j, i * count + (j + 1) % count
            faces.append((a, b, (i + 1) * count + (j + 1) % count, (i + 1) * count + j))
    mesh = bpy.data.meshes.new(name)
    mesh.from_pydata(verts, [], faces)
    mesh.update()
    for poly in mesh.polygons:
        poly.use_smooth = True
    obj = bpy.data.objects.new(name, mesh)
    solid = obj.modifiers.new("thickness", "SOLIDIFY")
    solid.thickness = thickness
    solid.offset = 0.0
    solid.use_even_offset = True
    if subdivide:
        sub = obj.modifiers.new("smooth", "SUBSURF")
        sub.levels = sub.render_levels = subdivide
    link(obj, group, bone, mat)
    return obj


def trim(name: str, points: list[Vector], group: str, bone: str, radius: float, cyclic: bool = False):
    curve = bpy.data.curves.new(name, "CURVE")
    curve.dimensions = "3D"
    curve.bevel_depth = radius
    curve.bevel_resolution = 3
    curve.use_fill_caps = True
    spline = curve.splines.new("POLY")
    spline.points.add(len(points) - 1)
    for point, co in zip(spline.points, points, strict=True):
        point.co = (*co, 1.0)
    spline.use_cyclic_u = cyclic
    obj = bpy.data.objects.new(name, curve)
    link(obj, group, bone, MAT["gold"])
    return obj


def rivet(name: str, at: Vector, group: str, bone: str) -> None:
    mesh = bpy.data.meshes.new(name)
    obj = bpy.data.objects.new(name, mesh)
    import bmesh
    bm = bmesh.new()
    bmesh.ops.create_uvsphere(bm, u_segments=10, v_segments=6, radius=P["rivet"])
    bm.to_mesh(mesh)
    bm.free()
    for poly in mesh.polygons:
        poly.use_smooth = True
    obj.location = at
    link(obj, group, bone, MAT["gold"])


def outward(point: Vector, centre_x: float, by: float) -> Vector:
    d = Vector((point.x - centre_x, point.y, 0.0))
    return point + (d.normalized() * by if d.length > 1e-6 else Vector())


# --- armour pieces ----------------------------------------------------------------------------


def breastplate() -> None:
    n = P["segments"]
    angles = [2 * math.pi * j / n for j in range(n)]

    def top(a):
        f = max(0.0, math.cos(a))
        return P["plate_top"] - P["neck_dip"] * f ** 3 - P["armhole"] * abs(math.sin(a)) ** 6

    def bottom(a):
        return P["plate_bottom"] - P["plackart_point"] * max(0.0, math.cos(a)) ** 6

    rows = 30
    rings = []
    for k in range(rows + 1):
        t = k / rows
        rings.append([torso_point(bottom(a) + (top(a) - bottom(a)) * t, a, P["plate_standoff"], P["plate_side"],
                                  P["keel"], P["flutes"]) for a in angles])
    loft("armor_breastplate", rings, "torso", "chest", MAT["purple"], P["plate_thickness"])
    edge = P["plate_thickness"] / 2
    back, front, _ = torso_section(1.3)
    cx = (front + back) / 2
    trim("armor_breastplate_trim_top", [outward(p, cx, edge) for p in rings[-1]], "torso", "chest", P["trim"], True)
    trim("armor_breastplate_trim_bottom", [outward(p, cx, edge) for p in rings[0]], "torso", "chest", P["trim"], True)
    keel = [outward(ring[0], cx, edge) for ring in rings[2:-2]]
    trim("armor_breastplate_keel", keel, "torso", "chest", P["trim_thin"])
    # Gothic arches: two gold ribs rising from the plackart point to the chest.
    for side in (-1, 1):
        rib = []
        for k in range(3, rows - 4):
            a = side * (0.05 + 0.55 * (k / rows) ** 1.4)
            z = bottom(a) + (top(a) - bottom(a)) * (k / rows)
            rib.append(outward(torso_point(z, a, P["plate_standoff"], P["plate_side"], P["keel"], P["flutes"]),
                               cx, edge))
        trim(f"armor_breastplate_rib_{'near' if side < 0 else 'far'}", rib, "torso", "chest", P["trim_thin"])


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
             P["plate_thickness"] * 0.9)
        back, front, _ = torso_section(z_bot)
        cx = (front + back) / 2
        trim(f"armor_lame_{i}_trim", [outward(p, cx, P["plate_thickness"] * 0.45) for p in rings[0]], "torso", bone,
             P["trim_thin"], True)
        for j in range(0, n, 9):
            rivet(f"armor_lame_{i}_rivet_{j}", outward(rings[3][j], cx, P["plate_thickness"] * 0.5), "torso", bone)


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
                     P["plate_thickness"] * 0.9, cyclic=False)
                back, front, _ = torso_section(0.96)
                cx = (front + back) / 2
                trim(name + "_trim", [outward(p, cx, P["plate_thickness"] * 0.45) for p in rings[0]], group, bone,
                     P["trim_thin"])


def gorget() -> None:
    n = P["segments"]
    centre = Vector((-0.007, 0.0))
    angles = [2 * math.pi * j / n for j in range(n)]
    rings = []
    for k in range(9):
        t = k / 8
        ring = []
        for a in angles:
            front = max(0.0, math.cos(a))
            top = P["gorget_back"] - (P["gorget_back"] - P["gorget_front"]) * front ** 1.5
            z = P["gorget_bottom"] + (top - P["gorget_bottom"]) * t
            r_x = 0.086 + P["gorget_flare"] * t ** 1.3 + (0.035 * (1 - t))
            r_y = 0.088 + P["gorget_flare"] * t ** 1.3 + (0.065 * (1 - t))
            ring.append(Vector((centre.x + r_x * math.cos(a), r_y * math.sin(a), z)))
        rings.append(ring)
    loft("armor_gorget", rings, "torso", "chest", MAT["purple_dark"], P["plate_thickness"])
    trim("armor_gorget_trim", [outward(p, centre.x, P["plate_thickness"] * 0.5) for p in rings[-1]], "torso", "chest",
         P["trim"], True)


def frame(axis: Vector) -> tuple[Vector, Vector]:
    helper = Vector((1, 0, 0)) if abs(axis.x) < 0.9 else Vector((0, 1, 0))
    e1 = axis.cross(helper).normalized()
    return e1, axis.cross(e1).normalized()


def pauldron(side_name: str, sign: int, bone: str, group: str) -> None:
    bone_data = ARM.data.bones[bone]
    shoulder = Vector(bone_data.head_local)
    down = (Vector(bone_data.tail_local) - shoulder).normalized()
    out = Vector((0.0, sign, 0.0))
    out = (out - down * out.dot(down)).normalized()
    fwd = down.cross(out).normalized()
    if fwd.x < 0:
        fwd = -fwd
    radius = P["pauldron_radius"]
    centre = shoulder + out * 0.012 + down * 0.010
    axis = (-down * 0.80 + out * 0.60).normalized()
    e1, e2 = frame(axis)
    segments, rows = 48, 18
    rings = []
    for k in range(rows + 1):
        ring = []
        for j in range(segments):
            psi = 2 * math.pi * j / segments
            rim = e1 * math.cos(psi) + e2 * math.sin(psi)
            limit = math.radians(102 - 42 * max(0.0, rim.dot(-out)) + 10 * max(0.0, rim.dot(down)))
            phi = limit * k / rows
            direction = axis * math.cos(phi) + rim * math.sin(phi)
            stretch = 1 + 0.18 * max(0.0, direction.dot(down))
            v = direction * radius * stretch
            ring.append(centre + v - axis * v.dot(axis) * P["pauldron_flatten"])
        rings.append(ring)
    loft(f"armor_pauldron_{side_name}_dome", rings[1:], group, bone, MAT["purple"], P["pauldron_thickness"])
    trim(f"armor_pauldron_{side_name}_rim", [p + (p - centre).normalized() * P["pauldron_thickness"] * 0.5 for p in rings[-1]],
         group, bone, P["trim"], True)
    crest = []
    for k in range(-14, 15):
        phi = math.radians(78 * k / 14)
        direction = axis * math.cos(phi) + fwd * math.sin(phi)
        v = direction * (radius + P["pauldron_thickness"] * 0.5)
        crest.append(centre + v - axis * v.dot(axis) * P["pauldron_flatten"])
    trim(f"armor_pauldron_{side_name}_crest", crest, group, bone, P["trim"] * 1.15)
    for i in range(P["pauldron_lames"]):
        s0 = 0.085 + 0.040 * i
        ring_r = radius * 0.86 - 0.006 * i
        angles = [math.radians(-118 + 236 * j / 24) for j in range(25)]
        lame_rings = []
        for k in range(6):
            t = k / 5
            along = s0 + 0.055 * t
            r = ring_r + 0.016 * t
            lame_rings.append([centre + down * along + (out * math.cos(a) + fwd * math.sin(a)) * r for a in angles])
        loft(f"armor_pauldron_{side_name}_lame_{i}", lame_rings, group, bone,
             MAT["purple_dark"] if i % 2 == 0 else MAT["purple"], P["pauldron_thickness"] * 0.9, cyclic=False)
        edge = [p + (p - (centre + down * (s0 + 0.055))).normalized() * P["pauldron_thickness"] * 0.45
                for p in lame_rings[-1]]
        trim(f"armor_pauldron_{side_name}_lame_{i}_trim", edge, group, bone, P["trim_thin"])
        for j in (4, 12, 20):
            p = lame_rings[2][j]
            rivet(f"armor_pauldron_{side_name}_lame_{i}_rivet_{j}",
                  p + (p - (centre + down * (s0 + 0.022))).normalized() * P["pauldron_thickness"] * 0.5, group, bone)


# --- poses, camera, render --------------------------------------------------------------------


def update() -> None:
    bpy.context.view_layer.update()


def head(name: str) -> Vector:
    return ARM.matrix_world @ PB[name].head


def tail(name: str) -> Vector:
    return ARM.matrix_world @ PB[name].tail


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


def rest_pose() -> None:
    if ARM.animation_data:
        ARM.animation_data.action = None
    for b in PB:
        b.matrix_basis = Matrix.Identity(4)
    update()


def keyframe(action_name: str) -> None:
    action = bpy.data.actions.get(action_name) or bpy.data.actions.new(action_name)
    action.use_fake_user = True
    ARM.animation_data.action = action
    for bone in PB:
        bone.keyframe_insert("location", frame=1)
        mode = "rotation_quaternion" if bone.rotation_mode == "QUATERNION" else "rotation_euler"
        bone.keyframe_insert(mode, frame=1)
    ARM.animation_data.action = None


def idle_shoulders(scale: float, pivot_x: float) -> dict:
    """Idle: shoulder on the visible sleeve cap (Etapa 02B anchor), arm re-aimed."""
    anchor = ANCHORS["poses"]["idle"]["anchors"]["near"]
    target_px = (anchor[0], anchor[1] - 1.0)  # anchor = top-of-sleeve centroid + 2 rows; joint 1 row higher

    def world(p):
        return Vector(((p[0] - pivot_x) / scale, 0.0, (SOLE_EDGE - p[1]) / scale))

    goal = world(target_px)
    report = {"target_px": target_px}
    for side in ("R", "L"):
        pivot = head(f"clavicle.{side}")
        point = head(f"upper_arm.{side}") - pivot
        best = (1e9, 0.0, 0.0)
        for ax10 in range(-450, 451, 5):
            rx = Matrix.Rotation(math.radians(ax10 / 10), 3, "X")
            for az10 in range(-450, 451, 5):
                p = Matrix.Rotation(math.radians(az10 / 10), 3, "Z") @ (rx @ point) + pivot
                d = math.hypot(p.x - goal.x, p.z - goal.z) + 1e-5 * (abs(ax10) + abs(az10)) / 10
                if d < best[0]:
                    best = (d, ax10 / 10, az10 / 10)
        _, ax, az = best
        rotate_about(f"clavicle.{side}", pivot,
                     Matrix.Rotation(math.radians(az), 3, "Z") @ Matrix.Rotation(math.radians(ax), 3, "X"))
        arm = TARGETS["idle"]["near_arm"]
        aim(f"upper_arm.{side}", lambda s=side: head(f"forearm.{s}"), world(arm[1]))
        aim(f"forearm.{side}", lambda s=side: head(f"hand.{s}"), world(arm[2]))
        aim(f"hand.{side}", lambda s=side: tail(f"hand.{s}"), world(arm[3]))
        end = head(f"upper_arm.{side}")
        report[side] = {"rot_x_deg": ax, "rot_z_deg": az,
                        "shoulder_px": [round(end.x * scale + pivot_x, 2), round(SOLE_EDGE - end.z * scale, 2)]}
    return report


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
    background.inputs["Color"].default_value = (0.22, 0.20, 0.25, 1.0)
    background.inputs["Strength"].default_value = 0.9
    return SCENE.render.engine


def side_camera(pose: dict, scale: float):
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


def render(path: Path, groups) -> None:
    for obj in COLLECTION.objects:
        obj.hide_render = obj.get("armor_group") not in groups
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
        direction = target - camera.location
        camera.rotation_euler = direction.to_track_quat("-Z", "Y").to_euler()
        render(OUT / f"model_{name}.png", GROUPS)


def main() -> None:
    global COLLECTION
    OUT.mkdir(parents=True, exist_ok=True)
    for obj in ORIGINAL_MESHES:
        obj.hide_render = True
    COLLECTION = bpy.data.collections.new("armor_pilot")
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
              "pieces": sorted((o.name, o.get("armor_group"), o.parent_bone) for o in COLLECTION.objects)}
    poses = {}
    for pose_name, pose in TARGETS.items():
        scale = FIT[pose_name]["scale_px_per_m"]
        load_action(f"FIT02B_{pose_name}")
        if pose_name == "idle":
            report["idle_shoulders"] = idle_shoulders(scale, pose["pivot"][0])
            keyframe("FIT03_idle")
            print("IDLE", report["idle_shoulders"])
        side_camera(pose, scale)
        render(OUT / f"armor_{pose_name}_all.png", GROUPS)
        for group in GROUPS:
            render(OUT / f"armor_{pose_name}_{group}.png", (group,))
        poses[pose_name] = {"action": "FIT03_idle" if pose_name == "idle" else f"FIT02B_{pose_name}",
                            "scale_px_per_m": scale, "canvas": pose["canvas"], "pivot": pose["pivot"]}
        print("POSE", pose_name, "rendered")
    report["poses"] = poses
    load_action("FIT03_idle")
    model_sheet()
    (OUT / "armor_build.json").write_text(json.dumps(report, indent=1, default=str))
    bpy.ops.wm.save_as_mainfile(filepath=str(SAVE), copy=True)
    print("SAVED", SAVE)


main()
