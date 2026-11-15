"""Technical animation tests of V1.2-G: walk cycle, basic attack, dodge.

Experimental, not the final art. Every frame is computed as a ``Pose`` of
the v005 pose system (rotations in world axes at rest, FK, grounding of
the root) and keyed on its own frame, so what is rendered is exactly what
is validated; there is no interpolation between keys in Blender.

Timing (production reference: 24 fps, ``rig_config.PRODUCTION_FPS``):
- walk: loop of ``WALK_CYCLE_FRAMES`` frames, frame f at phase f/N; the
  frame N would equal frame 0 and is NOT exported (no duplicated frame);
- attack_basic and dodge: one-shot clips of exactly 0.28 s (the game's
  ``BASIC_ATTACK_TIME``/``PLAYER_DODGE_TIME``). Frame i samples t = i/24 for
  i = 0..6; frames 0-5 last 1/24 s and frame 6 lasts 0.28 - 6/24 = 0.03 s,
  so the clip lasts 0.28 s without stretching it to a whole frame count.

Root motion: none. The root only moves vertically (grounding, jump of the
hips); the game moves the entity, so the sprites stay in place.

Attack: the hitbox exists from the first frame of the action
(``src/combat/attacks.rs``), so frame 0 is already the strike (blade in
mid-arc, in front of the body); the rest is follow-through and recovery,
and every frame is a valid pose to leave the clip from.

Walk legs: a sagittal two-bone IK computed here (law of cosines) turns an
ankle path into thigh, knee and foot angles, so the stance foot stays flat
and moves backward at constant speed (in place) and the swing foot lifts.
"""

from __future__ import annotations

import math
from dataclasses import dataclass

import bpy

import rig_config as C
from poses import Pose

Turns = dict[str, tuple[tuple[str, float], ...]]
Euler3 = tuple[float, float, float]  # degrees about world Y, X, Z (applied in that order)


@dataclass(frozen=True)
class Clip:
    key: str  # contract identifier (docs/ASSET_CONTRACT.md, section 2)
    group: str  # "locomotion" or "combat"
    loop: bool
    frames: int
    durations: tuple[float, ...]  # seconds per exported frame

    @property
    def duration(self) -> float:
        return sum(self.durations)

    def times(self) -> tuple[float, ...]:
        """Start of each frame: i / PRODUCTION_FPS (exact, not a running sum of rounded durations)."""
        return tuple(round(i / C.PRODUCTION_FPS, 6) for i in range(self.frames))


def _one_shot(key: str, group: str, seconds: float) -> Clip:
    count = math.ceil(seconds * C.PRODUCTION_FPS - 1e-9)
    durations = tuple(min(1.0 / C.PRODUCTION_FPS, seconds - i / C.PRODUCTION_FPS) for i in range(count))
    return Clip(key, group, False, count, durations)


CLIPS = {
    "walk": Clip("walk", "locomotion", True, C.WALK_CYCLE_FRAMES,
                 tuple(1.0 / C.PRODUCTION_FPS for _ in range(C.WALK_CYCLE_FRAMES))),
    "attack_basic": _one_shot("attack_basic", "combat", C.BASIC_ATTACK_TIME_S),
    "dodge": _one_shot("dodge", "locomotion", C.PLAYER_DODGE_TIME_S),
}


def turns(values: dict[str, Euler3]) -> Turns:
    out: Turns = {}
    for bone, (y, x, z) in values.items():
        seq = tuple((a, d) for a, d in (("Y", y), ("X", x), ("Z", z)) if abs(d) > 1e-9)
        if seq:
            out[bone] = seq
    return out


# --- walk ----------------------------------------------------------------------------------


@dataclass(frozen=True)
class Leg:
    hip: tuple[float, float]  # (x, z) at rest
    thigh: float
    shin: float
    rest_thigh: float  # angle from the downward vertical, + = forward (rad)
    rest_bend: float  # knee bend at rest (rad)


