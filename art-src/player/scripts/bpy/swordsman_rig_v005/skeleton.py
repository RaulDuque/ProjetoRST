"""Armature of v005: deform bones, controls and constraints.

Layout (``.R`` = character's right = near side, -Y; see ``rig_config``)::

    root (CTRL, global motion)
    └── pelvis (hips control + deform)
        ├── spine_01 ── spine_02 ── chest
        │                 │          ├── neck ── head
        │                 │          ├── clavicle.R ── upper_arm.R ── forearm.R ── hand.R ── sword
        │                 │          └── clavicle.L ── upper_arm.L ── forearm.L ── hand.L
        │                 │                 └── cape_arm.L_01 ── cape_arm.L_02
        │                 └── cape_back_01 ── cape_back_02
        ├── thigh.R ── shin.R ── foot.R ── toe.R      (same for .L)
        └── coat_front.R_01 ── coat_front.R_02        (front.L, back.R, back.L)
    CTRL_foot_ik.R / .L, CTRL_knee_pole.R / .L, CTRL_hand_ik.R, CTRL_elbow_pole.R (children of root)

The rest pose is the v004 pose (not a T-pose): the sword arm already holds
the sabre down and forward. Every bone's roll is aligned so its local Z axis
points to the character's front (+X) when the bone is vertical-ish, which
makes the local rotation of a thigh and of a coat panel comparable (the
coat copies the thigh swing in LOCAL space).

IK (legs, sword hand) is built with influence 0: the validation poses are
FK, and the IK chains are exercised by a dedicated test in ``rig_validate``.
"""

from __future__ import annotations

import math
from dataclasses import dataclass

import bpy
from mathutils import Vector

import params as P
import rig_config as C
import sword as v3sword  # v003 sword module: grip centre, guard and axis of the sabre

Vec3 = tuple[float, float, float]
FORWARD = Vector((1.0, 0.0, 0.0))
UP = Vector((0.0, 0.0, 1.0))


@dataclass(frozen=True)
class BoneSpec:
    name: str
    head: Vec3
    tail: Vec3
    parent: str | None
    collection: str  # DEF, CTRL, COAT, CAPE, SWORD
    deform: bool = True
    connect: bool = False
    roll_to: Vec3 = (1.0, 0.0, 0.0)  # local Z aligned to this (front by default)


def _v(p: Vec3 | Vector) -> Vec3:
    return (float(p[0]), float(p[1]), float(p[2]))


def _offset(p: Vec3, dx: float = 0.0, dy: float = 0.0, dz: float = 0.0) -> Vec3:
    return (p[0] + dx, p[1] + dy, p[2] + dz)


def _sample_surface(obj: bpy.types.Object, theta: float, z: float, shrink: float) -> Vec3:
    """Point inside a cloth mesh at angle theta (0 = front) and height z."""
    best, best_d = None, 1e9
    for v in obj.data.vertices:
        co = v.co
        ang = math.atan2(co.y, co.x)
        dang = (ang - theta + math.pi) % (2.0 * math.pi) - math.pi
        d = (dang * 0.25) ** 2 + (co.z - z) ** 2
        if d < best_d:
            best, best_d = co, d
    if best is None:
        raise RuntimeError(f"{obj.name} has no vertices")
    return (best.x * shrink, best.y * shrink, z)


def _cape_hem(obj: bpy.types.Object, theta: float) -> float:
    zs = [v.co.z for v in obj.data.vertices
          if abs((math.atan2(v.co.y, v.co.x) - theta + math.pi) % (2.0 * math.pi) - math.pi) < 0.12]
    return min(zs) if zs else 1.05


