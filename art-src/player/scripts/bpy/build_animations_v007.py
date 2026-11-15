"""Technical animation tests of V1.2-G on the v007 model: walk, attack_basic, dodge.

Opens swordsman_rig_v007.blend read-only, checks it and the gameplay timings
(read from src/core/constants.rs, never written), builds one action per
clip with a key on every frame (``swordsman_rig_v007/animations.py``),
validates every frame, renders the review views and the contract masters
of every frame, and saves a NEW file with the actions. Run from the root:

    blender --background --factory-startup art-src/player/blender/swordsman_rig_v007.blend \
        --python art-src/player/scripts/bpy/build_animations_v007.py -- [--overwrite] [--no-render] [--out DIR]

Outputs: art-src/player/blender/swordsman_rig_v007_animations.blend and
art-src/player/renders/v1_2g_animation_tests/ (or ``--out``, used by the
reproducibility run): masters/, review/, animation_manifest.json, read by
post/animation_sheets_v1_2g.py. Never writes v001-v007 model files.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

SCRIPTS_DIR = Path(__file__).resolve().parent
PACKAGES = (SCRIPTS_DIR, SCRIPTS_DIR / "swordsman_v003", SCRIPTS_DIR / "swordsman_v004",
            SCRIPTS_DIR / "swordsman_rig_v005", SCRIPTS_DIR / "swordsman_rig_v006",
            SCRIPTS_DIR / "swordsman_rig_v007")
for entry in PACKAGES:
    if str(entry) in sys.path:
        sys.path.remove(str(entry))
    sys.path.insert(0, str(entry))

import bpy  # noqa: E402
import numpy as np  # noqa: E402

import animations as AN  # noqa: E402
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

PROJECT_ROOT = SCRIPTS_DIR.parents[3]
CONSTANTS_RS = PROJECT_ROOT / "src" / "core" / "constants.rs"
GAME_CONSTANTS = {"BASIC_ATTACK_TIME": C.BASIC_ATTACK_TIME_S, "PLAYER_DODGE_TIME": C.PLAYER_DODGE_TIME_S,
                  "PLAYER_WALK_SPEED": C.PLAYER_WALK_SPEED_PX_S, "PLAYER_DODGE_SPEED": C.PLAYER_DODGE_SPEED_PX_S,
                  "HITBOX_ACTIVE_FRACTION": C.HITBOX_ACTIVE_FRACTION}
FEET = {"R": ("boot_sole_near", "boot_heel_near"), "L": ("boot_sole_far", "boot_heel_far")}
POP_TOLERANCE_M = 0.01  # final shell moving this much more than its rig stage = a pop


def parse_args() -> argparse.Namespace:
    argv = sys.argv[sys.argv.index("--") + 1 :] if "--" in sys.argv else []
    parser = argparse.ArgumentParser(prog="build_animations_v007.py")
    parser.add_argument("--overwrite", action="store_true")
    parser.add_argument("--no-render", action="store_true")
    parser.add_argument("--out", type=Path, default=C.ANIM_OUT_DIR)
    parser.add_argument("--no-save", action="store_true", help="do not write the .blend (reproducibility run)")
    return parser.parse_args(argv)


def read_game_constants() -> dict[str, object]:
    """Reads the gameplay constants (read-only) and checks them against rig_config."""
    text = CONSTANTS_RS.read_text(encoding="utf-8")
    found, mismatches = {}, []
    for name, expected in GAME_CONSTANTS.items():
        m = re.search(rf"pub const {name}: f32 = ([0-9.\-]+);", text)
        if m is None:
            mismatches.append(f"{name} not found")
            continue
        found[name] = float(m.group(1))
        if abs(found[name] - expected) > 1e-6:
            mismatches.append(f"{name} = {found[name]} in the game, {expected} in rig_config")
    if mismatches:
        raise SystemExit(f"gameplay timings changed, update rig_config first: {mismatches}")
    return found


def check_source(body_all: list[bpy.types.Object]) -> dict[str, object]:
    opened = Path(bpy.data.filepath).resolve() if bpy.data.filepath else None
    if opened != C.BLEND_PATH.resolve():
        raise SystemExit(f"open {C.BLEND_PATH} (got {opened})")
    arm = bpy.data.objects.get(C.ARMATURE_NAME)
    if arm is None:
        raise SystemExit(f"armature {C.ARMATURE_NAME} missing")
    manifest = json.loads((C.OUT_DIR / "rig_manifest_v007.json").read_text(encoding="utf-8"))
    structure = V.check_structure(arm, body_all)
    weights = V.check_weights(body_all)
    w_hash = V6.weight_hash(body_all)
    problems = list(structure["failures"]) + list(weights["failures"])
    if w_hash != manifest["weights_sha256"]:
        problems.append("weights differ from the v007 build")
    if V6.geometry_hash_without_rig(body_all) != manifest["geometry_sha256_without_rig"]:
        problems.append("geometry differs from the v007 build")
    if problems:
        raise SystemExit(f"v007 file is not the validated build: {problems}")
    return {"bones": structure["bones"], "deform_bones": structure["deform_bones"], "weights_sha256": w_hash}


def keyframe_hash(action: bpy.types.Action) -> str:
    """SHA-256 of every key (data path, index, frame, value rounded to 1e-6) of the action."""
    import hashlib

    if hasattr(action, "layers") and action.layers:
        curves = [fc for layer in action.layers for strip in layer.strips for bag in strip.channelbags
                  for fc in bag.fcurves]
    else:
        curves = list(getattr(action, "fcurves", []))
    h = hashlib.sha256()
    for fc in sorted(curves, key=lambda c: (c.data_path, c.array_index)):
        for kp in fc.keyframe_points:
            h.update(f"{fc.data_path}|{fc.array_index}|{kp.co[0]:.3f}|{kp.co[1]:.6f}|{kp.interpolation}".encode())
    return h.hexdigest()


def feet_state(side: str) -> tuple[float, float]:
    """(mean x, min z) of a boot's sole and heel, world space."""
    arrays = SF.final_arrays([bpy.data.objects[n] for n in FEET[side]])
    pts = np.concatenate(list(arrays.values()))
    return float(pts[:, 0].mean()), float(pts[:, 2].min())


