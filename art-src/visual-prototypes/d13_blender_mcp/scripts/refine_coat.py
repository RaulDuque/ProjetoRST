"""Lab note: coat already exists as mesh `coat` (+ collar/sleeves). Do not recreate.

Optional future: thicken rim loops / separate lining mesh. Idempotent no-op for now.
"""
from __future__ import annotations

import bpy


def main() -> None:
    coat = bpy.data.objects.get("coat")
    if coat is None:
        print("WARN coat missing")
        return
    print("OK refine_coat reuse", coat.name, "verts", len(coat.data.vertices))


if __name__ == "__main__":
    main()
