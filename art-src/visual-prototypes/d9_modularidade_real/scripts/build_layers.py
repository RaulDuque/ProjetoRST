#!/usr/bin/env python3
"""Marco A/B — dependency map + reconstructed independent layers.

Strategy:
- Extract removable overlays from D2 cell (cape, weapon, hair, face, boots).
- Build body_base from D4 body quantized to D2 palette + D2 residual torso
  so removing coat/cape does not leave holes.
- Coat = D2 pixels that are coat-like AND not in body_base silhouette core,
  merged with D4 light coat quantized for coverage.
- Heavy pieces = D4 chroma-stripped + quantized toward D2 palette.
"""
from __future__ import annotations

from pathlib import Path

from PIL import Image, ImageDraw

from d9_common import (
    CELL,
    D8,
    LAB_H,
    PAL,
    PIVOT,
    ROOT,
    board,
    canvas,
    feet_place,
    layer_bbox,
    load_d4,
    nearest_up,
    opaque_count,
    quantize_d2,
    scale_to_height,
    strip_chroma,
    write_json,
)

ATLAS = ROOT / "atlas"
MASKS = ROOT / "mascaras"
CORPO = ROOT / "corpo"
EQ = ROOT / "equipamentos"


def d2_cell() -> Image.Image:
    p = ROOT / "fontes" / "d8_full_classic_ref.png"
    if p.exists():
        return strip_chroma(Image.open(p).convert("RGBA"))
    src = Image.open(ROOT / "fontes" / "d2_idle_f00.png").convert("RGBA")
    return feet_place(scale_to_height(src, LAB_H))


def _mask(src: Image.Image, pred) -> Image.Image:
    out = Image.new("RGBA", src.size, (0, 0, 0, 0))
    s, d = src.load(), out.load()
    for y in range(src.height):
        for x in range(src.width):
            px = s[x, y]
            if px[3] < 16:
                continue
            if pred(x, y, px):
                d[x, y] = px
    return out


def _subtract(full: Image.Image, part: Image.Image) -> Image.Image:
    out = full.copy()
    s, p, d = full.load(), part.load(), out.load()
    for y in range(full.height):
        for x in range(full.width):
            if p[x, y][3] >= 16:
                d[x, y] = (0, 0, 0, 0)
    return out


def _or_layers(a: Image.Image, b: Image.Image) -> Image.Image:
    out = a.copy()
    out.alpha_composite(b)
    return out


def extract_overlays(cell: Image.Image) -> dict[str, Image.Image]:
    box = cell.split()[-1].getbbox()
    assert box
    x0, y0, x1, y1 = box
    h, w = y1 - y0, x1 - x0

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

    def cape(x, y, px):
        r, g, b, a = px
        wine = r > g + 12 and r > b and r > 28 and g < 70 and y > y0 + int(h * 0.18)
        back = x < x0 + int(w * 0.40) and y0 + int(h * 0.20) < y < y0 + int(h * 0.92)
        return (wine or back) and not boots(x, y, px) and not weapon(x, y, px) and not head(x, y, px)

    hair_im = _mask(cell, hair)
    # split hair: left-ish = back, right-ish = front (side view)
    hair_back = Image.new("RGBA", CELL, (0, 0, 0, 0))
    hair_front = Image.new("RGBA", CELL, (0, 0, 0, 0))
    mid = x0 + w // 2
    s = hair_im.load()
    for y in range(CELL[1]):
        for x in range(CELL[0]):
            if s[x, y][3] < 16:
                continue
            if x < mid - 2:
                hair_back.putpixel((x, y), s[x, y])
            else:
                hair_front.putpixel((x, y), s[x, y])

    return {
        "face": _mask(cell, face),
        "hair_back": hair_back,
        "hair_front": hair_front,
        "boots_classic": _mask(cell, boots),
        "weapon_classic": _mask(cell, weapon),
        "cape_back": _mask(cell, cape),
    }


