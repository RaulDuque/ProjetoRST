"""N3.6: decode the owner's run video and key the selected frames.

Source (read only): `Md-usario/assets-user/prota/base/trimmed_fd4c76cf-...mp4`,
496x864, 24 fps, 190 frames (H.264), green screen, the protagonist facing
right: standing in frame 0, then running in place with a fixed camera.

1. Decodes every frame with the system GStreamer (`gst-launch-1.0`, not a
   project dependency) into `cache/video/` (ignored by git).
2. For the selected frames, keys the green screen and writes a full-
   resolution RGBA crop to `frames/video_NNN.png` (the intermediate the
   sprites are reduced from). Every frame uses the same crop window, so
   their coordinates stay comparable.

Key: a pixel is background when green exceeds red and blue by more than
`KEY_MARGIN`; the figure mask is eroded by one pixel (drops the green-mixed
rim) and green spill is clamped to max(red, blue).

    python3 art-src/n3-6-animation-proof/scripts/extract_video.py
"""

from __future__ import annotations

import subprocess
from pathlib import Path

from PIL import Image, ImageFilter

ROOT = Path(__file__).resolve().parents[3]
VIDEO = ROOT / "Md-usario/assets-user/prota/base/trimmed_fd4c76cf-0416-4387-b451-da4cc90a2061.mp4"
HERE = ROOT / "art-src/n3-6-animation-proof"
CACHE = HERE / "cache/video"
FRAMES = HERE / "frames"
# Same window for every frame: full width, from above the hair to below the soles.
CROP = (0, 150, 496, 702)
KEY_MARGIN = 30
# Frame 0 (standing, the scale reference) and one full run cycle, 48..59.
SELECTED = [0, *range(48, 60)]


def decode() -> None:
    if CACHE.is_dir() and any(CACHE.glob("f_*.png")):
        return
    CACHE.mkdir(parents=True, exist_ok=True)
    subprocess.run(
        [
            "gst-launch-1.0", "-q", "filesrc", f"location={VIDEO}", "!", "decodebin", "!",
            "videoconvert", "!", "pngenc", "!", "multifilesink", f"location={CACHE}/f_%03d.png",
        ],
        check=True,
    )


def key(frame: Image.Image) -> Image.Image:
    rgb = frame.convert("RGB").crop(CROP)
    px = rgb.load()
    mask = Image.new("L", rgb.size, 0)
    mp = mask.load()
    for y in range(rgb.height):
        for x in range(rgb.width):
            r, g, b = px[x, y]
            if not (g > r + KEY_MARGIN and g > b + KEY_MARGIN):
                mp[x, y] = 255
            px[x, y] = (r, min(g, max(r, b)), b)
    mask = mask.filter(ImageFilter.MinFilter(3))
    out = rgb.convert("RGBA")
    out.putalpha(mask)
    return out


def main() -> None:
    decode()
    FRAMES.mkdir(parents=True, exist_ok=True)
    count = len(list(CACHE.glob("f_*.png")))
    for index in SELECTED:
        key(Image.open(CACHE / f"f_{index:03d}.png")).save(FRAMES / f"video_{index:03d}.png")
    print(f"decoded {count} frames, keyed {len(SELECTED)} into {FRAMES.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