def max_displacement(a: dict[str, np.ndarray], b: dict[str, np.ndarray]) -> dict[str, float]:
    out = {}
    for name in a:
        if a[name].shape == b[name].shape and a[name].size:
            out[name] = float(np.abs(a[name] - b[name]).reshape(-1, 3).max(axis=0).max())
    return out


def validate_clip(arm, key: str, body, body_all, shells, rest, rest_extra, sword_rest) -> dict[str, object]:
    clip = AN.CLIPS[key]
    frames, rig_prev, fin_prev, rig_first, fin_first = [], None, None, None, None
    for i in range(clip.frames):
        height = PS.apply(arm, AN.clip_pose(arm, key, i), body)
        issues = V.pose_issues(rest, V.snapshot()) + V6.extra_issues(rest_extra, V6.extra_snapshot())
        offset = V.sword_offset(arm)
        rig = V6.evaluated_arrays(body_all, armature_only=True)
        fin = V6.evaluated_arrays(body_all)
        entry = {
            "index": i, "t": clip.times()[i], "duration": round(clip.durations[i], 6),
            "root_height_m": round(height, 4), "max_severity": V.max_severity(issues), "issues": issues,
            "sword_offset_drift_m": round((offset.translation - sword_rest.translation).length, 7),
            "sword_angle_drift_deg": round(V.angle_between(offset, sword_rest), 5),
            "scale_failures": V.check_scale(arm), "shell_residual_m": SF.shell_residuals(arm, shells),
            "feet": {s: tuple(round(v, 4) for v in feet_state(s)) for s in ("R", "L")},
            "boot_volume_ratio": {n: round(v / rest["volumes"][n], 4) for n, v in V.snapshot()["volumes"].items()
                                  if n.startswith("boot_foot")},
            "lowest_z_m": round(PS.lowest_z([bpy.data.objects[n] for n in PS.FEET]), 5),
        }
        if rig_prev is not None:
            entry["step_from_previous"] = _step(rig_prev, rig, fin_prev, fin)
        else:
            rig_first, fin_first = rig, fin
        frames.append(entry)
        rig_prev, fin_prev = rig, fin
        print(f"[anim] {key} f{i:02d} {entry['max_severity']} shell {max(entry['shell_residual_m'].values()):.1e}")
    result: dict[str, object] = {"frames": frames}
    if clip.loop:
        # Closing step (last -> first) and the frame that would follow the last one.
        result["loop_step"] = _step(rig_prev, rig_first, fin_prev, fin_first)
        PS.apply(arm, AN.clip_pose(arm, key, clip.frames), body)
        closing = V6.evaluated_arrays(body_all, armature_only=True)
        result["loop_closure_max_m"] = round(max(max_displacement(closing, rig_first).values()), 7)
    PS.reset(arm)
    bpy.context.view_layer.update()
    return result


def _step(rig_a, rig_b, fin_a, fin_b) -> dict[str, object]:
    rig = max_displacement(rig_a, rig_b)
    fin = max_displacement(fin_a, fin_b)
    pops = {n: round(fin[n] - rig[n], 5) for n in fin if n in rig and fin[n] - rig[n] > POP_TOLERANCE_M}
    body_only = {n: v for n, v in rig.items() if not n.startswith(("blade", "sword_"))}
    return {"max_rig_m": round(max(rig.values()), 5), "max_body_m": round(max(body_only.values()), 5),
            "fastest": max(rig, key=rig.get), "pops": pops}