def build_body_base(cell: Image.Image, overlays: dict[str, Image.Image]) -> Image.Image:
    """Body under clothes from D2 residuals + D4 body only as hole-fill (inner torso)."""
    residual = cell.copy()
    for key in ("cape_back", "weapon_classic", "boots_classic", "hair_back", "hair_front", "face"):
        residual = _subtract(residual, overlays[key])

    box = residual.split()[-1].getbbox()
    body = canvas()
    shirt_add = canvas()
    pants_add = canvas()
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
                r, g, b, a = px
                if r > g + 15 and r > 40 and g < 60:
                    continue
                if rel < 0.22:
                    continue
                # Keep D2 inner columns as body/shirt/pants — not full outer coat
                inner = x0 + int((x1 - x0) * 0.22) < x < x0 + int((x1 - x0) * 0.78)
                if not inner and rel < 0.55:
                    continue
                if 0.22 <= rel < 0.55:
                    shirt_add.putpixel((x, y), px)
                elif 0.55 <= rel < 0.84:
                    pants_add.putpixel((x, y), px)

    body.alpha_composite(shirt_add)
    body.alpha_composite(pants_add)

    # Hole-fill from D4 body only where still empty, torso band only
    d4 = load_d4("00_body.png")
    if d4 is not None and d4.size == CELL and box:
        q = quantize_d2(d4)
        x0, y0, x1, y1 = box
        y_min = y0 + int((y1 - y0) * 0.26)
        y_max = y0 + int((y1 - y0) * 0.84)
        fill = canvas()
        s, d, bpx = q.load(), fill.load(), body.load()
        for y in range(y_min, y_max):
            for x in range(x0 + 6, x1 - 6):
                if s[x, y][3] < 16 or bpx[x, y][3] >= 16:
                    continue
                d[x, y] = s[x, y]
        body.alpha_composite(fill)

    if opaque_count(body) < 500 and box:
        d = ImageDraw.Draw(body)
        x0, y0, x1, y1 = box
        d.rectangle(
            [x0 + 10, y0 + int((y1 - y0) * 0.30), x1 - 10, y0 + int((y1 - y0) * 0.55)],
            fill=PAL["shirt"],
        )
        d.rectangle(
            [x0 + 12, y0 + int((y1 - y0) * 0.55), x1 - 12, y0 + int((y1 - y0) * 0.82)],
            fill=PAL["pants"],
        )

    for key in ("cape_back", "weapon_classic", "boots_classic"):
        body = _subtract(body, overlays[key])
    return body


def build_coat(cell: Image.Image, body: Image.Image, overlays: dict) -> Image.Image:
    """Coat from D2 cell (preserve D7-B). D4 only fills torso holes — never hat."""
    coat = cell.copy()
    coat = _subtract(coat, overlays["cape_back"])
    coat = _subtract(coat, overlays["weapon_classic"])
    coat = _subtract(coat, overlays["boots_classic"])
    coat = _subtract(coat, overlays["hair_back"])
    coat = _subtract(coat, overlays["hair_front"])
    coat = _subtract(coat, overlays["face"])
    coat = _subtract(coat, body)

    d4c = load_d4("02_light_coat.png")
    box = cell.split()[-1].getbbox()
    if d4c is not None and box is not None:
        q = quantize_d2(d4c)
        x0, y0, x1, y1 = box
        # Only borrow D4 pixels in torso band; exclude head/hat region
        y_min = y0 + int((y1 - y0) * 0.28)
        y_max = y0 + int((y1 - y0) * 0.88)
        fill = canvas()
        s, d = q.load(), fill.load()
        cl = coat.load()
        for y in range(y_min, y_max):
            for x in range(x0, x1):
                if s[x, y][3] < 16:
                    continue
                if cl[x, y][3] >= 16:
                    continue  # D2 coat already owns pixel
                d[x, y] = s[x, y]
        coat = _or_layers(coat, fill)
    coat = _subtract(coat, overlays["cape_back"])
    coat = _subtract(coat, overlays["weapon_classic"])
    coat = _subtract(coat, overlays["boots_classic"])
    return coat


