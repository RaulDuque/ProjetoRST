"""Etapa 02 (Erro-Corretion): fit the v007 rig to the four approved poses.

Runs ONLY on a work copy of swordsman_rig_v007.blend (never on the original):

    blender --background --factory-startup evidence/erro_corretion/etapa02/work/swordsman_rig_v007_work.blend \
        --python evidence/erro_corretion/etapa02/scripts/fit_rig.py -- \
        --targets evidence/erro_corretion/etapa02/out/targets.json \
        --out evidence/erro_corretion/etapa02/out \
        --save evidence/erro_corretion/etapa02/work/swordsman_rig_v007_fit_poses.blend

Method
- Scale: one constant, 106 texels for the rest height of the model (the N3.5/N3.6
  contract: the standing figure is 106 texels tall in both clips).
- Mapping: texel index (x, y) <-> world X = (x - pivot_x) / S, Z = (118.5 - y) / S,
  i.e. the pivot column is X = 0 and the bottom edge of the sole row 118 is Z = 0.
- Posing (FK, side plane only): every bone is rotated about the world Y axis
  through its head, so its projected tip lies on the line to the target. Bone
  LENGTHS are never changed: the residual error is what the rig proportions
  cannot reach. Order: root translation, spine_01 (shoulder midpoint), chest
  twist about Z (shoulder separation), neck (head centre), arms, legs. The root
  translation is then refitted to the mean residual (camera/position share) and
  the chain is solved again (4 passes).
- Guides: Workbench, flat object colours, no anti-aliasing, transparent film,
  8x supersampled orthographic side camera matching the canvas. Costume
  (coat, half cape, pauldron, sword, hair strands) is hidden: the rig is a
  mannequin, not the character.
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
OUT = Path(OPTS["--out"])
SAVE = Path(OPTS["--save"])
SUPER = 8
FIGURE_TEXELS = 106
SOLE_EDGE = 118.5  # texel-index y of Z = 0 (bottom edge of row 118)

if "rig_v007" not in bpy.data.filepath and "work" not in bpy.data.filepath:
    raise SystemExit("refusing to run outside a work copy")
if bpy.data.filepath.endswith("art-src/player/blender/swordsman_rig_v007.blend"):
    raise SystemExit("refusing to run on the original rig")

ARM = bpy.data.objects["rig_v007"]
PB = ARM.pose.bones
SCENE = bpy.context.scene

GROUPS = {
    "torso": (1.0, 0.0, 0.0),
    "head": (1.0, 1.0, 0.0),
    "near_arm": (0.0, 1.0, 0.0),
    "far_arm": (0.0, 0.0, 1.0),
    "near_leg": (1.0, 0.0, 1.0),
    "far_leg": (0.0, 1.0, 1.0),
}
HIDDEN_PREFIXES = ("coat", "half_cape", "cape_", "pauldron", "blade", "sword_", "hair_")


def group_of(name: str) -> str | None:
    if name == "hair_scalp":
        return "head"
    if name.startswith(HIDDEN_PREFIXES):
        return None
    if name.startswith(("sleeve_", "cuff_", "glove_")):
        return "near_arm" if "near" in name else "far_arm"
    if name.startswith(("leg_", "boot_")):
        return "near_leg" if "near" in name else "far_leg"
    if name.startswith(("head", "ear_", "eye_", "lid_", "brow_", "mouth")):
        return "head"
    return "torso"  # torso_vest, collar, vest_*, neck, belt*, torso_trousers, trouser_fly


def update() -> None:
    bpy.context.view_layer.update()


def head(name: str) -> Vector:
    return ARM.matrix_world @ PB[name].head


def tail(name: str) -> Vector:
    return ARM.matrix_world @ PB[name].tail


def mid(name: str) -> Vector:
    return (head(name) + tail(name)) / 2


def rest_height() -> float:
    depsgraph = bpy.context.evaluated_depsgraph_get()
    top, bottom = -1e9, 1e9
    for obj in bpy.data.objects:
        if obj.type != "MESH" or obj.hide_render:
            continue
        evaluated = obj.evaluated_get(depsgraph)
        mesh = evaluated.to_mesh()
        for vertex in mesh.vertices:
            z = (evaluated.matrix_world @ vertex.co).z
            top, bottom = max(top, z), min(bottom, z)
        evaluated.to_mesh_clear()
    return top - bottom


def rotate_about(name: str, pivot: Vector, rotation: Matrix) -> None:
    move = Matrix.Translation(pivot) @ rotation.to_4x4() @ Matrix.Translation(-pivot)
    PB[name].matrix = move @ PB[name].matrix
    update()


def aim(name: str, tip, target: Vector) -> None:
    """Rotate `name` about world Y through its head so that the projected tip points at target."""
    update()
    pivot = head(name)
    point = tip()
    now = math.atan2(point.z - pivot.z, point.x - pivot.x)
    want = math.atan2(target.z - pivot.z, target.x - pivot.x)
    rotate_about(name, pivot, Matrix.Rotation(-(want - now), 3, "Y"))


def translate_root(delta: Vector) -> None:
    PB["root"].matrix = Matrix.Translation(Vector((delta.x, 0.0, delta.z))) @ PB["root"].matrix
    update()


def landmarks() -> dict[str, Vector]:
    out = {"head": mid("head"), "neck": mid("neck"), "waist": head("spine_01")}
    for side, key in (("R", "near"), ("L", "far")):
        out[f"{key}_shoulder"] = head(f"upper_arm.{side}")
        out[f"{key}_elbow"] = head(f"forearm.{side}")
        out[f"{key}_wrist"] = head(f"hand.{side}")
        out[f"{key}_fist"] = tail(f"hand.{side}")
        out[f"{key}_hip"] = head(f"thigh.{side}")
        out[f"{key}_knee"] = head(f"shin.{side}")
        out[f"{key}_ankle"] = head(f"foot.{side}")
        out[f"{key}_toe"] = tail(f"toe.{side}")
    return out


def sprite_landmarks(pose: dict) -> dict[str, tuple[float, float]]:
    out = {"head": tuple(pose["head"]), "neck": tuple(pose["neck"])}
    for key in ("near", "far"):
        arm, leg = pose[f"{key}_arm"], pose[f"{key}_leg"]
        for label, point in zip(("shoulder", "elbow", "wrist", "fist"), arm, strict=True):
            out[f"{key}_{label}"] = tuple(point)
        for label, point in zip(("hip", "knee", "ankle", "toe"), leg, strict=True):
            out[f"{key}_{label}"] = tuple(point)
    return out


def fit(pose_name: str, pose: dict, scale: float) -> dict:
    pivot_x = pose["pivot"][0]

    def world(point) -> Vector:
        return Vector(((point[0] - pivot_x) / scale, 0.0, (SOLE_EDGE - point[1]) / scale))

    def texel(vector: Vector) -> tuple[float, float]:
        return (vector.x * scale + pivot_x, SOLE_EDGE - vector.z * scale)

    for bone in PB:
        bone.matrix_basis = Matrix.Identity(4)
    update()
    targets = sprite_landmarks(pose)
    hip_mid = (Vector(world(targets["near_hip"])) + Vector(world(targets["far_hip"]))) / 2
    rig_hips = (head("thigh.R") + head("thigh.L")) / 2
    translate_root(hip_mid - rig_hips)
    hip_lock = PB["root"].matrix.translation.copy()
    twist_degrees = 0.0
    pelvis_degrees = 0.0
    for _ in range(4):
        # Pelvis twist: horizontal separation of the hip joints (0 when hidden).
        hip_sep = 0.0 if pose["far_hidden"] else (
            targets["far_hip"][0] - targets["near_hip"][0]) / scale
        hip_now = head("thigh.L").x - head("thigh.R").x
        hip_width = abs(head("thigh.L").y - head("thigh.R").y)
        if hip_width > 1e-6:
            want = math.asin(max(-1.0, min(1.0, hip_sep / hip_width)))
            have = math.asin(max(-1.0, min(1.0, hip_now / hip_width)))
            rotate_about("pelvis", head("pelvis"), Matrix.Rotation(-(want - have), 3, "Z"))
            pelvis_degrees = math.degrees(want)
        shoulders =(world(targets["near_shoulder"]) + world(targets["far_shoulder"])) / 2
        aim("spine_01", lambda: (head("upper_arm.R") + head("upper_arm.L")) / 2, shoulders)
        # Torso twist: horizontal separation of the shoulders (0 when hidden).
        separation = 0.0 if pose["far_hidden"] else (
            targets["far_shoulder"][0] - targets["near_shoulder"][0]) / scale
        now = head("upper_arm.L").x - head("upper_arm.R").x
        width = abs(head("upper_arm.L").y - head("upper_arm.R").y)
        if width > 1e-6:
            want = math.asin(max(-1.0, min(1.0, separation / width)))
            have = math.asin(max(-1.0, min(1.0, now / width)))
            pivot = (head("upper_arm.R") + head("upper_arm.L")) / 2
            rotate_about("chest", pivot, Matrix.Rotation(-(want - have), 3, "Z"))
            twist_degrees = math.degrees(want)
        aim("neck", lambda: mid("head"), world(targets["head"]))
        for side, key in (("R", "near"), ("L", "far")):
            aim(f"upper_arm.{side}", lambda s=side: head(f"forearm.{s}"), world(targets[f"{key}_elbow"]))
            aim(f"forearm.{side}", lambda s=side: head(f"hand.{s}"), world(targets[f"{key}_wrist"]))
            aim(f"hand.{side}", lambda s=side: tail(f"hand.{s}"), world(targets[f"{key}_fist"]))
            aim(f"thigh.{side}", lambda s=side: head(f"shin.{s}"), world(targets[f"{key}_knee"]))
            aim(f"shin.{side}", lambda s=side: head(f"foot.{s}"), world(targets[f"{key}_ankle"]))
            aim(f"foot.{side}", lambda s=side: tail(f"toe.{s}"), world(targets[f"{key}_toe"]))
        rig = landmarks()
        keys = [k for k in targets if not (pose["far_hidden"] and k.startswith("far_"))]
        residual = sum((world(targets[k]) - rig[k] for k in keys), Vector()) / len(keys)
        translate_root(Vector((residual.x, 0.0, residual.z)))
    # Final solve after the last translation.
    aim("neck", lambda: mid("head"), world(targets["head"]))
    for side, key in (("R", "near"), ("L", "far")):
        aim(f"upper_arm.{side}", lambda s=side: head(f"forearm.{s}"), world(targets[f"{key}_elbow"]))
        aim(f"forearm.{side}", lambda s=side: head(f"hand.{s}"), world(targets[f"{key}_wrist"]))
        aim(f"hand.{side}", lambda s=side: tail(f"hand.{s}"), world(targets[f"{key}_fist"]))
        aim(f"thigh.{side}", lambda s=side: head(f"shin.{s}"), world(targets[f"{key}_knee"]))
        aim(f"shin.{side}", lambda s=side: head(f"foot.{s}"), world(targets[f"{key}_ankle"]))
        aim(f"foot.{side}", lambda s=side: tail(f"toe.{s}"), world(targets[f"{key}_toe"]))
    rig = landmarks()
    offset = (PB["root"].matrix.translation - hip_lock) * scale
    result = {
        "pose": pose_name, "scale_px_per_m": scale, "pivot": pose["pivot"], "canvas": pose["canvas"],
        "far_hidden": pose["far_hidden"], "chest_twist_deg": round(twist_degrees, 2),
        "pelvis_twist_deg": round(pelvis_degrees, 2),
        "root_offset_from_hip_lock_px": [round(offset.x, 2), round(-offset.z, 2)],
        "rig": {k: [round(c, 3) for c in texel(v)] for k, v in rig.items()},
        "sprite": {k: list(v) for k, v in targets.items()},
        "belt_y": pose["belt_y"],
        "bone_lengths_px": {b.name: round(b.length * scale, 2) for b in ARM.data.bones if b.use_deform},
    }
    return result


def keyframe(action_name: str) -> None:
    if ARM.animation_data is None:
        ARM.animation_data_create()
    action = bpy.data.actions.new(action_name)
    action.use_fake_user = True
    ARM.animation_data.action = action
    for bone in PB:
        bone.keyframe_insert("location", frame=1)
        mode = "rotation_quaternion" if bone.rotation_mode == "QUATERNION" else "rotation_euler"
        bone.keyframe_insert(mode, frame=1)


def setup_render(pose: dict, scale: float) -> None:
    SCENE.render.engine = "BLENDER_WORKBENCH"
    shading = SCENE.display.shading
    shading.light = "FLAT"
    shading.color_type = "OBJECT"
    shading.show_object_outline = False
    shading.show_shadows = False
    shading.show_cavity = False
    SCENE.display.render_aa = "OFF"
    SCENE.render.film_transparent = True
    SCENE.view_settings.view_transform = "Standard"
    SCENE.view_settings.look = "None"
    SCENE.render.image_settings.file_format = "PNG"
    SCENE.render.image_settings.color_mode = "RGBA"
    width, height = pose["canvas"]
    SCENE.render.resolution_x = width * SUPER
    SCENE.render.resolution_y = height * SUPER
    SCENE.render.resolution_percentage = 100
    SCENE.render.pixel_aspect_x = SCENE.render.pixel_aspect_y = 1.0
    camera = bpy.data.objects.get("fit_cam")
    if camera is None:
        camera = bpy.data.objects.new("fit_cam", bpy.data.cameras.new("fit_cam"))
        SCENE.collection.objects.link(camera)
    camera.data.type = "ORTHO"
    camera.data.sensor_fit = "VERTICAL"
    camera.data.ortho_scale = height / scale
    camera.data.clip_start, camera.data.clip_end = 0.1, 40.0
    centre_x = (width / 2 - 0.5 - pose["pivot"][0]) / scale
    centre_z = (SOLE_EDGE - (height / 2 - 0.5)) / scale
    camera.location = (centre_x, -10.0, centre_z)
    camera.rotation_euler = (math.radians(90), 0.0, 0.0)
    SCENE.camera = camera


def render(path: Path, only: str | None) -> None:
    for obj in bpy.data.objects:
        if obj.type != "MESH":
            continue
        group = group_of(obj.name)
        obj.hide_render = group is None or (only is not None and group != only)
        if group is not None:
            obj.color = (*GROUPS[group], 1.0)
    SCENE.render.filepath = str(path)
    bpy.ops.render.render(write_still=True)


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    for obj in bpy.data.objects:  # side check: "near" meshes are on -Y (rig .R)
        if obj.name in ("leg_near", "leg_far", "sleeve_near", "sleeve_far"):
            ys = [v.co.y for v in obj.data.vertices]
            print("SIDE", obj.name, round(sum(ys) / len(ys), 3))
    if ARM.animation_data and ARM.animation_data.action:
        ARM.animation_data.action = None
    for bone in PB:
        bone.matrix_basis = Matrix.Identity(4)
    update()
    height = rest_height()
    scale = FIGURE_TEXELS / height
    print("REST_HEIGHT", round(height, 4), "SCALE", round(scale, 4))
    results = {}
    for pose_name, pose in TARGETS.items():
        result = fit(pose_name, pose, scale)
        result["rest_height_m"] = height
        results[pose_name] = result
        keyframe(f"FIT_{pose_name}")
        setup_render(pose, scale)
        render(OUT / f"guide_{pose_name}_all.png", None)
        render(OUT / f"guide_{pose_name}_torso.png", "torso")
        render(OUT / f"guide_{pose_name}_near_arm.png", "near_arm")
        print("FIT", pose_name, "twist", result["chest_twist_deg"], "offset", result["root_offset_from_hip_lock_px"])
    (OUT / "fit_results.json").write_text(json.dumps(results, indent=1))
    bpy.ops.wm.save_as_mainfile(filepath=str(SAVE), copy=True)
    print("SAVED", SAVE)


main()
