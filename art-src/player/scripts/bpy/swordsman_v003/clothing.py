"""Clothes of v003: coat, collar, sleeves, belt, half-cape, pauldron, boots.

Cloth is a parametric surface around the body, thickened by Solidify (the
inside and the cut edges get the lining) and smoothed by Subdivision:

- coat: follows ``anatomy.torso_radius`` down to the waist (fitted
  shoulders and waist), then flares into a skirt whose back tails are longer
  than the front and split by a vent. The front opening widens from the
  chest to the hem (inverted V), leaving two independent front panels whose
  edges carry a wine facing. Folds deepen toward the hem;
- half-cape: hangs from the far shoulder, covers the far arm and crosses
  the back diagonally, ending short on the near side, so it shows in the
  side view without hiding the sword. A brass clasp and a cord across the
  chest hold it;
- boots: shaft along the leg curve, cuff band, anatomical foot loft with
  instep, toe spring, sole and a low heel.
"""

from __future__ import annotations

import math

from mathutils import Vector

import params as P
from anatomy import ARM_RADII, arm_curve, leg_curve, leg_radius, torso_radius, torso_section
from mesh_tools import (
    Vec3,
    angle_delta,
    catmull_rom,
    gauss,
    grid_object,
    profile,
    smoothstep,
    subdivide,
    thicken,
    tube,
)
from surfaces import shaded

COAT_TOP_Z = 1.50
COAT_OFFSET = 0.017  # air between vest and coat
HEM_SHAPE = (-0.035, 0.205, 0.28, 0.255)  # centre x, front depth, back depth, half width
SEGMENTS = 96


def hem_z(theta: float) -> float:
    return P.HEM_BACK_Z + (P.HEM_FRONT_Z - P.HEM_BACK_Z) * (1.0 + math.cos(theta)) / 2.0


def _fitted(z: float, theta: float, extra: float) -> tuple[float, float]:
    x, y = torso_radius(z, theta)
    cx = torso_section(z)[0]
    dx, dy = x - cx, y
    norm = math.hypot(dx, dy) or 1.0
    back = 0.006 * max(0.0, -math.cos(theta))
    return (x + dx / norm * (extra + back), y + dy / norm * (extra + back))


def coat_point(z: float, theta: float, extra: float = 0.0) -> Vec3:
    """Coat outer surface at (z, theta); ``extra`` pushes it outward."""
    if z >= P.WAIST_Z:
        x, y = _fitted(z, theta, COAT_OFFSET + extra)
        return (x, y, z)
    t = min((P.WAIST_Z - z) / (P.WAIST_Z - P.HEM_BACK_Z), 1.0)
    wx, wy = _fitted(P.WAIST_Z, theta, COAT_OFFSET)
    cx, front, back, half_w = HEM_SHAPE
    c, s = math.cos(theta), math.sin(theta)
    hx, hy = cx + (front if c >= 0.0 else back) * c, half_w * s
    ease = t ** 0.62  # flares early enough to clear the thighs
    x, y = wx + (hx - wx) * ease, wy + (hy - wy) * ease
    fold = 0.017 * t ** 1.2 * math.sin(P.FOLD_COUNT * theta + 0.6)
    norm = math.hypot(x - cx * ease, y) or 1.0
    push = fold + extra
    return (x + (x - cx * ease) / norm * push, y + y / norm * push, z)


def opening(z: float) -> float:
    """Half-angle (rad) of the front opening at height z."""
    return profile(((P.HEM_BACK_Z, 0.78), (0.9, 0.46), (P.WAIST_Z, 0.24), (1.27, 0.3),
                    (1.40, 0.38), (COAT_TOP_Z, 0.34)), z)


def build_coat() -> None:
    n_rows = 46
    thetas = [2.0 * math.pi * s / SEGMENTS for s in range(SEGMENTS)]
    rows = []
    for r in range(n_rows):
        v = r / (n_rows - 1)
        # Denser rows near the waist, where the shape changes fastest.
        rows.append([coat_point(hem_z(th) + (COAT_TOP_Z - hem_z(th)) * v, th) for th in thetas])

    def centre(r: int, c: int) -> tuple[float, float]:
        th = 2.0 * math.pi * (c + 0.5) / SEGMENTS
        v = (r + 0.5) / (n_rows - 1)
        return hem_z(th) + (COAT_TOP_Z - hem_z(th)) * v, th

    def keep(r: int, c: int) -> bool:
        z, th = centre(r, c)
        if abs(angle_delta(th, 0.0)) < opening(z):
            return False
        vent = 0.02 + 0.09 * smoothstep((0.93 - z) / 0.5)
        return not (z < 0.93 and abs(angle_delta(th, math.pi)) < vent)

    def material(r: int, c: int) -> int:
        z, th = centre(r, c)
        facing = 0.15 if z > P.WAIST_Z - 0.02 else 0.06
        return 1 if abs(angle_delta(th, 0.0)) < opening(z) + facing else 0

    lining = shaded("lining_satin")
    obj = grid_object("coat", rows, [shaded("coat_wool"), lining, lining], closed=True,
                      face_filter=keep, face_material=material)
    thicken(obj, P.CLOTH_THICKNESS)
    subdivide(obj, 1)


