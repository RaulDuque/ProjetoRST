"""Targeted corrections that turn the v005 rig into v006 (V1.2-F).

Each function changes one thing and returns a short record of what it
changed, for the build manifest. Nothing here adds geometry or touches the
v004 meshes themselves: only bones, constraints, vertex groups and the
Armature modifier flags change. The v005 modules are imported unchanged
(``skeleton``, ``skinning``); ``rig_config`` is the v006 one (see there).

1. ``fix_half_cape``  - cape arm chain re-pivoted on the shoulder with the
   rest frame of ``upper_arm.L``; asymmetric follow (back, forward,
   abduction); the cape top over the shoulder cap rides partly on the chain.
2. ``fix_coat``       - back panels hand the forward thigh swing back after
   ``COAT_BACK_FWD_PEAK``; the front waist is carried by the skirt bones.
3. ``fix_collar``     - new ``collar_back`` bone, partial copy of the neck and
   head rotation; back and sides of the collar weighted to it.
4. ``fix_volume``     - dual-quaternion skinning on the boot foot shells (ankle)
   and on the trousers (hip flexion).
"""

from __future__ import annotations

import math
from collections.abc import Callable

import bpy
from mathutils import Vector

import rig_config as C
import skinning as SK  # v005 rules, reused unchanged
from mesh_tools import smoothstep

Weights = dict[str, float]

CAPE_OBJECTS = ("half_cape", "cape_clasp")


def rename_armature() -> bpy.types.Object:
    arm = bpy.data.objects.get(C.SOURCE_ARMATURE)
    if arm is None or arm.type != "ARMATURE":
        raise RuntimeError(f"source armature {C.SOURCE_ARMATURE} missing")
    arm.name = C.ARMATURE_NAME
    arm.data.name = C.ARMATURE_NAME
    if arm.name != C.ARMATURE_NAME:
        raise RuntimeError(f"could not rename the armature to {C.ARMATURE_NAME}")
    return arm


def reskin(obj: bpy.types.Object, rule: Callable[[Vector], Weights]) -> int:
    """Replaces every vertex group of ``obj`` with the weights of ``rule``."""
    obj.vertex_groups.clear()
    groups: dict[str, bpy.types.VertexGroup] = {}
    for v in obj.data.vertices:
        for bone, w in SK.clean(rule(v.co)).items():
            vg = groups.get(bone)
            if vg is None:
                vg = groups[bone] = obj.vertex_groups.new(name=bone)
            vg.add([v.index], w, "REPLACE")
    return len(obj.data.vertices)


def _edit(arm: bpy.types.Object) -> None:
    bpy.context.view_layer.objects.active = arm
    arm.select_set(True)
    bpy.ops.object.mode_set(mode="EDIT")


def _object_mode() -> None:
    bpy.ops.object.mode_set(mode="OBJECT")


# --- 1. half-cape -------------------------------------------------------------------


def _transform(arm: bpy.types.Object, owner: str, target: str, name: str,
               x: tuple[float, float, float, float], z: tuple[float, float, float, float],
               z_from: str = "Z") -> None:
    """TRANSFORM constraint, swing of the target's local X/Z (degrees) -> owner (local).

    ``x``/``z`` = (from_min, from_max, to_min, to_max): the "from" pair is the
    range of that SOURCE axis, the "to" pair the range of that OUTPUT axis
    (whose source is ``map_to_*_from``). Outside the range the copy is clamped
    (no extrapolation).
    """
    con = arm.pose.bones[owner].constraints.new("TRANSFORM")
    con.name = name
    con.target, con.subtarget = arm, target
    con.owner_space = con.target_space = "LOCAL"
    con.map_from, con.map_to = "ROTATION", "ROTATION"
    con.from_rotation_mode = "SWING_TWIST_Y"
    con.use_motion_extrapolate = False
    con.mix_mode_rot = "AFTER"
    r = math.radians
    con.from_min_x_rot, con.from_max_x_rot, con.to_min_x_rot, con.to_max_x_rot = (r(v) for v in x)
    con.from_min_z_rot, con.from_max_z_rot, con.to_min_z_rot, con.to_max_z_rot = (r(v) for v in z)
    con.map_to_x_from, con.map_to_y_from, con.map_to_z_from = "X", "Y", z_from


