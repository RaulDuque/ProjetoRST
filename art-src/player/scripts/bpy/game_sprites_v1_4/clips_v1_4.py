"""Clips of the V1.4 sprite expansion (model v007, same rig, same pipeline as V1.3).

Identifiers follow docs/ASSET_CONTRACT.md (section 2) and the ``animation``
keys of the skill and spell RON files:

| Clip | Group | Timing | Gameplay state it shows |
|---|---|---|---|
| hurt | locomotion | 0.35 s, FitToAction | Hurt (PLAYER_HURT_TIME) |
| death | locomotion | 8 frames at 12 fps, Once (holds the last) | Dead |
| jump_rise | locomotion | 4 frames at 12 fps, Once | Jumping |
| fall | locomotion | 4 frames at 12 fps, Loop | Falling |
| land | locomotion | 4 frames at 24 fps, Once | Falling -> Idle |
| crouch | locomotion | 3 frames at 24 fps, Once | Crouching |
| guard_start | locomotion | 3 frames at 24 fps, Once | Guarding |
| slash_crescent | combat | 0.40 s, FitToAction | Casting (crescent_slash) |
| slash_rising | combat | 0.45 s, FitToAction | Casting (rising_edge) |
| cast_forward | combat | 0.30 s, FitToAction | Casting (ember_bolt) |
| cast_up | combat | 0.60 s, FitToAction | Casting (mending_light) |

Action clips sample t = i/24 and end with a shorter frame, so they last the
gameplay time exactly (as V1.2-G/V1.3). Effects of the skills start with
the action, so frame 0 is already the strike or the cast ("impact first").
No root motion: the sprites stay in place. The death sequence collapses onto
one knee and folds forward instead of lying flat: with skinned cloth (no
simulation) a body rotated to the floor left the coat standing up like a
tent and floating above the floor (V1.4 iteration), and a kneeling end pose
keeps the cloth hanging and the figure inside the 64-unit cell.

Keys use the V1.2-G convention: rotations in world axes at rest, plus
"_hip" (hip height above the ankle line) and "_legs.<side>" (ankle dx,
lift, foot pitch) solved by the sagittal leg IK.
"""

from __future__ import annotations

import math

import bpy

import animations as AN  # V1.2-G helpers (v007 package), unchanged
import rig_config as C
from poses import Pose

Keys = tuple[tuple[float, dict[str, AN.Euler3]], ...]


def _once(key: str, group: str, frames: int, fps: int, loop: bool = False) -> AN.Clip:
    return AN.Clip(key, group, loop, frames, tuple(1.0 / fps for _ in range(frames)))


ACTION_SECONDS = {"hurt": 0.35, "slash_crescent": 0.40, "slash_rising": 0.45, "cast_forward": 0.30,
                  "cast_up": 0.60}
CLIPS = {
    "hurt": AN._one_shot("hurt", "locomotion", ACTION_SECONDS["hurt"]),
    "death": _once("death", "locomotion", 8, 12),
    "jump_rise": _once("jump_rise", "locomotion", 4, 12),
    "fall": _once("fall", "locomotion", 4, 12, loop=True),
    "land": _once("land", "locomotion", 4, 24),
    "crouch": _once("crouch", "locomotion", 3, 24),
    "guard_start": _once("guard_start", "locomotion", 3, 24),
    "slash_crescent": AN._one_shot("slash_crescent", "combat", ACTION_SECONDS["slash_crescent"]),
    "slash_rising": AN._one_shot("slash_rising", "combat", ACTION_SECONDS["slash_rising"]),
    "cast_forward": AN._one_shot("cast_forward", "combat", ACTION_SECONDS["cast_forward"]),
    "cast_up": AN._one_shot("cast_up", "combat", ACTION_SECONDS["cast_up"]),
}
FPS = {k: (C.PRODUCTION_FPS if k in ACTION_SECONDS else round(1.0 / c.durations[0])) for k, c in CLIPS.items()}
MODE = {k: ("FitToAction" if k in ACTION_SECONDS else ("Loop" if c.loop else "Once")) for k, c in CLIPS.items()}
# Sprite grounding: "feet" (lowest boot on the floor), "all" (lowest body point) or "air".
GROUND = {"death": "all", "jump_rise": "air", "fall": "air"}


