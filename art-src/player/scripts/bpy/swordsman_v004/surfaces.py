"""v004 materials: less plastic than v003, same palette and same toon test.

Changes from v003 (``swordsman_v003/surfaces.py``, untouched):

- ``Specular IOR Level`` is set per surface (v003 left Blender's 0.5 on
  everything, which made wool, hair and leather shine like plastic);
- cloth, hair and leather get a fine procedural bump (Noise texture ->
  Bump, no image textures) that breaks the highlight into a surface grain;
- roughness rises on wool, hair, leather and the dark metal; metal keeps a
  real metallic response so it still reads as metal;
- the amber eye emission drops from 0.6 to 0.12, plus new ``sclera``,
  ``lip`` and ``brow`` surfaces for the refined face.

Materials are named ``v4_<surface>`` (shaded) and ``v4t_<surface>`` (flat
two-band "toon", finish test C, same construction as v003). Node lookups use
node types, never names (names are localised).
"""

from __future__ import annotations

from dataclasses import dataclass

import bpy

from config import CHARACTER, color

SHADED_PREFIX = "v4_"
TOON_PREFIX = "v4t_"


@dataclass(frozen=True)
class Grain:
    """Procedural bump: noise ``scale`` (1/m), bump ``strength``, ``distance`` (m)."""

    scale: float
    strength: float
    distance: float = 0.0015
    stretch_z: float = 1.0  # >1 stretches the noise along Z (fibres, hair streaks)


@dataclass(frozen=True)
class Surface:
    palette: str
    roughness: float
    specular: float = 0.5  # Specular IOR Level (Blender default 0.5)
    metallic: float = 0.0
    sheen: float = 0.0
    subsurface: float = 0.0
    coat: float = 0.0
    emission: float = 0.0
    tint: float = 1.0  # multiplies the palette colour (linear), for derived tones
    grain: Grain | None = None


WOOL = Grain(520.0, 0.22, 0.0012)
LEATHER = Grain(260.0, 0.14, 0.0010)
HAIR = Grain(900.0, 0.35, 0.0010, stretch_z=0.25)

SURFACES = {
    "skin": Surface("skin", 0.62, specular=0.32, subsurface=0.15),
    "skin_shadow": Surface("skin_shadow", 0.66, specular=0.3, subsurface=0.1),
    "lip": Surface("skin_shadow", 0.55, specular=0.35, subsurface=0.12, tint=0.92),
    "sclera": Surface("silver_light", 0.5, specular=0.4, tint=0.62),
    "eye": Surface("amber", 0.35, specular=0.5, emission=0.12),
    "lash": Surface("outline_dark", 0.75, specular=0.2),
    "brow": Surface("hair", 0.9, specular=0.15, tint=0.85, grain=HAIR),
    "hair": Surface("hair", 0.66, specular=0.22, sheen=0.15, grain=HAIR),
    "hair_light": Surface("hair_light", 0.62, specular=0.24, sheen=0.18, grain=HAIR),
    "coat_wool": Surface("coat", 0.95, specular=0.12, sheen=0.4, grain=WOOL),
    "vest": Surface("coat_light", 0.88, specular=0.18, sheen=0.25, grain=WOOL),
    "vest_trim": Surface("coat_light", 0.7, specular=0.25, sheen=0.2, tint=0.7),
    "lining_satin": Surface("lining_wine", 0.34, specular=0.45, sheen=0.3),
    # Brighter than the coat on purpose: the wine half-cape is one of the
    # colour marks that must survive at 52 px (a darker cape snaps to the
    # glove colour in finish A).
    "cape_wool": Surface("cape_wine", 0.85, specular=0.28, sheen=0.15, tint=1.15, grain=WOOL),
    "cape_inner": Surface("cape_shadow", 0.9, specular=0.12, sheen=0.25, grain=WOOL),
    "trousers": Surface("trousers", 0.9, specular=0.15, sheen=0.2, grain=WOOL),
    "boot_leather": Surface("boots", 0.6, specular=0.3, coat=0.03, grain=LEATHER),
    "boot_cuff": Surface("boot_cuff", 0.68, specular=0.28, grain=LEATHER),
    "stitch": Surface("boot_cuff", 0.8, specular=0.2, tint=0.8),
    "sole": Surface("outline_dark", 0.82, specular=0.2),
    "glove_leather": Surface("glove", 0.62, specular=0.3, coat=0.03, grain=LEATHER),
    "belt_leather": Surface("boots", 0.66, specular=0.28, tint=1.25, grain=LEATHER),
    "blade": Surface("silver", 0.24, metallic=0.55),
    "blade_edge": Surface("silver_light", 0.18, metallic=0.4, emission=0.18),
    "metal_dark": Surface("silver_dark", 0.46, specular=0.45, metallic=0.8),
    "brass": Surface("amber", 0.36, metallic=0.75),
}

