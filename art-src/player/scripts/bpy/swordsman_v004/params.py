"""Generation parameters of the v004 model (recorded in the build report).

v004 refines v003 locally, so the landmarks, the pose and the sword are the
v003 values; only the version, the output paths and the new refinement
parameters change. Modules reused from ``swordsman_v003`` (``mesh_tools``,
``sword``, ``staging``, ``clothing.build_collar``) do ``import params``; the
v004 entry script puts this package first on ``sys.path``, so they read the
values below. The v003 build never sees this file.

Units: metres. Axes as in v003: +X front, -Y near side (sword arm, facing the
side camera), +Z up, sole on z = 0, hair top on z = MODEL_HEIGHT_M.
"""

from __future__ import annotations

import sys
from pathlib import Path

BPY_DIR = Path(__file__).resolve().parents[1]
if str(BPY_DIR) not in sys.path:
    sys.path.insert(0, str(BPY_DIR))

from config import BLEND_DIR, HEAD_M, MODEL_HEIGHT_M, RENDERS_DIR  # noqa: E402

MODEL_NAME = "swordsman_base"
MODEL_VERSION = "v004"
COLLECTION_NAME = "swordsman_v004"
SEED = 4  # hair lock jitter; change only together with MODEL_VERSION
OUT_DIR = RENDERS_DIR / MODEL_VERSION
BLEND_PATH = BLEND_DIR / f"{MODEL_NAME}_{MODEL_VERSION}.blend"

H = HEAD_M  # 0.26 m

# --- unchanged from v003 (pose, landmarks, sword) ---------------------------

HEAD_CENTRE = (0.012, 0.0, 1.677)
HEAD_RADII = (0.098, 0.079, 0.117)
NECK_BASE_Z = 1.47

SHOULDER_NEAR = (0.0, -0.165, 1.445)
SHOULDER_FAR = (0.0, 0.165, 1.445)
ELBOW_NEAR = (0.085, -0.215, 1.205)
WRIST_NEAR = (0.255, -0.222, 1.035)
ELBOW_FAR = (-0.03, 0.212, 1.19)
WRIST_FAR = (-0.012, 0.222, 0.955)

LEG_NEAR = ((0.03, -0.088, 0.95), (0.05, -0.092, 0.50), (0.035, -0.094, 0.095))
LEG_FAR = ((-0.03, 0.088, 0.95), (-0.035, 0.092, 0.50), (-0.05, 0.094, 0.095))

SWORD_DIRECTION = (1.40, 0.0, -2.90)
BLADE_LENGTH = 0.80
BLADE_WIDTH = 0.040
BLADE_THICKNESS = 0.010
BLADE_CURVE = 0.045
GRIP_LENGTH = 0.17
READABLE_BLADE_SCALE = 1.7  # visual-only variant; reach, hitbox and damage untouched

WAIST_Z = 1.13
HEM_BACK_Z = 1.4 * H - 0.004
HEM_FRONT_Z = HEM_BACK_Z + 0.075
CLOTH_THICKNESS = 0.011
CAPE_THICKNESS = 0.009

# --- v004 refinements -------------------------------------------------------

# Head mesh resolution (v003: 42 x 56). Finer rows carry the nose and lids.
HEAD_ROWS = 72
HEAD_COLUMNS = 88

# Waistcoat: ends in two points in front, below the belt; trousers below.
VEST_HEM_SIDE_Z = 1.085
VEST_HEM_POINT_Z = 1.035
VEST_PROUD = 0.0035  # the vest stands this much off the trousers/shirt

# Coat folds: (count, amplitude m, phase, drift with height) summed.
FOLDS = ((5, 0.011, 0.4, 1.1), (8, 0.007, 1.9, -0.7), (13, 0.0035, 0.2, 2.3))
COAT_EDGE_FACING_COLUMNS = 2  # wine facing visible along each front edge

# Hair: number of locks per group and taper of the tips (v003 ended at 8 %).
HAIR_TIP_WIDTH = 0.025


def report() -> dict[str, object]:
    """Every parameter above, for the build report."""
    return {k: v for k, v in globals().items()
            if k.isupper() and isinstance(v, (int, float, str, tuple))}


__all__ = ["BLEND_PATH", "MODEL_HEIGHT_M", "OUT_DIR"]
