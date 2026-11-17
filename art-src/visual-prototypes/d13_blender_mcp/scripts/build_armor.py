"""Armor starter: pauldron + pauldron_lame. Future: cuirass/greaves as separate objects."""
from __future__ import annotations

import bpy


def main() -> None:
    parts = [n for n in ("pauldron", "pauldron_lame") if n in bpy.data.objects]
    print("OK build_armor parts", parts)


if __name__ == "__main__":
    main()
