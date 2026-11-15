"""Clothes of v004: coat, sleeves, belt, half-cape, pauldron and boots.

Refines ``swordsman_v003/clothing.py`` (untouched; its collar is reused as
is). The main changes:

- **coat**: v003 cut the front opening and the back vent by dropping grid
  faces, which left stair-stepped edges and a jagged wine facing. v004
  builds each half of the coat with columns that start exactly at the
  opening edge, so the edges and the facing are smooth curves; the back vent
  is a real slit (two panels whose seam opens below the hips). Folds are a
  sum of three fold families whose phase drifts with height (they fan out
  instead of running in parallel), with uneven amplitude; side-back seams,
  a waist seam where the skirt joins the body, a rolled front edge and a
  gently waving hem;
- **sleeves**: anatomical radius from ``body.arm_radius``, a few creases
  at the inner elbow, a turned-back cuff and a band computed from the same
  radius (v003's band and cuff had equal radii and z-fought into "teeth");
- **belt**: strap with a real buckle frame and prong, keeper loop and a
  tongue end with holes, instead of a solid brass block;
- **half-cape**: folds radiating from the clasp, deeper at the hem, an
  irregular hem that flips slightly outward, more weight over the arm;
- **pauldron**: lower and smaller dark plate with a second lame, so the
  shoulder reads as an accent and not as armour;
- **boots**: shaft over the leg radius (calf visible), top cut higher in
  front than behind, rolled cuff, side seam, creases over the ankle;
  foot with toe cap seam, welt, sole and a tapered heel.
"""

from __future__ import annotations

import math

import bmesh
import bpy
from mathutils import Vector

import params as P
from body import LEG_BASE as LEG_BASE_FOR_CLOTH
from body import arm_curve, arm_radius, leg_curve, leg_radius, torso_point, torso_radius
from mesh_tools import (
    Vec3,
    catmull_rom,
    gauss,
    grid_object,
    lerp,
    profile,
    smoothstep,
    subdivide,
    thicken,
    tube,
)
from shapes import frames, lobe, profiled_tube
from surfaces import shaded

COAT_TOP_Z = 1.50
COAT_OFFSET = 0.017  # air between vest and coat (as v003)
HEM_SHAPE = (-0.035, 0.205, 0.28, 0.255)  # centre x, front depth, back depth, half width
VENT_TOP_Z = 0.93


def merge_close(obj: bpy.types.Object, dist: float = 1e-5) -> None:
    bm = bmesh.new()
    bm.from_mesh(obj.data)
    bmesh.ops.remove_doubles(bm, verts=bm.verts, dist=dist)
    bm.to_mesh(obj.data)
    bm.free()
    obj.data.update()


# --- coat ----------------------------------------------------------------------


def hem_z(theta: float) -> float:
    base = P.HEM_BACK_Z + (P.HEM_FRONT_Z - P.HEM_BACK_Z) * (1.0 + math.cos(theta)) / 2.0
    return base + 0.006 * math.sin(7.0 * theta + 0.4) + 0.004 * math.sin(12.0 * theta + 2.2)


def _fitted(z: float, theta: float, extra: float) -> tuple[float, float]:
    p = torso_point(z, theta, extra + 0.006 * max(0.0, -math.cos(theta)))
    return (p.x, p.y)


def coat_point(z: float, theta: float, extra: float = 0.0) -> Vec3:
    """Coat outer surface at (z, theta); ``extra`` pushes it outward."""
    seams = -0.0022 * (lobe(theta, 1.85, 0.05) + lobe(theta, -1.85, 0.05))
    if z >= P.WAIST_Z:
        seams -= 0.002 * lobe(theta, math.pi, 0.04)  # centre back seam of the body
        ridge = 0.0025 * gauss(z, P.WAIST_Z + 0.006, 0.01)  # waist seam
        x, y = _fitted(z, theta, COAT_OFFSET + extra + seams + ridge)
        return (x, y, z)
    t = min((P.WAIST_Z - z) / (P.WAIST_Z - P.HEM_BACK_Z), 1.0)
    wx, wy = _fitted(P.WAIST_Z, theta, COAT_OFFSET)
    cx, front, back, half_w = HEM_SHAPE
    c, s = math.cos(theta), math.sin(theta)
    hx, hy = cx + (front if c >= 0.0 else back) * c, half_w * s
    ease = t ** 0.62  # flares early enough to clear the thighs (as v003)
    x, y = wx + (hx - wx) * ease, wy + (hy - wy) * ease
    fold = 0.0
    uneven = 1.0 + 0.35 * math.sin(theta + 1.3)
    for count, amp, phase, drift in P.FOLDS:
        fold += amp * math.sin(count * theta + phase + drift * t)
    fold *= t ** 1.15 * uneven
    skirt = 0.002 + 0.002 * smoothstep(t / 0.05)  # the skirt sits over the body seam
    norm = math.hypot(x - cx * ease, y) or 1.0
    push = fold + extra + seams + skirt
    x, y = x + (x - cx * ease) / norm * push, y + y / norm * push
    return (*clear_legs(x, y, z), z)


