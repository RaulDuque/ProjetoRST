"""Sword of v003 and the hand that holds it.

Long, slender sabre with a gentle curve toward the edge. The blade is a loft
of a six-point section (flat spine, bevelled faces meeting at the edge) that
narrows to a point; the edge bevel uses a lighter, slightly emissive
material so a bright line survives on dark backgrounds and at sprite scale.
Simple brass guard bent toward the blade, leather grip, brass pommel.

``build_sword(width_scale)`` can exaggerate only the blade width for a
readability variant render; the reach (hitboxes) is not touched.
"""

from __future__ import annotations

import math

from mathutils import Vector

import params as P
from anatomy import build_sword_hand
from mesh_tools import catmull_rom, profile, subdivide, tube
from surfaces import shaded

AXIS = Vector(P.SWORD_DIRECTION).normalized()
WIDTH_DIR = AXIS.cross(Vector((0.0, 1.0, 0.0))).normalized()  # forward-up; the edge side
WRIST = Vector(P.WRIST_NEAR)
FOREARM = (WRIST - Vector(P.ELBOW_NEAR)).normalized()
GRIP_CENTRE = WRIST + FOREARM * 0.06
GUARD = GRIP_CENTRE + AXIS * 0.072

# Blade section (x: thickness side, y: width side), counter-clockwise; the
# edge is vertex 2, so faces 1 and 2 form the edge bevel.
BLADE_SECTION = ((1.0, -0.95), (0.8, 0.3), (0.0, 1.0), (-0.8, 0.3), (-1.0, -0.95), (0.0, -1.04))
EDGE_FACES = (1, 2)
WIDTH_KEYS = ((0.0, 1.0), (0.1, 0.97), (0.75, 0.82), (0.9, 0.62), (0.97, 0.3), (1.0, 0.05))
THICK_KEYS = ((0.0, 1.0), (0.8, 0.65), (1.0, 0.25))


def _section(angle: float) -> tuple[float, float]:
    index = round(angle / (2.0 * math.pi) * len(BLADE_SECTION)) % len(BLADE_SECTION)
    return BLADE_SECTION[index]


def blade_point(s: float) -> Vector:
    """Point on the blade centre line, s = 0 at the guard and 1 at the tip."""
    bow = P.BLADE_CURVE * math.sin(math.pi * s * 0.9) * s ** 0.3
    return GUARD + AXIS * (0.006 + P.BLADE_LENGTH * s) + WIDTH_DIR * bow


def tip() -> Vector:
    return blade_point(1.0)


def build_blade(width_scale: float = 1.0, suffix: str = "") -> None:
    samples = 26
    path = [blade_point(i / (samples - 1)) for i in range(samples)]
    radii = []
    for i in range(samples):
        s = i / (samples - 1)
        radii.append((P.BLADE_THICKNESS / 2.0 * profile(THICK_KEYS, s),
                      P.BLADE_WIDTH / 2.0 * width_scale * profile(WIDTH_KEYS, s)))
    tube(f"blade{suffix}", path, radii, [shaded("blade"), shaded("blade_edge")],
         sides=len(BLADE_SECTION), ref=(0.0, 1.0, 0.0), section=_section,
         face_material=lambda r, c: 1 if c in EDGE_FACES else 0)


def build_sword() -> None:
    build_blade()
    # Ricasso collar.
    tube("sword_collar", [tuple(GUARD - AXIS * 0.002), tuple(GUARD + AXIS * 0.014)],
         [(0.0085, 0.024), (0.0075, 0.022)], [shaded("metal_dark")], sides=10, ref=(0.0, 1.0, 0.0))
    # Guard: a short bar across the blade, tips bent toward the point.
    guard = catmull_rom([tuple(GUARD - WIDTH_DIR * 0.066 + AXIS * 0.012), tuple(GUARD - WIDTH_DIR * 0.03),
                         tuple(GUARD), tuple(GUARD + WIDTH_DIR * 0.03),
                         tuple(GUARD + WIDTH_DIR * 0.066 + AXIS * 0.012)], 12)
    radii = [(0.0065 + 0.0025 * math.sin(math.pi * i / 11),) * 2 for i in range(12)]
    obj = tube("sword_guard", guard, radii, [shaded("brass")], sides=10, ref=(0.0, 1.0, 0.0))
    subdivide(obj, 1)
    # Grip (leather wrap) and pommel.
    grip_start = GUARD - AXIS * P.GRIP_LENGTH
    grip = [grip_start.lerp(GUARD, i / 7) for i in range(8)]
    radii = [(0.0135 + 0.0025 * math.sin(math.pi * i / 7),) * 2 for i in range(8)]
    obj = tube("sword_grip", grip, radii, [shaded("boot_leather")], sides=10, ref=(0.0, 1.0, 0.0))
    subdivide(obj, 1)
    pommel = [grip_start + AXIS * 0.004, grip_start - AXIS * 0.012, grip_start - AXIS * 0.028,
              grip_start - AXIS * 0.038]
    obj = tube("sword_pommel", pommel, [(0.012, 0.012), (0.019, 0.019), (0.016, 0.016), (0.006, 0.006)],
               [shaded("brass")], sides=12, ref=(0.0, 1.0, 0.0))
    subdivide(obj, 1)
    build_sword_hand(GRIP_CENTRE, AXIS, WRIST)
