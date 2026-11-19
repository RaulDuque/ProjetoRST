"""N4-A: reconstruction of the anatomy hidden by hair and clothes (idle N3.5).

THIS IS AN ARTISTIC INTERPRETATION BY THE AGENT, NOT RECOVERED ANATOMY.
Nothing under the hair or the clothes exists in the approved art; the shapes
below are authored row by row from the visible face, forearm and hand, the
clothing silhouette and the proportions of the owner's reference sheet
(side view of the neutral A-pose body). They need the owner's evaluation.

Rules that keep the approved look intact:
- a painted texel is only placed where the original has an opaque texel that
  belongs to a layer drawn ABOVE the painted layer (so the full outfit hides
  it and the recomposition stays exact); anything else is clipped and counted;
- original texels are never repainted;
- only palette keys of the approved art are used (skin S s z, outline K, and
  a neutral dark-grey undergarment q g on the hips instead of nudity).

Spans are inclusive canvas columns per row: {row: (left, right)}.
"""

from __future__ import annotations

# Skull under the hair (body layer). The face (x >= 41, rows 18..27) and the
# ear (x 39..40, rows 21..23) are original texels and stay as they are.
SKULL = {
    15: (38, 43), 16: (36, 44), 17: (35, 45), 18: (35, 45), 19: (34, 45), 20: (34, 45),
    21: (34, 46), 22: (34, 46), 23: (35, 46), 24: (35, 45), 25: (36, 44), 26: (37, 44),
}
NECK = {27: (38, 42), 28: (38, 42), 29: (38, 42), 30: (37, 43)}
# Profile torso: chest forward (x 46 at rows 34..36), shoulder blade back
# (x 34), narrower waist (rows 44..48).
TORSO = {
    31: (37, 43), 32: (36, 44), 33: (35, 45), 34: (34, 46), 35: (34, 46), 36: (34, 46),
    37: (34, 45), 38: (34, 44), 39: (34, 44), 40: (34, 44), 41: (35, 44), 42: (35, 44),
    43: (35, 43), 44: (35, 43), 45: (35, 43), 46: (35, 43), 47: (35, 43), 48: (35, 44),
    49: (35, 44), 50: (35, 44),
}
# Neutral undergarment (hips), drawn in the body layer.
HIPS = {
    51: (35, 45), 52: (35, 45), 53: (34, 45), 54: (34, 45), 55: (34, 45), 56: (34, 45),
    57: (34, 45), 58: (34, 45), 59: (34, 44), 60: (34, 44),
}
# Both legs, side by side in profile: thigh forward, knee at rows 80..84,
# calf bulging back at rows 91..98, straight shin, ankle.
LEGS = {
    **{y: (35, 44) for y in range(61, 67)},
    **{y: (35, 43) for y in range(67, 76)},
    **{y: (35, 42) for y in range(76, 80)},
    **{y: (35, 43) for y in range(80, 85)},
    **{y: (34, 41) for y in range(85, 91)},
    **{y: (33, 40) for y in range(91, 99)},
    **{y: (34, 39) for y in range(99, 105)},
    **{y: (35, 38) for y in range(105, 113)},
}
FEET = {
    113: (34, 39), 114: (34, 40), 115: (33, 42), 116: (33, 44), 117: (33, 46), 118: (33, 46),
}
# Upper front arm under the sleeve (arm_front layer): round shoulder,
# biceps, tapering to the elbow and the visible forearm (row 52, x 36..40).
UPPER_ARM = {
    31: (39, 42), 32: (38, 43), 33: (38, 43), 34: (38, 43), 35: (38, 43), 36: (38, 43),
    37: (38, 43), 38: (38, 42), 39: (38, 42), 40: (38, 42), 41: (38, 42), 42: (38, 42),
    43: (38, 42), 44: (37, 41), 45: (37, 41), 46: (37, 41), 47: (37, 41), 48: (37, 41),
    49: (36, 41), 50: (36, 41), 51: (36, 41),
}
# Wrist under the bracer (arm_front layer), between the visible forearm
# (rows 52..58) and the visible hand (rows 64..71).
WRIST = {59: (38, 41), 60: (39, 42), 61: (39, 42), 62: (39, 42), 63: (40, 43)}
# Small shading accents (key per texel), applied after the base shading:
# pectoral shadow, collarbone, abdominal line, knee light, shin light.
ACCENTS = {
    **{(x, 37): "z" for x in range(41, 45)},
    **{(x, 31): "s" for x in range(39, 43)},
    **{(41, y): "s" for y in range(40, 48)},
    **{(41, y): "S" for y in range(81, 84)},
    **{(38, y): "S" for y in range(100, 110)},
    # waistband of the undergarment
    **{(x, 52): "l" for x in range(36, 45)},
}

def skin_key(t: float) -> str:
    """Shade across a row, back (t = 0) to front (t = 1), like the visible
    forearm and face: dark back edge, lit middle, slightly darker front."""
    if t < 0.18:
        return "z"
    if t < 0.4:
        return "s"
    if t < 0.86:
        return "S"
    return "s"


def cloth_key(t: float) -> str:
    return "g" if 0.45 <= t < 0.8 else "q"


def spans_to_texels(spans: dict[int, tuple[int, int]], shade) -> dict[tuple[int, int], str]:
    out = {}
    for y, (a, b) in spans.items():
        for x in range(a, b + 1):
            t = 0.5 if b == a else (x - a) / (b - a)
            out[(x, y)] = shade(t)
    return out


def leg_split(texels: dict[tuple[int, int], str]) -> None:
    """A shadow line between the near and the far leg (rows 66..108)."""
    for y in range(66, 109):
        xs = [x for (x, yy) in texels if yy == y]
        if xs:
            mid = (min(xs) + max(xs)) // 2
            texels[(mid, y)] = "z"


def outline(texels: dict[tuple[int, int], str], neighbours: set[tuple[int, int]]) -> None:
    """Exposed edges like the approved art: a dark outline (K) on the back
    and bottom edges, the shadow tone (z) on the front and top edges (the
    face and the forearm end in z, not in a black line). Texels next to
    original texels of the same layer (face, forearm) are not outlined."""
    shape = set(texels) | neighbours
    for (x, y) in list(texels):
        back = (x - 1, y) not in shape
        bottom = (x, y + 1) not in shape
        front = (x + 1, y) not in shape
        top = (x, y - 1) not in shape
        if back or bottom:
            texels[(x, y)] = "K"
        elif front or top:
            texels[(x, y)] = "z"


def body_paint() -> dict[tuple[int, int], str]:
    paint: dict[tuple[int, int], str] = {}
    for spans in (SKULL, NECK, TORSO, LEGS, FEET):
        paint.update(spans_to_texels(spans, skin_key))
    paint.update(spans_to_texels(HIPS, cloth_key))
    leg_split(paint)
    paint.update({p: k for p, k in ACCENTS.items() if p in paint})
    return paint


def arm_paint() -> dict[tuple[int, int], str]:
    return spans_to_texels({**UPPER_ARM, **WRIST}, skin_key)
