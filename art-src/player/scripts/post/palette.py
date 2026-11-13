"""Preliminary palette of the swordsman (concept B) and of the current room.

Single source of the colours used by the experimental scripts. Values are
proposals for V1.1 (see docs/ART_DIRECTION.md) and may change after review.
"""

from __future__ import annotations


def hex_rgb(value: str) -> tuple[int, int, int]:
    value = value.lstrip("#")
    return (int(value[0:2], 16), int(value[2:4], 16), int(value[4:6], 16))


# Character palette (name -> hex). Order matters only for documentation.
CHARACTER = {
    "outline_dark": "#0d0b12",
    "rim": "#4a5570",
    "coat": "#1d2434",
    "coat_shadow": "#141a26",
    "coat_light": "#2c3650",
    "lining_wine": "#6e1f30",
    "cape_wine": "#571827",
    "cape_shadow": "#3b101b",
    "trousers": "#262a36",
    "boots": "#17161d",
    "boot_cuff": "#3a3446",
    "silver": "#b9c3cf",
    "silver_light": "#e6edf3",
    "silver_dark": "#7d8794",
    "skin": "#dcc6b4",
    "skin_shadow": "#b39a8a",
    "hair": "#3a3e4b",
    "hair_light": "#5d6476",
    "glove": "#2c2329",
    "amber": "#d18f2f",
    "amber_dark": "#8f5a1c",
}

CHARACTER_RGB = {name: hex_rgb(value) for name, value in CHARACTER.items()}


def srgb_float(r: float, g: float, b: float) -> tuple[int, int, int]:
    """Converts the 0..1 sRGB triples used by the game code and RON files."""
    return (round(r * 255), round(g * 255), round(b * 255))


# Colours of the current placeholder scene. Sources:
# - background: assets/maps/entry_hall.ron (`background`), read at runtime by
#   camera_compare.py; this value is only a fallback.
# - platforms: src/world/maps.rs (PLATFORM_COLOR, ONE_WAY_COLOR).
# - player idle box: src/player/animation.rs (state_color, Idle).
SCENE_BACKGROUND_FALLBACK = srgb_float(0.07, 0.06, 0.1)
SCENE_PLATFORM = srgb_float(0.32, 0.29, 0.36)
SCENE_ONE_WAY = srgb_float(0.45, 0.36, 0.25)
SCENE_PLAYER_BOX = srgb_float(0.75, 0.2, 0.25)
# - Ember Bolt square: src/magic/casting.rs (element_color, Element::Fire).
SCENE_FIRE_BOLT = srgb_float(1.0, 0.45, 0.1)
