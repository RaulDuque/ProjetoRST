"""Evaluation views beyond the sprite turnaround: 3/4 perspective and details.

Same lights as the sprite views (``lighting.py``) and transparent film, so
every image of a model version shares the lighting; the neutral background is
added afterwards by ``post/model_sheet.py``. These are art-review images, not
sprite masters: their scale is not tied to the frame contract.

Validated in Blender 5.2.4 LTS (V1.2-B).
"""

from __future__ import annotations

import math
from dataclasses import dataclass
from pathlib import Path

import bpy
from mathutils import Vector

from config import MODEL_NAME, MODEL_VERSION, RENDERS_DIR
from geometry import SWORD_HAND, SWORD_TIP
from render import render_still

Vec3 = tuple[float, float, float]


@dataclass(frozen=True)
class Shot:
    """Camera aimed at ``target`` from ``azimuth`` degrees around Z.

    Azimuth uses the geometry convention: 0 = front (+X), -90 = near side
    (-Y), 180 = back. ``size`` is the ortho scale (m) or, for perspective
    shots, the focal length (mm).
    """

    target: Vec3
    azimuth: float
    elevation: float
    distance: float
    size: float
    perspective: bool = False


SWORD_MID: Vec3 = tuple((a + b) / 2.0 for a, b in zip(SWORD_HAND, SWORD_TIP))

SHOTS = {
    "three_quarter": Shot((0.05, 0.0, 0.93), -40.0, 8.0, 4.7, 85.0, perspective=True),
    "detail_collar": Shot((0.0, 0.0, 1.60), -35.0, 8.0, 10.0, 0.55),
    "detail_coat": Shot((0.05, 0.0, 1.05), -20.0, 5.0, 10.0, 0.95),
    "detail_cape": Shot((-0.12, 0.05, 1.25), 140.0, 10.0, 10.0, 0.85),
    "detail_sword": Shot(SWORD_MID, -90.0, 0.0, 10.0, 1.15),
}


def presentation_path(view: str, out_dir: Path = RENDERS_DIR) -> Path:
    return out_dir / f"{MODEL_NAME}_{MODEL_VERSION}_{view}.png"


def aim_camera(name: str, shot: Shot) -> bpy.types.Object:
    obj = bpy.data.objects.get(name)
    if obj is None:
        data = bpy.data.cameras.new(name)
        obj = bpy.data.objects.new(name, data)
        bpy.context.scene.collection.objects.link(obj)
    az, el = math.radians(shot.azimuth), math.radians(shot.elevation)
    direction = Vector((math.cos(az) * math.cos(el), math.sin(az) * math.cos(el), math.sin(el)))
    target = Vector(shot.target)
    obj.location = target + direction * shot.distance
    obj.rotation_euler = (target - obj.location).to_track_quat("-Z", "Y").to_euler()
    if shot.perspective:
        obj.data.type = "PERSP"
        obj.data.lens = shot.size
    else:
        obj.data.type = "ORTHO"
        obj.data.ortho_scale = shot.size
    obj.data.clip_end = shot.distance * 3.0
    return obj


def render_presentation(out_dir: Path = RENDERS_DIR) -> list[Path]:
    written = []
    for view, shot in SHOTS.items():
        bpy.context.scene.camera = aim_camera(f"cam_{view}", shot)
        written.append(render_still(presentation_path(view, out_dir)))
    return written
