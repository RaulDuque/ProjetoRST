"""D15 pilot: export independent sword from swordsman_rig_v007 and assemble
an experimental scene (base + sword equipped / unequipped).

Opens the original rig READ-ONLY. Never overwrites art-src/player/blender/*.

Usage (from repo root):
  blender --background --factory-startup \\
    art-src/player/blender/swordsman_rig_v007.blend \\
    --python art-src/visual-prototypes/d15_equip_modular/scripts/export_sword_and_pilot.py
"""
from __future__ import annotations

import json
import re
from datetime import datetime, timezone
from pathlib import Path

import bpy
from mathutils import Matrix, Vector

ROOT = Path(__file__).resolve().parents[4]
PLAYER_BLEND = ROOT / "art-src" / "player" / "blender" / "swordsman_rig_v007.blend"
OUT_EQUIP = ROOT / "assets-user" / "equip" / "arma" / "espada_inicial"
OUT_BASE = ROOT / "assets-user" / "protagonista" / "base"
OUT_PILOT = ROOT / "art-src" / "visual-prototypes" / "d15_equip_modular" / "pilot"
OUT_RENDERS = ROOT / "art-src" / "visual-prototypes" / "d15_equip_modular" / "renders"
OUT_SOCKETS = ROOT / "art-src" / "visual-prototypes" / "d15_equip_modular" / "sockets"

SWORD_NAME_RE = re.compile(
    r"^(blade|blade_readable|sword_|sabre|guard|grip|pommel|collar)",
    re.IGNORECASE,
)
# Known object name fragments from swordsman builds
SWORD_FRAGMENTS = (
    "blade",
    "sword_guard",
    "sword_collar",
    "sword_grip",
    "sword_pommel",
    "sword_hand",  # keep hand with body, not sword export
)


def stamp() -> str:
    return datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")


def is_sword_mesh(obj: bpy.types.Object) -> bool:
    if obj.type != "MESH":
        return False
    name = obj.name.lower()
    if "hand" in name and "sword" not in name:
        return False
    if name.startswith("blade") or name.startswith("sword_"):
        return True
    if "sabre" in name or "pommel" in name:
        return True
    return False


def find_armature() -> bpy.types.Object | None:
    for obj in bpy.data.objects:
        if obj.type == "ARMATURE":
            return obj
    return None


def bone_world_matrix(arm: bpy.types.Object, bone_name: str) -> Matrix | None:
    bone = arm.pose.bones.get(bone_name)
    if bone is None:
        return None
    return arm.matrix_world @ bone.matrix


