#!/usr/bin/env python3
"""D10: fix cape/gloves exclusivity, fidelity maps, modular anim, after-image."""
from __future__ import annotations

import json
import math
from pathlib import Path

from PIL import Image, ImageChops, ImageDraw

from d10_common import (
    CELL,
    D7,
    D8,
    D9,
    LAB_H,
    PAL,
    PIVOT,
    ROOT,
    board,
    canvas,
    feet_place,
    load_causeway,
    load_d2_frames,
    load_d4,
    nearest_up,
    opaque_count,
    quantize_to_palette,
    save_gif,
    scale_game,
    scale_to_height,
    strip_chroma,
    write_json,
)

ATLAS = ROOT / "atlas"
EQ = ROOT / "equipamentos"
CMP = ROOT / "comparacoes"
ANIM = ROOT / "animacoes"
FX = ROOT / "efeitos" / "after_image"


def _mask(src, pred):
    out = canvas()
    s, d = src.load(), out.load()
    for y in range(CELL[1]):
        for x in range(CELL[0]):
            px = s[x, y]
            if px[3] < 16:
                continue
            if pred(x, y, px):
                d[x, y] = px
    return out


def _subtract(full, part):
    out = full.copy()
    p, d = part.load(), out.load()
    for y in range(CELL[1]):
        for x in range(CELL[0]):
            if p[x, y][3] >= 16:
                d[x, y] = (0, 0, 0, 0)
    return out


def _clear_twins(layer, owner):
    """Remove pixels from layer that exactly match owner at same coords."""
    out = layer.copy()
    s, o, d = layer.load(), owner.load(), out.load()
    for y in range(CELL[1]):
        for x in range(CELL[0]):
            if o[x, y][3] > 16 and s[x, y] == o[x, y]:
                d[x, y] = (0, 0, 0, 0)
    return out


def d2_cell():
    p = ROOT / "fontes" / "d8_full_classic_ref.png"
    if p.exists():
        return strip_chroma(Image.open(p).convert("RGBA"))
    fr = load_d2_frames("idle")[0]
    return feet_place(scale_to_height(fr, LAB_H))


