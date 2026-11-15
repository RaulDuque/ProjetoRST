"""Automated checks of the v005 rig (run in the build and again on reopen).

Structure: armature, required bones, unique names, acyclic hierarchy,
every mesh bound (Armature modifier first, pointing at the rig).
Weights: every vertex influenced, weights in [0, 1], normalised, at most
``MAX_INFLUENCES``, every group named after an existing deform bone.
Rest: the bound mesh at the rest pose matches v004 (max displacement).
Poses: intersections between pieces that move against each other (counted
against the rest pose), volume of closed limb meshes (collapsed joints,
inverted normals), unit pose scale (no proportion change), sabre rigidly
attached to the hand. IK: the leg and hand chains reach a moved target.

Severity follows the brief: Crítico, Alto, Médio, Baixo, Cosmético.
"""

from __future__ import annotations

import math
from collections.abc import Iterable

import bpy
from mathutils import Matrix, Vector
from mathutils.bvhtree import BVHTree

import rig_config as C

REQUIRED_BONES = (
    "root", "pelvis", "spine_01", "spine_02", "chest", "neck", "head",
    "clavicle.L", "upper_arm.L", "forearm.L", "hand.L", "clavicle.R", "upper_arm.R", "forearm.R", "hand.R",
    "thigh.L", "shin.L", "foot.L", "toe.L", "thigh.R", "shin.R", "foot.R", "toe.R", "sword",
    "coat_front.L_01", "coat_front.R_01", "coat_back.L_01", "coat_back.R_01",
    "cape_arm.L_01", "cape_back_01",
)

# (a, b, kind): kind picks the severity scale in ``classify``.
PAIRS = (
    ("coat", "leg_near", "coat_leg"), ("coat", "leg_far", "coat_leg"),
    ("coat", "boot_shaft_near", "coat_leg"), ("coat", "boot_shaft_far", "coat_leg"),
    ("half_cape", "torso_vest", "cape_body"), ("half_cape", "coat", "cape_body"),
    ("half_cape", "sleeve_far", "cape_arm"),
    ("sleeve_near", "torso_vest", "arm_body"), ("sleeve_far", "torso_vest", "arm_body"),
    ("sleeve_near", "coat", "arm_body"), ("sleeve_far", "coat", "arm_body"),
    ("pauldron", "coat", "arm_body"),
    ("collar", "head", "collar"), ("collar", "hair_back_0", "collar"), ("collar", "hair_back_5", "collar"),
    ("collar", "hair_side_near_4", "collar"),
    ("blade", "glove_near_palm", "sword_hand"), ("blade", "glove_near_finger_0", "sword_hand"),
    ("leg_near", "leg_far", "leg_leg"), ("boot_shaft_near", "boot_shaft_far", "leg_leg"),
)
CLOSED = ("leg_near", "leg_far", "sleeve_near", "sleeve_far",  # boot shafts are open at the top
          "boot_foot_near", "boot_foot_far", "neck", "torso_trousers")
SEVERITY_ORDER = ("Crítico", "Alto", "Médio", "Baixo", "Cosmético")


def _mesh_data(obj: bpy.types.Object, depsgraph: bpy.types.Depsgraph) -> tuple[list[Vector], list[tuple[int, ...]]]:
    ev = obj.evaluated_get(depsgraph)
    mesh = ev.to_mesh()
    try:
        mesh.calc_loop_triangles()
        verts = [obj.matrix_world @ v.co for v in mesh.vertices]
        tris = [tuple(t.vertices) for t in mesh.loop_triangles]
    finally:
        ev.to_mesh_clear()
    return verts, tris


def _volume(verts: list[Vector], tris: Iterable[tuple[int, ...]]) -> float:
    return sum(verts[a].dot(verts[b].cross(verts[c])) for a, b, c in tris) / 6.0


def snapshot() -> dict[str, dict[str, float]]:
    """Intersection counts per pair and volumes of the closed meshes."""
    depsgraph = bpy.context.evaluated_depsgraph_get()
    names = {n for a, b, _ in PAIRS for n in (a, b)} | set(CLOSED)
    data = {n: _mesh_data(bpy.data.objects[n], depsgraph) for n in names if n in bpy.data.objects}
    trees = {n: BVHTree.FromPolygons(v, t) for n, (v, t) in data.items()}
    overlaps = {f"{a} x {b}": len(trees[a].overlap(trees[b])) for a, b, _ in PAIRS if a in trees and b in trees}
    volumes = {n: _volume(*data[n]) for n in CLOSED if n in data}
    return {"overlaps": overlaps, "volumes": volumes}


