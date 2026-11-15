"""V2.2: weaponless body masters and per-frame equipment sockets.

The published V1.3-V1.5 masters were rendered with the sabre attached to the
right hand, so every equipped weapon showed the starting sword. This script
re-renders the **same poses** (the clip modules of V1.3, V1.4 and V1.5 are
reused unchanged; the action keyframe hashes are compared with the earlier
build manifests) with every sabre object hidden, and measures, frame by
frame, where the equipment sockets land in the game cell:

- ``weapon``: grip centre of the sabre, blade direction (pommel -> guard)
  and whether the hand is in front of the chest (towards the camera);
- ``hand_l``: the left hand (shield / off-hand);
- ``head``, ``chest`` and ``back`` (cape collar).

Socket positions are in world units (1 u = 1 contract pixel) relative to the
sole-line pivot of the clip's cell, x towards where the character faces and
y up; angles are degrees counter-clockwise from +x.

Opens swordsman_rig_v007.blend read-only and never saves a .blend. Run from
the project root, once per clip module:

    blender --background --factory-startup art-src/player/blender/swordsman_rig_v007.blend \
        --python art-src/player/scripts/bpy/build_game_sprites_v2_2.py -- \
        --module v1_3|v1_4|v1_5 [--clips a,b] [--limit N] [--sockets-only] [--overwrite]

Outputs (``renders/`` is ignored by Git): art-src/player/renders/v2_2_weaponless/<module>/
(masters/, game_sprites_manifest.json with the original clip metadata, the
new render paths and the sockets). post/game_sprites_v2_2.py stages the
candidate atlases from them; only publish_guard.py writes ``assets/``.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

SCRIPTS_DIR = Path(__file__).resolve().parent
MODULES = {
    "v1_3": ("game_sprites_v1_3", "clips", "v1_3_game_sprites", ("idle", "run", "attack_basic", "dodge")),
    "v1_4": ("game_sprites_v1_4", "clips_v1_4", "v1_4_game_sprites", None),
    "v1_5": ("game_sprites_v1_5", "clips_v1_5", "v1_5_game_sprites", ("hurt_a", "fall_v15", "attack_basic_air")),
}


def parse_args() -> argparse.Namespace:
    argv = sys.argv[sys.argv.index("--") + 1 :] if "--" in sys.argv else []
    parser = argparse.ArgumentParser(prog="build_game_sprites_v2_2.py")
    parser.add_argument("--module", choices=sorted(MODULES), required=True)
    parser.add_argument("--clips", default="", help="comma-separated subset")
    parser.add_argument("--limit", type=int, default=0, help="frames per clip (timing tests)")
    parser.add_argument("--sockets-only", action="store_true", help="measure the sockets, render nothing")
    parser.add_argument("--overwrite", action="store_true")
    return parser.parse_args(argv)


ARGS = parse_args()
PACKAGE = MODULES[ARGS.module][0]
for entry in (SCRIPTS_DIR, SCRIPTS_DIR / "swordsman_v003", SCRIPTS_DIR / "swordsman_v004",
              SCRIPTS_DIR / "swordsman_rig_v005", SCRIPTS_DIR / "swordsman_rig_v006",
              SCRIPTS_DIR / "swordsman_rig_v007", SCRIPTS_DIR / PACKAGE):
    if str(entry) in sys.path:
        sys.path.remove(str(entry))
    sys.path.insert(0, str(entry))

import importlib  # noqa: E402

import bpy  # noqa: E402
from bpy_extras.object_utils import world_to_camera_view  # noqa: E402
from mathutils import Vector  # noqa: E402

import build_animations_v007 as BA  # noqa: E402  (V1.2-G helpers, unchanged)
import poses as PS  # noqa: E402
import rig_config as C  # noqa: E402
import rig_render as RR  # noqa: E402
from camera import use_camera  # noqa: E402
from contract import COMBAT_FRAME, COMBAT_PIVOT  # noqa: E402
from render import configure, render_still  # noqa: E402
from staging import turn_rig  # noqa: E402
from surfaces import use_toon  # noqa: E402

CL = importlib.import_module(MODULES[ARGS.module][1])

PLAYER_DIR = SCRIPTS_DIR.parents[1]
OUT_ROOT = PLAYER_DIR / "renders" / "v2_2_weaponless"
ORIGINAL = PLAYER_DIR / "renders" / MODULES[ARGS.module][2] / "game_sprites_manifest.json"
SWORD_OBJECTS = ("blade", "blade_readable", "sword_collar", "sword_guard", "sword_grip", "sword_pommel")
LOCOMOTION_CELL, LOCOMOTION_PIVOT = (64, 64), (32.0, 63.0)


def hide_sword() -> dict[str, bool]:
    """Hides every sabre object from the render; returns the earlier flags."""
    earlier = {}
    for name in SWORD_OBJECTS:
        obj = bpy.data.objects[name]
        earlier[name] = obj.hide_render
        obj.hide_render = True
    return earlier


def evaluated_centroid(name: str) -> Vector:
    """World-space centroid of a mesh after the armature deformation."""
    depsgraph = bpy.context.evaluated_depsgraph_get()
    obj = bpy.data.objects[name].evaluated_get(depsgraph)
    mesh = obj.to_mesh()
    try:
        total = Vector((0.0, 0.0, 0.0))
        for v in mesh.vertices:
            total += v.co
        return obj.matrix_world @ (total / max(len(mesh.vertices), 1))
    finally:
        obj.to_mesh_clear()


def bone_point(arm, bone: str, t: float) -> Vector:
    pb = arm.pose.bones[bone]
    return arm.matrix_world @ pb.head.lerp(pb.tail, t)


def frame_camera(group: str):
    """Same camera and resolution as rig_render's contract/combat masters."""
    configure()
    if group == "combat":
        w, h = COMBAT_FRAME
        RR.set_resolution(w * RR.COMBAT_SUPERSAMPLE, h * RR.COMBAT_SUPERSAMPLE)
        bpy.context.scene.camera = RR.combat_camera()
        cell, pivot = COMBAT_FRAME, COMBAT_PIVOT
    else:
        use_camera("side")
        cell, pivot = LOCOMOTION_CELL, LOCOMOTION_PIVOT
    turn_rig(-90.0)
    bpy.context.view_layer.update()
    return bpy.context.scene.camera, cell, pivot


