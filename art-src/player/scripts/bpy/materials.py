"""Flat Principled materials built from the shared palette.

Validated in Blender 5.2.4 LTS (V1.2-B).
"""

from __future__ import annotations

import bpy

from config import CHARACTER, color

# Low metallic on purpose: with only sun lights and a near-black world, a
# physically metallic blade reflects the dark world and loses the light stripe
# that must survive at sprite scale. Metals read through colour + roughness.
METALLIC = {"silver": 0.2, "silver_light": 0.2, "silver_dark": 0.2, "amber": 0.15, "amber_dark": 0.15}
ROUGHNESS_CLOTH = 0.75
ROUGHNESS_METAL = 0.4


def palette_material(name: str) -> bpy.types.Material:
    """Returns (creating once) the material named ``pal_<name>``."""
    if name not in CHARACTER:
        raise KeyError(f"unknown palette colour: {name}")
    mat_name = f"pal_{name}"
    existing = bpy.data.materials.get(mat_name)
    if existing is not None:
        return existing
    mat = bpy.data.materials.new(mat_name)
    if hasattr(mat, "use_nodes"):
        mat.use_nodes = True
    # Looked up by type: node names are localised in non-English UIs.
    bsdf = next((n for n in mat.node_tree.nodes if n.type == "BSDF_PRINCIPLED"), None)
    if bsdf is None:
        raise RuntimeError("Principled BSDF node not found in a new material")
    bsdf.inputs["Base Color"].default_value = color(name)
    metallic = METALLIC.get(name, 0.0)
    bsdf.inputs["Metallic"].default_value = metallic
    bsdf.inputs["Roughness"].default_value = ROUGHNESS_METAL if metallic else ROUGHNESS_CLOTH
    mat.diffuse_color = color(name)  # viewport / Workbench colour
    return mat


def assign(obj: bpy.types.Object, name: str) -> None:
    obj.data.materials.clear()
    obj.data.materials.append(palette_material(name))
