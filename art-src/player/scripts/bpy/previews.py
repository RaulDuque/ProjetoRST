"""Turnaround previews (side, front, back) of the neutral pose.

The side master feeds post/pixelize.py, which produces finish A and B from a
real render exactly as it does today from the vector stand-in.

Validated in Blender 5.2.4 LTS (V1.2-B).
"""

from __future__ import annotations

from pathlib import Path

from camera import use_camera
from config import MODEL_NAME, MODEL_VERSION, RENDERS_DIR
from render import render_still

VIEW_ORDER = ("side", "front", "back")


def master_path(view: str, out_dir: Path = RENDERS_DIR) -> Path:
    return out_dir / f"{MODEL_NAME}_{MODEL_VERSION}_{view}_master.png"


def render_turnaround(out_dir: Path = RENDERS_DIR) -> list[Path]:
    written = []
    for view in VIEW_ORDER:
        use_camera(view)
        written.append(render_still(master_path(view, out_dir)))
    return written
