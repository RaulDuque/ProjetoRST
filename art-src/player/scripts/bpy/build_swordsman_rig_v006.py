"""Entry point of the experimental v006 rig (V1.2-F): v005 -> v006 (stabilisation).

Opens the v005 .blend read-only, checks it, measures it ("before"), applies
the v006 corrections (``swordsman_rig_v006/fixes.py``), measures again
("after") with the same code, cameras, lights and materials, saves NEW
files and renders the review images. Run from the project root, in a
separate Blender process (``--factory-startup`` keeps user add-ons, e.g. the
MCP server, out of it):

    blender --background --factory-startup art-src/player/blender/swordsman_rig_v005.blend \
        --python art-src/player/scripts/bpy/build_swordsman_rig_v006.py -- [--overwrite] [--no-render]

Outputs: art-src/player/blender/swordsman_rig_v006.blend (rig + POSE_
actions), swordsman_rig_v006_anim_tests.blend (same + TEST_ actions) and
art-src/player/renders/v006/ (renders + rig_manifest_v006.json, read by
post/rig_sheets_v006.py). Never writes a v001-v005 file; refuses to
overwrite v006 outputs without --overwrite.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

SCRIPTS_DIR = Path(__file__).resolve().parent
PACKAGES = (SCRIPTS_DIR, SCRIPTS_DIR / "swordsman_v003", SCRIPTS_DIR / "swordsman_v004",
            SCRIPTS_DIR / "swordsman_rig_v005", SCRIPTS_DIR / "swordsman_rig_v006")
for entry in PACKAGES:  # last inserted wins: v006, then v005 rig, then v004, then v003
    if str(entry) in sys.path:
        sys.path.remove(str(entry))
    sys.path.insert(0, str(entry))

import bpy  # noqa: E402
import numpy as np  # noqa: E402

import animation_tests as A  # noqa: E402  (v005, unchanged)
import fixes as F  # noqa: E402
import poses as PS  # noqa: E402  (v005, unchanged)
import rig_config as C  # noqa: E402  (v006)
import rig_render as RR  # noqa: E402  (v005, unchanged)
import rig_render_v006 as R6  # noqa: E402
import rig_validate as V  # noqa: E402  (v005, unchanged)
import validate_v006 as V6  # noqa: E402
from render import configure  # noqa: E402
from staging import SIDE_AZIMUTH, render_master, turn_rig  # noqa: E402
from surfaces import use_toon  # noqa: E402

# Before/after images: pose (or animation frame) -> views. Cape problems are
# shown from the far side, where the cape is (the game side view hides it).
COMPARE = (
    ("P05_jump_fall", ("far_back", "close_cape", "side")),
    ("P07_crescent_slash", ("far_back", "close_cape", "side")),
    ("P11_hurt", ("far_back", "close_cape")),
    ("P09_dodge", ("side", "three_quarter", "far_back", "close_coat_side")),
    ("P13_death", ("side", "three_quarter", "close_coat_3q", "close_ankle")),
    ("T06_head_up", ("close_collar", "close_collar_back")),
    ("T08_head_turn", ("close_collar", "close_collar_back")),
    ("T10_squat", ("side", "three_quarter", "close_coat_3q")),
    ("T11_cape_arm_up", ("far_back", "close_cape", "close_cape_front")),
)
COMPARE_ANIM = {"attack": ((2, "far_back"), (3, "far_back")), "dodge": ((5, "far_back"), (5, "side"))}
TEST_VIEW = {"T01": "test_shoulder", "T02": "test_shoulder", "T03": "test_shoulder", "T04": "test_shoulder",
             "T05": "test_shoulder", "T06": "close_collar", "T07": "close_collar", "T08": "close_collar",
             "T09": "test_torso", "T10": "test_knees", "T11": "test_cape"}
GAME_POSES = ("P01_neutral", "P02_walk_contact", "P03_run_extreme", "P09_dodge")
COMBAT_POSES = ("P06_basic_attack",)
EXPECTED_CHANGED = ("coat", "half_cape", "cape_clasp", "collar")  # + the PRESERVE_VOLUME_EXTRA meshes


def parse_args() -> argparse.Namespace:
    argv = sys.argv[sys.argv.index("--") + 1 :] if "--" in sys.argv else []
    parser = argparse.ArgumentParser(prog="build_swordsman_rig_v006.py")
    parser.add_argument("--overwrite", action="store_true")
    parser.add_argument("--no-render", action="store_true")
    return parser.parse_args(argv)


def check_paths() -> None:
    opened = Path(bpy.data.filepath).resolve() if bpy.data.filepath else None
    if opened != C.SOURCE_BLEND.resolve():
        raise SystemExit(f"open {C.SOURCE_BLEND} (got {opened}); see the usage in this file")
    protected = {p.resolve() for p in C.PROTECTED_BLENDS}
    for out in (C.BLEND_PATH, C.ANIM_TESTS_BLEND):
        if out.resolve() in protected or out.resolve() == opened:
            raise SystemExit(f"refusing: {out} is a protected earlier version")


def meshes() -> list[bpy.types.Object]:
    return [o for o in bpy.data.collections[C.SOURCE_COLLECTION].objects if o.type == "MESH"]


def check_source(body_all: list[bpy.types.Object]) -> dict[str, object]:
    arm = bpy.data.objects.get(C.SOURCE_ARMATURE)
    if arm is None or arm.type != "ARMATURE":
        raise SystemExit(f"source armature {C.SOURCE_ARMATURE} missing")
    bones = len(arm.data.bones)
    deform = sum(b.use_deform for b in arm.data.bones)
    structure = V.check_structure(arm, body_all)
    weights = V.check_weights(body_all)
    geo = V6.geometry_hash_without_rig(body_all)
    problems = list(structure["failures"]) + list(weights["failures"])
    if (bones, deform) != (C.SOURCE_BONE_COUNT, C.SOURCE_DEFORM_COUNT):
        problems.append(f"{bones} bones / {deform} deform, expected {C.SOURCE_BONE_COUNT}/{C.SOURCE_DEFORM_COUNT}")
    if not geo.startswith(C.SOURCE_GEOMETRY_SHA256_PREFIX):
        problems.append(f"v004 geometry hash {geo[:16]} != {C.SOURCE_GEOMETRY_SHA256_PREFIX}")
    if len(body_all) != 132:
        problems.append(f"{len(body_all)} meshes, expected 132")
    if problems:
        raise SystemExit(f"source v005 is not the validated V1.2-E file: {problems}")
    return {"armature": arm.name, "bones": bones, "deform_bones": deform, "meshes": len(body_all),
            "geometry_sha256_without_rig": geo, "weights_sha256": V6.weight_hash(body_all),
            "pose_actions": sorted(a.name for a in bpy.data.actions if a.name.startswith("POSE_"))}


def all_poses() -> dict[str, PS.Pose]:
    return {p.key: p for p in (*PS.POSES, *PS.TESTS)}


def first_vertex(name: str):
    depsgraph = bpy.context.evaluated_depsgraph_get()
    ev = bpy.data.objects[name].evaluated_get(depsgraph)
    mesh = ev.to_mesh()
    co = mesh.vertices[0].co.copy()
    ev.to_mesh_clear()
    return co


def root_relative(arrays: dict[str, np.ndarray], arm: bpy.types.Object) -> dict[str, np.ndarray]:
    """Vertex arrays relative to the root bone, so a different grounding height is not a change."""
    root = np.array(arm.matrix_world @ arm.pose.bones["root"].head, dtype=np.float32)
    return {n: (a.reshape(-1, 3) - root).ravel() for n, a in arrays.items()}


class Measure:
    """Same measurement for v005 (before) and v006 (after)."""

    def __init__(self, arm: bpy.types.Object, body: list[bpy.types.Object], body_all: list[bpy.types.Object]):
        self.arm, self.body, self.body_all = arm, body, body_all
        PS.reset(arm)
        bpy.context.view_layer.update()
        self.rest = V.snapshot()
        self.rest_extra = V6.extra_snapshot()
        self.sword_rest = V.sword_offset(arm)
        self.blade_rest = tuple(first_vertex(n) for n in ("blade", "sword_grip"))

    def issues(self) -> tuple[list[dict[str, object]], list[dict[str, object]]]:
        return V.pose_issues(self.rest, V.snapshot()), V6.extra_issues(self.rest_extra, V6.extra_snapshot())

    def sword_vertex_drift(self) -> float:
        m = self.arm.matrix_world @ self.arm.pose.bones["sword"].matrix
        rest_m = self.arm.matrix_world @ self.arm.data.bones["sword"].matrix_local
        worst = 0.0
        for name, ref in zip(("blade", "sword_grip"), self.blade_rest):
            worst = max(worst, (first_vertex(name) - m @ (rest_m.inverted() @ ref)).length)
        return worst

    def poses(self, store_actions: bool) -> tuple[dict[str, object], dict[str, dict[str, np.ndarray]]]:
        results, arrays = {}, {}
        for key, pose in all_poses().items():
            height = PS.apply(self.arm, pose, self.body)
            base, extra = self.issues()
            offset = V.sword_offset(self.arm)
            results[key] = {
                "label": pose.label, "root_height_m": round(height, 4), "issues": base, "extra_issues": extra,
                "max_severity_v005_pairs": V.max_severity(base), "max_severity": V.max_severity(base + extra),
                "scale_failures": V.check_scale(self.arm),
                "sword_angle_drift_deg": round(V.angle_between(offset, self.sword_rest), 4),
                "sword_offset_drift_m": round((offset.translation - self.sword_rest.translation).length, 6),
                "sword_vertex_drift_m": round(self.sword_vertex_drift(), 6),
            }
            arrays[key] = {
                "rig": root_relative(V6.evaluated_arrays(self.body_all, armature_only=True), self.arm),
                "final": root_relative(V6.evaluated_arrays(self.body_all), self.arm),
            }
            if store_actions and key.startswith("P"):
                PS.store_action(self.arm, f"POSE_{key}")
        PS.reset(self.arm)
        bpy.context.view_layer.update()
        return results, arrays

    def animations(self, renders: dict[str, tuple[tuple[int, str], ...]], tag: str,
                   keep: bool) -> dict[str, object]:
        results: dict[str, object] = {}
        for test in A.TESTS:
            action = A.build_action(self.arm, test, self.body)
            frames = {}
            wanted = renders.get(test, ())
            for frame in sorted(set(A.RENDER_FRAMES[test]) | {f for f, _ in wanted}):
                A.play(self.arm, action, frame)
                base, extra = self.issues()
                entry: dict[str, object] = {"max_severity": V.max_severity(base + extra), "issues": base + extra,
                                            "images": {}}
                for f, view in wanted:
                    if f == frame:
                        path = C.OUT_DIR / "anim" / tag / f"{test}_f{frame:02d}_{view}.png"
                        entry["images"][view] = str(R6.render_view(view, path, 384 if tag == "v006" else 512))
                frames[frame] = entry
            self.arm.animation_data.action = None
            results[test] = {"action": action.name, "frames": frames,
                             "frame_range": [int(action.frame_range[0]), int(action.frame_range[1])]}
            if not keep:
                bpy.data.actions.remove(action)
            print(f"[v006] {tag} animation {test}: {[(f, e['max_severity']) for f, e in frames.items()]}")
        PS.reset(self.arm)
        bpy.context.view_layer.update()
        return results


def render_compare(arm: bpy.types.Object, body: list[bpy.types.Object], tag: str) -> dict[str, dict[str, str]]:
    poses = all_poses()
    written: dict[str, dict[str, str]] = {}
    for key, views in COMPARE:
        PS.apply(arm, poses[key], body)
        written[key] = {v: str(R6.render_view(v, C.OUT_DIR / "compare" / tag / f"{key}_{v}.png")) for v in views}
    PS.reset(arm)
    bpy.context.view_layer.update()
    return written


def compare_with_v005_manifest(before: dict[str, object]) -> dict[str, object]:
    """The 'before' numbers must reproduce the V1.2-E manifest (same file, same code)."""
    path = C.V005.OUT_DIR / "rig_manifest_v005.json"
    if not path.exists():
        return {"available": False}
    old = json.loads(path.read_text(encoding="utf-8"))["poses"]
    mismatches = []
    for key, entry in before.items():
        a = [(i["what"], i["value"]) for i in old.get(key, {}).get("issues", [])]
        b = [(i["what"], i["value"]) for i in entry["issues"]]
        if a != b:
            mismatches.append(key)
    return {"available": True, "mismatches": mismatches}


def main() -> None:
    args = parse_args()
    check_paths()
    out = C.OUT_DIR
    if not args.overwrite:
        existing = [p for p in (C.BLEND_PATH, C.ANIM_TESTS_BLEND, out / "rig_manifest_v006.json") if p.exists()]
        if existing:
            raise SystemExit(f"already exists: {existing}; pass --overwrite")
    print(f"[v006] Blender {bpy.app.version_string}, source {bpy.data.filepath}")
    body_all = meshes()
    source = check_source(body_all)
    print(f"[v006] source ok: {source['bones']} bones, geometry {source['geometry_sha256_without_rig'][:16]}")
    originals = {o.name: [v.co.copy() for v in o.data.vertices] for o in body_all}
    arm = F.rename_armature()
    configure()
    bpy.context.scene.render.fps = C.TEST_FPS
    body = PS.all_body()

    # --- before: the v005 rig as saved by V1.2-E -----------------------------------
    frames_before = V6.bone_frames(arm)
    m5 = Measure(arm, body, body_all)
    before, arrays_before = m5.poses(store_actions=False)
    anim_before = m5.animations(COMPARE_ANIM if not args.no_render else {}, "v005", keep=False)
    images_before = render_compare(arm, body, "v005") if not args.no_render else {}
    print("[v006] before: " + ", ".join(f"{k}={v['max_severity']}" for k, v in before.items()))

    # --- corrections ------------------------------------------------------------------
    for action in [a for a in bpy.data.actions if a.name.startswith("POSE_")]:
        bpy.data.actions.remove(action)  # v005 keys; re-keyed below on the v006 rig
    fixes = F.apply_all(arm)
    pivots = V6.pivot_changes(frames_before, V6.bone_frames(arm))
    rest_disp = V.rest_displacement(body_all, originals)
    structure = V.check_structure(arm, body_all)
    weights = V.check_weights(body_all)
    ik = V.check_ik(arm)
    m6 = Measure(arm, body, body_all)
    after, arrays_after = m6.poses(store_actions=True)
    # Rig stage (Armature modifier output): what the rig changed. Final stage
    # (after Solidify/Subdivision): reported too, it shows the pauldron shell
    # instability documented in validate_v006.evaluated_arrays.
    changed = {key: V6.changed_meshes(arrays_before[key]["rig"], arrays_after[key]["rig"]) for key in after}
    changed_final = {key: V6.changed_meshes(arrays_before[key]["final"], arrays_after[key]["final"])
                     for key in after}
    allowed = set(EXPECTED_CHANGED) | {o.name for o in body_all if o.name.startswith(C.PRESERVE_VOLUME_EXTRA)}
    unexpected = {key: sorted(set(c) - allowed) for key, c in changed.items() if set(c) - allowed}
    unexpected_final = {key: {n: v for n, v in c.items() if n not in allowed} for key, c in changed_final.items()
                        if set(c) - allowed}
    del arrays_before, arrays_after
    print(f"[v006] structure {structure}\n[v006] weights {weights}\n[v006] ik {ik}\n[v006] pivots {pivots}")
    print("[v006] after: " + ", ".join(f"{k}={v['max_severity']}" for k, v in after.items()))
    images_after = render_compare(arm, body, "v006") if not args.no_render else {}

    manifest: dict[str, object] = {
        "rig": f"{C.RIG_NAME}_{C.RIG_VERSION}", "source": C.SOURCE_BLEND.name, "source_check": source,
        "blender": bpy.app.version_string, "engine": bpy.context.scene.render.engine, "test_fps": C.TEST_FPS,
        "bones": [{"name": b.name, "parent": b.parent.name if b.parent else None,
                   "collection": b.collections[0].name if b.collections else None, "deform": b.use_deform}
                  for b in arm.data.bones],
        "fixes": fixes, "pivots": pivots, "rest_displacement_m": rest_disp, "structure": structure,
        "weights": weights, "weights_sha256": V6.weight_hash(body_all), "ik": ik,
        "rest_overlaps": m6.rest["overlaps"], "rest_overlaps_v005": m5.rest["overlaps"],
        "rest_extra_overlaps": m6.rest_extra, "poses": after, "poses_v005": before,
        "v005_manifest_reproduced": compare_with_v005_manifest(before),
        "changed_meshes_per_pose": changed, "unexpected_changed_meshes": unexpected,
        "changed_meshes_final_shell": changed_final, "unexpected_changed_final_shell": unexpected_final,
        "animation_tests_v005": anim_before, "compare_images": {"v005": images_before, "v006": images_after},
        "constraints": {pb.name: [(c.type, c.name, round(c.influence, 3)) for c in pb.constraints]
                        for pb in arm.pose.bones if pb.constraints},
        "preserve_volume": sorted(o.name for o in body_all if o.modifiers[0].use_deform_preserve_volume),
    }
    R6.remove_review_cameras()
    turn_rig(SIDE_AZIMUTH)
    out.mkdir(parents=True, exist_ok=True)
    bpy.ops.wm.save_as_mainfile(filepath=str(C.BLEND_PATH), copy=True)
    print(f"[v006] saved {C.BLEND_PATH}")

    if not args.no_render:
        manifest["renders"] = render_all(arm, body)
    anim_renders = {} if args.no_render else {t: tuple((f, v) for f in A.RENDER_FRAMES[t] for v in ("side", "far_back"))
                                              for t in A.TESTS}
    manifest["animation_tests"] = m6.animations(anim_renders, "v006", keep=True)
    R6.remove_review_cameras()
    turn_rig(SIDE_AZIMUTH)
    bpy.ops.wm.save_as_mainfile(filepath=str(C.ANIM_TESTS_BLEND), copy=True)
    print(f"[v006] saved {C.ANIM_TESTS_BLEND}")
    (out / "rig_manifest_v006.json").write_text(json.dumps(manifest, indent=2, ensure_ascii=False, default=str)
                                                + "\n", encoding="utf-8")
    failures = list(structure["failures"]) + list(weights["failures"])
    if unexpected:
        failures.append(f"meshes outside the corrected areas changed: {unexpected}")
    if pivots["removed"] or set(pivots["moved"]) - {"cape_arm.L_01", "cape_arm.L_02"}:
        failures.append(f"unexpected pivot changes: {pivots}")
    for failure in failures:
        print(f"[v006] FAIL {failure}")
    if failures or not ik["ok"] or rest_disp > 1e-4:
        raise SystemExit(f"validation failed: {failures}, ik ok={ik['ok']}, rest {rest_disp:.2e}")


def render_all(arm: bpy.types.Object, body: list[bpy.types.Object]) -> dict[str, object]:
    out = C.OUT_DIR
    written: dict[str, object] = {"poses": {}, "tests": {}, "game": {}, "neutral": []}
    for key, pose in all_poses().items():
        PS.apply(arm, pose, body)
        if key.startswith("P"):
            written["poses"][key] = {v: str(R6.render_view(v, out / "poses" / f"{key}_{v}.png"))
                                     for v in ("side", "three_quarter", "far_back")}
        else:
            view = TEST_VIEW[key[:3]]
            written["tests"][key] = str(R6.render_view(view, out / "tests" / f"{key}.png"))
    # Neutral contract masters, same names as v004/v005 (for version_compare and the sheets).
    PS.apply(arm, all_poses()["P01_neutral"], body)
    configure()
    for view in ("side", "front", "back"):
        written["neutral"].append(str(render_master(view, out / f"{C.RIG_NAME}_{C.RIG_VERSION}_{view}_master.png")))
    # Game scale: locomotion cell and combat cell, shaded and flat; sabre N and W (as v005).
    for key in (*GAME_POSES, *COMBAT_POSES):
        PS.apply(arm, all_poses()[key], body)
        for tag, wide in (("N", False), ("W", True)):
            RR.set_readable_blade(wide)
            for finish, toon in (("shaded", False), ("flat", True)):
                use_toon(toon)
                path = out / "game" / f"{key}_{tag}_{finish}_master.png"
                if key in COMBAT_POSES:
                    RR.render_combat_side(path)
                else:
                    RR.render_contract_side(path)
                written["game"].setdefault(key, []).append(str(path))
            use_toon(False)
        RR.set_readable_blade(False)
    # Bone diagram at rest (v005 method).
    PS.reset(arm)
    bpy.context.view_layer.update()
    viz = RR.build_bone_viz(arm)
    structure = {}
    for view in ("side", "front"):
        RR.set_visible(viz, False)
        use_toon(True)
        body_png = RR.render_review(view, out / "structure" / f"body_{view}.png", 900)
        use_toon(False)
        for obj in body:
            obj.hide_render = True
        RR.set_visible(viz, True)
        bones_png = RR.render_review(view, out / "structure" / f"bones_{view}.png", 900)
        for obj in body:
            obj.hide_render = False
        structure[view] = {"body": str(body_png), "bones": str(bones_png),
                           "labels": RR.label_positions(arm, view, 900)}
    RR.remove_bone_viz(viz)
    written["structure"] = structure
    configure()
    return written


if __name__ == "__main__":
    main()
