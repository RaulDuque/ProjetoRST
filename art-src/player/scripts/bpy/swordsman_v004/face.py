"""Head of v004: refined face (planes, nose, lids, brows, mouth) and hair.

Same parametric approach as v003 (``swordsman_v003/head.py``, untouched): an
ellipsoid parameterised by (u, theta), u = cos(polar angle) from +1 (crown)
to -1 (chin), theta counter-clockwise from +X (the face), with anatomy added
as displacements along the normal. What changes:

- **resolution where the face is**: rows are uniform in polar angle and the
  columns are warped so the front of the face gets ~2.5x the density of the
  back; v003 had ~1.5 columns across the nose, v004 has ~6;
- **sharper planes**: narrower falloffs and ridges along curves (nose
  bridge, jaw line, cheekbone edge) instead of round blobs, so the face has
  planes and edges instead of a uniformly soft surface;
- **eyes**: almond opening defined by an upper and a lower lid curve, slight
  upward tilt to the outer corner; recessed sclera, amber iris disc with a
  darker ring (no pupil, as the art direction asks), heavy upper lid line
  with a short outer wing, thin lower lid and a lid crease;
- **brows**: flattened, tapered strips hugging the brow ridge, inner end
  lower and thicker (serious, determined);
- **mouth**: upper and lower lip volumes, mouth line with corners set
  level-to-slightly-down (neutral, determined), chin groove;
- **hair**: thinner locks with a lens-shaped section (sharp edges instead of
  round tubes), tips tapering to a point, widths and directions jittered,
  layered overlay locks, and a scalp cap that thins out at the hairline.

Limit of the approach: displacements on a single ellipsoid cannot make
undercuts (nostril openings, ear folds, lip overhang). Those need manual
sculpting (see the report, section C).
"""

from __future__ import annotations

import math
import random

from mathutils import Vector

import params as P
from mesh_tools import catmull_rom, gauss, grid_object, profile, smoothstep, subdivide, thicken, tube
from shapes import orient_outward
from surfaces import shaded

CENTRE = Vector(P.HEAD_CENTRE)
AX, AY, AZ = P.HEAD_RADII
COLUMN_WARP = 0.6  # 0 = uniform columns; 0.6 = 2.5x denser at the face than at the back


def _base(u: float, theta: float) -> Vector:
    s = math.sqrt(max(0.0, 1.0 - u * u))
    c, si = math.cos(theta), math.sin(theta)
    x, y, z = AX * s * c, AY * s * si, AZ * u
    low = smoothstep((-u - 0.05) / 0.9)  # 0 above the cheek line, 1 at the chin
    y *= 1.0 - 0.21 * low ** 1.4  # jaw narrows toward the chin (v003: 0.30, pointier)
    temple = gauss(u, 0.3, 0.25) * gauss(abs(math.atan2(si, c)), 1.1, 0.5)
    y *= 1.0 - 0.035 * temple  # flatter temples: the skull reads less like a ball
    if c < 0.0:
        x *= 1.0 + 0.07 * smoothstep((u + 0.2) / 0.6) - 0.5 * low
    else:
        x *= 1.0 - 0.03 * low
        # Flatter face. v003 used gauss(theta, ...) with theta in [0, 2pi), so
        # the near half (theta ~ 2pi) was never flattened; wrapped here.
        wrapped = math.atan2(si, c)
        x *= 1.0 - 0.06 * gauss(wrapped, 0.0, 0.9) * smoothstep((u + 0.3) / 0.5)
    return Vector((x, y, z))


def _normal(u: float, theta: float) -> Vector:
    eps = 1e-3
    du = _base(min(u + eps, 1.0), theta) - _base(max(u - eps, -1.0), theta)
    dt = _base(u, theta + eps) - _base(u, theta - eps)
    n = dt.cross(du)
    if n.length < 1e-9:
        return Vector((0.0, 0.0, 1.0 if u > 0 else -1.0))
    return n.normalized()


