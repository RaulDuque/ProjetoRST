"""Validation poses of v005 (P01-P13) and deformation test poses (T01-T11).

These are static art tests, not animations: they change no gameplay value
(reach, hitboxes, damage, timings). Rotations are written in **world axes
at rest** and applied FK, parent first, so a child inherits its parent's
motion. With the model facing +X and +Z up:

- ``Y`` (pitch): positive tilts an up-pointing bone forward (lean, nod) and
  swings a down-pointing bone backward. Thigh forward = negative Y; knee
  bend (shin back) = positive Y; arm raised forward = negative Y.
- ``X`` (roll): positive tilts up-pointing bones toward -Y (the right/near
  side) and swings down-pointing bones toward +Y. Right arm out to the side
  = negative X; left arm out = positive X.
- ``Z`` (yaw): positive turns the front toward +Y; it brings the right
  (sword) shoulder forward.

``root`` carries the global motion; grounded poses are then lowered or
raised so the lowest boot point touches z = 0, airborne poses are lifted.
"""

from __future__ import annotations

from dataclasses import dataclass, field

import bpy
from mathutils import Quaternion, Vector

import rig_config as C
from skeleton import set_ik

AXES = {"X": Vector((1.0, 0.0, 0.0)), "Y": Vector((0.0, 1.0, 0.0)), "Z": Vector((0.0, 0.0, 1.0))}
Turn = tuple[str, float]
FEET = ("boot_sole_near", "boot_sole_far", "boot_heel_near", "boot_heel_far", "boot_foot_near", "boot_foot_far")


@dataclass(frozen=True)
class Pose:
    key: str
    label: str
    turns: dict[str, tuple[Turn, ...]] = field(default_factory=dict)
    ground: str = "feet"  # "feet", "all" (lowest point of the body, sabre excluded) or "air"
    lift: float = 0.0  # extra height after grounding (airborne poses)
    shift_x: float = 0.0


def _q(turns: tuple[Turn, ...]) -> Quaternion:
    q = Quaternion()
    for axis, deg in turns:
        q = Quaternion(AXES[axis], deg * 3.141592653589793 / 180.0) @ q
    return q


def reset(arm: bpy.types.Object) -> None:
    set_ik(arm, False)
    for pb in arm.pose.bones:
        pb.rotation_quaternion = Quaternion()
        pb.location = Vector()
        pb.scale = Vector((1.0, 1.0, 1.0))


def _set_world_rotation(arm: bpy.types.Object, bone: str, q_world: Quaternion) -> None:
    rest = arm.data.bones[bone].matrix_local.to_quaternion()
    arm.pose.bones[bone].rotation_quaternion = rest.inverted() @ q_world @ rest


def _move_root(arm: bpy.types.Object, world: Vector) -> None:
    rest = arm.data.bones["root"].matrix_local.to_3x3()
    arm.pose.bones["root"].location = rest.inverted() @ world


def lowest_z(objects: list[bpy.types.Object]) -> float:
    depsgraph = bpy.context.evaluated_depsgraph_get()
    low = 1e9
    for obj in objects:
        ev = obj.evaluated_get(depsgraph)
        mesh = ev.to_mesh()
        try:
            for v in mesh.vertices:
                low = min(low, (obj.matrix_world @ v.co).z)
        finally:
            ev.to_mesh_clear()
    return low


def apply(arm: bpy.types.Object, pose: Pose, body: list[bpy.types.Object]) -> float:
    """Poses the rig; returns the root height that was applied."""
    reset(arm)
    for bone, turns in pose.turns.items():
        _set_world_rotation(arm, bone, _q(turns))
    bpy.context.view_layer.update()
    feet = [bpy.data.objects[n] for n in FEET]
    grounded = [o for o in body if not o.name.startswith(("blade", "sword_"))]  # the sabre may touch or cross the floor
    low = lowest_z(grounded if pose.ground == "all" else feet)
    height = -low + pose.lift
    _move_root(arm, Vector((pose.shift_x, 0.0, height)))
    bpy.context.view_layer.update()
    return height


