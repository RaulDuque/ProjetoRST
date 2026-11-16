"""Export D6 modular layers: light groups, heavy plates, run stacks.

  blender --background --factory-startup \\
    art-src/visual-prototypes/d6_fidelidade_d2/blender/swordsman_rig_v007_d6_lab.blend \\
    --python art-src/visual-prototypes/d6_fidelidade_d2/scripts/export_d6_layers.py -- \\
    [--limit N] [--overwrite]
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

SCRIPTS = Path(__file__).resolve().parents[3] / "player" / "scripts" / "bpy"
D6 = Path(__file__).resolve().parents[1]

for entry in (
    SCRIPTS,
    SCRIPTS / "swordsman_v003",
    SCRIPTS / "swordsman_v004",
    SCRIPTS / "swordsman_rig_v005",
    SCRIPTS / "swordsman_rig_v006",
    SCRIPTS / "swordsman_rig_v007",
    SCRIPTS / "game_sprites_v1_3",
):
    s = str(entry)
    if s in sys.path:
        sys.path.remove(s)
    sys.path.insert(0, s)

import bpy  # noqa: E402

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
OLD_HEAVY = {"pauldron", "pauldron_lame"}
HEAVY = {n for n in bpy.data.objects.keys() if n.startswith("d6_")}
BOOTS = {n for n in bpy.data.objects.keys() if n.startswith("boot_")}
GLOVES = {n for n in bpy.data.objects.keys() if n.startswith("glove_")}
PANTS = {"torso_trousers", "trouser_fly", "leg_far", "leg_near"}
SHIRT = {
    "torso_vest",
    "vest_button_0",
    "vest_button_1",
    "vest_button_2",
    "vest_button_3",
    "vest_hem_trim",
    "vest_placket",
    "collar",
    "sleeve_far",
    "sleeve_near",
    "cuff_far",
    "cuff_near",
    "cuff_band_far",
    "cuff_band_near",
}
EQUIP = SWORD | CAPE | COAT | OLD_HEAVY | HEAVY


def parse_args() -> argparse.Namespace:
    argv = sys.argv[sys.argv.index("--") + 1 :] if "--" in sys.argv else []
    p = argparse.ArgumentParser()
    p.add_argument("--limit", type=int, default=0)
    p.add_argument("--overwrite", action="store_true")
    p.add_argument("--statics-only", action="store_true")
    return p.parse_args(argv)


def meshes() -> list[bpy.types.Object]:
    return [o for o in bpy.data.objects if o.type == "MESH"]


def set_visible(names: set[str] | None) -> None:
    """If names is None, show everything. Else show only those meshes."""
    for obj in meshes():
        if names is None:
            show = True
        else:
            show = obj.name in names
        obj.hide_render = not show
        obj.hide_viewport = not show


def body_only() -> None:
    set_visible(None)
    for obj in meshes():
        if obj.name in EQUIP:
            obj.hide_render = True
            obj.hide_viewport = True


def render_to(path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    use_toon(True)
    use_camera("side")
    turn_rig(-90.0)
    R.configure()
    RR.set_resolution(256, 256)
    bpy.context.scene.render.filepath = str(path)
    bpy.context.scene.render.image_settings.file_format = "PNG"
    bpy.context.scene.render.film_transparent = True
    bpy.ops.render.render(write_still=True)
    use_toon(False)


def ensure_shield() -> None:
    if "d6_shield" in bpy.data.objects:
        return
    arm = bpy.data.objects[C.ARMATURE_NAME]
    bpy.ops.mesh.primitive_cylinder_add(radius=0.12, depth=0.04)
    obj = bpy.context.active_object
    obj.name = "d6_shield"
    obj.rotation_euler[0] = 1.5708
    obj.parent = arm
    obj.parent_type = "BONE"
    obj.parent_bone = "hand.L"
    obj.location = (0.06, 0.0, 0.02)
    mat = bpy.data.materials.get("D6_ArmorMetal")
    if mat:
        if obj.data.materials:
            obj.data.materials[0] = mat
        else:
            obj.data.materials.append(mat)
    coll = bpy.data.collections.get("D6_heavy_armor")
    if coll:
        for c in list(obj.users_collection):
            c.objects.unlink(obj)
        coll.objects.link(obj)
    HEAVY.add("d6_shield")


def main() -> None:
    args = parse_args()
    ensure_shield()
    HEAVY.update(n for n in bpy.data.objects.keys() if n.startswith("d6_"))

    arm = bpy.data.objects[C.ARMATURE_NAME]
    body = PS.all_body()
    R.configure()
    bpy.context.scene.render.fps = C.PRODUCTION_FPS
    PS.reset(arm)

    # --- static piece sheets (POSE_P01 via reset / neutral) ---
    static_dir = D6 / "equipamentos"
    piece_map = {
        "pesados/cuirass": {"d6_cuirass"},
        "pesados/pauldron_R": {"d6_pauldron_R"},
        "pesados/pauldron_L": {"d6_pauldron_L"},
        "pesados/bracer_R": {"d6_bracer_R"},
        "pesados/bracer_L": {"d6_bracer_L"},
        "pesados/gauntlet_R": {"d6_gauntlet_R"},
        "pesados/gauntlet_L": {"d6_gauntlet_L"},
        "pesados/fauld": {"d6_fauld"},
        "pesados/cuisse_R": {"d6_cuisse_R"},
        "pesados/cuisse_L": {"d6_cuisse_L"},
        "pesados/greave_R": {"d6_greave_R"},
        "pesados/greave_L": {"d6_greave_L"},
        "pesados/sabaton_R": {"d6_sabaton_R"},
        "pesados/sabaton_L": {"d6_sabaton_L"},
        "cabeca/helm": {"d6_helm"},
        "armas/shield": {"d6_shield"},
        "armas/sword": SWORD,
        "capas/cape": CAPE,
        "leves/coat": COAT,
        "leves/shirt": SHIRT & {o.name for o in meshes()},
        "pernas/pants": PANTS & {o.name for o in meshes()},
        "maos/gloves": GLOVES,
        "botas/boots": BOOTS,
    }
    for rel, names in piece_map.items():
        path = static_dir / f"{rel}.png"
        if path.exists() and not args.overwrite:
            continue
        set_visible(set(names))
        render_to(path)
        print("[d6] static", path)

    # full static masters
    masters = {
        "personagem/D6_A/idle_side_toon.png": "A",
        "personagem/D6_A/idle_side_heavy.png": "B",
    }
    # A = light traveler
    body_only()
    for obj in meshes():
        if obj.name in (COAT | CAPE | SWORD | BOOTS | GLOVES | (SHIRT & {o.name for o in meshes()}) | (PANTS & {o.name for o in meshes()})):
            obj.hide_render = False
            obj.hide_viewport = False
    render_to(D6 / "personagem/D6_A/idle_side_toon.png")
    # B heavy
    body_only()
    for obj in meshes():
        if obj.name in (HEAVY | CAPE | SWORD | {"d6_shield"}):
            obj.hide_render = False
            obj.hide_viewport = False
    # hide hair under helm optionally? keep for now
    render_to(D6 / "personagem/D6_A/idle_side_heavy.png")
    print("[d6] static masters")

    if args.statics_only:
        return

    # --- run layers ---
    n = args.limit or CL.CLIPS["run"].frames
    action = CL.build_action(arm, "run", body)
    arm.animation_data.action = action
    layer_dirs = {
        "body": "animacoes/run/body",
        "coat": "equipamentos/leves/run_coat",
        "cape": "equipamentos/capas/run_cape",
        "sword": "equipamentos/armas/run_sword",
        "heavy": "equipamentos/pesados/run_set",
        "helm": "equipamentos/cabeca/run_helm",
        "shield": "equipamentos/armas/run_shield",
        "boots": "equipamentos/botas/run_boots",
        "gloves": "equipamentos/maos/run_gloves",
    }
    for i in range(n):
        bpy.context.scene.frame_set(i + 1)
        bpy.context.view_layer.update()

        def out(key: str) -> Path:
            return D6 / layer_dirs[key] / f"run_f{i:02d}.png"

        # body
        p = out("body")
        if not p.exists() or args.overwrite:
            body_only()
            render_to(p)
        # coat
        p = out("coat")
        if not p.exists() or args.overwrite:
            set_visible(COAT)
            render_to(p)
        # cape
        p = out("cape")
        if not p.exists() or args.overwrite:
            set_visible(CAPE)
            render_to(p)
        # sword
        p = out("sword")
        if not p.exists() or args.overwrite:
            set_visible(SWORD)
            render_to(p)
        # heavy set (all d6 plates except shield/helm optional separate)
        p = out("heavy")
        if not p.exists() or args.overwrite:
            set_visible({n for n in HEAVY if n not in {"d6_helm", "d6_shield"}})
            render_to(p)
        # helm
        p = out("helm")
        if not p.exists() or args.overwrite:
            set_visible({"d6_helm"})
            render_to(p)
        # shield
        p = out("shield")
        if not p.exists() or args.overwrite:
            set_visible({"d6_shield"})
            render_to(p)
        # boots / gloves
        p = out("boots")
        if not p.exists() or args.overwrite:
            set_visible(BOOTS)
            render_to(p)
        p = out("gloves")
        if not p.exists() or args.overwrite:
            set_visible(GLOVES)
            render_to(p)
        print(f"[d6] run frame {i}")

    (D6 / "testes" / "export_manifest.json").write_text(
        json.dumps(
            {
                "frames": n,
                "fps": CL.FPS["run"],
                "heavy_pieces": sorted(HEAVY),
                "layers": list(layer_dirs),
            },
            indent=2,
        )
    )
    print("[d6] export done")


if __name__ == "__main__":
    main()
