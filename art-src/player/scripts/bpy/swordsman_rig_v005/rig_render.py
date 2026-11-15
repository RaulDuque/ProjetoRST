"""Renders of v005: review views, contract-scale masters and the bone diagram.

Same light rig, world, engine and colour settings as v003/v004 (reused from
``staging``/``render``), so v004 x v005 images are comparable.

- review views: fixed orthographic side camera large enough for every pose
  (and a 3/4 perspective), for the validation sheets;
- contract masters: ``camera.use_camera`` (64x64 locomotion cell, pivot
  (32, 63), 16x supersampling) and a combat-cell camera (128x96, pivot
  (40, 95), 8x supersampling) built from the same contract numbers;
- bone diagram: Blender does not render armatures, so each bone becomes a
  temporary octahedron mesh with a flat colour per bone collection, rendered
  alone; label positions are projected to pixels for ``post/rig_sheets.py``.
"""

from __future__ import annotations

from pathlib import Path

import bpy
from bpy_extras.object_utils import world_to_camera_view
from mathutils import Matrix, Vector

import rig_config as C
from camera import use_camera
from config import CAMERA_DISTANCE_M, METRES_PER_PX, SUPERSAMPLE
from contract import COMBAT_FRAME as COMBAT_CELL
from contract import COMBAT_PIVOT
from presentation import Shot, aim_camera
from render import configure, render_still
from staging import turn_rig

REVIEW = {
    "side": Shot((0.25, 0.0, 1.1), -90.0, 0.0, 10.0, 3.3),
    "three_quarter": Shot((0.2, 0.0, 1.05), -40.0, 8.0, 7.0, 50.0, perspective=True),
    "front": Shot((0.0, 0.0, 1.1), 0.0, 0.0, 10.0, 3.3),
}
COMBAT_SUPERSAMPLE = 8


def set_resolution(width: int, height: int) -> None:
    r = bpy.context.scene.render
    r.resolution_x, r.resolution_y, r.resolution_percentage = width, height, 100


def render_review(view: str, path: Path, size: int = 512) -> Path:
    shot = REVIEW[view]
    set_resolution(size, size)
    bpy.context.scene.camera = aim_camera(f"cam_review_{view}", shot)
    turn_rig(shot.azimuth)
    return render_still(path)


def render_shot(name: str, shot: Shot, path: Path, size: int = 512) -> Path:
    set_resolution(size, size)
    bpy.context.scene.camera = aim_camera(f"cam_{name}", shot)
    turn_rig(shot.azimuth)
    return render_still(path)


def render_contract_side(path: Path) -> Path:
    configure()  # 1024 x 1024 = 64 px cell x 16
    use_camera("side")
    turn_rig(-90.0)
    return render_still(path)


def combat_camera() -> bpy.types.Object:
    """Side camera of the 128x96 combat cell; the pivot lands on the sole line."""
    name = "cam_combat_side"
    obj = bpy.data.objects.get(name)
    if obj is None:
        obj = bpy.data.objects.new(name, bpy.data.cameras.new(name))
        bpy.context.scene.collection.objects.link(obj)
    w, h = COMBAT_CELL
    obj.data.type = "ORTHO"
    obj.data.sensor_fit = "HORIZONTAL"
    obj.data.ortho_scale = w * METRES_PER_PX
    centre_x = (w / 2.0 - COMBAT_PIVOT[0]) * METRES_PER_PX
    centre_z = (COMBAT_PIVOT[1] - h / 2.0) * METRES_PER_PX
    obj.location = (centre_x, -CAMERA_DISTANCE_M, centre_z)
    obj.rotation_euler = (1.5707963267948966, 0.0, 0.0)
    return obj


def render_combat_side(path: Path) -> Path:
    configure()
    w, h = COMBAT_CELL
    set_resolution(w * COMBAT_SUPERSAMPLE, h * COMBAT_SUPERSAMPLE)
    bpy.context.scene.camera = combat_camera()
    turn_rig(-90.0)
    out = render_still(path)
    configure()
    return out


# --- bone diagram ------------------------------------------------------------------

