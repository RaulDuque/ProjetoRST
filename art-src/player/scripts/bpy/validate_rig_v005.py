"""Reopen check of the saved v005 files, in a new Blender process (V1.2-E).

    blender --background --factory-startup art-src/player/blender/swordsman_rig_v005.blend \
        --python art-src/player/scripts/bpy/validate_rig_v005.py
    blender --background --factory-startup art-src/player/blender/swordsman_rig_v005_anim_tests.blend \
        --python art-src/player/scripts/bpy/validate_rig_v005.py

Checks armature, bones, constraints, bound meshes, weights, materials,
cameras, lights, pose actions (and TEST_ actions in the tests file), then
plays a pose action and checks the sabre attachment and IK. Writes
renders/v005/reopen_check_<file>.json; exits non-zero on any failure. Never
saves the file it opened.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

SCRIPTS_DIR = Path(__file__).resolve().parent
for entry in (SCRIPTS_DIR, SCRIPTS_DIR / "swordsman_v003", SCRIPTS_DIR / "swordsman_v004",
              SCRIPTS_DIR / "swordsman_rig_v005"):
    if str(entry) in sys.path:
        sys.path.remove(str(entry))
    sys.path.insert(0, str(entry))

import bpy  # noqa: E402

import rig_config as C  # noqa: E402
import rig_validate as V  # noqa: E402

POSE_KEYS = ("P01_neutral", "P02_walk_contact", "P03_run_extreme", "P04_jump_rise", "P05_jump_fall",
             "P06_basic_attack", "P07_crescent_slash", "P08_rising_edge", "P09_dodge", "P10_guard", "P11_hurt",
             "P12_magic_cast", "P13_death")


def fcurve_count(action: bpy.types.Action) -> int:
    if hasattr(action, "layers") and action.layers:
        return sum(len(bag.fcurves) for layer in action.layers for strip in layer.strips
                   for bag in strip.channelbags)
    return len(getattr(action, "fcurves", []))


def main() -> None:
    path = Path(bpy.data.filepath)
    tests_file = path.resolve() == C.ANIM_TESTS_BLEND.resolve()
    if path.resolve() not in (C.BLEND_PATH.resolve(), C.ANIM_TESTS_BLEND.resolve()):
        raise SystemExit(f"open a v005 file, got {path}")
    failures: list[str] = []
    arm = bpy.data.objects.get(C.ARMATURE_NAME)
    if arm is None or arm.type != "ARMATURE":
        raise SystemExit("armature missing")
    meshes = [o for o in bpy.data.collections[C.SOURCE_COLLECTION].objects if o.type == "MESH"]
    structure = V.check_structure(arm, meshes)
    weights = V.check_weights(meshes)
    failures += structure["failures"] + weights["failures"]
    if len(meshes) != 132:
        failures.append(f"{len(meshes)} meshes, expected 132 (v004)")
    materials = sorted({s.material.name for o in meshes for s in o.material_slots if s.material})
    cameras = sorted(o.name for o in bpy.data.objects if o.type == "CAMERA")
    lights = sorted(o.name for o in bpy.data.objects if o.type == "LIGHT")
    if len(lights) != 4:
        failures.append(f"{len(lights)} lights, expected the 4 of the v003/v004 rig")
    actions = {a.name: fcurve_count(a) for a in bpy.data.actions}
    for key in POSE_KEYS:
        if actions.get(f"POSE_{key}", 0) == 0:
            failures.append(f"pose action POSE_{key} missing or empty")
    tests = [n for n in actions if n.startswith("TEST_")]
    if tests_file and sorted(tests) != ["TEST_attack", "TEST_dodge", "TEST_walk"]:
        failures.append(f"test actions {tests}")
    if not tests_file and tests:
        failures.append("TEST_ actions belong to the anim tests file only")
    constraints = {pb.name: [c.name for c in pb.constraints] for pb in arm.pose.bones if pb.constraints}
    rest_offset = V.sword_offset(arm)
    arm.animation_data_create()
    arm.animation_data.action = bpy.data.actions["POSE_P06_basic_attack"]
    bpy.context.scene.frame_set(1)
    posed_offset = V.sword_offset(arm)
    moved = (arm.pose.bones["upper_arm.R"].matrix.to_quaternion()
             .rotation_difference(arm.data.bones["upper_arm.R"].matrix_local.to_quaternion()).angle)
    sword_drift = (posed_offset.translation - rest_offset.translation).length
    if moved < 0.5:
        failures.append("POSE_P06 did not move the sword arm after reopening")
    if sword_drift > 1e-5 or V.angle_between(posed_offset, rest_offset) > 1e-3:
        failures.append("sabre moved relative to the hand")
    arm.animation_data.action = None
    for pb in arm.pose.bones:
        pb.rotation_quaternion = (1.0, 0.0, 0.0, 0.0)
        pb.location = (0.0, 0.0, 0.0)
    bpy.context.view_layer.update()
    ik = V.check_ik(arm)
    if not ik["ok"]:
        failures.append(f"IK check failed: {ik}")
    report = {
        "file": path.name, "blender": bpy.app.version_string, "bones": structure["bones"],
        "deform_bones": structure["deform_bones"], "meshes": len(meshes), "vertices": weights["vertices"],
        "max_influences": weights["max_influences"], "materials": len(materials), "cameras": cameras,
        "lights": lights, "actions": actions, "constraints": constraints,
        "pose_P06_upper_arm_rotation_deg": round(moved * 57.29578, 2), "sword_drift_m": sword_drift, "ik": ik,
        "failures": failures,
    }
    out = C.OUT_DIR / f"reopen_check_{path.stem}.json"
    out.write_text(json.dumps(report, indent=2, ensure_ascii=False, default=str) + "\n", encoding="utf-8")
    print(f"[reopen] {path.name}: {len(failures)} failure(s) -> {out}")
    for f in failures:
        print(f"[reopen] FAIL {f}")
    if failures:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
