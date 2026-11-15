"""Image resizing that survives a broken nearest-neighbour path in Pillow.

V1.2-D: the system Pillow (python3-pil 12.1.1-2ubuntu1.5, Python 3.14.4)
raises ``SystemError: ... returned NULL`` on *every* ``Image.resize`` with
``NEAREST`` (any size, any mode), while the other filters and
``Image.transform`` keep working. ``resize`` tries Pillow first and, only
for that error, samples the same pixels through an affine transform
(nearest neighbour at pixel centres), so the scripts behave as before on a
healthy Pillow.
"""

from __future__ import annotations

from PIL import Image


def resize(image: Image.Image, size: tuple[int, int], method: Image.Resampling) -> Image.Image:
    try:
        return image.resize(size, method)
    except SystemError:
        if method != Image.Resampling.NEAREST:
            raise
    sx, sy = image.size[0] / size[0], image.size[1] / size[1]
    return image.transform(size, Image.Transform.AFFINE, (sx, 0.0, 0.0, 0.0, sy, 0.0), Image.Resampling.NEAREST)
