"""Helmet not present on v007 lab yet — placeholder documenting slot Helmet."""
from __future__ import annotations

import bpy


def main() -> None:
    if "helmet" in bpy.data.objects:
        print("OK helmet exists")
    else:
        print("PARTIAL helmet slot empty — do not invent Castlevania look; await art brief")


if __name__ == "__main__":
    main()