def measure(arm, group: str) -> dict[str, object]:
    cam, (w, h), (px, py) = frame_camera(group)
    scene = bpy.context.scene

    def project(point: Vector) -> tuple[float, float, float]:
        co = world_to_camera_view(scene, cam, point)
        return co.x * w - px, py - (1.0 - co.y) * h, co.z

    grip = project(evaluated_centroid("sword_grip"))
    pommel = project(evaluated_centroid("sword_pommel"))
    guard = project(evaluated_centroid("sword_collar"))
    chest = project(bone_point(arm, "chest", 0.5))
    hand_l = project(bone_point(arm, "hand.L", 0.6))
    head = project(bone_point(arm, "head", 0.45))
    back = project(bone_point(arm, "collar_back", 0.0))
    import math

    angle = math.degrees(math.atan2(guard[1] - pommel[1], guard[0] - pommel[0]))
    r = lambda v: round(v, 2)  # noqa: E731
    configure()
    return {
        "weapon": {"x": r(grip[0]), "y": r(grip[1]), "angle": r(angle), "front": grip[2] < chest[2]},
        "hand_l": {"x": r(hand_l[0]), "y": r(hand_l[1]), "front": hand_l[2] < chest[2]},
        "head": {"x": r(head[0]), "y": r(head[1])},
        "chest": {"x": r(chest[0]), "y": r(chest[1])},
        "back": {"x": r(back[0]), "y": r(back[1])},
    }


def render_master(group: str, path: Path) -> None:
    use_toon(True)  # finish C (flat colours), the published finish
    if group == "combat":
        RR.render_combat_side(path)
    else:
        RR.render_contract_side(path)
    use_toon(False)


def main() -> None:
    original = json.loads(ORIGINAL.read_text(encoding="utf-8"))
    wanted = MODULES[ARGS.module][3] or tuple(CL.CLIPS)
    keys = [k for k in ARGS.clips.split(",") if k] or list(wanted)
    unknown = sorted(set(keys) - set(CL.CLIPS))
    if unknown:
        raise SystemExit(f"unknown clips {unknown}")
    out = OUT_ROOT / ARGS.module
    manifest_path = out / ("game_sprites_manifest.json" if not ARGS.clips and not ARGS.limit
                           else "partial_manifest.json")
    if manifest_path.exists() and not ARGS.overwrite and not ARGS.sockets_only:
        raise SystemExit(f"already exists: {manifest_path}; pass --overwrite")
    arm = bpy.data.objects[C.ARMATURE_NAME]
    body = PS.all_body()
    configure()
    bpy.context.scene.render.fps = C.PRODUCTION_FPS
    PS.reset(arm)
    hidden = hide_sword()
    result = {"clips": {}, "renders": {}, "sockets": {}}
    for key in keys:
        clip = CL.CLIPS[key]
        action = CL.build_action(arm, key, body)
        arm.animation_data.action = None
        expected = original["clips"][key]["keyframes_sha256"]
        actual = BA.keyframe_hash(action)
        if actual != expected:
            raise SystemExit(f"{key}: pose keyframes differ from {ORIGINAL.name} ({actual} != {expected})")
        frames = clip.frames if not ARGS.limit else min(ARGS.limit, clip.frames)
        sockets, masters = [], {}
        for i in range(frames):
            PS.apply(arm, CL.clip_pose(arm, key, i), body)
            bpy.context.view_layer.update()
            sockets.append(measure(arm, clip.group))
            if not ARGS.sockets_only:
                path = out / "masters" / key / f"{key}_W_flat_{i:03d}.png"
                render_master(clip.group, path)
                masters[str(i)] = {"W_flat": str(path)}
        PS.reset(arm)
        result["clips"][key] = {"keyframes_sha256": actual, "frames": frames, "group": clip.group}
        result["renders"][key] = {"masters": masters}
        result["sockets"][key] = sockets
        print(f"[v2_2] {key}: {frames} frames, keyframes match the {ARGS.module} build")
    for name, flag in hidden.items():
        bpy.data.objects[name].hide_render = flag
    merged = dict(original)
    merged["renders"] = result["renders"]
    merged["clips"] = {k: original["clips"][k] for k in keys}
    merged["sockets"] = result["sockets"]
    merged["weaponless"] = {"hidden_objects": list(SWORD_OBJECTS), "source_manifest": str(ORIGINAL),
                            "blender": bpy.app.version_string, "finish": "C (W_flat)"}
    out.mkdir(parents=True, exist_ok=True)
    name = manifest_path.name if not ARGS.sockets_only else "sockets_only.json"
    (out / name).write_text(json.dumps(merged, indent=2, ensure_ascii=False, default=str) + "\n", encoding="utf-8")
    print(f"[v2_2] wrote {out / name}")


if __name__ == "__main__":
    main()
