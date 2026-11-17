"""Build D13_* collections and export modular layer renders."""
from __future__ import annotations

import os
import sys

import bpy

# Allow importing sibling scripts when run via blender --python
SCRIPTS = os.path.dirname(os.path.abspath(__file__))
if SCRIPTS not in sys.path:
    sys.path.insert(0, SCRIPTS)

import build_materials  # noqa: E402
import configure_camera  # noqa: E402
import configure_lighting  # noqa: E402

PREFIXES = {
    "body": ("head", "neck", "mouth", "ear_", "eye_", "brow_", "lid_", "hair_", "leg_"),
    "coat": (
        "coat",
        "collar",
        "sleeve_",
        "cuff_",
        "torso_vest",
        "vest_",
        "torso_trousers",
        "trouser_",
        "belt",
    ),
    "cape": ("half_cape", "cape_"),
    "sword": ("blade", "sword_", "blade_readable"),
    "armor": ("pauldron",),
    "boots": ("boot_",),
    "gloves": ("glove_",),
}

LAB = os.path.abspath(os.path.join(SCRIPTS, ".."))
EXP = os.path.join(LAB, "exports")
REND = os.path.join(LAB, "renders")


def ensure_col(name: str):
    if name not in bpy.data.collections:
        col = bpy.data.collections.new(name)
        bpy.context.scene.collection.children.link(col)
    return bpy.data.collections[name]


def build_collections() -> None:
    mnames = [o for o in bpy.data.objects if o.type == "MESH"]
    for slot, prefs in PREFIXES.items():
        col = ensure_col(f"D13_{slot}")
        for o in list(col.objects):
            col.objects.unlink(o)
        for o in mnames:
            n = o.name
            if n == prefs[0] or any(n.startswith(p) for p in prefs):
                col.objects.link(o)
        print("COL", col.name, len(col.objects))


def set_visibility(only: list[str] | None) -> None:
    for s in PREFIXES:
        col = bpy.data.collections.get(f"D13_{s}")
        if not col:
            continue
        hide = only is not None and s not in only
        col.hide_render = hide
        col.hide_viewport = hide


def render_to(path: str) -> None:
    os.makedirs(os.path.dirname(path), exist_ok=True)
    bpy.context.scene.render.filepath = path
    bpy.ops.render.render(write_still=True)
    print("WROTE", path)


def main() -> None:
    configure_camera.main()
    build_collections()
    rig = bpy.data.objects.get("rig_v007")
    if rig and "POSE_P01_neutral" in bpy.data.actions:
        if rig.animation_data is None:
            rig.animation_data_create()
        rig.animation_data.action = bpy.data.actions["POSE_P01_neutral"]

    set_visibility(None)
    for mode in ("A", "B", "C"):
        build_materials.apply(mode)
        configure_lighting.apply(mode)
        render_to(os.path.join(REND, f"materials_lights_{mode}.png"))

    build_materials.apply("B")
    configure_lighting.apply("B")
    layers = [
        (["body", "boots", "gloves"], "body_boots_gloves"),
        (["coat"], "coat"),
        (["cape"], "cape"),
        (["sword"], "sword"),
        (["armor"], "armor"),
        (None, "full"),
    ]
    for only, tag in layers:
        set_visibility(only)
        render_to(os.path.join(EXP, f"layer_{tag}.png"))

    set_visibility(["body", "boots", "gloves", "coat", "sword"])
    render_to(os.path.join(REND, "mod_no_cape.png"))
    set_visibility(["body", "boots", "gloves", "coat", "sword", "armor", "cape"])
    render_to(os.path.join(REND, "mod_with_armor.png"))
    set_visibility(["body", "boots", "gloves", "cape", "sword", "armor"])  # no coat
    render_to(os.path.join(REND, "mod_no_coat.png"))
    set_visibility(None)
    # hierarchy / ortho evidence
    render_to(os.path.join(REND, "full_ortho_B.png"))
    print("OK export_layers")


if __name__ == "__main__":
    main()
