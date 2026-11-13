"""Render settings and still renders: transparent RGBA PNG, sRGB, Standard.

The engine is EEVEE under whichever name the installed Blender uses
(BLENDER_EEVEE in 4.0-4.1 and 5.x, BLENDER_EEVEE_NEXT in 4.2-4.x).
Workbench ignores the lights and changes the look, so it is used only when
explicitly allowed.

Validated in Blender 5.2.4 LTS (V1.2-B).
"""

from __future__ import annotations

from pathlib import Path

import bpy

from config import FRAME_PX, SUPERSAMPLE

ENGINE_PREFERENCE = ("BLENDER_EEVEE_NEXT", "BLENDER_EEVEE")
FALLBACK_ENGINE = "BLENDER_WORKBENCH"


def available_engines() -> set[str]:
    prop = bpy.types.RenderSettings.bl_rna.properties["engine"]
    return {item.identifier for item in prop.enum_items}


def configure(supersample: int = SUPERSAMPLE, allow_workbench: bool = False) -> str:
    scene = bpy.context.scene
    engines = available_engines()
    engine = next((e for e in ENGINE_PREFERENCE if e in engines), None)
    if engine is None and allow_workbench and FALLBACK_ENGINE in engines:
        engine = FALLBACK_ENGINE
    if engine is None:
        raise RuntimeError(
            f"EEVEE not available (engines: {sorted(engines)}); pass --allow-workbench to accept "
            "a different look"
        )
    scene.render.engine = engine
    scene.render.film_transparent = True
    scene.render.resolution_x = FRAME_PX * supersample
    scene.render.resolution_y = FRAME_PX * supersample
    scene.render.resolution_percentage = 100
    settings = scene.render.image_settings
    settings.file_format = "PNG"
    settings.color_mode = "RGBA"
    settings.color_depth = "8"
    # "Standard" avoids the extra tone shift of Filmic/AgX; lighting still
    # changes the authored palette colours, which post/pixelize.py snaps back.
    try:
        scene.view_settings.view_transform = "Standard"
        scene.view_settings.look = "None"
    except TypeError as error:
        print(f"[render] warning: could not set Standard view transform: {error}")
    scene.display_settings.display_device = "sRGB"
    return engine


def render_still(path: Path) -> Path:
    path.parent.mkdir(parents=True, exist_ok=True)
    bpy.context.scene.render.filepath = str(path)
    bpy.ops.render.render(write_still=True)
    return path
