"""Validate rig_v007 has expected control bones for coat/cape/sword."""
from __future__ import annotations

import sys

import bpy

REQUIRED = [
    "root",
    "pelvis",
    "spine_01",
    "chest",
    "neck",
    "head",
    "hand.R",
    "hand.L",
    "sword",
    "cape_back_01",
    "coat_front.R_01",
]


def main() -> None:
    rig = bpy.data.objects.get("rig_v007")
    if rig is None or rig.type != "ARMATURE":
        print("FAIL no rig_v007")
        sys.exit(1)
    names = {b.name for b in rig.data.bones}
    missing = [n for n in REQUIRED if n not in names]
    if missing:
        print("FAIL missing bones", missing)
        sys.exit(1)
    print("OK validate_rig", len(names), "bones")


if __name__ == "__main__":
    main()
