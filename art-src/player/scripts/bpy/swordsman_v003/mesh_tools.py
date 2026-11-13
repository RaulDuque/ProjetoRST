"""Procedural mesh builders: rings, lofts, tubes along curves, cloth surfaces.

Everything is built from explicit vertices and faces (``Mesh.from_pydata``),
so the shapes come from anatomical profiles and not from primitives. Faces
are wound so normals point outward; open cloth surfaces get thickness from a
Solidify modifier and smoothness from Subdivision Surface.
"""

from __future__ import annotations

import math
from collections.abc import Callable, Sequence

import bmesh
import bpy
from mathutils import Vector

Vec3 = tuple[float, float, float]

COLLECTION_NAME = "swordsman_v003"


def collection() -> bpy.types.Collection:
    coll = bpy.data.collections.get(COLLECTION_NAME)
    if coll is None:
        coll = bpy.data.collections.new(COLLECTION_NAME)
        bpy.context.scene.collection.children.link(coll)
    return coll


def make_object(name: str, verts: Sequence[Sequence[float]], faces: Sequence[Sequence[int]],
                materials: Sequence[bpy.types.Material], smooth: bool = True) -> bpy.types.Object:
    mesh = bpy.data.meshes.new(name)
    mesh.from_pydata([tuple(v) for v in verts], [], [tuple(f) for f in faces])
    mesh.validate(clean_customdata=False)
    mesh.update()
    for mat in materials:
        mesh.materials.append(mat)
    if smooth:
        mesh.shade_smooth()
    obj = bpy.data.objects.new(name, mesh)
    collection().objects.link(obj)
    return obj


def subdivide(obj: bpy.types.Object, levels: int = 1) -> None:
    mod = obj.modifiers.new("subsurf", "SUBSURF")
    mod.levels = levels
    mod.render_levels = levels


def thicken(obj: bpy.types.Object, thickness: float, inner_offset: int = 1) -> None:
    """Solidify inward; inside faces and cut rims use material index + offset."""
    mod = obj.modifiers.new("thickness", "SOLIDIFY")
    mod.thickness = thickness
    mod.offset = -1.0
    mod.use_rim = True
    mod.use_even_offset = True
    mod.material_offset = inner_offset
    mod.material_offset_rim = inner_offset


def remove_loose(obj: bpy.types.Object) -> None:
    bm = bmesh.new()
    bm.from_mesh(obj.data)
    loose = [v for v in bm.verts if not v.link_faces]
    bmesh.ops.delete(bm, geom=loose, context="VERTS")
    bm.to_mesh(obj.data)
    bm.free()
    obj.data.update()


# --- curves -----------------------------------------------------------------


def catmull_rom(points: Sequence[Vec3], samples: int) -> list[Vector]:
    """Smooth curve through ``points`` (centripetal-free uniform Catmull-Rom)."""
    pts = [Vector(p) for p in points]
    if len(pts) < 2:
        raise ValueError("a curve needs at least two points")
    ext = [pts[0] * 2 - pts[1], *pts, pts[-1] * 2 - pts[-2]]
    out: list[Vector] = []
    spans = len(pts) - 1
    for i in range(samples):
        t = i / (samples - 1) * spans
        k = min(int(t), spans - 1)
        u = t - k
        p0, p1, p2, p3 = ext[k], ext[k + 1], ext[k + 2], ext[k + 3]
        out.append(0.5 * ((2 * p1) + (-p0 + p2) * u + (2 * p0 - 5 * p1 + 4 * p2 - p3) * u * u
                          + (-p0 + 3 * p1 - 3 * p2 + p3) * u * u * u))
    return out


def lerp(a: float, b: float, t: float) -> float:
    return a + (b - a) * t


def smoothstep(t: float) -> float:
    t = min(max(t, 0.0), 1.0)
    return t * t * (3.0 - 2.0 * t)


def profile(keys: Sequence[tuple[float, float]], t: float) -> float:
    """Piecewise-smooth interpolation of (t, value) keys, t ascending."""
    if t <= keys[0][0]:
        return keys[0][1]
    for (t0, v0), (t1, v1) in zip(keys, keys[1:]):
        if t <= t1:
            return lerp(v0, v1, smoothstep((t - t0) / (t1 - t0)))
    return keys[-1][1]


def gauss(x: float, centre: float, width: float) -> float:
    return math.exp(-((x - centre) / width) ** 2)