def mirrored(turns: dict[str, tuple[Turn, ...]]) -> dict[str, tuple[Turn, ...]]:
    """Swaps .R/.L limbs (walk cycle second half); X and Z flip sign."""
    out: dict[str, tuple[Turn, ...]] = {}
    for bone, ts in turns.items():
        other = bone.replace(".R", ".#").replace(".L", ".R").replace(".#", ".L")
        if other.startswith(("clavicle", "upper_arm", "forearm", "hand")) or bone == "sword":
            out[bone] = ts  # arms are not symmetric (one holds the sword): keep them
            continue
        out[other] = tuple((a, -d if a in ("X", "Z") else d) for a, d in ts)
    return out


# --- P01-P13 ---------------------------------------------------------------------

WALK_R = {
    "pelvis": (("Z", -4.0),), "spine_01": (("Y", 3.0),), "chest": (("Z", 5.0),),
    "thigh.R": (("Y", -24.0),), "shin.R": (("Y", 6.0),),
    "thigh.L": (("Y", 18.0),), "shin.L": (("Y", 14.0),),
    "upper_arm.L": (("Y", -16.0),), "forearm.L": (("Y", -12.0),),
    "upper_arm.R": (("Y", 8.0),),
}

POSES = (
    Pose("P01_neutral", "P01 Neutral"),
    Pose("P02_walk_contact", "P02 Walk contact", WALK_R),
    Pose("P03_run_extreme", "P03 Run extreme", {
        "spine_01": (("Y", 8.0),), "spine_02": (("Y", 6.0),), "chest": (("Y", 4.0), ("Z", 6.0)),
        "head": (("Y", -12.0),),
        "thigh.R": (("Y", -50.0),), "shin.R": (("Y", 45.0),), "foot.R": (("Y", -8.0),),
        "thigh.L": (("Y", 30.0),), "shin.L": (("Y", 70.0),), "foot.L": (("Y", 15.0),),
        "upper_arm.L": (("Y", -40.0),), "forearm.L": (("Y", -50.0),),
        "upper_arm.R": (("Y", 25.0),), "hand.R": (("Y", 60.0),),
        "coat_back.R_01": (("Y", 14.0),), "coat_back.L_01": (("Y", 14.0),),
        "coat_back.R_02": (("Y", 14.0),), "coat_back.L_02": (("Y", 14.0),),
        "cape_back_01": (("Y", 27.0),), "cape_arm.L_01": (("Y", 12.0),),
    }),
    Pose("P04_jump_rise", "P04 Jump rise", {
        "spine_01": (("Y", 5.0),), "head": (("Y", -6.0),),
        "thigh.R": (("Y", -60.0),), "shin.R": (("Y", 80.0),), "foot.R": (("Y", 15.0),),
        "thigh.L": (("Y", -20.0),), "shin.L": (("Y", 40.0),), "foot.L": (("Y", 20.0),),
        "upper_arm.L": (("Y", -60.0),), "forearm.L": (("Y", -30.0),),
        "upper_arm.R": (("Y", -20.0),),
        "coat_back.R_01": (("Y", 8.0),), "coat_back.L_01": (("Y", 8.0),),
        "coat_back.R_02": (("Y", 8.0),), "coat_back.L_02": (("Y", 8.0),),
    }, ground="air", lift=0.35),
    Pose("P05_jump_fall", "P05 Jump fall", {
        "spine_01": (("Y", -4.0),),
        "thigh.R": (("Y", -25.0),), "shin.R": (("Y", 22.0),), "foot.R": (("Y", 10.0),),
        "thigh.L": (("Y", -8.0),), "shin.L": (("Y", 32.0),), "foot.L": (("Y", 18.0),),
        "upper_arm.L": (("X", 45.0), ("Y", -10.0)), "forearm.L": (("Y", -20.0),),
        "upper_arm.R": (("X", -30.0),),
        "coat_back.R_01": (("Y", 28.0),), "coat_back.L_01": (("Y", 28.0),),
        "coat_back.R_02": (("Y", 18.0),), "coat_back.L_02": (("Y", 18.0),),
        "coat_front.R_01": (("Y", -12.0),), "coat_front.L_01": (("Y", -12.0),),
        "cape_back_01": (("Y", 40.0),), "cape_back_02": (("Y", 18.0),), "cape_arm.L_01": (("Y", 20.0),),
    }, ground="air", lift=0.3),
    Pose("P06_basic_attack", "P06 Basic attack", {
        "pelvis": (("Z", 10.0),), "spine_01": (("Y", 6.0), ("Z", 6.0)), "spine_02": (("Z", 12.0),), "chest": (("Z", 14.0),),
        "upper_arm.R": (("Y", -60.0),), "forearm.R": (("Y", -25.0),),
        "thigh.R": (("Y", -35.0),), "shin.R": (("Y", 30.0),),
        "thigh.L": (("Y", 22.0),), "shin.L": (("Y", 10.0),),
        "upper_arm.L": (("Y", 25.0),),
        "coat_back.R_02": (("Y", 10.0),), "coat_back.L_02": (("Y", 10.0),),
    }),
    Pose("P07_crescent_slash", "P07 Crescent Slash", {
        "spine_01": (("Y", -6.0), ("Z", -6.0)), "spine_02": (("Z", -8.0),), "chest": (("Z", -8.0), ("Y", -4.0)),
        "upper_arm.R": (("Y", -140.0),), "forearm.R": (("Y", -20.0),),
        "thigh.R": (("Y", -30.0),), "shin.R": (("Y", 22.0),),
        "thigh.L": (("Y", 25.0),), "shin.L": (("Y", 15.0),),
        "upper_arm.L": (("Y", 20.0), ("X", 20.0)),
    }),
    Pose("P08_rising_edge", "P08 Rising Edge", {
        "spine_01": (("Y", -6.0), ("Z", 6.0)), "spine_02": (("Z", 6.0),), "chest": (("Z", 6.0),), "head": (("Y", -10.0),),
        "upper_arm.R": (("Y", -120.0),), "forearm.R": (("Y", -10.0),),
        "thigh.R": (("Y", -25.0),), "shin.R": (("Y", 15.0),),
        "thigh.L": (("Y", 15.0),), "shin.L": (("Y", 20.0),), "foot.L": (("Y", 20.0),),
        "upper_arm.L": (("Y", 30.0),),
    }),
    Pose("P09_dodge", "P09 Dodge", {
        "spine_01": (("Y", 15.0),), "spine_02": (("Y", 10.0),), "chest": (("Y", 5.0),), "head": (("Y", -6.0),),
        "thigh.R": (("Y", -55.0),), "shin.R": (("Y", 60.0),),
        "thigh.L": (("Y", 35.0),), "shin.L": (("Y", 40.0),), "foot.L": (("Y", 25.0),),
        "upper_arm.L": (("Y", 40.0),), "upper_arm.R": (("Y", 25.0),), "hand.R": (("Y", 55.0),),
        "coat_back.R_01": (("Y", 26.0),), "coat_back.L_01": (("Y", 26.0),),
        "coat_back.R_02": (("Y", 18.0),), "coat_back.L_02": (("Y", 18.0),),
        "cape_back_01": (("Y", 45.0),), "cape_arm.L_01": (("Y", 22.0),),
    }),
    Pose("P10_guard", "P10 Guard", {
        "spine_01": (("Y", 6.0),),
        "upper_arm.R": (("Y", -45.0),), "forearm.R": (("Y", -70.0),), "hand.R": (("Y", -25.0),),
        "upper_arm.L": (("Y", -35.0),), "forearm.L": (("Y", -60.0),),
        "thigh.R": (("Y", -20.0),), "shin.R": (("Y", 25.0),),
        "thigh.L": (("Y", 15.0),), "shin.L": (("Y", 25.0),),
    }),
    Pose("P11_hurt", "P11 Hurt", {
        "spine_01": (("Y", -10.0),), "spine_02": (("Y", -8.0),), "chest": (("Y", -5.0),), "head": (("Y", 14.0),),
        "upper_arm.L": (("Y", 30.0), ("X", 20.0)), "upper_arm.R": (("Y", 20.0), ("X", -15.0)),
        "thigh.R": (("Y", -15.0),), "shin.R": (("Y", 15.0),),
        "thigh.L": (("Y", 10.0),), "shin.L": (("Y", 20.0),),
        "coat_front.R_01": (("Y", -12.0),), "coat_front.L_01": (("Y", -12.0),),
    }),
    Pose("P12_magic_cast", "P12 Magic cast (Ember Bolt)", {
        "spine_01": (("Z", -7.0),), "spine_02": (("Z", -7.0),), "chest": (("Z", -6.0),),
        "upper_arm.L": (("Y", -85.0),), "forearm.L": (("Y", -5.0),),
        "upper_arm.R": (("Y", 20.0),),
        "thigh.L": (("Y", -20.0),), "shin.L": (("Y", 15.0),),
        "thigh.R": (("Y", 10.0),), "shin.R": (("Y", 8.0),),
    }),
    Pose("P13_death", "P13 Death (collapse)", {
        "thigh.R": (("Y", -80.0),), "shin.R": (("Y", 110.0),), "foot.R": (("Y", 30.0),),
        "thigh.L": (("Y", -70.0),), "shin.L": (("Y", 115.0),), "foot.L": (("Y", 30.0),),
        "spine_01": (("Y", 25.0),), "spine_02": (("Y", 15.0),), "chest": (("Y", 10.0),), "head": (("Y", 25.0),),
        "upper_arm.R": (("Y", 10.0),), "upper_arm.L": (("Y", -5.0),), "hand.R": (("Y", 70.0),),
        "coat_front.R_01": (("Y", -15.0),), "coat_front.L_01": (("Y", -15.0),),
        "coat_front.R_02": (("Y", -25.0),), "coat_front.L_02": (("Y", -25.0),),
    }, ground="all"),
)