LEG_CLEARANCE = P.CLOTH_THICKNESS + 0.005  # solidify grows inward, plus air
LEG_MAX_BULGE = 0.016  # largest outward term of body.leg_radius
_LEG_AXES = {near: leg_curve(near, 40) for near in (True, False)}


def clear_legs(x: float, y: float, z: float) -> tuple[float, float]:
    """Pushes a skirt point out of both thighs, horizontally (cloth resting on them)."""
    p = Vector((x, y, z))
    for near, axis in _LEG_AXES.items():
        best, best_t, best_q = 1e9, 0.0, axis[0]
        for i in range(len(axis) - 1):
            a, b = axis[i], axis[i + 1]
            ab = b - a
            f = max(0.0, min(1.0, (p - a).dot(ab) / ab.length_squared))
            q = a + ab * f
            d = (p - q).length
            if d < best:
                best, best_t, best_q = d, (i + f) / (len(axis) - 1), q
        need = profile(LEG_BASE_FOR_CLOTH, best_t) + LEG_MAX_BULGE + LEG_CLEARANCE
        flat = Vector((p.x - best_q.x, p.y - best_q.y, 0.0))
        if flat.length < need and flat.length > 1e-6:
            p = Vector((best_q.x, best_q.y, p.z)) + flat.normalized() * need
    return p.x, p.y


def opening(z: float) -> float:
    """Half-angle (rad) of the front opening at height z (as v003)."""
    return profile(((P.HEM_BACK_Z, 0.78), (0.9, 0.46), (P.WAIST_Z, 0.24), (1.27, 0.3),
                    (1.40, 0.38), (COAT_TOP_Z, 0.34)), z)


def vent(z: float) -> float:
    """Half-angle of the back vent opening (0 above VENT_TOP_Z)."""
    return 0.05 * smoothstep((VENT_TOP_Z - z) / 0.45)


def _coat_theta(near_half: bool, f: float, v: float) -> tuple[float, float]:
    """Angle and height of a coat vertex.

    ``f`` runs from the front edge (0) to the back seam (1) of one half;
    ``v`` from the hem (0) to the top (1). The near half lives at negative
    angles, the far half at positive ones; they meet at pi, where the vent
    keeps them apart below VENT_TOP_Z.
    """
    z = lerp(P.HEM_FRONT_Z, COAT_TOP_Z, v)
    theta = 0.0
    for _ in range(4):  # the edge depends on z and z on the hem under theta
        op = opening(z)
        theta = op + (math.pi - vent(z) - op) * f
        if near_half:
            theta = -theta
        z = hem_z(theta) + (COAT_TOP_Z - hem_z(theta)) * v
    return theta, z


def build_coat() -> None:
    half = 56
    cols = 2 * half + 1  # the two middle columns coincide above the vent
    n_rows = 48
    rows = []
    for r in range(n_rows):
        v = r / (n_rows - 1)
        row = []
        for c in range(cols + 1):
            if c <= half:
                theta, z = _coat_theta(True, c / half, v)
            else:
                theta, z = _coat_theta(False, (cols - c) / half, v)
            # Rolled front edge above the waist: the lapel stands off a little.
            edge = min(c, cols - c)
            roll = 0.0035 * gauss(edge, 0.0, 2.5) * smoothstep((z - 1.0) / 0.2)
            row.append(coat_point(z, theta, roll))
        rows.append(row)

    def keep(r: int, c: int) -> bool:
        if c != half:
            return True
        z = lerp(P.HEM_BACK_Z, COAT_TOP_Z, (r + 0.5) / (n_rows - 1))
        return z >= VENT_TOP_Z

    facing = P.COAT_EDGE_FACING_COLUMNS

    def material(r: int, c: int) -> int:
        return 1 if c < facing or c >= cols - facing else 0

    lining = shaded("lining_satin")
    # Columns run near-front -> back -> far-front; reverse them so the faces
    # point outward like the v003 coat.
    rows = [list(reversed(row)) for row in rows]
    obj = grid_object("coat", rows, [shaded("coat_wool"), lining, lining],
                      face_filter=keep, face_material=material)  # both are symmetric in c
    merge_close(obj)
    thicken(obj, P.CLOTH_THICKNESS)
    subdivide(obj, 1)


