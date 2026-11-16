"""D5 lab: export run clip as independent mesh layers (same poses as V1.3).

Opens the D5 lab .blend copy only. Never writes assets/ or the original
player blends. Run from repo root:

  blender --background --factory-startup \\
    art-src/visual-prototypes/d5_corrida_modular/blender/swordsman_rig_v007_d5_lab.blend \\
    --python art-src/visual-prototypes/d5_corrida_modular/scripts/export_modular_run.py -- \\
    [--limit N] [--overwrite]
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

SCRIPTS_DIR = Path(__file__).resolve().parents[3] / "player" / "scripts" / "bpy"
D5 = Path(__file__).resolve().parents[1]
OUT = D5 / "camadas"
RIG_META = D5 / "rig" / "export_manifest.json"

for entry in (
    SCRIPTS_DIR,
    SCRIPTS_DIR / "swordsman_v003",
    SCRIPTS_DIR / "swordsman_v004",
    SCRIPTS_DIR / "swordsman_rig_v005",
    SCRIPTS_DIR / "swordsman_rig_v006",
    SCRIPTS_DIR / "swordsman_rig_v007",
    SCRIPTS_DIR / "game_sprites_v1_3",
):
    s = str(entry)
    if s in sys.path:
        sys.path.remove(s)
    sys.path.insert(0, s)

import bpy  # noqa: E402
from bpy_extras.object_utils import world_to_camera_view  # noqa: E402
from mathutils import Vector  # noqa: E402

import build_animations_v007 as BA  # noqa: E402
import clips as CL  # noqa: E402
import poses as PS  # noqa: E402
import render as R  # noqa: E402
import rig_config as C  # noqa: E402
import rig_render as RR  # noqa: E402
from camera import use_camera  # noqa: E402
from staging import turn_rig  # noqa: E402
from surfaces import use_toon  # noqa: E402

SWORD = {
    "blade",
    "blade_readable",
    "sword_collar",
    "sword_guard",
    "sword_grip",
    "sword_pommel",
}
CAPE = {"half_cape", "cape_clasp", "cape_cord", "cape_stud"}
COAT = {"coat"}
HEAVY = {"pauldron", "pauldron_lame"}
# Everything mesh except equipment layers → body_base (still includes light clothes)
EQUIP = SWORD | CAPE | COAT | HEAVY

LAYERS = {
    "body": None,  # all meshes minus EQUIP
    "light": COAT,
    "cape": CAPE,
    "heavy": HEAVY,
    "weapon": SWORD,
}


def parse_args() -> argparse.Namespace:
    argv = sys.argv[sys.argv.index("--") + 1 :] if "--" in sys.argv else []
    p = argparse.ArgumentParser()
    p.add_argument("--limit", type=int, default=0)
    p.add_argument("--overwrite", action="store_true")
    return p.parse_args(argv)


def all_meshes() -> list[bpy.types.Object]:
    return [o for o in bpy.data.objects if o.type == "MESH"]


def set_layer(layer: str) -> None:
    """Show only the meshes that belong to ``layer`` (plus nothing else)."""
    for obj in all_meshes():
        if layer == "body":
            obj.hide_render = obj.name in EQUIP
            obj.hide_viewport = obj.hide_render
        else:
            wanted = LAYERS[layer]
            show = obj.name in wanted
            obj.hide_render = not show
            obj.hide_viewport = not show


def show_full_loadout(kind: str) -> None:
    """kind: A light+cape+sword | B heavy+cape+sword | body-only."""
    for obj in all_meshes():
        name = obj.name
        if kind == "A":
            hide = name in HEAVY
        elif kind == "B":
            hide = False  # coat+cape+pauldron+sword
        elif kind == "body":
            hide = name in EQUIP
        else:
            hide = False
        obj.hide_render = hide
        obj.hide_viewport = hide


def evaluated_centroid(name: str) -> Vector:
    depsgraph = bpy.context.evaluated_depsgraph_get()
    obj = bpy.data.objects[name].evaluated_get(depsgraph)
    mesh = obj.to_mesh()
    try:
        total = Vector((0.0, 0.0, 0.0))
        for v in mesh.vertices:
            total += v.co
        return obj.matrix_world @ (total / max(len(mesh.vertices), 1))
    finally:
        obj.to_mesh_clear()


def bone_point(arm, bone: str, t: float) -> Vector:
    pb = arm.pose.bones[bone]
    return arm.matrix_world @ pb.head.lerp(pb.tail, t)


def measure_sockets(arm) -> dict:
    import math

    use_camera("side")
    turn_rig(-90.0)
    bpy.context.view_layer.update()
    cam = bpy.context.scene.camera
    w, h = 64, 64
    px, py = 32.0, 63.0
    scene = bpy.context.scene

    def project(point: Vector):
        co = world_to_camera_view(scene, cam, point)
        return co.x * w - px, py - (1.0 - co.y) * h, co.z

    grip = project(evaluated_centroid("sword_grip"))
    pommel = project(evaluated_centroid("sword_pommel"))
    guard = project(evaluated_centroid("sword_collar"))
    chest = project(bone_point(arm, "chest", 0.5))
    hand_l = project(bone_point(arm, "hand.L", 0.6))
    head = project(bone_point(arm, "head", 0.45))
    back = project(bone_point(arm, "collar_back", 0.0))
    angle = math.degrees(math.atan2(guard[1] - pommel[1], guard[0] - pommel[0]))
    r = lambda v: round(float(v), 2)  # noqa: E731
    return {
        "weapon": {
            "x": r(grip[0]),
            "y": r(grip[1]),
            "angle": r(angle),
            "front": grip[2] < chest[2],
        },
        "hand_l": {"x": r(hand_l[0]), "y": r(hand_l[1]), "front": hand_l[2] < chest[2]},
        "head": {"x": r(head[0]), "y": r(head[1])},
        "chest": {"x": r(chest[0]), "y": r(chest[1])},
        "back": {"x": r(back[0]), "y": r(back[1])},
    }


def render_side(path: Path) -> None:
    use_toon(True)
    use_camera("side")
    turn_rig(-90.0)
    R.configure()
    RR.set_resolution(256, 256)  # lab supersample; post downscales
    bpy.context.scene.render.filepath = str(path)
    bpy.context.scene.render.image_settings.file_format = "PNG"
    bpy.context.scene.render.film_transparent = True
    bpy.ops.render.render(write_still=True)
    use_toon(False)


def main() -> None:
    args = parse_args()
    arm = bpy.data.objects[C.ARMATURE_NAME]
    body = PS.all_body()
    R.configure()
    bpy.context.scene.render.fps = C.PRODUCTION_FPS
    PS.reset(arm)

    n = args.limit or CL.CLIPS["run"].frames
    action = CL.build_action(arm, "run", body)
    action_hash = BA.keyframe_hash(action)
    arm.animation_data.action = action

    sockets = []
    renders = {k: [] for k in list(LAYERS) + ["full_A", "full_B", "raw_shaded"]}

    for i in range(n):
        frame = i + 1
        bpy.context.scene.frame_set(frame)
        bpy.context.view_layer.update()
        # temporarily show sword for socket measure
        for name in SWORD:
            bpy.data.objects[name].hide_render = False
        sockets.append(measure_sockets(arm))

        for layer in LAYERS:
            dest_dir = OUT / ("body" if layer == "body" else layer if layer != "light" else "light")
            if layer == "cape":
                dest_dir = OUT / "cape"
            elif layer == "heavy":
                dest_dir = OUT / "heavy"
            elif layer == "weapon":
                dest_dir = OUT / "weapon"
            dest_dir.mkdir(parents=True, exist_ok=True)
            path = dest_dir / f"run_f{i:02d}.png"
            if path.exists() and not args.overwrite:
                renders[layer].append(str(path))
                continue
            set_layer(layer)
            render_side(path)
            renders[layer].append(str(path))
            print(f"[d5] layer={layer} frame={i} -> {path}")

        for kind, key in (("A", "full_A"), ("B", "full_B")):
            path = D5 / "acabamento" / f"blender_{key}_f{i:02d}.png"
            path.parent.mkdir(parents=True, exist_ok=True)
            if path.exists() and not args.overwrite and i > 0:
                # still render frame 0 always for identity boards
                renders[key].append(str(path))
                continue
            if i <= 2 or args.overwrite:
                show_full_loadout(kind)
                render_side(path)
                renders[key].append(str(path))
                print(f"[d5] full {kind} frame={i}")

        # raw shaded sample frame 0 only
        if i == 0:
            path = D5 / "acabamento" / "blender_raw_shaded_f00.png"
            show_full_loadout("A")
            use_toon(False)
            use_camera("side")
            turn_rig(-90.0)
            R.configure()
            RR.set_resolution(256, 256)
            bpy.context.scene.render.filepath = str(path)
            bpy.context.scene.render.film_transparent = True
            bpy.ops.render.render(write_still=True)
            renders["raw_shaded"].append(str(path))
            print(f"[d5] raw shaded -> {path}")

    manifest = {
        "clip": "run",
        "frames": n,
        "fps": CL.FPS["run"],
        "action_hash": action_hash,
        "cell_note": "256x256 EEVEE toon side camera; sole pivot post-processed to 128x160 lab cell",
        "layers": list(LAYERS),
        "sockets_lab": sockets,
        "renders": renders,
        "equipment_objects": {
            "sword": sorted(SWORD),
            "cape": sorted(CAPE),
            "coat": sorted(COAT),
            "heavy": sorted(HEAVY),
        },
    }
    RIG_META.write_text(json.dumps(manifest, indent=2))
    print("[d5] wrote", RIG_META)


if __name__ == "__main__":
    main()