def leg_from_rig(arm: bpy.types.Object, side: str) -> Leg:
    b = arm.data.bones
    hip, knee, ankle = b[f"thigh.{side}"].head_local, b[f"shin.{side}"].head_local, b[f"shin.{side}"].tail_local
    thigh_angle = math.atan2(knee.x - hip.x, hip.z - knee.z)
    shin_angle = math.atan2(ankle.x - knee.x, knee.z - ankle.z)
    return Leg((hip.x, hip.z), (knee - hip).length, (ankle - knee).length, thigh_angle, thigh_angle - shin_angle)


def solve_leg(leg: Leg, dx: float, dz: float) -> tuple[float, float]:
    """(thigh angle from vertical, knee bend) that put the ankle at (dx, dz) from the hip."""
    reach = min(math.hypot(dx, dz), leg.thigh + leg.shin - 1e-4)
    cos_knee = (leg.thigh ** 2 + leg.shin ** 2 - reach ** 2) / (2 * leg.thigh * leg.shin)
    bend = math.pi - math.acos(max(-1.0, min(1.0, cos_knee)))
    cos_hip = (leg.thigh ** 2 + reach ** 2 - leg.shin ** 2) / (2 * leg.thigh * reach)
    thigh = math.atan2(dx, -dz) + math.acos(max(-1.0, min(1.0, cos_hip)))  # knee in front
    return thigh, bend


# Distances from the ankle along the sole (m), for the foot-roll compensation:
# rolling onto the ball (toes down) or the heel (toes up) about a point on the
# ground lifts the ankle instead of pushing the toe or heel through the floor.
BALL_M = 0.13
HEEL_M = 0.05


def roll_lift(pitch_deg: float) -> float:
    p = math.radians(pitch_deg)
    return BALL_M * math.sin(p) if p > 0 else HEEL_M * math.sin(-p)


def leg_turns(arm: bpy.types.Object, side: str, dx: float, lift: float, hip_z: float,
              pitch: float) -> dict[str, Euler3]:
    """Thigh, shin and foot turns that put the ankle at (dx, lift) under a hip at ``hip_z``.

    ``pitch`` (deg, + = toes down) is the foot angle against the ground; the
    ankle is raised by ``roll_lift`` so the ball or heel stays on the floor.
    """
    leg = leg_from_rig(arm, side)
    thigh, bend = solve_leg(leg, dx, -(hip_z - lift - roll_lift(pitch)))
    thigh_turn = -math.degrees(thigh - leg.rest_thigh)  # thigh forward = negative world Y
    shin_turn = math.degrees(bend - leg.rest_bend)  # knee bend = positive world Y
    foot_turn = -(thigh_turn + shin_turn) + pitch  # level sole, then the roll
    return {f"thigh.{side}": (thigh_turn, 0.0, 0.0), f"shin.{side}": (shin_turn, 0.0, 0.0),
            f"foot.{side}": (foot_turn, 0.0, 0.0)}


WALK = {
    "step_m": 0.50,  # ankle travel during stance (front to back, relative to the hip)
    "stance": 0.6,  # stance share of the cycle
    "lift_m": 0.10,  # swing-foot clearance
    # Hip above the stance ankle (rest, straight leg: 0.855 m). Below the reach at
    # the ends of the stride too (hypot(0.25, 0.815) < 0.855): the knee never locks
    # and the IK never clamps, so the stance foot really stays on the ground.
    "hip_height_m": 0.80,
    "bob_m": 0.015,  # hips higher while passing (2 bobs per cycle)
    "toe_off_deg": 18.0,  # foot pitch at the end of stance
    "heel_strike_deg": -10.0,  # toes up at contact
}


