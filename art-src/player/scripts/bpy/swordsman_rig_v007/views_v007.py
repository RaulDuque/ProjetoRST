"""Extra review views of V1.2-G, registered on top of the v006 ones.

``rig_render_v006`` keeps its views in module dictionaries; this module adds
the shoulder close-ups to them at import time (the v006 file is not
edited), so every render goes through the same camera code, light rig and
materials as the V1.2-F images.
"""

from __future__ import annotations

import rig_render_v006 as R6
from presentation import Shot

EXTRA_VIEWS = {
    # Near side, slightly from the front and above: the plate catches the key light.
    "close_pauldron": Shot((0.0, -0.19, 1.40), -60.0, 12.0, 10.0, 0.32),
    # Grazing view from behind, where a flipped shell shows as a dent in the shading.
    "close_pauldron_back": Shot((-0.02, -0.19, 1.40), -130.0, 12.0, 10.0, 0.32),
}
EXTRA_TRACK = {"close_pauldron": "upper_arm.R", "close_pauldron_back": "upper_arm.R"}

R6.VIEWS.update(EXTRA_VIEWS)
R6.TRACK.update(EXTRA_TRACK)

render_view = R6.render_view
remove_review_cameras = R6.remove_review_cameras
