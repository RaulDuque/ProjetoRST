"""Entry point of v007 (V1.2-G): v006 + stable Solidify shells, rig unchanged.

Opens the v006 .blend read-only, checks it, measures the Solidify shells
("before"), merges the collapsed pole of the two pauldron plates and turns
off the even thickness of the coat and half-cape shells
(``swordsman_rig_v007/shell_fix.py``), measures again with the same code,
cameras and light, and saves a NEW file. The 43 bones, constraints,
weights and pose actions of v006 are kept. Run from the project root:

    blender --background --factory-startup art-src/player/blender/swordsman_rig_v006.blend \
        --python art-src/player/scripts/bpy/build_swordsman_rig_v007.py -- [--overwrite] [--no-render]

Outputs: art-src/player/blender/swordsman_rig_v007.blend and
art-src/player/renders/v007/ (shoulder before/after renders,
rig_manifest_v007.json). Never writes a v001-v006 file.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

SCRIPTS_DIR = Path(__file__).resolve().parent
PACKAGES = (SCRIPTS_DIR, SCRIPTS_DIR / "swordsman_v003", SCRIPTS_DIR / "swordsman_v004",
            SCRIPTS_DIR / "swordsman_rig_v005", SCRIPTS_DIR / "swordsman_rig_v006",
            SCRIPTS_DIR / "swordsman_rig_v007")
for entry in PACKAGES:  # last inserted wins: v007, v006, v005 rig, v004, v003
    if str(entry) in sys.path:
        sys.path.remove(str(entry))
    sys.path.insert(0, str(entry))

import bpy  # noqa: E402
import numpy as np  # noqa: E402
from mathutils.kdtree import KDTree  # noqa: E402

import poses as PS  # noqa: E402  (v005, unchanged)
import rig_config as C  # noqa: E402  (v007)
import rig_validate as V  # noqa: E402  (v005, unchanged)
import shell_fix as SF  # noqa: E402
import validate_v006 as V6  # noqa: E402  (v006, unchanged)
import views_v007 as VW  # noqa: E402
from render import configure  # noqa: E402
from staging import SIDE_AZIMUTH, render_master, turn_rig  # noqa: E402

SHOULDER_POSES = ("P01_neutral", "P02_walk_contact", "P03_run_extreme", "P06_basic_attack", "P09_dodge",
                  "T03_arm_raised", "T09_torso_twist", "P12_magic_cast", "P13_death")
SHOULDER_VIEWS = ("close_pauldron", "close_pauldron_back")


def parse_args() -> argparse.Namespace:
    argv = sys.argv[sys.argv.index("--") + 1 :] if "--" in sys.argv else []
    parser = argparse.ArgumentParser(prog="build_swordsman_rig_v007.py")
    parser.add_argument("--overwrite", action="store_true")
    parser.add_argument("--no-render", action="store_true")
    return parser.parse_args(argv)


def check_paths() -> None:
    opened = Path(bpy.data.filepath).resolve() if bpy.data.filepath else None
    if opened != C.SOURCE_BLEND.resolve():
        raise SystemExit(f"open {C.SOURCE_BLEND} (got {opened}); see the usage in this file")
    protected = {p.resolve() for p in C.PROTECTED_BLENDS}
    if C.BLEND_PATH.resolve() in protected or C.BLEND_PATH.resolve() == opened:
        raise SystemExit(f"refusing: {C.BLEND_PATH} is a protected earlier version")


def meshes() -> list[bpy.types.Object]:
    return [o for o in bpy.data.collections[C.SOURCE_COLLECTION].objects if o.type == "MESH"]


def check_source(body_all: list[bpy.types.Object]) -> dict[str, object]:
    arm = bpy.data.objects.get(C.SOURCE_ARMATURE)
    if arm is None or arm.type != "ARMATURE":
        raise SystemExit(f"source armature {C.SOURCE_ARMATURE} missing")
    structure = V.check_structure(arm, body_all)
    weights = V.check_weights(body_all)
    geo = V6.geometry_hash_without_rig(body_all)
    w_hash = V6.weight_hash(body_all)
    problems = list(structure["failures"]) + list(weights["failures"])
    if (structure["bones"], structure["deform_bones"]) != (C.SOURCE_BONE_COUNT, C.SOURCE_DEFORM_COUNT):
        problems.append(f"{structure['bones']}/{structure['deform_bones']} bones")
    if not geo.startswith(C.SOURCE_GEOMETRY_SHA256_PREFIX):
        problems.append(f"v004 geometry hash {geo[:16]}")
    if not w_hash.startswith(C.SOURCE_WEIGHTS_SHA256_PREFIX):
        problems.append(f"v006 weight hash {w_hash[:16]}")
    if problems:
        raise SystemExit(f"source v006 is not the validated V1.2-F file: {problems}")
    return {"armature": arm.name, "bones": structure["bones"], "deform_bones": structure["deform_bones"],
            "meshes": len(body_all), "geometry_sha256_without_rig": geo, "weights_sha256": w_hash,
            "pose_actions": sorted(a.name for a in bpy.data.actions if a.name.startswith("POSE_"))}


def all_poses() -> dict[str, PS.Pose]:
    return {p.key: p for p in (*PS.POSES, *PS.TESTS)}


def measure(arm: bpy.types.Object, body: list[bpy.types.Object], shells: list[bpy.types.Object]) -> dict:
    """Shell residuals and the v005/v006 deformation checks, every pose and test."""
    PS.reset(arm)
    bpy.context.view_layer.update()
    rest, rest_extra = V.snapshot(), V6.extra_snapshot()
    out = {}
    for key, pose in all_poses().items():
        PS.apply(arm, pose, body)
        issues = V.pose_issues(rest, V.snapshot()) + V6.extra_issues(rest_extra, V6.extra_snapshot())
        out[key] = {"shell_residual_m": SF.shell_residuals(arm, shells), "issues": issues,
                    "max_severity": V.max_severity(issues)}
    PS.reset(arm)
    bpy.context.view_layer.update()
    return out


def dqs_trigger(arm: bpy.types.Object, body: list[bpy.types.Object]) -> dict[str, float]:
    """The V1.2-F reproduction: toggling dual quaternion on an unrelated mesh (a sole)."""
    sole = bpy.data.objects["boot_sole_near"].modifiers[0]
    plates = [bpy.data.objects[n] for n in C.PAULDRON_OBJECTS]
    out = {}
    for key in ("P12_magic_cast", "P13_death"):
        PS.apply(arm, all_poses()[key], body)
        a = SF.final_arrays(plates)
        sole.use_deform_preserve_volume = not sole.use_deform_preserve_volume
        b = SF.final_arrays(plates)
        sole.use_deform_preserve_volume = not sole.use_deform_preserve_volume
        out[key] = round(max(float(np.abs(a[n] - b[n]).max()) for n in a), 7)
    PS.reset(arm)
    return out


def render_shoulder(arm: bpy.types.Object, body: list[bpy.types.Object], tag: str) -> dict[str, dict[str, str]]:
    written = {}
    for key in SHOULDER_POSES:
        PS.apply(arm, all_poses()[key], body)
        written[key] = {v: str(VW.render_view(v, C.OUT_DIR / "shoulder" / tag / f"{key}_{v}.png"))
                        for v in SHOULDER_VIEWS}
    PS.reset(arm)
    return written


def rest_shell_change(before: dict[str, np.ndarray], after: dict[str, np.ndarray]) -> dict[str, float]:
    """Max distance (m) from each final shell vertex to the nearest v006 vertex, at rest."""
    out = {}
    for name in before:
        tree = KDTree(len(before[name]))
        for i, co in enumerate(before[name]):
            tree.insert(co, i)
        tree.balance()
        out[name] = round(max(tree.find(co)[2] for co in after[name]), 6)
    return out


def main() -> None:
    args = parse_args()
    check_paths()
    out = C.OUT_DIR
    if not args.overwrite:
        existing = [p for p in (C.BLEND_PATH, out / "rig_manifest_v007.json") if p.exists()]
        if existing:
            raise SystemExit(f"already exists: {existing}; pass --overwrite")
    print(f"[v007] Blender {bpy.app.version_string}, source {bpy.data.filepath}")
    body_all = meshes()
    source = check_source(body_all)
    degenerate_before = SF.scan_degenerate(body_all)
    print(f"[v007] source ok: {source['bones']} bones; degenerate meshes {sorted(degenerate_before)}")
    arm = bpy.data.objects[C.SOURCE_ARMATURE]
    arm.name = arm.data.name = C.ARMATURE_NAME
    configure()
    body = PS.all_body()
    shells = SF.solidify_objects(body_all)
    plates = [bpy.data.objects[n] for n in C.PAULDRON_OBJECTS]
    frames_before = V6.bone_frames(arm)
    others = [o for o in body_all if o.name not in C.PAULDRON_OBJECTS]

    PS.reset(arm)
    bpy.context.view_layer.update()
    cloth = [bpy.data.objects[n] for n in C.EVEN_OFFSET_OFF]
    rest_before = SF.final_arrays(plates + cloth)
    before = measure(arm, body, shells)
    trigger_before = dqs_trigger(arm, body)
    rig_before = {}
    for key, pose in all_poses().items():
        PS.apply(arm, pose, body)
        rig_before[key] = V6.evaluated_arrays(others, armature_only=True)
    images_before = render_shoulder(arm, body, "v006") if not args.no_render else {}
    print("[v007] before: " + ", ".join(f"{k}={max(v['shell_residual_m'].values()):.2e}" for k, v in before.items()))

    fix = {"pauldron": SF.fix_pauldron(), "cloth_shells": SF.fix_cloth_shells()}
    PS.reset(arm)
    bpy.context.view_layer.update()
    rest_after = SF.final_arrays(plates + cloth)
    after = measure(arm, body, shells)
    trigger_after = dqs_trigger(arm, body)
    others_changed = {}
    for key, pose in all_poses().items():
        PS.apply(arm, pose, body)
        changed = V6.changed_meshes(rig_before[key], V6.evaluated_arrays(others, armature_only=True))
        if changed:
            others_changed[key] = changed
    del rig_before
    images_after = render_shoulder(arm, body, "v007") if not args.no_render else {}
    masters = []
    if not args.no_render:  # neutral contract masters, compared with the v006 ones (rest look)
        PS.reset(arm)
        configure()
        masters = [str(render_master(v, C.OUT_DIR / f"{C.RIG_NAME}_{C.RIG_VERSION}_{v}_master.png"))
                   for v in ("side", "front", "back")]
    structure = V.check_structure(arm, body_all)
    weights = V.check_weights(body_all)
    ik = V.check_ik(arm)
    pivots = V6.pivot_changes(frames_before, V6.bone_frames(arm))
    degenerate_after = SF.scan_degenerate(body_all)
    print("[v007] after: " + ", ".join(f"{k}={max(v['shell_residual_m'].values()):.2e}" for k, v in after.items()))

    unstable_after = {k: {n: r for n, r in v["shell_residual_m"].items() if r > C.SHELL_JUMP_TOLERANCE_M}
                      for k, v in after.items()}
    unstable_after = {k: v for k, v in unstable_after.items() if v}
    manifest = {
        "model": f"{C.RIG_NAME}_{C.RIG_VERSION}", "source": C.SOURCE_BLEND.name, "source_check": source,
        "blender": bpy.app.version_string, "fix": fix, "degenerate_before": degenerate_before,
        "degenerate_after": degenerate_after, "rest_shell_change_m": rest_shell_change(rest_before, rest_after),
        "shell_tolerance_m": C.SHELL_JUMP_TOLERANCE_M, "poses_v006": before, "poses": after,
        "dqs_trigger_jump_m": {"v006": trigger_before, "v007": trigger_after},
        "unstable_shells_after": unstable_after, "other_meshes_changed": others_changed,
        "structure": structure, "weights": weights, "weights_sha256": V6.weight_hash(body_all), "ik": ik,
        "pivots": pivots, "geometry_sha256_without_rig": V6.geometry_hash_without_rig(body_all),
        "shoulder_images": {"v006": images_before, "v007": images_after}, "neutral_masters": masters,
    }
    VW.remove_review_cameras()
    turn_rig(SIDE_AZIMUTH)
    PS.reset(arm)
    bpy.context.view_layer.update()
    out.mkdir(parents=True, exist_ok=True)
    bpy.ops.wm.save_as_mainfile(filepath=str(C.BLEND_PATH), copy=True)
    print(f"[v007] saved {C.BLEND_PATH}")
    (out / "rig_manifest_v007.json").write_text(json.dumps(manifest, indent=2, ensure_ascii=False, default=str)
                                                + "\n", encoding="utf-8")
    failures = list(structure["failures"]) + list(weights["failures"])
    if pivots["moved"] or pivots["added"] or pivots["removed"]:
        failures.append(f"the rig changed: {pivots}")
    if others_changed:
        failures.append(f"meshes other than the pauldron changed: {others_changed}")
    if unstable_after:
        failures.append(f"shells still unstable: {unstable_after}")
    if not ik["ok"]:
        failures.append("IK check failed")
    for failure in failures:
        print(f"[v007] FAIL {failure}")
    if failures:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