def _foot_path(phase: float) -> tuple[float, float, float]:
    """Ankle (dx, lift, foot pitch) of a leg at phase 0..1; phase 0 = heel contact in front."""
    w = WALK
    half = w["step_m"] / 2.0
    if phase < w["stance"]:
        s = phase / w["stance"]
        pitch = w["heel_strike_deg"] * max(0.0, 1.0 - s / 0.15) + w["toe_off_deg"] * max(0.0, (s - 0.8) / 0.2)
        return half - s * w["step_m"], 0.0, pitch
    s = (phase - w["stance"]) / (1.0 - w["stance"])
    ease = 0.5 - 0.5 * math.cos(math.pi * s)
    pitch = w["toe_off_deg"] * (1.0 - s) + w["heel_strike_deg"] * s  # toes come up before contact
    return -half + ease * w["step_m"], w["lift_m"] * math.sin(math.pi * s), pitch


def walk_pose(arm: bpy.types.Object, frame: int) -> Pose:
    n = C.WALK_CYCLE_FRAMES
    phase = frame / n
    w = WALK
    hip_z = w["hip_height_m"] + w["bob_m"] * math.cos(4.0 * math.pi * (phase - 0.3))
    values: dict[str, Euler3] = {}
    for side, offset in (("R", 0.0), ("L", 0.5)):
        dx, lift, pitch = _foot_path((phase + offset) % 1.0)
        values.update(leg_turns(arm, side, dx, lift, hip_z, pitch))
    right_forward = math.cos(2.0 * math.pi * phase)  # +1 at right heel contact
    swing = math.sin(2.0 * math.pi * (phase + 0.25))  # = right_forward, written as an arm swing
    values.update({
        "pelvis": (0.0, 0.0, 4.0 * right_forward),  # + yaw brings the right hip forward
        "spine_01": (4.0, 0.0, 0.0),
        "chest": (0.0, 0.0, -5.0 * right_forward),  # shoulders counter-rotate
        "head": (-3.0, 0.0, 1.5 * right_forward),
        # Left arm swings against the left leg (forward when the right leg is forward).
        "upper_arm.L": (-18.0 * swing, 0.0, 0.0),
        "forearm.L": (-14.0 - 8.0 * max(0.0, swing), 0.0, 0.0),
        # Sword arm: small swing, the sabre stays down and forward.
        "upper_arm.R": (7.0 * swing, 0.0, 0.0),
        "forearm.R": (-4.0, 0.0, 0.0),
        # Secondary motion, slightly behind the body (phase lag).
        "coat_back.R_02": (6.0 + 4.0 * math.sin(2.0 * math.pi * (phase - 0.12)), 0.0, 0.0),
        "coat_back.L_02": (6.0 + 4.0 * math.sin(2.0 * math.pi * (phase + 0.38)), 0.0, 0.0),
        "cape_back_01": (8.0 + 3.0 * math.sin(4.0 * math.pi * (phase - 0.1)), 0.0, 0.0),
        "cape_back_02": (4.0 + 2.0 * math.sin(4.0 * math.pi * (phase - 0.18)), 0.0, 0.0),
        "cape_arm.L_01": (3.0 + 2.0 * math.sin(2.0 * math.pi * (phase - 0.1)), 0.0, 0.0),
    })
    return Pose(f"walk_{frame:03d}", f"walk f{frame}", turns(values))


def walk_speed_m_s() -> float:
    """Body speed that makes the stance foot static on the ground (in-place cycle)."""
    w = WALK
    stance_s = w["stance"] * C.WALK_CYCLE_FRAMES / C.PRODUCTION_FPS
    return w["step_m"] / stance_s


# --- one-shot clips: keyed poses, Catmull-Rom between keys ------------------------------------