# --- sleeves -----------------------------------------------------------------

SLEEVE_EASE = 0.005  # cloth over the arm
CUFF_START = 0.8  # t where the turned-back cuff begins


def sleeve_radius(near: bool, t: float, ang: float) -> float:
    r = arm_radius(near, t, ang) + SLEEVE_EASE
    # Inner-elbow creases: three short folds on the front of the joint.
    crease = sum(gauss(t, c, 0.012) for c in (0.47, 0.51, 0.555))
    r -= 0.0028 * crease * lobe(ang, 0.0, 0.7)
    r += 0.0015 * math.sin(9.0 * ang + 5.0 * t) * gauss(t, 0.5, 0.08)  # cloth bunching
    return r


def build_sleeves() -> None:
    for side, near in (("near", True), ("far", False)):
        path = arm_curve(near)
        obj = profiled_tube(f"sleeve_{side}", path, lambda t, a, n=near: sleeve_radius(n, t, a),
                            [shaded("coat_wool")], sides=24, ref=(1.0, 0.0, 0.0))
        subdivide(obj, 1)
        n = len(path)
        first = int(CUFF_START * (n - 1))
        cuff = path[first:]
        m = len(cuff)

        def cuff_radius(t: float, a: float, n_=near, first_=first, m_=m) -> float:
            tt = (first_ + t * (m_ - 1)) / (n - 1)
            flare = 0.004 * smoothstep(t)
            return arm_radius(n_, tt, a) + SLEEVE_EASE + 0.006 + flare

        obj = profiled_tube(f"cuff_{side}", cuff, cuff_radius, [shaded("vest")], sides=24, ref=(1.0, 0.0, 0.0))
        subdivide(obj, 1)
        band = cuff[:3]
        obj = profiled_tube(f"cuff_band_{side}", band,
                            lambda t, a, n_=near, f_=first: arm_radius(n_, (f_ + t * 2) / (n - 1), a)
                            + SLEEVE_EASE + 0.0085,
                            [shaded("metal_dark")], sides=24, ref=(1.0, 0.0, 0.0))
        subdivide(obj, 1)


# --- belt --------------------------------------------------------------------

BELT_Z = 1.128
BELT_HALF_HEIGHT = 0.021


def build_belt() -> None:
    thetas = [2.0 * math.pi * s / 64 for s in range(64)]
    off = P.VEST_PROUD + 0.004
    rows = [[tuple(torso_point(z, th, off)) for th in thetas]
            for z in (BELT_Z - BELT_HALF_HEIGHT, BELT_Z, BELT_Z + BELT_HALF_HEIGHT)]
    obj = grid_object("belt", rows, [shaded("belt_leather"), shaded("belt_leather")], closed=True)
    thicken(obj, 0.005)
    subdivide(obj, 1)
    front = torso_point(BELT_Z, 0.0, off + 0.002)
    out = Vector((1.0, 0.0, 0.0))

    def square(ang: float) -> tuple[float, float]:
        return (math.cos(ang + math.pi / 4) * math.sqrt(2.0), math.sin(ang + math.pi / 4) * math.sqrt(2.0))

    # Buckle frame: a rectangular ring (four bars) and the prong across it.
    hw, hh, bar = 0.022, 0.027, 0.0042
    corners = [front + Vector((0.004, y, z)) for y, z in ((-hw, -hh), (hw, -hh), (hw, hh), (-hw, hh))]
    for i in range(4):
        a, b = corners[i], corners[(i + 1) % 4]
        tube(f"belt_buckle_bar_{i}", [a, b], [(bar, bar), (bar, bar)], [shaded("brass")], sides=8,
             ref=tuple(out), section=square)
    tube("belt_prong", [front + Vector((0.006, -hw, 0.0)), front + Vector((0.006, -hw * 0.15, 0.0))],
         [(0.0022, 0.0022), (0.0016, 0.0016)], [shaded("brass")], sides=8, ref=(0.0, 0.0, 1.0))
    # Tongue: the strap end passes the buckle and tucks into a keeper loop.
    tongue = [torso_point(BELT_Z, th, off + 0.0055) for th in (-0.02, -0.18, -0.34, -0.46)]
    tube("belt_tongue", tongue, [(0.0028, BELT_HALF_HEIGHT * 0.86)] * 4, [shaded("belt_leather")], sides=8,
         ref=(1.0, 0.0, 0.0))
    for i, th in enumerate((-0.24, -0.32)):
        p = torso_point(BELT_Z, th, off + 0.0085)
        tube(f"belt_hole_{i}", [p - out * 0.001, p + out * 0.0005], [(0.0022, 0.0022)] * 2,
             [shaded("sole")], sides=8, ref=(0.0, 0.0, 1.0))
    keeper = [torso_point(BELT_Z, th, off + 0.009) for th in (-0.5, -0.5)]
    keeper[1] = keeper[1] + Vector((0.0, -0.008, 0.0))
    tube("belt_keeper", keeper, [(0.0035, BELT_HALF_HEIGHT * 1.1)] * 2, [shaded("belt_leather")], sides=8,
         ref=(1.0, 0.0, 0.0))


