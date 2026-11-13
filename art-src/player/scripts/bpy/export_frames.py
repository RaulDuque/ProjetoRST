"""Exports an animation's frames as numbered PNGs (for the future rig).

Output: renders/frames/<animation_id>/<animation_id>_<NNN>.png, frame
numbers starting at 000 regardless of the timeline start, so the sheet
packer can rely on the contract naming (docs/ASSET_CONTRACT.md).

NOT VALIDATED: written without access to Blender. Unused until a rig and
actions exist (V1.4+).
"""

from __future__ import annotations

import re
from pathlib import Path

import bpy

from camera import use_camera
from config import RENDERS_DIR
from render import render_still

ANIMATION_ID = re.compile(r"^[a-z][a-z0-9_]*$")


def export_animation(animation_id: str, frame_start: int, frame_end: int, view: str = "side",
                     out_root: Path = RENDERS_DIR / "frames") -> list[Path]:
    if not ANIMATION_ID.match(animation_id):
        raise ValueError(f"animation id must be snake_case: {animation_id!r}")
    if frame_end < frame_start:
        raise ValueError("frame_end must be >= frame_start")
    scene = bpy.context.scene
    use_camera(view)
    written = []
    for index, frame in enumerate(range(frame_start, frame_end + 1)):
        scene.frame_set(frame)
        path = out_root / animation_id / f"{animation_id}_{index:03d}.png"
        written.append(render_still(path))
    return written
