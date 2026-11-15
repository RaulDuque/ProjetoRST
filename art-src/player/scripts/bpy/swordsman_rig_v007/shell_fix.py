"""Pauldron shell: diagnosis, fix and stability test (V1.2-G).

Diagnosis (measured on v006, see ANIMATION_VALIDATION_V1_2G.md):
- each plate (``pauldron``, ``pauldron_lame``) is a ring grid whose last
  ring of 37 vertices was collapsed onto one point instead of merged:
  36 zero-length edges and 36 zero-area quads at the pole;
- the Armature output is continuous (identical within 2.4e-7 m between the
  compared states); the Solidify modifier (even thickness, offset -1) needs
  a normal per face, and a zero-area face has none: its direction comes
  from float noise, so a 1e-7 m change in the input flips part of the 4 mm
  shell (217 vertices, 3.5 mm) and the Subdivision after it smooths the
  flip over the pole area.

Second fix, found with the same test in the animation frames: the coat and
half-cape shells (no degenerate faces) flipped where the cloth folds hard,
because of the even-thickness option; plain thickness is stable
(``fix_cloth_shells``).

Fix: merge the coincident vertices into one pole (``remove_doubles`` with a
1e-6 m distance, which touches only that cluster). The quads at the pole
become a triangle fan of real faces. No vertex moves, the rule-based
weights are constant over each plate (``skinning.rule_for``: pauldron ->
55 % upper_arm.R + 45 % clavicle.R), so the merged vertex keeps the same
weights; modifiers, materials and the rig are untouched.

Stability test: the armature object is nudged by tiny rigid motions
(1e-6 m, 1e-6 rad). A stable shell moves with them (the residual after
undoing the nudge is float noise); an unstable one jumps.
"""

from __future__ import annotations

import bmesh
import bpy
import numpy as np
from mathutils import Euler, Matrix, Vector

import rig_config as C


def topology(obj: bpy.types.Object) -> dict[str, int]:
    bm = bmesh.new()
    bm.from_mesh(obj.data)
    try:
        return {
            "vertices": len(bm.verts), "faces": len(bm.faces),
            "zero_area_faces": sum(1 for f in bm.faces if f.calc_area() < 1e-10),
            "zero_length_edges": sum(1 for e in bm.edges if e.calc_length() < 1e-7),
            "boundary_edges": sum(1 for e in bm.edges if e.is_boundary),
            "non_manifold_edges": sum(1 for e in bm.edges if not e.is_manifold),
        }
    finally:
        bm.free()


def scan_degenerate(meshes: list[bpy.types.Object]) -> dict[str, dict[str, object]]:
    """Every mesh with zero-area faces or zero-length edges, and whether it has a Solidify."""
    out = {}
    for obj in meshes:
        t = topology(obj)
        if t["zero_area_faces"] or t["zero_length_edges"]:
            out[obj.name] = {**t, "solidify": any(m.type == "SOLIDIFY" for m in obj.modifiers)}
    return out


def weights_of(obj: bpy.types.Object) -> set[tuple[str, float]]:
    names = {vg.index: vg.name for vg in obj.vertex_groups}
    return {(names[g.group], round(g.weight, 6)) for v in obj.data.vertices for g in v.groups}


def fix_pauldron() -> dict[str, object]:
    report = {}
    for name in C.PAULDRON_OBJECTS:
        obj = bpy.data.objects[name]
        before = topology(obj)
        weights_before = weights_of(obj)
        bm = bmesh.new()
        bm.from_mesh(obj.data)
        result = bmesh.ops.find_doubles(bm, verts=bm.verts, dist=C.MERGE_DISTANCE_M)
        merged = len(result["targetmap"])
        bmesh.ops.remove_doubles(bm, verts=bm.verts, dist=C.MERGE_DISTANCE_M)
        bm.to_mesh(obj.data)
        bm.free()
        obj.data.update()
        after = topology(obj)
        weights_after = weights_of(obj)
        if after["zero_area_faces"] or after["zero_length_edges"]:
            raise RuntimeError(f"{name}: degenerate geometry left after the merge: {after}")
        if weights_after != weights_before:
            raise RuntimeError(f"{name}: weights changed by the merge")
        report[name] = {"before": before, "after": after, "merged_vertices": merged,
                        "weights": sorted(weights_after)}
    return report


def fix_cloth_shells() -> dict[str, object]:
    """Even thickness off on the coat and half-cape Solidify (see rig_config.EVEN_OFFSET_OFF)."""
    report = {}
    for name in C.EVEN_OFFSET_OFF:
        mod = next(m for m in bpy.data.objects[name].modifiers if m.type == "SOLIDIFY")
        report[name] = {"use_even_offset": [mod.use_even_offset, False], "thickness_m": round(mod.thickness, 4)}
        mod.use_even_offset = False
    return report


def final_arrays(objs: list[bpy.types.Object]) -> dict[str, np.ndarray]:
    depsgraph = bpy.context.evaluated_depsgraph_get()
    depsgraph.update()
    out = {}
    for obj in objs:
        ev = obj.evaluated_get(depsgraph)
        mesh = ev.to_mesh()
        arr = np.empty(len(mesh.vertices) * 3, dtype=np.float64)
        mesh.vertices.foreach_get("co", arr)
        ev.to_mesh_clear()
        out[obj.name] = arr.reshape(-1, 3)
    return out


def _nudges() -> list[Matrix]:
    t, r = C.PERTURB_TRANSLATION_M, C.PERTURB_ROTATION_RAD
    moves = [Matrix.Translation(Vector(v)) for v in ((t, 0, 0), (0, t, 0), (0, 0, t))]
    moves += [Euler(v).to_matrix().to_4x4() for v in ((r, 0, 0), (0, r, 0), (0, 0, r))]
    return moves


def shell_residuals(arm: bpy.types.Object, objs: list[bpy.types.Object]) -> dict[str, float]:
    """Max distance (m) between the shell and the rigidly nudged shell, per object.

    The meshes have identity object transforms, so nudging the armature
    object by ``delta`` moves the deformed (and, if stable, the solidified)
    vertices by exactly ``delta``.
    """
    base = final_arrays(objs)
    original = arm.matrix_world.copy()
    worst = {o.name: 0.0 for o in objs}
    try:
        for delta in _nudges():
            arm.matrix_world = delta @ original
            bpy.context.view_layer.update()
            moved = final_arrays(objs)
            m = np.array(delta, dtype=np.float64)
            for name, b in base.items():
                expected = b @ m[:3, :3].T + m[:3, 3]
                if moved[name].shape != expected.shape:
                    worst[name] = float("inf")
                    continue
                worst[name] = max(worst[name], float(np.abs(moved[name] - expected).max()))
    finally:
        arm.matrix_world = original
        bpy.context.view_layer.update()
    return {n: round(v, 7) for n, v in worst.items()}


def solidify_objects(meshes: list[bpy.types.Object]) -> list[bpy.types.Object]:
    return [o for o in meshes if any(m.type == "SOLIDIFY" for m in o.modifiers)]