def classify(kind: str, delta: int) -> str | None:
    if delta <= 0:
        return None
    scales = {
        "coat_leg": ((300, "Alto"), (60, "Médio"), (1, "Baixo")),
        "cape_body": ((150, "Alto"), (20, "Médio"), (1, "Baixo")),
        "cape_arm": ((300, "Médio"), (1, "Baixo")),
        "arm_body": ((400, "Médio"), (60, "Baixo"), (1, "Cosmético")),
        "collar": ((150, "Médio"), (1, "Baixo")),
        "sword_hand": ((1, "Alto"),),
        "leg_leg": ((200, "Médio"), (1, "Baixo")),
    }
    for threshold, label in scales[kind]:
        if delta >= threshold:
            return label
    return None


def pose_issues(rest: dict[str, dict[str, float]], posed: dict[str, dict[str, float]]) -> list[dict[str, object]]:
    issues: list[dict[str, object]] = []
    for a, b, kind in PAIRS:
        key = f"{a} x {b}"
        if key not in posed["overlaps"]:
            continue
        delta = posed["overlaps"][key] - rest["overlaps"].get(key, 0)
        severity = classify(kind, delta)
        if severity:
            issues.append({"severity": severity, "what": f"interseção {key}", "value": delta})
    for name, vol in posed["volumes"].items():
        ratio = vol / rest["volumes"][name] if rest["volumes"].get(name) else 1.0
        severity = None
        if ratio <= 0.0:
            severity, what = "Crítico", "normais invertidas / volume negativo"
        elif ratio < 0.75:
            severity, what = "Crítico", "colapso grave de volume"
        elif ratio < 0.85:
            severity, what = "Alto", "perda de volume na articulação"
        elif ratio < 0.93 or ratio > 1.12:
            severity, what = "Médio", "variação de volume"
        if severity:
            issues.append({"severity": severity, "what": f"{name}: {what}", "value": round(ratio, 3)})
    issues.sort(key=lambda i: SEVERITY_ORDER.index(str(i["severity"])))
    return issues


# --- structure, weights, rest, sword, IK ---------------------------------------------


def check_structure(arm: bpy.types.Object, meshes: list[bpy.types.Object]) -> dict[str, object]:
    bones = arm.data.bones
    names = [b.name for b in bones]
    failures = [f"missing bone {n}" for n in REQUIRED_BONES if n not in bones]
    if len(set(names)) != len(names):
        failures.append("duplicate bone names")
    for b in bones:  # a parent chain longer than the bone count means a cycle
        seen, p = 0, b.parent
        while p is not None:
            seen += 1
            p = p.parent
            if seen > len(bones):
                failures.append(f"cycle above {b.name}")
                break
    deform = {b.name for b in bones if b.use_deform}
    for obj in meshes:
        mods = [m for m in obj.modifiers if m.type == "ARMATURE"]
        if len(mods) != 1 or mods[0].object != arm or obj.modifiers[0] != mods[0]:
            failures.append(f"{obj.name}: armature modifier missing, wrong target or not first")
        for vg in obj.vertex_groups:
            if vg.name not in deform:
                failures.append(f"{obj.name}: group {vg.name} is not a deform bone")
    return {"bones": len(bones), "deform_bones": len(deform), "failures": failures}


def check_weights(meshes: list[bpy.types.Object]) -> dict[str, object]:
    failures: list[str] = []
    worst_sum, max_inf, verts = 0.0, 0, 0
    for obj in meshes:
        for v in obj.data.vertices:
            verts += 1
            ws = [g.weight for g in v.groups]
            if not ws:
                failures.append(f"{obj.name}: vertex {v.index} without influence")
                break
            if any(w < 0.0 or w > 1.0 for w in ws):
                failures.append(f"{obj.name}: vertex {v.index} weight out of [0, 1]")
                break
            worst_sum = max(worst_sum, abs(sum(ws) - 1.0))
            max_inf = max(max_inf, len(ws))
    if worst_sum > 1e-3:
        failures.append(f"weights not normalised (worst |sum - 1| = {worst_sum:.4f})")
    if max_inf > C.MAX_INFLUENCES:
        failures.append(f"{max_inf} influences on one vertex (limit {C.MAX_INFLUENCES})")
    return {"vertices": verts, "max_influences": max_inf, "worst_sum_error": round(worst_sum, 6),
            "failures": failures}


