"""Body of v004: torso split into waistcoat and trousers, arms and legs.

v003 (``swordsman_v003/anatomy.py``, untouched) lofted one torso in the vest
material from the hips to the neck, which read as a jumpsuit, and its legs
and sleeves were round tubes with a radius profile. v004:

- **torso**: same sections, with a sloped trapezius between neck and
  shoulder (v003 dropped from 0.168 to 0.110 half-width in 4.5 cm). The
  trousers cover the hips; the **waistcoat** is a separate shell slightly
  proud of them, whose hem is a smooth curve ending in two points in front
  (the classic waistcoat cut) with a trim, a placket and buttons;
- **legs**: ``shapes.profiled_tube`` with a front thigh volume, a knee
  that stands out in front, a calf behind the shin and a narrow ankle. The
  boots (``garments``) reuse the same radius so the calf shows through;
- **arms**: only the curves and radius live here (the sleeves are
  ``garments``); deltoid on the outer side, biceps in front, the elbow
  point behind, forearm swell and a narrow wrist.

The hands, the neck and the pose curves are reused from v003 unchanged.
"""

from __future__ import annotations

import math

from mathutils import Vector

import params as P
from mesh_tools import Vec3, catmull_rom, gauss, grid_object, profile, smoothstep, subdivide, tube
from shapes import lobe, profiled_tube
from surfaces import shaded

# Torso sections: z -> (centre x, front depth, back depth, half width).
# v003 keys with the trapezius slope (1.47-1.51) added.
TORSO_KEYS = (
    (0.90, (0.000, 0.080, 0.085, 0.118)),
    (0.98, (0.000, 0.092, 0.098, 0.142)),
    (1.06, (0.004, 0.084, 0.088, 0.126)),
    (P.WAIST_Z, (0.008, 0.080, 0.080, 0.116)),
    (1.22, (0.012, 0.090, 0.084, 0.132)),
    (1.31, (0.012, 0.100, 0.088, 0.148)),
    (1.39, (0.008, 0.096, 0.088, 0.160)),
    (1.445, (0.002, 0.080, 0.082, 0.166)),
    (1.475, (-0.003, 0.066, 0.074, 0.138)),
    (1.505, (-0.007, 0.052, 0.058, 0.096)),
    (1.535, (-0.008, 0.044, 0.046, 0.052)),
)
TORSO_BOTTOM = TORSO_KEYS[0][0]
TORSO_TOP = TORSO_KEYS[-1][0]
SUPERELLIPSE = 2.6


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
    bulge = 0.0
    if c > 0.0:
        bulge += 0.011 * gauss(abs(theta), 0.5, 0.32) * gauss(z, 1.32, 0.055)  # pectorals
    else:
        bulge += 0.007 * gauss(abs(abs(theta) - math.pi), 0.55, 0.35) * gauss(z, 1.38, 0.06)  # blades
    norm = math.hypot(x, y) or 1.0
    return (cx + x + bulge * x / norm, y + bulge * y / norm)


def torso_point(z: float, theta: float, extra: float = 0.0) -> Vector:
    x, y = torso_radius(z, theta)
    cx = torso_section(z)[0]
    dx, dy = x - cx, y
    norm = math.hypot(dx, dy) or 1.0
    return Vector((x + dx / norm * extra, y + dy / norm * extra, z))


# --- torso: trousers + waistcoat ---------------------------------------------


def vest_hem(theta: float) -> float:
    """z of the waistcoat hem: level at the sides and back, two points in front."""
    t = (theta + math.pi) % (2.0 * math.pi) - math.pi
    at = abs(t)
    points = (P.VEST_HEM_SIDE_Z - P.VEST_HEM_POINT_Z) * gauss(at, 0.3, 0.2)
    notch = 0.035 * gauss(at, 0.0, 0.09)  # inverted V between the points
    back = 0.012 * smoothstep((at - 1.6) / 1.0)  # slightly higher at the back
    return P.VEST_HEM_SIDE_Z - points + notch + back


SEGMENTS = 72


