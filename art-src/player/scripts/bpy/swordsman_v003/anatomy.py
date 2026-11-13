"""Body of v003: torso (under a fitted vest), neck, arm curves, hands, legs.

The torso is a loft of anatomical cross-sections (hip, waist, ribs, chest
with pectoral volume, shoulders, trapezius). ``torso_radius`` is shared with
``clothing`` so the coat follows the body instead of crossing it. The arms
themselves are only curves here: what is seen of them is the coat sleeve
(``clothing``) and the gloved hands built below.
"""

from __future__ import annotations

import math

from mathutils import Vector

import params as P
from mesh_tools import (
    Vec3,
    catmull_rom,
    gauss,
    grid_object,
    profile,
    radial_rows,
    subdivide,
    tube,
)
from surfaces import shaded

# Torso sections: z -> (centre x, front depth, back depth, half width).
TORSO_KEYS = (
    (0.90, (0.000, 0.080, 0.085, 0.118)),
    (0.98, (0.000, 0.092, 0.098, 0.142)),
    (1.06, (0.004, 0.084, 0.088, 0.126)),
    (P.WAIST_Z, (0.008, 0.080, 0.080, 0.116)),
    (1.22, (0.012, 0.090, 0.084, 0.132)),
    (1.31, (0.012, 0.100, 0.088, 0.148)),
    (1.39, (0.008, 0.096, 0.088, 0.160)),
    (1.45, (0.000, 0.078, 0.080, 0.168)),
    (1.495, (-0.006, 0.058, 0.066, 0.110)),
    (1.53, (-0.008, 0.044, 0.046, 0.050)),
)
TORSO_BOTTOM = TORSO_KEYS[0][0]
TORSO_TOP = TORSO_KEYS[-1][0]
SUPERELLIPSE = 2.6  # >2 squares the section slightly (ribcage, not a cylinder)


def torso_section(z: float) -> tuple[float, float, float, float]:
    zs = [k[0] for k in TORSO_KEYS]
    t = min(max(z, zs[0]), zs[-1])
    return tuple(profile([(k[0], k[1][i]) for k in TORSO_KEYS], t) for i in range(4))  # type: ignore[return-value]


def torso_radius(z: float, theta: float) -> tuple[float, float]:
    """(x, y) of the torso surface at height z and angle theta (0 = front)."""
    cx, front, back, half_w = torso_section(z)
    c, s = math.cos(theta), math.sin(theta)
    depth = front if c >= 0.0 else back
    k = 2.0 / SUPERELLIPSE
    x = math.copysign(abs(c) ** k, c) * depth
    y = math.copysign(abs(s) ** k, s) * half_w
    # Pectorals and shoulder blades: small radial bulges.
    bulge = 0.0
    if c > 0.0:
        bulge += 0.011 * gauss(abs(theta), 0.5, 0.32) * gauss(z, 1.32, 0.055)
    else:
        bulge += 0.007 * gauss(abs(abs(theta) - math.pi), 0.55, 0.35) * gauss(z, 1.38, 0.06)
    norm = math.hypot(x, y) or 1.0
    return (cx + x + bulge * x / norm, y + bulge * y / norm)


def build_torso() -> None:
    zs = [TORSO_BOTTOM + (TORSO_TOP - TORSO_BOTTOM) * i / 27 for i in range(28)]

    def point(z: float, theta: float) -> Vec3:
        x, y = torso_radius(z, theta)
        return (x, y, z)

    obj = grid_object("torso_vest", radial_rows(zs, 40, point), [shaded("vest")],
                      closed=True, cap_start=True, cap_end=True)
    subdivide(obj, 1)
    # Waistcoat buttons down the visible front.
    for i, z in enumerate((1.17, 1.235, 1.30, 1.365)):
        x, _ = torso_radius(z, 0.0)
        tube(f"vest_button_{i}", [(x - 0.004, 0.0, z), (x + 0.006, 0.0, z)],
             [(0.008, 0.008), (0.007, 0.007)], [shaded("metal_dark")], sides=10, ref=(0.0, 0.0, 1.0))


def build_neck() -> None:
    hx, _, hz = P.HEAD_CENTRE
    path = catmull_rom([(-0.012, 0.0, P.NECK_BASE_Z), (-0.002, 0.0, 1.55), (hx - 0.01, 0.0, hz - 0.03)], 8)
    radii = [(r * 1.05, r) for r in (0.050, 0.047, 0.044, 0.042, 0.041, 0.041, 0.042, 0.044)]
    obj = tube("neck", path, radii, [shaded("skin")], sides=14, ref=(1.0, 0.0, 0.0))
    subdivide(obj, 1)


# --- arms -------------------------------------------------------------------


def arm_curve(near: bool) -> list[Vector]:
    """Shoulder (inside the torso) -> elbow -> wrist, as a smooth curve."""
    if near:
        s, e, w = P.SHOULDER_NEAR, P.ELBOW_NEAR, P.WRIST_NEAR
    else:
        s, e, w = P.SHOULDER_FAR, P.ELBOW_FAR, P.WRIST_FAR
    side = -1.0 if near else 1.0
    inner = (s[0] - 0.005, s[1] - side * 0.05, s[2] + 0.005)
    cap = (s[0], s[1] + side * 0.03, s[2] - 0.02)
    return catmull_rom([inner, cap, e, w], 18)


