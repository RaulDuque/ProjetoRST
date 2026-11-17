"""Sword already modular: blade, blade_readable, guard, grip, pommel, collar."""
from __future__ import annotations

import bpy

PARTS = ["blade", "blade_readable", "sword_guard", "sword_grip", "sword_pommel", "sword_collar"]


def main() -> None:
    found = [n for n in PARTS if n in bpy.data.objects]
    print("OK build_sword parts", found)


if __name__ == "__main__":
    main()
