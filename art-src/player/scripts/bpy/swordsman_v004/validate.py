"""Checks of the built v004 scene and the JSON build report.

Extends the v003 checks (height, sole, faces, material slots) with an
intersection survey between the pieces that a future rig will move against
each other. Penetrations between pieces that are meant to overlap (a sleeve
going into the torso, a lock into the scalp) are listed for information;
only the coat crossing the legs at rest is a failure, as the brief asks.
"""

from __future__ import annotations

import json
from pathlib import Path

import bpy
from mathutils import Vector
from mathutils.bvhtree import BVHTree

import params as P

HEIGHT_TOLERANCE_M = 0.012  # ~1/3 of a sprite pixel at 1:1

# (a, b, is_failure): pairs checked for intersecting triangles.
PAIRS = (
    ("coat", "leg_near", True),
    ("coat", "leg_far", True),
    ("coat", "boot_shaft_near", True),
    ("coat", "boot_shaft_far", True),
    ("half_cape", "sleeve_far", False),
    ("half_cape", "coat", False),
    ("coat", "sleeve_near", False),
    ("coat", "sleeve_far", False),
    ("collar", "head", False),
    ("collar", "hair_back_0", False),
    ("pauldron", "sleeve_near", False),
    ("glove_near_palm", "sword_grip", False),
    ("leg_near", "leg_far", False),
)


def _evaluated(obj: bpy.types.Object, depsgraph: bpy.types.Depsgraph) -> tuple[int, BVHTree]:
    ev = obj.evaluated_get(depsgraph)
    mesh = ev.to_mesh()
    try:
        tris = sum(len(p.vertices) - 2 for p in mesh.polygons)
        verts = [obj.matrix_world @ v.co for v in mesh.vertices]
        tree = BVHTree.FromPolygons(verts, [tuple(p.vertices) for p in mesh.polygons])
    finally:
        ev.to_mesh_clear()
    return tris, tree


def check_scene() -> dict[str, object]:
    depsgraph = bpy.context.evaluated_depsgraph_get()
    coll = bpy.data.collections[P.COLLECTION_NAME]
    objects = [o for o in coll.objects if o.type == "MESH"]
    failures: list[str] = []
    warnings: list[str] = []
    triangles = 0
    zs: list[float] = []
    materials: set[str] = set()
    trees: dict[str, BVHTree] = {}
    per_object: dict[str, int] = {}
    for obj in objects:
        if not obj.data.polygons:
            failures.append(f"{obj.name}: no faces")
        if not obj.material_slots or any(s.material is None for s in obj.material_slots):
            failures.append(f"{obj.name}: empty material slot")
        materials.update(s.material.name for s in obj.material_slots if s.material)
        if obj.hide_render:
            continue
        tris, tree = _evaluated(obj, depsgraph)
        triangles += tris
        per_object[obj.name] = tris
        trees[obj.name] = tree
        zs.extend((obj.matrix_world @ Vector(c)).z for c in obj.bound_box)
    top, bottom = max(zs), min(zs)
    if abs(top - P.MODEL_HEIGHT_M) > HEIGHT_TOLERANCE_M:
        failures.append(f"top {top:.3f} m differs from {P.MODEL_HEIGHT_M} m")
    if abs(bottom) > 0.002:
        failures.append(f"sole {bottom:.4f} m is not on z = 0")
    intersections = {}
    for a, b, is_failure in PAIRS:
        if a not in trees or b not in trees:
            warnings.append(f"intersection pair {a} x {b}: object missing")
            continue
        count = len(trees[a].overlap(trees[b]))
        intersections[f"{a} x {b}"] = count
        if count and is_failure:
            failures.append(f"{a} crosses {b} at rest ({count} triangle pairs)")
    surfaces = sorted(m for m in materials if m.startswith("v4_"))
    return {
        "objects": len(objects),
        "triangles_rendered": triangles,
        "heaviest_objects": dict(sorted(per_object.items(), key=lambda kv: -kv[1])[:8]),
        "top_m": round(top, 4),
        "bottom_m": round(bottom, 4),
        "surfaces": surfaces,
        "intersections": intersections,
        "failures": failures,
        "warnings": warnings,
    }


def write_report(path: Path, checks: dict[str, object], extra: dict[str, object]) -> None:
    data = {
        "model": f"{P.MODEL_NAME}_{P.MODEL_VERSION}",
        "blender": bpy.app.version_string,
        "engine": bpy.context.scene.render.engine,
        "parameters": P.report(),
        "checks": checks,
        **extra,
    }
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2, ensure_ascii=False, default=str) + "\n", encoding="utf-8")