# --- deformation tests --------------------------------------------------------------

TESTS = (
    Pose("T01_arm_down", "T01 Braço abaixado", {"upper_arm.R": (("Y", 20.0),), "forearm.R": (("Y", 40.0),)}),
    Pose("T02_arm_horizontal", "T02 Braços na horizontal", {
        "upper_arm.R": (("X", -75.0),), "upper_arm.L": (("X", 75.0),)}),
    Pose("T03_arm_raised", "T03 Braços elevados", {
        "upper_arm.R": (("Y", -160.0),), "upper_arm.L": (("Y", -160.0),)}),
    Pose("T04_arm_cross", "T04 Braço cruzando o peito", {"upper_arm.R": (("Y", -80.0), ("Z", 60.0))}),
    Pose("T05_arm_extended", "T05 Braço estendido (ataque)", {
        "upper_arm.R": (("Y", -85.0),), "forearm.R": (("Y", 35.0),)}),
    Pose("T06_head_up", "T06 Olhar para cima", {"neck": (("Y", -15.0),), "head": (("Y", -25.0),)}),
    Pose("T07_head_down", "T07 Olhar para baixo", {"neck": (("Y", 15.0),), "head": (("Y", 25.0),)}),
    Pose("T08_head_turn", "T08 Rotação lateral", {"neck": (("Z", 25.0),), "head": (("Z", 35.0),)}),
    Pose("T09_torso_twist", "T09 Rotação do tronco", {"spine_02": (("Z", 20.0),), "chest": (("Z", 25.0),)}),
    Pose("T10_squat", "T10 Agachamento (joelhos)", {
        "thigh.R": (("Y", -90.0),), "shin.R": (("Y", 120.0),), "foot.R": (("Y", -30.0),),
        "thigh.L": (("Y", -90.0),), "shin.L": (("Y", 120.0),), "foot.L": (("Y", -30.0),),
        "spine_01": (("Y", 20.0),)}),
    Pose("T11_cape_arm_up", "T11 Meia-capa com braço erguido", {
        "upper_arm.L": (("Y", -120.0), ("X", 40.0)), "chest": (("Z", -15.0),)}),
)


def store_action(arm: bpy.types.Object, name: str, frame: int = 1) -> bpy.types.Action:
    """Keys every pose bone (rotation, root location) into a fake-user action."""
    arm.animation_data_create()
    arm.animation_data.action = None
    for pb in arm.pose.bones:
        pb.keyframe_insert("rotation_quaternion", frame=frame)
    arm.pose.bones["root"].keyframe_insert("location", frame=frame)
    action = arm.animation_data.action
    action.name = name
    action.use_fake_user = True
    arm.animation_data.action = None
    return action


def all_body(collection: str = C.SOURCE_COLLECTION) -> list[bpy.types.Object]:
    return [o for o in bpy.data.collections[collection].objects if o.type == "MESH" and not o.hide_render]