def fix_half_cape(arm: bpy.types.Object) -> dict[str, object]:
    before = {n: (tuple(round(c, 4) for c in arm.data.bones[n].head_local),
                  tuple(round(c, 4) for c in arm.data.bones[n].tail_local)) for n in ("cape_arm.L_01", "cape_arm.L_02")}
    _edit(arm)
    eb = arm.data.edit_bones
    upper = eb["upper_arm.L"]
    first, second = eb["cape_arm.L_01"], eb["cape_arm.L_02"]
    len1, len2 = first.length, second.length
    second.use_connect = False
    # Same head, direction and roll as upper_arm.L: identical rest frames, so a
    # copied local rotation is the same world rotation about the same pivot.
    first.head = upper.head.copy()
    first.tail = upper.head + (upper.tail - upper.head).normalized() * len1
    first.roll = upper.roll
    second.head = first.tail.copy()
    second.tail = first.tail + (upper.tail - upper.head).normalized() * len2
    second.roll = upper.roll
    second.use_connect = True
    rest_gap = (first.matrix.to_3x3() - upper.matrix.to_3x3())
    frame_error = max(abs(v) for row in rest_gap for v in row)
    _object_mode()
    pb = arm.pose.bones["cape_arm.L_01"]
    old = pb.constraints.get("follow_arm")
    if old is not None:
        pb.constraints.remove(old)
    f = C.CAPE_ARM_FOLLOW
    # Local X of upper_arm.L is world -Y: + = forward swing, - = backward swing.
    # Local Z is the front axis: + = abduction of the left arm (outward), - = adduction.
    # Two constraints because one TRANSFORM maps each axis linearly over one range.
    _transform(arm, "cape_arm.L_01", "upper_arm.L", "follow_arm_back_out",
               (-180.0, 0.0, -180.0 * f["back"], 0.0), (0.0, 180.0, 0.0, 180.0 * f["out"]))
    _transform(arm, "cape_arm.L_01", "upper_arm.L", "follow_arm_fwd_in",
               (0.0, 180.0, 0.0, 180.0 * f["forward"]), (-180.0, 0.0, -180.0 * f["in"], 0.0))
    _counter_chest_twist(arm)
    chains = SK.build_chains()
    counts = {name: reskin(bpy.data.objects[name], lambda p: cape_v006(chains, p)) for name in CAPE_OBJECTS}
    return {"bones_moved": before, "rest_frame_error_vs_upper_arm": round(frame_error, 6),
            "constraints": ["follow_arm_back_out", "follow_arm_fwd_in", "counter_chest_twist"],
            "follow": C.CAPE_ARM_FOLLOW, "chest_twist_counter": C.CAPE_CHEST_TWIST_COUNTER,
            "top_arm_share": C.CAPE_TOP_ARM_SHARE, "reskinned_vertices": counts}


def _counter_chest_twist(arm: bpy.types.Object) -> None:
    """Undoes part of a chest twist toward the sword side on the arm-side cape chain.

    The chain hangs from the chest, the coat under it from the lower spine: a
    chest twist toward the sword side (negative local Y twist of ``chest``)
    slid the lower cape across the coat (v005 T11). Measured: the opposite
    twist (T09) needs no correction and gets worse with one, so only the
    negative half is mapped (negative chest twist -> positive twist here).
    """
    con = arm.pose.bones["cape_arm.L_01"].constraints.new("TRANSFORM")
    con.name = "counter_chest_twist"
    con.target, con.subtarget = arm, "chest"
    con.owner_space = con.target_space = "LOCAL"
    con.map_from, con.map_to = "ROTATION", "ROTATION"
    con.from_rotation_mode = "SWING_TWIST_Y"
    con.use_motion_extrapolate = False
    con.mix_mode_rot = "AFTER"
    half = math.pi / 2.0
    con.from_min_y_rot, con.from_max_y_rot = -half, 0.0
    con.to_min_y_rot, con.to_max_y_rot = half * C.CAPE_CHEST_TWIST_COUNTER, 0.0
    con.map_to_x_from, con.map_to_y_from, con.map_to_z_from = "X", "Y", "Z"