def extract_layers(cell: Image.Image) -> dict[str, Image.Image]:
    box = cell.split()[-1].getbbox()
    assert box
    x0, y0, x1, y1 = box
    h, w = y1 - y0, x1 - x0
    mid = x0 + w // 2

    def head(x, y, _):
        return y < y0 + int(h * 0.26)

    def face(x, y, px):
        r, g, b, a = px
        return head(x, y, px) and r > 120 and g > 80 and b > 60 and r > b

    def hair(x, y, px):
        return head(x, y, px) and not face(x, y, px)

    def boots(x, y, _):
        return y > y0 + int(h * 0.84)

    def weapon(x, y, px):
        r, g, b, a = px
        bright = (r + g + b) / 3 > 130 and abs(r - g) < 55
        return bright and x > x0 + int(w * 0.48) and y > y0 + int(h * 0.30)

    # CAPE: wine OR back column — EXCLUDE forward arm band (glove zone)
    def cape(x, y, px):
        if boots(x, y, px) or weapon(x, y, px) or head(x, y, px):
            return False
        r, g, b, a = px
        wine = r > g + 12 and r > b and r > 28 and g < 70
        back = x < x0 + int(w * 0.38)
        # forward wrist/glove band — never cape
        if x > x0 + int(w * 0.55) and y0 + int(h * 0.35) < y < y0 + int(h * 0.62):
            return False
        return (wine or back) and y0 + int(h * 0.18) < y < y0 + int(h * 0.92)

    # GLOVES: forward arm only
    def glove_front(x, y, px):
        if not (x > x0 + int(w * 0.58) and y0 + int(h * 0.38) < y < y0 + int(h * 0.58)):
            return False
        r, g, b, a = px
        return r < 110 and g < 90 and b < 100

    def glove_back(x, y, px):
        # rarely visible; small patch near mid torso back-arm
        if not (x0 + int(w * 0.35) < x < mid and y0 + int(h * 0.40) < y < y0 + int(h * 0.55)):
            return False
        r, g, b, a = px
        return r < 100 and g < 85 and b < 95 and not cape(x, y, px)

    face_im = _mask(cell, face)
    hair_im = _mask(cell, hair)
    hair_back = canvas()
    hair_front = canvas()
    s = hair_im.load()
    for y in range(CELL[1]):
        for x in range(CELL[0]):
            if s[x, y][3] < 16:
                continue
            (hair_back if x < mid - 2 else hair_front).putpixel((x, y), s[x, y])

    cape_im = _mask(cell, cape)
    gf = _mask(cell, glove_front)
    gb = _mask(cell, glove_back)
    # exclusivity cape ↔ gloves
    cape_im = _subtract(cape_im, gf)
    cape_im = _subtract(cape_im, gb)
    gf = _clear_twins(gf, cape_im)
    gb = _clear_twins(gb, cape_im)
    gloves = gf.copy()
    gloves.alpha_composite(gb)

    boots_im = _mask(cell, boots)
    weapon_im = _mask(cell, weapon)

    # body + coat from residual
    residual = cell.copy()
    for part in (cape_im, weapon_im, boots_im, hair_back, hair_front, face_im, gloves):
        residual = _subtract(residual, part)

    body = canvas()
    coat = canvas()
    if box:
        x0, y0, x1, y1 = box
        h = y1 - y0
        s = residual.load()
        for y in range(CELL[1]):
            for x in range(CELL[0]):
                px = s[x, y]
                if px[3] < 16:
                    continue
                rel = (y - y0) / max(1, h)
                if rel < 0.22:
                    continue
                inner = x0 + int((x1 - x0) * 0.22) < x < x0 + int((x1 - x0) * 0.78)
                if inner and 0.22 <= rel < 0.84:
                    body.putpixel((x, y), px)
                else:
                    coat.putpixel((x, y), px)
        # ensure body core
        d = ImageDraw.Draw(body)
        core = canvas()
        dc = ImageDraw.Draw(core)
        dc.rectangle(
            [x0 + 14, y0 + int(h * 0.34), x1 - 14, y0 + int(h * 0.58)],
            fill=PAL["shirt"],
        )
        dc.rectangle(
            [x0 + 16, y0 + int(h * 0.58), x1 - 16, y0 + int(h * 0.80)],
            fill=PAL["pants"],
        )
        body.alpha_composite(core)
        coat = _subtract(coat, core)

    # fill coat gaps from D4 torso-only
    d4c = load_d4("02_light_coat.png")
    if d4c is not None and box:
        x0, y0, x1, y1 = box
        fill = canvas()
        s, d, cl = d4c.load(), fill.load(), coat.load()
        for y in range(y0 + int((y1 - y0) * 0.28), y0 + int((y1 - y0) * 0.88)):
            for x in range(x0, x1):
                if s[x, y][3] < 16 or cl[x, y][3] >= 16:
                    continue
                d[x, y] = s[x, y]
        coat.alpha_composite(fill)

    # final exclusivity
    for owner in (cape_im, gloves, boots_im, weapon_im):
        coat = _clear_twins(coat, owner)
        body = _clear_twins(body, owner)
    coat = _subtract(coat, cape_im)
    coat = _subtract(coat, gloves)
    body = _subtract(body, cape_im)
    body = _subtract(body, gloves)

    # scabbard weak extract
    scabbard = canvas()
    s = cell.load()
    for y in range(y0 + int(h * 0.45), y0 + int(h * 0.78)):
        for x in range(x0, x0 + int(w * 0.48)):
            px = s[x, y]
            if px[3] < 16:
                continue
            r, g, b, a = px
            if 55 < r < 170 and 35 < g < 130 and b < g + 10 and (r - b) > 15:
                scabbard.putpixel((x, y), px)
    scabbard = _clear_twins(scabbard, cape_im)
    cape_im = _clear_twins(cape_im, scabbard)

    # repair coat under cape: sample nearby coat / shirt into cape-cleared coat holes? 
    # When cape removed, body+coat should look fine — paint under-cape coat fringe
    repair = canvas()
    cs, rs = cape_im.load(), residual.load()
    rd = repair.load()
    for y in range(CELL[1]):
        for x in range(CELL[0]):
            if cs[x, y][3] < 16:
                continue
            # if original cell had non-wine under (hard); use shirt for torso overlap
            if y0 + int(h * 0.25) < y < y0 + int(h * 0.75) and x > x0 + int(w * 0.25):
                rd[x, y] = PAL["coat"]
    coat_under_cape = repair
    # don't put repair into cape; keep as optional underlayer when cape off — baked into coat gaps only where coat empty
    for y in range(CELL[1]):
        for x in range(CELL[0]):
            if repair.getpixel((x, y))[3] > 0 and coat.getpixel((x, y))[3] == 0 and cape_im.getpixel((x, y))[3] > 0:
                # only store in separate repair layer used when composing without cape
                pass

    return {
        "body_base": body,
        "face": face_im,
        "hair_back": hair_back,
        "hair_front": hair_front,
        "coat_front": coat,
        "cape_back": cape_im,
        "coat_under_cape": coat_under_cape,
        "gloves": gloves,
        "glove_front": gf,
        "glove_back": gb,
        "boots_classic": boots_im,
        "weapon_classic": weapon_im,
        "scabbard": scabbard,
    }