# Leg entries are not bone turns: "_hip" = (hip height above the ankle line, 0, 0)
# and "_legs.<side>" = (ankle dx from the hip, ankle lift, foot pitch); they are
# interpolated like the rest and solved by ``leg_turns`` (both feet on the floor).
ATTACK_KEYS: tuple[tuple[float, dict[str, Euler3]], ...] = (
    (0.00, {  # strike: blade in mid-arc in front of the body (hitbox already active)
        "_hip": (0.76, 0.0, 0.0), "_legs.R": (0.30, 0.0, 0.0), "_legs.L": (-0.30, 0.0, 8.0),
        "pelvis": (0.0, 0.0, 10.0), "spine_01": (6.0, 0.0, 6.0), "spine_02": (0.0, 0.0, 12.0),
        "chest": (0.0, 0.0, 12.0), "head": (-4.0, 0.0, -8.0),
        "upper_arm.R": (-75.0, -10.0, 0.0), "forearm.R": (-20.0, 0.0, 0.0), "hand.R": (-15.0, 0.0, 0.0),
        "upper_arm.L": (25.0, 8.0, 0.0), "forearm.L": (-15.0, 0.0, 0.0),
        "coat_back.R_02": (14.0, 0.0, 0.0), "coat_back.L_02": (14.0, 0.0, 0.0), "cape_back_01": (18.0, 0.0, 0.0),
    }),
    (0.08, {  # follow-through: the blade sweeps down and forward, torso keeps turning
        "_hip": (0.74, 0.0, 0.0), "_legs.R": (0.33, 0.0, 0.0), "_legs.L": (-0.33, 0.0, 12.0),
        "pelvis": (0.0, 0.0, 14.0), "spine_01": (10.0, 0.0, 8.0), "spine_02": (0.0, 0.0, 14.0),
        "chest": (0.0, 0.0, 16.0), "head": (-6.0, 0.0, -10.0),
        "upper_arm.R": (-55.0, -6.0, 0.0), "forearm.R": (-8.0, 0.0, 0.0), "hand.R": (25.0, 0.0, 0.0),
        "upper_arm.L": (32.0, 10.0, 0.0), "forearm.L": (-20.0, 0.0, 0.0),
        "coat_back.R_02": (22.0, 0.0, 0.0), "coat_back.L_02": (22.0, 0.0, 0.0), "cape_back_01": (28.0, 0.0, 0.0),
        "cape_back_02": (10.0, 0.0, 0.0),
    }),
    (0.17, {  # end of the hitbox window: blade low, weight settling on the front leg
        "_hip": (0.76, 0.0, 0.0), "_legs.R": (0.30, 0.0, 0.0), "_legs.L": (-0.30, 0.0, 8.0),
        "pelvis": (0.0, 0.0, 9.0), "spine_01": (8.0, 0.0, 5.0), "spine_02": (0.0, 0.0, 8.0),
        "chest": (0.0, 0.0, 9.0), "head": (-4.0, 0.0, -6.0),
        "upper_arm.R": (-35.0, -4.0, 0.0), "forearm.R": (-10.0, 0.0, 0.0), "hand.R": (35.0, 0.0, 0.0),
        "upper_arm.L": (22.0, 6.0, 0.0), "forearm.L": (-15.0, 0.0, 0.0),
        "coat_back.R_02": (16.0, 0.0, 0.0), "coat_back.L_02": (16.0, 0.0, 0.0), "cape_back_01": (20.0, 0.0, 0.0),
        "cape_back_02": (8.0, 0.0, 0.0),
    }),
    (0.28, {  # recovery: ready stance, close to idle (any next clip can start from here)
        "_hip": (0.81, 0.0, 0.0), "_legs.R": (0.15, 0.0, 0.0), "_legs.L": (-0.15, 0.0, 0.0),
        "pelvis": (0.0, 0.0, 3.0), "spine_01": (4.0, 0.0, 2.0), "chest": (0.0, 0.0, 3.0),
        "upper_arm.R": (-12.0, 0.0, 0.0), "forearm.R": (-12.0, 0.0, 0.0), "hand.R": (10.0, 0.0, 0.0),
        "upper_arm.L": (8.0, 3.0, 0.0), "forearm.L": (-10.0, 0.0, 0.0),
        "coat_back.R_02": (6.0, 0.0, 0.0), "coat_back.L_02": (6.0, 0.0, 0.0), "cape_back_01": (8.0, 0.0, 0.0),
    }),
)

