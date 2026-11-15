"""In-game sprite masters of the V1.4 expansion from the v007 model.

Clips (``game_sprites_v1_4/clips_v1_4.py``): hurt, death, jump_rise, fall,
land, crouch, guard_start, slash_crescent, slash_rising, cast_forward,
cast_up. Same pipeline and checks as build_game_sprites_v1_3.py, whose
script and outputs are not touched.

Opens swordsman_rig_v007.blend read-only, checks it against its build
manifest and the gameplay timings (read from src/core/constants.rs, never
written), builds one action per clip, validates every frame (deformation,
sabre, shells, pops, grounding and the sabre above the floor), renders the
contract masters with the x1.7 blade (shaded for finish B, flat for finish
C) and a side review image per frame, and saves a NEW file with the
actions. Run from the project root:

    blender --background --factory-startup art-src/player/blender/swordsman_rig_v007.blend \
        --python art-src/player/scripts/bpy/build_game_sprites_v1_4.py -- [--overwrite] [--no-save] [--out DIR]

Outputs: art-src/player/blender/swordsman_rig_v007_game_v1_4.blend and
art-src/player/renders/v1_4_game_sprites/ (masters/, review/,
game_sprites_manifest.json). post/game_sprites_v1_4.py packs these masters
together with the V1.3 ones into the game atlases.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

SCRIPTS_DIR = Path(__file__).resolve().parent
PACKAGES = (SCRIPTS_DIR, SCRIPTS_DIR / "swordsman_v003", SCRIPTS_DIR / "swordsman_v004",
            SCRIPTS_DIR / "swordsman_rig_v005", SCRIPTS_DIR / "swordsman_rig_v006",
            SCRIPTS_DIR / "swordsman_rig_v007", SCRIPTS_DIR / "game_sprites_v1_4")
for entry in PACKAGES:
    if str(entry) in sys.path:
        sys.path.remove(str(entry))
    sys.path.insert(0, str(entry))

import bpy  # noqa: E402

import build_animations_v007 as BA  # noqa: E402  (V1.2-G helpers, unchanged)
import clips_v1_4 as CL  # noqa: E402
import poses as PS  # noqa: E402
import rig_config as C  # noqa: E402
import rig_render as RR  # noqa: E402
import rig_validate as V  # noqa: E402
import shell_fix as SF  # noqa: E402
import validate_v006 as V6  # noqa: E402
import views_v007 as VW  # noqa: E402
from render import configure  # noqa: E402
from staging import SIDE_AZIMUTH, turn_rig  # noqa: E402
from surfaces import use_toon  # noqa: E402

OUT_DIR = Path(C.ANIM_OUT_DIR).parent / "v1_4_game_sprites"
GAME_BLEND = C.BLEND_PATH.with_name("swordsman_rig_v007_game_v1_4.blend")
V1_3_BLEND = C.BLEND_PATH.with_name("swordsman_rig_v007_game_v1_3.blend")


def parse_args() -> argparse.Namespace:
    argv = sys.argv[sys.argv.index("--") + 1 :] if "--" in sys.argv else []
    parser = argparse.ArgumentParser(prog="build_game_sprites_v1_4.py")
    parser.add_argument("--overwrite", action="store_true")
    parser.add_argument("--no-save", action="store_true", help="no .blend (reproducibility run)")
    parser.add_argument("--out", type=Path, default=OUT_DIR)
    return parser.parse_args(argv)


def stance_speed(frames: list[dict], fps: int) -> dict[str, object]:
    """Backward speed of each foot while it is on the ground (z ~ 0), in m/s."""
    out = {}
    for side in ("R", "L"):
        steps = []
        n = len(frames)
        for i in range(n):
            a, b = frames[i]["feet"][side], frames[(i + 1) % n]["feet"][side]
            if a[1] < 0.005 and b[1] < 0.005:  # on the ground in both frames
                steps.append((a[0] - b[0]) * fps)
        out[side] = [round(s, 3) for s in steps]
    return out


def validate(arm, key: str, body, body_all, shells, rest, rest_extra, sword_rest) -> dict[str, object]:
    clip = CL.CLIPS[key]
    frames, prev, first = [], None, None
    for i in range(clip.frames):
        height = PS.apply(arm, CL.clip_pose(arm, key, i), body)
        issues = V.pose_issues(rest, V.snapshot()) + V6.extra_issues(rest_extra, V6.extra_snapshot())
        offset = V.sword_offset(arm)
        rig = V6.evaluated_arrays(body_all, armature_only=True)
        fin = V6.evaluated_arrays(body_all)
        entry = {
            "index": i, "t": CL.times(key)[i], "duration": round(clip.durations[i], 6),
            "root_height_m": round(height, 4), "max_severity": V.max_severity(issues), "issues": issues,
            "sword_offset_drift_m": round((offset.translation - sword_rest.translation).length, 7),
            "scale_failures": V.check_scale(arm),
            "shell_residual_m": max(SF.shell_residuals(arm, shells).values()),
            "feet": {s: tuple(round(v, 4) for v in BA.feet_state(s)) for s in ("R", "L")},
            "lowest_z_m": round(PS.lowest_z([bpy.data.objects[n] for n in PS.FEET]), 5),
            # The sabre is not part of the grounding: it must still stay above the floor.
            "blade_lowest_z_m": round(PS.lowest_z([bpy.data.objects["blade_readable"]]), 4),
        }
        if prev is not None:
            entry["step_from_previous"] = BA._step(prev[0], rig, prev[1], fin)
        else:
            first = (rig, fin)
        frames.append(entry)
        prev = (rig, fin)
    result: dict[str, object] = {"frames": frames}
    if clip.loop:
        result["loop_step"] = BA._step(prev[0], first[0], prev[1], first[1])
        PS.apply(arm, CL.clip_pose(arm, key, clip.frames), body)
        closing = V6.evaluated_arrays(body_all, armature_only=True)
        result["loop_closure_max_m"] = round(max(BA.max_displacement(closing, first[0]).values()), 7)
        if key != "fall":
            result["stance_speed_m_s"] = stance_speed(frames, CL.FPS[key])
    PS.reset(arm)
    bpy.context.view_layer.update()
    print(f"[game] {key}: {[f['max_severity'] for f in frames]}")
    return result


def render(arm, key: str, body, out: Path) -> dict[str, object]:
    clip = CL.CLIPS[key]
    written: dict[str, object] = {"review": {}, "masters": {}}
    RR.set_readable_blade(True)  # V1.3: x1.7 blade for the in-game tests
    for i in range(clip.frames):
        PS.apply(arm, CL.clip_pose(arm, key, i), body)
        written["review"][i] = str(VW.render_view("side", out / "review" / key / f"{key}_{i:03d}.png", 384))
        masters = {}
        for finish, toon in (("shaded", False), ("flat", True)):
            use_toon(toon)
            path = out / "masters" / key / f"{key}_W_{finish}_{i:03d}.png"
            if clip.group == "combat":
                RR.render_combat_side(path)
            else:
                RR.render_contract_side(path)
            masters[f"W_{finish}"] = str(path)
        use_toon(False)
        written["masters"][i] = masters
    RR.set_readable_blade(False)
    PS.reset(arm)
    configure()
    return written


def main() -> None:
    args = parse_args()
    out = args.out
    if not args.overwrite and not args.no_save:
        existing = [p for p in (GAME_BLEND, out / "game_sprites_manifest.json") if p.exists()]
        if existing:
            raise SystemExit(f"already exists: {existing}; pass --overwrite")
    if GAME_BLEND.resolve() in {p.resolve() for p in (*C.PROTECTED_BLENDS, C.BLEND_PATH, C.ANIM_BLEND, V1_3_BLEND)}:
        raise SystemExit("refusing to write over an earlier file")
    body_all = [o for o in bpy.data.collections[C.SOURCE_COLLECTION].objects if o.type == "MESH"]
    source = BA.check_source(body_all)
    game = BA.read_game_constants()
    arm = bpy.data.objects[C.ARMATURE_NAME]
    configure()
    bpy.context.scene.render.fps = C.PRODUCTION_FPS
    body = PS.all_body()
    shells = SF.solidify_objects(body_all)
    PS.reset(arm)
    bpy.context.view_layer.update()
    rest, rest_extra, sword_rest = V.snapshot(), V6.extra_snapshot(), V.sword_offset(arm)
    clips: dict[str, object] = {}
    for key, clip in CL.CLIPS.items():
        action = CL.build_action(arm, key, body)
        arm.animation_data.action = None
        clips[key] = {
            "action": action.name, "keyframes_sha256": BA.keyframe_hash(action), "group": clip.group,
            "loop": clip.loop, "frames": clip.frames, "fps": CL.FPS[key], "times_s": CL.times(key),
            "durations_s": [round(d, 6) for d in clip.durations], "duration_s": round(clip.duration, 6),
            "validation": validate(arm, key, body, body_all, shells, rest, rest_extra, sword_rest),
        }
    for key in clips:
        clips[key]["mode"] = CL.MODE[key]
    manifest: dict[str, object] = {
        "model": C.BLEND_PATH.name, "source_check": source, "blender": bpy.app.version_string,
        "game_constants": game, "blade": "x1.7 (blade_readable)", "clips": clips,
        "contract": {"locomotion_cell": [64, 64], "locomotion_pivot": [32.0, 63.0], "combat_cell": [128, 96],
                     "combat_pivot": [40.0, 95.0], "metres_per_px": RR.METRES_PER_PX},
        "renders": {key: render(arm, key, body, out) for key in CL.CLIPS},
    }
    VW.remove_review_cameras()
    turn_rig(SIDE_AZIMUTH)
    PS.reset(arm)
    bpy.context.view_layer.update()
    if not args.no_save:
        bpy.ops.wm.save_as_mainfile(filepath=str(GAME_BLEND), copy=True)
        print(f"[game] saved {GAME_BLEND}")
    out.mkdir(parents=True, exist_ok=True)
    (out / "game_sprites_manifest.json").write_text(json.dumps(manifest, indent=2, ensure_ascii=False, default=str)
                                                    + "\n", encoding="utf-8")
    worst = {k: V.max_severity([i for f in c["validation"]["frames"] for i in f["issues"]]) for k, c in clips.items()}
    print(f"[game] worst severity per clip: {worst}")


if __name__ == "__main__":
    main()