# Nose bridge height (m) along u: grows from the brow to the tip, then drops.
NOSE_RIDGE = ((0.10, 0.0), (0.03, 0.0035), (-0.08, 0.0075), (-0.17, 0.0125), (-0.215, 0.0155),
              (-0.255, 0.012), (-0.29, 0.0))


def _jaw_line(at: float) -> float:
    """u of the lower jaw edge at |theta| = at (chin at 0, jaw angle ~1.15)."""
    return -0.9 + 0.2 * smoothstep(at / 1.15)


def _features(u: float, theta: float) -> float:
    """Outward displacement (m) of the facial anatomy."""
    t = theta if theta <= math.pi else theta - 2.0 * math.pi
    at = abs(t)
    d = 0.0
    # Forehead and brow: ridge over the eyes, a small glabella between them.
    d += 0.0075 * gauss(u, 0.145, 0.05) * gauss(at, 0.36, 0.3)
    d += 0.003 * gauss(u, 0.12, 0.05) * gauss(t, 0.0, 0.12)
    # Eye sockets: deeper toward the inner corner, under the brow ridge.
    d -= 0.0085 * gauss(u, 0.04, 0.06) * gauss(at, 0.33, 0.15)
    d -= 0.003 * gauss(u, 0.05, 0.05) * gauss(at, 0.2, 0.07)
    # Nose: a narrow ridge whose height follows NOSE_RIDGE, then the tip,
    # the wings and the dents of the nostrils.
    if -0.35 < u < 0.15:
        d += profile(tuple(reversed(NOSE_RIDGE)), u) * gauss(t, 0.0, 0.05 + 0.03 * smoothstep(-u / 0.25))
    d += 0.004 * gauss(u, -0.225, 0.04) * gauss(t, 0.0, 0.06)  # tip (rounded, not pointed)
    d += 0.0045 * gauss(u, -0.25, 0.035) * gauss(at, 0.115, 0.045)  # wings (alae)
    d -= 0.002 * gauss(u, -0.21, 0.035) * gauss(at, 0.15, 0.03)  # crease above the wings
    d -= 0.0028 * gauss(u, -0.29, 0.02) * gauss(at, 0.06, 0.03)  # nostrils
    # Cheekbones: a volume with a defined lower edge (hollow under it).
    d += 0.007 * gauss(u, -0.11, 0.08) * gauss(at, 0.82, 0.22)
    d -= 0.0016 * gauss(u, -0.42, 0.14) * gauss(at, 0.7, 0.2)
    d -= 0.001 * gauss(u, -0.5, 0.12) * gauss(at, 0.5, 0.14)  # nasolabial plane
    # Mouth: muzzle, upper lip, mouth line, lower lip, groove above the chin.
    d += 0.003 * gauss(u, -0.5, 0.15) * gauss(t, 0.0, 0.3)
    d += 0.0028 * gauss(u, -0.47, 0.03) * gauss(t, 0.0, 0.2)
    d -= 0.0012 * gauss(u, -0.43, 0.03) * gauss(t, 0.0, 0.035)  # philtrum
    d -= 0.0042 * gauss(u, -0.522, 0.014) * gauss(t, 0.0, 0.22)
    d += 0.0026 * gauss(u, -0.575, 0.03) * gauss(t, 0.0, 0.15)
    d -= 0.003 * gauss(u, -0.66, 0.035) * gauss(t, 0.0, 0.2)
    # Chin: square-ish (two side lobes beside the centre), strong in profile.
    d += 0.0185 * gauss(u, -0.79, 0.12) * gauss(t, 0.0, 0.34)
    d += 0.0025 * gauss(u, -0.83, 0.08) * gauss(at, 0.26, 0.12)
    # Jaw: a ridge along the mandible edge and a marked angle under the ear.
    if at < 1.4:
        d += 0.0025 * gauss(u - _jaw_line(at), 0.0, 0.07) * smoothstep(at / 0.3)
    d += 0.0028 * gauss(u, -0.7, 0.12) * gauss(at, 1.12, 0.22)
    return d


