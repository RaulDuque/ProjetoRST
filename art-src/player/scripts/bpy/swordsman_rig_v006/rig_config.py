"""Paths, version and fixed numbers of the experimental v006 rig (V1.2-F).

v006 is a stabilisation of v005: the v005 .blend is opened read-only, a
few targeted corrections are applied (half-cape, coat, collar, ankle) and
the result is saved under a new name. Nothing here is read by the game.

This module **shadows** the v005 ``rig_config`` on ``sys.path`` (the same
pattern v004 uses over v003): the unchanged v005 modules (``poses``,
``skinning``, ``rig_validate``, ``rig_render``...) import ``rig_config`` and
get this one. Every v005 value is loaded from the v005 file itself and only
the names below are overridden or added, so the two versions cannot drift
apart silently.
"""

from __future__ import annotations

import importlib.util
from pathlib import Path

import params as P  # v004 parameters (landmarks, pose), read-only

_V005_FILE = Path(__file__).resolve().parents[1] / "swordsman_rig_v005" / "rig_config.py"
_spec = importlib.util.spec_from_file_location("rig_config_v005", _V005_FILE)
if _spec is None or _spec.loader is None:
    raise ImportError(f"cannot load {_V005_FILE}")
V005 = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(V005)

# --- unchanged from v005 (re-exported for the v005 modules) ---------------------------
RIG_NAME = V005.RIG_NAME
SOURCE_COLLECTION = V005.SOURCE_COLLECTION  # the meshes keep their v004 collection
PLAYER_DIR = V005.PLAYER_DIR
MAX_INFLUENCES = V005.MAX_INFLUENCES
MIN_WEIGHT = V005.MIN_WEIGHT
COAT_FOLLOW = V005.COAT_FOLLOW
COAT_BACK_FOLLOW_SHIN = V005.COAT_BACK_FOLLOW_SHIN
CAPE_TORSO_SHARE = V005.CAPE_TORSO_SHARE
CAPE_FOLLOW_ARM = V005.CAPE_FOLLOW_ARM
TEST_FPS = V005.TEST_FPS  # same test timing as v005, for a direct comparison

# --- v006 identity and paths ----------------------------------------------------------
RIG_VERSION = "v006"
SOURCE_VERSION = "v005"
SOURCE_BLEND = Path(V005.BLEND_PATH)  # swordsman_rig_v005.blend (opened read-only)
SOURCE_ARMATURE = V005.ARMATURE_NAME  # "rig_v005"
SOURCE_BONE_COUNT = 42
SOURCE_DEFORM_COUNT = 35
# v004 geometry (V1.2-D hash); in v005 it is measured with the Armature modifier off.
SOURCE_GEOMETRY_SHA256_PREFIX = V005.SOURCE_GEOMETRY_SHA256_PREFIX
PROTECTED_BLENDS = tuple(Path(P.BLEND_DIR) / n for n in (
    "swordsman_base_v001.blend", "swordsman_base_v002.blend", "swordsman_base_v003.blend",
    "swordsman_base_v004.blend", "swordsman_rig_v005.blend", "swordsman_rig_v005_anim_tests.blend"))

BLEND_PATH = Path(P.BLEND_DIR) / f"{RIG_NAME}_{RIG_VERSION}.blend"
ANIM_TESTS_BLEND = Path(P.BLEND_DIR) / f"{RIG_NAME}_{RIG_VERSION}_anim_tests.blend"
OUT_DIR = Path(P.RENDERS_DIR) / RIG_VERSION
ARMATURE_NAME = "rig_v006"
VIZ_COLLECTION = "rig_v006_viz"

# --- corrections (each one is justified in RIG_VALIDATION_V006.md) ----------------------

# 1. Half-cape over the far arm. The v005 chain pivoted at the top of the cape
#    and copied the arm's LOCAL rotation from a bone with a different rest
#    orientation, so the cape lagged behind the arm (sleeve through the cape).
#    v006 puts the chain's pivot on the shoulder joint with the rest frame of
#    upper_arm.L: a copied local rotation is then the same world rotation.
#    Share of each component of the arm swing that the cape copies:
#    - back: all of it (the arm would otherwise come out through the cape);
#    - forward: none (measured: 0.35 already drives the back of the cape
#      through the coat in walk, jump rise, guard and magic cast);
#    - out (abduction): all of it, it lifts the cape away from the torso;
#    - in (adduction, e.g. an arm raised forward and pulled across): none,
#      it would drag the cape into the torso (T11 in v005).
CAPE_ARM_FOLLOW = {"back": 1.0, "forward": 0.0, "out": 1.0, "in": 0.0}
# Upper cape over the shoulder cap: share that rides on the arm chain (the
#    deltoid rises into a cape top that only followed the clavicle).
CAPE_TOP_ARM_SHARE = 0.55
# Chest twist toward the sword side: share undone on the arm-side cape chain
#    (the cape hangs from the chest, the coat under it from the lower spine).
CAPE_CHEST_TWIST_COUNTER = 0.35

# 2. Coat. Back panels: forward thigh swing is copied only up to
#    COAT_BACK_FWD_PEAK degrees, then handed back so that at COAT_BACK_FWD_ZERO
#    the panel hangs from the hips again (in a squat or a collapse the buttock
#    moves backward, into a panel that kept following the thigh forward).
COAT_BACK_FWD_PEAK = 40.0
COAT_BACK_FWD_ZERO = 100.0
# Front waist: the skirt bones take over from this height (v005: 1.06 m with a
#    0.14 m ramp left 60 % of the waist on the pelvis while the thigh rose into it).
COAT_FRONT_SKIRT_TOP_Z = 1.10
COAT_FRONT_SKIRT_RAMP = 0.10
# Front panels: outward flare (degrees at 90 degrees of thigh flexion) so the side
#    of the panel clears the outer thigh and knee, which share its plane.
COAT_FRONT_FLARE_DEG = 8.0
# Lower front panels: share of the knee bend copied (the hem drapes over the knee).
COAT_FRONT_FOLLOW_SHIN = 0.0

# 3. Collar: a ``collar_back`` bone (child of chest, neck rest frame) copies part
#    of the neck and head rotation; the back and the sides of the collar ride on it.
COLLAR_FOLLOW_NECK = 0.8
COLLAR_FOLLOW_HEAD = 0.6
# Angle (rad from the front) where the collar starts riding on collar_back, and the ramp.
COLLAR_SIDE_START = 0.2
COLLAR_SIDE_RAMP = 0.8

# 4. Volume: dual-quaternion (preserve volume) skinning on these meshes (prefixes):
#    boot foot shells (ankle) and the trousers (hip flexion).
PRESERVE_VOLUME_EXTRA = ("boot_foot_", "boot_sole_", "boot_welt_", "boot_heel_", "boot_toe_seam_",
                         "torso_trousers")

# Review cameras: the v005 3/4 view (50 mm at 7 m) left the figure ~1/5 of the frame.
THREE_QUARTER_LENS_MM = 85.0
THREE_QUARTER_DISTANCE_M = 7.0
