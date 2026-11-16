"""Inventory cameras, actions, NLA on the D5 lab blend. Run via blender --python."""
import json
from pathlib import Path

import bpy

out = Path("art-src/visual-prototypes/d5_corrida_modular/rig/blender_actions.json")
actions = []
for action in bpy.data.actions:
    actions.append(
        {
            "name": action.name,
            "frame_range": [float(action.frame_range[0]), float(action.frame_range[1])],
        }
    )

arm = next(o for o in bpy.data.objects if o.type == "ARMATURE")
nla = []
if arm.animation_data:
    for track in arm.animation_data.nla_tracks:
        strips = [
            {
                "name": s.name,
                "action": s.action.name if s.action else None,
                "frame_start": s.frame_start,
                "frame_end": s.frame_end,
            }
            for s in track.strips
        ]
        nla.append({"track": track.name, "strips": strips})

payload = {
    "actions": actions,
    "armature": arm.name,
    "active_action": arm.animation_data.action.name
    if arm.animation_data and arm.animation_data.action
    else None,
    "nla": nla,
    "cameras": [o.name for o in bpy.data.objects if o.type == "CAMERA"],
    "lights": [o.name for o in bpy.data.objects if o.type == "LIGHT"],
    "engine": bpy.context.scene.render.engine,
    "resolution": [
        bpy.context.scene.render.resolution_x,
        bpy.context.scene.render.resolution_y,
    ],
}
out.write_text(json.dumps(payload, indent=2))
print(json.dumps(payload, indent=2))
