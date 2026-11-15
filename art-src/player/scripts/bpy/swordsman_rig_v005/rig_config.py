"""Paths, version and fixed numbers of the experimental v005 rig (V1.2-E).

v005 is a technical derivation of v004: the v004 .blend is opened read-only,
the rig is added and the result is saved under a new name. Nothing here is
read by the game.

Sides: the model faces +X with +Z up, so its left is +Y. The near side of
the side camera (-Y, the sword arm) is therefore the character's RIGHT and
its bones end in ``.R``; the far side (+Y) ends in ``.L``.
"""

from __future__ import annotations

from pathlib import Path

import params as P  # v004 parameters (landmarks, pose), read-only

RIG_VERSION = "v005"
RIG_NAME = "swordsman_rig"
SOURCE_VERSION = "v004"
SOURCE_COLLECTION = "swordsman_v004"
# Geometry hash of the v004 build (V1.2-D report); a different value means the
# v004 .blend was rebuilt from changed scripts, which the report records.
SOURCE_GEOMETRY_SHA256_PREFIX = "a907cec1da088c4c"

PLAYER_DIR = Path(P.BLEND_DIR).parent
SOURCE_BLEND = Path(P.BLEND_PATH)
BLEND_PATH = Path(P.BLEND_DIR) / f"{RIG_NAME}_{RIG_VERSION}.blend"
ANIM_TESTS_BLEND = Path(P.BLEND_DIR) / f"{RIG_NAME}_{RIG_VERSION}_anim_tests.blend"
OUT_DIR = Path(P.RENDERS_DIR) / RIG_VERSION
ARMATURE_NAME = "rig_v005"
VIZ_COLLECTION = "rig_v005_viz"

# Weight hygiene.
MAX_INFLUENCES = 4
MIN_WEIGHT = 0.01

# Coat secondary motion: share of the thigh swing copied by each panel,
# (forward swing, backward swing). Front panels ride on the raised knee;
# back panels ride on the leg pushed back.
COAT_FOLLOW = {"front": (0.9, 0.35), "back": (0.45, 0.8)}
# Lower back panels also copy part of the knee bend (rear leg kicked up in a run).
COAT_BACK_FOLLOW_SHIN = 0.45
# Lower half-cape: share of the weights that follows the torso under it (the
# coat), so a twisting chest does not drive the cape through the coat.
CAPE_TORSO_SHARE = 0.6  # the cape bones keep 40 %: flare poses use larger angles
# Half-cape: how much of the far upper-arm motion the arm-side chain copies.
# All of its abduction and backward swing (less lets the arm come out through the cape).
CAPE_FOLLOW_ARM = 1.0

# Animation tests (art only; gameplay timings come from the game, not from here).
TEST_FPS = 24
