"""Clips of the first in-game sprite set (V1.3): idle, run, attack_basic, dodge.

Built on the v007 model. ``attack_basic`` and ``dodge`` are the V1.2-G
clips unchanged (same keys, same sampling: exactly 0.28 s, impact on frame
0, no root motion). ``idle`` and ``run`` are new:

- **idle**: 12 frames at 12 fps (1.0 s loop), breathing and a slow sway of
  the half-cape; both feet planted (sagittal leg IK, ``animations.leg_turns``).
- **run**: 10 frames at 24 fps (0.417 s loop) authored for the game's normal
  ground speed, ``PLAYER_WALK_SPEED`` = 190 px/s = 6.65 m/s at the contract
  scale (``METRES_PER_PX`` = 1.82 m / 52 px). The stance foot moves backward
  at exactly that speed (stance travel ``RUN_STRIDE_M`` over a stance share
  computed from it), so it does not slide when the game moves the entity at
  190 px/s; there are flight phases (both feet off the ground).

The frame N of a loop would equal frame 0 and is not exported.
"""

from __future__ import annotations

import math

import bpy

import animations as AN  # V1.2-G clips and helpers (v007 package), unchanged
import rig_config as C
from poses import Pose

GAME_SPEED_PX_S = C.PLAYER_WALK_SPEED_PX_S
METRES_PER_PX = 1.82 / 52.0  # config.METRES_PER_PX (contract scale)
RUN_SPEED_M_S = GAME_SPEED_PX_S * METRES_PER_PX

IDLE_FPS = 12
IDLE_FRAMES = 12
RUN_FPS = 24
RUN_FRAMES = 10
RUN_STRIDE_M = 0.76  # ankle travel during stance, relative to the hip
RUN_STANCE = RUN_STRIDE_M / (RUN_SPEED_M_S * RUN_FRAMES / RUN_FPS)  # share of the cycle on the ground


def _loop(key: str, frames: int, fps: int) -> AN.Clip:
    return AN.Clip(key, "locomotion", True, frames, tuple(1.0 / fps for _ in range(frames)))


CLIPS = {
    "idle": _loop("idle", IDLE_FRAMES, IDLE_FPS),
    "run": _loop("run", RUN_FRAMES, RUN_FPS),
    "attack_basic": AN.CLIPS["attack_basic"],
    "dodge": AN.CLIPS["dodge"],
}
FPS = {"idle": IDLE_FPS, "run": RUN_FPS, "attack_basic": C.PRODUCTION_FPS, "dodge": C.PRODUCTION_FPS}


def times(key: str) -> tuple[float, ...]:
    return tuple(round(i / FPS[key], 6) for i in range(CLIPS[key].frames))


# --- idle --------------------------------------------------------------------------------


def idle_pose(arm: bpy.types.Object, frame: int) -> Pose:
    phase = frame / IDLE_FRAMES
    breath = math.sin(2.0 * math.pi * phase)  # one breath per loop
    lag = math.sin(2.0 * math.pi * (phase - 0.15))
    hip_z = 0.835 + 0.004 * breath  # knees soften slightly on the exhale
    values: dict[str, AN.Euler3] = {}
    values.update(AN.leg_turns(arm, "R", 0.10, 0.0, hip_z, 0.0))
    values.update(AN.leg_turns(arm, "L", -0.10, 0.0, hip_z, 0.0))
    values.update({
        "spine_01": (2.0, 0.0, 0.0), "spine_02": (0.8 * breath, 0.0, 0.0), "chest": (-1.2 * breath, 0.0, 0.0),
        "head": (0.8 * breath, 0.0, 0.0),
        "upper_arm.L": (3.0 + 1.5 * breath, 0.0, 0.0), "forearm.L": (-8.0, 0.0, 0.0),
        "upper_arm.R": (1.0 * breath, 0.0, 0.0),
        "cape_back_01": (4.0 + 2.0 * lag, 0.0, 0.0), "cape_back_02": (2.0 + 1.5 * lag, 0.0, 0.0),
        "cape_arm.L_01": (1.5 * lag, 0.0, 0.0),
        "coat_back.R_02": (2.0 + 1.0 * lag, 0.0, 0.0), "coat_back.L_02": (2.0 + 1.0 * lag, 0.0, 0.0),
    })
    return Pose(f"idle_{frame:03d}", f"idle f{frame}", AN.turns(values))


# --- run ---------------------------------------------------------------------------------

SWING_ANKLE_DEG = 25.0  # ankle flex limit (relative to the shin) while the foot is in the air
RUN = {
    "hip_height_m": 0.74,  # hip above the stance ankle at mid-stance
    "dip_m": 0.03,  # lower at mid-stance, higher at contact and toe-off
    "lift_m": 0.40,  # heel recovery height (swing)
    "toe_off_deg": 32.0,
    "landing_deg": -6.0,
}


