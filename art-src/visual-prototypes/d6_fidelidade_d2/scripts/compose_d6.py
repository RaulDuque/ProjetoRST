#!/usr/bin/env python3
"""Compose D6 loadouts, independence tests, boards, GIFs, automated checks."""
from __future__ import annotations

import hashlib
import json
import shutil
from pathlib import Path

from PIL import Image, ImageDraw, ImageEnhance, ImageFilter

ROOT = Path(__file__).resolve().parents[1]
REPO = ROOT.parents[2]
D5 = REPO / "art-src/visual-prototypes/d5_corrida_modular"
D2 = REPO / "art-src/visual-prototypes/d2_protagonista_B"
CELL = (128, 160)
FRAMES = 10
FPS = 24
DURATION = int(round(1000 / FPS))
CAUSEWAY = REPO / "target/world_visual_showcase/after_causeway.png"


def nearest(im: Image.Image, tw: int, th: int) -> Image.Image:
    im = im.convert("RGBA")
    w, h = im.size
    out = Image.new("RGBA", (tw, th))
    s, d = im.load(), out.load()
    for y in range(th):
        sy = min(h - 1, y * h // th)
        for x in range(tw):
            sx = min(w - 1, x * w // tw)
            d[x, y] = s[sx, sy]
    return out


def is_matte(r: int, g: int, b: int) -> bool:
    if r > 235 and g > 235 and b > 235:
        return True
    if r > 160 and b > 160 and g < 155:
        return True
    if r > 170 and g < 100 and (r - g) > 90:
        return True
    return False


def chroma(im: Image.Image) -> Image.Image:
    im = im.convert("RGBA")
    p = im.load()
    for y in range(im.height):
        for x in range(im.width):
            r, g, b, a = p[x, y]
            if is_matte(r, g, b):
                p[x, y] = (0, 0, 0, 0)
    return im


def feet_cell(im: Image.Image) -> Image.Image:
    """Fit render into shared CELL (same as D5)."""
    return nearest(im.convert("RGBA"), CELL[0], CELL[1])


def load_png(path: Path) -> Image.Image:
    return feet_cell(Image.open(path))


def finish_b(im: Image.Image) -> Image.Image:
    """Controlled hybrid finish: contrast + mild edge darken (not claimed as hand pixel)."""
    im = im.convert("RGBA")
    rgb = ImageEnhance.Color(im.convert("RGB")).enhance(1.08)
    rgb = ImageEnhance.Contrast(rgb).enhance(1.12)
    out = rgb.convert("RGBA")
    out.putalpha(im.split()[-1])
    a = out.split()[-1]
    edge = a.filter(ImageFilter.FIND_EDGES)
    px, ep = out.load(), edge.load()
    for y in range(out.height):
        for x in range(out.width):
            if ep[x, y] > 50 and px[x, y][3] > 0:
                r, g, b, a0 = px[x, y]
                px[x, y] = ((r * 4) // 5, (g * 4) // 5, (b * 4) // 5, a0)
    return out


def layer(kind: str, i: int) -> Image.Image:
    paths = {
        "body": ROOT / "animacoes/run/body" / f"run_f{i:02d}.png",
        "coat": ROOT / "equipamentos/leves/run_coat" / f"run_f{i:02d}.png",
        "cape": ROOT / "equipamentos/capas/run_cape" / f"run_f{i:02d}.png",
        "sword": ROOT / "equipamentos/armas/run_sword" / f"run_f{i:02d}.png",
        "heavy": ROOT / "equipamentos/pesados/run_set" / f"run_f{i:02d}.png",
        "helm": ROOT / "equipamentos/cabeca/run_helm" / f"run_f{i:02d}.png",
        "shield": ROOT / "equipamentos/armas/run_shield" / f"run_f{i:02d}.png",
        "boots": ROOT / "equipamentos/botas/run_boots" / f"run_f{i:02d}.png",
        "gloves": ROOT / "equipamentos/maos/run_gloves" / f"run_f{i:02d}.png",
    }
    return load_png(paths[kind])


def stack(parts: list[str], i: int, cape_lag: int = 0) -> Image.Image:
    canvas = Image.new("RGBA", CELL, (0, 0, 0, 0))
    for p in parts:
        if p == "cape":
            canvas.alpha_composite(layer("cape", (i - cape_lag) % FRAMES))
        else:
            canvas.alpha_composite(layer(p, i))
    return canvas


LOADOUTS = {
    "A_viajante": {
        "parts": ["cape", "body", "coat", "boots", "gloves", "sword"],
        "cape_lag": 0,
    },
    "B_cavaleiro": {
        "parts": ["cape", "body", "heavy", "helm", "boots", "sword", "shield"],
        "cape_lag": 1,
    },
    "C_misto": {
        "parts": ["body", "coat", "gloves", "boots", "sword", "shield", "helm"],
        "cape_lag": 0,
    },
}


def compose_loadout(key: str, i: int) -> Image.Image:
    cfg = LOADOUTS[key]
    return finish_b(stack(cfg["parts"], i, cfg["cape_lag"]))


def save_gif(frames: list[Image.Image], path: Path, bg=(16, 12, 22)) -> None:
    out = []
    for f in frames:
        bg_im = Image.new("RGBA", f.size, (*bg, 255))
        bg_im.alpha_composite(f)
        out.append(bg_im.convert("P", palette=Image.ADAPTIVE))
    out[0].save(
        path, save_all=True, append_images=out[1:], duration=DURATION, loop=0, disposal=2
    )


def board(rows: list[tuple[str, Image.Image]], path: Path, title: str) -> None:
    imgs = []
    for lab, im in rows:
        if im.width < 90:
            im = nearest(im, im.width * 2, im.height * 2)
        imgs.append((lab, im.convert("RGBA")))
    gap = 12
    max_h = max(im.height for _, im in imgs)
    w = gap + sum(im.width + gap for _, im in imgs)
    h = max_h + 48
    out = Image.new("RGB", (w, h), (12, 10, 18))
    d = ImageDraw.Draw(out)
    d.text((8, 6), title, fill=(230, 210, 170))
    x = gap
    for lab, im in imgs:
        y = 28 + (max_h - im.height)
        out.paste(im.convert("RGB"), (x, y), im)
        d.text((x, h - 16), lab[:18], fill=(180, 180, 190))
        x += im.width + gap
    path.parent.mkdir(parents=True, exist_ok=True)
    out.save(path)


def md5(path: Path) -> str:
    return hashlib.md5(path.read_bytes()).hexdigest()


def prep_masters() -> dict[str, Image.Image]:
    """D6-A blender, D6-B hybrid finish, D6-C D2-style concepts (chroma)."""
    a_toon = load_png(ROOT / "personagem/D6_A/idle_side_toon.png")
    a_heavy = load_png(ROOT / "personagem/D6_A/idle_side_heavy.png")
    a_toon.save(ROOT / "personagem/D6_A/master_cell.png")
    finish_b(a_toon).save(ROOT / "personagem/D6_B/master_light.png")
    finish_b(a_heavy).save(ROOT / "personagem/D6_B/master_heavy.png")

    masters = {"A_light": a_toon, "A_heavy": a_heavy}
    for name, src in (
        ("light", ROOT / "personagem/D6_C/master_light_d2style.jpg"),
        ("heavy", ROOT / "personagem/D6_C/master_heavy_d2style.jpg"),
        ("mixed", ROOT / "personagem/D6_C/master_mixed_d2style.jpg"),
    ):
        if src.exists():
            im = feet_cell(chroma(Image.open(src)))
            im.save(ROOT / "personagem/D6_C" / f"master_{name}_cell.png")
            masters[f"C_{name}"] = im
    masters["B_light"] = finish_b(a_toon)
    masters["B_heavy"] = finish_b(a_heavy)
    return masters


def main() -> None:
    masters = prep_masters()

    # loadouts
    composed: dict[str, list[Image.Image]] = {}
    for key in LOADOUTS:
        frames = []
        out_dir = ROOT / "loadouts" / key
        out_dir.mkdir(parents=True, exist_ok=True)
        for i in range(FRAMES):
            fr = compose_loadout(key, i)
            fr.save(out_dir / f"run_f{i:02d}.png")
            frames.append(fr)
        composed[key] = frames
        save_gif(frames, ROOT / "previews" / f"run_{key}.gif")

    swap = []
    for t, key in enumerate(["A_viajante"] * FRAMES + ["B_cavaleiro"] * FRAMES + ["C_misto"] * FRAMES):
        swap.append(composed[key][t % FRAMES])
    save_gif(swap, ROOT / "previews" / "run_dynamic_swap.gif")

    # independence swaps at frame 0 base = A without cape lag
    base_parts = ["cape", "body", "coat", "boots", "gloves", "sword"]
    independence = [
        ("base A", finish_b(stack(base_parts, 0))),
        ("+heavy boots", finish_b(stack(["cape", "body", "coat", "boots", "gloves", "sword", "heavy"], 0))),
        ("gloves only Δ", finish_b(stack(["cape", "body", "coat", "boots", "sword"], 0))),  # no gloves
        ("chest→heavy", finish_b(stack(["cape", "body", "heavy", "boots", "gloves", "sword"], 0))),
        ("+helm", finish_b(stack(base_parts + ["helm"], 0))),
        ("no cape", finish_b(stack(["body", "coat", "boots", "gloves", "sword"], 0))),
        ("no sword", finish_b(stack(["cape", "body", "coat", "boots", "gloves"], 0))),
        ("+shield", finish_b(stack(base_parts + ["shield"], 0))),
    ]
    # clearer independence: single-piece toggles from traveler
    independence = [
        ("A full", finish_b(stack(base_parts, 0))),
        ("boots off", finish_b(stack(["cape", "body", "coat", "gloves", "sword"], 0))),
        ("gloves off", finish_b(stack(["cape", "body", "coat", "boots", "sword"], 0))),
        ("coat→heavy", finish_b(stack(["cape", "body", "heavy", "boots", "gloves", "sword"], 0))),
        ("helm on", finish_b(stack(base_parts + ["helm"], 0))),
        ("cape off", finish_b(stack(["body", "coat", "boots", "gloves", "sword"], 0))),
        ("sword off", finish_b(stack(["cape", "body", "coat", "boots", "gloves"], 0))),
        ("shield on", finish_b(stack(base_parts + ["shield"], 0))),
    ]
    board(independence, ROOT / "comparacoes" / "07_troca_individual_pecas.png", "07 — Individual piece swaps (same body f0)")

    # Etapa 0 / boards
    d2 = feet_cell(Image.open(D2 / "animacoes/idle/frames/idle_f00_3x.png"))
    d5_raw = load_png(D5 / "acabamento/blender_raw_shaded_f00.png") if (D5 / "acabamento/blender_raw_shaded_f00.png").exists() else masters["A_light"]
    d5_toon = load_png(D5 / "acabamento/blender_full_A_f00.png") if (D5 / "acabamento/blender_full_A_f00.png").exists() else masters["A_light"]
    d5_outline = finish_b(d5_toon)

    board(
        [
            ("D2", d2),
            ("D5 raw", d5_raw),
            ("D5 toon", d5_toon),
            ("D5 outline", d5_outline),
            ("D6-A", masters["A_light"]),
            ("D6-B", masters["B_light"]),
            ("D6-C", masters.get("C_light", masters["A_light"])),
        ],
        ROOT / "comparacoes" / "01_D2_vs_D5_vs_D6.png",
        "01 — D2 vs D5 vs D6 (side identity)",
    )
    board(
        [
            ("D6-A blender", masters["A_light"]),
            ("D6-B hybrid", masters["B_light"]),
            ("D6-C D2-aim", masters.get("C_light", masters["A_light"])),
            ("D6-A heavy", masters["A_heavy"]),
            ("D6-C heavy", masters.get("C_heavy", masters["A_heavy"])),
        ],
        ROOT / "comparacoes" / "02_D6_acabamentos.png",
        "02 — D6 finishes A/B/C",
    )

    # heavy pieces board
    heavy_dir = ROOT / "equipamentos/pesados"
    heavy_imgs = []
    for name in [
        "cuirass",
        "pauldron_R",
        "pauldron_L",
        "bracer_R",
        "gauntlet_R",
        "fauld",
        "cuisse_R",
        "greave_R",
        "sabaton_R",
    ]:
        p = heavy_dir / f"{name}.png"
        if p.exists():
            heavy_imgs.append((name, load_png(p)))
    helm = ROOT / "equipamentos/cabeca/helm.png"
    if helm.exists():
        heavy_imgs.append(("helm", load_png(helm)))
    board(heavy_imgs[:10], ROOT / "comparacoes" / "03_armadura_pesada_pecas.png", "03 — Heavy armor pieces (lab plates)")

    light_imgs = []
    for lab, p in [
        ("coat", ROOT / "equipamentos/leves/coat.png"),
        ("shirt", ROOT / "equipamentos/leves/shirt.png"),
        ("pants", ROOT / "equipamentos/pernas/pants.png"),
        ("gloves", ROOT / "equipamentos/maos/gloves.png"),
        ("boots", ROOT / "equipamentos/botas/boots.png"),
        ("cape", ROOT / "equipamentos/capas/cape.png"),
    ]:
        if p.exists():
            light_imgs.append((lab, load_png(p)))
    board(light_imgs, ROOT / "comparacoes" / "04_roupa_leve_pecas.png", "04 — Light outfit separated pieces")

    # slots diagram (text board)
    slot_img = Image.new("RGB", (900, 320), (12, 10, 18))
    d = ImageDraw.Draw(slot_img)
    d.text((12, 10), "05 — Visual slots (proposal) vs functional EquipmentSlot", fill=(230, 210, 170))
    lines = [
        "Visual: Head Face Torso Arms Hands Legs Feet Back MainHand OffHand FX",
        "Functional today: Weapon Offhand Head Body Gloves Boots Acc1 Acc2",
        "Map: Head<-Head  Torso/Back<-Body  Hands<-Gloves  Feet/Legs<-Boots",
        "MainHand<-Weapon  OffHand<-Offhand  Face/FX <- Accessory*",
        "Occlusion: closed helm hides hair+glasses; long coat hides pants upper;",
        "heavy cuirass replaces light coat torso but may keep separate boots/gloves.",
    ]
    y = 40
    for line in lines:
        d.text((12, y), line, fill=(190, 190, 200))
        y += 28
    slot_img.save(ROOT / "comparacoes" / "05_slots_visuais.png")

    board(
        [
            ("A viajante", composed["A_viajante"][0]),
            ("B cavaleiro", composed["B_cavaleiro"][0]),
            ("C misto", composed["C_misto"][0]),
            ("C concept", masters.get("C_mixed", composed["C_misto"][0])),
        ],
        ROOT / "comparacoes" / "06_loadouts_A_B_C.png",
        "06 — Loadouts A/B/C (shared run index 0)",
    )

    # clipping problems
    board(
        [
            ("A f3", composed["A_viajante"][3]),
            ("B f3", composed["B_cavaleiro"][3]),
            ("heavy alone f3", finish_b(layer("heavy", 3))),
            ("helm f3", finish_b(layer("helm", 3))),
        ],
        ROOT / "comparacoes" / "09_problemas_clipping.png",
        "09 — Clipping/readability check (plates are lab proxies)",
    )

    board(
        [
            ("D2", d2),
            ("D6-C light", masters.get("C_light", masters["B_light"])),
            ("D6-C heavy", masters.get("C_heavy", masters["B_heavy"])),
            ("Run A", composed["A_viajante"][0]),
            ("Run B", composed["B_cavaleiro"][0]),
            ("Run C", composed["C_misto"][0]),
        ],
        ROOT / "comparacoes" / "10_comparacao_final.png",
        "10 — Final compare (art decision belongs to owner)",
    )

    # causeway
    if CAUSEWAY.is_file():
        bg = Image.open(CAUSEWAY).convert("RGBA")
        comp = bg.copy()
        dr = ImageDraw.Draw(comp)
        feet_y = 515
        entries = [
            (140, d2, "D2"),
            (300, masters["A_light"], "D6A"),
            (460, masters["B_light"], "D6B"),
            (620, masters.get("C_light", masters["B_light"]), "D6C"),
            (780, composed["A_viajante"][0], "A"),
            (940, composed["B_cavaleiro"][0], "B"),
            (1100, composed["C_misto"][0], "C"),
        ]
        for x, sp, lab in entries:
            box = sp.split()[-1].getbbox()
            if not box:
                continue
            crop = sp.crop(box)
            th = 52
            tw = max(8, int(round(crop.width * th / crop.height)))
            scaled = nearest(crop, tw, th)
            comp.alpha_composite(scaled, (x - scaled.width // 2, feet_y - scaled.height))
            dr.text((x - 12, feet_y + 6), lab, fill=(230, 210, 170))
        dr.text((24, 24), "08 — D6 @ causeway V2.3 gameplay scale", fill=(230, 210, 170))
        comp.convert("RGB").save(ROOT / "comparacoes" / "08_causeway_escala_real.png")

    for name, color in (("dark", (12, 10, 18)), ("light", (220, 215, 205))):
        row = [(k, composed[k][0]) for k in ("A_viajante", "B_cavaleiro", "C_misto")]
        imgs = []
        for lab, im in row:
            bg = Image.new("RGBA", (im.width * 2, im.height * 2), (*color, 255))
            big = nearest(im, im.width * 2, im.height * 2)
            bg.alpha_composite(big)
            imgs.append((lab[:1], bg))
        board(imgs, ROOT / "comparacoes" / f"scale_{name}.png", f"NN enlarge on {name}")

    # tests
    report = {"ok": True, "checks": []}

    def check(name: str, cond: bool, detail: str = "") -> None:
        report["checks"].append({"name": name, "ok": bool(cond), "detail": detail})
        if not cond:
            report["ok"] = False

    for kind, path in [
        ("body", ROOT / "animacoes/run/body"),
        ("heavy", ROOT / "equipamentos/pesados/run_set"),
        ("helm", ROOT / "equipamentos/cabeca/run_helm"),
        ("shield", ROOT / "equipamentos/armas/run_shield"),
    ]:
        files = sorted(path.glob("run_f*.png"))
        check(f"{kind}_count", len(files) == FRAMES, str(len(files)))
        for f in files:
            im = Image.open(f)
            check(f"{f.name}_rgba", im.mode == "RGBA")

    for piece in ["cuirass", "pauldron_R", "pauldron_L", "fauld", "helm"]:
        p = ROOT / "equipamentos/pesados" / f"{piece}.png"
        if piece == "helm":
            p = ROOT / "equipamentos/cabeca/helm.png"
        check(f"piece_{piece}", p.exists())

    check(
        "A_ne_B",
        md5(ROOT / "loadouts/A_viajante/run_f00.png")
        != md5(ROOT / "loadouts/B_cavaleiro/run_f00.png"),
    )
    a1 = compose_loadout("A_viajante", 0)
    a2 = compose_loadout("A_viajante", 0)
    check("deterministic", list(a1.getdata()) == list(a2.getdata()))

    tmp = ROOT / "testes/_repro"
    if tmp.exists():
        shutil.rmtree(tmp)
    tmp.mkdir()
    for i in range(FRAMES):
        compose_loadout("A_viajante", i).save(tmp / f"f{i:02d}.png")
    check(
        "repro",
        all(
            md5(tmp / f"f{i:02d}.png") == md5(ROOT / "loadouts/A_viajante" / f"run_f{i:02d}.png")
            for i in range(FRAMES)
        ),
    )
    shutil.rmtree(tmp)

    (ROOT / "testes/modular_tests.json").write_text(json.dumps(report, indent=2))
    print("tests ok=", report["ok"], "fails", [c for c in report["checks"] if not c["ok"]][:6])


if __name__ == "__main__":
    main()
