"""Sprite contract numbers used by every script (PIL and Blender).

Mirrors docs/ASSET_CONTRACT.md (the source of truth); change both together.
Values are for finish A at 1 texel per world unit; finish B multiplies by
HD_DENSITY. Preliminary while the finish (D2) is undecided.
"""

from __future__ import annotations

FRAME = 64  # locomotion cell, square
PIVOT = (32.0, 63.0)  # sole line, from the cell's top-left corner
FIGURE_HEIGHT_PX = 52.0  # sole to top of the hair
HEADS = 7.0
HD_DENSITY = 2  # finish B texels per world unit
