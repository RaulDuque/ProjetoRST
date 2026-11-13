"""Checks of the built v003 scene and the JSON build report.

Run inside Blender after the build. Failures are collected, not raised, so
the report lists every problem; the entry script exits with an error when
any check fails.
"""

from __future__ import annotations

import json
from pathlib import Path

import bpy
from mathutils import Vector

import params as P
from mesh_tools import COLLECTION_NAME

HEIGHT_TOLERANCE_M = 0.012  # ~1/3 of a sprite pixel at 1:1


def evaluated_triangles(obj: bpy.types.Object, depsgraph: bpy.types.Depsgraph) -> int:
    mesh = obj.evaluated_get(depsgraph).to_mesh()
    try:
        return sum(len(p.vertices) - 2 for p in mesh.polygons)
    finally:
        obj.evaluated_get(depsgraph).to_mesh_clear()


def check_scene() -> dict[str, object]:
    depsgraph = bpy.context.evaluated_depsgraph_get()
    coll = bpy.data.collections[COLLECTION_NAME]
    objects = [o for o in coll.objects if o.type == "MESH"]
    failures: list[str] = []
    warnings: list[str] = []
    triangles = 0
    zs: list[float] = []
    materials: set[str] = set()
    for obj in objects:
        if not obj.data.polygons:
            failures.append(f"{obj.name}: no faces")
        if not obj.material_slots or any(s.material is None for s in obj.material_slots):
            failures.append(f"{obj.name}: empty material slot")
        materials.update(s.material.name for s in obj.material_slots if s.material)
        if obj.hide_render:
            continue
        triangles += evaluated_triangles(obj, depsgraph)
        zs.extend((obj.matrix_world @ Vector(c)).z for c in obj.bound_box)
    top, bottom = max(zs), min(zs)
    if abs(top - P.MODEL_HEIGHT_M) > HEIGHT_TOLERANCE_M:
        failures.append(f"top {top:.3f} m differs from {P.MODEL_HEIGHT_M} m")
    if abs(bottom) > 0.002:
        failures.append(f"sole {bottom:.4f} m is not on z = 0")
    surfaces = sorted(m for m in materials if m.startswith("v3_"))
    if len(surfaces) < 10:
        warnings.append(f"only {len(surfaces)} distinct surfaces")
    return {
        "objects": len(objects),
        "triangles_rendered": triangles,
        "top_m": round(top, 4),
        "bottom_m": round(bottom, 4),
        "surfaces": surfaces,
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
