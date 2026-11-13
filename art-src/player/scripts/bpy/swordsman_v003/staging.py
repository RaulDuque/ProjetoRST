"""World, light rig, cameras and renders of v003.

Light rig: key, fill and two rims, defined for the side view and parented to
an empty that turns with each camera. Every view (front, side, back, 3/4,
close-ups) therefore gets the same light relative to the camera, so the
back view is not darker just because the key favours the profile. A dim
neutral world fills the shadows and gives the metal something to reflect.

Sprite masters reuse ``camera.py`` (contract scale and pivot); close-ups and
the 3/4 view reuse ``presentation.Shot`` / ``aim_camera``.
"""

from __future__ import annotations

import math
from pathlib import Path

import bpy
from mathutils import Vector

import params as P
from camera import use_camera
from presentation import Shot, aim_camera
from render import render_still

SIDE_AZIMUTH = -90.0  # the side camera sits on -Y
WORLD_COLOR = (0.045, 0.045, 0.05, 1.0)

# name -> (direction the light comes FROM, in the side-view frame; energy; colour; shadow)
LIGHTS = {
    "key": ((0.55, -0.75, 0.65), 3.2, (1.0, 0.95, 0.88), True),
    "fill": ((-0.35, -0.9, 0.05), 0.9, (0.78, 0.84, 1.0), False),
    "rim_back": ((-0.75, 0.6, 0.45), 3.6, (0.72, 0.8, 1.0), False),
    "rim_front": ((0.75, 0.6, 0.3), 2.0, (0.75, 0.8, 1.0), False),
}

def setup_world() -> None:
    world = bpy.context.scene.world
    if world is None:
        world = bpy.data.worlds.new("v003_world")
        bpy.context.scene.world = world
    if hasattr(world, "use_nodes"):
        world.use_nodes = True
    background = next((n for n in world.node_tree.nodes if n.type == "BACKGROUND"), None)
    if background is None:
        raise RuntimeError("world Background node not found")
    background.inputs["Color"].default_value = WORLD_COLOR
    background.inputs["Strength"].default_value = 1.0


def build_light_rig() -> bpy.types.Object:
    rig = bpy.data.objects.get("light_rig")
    if rig is None:
        rig = bpy.data.objects.new("light_rig", None)
        bpy.context.scene.collection.objects.link(rig)
    for name, (source, energy, rgb, shadow) in LIGHTS.items():
        obj_name = f"v3_light_{name}"
        obj = bpy.data.objects.get(obj_name)
        if obj is None:
            data = bpy.data.lights.new(obj_name, type="SUN")
            obj = bpy.data.objects.new(obj_name, data)
            bpy.context.scene.collection.objects.link(obj)
        obj.data.energy = energy
        obj.data.color = rgb
        if hasattr(obj.data, "use_shadow"):
            obj.data.use_shadow = shadow
        if hasattr(obj.data, "angle"):
            obj.data.angle = math.radians(6.0)
        travel = -Vector(source).normalized()
        obj.rotation_euler = travel.to_track_quat("-Z", "Y").to_euler()
        obj.parent = rig
    return rig


def turn_rig(view_azimuth: float) -> None:
    rig = bpy.data.objects["light_rig"]
    rig.rotation_euler = (0.0, 0.0, math.radians(view_azimuth - SIDE_AZIMUTH))
    bpy.context.view_layer.update()


MASTER_AZIMUTH = {"side": -90.0, "front": 0.0, "back": 180.0}


def shots(sword_mid: Vector) -> dict[str, Shot]:
    return {
        "three_quarter": Shot((0.05, 0.0, 0.93), -40.0, 8.0, 4.7, 85.0, perspective=True),
        "close_face": Shot((0.06, 0.0, 1.668), -28.0, 3.0, 10.0, 0.30),
        "close_hair": Shot((-0.005, 0.0, 1.71), -128.0, 16.0, 10.0, 0.36),
        "close_coat": Shot((0.03, 0.0, 1.0), -32.0, 4.0, 10.0, 1.2),
        "close_cape": Shot((-0.06, 0.05, 1.22), 140.0, 12.0, 10.0, 0.82),
        "close_boots": Shot((0.02, 0.0, 0.21), -55.0, 8.0, 10.0, 0.6),
        "close_sword": Shot(tuple(sword_mid), -90.0, 0.0, 10.0, 1.08),
    }


def image_path(view: str) -> Path:
    return P.OUT_DIR / f"{P.MODEL_NAME}_{P.MODEL_VERSION}_{view}.png"


def render_master(view: str, path: Path) -> Path:
    use_camera(view)
    turn_rig(MASTER_AZIMUTH[view])
    return render_still(path)


def render_shot(view: str, shot: Shot, path: Path) -> Path:
    bpy.context.scene.camera = aim_camera(f"cam_{view}", shot)
    turn_rig(shot.azimuth)
    return render_still(path)