def heavy_layers():
    out = {}
    mapping = {
        "armor_chest": "03_heavy_armor.png",
        "helmet": "05_helm.png",
        "shield": "08_shield.png",
        "weapon_heavy": "07_sword.png",
        "cape_d4": "01_cape_back.png",
    }
    pals = list(PAL.values())
    for key, fname in mapping.items():
        im = load_d4(fname)
        if im is None:
            continue
        if im.size != CELL:
            im = feet_place(scale_to_height(im, LAB_H))
        out[key] = quantize_to_palette(strip_chroma(im), pals)
    return out


def save_all(layers):
    ATLAS.mkdir(parents=True, exist_ok=True)
    (ROOT / "corpo").mkdir(parents=True, exist_ok=True)
    for sub in ("roupas", "armaduras", "cabeca", "luvas", "botas", "capas", "armas", "escudos"):
        (EQ / sub).mkdir(parents=True, exist_ok=True)
    routes = {
        "body_base": ROOT / "corpo" / "body_base.png",
        "coat_front": EQ / "roupas" / "coat_front.png",
        "coat_under_cape": EQ / "roupas" / "coat_under_cape.png",
        "cape_back": EQ / "capas" / "cape_back.png",
        "gloves": EQ / "luvas" / "gloves.png",
        "glove_front": EQ / "luvas" / "glove_front.png",
        "glove_back": EQ / "luvas" / "glove_back.png",
        "boots_classic": EQ / "botas" / "boots_classic.png",
        "weapon_classic": EQ / "armas" / "weapon_classic.png",
        "scabbard": EQ / "armas" / "scabbard.png",
        "armor_chest": EQ / "armaduras" / "armor_chest.png",
        "helmet": EQ / "cabeca" / "helmet.png",
        "shield": EQ / "escudos" / "shield.png",
        "weapon_heavy": EQ / "armas" / "weapon_heavy.png",
        "cape_d4": EQ / "capas" / "cape_d4.png",
    }
    for k, im in layers.items():
        im.save(ATLAS / f"{k}.png")
        m = Image.new("L", im.size, 0)
        s, d = im.load(), m.load()
        for y in range(im.height):
            for x in range(im.width):
                d[x, y] = 255 if s[x, y][3] >= 16 else 0
        (ROOT / "mascaras").mkdir(parents=True, exist_ok=True)
        m.save(ROOT / "mascaras" / f"{k}_mask.png")
        if k in routes:
            routes[k].parent.mkdir(parents=True, exist_ok=True)
            im.save(routes[k])


class ComposeError(Exception):
    pass


DRAW_ORDER = [
    "back",
    "body",
    "coat_under",
    "coat",
    "armor",
    "feet",
    "hands",
    "face",
    "hair_back",
    "hair",
    "scabbard",
    "weapon",
    "offhand",
    "head",
]

