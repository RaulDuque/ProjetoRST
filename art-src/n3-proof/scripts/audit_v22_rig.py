"""Read-only inventory of swordsman_rig_v007.blend. Never saves the file."""

from __future__ import annotations

import json
from pathlib import Path

import bpy

OUT = Path(__file__).resolve().parents[1] / "blender" / "audit_v22.json"


def main() -> None:
    arm = bpy.data.objects.get("rig_v006") or next(
        (obj for obj in bpy.data.objects if obj.type == "ARMATURE"), None
    )
    bones = []
    if arm is not None:
        bones = [
            {"name": bone.name, "parent": bone.parent.name if bone.parent else None}
            for bone in arm.data.bones
        ]
    meshes = []
    for obj in bpy.data.objects:
        if obj.type != "MESH":
            continue
        meshes.append(
            {
                "name": obj.name,
                "verts": len(obj.data.vertices),
                "hide_render": obj.hide_render,
                "materials": [slot.name for slot in obj.material_slots if slot.material],
            }
        )
    report = {
        "blend": bpy.data.filepath,
        "blender": bpy.app.version_string,
        "saved": False,
        "armature": None if arm is None else arm.name,
        "bone_count": len(bones),
        "bones": bones,
        "mesh_count": len(meshes),
        "meshes": meshes,
        "materials": sorted(mat.name for mat in bpy.data.materials),
        "cameras": [
            {"name": obj.name, "type": obj.data.type, "ortho_scale": obj.data.ortho_scale}
            for obj in bpy.data.objects
            if obj.type == "CAMERA"
        ],
        "actions": sorted(action.name for action in bpy.data.actions),
        "lights": [obj.name for obj in bpy.data.objects if obj.type == "LIGHT"],
    }
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(f"[n3.1 audit] bones={report['bone_count']} meshes={report['mesh_count']} -> {OUT}")


if __name__ == "__main__":
    main()