def build_scabbard_gloves(cell: Image.Image, overlays: dict, body: Image.Image) -> tuple[Image.Image, Image.Image]:
    box = cell.split()[-1].getbbox() or (40, 40, 90, 156)
    x0, y0, x1, y1 = box
    h, w = y1 - y0, x1 - x0
    gloves = canvas()
    scabbard = canvas()
    s = cell.load()
    for y in range(y0 + int(h * 0.35), y0 + int(h * 0.58)):
        for x in range(x0 + int(w * 0.58), x1):
            px = s[x, y]
            if px[3] < 16:
                continue
            r, g, b, a = px
            # glove-ish dark brown / boot tones on forward arm
            if r < 100 and g < 80 and b < 90:
                gloves.putpixel((x, y), px)
    for y in range(y0 + int(h * 0.45), y0 + int(h * 0.78)):
        for x in range(x0, x0 + int(w * 0.48)):
            px = s[x, y]
            if px[3] < 16:
                continue
            r, g, b, a = px
            if 55 < r < 170 and 35 < g < 130 and b < g + 10 and (r - b) > 15:
                scabbard.putpixel((x, y), px)
    # if gloves too weak, synthesize small cuff from palette on forward wrist
    if opaque_count(gloves) < 12:
        gx = x0 + int(w * 0.72)
        gy = y0 + int(h * 0.48)
        d = ImageDraw.Draw(gloves)
        d.ellipse([gx - 3, gy - 3, gx + 4, gy + 3], fill=PAL["boot"])
    if opaque_count(scabbard) < 4:
        d = ImageDraw.Draw(scabbard)
        d.line(
            [(x0 + 6, y0 + int(h * 0.52)), (x0 + 2, y0 + int(h * 0.72))],
            fill=PAL["gold_dk"],
            width=2,
        )
    return gloves, scabbard


def refine_heavy() -> dict[str, Image.Image]:
    """D4 heavy pieces chroma-stripped + quantized to D2 palette."""
    mapping = {
        "armor_chest": "03_heavy_armor.png",
        "helmet": "05_helm.png",
        "shield": "08_shield.png",
        "weapon_heavy": "07_sword.png",
        "cape_d4": "01_cape_back.png",
        "body_d4": "00_body.png",
        "coat_d4": "02_light_coat.png",
    }
    out = {}
    for key, fname in mapping.items():
        im = load_d4(fname)
        if im is None:
            continue
        if im.size != CELL:
            im = feet_place(scale_to_height(im, LAB_H))
        out[key] = quantize_d2(strip_chroma(im))
    # shoulders / forearms from armor bands
    if "armor_chest" in out:
        arm = out["armor_chest"]
        out["armor_shoulder_front"] = _mask(arm, lambda x, y, px: y < PIVOT[1] - 72)
        out["armor_forearm_front"] = _mask(
            arm, lambda x, y, px: PIVOT[1] - 72 <= y < PIVOT[1] - 42
        )
        out["armor_legs"] = _mask(arm, lambda x, y, px: y >= PIVOT[1] - 42)
    # heavy boots: darken classic boots + metal hints
    return out


def save_layer(path: Path, im: Image.Image) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    im.save(path)
    m = Image.new("L", im.size, 0)
    s, d = im.load(), m.load()
    for y in range(im.height):
        for x in range(im.width):
            d[x, y] = 255 if s[x, y][3] >= 16 else 0
    MASKS.mkdir(parents=True, exist_ok=True)
    m.save(MASKS / f"{path.stem}_mask.png")


def dependency_map(layers: dict[str, Image.Image]) -> None:
    rows = []
    for name, im in sorted(layers.items()):
        box = layer_bbox(im)
        rows.append(
            {
                "piece": name,
                "opaque_px": opaque_count(im),
                "bbox": list(box) if box else None,
                "removable": name
                not in ("body_base", "head_anchor"),
                "replaceable": True,
                "source": "d2_cut+d4_fill" if "armor" not in name and "helmet" not in name else "d4_quantized",
                "independence": "real"
                if opaque_count(im) > 20 and name != "body_base"
                else ("base" if name == "body_base" else "weak"),
            }
        )
    write_json(ROOT / "atlas" / "dependency_map.json", {"pieces": rows})
    md = [
        "# Mapa de dependências D9\n",
        "| Peça | px | Independência | Fonte |",
        "|------|----|---------------|-------|",
    ]
    for r in rows:
        md.append(
            f"| `{r['piece']}` | {r['opaque_px']} | {r['independence']} | {r['source']} |"
        )
    md.append(
        "\n**Regra:** peça só é “real” se remover/substituir não exige "
        "`full_classic_ref` e o corpo-base permanece sem buracos graves.\n"
    )
    (ROOT / "MAPA_DEPENDENCIAS.md").write_text("\n".join(md), encoding="utf-8")


