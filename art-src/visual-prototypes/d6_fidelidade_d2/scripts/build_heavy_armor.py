"""D6 lab: add experimental heavy-armor plate meshes to the lab blend and save.

Never touches original player blends. Run:

  blender --background --factory-startup \\
    art-src/visual-prototypes/d6_fidelidade_d2/blender/swordsman_rig_v007_d6_lab.blend \\
    --python art-src/visual-prototypes/d6_fidelidade_d2/scripts/build_heavy_armor.py
"""
from __future__ import annotations

import json
from pathlib import Path

import bpy
from mathutils import Vector

D6 = Path(__file__).resolve().parents[1]
OUT_BLEND = D6 / "blender" / "swordsman_rig_v007_d6_lab.blend"
META = D6 / "blender" / "heavy_armor_manifest.json"
ARMOR_COLL = "D6_heavy_armor"

# name -> (parent_bone, size_xyz_m, local_offset, local_euler_xyz_deg)
# Sizes are lab approximations — identifiable plates, not final art.
SPECS: dict[str, tuple[str, tuple[float, float, float], tuple[float, float, float], tuple[float, float, float]]] = {
    "d6_cuirass": ("chest", (0.28, 0.18, 0.34), (0.04, 0.0, 0.02), (0.0, 0.0, 0.0)),
    "d6_pauldron_R": ("upper_arm.R", (0.14, 0.12, 0.12), (0.06, -0.04, 0.02), (15.0, 0.0, -20.0)),
    "d6_pauldron_L": ("upper_arm.L", (0.14, 0.12, 0.12), (0.06, 0.04, 0.02), (15.0, 0.0, 20.0)),
    "d6_bracer_R": ("forearm.R", (0.16, 0.07, 0.07), (0.08, 0.0, 0.0), (0.0, 0.0, 0.0)),
    "d6_bracer_L": ("forearm.L", (0.16, 0.07, 0.07), (0.08, 0.0, 0.0), (0.0, 0.0, 0.0)),
    "d6_gauntlet_R": ("hand.R", (0.10, 0.08, 0.05), (0.05, 0.0, 0.0), (0.0, 0.0, 0.0)),
    "d6_gauntlet_L": ("hand.L", (0.10, 0.08, 0.05), (0.05, 0.0, 0.0), (0.0, 0.0, 0.0)),
    "d6_fauld": ("pelvis", (0.26, 0.16, 0.12), (0.02, 0.0, -0.05), (0.0, 0.0, 0.0)),
    "d6_cuisse_R": ("thigh.R", (0.18, 0.10, 0.10), (0.10, 0.0, 0.0), (0.0, 0.0, 0.0)),
    "d6_cuisse_L": ("thigh.L", (0.18, 0.10, 0.10), (0.10, 0.0, 0.0), (0.0, 0.0, 0.0)),
    "d6_greave_R": ("shin.R", (0.18, 0.08, 0.08), (0.10, 0.0, 0.0), (0.0, 0.0, 0.0)),
    "d6_greave_L": ("shin.L", (0.18, 0.08, 0.08), (0.10, 0.0, 0.0), (0.0, 0.0, 0.0)),
    "d6_sabaton_R": ("foot.R", (0.14, 0.08, 0.06), (0.06, 0.0, -0.02), (0.0, 0.0, 0.0)),
    "d6_sabaton_L": ("foot.L", (0.14, 0.08, 0.06), (0.06, 0.0, -0.02), (0.0, 0.0, 0.0)),
    "d6_helm": ("head", (0.16, 0.18, 0.20), (0.02, 0.0, 0.06), (0.0, 0.0, 0.0)),
}


def ensure_collection(name: str) -> bpy.types.Collection:
    if name in bpy.data.collections:
        coll = bpy.data.collections[name]
    else:
        coll = bpy.data.collections.new(name)
        bpy.context.scene.collection.children.link(coll)
    return coll


def metal_material() -> bpy.types.Material:
    name = "D6_ArmorMetal"
    if name in bpy.data.materials:
        return bpy.data.materials[name]
    mat = bpy.data.materials.new(name)
    mat.use_nodes = True
    nt = mat.node_tree
    bsdf = nt.nodes.get("Principled BSDF")
    if bsdf:
        bsdf.inputs["Base Color"].default_value = (0.08, 0.09, 0.12, 1.0)
        if "Metallic" in bsdf.inputs:
            bsdf.inputs["Metallic"].default_value = 0.85
        if "Roughness" in bsdf.inputs:
            bsdf.inputs["Roughness"].default_value = 0.35
    return mat


def clear_previous(coll: bpy.types.Collection) -> None:
    for obj in list(coll.objects):
        bpy.data.objects.remove(obj, do_unlink=True)


def add_plate(
    name: str,
    bone: str,
    size: tuple[float, float, float],
    offset: tuple[float, float, float],
    euler_deg: tuple[float, float, float],
    arm: bpy.types.Object,
    coll: bpy.types.Collection,
    mat: bpy.types.Material,
) -> None:
    import math

    bpy.ops.mesh.primitive_cube_add(size=1.0)
    obj = bpy.context.active_object
    obj.name = name
    obj.scale = Vector(size)
    bpy.ops.object.transform_apply(scale=True)
    if obj.data.materials:
        obj.data.materials[0] = mat
    else:
        obj.data.materials.append(mat)

    # Parent with bone relative
    obj.parent = arm
    obj.parent_type = "BONE"
    obj.parent_bone = bone
    obj.location = Vector(offset)
    obj.rotation_euler = tuple(math.radians(a) for a in euler_deg)

    # Move to armor collection
    for c in list(obj.users_collection):
        c.objects.unlink(obj)
    coll.objects.link(obj)


def main() -> None:
    arm = next(o for o in bpy.data.objects if o.type == "ARMATURE")
    coll = ensure_collection(ARMOR_COLL)
    clear_previous(coll)
    mat = metal_material()
    created = []
    for name, (bone, size, offset, euler) in SPECS.items():
        if bone not in arm.data.bones:
            raise SystemExit(f"missing bone {bone}")
        add_plate(name, bone, size, offset, euler, arm, coll, mat)
        created.append({"name": name, "bone": bone, "size": list(size)})
        print(f"[d6] plate {name} -> {bone}")

    # Soften cubes: bevel-ish via shade smooth
    for name in SPECS:
        obj = bpy.data.objects[name]
        bpy.context.view_layer.objects.active = obj
        obj.select_set(True)
        bpy.ops.object.shade_smooth()
        obj.select_set(False)

    bpy.ops.wm.save_as_mainfile(filepath=str(OUT_BLEND))
    META.write_text(json.dumps({"pieces": created, "collection": ARMOR_COLL}, indent=2))
    print("[d6] saved", OUT_BLEND)


if __name__ == "__main__":
    main()
