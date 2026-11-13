"""Low-detail geometry of the swordsman base model (concept B), neutral pose.

Built only from primitives so the script is reproducible and has no external
assets. Priority is silhouette and proportion, not detail. Every object goes
into the ``swordsman_base`` collection; nothing outside it is touched.

Axes: +X front, -Y is the side nearer to the side camera (sword arm),
+Z up, sole on z = 0. Limbs are segments in an XZ plane.

Validated in Blender 5.2.4 LTS (V1.2-B, headless ``--factory-startup``).
"""

from __future__ import annotations

import math
from collections.abc import Callable

import bmesh
import bpy

from config import COLLECTION_NAME, HEAD_M
from materials import assign, palette_material

Vec3 = tuple[float, float, float]

NEAR = -0.17  # y of the near (sword) side
FAR = 0.17
SEGMENTS = 24
SHELL_THICKNESS = 0.012  # cloth shells (coat, collar, half-cape), metres


def collection() -> bpy.types.Collection:
    coll = bpy.data.collections.get(COLLECTION_NAME)
    if coll is None:
        coll = bpy.data.collections.new(COLLECTION_NAME)
        bpy.context.scene.collection.children.link(coll)
    return coll


def _finish(name: str, material: str) -> bpy.types.Object:
    obj = bpy.context.active_object
    obj.name = name
    for owner in list(obj.users_collection):
        owner.objects.unlink(obj)
    collection().objects.link(obj)
    assign(obj, material)
    smooth(obj)
    return obj


def smooth(obj: bpy.types.Object) -> None:
    """Smooth shading on curved sides; caps (n-gons) and boxes stay flat."""
    if len(obj.data.vertices) <= 8:
        return
    for poly in obj.data.polygons:
        poly.use_smooth = len(poly.vertices) <= 4


def cut_faces(obj: bpy.types.Object, keep: Callable[[float], bool]) -> None:
    """Deletes the faces whose centre angle around local Z fails ``keep``.

    Angles are in degrees from +X (front), counter-clockwise: +90 is the far
    side (+Y), 180 the back, -90 the near (sword) side.
    """
    bm = bmesh.new()
    bm.from_mesh(obj.data)
    dead = []
    for face in bm.faces:
        centre = face.calc_center_median()
        if not keep(math.degrees(math.atan2(centre.y, centre.x))):
            dead.append(face)
    bmesh.ops.delete(bm, geom=dead, context="FACES")
    bm.to_mesh(obj.data)
    bm.free()
    obj.data.update()


def shell(name: str, bottom: float, top: float, z0: float, z1: float, xy: tuple[float, float],
          outer: str, inner: str, keep: Callable[[float], bool],
          squash_y: float = 1.0) -> bpy.types.Object:
    """Open cloth shell: cone side without caps, cut by angle, given thickness.

    The Solidify modifier grows the shell inward and puts the ``inner``
    material on the inside faces and on the cut edges, so an open coat shows
    its wine lining along the opening.
    """
    depth = z1 - z0
    bpy.ops.mesh.primitive_cone_add(radius1=bottom, radius2=top, depth=depth,
                                    vertices=SEGMENTS * 2, end_fill_type="NOTHING",
                                    location=(xy[0], xy[1], z0 + depth / 2.0))
    obj = _finish(name, outer)
    obj.data.materials.append(palette_material(inner))
    cut_faces(obj, keep)
    obj.scale = (1.0, squash_y, 1.0)
    mod = obj.modifiers.new("shell", "SOLIDIFY")
    mod.thickness = SHELL_THICKNESS
    mod.offset = -1.0
    mod.use_rim = True
    mod.material_offset = 1
    mod.material_offset_rim = 1
    return obj


def sphere(name: str, radius: float, loc: Vec3, scale: Vec3, material: str) -> bpy.types.Object:
    bpy.ops.mesh.primitive_uv_sphere_add(radius=radius, location=loc, segments=SEGMENTS, ring_count=12)
    obj = _finish(name, material)
    obj.scale = scale
    return obj


def box(name: str, size: Vec3, loc: Vec3, material: str, rot_y: float = 0.0) -> bpy.types.Object:
    bpy.ops.mesh.primitive_cube_add(size=1.0, location=loc, rotation=(0.0, rot_y, 0.0))
    obj = _finish(name, material)
    obj.scale = size
    return obj