def bone_specs() -> list[BoneSpec]:
    coat = bpy.data.objects["coat"]
    cape = bpy.data.objects["half_cape"]
    specs = [
        BoneSpec("root", (0.0, 0.0, 0.0), (0.3, 0.0, 0.0), None, "CTRL", deform=False, roll_to=(0, 0, 1)),
        BoneSpec("pelvis", (0.0, 0.0, 0.95), (0.0, 0.0, 1.06), "root", "DEF"),
        BoneSpec("spine_01", (0.0, 0.0, 1.06), (0.006, 0.0, 1.18), "pelvis", "DEF", connect=True),
        BoneSpec("spine_02", (0.006, 0.0, 1.18), (0.01, 0.0, 1.31), "spine_01", "DEF", connect=True),
        BoneSpec("chest", (0.01, 0.0, 1.31), (0.002, 0.0, 1.445), "spine_02", "DEF", connect=True),
        BoneSpec("neck", (-0.008, 0.0, 1.47), (0.0, 0.0, 1.585), "chest", "DEF"),
        BoneSpec("head", (0.0, 0.0, 1.585), (0.012, 0.0, 1.81), "neck", "DEF", connect=True),
    ]
    grip = v3sword.GRIP_CENTRE
    forearm_dir = (Vector(P.WRIST_NEAR) - Vector(P.ELBOW_NEAR)).normalized()
    hand_r_tail = grip + forearm_dir * 0.035
    far_down = Vector((0.03, 0.0, -1.0)).normalized()
    hand_l_tail = Vector(P.WRIST_FAR) + far_down * 0.09
    for side, sign, sh, el, wr, tail in (("R", -1.0, P.SHOULDER_NEAR, P.ELBOW_NEAR, P.WRIST_NEAR, hand_r_tail),
                                         ("L", 1.0, P.SHOULDER_FAR, P.ELBOW_FAR, P.WRIST_FAR, hand_l_tail)):
        specs += [
            BoneSpec(f"clavicle.{side}", (0.0, sign * 0.035, 1.43), _v(sh), "chest", "DEF", roll_to=(0, 0, 1)),
            BoneSpec(f"upper_arm.{side}", _v(sh), _v(el), f"clavicle.{side}", "DEF", connect=True),
            BoneSpec(f"forearm.{side}", _v(el), _v(wr), f"upper_arm.{side}", "DEF", connect=True),
            BoneSpec(f"hand.{side}", _v(wr), _v(tail), f"forearm.{side}", "DEF", connect=True),
        ]
    specs.append(BoneSpec("sword", _v(grip), _v(v3sword.GUARD + v3sword.AXIS * 0.15), "hand.R", "SWORD"))
    for side, (hip, knee, ankle) in (("R", P.LEG_NEAR), ("L", P.LEG_FAR)):
        ball = (ankle[0] + 0.13, ankle[1], 0.03)
        tip = (ankle[0] + 0.2, ankle[1], 0.026)
        specs += [
            BoneSpec(f"thigh.{side}", _v(hip), _v(knee), "pelvis", "DEF"),
            BoneSpec(f"shin.{side}", _v(knee), _v(ankle), f"thigh.{side}", "DEF", connect=True),
            BoneSpec(f"foot.{side}", _v(ankle), ball, f"shin.{side}", "DEF", connect=True, roll_to=(0, 0, 1)),
            BoneSpec(f"toe.{side}", ball, tip, f"foot.{side}", "DEF", connect=True, roll_to=(0, 0, 1)),
            # IK control: same rest as the foot, so the copied rotation matches at rest.
            BoneSpec(f"CTRL_foot_ik.{side}", _v(ankle), ball, "root", "CTRL", deform=False, roll_to=(0, 0, 1)),
            BoneSpec(f"CTRL_knee_pole.{side}", _offset(knee, 0.4), _offset(knee, 0.45), "root", "CTRL",
                     deform=False, roll_to=(0, 0, 1)),
        ]
    specs += [
        BoneSpec("CTRL_hand_ik.R", _v(P.WRIST_NEAR), _v(hand_r_tail), "root", "CTRL", deform=False),
        BoneSpec("CTRL_elbow_pole.R", _offset(P.ELBOW_NEAR, -0.35, -0.1), _offset(P.ELBOW_NEAR, -0.4, -0.1),
                 "root", "CTRL", deform=False, roll_to=(0, 0, 1)),
    ]
    # Coat panels: two bones per chain, from the waist down to the hem.
    for side, sign in (("R", -1.0), ("L", 1.0)):
        for part, theta, hem in (("front", 0.62, P.HEM_FRONT_Z + 0.02), ("back", 2.45, P.HEM_BACK_Z + 0.02)):
            th = sign * theta
            top = _sample_surface(coat, th, 1.06, 0.82)
            mid = _sample_surface(coat, th, 0.72, 0.85)
            low = _sample_surface(coat, th, hem, 0.88)
            specs += [
                BoneSpec(f"coat_{part}.{side}_01", top, mid, "pelvis", "COAT"),
                BoneSpec(f"coat_{part}.{side}_02", mid, low, f"coat_{part}.{side}_01", "COAT", connect=True),
            ]
    # Half-cape: one chain over the far arm, one across the back.
    for chain, theta, parent in (("cape_arm.L", 1.6, "clavicle.L"), ("cape_back", 2.75, "spine_02")):
        hem = _cape_hem(cape, theta) + 0.02
        top = _sample_surface(cape, theta, 1.43, 0.9)
        mid = _sample_surface(cape, theta, (1.43 + hem) / 2.0 + 0.02, 0.92)
        low = _sample_surface(cape, theta, hem, 0.94)
        specs += [
            BoneSpec(f"{chain}_01", top, mid, parent, "CAPE"),
            BoneSpec(f"{chain}_02", mid, low, f"{chain}_01", "CAPE", connect=True),
        ]
    return specs


