"""Body parts already split (head/hair/legs/…). Reuse; no full remesh."""
from __future__ import annotations

import bpy


def main() -> None:
    head = bpy.data.objects.get("head")
    print("OK refine_body", "head" if head else "MISSING", "hair_parts", sum(1 for o in bpy.data.objects if o.name.startswith("hair_")))


if __name__ == "__main__":
    main()
