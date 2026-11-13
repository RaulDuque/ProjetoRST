"""Generation parameters of the v003 model (recorded in the build report).

Units: metres. Axes as in v002: +X front, -Y near side (sword arm, facing the
side camera), +Z up, sole on z = 0, hair top on z = MODEL_HEIGHT_M.
Landmarks follow docs/ART_DIRECTION.md, section 2 (7 heads of ~0.26 m).
"""

from __future__ import annotations

import sys
from pathlib import Path

BPY_DIR = Path(__file__).resolve().parents[1]
if str(BPY_DIR) not in sys.path:
    sys.path.insert(0, str(BPY_DIR))

from config import BLEND_DIR, HEAD_M, MODEL_HEIGHT_M, RENDERS_DIR  # noqa: E402

MODEL_NAME = "swordsman_base"
MODEL_VERSION = "v003"
SEED = 3  # hair lock jitter; change only together with MODEL_VERSION
OUT_DIR = RENDERS_DIR / MODEL_VERSION
BLEND_PATH = BLEND_DIR / f"{MODEL_NAME}_{MODEL_VERSION}.blend"

H = HEAD_M  # 0.26 m

# Head: centre and ellipsoid radii (x depth, y width, z height).
HEAD_CENTRE = (0.012, 0.0, 1.677)
HEAD_RADII = (0.098, 0.079, 0.117)
NECK_BASE_Z = 1.47

# Shoulder joints (y negative = near/sword side).
SHOULDER_NEAR = (0.0, -0.165, 1.445)
SHOULDER_FAR = (0.0, 0.165, 1.445)

# Sword arm: elbow, wrist and grip centre (hand). Far arm hangs relaxed.
ELBOW_NEAR = (0.085, -0.215, 1.205)
WRIST_NEAR = (0.255, -0.222, 1.035)
ELBOW_FAR = (-0.03, 0.212, 1.19)
WRIST_FAR = (-0.012, 0.222, 0.955)

# Legs: hip joint, knee, ankle (near leg slightly forward).
LEG_NEAR = ((0.03, -0.088, 0.95), (0.05, -0.092, 0.50), (0.035, -0.094, 0.095))
LEG_FAR = ((-0.03, 0.088, 0.95), (-0.035, 0.092, 0.50), (-0.05, 0.094, 0.095))

# Sword: blade direction in XZ (hand-to-tip of the v002 pose, ~3.2 heads).
SWORD_DIRECTION = (1.40, 0.0, -2.90)
BLADE_LENGTH = 0.80
BLADE_WIDTH = 0.040  # at the guard; v002 was 0.034 at the base and 0.012 at the tip
BLADE_THICKNESS = 0.010
BLADE_CURVE = 0.045
GRIP_LENGTH = 0.17

# Coat.
WAIST_Z = 1.13
HEM_BACK_Z = 1.4 * H - 0.004  # mid-calf, as in the proportions table (~0.36)
HEM_FRONT_Z = HEM_BACK_Z + 0.075  # cutaway: front panels slightly shorter
CLOTH_THICKNESS = 0.011
FOLD_COUNT = 9

# Half-cape: far shoulder, diagonal across the back.
CAPE_THICKNESS = 0.009


def report() -> dict[str, object]:
    """Every parameter above, for the build report."""
    return {k: v for k, v in globals().items()
            if k.isupper() and isinstance(v, (int, float, str, tuple))}


__all__ = ["BLEND_PATH", "MODEL_HEIGHT_M", "OUT_DIR"]