def rest_displacement(meshes: list[bpy.types.Object], originals: dict[str, list[Vector]]) -> float:
    """Max distance between the bound mesh at rest and the unbound v004 base mesh."""
    depsgraph = bpy.context.evaluated_depsgraph_get()
    worst = 0.0
    for obj in meshes:
        arm_mod = obj.modifiers[0]
        states = [(m, m.show_viewport) for m in obj.modifiers[1:]]
        for m, _ in states:
            m.show_viewport = False
        depsgraph.update()
        ev = obj.evaluated_get(depsgraph)
        mesh = ev.to_mesh()
        try:
            for v, o in zip(mesh.vertices, originals[obj.name]):
                worst = max(worst, (v.co - o).length)
        finally:
            ev.to_mesh_clear()
            for m, shown in states:
                m.show_viewport = shown
        if not arm_mod.show_viewport:
            raise RuntimeError("armature modifier disabled")
    depsgraph.update()
    return worst


def sword_offset(arm: bpy.types.Object) -> Matrix:
    """Sword bone matrix relative to the hand (constant if the sabre is attached)."""
    hand = arm.pose.bones["hand.R"].matrix
    return hand.inverted() @ arm.pose.bones["sword"].matrix


def check_ik(arm: bpy.types.Object) -> dict[str, object]:
    """Moves the IK targets and measures how far the chain ends are from them."""
    from skeleton import set_ik

    pb = arm.pose.bones
    set_ik(arm, True)
    moves = {"CTRL_foot_ik.R": Vector((0.12, 0.0, 0.18)), "CTRL_foot_ik.L": Vector((-0.1, 0.0, 0.1)),
             "CTRL_hand_ik.R": Vector((0.1, 0.0, 0.15))}
    for name, world in moves.items():
        rest = arm.data.bones[name].matrix_local.to_3x3()
        pb[name].location = rest.inverted() @ world
    bpy.context.view_layer.update()
    errors = {}
    for ctrl, end in (("CTRL_foot_ik.R", "shin.R"), ("CTRL_foot_ik.L", "shin.L"), ("CTRL_hand_ik.R", "forearm.R")):
        target = arm.matrix_world @ pb[ctrl].head
        tip = arm.matrix_world @ pb[end].tail
        errors[f"{end} -> {ctrl}"] = round((target - tip).length, 5)
    rest_err = {}
    for name in moves:
        pb[name].location = Vector()
    bpy.context.view_layer.update()
    for bone in ("thigh.R", "shin.R", "upper_arm.R", "forearm.R"):  # IK at rest = rest pose
        rest_m = arm.data.bones[bone].matrix_local
        rest_err[bone] = round(max(abs(a - b) for ra, rb in zip(pb[bone].matrix, rest_m)
                                   for a, b in zip(ra, rb)), 5)
    set_ik(arm, False)
    bpy.context.view_layer.update()
    ok = all(e < 0.005 for e in errors.values()) and all(e < 1e-3 for e in rest_err.values())
    return {"reach_error_m": errors, "rest_drift": rest_err, "ok": ok}


def check_scale(arm: bpy.types.Object) -> list[str]:
    bad = [pb.name for pb in arm.pose.bones if any(abs(s - 1.0) > 1e-6 for s in pb.scale)]
    return [f"pose scale changed on {', '.join(bad)}"] if bad else []


def max_severity(issues: list[dict[str, object]]) -> str:
    if not issues:
        return "OK"
    return min((str(i["severity"]) for i in issues), key=SEVERITY_ORDER.index)


def angle_between(a: Matrix, b: Matrix) -> float:
    return math.degrees(a.to_quaternion().rotation_difference(b.to_quaternion()).angle)