def _run_foot(phase: float) -> tuple[float, float, float]:
    """Ankle (dx, lift, pitch) of a leg; phase 0 = contact in front."""
    half = RUN_STRIDE_M / 2.0
    if phase < RUN_STANCE:
        s = phase / RUN_STANCE
        pitch = RUN["landing_deg"] * max(0.0, 1.0 - s / 0.2) + RUN["toe_off_deg"] * max(0.0, (s - 0.6) / 0.4)
        return half - s * RUN_STRIDE_M, 0.0, pitch
    s = (phase - RUN_STANCE) / (1.0 - RUN_STANCE)
    ease = 0.5 - 0.5 * math.cos(math.pi * s)
    lift = RUN["lift_m"] * math.sin(math.pi * s ** 0.8)  # the heel kicks up early, then reaches forward
    pitch = RUN["toe_off_deg"] * (1.0 - s) ** 2 + RUN["landing_deg"] * s
    return -half + ease * RUN_STRIDE_M, lift, pitch


def run_pose(arm: bpy.types.Object, frame: int) -> Pose:
    phase = frame / RUN_FRAMES
    feet = {side: _run_foot((phase + offset) % 1.0) for side, offset in (("R", 0.0), ("L", 0.5))}
    # The hips dip at mid-stance of either leg (two dips per cycle).
    stance_s = min(((phase + o) % 1.0) / RUN_STANCE for o in (0.0, 0.5))
    dip = RUN["dip_m"] * math.sin(math.pi * min(stance_s, 1.0)) if stance_s < 1.0 else 0.0
    hip_z = RUN["hip_height_m"] + RUN["dip_m"] - dip
    values: dict[str, AN.Euler3] = {}
    for side, (dx, lift, pitch) in feet.items():
        values.update(AN.leg_turns(arm, side, dx, lift, hip_z, pitch))
        if lift > 0.0:
            # Swing: let the foot hang from the shin instead of staying level with the
            # ground; holding it level bent the ankle far enough to inflate the boot
            # (dual-quaternion bulge, +15 % volume) while the knee folds.
            y, x, z = values[f"foot.{side}"]
            values[f"foot.{side}"] = (max(-SWING_ANKLE_DEG, min(SWING_ANKLE_DEG, y)), x, z)
    right_forward = math.cos(2.0 * math.pi * phase)  # +1 at right contact
    lag = math.sin(2.0 * math.pi * (phase - 0.12))
    values.update({
        "pelvis": (0.0, 0.0, 6.0 * right_forward),
        "spine_01": (10.0, 0.0, 0.0), "spine_02": (5.0, 0.0, 0.0),
        "chest": (2.0, 0.0, -8.0 * right_forward), "head": (-13.0, 0.0, 2.0 * right_forward),
        # Left arm pumps against the left leg (forward when the right leg is forward).
        "upper_arm.L": (-38.0 * right_forward, 6.0, 0.0), "forearm.L": (-75.0 + 10.0 * right_forward, 0.0, 0.0),
        # Sword arm: smaller swing, elbow bent, the sabre trails back and down.
        "upper_arm.R": (22.0 * right_forward, -4.0, 0.0), "forearm.R": (-35.0, 0.0, 0.0),
        "hand.R": (40.0, 0.0, 0.0),
        "coat_back.R_02": (18.0 + 6.0 * lag, 0.0, 0.0), "coat_back.L_02": (18.0 - 6.0 * lag, 0.0, 0.0),
        "cape_back_01": (32.0 + 5.0 * math.sin(4.0 * math.pi * (phase - 0.1)), 0.0, 0.0),
        "cape_back_02": (12.0 + 4.0 * math.sin(4.0 * math.pi * (phase - 0.18)), 0.0, 0.0),
        "cape_arm.L_01": (10.0 + 3.0 * lag, 0.0, 0.0),
    })
    airborne = min(lift for _, lift, _ in feet.values())  # flight phase: both feet up
    return Pose(f"run_{frame:03d}", f"run f{frame}", AN.turns(values), ground="air", lift=airborne)


def clip_pose(arm: bpy.types.Object, key: str, index: int) -> Pose:
    if key == "idle":
        return idle_pose(arm, index)
    if key == "run":
        return run_pose(arm, index)
    return AN.clip_pose(arm, key, index)


def build_action(arm: bpy.types.Object, key: str, body: list[bpy.types.Object]) -> bpy.types.Action:
    """Keys every frame (frame 1 = index 0), constant interpolation."""
    from poses import apply

    arm.animation_data_create()
    arm.animation_data.action = None
    for i in range(CLIPS[key].frames):
        apply(arm, clip_pose(arm, key, i), body)
        for pb in arm.pose.bones:
            pb.keyframe_insert("rotation_quaternion", frame=i + 1)
        arm.pose.bones["root"].keyframe_insert("location", frame=i + 1)
    action = arm.animation_data.action
    action.name = f"GAME_{key}"
    action.use_fake_user = True
    AN._constant_interpolation(action)
    return action
