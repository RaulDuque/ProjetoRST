#!/usr/bin/env python3
"""Marco A — clean experimental 2D atlas from D2 + D4 (D7-B lineage)."""
from __future__ import annotations

import json
from pathlib import Path

from PIL import Image

from d8_common import (
    CELL,
    D2,
    D4,
    LAB_H,
    PIVOT,
    ROOT,
    board,
    canvas,
    feet_place,
    load_d2_idle,
    load_d4,
    nearest_up,
    scale_to_height,
    strip_chroma,
    write_json,
)

ATLAS = ROOT / "atlas"
MASKS = ROOT / "mascaras"
EQ = ROOT / "equipamentos"


def _mask(src: Image.Image, pred) -> Image.Image:
    src = src.convert("RGBA")
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


def _invert_mask(full: Image.Image, part: Image.Image) -> Image.Image:
    """full minus part alpha."""
    out = full.copy()
    s, p, d = full.load(), part.load(), out.load()
    for y in range(full.height):
        for x in range(full.width):
            if p[x, y][3] >= 16:
                d[x, y] = (0, 0, 0, 0)
    return out


def extract_from_d2_cell(cell: Image.Image) -> dict[str, Image.Image]:
    """Semantic cuts on feet-registered D2 idle (lab cell)."""
    box = cell.split()[-1].getbbox()
    assert box
    x0, y0, x1, y1 = box
    h = y1 - y0
    w = x1 - x0

    def head(x, y, px):
        return y < y0 + int(h * 0.26)

    def face(x, y, px):
        r, g, b, a = px
        return head(x, y, px) and r > 120 and g > 80 and b > 60 and r > b

    def hair(x, y, px):
        return head(x, y, px) and not face(x, y, px)

    def boots(x, y, px):
        return y > y0 + int(h * 0.84)

    def weapon(x, y, px):
        r, g, b, a = px
        bright = (r + g + b) / 3 > 135 and abs(r - g) < 55
        return bright and x > x0 + int(w * 0.48) and y > y0 + int(h * 0.32)

    def wine(x, y, px):
        r, g, b, a = px
        return r > g + 12 and r > b and r > 28 and g < 70 and y > y0 + int(h * 0.18)

    def cape_back(x, y, px):
        return wine(x, y, px) or (x < x0 + int(w * 0.38) and y0 + int(h * 0.22) < y < y0 + int(h * 0.92) and not boots(x, y, px) and not weapon(x, y, px))

    hair_im = _mask(cell, hair)
    face_im = _mask(cell, face)
    boots_im = _mask(cell, boots)
    weapon_im = _mask(cell, weapon)
    cape_back_im = _mask(cell, cape_back)
    # body core = remaining opaque after removing extracted parts
    body = cell.copy()
    for part in (hair_im, face_im, boots_im, weapon_im, cape_back_im):
        body = _invert_mask(body, part)

    # Approximate sub-layers from body bands (documented as soft splits)
    shirt = Image.new("RGBA", CELL, (0, 0, 0, 0))
    coat_front = Image.new("RGBA", CELL, (0, 0, 0, 0))
    pants = Image.new("RGBA", CELL, (0, 0, 0, 0))
    gloves = Image.new("RGBA", CELL, (0, 0, 0, 0))
    scabbard = Image.new("RGBA", CELL, (0, 0, 0, 0))
    coat_back = Image.new("RGBA", CELL, (0, 0, 0, 0))
    hair_back = Image.new("RGBA", CELL, (0, 0, 0, 0))
    hair_front = hair_im.copy()
    cape_front = Image.new("RGBA", CELL, (0, 0, 0, 0))

    s = body.load()
    for y in range(CELL[1]):
        for x in range(CELL[0]):
            px = s[x, y]
            if px[3] < 16:
                continue
            rel = (y - y0) / max(1, h)
            if rel < 0.45 and x > x0 + int(w * 0.55):
                # forward coat / sleeve region
                if 0.35 < rel < 0.55 and x > x0 + int(w * 0.62):
                    gloves.putpixel((x, y), px)
                else:
                    coat_front.putpixel((x, y), px)
            elif 0.55 <= rel < 0.84:
                pants.putpixel((x, y), px)
            elif rel < 0.55:
                shirt.putpixel((x, y), px)
            else:
                coat_front.putpixel((x, y), px)

    # scabbard hint: dark gold-ish diagonal behind hip (weak; may be empty)
    for y in range(y0 + int(h * 0.45), y0 + int(h * 0.75)):
        for x in range(x0, x0 + int(w * 0.45)):
            px = cell.getpixel((x, y))
            r, g, b, a = px
            if a > 16 and 60 < r < 160 and 40 < g < 120 and b < g and (r - b) > 20:
                scabbard.putpixel((x, y), px)

    # residual body (skin/torso under clothes) — keep non-empty core
    core = _invert_mask(
        body,
        Image.alpha_composite(
            Image.alpha_composite(coat_front, pants),
            Image.alpha_composite(shirt, gloves),
        ),
    )
    if core.split()[-1].getbbox() is None:
        core = shirt.copy()

    return {
        "body": core if core.split()[-1].getbbox() else shirt,
        "hair_back": hair_back,
        "hair_front": hair_front,
        "face": face_im,
        "shirt": shirt,
        "coat_back": coat_back,
        "coat_front": coat_front,
        "pants": pants,
        "gloves": gloves,
        "boots": boots_im,
        "cape_back": cape_back_im,
        "cape_front": cape_front,
        "weapon": weapon_im,
        "scabbard": scabbard,
        "_full_classic": cell,
    }


