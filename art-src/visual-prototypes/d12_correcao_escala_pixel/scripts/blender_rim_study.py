#!/usr/bin/env python3
"""Blender 5.x lab: rim-light stills for Pipeline B (does NOT modify originals).

Run:
  blender --background art-src/player/blender/swordsman_rig_v007_game_v1_5.blend \\
    --python art-src/visual-prototypes/d12_correcao_escala_pixel/scripts/blender_rim_study.py

Outputs PNGs under d12_correcao_escala_pixel/blender/ (clearly labelled Blender, not Bevy).
"""
from __future__ import annotations

import math
import sys
from pathlib import Path

try:
    import bpy
except ImportError:
    print("Run inside Blender: blender --background <blend> --python this_script.py")
    sys.exit(1)

LAB = Path(__file__).resolve().parents[1]
OUT = LAB / "blender"
OUT.mkdir(parents=True, exist_ok=True)


def ensure_ortho_camera() -> bpy.types.Object:
    cam = None
    for obj in bpy.data.objects:
        if obj.type == "CAMERA":
            cam = obj
            break
    if cam is None:
        bpy.ops.object.camera_add()
        cam = bpy.context.object
    cam.data.type = "ORTHO"
    cam.data.ortho_scale = 2.4
    cam.location = (0.0, -4.0, 1.0)
    cam.rotation_euler = (math.radians(90), 0.0, 0.0)
    bpy.context.scene.camera = cam
    return cam


def set_rim(intensity: float, name: str) -> None:
    # Remove previous lab rim
    for obj in list(bpy.data.objects):
        if obj.name.startswith("D12_Rim"):
            bpy.data.objects.remove(obj, do_unlink=True)
    if intensity <= 0.0:
        return
    light_data = bpy.data.lights.new(name=name, type="AREA")
    light_data.energy = intensity
    light_data.color = (0.75, 0.85, 1.0)
    light_data.size = 2.0
    light = bpy.data.objects.new(name, light_data)
    bpy.context.scene.collection.objects.link(light)
    light.location = (-1.2, 1.5, 2.2)
    light.rotation_euler = (math.radians(-35), math.radians(-25), 0.0)


def render_variant(tag: str, energy: float) -> Path:
    scene = bpy.context.scene
    scene.render.engine = "BLENDER_EEVEE"
    # Blender 5 may use EEVEE_NEXT — fall back.
    if hasattr(bpy.types, "RenderEngine") or True:
        try:
            scene.render.engine = "BLENDER_EEVEE_NEXT"
        except Exception:
            scene.render.engine = "BLENDER_EEVEE"
    scene.render.resolution_x = 256
    scene.render.resolution_y = 320
    scene.render.resolution_percentage = 100
    scene.render.image_settings.file_format = "PNG"
    scene.render.image_settings.color_mode = "RGBA"
    scene.render.film_transparent = True
    ensure_ortho_camera()
    set_rim(energy, f"D12_Rim_{tag}")
    path = OUT / f"rim_{tag}.png"
    scene.render.filepath = str(path)
    bpy.ops.render.render(write_still=True)
    print("wrote", path)
    return path


def main() -> None:
    # Document engines available
    meta = OUT / "blender_session.txt"
    engines = []
    for eng in ("BLENDER_EEVEE", "BLENDER_EEVEE_NEXT", "CYCLES", "BLENDER_WORKBENCH"):
        engines.append(eng)
    meta.write_text(
        "D12 Blender rim study\n"
        f"blender={bpy.app.version_string}\n"
        f"file={bpy.data.filepath}\n"
        f"engines_tried={engines}\n"
        "NOTE: EEVEE stills are comparison only; not Bevy evidence.\n"
        "D3–D6 already showed EEVEE/toon does not auto-recover D2 quality.\n",
        encoding="utf-8",
    )
    render_variant("A_none", 0.0)
    render_variant("B_soft", 80.0)
    render_variant("C_dramatic", 220.0)


if __name__ == "__main__":
    main()