def build_collar() -> None:
    """High stiff collar, open in front, flaring outward, wine inside."""
    bottom, top_back, top_front = 1.452, 1.648, 1.578
    t0, t1 = 0.78, 2.0 * math.pi - 0.78
    cols = 40
    rows = []
    for r in range(14):
        v = r / 13
        row = []
        for c in range(cols + 1):
            th = t0 + (t1 - t0) * c / cols
            top = top_front + (top_back - top_front) * (1.0 - math.cos(th)) / 2.0
            z = bottom + (top - bottom) * v
            depth = profile(((0.0, 0.068), (0.55, 0.08), (1.0, 0.097)), v)
            width = profile(((0.0, 0.088), (0.55, 0.089), (1.0, 0.102)), v)
            x = -0.012 + depth * math.cos(th) * (1.06 if math.cos(th) < 0 else 1.0)
            row.append((x, width * math.sin(th), z))
        rows.append(row)
    obj = grid_object("collar", rows, [shaded("vest"), shaded("lining_satin")])
    thicken(obj, 0.015)
    subdivide(obj, 2)


def build_sleeves() -> None:
    for side, near in (("near", True), ("far", False)):
        path = arm_curve(near)
        n = len(path)
        radii = [(profile(ARM_RADII, i / (n - 1)) + 0.004,) * 2 for i in range(n)]
        obj = tube(f"sleeve_{side}", path, radii, [shaded("coat_wool")], sides=16, ref=(1.0, 0.0, 0.0))
        subdivide(obj, 1)
        # Turned-back cuff and a thin dark-silver band at the wrist.
        cuff = path[-5:]
        obj = tube(f"cuff_{side}", cuff, [(0.047, 0.047), (0.048, 0.048), (0.049, 0.049),
                                          (0.050, 0.050), (0.051, 0.051)],
                   [shaded("vest")], sides=16, ref=(1.0, 0.0, 0.0))
        subdivide(obj, 1)
        band = path[-6:-4]
        tube(f"cuff_band_{side}", band, [(0.0475, 0.0475), (0.0475, 0.0475)], [shaded("metal_dark")],
             sides=16, ref=(1.0, 0.0, 0.0))


def build_belt() -> None:
    rows = []
    for z in (1.105, 1.13, 1.152):
        rows.append([(*_fitted(z, 2.0 * math.pi * s / 48, 0.006), z) for s in range(48)])
    obj = grid_object("belt", rows, [shaded("boot_leather"), shaded("boot_leather")], closed=True)
    thicken(obj, 0.006)
    x, _ = _fitted(1.128, 0.0, 0.012)

    def square(ang: float) -> tuple[float, float]:
        return (math.cos(ang + math.pi / 4) * math.sqrt(2.0), math.sin(ang + math.pi / 4) * math.sqrt(2.0))

    tube("belt_buckle", [(x - 0.004, 0.0, 1.128), (x + 0.008, 0.0, 1.128)], [(0.024, 0.03), (0.022, 0.028)],
         [shaded("brass")], sides=4, ref=(0.0, 0.0, 1.0), section=square)


def build_half_cape() -> None:
    t0, t1 = 1.22, 2.0 * math.pi - 1.95
    cols, n_rows = 64, 30

    def top(u: float) -> float:
        return profile(((0.0, 1.47), (0.4, 1.485), (1.0, 1.40)), u)

    def hem(u: float) -> float:
        return profile(((0.0, 1.0), (0.4, 1.05), (1.0, 1.22)), u) + 0.012 * math.sin(5.0 * math.pi * u)

    rows = []
    for r in range(n_rows):
        v = r / (n_rows - 1)  # 0 = hem, 1 = top edge
        row = []
        for c in range(cols + 1):
            u = c / cols
            th = t0 + (t1 - t0) * u
            z = hem(u) + (top(u) - hem(u)) * v
            drop = 1.0 - v
            over_arm = 0.095 * gauss(angle_delta(th, 1.6), 0.0, 0.55)
            fold = 0.011 * drop * math.sin(11.0 * th + 1.1)
            row.append(coat_point(z, th, 0.02 + 0.055 * drop ** 1.2 + over_arm + fold))
        rows.append(row)
    obj = grid_object("half_cape", rows, [shaded("cape_wool"), shaded("cape_inner")])
    thicken(obj, P.CAPE_THICKNESS)
    subdivide(obj, 1)
    # Clasp on the far shoulder and a cord across the chest to the near side.
    clasp = Vector(coat_point(1.445, 0.95, 0.03))
    tube("cape_clasp", [tuple(clasp - Vector((0.008, 0.0, 0.0))), tuple(clasp + Vector((0.008, 0.0, 0.0)))],
         [(0.019, 0.019), (0.016, 0.016)], [shaded("brass")], sides=16, ref=(0.0, 0.0, 1.0))
    stud = Vector(coat_point(1.44, -0.72, 0.02))
    mid = Vector((torso_radius(1.395, 0.0)[0] + 0.035, -0.01, 1.392))
    cord = catmull_rom([tuple(clasp), tuple(Vector(coat_point(1.415, 0.45, 0.022))), tuple(mid),
                        tuple(Vector(coat_point(1.415, -0.42, 0.022))), tuple(stud)], 16)
    tube("cape_cord", cord, [(0.0042, 0.0042)] * len(cord), [shaded("metal_dark")], sides=8)
    tube("cape_stud", [tuple(stud - Vector((0.005, 0.0, 0.0))), tuple(stud + Vector((0.006, 0.0, 0.0)))],
         [(0.010, 0.010), (0.008, 0.008)], [shaded("brass")], sides=12, ref=(0.0, 0.0, 1.0))