def export_selected_glb(path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    bpy.ops.export_scene.gltf(
        filepath=str(path),
        export_format="GLB",
        use_selection=True,
        export_apply=True,
        export_yup=True,
    )


def clear_selection() -> None:
    bpy.ops.object.select_all(action="DESELECT")


def dims_of(objs: list[bpy.types.Object]) -> dict:
    if not objs:
        return {"count": 0}
    min_c = Vector((1e9, 1e9, 1e9))
    max_c = Vector((-1e9, -1e9, -1e9))
    verts = faces = 0
    for o in objs:
        for corner in o.bound_box:
            w = o.matrix_world @ Vector(corner)
            min_c = Vector((min(min_c.x, w.x), min(min_c.y, w.y), min(min_c.z, w.z)))
            max_c = Vector((max(max_c.x, w.x), max(max_c.y, w.y), max(max_c.z, w.z)))
        verts += len(o.data.vertices)
        faces += len(o.data.polygons)
    size = max_c - min_c
    return {
        "count": len(objs),
        "vertices": verts,
        "faces": faces,
        "bbox_min": [min_c.x, min_c.y, min_c.z],
        "bbox_max": [max_c.x, max_c.y, max_c.z],
        "dimensions": [size.x, size.y, size.z],
    }


def render_ortho(path: Path, camera_loc: tuple[float, float, float], look_at: Vector) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    cam_data = bpy.data.cameras.new("D15_Cam")
    cam_data.type = "ORTHO"
    cam_data.ortho_scale = 2.4
    cam = bpy.data.objects.new("D15_Cam", cam_data)
    bpy.context.scene.collection.objects.link(cam)
    cam.location = camera_loc
    direction = look_at - cam.location
    cam.rotation_euler = direction.to_track_quat("-Z", "Y").to_euler()
    bpy.context.scene.camera = cam
    bpy.context.scene.render.resolution_x = 768
    bpy.context.scene.render.resolution_y = 1024
    bpy.context.scene.render.filepath = str(path)
    bpy.context.scene.render.image_settings.file_format = "PNG"
    # Prefer EEVEE if present; fall back silently
    try:
        bpy.context.scene.render.engine = "BLENDER_EEVEE_NEXT"
    except Exception:
        try:
            bpy.context.scene.render.engine = "BLENDER_WORKBENCH"
        except Exception:
            pass
    bpy.ops.render.render(write_still=True)
    bpy.data.objects.remove(cam, do_unlink=True)
    bpy.data.cameras.remove(cam_data)


def main() -> None:
    OUT_EQUIP.mkdir(parents=True, exist_ok=True)
    OUT_BASE.mkdir(parents=True, exist_ok=True)
    OUT_PILOT.mkdir(parents=True, exist_ok=True)
    OUT_RENDERS.mkdir(parents=True, exist_ok=True)
    OUT_SOCKETS.mkdir(parents=True, exist_ok=True)

    # Scene already loaded via CLI blend path; verify
    report: dict = {
        "utc": stamp(),
        "source_blend": str(PLAYER_BLEND),
        "sword_objects": [],
        "sockets": {},
        "notes": [],
    }

    arm = find_armature()
    if arm is None:
        report["notes"].append("No armature found in v007 scene")
    else:
        report["armature"] = arm.name
        for bone_name in (
            "hand.R",
            "hand.L",
            "sword",
            "head",
            "chest",
            "pelvis",
            "foot.R",
            "foot.L",
            "cape_back_01",
        ):
            m = bone_world_matrix(arm, bone_name)
            if m is None:
                report["sockets"][bone_name] = None
                continue
            loc = m.to_translation()
            report["sockets"][bone_name] = {
                "location": [loc.x, loc.y, loc.z],
                "bone": bone_name,
            }

    sword_objs = [o for o in bpy.data.objects if is_sword_mesh(o)]
    # Exclude glove/hand meshes accidentally
    sword_objs = [o for o in sword_objs if "hand" not in o.name.lower()]
    report["sword_objects"] = [o.name for o in sword_objs]
    report["sword_stats"] = dims_of(sword_objs)

    if not sword_objs:
        report["notes"].append("No sword mesh objects matched; inspect naming in v007")
    else:
        # Export independent sword GLB (canonical geometry from approved model)
        clear_selection()
        for o in sword_objs:
            o.hide_set(False)
            o.hide_viewport = False
            o.select_set(True)
        glb_path = OUT_EQUIP / "modelo" / "espada_inicial_canonical_v007.glb"
        export_selected_glb(glb_path)
        report["sword_glb"] = str(glb_path)

        # Also save a dedicated blend with only sword (linked copy via duplicate)
        bpy.ops.wm.read_factory_settings(use_empty=True)
        # Re-open source to continue? Better: save pilot from current state before factory.
        # We already exported GLB; re-open original for pilot assembly.
        bpy.ops.wm.open_mainfile(filepath=str(PLAYER_BLEND))

        # Re-fetch after reopen
        arm = find_armature()
        sword_objs = [o for o in bpy.data.objects if is_sword_mesh(o) and "hand" not in o.name.lower()]

    # Pilot: duplicate scene state into new file under d15, never overwrite source
    pilot_path = OUT_PILOT / f"{stamp()}_base_plus_sword.blend"
    # Hide sword for "unequipped" collection
    unequip = bpy.data.collections.new("D15_Unequipped_HideSword")
    bpy.context.scene.collection.children.link(unequip)
    equip = bpy.data.collections.new("D15_Equipped_ShowSword")
    bpy.context.scene.collection.children.link(equip)

    for o in sword_objs:
        # Mark for visibility toggles in report; keep visible (equipped default)
        o["d15_equipment"] = "weapon"
        try:
            equip.objects.link(o)
        except RuntimeError:
            pass

    # Empty socket markers
    if arm is not None:
        for bone_name, key in (
            ("hand.R", "socket_hand_r"),
            ("hand.L", "socket_hand_l"),
            ("sword", "socket_weapon"),
            ("head", "socket_head"),
            ("chest", "socket_chest"),
            ("pelvis", "socket_hips"),
            ("foot.R", "socket_foot_r"),
            ("foot.L", "socket_foot_l"),
            ("cape_back_01", "socket_back_cape"),
        ):
            m = bone_world_matrix(arm, bone_name)
            if m is None:
                continue
            empty = bpy.data.objects.new(key, None)
            empty.empty_display_type = "ARROWS"
            empty.empty_display_size = 0.08
            empty.matrix_world = m
            # Parent to bone so it follows animation
            empty.parent = arm
            empty.parent_type = "BONE"
            empty.parent_bone = bone_name
            empty.matrix_parent_inverse = (arm.matrix_world @ arm.pose.bones[bone_name].matrix).inverted()
            bpy.context.scene.collection.objects.link(empty)

    bpy.ops.wm.save_as_mainfile(filepath=str(pilot_path))
    report["pilot_blend"] = str(pilot_path)

    # Quick renders: side + front with sword visible
    look = Vector((0.0, 0.0, 0.9))
    try:
        render_ortho(OUT_RENDERS / "pilot_front_equipped.png", (3.2, 0.0, 0.9), look)
        render_ortho(OUT_RENDERS / "pilot_side_equipped.png", (0.0, -3.2, 0.9), look)
        # Unequip: hide sword meshes
        for o in sword_objs:
            o.hide_render = True
            o.hide_viewport = True
        render_ortho(OUT_RENDERS / "pilot_front_unequipped.png", (3.2, 0.0, 0.9), look)
        render_ortho(OUT_RENDERS / "pilot_side_unequipped.png", (0.0, -3.2, 0.9), look)
        for o in sword_objs:
            o.hide_render = False
            o.hide_viewport = False
        # Sword only
        for o in bpy.data.objects:
            if o.type == "MESH" and o not in sword_objs:
                o.hide_render = True
        render_ortho(OUT_RENDERS / "pilot_sword_isolated.png", (0.0, -2.0, 1.0), Vector((0.0, 0.0, 1.0)))
        report["renders"] = [
            "pilot_front_equipped.png",
            "pilot_side_equipped.png",
            "pilot_front_unequipped.png",
            "pilot_side_unequipped.png",
            "pilot_sword_isolated.png",
        ]
    except Exception as e:
        report["notes"].append(f"render failed: {e}")

    # Persist sockets JSON + report
    (OUT_SOCKETS / "v007_bone_sockets.json").write_text(
        json.dumps({"armature": report.get("armature"), "sockets": report["sockets"]}, indent=2),
        encoding="utf-8",
    )
    (OUT_PILOT / "pilot_report.json").write_text(json.dumps(report, indent=2), encoding="utf-8")
    # Pointer for base model (do not copy multi-hundred-MB blends; document path)
    (OUT_BASE / "modelo" / "SOURCE.txt").write_text(
        "Interim base (NOT a new identity): art-src/player/blender/swordsman_rig_v007.blend\n"
        "Do not modify the original. Pilot scene is under art-src/visual-prototypes/d15_equip_modular/pilot/\n",
        encoding="utf-8",
    )
    print("D15_OK", json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
