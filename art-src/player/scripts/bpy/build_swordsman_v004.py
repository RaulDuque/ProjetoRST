"""Entry point of the v004 swordsman (V1.2-D): build, validate, save, render.

Run from the project root, in a separate Blender process (the user's open
scene is never touched):

    blender --background --factory-startup \
        --python art-src/player/scripts/bpy/build_swordsman_v004.py -- [--overwrite] [--no-render]

    # quick look at some views only (half resolution, renders/v004/preview/):
    blender --background --factory-startup \
        --python art-src/player/scripts/bpy/build_swordsman_v004.py -- --preview close_face,side_master

Outputs: art-src/player/blender/swordsman_base_v004.blend and
art-src/player/renders/v004/ (masters, close-ups, toon and readable-sword
variants, build_report_v004.json). Refuses to overwrite without --overwrite
and never writes v001-v003 files.

v004 refines v003 locally: see ``swordsman_v004/__init__.py`` for which v003
modules are reused and which are replaced. Cameras, light rig and pose are
the v003 ones, so v003 x v004 renders are directly comparable.
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

SCRIPTS_DIR = Path(__file__).resolve().parent
V003_DIR = SCRIPTS_DIR / "swordsman_v003"
V004_DIR = SCRIPTS_DIR / "swordsman_v004"
# Order matters: v004 modules shadow the v003 modules of the same name.
for entry in (SCRIPTS_DIR, V003_DIR, V004_DIR):
    if str(entry) in sys.path:
        sys.path.remove(str(entry))
    sys.path.insert(0, str(entry))

import bpy  # noqa: E402

import mesh_tools  # noqa: E402  (v003)
import params as P  # noqa: E402  (v004)

if not Path(P.__file__).resolve().is_relative_to(V004_DIR):
    raise SystemExit(f"params resolved to {P.__file__}, expected the v004 package")
mesh_tools.COLLECTION_NAME = P.COLLECTION_NAME  # v003 helper, v004 collection

from anatomy import build_free_hand, build_neck  # noqa: E402  (v003, unchanged)
from body import build_legs, build_torso  # noqa: E402
from clothing import build_collar  # noqa: E402  (v003, unchanged)
from face import build_face, build_hair  # noqa: E402
from garments import (  # noqa: E402
    build_belt,
    build_boots,
    build_coat,
    build_half_cape,
    build_pauldron,
    build_sleeves,
)
from mathutils import Vector  # noqa: E402
from camera import use_camera  # noqa: E402
from presentation import Shot, aim_camera  # noqa: E402
from render import configure  # noqa: E402
from staging import (  # noqa: E402  (v003 cameras and light rig)
    MASTER_AZIMUTH,
    build_light_rig,
    image_path,
    render_master,
    render_shot,
    setup_world,
    shots,
)
from surfaces import use_toon  # noqa: E402
from sword import blade_point, build_blade, build_sword  # noqa: E402  (v003, unchanged)
from validate import check_scene, write_report  # noqa: E402

PREVIEW_DIR = P.OUT_DIR / "preview"
PREVIEW_SUPERSAMPLE = 8  # 512 px instead of 1024
# New in v004: torso/waistcoat close-up (the v003 shots are kept as they were).
EXTRA_SHOTS = {
    "close_torso": Shot((0.04, 0.0, 1.17), -24.0, 4.0, 10.0, 0.62),
    "close_face_front": Shot((0.06, 0.0, 1.66), 0.0, 2.0, 10.0, 0.27),
    "close_face_profile": Shot((0.03, 0.0, 1.66), -90.0, 2.0, 10.0, 0.27),
}


def parse_args() -> argparse.Namespace:
    argv = sys.argv[sys.argv.index("--") + 1 :] if "--" in sys.argv else []
    parser = argparse.ArgumentParser(prog="build_swordsman_v004.py")
    parser.add_argument("--overwrite", action="store_true", help="replace existing v004 outputs")
    parser.add_argument("--no-render", action="store_true", help="only build, validate and save")
    parser.add_argument("--preview", help="comma-separated views; renders only those, no .blend saved")
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
    # Readability variant of the blade (x1.7 width, same length), hidden by default.
    build_blade(P.READABLE_BLADE_SCALE, suffix="_readable")
    readable = bpy.data.objects["blade_readable"]
    readable.hide_render = True
    readable.hide_set(True)


def set_readable_blade(enabled: bool) -> None:
    bpy.data.objects["blade"].hide_render = enabled
    bpy.data.objects["blade_readable"].hide_render = not enabled


def all_shots() -> dict[str, Shot]:
    return {**shots(blade_point(0.45)), **EXTRA_SHOTS}


def render_view(view: str, path: Path) -> Path:
    """Renders one named view: masters, shots and the toon/readable variants."""
    base = view.removesuffix("_toon").removesuffix("_readable")
    toon = "_toon" in view
    readable = "_readable" in view
    use_toon(toon)
    set_readable_blade(readable)
    try:
        if base.endswith("_master"):
            return render_master(base.removesuffix("_master"), path)
        shot = all_shots().get(base)
        if shot is None:
            raise SystemExit(f"unknown view: {view}")
        return render_shot(base, shot, path)
    finally:
        use_toon(False)
        set_readable_blade(False)


def stage_cameras() -> None:
    """Creates every review camera before saving, so the .blend carries them."""
    for view, shot in all_shots().items():
        aim_camera(f"cam_{view}", shot)
    for view in MASTER_AZIMUTH:
        use_camera(view)
    use_camera("side")  # active camera: the game view


def views() -> list[str]:
    names = [f"{v}_master" for v in MASTER_AZIMUTH]
    names += list(all_shots())
    names += ["side_master_toon", "front_master_toon", "side_master_readable", "side_master_readable_toon"]
    return names


def main() -> None:
    args = parse_args()
    print(f"[v004] Blender {bpy.app.version_string}")
    preview = [v for v in (args.preview or "").split(",") if v]
    if not preview:
        refuse_existing([P.BLEND_PATH, image_path("side_master")], args.overwrite)
    clear_startup_scene()
    build_model()
    setup_world()
    build_light_rig()
    engine = configure(PREVIEW_SUPERSAMPLE) if preview else configure()
    print(f"[v004] render engine: {engine}")
    if preview:
        for view in preview:
            print(f"[v004] preview {render_view(view, PREVIEW_DIR / f'{view}.png')}")
        return
    checks = check_scene()
    for failure in checks["failures"]:
        print(f"[v004] FAIL {failure}")
    for warning in checks["warnings"]:
        print(f"[v004] warning {warning}")
    print(f"[v004] {checks['objects']} objects, {checks['triangles_rendered']} rendered triangles, "
          f"top {checks['top_m']} m, sole {checks['bottom_m']} m")
    print(f"[v004] intersections: {checks['intersections']}")
    stage_cameras()
    P.BLEND_PATH.parent.mkdir(parents=True, exist_ok=True)
    bpy.ops.wm.save_as_mainfile(filepath=str(P.BLEND_PATH))
    print(f"[v004] saved {P.BLEND_PATH}")
    written = [] if args.no_render else [render_view(v, image_path(v)) for v in views()]
    for path in written:
        print(f"[v004] rendered {path}")
    write_report(P.OUT_DIR / f"build_report_{P.MODEL_VERSION}.json", checks,
                 {"renders": [p.name for p in written], "readable_blade_scale": P.READABLE_BLADE_SCALE})
    if checks["failures"]:
        raise SystemExit(f"{len(checks['failures'])} validation failure(s)")


if __name__ == "__main__":
    main()