def deform_bone_names() -> list[str]:
    return [s.name for s in bone_specs() if s.deform]


def _signed_angle(u: Vector, v: Vector, normal: Vector) -> float:
    a = u.angle(v)
    if u.cross(v).angle(normal) < 1.0:
        a = -a
    return a


def _pole_angle(base: bpy.types.Bone, ik: bpy.types.Bone, pole: Vector) -> float:
    """Pole angle that keeps the chain in its rest orientation (standard formula)."""
    base_head, base_tail = base.head_local, base.tail_local
    pole_normal = (ik.tail_local - base_head).cross(pole - base_head)
    projected = pole_normal.cross(base_tail - base_head)
    x_axis = base.matrix_local.to_3x3().col[0]
    return _signed_angle(x_axis, projected, base_tail - base_head)


def build_armature() -> bpy.types.Object:
    if bpy.data.objects.get(C.ARMATURE_NAME) is not None:
        raise RuntimeError(f"{C.ARMATURE_NAME} already exists in the source file")
    specs = bone_specs()
    data = bpy.data.armatures.new(C.ARMATURE_NAME)
    arm = bpy.data.objects.new(C.ARMATURE_NAME, data)
    bpy.context.scene.collection.objects.link(arm)
    data.display_type = "OCTAHEDRAL"
    bpy.context.view_layer.objects.active = arm
    bpy.ops.object.mode_set(mode="EDIT")
    for spec in specs:
        eb = data.edit_bones.new(spec.name)
        eb.head, eb.tail = Vector(spec.head), Vector(spec.tail)
        eb.align_roll(Vector(spec.roll_to))
        eb.use_deform = spec.deform
    for spec in specs:
        if spec.parent:
            eb = data.edit_bones[spec.name]
            eb.parent = data.edit_bones[spec.parent]
            eb.use_connect = spec.connect
    bpy.ops.object.mode_set(mode="OBJECT")
    for spec in specs:
        coll = data.collections.get(spec.collection) or data.collections.new(spec.collection)
        coll.assign(data.bones[spec.name])
    for pb in arm.pose.bones:
        pb.rotation_mode = "QUATERNION"
    add_constraints(arm)
    return arm


def _ik(arm: bpy.types.Object, owner: str, target: str, pole: str, base: str) -> None:
    pb = arm.pose.bones[owner]
    con = pb.constraints.new("IK")
    con.name = "IK"
    con.target, con.subtarget = arm, target
    con.pole_target, con.pole_subtarget = arm, pole
    con.chain_count = 2
    con.use_stretch = False
    bones = arm.data.bones
    con.pole_angle = _pole_angle(bones[base], bones[owner], bones[pole].head_local)
    con.influence = 0.0


def _copy_rot(arm: bpy.types.Object, owner: str, target: str, influence: float, space: str,
              name: str) -> None:
    con = arm.pose.bones[owner].constraints.new("COPY_ROTATION")
    con.name = name
    con.target, con.subtarget = arm, target
    con.owner_space = con.target_space = space
    if space == "LOCAL":
        con.mix_mode = "AFTER"
    con.influence = influence