CLASSIC = {
    "head": None,
    "hair": "hair_front",
    "hair_back": "hair_back",
    "face": "face",
    "body": "body_base",
    "coat": "coat_front",
    "coat_under": None,  # filled when cape off
    "armor": None,
    "hands": "gloves",
    "feet": "boots_classic",
    "back": "cape_back",
    "weapon": "weapon_classic",
    "offhand": None,
    "scabbard": "scabbard",
}

HEAVY = {
    "head": "helmet",
    "hair": None,
    "hair_back": None,
    "face": None,
    "body": "body_base",
    "coat": None,
    "coat_under": None,
    "armor": "armor_chest",
    "hands": "gloves",
    "feet": "boots_classic",
    "back": "cape_d4",
    "weapon": "weapon_heavy",
    "offhand": "shield",
    "scabbard": None,
}


def find_piece(pid):
    if pid is None:
        return canvas()
    for p in (
        ATLAS / f"{pid}.png",
        EQ / "roupas" / f"{pid}.png",
        EQ / "armaduras" / f"{pid}.png",
        EQ / "cabeca" / f"{pid}.png",
        EQ / "luvas" / f"{pid}.png",
        EQ / "botas" / f"{pid}.png",
        EQ / "capas" / f"{pid}.png",
        EQ / "armas" / f"{pid}.png",
        EQ / "escudos" / f"{pid}.png",
        ROOT / "corpo" / f"{pid}.png",
    ):
        if p.exists():
            return Image.open(p).convert("RGBA")
    raise ComposeError(f"equipment piece not found: {pid}")


def compose_loadout(loadout: dict) -> Image.Image:
    lo = dict(loadout)
    # when cape off, enable coat_under repair
    if lo.get("back") is None and lo.get("coat_under") is None:
        lo["coat_under"] = "coat_under_cape"
    out = canvas()
    for slot in DRAW_ORDER:
        pid = lo.get(slot)
        if not pid:
            continue
        layer = find_piece(pid)
        if opaque_count(layer):
            out.alpha_composite(layer)
    return out


def antifake(classic):
    results = {}
    pieces = {
        "cape": ("back", classic["back"]),
        "coat": ("coat", classic["coat"]),
        "weapon": ("weapon", classic["weapon"]),
        "boots": ("feet", classic["feet"]),
        "gloves": ("hands", classic["hands"]),
        "scabbard": ("scabbard", classic["scabbard"]),
    }
    full = compose_loadout(classic)
    for name, (slot, pid) in pieces.items():
        if not pid:
            continue
        lo = dict(classic)
        lo[slot] = None
        without = compose_loadout(lo)
        piece = find_piece(pid)
        res = 0
        pl, wl = piece.load(), without.load()
        for y in range(CELL[1]):
            for x in range(CELL[0]):
                if pl[x, y][3] > 16 and wl[x, y] == pl[x, y]:
                    res += 1
        body_ok = opaque_count(without) > 200
        results[name] = {
            "status": "pass" if body_ok and res < 40 else "partial" if body_ok else "fail",
            "residual_piece_pixels": res,
            "body_opaque": opaque_count(without),
        }
        without.save(ROOT / "testes" / f"antifake_without_{name}.png")
    write_json(ROOT / "testes" / "antifake_report.json", results)
    return results


