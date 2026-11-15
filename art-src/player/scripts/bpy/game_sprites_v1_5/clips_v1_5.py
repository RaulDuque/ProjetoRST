"""Clips of the V1.5 art polish (model v007, same rig, same pipeline as V1.4).

Variants for comparison; the clips of V1.4 are kept unchanged in their own
files and renders. Keys of this module are render ids, not game ids: the
post script (post/game_sprites_v1_5.py) decides which ones reach the game.

| Render id | Group | Timing | Purpose |
|---|---|---|---|
| hurt_a | locomotion | 0.35 s, FitToAction (as hurt) | Stronger recoil: shoulders, torso, head, cape follow-through |
| hurt_b | locomotion | 0.35 s, FitToAction | Twisted recoil (the hit turns the near shoulder away) |
| fall_v15 | locomotion | 4 frames at 12 fps, Loop (as fall) | Clear descent: leg reaching down, knee tucked, arms up, cloth lifted |
| attack_basic_air | combat | 0.28 s, FitToAction (as attack_basic) | The ground strike with airborne legs |
| death_lying | combat | 12 frames at 12 fps, Once | Experimental: full fall to lying face down (the V1.4 kneeling death is kept) |

Same conventions as V1.4: keys in world axes at rest, "_hip"/"_legs.<side>"
solved by the sagittal leg IK, impact first, no root motion, durations read
from the game by the post script. ``death_lying`` needs the combat cell
(the lying body is longer than the 64-unit locomotion cell) and leaves the
leg IK once the knees are down: its later keys are plain FK turns.
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


ACTION_SECONDS = {"hurt_a": 0.35, "hurt_b": 0.35, "attack_basic_air": C.BASIC_ATTACK_TIME_S}
CLIPS = {
    "hurt_a": AN._one_shot("hurt_a", "locomotion", ACTION_SECONDS["hurt_a"]),
    "hurt_b": AN._one_shot("hurt_b", "locomotion", ACTION_SECONDS["hurt_b"]),
    "fall_v15": _once("fall_v15", "locomotion", 4, 12, loop=True),
    "attack_basic_air": AN._one_shot("attack_basic_air", "combat", ACTION_SECONDS["attack_basic_air"]),
    "death_lying": _once("death_lying", "combat", 12, 12),
}
FPS = {k: (C.PRODUCTION_FPS if k in ACTION_SECONDS else round(1.0 / c.durations[0])) for k, c in CLIPS.items()}
MODE = {k: ("FitToAction" if k in ACTION_SECONDS else ("Loop" if c.loop else "Once")) for k, c in CLIPS.items()}
GROUND = {"fall_v15": "air", "attack_basic_air": "air", "death_lying": "all"}
# Game clip each render id stands for (durations, comparison and packing).
GAME_CLIP = {"hurt_a": "hurt", "hurt_b": "hurt", "fall_v15": "fall", "attack_basic_air": "attack_basic_air",
             "death_lying": "death"}


def times(key: str) -> tuple[float, ...]:
    return tuple(round(i / FPS[key], 6) for i in range(CLIPS[key].frames))


STANCE = {"_hip": (0.80, 0.0, 0.0), "_legs.R": (0.14, 0.0, 0.0), "_legs.L": (-0.15, 0.0, 0.0)}

KEYS: dict[str, Keys] = {
    # A: the V1.4 recoil was ~10 deg of torso; here the hit reads at 1:1 through the
    # shoulders (free arm flung forward, sword arm swung back), a head snap with a
    # one-frame lag, the hips dropping and the cape overshooting on the recovery.
    "hurt_a": (
        (0.00, {"_hip": (0.76, 0.0, 0.0), "_legs.R": (0.16, 0.0, 0.0), "_legs.L": (-0.26, 0.0, 10.0),  # the hit
                "pelvis": (-4.0, 0.0, 0.0), "spine_01": (-14.0, 0.0, 0.0), "spine_02": (-10.0, 0.0, 0.0),
                "chest": (-8.0, 0.0, 0.0), "head": (6.0, 0.0, 0.0),
                "upper_arm.L": (-38.0, 22.0, 0.0), "forearm.L": (-30.0, 0.0, 0.0),
                "upper_arm.R": (34.0, -18.0, 0.0), "forearm.R": (-10.0, 0.0, 0.0), "hand.R": (28.0, 0.0, 0.0),
                "cape_back_01": (8.0, 0.0, 0.0), "cape_back_02": (4.0, 0.0, 0.0),
                "coat_front.R_01": (-14.0, 0.0, 0.0), "coat_front.L_01": (-14.0, 0.0, 0.0),
                "coat_back.R_02": (8.0, 0.0, 0.0), "coat_back.L_02": (8.0, 0.0, 0.0)}),
        (0.08, {"_hip": (0.74, 0.0, 0.0), "_legs.R": (0.17, 0.0, 0.0), "_legs.L": (-0.30, 0.0, 14.0),  # deepest
                "pelvis": (-6.0, 0.0, 0.0), "spine_01": (-18.0, 0.0, 0.0), "spine_02": (-12.0, 0.0, 0.0),
                "chest": (-8.0, 0.0, 0.0), "head": (14.0, 0.0, 0.0),
                "upper_arm.L": (-46.0, 26.0, 0.0), "forearm.L": (-38.0, 0.0, 0.0),
                "upper_arm.R": (40.0, -22.0, 0.0), "forearm.R": (-14.0, 0.0, 0.0), "hand.R": (32.0, 0.0, 0.0),
                "cape_back_01": (16.0, 0.0, 0.0), "cape_back_02": (10.0, 0.0, 0.0),
                "coat_front.R_01": (-18.0, 0.0, 0.0), "coat_front.L_01": (-18.0, 0.0, 0.0),
                "coat_back.R_02": (12.0, 0.0, 0.0), "coat_back.L_02": (12.0, 0.0, 0.0)}),
        (0.21, {"_hip": (0.78, 0.0, 0.0), "_legs.R": (0.15, 0.0, 0.0), "_legs.L": (-0.20, 0.0, 4.0),  # recovering
                "spine_01": (-4.0, 0.0, 0.0), "spine_02": (-2.0, 0.0, 0.0), "head": (2.0, 0.0, 0.0),
                "upper_arm.L": (-10.0, 10.0, 0.0), "forearm.L": (-20.0, 0.0, 0.0),
                "upper_arm.R": (12.0, -6.0, 0.0), "hand.R": (24.0, 0.0, 0.0),
                "cape_back_01": (-2.0, 0.0, 0.0), "cape_back_02": (-2.0, 0.0, 0.0),  # cloth swings back in
                "coat_front.R_01": (4.0, 0.0, 0.0), "coat_front.L_01": (4.0, 0.0, 0.0)}),
        (0.35, {**STANCE, "spine_01": (3.0, 0.0, 0.0), "upper_arm.L": (8.0, 4.0, 0.0),  # back to the stance
                "forearm.L": (-10.0, 0.0, 0.0), "upper_arm.R": (2.0, 0.0, 0.0), "cape_back_01": (4.0, 0.0, 0.0)}),
    ),
    # B: a smaller lean with a twist: the near (sword) shoulder is knocked back
    # (negative yaw), the head turns away and the torso hunches.
    "hurt_b": (
        (0.00, {"_hip": (0.76, 0.0, 0.0), "_legs.R": (0.16, 0.0, 0.0), "_legs.L": (-0.24, 0.0, 8.0),
                "pelvis": (0.0, 0.0, -8.0), "spine_01": (-10.0, 0.0, -6.0), "spine_02": (-6.0, 0.0, -8.0),
                "chest": (4.0, 0.0, -10.0), "head": (12.0, 0.0, -8.0),
                "upper_arm.L": (-24.0, 20.0, 0.0), "forearm.L": (-34.0, 0.0, 0.0),
                "upper_arm.R": (36.0, -20.0, 0.0), "hand.R": (30.0, 0.0, 0.0),
                "cape_back_01": (6.0, 0.0, 0.0), "coat_front.R_01": (-10.0, 0.0, 0.0),
                "coat_front.L_01": (-10.0, 0.0, 0.0)}),
        (0.08, {"_hip": (0.75, 0.0, 0.0), "_legs.R": (0.17, 0.0, 0.0), "_legs.L": (-0.28, 0.0, 12.0),
                "pelvis": (0.0, 0.0, -12.0), "spine_01": (-12.0, 0.0, -8.0), "spine_02": (-8.0, 0.0, -10.0),
                "chest": (6.0, 0.0, -12.0), "head": (18.0, 0.0, -10.0),
                "upper_arm.L": (-30.0, 24.0, 0.0), "forearm.L": (-40.0, 0.0, 0.0),
                "upper_arm.R": (42.0, -24.0, 0.0), "hand.R": (34.0, 0.0, 0.0),
                "cape_back_01": (12.0, 0.0, 0.0), "cape_back_02": (6.0, 0.0, 0.0),
                "coat_front.R_01": (-14.0, 0.0, 0.0), "coat_front.L_01": (-14.0, 0.0, 0.0)}),
        (0.21, {"_hip": (0.78, 0.0, 0.0), "_legs.R": (0.15, 0.0, 0.0), "_legs.L": (-0.19, 0.0, 4.0),
                "pelvis": (0.0, 0.0, -3.0), "spine_01": (-3.0, 0.0, -2.0), "chest": (0.0, 0.0, -3.0),
                "head": (4.0, 0.0, -2.0), "upper_arm.L": (-8.0, 10.0, 0.0), "forearm.L": (-20.0, 0.0, 0.0),
                "upper_arm.R": (12.0, -8.0, 0.0), "hand.R": (24.0, 0.0, 0.0), "cape_back_01": (0.0, 0.0, 0.0)}),
        (0.35, {**STANCE, "spine_01": (3.0, 0.0, 0.0), "upper_arm.L": (8.0, 4.0, 0.0),
                "forearm.L": (-10.0, 0.0, 0.0), "upper_arm.R": (2.0, 0.0, 0.0), "cape_back_01": (4.0, 0.0, 0.0)}),
    ),
}

# Airborne legs of the air attack: front knee up, back leg trailing (no floor contact).
AIR_LEGS = {"_hip": (0.82, 0.0, 0.0), "_legs.R": (0.16, 0.22, 22.0), "_legs.L": (-0.14, 0.08, 34.0)}
KEYS["attack_basic_air"] = tuple(
    (t, {**{k: v for k, v in values.items() if not k.startswith("_")}, **AIR_LEGS,
         # The air under the coat lifts it a little more than on the ground.
         "coat_back.R_02": (values.get("coat_back.R_02", (0.0, 0.0, 0.0))[0] + 8.0, 0.0, 0.0),
         "coat_back.L_02": (values.get("coat_back.L_02", (0.0, 0.0, 0.0))[0] + 8.0, 0.0, 0.0)})
    for t, values in AN.ATTACK_KEYS)

# Death lying (experimental). Keys up to 0.333 s are the V1.4 collapse onto one knee
# (IK legs); then the leg IK is left: pelvis pitch tips the whole body forward,
# thighs/shins/feet are FK. The cloth is posed, not simulated: the front coat
# flaps are tucked against the thighs (they would otherwise push the body off the
# floor), the back flaps and the cape settle on the legs and the back.
DEATH_IK = (
    (0.000, {**STANCE, "spine_01": (-10.0, 0.0, 0.0), "chest": (-6.0, 0.0, 0.0), "head": (16.0, 0.0, 0.0),
             "upper_arm.L": (30.0, 18.0, 0.0), "upper_arm.R": (20.0, -12.0, 0.0), "hand.R": (35.0, 0.0, 0.0)}),
    (0.167, {"_hip": (0.62, 0.0, 0.0), "_legs.R": (0.16, 0.0, 0.0), "_legs.L": (-0.12, 0.0, 12.0),
             "spine_01": (12.0, 0.0, 0.0), "spine_02": (6.0, 0.0, 0.0), "head": (20.0, 0.0, 0.0),
             "upper_arm.L": (12.0, 10.0, 0.0), "upper_arm.R": (10.0, -8.0, 0.0), "hand.R": (55.0, 0.0, 0.0),
             "cape_back_01": (-6.0, 0.0, 0.0)}),
    (0.333, {"_hip": (0.46, 0.0, 0.0), "_legs.R": (0.16, 0.0, 0.0), "_legs.L": (-0.18, 0.0, 40.0),
             "spine_01": (16.0, 0.0, 0.0), "spine_02": (8.0, 0.0, 0.0), "head": (24.0, 0.0, 0.0),
             "upper_arm.L": (4.0, 8.0, 0.0), "upper_arm.R": (-4.0, -6.0, 0.0), "hand.R": (72.0, 0.0, 0.0),
             "cape_back_01": (6.0, 0.0, 0.0)}),
)
DEATH_FK = (
    (0.500, {"pelvis": (30.0, 0.0, 0.0), "thigh.R": (-60.0, 0.0, 0.0), "shin.R": (95.0, 0.0, 0.0),  # tipping over
             "foot.R": (-10.0, 0.0, 0.0), "thigh.L": (-25.0, 0.0, 0.0), "shin.L": (100.0, 0.0, 0.0),
             "foot.L": (10.0, 0.0, 0.0), "spine_01": (16.0, 0.0, 0.0), "spine_02": (8.0, 0.0, 0.0),
             "head": (14.0, 0.0, 0.0), "upper_arm.L": (-40.0, 10.0, 0.0), "forearm.L": (-20.0, 0.0, 0.0),
             "upper_arm.R": (-35.0, -10.0, 0.0), "forearm.R": (-10.0, 0.0, 0.0), "hand.R": (78.0, 0.0, 0.0),
             "cape_back_01": (10.0, 0.0, 0.0), "cape_arm.L_01": (6.0, 0.0, 0.0)}),
    (0.667, {"pelvis": (62.0, 0.0, 0.0), "thigh.R": (-40.0, 0.0, 0.0), "shin.R": (70.0, 0.0, 0.0),  # hands to the floor
             "foot.R": (20.0, 0.0, 0.0), "thigh.L": (-22.0, 0.0, 0.0), "shin.L": (60.0, 0.0, 0.0),
             "foot.L": (25.0, 0.0, 0.0), "spine_01": (12.0, 0.0, 0.0), "spine_02": (4.0, 0.0, 0.0),
             "head": (-10.0, 0.0, 0.0), "upper_arm.L": (-60.0, 14.0, 0.0), "forearm.L": (-10.0, 0.0, 0.0),
             "upper_arm.R": (-50.0, -14.0, 0.0), "forearm.R": (10.0, 0.0, 0.0), "hand.R": (70.0, 0.0, 0.0),
             "cape_back_01": (4.0, 0.0, 0.0), "cape_arm.L_01": (12.0, 0.0, 0.0),
             "coat_front.R_01": (6.0, 0.0, 0.0), "coat_front.L_01": (6.0, 0.0, 0.0)}),
    (0.833, {"pelvis": (86.0, 0.0, 0.0), "thigh.R": (-6.0, 0.0, 0.0), "shin.R": (22.0, 0.0, 0.0),  # chest down
             "foot.R": (30.0, 0.0, 0.0), "thigh.L": (-4.0, 0.0, 0.0), "shin.L": (14.0, 0.0, 0.0),
             "foot.L": (30.0, 0.0, 0.0), "spine_01": (2.0, 0.0, 0.0), "head": (-22.0, 0.0, 0.0),
             "upper_arm.L": (0.0, 12.0, 0.0), "forearm.L": (10.0, 0.0, 0.0), "cape_arm.L_01": (16.0, 0.0, 0.0),
             "upper_arm.R": (10.0, -20.0, 0.0), "forearm.R": (10.0, 0.0, 0.0), "hand.R": (20.0, 0.0, 0.0),
             # The cloth is still in the air: the back flaps and the cape lift off the legs.
             "cape_back_01": (8.0, 0.0, 0.0), "cape_back_02": (6.0, 0.0, 0.0),
             "coat_back.R_02": (12.0, 0.0, 0.0), "coat_back.L_02": (12.0, 0.0, 0.0)}),
    (1.000, {"pelvis": (90.0, 0.0, 0.0), "thigh.R": (-2.0, 0.0, 0.0), "shin.R": (6.0, 0.0, 0.0),  # settled
             "foot.R": (27.0, 0.0, 0.0), "thigh.L": (0.0, 0.0, 0.0), "shin.L": (4.0, 0.0, 0.0),
             "foot.L": (28.0, 0.0, 0.0), "head": (-28.0, 0.0, 0.0),
             "upper_arm.L": (10.0, 10.0, 0.0), "forearm.L": (20.0, 0.0, 0.0), "cape_arm.L_01": (12.0, 0.0, 0.0),
             "upper_arm.R": (20.0, -20.0, 0.0), "forearm.R": (40.0, 0.0, 0.0), "hand.R": (-40.0, 0.0, 0.0),
             # Settled on the body: front flaps flat under the thighs (no lift), back flaps and cape on top.
             "cape_back_01": (-8.0, 0.0, 0.0), "cape_back_02": (-4.0, 0.0, 0.0),
             "coat_back.R_01": (-6.0, 0.0, 0.0), "coat_back.L_01": (-6.0, 0.0, 0.0),
             "coat_back.R_02": (-8.0, 0.0, 0.0), "coat_back.L_02": (-8.0, 0.0, 0.0)}),
)


def _resolved(arm: bpy.types.Object, values: dict[str, AN.Euler3]) -> dict[str, AN.Euler3]:
    """Solves "_hip"/"_legs" into FK thigh/shin/foot turns."""
    out = dict(values)
    if "_hip" not in out:
        return out
    hip_z = out.pop("_hip")[0]
    for side in ("R", "L"):
        dx, lift, pitch = out.pop(f"_legs.{side}")
        out.update(AN.leg_turns(arm, side, dx, lift, hip_z, pitch))
    return out


def _death_lying_keys(arm: bpy.types.Object) -> Keys:
    return tuple((t, _resolved(arm, v)) for t, v in DEATH_IK) + DEATH_FK


def _fall_pose(arm: bpy.types.Object, frame: int) -> Pose:
    """V1.5 fall: reads as going down (one leg reaching for the floor, the other
    knee tucked, arms raised by the air, cloth blown up), unlike jump_rise
    (legs tucked under, arm swung back)."""
    phase = frame / CLIPS["fall_v15"].frames
    flutter = math.sin(2.0 * math.pi * phase)
    lag = math.sin(2.0 * math.pi * (phase - 0.15))
    values: dict[str, AN.Euler3] = {}
    values.update(AN.leg_turns(arm, "R", 0.10, 0.02, 0.82, 26.0))  # front leg reaching down, toes pointed
    values.update(AN.leg_turns(arm, "L", -0.06, 0.24, 0.82, 30.0))  # back knee tucked up
    values.update({
        "pelvis": (-4.0, 0.0, 0.0), "spine_01": (-2.0, 0.0, 0.0), "chest": (-2.0, 0.0, 0.0),
        "head": (16.0, 0.0, 0.0),  # looking down at the landing
        "upper_arm.L": (-62.0 - 3.0 * flutter, 34.0, 0.0), "forearm.L": (-30.0, 0.0, 0.0),
        # Sword arm out to the side for balance, blade angled down: the tip stays in the cell.
        "upper_arm.R": (-12.0 + 3.0 * flutter, -34.0, 0.0), "forearm.R": (-12.0, 0.0, 0.0),
        "hand.R": (55.0, 0.0, 0.0),
        # Air rushes up past the body: the cloth rises higher than in V1.4 and flutters.
        "cape_back_01": (40.0 + 6.0 * lag, 0.0, 0.0), "cape_back_02": (22.0 + 6.0 * lag, 0.0, 0.0),
        "cape_arm.L_01": (20.0 + 4.0 * lag, 0.0, 0.0), "cape_arm.L_02": (10.0 + 3.0 * lag, 0.0, 0.0),
        "coat_back.R_01": (10.0, 0.0, 0.0), "coat_back.L_01": (10.0, 0.0, 0.0),
        "coat_back.R_02": (22.0 + 6.0 * lag, 0.0, 0.0), "coat_back.L_02": (22.0 - 6.0 * lag, 0.0, 0.0),
        "coat_front.R_01": (-12.0 - 3.0 * flutter, 0.0, 0.0), "coat_front.L_01": (-12.0 + 3.0 * flutter, 0.0, 0.0),
    })
    return Pose(f"fall_v15_{frame:03d}", f"fall_v15 f{frame}", AN.turns(values), ground="air")


def clip_pose(arm: bpy.types.Object, key: str, index: int) -> Pose:
    if key == "fall_v15":
        return _fall_pose(arm, index)
    t = times(key)[index]
    if key == "death_lying":
        values = AN.sample_keys(_death_lying_keys(arm), t)
    else:
        values = _resolved(arm, AN.sample_keys(KEYS[key], t))
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
    action.name = f"GAME_V15_{key}"
    action.use_fake_user = True
    AN._constant_interpolation(action)
    return action
