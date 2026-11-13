"""Head of v003: sculpted parametric skull and face, ears, brows, eyes, hair.

The head is an ellipsoid parameterised by (u, theta): u = cos(polar angle)
from +1 (crown) to -1 (chin), theta counter-clockwise from +X (the face).
Anatomy is added as smooth displacements along the surface normal: brow
ridge, eye sockets, nose, cheekbones, jaw, chin, back of the skull. The face
looks forward with a neutral, serious expression (brows set low and straight).

Hair: a thin scalp cap stops at an irregular hairline, and stylised locks
(tapered, flattened tubes following the skull) give the silhouette its
breaks: fringe pushed forward over the brow, swept sides above the ears and
layered locks down the back. No individual strands.
"""

from __future__ import annotations

import math
import random

from mathutils import Vector

import params as P
from mesh_tools import catmull_rom, gauss, grid_object, smoothstep, subdivide, thicken, tube
from surfaces import shaded

CENTRE = Vector(P.HEAD_CENTRE)
AX, AY, AZ = P.HEAD_RADII


def _base(u: float, theta: float) -> Vector:
    s = math.sqrt(max(0.0, 1.0 - u * u))
    c, si = math.cos(theta), math.sin(theta)
    x, y, z = AX * s * c, AY * s * si, AZ * u
    low = smoothstep((-u - 0.05) / 0.9)  # 0 above the cheek line, 1 at the chin
    y *= 1.0 - 0.30 * low ** 1.3  # jaw narrows toward the chin
    if c < 0.0:
        x *= 1.0 + 0.07 * smoothstep((u + 0.2) / 0.6) - 0.5 * low  # skull back, nape tuck
    else:
        x *= 1.0 - 0.03 * low
        x *= 1.0 - 0.05 * gauss(theta, 0.0, 0.9) * smoothstep((u + 0.3) / 0.5)  # flatter face
    return Vector((x, y, z))


def _normal(u: float, theta: float) -> Vector:
    eps = 1e-3
    du = _base(min(u + eps, 1.0), theta) - _base(max(u - eps, -1.0), theta)
    dt = _base(u, theta + eps) - _base(u, theta - eps)
    n = dt.cross(du)
    if n.length < 1e-9:
        return Vector((0.0, 0.0, 1.0 if u > 0 else -1.0))
    return n.normalized()


def _features(u: float, theta: float) -> float:
    """Outward displacement (m) of the facial anatomy."""
    t = theta if theta <= math.pi else theta - 2.0 * math.pi
    at = abs(t)
    d = 0.0
    d += 0.006 * gauss(u, 0.13, 0.06) * gauss(at, 0.32, 0.3)  # brow ridge
    d -= 0.007 * gauss(u, 0.03, 0.065) * gauss(at, 0.36, 0.16)  # eye sockets
    d += 0.005 * gauss(u, -0.06, 0.08) * gauss(t, 0.0, 0.06)  # nose bridge
    d += 0.015 * gauss(u, -0.25, 0.075) * gauss(t, 0.0, 0.085)  # nose tip
    d += 0.004 * gauss(u, -0.29, 0.05) * gauss(at, 0.13, 0.06)  # nostril wings
    d += 0.008 * gauss(u, -0.18, 0.1) * gauss(at, 0.8, 0.22)  # cheekbones
    d -= 0.0025 * gauss(u, -0.38, 0.08) * gauss(at, 0.55, 0.18)  # under the cheekbone
    d += 0.006 * gauss(u, -0.52, 0.16) * gauss(t, 0.0, 0.3)  # muzzle (teeth volume)
    d += 0.0035 * gauss(u, -0.47, 0.035) * gauss(t, 0.0, 0.2)  # upper lip
    d -= 0.0045 * gauss(u, -0.535, 0.022) * gauss(t, 0.0, 0.22)  # mouth line
    d += 0.003 * gauss(u, -0.6, 0.035) * gauss(t, 0.0, 0.17)  # lower lip
    d += 0.015 * gauss(u, -0.84, 0.085) * gauss(t, 0.0, 0.32)  # chin
    d += 0.004 * gauss(u, -0.72, 0.1) * gauss(at, 1.05, 0.2)  # jaw angle
    d -= 0.002 * gauss(u, -0.55, 0.15) * gauss(at, 0.65, 0.18)  # cheek hollow beside the mouth
    return d


