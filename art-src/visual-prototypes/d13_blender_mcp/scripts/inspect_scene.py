"""Inspect lab swordsman scene; print JSON-friendly summary."""
from __future__ import annotations

import json
from collections import Counter

import bpy


def main() -> None:
    counts = Counter(o.type for o in bpy.data.objects)
    meshes = sorted(o.name for o in bpy.data.objects if o.type == "MESH")
    mats = sorted(m.name for m in bpy.data.materials)
    arms = []
    for o in bpy.data.objects:
        if o.type != "ARMATURE":
            continue
        arms.append(
            {
                "name": o.name,
                "bones": len(o.data.bones),
                "bone_names": [b.name for b in o.data.bones],
            }
        )
    out = {
        "file": bpy.data.filepath,
        "engine": bpy.context.scene.render.engine,
        "counts": dict(counts),
        "armatures": arms,
        "mesh_count": len(meshes),
        "meshes": meshes,
        "materials": mats,
        "actions": sorted(a.name for a in bpy.data.actions),
    }
    print(json.dumps(out, indent=2))


if __name__ == "__main__":
    main()