def surface(u: float, theta: float, offset: float = 0.0) -> Vector:
    """World point on the head surface, pushed ``offset`` along the normal."""
    return CENTRE + _base(u, theta) + _normal(u, theta) * (_features(u, theta) + offset)


def _column_theta(s: int, count: int) -> float:
    phi = -math.pi + 2.0 * math.pi * s / count
    return (phi - COLUMN_WARP * math.sin(phi)) % (2.0 * math.pi)


def build_head() -> None:
    rows = []
    for i in range(P.HEAD_ROWS):
        polar = math.radians(178.5 - 177.0 * i / (P.HEAD_ROWS - 1))
        u = math.cos(polar)
        rows.append([surface(u, _column_theta(s, P.HEAD_COLUMNS)) for s in range(P.HEAD_COLUMNS)])
    head = grid_object("head", rows, [shaded("skin")], closed=True, cap_start=True, cap_end=True)
    subdivide(head, 1)


# --- eyes, lids, brows, mouth, ears ------------------------------------------

EYE_INNER = 0.21  # |theta| of the inner corner
EYE_OUTER = 0.49  # |theta| of the outer corner
EYE_CENTRE_U = 0.03
EYE_TILT_U = 0.012  # outer corner this much higher than the inner one
EYE_UPPER_U = 0.036  # opening above the centre line (peak toward the inner third)
EYE_LOWER_U = 0.019
IRIS_AT = 0.33  # |theta| of the iris centre: looking straight ahead
IRIS_RADIUS = 0.0036
# The subdivided head rises above its control surface in the concave sockets,
# so the eye sits this far off it to stay visible.
EYE_SURFACE_OFFSET = 0.0016


def _eye_centre_u(f: float) -> float:
    return EYE_CENTRE_U + EYE_TILT_U * f


def eye_upper(f: float) -> float:
    return _eye_centre_u(f) + EYE_UPPER_U * math.sin(math.pi * f) ** 0.75 * (1.0 - 0.25 * f)


def eye_lower(f: float) -> float:
    return _eye_centre_u(f) - EYE_LOWER_U * math.sin(math.pi * f) ** 1.1


def _eye_theta(sign: float, f: float) -> float:
    return sign * (EYE_INNER + (EYE_OUTER - EYE_INNER) * f)


