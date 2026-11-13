"""Shared settings of the Blender pipeline (scale, frame, paths, colours).

Units: Blender metres. The model faces +X, its sole is on z = 0 and the
side camera looks along +Y, so screen-right is the character's front (the
sprite contract draws every animation facing right; Bevy mirrors with flip_x).

Validated in Blender 5.2.4 LTS (V1.2-B). See art-src/README.md.
"""

from __future__ import annotations

import sys
from pathlib import Path

SCRIPTS_DIR = Path(__file__).resolve().parent
PLAYER_DIR = SCRIPTS_DIR.parents[1]
POST_DIR = SCRIPTS_DIR.parent / "post"
BLEND_DIR = PLAYER_DIR / "blender"
RENDERS_DIR = PLAYER_DIR / "renders"

# The palette lives in one place, shared with the PIL scripts.
if str(POST_DIR) not in sys.path:
    sys.path.insert(0, str(POST_DIR))
from contract import FIGURE_HEIGHT_PX, FRAME, HEADS, PIVOT  # noqa: E402
from palette import CHARACTER  # noqa: E402

MODEL_NAME = "swordsman_base"
# v001: first real run (V1.2-B) of the unchanged V1.1 geometry.
# v002: open coat with lining, open collar, half-cape shell, fringe, eyes.
MODEL_VERSION = "v002"
COLLECTION_NAME = "swordsman_base"

# Proportions: ~7 heads (landmarks in docs/ART_DIRECTION.md, section 2).
MODEL_HEIGHT_M = 1.82
HEAD_M = MODEL_HEIGHT_M / HEADS

# Sprite contract (docs/ASSET_CONTRACT.md, mirrored in post/contract.py).
FRAME_PX = FRAME
PIVOT_PX = PIVOT  # from the frame's top-left corner; sole line
METRES_PER_PX = MODEL_HEIGHT_M / FIGURE_HEIGHT_PX
SUPERSAMPLE = 16  # master renders are FRAME_PX * SUPERSAMPLE wide

CAMERA_DISTANCE_M = 10.0


def srgb_to_linear(channel: float) -> float:
    if channel <= 0.04045:
        return channel / 12.92
    return ((channel + 0.055) / 1.055) ** 2.4


def color(name: str) -> tuple[float, float, float, float]:
    """Palette colour as linear RGBA, the space Blender's colour inputs use."""
    value = CHARACTER[name].lstrip("#")
    rgb = [int(value[i : i + 2], 16) / 255.0 for i in (0, 2, 4)]
    return (srgb_to_linear(rgb[0]), srgb_to_linear(rgb[1]), srgb_to_linear(rgb[2]), 1.0)