def build_pauldron() -> None:
    """Small dark-silver plate on the sword shoulder only."""
    centre = Vector((0.0, -0.198, 1.425))
    axis = Vector((0.15, -0.55, 0.82)).normalized()
    e1 = axis.orthogonal().normalized()
    e2 = axis.cross(e1)
    radius = 0.066
    rows = []
    for r in range(10):
        polar = 1.0 * (1.0 - r / 9)
        row = []
        for s in range(32):
            a = 2.0 * math.pi * s / 32
            d = axis * math.cos(polar) + (e1 * math.cos(a) + e2 * math.sin(a)) * math.sin(polar)
            row.append(tuple(centre + d * radius))
        rows.append(row)
    obj = grid_object("pauldron", rows, [shaded("metal_dark"), shaded("metal_dark")], closed=True, cap_end=True)
    thicken(obj, 0.005)
    subdivide(obj, 1)


def build_boots() -> None:
    for side, near in (("near", True), ("far", False)):
        path = leg_curve(near)
        n = len(path)
        start = int(n * 0.43)
        shaft = path[start:]
        m = len(shaft)
        radii = []
        for i in range(m):
            t = (start + i) / (n - 1)
            flare = 0.008 * (1.0 - smoothstep(i / 3.0))
            r = leg_radius(t) + 0.009 + flare
            radii.append((r * 1.04, r))
        obj = tube(f"boot_shaft_{side}", shaft, radii, [shaded("boot_leather")], sides=16, ref=(1.0, 0.0, 0.0))
        subdivide(obj, 1)
        top_t = start / (n - 1)
        cuff = shaft[:3]
        r0 = leg_radius(top_t) + 0.016
        obj = tube(f"boot_cuff_{side}", cuff, [(r0, r0 * 0.97), (r0 - 0.002, r0 * 0.95), (r0 - 0.004, r0 * 0.93)],
                   [shaded("boot_cuff")], sides=16, ref=(1.0, 0.0, 0.0))
        subdivide(obj, 1)
        build_foot(side, Vector(path[-1]))


# Foot loft along +X from the heel: (x from ankle, centre z, half height, half width).
FOOT_SECTIONS = ((-0.058, 0.052, 0.045, 0.036), (-0.03, 0.058, 0.052, 0.041), (0.02, 0.054, 0.046, 0.043),
                 (0.08, 0.040, 0.034, 0.046), (0.13, 0.032, 0.026, 0.044), (0.168, 0.028, 0.019, 0.036),
                 (0.192, 0.026, 0.011, 0.022))
SOLE_Z = 0.014
HEEL_Z = 0.026


def build_foot(side: str, ankle: Vector) -> None:
    path = [(ankle.x + dx, ankle.y, cz) for dx, cz, _, _ in FOOT_SECTIONS]
    last = len(FOOT_SECTIONS) - 1
    heights = [(k / last, sec[2]) for k, sec in enumerate(FOOT_SECTIONS)]
    widths = [(k / last, sec[3]) for k, sec in enumerate(FOOT_SECTIONS)]
    samples = 16
    radii = [(profile(heights, i / (samples - 1)), profile(widths, i / (samples - 1))) for i in range(samples)]
    obj = tube(f"boot_foot_{side}", catmull_rom(path, samples), radii,
               [shaded("boot_leather")], sides=14, ref=(0.0, 0.0, 1.0))
    subdivide(obj, 1)
    # Sole outline from the foot widths, extruded; low heel block at the back.
    near_side = [(ankle.x + dx, ankle.y - (hw + 0.004)) for dx, _, _, hw in FOOT_SECTIONS]
    far_side = [(ankle.x + dx, ankle.y + (hw + 0.004)) for dx, _, _, hw in reversed(FOOT_SECTIONS)]
    outline = [*near_side, *far_side]
    rows = [[(x, y, z) for x, y in outline] for z in (0.0, SOLE_Z)]
    grid_object(f"boot_sole_{side}", rows, [shaded("sole")], closed=True, cap_start=True, cap_end=True)
    x0, x1 = ankle.x - 0.062, ankle.x - 0.005
    hw = 0.034
    heel = [(x0, ankle.y - hw), (x1, ankle.y - hw), (x1, ankle.y + hw), (x0, ankle.y + hw)]
    rows = [[(x, y, z) for x, y in heel] for z in (0.0, HEEL_Z)]
    grid_object(f"boot_heel_{side}", rows, [shaded("sole")], closed=True, cap_start=True, cap_end=True)
