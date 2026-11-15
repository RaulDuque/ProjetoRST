"""Checks added in v006 on top of the v005 ones (``rig_validate``, unchanged).

- extra intersection pairs the V1.2-F brief asks about (collar x neck,
  pauldron x near sleeve). They are measured on v005 and v006 with the
  same code, so the before/after numbers stay comparable, and reported
  apart from the v005 pairs (whose numbers stay comparable with V1.2-E);
- source integrity of the v005 file (bone count, v004 geometry hash with
  the Armature modifiers off, weight hash);
- geometry preserved outside the corrected areas: per pose, the evaluated
  vertices of every mesh NOT touched by a fix must be identical in v005 and
  v006;
- pivots: every bone kept from v005 keeps its rest head, tail and roll.
"""

from __future__ import annotations

import hashlib
import struct

import bpy
import numpy as np

import rig_validate as V

EXTRA_PAIRS = (
    ("collar", "neck", "collar"),
    ("pauldron", "sleeve_near", "arm_body"),
)


def extra_snapshot() -> dict[str, int]:
    from mathutils.bvhtree import BVHTree

    depsgraph = bpy.context.evaluated_depsgraph_get()
    names = {n for a, b, _ in EXTRA_PAIRS for n in (a, b)}
    trees = {n: BVHTree.FromPolygons(*V._mesh_data(bpy.data.objects[n], depsgraph)) for n in names}
    return {f"{a} x {b}": len(trees[a].overlap(trees[b])) for a, b, _ in EXTRA_PAIRS}


def extra_issues(rest: dict[str, int], posed: dict[str, int]) -> list[dict[str, object]]:
    issues = []
    for a, b, kind in EXTRA_PAIRS:
        key = f"{a} x {b}"
        delta = posed[key] - rest.get(key, 0)
        severity = V.classify(kind, delta)
        if severity:
            issues.append({"severity": severity, "what": f"interseção {key}", "value": delta})
    return issues


def geometry_hash_without_rig(meshes: list[bpy.types.Object]) -> str:
    """Hash of the evaluated meshes with the Armature modifier off (= the v004 build hash)."""
    states = []
    for obj in meshes:
        mod = obj.modifiers[0]
        if mod.type != "ARMATURE":
            raise RuntimeError(f"{obj.name}: first modifier is not the rig")
        states.append((mod, mod.show_viewport))
        mod.show_viewport = False
    depsgraph = bpy.context.evaluated_depsgraph_get()
    depsgraph.update()
    h = hashlib.sha256()
    try:
        for obj in sorted(meshes, key=lambda o: o.name):
            ev = obj.evaluated_get(depsgraph)
            mesh = ev.to_mesh()
            for v in mesh.vertices:
                h.update(struct.pack("3f", *v.co))
            ev.to_mesh_clear()
    finally:
        for mod, shown in states:
            mod.show_viewport = shown
        depsgraph.update()
    return h.hexdigest()


def weight_hash(meshes: list[bpy.types.Object]) -> str:
    h = hashlib.sha256()
    for obj in sorted(meshes, key=lambda o: o.name):
        names = {vg.index: vg.name for vg in obj.vertex_groups}
        for v in obj.data.vertices:
            for g in sorted(v.groups, key=lambda g: names[g.group]):
                h.update(f"{obj.name}|{v.index}|{names[g.group]}|{g.weight:.6f}".encode())
    return h.hexdigest()


def evaluated_arrays(meshes: list[bpy.types.Object], armature_only: bool = False) -> dict[str, np.ndarray]:
    """World-space vertices of the evaluated meshes (object space = world: no object transforms).

    ``armature_only`` turns off every modifier after the Armature one, i.e. it
    returns what the rig produces. Measured in V1.2-F: the pauldron's
    Solidify is numerically unstable (217 vertices flip 3.5 mm across the
    4 mm shell when its input moves by ~1e-7 m, e.g. when ANOTHER object
    switches to dual-quaternion skinning), so a rig-preservation check must
    compare the Armature stage, not the final shell.
    """
    states = []
    if armature_only:
        for obj in meshes:
            for mod in obj.modifiers[1:]:
                states.append((mod, mod.show_viewport))
                mod.show_viewport = False
    depsgraph = bpy.context.evaluated_depsgraph_get()
    depsgraph.update()
    out = {}
    try:
        for obj in meshes:
            ev = obj.evaluated_get(depsgraph)
            mesh = ev.to_mesh()
            arr = np.empty(len(mesh.vertices) * 3, dtype=np.float32)
            mesh.vertices.foreach_get("co", arr)
            ev.to_mesh_clear()
            out[obj.name] = arr
    finally:
        for mod, shown in states:
            mod.show_viewport = shown
        if states:
            depsgraph.update()
    return out


def changed_meshes(before: dict[str, np.ndarray], after: dict[str, np.ndarray], tol: float = 1e-5) -> dict[str, float]:
    """Mesh -> max vertex displacement (m) between v005 and v006 in the same pose.

    Tolerance 10 micrometres: float32 noise of root-relative coordinates
    (~2 m from the origin) is ~1e-6 m; a real rig change is millimetres.
    """
    out = {}
    for name, a in before.items():
        b = after[name]
        if a.shape != b.shape:
            out[name] = float("inf")
            continue
        d = float(np.abs(a - b).max()) if a.size else 0.0
        if d > tol:
            out[name] = round(d, 6)
    return out


def bone_frames(arm: bpy.types.Object) -> dict[str, tuple[tuple[float, ...], ...]]:
    return {b.name: tuple(tuple(round(v, 6) for v in row) for row in b.matrix_local) for b in arm.data.bones}


def pivot_changes(before: dict[str, tuple], after: dict[str, tuple]) -> dict[str, list[str]]:
    moved = sorted(n for n in before if n in after and before[n] != after[n])
    return {"moved": moved, "added": sorted(set(after) - set(before)), "removed": sorted(set(before) - set(after))}

