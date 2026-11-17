"""Shared render settings for D13 exports."""
from __future__ import annotations

import bpy


def main() -> None:
    scene = bpy.context.scene
    try:
        scene.render.engine = "BLENDER_EEVEE_NEXT"
    except TypeError:
        try:
            scene.render.engine = "BLENDER_EEVEE"
        except TypeError:
            pass
    scene.render.resolution_x = 512
    scene.render.resolution_y = 640
    scene.render.image_settings.file_format = "PNG"
    scene.render.image_settings.color_mode = "RGBA"
    scene.render.film_transparent = True
    print("OK render", scene.render.engine, scene.render.resolution_x, scene.render.resolution_y)


if __name__ == "__main__":
    main()
