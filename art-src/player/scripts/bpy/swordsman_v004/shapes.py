"""Shape helpers added in v004 on top of ``swordsman_v003/mesh_tools``.

``profiled_tube`` generalises ``mesh_tools.tube``: instead of one ellipse
per ring, the radius is any function of the position along the path and of
the angle around it. That is what lets a leg have a knee in front and a calf
behind, or a sleeve gather at the elbow, without stacking primitives.

Angle convention around the path: 0 = the frame normal (``ref`` projected
on the first ring plane, then parallel-transported), pi/2 = the binormal
(tangent x normal).
"""

from __future__ import annotations

import math
from collections.abc import Callable, Sequence

import bpy
from mathutils import Vector

from mesh_tools import Vec3, grid_object

RadiusFn = Callable[[float, float], float]


def frames(path: Sequence[Vector], ref: Vec3) -> list[tuple[Vector, Vector, Vector]]:
    """(tangent, normal, binormal) per point, parallel transport from ``ref``."""
    n = len(path)
    tangents = [(path[min(i + 1, n - 1)] - path[max(i - 1, 0)]).normalized() for i in range(n)]
    normal = Vector(ref) - tangents[0] * Vector(ref).dot(tangents[0])
    if normal.length < 1e-6:
        normal = tangents[0].orthogonal()
    normal.normalize()
    out = []
    for i, t in enumerate(tangents):
        if i > 0:
            normal = (normal - t * normal.dot(t)).normalized()
        out.append((t, normal.copy(), t.cross(normal)))
    return out


def profiled_tube(name: str, path: Sequence[Vec3 | Vector], radius: RadiusFn,
                  materials: Sequence[bpy.types.Material], sides: int = 20,
                  ref: Vec3 = (1.0, 0.0, 0.0), caps: bool = True,
                  face_material: Callable[[int, int], int] | None = None) -> bpy.types.Object:
    """Closed tube; ``radius(t, angle)`` with t in [0, 1] from the first point."""
    pts = [Vector(p) for p in path]
    n = len(pts)
    rows = []
    for i, (p, (_, normal, binormal)) in enumerate(zip(pts, frames(pts, ref))):
        t = i / (n - 1)
        ring = []
        for s in range(sides):
            ang = 2.0 * math.pi * s / sides
            r = radius(t, ang)
            ring.append(p + normal * (r * math.cos(ang)) + binormal * (r * math.sin(ang)))
        rows.append(ring)
    return grid_object(name, rows, materials, closed=True, cap_start=caps, cap_end=caps,
                       face_material=face_material)


def lobe(ang: float, centre: float, width: float) -> float:
    """Angular bump in [0, 1] around ``centre`` (radians), periodic."""
    d = (ang - centre + math.pi) % (2.0 * math.pi) - math.pi
    return math.exp(-(d / width) ** 2)


def ribbon(name: str, path: Sequence[Vec3 | Vector], normals: Sequence[Vector], widths: Sequence[float],
           materials: Sequence[bpy.types.Material], thickness: float) -> bpy.types.Object:
    """Flat strip along ``path`` lying on the surface with the given normals.

    Used for seams, welts and trims that must hug a surface instead of
    floating as round tubes. Thickness comes out along the normal.
    """
    pts = [Vector(p) for p in path]
    n = len(pts)
    rows: list[list[Vector]] = []
    for i, p in enumerate(pts):
        t = (pts[min(i + 1, n - 1)] - pts[max(i - 1, 0)]).normalized()
        nrm = Vector(normals[i]).normalized()
        side = t.cross(nrm).normalized() * widths[i]
        lift = nrm * thickness
        rows.append([p - side, p + side, p + side + lift, p - side + lift])
    return grid_object(name, rows, materials, closed=True, cap_start=True, cap_end=True)


def orient_outward(obj: bpy.types.Object, inside: Vec3 | Vector) -> bpy.types.Object:
    """Flips an open surface whose faces point toward ``inside`` (on average)."""
    centre = Vector(inside)
    score = sum(p.normal.dot(Vector(p.center) - centre) for p in obj.data.polygons)
    if score < 0.0:
        obj.data.flip_normals()
        obj.data.update()
    return obj