def times(key: str) -> tuple[float, ...]:
    return tuple(round(i / FPS[key], 6) for i in range(CLIPS[key].frames))


STANCE = {"_hip": (0.80, 0.0, 0.0), "_legs.R": (0.14, 0.0, 0.0), "_legs.L": (-0.15, 0.0, 0.0)}

KEYS: dict[str, Keys] = {
    "hurt": (
        (0.00, {**STANCE, "_hip": (0.80, 0.0, 0.0), "_legs.L": (-0.20, 0.0, 6.0),  # the hit: thrown back
                "spine_01": (-10.0, 0.0, 0.0), "spine_02": (-8.0, 0.0, 0.0), "chest": (-6.0, 0.0, 0.0),
                "head": (14.0, 0.0, 0.0), "upper_arm.L": (30.0, 20.0, 0.0), "forearm.L": (-20.0, 0.0, 0.0),
                "upper_arm.R": (20.0, -15.0, 0.0), "hand.R": (20.0, 0.0, 0.0),
                "cape_back_01": (-6.0, 0.0, 0.0), "coat_front.R_01": (-8.0, 0.0, 0.0),
                "coat_front.L_01": (-8.0, 0.0, 0.0)}),
        (0.12, {**STANCE, "_hip": (0.77, 0.0, 0.0), "_legs.L": (-0.24, 0.0, 10.0),  # deepest recoil
                "spine_01": (-14.0, 0.0, 0.0), "spine_02": (-10.0, 0.0, 0.0), "chest": (-6.0, 0.0, 0.0),
                "head": (18.0, 0.0, 0.0), "upper_arm.L": (34.0, 24.0, 0.0), "forearm.L": (-26.0, 0.0, 0.0),
                "upper_arm.R": (24.0, -18.0, 0.0), "hand.R": (24.0, 0.0, 0.0),
                "cape_back_01": (-8.0, 0.0, 0.0), "cape_back_02": (-6.0, 0.0, 0.0),
                "coat_front.R_01": (-12.0, 0.0, 0.0), "coat_front.L_01": (-12.0, 0.0, 0.0)}),
        (0.35, {**STANCE, "spine_01": (3.0, 0.0, 0.0), "upper_arm.L": (8.0, 4.0, 0.0),  # back to the stance
                "forearm.L": (-10.0, 0.0, 0.0), "upper_arm.R": (2.0, 0.0, 0.0), "cape_back_01": (4.0, 0.0, 0.0)}),
    ),
    "death": (  # collapse onto one knee, then the torso folds forward over it and stays down
        (0.000, {**STANCE, "spine_01": (-10.0, 0.0, 0.0), "chest": (-6.0, 0.0, 0.0), "head": (16.0, 0.0, 0.0),
                 "upper_arm.L": (30.0, 18.0, 0.0), "upper_arm.R": (20.0, -12.0, 0.0), "hand.R": (35.0, 0.0, 0.0)}),
        (0.167, {"_hip": (0.62, 0.0, 0.0), "_legs.R": (0.16, 0.0, 0.0), "_legs.L": (-0.12, 0.0, 12.0),
                 "spine_01": (12.0, 0.0, 0.0), "spine_02": (6.0, 0.0, 0.0), "head": (20.0, 0.0, 0.0),
                 "upper_arm.L": (12.0, 10.0, 0.0), "upper_arm.R": (10.0, -8.0, 0.0), "hand.R": (55.0, 0.0, 0.0),
                 "cape_back_01": (-6.0, 0.0, 0.0)}),
        (0.333, {"_hip": (0.46, 0.0, 0.0), "_legs.R": (0.16, 0.0, 0.0), "_legs.L": (-0.18, 0.0, 40.0),  # dropping to a knee
                 "spine_01": (16.0, 0.0, 0.0), "spine_02": (8.0, 0.0, 0.0), "head": (24.0, 0.0, 0.0),
                 "upper_arm.L": (4.0, 8.0, 0.0), "upper_arm.R": (-4.0, -6.0, 0.0), "hand.R": (72.0, 0.0, 0.0),
                 "cape_back_01": (6.0, 0.0, 0.0)}),
        (0.500, {"_hip": (0.42, 0.0, 0.0), "_legs.R": (0.20, 0.0, 0.0), "_legs.L": (-0.22, 0.0, 50.0),
                 "spine_01": (32.0, 0.0, 0.0), "spine_02": (16.0, 0.0, 0.0), "chest": (8.0, 0.0, 0.0),
                 "head": (26.0, 0.0, 0.0), "upper_arm.L": (-20.0, 6.0, 0.0), "upper_arm.R": (-25.0, -6.0, 0.0),
                 "hand.R": (80.0, 0.0, 0.0), "cape_back_01": (10.0, 0.0, 0.0)}),
        (0.667, {"_hip": (0.42, 0.0, 0.0), "_legs.R": (0.22, 0.0, 0.0), "_legs.L": (-0.20, 0.0, 50.0),  # final
                 "spine_01": (40.0, 0.0, 0.0), "spine_02": (20.0, 0.0, 0.0), "chest": (12.0, 0.0, 0.0),
                 "head": (30.0, 0.0, 0.0), "upper_arm.L": (-30.0, 6.0, 0.0), "forearm.L": (-10.0, 0.0, 0.0),
                 "upper_arm.R": (-35.0, -6.0, 0.0), "forearm.R": (-5.0, 0.0, 0.0), "hand.R": (75.0, 0.0, 0.0),
                 "cape_back_01": (14.0, 0.0, 0.0), "cape_back_02": (6.0, 0.0, 0.0)}),
    ),
    "jump_rise": (
        (0.000, {"_hip": (0.83, 0.0, 0.0), "_legs.R": (0.06, 0.0, 25.0), "_legs.L": (-0.12, 0.0, 30.0),  # push-off
                 "spine_01": (6.0, 0.0, 0.0), "head": (-8.0, 0.0, 0.0), "upper_arm.L": (-40.0, 10.0, 0.0),
                 "forearm.L": (-20.0, 0.0, 0.0), "upper_arm.R": (25.0, -10.0, 0.0), "hand.R": (20.0, 0.0, 0.0),
                 "coat_back.R_02": (-4.0, 0.0, 0.0), "coat_back.L_02": (-4.0, 0.0, 0.0),
                 "cape_back_01": (-6.0, 0.0, 0.0)}),
        (0.167, {"_hip": (0.80, 0.0, 0.0), "_legs.R": (0.20, 0.28, 0.0), "_legs.L": (-0.05, 0.12, 25.0),
                 "spine_01": (8.0, 0.0, 0.0), "head": (-10.0, 0.0, 0.0), "upper_arm.L": (-60.0, 15.0, 0.0),
                 "forearm.L": (-35.0, 0.0, 0.0), "upper_arm.R": (10.0, -15.0, 0.0), "hand.R": (60.0, 0.0, 0.0),
                 "coat_back.R_02": (8.0, 0.0, 0.0), "coat_back.L_02": (8.0, 0.0, 0.0), "cape_back_01": (6.0, 0.0, 0.0)}),
        (0.250, {"_hip": (0.80, 0.0, 0.0), "_legs.R": (0.22, 0.34, 0.0), "_legs.L": (0.02, 0.20, 15.0),  # tucked
                 "spine_01": (10.0, 0.0, 0.0), "head": (-10.0, 0.0, 0.0), "upper_arm.L": (-50.0, 18.0, 0.0),
                 "forearm.L": (-40.0, 0.0, 0.0), "upper_arm.R": (5.0, -15.0, 0.0), "hand.R": (70.0, 0.0, 0.0),
                 "coat_back.R_02": (12.0, 0.0, 0.0), "coat_back.L_02": (12.0, 0.0, 0.0),
                 "cape_back_01": (10.0, 0.0, 0.0)}),
    ),
    "land": (
        (0.000, {"_hip": (0.66, 0.0, 0.0), "_legs.R": (0.16, 0.0, 0.0), "_legs.L": (-0.14, 0.0, 0.0),  # impact
                 "spine_01": (14.0, 0.0, 0.0), "spine_02": (6.0, 0.0, 0.0), "head": (-10.0, 0.0, 0.0),
                 "upper_arm.L": (-25.0, 20.0, 0.0), "forearm.L": (-25.0, 0.0, 0.0), "upper_arm.R": (10.0, -18.0, 0.0),
                 "hand.R": (50.0, 0.0, 0.0), "cape_back_01": (18.0, 0.0, 0.0), "coat_back.R_02": (-6.0, 0.0, 0.0),
                 "coat_back.L_02": (-6.0, 0.0, 0.0)}),
        (0.125, {**STANCE, "spine_01": (4.0, 0.0, 0.0), "upper_arm.L": (5.0, 4.0, 0.0), "hand.R": (25.0, 0.0, 0.0),
                 "cape_back_01": (6.0, 0.0, 0.0)}),
    ),
    "crouch": (
        (0.000, {"_hip": (0.72, 0.0, 0.0), "_legs.R": (0.16, 0.0, 0.0), "_legs.L": (-0.16, 0.0, 0.0),
                 "spine_01": (8.0, 0.0, 0.0), "hand.R": (-10.0, 0.0, 0.0)}),
        (0.083, {"_hip": (0.50, 0.0, 0.0), "_legs.R": (0.20, 0.0, 0.0), "_legs.L": (-0.18, 0.0, 25.0),
                 "spine_01": (18.0, 0.0, 0.0), "spine_02": (6.0, 0.0, 0.0), "head": (-16.0, 0.0, 0.0),
                 "upper_arm.R": (0.0, -6.0, 0.0), "forearm.R": (-25.0, 0.0, 0.0), "hand.R": (-18.0, 0.0, 0.0),
                 "upper_arm.L": (-10.0, 10.0, 0.0), "forearm.L": (-40.0, 0.0, 0.0), "cape_back_01": (12.0, 0.0, 0.0),
                 "coat_back.R_02": (10.0, 0.0, 0.0), "coat_back.L_02": (10.0, 0.0, 0.0)}),
    ),
    "guard_start": (
        (0.000, {**STANCE, "upper_arm.R": (-30.0, 0.0, 0.0), "forearm.R": (-55.0, 0.0, 0.0), "hand.R": (-30.0, 0.0, 0.0),
                 "upper_arm.L": (-10.0, 0.0, 0.0), "forearm.L": (-20.0, 0.0, 0.0)}),
        (0.083, {"_hip": (0.76, 0.0, 0.0), "_legs.R": (0.20, 0.0, 0.0), "_legs.L": (-0.22, 0.0, 8.0),  # guard
                 "spine_01": (6.0, 0.0, 0.0), "head": (-4.0, 0.0, 0.0),
                 "upper_arm.R": (-40.0, 0.0, 0.0), "forearm.R": (-60.0, 0.0, 0.0), "hand.R": (-15.0, 0.0, 0.0),
                 "upper_arm.L": (-35.0, 0.0, 0.0), "forearm.L": (-60.0, 0.0, 0.0), "cape_back_01": (4.0, 0.0, 0.0)}),
    ),
    "slash_crescent": (
        (0.00, {"_hip": (0.74, 0.0, 0.0), "_legs.R": (0.32, 0.0, 0.0), "_legs.L": (-0.32, 0.0, 10.0),  # high-front
                "pelvis": (0.0, 0.0, -6.0), "spine_01": (4.0, 0.0, -6.0), "chest": (0.0, 0.0, -10.0),
                "upper_arm.R": (-115.0, -25.0, 0.0), "forearm.R": (-10.0, 0.0, 0.0), "hand.R": (-10.0, 0.0, 0.0),
                "upper_arm.L": (20.0, 12.0, 0.0), "forearm.L": (-20.0, 0.0, 0.0), "cape_back_01": (16.0, 0.0, 0.0)}),
        (0.12, {"_hip": (0.72, 0.0, 0.0), "_legs.R": (0.36, 0.0, 0.0), "_legs.L": (-0.36, 0.0, 14.0),  # through the front
                "pelvis": (0.0, 0.0, 8.0), "spine_01": (8.0, 0.0, 8.0), "spine_02": (0.0, 0.0, 10.0),
                "chest": (0.0, 0.0, 14.0), "head": (-6.0, 0.0, -10.0),
                "upper_arm.R": (-80.0, -20.0, 0.0), "forearm.R": (-5.0, 0.0, 0.0), "hand.R": (10.0, 0.0, 0.0),
                "upper_arm.L": (30.0, 14.0, 0.0), "forearm.L": (-20.0, 0.0, 0.0), "cape_back_01": (30.0, 0.0, 0.0),
                "coat_back.R_02": (18.0, 0.0, 0.0), "coat_back.L_02": (18.0, 0.0, 0.0)}),
        (0.24, {"_hip": (0.72, 0.0, 0.0), "_legs.R": (0.36, 0.0, 0.0), "_legs.L": (-0.36, 0.0, 14.0),  # crescent end, low
                "pelvis": (0.0, 0.0, 14.0), "spine_01": (12.0, 0.0, 10.0), "spine_02": (0.0, 0.0, 14.0),
                "chest": (0.0, 0.0, 18.0), "head": (-6.0, 0.0, -12.0),
                "upper_arm.R": (-35.0, -10.0, 0.0), "forearm.R": (-10.0, 0.0, 0.0), "hand.R": (40.0, 0.0, 0.0),
                "upper_arm.L": (34.0, 14.0, 0.0), "forearm.L": (-25.0, 0.0, 0.0), "cape_back_01": (34.0, 0.0, 0.0),
                "cape_back_02": (12.0, 0.0, 0.0), "coat_back.R_02": (22.0, 0.0, 0.0),
                "coat_back.L_02": (22.0, 0.0, 0.0)}),
        (0.40, {"_hip": (0.79, 0.0, 0.0), "_legs.R": (0.18, 0.0, 0.0), "_legs.L": (-0.18, 0.0, 0.0),  # recovery
                "pelvis": (0.0, 0.0, 4.0), "spine_01": (4.0, 0.0, 2.0), "chest": (0.0, 0.0, 4.0),
                "upper_arm.R": (-14.0, 0.0, 0.0), "forearm.R": (-14.0, 0.0, 0.0), "hand.R": (12.0, 0.0, 0.0),
                "upper_arm.L": (8.0, 3.0, 0.0), "forearm.L": (-10.0, 0.0, 0.0), "cape_back_01": (10.0, 0.0, 0.0)}),
    ),
    "slash_rising": (
        (0.00, {"_hip": (0.74, 0.0, 0.0), "_legs.R": (0.26, 0.0, 0.0), "_legs.L": (-0.26, 0.0, 12.0),  # blade low, rising
                "spine_01": (10.0, 0.0, 4.0), "chest": (0.0, 0.0, 6.0),
                "upper_arm.R": (-40.0, -8.0, 0.0), "forearm.R": (-15.0, 0.0, 0.0), "hand.R": (-35.0, 0.0, 0.0),
                "upper_arm.L": (25.0, 10.0, 0.0), "forearm.L": (-15.0, 0.0, 0.0), "cape_back_01": (10.0, 0.0, 0.0)}),
        (0.12, {"_hip": (0.80, 0.0, 0.0), "_legs.R": (0.24, 0.0, 0.0), "_legs.L": (-0.22, 0.0, 22.0),  # cutting up
                "spine_01": (-4.0, 0.0, 6.0), "spine_02": (0.0, 0.0, 6.0), "chest": (-4.0, 0.0, 6.0),
                "head": (-10.0, 0.0, 0.0),
                "upper_arm.R": (-100.0, -10.0, 0.0), "forearm.R": (-10.0, 0.0, 0.0), "hand.R": (-20.0, 0.0, 0.0),
                "upper_arm.L": (30.0, 12.0, 0.0), "forearm.L": (-20.0, 0.0, 0.0), "cape_back_01": (-4.0, 0.0, 0.0),
                "coat_back.R_02": (-6.0, 0.0, 0.0), "coat_back.L_02": (-6.0, 0.0, 0.0)}),
        (0.25, {"_hip": (0.82, 0.0, 0.0), "_legs.R": (0.22, 0.0, 0.0), "_legs.L": (-0.20, 0.0, 26.0),  # blade overhead
                "spine_01": (-8.0, 0.0, 6.0), "spine_02": (0.0, 0.0, 6.0), "chest": (-4.0, 0.0, 6.0),
                "head": (-14.0, 0.0, 0.0),
                "upper_arm.R": (-140.0, -6.0, 0.0), "forearm.R": (-15.0, 0.0, 0.0), "hand.R": (-10.0, 0.0, 0.0),
                "upper_arm.L": (28.0, 14.0, 0.0), "forearm.L": (-20.0, 0.0, 0.0), "cape_back_01": (-8.0, 0.0, 0.0),
                "cape_back_02": (-6.0, 0.0, 0.0)}),
        (0.45, {"_hip": (0.79, 0.0, 0.0), "_legs.R": (0.18, 0.0, 0.0), "_legs.L": (-0.18, 0.0, 0.0),  # recovery
                "spine_01": (4.0, 0.0, 2.0), "upper_arm.R": (-20.0, 0.0, 0.0), "forearm.R": (-15.0, 0.0, 0.0),
                "hand.R": (10.0, 0.0, 0.0), "upper_arm.L": (8.0, 3.0, 0.0), "forearm.L": (-10.0, 0.0, 0.0),
                "cape_back_01": (8.0, 0.0, 0.0)}),
    ),
    "cast_forward": (
        (0.00, {"_hip": (0.78, 0.0, 0.0), "_legs.R": (-0.16, 0.0, 6.0), "_legs.L": (0.22, 0.0, 0.0),  # bolt released
                "pelvis": (0.0, 0.0, -6.0), "spine_01": (6.0, 0.0, -6.0), "spine_02": (0.0, 0.0, -7.0),
                "chest": (0.0, 0.0, -8.0),
                "upper_arm.L": (-85.0, 0.0, 0.0), "forearm.L": (-5.0, 0.0, 0.0),
                "upper_arm.R": (22.0, -6.0, 0.0), "hand.R": (20.0, 0.0, 0.0), "cape_back_01": (14.0, 0.0, 0.0)}),
        (0.12, {"_hip": (0.77, 0.0, 0.0), "_legs.R": (-0.18, 0.0, 8.0), "_legs.L": (0.24, 0.0, 0.0),  # follow-through
                "pelvis": (0.0, 0.0, -8.0), "spine_01": (8.0, 0.0, -8.0), "spine_02": (0.0, 0.0, -8.0),
                "chest": (0.0, 0.0, -10.0),
                "upper_arm.L": (-90.0, 4.0, 0.0), "forearm.L": (-2.0, 0.0, 0.0),
                "upper_arm.R": (26.0, -8.0, 0.0), "hand.R": (24.0, 0.0, 0.0), "cape_back_01": (20.0, 0.0, 0.0)}),
        (0.30, {**STANCE, "spine_01": (3.0, 0.0, 0.0), "upper_arm.L": (-10.0, 3.0, 0.0),  # recovery
                "forearm.L": (-15.0, 0.0, 0.0), "upper_arm.R": (4.0, 0.0, 0.0), "hand.R": (20.0, 0.0, 0.0),
                "cape_back_01": (6.0, 0.0, 0.0)}),
    ),
    "cast_up": (
        (0.00, {"_hip": (0.80, 0.0, 0.0), "_legs.R": (0.12, 0.0, 0.0), "_legs.L": (-0.14, 0.0, 0.0),  # gathering light
                "spine_01": (-2.0, 0.0, 0.0), "chest": (-4.0, 0.0, 0.0), "head": (-12.0, 0.0, 0.0),
                "upper_arm.L": (-110.0, 18.0, 0.0), "forearm.L": (-45.0, 0.0, 0.0),
                "upper_arm.R": (6.0, -4.0, 0.0), "cape_back_01": (2.0, 0.0, 0.0)}),
        (0.25, {"_hip": (0.81, 0.0, 0.0), "_legs.R": (0.12, 0.0, 0.0), "_legs.L": (-0.14, 0.0, 6.0),  # raised
                "spine_01": (-4.0, 0.0, 0.0), "chest": (-6.0, 0.0, 0.0), "head": (-18.0, 0.0, 0.0),
                "upper_arm.L": (-150.0, 14.0, 0.0), "forearm.L": (-20.0, 0.0, 0.0),
                "upper_arm.R": (8.0, -6.0, 0.0), "cape_back_01": (4.0, 0.0, 0.0), "cape_back_02": (2.0, 0.0, 0.0),
                "coat_back.R_02": (-4.0, 0.0, 0.0), "coat_back.L_02": (-4.0, 0.0, 0.0)}),
        (0.42, {"_hip": (0.80, 0.0, 0.0), "_legs.R": (0.12, 0.0, 0.0), "_legs.L": (-0.14, 0.0, 0.0),  # held
                "spine_01": (-2.0, 0.0, 0.0), "chest": (-4.0, 0.0, 0.0), "head": (-12.0, 0.0, 0.0),
                "upper_arm.L": (-140.0, 14.0, 0.0), "forearm.L": (-25.0, 0.0, 0.0),
                "upper_arm.R": (6.0, -4.0, 0.0), "cape_back_01": (4.0, 0.0, 0.0)}),
        (0.60, {**STANCE, "spine_01": (3.0, 0.0, 0.0), "upper_arm.L": (5.0, 3.0, 0.0),  # recovery
                "forearm.L": (-10.0, 0.0, 0.0), "cape_back_01": (4.0, 0.0, 0.0)}),
    ),
}