DODGE_KEYS: tuple[tuple[float, dict[str, Euler3]], ...] = (
    (0.00, {  # push-off: already low and leaning (the dash starts with the action)
        "_hip": (0.70, 0.0, 0.0), "_legs.R": (0.28, 0.0, 0.0), "_legs.L": (-0.35, 0.0, 20.0),
        "spine_01": (12.0, 0.0, 0.0), "spine_02": (6.0, 0.0, 0.0), "head": (-8.0, 0.0, 0.0),
        "upper_arm.L": (25.0, 0.0, 0.0), "upper_arm.R": (15.0, 0.0, 0.0), "hand.R": (30.0, 0.0, 0.0),
        # The cape opens before the coat panels (coat over cape otherwise, V1.2-G iteration 1).
        "coat_back.R_01": (8.0, 0.0, 0.0), "coat_back.L_01": (8.0, 0.0, 0.0), "cape_back_01": (32.0, 0.0, 0.0),
    }),
    (0.07, {  # full dash: lowest, longest silhouette (as P09)
        "_hip": (0.62, 0.0, 0.0), "_legs.R": (0.38, 0.0, 0.0), "_legs.L": (-0.48, 0.0, 30.0),
        "spine_01": (15.0, 0.0, 0.0), "spine_02": (10.0, 0.0, 0.0), "chest": (5.0, 0.0, 0.0),
        "head": (-6.0, 0.0, 0.0),
        "upper_arm.L": (40.0, 0.0, 0.0), "upper_arm.R": (25.0, 0.0, 0.0), "hand.R": (55.0, 0.0, 0.0),
        "coat_back.R_01": (26.0, 0.0, 0.0), "coat_back.L_01": (26.0, 0.0, 0.0),
        "coat_back.R_02": (18.0, 0.0, 0.0), "coat_back.L_02": (18.0, 0.0, 0.0),
        "cape_back_01": (48.0, 0.0, 0.0), "cape_arm.L_01": (22.0, 0.0, 0.0),
    }),
    (0.17, {  # still dashing, the cloth trails further
        "_hip": (0.64, 0.0, 0.0), "_legs.R": (0.36, 0.0, 0.0), "_legs.L": (-0.44, 0.0, 28.0),
        "spine_01": (14.0, 0.0, 0.0), "spine_02": (9.0, 0.0, 0.0), "chest": (4.0, 0.0, 0.0),
        "head": (-6.0, 0.0, 0.0),
        "upper_arm.L": (35.0, 0.0, 0.0), "upper_arm.R": (22.0, 0.0, 0.0), "hand.R": (50.0, 0.0, 0.0),
        "coat_back.R_01": (30.0, 0.0, 0.0), "coat_back.L_01": (30.0, 0.0, 0.0),
        "coat_back.R_02": (22.0, 0.0, 0.0), "coat_back.L_02": (22.0, 0.0, 0.0),
        "cape_back_01": (52.0, 0.0, 0.0), "cape_back_02": (15.0, 0.0, 0.0), "cape_arm.L_01": (25.0, 0.0, 0.0),
    }),
    (0.28, {  # recovery: rising toward the stance, cloth settling
        "_hip": (0.79, 0.0, 0.0), "_legs.R": (0.18, 0.0, 0.0), "_legs.L": (-0.20, 0.0, 5.0),
        "spine_01": (6.0, 0.0, 0.0), "upper_arm.L": (10.0, 0.0, 0.0),
        "upper_arm.R": (6.0, 0.0, 0.0), "hand.R": (15.0, 0.0, 0.0),
        "coat_back.R_01": (12.0, 0.0, 0.0), "coat_back.L_01": (12.0, 0.0, 0.0), "cape_back_01": (20.0, 0.0, 0.0),
        "cape_back_02": (6.0, 0.0, 0.0),
    }),
)