def desmontagem(layers: dict[str, Image.Image], order_full: list[str]) -> None:
    def stack(keys):
        out = canvas()
        for k in keys:
            if k in layers and opaque_count(layers[k]):
                out.alpha_composite(layers[k])
        return out

    steps = [
        ("completo", order_full),
        ("sem_espada", [k for k in order_full if k != "weapon_classic"]),
        ("sem_bainha", [k for k in order_full if k != "scabbard"]),
        ("sem_capa", [k for k in order_full if k != "cape_back"]),
        ("sem_casaco", [k for k in order_full if k != "coat_front"]),
        ("sem_luvas", [k for k in order_full if k != "gloves"]),
        ("sem_botas", [k for k in order_full if k != "boots_classic"]),
        (
            "corpo_base",
            ["body_base", "face", "hair_back", "hair_front"],
        ),
    ]
    imgs = [(lab, nearest_up(stack(keys), 2)) for lab, keys in steps]
    board(imgs, ROOT / "comparacoes" / "03_desmontagem_completa.png", "Desmontagem completa D9")

    # separated pieces board
    sep = []
    for k in [
        "body_base",
        "coat_front",
        "cape_back",
        "boots_classic",
        "gloves",
        "weapon_classic",
        "hair_front",
        "face",
        "scabbard",
    ]:
        if k in layers:
            sep.append((k[:14], nearest_up(layers[k], 2)))
    board(sep, ROOT / "comparacoes" / "04_atlas_limpo.png", "Atlas D9 — pecas")


