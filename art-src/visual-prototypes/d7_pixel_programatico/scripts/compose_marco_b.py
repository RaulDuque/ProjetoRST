#!/usr/bin/env python3
"""Marco B: independent layers, two loadouts, piece swaps, idle/run pilot.

Motion body-base = D2 approved frames. Equipment pieces = D4 2D layers
composited by code (not Blender EEVEE finish).
"""
from __future__ import annotations

import json
from pathlib import Path

from PIL import Image, ImageDraw, ImageSequence

from d7_common import (
    D2,
    D4,
    ROOT,
    board,
    canvas,
    ensure_dirs,
    feet_place,
    load_causeway,
    load_d2_idle,
    nearest_up,
    scale_game,
    scale_to_height,
    trim,
)

EQ = ROOT / "equipamentos"
LAY = ROOT / "personagem"
LOAD_L = ROOT / "loadouts" / "leve"
LOAD_H = ROOT / "loadouts" / "pesado"
ANIM = ROOT / "animacoes"
CMP = ROOT / "comparacoes"

# Draw order (back → front)
LAYER_ORDER = [
    "cape_back",
    "body",
    "pants_boots",  # included in body for D2; placeholder
    "coat_or_armor",
    "gloves",
    "cape_front",
    "head",
    "helm_or_hat",
    "sword",
    "shield",
]


def _strip_magenta(im: Image.Image) -> Image.Image:
    """Remove D4 hot-pink / magenta matte fringes without eating wine lining."""
    im = im.convert("RGBA")
    px = im.load()
    for y in range(im.height):
        for x in range(im.width):
            r, g, b, a = px[x, y]
            if a < 8:
                continue
            # classic / hot-pink mattes (high R, low G); keep deep wine (low luminance)
            if r > 160 and g < 110 and b > 70 and (r - g) > 70 and (r + g + b) > 280:
                px[x, y] = (0, 0, 0, 0)
            elif r > 200 and g < 80 and b > 100:
                px[x, y] = (0, 0, 0, 0)
    return im


def load_d4(name: str) -> Image.Image | None:
    p = D4 / "camadas" / name
    if not p.exists():
        return None
    return _strip_magenta(Image.open(p).convert("RGBA"))


def place_d4_layer(raw: Image.Image | None) -> Image.Image:
    """D4 camadas are already 128×160 with sole registration — use as-is."""
    if raw is None:
        return canvas()
    im = raw.convert("RGBA")
    if im.size == (128, 160):
        return im
    return feet_place(scale_to_height(im, 110))


def compose(layers: dict[str, Image.Image], order: list[str]) -> Image.Image:
    out = canvas()
    for key in order:
        im = layers.get(key)
        if im is not None:
            out.alpha_composite(im)
    return out


def build_catalog() -> dict[str, Image.Image]:
    """Visual equipment catalog from D4 + D2 body."""
    d2 = feet_place(scale_to_height(load_d2_idle(), 110))
    cat = {
        "body_base": d2,
        "body_d4": place_d4_layer(load_d4("00_body.png")),
        "cape_back": place_d4_layer(load_d4("01_cape_back.png")),
        "coat_light": place_d4_layer(load_d4("02_light_coat.png")),
        "armor_heavy": place_d4_layer(load_d4("03_heavy_armor.png")),
        "hat": place_d4_layer(load_d4("04_hat.png")),
        "helm": place_d4_layer(load_d4("05_helm.png")),
        "glasses": place_d4_layer(load_d4("06_glasses.png")),
        "sword": place_d4_layer(load_d4("07_sword.png")),
        "shield": place_d4_layer(load_d4("08_shield.png")),
    }
    return cat


def loadout_leve(cat: dict[str, Image.Image]) -> Image.Image:
    # Aristocrata: D2 body-base (identidade) + capa/casaco/espada D4 quando não ocluem a cabeça
    return compose(
        {
            "cape_back": cat["cape_back"],
            "body": cat["body_base"],
            "coat_or_armor": cat["coat_light"],
            "sword": cat["sword"],
        },
        ["cape_back", "body", "coat_or_armor", "sword"],
    )


def loadout_pesado(cat: dict[str, Image.Image]) -> Image.Image:
    # Cavaleiro: corpo D4 + armadura + elmo + escudo + espada (silhueta distinta)
    return compose(
        {
            "cape_back": cat["cape_back"],
            "body": cat["body_d4"],
            "coat_or_armor": cat["armor_heavy"],
            "helm_or_hat": cat["helm"],
            "sword": cat["sword"],
            "shield": cat["shield"],
        },
        ["cape_back", "body", "coat_or_armor", "sword", "shield", "helm_or_hat"],
    )