def _catmull_rom(p0: float, p1: float, p2: float, p3: float, s: float) -> float:
    return 0.5 * ((2 * p1) + (-p0 + p2) * s + (2 * p0 - 5 * p1 + 4 * p2 - p3) * s * s
                  + (-p0 + 3 * p1 - 3 * p2 + p3) * s ** 3)


def sample_keys(keys: tuple[tuple[float, dict[str, Euler3]], ...], t: float) -> dict[str, Euler3]:
    """Catmull-Rom through the keys (end keys repeated); missing bones are 0."""
    times = [k[0] for k in keys]
    t = min(max(t, times[0]), times[-1])
    i = max(j for j in range(len(times) - 1) if times[j] <= t) if t < times[-1] else len(times) - 2
    s = (t - times[i]) / (times[i + 1] - times[i])
    idx = (max(i - 1, 0), i, i + 1, min(i + 2, len(keys) - 1))
    bones = {b for _, values in keys for b in values}
    out = {}
    for bone in bones:
        pts = [keys[j][1].get(bone, (0.0, 0.0, 0.0)) for j in idx]
        out[bone] = tuple(_catmull_rom(*(p[a] for p in pts), s) for a in range(3))
    return out


def clip_pose(arm: bpy.types.Object, key: str, index: int) -> Pose:
    clip = CLIPS[key]
    if key == "walk":
        return walk_pose(arm, index)
    t = clip.times()[index]
    keys = ATTACK_KEYS if key == "attack_basic" else DODGE_KEYS
    values = sample_keys(keys, t)
    hip_z = values.pop("_hip")[0]
    for side in ("R", "L"):
        dx, lift, pitch = values.pop(f"_legs.{side}")
        values.update(leg_turns(arm, side, dx, lift, hip_z, pitch))
    return Pose(f"{key}_{index:03d}", f"{key} f{index} t={t:.3f}s", turns(values))


def derived_indices(clip: Clip, fps: int) -> tuple[tuple[int, float], ...]:
    """(source frame, duration) of the clip resampled at ``fps`` (every k-th 24 fps frame)."""
    step = C.PRODUCTION_FPS // fps
    picked = list(range(0, clip.frames, step))
    out = []
    for n, i in enumerate(picked):
        end = clip.duration if n + 1 == len(picked) else clip.times()[picked[n + 1]]
        out.append((i, round(end - clip.times()[i], 6)))
    return tuple(out)


def build_action(arm: bpy.types.Object, key: str, body: list[bpy.types.Object]) -> tuple[bpy.types.Action, list[float]]:
    """Keys every frame of the clip (frame 1 = index 0); returns the action and root heights."""
    from poses import apply

    arm.animation_data_create()
    arm.animation_data.action = None
    heights = []
    for i in range(CLIPS[key].frames):
        heights.append(apply(arm, clip_pose(arm, key, i), body))
        for pb in arm.pose.bones:
            pb.keyframe_insert("rotation_quaternion", frame=i + 1)
        arm.pose.bones["root"].keyframe_insert("location", frame=i + 1)
    action = arm.animation_data.action
    action.name = f"ANIM_{key}"
    action.use_fake_user = True
    _constant_interpolation(action)
    return action, heights


def _constant_interpolation(action: bpy.types.Action) -> None:
    """Every frame is a key: hold it (sprite sampling), never blend between frames."""
    curves = []
    if hasattr(action, "layers") and action.layers:
        curves = [fc for layer in action.layers for strip in layer.strips for bag in strip.channelbags
                  for fc in bag.fcurves]
    else:
        curves = list(getattr(action, "fcurves", []))
    for fc in curves:
        for kp in fc.keyframe_points:
            kp.interpolation = "CONSTANT"
