"""Fixed three-light setup: key, fill and rim. No cast shadows.

The rim light separates the dark coat from the dark castle background, the
main readability problem found in the V1.1 comparisons. Lights are fixed in
world space and tuned for the side view (the game view); in the front and
back turnaround views the rim no longer acts as a rim.

Validated in Blender 5.2.4 LTS (V1.2-B).
"""

from __future__ import annotations

import math

import bpy

# name -> (energy, euler rotation in degrees, colour)
LIGHTS = {
    "key": (3.0, (50.0, 0.0, 60.0), (1.0, 0.96, 0.9)),
    "fill": (0.8, (70.0, 0.0, -60.0), (0.75, 0.8, 1.0)),
    # Rx(60) then Rz(180): light travels toward -Y and downward, i.e. it comes
    # from behind (+Y) and above the character for the side view.
    "rim": (4.0, (60.0, 0.0, 180.0), (0.7, 0.78, 1.0)),
}


def build_lights() -> None:
    for name, (energy, rotation, rgb) in LIGHTS.items():
        obj_name = f"light_{name}"
        obj = bpy.data.objects.get(obj_name)
        if obj is None:
            data = bpy.data.lights.new(obj_name, type="SUN")
            obj = bpy.data.objects.new(obj_name, data)
            bpy.context.scene.collection.objects.link(obj)
        obj.data.energy = energy
        obj.data.color = rgb
        if hasattr(obj.data, "use_shadow"):
            obj.data.use_shadow = False
        obj.rotation_euler = tuple(math.radians(v) for v in rotation)
