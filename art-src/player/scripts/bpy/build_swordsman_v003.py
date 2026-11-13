"""Entry point of the v003 swordsman (V1.2-C): build, validate, save, render.

Run from the project root, in a separate Blender process (the user's open
scene is never touched):

    blender --background --factory-startup \
        --python art-src/player/scripts/bpy/build_swordsman_v003.py -- [--overwrite] [--no-render]

    # v002 geometry under the v003 lights, for the v002 x v003 comparison:
    blender --background --factory-startup \
        --python art-src/player/scripts/bpy/build_swordsman_v003.py -- --relight-v002

Outputs: art-src/player/blender/swordsman_base_v003.blend and
art-src/player/renders/v003/ (masters, close-ups, toon and readable-sword
variants, build_report_v003.json). Refuses to overwrite without --overwrite
and never writes v001/v002 files. v002 itself is still built by
build_base_model.py.

Validated in Blender 5.2.4 LTS (V1.2-C).
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

SCRIPTS_DIR = Path(__file__).resolve().parent
PACKAGE_DIR = SCRIPTS_DIR / "swordsman_v003"
for entry in (SCRIPTS_DIR, PACKAGE_DIR):
    if str(entry) not in sys.path:
        sys.path.insert(0, str(entry))

import bpy  # noqa: E402

import params as P  # noqa: E402
from anatomy import build_free_hand, build_legs, build_neck, build_torso  # noqa: E402
from clothing import (  # noqa: E402
    build_belt,
    build_boots,
    build_coat,
    build_collar,
    build_half_cape,
    build_pauldron,
    build_sleeves,
)
from head import build_face, build_hair  # noqa: E402
from mathutils import Vector  # noqa: E402
from render import configure  # noqa: E402
from staging import (  # noqa: E402
    MASTER_AZIMUTH,
    build_light_rig,
    image_path,
    render_master,
    render_shot,
    setup_world,
    shots,
)
from surfaces import use_toon  # noqa: E402
from sword import blade_point, build_blade, build_sword  # noqa: E402
from validate import check_scene, write_report  # noqa: E402

READABLE_BLADE_SCALE = 1.7
RELIT_DIR = P.OUT_DIR / "relit_v002"


def parse_args() -> argparse.Namespace:
    argv = sys.argv[sys.argv.index("--") + 1 :] if "--" in sys.argv else []
    parser = argparse.ArgumentParser(prog="build_swordsman_v003.py")
    parser.add_argument("--overwrite", action="store_true", help="replace existing v003 outputs")
    parser.add_argument("--no-render", action="store_true", help="only build, validate and save")
    parser.add_argument("--relight-v002", action="store_true",
                        help="render the v002 geometry with the v003 lights (comparison only)")
    return parser.parse_args(argv)


def clear_startup_scene() -> None:
    if bpy.data.filepath:
        raise SystemExit(f"refusing to clear a saved file ({bpy.data.filepath}); use --factory-startup")
    for obj in list(bpy.context.scene.objects):
        bpy.data.objects.remove(obj, do_unlink=True)


def refuse_existing(paths: list[Path], overwrite: bool) -> None:
    existing = [p for p in paths if p.exists()]
    if existing and not overwrite:
        names = ", ".join(str(p) for p in existing)
        raise SystemExit(f"already exists: {names}; pass --overwrite")


def build_model() -> None:
    build_torso()
    build_neck()
    build_face()
    build_hair()
    build_legs()
    build_boots()
    build_coat()
    build_collar()
    build_sleeves()
    build_belt()
    build_half_cape()
    build_pauldron()
    build_sword()
    build_free_hand(Vector(P.WRIST_FAR))
    # Readability variant of the blade, hidden unless explicitly rendered.
    build_blade(READABLE_BLADE_SCALE, suffix="_readable")
    readable = bpy.data.objects["blade_readable"]
    readable.hide_render = True
    readable.hide_set(True)


def set_readable_blade(enabled: bool) -> None:
    bpy.data.objects["blade"].hide_render = enabled
    bpy.data.objects["blade_readable"].hide_render = not enabled


def render_all() -> list[Path]:
    written = []
    for view in MASTER_AZIMUTH:
        written.append(render_master(view, image_path(f"{view}_master")))
    for view, shot in shots(blade_point(0.45)).items():
        written.append(render_shot(view, shot, image_path(view)))
    use_toon(True)
    written.append(render_master("side", image_path("side_master_toon")))
    written.append(render_master("front", image_path("front_master_toon")))
    use_toon(False)
    set_readable_blade(True)
    written.append(render_master("side", image_path("side_master_readable")))
    set_readable_blade(False)
    return written


def relight_v002(overwrite: bool) -> None:
    from geometry import build_all  # v002 geometry, unchanged

    paths = [RELIT_DIR / f"{P.MODEL_NAME}_v002_relit_{v}_master.png" for v in MASTER_AZIMUTH]
    refuse_existing(paths, overwrite)
    clear_startup_scene()
    build_all()
    setup_world()
    build_light_rig()
    configure()
    for view, path in zip(MASTER_AZIMUTH, paths):
        print(f"[v003] rendered {render_master(view, path)}")


def main() -> None:
    args = parse_args()
    print(f"[v003] Blender {bpy.app.version_string}")
    if args.relight_v002:
        relight_v002(args.overwrite)
        return
    planned = [P.BLEND_PATH, image_path("side_master")]
    refuse_existing(planned, args.overwrite)
    clear_startup_scene()
    build_model()
    setup_world()
    build_light_rig()
    engine = configure()
    print(f"[v003] render engine: {engine}")
    checks = check_scene()
    for failure in checks["failures"]:
        print(f"[v003] FAIL {failure}")
    for warning in checks["warnings"]:
        print(f"[v003] warning {warning}")
    print(f"[v003] {checks['objects']} objects, {checks['triangles_rendered']} rendered triangles, "
          f"top {checks['top_m']} m, sole {checks['bottom_m']} m")
    P.BLEND_PATH.parent.mkdir(parents=True, exist_ok=True)
    bpy.ops.wm.save_as_mainfile(filepath=str(P.BLEND_PATH))
    print(f"[v003] saved {P.BLEND_PATH}")
    written = [] if args.no_render else render_all()
    for path in written:
        print(f"[v003] rendered {path}")
    write_report(P.OUT_DIR / f"build_report_{P.MODEL_VERSION}.json", checks,
                 {"renders": [p.name for p in written], "readable_blade_scale": READABLE_BLADE_SCALE})
    if checks["failures"]:
        raise SystemExit(f"{len(checks['failures'])} validation failure(s)")


if __name__ == "__main__":
    main()