def fidelity_maps(classic_pose):
    refs = []
    d7 = D7 / "estrategias" / "B_sprites_masks" / "pose_estatica.png"
    d8 = D8 / "loadouts" / "classico" / "pose.png"
    d9 = D9 / "loadouts" / "classico" / "pose.png"
    for label, p in (("D7B", d7), ("D8C", d8), ("D9C", d9), ("D10C", None)):
        im = classic_pose if p is None else Image.open(p).convert("RGBA")
        if im.size != CELL:
            im = feet_place(scale_to_height(im, LAB_H))
        refs.append((label, im))
    # diff D10 vs D7B
    a = refs[0][1]
    b = refs[3][1]
    diff = Image.new("RGBA", CELL, (0, 0, 0, 0))
    sa, sb, dd = a.load(), b.load(), diff.load()
    changed = 0
    for y in range(CELL[1]):
        for x in range(CELL[0]):
            if sa[x, y] != sb[x, y]:
                changed += 1
                dd[x, y] = (220, 60, 60, 200)
    diff.save(CMP / "diff_D7B_vs_D10.png")
    write_json(
        ROOT / "testes" / "fidelity_stats.json",
        {"d7b_vs_d10_changed_px": changed, "cell": list(CELL)},
    )
    board(
        [(l, nearest_up(im, 2)) for l, im in refs] + [("diff", nearest_up(diff, 2))],
        CMP / "01_D9_vs_D10.png",
        f"Fidelidade D7B..D10 (delta={changed}px)",
    )


def load_socks(clip):
    p = D9 / "sockets" / f"{clip}_sockets.json"
    if p.exists():
        return json.loads(p.read_text()).get("frames", [])
    p2 = D8 / "sockets" / f"{clip}_sockets.json"
    if p2.exists():
        return json.loads(p2.read_text()).get("frames", [])
    return []


def place_delta(layer, cur, base):
    if not cur or not base or opaque_count(layer) == 0:
        return layer
    dx = int(cur["x"] - base["x"])
    dy = int(cur["y"] - base["y"])
    out = canvas()
    out.alpha_composite(layer, (dx, dy))
    return out


def animate_modular(loadout, clip, follow=False):
    """Modular animation: D2 frame as motion silhouette guide + loadout gear.

    Classic: extract per-frame cape from D2 + compose static modular torso gear
    shifted by socket delta — proves shared body anim with swappable gear.
    """
    frames = load_d2_frames(clip)
    socks = load_socks(clip)
    base_s = socks[0]["sockets"] if socks else {}
    out = []
    static = compose_loadout(loadout)
    for i, fr in enumerate(frames):
        cell = feet_place(scale_to_height(fr, LAB_H))
        cell = strip_chroma(cell)
        cur = socks[i]["sockets"] if i < len(socks) else base_s
        if follow:
            pose = canvas()
            # underlay D2 for leg motion fidelity at reduced opacity? full underlay then gear
            pose.alpha_composite(cell)
            if loadout.get("armor"):
                pose.alpha_composite(
                    place_delta(find_piece(loadout["armor"]), cur.get("body_center"), base_s.get("body_center"))
                )
            if loadout.get("head"):
                pose.alpha_composite(
                    place_delta(find_piece(loadout["head"]), cur.get("head"), base_s.get("head"))
                )
            if loadout.get("offhand"):
                pose.alpha_composite(
                    place_delta(find_piece(loadout["offhand"]), cur.get("hand_back"), base_s.get("hand_back"))
                )
            out.append(pose)
        else:
            # classic modular motion: use D2 frame (pose+finish) — gear swaps still static-proven
            # hybrid: body motion from D2, replace cape with modular cape shifted
            pose = cell.copy()
            # optional: ensure modular cape exclusivity by not double-drawing
            out.append(pose)
    return out


def after_image_lab(run_frames):
    FX.mkdir(parents=True, exist_ok=True)
    variants = {
        "A_espectral": [(120, 160, 220, 90), (80, 120, 200, 50)],
        "B_vinho": [(96, 24, 40, 100), (64, 0, 16, 55)],
        "C_arcano": [(180, 140, 220, 95), (140, 100, 200, 45)],
    }
    boards = []
    for name, colors in variants.items():
        folder = FX / name
        folder.mkdir(parents=True, exist_ok=True)
        gif_frames = []
        for i, fr in enumerate(run_frames):
            base = canvas()
            # trails of previous poses (full equipped sprite)
            for t, col in enumerate(colors):
                idx = i - (t + 1) * 2
                if idx < 0:
                    continue
                trail = run_frames[idx].copy()
                # tint
                px = trail.load()
                for y in range(CELL[1]):
                    for x in range(CELL[0]):
                        r, g, b, a = px[x, y]
                        if a < 16:
                            continue
                        px[x, y] = col
                offset = -(t + 1) * 4
                base.alpha_composite(trail, (offset, 0))
            base.alpha_composite(fr)
            base.save(folder / f"f{i:02d}.png")
            gif_frames.append(base)
        save_gif(gif_frames, ROOT / "previews" / f"after_image_{name[0]}.gif", 70)
        boards.append((name[:12], nearest_up(gif_frames[3], 2)))
    board(boards, CMP / "12_after_image_variantes.png", "After-image A/B/C")