def cape_v006(chains: dict[str, SK.Chain], p: Vector) -> Weights:
    """v005 cape rule; the top over the shoulder cap shares ``cape_arm.L_01``."""
    theta = math.atan2(p.y, p.x) % (2.0 * math.pi)
    w_top = smoothstep((p.z - SK.CAPE_TOP_Z) / 0.1)
    arm_side = 1.0 - smoothstep((theta - 1.9) / 0.5)
    lateral = smoothstep((p.y - 0.08) / 0.08)  # 0 at the neck, 1 over the shoulder cap
    on_arm = arm_side * C.CAPE_TOP_ARM_SHARE * lateral
    top = {"clavicle.L": arm_side - on_arm, "cape_arm.L_01": on_arm, "chest": 1.0 - arm_side}
    w_back = smoothstep((theta - 2.5) / 0.6)
    low_arm = smoothstep((1.27 - p.z) / 0.1)
    low_back = smoothstep((1.30 - p.z) / 0.1)
    lower = {
        "cape_arm.L_01": (1.0 - w_back) * (1.0 - low_arm), "cape_arm.L_02": (1.0 - w_back) * low_arm,
        "cape_back_01": w_back * (1.0 - low_back), "cape_back_02": w_back * low_back,
    }
    share = C.CAPE_TORSO_SHARE * w_back
    return SK._mix((w_top, top), ((1.0 - w_top) * (1.0 - share), lower),
                   ((1.0 - w_top) * share, SK.upper_body(chains, p)))


# --- 2. coat -------------------------------------------------------------------------


def fix_coat(arm: bpy.types.Object) -> dict[str, object]:
    back_fwd_factor = C.COAT_FOLLOW["back"][0]
    peak, zero = C.COAT_BACK_FWD_PEAK, C.COAT_BACK_FWD_ZERO
    changed = []
    for side in ("R", "L"):
        owner = f"coat_back.{side}_01"
        pb = arm.pose.bones[owner]
        old = pb.constraints.get("follow_thigh_fwd")
        if old is None:
            raise RuntimeError(f"{owner}: v005 constraint follow_thigh_fwd missing")
        pb.constraints.remove(old)
        lift = peak * back_fwd_factor
        _transform(arm, owner, f"thigh.{side}", "follow_thigh_fwd", (0.0, peak, 0.0, lift), (0.0, 0.0, 0.0, 0.0))
        _transform(arm, owner, f"thigh.{side}", "release_thigh_fwd", (peak, zero, 0.0, -lift), (0.0, 0.0, 0.0, 0.0))
        changed.append(owner)
    flare, drape = C.COAT_FRONT_FLARE_DEG, C.COAT_FRONT_FOLLOW_SHIN
    for side, out in (("R", -1.0), ("L", 1.0)):  # local +Z swings a down bone toward +Y
        if flare:
            # Thigh flexion (local X, 0..120) -> outward swing of the panel (local Z).
            _transform(arm, f"coat_front.{side}_01", f"thigh.{side}", "flare_thigh",
                       (0.0, 120.0, 0.0, 0.0), (0.0, 0.0, 0.0, out * flare * 120.0 / 90.0), z_from="X")
            changed.append(f"coat_front.{side}_01")
        if drape:
            _transform(arm, f"coat_front.{side}_02", f"shin.{side}", "follow_shin",
                       (-150.0, 0.0, -150.0 * drape, 0.0), (0.0, 0.0, 0.0, 0.0))
            changed.append(f"coat_front.{side}_02")
    chains = SK.build_chains()
    count = reskin(bpy.data.objects["coat"], lambda p: coat_v006(chains, p))
    return {"back_panels": changed, "back_forward_profile_deg": {"0": 0.0, str(peak): peak * back_fwd_factor,
                                                                 str(zero): 0.0},
            "front_flare_deg_at_90": flare, "front_follow_shin": drape,
            "front_skirt_top_z": C.COAT_FRONT_SKIRT_TOP_Z, "front_skirt_ramp": C.COAT_FRONT_SKIRT_RAMP,
            "reskinned_vertices": count}