def surface(u: float, theta: float, offset: float = 0.0) -> Vector:
    """World point on the head surface, pushed ``offset`` along the normal."""
    return CENTRE + _base(u, theta) + _normal(u, theta) * (_features(u, theta) + offset)


def build_face() -> None:
    rows = []
    n_u = 42
    for i in range(n_u):
        u = -0.985 + 1.97 * i / (n_u - 1)
        rows.append([surface(u, 2.0 * math.pi * s / 56) for s in range(56)])
    head = grid_object("head", rows, [shaded("skin")], closed=True, cap_start=True, cap_end=True)
    subdivide(head, 1)
    # Mouth: a short, closed, straight line (serious expression).
    mouth = [surface(-0.535, a, 0.0012) for a in (-0.2, -0.1, 0.0, 0.1, 0.2)]
    tube("mouth", catmull_rom([tuple(p) for p in mouth], 9),
         [(0.0016, 0.0022 * (1.0 - abs(i - 4) / 5.0) + 0.0008) for i in range(9)],
         [shaded("skin_shadow")], sides=6, ref=(0.0, 0.0, 1.0))
    for side, sign in (("near", -1.0), ("far", 1.0)):
        # Eye: a narrow amber almond set into the socket, a dark upper lid on it.
        eye = [surface(0.035, sign * a, 0.001) for a in (0.22, 0.29, 0.37, 0.45)]
        tube(f"eye_{side}", eye, [(0.0035, 0.003), (0.0062, 0.0045), (0.0062, 0.0045), (0.003, 0.0025)],
             [shaded("eye")], sides=8, ref=(0.0, 0.0, 1.0))
        lid = [surface(u, sign * a, 0.0028) for u, a in ((0.045, 0.2), (0.062, 0.29), (0.062, 0.38), (0.05, 0.47))]
        tube(f"lid_{side}", catmull_rom([tuple(p) for p in lid], 8), [(0.0017, 0.0017)] * 8, [shaded("lash")], sides=6)
        # Brow: low, straight, inner end slightly lower (serious, determined).
        brow = [surface(u, sign * a, 0.003) for u, a in ((0.135, 0.12), (0.165, 0.3), (0.165, 0.48), (0.145, 0.62))]
        obj = tube(f"brow_{side}", brow, [(0.0035, 0.0055), (0.0045, 0.007), (0.004, 0.006), (0.0025, 0.004)],
                   [shaded("hair")], sides=8, ref=(0.0, 0.0, 1.0))
        subdivide(obj, 1)
        # Ear: a flattened oval shell with a rim, set behind the jaw line.
        base = surface(-0.06, sign * 1.62)
        path = [base + Vector((0.0, sign * 0.004, 0.024)), base + Vector((-0.012, sign * 0.012, 0.012)),
                base + Vector((-0.014, sign * 0.013, -0.008)), base + Vector((-0.004, sign * 0.006, -0.026))]
        obj = tube(f"ear_{side}", catmull_rom([tuple(p) for p in path], 8),
                   [(0.006, 0.0105), (0.0075, 0.0125), (0.0085, 0.0135), (0.0085, 0.013),
                    (0.0078, 0.012), (0.007, 0.011), (0.006, 0.0095), (0.0045, 0.007)],
                   [shaded("skin_shadow")], sides=10, ref=(0.0, sign, 0.0))
        subdivide(obj, 1)


# --- hair -------------------------------------------------------------------

HAIRLINE_FRONT = 0.38  # u of the hairline over the forehead
HAIRLINE_SIDE = 0.24  # above the ears
HAIRLINE_BACK = -0.52  # nape


def hairline(theta: float) -> float:
    c = math.cos(theta)
    jag = 0.035 * math.sin(11.0 * theta + 0.7)  # irregular edge, no straight band
    if c >= 0.0:
        return HAIRLINE_SIDE + (HAIRLINE_FRONT - HAIRLINE_SIDE) * c ** 1.5 + jag
    return HAIRLINE_SIDE + (HAIRLINE_BACK - HAIRLINE_SIDE) * (-c) ** 0.8 + jag


def build_scalp() -> None:
    rows = []
    n_t = 56
    n_u = 16
    for i in range(n_u):
        f = i / (n_u - 1)
        row = []
        for s in range(n_t):
            th = 2.0 * math.pi * s / n_t
            u = hairline(th) + (0.995 - hairline(th)) * f
            row.append(surface(u, th, 0.006))
        rows.append(row)
    obj = grid_object("hair_scalp", rows, [shaded("hair"), shaded("hair")], closed=True, cap_end=True)
    thicken(obj, 0.006)
    subdivide(obj, 1)


