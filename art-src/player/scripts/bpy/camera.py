"""Orthographic cameras for the sprite views (side, front, back).

The ortho scale makes one frame pixel equal METRES_PER_PX, and the camera is
centred so the sole lands on the pivot line of the frame (PIVOT_PX).

Validated in Blender 5.2.4 LTS (V1.2-B).
"""

from __future__ import annotations

import math

import bpy

from config import CAMERA_DISTANCE_M, FRAME_PX, METRES_PER_PX, PIVOT_PX

# view -> (location direction, euler rotation). Default camera looks down -Z.
VIEWS = {
    # From -Y looking +Y: screen-right is +X (the character's front).
    "side": ((0.0, -1.0), (math.radians(90.0), 0.0, 0.0)),
    # From +X looking -X: the face.
    "front": ((1.0, 0.0), (math.radians(90.0), 0.0, math.radians(90.0))),
    # From -X looking +X: the back.
    "back": ((-1.0, 0.0), (math.radians(90.0), 0.0, math.radians(-90.0))),
}


def frame_centre_height() -> float:
    """World z of the frame centre: the sole sits PIVOT_PX[1] px from the top."""
    return (PIVOT_PX[1] - FRAME_PX / 2.0) * METRES_PER_PX


def frame_centre_offset() -> float:
    """Horizontal offset of the frame centre from the pivot (0 when centred)."""
    return (FRAME_PX / 2.0 - PIVOT_PX[0]) * METRES_PER_PX


def ensure_camera(view: str) -> bpy.types.Object:
    if view not in VIEWS:
        raise KeyError(f"unknown view: {view}")
    name = f"cam_{view}"
    obj = bpy.data.objects.get(name)
    if obj is None:
        data = bpy.data.cameras.new(name)
        obj = bpy.data.objects.new(name, data)
        bpy.context.scene.collection.objects.link(obj)
    (dx, dy), rotation = VIEWS[view]
    obj.data.type = "ORTHO"
    obj.data.ortho_scale = FRAME_PX * METRES_PER_PX
    obj.data.sensor_fit = "AUTO"
    offset = frame_centre_offset()
    # Offset is along the screen-right axis of each view.
    right = {"side": (1.0, 0.0), "front": (0.0, 1.0), "back": (0.0, -1.0)}[view]
    obj.location = (
        dx * CAMERA_DISTANCE_M + right[0] * offset,
        dy * CAMERA_DISTANCE_M + right[1] * offset,
        frame_centre_height(),
    )
    obj.rotation_euler = rotation
    return obj


def use_camera(view: str) -> bpy.types.Object:
    obj = ensure_camera(view)
    bpy.context.scene.camera = obj
    return obj