def build_eye(side: str, sign: float) -> None:
    cols, rows_n = 16, 7
    rows = []
    for r in range(rows_n):
        v = r / (rows_n - 1)
        row = []
        for c in range(cols + 1):
            f = c / cols
            u = eye_lower(f) + (eye_upper(f) - eye_lower(f)) * v
            row.append(surface(u, _eye_theta(sign, f), EYE_SURFACE_OFFSET))
        rows.append(row)
    orient_outward(grid_object(f"eye_sclera_{side}", rows, [shaded("sclera")]), CENTRE)
    # Iris: two discs along the surface normal, darker ring behind.
    f_iris = (IRIS_AT - EYE_INNER) / (EYE_OUTER - EYE_INNER)
    u_iris = (eye_lower(f_iris) + eye_upper(f_iris)) / 2.0 + 0.004
    centre = surface(u_iris, sign * IRIS_AT, EYE_SURFACE_OFFSET + 0.0001)
    normal = _normal(u_iris, sign * IRIS_AT)
    for name, radius, lift, material in (("ring", IRIS_RADIUS, 0.0, "lash"),
                                         ("iris", IRIS_RADIUS * 0.8, 0.00025, "eye")):
        tube(f"eye_{name}_{side}", [centre + normal * lift, centre + normal * (lift + 0.0004)],
             [(radius, radius), (radius * 0.97, radius * 0.97)], [shaded(material)], sides=20,
             ref=(0.0, 0.0, 1.0))
    # Upper lid: heavy dark line, thicker toward the outer corner, with a short wing.
    lid_pts = [surface(eye_upper(f) + 0.004, _eye_theta(sign, f), EYE_SURFACE_OFFSET + 0.0012) for f in (0.0, 0.2, 0.45, 0.7, 0.9, 1.0)]
    wing = surface(_eye_centre_u(1.0) + 0.012, sign * (EYE_OUTER + 0.045), 0.0004)
    lid_path = catmull_rom([tuple(p) for p in [*lid_pts, wing]], 16)
    lid_radii = [(0.0011 + 0.0006 * k / 15, 0.0012 + 0.0012 * math.sin(math.pi * min(k / 13, 1.0)))
                 for k in range(16)]
    tube(f"lid_upper_{side}", lid_path, lid_radii, [shaded("lash")], sides=6,
         ref=tuple(_normal(eye_upper(0.5), _eye_theta(sign, 0.5))))
    # Lid crease (skin fold above the lid) and lower lid: thin skin-shadow lines.
    crease = [surface(eye_upper(f) + 0.03 + 0.008 * math.sin(math.pi * f), _eye_theta(sign, f), 0.0012)
              for f in (0.12, 0.4, 0.7, 0.95)]
    tube(f"lid_crease_{side}", catmull_rom([tuple(p) for p in crease], 10),
         [(0.0006, 0.0009 * (0.4 + 0.6 * math.sin(math.pi * k / 9))) for k in range(10)],
         [shaded("skin_shadow")], sides=6)
    lower = [surface(eye_lower(f) - 0.003, _eye_theta(sign, f), EYE_SURFACE_OFFSET + 0.0003) for f in (0.05, 0.35, 0.65, 0.95)]
    tube(f"lid_lower_{side}", catmull_rom([tuple(p) for p in lower], 10),
         [(0.0007, 0.0008 * (0.5 + 0.5 * math.sin(math.pi * k / 9))) for k in range(10)],
         [shaded("skin_shadow")], sides=6)


def build_brow(side: str, sign: float) -> None:
    # Inner end low and thick, rising, then a gentle fall to a thin tail.
    keys = ((0.118, 0.105, 0.0042), (0.142, 0.2, 0.0046), (0.163, 0.33, 0.004),
            (0.168, 0.46, 0.0031), (0.15, 0.6, 0.0016))
    path = catmull_rom([tuple(surface(u, sign * a, 0.0016)) for u, a, _ in keys], 14)
    widths = [profile(tuple((k / 4, w) for k, (_, _, w) in enumerate(keys)), i / 13) for i in range(14)]
    ref = tuple(_normal(0.155, sign * 0.3))
    obj = tube(f"brow_{side}", path, [(0.0016, w) for w in widths], [shaded("brow")], sides=8, ref=ref)
    subdivide(obj, 1)


def build_mouth() -> None:
    keys = ((-0.534, -0.2), (-0.527, -0.12), (-0.522, 0.0), (-0.527, 0.12), (-0.534, 0.2))
    path = catmull_rom([tuple(surface(u, a, 0.0009)) for u, a in keys], 13)
    tube("mouth", path, [(0.0011, 0.0006 + 0.0011 * math.sin(math.pi * i / 12)) for i in range(13)],
         [shaded("lip")], sides=6, ref=(1.0, 0.0, 0.0))


def build_ear(side: str, sign: float) -> None:
    """Same shell as v003, with a darker inner rim for depth."""
    base = surface(-0.06, sign * 1.62)
    path = [base + Vector((0.0, sign * 0.004, 0.024)), base + Vector((-0.012, sign * 0.012, 0.012)),
            base + Vector((-0.014, sign * 0.013, -0.008)), base + Vector((-0.004, sign * 0.006, -0.026))]
    obj = tube(f"ear_{side}", catmull_rom([tuple(p) for p in path], 8),
               [(0.006, 0.0105), (0.0075, 0.0125), (0.0085, 0.0135), (0.0085, 0.013),
                (0.0078, 0.012), (0.007, 0.011), (0.006, 0.0095), (0.0045, 0.007)],
               [shaded("skin")], sides=10, ref=(0.0, sign, 0.0))
    subdivide(obj, 1)