VIZ_COLOURS = {  # linear RGB, flat emission
    "DEF": (0.55, 0.62, 0.78), "CTRL": (0.2, 0.75, 0.3), "COAT": (0.75, 0.2, 0.28),
    "CAPE": (0.95, 0.45, 0.1), "SWORD": (0.95, 0.75, 0.15),
}


def _flat(name: str, rgb: tuple[float, float, float]) -> bpy.types.Material:
    mat = bpy.data.materials.get(name) or bpy.data.materials.new(name)
    if hasattr(mat, "use_nodes"):
        mat.use_nodes = True
    tree = mat.node_tree
    for node in list(tree.nodes):
        tree.nodes.remove(node)
    out = tree.nodes.new("ShaderNodeOutputMaterial")
    em = tree.nodes.new("ShaderNodeEmission")
    em.inputs["Color"].default_value = (*rgb, 1.0)
    em.inputs["Strength"].default_value = 1.0
    tree.links.new(em.outputs[0], out.inputs["Surface"])
    return mat


def build_bone_viz(arm: bpy.types.Object) -> list[bpy.types.Object]:
    coll = bpy.data.collections.get(C.VIZ_COLLECTION) or bpy.data.collections.new(C.VIZ_COLLECTION)
    if coll.name not in bpy.context.scene.collection.children:
        bpy.context.scene.collection.children.link(coll)
    objs = []
    for bone in arm.data.bones:
        group = bone.collections[0].name if bone.collections else "DEF"
        length = bone.length
        w = min(max(length * 0.12, 0.008), 0.03)
        verts = [(0.0, 0.0, 0.0), (w, length * 0.2, 0.0), (0.0, length * 0.2, w), (-w, length * 0.2, 0.0),
                 (0.0, length * 0.2, -w), (0.0, length, 0.0)]
        faces = [(0, 1, 2), (0, 2, 3), (0, 3, 4), (0, 4, 1), (5, 2, 1), (5, 3, 2), (5, 4, 3), (5, 1, 4)]
        mesh = bpy.data.meshes.new(f"viz_{bone.name}")
        mesh.from_pydata(verts, [], faces)
        mesh.materials.append(_flat(f"viz_{group}", VIZ_COLOURS.get(group, (1, 1, 1))))
        obj = bpy.data.objects.new(f"viz_{bone.name}", mesh)
        obj.matrix_world = arm.matrix_world @ bone.matrix_local @ Matrix.Identity(4)
        coll.objects.link(obj)
        objs.append(obj)
    return objs


def remove_bone_viz(objs: list[bpy.types.Object]) -> None:
    for obj in objs:
        mesh = obj.data
        bpy.data.objects.remove(obj, do_unlink=True)
        bpy.data.meshes.remove(mesh)
    coll = bpy.data.collections.get(C.VIZ_COLLECTION)
    if coll is not None:
        bpy.data.collections.remove(coll)
    for name in list(VIZ_COLOURS):
        mat = bpy.data.materials.get(f"viz_{name}")
        if mat is not None:
            bpy.data.materials.remove(mat)


def label_positions(arm: bpy.types.Object, view: str, size: int) -> dict[str, tuple[float, float]]:
    """Pixel position (from the top-left) of each bone's midpoint in a review view."""
    cam = bpy.data.objects[f"cam_review_{view}"]
    scene = bpy.context.scene
    out = {}
    for bone in arm.data.bones:
        mid = arm.matrix_world @ ((bone.head_local + bone.tail_local) / 2.0)
        co = world_to_camera_view(scene, cam, Vector(mid))
        out[bone.name] = (round(co.x * size, 1), round((1.0 - co.y) * size, 1))
    return out


def set_visible(objs: list[bpy.types.Object], visible: bool) -> None:
    for obj in objs:
        obj.hide_render = not visible


def set_readable_blade(enabled: bool) -> None:
    bpy.data.objects["blade"].hide_render = enabled
    bpy.data.objects["blade_readable"].hide_render = not enabled


__all__ = ["SUPERSAMPLE"]
