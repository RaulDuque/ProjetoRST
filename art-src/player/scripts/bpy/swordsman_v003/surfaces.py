"""v003 materials: one surface per kind of material, from the shared palette.

Shaded mode: Principled BSDF with roughness, metallic, sheen and small
subsurface values that tell cloth, satin, leather, skin and metal apart.
Toon mode (third finish test): diffuse light quantised to two bands by
``Shader to RGB`` + a constant colour ramp, times the palette colour, as
emission. Lit areas come out exactly in the palette colour (Standard view
transform), which helps the pixel-art palette snap.

Node lookups use node types, never names (names are localised).
"""

from __future__ import annotations

from dataclasses import dataclass

import bpy

from config import CHARACTER, color


@dataclass(frozen=True)
class Surface:
    palette: str
    roughness: float
    metallic: float = 0.0
    sheen: float = 0.0
    subsurface: float = 0.0
    coat: float = 0.0
    emission: float = 0.0


SURFACES = {
    "skin": Surface("skin", 0.55, subsurface=0.12),
    "skin_shadow": Surface("skin_shadow", 0.6, subsurface=0.08),
    "eye": Surface("amber", 0.3, emission=0.6),
    "lash": Surface("outline_dark", 0.6),
    "hair": Surface("hair", 0.42, sheen=0.25),
    "hair_light": Surface("hair_light", 0.38, sheen=0.3),
    "coat_wool": Surface("coat", 0.9, sheen=0.35),
    "vest": Surface("coat_light", 0.78, sheen=0.2),
    "lining_satin": Surface("lining_wine", 0.3, sheen=0.15),
    "cape_wool": Surface("cape_wine", 0.85, sheen=0.35),
    "cape_inner": Surface("cape_shadow", 0.8, sheen=0.2),
    "trousers": Surface("trousers", 0.85, sheen=0.15),
    "boot_leather": Surface("boots", 0.5, coat=0.08),
    "boot_cuff": Surface("boot_cuff", 0.5),
    "sole": Surface("outline_dark", 0.7),
    "glove_leather": Surface("glove", 0.45, coat=0.1),
    "blade": Surface("silver", 0.24, metallic=0.55),
    "blade_edge": Surface("silver_light", 0.18, metallic=0.4, emission=0.18),
    "metal_dark": Surface("silver_dark", 0.38, metallic=0.75),
    "brass": Surface("amber", 0.32, metallic=0.7),
}

TOON_SHADOW = 0.58  # brightness of the shadow band
TOON_THRESHOLD = 0.32  # diffuse light level splitting shadow and lit bands


def _set(node: bpy.types.Node, name: str, value: object) -> None:
    if name in node.inputs:
        node.inputs[name].default_value = value


def shaded(name: str) -> bpy.types.Material:
    mat_name = f"v3_{name}"
    existing = bpy.data.materials.get(mat_name)
    if existing is not None:
        return existing
    spec = SURFACES[name]
    if spec.palette not in CHARACTER:
        raise KeyError(f"unknown palette colour: {spec.palette}")
    mat = bpy.data.materials.new(mat_name)
    if hasattr(mat, "use_nodes"):
        mat.use_nodes = True
    bsdf = next((n for n in mat.node_tree.nodes if n.type == "BSDF_PRINCIPLED"), None)
    if bsdf is None:
        raise RuntimeError("Principled BSDF node not found in a new material")
    rgba = color(spec.palette)
    _set(bsdf, "Base Color", rgba)
    _set(bsdf, "Roughness", spec.roughness)
    _set(bsdf, "Metallic", spec.metallic)
    _set(bsdf, "Sheen Weight", spec.sheen)
    _set(bsdf, "Subsurface Weight", spec.subsurface)
    _set(bsdf, "Coat Weight", spec.coat)
    if spec.emission > 0.0:
        _set(bsdf, "Emission Color", rgba)
        _set(bsdf, "Emission Strength", spec.emission)
    mat.diffuse_color = rgba
    mat.roughness = spec.roughness
    return mat


def toon(name: str) -> bpy.types.Material:
    """Flat two-band version of a surface (named ``v3t_<name>``)."""
    mat_name = f"v3t_{name}"
    existing = bpy.data.materials.get(mat_name)
    if existing is not None:
        return existing
    spec = SURFACES[name]
    mat = bpy.data.materials.new(mat_name)
    if hasattr(mat, "use_nodes"):
        mat.use_nodes = True
    tree = mat.node_tree
    for node in list(tree.nodes):
        if node.type != "OUTPUT_MATERIAL":
            tree.nodes.remove(node)
    output = next(n for n in tree.nodes if n.type == "OUTPUT_MATERIAL")
    diffuse = tree.nodes.new("ShaderNodeBsdfDiffuse")
    to_rgb = tree.nodes.new("ShaderNodeShaderToRGB")
    ramp = tree.nodes.new("ShaderNodeValToRGB")
    multiply = tree.nodes.new("ShaderNodeVectorMath")
    emission = tree.nodes.new("ShaderNodeEmission")
    diffuse.inputs["Color"].default_value = (1.0, 1.0, 1.0, 1.0)
    ramp.color_ramp.interpolation = "CONSTANT"
    stops = ramp.color_ramp.elements
    stops[0].position = 0.0
    stops[0].color = (TOON_SHADOW, TOON_SHADOW, TOON_SHADOW, 1.0)
    stops[1].position = TOON_THRESHOLD
    stops[1].color = (1.0, 1.0, 1.0, 1.0)
    multiply.operation = "MULTIPLY"
    rgba = color(spec.palette)
    multiply.inputs[0].default_value = rgba[:3]
    lit = 1.0 if spec.emission == 0.0 else 1.15
    emission.inputs["Strength"].default_value = lit
    links = tree.links
    links.new(diffuse.outputs[0], to_rgb.inputs[0])
    links.new(to_rgb.outputs["Color"], ramp.inputs["Fac"])
    links.new(ramp.outputs["Color"], multiply.inputs[1])
    links.new(multiply.outputs[0], emission.inputs["Color"])
    links.new(emission.outputs[0], output.inputs["Surface"])
    mat.diffuse_color = rgba
    return mat


def use_toon(enabled: bool) -> None:
    """Swaps every v3_/v3t_ material slot of the scene between the two modes."""
    for obj in bpy.context.scene.objects:
        if obj.type != "MESH":
            continue
        for slot in obj.material_slots:
            mat = slot.material
            if mat is None:
                continue
            if enabled and mat.name.startswith("v3_"):
                slot.material = toon(mat.name[3:])
            elif not enabled and mat.name.startswith("v3t_"):
                slot.material = shaded(mat.name[4:])