# Sleeve radius along the arm curve (t from shoulder to wrist): deltoid,
# biceps, elbow, forearm, wrist.
ARM_RADII = ((0.0, 0.042), (0.12, 0.049), (0.35, 0.045), (0.55, 0.040),
             (0.68, 0.045), (0.9, 0.039), (1.0, 0.037))


# --- hands ------------------------------------------------------------------


def _finger(name: str, points: list[Vec3], base: float, tip: float) -> None:
    path = catmull_rom(points, 7)
    radii = [(base + (tip - base) * i / 6,) * 2 for i in range(7)]
    obj = tube(name, path, radii, [shaded("glove_leather")], sides=8)
    subdivide(obj, 1)


def build_sword_hand(grip_centre: Vector, grip_axis: Vector, wrist: Vector) -> None:
    """Gloved fist closed around the grip: palm, four fingers, thumb."""
    out = Vector((0.0, -1.0, 0.0))  # back of the hand faces the side camera
    across = grip_axis.cross(out).normalized()
    knuckle_c = grip_centre + out * 0.022
    palm = catmull_rom([wrist, wrist.lerp(knuckle_c, 0.5) + out * 0.004, knuckle_c], 6)
    radii = [(0.028, 0.017), (0.032, 0.018), (0.035, 0.018), (0.036, 0.017), (0.035, 0.016), (0.032, 0.014)]
    obj = tube("glove_near_palm", palm, radii, [shaded("glove_leather")], sides=12, ref=tuple(grip_axis))
    subdivide(obj, 1)
    for i in range(4):
        along = grip_axis * (-0.026 + 0.018 * i)
        r = 0.0215
        pts = []
        for k in range(5):
            ang = math.radians(-30 + 62 * k)
            pts.append(tuple(grip_centre + along + out * (r * math.cos(ang)) + across * (r * math.sin(ang))))
        _finger(f"glove_near_finger_{i}", pts, 0.0105, 0.0085)
    thumb = [tuple(wrist + out * 0.01 - across * 0.012),
             tuple(grip_centre - grip_axis * 0.03 - across * 0.02 + out * 0.005),
             tuple(grip_centre - grip_axis * 0.005 - across * 0.024 - out * 0.004),
             tuple(grip_centre + grip_axis * 0.012 - across * 0.016 - out * 0.012)]
    _finger("glove_near_thumb", thumb, 0.012, 0.009)


def build_free_hand(wrist: Vector) -> None:
    """Relaxed far hand hanging beside the thigh, fingers slightly curled."""
    down = Vector((0.03, 0.0, -1.0)).normalized()
    inward = Vector((0.0, -1.0, 0.0))  # palm toward the leg
    knuckles = wrist + down * 0.075
    palm = catmull_rom([wrist, wrist.lerp(knuckles, 0.5), knuckles], 6)
    radii = [(0.017, 0.027), (0.018, 0.032), (0.018, 0.035), (0.017, 0.036), (0.016, 0.035), (0.014, 0.031)]
    obj = tube("glove_far_palm", palm, radii, [shaded("glove_leather")], sides=12, ref=(0.0, 1.0, 0.0))
    subdivide(obj, 1)
    forward = Vector((1.0, 0.0, 0.0))
    for i in range(4):
        base = knuckles + forward * (0.021 - 0.014 * i) - down * 0.004
        lengths = 0.07 - abs(i - 1.3) * 0.008
        pts = [tuple(base), tuple(base + down * lengths * 0.5 + inward * 0.006),
               tuple(base + down * lengths * 0.85 + inward * 0.02),
               tuple(base + down * lengths + inward * 0.036)]
        _finger(f"glove_far_finger_{i}", pts, 0.0095, 0.0078)
    thumb = [tuple(wrist + forward * 0.02 + down * 0.02 + inward * 0.004),
             tuple(wrist + forward * 0.04 + down * 0.05 + inward * 0.012),
             tuple(wrist + forward * 0.045 + down * 0.078 + inward * 0.022)]
    _finger("glove_far_thumb", thumb, 0.011, 0.0085)


# --- legs -------------------------------------------------------------------

# Leg radius along the curve (t from hip to ankle): thigh, above knee, knee,
# calf, ankle. Trousers are fitted, so this is the visible leg shape.
LEG_RADII = ((0.0, 0.080), (0.2, 0.071), (0.42, 0.054), (0.5, 0.050),
             (0.62, 0.056), (0.72, 0.054), (0.92, 0.038), (1.0, 0.036))


def leg_curve(near: bool) -> list[Vector]:
    hip, knee, ankle = P.LEG_NEAR if near else P.LEG_FAR
    return catmull_rom([hip, knee, ankle], 22)


def leg_radius(t: float) -> float:
    return profile(LEG_RADII, t)


def build_legs() -> None:
    for side, near in (("near", True), ("far", False)):
        path = leg_curve(near)
        n = len(path)
        # Calf muscle sits behind the shin: slightly deeper (a) than wide (b).
        radii = [(leg_radius(i / (n - 1)) * (1.0 + 0.1 * math.sin(math.pi * i / (n - 1))),
                  leg_radius(i / (n - 1))) for i in range(n)]
        obj = tube(f"leg_{side}", path, radii, [shaded("trousers")], sides=14, ref=(1.0, 0.0, 0.0))
        subdivide(obj, 1)
