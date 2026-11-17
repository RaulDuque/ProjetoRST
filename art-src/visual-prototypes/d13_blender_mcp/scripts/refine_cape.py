"""Lab note: cape is `half_cape` + clasp/cord/stud. Reuse; no full rebuild."""
from __future__ import annotations

import bpy


def main() -> None:
    cape = bpy.data.objects.get("half_cape")
    if cape is None:
        print("WARN half_cape missing")
        return
    print("OK refine_cape reuse", cape.name, "verts", len(cape.data.vertices))


if __name__ == "__main__":
    main()