def build_torso() -> None:
    thetas = [2.0 * math.pi * s / SEGMENTS for s in range(SEGMENTS)]
    # Trousers / shirt core: hips up to just under the vest hem (hidden above).
    zs = [TORSO_BOTTOM + (1.16 - TORSO_BOTTOM) * i / 15 for i in range(16)]
    rows = [[tuple(torso_point(z, th)) for th in thetas] for z in zs]
    obj = grid_object("torso_trousers", rows, [shaded("trousers")], closed=True, cap_start=True, cap_end=True)
    subdivide(obj, 1)
    # Waistcoat shell: rows follow the hem curve, so its edge is smooth.
    n_rows = 30
    rows = []
    for r in range(n_rows):
        v = r / (n_rows - 1)
        row = []
        for th in thetas:
            z = vest_hem(th) + (TORSO_TOP - vest_hem(th)) * v
            row.append(tuple(torso_point(z, th, P.VEST_PROUD)))
        rows.append(row)
    obj = grid_object("torso_vest", rows, [shaded("vest")], closed=True, cap_end=True)
    subdivide(obj, 1)
    # Hem trim and placket: thin bands that give the cut a visible edge.
    hem = [torso_point(vest_hem(th) + 0.003, th, P.VEST_PROUD + 0.0012) for th in thetas]
    tube("vest_hem_trim", [*hem, hem[0]], [(0.0028, 0.0028)] * (len(hem) + 1), [shaded("vest_trim")],
         sides=8, ref=(0.0, 0.0, 1.0), caps=False)
    placket = [torso_point(z, 0.0, P.VEST_PROUD + 0.001) for z in (vest_hem(0.0) + 0.004, 1.2, 1.3, 1.41)]
    tube("vest_placket", catmull_rom([tuple(p) for p in placket], 12), [(0.0022, 0.006)] * 12,
         [shaded("vest_trim")], sides=8, ref=(1.0, 0.0, 0.0))
    for i, z in enumerate((1.175, 1.235, 1.295, 1.355)):
        p = torso_point(z, 0.0, P.VEST_PROUD + 0.003)
        tube(f"vest_button_{i}", [tuple(p - Vector((0.003, 0.0, 0.0))), tuple(p + Vector((0.004, 0.0, 0.0)))],
             [(0.0065, 0.0065), (0.0055, 0.0055)], [shaded("metal_dark")], sides=12, ref=(0.0, 0.0, 1.0))
    # Trouser fly seam, visible between the vest points.
    fly = [torso_point(z, 0.0, 0.001) for z in (0.93, 0.98, 1.03, 1.07)]
    tube("trouser_fly", catmull_rom([tuple(p) for p in fly], 8), [(0.0012, 0.003)] * 8,
         [shaded("vest_trim")], sides=6, ref=(1.0, 0.0, 0.0))


# --- arms ----------------------------------------------------------------------


def arm_curve(near: bool) -> list[Vector]:
    """Shoulder (inside the torso) -> elbow -> wrist (same curve as v003)."""
    if near:
        s, e, w = P.SHOULDER_NEAR, P.ELBOW_NEAR, P.WRIST_NEAR
    else:
        s, e, w = P.SHOULDER_FAR, P.ELBOW_FAR, P.WRIST_FAR
    side = -1.0 if near else 1.0
    inner = (s[0] - 0.005, s[1] - side * 0.05, s[2] + 0.005)
    cap = (s[0], s[1] + side * 0.03, s[2] - 0.02)
    return catmull_rom([inner, cap, e, w], 24)


ARM_BASE = ((0.0, 0.030), (0.08, 0.038), (0.16, 0.044), (0.32, 0.041), (0.52, 0.036), (0.62, 0.039),
            (0.75, 0.037), (0.92, 0.031), (1.0, 0.030))
ARM_REF = (1.0, 0.0, 0.0)  # angle 0 = front; pi/2 = -Y (outer side of the near arm)


def arm_radius(near: bool, t: float, ang: float) -> float:
    outer = math.pi / 2.0 if near else -math.pi / 2.0
    r = profile(ARM_BASE, t)
    r += 0.007 * gauss(t, 0.19, 0.08) * lobe(ang, outer, 0.9)  # deltoid, below the shoulder line
    r += 0.004 * gauss(t, 0.3, 0.1) * lobe(ang, 0.0, 0.8)  # biceps
    r += 0.004 * gauss(t, 0.54, 0.04) * lobe(ang, math.pi, 0.5)  # elbow point
    r += 0.004 * gauss(t, 0.66, 0.09) * lobe(ang, outer + math.copysign(0.6, outer), 0.9)  # forearm
    flat = 0.12 * gauss(t, 0.88, 0.1)  # the wrist is wider than deep
    return r * (1.0 - flat * abs(math.cos(ang)))


# --- legs ----------------------------------------------------------------------

LEG_BASE = ((0.0, 0.078), (0.18, 0.071), (0.4, 0.054), (0.48, 0.050), (0.62, 0.051),
            (0.75, 0.046), (0.9, 0.036), (1.0, 0.034))


def leg_curve(near: bool, samples: int = 30) -> list[Vector]:
    hip, knee, ankle = P.LEG_NEAR if near else P.LEG_FAR
    return catmull_rom([hip, knee, ankle], samples)


def leg_radius(near: bool, t: float, ang: float) -> float:
    """Radius at t (0 hip, 1 ankle) and angle (0 front, pi back, +/-pi/2 sides)."""
    inner = -math.pi / 2.0 if near else math.pi / 2.0  # toward the other leg
    r = profile(LEG_BASE, t)
    r += 0.008 * gauss(t, 0.2, 0.12) * lobe(ang, 0.25 * (1 if near else -1), 0.9)  # quadriceps
    r -= 0.008 * gauss(t, 0.12, 0.14) * lobe(ang, inner, 0.7)  # inner thigh, clears the crotch
    r += 0.006 * gauss(t, 0.47, 0.04) * lobe(ang, 0.0, 0.7)  # knee
    r += 0.016 * gauss(t, 0.62, 0.09) * lobe(ang, math.pi + 0.3 * (1 if near else -1), 0.85)  # calf
    r -= 0.004 * gauss(t, 0.92, 0.06) * lobe(ang, 0.0, 0.9)  # shin narrows into the ankle
    return r


def build_legs() -> None:
    for side, near in (("near", True), ("far", False)):
        path = leg_curve(near)
        obj = profiled_tube(f"leg_{side}", path, lambda t, a, n=near: leg_radius(n, t, a),
                            [shaded("trousers")], sides=24, ref=(1.0, 0.0, 0.0))
        subdivide(obj, 1)
