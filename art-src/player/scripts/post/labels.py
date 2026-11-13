"""Font lookup for the comparison sheets (Portuguese labels need accents)."""

from __future__ import annotations

from pathlib import Path

from PIL import ImageFont

# Common Linux locations; the first one found is used. Pillow's built-in font
# is the last resort and may not render accented characters.
CANDIDATES = (
    Path("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"),
    Path("/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf"),
    Path("/usr/share/fonts/TTF/DejaVuSans.ttf"),
)


def font(size: int) -> ImageFont.ImageFont | ImageFont.FreeTypeFont:
    for path in CANDIDATES:
        if path.is_file():
            return ImageFont.truetype(str(path), size)
    return ImageFont.load_default(size=size)
