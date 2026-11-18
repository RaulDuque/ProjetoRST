"""Render the existing v007 walk cycle. Does not save the .blend.

The published sprite module has ``run``, not this walk. The walk lives in
``swordsman_rig_v007/animations.py`` and is the cycle with planted feet.
Output stays under ``art-src/n3-proof/blender/walk/``, away from ``assets/``.
"""

from __future__ import annotations

import sys
from pathlib import Path

import bpy

SCRIPTS = Path(__file__).resolve().parents[2] / "player" / "scripts" / "bpy"
for entry in (
    SCRIPTS,
    SCRIPTS / "swordsman_v003",
    SCRIPTS / "swordsman_v004",
    SCRIPTS / "swordsman_rig_v005",
    SCRIPTS / "swordsman_rig_v006",
    SCRIPTS / "swordsman_rig_v007",
):
    if str(entry) in sys.path:
        sys.path.remove(str(entry))
    sys.path.insert(0, str(entry))

import animations as AN  # noqa: E402
import poses as PS  # noqa: E402
import rig_config as C  # noqa: E402
import rig_render as RR  # noqa: E402
from surfaces import use_toon  # noqa: E402

OUT = Path(__file__).resolve().parents[1] / "blender" / "walk"
SWORD = ("blade", "blade_readable", "sword_collar", "sword_guard", "sword_grip", "sword_pommel")


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    arm = bpy.data.objects[C.ARMATURE_NAME]
    body = PS.all_body()
    hidden = {}
    for name in SWORD:
        obj = bpy.data.objects[name]
        hidden[name] = obj.hide_render
        obj.hide_render = True
    PS.reset(arm)
    count = AN.CLIPS["walk"].frames
    for index in range(count):
        PS.apply(arm, AN.clip_pose(arm, "walk", index), body)
        bpy.context.view_layer.update()
        use_toon(True)
        path = OUT / f"walk_W_flat_{index:03d}.png"
        RR.render_contract_side(path)
        use_toon(False)
        print(f"[n3.1 walk] {index + 1}/{count} {path}")
    PS.reset(arm)
    for name, flag in hidden.items():
        bpy.data.objects[name].hide_render = flag
    print("[n3.1 walk] done, blend not saved")


if __name__ == "__main__":
    main()