def _fall_pose(arm: bpy.types.Object, frame: int) -> Pose:
    phase = frame / CLIPS["fall"].frames
    flutter = math.sin(2.0 * math.pi * phase)
    values: dict[str, AN.Euler3] = {}
    values.update(AN.leg_turns(arm, "R", 0.14, 0.10, 0.80, 12.0))  # front leg reaching down
    values.update(AN.leg_turns(arm, "L", -0.10, 0.0, 0.80, 22.0))
    values.update({
        "spine_01": (2.0, 0.0, 0.0), "head": (6.0, 0.0, 0.0),
        "upper_arm.L": (-25.0, 28.0 + 3.0 * flutter, 0.0), "forearm.L": (-20.0, 0.0, 0.0),
        "upper_arm.R": (5.0, -22.0 - 3.0 * flutter, 0.0), "hand.R": (15.0, 0.0, 0.0),
        # Air rushes up past the body: the cloth lifts and flutters.
        "cape_back_01": (26.0 + 5.0 * flutter, 0.0, 0.0), "cape_back_02": (12.0 + 4.0 * flutter, 0.0, 0.0),
        "cape_arm.L_01": (10.0 + 3.0 * flutter, 0.0, 0.0),
        "coat_back.R_02": (14.0 + 4.0 * flutter, 0.0, 0.0), "coat_back.L_02": (14.0 - 4.0 * flutter, 0.0, 0.0),
    })
    return Pose(f"fall_{frame:03d}", f"fall f{frame}", AN.turns(values), ground="air")


def clip_pose(arm: bpy.types.Object, key: str, index: int) -> Pose:
    if key == "fall":
        return _fall_pose(arm, index)
    t = times(key)[index]
    values = AN.sample_keys(KEYS[key], t)
    hip_z = values.pop("_hip")[0]
    for side in ("R", "L"):
        dx, lift, pitch = values.pop(f"_legs.{side}")
        values.update(AN.leg_turns(arm, side, dx, lift, hip_z, pitch))
    return Pose(f"{key}_{index:03d}", f"{key} f{index} t={t:.3f}s", AN.turns(values),
                ground=GROUND.get(key, "feet"))


def build_action(arm: bpy.types.Object, key: str, body: list[bpy.types.Object]) -> bpy.types.Action:
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