TOON_SHADOW = 0.58  # brightness of the shadow band (as v003)
TOON_THRESHOLD = 0.32  # diffuse light level splitting shadow and lit bands


def _set(node: bpy.types.Node, name: str, value: object) -> None:
    if name not in node.inputs:
        raise KeyError(f"Principled BSDF has no input {name!r} in Blender {bpy.app.version_string}")
    node.inputs[name].default_value = value


def _rgba(spec: Surface) -> tuple[float, float, float, float]:
    if spec.palette not in CHARACTER:
        raise KeyError(f"unknown palette colour: {spec.palette}")
    r, g, b, a = color(spec.palette)
    return (min(r * spec.tint, 1.0), min(g * spec.tint, 1.0), min(b * spec.tint, 1.0), a)


def _add_grain(tree: bpy.types.NodeTree, bsdf: bpy.types.Node, grain: Grain) -> None:
    coords = tree.nodes.new("ShaderNodeTexCoord")
    mapping = tree.nodes.new("ShaderNodeMapping")
    noise = tree.nodes.new("ShaderNodeTexNoise")
    bump = tree.nodes.new("ShaderNodeBump")
    mapping.inputs["Scale"].default_value = (grain.scale, grain.scale, grain.scale * grain.stretch_z)
    noise.inputs["Scale"].default_value = 1.0
    noise.inputs["Detail"].default_value = 3.0
    bump.inputs["Strength"].default_value = grain.strength
    bump.inputs["Distance"].default_value = grain.distance
    tree.links.new(coords.outputs["Object"], mapping.inputs["Vector"])
    tree.links.new(mapping.outputs["Vector"], noise.inputs["Vector"])
    tree.links.new(noise.outputs["Fac"], bump.inputs["Height"])
    tree.links.new(bump.outputs["Normal"], bsdf.inputs["Normal"])


def shaded(name: str) -> bpy.types.Material:
    mat_name = f"{SHADED_PREFIX}{name}"
    existing = bpy.data.materials.get(mat_name)
    if existing is not None:
        return existing
    spec = SURFACES[name]
    mat = bpy.data.materials.new(mat_name)
    if hasattr(mat, "use_nodes"):
        mat.use_nodes = True
    bsdf = next((n for n in mat.node_tree.nodes if n.type == "BSDF_PRINCIPLED"), None)
    if bsdf is None:
        raise RuntimeError("Principled BSDF node not found in a new material")
    rgba = _rgba(spec)
    _set(bsdf, "Base Color", rgba)
    _set(bsdf, "Roughness", spec.roughness)
    _set(bsdf, "Specular IOR Level", spec.specular)
    _set(bsdf, "Metallic", spec.metallic)
    _set(bsdf, "Sheen Weight", spec.sheen)
    _set(bsdf, "Subsurface Weight", spec.subsurface)
    _set(bsdf, "Coat Weight", spec.coat)
    if spec.emission > 0.0:
        _set(bsdf, "Emission Color", rgba)
        _set(bsdf, "Emission Strength", spec.emission)
    if spec.grain is not None:
        _add_grain(mat.node_tree, bsdf, spec.grain)
    mat.diffuse_color = rgba
    mat.roughness = spec.roughness
    return mat


def toon(name: str) -> bpy.types.Material:
    """Flat two-band version of a surface (same construction as v003)."""
    mat_name = f"{TOON_PREFIX}{name}"
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
    rgba = _rgba(spec)
    multiply.inputs[0].default_value = rgba[:3]
    emission.inputs["Strength"].default_value = 1.0 if spec.emission == 0.0 else 1.15
    links = tree.links
    links.new(diffuse.outputs[0], to_rgb.inputs[0])
    links.new(to_rgb.outputs["Color"], ramp.inputs["Fac"])
    links.new(ramp.outputs["Color"], multiply.inputs[1])
    links.new(multiply.outputs[0], emission.inputs["Color"])
    links.new(emission.outputs[0], output.inputs["Surface"])
    mat.diffuse_color = rgba
    return mat


def use_toon(enabled: bool) -> None:
    """Swaps every v4_/v4t_ material slot of the scene between the two modes."""
    for obj in bpy.context.scene.objects:
        if obj.type != "MESH":
            continue
        for slot in obj.material_slots:
            mat = slot.material
            if mat is None:
                continue
            if enabled and mat.name.startswith(SHADED_PREFIX):
                slot.material = toon(mat.name[len(SHADED_PREFIX):])
            elif not enabled and mat.name.startswith(TOON_PREFIX):
                slot.material = shaded(mat.name[len(TOON_PREFIX):])
