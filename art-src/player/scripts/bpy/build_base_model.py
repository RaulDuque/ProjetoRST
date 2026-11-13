"""Entry point: builds the swordsman base model, saves it and renders previews.

Run from the project root (Blender must be installed):

    blender --background --factory-startup \
        --python art-src/player/scripts/bpy/build_base_model.py -- [--overwrite] [--no-render]

Then, with the system Python (Pillow):

    python3 art-src/player/scripts/post/pixelize.py \
        art-src/player/renders/swordsman_base_v001_side_master.png \
        --out-dir art-src/player/renders --name swordsman_base_v001_side

Safety: runs only on an unsaved file (``--factory-startup``), writes only to
art-src/player/blender/ and art-src/player/renders/, and refuses to overwrite
an existing .blend, master or presentation view of the same MODEL_VERSION
without --overwrite.
EEVEE is required unless --allow-workbench is given.

Validated in Blender 5.2.4 LTS (V1.2-B).
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

SCRIPTS_DIR = Path(__file__).resolve().parent
if str(SCRIPTS_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPTS_DIR))

import bpy  # noqa: E402

from config import BLEND_DIR, MODEL_NAME, MODEL_VERSION, RENDERS_DIR  # noqa: E402
from geometry import build_all  # noqa: E402
from lighting import build_lights  # noqa: E402
from presentation import SHOTS, presentation_path, render_presentation  # noqa: E402
from previews import VIEW_ORDER, master_path, render_turnaround  # noqa: E402
from render import configure  # noqa: E402


def parse_args() -> argparse.Namespace:
    argv = sys.argv[sys.argv.index("--") + 1 :] if "--" in sys.argv else []
    parser = argparse.ArgumentParser(prog="build_base_model.py")
    parser.add_argument("--overwrite", action="store_true", help="replace an existing .blend")
    parser.add_argument("--no-render", action="store_true", help="only build and save the model")
    parser.add_argument("--allow-workbench", action="store_true",
                        help="render with Workbench if EEVEE is unavailable (different look)")
    return parser.parse_args(argv)


def clear_startup_scene() -> None:
    """Removes the factory-startup objects (cube, camera, light) only."""
    if bpy.data.filepath:
        raise SystemExit(
            f"refusing to clear a saved file ({bpy.data.filepath}); run with --factory-startup"
        )
    for obj in list(bpy.context.scene.objects):
        bpy.data.objects.remove(obj, do_unlink=True)


def main() -> None:
    args = parse_args()
    blend_path = BLEND_DIR / f"{MODEL_NAME}_{MODEL_VERSION}.blend"
    outputs = [blend_path, *(master_path(v) for v in VIEW_ORDER),
               *(presentation_path(v) for v in SHOTS)]
    existing = [p for p in outputs if p.exists()]
    if existing and not args.overwrite:
        names = ", ".join(str(p) for p in existing)
        raise SystemExit(f"already exists: {names}; bump MODEL_VERSION or pass --overwrite")

    version = bpy.app.version_string
    print(f"[build_base_model] Blender {version}")
    clear_startup_scene()
    build_all()
    build_lights()
    engine = configure(allow_workbench=args.allow_workbench)
    print(f"[build_base_model] render engine: {engine}")

    BLEND_DIR.mkdir(parents=True, exist_ok=True)
    bpy.ops.wm.save_as_mainfile(filepath=str(blend_path))
    print(f"[build_base_model] saved {blend_path}")

    RENDERS_DIR.mkdir(parents=True, exist_ok=True)
    record = RENDERS_DIR / f"blender_version_{MODEL_VERSION}.txt"
    record.write_text(f"{version}\n{engine}\n", encoding="utf-8")
    if not args.no_render:
        for path in [*render_turnaround(), *render_presentation()]:
            print(f"[build_base_model] rendered {path}")


if __name__ == "__main__":
    main()