def _direction(u: float, theta: float) -> Vector:
    s = math.sqrt(max(0.0, 1.0 - u * u))
    return Vector((s * math.cos(theta), s * math.sin(theta), u))


def _lock(name: str, start: tuple[float, float], end: tuple[float, float], lift: float,
          tip_out: float, width: float, material: str, curl: float = 0.0) -> None:
    """A lock from start (u, theta) to end along the great circle over the skull.

    ``lift`` raises the middle of the lock off the scalp, ``tip_out`` pushes
    the tip outward (the irregular silhouette), ``curl`` bends it sideways.
    """
    a, b = _direction(*start), _direction(*end)
    pts = []
    n = 9
    for i in range(n):
        f = i / (n - 1)
        d = a.slerp(b, f) if a.angle(b) > 1e-4 else a
        u = max(min(d.z, 0.999), -0.999)
        th = math.atan2(d.y, d.x) + curl * math.sin(math.pi * f)
        off = 0.008 + lift * math.sin(math.pi * min(f * 1.15, 1.0)) + tip_out * f ** 2
        pts.append(surface(u, th, off))
    path = catmull_rom([tuple(p) for p in pts], 14)
    m = len(path)
    radii = []
    for i in range(m):
        f = i / (m - 1)
        taper = (1.0 - f) ** 0.8 * 0.92 + 0.08
        radii.append((0.0045 * taper + 0.0008, width * taper))
    # Flattened section: thin along the surface normal, wide across.
    ref = tuple(_normal(start[0], start[1]))
    obj = tube(name, path, radii, [shaded(material)], sides=8, ref=ref)
    subdivide(obj, 1)


def build_hair() -> None:
    build_scalp()
    rng = random.Random(P.SEED)

    def j(scale: float) -> float:
        return rng.uniform(-scale, scale)

    # Fringe: from the crown forward and down past the brow, tips projecting.
    for i, (th, end_u) in enumerate(((-0.62, 0.3), (-0.42, 0.2), (-0.22, 0.27), (-0.04, 0.17),
                                     (0.14, 0.25), (0.32, 0.19), (0.5, 0.28), (0.66, 0.33))):
        _lock(f"hair_fringe_{i}", (0.9 + j(0.04), th * 0.4 + j(0.05)), (end_u + j(0.03), th * 1.15 + j(0.05)),
              lift=0.011 + j(0.003), tip_out=0.022 + j(0.006), width=0.016 + j(0.002),
              material="hair_light" if i % 3 == 1 else "hair", curl=j(0.08))
    # Top: layered locks swept back, lifted for an irregular crown silhouette.
    for i, th in enumerate((-0.55, -0.2, 0.15, 0.5)):
        _lock(f"hair_top_{i}", (0.78 + j(0.04), th), (0.25 + j(0.06), th + math.pi + j(0.15)),
              lift=0.006 + j(0.002), tip_out=0.006 + j(0.003), width=0.02 + j(0.002), material="hair")
    # Sides: swept back and down over the temples, stopping above the ears.
    for sign in (-1.0, 1.0):
        for i, (u0, u1, th0, th1) in enumerate(((0.75, 0.2, 0.75, 1.25), (0.7, 0.18, 1.1, 1.6),
                                                (0.72, 0.12, 1.45, 1.95), (0.62, 0.02, 1.85, 2.35))):
            _lock(f"hair_side_{'near' if sign < 0 else 'far'}_{i}", (u0 + j(0.03), sign * (th0 + j(0.05))),
                  (u1 + j(0.03), sign * (th1 + j(0.06))), lift=0.008 + j(0.003), tip_out=0.008 + j(0.004),
                  width=0.017 + j(0.002), material="hair_light" if i == 1 else "hair")
    # Back: layered locks down to the nape, uneven tips.
    for i, th in enumerate((2.35, 2.6, 2.85, 3.1, 3.35, 3.6, 3.85, 4.1)):
        _lock(f"hair_back_{i}", (0.7 + j(0.05), th + j(0.06)), (-0.42 + j(0.08), th + j(0.1)),
              lift=0.011 + j(0.004), tip_out=0.01 + j(0.005), width=0.02 + j(0.002), material="hair",
              curl=j(0.1))
