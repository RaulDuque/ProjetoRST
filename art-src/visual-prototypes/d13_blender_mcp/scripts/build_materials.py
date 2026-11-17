"""Non-destructive material studies on existing v4_* shaders."""
from __future__ import annotations

import bpy


def apply(mode: str = "B") -> None:
    for mat in bpy.data.materials:
        if not mat.use_nodes or not mat.name.startswith("v4_"):
            continue
        bsdf = next((n for n in mat.node_tree.nodes if n.type == "BSDF_PRINCIPLED"), None)
        if bsdf is None:
            continue
        if "d13_base" not in mat:
            mat["d13_base"] = list(bsdf.inputs["Base Color"].default_value)
            mat["d13_rough"] = float(bsdf.inputs["Roughness"].default_value)
            mat["d13_metal"] = float(bsdf.inputs["Metallic"].default_value)
        base = list(mat["d13_base"])
        rough = float(mat["d13_rough"])
        metal = float(mat["d13_metal"])
        if mode == "A":
            bsdf.inputs["Base Color"].default_value = base
            bsdf.inputs["Roughness"].default_value = min(1.0, rough * 1.05 + 0.05)
            bsdf.inputs["Metallic"].default_value = metal * 0.85
        elif mode == "B":
            bsdf.inputs["Base Color"].default_value = (
                base[0] * 0.92,
                base[1] * 0.9,
                base[2] * 0.95,
                base[3],
            )
            bsdf.inputs["Roughness"].default_value = max(0.15, rough * 0.85)
            bsdf.inputs["Metallic"].default_value = min(1.0, metal * 1.15 + 0.05)
        else:
            bsdf.inputs["Base Color"].default_value = (
                min(1.0, base[0] * 1.05 + 0.02),
                min(1.0, base[1] * 1.02),
                min(1.0, base[2] * 1.02),
                base[3],
            )
            bsdf.inputs["Roughness"].default_value = max(0.12, rough * 0.7)
            bsdf.inputs["Metallic"].default_value = min(1.0, metal * 1.25 + 0.08)
    print("OK materials", mode)


if __name__ == "__main__":
    apply("B")