def build_face() -> None:
    build_head()
    build_mouth()
    for side, sign in (("near", -1.0), ("far", 1.0)):
        build_eye(side, sign)
        build_brow(side, sign)
        build_ear(side, sign)


# --- hair -------------------------------------------------------------------

HAIRLINE_FRONT = 0.4
HAIRLINE_SIDE = 0.25
HAIRLINE_BACK = -0.52


def hairline(theta: float) -> float:
    c = math.cos(theta)
    jag = 0.02 * math.sin(11.0 * theta + 0.7) + 0.012 * math.sin(23.0 * theta + 2.1)
    if c >= 0.0:
        return HAIRLINE_SIDE + (HAIRLINE_FRONT - HAIRLINE_SIDE) * c ** 1.5 + jag
    return HAIRLINE_SIDE + (HAIRLINE_BACK - HAIRLINE_SIDE) * (-c) ** 0.8 + jag


def build_scalp() -> None:
    """Cap under the locks; thins to the skin at the hairline (no hard band)."""
    rows = []
    n_t, n_u = 72, 16
    for i in range(n_u):
        f = i / (n_u - 1)
        off = 0.0012 + 0.0048 * smoothstep(f / 0.3)
        row = []
        for s in range(n_t):
            th = 2.0 * math.pi * s / n_t
            u = hairline(th) + (0.995 - hairline(th)) * f
            row.append(surface(u, th, off))
        rows.append(row)
    obj = grid_object("hair_scalp", rows, [shaded("hair"), shaded("hair")], closed=True, cap_end=True)
    thicken(obj, 0.004)
    subdivide(obj, 1)


def _direction(u: float, theta: float) -> Vector:
    s = math.sqrt(max(0.0, 1.0 - u * u))
    return Vector((s * math.cos(theta), s * math.sin(theta), u))


def _lens(ang: float) -> tuple[float, float]:
    """Lock section: domed on top, flat underneath, sharp side edges."""
    c, s = math.cos(ang), math.sin(ang)
    x = c * (0.3 + 0.7 * abs(c))
    return (x if c >= 0.0 else 0.45 * x, s)


def _lock(name: str, start: tuple[float, float], end: tuple[float, float], lift: float,
          tip_out: float, width: float, material: str, curl: float = 0.0, thickness: float = 0.0042,
          base_off: float = 0.007) -> None:
    """A lock from start (u, theta) to end over the skull, tapering to a point."""
    a, b = _direction(*start), _direction(*end)
    pts = []
    n = 10
    for i in range(n):
        f = i / (n - 1)
        d = a.slerp(b, f) if a.angle(b) > 1e-4 else a
        u = max(min(d.z, 0.999), -0.999)
        th = math.atan2(d.y, d.x) + curl * math.sin(math.pi * f) * f
        off = base_off + lift * math.sin(math.pi * min(f * 1.15, 1.0)) + tip_out * f ** 2
        pts.append(surface(u, th, off))
    path = catmull_rom([tuple(p) for p in pts], 18)
    m = len(path)
    radii = []
    for i in range(m):
        f = i / (m - 1)
        root = 0.65 + 0.35 * smoothstep(f / 0.18)
        taper = (1.0 - f) ** 1.15 * (1.0 - P.HAIR_TIP_WIDTH) + P.HAIR_TIP_WIDTH
        radii.append((thickness * (0.35 + 0.65 * taper) * root, width * taper * root))
    ref = tuple(_normal(start[0], start[1]))
    obj = tube(name, path, radii, [shaded(material)], sides=10, ref=ref, section=_lens)
    subdivide(obj, 1)