# --- half-cape ---------------------------------------------------------------

CLASP_Z, CLASP_THETA = 1.445, 0.95


def build_half_cape() -> None:
    t0, t1 = 1.22, 2.0 * math.pi - 1.95
    cols, n_rows = 72, 32

    def top(u: float) -> float:
        return profile(((0.0, 1.47), (0.4, 1.487), (1.0, 1.40)), u)

    def hem(u: float) -> float:
        return (profile(((0.0, 0.99), (0.35, 1.03), (1.0, 1.22)), u)
                + 0.013 * math.sin(5.0 * math.pi * u + 0.3) + 0.006 * math.sin(13.0 * math.pi * u + 1.0))

    rows = []
    for r in range(n_rows):
        v = r / (n_rows - 1)  # 0 = hem, 1 = top edge
        row = []
        for c in range(cols + 1):
            u = c / cols
            th = t0 + (t1 - t0) * u
            z = hem(u) + (top(u) - hem(u)) * v
            drop = 1.0 - v
            over_arm = 0.105 * gauss(angle_delta(th, 1.6), 0.0, 0.55)
            # Folds fan out from the clasp: phase depends on the angle from it.
            spread = angle_delta(th, CLASP_THETA) * (0.8 + 0.5 * drop)
            fold = (0.009 * math.sin(6.0 * spread + 0.8) + 0.0035 * math.sin(11.0 * spread + 2.0)) * drop ** 1.3
            fold += 0.007 * math.sin(9.0 * th + 0.5 + 0.8 * drop) * drop ** 1.6  # weight pulling down the back
            flip = 0.01 * (1.0 - smoothstep(v / 0.12))  # hem turns slightly outward
            wrap = -0.012 * (gauss(u, 0.0, 0.08) + gauss(u, 1.0, 0.08)) * drop  # side edges curl in
            row.append(coat_point(z, th, 0.02 + 0.05 * drop ** 1.25 + over_arm + fold + flip + wrap))
        rows.append(row)
    obj = grid_object("half_cape", rows, [shaded("cape_wool"), shaded("cape_inner")])
    thicken(obj, P.CAPE_THICKNESS)
    subdivide(obj, 1)
    clasp = Vector(coat_point(CLASP_Z, CLASP_THETA, 0.03))
    tube("cape_clasp", [tuple(clasp - Vector((0.008, 0.0, 0.0))), tuple(clasp + Vector((0.008, 0.0, 0.0)))],
         [(0.019, 0.019), (0.016, 0.016)], [shaded("brass")], sides=20, ref=(0.0, 0.0, 1.0))
    stud = Vector(coat_point(1.44, -0.72, 0.02))
    mid = Vector((torso_radius(1.395, 0.0)[0] + 0.035, -0.01, 1.392))
    cord = catmull_rom([tuple(clasp), tuple(Vector(coat_point(1.415, 0.45, 0.022))), tuple(mid),
                        tuple(Vector(coat_point(1.415, -0.42, 0.022))), tuple(stud)], 16)
    tube("cape_cord", cord, [(0.0038, 0.0038)] * len(cord), [shaded("metal_dark")], sides=8)
    tube("cape_stud", [tuple(stud - Vector((0.005, 0.0, 0.0))), tuple(stud + Vector((0.006, 0.0, 0.0)))],
         [(0.010, 0.010), (0.008, 0.008)], [shaded("brass")], sides=12, ref=(0.0, 0.0, 1.0))


