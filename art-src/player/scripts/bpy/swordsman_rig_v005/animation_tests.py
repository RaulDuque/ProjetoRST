"""Short motion tests of v005 (walk, basic attack, dodge). Not final animations.

They exist to watch the rig move: legs, arms, hips, coat, half-cape and
sword between keys. Timing uses ``TEST_FPS`` (24, an art choice for the
test only; the sprite frame rate is still undecided). The attack and the
dodge last 0.28 s in the game (``BASIC_ATTACK_TIME``, ``PLAYER_DODGE_TIME``
in ``src/core/constants.rs``), i.e. ~7 frames at 24 fps, and are keyed over
7 frames here; nothing in the game is changed or read at runtime.

Each test is one action with keys at fixed frames; every key reuses the
pose machinery (FK, then grounding of the root), and the in-betweens are
Blender's default Bezier interpolation, so feet may slide or dip slightly
between keys (documented, acceptable for a rig test).
"""

from __future__ import annotations

import bpy

from poses import POSES, Pose, apply

WALK_CONTACT_R = POSES[1].turns
WALK_PASS_R = {  # right foot planted under the body, left leg passing
    "pelvis": (("Z", -1.0),), "spine_01": (("Y", 3.0),),
    "thigh.R": (("Y", 2.0),), "shin.R": (("Y", 4.0),),
    "thigh.L": (("Y", -12.0),), "shin.L": (("Y", 38.0),), "foot.L": (("Y", 8.0),),
    "upper_arm.L": (("Y", 0.0),), "upper_arm.R": (("Y", 2.0),),
}
WALK_CONTACT_L = {
    "pelvis": (("Z", 4.0),), "spine_01": (("Y", 3.0),), "chest": (("Z", -5.0),),
    "thigh.L": (("Y", -24.0),), "shin.L": (("Y", 6.0),),
    "thigh.R": (("Y", 18.0),), "shin.R": (("Y", 14.0),),
    "upper_arm.L": (("Y", 16.0),), "forearm.L": (("Y", -6.0),),
    "upper_arm.R": (("Y", -6.0),),
}
WALK_PASS_L = {
    "pelvis": (("Z", 1.0),), "spine_01": (("Y", 3.0),),
    "thigh.L": (("Y", 2.0),), "shin.L": (("Y", 4.0),),
    "thigh.R": (("Y", -12.0),), "shin.R": (("Y", 38.0),), "foot.R": (("Y", 8.0),),
    "upper_arm.R": (("Y", 0.0),),
}
ATTACK_WINDUP = {
    "chest": (("Z", -15.0),), "spine_01": (("Y", -3.0),),
    "upper_arm.R": (("Y", -100.0), ("X", -15.0)), "forearm.R": (("Y", -40.0),),
    "thigh.R": (("Y", -15.0),), "shin.R": (("Y", 15.0),), "thigh.L": (("Y", 10.0),), "shin.L": (("Y", 10.0),),
}
ATTACK_RECOVER = {
    "chest": (("Z", 10.0),), "spine_01": (("Y", 4.0),),
    "upper_arm.R": (("Y", -25.0),), "forearm.R": (("Y", -10.0),),
    "thigh.R": (("Y", -20.0),), "shin.R": (("Y", 18.0),), "thigh.L": (("Y", 12.0),), "shin.L": (("Y", 8.0),),
}
DODGE_CROUCH = {
    "spine_01": (("Y", 8.0),), "thigh.R": (("Y", -25.0),), "shin.R": (("Y", 35.0),),
    "thigh.L": (("Y", 15.0),), "shin.L": (("Y", 30.0),), "upper_arm.L": (("Y", 15.0),),
}
DODGE_RECOVER = {
    "spine_01": (("Y", 6.0),), "thigh.R": (("Y", -20.0),), "shin.R": (("Y", 20.0),),
    "thigh.L": (("Y", 10.0),), "shin.L": (("Y", 15.0),),
    "coat_back.R_01": (("Y", 10.0),), "coat_back.L_01": (("Y", 10.0),),
}

TESTS = {
    "walk": ((1, Pose("w1", "", WALK_CONTACT_R)), (5, Pose("w2", "", WALK_PASS_R)),
             (9, Pose("w3", "", WALK_CONTACT_L)), (13, Pose("w4", "", WALK_PASS_L)),
             (17, Pose("w5", "", WALK_CONTACT_R))),
    "attack": ((1, Pose("a1", "")), (3, Pose("a2", "", ATTACK_WINDUP)), (5, POSES[5]),
               (7, Pose("a4", "", ATTACK_RECOVER))),
    "dodge": ((1, Pose("d1", "")), (3, Pose("d2", "", DODGE_CROUCH)), (5, POSES[8]),
              (7, Pose("d4", "", DODGE_RECOVER))),
}
# Frames rendered for the sheet (the walk loops: frame 17 = frame 1).
RENDER_FRAMES = {"walk": tuple(range(1, 17, 2)), "attack": tuple(range(1, 8)), "dodge": tuple(range(1, 8))}


def build_action(arm: bpy.types.Object, test: str, body: list[bpy.types.Object]) -> bpy.types.Action:
    arm.animation_data_create()
    arm.animation_data.action = None
    for frame, pose in TESTS[test]:
        apply(arm, pose, body)
        for pb in arm.pose.bones:
            pb.keyframe_insert("rotation_quaternion", frame=frame)
        arm.pose.bones["root"].keyframe_insert("location", frame=frame)
    action = arm.animation_data.action
    action.name = f"TEST_{test}"
    action.use_fake_user = True
    return action


def play(arm: bpy.types.Object, action: bpy.types.Action, frame: int) -> None:
    arm.animation_data.action = action
    bpy.context.scene.frame_set(frame)