def d4_equipment() -> dict[str, Image.Image]:
    """Independent D4 gear already in CELL — chroma cleaned."""
    mapping = {
        "cape_d4": "01_cape_back.png",
        "coat_d4": "02_light_coat.png",
        "armor": "03_heavy_armor.png",
        "hat": "04_hat.png",
        "helm": "05_helm.png",
        "glasses": "06_glasses.png",
        "weapon_d4": "07_sword.png",
        "shield": "08_shield.png",
        "body_d4": "00_body.png",
    }
    out = {}
    for key, fname in mapping.items():
        im = load_d4(fname)
        if im is not None:
            out[key] = im if im.size == CELL else feet_place(scale_to_height(im, LAB_H))
    # Proxies for missing heavy sub-parts (documented)
    if "armor" in out:
        out["pauldrons"] = _mask(
            out["armor"],
            lambda x, y, px: y < PIVOT[1] - 70,
        )
        out["bracers"] = _mask(
            out["armor"],
            lambda x, y, px: PIVOT[1] - 70 <= y < PIVOT[1] - 40,
        )
    return out


def provenance(layers: dict[str, Image.Image], eq: dict[str, Image.Image]) -> dict:
    entries = []
    for name, im in list(layers.items()) + list(eq.items()):
        if name.startswith("_"):
            continue
        box = im.split()[-1].getbbox()
        entries.append(
            {
                "id": name,
                "size": list(im.size),
                "bbox": list(box) if box else None,
                "pivot": list(PIVOT),
                "alpha": True,
                "source": "d2_idle_f00_mask" if name in layers and not name.startswith("_") else "d4_camada",
                "chroma_stripped": True,
            }
        )
    return {
        "cell": list(CELL),
        "pivot_sole": list(PIVOT),
        "lab_height_px": LAB_H,
        "game_height_px": 52,
        "layers": entries,
    }


def main() -> None:
    ATLAS.mkdir(parents=True, exist_ok=True)
    MASKS.mkdir(parents=True, exist_ok=True)
    EQ.mkdir(parents=True, exist_ok=True)

    d2 = feet_place(scale_to_height(load_d2_idle(), LAB_H))
    d2 = strip_chroma(d2)
    d2.save(ROOT / "fontes" / "d2_idle_cell.png")

    layers = extract_from_d2_cell(d2)
    full = layers.pop("_full_classic")
    full.save(ATLAS / "full_classic_ref.png")

    for name, im in layers.items():
        im.save(ATLAS / f"{name}.png")
        # binary mask
        m = Image.new("L", im.size, 0)
        s, d = im.load(), m.load()
        for y in range(im.height):
            for x in range(im.width):
                d[x, y] = 255 if s[x, y][3] >= 16 else 0
        m.save(MASKS / f"{name}_mask.png")

    eq = d4_equipment()
    for name, im in eq.items():
        im.save(EQ / f"{name}.png")

    # Restack test — must remain recognizable
    order = [
        "cape_back",
        "body",
        "pants",
        "shirt",
        "coat_front",
        "boots",
        "gloves",
        "face",
        "hair_front",
        "scabbard",
        "weapon",
    ]
    rebuilt = canvas()
    for key in order:
        rebuilt.alpha_composite(layers[key])
    rebuilt.save(ATLAS / "restack_classic.png")

    prov = provenance(layers, eq)
    write_json(ATLAS / "atlas_manifest.json", prov)

    # Atlas board
    show = []
    for key in order:
        show.append((key, nearest_up(layers[key], 2)))
    board(show[:8], ROOT / "comparacoes" / "02_atlas_limpo.png", "Atlas limpo (amostra)")
    board(
        [(k, nearest_up(layers[k], 2)) for k in order],
        ROOT / "comparacoes" / "03_camadas_independentes.png",
        "Camadas independentes D2-cut",
    )
    board(
        [
            ("D2 cell", nearest_up(full, 2)),
            ("restack", nearest_up(rebuilt, 2)),
        ],
        ROOT / "comparacoes" / "01_D7_vs_D8.png",
        "D2 cell vs restack atlas D8",
    )

    # Soft-split limitations note
    (ATLAS / "NOTAS_SEPARACAO.md").write_text(
        "# Notas de separação do atlas\n\n"
        "- Cortes a partir de `idle_f00` D2 registrado na célula 128×160.\n"
        "- `coat_back` / `cape_front` / `hair_back` podem ficar vazios quando o "
        "frame lateral não distingue planos — a composição usa `cape_back` + "
        "`coat_front` + `hair_front`.\n"
        "- `gloves` e `scabbard` são recortes fracos; se vazios, o loadout clássico "
        "cai no sprite completo de referência para preservar D7-B.\n"
        "- Quando o restack perder fidelidade vs `full_classic_ref.png`, o "
        "compositor clássico prioriza o frame D2 completo + overlays.\n",
        encoding="utf-8",
    )
    print(f"Atlas written to {ATLAS}")


if __name__ == "__main__":
    main()