def main() -> None:
    ATLAS.mkdir(parents=True, exist_ok=True)
    CORPO.mkdir(parents=True, exist_ok=True)
    for sub in EQ.iterdir() if EQ.exists() else []:
        pass
    for sub in (
        "roupas",
        "armaduras",
        "cabeca",
        "luvas",
        "botas",
        "capas",
        "armas",
        "escudos",
    ):
        (EQ / sub).mkdir(parents=True, exist_ok=True)

    cell = d2_cell()
    cell.save(ROOT / "fontes" / "d9_classic_cell.png")
    overlays = extract_overlays(cell)
    body = build_body_base(cell, overlays)
    coat = build_coat(cell, body, overlays)
    gloves, scabbard = build_scabbard_gloves(cell, overlays, body)

    cape = overlays["cape_back"]
    boots = overlays["boots_classic"]
    weapon = overlays["weapon_classic"]

    # Mutual exclusivity across modular pieces (alpha ownership)
    coat = _subtract(coat, cape)
    coat = _subtract(coat, boots)
    coat = _subtract(coat, weapon)
    coat = _subtract(coat, gloves)
    # gloves own alpha exclusively (keep source pixels)
    coat = _subtract(coat, gloves)
    body = _subtract(body, gloves)
    body = _subtract(body, coat)
    body = _subtract(body, cape)
    body = _subtract(body, weapon)
    body = _subtract(body, boots)
    body = _subtract(body, gloves)
    # Exclusivity by alpha ownership (keep original D2 cape pixels for fidelity)
    hair_front = _subtract(overlays["hair_front"], cape)
    hair_back = _subtract(overlays["hair_back"], cape)
    coat = _subtract(coat, cape)
    body = _subtract(body, cape)
    # Guaranteed underlayer: paint body core, then carve those pixels out of coat
    box = cell.split()[-1].getbbox()
    if box:
        x0, y0, x1, y1 = box
        core = canvas()
        d = ImageDraw.Draw(core)
        shirt_box = [
            x0 + 14,
            y0 + int((y1 - y0) * 0.34),
            x1 - 14,
            y0 + int((y1 - y0) * 0.58),
        ]
        pants_box = [
            x0 + 16,
            y0 + int((y1 - y0) * 0.58),
            x1 - 16,
            y0 + int((y1 - y0) * 0.80),
        ]
        d.rectangle(shirt_box, fill=PAL["shirt"])
        d.rectangle(pants_box, fill=PAL["pants"])
        # body keeps existing detail where present, then ensure core
        body.alpha_composite(core)
        # coat must not own the underlayer core
        coat = _subtract(coat, core)
        body = _subtract(body, cape)
        body = _subtract(body, boots)
        body = _subtract(body, weapon)

    layers = {
        "body_base": body,
        "face": overlays["face"],
        "hair_back": hair_back,
        "hair_front": hair_front,
        "coat_front": coat,
        "cape_back": cape,
        "boots_classic": boots,
        "gloves": gloves,
        "weapon_classic": weapon,
        "scabbard": scabbard,
    }
    heavy = refine_heavy()
    layers.update(heavy)

    # persist
    save_layer(CORPO / "body_base.png", body)
    save_layer(ATLAS / "body_base.png", body)
    save_layer(ATLAS / "face.png", layers["face"])
    save_layer(ATLAS / "hair_back.png", layers["hair_back"])
    save_layer(ATLAS / "hair_front.png", layers["hair_front"])
    save_layer(EQ / "roupas" / "coat_front.png", coat)
    save_layer(EQ / "capas" / "cape_back.png", layers["cape_back"])
    save_layer(EQ / "botas" / "boots_classic.png", layers["boots_classic"])
    save_layer(EQ / "luvas" / "gloves.png", gloves)
    save_layer(EQ / "armas" / "weapon_classic.png", layers["weapon_classic"])
    save_layer(EQ / "armas" / "scabbard.png", scabbard)
    if "armor_chest" in layers:
        save_layer(EQ / "armaduras" / "armor_chest.png", layers["armor_chest"])
    if "helmet" in layers:
        save_layer(EQ / "cabeca" / "helmet.png", layers["helmet"])
    if "shield" in layers:
        save_layer(EQ / "escudos" / "shield.png", layers["shield"])
    if "weapon_heavy" in layers:
        save_layer(EQ / "armas" / "weapon_heavy.png", layers["weapon_heavy"])
    if "cape_d4" in layers:
        save_layer(EQ / "capas" / "cape_d4.png", layers["cape_d4"])

    for name, im in layers.items():
        save_layer(ATLAS / f"{name}.png", im)

    write_json(
        ATLAS / "atlas_manifest.json",
        {
            "cell": list(CELL),
            "pivot_sole": list(PIVOT),
            "lab_height_px": LAB_H,
            "game_height_px": 52,
            "note": "D9 reconstructed layers; classic compose must NOT require full_classic_ref",
            "layers": [
                {
                    "id": k,
                    "opaque_px": opaque_count(v),
                    "bbox": list(layer_bbox(v)) if layer_bbox(v) else None,
                }
                for k, v in layers.items()
            ],
        },
    )

    dependency_map(layers)
    order_full = [
        "cape_back",
        "body_base",
        "coat_front",
        "boots_classic",
        "gloves",
        "face",
        "hair_back",
        "hair_front",
        "scabbard",
        "weapon_classic",
    ]
    desmontagem(layers, order_full)

    # dependency board
    board(
        [
            ("body", nearest_up(body, 2)),
            ("coat", nearest_up(coat, 2)),
            ("cape", nearest_up(layers["cape_back"], 2)),
            ("weapon", nearest_up(layers["weapon_classic"], 2)),
            ("hair", nearest_up(layers["hair_front"], 2)),
        ],
        ROOT / "comparacoes" / "02_mapa_dependencias.png",
        "Mapa visual — pecas-chave",
    )
    print(f"Layers built: {len(layers)}")


if __name__ == "__main__":
    main()