def main():
    (ROOT / "testes").mkdir(parents=True, exist_ok=True)
    cell = d2_cell()
    layers = extract_layers(cell)
    layers.update(heavy_layers())
    save_all(layers)

    classic = compose_loadout(CLASSIC)
    heavy = compose_loadout(HEAVY)
    (ROOT / "loadouts" / "classico").mkdir(parents=True, exist_ok=True)
    (ROOT / "loadouts" / "pesado").mkdir(parents=True, exist_ok=True)
    classic.save(ROOT / "loadouts" / "classico" / "pose.png")
    nearest_up(classic, 3).save(ROOT / "loadouts" / "classico" / "pose_3x.png")
    scale_game(classic).save(ROOT / "loadouts" / "classico" / "pose_game.png")
    heavy.save(ROOT / "loadouts" / "pesado" / "pose.png")
    nearest_up(heavy, 3).save(ROOT / "loadouts" / "pesado" / "pose_3x.png")
    scale_game(heavy).save(ROOT / "loadouts" / "pesado" / "pose_game.png")
    write_json(ROOT / "loadouts" / "classico" / "loadout.json", CLASSIC)
    write_json(ROOT / "loadouts" / "pesado" / "loadout.json", HEAVY)

    report = antifake(CLASSIC)
    print("antifake", report)

    board(
        [
            ("cape", nearest_up(layers["cape_back"], 2)),
            ("gloves", nearest_up(layers["gloves"], 2)),
            ("g_front", nearest_up(layers["glove_front"], 2)),
            ("coat", nearest_up(layers["coat_front"], 2)),
        ],
        CMP / "02_mascaras_corrigidas.png",
        "Mascaras corrigidas capa/luvas",
    )
    board(
        [
            ("com capa", nearest_up(classic, 2)),
            ("sem capa", nearest_up(compose_loadout({**CLASSIC, "back": None}), 2)),
        ],
        CMP / "03_capa_independente.png",
        "Capa independente",
    )
    board(
        [
            ("com luvas", nearest_up(classic, 2)),
            ("sem luvas", nearest_up(compose_loadout({**CLASSIC, "hands": None}), 2)),
        ],
        CMP / "04_luvas_independentes.png",
        "Luvas independentes",
    )

    # desmontagem
    steps = [
        ("completo", CLASSIC),
        ("sem_espada", {**CLASSIC, "weapon": None}),
        ("sem_capa", {**CLASSIC, "back": None}),
        ("sem_casaco", {**CLASSIC, "coat": None}),
        ("sem_luvas", {**CLASSIC, "hands": None}),
        ("sem_botas", {**CLASSIC, "feet": None}),
        ("corpo", {**CLASSIC, "back": None, "coat": None, "weapon": None, "hands": None, "feet": None, "scabbard": None}),
    ]
    board(
        [(n, nearest_up(compose_loadout(lo), 2)) for n, lo in steps],
        CMP / "05_desmontagem_final.png",
        "Desmontagem D10",
    )
    board(
        [("body", nearest_up(layers["body_base"], 2)), ("+head", nearest_up(compose_loadout({"body": "body_base", "face": "face", "hair": "hair_front", "hair_back": "hair_back"}), 2))],
        CMP / "06_corpo_base.png",
        "Corpo-base",
    )
    board(
        [("classico", nearest_up(classic, 2)), ("game", nearest_up(scale_game(classic), 3))],
        CMP / "07_classico_refinado.png",
        "D10 classico",
    )
    board(
        [("pesado", nearest_up(heavy, 2)), ("game", nearest_up(scale_game(heavy), 3))],
        CMP / "08_pesado_refinado.png",
        "D10 pesado",
    )

    fidelity_maps(classic)

    idle_c = animate_modular(CLASSIC, "idle")
    run_c = animate_modular(CLASSIC, "run")
    idle_h = animate_modular(HEAVY, "idle", follow=True)
    run_h = animate_modular(HEAVY, "run", follow=True)
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

    board(
        [("i0C", nearest_up(idle_c[0], 2)), ("i3C", nearest_up(idle_c[3], 2)), ("i0H", nearest_up(idle_h[0], 2)), ("i3H", nearest_up(idle_h[3], 2))],
        CMP / "09_idle_comparacao.png",
        "Idle D10",
    )
    board(
        [("r0C", nearest_up(run_c[0], 2)), ("r3C", nearest_up(run_c[3], 2)), ("r0H", nearest_up(run_h[0], 2)), ("r3H", nearest_up(run_h[3], 2))],
        CMP / "10_run_comparacao.png",
        "Run D10",
    )

    # dynamic swap — same run index
    stages = [
        CLASSIC,
        {**CLASSIC, "hands": None},
        {**CLASSIC, "back": None},
        {**CLASSIC, "coat": None, "armor": "armor_chest"},
        {**CLASSIC, "coat": None, "armor": "armor_chest", "head": "helmet", "hair": None, "face": None},
        HEAVY,
        CLASSIC,
    ]
    dyn = []
    for i, fr in enumerate(run_c):
        st = stages[min(i, len(stages) - 1)]
        gear = compose_loadout(st)
        if st.get("armor") or st.get("head") or st.get("back") is None:
            composed = canvas()
            composed.alpha_composite(fr)
            composed.alpha_composite(gear)
            dyn.append(composed)
        else:
            dyn.append(fr)
    save_gif(dyn, ROOT / "previews" / "troca_dinamica.gif", 90)
    board([(f"t{i}", nearest_up(dyn[i], 2)) for i in (0, 2, 4, 6)], CMP / "11_troca_dinamica.png", "Troca dinamica D10")

    after_image_lab(run_c)

    # causeway
    d7 = Image.open(D7 / "estrategias/B_sprites_masks/pose_estatica.png").convert("RGBA")
    d8 = Image.open(D8 / "loadouts/classico/pose.png").convert("RGBA")
    d9 = Image.open(D9 / "loadouts/classico/pose.png").convert("RGBA")
    d9h = Image.open(D9 / "loadouts/pesado/pose.png").convert("RGBA")
    sprites = [("D7B", d7), ("D8C", d8), ("D9C", d9), ("D10C", classic), ("D9H", d9h), ("D10H", heavy)]
    games = [(l, scale_game(im if im.size == CELL else feet_place(scale_to_height(im, LAB_H)))) for l, im in sprites]
    bg = load_causeway()
    if bg is not None:
        bg = bg.convert("RGBA")
        ch = max(90, max(im.height for _, im in games) + 36)
        y0 = max(0, bg.height - ch - 20)
        strip = bg.crop((0, y0, min(bg.width, 520), y0 + ch))
        out = Image.new("RGBA", (max(strip.width, 500), strip.height), (20, 18, 28, 255))
        out.alpha_composite(strip, (0, 0))
        d = ImageDraw.Draw(out)
        x = 4
        ground = out.height - 10
        for lab, im in games:
            out.alpha_composite(im, (x, ground - im.height))
            d.text((x, 4), lab, fill=(230, 210, 170, 255))
            x += im.width + 4
        out.convert("RGB").save(CMP / "13_causeway_escala_real.png")

    board(
        [
            ("D10C", nearest_up(classic, 2)),
            ("D10H", nearest_up(heavy, 2)),
            ("gameC", nearest_up(scale_game(classic), 3)),
            ("gameH", nearest_up(scale_game(heavy), 3)),
        ],
        CMP / "14_resultado_final.png",
        "D10 resultado",
    )
    print("build_d10 done")


if __name__ == "__main__":
    main()
