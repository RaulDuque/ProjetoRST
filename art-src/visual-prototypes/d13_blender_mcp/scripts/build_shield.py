"""Shield not present on v007 lab yet — placeholder for Offhand visual slot."""
from __future__ import annotations

import bpy


def main() -> None:
    if "shield" in bpy.data.objects:
        print("OK shield exists")
    else:
        print("PARTIAL shield slot empty")


if __name__ == "__main__":
    main()