def angle_delta(a: float, b: float) -> float:
    return (a - b + math.pi) % (2.0 * math.pi) - math.pi


# --- pauldron ------------------------------------------------------------------


def _plate(name: str, centre: Vector, axis: Vector, radius: float, polar_max: float) -> None:
    e1 = axis.orthogonal().normalized()
    e2 = axis.cross(e1)
    rows = []
    for r in range(10):
        polar = polar_max * (1.0 - r / 9)
        row = []
        for s in range(36):
            a = 2.0 * math.pi * s / 36
            d = axis * math.cos(polar) + (e1 * math.cos(a) + e2 * math.sin(a)) * math.sin(polar)
            row.append(tuple(centre + d * radius))
        rows.append(row)
    obj = grid_object(name, rows, [shaded("metal_dark"), shaded("metal_dark")], closed=True, cap_end=True)
    thicken(obj, 0.004)
    subdivide(obj, 1)


def build_pauldron() -> None:
    """Small dark plate on the sword shoulder, with one lame under it."""
    axis = Vector((0.12, -0.6, 0.79)).normalized()
    _plate("pauldron", Vector((0.0, -0.188, 1.418)), axis, 0.064, 0.82)
    lame_axis = Vector((0.1, -0.85, 0.52)).normalized()
    _plate("pauldron_lame", Vector((0.002, -0.19, 1.392)), lame_axis, 0.058, 0.62)


# --- boots ----------------------------------------------------------------------

BOOT_EASE = 0.009
BOOT_TOP_T = 0.43  # t on the leg curve of the top edge at the sides
BOOT_TOP_FRONT = 0.035  # the front of the shaft rises this much in t


def boot_radius(near: bool, t: float, ang: float) -> float:
    r = leg_radius(near, t, ang) + BOOT_EASE
    creases = sum(gauss(t, c, 0.012) for c in (0.84, 0.875, 0.91))
    r += 0.0035 * creases * lobe(ang, 0.0, 0.9)  # ankle folds on the front of the shaft
    return r


def build_boot_shaft(side: str, near: bool) -> None:
    path = leg_curve(near, 60)
    fr = frames(path, (1.0, 0.0, 0.0))
    n = len(path)

    def at(t: float) -> tuple[Vector, Vector, Vector]:
        x = t * (n - 1)
        i = min(int(x), n - 2)
        f = x - i
        _, n0, b0 = fr[i]
        _, n1, b1 = fr[i + 1]
        return path[i].lerp(path[i + 1], f), n0.lerp(n1, f).normalized(), b0.lerp(b1, f).normalized()

    sides, n_rows = 28, 30
    cuff_rows = 5
    rows = []
    for r in range(n_rows):
        s = r / (n_rows - 1)
        row = []
        for k in range(sides):
            ang = 2.0 * math.pi * k / sides
            t_top = BOOT_TOP_T - BOOT_TOP_FRONT * (0.5 + 0.5 * math.cos(ang))
            t = t_top + (1.0 - t_top) * s
            p, nrm, bin_ = at(t)
            rad = boot_radius(near, t, ang)
            if r < cuff_rows:  # rolled top: the cuff band stands off the shaft
                rad += 0.004 + 0.0025 * (1.0 - r / cuff_rows)
            row.append(p + nrm * (rad * math.cos(ang)) + bin_ * (rad * math.sin(ang)))
        rows.append(row)
    obj = grid_object(f"boot_shaft_{side}", rows, [shaded("boot_leather"), shaded("boot_cuff")], closed=True,
                      cap_end=True, face_material=lambda r, c: 1 if r < cuff_rows - 1 else 0)
    subdivide(obj, 1)
    # Side seam on the outer side of the shaft.
    outer = math.pi / 2.0 if near else -math.pi / 2.0
    seam = []
    for i in range(14):
        s = i / 13
        t = BOOT_TOP_T + 0.01 + (0.9 - BOOT_TOP_T) * s
        p, nrm, bin_ = at(t)
        rad = boot_radius(near, t, outer) + 0.0008
        seam.append(p + nrm * (rad * math.cos(outer)) + bin_ * (rad * math.sin(outer)))
    tube(f"boot_seam_{side}", seam, [(0.0013, 0.0013)] * len(seam), [shaded("stitch")], sides=6)