def coat_v006(chains: dict[str, SK.Chain], p: Vector) -> Weights:
    """v005 coat rule; the front waist hands over to the skirt bones higher up."""
    angle = abs(math.atan2(p.y, p.x))
    w_back = smoothstep((angle - 0.9) / 1.3)
    w_v005 = smoothstep((SK.WAIST_BONE_Z - p.z) / 0.14)
    w_front = smoothstep((C.COAT_FRONT_SKIRT_TOP_Z - p.z) / C.COAT_FRONT_SKIRT_RAMP)
    w_skirt = (1.0 - w_back) * max(w_front, w_v005) + w_back * w_v005
    upper = SK.upper_body(chains, p)
    s_r = 1.0 - smoothstep((p.y + 0.02) / 0.04)
    low = smoothstep((SK.COAT_MID_Z + 0.08 - p.z) / 0.16)
    skirt: Weights = {}
    for side, s in (("R", s_r), ("L", 1.0 - s_r)):
        for part, w in (("front", 1.0 - w_back), ("back", w_back)):
            skirt[f"coat_{part}.{side}_01"] = s * w * (1.0 - low)
            skirt[f"coat_{part}.{side}_02"] = s * w * low
    return SK._mix((1.0 - w_skirt, upper), (w_skirt, skirt))


# --- 3. collar -----------------------------------------------------------------------


def fix_collar(arm: bpy.types.Object) -> dict[str, object]:
    if "collar_back" in arm.data.bones:
        raise RuntimeError("collar_back already exists")
    _edit(arm)
    eb = arm.data.edit_bones
    neck = eb["neck"]
    bone = eb.new("collar_back")
    bone.head, bone.tail, bone.roll = neck.head.copy(), neck.tail.copy(), neck.roll
    bone.parent = eb["chest"]
    bone.use_connect = False
    bone.use_deform = True
    _object_mode()
    arm.data.collections["DEF"].assign(arm.data.bones["collar_back"])
    pb = arm.pose.bones["collar_back"]
    pb.rotation_mode = "QUATERNION"
    for target, influence in (("neck", C.COLLAR_FOLLOW_NECK), ("head", C.COLLAR_FOLLOW_HEAD)):
        con = pb.constraints.new("COPY_ROTATION")
        con.name = f"follow_{target}"
        con.target, con.subtarget = arm, target
        con.owner_space = con.target_space = "LOCAL"
        con.mix_mode = "AFTER"
        con.influence = influence
    chains = SK.build_chains()
    count = reskin(bpy.data.objects["collar"], lambda p: collar_v006(chains, p))
    return {"bone": "collar_back", "parent": "chest", "follow": {"neck": C.COLLAR_FOLLOW_NECK,
                                                                 "head": C.COLLAR_FOLLOW_HEAD},
            "reskinned_vertices": count}


def collar_v006(chains: dict[str, SK.Chain], p: Vector) -> Weights:
    """Back and sides of the collar ride on ``collar_back``; the open front keeps v005."""
    angle = abs(math.atan2(p.y, p.x))  # 0 front .. pi back
    w_angle = smoothstep((angle - C.COLLAR_SIDE_START) / C.COLLAR_SIDE_RAMP)
    w_height = smoothstep((p.z - 1.46) / 0.12)
    w_cb = w_angle * w_height
    return SK._mix((1.0 - w_cb, SK.collar(chains, p)), (w_cb, {"collar_back": 1.0}))


# --- 4. volume (ankle, hip) -----------------------------------------------------------


def fix_volume() -> dict[str, object]:
    changed = []
    for obj in bpy.data.collections[C.SOURCE_COLLECTION].objects:
        if obj.type == "MESH" and obj.name.startswith(C.PRESERVE_VOLUME_EXTRA):
            mod = obj.modifiers[0]
            if mod.type != "ARMATURE":
                raise RuntimeError(f"{obj.name}: first modifier is not the rig")
            mod.use_deform_preserve_volume = True
            changed.append(obj.name)
    return {"preserve_volume": sorted(changed)}


def apply_all(arm: bpy.types.Object) -> dict[str, object]:
    return {"half_cape": fix_half_cape(arm), "coat": fix_coat(arm), "collar": fix_collar(arm),
            "volume": fix_volume()}

