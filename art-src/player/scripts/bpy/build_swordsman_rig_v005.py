"""Entry point of the experimental v005 rig (V1.2-E): v004 -> v005 rig experimental.

Opens the v004 .blend read-only, adds the armature, skins every mesh,
validates, keys the validation poses, saves a NEW file and renders the
review images. Run from the project root, in a separate Blender process
(``--factory-startup`` keeps user add-ons, e.g. the MCP server, out of it):

    blender --background --factory-startup art-src/player/blender/swordsman_base_v004.blend \
        --python art-src/player/scripts/bpy/build_swordsman_rig_v005.py -- [--overwrite] [--no-render]

    # quick look at some poses only (side view, renders/v005/preview/, nothing saved):
    ... --python art-src/player/scripts/bpy/build_swordsman_rig_v005.py -- --preview P03_run_extreme,T03_arm_raised

Outputs: art-src/player/blender/swordsman_rig_v005.blend (rig + pose
actions), swordsman_rig_v005_anim_tests.blend (same + TEST_ actions) and
art-src/player/renders/v005/ (renders + rig_manifest_v005.json, read by
post/rig_sheets.py). Never writes the v004 file; refuses to overwrite v005
outputs without --overwrite.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import struct
import sys
from pathlib import Path

SCRIPTS_DIR = Path(__file__).resolve().parent
PACKAGES = (SCRIPTS_DIR, SCRIPTS_DIR / "swordsman_v003", SCRIPTS_DIR / "swordsman_v004",
            SCRIPTS_DIR / "swordsman_rig_v005")
for entry in PACKAGES:  # last inserted wins: rig, then v004, then v003
    if str(entry) in sys.path:
        sys.path.remove(str(entry))
    sys.path.insert(0, str(entry))

import bpy  # noqa: E402
from mathutils import Vector  # noqa: E402

import animation_tests as A  # noqa: E402
import poses as PS  # noqa: E402
import rig_config as C  # noqa: E402
import rig_render as RR  # noqa: E402
import rig_validate as V  # noqa: E402
from presentation import Shot  # noqa: E402
from render import configure  # noqa: E402
from skeleton import bone_specs, build_armature  # noqa: E402
from skinning import skin  # noqa: E402
from surfaces import use_toon  # noqa: E402

TEST_SHOTS = {
    "shoulder": Shot((0.05, -0.05, 1.3), -35.0, 8.0, 10.0, 1.25),
    "collar": Shot((0.03, 0.0, 1.6), -60.0, 5.0, 10.0, 0.55),
    "torso": Shot((0.0, 0.0, 1.2), -20.0, 5.0, 10.0, 1.3),
    "knees": Shot((0.1, 0.0, 0.6), -90.0, 0.0, 10.0, 1.5),
    "cape": Shot((-0.05, 0.1, 1.3), 140.0, 10.0, 10.0, 1.3),
}
TEST_VIEW = {"T01": "shoulder", "T02": "shoulder", "T03": "shoulder", "T04": "shoulder", "T05": "shoulder",
             "T06": "collar", "T07": "collar", "T08": "collar", "T09": "torso", "T10": "knees", "T11": "cape"}
GAME_POSES = ("P01_neutral", "P02_walk_contact", "P03_run_extreme", "P09_dodge")
COMBAT_POSES = ("P06_basic_attack",)


def parse_args() -> argparse.Namespace:
    argv = sys.argv[sys.argv.index("--") + 1 :] if "--" in sys.argv else []
    parser = argparse.ArgumentParser(prog="build_swordsman_rig_v005.py")
    parser.add_argument("--overwrite", action="store_true")
    parser.add_argument("--no-render", action="store_true")
    parser.add_argument("--preview", help="comma-separated pose keys; side renders only, nothing saved")
    return parser.parse_args(argv)


def check_source() -> None:
    opened = Path(bpy.data.filepath).resolve() if bpy.data.filepath else None
    if opened != C.SOURCE_BLEND.resolve():
        raise SystemExit(f"open {C.SOURCE_BLEND} (got {opened}); see the usage in this file")
    if C.BLEND_PATH.resolve() == opened or C.ANIM_TESTS_BLEND.resolve() == opened:
        raise SystemExit("refusing: the output path equals the source file")


def geometry_hash(objects: list[bpy.types.Object]) -> str:
    depsgraph = bpy.context.evaluated_depsgraph_get()
    h = hashlib.sha256()
    for obj in sorted(objects, key=lambda o: o.name):
        ev = obj.evaluated_get(depsgraph)
        mesh = ev.to_mesh()
        for v in mesh.vertices:
            h.update(struct.pack("3f", *v.co))
        ev.to_mesh_clear()
    return h.hexdigest()


def meshes() -> list[bpy.types.Object]:
    return [o for o in bpy.data.collections[C.SOURCE_COLLECTION].objects if o.type == "MESH"]


def all_poses() -> dict[str, PS.Pose]:
    return {p.key: p for p in (*PS.POSES, *PS.TESTS)}


def first_vertex(name: str) -> Vector:
    depsgraph = bpy.context.evaluated_depsgraph_get()
    ev = bpy.data.objects[name].evaluated_get(depsgraph)
    mesh = ev.to_mesh()
    co = mesh.vertices[0].co.copy()
    ev.to_mesh_clear()
    return co


def sword_vertex_drift(arm: bpy.types.Object, rest: tuple[Vector, ...]) -> float:
    """Distance of blade/grip vertices from where the sword bone says they should be."""
    m = arm.matrix_world @ arm.pose.bones["sword"].matrix
    rest_m = arm.matrix_world @ arm.data.bones["sword"].matrix_local
    worst = 0.0
    for name, ref in zip(("blade", "sword_grip"), rest):
        expected = m @ (rest_m.inverted() @ ref)
        worst = max(worst, (first_vertex(name) - expected).length)
    return worst


def main() -> None:
    args = parse_args()
    check_source()
    out = C.OUT_DIR
    preview = [k for k in (args.preview or "").split(",") if k]
    if not preview and not args.overwrite:
        existing = [p for p in (C.BLEND_PATH, C.ANIM_TESTS_BLEND, out / "rig_manifest_v005.json") if p.exists()]
        if existing:
            raise SystemExit(f"already exists: {existing}; pass --overwrite")
    print(f"[v005] Blender {bpy.app.version_string}, source {bpy.data.filepath}")
    body_all = meshes()
    source_hash = geometry_hash(body_all)
    print(f"[v005] v004 geometry sha256 {source_hash[:16]} (expected {C.SOURCE_GEOMETRY_SHA256_PREFIX})")
    originals = {o.name: [v.co.copy() for v in o.data.vertices] for o in body_all}

    arm = build_armature()
    kinds = skin(body_all, arm)
    configure()
    bpy.context.scene.render.fps = C.TEST_FPS
    body = PS.all_body()
    rest_disp = V.rest_displacement(body_all, originals)
    rest_snap = V.snapshot()
    sword_rest_offset = V.sword_offset(arm)
    ev_rest = tuple(first_vertex(n) for n in ("blade", "sword_grip"))
    print(f"[v005] rest displacement {rest_disp:.2e} m")

    poses = all_poses()
    if preview:
        for key in preview:
            PS.apply(arm, poses[key], body)
            RR.render_review("side", out / "preview" / f"{key}_side.png")
            print(f"[v005] preview {key}: {V.pose_issues(rest_snap, V.snapshot())}")
        return

    structure = V.check_structure(arm, body_all)
    weights = V.check_weights(body_all)
    ik = V.check_ik(arm)
    print(f"[v005] structure {structure}\n[v005] weights {weights}\n[v005] ik {ik}")
    results = {}
    for key, pose in poses.items():
        height = PS.apply(arm, pose, body)
        issues = V.pose_issues(rest_snap, V.snapshot())
        offset = V.sword_offset(arm)
        results[key] = {
            "label": pose.label, "root_height_m": round(height, 4), "issues": issues,
            "max_severity": V.max_severity(issues), "scale_failures": V.check_scale(arm),
            "sword_angle_drift_deg": round(V.angle_between(offset, sword_rest_offset), 4),
            "sword_offset_drift_m": round((offset.translation - sword_rest_offset.translation).length, 6),
            "sword_vertex_drift_m": round(sword_vertex_drift(arm, ev_rest), 6),
        }
        if key.startswith("P"):
            PS.store_action(arm, f"POSE_{key}")
        print(f"[v005] {key}: {results[key]['max_severity']} {[(i['severity'], i['what'], i['value']) for i in issues]}")
    PS.reset(arm)
    bpy.context.view_layer.update()

    manifest: dict[str, object] = {
        "rig": f"{C.RIG_NAME}_{C.RIG_VERSION}", "source": str(C.SOURCE_BLEND.name),
        "source_geometry_sha256": source_hash, "blender": bpy.app.version_string,
        "engine": bpy.context.scene.render.engine,
        "bones": [{"name": s.name, "parent": s.parent, "collection": s.collection, "deform": s.deform}
                  for s in bone_specs()],
        "skinning_rules": kinds, "rest_displacement_m": rest_disp, "structure": structure, "weights": weights,
        "ik": ik, "rest_overlaps": rest_snap["overlaps"], "poses": results,
        "constraints": {pb.name: [(c.type, c.name, round(c.influence, 3)) for c in pb.constraints]
                        for pb in arm.pose.bones if pb.constraints},
    }
    out.mkdir(parents=True, exist_ok=True)
    bpy.ops.wm.save_as_mainfile(filepath=str(C.BLEND_PATH), copy=True)
    print(f"[v005] saved {C.BLEND_PATH}")

    if not args.no_render:
        manifest["renders"] = render_all(arm, body, poses)
    manifest["animation_tests"] = run_animation_tests(arm, body, not args.no_render)
    bpy.ops.wm.save_as_mainfile(filepath=str(C.ANIM_TESTS_BLEND), copy=True)
    print(f"[v005] saved {C.ANIM_TESTS_BLEND}")
    (out / "rig_manifest_v005.json").write_text(json.dumps(manifest, indent=2, ensure_ascii=False, default=str)
                                                + "\n", encoding="utf-8")
    failures = structure["failures"] + weights["failures"]
    if failures or not ik["ok"] or rest_disp > 1e-4:
        raise SystemExit(f"validation failed: {failures}, ik ok={ik['ok']}, rest {rest_disp:.2e}")


def render_all(arm: bpy.types.Object, body: list[bpy.types.Object], poses: dict[str, PS.Pose]) -> dict[str, object]:
    out = C.OUT_DIR
    written: dict[str, object] = {"poses": {}, "tests": {}, "game": {}, "neutral": []}
    for key, pose in poses.items():
        PS.apply(arm, pose, body)
        if key.startswith("P"):
            written["poses"][key] = [str(RR.render_review(v, out / "poses" / f"{key}_{v}.png"))
                                     for v in ("side", "three_quarter")]
        else:
            view = TEST_VIEW[key[:3]]
            written["tests"][key] = str(RR.render_shot(f"test_{view}", TEST_SHOTS[view],
                                                       out / "tests" / f"{key}.png"))
    # Neutral contract masters, same names as v004 (for version_compare).
    PS.apply(arm, poses["P01_neutral"], body)
    from staging import render_master
    configure()
    for view in ("side", "front", "back"):
        written["neutral"].append(str(render_master(view, out / f"{C.RIG_NAME}_{C.RIG_VERSION}_{view}_master.png")))
    # Game scale: locomotion cell and combat cell, shaded and flat; sword N and W.
    for key in (*GAME_POSES, *COMBAT_POSES):
        PS.apply(arm, poses[key], body)
        variants = [("N", False)] + ([("W", True)] if key in COMBAT_POSES else [("W", True)])
        for tag, wide in variants:
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
    # Bone diagram at rest.
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
        RR.set_readable_blade(False)
        structure[view] = {"body": str(body_png), "bones": str(bones_png),
                           "labels": RR.label_positions(arm, view, 900)}
    RR.remove_bone_viz(viz)
    written["structure"] = structure
    configure()
    return written


def run_animation_tests(arm: bpy.types.Object, body: list[bpy.types.Object], render: bool) -> dict[str, object]:
    results: dict[str, object] = {}
    rest_snap = None
    PS.reset(arm)
    bpy.context.view_layer.update()
    rest_snap = V.snapshot()
    for test in A.TESTS:
        action = A.build_action(arm, test, body)
        frames = {}
        for frame in A.RENDER_FRAMES[test]:
            A.play(arm, action, frame)
            issues = V.pose_issues(rest_snap, V.snapshot())
            entry = {"max_severity": V.max_severity(issues), "issues": issues}
            if render:
                entry["image"] = str(RR.render_review("side", C.OUT_DIR / "anim" / f"{test}_f{frame:02d}.png", 384))
            frames[frame] = entry
        arm.animation_data.action = None
        results[test] = {"action": action.name, "frames": frames,
                         "frame_range": [int(action.frame_range[0]), int(action.frame_range[1])]}
        print(f"[v005] animation test {test}: {[(f, e['max_severity']) for f, e in frames.items()]}")
    PS.reset(arm)
    return results


if __name__ == "__main__":
    main()