def swap_tests(cat: dict[str, Image.Image]) -> list[tuple[str, Image.Image]]:
    base_layers = {
        "cape_back": cat["cape_back"],
        "body": cat["body_base"],
        "coat_or_armor": cat["coat_light"],
        "sword": cat["sword"],
    }
    order = ["cape_back", "body", "coat_or_armor", "helm_or_hat", "sword", "shield"]
    tests = []
    # 1 boots — no separate boots layer in D4; document proxy via body_d4 feet
    t = dict(base_layers)
    t["body"] = cat["body_d4"]
    tests.append(("swap_body_proxy_boots", compose(t, order)))
    # 2 gloves — proxy glasses slot unused; skip visual, use glasses as stand-in marker
    t = dict(base_layers)
    t["helm_or_hat"] = cat["glasses"]
    tests.append(("swap_glasses_proxy_gloves", compose(t, order)))
    # 3 coat → heavy
    t = dict(base_layers)
    t["coat_or_armor"] = cat["armor_heavy"]
    tests.append(("swap_coat_to_armor", compose(t, order)))
    # 4 peitoral only (armor without helm)
    t = dict(base_layers)
    t["coat_or_armor"] = cat["armor_heavy"]
    t.pop("helm_or_hat", None)
    tests.append(("swap_peitoral", compose(t, order)))
    # 5 cape off
    t = dict(base_layers)
    t["cape_back"] = canvas()
    tests.append(("swap_cape_off", compose(t, order)))
    # 6 sword off
    t = dict(base_layers)
    t["sword"] = canvas()
    tests.append(("swap_sword_off", compose(t, order)))
    # 7 shield on
    t = dict(base_layers)
    t["shield"] = cat["shield"]
    tests.append(("swap_shield_on", compose(t, order)))
    # 8 helm on
    t = dict(base_layers)
    t["helm_or_hat"] = cat["helm"]
    tests.append(("swap_helm_on", compose(t, order)))
    return tests


def load_clip_frames(kind: str) -> list[Image.Image]:
    if kind == "idle":
        folder = D2 / "animacoes" / "idle" / "frames"
        names = [f"idle_f{i:02d}.png" for i in range(6)]
    else:
        folder = D2 / "animacoes" / "run" / "frames"
        names = [f"run_f{i:02d}.png" for i in range(8)]
    frames = []
    for n in names:
        p = folder / n
        if p.exists():
            frames.append(Image.open(p).convert("RGBA"))
    return frames


def animate_loadout(
    frames: list[Image.Image],
    extras: list[Image.Image],
    height: int = 110,
) -> list[Image.Image]:
    """Body from D2 frames; static equipment extras stacked each frame (pilot)."""
    out_frames = []
    for fr in frames:
        body = feet_place(scale_to_height(fr, height))
        composed = body.copy()
        for ex in extras:
            composed.alpha_composite(ex)
        out_frames.append(composed)
    return out_frames


def save_gif(frames: list[Image.Image], path: Path, duration: int = 80) -> None:
    if not frames:
        return
    rgb = []
    for fr in frames:
        bg = Image.new("RGBA", fr.size, (12, 10, 18, 255))
        bg.alpha_composite(fr)
        rgb.append(nearest_up(bg.convert("RGBA"), 2).convert("P", palette=Image.ADAPTIVE))
    path.parent.mkdir(parents=True, exist_ok=True)
    rgb[0].save(
        path,
        save_all=True,
        append_images=rgb[1:],
        duration=duration,
        loop=0,
        optimize=False,
    )


def write_layer_manifest(cat: dict[str, Image.Image]) -> None:
    manifest = {
        "cell": [128, 160],
        "pivot_sole": [64, 156],
        "game_height_px": 52,
        "draw_order": LAYER_ORDER,
        "layers": [],
    }
    LAY.mkdir(parents=True, exist_ok=True)
    EQ.mkdir(parents=True, exist_ok=True)
    for key, im in cat.items():
        fname = f"{key}.png"
        dest = EQ / fname if key != "body_base" else LAY / fname
        im.save(dest)
        box = im.split()[-1].getbbox()
        manifest["layers"].append(
            {
                "id": key,
                "file": str(dest.relative_to(ROOT)),
                "size": list(im.size),
                "bbox": list(box) if box else None,
                "pivot": [64, 156],
                "palette": "d2_inspired",
                "visibility": True,
            }
        )
    (LAY / "layers_manifest.json").write_text(
        json.dumps(manifest, indent=2), encoding="utf-8"
    )