def add_constraints(arm: bpy.types.Object) -> None:
    for side in ("R", "L"):
        _ik(arm, f"shin.{side}", f"CTRL_foot_ik.{side}", f"CTRL_knee_pole.{side}", f"thigh.{side}")
        _copy_rot(arm, f"foot.{side}", f"CTRL_foot_ik.{side}", 0.0, "WORLD", "IK_foot")
        for part in ("front", "back"):
            forward, backward = C.COAT_FOLLOW[part]
            _swing_follow(arm, f"coat_{part}.{side}_01", f"thigh.{side}", "follow_thigh_fwd", 0.0, 120.0, forward)
            _swing_follow(arm, f"coat_{part}.{side}_01", f"thigh.{side}", "follow_thigh_back", -120.0, 0.0, backward)
        # Knee bend (shin swinging back = negative local X) lifts the lower back panel.
        _swing_follow(arm, f"coat_back.{side}_02", f"shin.{side}", "follow_shin", -150.0, 0.0,
                      C.COAT_BACK_FOLLOW_SHIN)
    _ik(arm, "forearm.R", "CTRL_hand_ik.R", "CTRL_elbow_pole.R", "upper_arm.R")
    _copy_rot(arm, "hand.R", "CTRL_hand_ik.R", 0.0, "WORLD", "IK_hand")
    _cape_follow_arm(arm)


def _swing_follow(arm: bpy.types.Object, owner: str, target: str, name: str, lo: float, hi: float,
                  factor: float) -> None:
    """Copies the target's local X swing within [lo, hi] degrees, scaled by ``factor``.

    Local X of the leg and coat bones is world -Y (rolls aligned to the
    front), so a positive value is a forward swing. Outside the range the
    copy is clamped, which gives different follow factors forward and back.
    """
    con = arm.pose.bones[owner].constraints.new("TRANSFORM")
    con.name = name
    con.target, con.subtarget = arm, target
    con.owner_space = con.target_space = "LOCAL"
    con.map_from, con.map_to = "ROTATION", "ROTATION"
    con.from_rotation_mode = "SWING_TWIST_Y"
    con.use_motion_extrapolate = False
    con.mix_mode_rot = "AFTER"
    con.from_min_x_rot, con.from_max_x_rot = math.radians(lo), math.radians(hi)
    con.to_min_x_rot, con.to_max_x_rot = math.radians(lo) * factor, math.radians(hi) * factor
    con.map_to_x_from, con.map_to_y_from, con.map_to_z_from = "X", "Y", "Z"


def _cape_follow_arm(arm: bpy.types.Object) -> None:
    """The arm-side cape chain copies the far arm's abduction and backward swing.

    Copying the forward swing too would drive the back of the cape through the
    torso (the cape wraps the arm from outside), so that range is clamped to 0.
    Local X of a down-pointing bone here is world -Y: a backward swing is a
    negative local X rotation; local Z is the forward axis (abduction).
    """
    con = arm.pose.bones["cape_arm.L_01"].constraints.new("TRANSFORM")
    con.name = "follow_arm"
    con.target, con.subtarget = arm, "upper_arm.L"
    con.owner_space = con.target_space = "LOCAL"
    con.map_from, con.map_to = "ROTATION", "ROTATION"
    con.from_rotation_mode = "SWING_TWIST_Y"  # a forward raise must not leak into Z
    con.use_motion_extrapolate = False
    con.mix_mode_rot = "AFTER"
    half = math.pi / 2.0
    con.from_min_x_rot, con.from_max_x_rot = -half, 0.0
    con.to_min_x_rot, con.to_max_x_rot = -half * C.CAPE_FOLLOW_ARM, 0.0
    con.from_min_z_rot, con.from_max_z_rot = -half, half
    con.to_min_z_rot, con.to_max_z_rot = -half * C.CAPE_FOLLOW_ARM, half * C.CAPE_FOLLOW_ARM
    con.map_to_x_from, con.map_to_y_from, con.map_to_z_from = "X", "Y", "Z"


def set_ik(arm: bpy.types.Object, enabled: bool) -> None:
    """IK/FK switch: influence of the IK chains (legs and sword hand)."""
    value = 1.0 if enabled else 0.0
    for side in ("R", "L"):
        arm.pose.bones[f"shin.{side}"].constraints["IK"].influence = value
        arm.pose.bones[f"foot.{side}"].constraints["IK_foot"].influence = value
    arm.pose.bones["forearm.R"].constraints["IK"].influence = value
    arm.pose.bones["hand.R"].constraints["IK_hand"].influence = value
