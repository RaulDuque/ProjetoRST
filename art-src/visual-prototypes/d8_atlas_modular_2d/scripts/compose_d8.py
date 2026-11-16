#!/usr/bin/env python3
"""Marco C–E: loadouts, swaps, idle/run, cape method comparison."""
from __future__ import annotations

import json
import math
from pathlib import Path

from PIL import Image, ImageDraw

from d8_common import (
    CELL,
    DRAW_ORDER_CLASSIC,
    DRAW_ORDER_HEAVY,
    LAB_H,
    PIVOT,
    ROOT,
    board,
    canvas,
    feet_place,
    load_causeway,
    load_d2_frames,
    load_d2_idle,
    nearest_up,
    save_gif,
    scale_game,
    scale_to_height,
    strip_chroma,
)

ATLAS = ROOT / "atlas"
EQ = ROOT / "equipamentos"
LOAD_C = ROOT / "loadouts" / "classico"
LOAD_P = ROOT / "loadouts" / "pesado"
ANIM = ROOT / "animacoes"
CAPA = ROOT / "capa"
CMP = ROOT / "comparacoes"
SOCK = ROOT / "sockets"


def load_layer(name: str) -> Image.Image:
    for folder in (ATLAS, EQ):
        p = folder / f"{name}.png"
        if p.exists():
            return Image.open(p).convert("RGBA")
    return canvas()


def compose(parts: dict[str, Image.Image], order: list[str]) -> Image.Image:
    out = canvas()
    for key in order:
        im = parts.get(key)
        if im is not None and im.split()[-1].getbbox():
            out.alpha_composite(im)
    return out


def classic_parts() -> dict[str, Image.Image]:
    """Prefer full D2 cell for identity; keep modular layers available."""
    full = load_layer("full_classic_ref")
    if full.split()[-1].getbbox() is None:
        full = feet_place(scale_to_height(load_d2_idle(), LAB_H))
    return {
        "cape_back": load_layer("cape_back"),
        "body": full,  # identity anchor
        "pants": canvas(),
        "shirt": canvas(),
        "coat_front": canvas(),
        "boots": canvas(),
        "gloves": canvas(),
        "face": canvas(),
        "hair_front": canvas(),
        "scabbard": load_layer("scabbard"),
        "weapon": canvas(),  # already in full
        "_modular_body": load_layer("body"),
        "_coat": load_layer("coat_front"),
        "_cape": load_layer("cape_back"),
        "_weapon": load_layer("weapon"),
        "_boots": load_layer("boots"),
    }


def heavy_parts() -> dict[str, Image.Image]:
    return {
        "cape_back": load_layer("cape_d4") if (EQ / "cape_d4.png").exists() else load_layer("cape_back"),
        "body": load_layer("body_d4") if (EQ / "body_d4.png").exists() else load_layer("body"),
        "pants": canvas(),
        "boots": load_layer("boots"),
        "armor": load_layer("armor"),
        "pauldrons": load_layer("pauldrons"),
        "bracers": load_layer("bracers"),
        "gloves": load_layer("gloves"),
        "face": canvas(),
        "hair_front": canvas(),
        "helm": load_layer("helm"),
        "cape_front": canvas(),
        "weapon": load_layer("weapon_d4") if (EQ / "weapon_d4.png").exists() else load_layer("weapon"),
        "shield": load_layer("shield"),
    }


def occlusion_rules() -> dict:
    return {
        "helm_hides": ["hair_front", "hair_back", "face"],
        "coat_covers": ["pants"],
        "armor_covers": ["shirt", "coat_front"],
        "shield_plane": "front_of_torso_default",
        "weapon_follows": "hand_front",
        "scabbard_follows": "hip",
        "cape_follows": "back",
    }


def apply_occlusion(parts: dict[str, Image.Image], rules: dict, has_helm: bool) -> dict[str, Image.Image]:
    out = dict(parts)
    if has_helm:
        for k in rules.get("helm_hides", []):
            out[k] = canvas()
    return out


def load_sockets_frame(clip: str, index: int) -> dict:
    p = SOCK / f"{clip}_sockets.json"
    if not p.exists():
        return {}
    data = json.loads(p.read_text(encoding="utf-8"))
    frames = data.get("frames", [])
    if index >= len(frames):
        return {}
    return frames[index].get("sockets", {})