def cone(name: str, bottom: float, top: float, z0: float, z1: float, xy: tuple[float, float],
         material: str, squash_y: float = 1.0) -> bpy.types.Object:
    depth = z1 - z0
    bpy.ops.mesh.primitive_cone_add(radius1=bottom, radius2=top, depth=depth, vertices=SEGMENTS,
                                    location=(xy[0], xy[1], z0 + depth / 2.0))
    obj = _finish(name, material)
    obj.scale = (1.0, squash_y, 1.0)
    return obj


def segment(name: str, a: Vec3, b: Vec3, radius: float, material: str) -> bpy.types.Object:
    """Cylinder from a to b; a and b must share y (limbs live in XZ planes)."""
    dx, dz = b[0] - a[0], b[2] - a[2]
    length = math.hypot(dx, dz)
    mid = ((a[0] + b[0]) / 2.0, a[1], (a[2] + b[2]) / 2.0)
    bpy.ops.mesh.primitive_cylinder_add(radius=radius, depth=length, vertices=16, location=mid,
                                        rotation=(0.0, math.atan2(dx, dz), 0.0))
    return _finish(name, material)


def build_body() -> None:
    h = HEAD_M
    # Head (~1 head tall, chin at 6 heads). The hair caps the skull, so the
    # hair top (not the skin) defines 7.0 * h, the FIGURE_HEIGHT_PX line.
    sphere("head", 0.105, (0.02, 0.0, 7.0 * h - 0.135), (1.0, 0.92, 1.2), "skin")
    sphere("hair", 0.125, (-0.015, 0.0, 7.0 * h - 0.125 * 0.6), (1.0, 0.92, 0.6), "hair")
    sphere("hair_nape", 0.085, (-0.06, 0.0, 1.70), (1.0, 1.05, 1.1), "hair")
    # Short fringe pushed forward past the brow (originality mark).
    sphere("hair_fringe", 0.06, (0.085, 0.0, 7.0 * h - 0.045), (1.1, 1.45, 0.5), "hair_light")
    # Eyes: one amber stroke per side, no pupil.
    for side, y in (("near", -0.04), ("far", 0.04)):
        box(f"eye_{side}", (0.012, 0.022, 0.008), (0.117, y, 7.0 * h - 0.14), "amber")
    segment("neck", (0.01, 0.0, 1.49), (0.01, 0.0, 1.61), 0.045, "skin_shadow")
    # High, stiff collar flaring upward, open in front (face stays visible).
    shell("collar", 0.085, 0.105, 1.45, 1.66, (-0.02, 0.0), "coat_light", "lining_wine",
          keep=lambda a: abs(a) > 55.0)
    # Chest under the open coat, belt and buckle.
    # Rounded torso: shoulders slope into the sleeves instead of a box edge.
    sphere("chest", 0.15, (0.02, 0.0, 1.32), (0.7, 1.15, 1.45), "coat_light")
    cone("belt", 0.165, 0.165, 1.075, 1.125, (0.02, 0.0), "boots", squash_y=0.9)
    box("buckle", (0.03, 0.05, 0.06), (0.19, 0.0, 1.10), "amber")
    # Long coat, open in front: the opening widens toward the hem (inverted V)
    # and the Solidify shell shows the wine lining. Silver hem band.
    hem = 1.4 * h  # mid-calf, as in the proportions table
    def coat_open(a: float) -> bool:
        return abs(a) > 28.0

    shell("coat", 0.30, 0.17, hem, 1.53, (-0.05, 0.0), "coat", "lining_wine",
          keep=coat_open, squash_y=0.85)
    shell("coat_hem", 0.305, 0.30, hem, hem + 0.03, (-0.05, 0.0), "silver_dark", "silver_dark",
          keep=coat_open, squash_y=0.86)
    # Asymmetric half-cape: far shoulder and back only, down to the hip.
    shell("half_cape", 0.24, 0.12, 1.02, 1.52, (-0.10, 0.05), "cape_wine", "cape_shadow",
          keep=lambda a: 35.0 < a <= 180.0 or a < -150.0, squash_y=1.1)
    sphere("pauldron", 0.07, (0.02, NEAR, 1.47), (1.0, 1.0, 0.6), "silver_dark")