def render_clip(arm, key: str, body, out: Path) -> dict[str, object]:
    clip = AN.CLIPS[key]
    written: dict[str, object] = {"review": {}, "masters": {}}
    for i in range(clip.frames):
        PS.apply(arm, AN.clip_pose(arm, key, i), body)
        written["review"][i] = {v: str(VW.render_view(v, out / "review" / key / f"{key}_{i:03d}_{v}.png", 384))
                                for v in ("side", "far_back")}
        variants = [("N", False), ("W", True)] if clip.group == "combat" else [("N", False)]
        masters = {}
        for tag, wide in variants:
            RR.set_readable_blade(wide)
            for finish, toon in (("shaded", False), ("flat", True)):
                use_toon(toon)
                path = out / "masters" / key / f"{key}_{tag}_{finish}_{i:03d}.png"
                if clip.group == "combat":
                    RR.render_combat_side(path)
                else:
                    RR.render_contract_side(path)
                masters[f"{tag}_{finish}"] = str(path)
            use_toon(False)
        RR.set_readable_blade(False)
        written["masters"][i] = masters
    PS.reset(arm)
    configure()
    return written


def main() -> None:
    args = parse_args()
    out = args.out
    if not args.overwrite and not args.no_save:
        existing = [p for p in (C.ANIM_BLEND, out / "animation_manifest.json") if p.exists()]
        if existing:
            raise SystemExit(f"already exists: {existing}; pass --overwrite")
    if C.ANIM_BLEND.resolve() in {p.resolve() for p in (*C.PROTECTED_BLENDS, C.BLEND_PATH)}:
        raise SystemExit("refusing to write over a model file")
    body_all = [o for o in bpy.data.collections[C.SOURCE_COLLECTION].objects if o.type == "MESH"]
    source = check_source(body_all)
    game = read_game_constants()
    print(f"[anim] Blender {bpy.app.version_string}; game timings {game}")
    arm = bpy.data.objects[C.ARMATURE_NAME]
    configure()
    scene = bpy.context.scene
    scene.render.fps = C.PRODUCTION_FPS
    body = PS.all_body()
    shells = SF.solidify_objects(body_all)
    PS.reset(arm)
    bpy.context.view_layer.update()
    rest, rest_extra, sword_rest = V.snapshot(), V6.extra_snapshot(), V.sword_offset(arm)

    clips: dict[str, object] = {}
    for key, clip in AN.CLIPS.items():
        action, heights = AN.build_action(arm, key, body)
        arm.animation_data.action = None
        clips[key] = {
            "action": action.name, "keyframes_sha256": keyframe_hash(action), "group": clip.group, "loop": clip.loop, "frames": clip.frames,
            "production_fps": C.PRODUCTION_FPS, "times_s": clip.times(),
            "durations_s": [round(d, 6) for d in clip.durations], "duration_s": round(clip.duration, 6),
            "derived": {str(C.DERIVED_FPS): [{"source_frame": i, "duration_s": d}
                                             for i, d in AN.derived_indices(clip, C.DERIVED_FPS)]},
            "root_motion": "none: vertical grounding only, the game moves the entity",
            "root_heights_m": [round(h, 4) for h in heights],
            "validation": validate_clip(arm, key, body, body_all, shells, rest, rest_extra, sword_rest),
        }
    clips["walk"]["walk_speed_m_s"] = round(AN.walk_speed_m_s(), 4)
    clips["walk"]["walk_speed_px_s"] = round(AN.walk_speed_m_s() / RR.METRES_PER_PX, 2)
    clips["walk"]["game_walk_speed_px_s"] = game["PLAYER_WALK_SPEED"]
    clips["attack_basic"]["hitbox_window_s"] = [0.0, round(C.BASIC_ATTACK_TIME_S * C.HITBOX_ACTIVE_FRACTION, 4)]
    clips["dodge"]["game_dodge_distance_px"] = round(game["PLAYER_DODGE_SPEED"] * game["PLAYER_DODGE_TIME"], 1)

    manifest: dict[str, object] = {
        "model": C.BLEND_PATH.name, "source_check": source, "blender": bpy.app.version_string,
        "engine": scene.render.engine, "game_constants": game, "clips": clips,
        "contract": {"locomotion_cell": [64, 64], "locomotion_pivot": [32.0, 63.0], "combat_cell": [128, 96],
                     "combat_pivot": [40.0, 95.0], "metres_per_px": RR.METRES_PER_PX},
    }
    if not args.no_render:
        manifest["renders"] = {key: render_clip(arm, key, body, out) for key in AN.CLIPS}
    VW.remove_review_cameras()
    turn_rig(SIDE_AZIMUTH)
    PS.reset(arm)
    bpy.context.view_layer.update()
    if not args.no_save:
        bpy.ops.wm.save_as_mainfile(filepath=str(C.ANIM_BLEND), copy=True)
        print(f"[anim] saved {C.ANIM_BLEND}")
    out.mkdir(parents=True, exist_ok=True)
    (out / "animation_manifest.json").write_text(json.dumps(manifest, indent=2, ensure_ascii=False, default=str)
                                                 + "\n", encoding="utf-8")
    worst = {k: V.max_severity([i for f in c["validation"]["frames"] for i in f["issues"]]) for k, c in clips.items()}
    print(f"[anim] worst severity per clip: {worst}")


if __name__ == "__main__":
    main()
