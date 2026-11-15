"""Paths, version and fixed numbers of v007 (V1.2-G) and of its animation tests.

v007 = v006 with one geometry fix: the pauldron plates had a collapsed end
ring (37 coincident vertices, 36 zero-area faces) that made their Solidify
shell flip under 1e-7 m of input noise. The rig (43 bones, constraints,
weights) is unchanged.

Like the v006 config, this module **shadows** the earlier ``rig_config`` on
``sys.path``: every public v006 value (which itself carries the v005 ones)
is loaded from the v006 file and re-exported, and only the names below are
overridden or added.
"""

from __future__ import annotations

import importlib.util
from pathlib import Path

import params as P  # v004 parameters, read-only

_V006_FILE = Path(__file__).resolve().parents[1] / "swordsman_rig_v006" / "rig_config.py"
_spec = importlib.util.spec_from_file_location("rig_config_v006", _V006_FILE)
if _spec is None or _spec.loader is None:
    raise ImportError(f"cannot load {_V006_FILE}")
V006 = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(V006)
globals().update({k: v for k, v in vars(V006).items() if k.isupper()})
V005 = V006.V005

# --- v007 identity and paths ---------------------------------------------------------
RIG_VERSION = "v007"
SOURCE_VERSION = "v006"
SOURCE_BLEND = Path(V006.BLEND_PATH)  # swordsman_rig_v006.blend (opened read-only)
SOURCE_ARMATURE = V006.ARMATURE_NAME  # "rig_v006"
SOURCE_BONE_COUNT = 43
SOURCE_DEFORM_COUNT = 36
SOURCE_WEIGHTS_SHA256_PREFIX = "a0e2013a0a730ddf"  # v006 build manifest (V1.2-F)
PROTECTED_BLENDS = (*V006.PROTECTED_BLENDS, Path(V006.BLEND_PATH), Path(V006.ANIM_TESTS_BLEND))

BLEND_PATH = Path(P.BLEND_DIR) / f"{V006.RIG_NAME}_{RIG_VERSION}.blend"
ANIM_BLEND = Path(P.BLEND_DIR) / f"{V006.RIG_NAME}_{RIG_VERSION}_animations.blend"
ANIM_TESTS_BLEND = ANIM_BLEND  # name used by shared validators
OUT_DIR = Path(P.RENDERS_DIR) / RIG_VERSION
ANIM_OUT_DIR = Path(P.RENDERS_DIR) / "v1_2g_animation_tests"
ARMATURE_NAME = "rig_v007"
VIZ_COLLECTION = "rig_v007_viz"

# --- pauldron fix --------------------------------------------------------------------
PAULDRON_OBJECTS = ("pauldron", "pauldron_lame")
MERGE_DISTANCE_M = 1e-6  # only exactly coincident vertices (the collapsed ring) merge

# --- cloth shells (found by the same stability test during the animation tests) -------------
# The coat and half-cape Solidify used even thickness: where the cloth folds hard
# (front waist in a deep dodge) the even-thickness factor flipped part of the shell
# (4.2 mm in P09, 83 mm in a frame of the dodge test). Plain thickness is stable; at rest
# it moves the shell by at most 6.6 mm (coat) and 1.2 mm (cape), measured.
EVEN_OFFSET_OFF = ("coat", "half_cape")

# --- shell stability test ---------------------------------------------------------------
# The armature object is nudged by these tiny rigid motions; a stable shell
# follows rigidly (residual ~1e-7 m), an unstable one jumps by millimetres.
PERTURB_TRANSLATION_M = 1e-6
PERTURB_ROTATION_RAD = 1e-6
SHELL_JUMP_TOLERANCE_M = 1e-4  # residual above this = discontinuous shell

# --- animation tests ---------------------------------------------------------------------
PRODUCTION_FPS = 24  # reference sampling (V1.2-G brief); 12 fps is derived from it
DERIVED_FPS = 12
# Gameplay durations, read from the game code (src/core/constants.rs), not changed.
BASIC_ATTACK_TIME_S = 0.28
PLAYER_DODGE_TIME_S = 0.28
HITBOX_ACTIVE_FRACTION = 0.6
PLAYER_WALK_SPEED_PX_S = 190.0
PLAYER_DODGE_SPEED_PX_S = 480.0
WALK_CYCLE_FRAMES = 16  # at 24 fps = 0.667 s; the contract estimate is 8 frames at 12 fps