def main() -> None:
    ensure_dirs()
    cat = build_catalog()
    write_layer_manifest(cat)

    leve = loadout_leve(cat)
    pesado = loadout_pesado(cat)
    LOAD_L.mkdir(parents=True, exist_ok=True)
    LOAD_H.mkdir(parents=True, exist_ok=True)
    leve.save(LOAD_L / "pose.png")
    nearest_up(leve, 3).save(LOAD_L / "pose_3x.png")
    pesado.save(LOAD_H / "pose.png")
    nearest_up(pesado, 3).save(LOAD_H / "pose_3x.png")

    board(
        [
            ("layers atlas", nearest_up(cat["body_base"], 2)),
            ("cape", nearest_up(cat["cape_back"], 2)),
            ("coat", nearest_up(cat["coat_light"], 2)),
            ("armor", nearest_up(cat["armor_heavy"], 2)),
            ("helm", nearest_up(cat["helm"], 2)),
            ("sword", nearest_up(cat["sword"], 2)),
            ("shield", nearest_up(cat["shield"], 2)),
        ],
        CMP / "05_D7_camadas.png",
        "Camadas independentes (D2 body + D4 gear)",
    )

    board(
        [
            ("leve", nearest_up(leve, 2)),
            ("pesado", nearest_up(pesado, 2)),
            ("leve game", nearest_up(scale_game(leve), 3)),
            ("pesado game", nearest_up(scale_game(pesado), 3)),
        ],
        CMP / "06_D7_loadouts.png",
        "Dois loadouts — silhuetas distintas",
    )

    swaps = swap_tests(cat)
    for name, im in swaps:
        im.save(ROOT / "testes" / f"{name}.png")
    board(
        [(n.replace("swap_", "")[:18], nearest_up(im, 2)) for n, im in swaps],
        CMP / "07_D7_troca_equipamentos.png",
        "Troca individual de pecas",
    )

    # Animations — leve = D2 body-base only (already clothed).
    # Pesado = same frames + heavy pieces on top (static extras; deformation TBD).
    idle_fr = load_clip_frames("idle")
    run_fr = load_clip_frames("run")
    idle_leve = animate_loadout(idle_fr, [])
    run_leve = animate_loadout(run_fr, [])
    idle_pes = animate_loadout(idle_fr, [cat["armor_heavy"], cat["helm"], cat["shield"]])
    run_pes = animate_loadout(run_fr, [cat["armor_heavy"], cat["helm"], cat["shield"]])

    for i, fr in enumerate(idle_leve):
        fr.save(ANIM / "idle" / f"leve_f{i:02d}.png")
    for i, fr in enumerate(run_leve):
        fr.save(ANIM / "run" / f"leve_f{i:02d}.png")
    for i, fr in enumerate(run_pes):
        fr.save(ANIM / "run" / f"pesado_f{i:02d}.png")

    save_gif(idle_leve, ROOT / "previews" / "idle_leve.gif", duration=100)
    save_gif(run_leve, ROOT / "previews" / "run_leve.gif", duration=70)
    save_gif(run_pes, ROOT / "previews" / "run_pesado.gif", duration=70)

    # Update causeway with loadouts
    bg = load_causeway()
    sprites = [
        ("D2", feet_place(scale_to_height(load_d2_idle(), 110))),
        ("leve", leve),
        ("pesado", pesado),
    ]
    games = [(lab, scale_game(im)) for lab, im in sprites]
    if bg is not None:
        bg = bg.convert("RGBA")
        ch = max(80, max(im.height for _, im in games) + 40)
        y0 = max(0, bg.height - ch - 20)
        strip = bg.crop((0, y0, min(bg.width, 280), y0 + ch))
        out = Image.new("RGBA", (max(strip.width, 300), strip.height), (20, 18, 28, 255))
        out.alpha_composite(strip, (0, 0))
        d = ImageDraw.Draw(out)
        x = 10
        ground = out.height - 12
        for lab, im in games:
            out.alpha_composite(im, (x, ground - im.height))
            d.text((x, 4), lab, fill=(230, 210, 170, 255))
            x += im.width + 10
        out.convert("RGB").save(CMP / "08_D7_causeway.png")

    print("Marco B complete")


if __name__ == "__main__":
    main()
