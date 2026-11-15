"""Review cameras of v006 and of the v005 x v006 comparison (V1.2-F).

The light rig, world, engine, materials and contract cameras are the v005
ones (``rig_render`` from the v005 package, imported unchanged). What
changes is the set of review views:

- ``side``: the v005 orthographic side view (game view), unchanged;
- ``three_quarter``: near-side 3/4 with the figure filling the frame
  (v005 used 50 mm at 7 m, the figure was ~1/5 of the frame);
- ``far_back``: 3/4 from behind the FAR side (+Y), where the half-cape and
  the left arm are; the side view hides that whole side;
- closes for the problem areas (cape, coat, collar, ankle, knees).

Before (v005) and after (v006) images are rendered in the same Blender
process with these same cameras, lights and materials.
"""

from __future__ import annotations

from pathlib import Path

import bpy

import rig_config as C
import rig_render as RR  # v005 module, unchanged
from presentation import Shot, aim_camera
from render import render_still
from staging import turn_rig

LENS, DIST = C.THREE_QUARTER_LENS_MM, C.THREE_QUARTER_DISTANCE_M
VIEWS = {
    "side": RR.REVIEW["side"],
    "three_quarter": Shot((0.15, 0.0, 1.0), -40.0, 8.0, DIST, LENS, perspective=True),
    "far_back": Shot((0.0, 0.0, 1.0), 130.0, 10.0, DIST, LENS, perspective=True),
    "close_cape": Shot((-0.05, 0.12, 1.25), 125.0, 8.0, 10.0, 1.0),
    "close_cape_front": Shot((0.05, 0.12, 1.25), 60.0, 6.0, 10.0, 1.0),
    "close_coat_side": Shot((0.15, 0.0, 0.75), -90.0, 0.0, 10.0, 1.35),
    "close_coat_3q": Shot((0.15, 0.0, 0.75), -35.0, 6.0, 10.0, 1.35),
    "close_collar": Shot((0.0, 0.0, 1.6), -60.0, 5.0, 10.0, 0.55),
    "close_collar_back": Shot((0.0, 0.0, 1.6), 150.0, 8.0, 10.0, 0.55),
    "close_ankle": Shot((0.1, 0.0, 0.15), -70.0, 8.0, 10.0, 0.75),
    # Deformation-test closes, same numbers as the v005 build (TEST_SHOTS), for T01-T11.
    "test_shoulder": Shot((0.05, -0.05, 1.3), -35.0, 8.0, 10.0, 1.25),
    "test_torso": Shot((0.0, 0.0, 1.2), -20.0, 5.0, 10.0, 1.3),
    "test_knees": Shot((0.1, 0.0, 0.6), -90.0, 0.0, 10.0, 1.5),
    "test_cape": Shot((-0.05, 0.1, 1.3), 140.0, 10.0, 10.0, 1.3),
}


# Close-ups follow a bone: the shot target moves by the bone's posed - rest
# offset, so an airborne or collapsed pose keeps the area in frame (v1 of the
# V1.2-F closes used fixed targets and lost the cape in P05 and the ankle in P13).
TRACK = {"close_cape": "clavicle.L", "close_cape_front": "clavicle.L", "close_coat_side": "pelvis",
         "close_coat_3q": "pelvis", "close_collar": "neck", "close_collar_back": "neck", "close_ankle": "foot.R"}


def _tracked(view: str) -> Shot:
    shot = VIEWS[view]
    bone = TRACK.get(view)
    arm = bpy.data.objects.get(C.ARMATURE_NAME)
    if bone is None or arm is None:
        return shot
    offset = arm.matrix_world @ arm.pose.bones[bone].head - arm.matrix_world @ arm.data.bones[bone].head_local
    target = tuple(t + o for t, o in zip(shot.target, offset))
    return Shot(target, shot.azimuth, shot.elevation, shot.distance, shot.size, shot.perspective)


def render_view(view: str, path: Path, size: int = 512) -> Path:
    shot = _tracked(view)
    RR.set_resolution(size, size)
    bpy.context.scene.camera = aim_camera(f"cam_v006_{view}", shot)
    turn_rig(shot.azimuth)
    return render_still(path)


def remove_review_cameras() -> None:
    """The review cameras are build helpers; the saved file keeps only the v005 set."""
    for name in [o.name for o in bpy.data.objects if o.name.startswith("cam_v006_")]:
        obj = bpy.data.objects[name]
        data = obj.data
        bpy.data.objects.remove(obj, do_unlink=True)
        bpy.data.cameras.remove(data)