# Foot loft along +X from the heel: (x from ankle, centre z, half height, half width).
FOOT_SECTIONS = ((-0.06, 0.054, 0.046, 0.036), (-0.03, 0.06, 0.053, 0.041), (0.02, 0.056, 0.046, 0.043),
                 (0.08, 0.042, 0.033, 0.046), (0.13, 0.033, 0.024, 0.045), (0.168, 0.029, 0.017, 0.039),
                 (0.19, 0.027, 0.011, 0.028), (0.198, 0.026, 0.006, 0.016))
SOLE_Z = 0.013
WELT_Z = 0.017
HEEL_Z = 0.03


def build_foot(side: str, ankle: Vector) -> None:
    path = catmull_rom([(ankle.x + dx, ankle.y, cz) for dx, cz, _, _ in FOOT_SECTIONS], 22)
    last = len(FOOT_SECTIONS) - 1
    heights = tuple((k / last, sec[2]) for k, sec in enumerate(FOOT_SECTIONS))
    widths = tuple((k / last, sec[3]) for k, sec in enumerate(FOOT_SECTIONS))

    def radius(t: float, ang: float) -> float:
        h, w = profile(heights, t), profile(widths, t)
        c, s = math.cos(ang), math.sin(ang)
        r = 1.0 / math.sqrt((c / h) ** 2 + (s / w) ** 2)
        if c < 0.0:  # flat underside
            r *= 1.0 - 0.18 * c * c
        return r

    obj = profiled_tube(f"boot_foot_{side}", path, radius, [shaded("boot_leather")], sides=24, ref=(0.0, 0.0, 1.0))
    subdivide(obj, 1)
    # Toe cap seam across the top of the foot.
    fr = frames(path, (0.0, 0.0, 1.0))
    i = int(0.62 * (len(path) - 1))
    seam = []
    for k in range(11):
        ang = -1.2 + 2.4 * k / 10
        rr = radius(i / (len(path) - 1), ang) + 0.0006
        _, nrm, bin_ = fr[i]
        seam.append(path[i] + nrm * (rr * math.cos(ang)) + bin_ * (rr * math.sin(ang))
                    - Vector((0.012 * (1.0 - math.cos(ang)), 0.0, 0.0)))
    tube(f"boot_toe_seam_{side}", catmull_rom([tuple(p) for p in seam], 14), [(0.0011, 0.0011)] * 14,
         [shaded("stitch")], sides=6)
    # Sole and welt from a smooth outline of the foot widths; tapered heel.
    outline_pts = catmull_rom([(ankle.x + dx, hw) for dx, _, _, hw in FOOT_SECTIONS], 20)
    near_side = [(p.x, ankle.y - (p.y + 0.0045)) for p in outline_pts]
    far_side = [(p.x, ankle.y + (p.y + 0.0045)) for p in reversed(outline_pts)]
    outline = [*near_side, *far_side]
    toe_lift = [0.004 * smoothstep((x - (ankle.x + 0.12)) / 0.08) for x, _ in outline]
    rows = [[(x, y, z + lift) for (x, y), lift in zip(outline, toe_lift)] for z in (0.0, SOLE_Z)]
    grid_object(f"boot_sole_{side}", rows, [shaded("sole")], closed=True, cap_start=True, cap_end=True)
    welt = [(x, ankle.y + (y - ankle.y) * 1.03) for x, y in outline]
    rows = [[(x, y, z + lift) for (x, y), lift in zip(welt, toe_lift)] for z in (SOLE_Z, WELT_Z)]
    grid_object(f"boot_welt_{side}", rows, [shaded("stitch")], closed=True, cap_start=True, cap_end=True)
    x0, x1 = ankle.x - 0.064, ankle.x - 0.004
    rows = []
    for z, inset in ((0.0, 0.004), (HEEL_Z * 0.5, 0.002), (HEEL_Z, 0.0)):
        hw = 0.035 - inset
        rows.append([(x0 + inset, ankle.y - hw, z), (x1, ankle.y - hw, z), (x1, ankle.y + hw, z),
                     (x0 + inset, ankle.y + hw, z)])
    grid_object(f"boot_heel_{side}", rows, [shaded("sole")], closed=True, cap_start=True, cap_end=True)


def build_boots() -> None:
    for side, near in (("near", True), ("far", False)):
        build_boot_shaft(side, near)
        build_foot(side, Vector(leg_curve(near)[-1]))