def build_hair() -> None:
    build_scalp()
    rng = random.Random(P.SEED)

    def j(scale: float) -> float:
        return rng.uniform(-scale, scale)

    # Fringe: from the crown forward and down past the brow; uneven lengths,
    # tips projecting forward (the silhouette mark of the design).
    fringe = ((-0.66, 0.32, 1.0), (-0.5, 0.22, 0.75), (-0.36, 0.28, 1.1), (-0.22, 0.16, 0.8),
              (-0.08, 0.24, 1.15), (0.06, 0.13, 0.7), (0.2, 0.22, 1.05), (0.34, 0.17, 0.85),
              (0.48, 0.27, 1.0), (0.62, 0.3, 0.8), (0.74, 0.36, 0.9))
    for i, (th, end_u, w) in enumerate(fringe):
        _lock(f"hair_fringe_{i}", (0.9 + j(0.04), th * 0.4 + j(0.05)), (end_u + j(0.03), th * 1.15 + j(0.06)),
              lift=0.01 + j(0.003), tip_out=0.02 + j(0.007), width=0.019 * w + j(0.002),
              material="hair_light" if i % 4 == 1 else "hair", curl=j(0.14))
    # Overlay fringe: shorter, thinner locks on top, breaking the regular rhythm.
    for i, th in enumerate((-0.42, -0.05, 0.27, 0.55)):
        _lock(f"hair_fringe_over_{i}", (0.93 + j(0.03), th * 0.3), (0.36 + j(0.05), th * 1.05 + j(0.08)),
              lift=0.008 + j(0.002), tip_out=0.016 + j(0.005), width=0.014 + j(0.002),
              material="hair_light" if i % 2 else "hair", curl=j(0.18), thickness=0.0032, base_off=0.009)
    # Top: layered locks swept back, lifted for an irregular crown silhouette.
    for i, th in enumerate((-0.7, -0.4, -0.12, 0.16, 0.44, 0.72)):
        _lock(f"hair_top_{i}", (0.8 + j(0.05), th + j(0.05)), (0.22 + j(0.08), th + math.pi + j(0.2)),
              lift=0.005 + j(0.002), tip_out=0.006 + j(0.003), width=0.021 + j(0.003), material="hair",
              base_off=0.0085)
    # Sides: swept back and down over the temples, stopping above the ears.
    sides = ((0.76, 0.2, 0.72, 1.22), (0.72, 0.15, 1.0, 1.5), (0.7, 0.1, 1.3, 1.82),
             (0.66, 0.06, 1.6, 2.1), (0.6, 0.0, 1.9, 2.4))
    for sign in (-1.0, 1.0):
        for i, (u0, u1, th0, th1) in enumerate(sides):
            _lock(f"hair_side_{'near' if sign < 0 else 'far'}_{i}", (u0 + j(0.03), sign * (th0 + j(0.05))),
                  (u1 + j(0.04), sign * (th1 + j(0.07))), lift=0.007 + j(0.003), tip_out=0.008 + j(0.005),
                  width=0.019 + j(0.003), material="hair_light" if i == 2 else "hair", curl=j(0.1))
    # Back: layered locks down to the nape, uneven tips; a second shorter layer.
    for i, th in enumerate((2.3, 2.5, 2.7, 2.9, 3.1, 3.3, 3.5, 3.7, 3.9, 4.1)):
        _lock(f"hair_back_{i}", (0.7 + j(0.05), th + j(0.06)), (-0.4 + j(0.1), th + j(0.12)),
              lift=0.01 + j(0.004), tip_out=0.011 + j(0.006), width=0.021 + j(0.003), material="hair",
              curl=j(0.14))
    for i, th in enumerate((2.6, 3.0, 3.4, 3.8)):
        _lock(f"hair_back_over_{i}", (0.6 + j(0.04), th + j(0.08)), (-0.1 + j(0.08), th + j(0.12)),
              lift=0.008 + j(0.003), tip_out=0.01 + j(0.004), width=0.017 + j(0.002),
              material="hair_light" if i == 2 else "hair", curl=j(0.16), thickness=0.0034, base_off=0.009)