def build_limbs() -> None:
    # Legs: near leg slightly forward, far leg slightly back; tall boots with cuffs.
    for side, y, x in (("near", -0.08, 0.06), ("far", 0.08, -0.05)):
        segment(f"thigh_{side}", (x, y, 1.05), (x, y, 0.50), 0.062, "trousers")
        segment(f"boot_{side}", (x, y, 0.55), (x, y, 0.04), 0.068, "boots")
        segment(f"boot_cuff_{side}", (x, y, 0.56), (x, y, 0.51), 0.074, "boot_cuff")
        box(f"foot_{side}", (0.22, 0.09, 0.07), (x + 0.05, y, 0.035), "boots")
    # Sword arm (near): shoulder -> elbow -> hand, held forward and low.
    segment("upper_arm_near", (0.02, NEAR, 1.47), (0.14, NEAR, 1.20), 0.045, "coat")
    segment("forearm_near", (0.14, NEAR, 1.20), (0.31, NEAR, 1.02), 0.04, "coat")
    sphere("glove_near", 0.05, (0.31, NEAR, 1.02), (1.0, 1.0, 1.0), "glove")
    # Far arm hanging.
    segment("upper_arm_far", (0.0, FAR, 1.47), (-0.02, FAR, 1.22), 0.045, "coat")
    segment("forearm_far", (-0.02, FAR, 1.22), (-0.02, FAR, 1.00), 0.04, "coat")
    sphere("glove_far", 0.05, (-0.02, FAR, 0.98), (1.0, 1.0, 1.0), "glove")


# Hand-to-tip vector of the vector mock (post/mock_figure.py), in heads:
# (2.10 - 0.70, 1.30 - 4.20) -> ~3.2 heads, matching docs/ART_DIRECTION.md.
SWORD_REACH_HEADS = (1.40, -2.90)
SWORD_HAND: Vec3 = (0.31, NEAR - 0.03, 1.02)
SWORD_TIP: Vec3 = (SWORD_HAND[0] + SWORD_REACH_HEADS[0] * HEAD_M, NEAR - 0.03,
                   SWORD_HAND[2] + SWORD_REACH_HEADS[1] * HEAD_M)


def build_sword(hand: Vec3 = SWORD_HAND, tip: Vec3 = SWORD_TIP,
                curve: float = 0.04, pieces: int = 10) -> None:
    """Long, thin, slightly curved blade made of short tapered pieces."""
    dx, dz = tip[0] - hand[0], tip[2] - hand[2]
    length = math.hypot(dx, dz)
    ux, uz = dx / length, dz / length
    nx, nz = -uz, ux  # normal toward the back/upper side
    y = hand[1]
    segment("grip", (hand[0] - ux * 0.12, y, hand[2] - uz * 0.12),
            (hand[0] + ux * 0.06, y, hand[2] + uz * 0.06), 0.014, "glove")
    sphere("pommel", 0.022, (hand[0] - ux * 0.13, y, hand[2] - uz * 0.13), (1.0, 1.0, 1.0), "amber")
    guard_c = (hand[0] + ux * 0.07, y, hand[2] + uz * 0.07)
    # Local Z follows the blade after rot_y, so the 0.12 m span goes on local X
    # to cross the blade.
    box("guard", (0.12, 0.03, 0.02), guard_c, "amber", rot_y=math.atan2(dx, dz))
    start = (hand[0] + ux * 0.08, hand[2] + uz * 0.08)
    points = []
    for i in range(pieces + 1):
        t = i / pieces
        bend = curve * 4.0 * t * (1.0 - t)
        points.append((start[0] + (tip[0] - start[0]) * t + nx * bend,
                       start[1] + (tip[2] - start[1]) * t + nz * bend))
    for i in range(pieces):
        (ax, az), (bx, bz) = points[i], points[i + 1]
        width = 0.034 * (1.0 - i / pieces) + 0.012
        seg_len = math.hypot(bx - ax, bz - az)
        box(f"blade_{i:02d}", (width, 0.006, seg_len * 1.05), ((ax + bx) / 2.0, y, (az + bz) / 2.0),
            "silver", rot_y=math.atan2(bx - ax, bz - az))


def build_all() -> bpy.types.Collection:
    build_body()
    build_limbs()
    build_sword()
    return collection()
