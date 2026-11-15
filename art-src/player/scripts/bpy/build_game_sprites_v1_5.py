"""Sprite masters of the V1.5 art polish from the v007 model.

Clips (``game_sprites_v1_5/clips_v1_5.py``): hurt_a, hurt_b, fall_v15,
attack_basic_air and death_lying (experimental). Same validation and render
as build_game_sprites_v1_4.py, whose functions are reused unchanged with the
V1.5 clip module; the V1.3/V1.4 scripts, blends and renders are not touched.

Opens swordsman_rig_v007.blend read-only and saves a NEW file with the
actions. Run from the project root:

    blender --background --factory-startup art-src/player/blender/swordsman_rig_v007.blend \
        --python art-src/player/scripts/bpy/build_game_sprites_v1_5.py -- \
        [--overwrite] [--no-save] [--out DIR] [--clips a,b] [--review-only]

``--review-only`` renders only the side review images (fast pose iteration);
``--clips`` limits the run to some clips (implies ``--no-save``).

Outputs: art-src/player/blender/swordsman_rig_v007_game_v1_5.blend and
art-src/player/renders/v1_5_game_sprites/ (masters/, review/,
game_sprites_manifest.json). post/game_sprites_v1_5.py stages the game
atlases from the V1.3, V1.4 and V1.5 masters; only publish_guard.py writes
``assets/``.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

SCRIPTS_DIR = Path(__file__).resolve().parent
PACKAGES = (SCRIPTS_DIR, SCRIPTS_DIR / "swordsman_v003", SCRIPTS_DIR / "swordsman_v004",
            SCRIPTS_DIR / "swordsman_rig_v005", SCRIPTS_DIR / "swordsman_rig_v006",
            SCRIPTS_DIR / "swordsman_rig_v007", SCRIPTS_DIR / "game_sprites_v1_4",
            SCRIPTS_DIR / "game_sprites_v1_5")
for entry in PACKAGES:
    if str(entry) in sys.path:
        sys.path.remove(str(entry))
    sys.path.insert(0, str(entry))

import bpy  # noqa: E402

import build_animations_v007 as BA  # noqa: E402  (V1.2-G helpers, unchanged)
import build_game_sprites_v1_4 as B4  # noqa: E402  (V1.4 validate/render, unchanged)
import clips_v1_5 as CL  # noqa: E402
import poses as PS  # noqa: E402
import rig_config as C  # noqa: E402
import rig_validate as V  # noqa: E402
import shell_fix as SF  # noqa: E402
import validate_v006 as V6  # noqa: E402
import views_v007 as VW  # noqa: E402
from render import configure  # noqa: E402
from staging import SIDE_AZIMUTH, turn_rig  # noqa: E402

# The V1.4 validate/render read the clip module through their global ``CL``.
B4.CL = CL

OUT_DIR = Path(C.ANIM_OUT_DIR).parent / "v1_5_game_sprites"
GAME_BLEND = C.BLEND_PATH.with_name("swordsman_rig_v007_game_v1_5.blend")
EARLIER = (C.BLEND_PATH.with_name("swordsman_rig_v007_game_v1_3.blend"),
           C.BLEND_PATH.with_name("swordsman_rig_v007_game_v1_4.blend"))


def parse_args() -> argparse.Namespace:
    argv = sys.argv[sys.argv.index("--") + 1 :] if "--" in sys.argv else []
    parser = argparse.ArgumentParser(prog="build_game_sprites_v1_5.py")
    parser.add_argument("--overwrite", action="store_true")
    parser.add_argument("--no-save", action="store_true", help="no .blend (reproducibility run)")
    parser.add_argument("--out", type=Path, default=OUT_DIR)
    parser.add_argument("--clips", default="", help="comma-separated subset (implies --no-save)")
    parser.add_argument("--review-only", action="store_true", help="side review images only, no masters")
    return parser.parse_args(argv)


def review_only(arm, key: str, body, out: Path) -> dict[str, object]:
    from rig_render import set_readable_blade

    set_readable_blade(True)
    written = {"review": {}}
    for i in range(CL.CLIPS[key].frames):
        PS.apply(arm, CL.clip_pose(arm, key, i), body)
        written["review"][i] = str(VW.render_view("side", out / "review" / key / f"{key}_{i:03d}.png", 384))
    set_readable_blade(False)
    PS.reset(arm)
    configure()
    return written


def main() -> None:
    args = parse_args()
    keys = [k for k in args.clips.split(",") if k] or list(CL.CLIPS)
    unknown = sorted(set(keys) - set(CL.CLIPS))
    if unknown:
        raise SystemExit(f"unknown clips {unknown}")
    save = not args.no_save and not args.clips and not args.review_only
    out = args.out
    if not args.overwrite and save:
        existing = [p for p in (GAME_BLEND, out / "game_sprites_manifest.json") if p.exists()]
        if existing:
            raise SystemExit(f"already exists: {existing}; pass --overwrite")
    protected = {p.resolve() for p in (*C.PROTECTED_BLENDS, C.BLEND_PATH, C.ANIM_BLEND, *EARLIER)}
    if GAME_BLEND.resolve() in protected:
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
    for key in keys:
        clip = CL.CLIPS[key]
        action = CL.build_action(arm, key, body)
        arm.animation_data.action = None
        clips[key] = {
            "action": action.name, "keyframes_sha256": BA.keyframe_hash(action), "group": clip.group,
            "loop": clip.loop, "frames": clip.frames, "fps": CL.FPS[key], "times_s": CL.times(key),
            "durations_s": [round(d, 6) for d in clip.durations], "duration_s": round(clip.duration, 6),
            "mode": CL.MODE[key], "game_clip": CL.GAME_CLIP[key],
            "validation": B4.validate(arm, key, body, body_all, shells, rest, rest_extra, sword_rest),
        }
    render = review_only if args.review_only else B4.render
    manifest: dict[str, object] = {
        "model": C.BLEND_PATH.name, "source_check": source, "blender": bpy.app.version_string,
        "game_constants": game, "blade": "x1.7 (blade_readable)", "clips": clips,
        "contract": {"locomotion_cell": [64, 64], "locomotion_pivot": [32.0, 63.0], "combat_cell": [128, 96],
                     "combat_pivot": [40.0, 95.0], "metres_per_px": B4.RR.METRES_PER_PX},
        "renders": {key: render(arm, key, body, out) for key in keys},
    }
    VW.remove_review_cameras()
    turn_rig(SIDE_AZIMUTH)
    PS.reset(arm)
    bpy.context.view_layer.update()
    if save:
        bpy.ops.wm.save_as_mainfile(filepath=str(GAME_BLEND), copy=True)
        print(f"[game15] saved {GAME_BLEND}")
    out.mkdir(parents=True, exist_ok=True)
    name = "game_sprites_manifest.json" if not args.clips and not args.review_only else "partial_manifest.json"
    (out / name).write_text(json.dumps(manifest, indent=2, ensure_ascii=False, default=str) + "\n",
                            encoding="utf-8")
    worst = {k: V.max_severity([i for f in c["validation"]["frames"] for i in f["issues"]]) for k, c in clips.items()}
    lows = {k: min(f["blade_lowest_z_m"] for f in c["validation"]["frames"]) for k, c in clips.items()}
    print(f"[game15] worst severity per clip: {worst}")
    print(f"[game15] lowest sabre point per clip (m): {lows}")


if __name__ == "__main__":
    main()
