"""Etapa 02B (Erro-Corretion): clavicle/shoulder fix of the invisible mannequin.

Runs ONLY on a work copy of the Etapa 02 fit (never on the original rig):

    blender --background --factory-startup evidence/erro_corretion/etapa02b/work/swordsman_rig_v007_02b_work.blend \
        --python evidence/erro_corretion/etapa02b/scripts/fit_shoulders.py -- \
        --targets evidence/erro_corretion/etapa02/out/targets.json \
        --fit evidence/erro_corretion/etapa02/out/fit_results.json \
        --out evidence/erro_corretion/etapa02b/out \
        --save evidence/erro_corretion/etapa02b/work/swordsman_rig_v007_02b_shoulders.blend

For each pose the Etapa 02 action FIT_<pose> is loaded unchanged. In C1 #0 and #2
only the clavicles are rotated (2 degrees of freedom, about the world X and Z axes
through the clavicle head) so that the projected shoulder joint lands on the
sprite's shoulder; the arm below is then re-aimed at the same elbow/wrist/fist
targets as in Etapa 02 (same method). Spine, neck, root, legs, camera, scale and
pivot are untouched. Nothing is rendered: the mannequin stays invisible; the
evidence uses the joint positions only. Result: out/shoulders.json and the new
actions FIT02B_<pose> in the saved copy.
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
OUT = Path(OPTS["--out"])
SAVE = Path(OPTS["--save"])
SOLE_EDGE = 118.5
FIX_POSES = {"run_c1_0", "run_c1_2"}
SEARCH = [i * 0.25 for i in range(-180, 181)]  # degrees, +-45

if "02b" not in bpy.data.filepath:
    raise SystemExit("refusing to run outside the 02B work copy")

ARM = bpy.data.objects["rig_v007"]
PB = ARM.pose.bones
SCENE = bpy.context.scene


def update() -> None:
    bpy.context.view_layer.update()


def head(name: str) -> Vector:
    return ARM.matrix_world @ PB[name].head


def tail(name: str) -> Vector:
    return ARM.matrix_world @ PB[name].tail


def mid(name: str) -> Vector:
    return (head(name) + tail(name)) / 2


def rotate_about(name: str, pivot: Vector, rotation: Matrix) -> None:
    move = Matrix.Translation(pivot) @ rotation.to_4x4() @ Matrix.Translation(-pivot)
    PB[name].matrix = move @ PB[name].matrix
    update()


def aim(name: str, tip, target: Vector) -> None:
    update()
    pivot = head(name)
    point = tip()
    now = math.atan2(point.z - pivot.z, point.x - pivot.x)
    want = math.atan2(target.z - pivot.z, target.x - pivot.x)
    rotate_about(name, pivot, Matrix.Rotation(-(want - now), 3, "Y"))


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


def keyframe(action_name: str) -> None:
    action = bpy.data.actions.new(action_name)
    action.use_fake_user = True
    ARM.animation_data.action = action
    for bone in PB:
        bone.keyframe_insert("location", frame=1)
        mode = "rotation_quaternion" if bone.rotation_mode == "QUATERNION" else "rotation_euler"
        bone.keyframe_insert(mode, frame=1)
    ARM.animation_data.action = None


def solve_clavicle(side: str, target: Vector) -> tuple[float, float, float]:
    """Best (x_deg, z_deg) rotation of the clavicle; returns them and the residual (m)."""
    pivot = head(f"clavicle.{side}")
    point = head(f"upper_arm.{side}") - pivot
    best = (1e9, 0.0, 0.0)
    for ax in SEARCH:
        rx = Matrix.Rotation(math.radians(ax), 3, "X")
        for az in SEARCH[::2]:
            p = Matrix.Rotation(math.radians(az), 3, "Z") @ (rx @ point) + pivot
            d = math.hypot(p.x - target.x, p.z - target.z)
            # Prefer the smallest rotation among equally good solutions.
            score = d + 1e-5 * (abs(ax) + abs(az))
            if score < best[0]:
                best = (score, ax, az)
    _, ax, az = best
    rotation = Matrix.Rotation(math.radians(az), 3, "Z") @ Matrix.Rotation(math.radians(ax), 3, "X")
    rotate_about(f"clavicle.{side}", pivot, rotation)
    end = head(f"upper_arm.{side}")
    return ax, az, math.hypot(end.x - target.x, end.z - target.z)


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    if ARM.animation_data is None:
        ARM.animation_data_create()
    results = {}
    for pose_name, pose in TARGETS.items():
        scale = FIT[pose_name]["scale_px_per_m"]
        pivot_x = pose["pivot"][0]

        def world(point, scale=scale, pivot_x=pivot_x) -> Vector:
            return Vector(((point[0] - pivot_x) / scale, 0.0, (SOLE_EDGE - point[1]) / scale))

        def texel(v: Vector, scale=scale, pivot_x=pivot_x) -> list[float]:
            return [round(v.x * scale + pivot_x, 3), round(SOLE_EDGE - v.z * scale, 3)]

        load_action(f"FIT_{pose_name}")
        before = {k: texel(v) for k, v in landmarks().items()}
        clavicles = {}
        if pose_name in FIX_POSES:
            for side, key in (("R", "near"), ("L", "far")):
                arm = pose[f"{key}_arm"]
                ax, az, residual = solve_clavicle(side, world(arm[0]))
                clavicles[side] = {"rot_x_deg": ax, "rot_z_deg": az, "residual_px": round(residual * scale, 3)}
                aim(f"upper_arm.{side}", lambda s=side: head(f"forearm.{s}"), world(arm[1]))
                aim(f"forearm.{side}", lambda s=side: head(f"hand.{s}"), world(arm[2]))
                aim(f"hand.{side}", lambda s=side: tail(f"hand.{s}"), world(arm[3]))
        after = {k: texel(v) for k, v in landmarks().items()}
        keyframe(f"FIT02B_{pose_name}")
        results[pose_name] = {"fixed": pose_name in FIX_POSES, "clavicles": clavicles,
                              "before": before, "after": after, "scale_px_per_m": scale}
        print("POSE", pose_name, clavicles)
    (OUT / "shoulders.json").write_text(json.dumps(results, indent=1))
    bpy.ops.wm.save_as_mainfile(filepath=str(SAVE), copy=True)
    print("SAVED", SAVE)


main()
