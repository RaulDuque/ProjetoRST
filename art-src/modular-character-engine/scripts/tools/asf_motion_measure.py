"""ASF pixel_motion measurement on ready-made frames (offline tool).

`pixel_motion.py measure` only reads the output folder of its own video
pipeline (`frames.json` written by `pixel_motion.py frames`); our frames are
approved art or Bevy captures, not a video registration. So this wrapper
imports the tool's module and calls the same functions the `measure` command
uses (`measure_walk`, `legs_picture`) on our frames, with the feet on the
cell's pivot. Agent Sprite Forge is MIT, Copyright (c) 2026 0x0funky.

    python -B asf_motion_measure.py FRAMES_DIR OUT_DIR
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

import numpy as np
from PIL import Image

SCRIPTS = Path.home() / ".claude/plugins/cache/agent-sprite-forge/agent-sprite-forge/0.6.0/skills/video2dsprite/scripts"
sys.path.insert(0, str(SCRIPTS))
import pixel_motion  # noqa: E402

PIVOT = (56, 117)  # feet middle and the last row above the sole line
FPS = 15.0
# The run is a 6-frame loop; the tool finds steps from repetition, so the
# loop is played three times (as the game plays it).
CYCLES = 3


def main() -> None:
    frames_dir, out = Path(sys.argv[1]), Path(sys.argv[2])
    out.mkdir(parents=True, exist_ok=True)
    files = sorted(frames_dir.glob("frame_*.png"))
    frames = [np.array(Image.open(f).convert("RGBA")) for f in files] * CYCLES
    h, w = frames[0].shape[:2]
    found = pixel_motion.measure_walk(frames, PIVOT, w, FPS, facing="right", per_step=3, height=h)
    shown = list(range(len(files)))
    pixel_motion.legs_picture(frames, shown, PIVOT, w, h).save(out / "legs.png")
    keep = {k: found[k] for k in found if not isinstance(found[k], (list, dict)) or k in
            ("footfalls", "stepFrames", "stepByLags", "warnings", "marks", "repeatingFrames", "stepPair")}
    (out / "measure.json").write_text(json.dumps(keep, indent=1, default=str) + "\n")
    print(json.dumps(keep, default=str))


if __name__ == "__main__":
    main()
