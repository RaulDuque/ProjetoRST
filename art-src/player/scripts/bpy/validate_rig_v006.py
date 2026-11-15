"""Reopen check of the saved v006 files, in a new Blender process (V1.2-F).

    blender --background --factory-startup art-src/player/blender/swordsman_rig_v006.blend \
        --python art-src/player/scripts/bpy/validate_rig_v006.py
    blender --background --factory-startup art-src/player/blender/swordsman_rig_v006_anim_tests.blend \
        --python art-src/player/scripts/bpy/validate_rig_v006.py

Checks armature, bones (the 42 of v005 + collar_back), constraints of the
v006 corrections, bound meshes, weights, dual-quaternion flags, materials,
cameras, lights, pose actions (and TEST_ actions in the tests file); then
replays every POSE_ action and checks the sabre attachment, the pose scale,
the grounding against the build manifest and the IK. Writes
renders/v006/reopen_check_<file>.json; exits non-zero on any failure. Never
saves the file it opened.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

SCRIPTS_DIR = Path(__file__).resolve().parent
for entry in (SCRIPTS_DIR, SCRIPTS_DIR / "swordsman_v003", SCRIPTS_DIR / "swordsman_v004",
              SCRIPTS_DIR / "swordsman_rig_v005", SCRIPTS_DIR / "swordsman_rig_v006"):
    if str(entry) in sys.path:
        sys.path.remove(str(entry))
    sys.path.insert(0, str(entry))

import bpy  # noqa: E402

import poses as PS  # noqa: E402
import rig_config as C  # noqa: E402
import rig_validate as V  # noqa: E402
from skeleton import set_ik  # noqa: E402

POSE_KEYS = tuple(p.key for p in PS.POSES)
V006_CONSTRAINTS = {
    "cape_arm.L_01": {"follow_arm_back_out", "follow_arm_fwd_in", "counter_chest_twist"},
    "coat_back.R_01": {"follow_thigh_fwd", "release_thigh_fwd", "follow_thigh_back"},
    "coat_back.L_01": {"follow_thigh_fwd", "release_thigh_fwd", "follow_thigh_back"},
    "coat_front.R_01": {"flare_thigh"},
    "coat_front.L_01": {"flare_thigh"},
    "collar_back": {"follow_neck", "follow_head"},
}


def fcurve_count(action: bpy.types.Action) -> int:
    if hasattr(action, "layers") and action.layers:
        return sum(len(bag.fcurves) for layer in action.layers for strip in layer.strips
                   for bag in strip.channelbags)
    return len(getattr(action, "fcurves", []))


def main() -> None:
    path = Path(bpy.data.filepath)
    tests_file = path.resolve() == C.ANIM_TESTS_BLEND.resolve()
    if path.resolve() not in (C.BLEND_PATH.resolve(), C.ANIM_TESTS_BLEND.resolve()):
        raise SystemExit(f"open a v006 file, got {path}")
    failures: list[str] = []
    arm = bpy.data.objects.get(C.ARMATURE_NAME)
    if arm is None or arm.type != "ARMATURE":
        raise SystemExit("armature rig_v006 missing")
    if bpy.data.objects.get(C.SOURCE_ARMATURE) is not None:
        failures.append("the v005 armature is still present")
    meshes = [o for o in bpy.data.collections[C.SOURCE_COLLECTION].objects if o.type == "MESH"]
    structure = V.check_structure(arm, meshes)
    weights = V.check_weights(meshes)
    failures += structure["failures"] + weights["failures"]
    if (structure["bones"], structure["deform_bones"]) != (C.SOURCE_BONE_COUNT + 1, C.SOURCE_DEFORM_COUNT + 1):
        failures.append(f"{structure['bones']}/{structure['deform_bones']} bones, expected 43/36")
    if len(meshes) != 132:
        failures.append(f"{len(meshes)} meshes, expected 132 (v004)")
    bones = arm.data.bones
    if "collar_back" not in bones or bones["collar_back"].parent is None or bones["collar_back"].parent.name != "chest":
        failures.append("collar_back missing or not a child of chest")
    gap = (bones["cape_arm.L_01"].head_local - bones["upper_arm.L"].head_local).length
    if gap > 1e-5:
        failures.append(f"cape_arm.L_01 pivot is {gap:.4f} m away from the shoulder")
    for bone, names in V006_CONSTRAINTS.items():
        present = {c.name for c in arm.pose.bones[bone].constraints}
        if not names <= present:
            failures.append(f"{bone}: constraints {sorted(names - present)} missing")
    dqs = sorted(o.name for o in meshes if o.modifiers[0].use_deform_preserve_volume)
    for obj in meshes:
        if obj.name.startswith(C.PRESERVE_VOLUME_EXTRA) and obj.name not in dqs:
            failures.append(f"{obj.name}: preserve volume off")
    materials = sorted({s.material.name for o in meshes for s in o.material_slots if s.material})
    cameras = sorted(o.name for o in bpy.data.objects if o.type == "CAMERA")
    lights = sorted(o.name for o in bpy.data.objects if o.type == "LIGHT")
    if len(lights) != 4:
        failures.append(f"{len(lights)} lights, expected the 4 of the v003-v005 rig")
    actions = {a.name: fcurve_count(a) for a in bpy.data.actions}
    for key in POSE_KEYS:
        if actions.get(f"POSE_{key}", 0) == 0:
            failures.append(f"pose action POSE_{key} missing or empty")
    tests = [n for n in actions if n.startswith("TEST_")]
    if tests_file and sorted(tests) != ["TEST_attack", "TEST_dodge", "TEST_walk"]:
        failures.append(f"test actions {tests}")
    if not tests_file and tests:
        failures.append("TEST_ actions belong to the anim tests file only")

    manifest_path = C.OUT_DIR / "rig_manifest_v006.json"
    manifest = json.loads(manifest_path.read_text(encoding="utf-8")) if manifest_path.exists() else {}
    feet = [bpy.data.objects[n] for n in PS.FEET]
    body = PS.all_body()
    grounded = [o for o in body if not o.name.startswith(("blade", "sword_"))]
    set_ik(arm, False)
    rest_offset = V.sword_offset(arm)
    arm.animation_data_create()
    replay = {}
    for key, pose in ((p.key, p) for p in PS.POSES):
        arm.animation_data.action = bpy.data.actions[f"POSE_{key}"]
        bpy.context.scene.frame_set(1)
        offset = V.sword_offset(arm)
        drift = (offset.translation - rest_offset.translation).length
        low = PS.lowest_z(grounded if pose.ground == "all" else feet)
        expected = manifest.get("poses", {}).get(key, {}).get("root_height_m")
        replay[key] = {"sword_drift_m": round(drift, 7), "sword_angle_deg": round(V.angle_between(offset, rest_offset), 5),
                       "lowest_z_m": round(low, 5), "scale_failures": V.check_scale(arm)}
        if drift > 1e-5 or V.angle_between(offset, rest_offset) > 1e-3:
            failures.append(f"{key}: sabre moved relative to the hand")
        if replay[key]["scale_failures"]:
            failures.append(f"{key}: {replay[key]['scale_failures']}")
        if pose.ground != "air" and abs(low) > 1e-3:
            failures.append(f"{key}: lowest point at {low:.4f} m, not on the floor (pivot line)")
        if pose.ground == "air" and low < 0.05:
            failures.append(f"{key}: airborne pose touches the floor")
        if expected is not None:
            replay[key]["root_height_manifest_m"] = expected
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
        "max_influences": weights["max_influences"], "worst_sum_error": weights["worst_sum_error"],
        "materials": len(materials), "cameras": cameras, "lights": lights, "actions": actions,
        "preserve_volume": dqs, "replay": replay, "ik": ik, "failures": failures,
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
