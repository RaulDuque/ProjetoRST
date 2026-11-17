"""D13 lighting studies A soft / B gothic+rim / C dramatic metal."""
from __future__ import annotations

import math

import bpy
from mathutils import Euler


def _clear() -> None:
    for o in list(bpy.data.objects):
        if o.name.startswith("D13_light_"):
            bpy.data.objects.remove(o, do_unlink=True)


def _mute_stock(mute: bool = True) -> None:
    for o in bpy.data.objects:
        if o.type == "LIGHT" and not o.name.startswith("D13_light_"):
            o.hide_render = mute
            o.hide_viewport = mute


def _area(name, loc, rot_deg, energy, color, size=2.0):
    data = bpy.data.lights.new(name, type="AREA")
    data.energy = energy
    data.color = color
    data.size = size
    obj = bpy.data.objects.new(name, data)
    bpy.context.scene.collection.objects.link(obj)
    obj.location = loc
    obj.rotation_euler = Euler(tuple(math.radians(a) for a in rot_deg), "XYZ")
    return obj


def apply(mode: str = "B") -> None:
    _clear()
    _mute_stock(True)
    if mode == "A":
        _area("D13_light_key", (1.6, -2.2, 2.0), (-50, -20, 0), 40, (1.0, 0.96, 0.92), 3.0)
        _area("D13_light_fill", (-1.8, -1.5, 1.4), (-40, 25, 0), 12, (0.75, 0.82, 1.0), 3.5)
    elif mode == "B":
        _area("D13_light_key", (1.4, -2.0, 1.8), (-45, -15, 0), 55, (1.0, 0.93, 0.88), 2.4)
        _area("D13_light_fill", (-1.6, -1.2, 1.2), (-35, 20, 0), 8, (0.55, 0.65, 0.9), 3.0)
        _area("D13_light_rim", (-1.2, 1.6, 2.1), (-145, -30, 0), 70, (0.7, 0.82, 1.0), 1.8)
    else:
        _area("D13_light_key", (1.5, -2.1, 1.9), (-48, -18, 0), 65, (1.0, 0.95, 0.9), 2.0)
        _area("D13_light_fill", (-1.7, -1.3, 1.3), (-35, 22, 0), 6, (0.5, 0.6, 0.85), 2.8)
        _area("D13_light_rim", (-1.0, 1.8, 2.3), (-150, -25, 0), 110, (0.75, 0.85, 1.0), 1.5)
        _area("D13_light_metal", (0.8, -2.5, 1.2), (-30, 0, 0), 25, (0.9, 0.95, 1.0), 1.2)
    print("OK lights", mode)


if __name__ == "__main__":
    apply("B")