def place_at_socket(
    layer: Image.Image,
    socket: dict | None,
    base_socket: dict | None,
) -> Image.Image:
    """Translate layer so its content follows socket delta from base pose."""
    if not socket or not base_socket or layer.split()[-1].getbbox() is None:
        return layer
    dx = socket["x"] - base_socket["x"]
    dy = socket["y"] - base_socket["y"]
    if dx == 0 and dy == 0:
        return layer
    out = canvas()
    # shift pixels
    out.alpha_composite(layer, (dx, dy))
    return out


# --- Cape methods ---

def cape_method_a_frames() -> list[Image.Image]:
    """A: cape crops from D2 run frames (drawn sequence)."""
    frames = []
    for fr in load_d2_frames("run"):
        cell = feet_place(scale_to_height(fr, LAB_H))
        # wine / left bias cape extract
        cape = canvas()
        s, d = cell.load(), cape.load()
        box = cell.split()[-1].getbbox() or (0, 0, 128, 160)
        x0, y0, x1, y1 = box
        for y in range(CELL[1]):
            for x in range(CELL[0]):
                r, g, b, a = s[x, y]
                if a < 16:
                    continue
                wine = r > g + 12 and r > b and r > 28 and g < 70
                back = x < x0 + int((x1 - x0) * 0.42)
                if wine or (back and y0 + 10 < y < y1 - 8):
                    # only keep if likely cape (not boots)
                    if y < y1 - 6:
                        d[x, y] = (r, g, b, a)
        frames.append(cape)
    return frames


def cape_method_b_mesh(base_cape: Image.Image, t: float) -> Image.Image:
    """B: simple horizontal shear + vertical sway (pseudo mesh)."""
    box = base_cape.split()[-1].getbbox()
    if not box:
        return canvas()
    # Use affine transform for sway
    amp = 3.0 * math.sin(t * math.pi * 2)
    # PIL AFFINE: coeffs for inverse transform
    # x' = x + amp * (y/h)
    w, h = CELL
    # shear based on y
    out = canvas()
    s = base_cape.load()
    d = out.load()
    for y in range(h):
        shear = int(amp * ((y - box[1]) / max(1, box[3] - box[1])))
        for x in range(w):
            sx = x - shear
            if 0 <= sx < w:
                d[x, y] = s[sx, y]
    return out


