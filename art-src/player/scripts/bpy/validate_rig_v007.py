"""Reopen check of the v007 files, in a new Blender process (V1.2-G).

    blender --background --factory-startup art-src/player/blender/swordsman_rig_v007.blend \
        --python art-src/player/scripts/bpy/validate_rig_v007.py
    blender --background --factory-startup art-src/player/blender/swordsman_rig_v007_animations.blend \
        --python art-src/player/scripts/bpy/validate_rig_v007.py

Model file: armature (43 bones, 36 deform), bound meshes, weights, the
pauldron poles merged, the cloth shells without even thickness, the 13
POSE_ actions, sabre and IK; shells stable in every pose action.
Animations file: also the three ANIM_ actions (walk, attack_basic, dodge):
keys on every frame, constant interpolation, frame ranges, the keyframe
hashes of the build manifest, and every frame replayed (sabre, grounding,
pose scale). Writes renders/v007/reopen_check_<file>.json; never saves.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

SCRIPTS_DIR = Path(__file__).resolve().parent
for entry in (SCRIPTS_DIR, SCRIPTS_DIR / "swordsman_v003", SCRIPTS_DIR / "swordsman_v004",
              SCRIPTS_DIR / "swordsman_rig_v005", SCRIPTS_DIR / "swordsman_rig_v006",
              SCRIPTS_DIR / "swordsman_rig_v007"):
    if str(entry) in sys.path:
        sys.path.remove(str(entry))
    sys.path.insert(0, str(entry))

import bpy  # noqa: E402

import animations as AN  # noqa: E402
import poses as PS  # noqa: E402
import rig_config as C  # noqa: E402
import rig_validate as V  # noqa: E402
import shell_fix as SF  # noqa: E402
from build_animations_v007 import keyframe_hash  # noqa: E402
from skeleton import set_ik  # noqa: E402


def fcurves(action: bpy.types.Action) -> list[bpy.types.FCurve]:
    if hasattr(action, "layers") and action.layers:
        return [fc for layer in action.layers for strip in layer.strips for bag in strip.channelbags
                for fc in bag.fcurves]
    return list(getattr(action, "fcurves", []))


def main() -> None:
    path = Path(bpy.data.filepath)
    anim_file = path.resolve() == C.ANIM_BLEND.resolve()
    if path.resolve() not in (C.BLEND_PATH.resolve(), C.ANIM_BLEND.resolve()):
        raise SystemExit(f"open a v007 file, got {path}")
    failures: list[str] = []
    arm = bpy.data.objects.get(C.ARMATURE_NAME)
    if arm is None:
        raise SystemExit("armature rig_v007 missing")
    meshes = [o for o in bpy.data.collections[C.SOURCE_COLLECTION].objects if o.type == "MESH"]
    structure = V.check_structure(arm, meshes)
    weights = V.check_weights(meshes)
    failures += structure["failures"] + weights["failures"]
    if (structure["bones"], structure["deform_bones"]) != (43, 36) or len(meshes) != 132:
        failures.append(f"{structure['bones']}/{structure['deform_bones']} bones, {len(meshes)} meshes")
    for name in C.PAULDRON_OBJECTS:
        topo = SF.topology(bpy.data.objects[name])
        if topo["zero_area_faces"] or topo["zero_length_edges"] or topo["vertices"] != 325:
            failures.append(f"{name}: pole not merged {topo}")
    for name in C.EVEN_OFFSET_OFF:
        mod = next(m for m in bpy.data.objects[name].modifiers if m.type == "SOLIDIFY")
        if mod.use_even_offset:
            failures.append(f"{name}: even thickness still on")
    actions = {a.name: a for a in bpy.data.actions}
    for p in PS.POSES:
        if f"POSE_{p.key}" not in actions:
            failures.append(f"POSE_{p.key} missing")
    anim_names = sorted(n for n in actions if n.startswith("ANIM_"))
    expected_anims = sorted(f"ANIM_{k}" for k in AN.CLIPS)
    if anim_file and anim_names != expected_anims:
        failures.append(f"animation actions {anim_names}")
    if not anim_file and anim_names:
        failures.append("ANIM_ actions belong to the animations file only")

    shells = SF.solidify_objects(meshes)
    set_ik(arm, False)
    rest_offset = V.sword_offset(arm)
    arm.animation_data_create()
    replay: dict[str, object] = {}
    worst_shell = 0.0
    for p in PS.POSES:
        arm.animation_data.action = actions[f"POSE_{p.key}"]
        bpy.context.scene.frame_set(1)
        worst_shell = max(worst_shell, max(SF.shell_residuals(arm, shells).values()))
    replay["pose_actions_max_shell_residual_m"] = worst_shell
    if worst_shell > C.SHELL_JUMP_TOLERANCE_M:
        failures.append(f"unstable shell in a pose action: {worst_shell} m")

    if anim_file:
        manifest = json.loads((C.ANIM_OUT_DIR / "animation_manifest.json").read_text(encoding="utf-8"))
        feet = [bpy.data.objects[n] for n in PS.FEET]
        for key, clip in AN.CLIPS.items():
            action = actions[f"ANIM_{key}"]
            info: dict[str, object] = {"frame_range": [int(action.frame_range[0]), int(action.frame_range[1])]}
            if info["frame_range"] != [1, clip.frames]:
                failures.append(f"{key}: frame range {info['frame_range']}, expected [1, {clip.frames}]")
            curves = fcurves(action)
            bad_keys = [fc.data_path for fc in curves if len(fc.keyframe_points) != clip.frames
                        or any(kp.interpolation != "CONSTANT" for kp in fc.keyframe_points)]
            if bad_keys:
                failures.append(f"{key}: {len(bad_keys)} curves without a constant key per frame")
            info["keyframes_sha256"] = keyframe_hash(action)
            if info["keyframes_sha256"] != manifest["clips"][key]["keyframes_sha256"]:
                failures.append(f"{key}: keyframes differ from the build manifest")
            arm.animation_data.action = action
            drift, lows, shell = 0.0, [], 0.0
            for frame in range(1, clip.frames + 1):
                bpy.context.scene.frame_set(frame)
                offset = V.sword_offset(arm)
                drift = max(drift, (offset.translation - rest_offset.translation).length)
                lows.append(PS.lowest_z(feet))
                shell = max(shell, max(SF.shell_residuals(arm, shells).values()))
                if V.check_scale(arm):
                    failures.append(f"{key} frame {frame}: pose scale changed")
            info.update({"sword_drift_m": drift, "lowest_z_range_m": [min(lows), max(lows)],
                         "max_shell_residual_m": shell})
            if drift > 1e-5:
                failures.append(f"{key}: sabre moved relative to the hand")
            if max(abs(z) for z in lows) > 1e-3:
                failures.append(f"{key}: a frame is not grounded ({min(lows):.4f}..{max(lows):.4f} m)")
            if shell > C.SHELL_JUMP_TOLERANCE_M:
                failures.append(f"{key}: unstable shell {shell} m")
            replay[key] = info
        arm.animation_data.action = None
    for pb in arm.pose.bones:
        pb.rotation_quaternion = (1.0, 0.0, 0.0, 0.0)
        pb.location = (0.0, 0.0, 0.0)
    bpy.context.view_layer.update()
    ik = V.check_ik(arm)
    if not ik["ok"]:
        failures.append(f"IK check failed: {ik}")
    report = {"file": path.name, "blender": bpy.app.version_string, "bones": structure["bones"],
              "deform_bones": structure["deform_bones"], "meshes": len(meshes), "vertices": weights["vertices"],
              "max_influences": weights["max_influences"], "actions": sorted(actions), "replay": replay, "ik": ik,
              "failures": failures}
    out = C.OUT_DIR / f"reopen_check_{path.stem}.json"
    out.write_text(json.dumps(report, indent=2, ensure_ascii=False, default=str) + "\n", encoding="utf-8")
    print(f"[reopen] {path.name}: {len(failures)} failure(s) -> {out}")
    for f in failures:
        print(f"[reopen] FAIL {f}")
    if failures:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