def angle_delta(a: float, b: float) -> float:
    """Smallest signed difference a - b between two angles (radians)."""
    return (a - b + math.pi) % (2.0 * math.pi) - math.pi


# --- builders ---------------------------------------------------------------


def grid_object(name: str, rows: Sequence[Sequence[Vec3 | Vector]], materials: Sequence[bpy.types.Material],
                closed: bool = False, cap_start: bool = False, cap_end: bool = False,
                face_filter: Callable[[int, int], bool] | None = None,
                face_material: Callable[[int, int], int] | None = None) -> bpy.types.Object:
    """Quads between consecutive rows; rows run in the outward-normal order.

    ``closed`` wraps each row into a ring. ``face_filter(row, col)`` drops
    faces (cloth openings); ``face_material(row, col)`` picks a slot index.
    """
    width = len(rows[0])
    verts = [tuple(p) for row in rows for p in row]
    faces: list[tuple[int, ...]] = []
    mats: list[int] = []
    cols = width if closed else width - 1
    for r in range(len(rows) - 1):
        for c in range(cols):
            if face_filter is not None and not face_filter(r, c):
                continue
            c1 = (c + 1) % width
            faces.append((r * width + c, r * width + c1, (r + 1) * width + c1, (r + 1) * width + c))
            mats.append(face_material(r, c) if face_material else 0)
    if cap_start:
        centre = sum((Vector(p) for p in rows[0]), Vector()) / width
        verts.append(tuple(centre))
        ci = len(verts) - 1
        for c in range(width):
            faces.append((ci, (c + 1) % width, c))
            mats.append(0)
    if cap_end:
        base = (len(rows) - 1) * width
        centre = sum((Vector(p) for p in rows[-1]), Vector()) / width
        verts.append(tuple(centre))
        ci = len(verts) - 1
        for c in range(width):
            faces.append((ci, base + c, base + (c + 1) % width))
            mats.append(0)
    obj = make_object(name, verts, faces, materials)
    for poly, index in zip(obj.data.polygons, mats):
        poly.material_index = index
    if face_filter is not None:
        remove_loose(obj)
    return obj


def tube(name: str, path: Sequence[Vec3 | Vector], radii: Sequence[tuple[float, float]],
         materials: Sequence[bpy.types.Material], sides: int = 12,
         ref: Vec3 = (0.0, 0.0, 1.0), caps: bool = True,
         section: Callable[[float], tuple[float, float]] | None = None,
         face_material: Callable[[int, int], int] | None = None) -> bpy.types.Object:
    """Elliptic tube along ``path``; radii[i] = (a along the frame normal, b along the binormal).

    Frames use parallel transport from ``ref`` projected on the first normal
    plane, so the ellipse does not twist along bends. ``section(angle)`` can
    replace the ellipse with any closed profile (unit-scaled);
    ``face_material(ring, side)`` picks a material slot per face.
    """
    pts = [Vector(p) for p in path]
    n_pts = len(pts)
    if len(radii) != n_pts:
        raise ValueError("one radius pair per path point")
    tangents = []
    for i in range(n_pts):
        a = pts[max(i - 1, 0)]
        b = pts[min(i + 1, n_pts - 1)]
        tangents.append((b - a).normalized())
    normal = Vector(ref) - tangents[0] * Vector(ref).dot(tangents[0])
    if normal.length < 1e-6:
        normal = tangents[0].orthogonal()
    normal.normalize()
    rows = []
    for i, (p, t) in enumerate(zip(pts, tangents)):
        if i > 0:
            normal = normal - t * normal.dot(t)
            normal.normalize()
        binormal = t.cross(normal)
        a, b = radii[i]
        ring = []
        for s in range(sides):
            ang = 2.0 * math.pi * s / sides
            cx, cy = section(ang) if section else (math.cos(ang), math.sin(ang))
            ring.append(p + normal * (a * cx) + binormal * (b * cy))
        rows.append(ring)
    return grid_object(name, rows, materials, closed=True, cap_start=caps, cap_end=caps,
                       face_material=face_material)


def radial_rows(z_values: Sequence[float], segments: int,
                point: Callable[[float, float], Vec3]) -> list[list[Vec3]]:
    """Rings around Z: point(z, theta) for theta counter-clockwise from +X."""
    return [[point(z, 2.0 * math.pi * s / segments) for s in range(segments)] for z in z_values]