def cape_method_c_sim(base_cape: Image.Image, velocity: float, t: float) -> Image.Image:
    """C: secondary motion from velocity — lag offset."""
    lag = int(-velocity * 2.5 + 2.0 * math.sin(t * math.pi * 2))
    out = canvas()
    out.alpha_composite(base_cape, (lag, abs(lag) // 2))
    return out


def build_cape_comparison(base_cape: Image.Image) -> None:
    CAPA.mkdir(parents=True, exist_ok=True)
    a_frames = cape_method_a_frames()
    for i, fr in enumerate(a_frames):
        fr.save(CAPA / "A_quadros" / f"cape_f{i:02d}.png")
    b_frames = [cape_method_b_mesh(base_cape, i / max(1, len(a_frames) - 1)) for i in range(8)]
    for i, fr in enumerate(b_frames):
        fr.save(CAPA / "B_malha" / f"cape_f{i:02d}.png")
    c_frames = [
        cape_method_c_sim(base_cape, velocity=1.0, t=i / 7) for i in range(8)
    ]
    for i, fr in enumerate(c_frames):
        fr.save(CAPA / "C_simulacao" / f"cape_f{i:02d}.png")

    board(
        [
            ("A quadro0", nearest_up(a_frames[0] if a_frames else canvas(), 2)),
            ("A quadro3", nearest_up(a_frames[3] if len(a_frames) > 3 else canvas(), 2)),
            ("B malha", nearest_up(b_frames[2], 2)),
            ("C sim", nearest_up(c_frames[2], 2)),
        ],
        CMP / "10_capa_metodos.png",
        "Capa: A quadros | B malha | C simulacao",
    )
    save_gif(a_frames[:8] or b_frames, ROOT / "previews" / "capa_A.gif", 70)
    save_gif(b_frames, ROOT / "previews" / "capa_B.gif", 70)
    save_gif(c_frames, ROOT / "previews" / "capa_C.gif", 70)

    (CAPA / "COMPARACAO_METODOS.md").write_text(
        "# Métodos de capa\n\n"
        "| Método | Fidelidade | Textura | Silhueta | Custo | Reprodutível |\n"
        "|--------|------------|---------|----------|-------|-------------|\n"
        "| A quadros D2 | Alta | Preserva pixels | Boa | Alto (arte) | Sim |\n"
        "| B malha shear | Média | Pode esticar | Risco | Baixo | Sim |\n"
        "| C simulação | Baixa–média | Offset só | OK | Baixo | Sim |\n\n"
        "**Escolha D8:** método **A** para o piloto de corrida (fidelidade D7-B), "
        "com **C** como reforço leve de offset se A falhar em um quadro.\n",
        encoding="utf-8",
    )


def animate_classic(clip: str) -> list[Image.Image]:
    """Body = D2 frames; cape secondary from method A on run."""
    frames = load_d2_frames(clip)
    out = []
    cape_a = cape_method_a_frames() if clip == "run" else []
    base_socks = load_sockets_frame(clip, 0)
    for i, fr in enumerate(frames):
        cell = feet_place(scale_to_height(fr, LAB_H))
        cell = strip_chroma(cell)
        if clip == "run" and cape_a:
            # optional: composite cape A under — but D2 already has cape;
            # store body as-is to preserve D7-B; cape gif is separate proof
            pass
        out.append(cell)
    return out


def animate_heavy(clip: str) -> list[Image.Image]:
    """Shared body animation + heavy gear following sockets."""
    frames = load_d2_frames(clip)
    armor = load_layer("armor")
    helm = load_layer("helm")
    shield = load_layer("shield")
    base_socks = load_sockets_frame(clip, 0)
    out = []
    for i, fr in enumerate(frames):
        body = feet_place(scale_to_height(fr, LAB_H))
        body = strip_chroma(body)
        socks = load_sockets_frame(clip, i)
        composed = body.copy()
        # Follow body_center / back / head deltas
        arm = place_at_socket(armor, socks.get("body_center"), base_socks.get("body_center"))
        hm = place_at_socket(helm, socks.get("head"), base_socks.get("head"))
        sh = place_at_socket(shield, socks.get("hand_back"), base_socks.get("hand_back"))
        # occlusion: hide hair region under helm by drawing helm last
        composed.alpha_composite(arm)
        composed.alpha_composite(sh)
        composed.alpha_composite(hm)
        out.append(composed)
    return out


def swap_board(classic_full: Image.Image) -> None:
    base = {
        "cape_back": load_layer("cape_back"),
        "body": classic_full,
        "armor": canvas(),
        "helm": canvas(),
        "shield": canvas(),
        "weapon": canvas(),
        "coat": canvas(),
        "gloves": load_layer("gloves"),
        "boots": load_layer("boots"),
    }
    order = ["cape_back", "body", "coat", "armor", "boots", "gloves", "weapon", "shield", "helm"]
    tests = []
    # 1 helm
    t = dict(base)
    t["helm"] = load_layer("helm")
    tests.append(("helm", compose(t, order)))
    # 2 coat (D4)
    t = dict(base)
    t["coat"] = load_layer("coat_d4")
    tests.append(("casaco", compose(t, order)))
    # 3 peitoral
    t = dict(base)
    t["armor"] = load_layer("armor")
    tests.append(("peitoral", compose(t, order)))
    # 4 gloves
    t = dict(base)
    t["gloves"] = load_layer("gloves")
    tests.append(("luvas", compose(t, order)))
    # 5 boots
    t = dict(base)
    t["boots"] = load_layer("boots")
    tests.append(("botas", compose(t, order)))
    # 6 cape off
    t = dict(base)
    t["cape_back"] = canvas()
    tests.append(("capa_off", compose(t, order)))
    # 7 sword d4
    t = dict(base)
    t["weapon"] = load_layer("weapon_d4")
    tests.append(("espada", compose(t, order)))
    # 8 shield
    t = dict(base)
    t["shield"] = load_layer("shield")
    tests.append(("escudo", compose(t, order)))

    for name, im in tests:
        im.save(ROOT / "testes" / f"swap_{name}.png")
    board(
        [(n, nearest_up(im, 2)) for n, im in tests],
        CMP / "07_troca_individual.png",
        "Troca individual (corpo D2 + peca)",
    )


def stress_combos(classic_full: Image.Image) -> None:
    combos = [
        ("coat+heavy_boots", ["cape_back", "body", "coat_d4", "boots"]),
        ("armor_no_helm", ["body_d4", "armor", "weapon_d4"]),
        ("coat+helm", ["body", "coat_d4", "helm"]),
        ("armor+cape", ["cape_d4", "body_d4", "armor", "helm"]),
        ("coat+shield", ["body", "coat_d4", "shield"]),
        ("armor+weapon", ["body_d4", "armor", "helm", "weapon_d4", "shield"]),
    ]
    items = []
    for name, keys in combos:
        parts = {}
        order = []
        for k in keys:
            im = load_layer(k if k != "body" else "full_classic_ref")
            if k == "body":
                im = classic_full
            parts[k] = im
            order.append(k)
        # remap
        mapped = {}
        ord2 = []
        for k in order:
            key = {
                "coat_d4": "coat",
                "cape_d4": "cape",
                "cape_back": "cape",
                "body_d4": "body",
                "full_classic_ref": "body",
                "weapon_d4": "weapon",
            }.get(k, k)
            mapped[key] = parts[k]
            if key not in ord2:
                ord2.append(key)
        im = compose(mapped, ord2)
        im.save(ROOT / "testes" / f"stress_{name}.png")
        items.append((name[:18], nearest_up(im, 2)))
    board(items, CMP / "07b_stress_modular.png", "Estresse modular")


def causeway(sprites: list[tuple[str, Image.Image]]) -> None:
    bg = load_causeway()
    games = [(lab, scale_game(im)) for lab, im in sprites]
    if bg is None:
        board([(l, nearest_up(g, 3)) for l, g in games], CMP / "11_causeway_escala_real.png", "causeway fallback")
        return
    bg = bg.convert("RGBA")
    ch = max(90, max(im.height for _, im in games) + 36)
    y0 = max(0, bg.height - ch - 20)
    strip = bg.crop((0, y0, min(bg.width, 420), y0 + ch))
    out = Image.new("RGBA", (max(strip.width, 400), strip.height), (20, 18, 28, 255))
    out.alpha_composite(strip, (0, 0))
    d = ImageDraw.Draw(out)
    x = 8
    ground = out.height - 10
    for lab, im in games:
        out.alpha_composite(im, (x, ground - im.height))
        d.text((x, 4), lab, fill=(230, 210, 170, 255))
        x += im.width + 8
    out.convert("RGB").save(CMP / "11_causeway_escala_real.png")


def main() -> None:
    LOAD_C.mkdir(parents=True, exist_ok=True)
    LOAD_P.mkdir(parents=True, exist_ok=True)
    ANIM.mkdir(parents=True, exist_ok=True)
    (ROOT / "testes").mkdir(parents=True, exist_ok=True)

    rules = occlusion_rules()
    (ROOT / "REGRAS_OCLUSAO.md").write_text(
        "# Regras de oclusão D8\n\n```json\n"
        + json.dumps(rules, indent=2)
        + "\n```\n\nOrdem clássica: "
        + " → ".join(DRAW_ORDER_CLASSIC)
        + "\n\nOrdem pesada: "
        + " → ".join(DRAW_ORDER_HEAVY)
        + "\n",
        encoding="utf-8",
    )

    cp = classic_parts()
    classic = compose(
        apply_occlusion(cp, rules, has_helm=False),
        ["cape_back", "body", "scabbard", "weapon"],
    )
    # If cape empty, body alone is fine (body is full D2)
    classic.save(LOAD_C / "pose.png")
    nearest_up(classic, 3).save(LOAD_C / "pose_3x.png")
    scale_game(classic).save(LOAD_C / "pose_game.png")

    hp = apply_occlusion(heavy_parts(), rules, has_helm=True)
    heavy = compose(hp, DRAW_ORDER_HEAVY)
    heavy.save(LOAD_P / "pose.png")
    nearest_up(heavy, 3).save(LOAD_P / "pose_3x.png")
    scale_game(heavy).save(LOAD_P / "pose_game.png")

    board(
        [("classico", nearest_up(classic, 2)), ("game", nearest_up(scale_game(classic), 3))],
        CMP / "05_loadout_classico.png",
        "Loadout A — classico D7-B",
    )
    board(
        [("pesado", nearest_up(heavy, 2)), ("game", nearest_up(scale_game(heavy), 3))],
        CMP / "06_loadout_pesado.png",
        "Loadout B — cavaleiro gotico",
    )

    swap_board(classic)
    stress_combos(classic)

    base_cape = load_layer("cape_back")
    if base_cape.split()[-1].getbbox() is None:
        base_cape = load_layer("cape_d4")
    build_cape_comparison(base_cape)

    idle_c = animate_classic("idle")
    run_c = animate_classic("run")
    idle_h = animate_heavy("idle")
    run_h = animate_heavy("run")

    for i, fr in enumerate(idle_c):
        fr.save(ANIM / "idle" / f"classico_f{i:02d}.png")
    for i, fr in enumerate(run_c):
        fr.save(ANIM / "run" / f"classico_f{i:02d}.png")
    for i, fr in enumerate(idle_h):
        fr.save(ANIM / "idle" / f"pesado_f{i:02d}.png")
    for i, fr in enumerate(run_h):
        fr.save(ANIM / "run" / f"pesado_f{i:02d}.png")

    save_gif(idle_c, ROOT / "previews" / "idle_classico.gif", 100)
    save_gif(idle_h, ROOT / "previews" / "idle_pesado.gif", 100)
    save_gif(run_c, ROOT / "previews" / "run_classico.gif", 70)
    save_gif(run_h, ROOT / "previews" / "run_pesado.gif", 70)
    # dynamic swap gif: classic ↔ heavy alternating
    dyn = []
    for i in range(min(len(run_c), len(run_h))):
        dyn.append(run_c[i] if i % 2 == 0 else run_h[i])
    save_gif(dyn, ROOT / "previews" / "troca_dinamica.gif", 90)

    board(
        [
            ("idle0 C", nearest_up(idle_c[0], 2)),
            ("idle3 C", nearest_up(idle_c[min(3, len(idle_c) - 1)], 2)),
            ("idle0 H", nearest_up(idle_h[0], 2)),
            ("idle3 H", nearest_up(idle_h[min(3, len(idle_h) - 1)], 2)),
        ],
        CMP / "08_idle_comparacao.png",
        "Idle classico vs pesado",
    )
    board(
        [
            ("run0 C", nearest_up(run_c[0], 2)),
            ("run3 C", nearest_up(run_c[min(3, len(run_c) - 1)], 2)),
            ("run0 H", nearest_up(run_h[0], 2)),
            ("run3 H", nearest_up(run_h[min(3, len(run_h) - 1)], 2)),
        ],
        CMP / "09_run_comparacao.png",
        "Run classico vs pesado",
    )

    d7b = ROOT.parent / "d7_pixel_programatico" / "estrategias" / "B_sprites_masks" / "pose_estatica.png"
    d7_im = Image.open(d7b).convert("RGBA") if d7b.exists() else classic
    causeway(
        [
            ("D2", classic),
            ("D7B", d7_im),
            ("D8C", classic),
            ("D8H", heavy),
        ]
    )

    board(
        [
            ("D7-B", nearest_up(d7_im, 2)),
            ("D8 classico", nearest_up(classic, 2)),
            ("D8 pesado", nearest_up(heavy, 2)),
            ("game C", nearest_up(scale_game(classic), 3)),
            ("game H", nearest_up(scale_game(heavy), 3)),
        ],
        CMP / "12_resultado_final.png",
        "D8 resultado para revisao",
    )

    # Refresh 01 with D7 vs D8
    board(
        [
            ("D7-B", nearest_up(d7_im, 2)),
            ("D8 classico", nearest_up(classic, 2)),
            ("D8 pesado", nearest_up(heavy, 2)),
        ],
        CMP / "01_D7_vs_D8.png",
        "D7-B vs D8",
    )
    print("compose_d8 done")


if __name__ == "__main__":
    main()
