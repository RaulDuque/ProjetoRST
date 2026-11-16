#!/usr/bin/env python3
"""D4 modular compositor: chroma layers, register to feet, stack combinations.

Regenerates camadas/, combinacoes/, movimento/run_*, previews/, comparacoes/
from referencia/ + movimento/d4_run_combo_*.jpg.

Run from repo root:
  python3 art-src/visual-prototypes/d4_modular_d2/scripts/compose_d4.py
"""
from __future__ import annotations

from collections import deque
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[1]
CELL = (128, 160)
PIVOT = (64, 156)
CAUSEWAY = Path("target/world_visual_showcase/after_causeway.png")


def nearest_down(im: Image.Image, tw: int, th: int) -> Image.Image:
    im = im.convert("RGBA")
    w, h = im.size
    out = Image.new("RGBA", (tw, th))
    src, dst = im.load(), out.load()
    for y in range(th):
        sy = min(h - 1, y * h // th)
        for x in range(tw):
            sx = min(w - 1, x * w // tw)
            dst[x, y] = src[sx, sy]
    return out


def nearest_up(im: Image.Image, scale: int) -> Image.Image:
    im = im.convert("RGBA")
    w, h = im.size
    out = Image.new("RGBA", (w * scale, h * scale))
    src, dst = im.load(), out.load()
    for y in range(h):
        for x in range(w):
            c = src[x, y]
            for dy in range(scale):
                for dx in range(scale):
                    dst[x * scale + dx, y * scale + dy] = c
    return out


def is_matte(r: int, g: int, b: int) -> bool:
    """Key classic magenta AND hot-pink JPEG mattes (high R, very low G)."""
    if r > 235 and g > 235 and b > 235:
        return True
    if r > 160 and b > 160 and g < 155:
        return True
    # hot pink / magenta JPEG drift (e.g. 219,2,117) — keep wine lining (r-g modest)
    if r > 170 and g < 100 and (r - g) > 90:
        return True
    if r > 200 and g < 120 and b > 80 and (r - g) > 80 and (b - g) > 20:
        return True
    return False


def chroma_flood(im: Image.Image) -> Image.Image:
    im = im.convert("RGBA")
    w, h = im.size
    pix = im.load()
    for y in range(h):
        for x in range(w):
            r, g, b, a = pix[x, y]
            if is_matte(r, g, b):
                pix[x, y] = (0, 0, 0, 0)

    seen = [[False] * w for _ in range(h)]
    q: deque[tuple[int, int]] = deque()

    def seed(x: int, y: int) -> None:
        if 0 <= x < w and 0 <= y < h and not seen[y][x]:
            r, g, b, a = pix[x, y]
            if a == 0 or is_matte(r, g, b):
                seen[y][x] = True
                q.append((x, y))

    for x in range(w):
        seed(x, 0)
        seed(x, h - 1)
    for y in range(h):
        seed(0, y)
        seed(w - 1, y)

    while q:
        x, y = q.popleft()
        pix[x, y] = (0, 0, 0, 0)
        for nx, ny in ((x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)):
            if 0 <= nx < w and 0 <= ny < h and not seen[ny][nx]:
                r, g, b, a = pix[nx, ny]
                if a == 0 or is_matte(r, g, b) or (r > 190 and g < 130 and b > 70):
                    seen[ny][nx] = True
                    q.append((nx, ny))
    return im


def trim(im: Image.Image, pad: int = 1) -> Image.Image:
    box = im.split()[-1].getbbox()
    if not box:
        return im
    x0, y0, x1, y1 = box
    return im.crop(
        (
            max(0, x0 - pad),
            max(0, y0 - pad),
            min(im.width, x1 + pad),
            min(im.height, y1 + pad),
        )
    )


def to_height(im: Image.Image, th: int) -> Image.Image:
    im = trim(chroma_flood(im))
    w, h = im.size
    if h < 1:
        return im
    return nearest_down(im, max(8, int(round(w * th / h))), th)


def feet_register(
    im: Image.Image, cell: tuple[int, int] = CELL, pivot: tuple[int, int] = PIVOT
) -> Image.Image:
    canvas = Image.new("RGBA", cell, (0, 0, 0, 0))
    im = trim(im)
    x = pivot[0] - im.width // 2
    y = pivot[1] - im.height + 1
    canvas.alpha_composite(im, (max(0, x), max(0, y)))
    return canvas


def split_horizontal(im: Image.Image, n: int) -> list[Image.Image]:
    im = im.convert("RGBA")
    w, h = im.size
    cw = w // n
    return [im.crop((i * cw, 0, (i + 1) * cw if i < n - 1 else w, h)) for i in range(n)]


def small_layer(raw: Image.Image, th: int, y_lift: int = 0) -> Image.Image:
    im = to_height(raw, th)
    canvas = Image.new("RGBA", CELL, (0, 0, 0, 0))
    x = PIVOT[0] - im.width // 2
    y = PIVOT[1] - 95 - y_lift - im.height // 2
    canvas.alpha_composite(im, (max(0, x), max(0, y)))
    return canvas


def board(rows: list[tuple[str, Image.Image]], path: Path, title: str) -> None:
    imgs = [
        (lab, nearest_up(im, 2) if im.width <= CELL[0] else im) for lab, im in rows
    ]
    max_h = max(im.height for _, im in imgs)
    gap = 16
    width = gap + sum(im.width + gap for _, im in imgs)
    height = max_h + 50
    out = Image.new("RGB", (width, height), (12, 10, 18))
    draw = ImageDraw.Draw(out)
    draw.text((10, 6), title, fill=(230, 210, 170))
    x = gap
    for lab, im in imgs:
        y = 40 + (max_h - im.height)
        if im.mode == "RGBA":
            out.paste(im.convert("RGB"), (x, y), im)
        else:
            out.paste(im.convert("RGB"), (x, y))
        draw.text((x, height - 18), lab, fill=(180, 180, 190))
        x += im.width + gap
    out.save(path)


def gif_pair(a: Image.Image, b: Image.Image, path: Path, duration: int = 180) -> None:
    frames = []
    for frame in (a, b):
        big = nearest_up(frame, 2)
        bg = Image.new("RGBA", big.size, (16, 12, 22, 255))
        bg.alpha_composite(big)
        frames.append(bg.convert("P", palette=Image.ADAPTIVE))
    frames[0].save(
        path,
        save_all=True,
        append_images=frames[1:],
        duration=duration,
        loop=0,
        disposal=2,
    )


def main() -> None:
    refs = ROOT / "referencia"
    layers_dir = ROOT / "camadas"
    layers_dir.mkdir(exist_ok=True)
    (ROOT / "combinacoes").mkdir(exist_ok=True)
    (ROOT / "movimento").mkdir(exist_ok=True)
    (ROOT / "previews").mkdir(exist_ok=True)
    (ROOT / "comparacoes").mkdir(exist_ok=True)
    (ROOT / "testes").mkdir(exist_ok=True)

    body = feet_register(to_height(Image.open(refs / "d4_layer_body.jpg"), 110))
    light = feet_register(to_height(Image.open(refs / "d4_layer_light_coat.jpg"), 110))
    heavy = feet_register(to_height(Image.open(refs / "d4_layer_heavy_armor.jpg"), 100))
    cape = feet_register(to_height(Image.open(refs / "d4_layer_cape.jpg"), 120))

    hat_raw, helm_raw = split_horizontal(Image.open(refs / "d4_layer_headgear.jpg"), 2)
    shield_raw, sword_raw, glasses_raw = split_horizontal(
        Image.open(refs / "d4_layer_held_face.jpg"), 3
    )

    hat = small_layer(hat_raw, 36, y_lift=20)
    helm = small_layer(helm_raw, 40, y_lift=18)
    glasses = small_layer(glasses_raw, 18, y_lift=28)

    sword = Image.new("RGBA", CELL, (0, 0, 0, 0))
    sword.alpha_composite(to_height(sword_raw, 70), (PIVOT[0] + 8, PIVOT[1] - 90))
    shield = Image.new("RGBA", CELL, (0, 0, 0, 0))
    shield.alpha_composite(to_height(shield_raw, 48), (PIVOT[0] - 40, PIVOT[1] - 85))

    named = {
        "00_body": body,
        "01_cape_back": cape,
        "02_light_coat": light,
        "03_heavy_armor": heavy,
        "04_hat": hat,
        "05_helm": helm,
        "06_glasses": glasses,
        "07_sword": sword,
        "08_shield": shield,
    }
    for name, im in named.items():
        im.save(layers_dir / f"{name}.png")
        nearest_up(im, 2).save(layers_dir / f"{name}_2x.png")

    atlas = Image.new("RGBA", (CELL[0] * len(named), CELL[1]), (0, 0, 0, 0))
    for i, im in enumerate(named.values()):
        atlas.alpha_composite(im, (i * CELL[0], 0))
    atlas.save(layers_dir / "atlas_layers.png")
    nearest_up(atlas, 2).save(layers_dir / "atlas_layers_2x.png")

    combos = {
        "bare_body": ["00_body"],
        "light_outfit": ["01_cape_back", "00_body", "02_light_coat"],
        "heavy_armor": ["00_body", "03_heavy_armor"],
        "light_hat_glasses": [
            "01_cape_back",
            "00_body",
            "02_light_coat",
            "04_hat",
            "06_glasses",
        ],
        "heavy_helm": ["00_body", "03_heavy_armor", "05_helm"],
        "cape_sword": ["01_cape_back", "00_body", "02_light_coat", "07_sword"],
        "heavy_shield_sword": [
            "00_body",
            "03_heavy_armor",
            "07_sword",
            "08_shield",
        ],
        "full_light": [
            "01_cape_back",
            "00_body",
            "02_light_coat",
            "04_hat",
            "06_glasses",
            "07_sword",
        ],
        "full_heavy": [
            "00_body",
            "03_heavy_armor",
            "05_helm",
            "07_sword",
            "08_shield",
        ],
    }

    def compose(parts: list[str], out_name: str) -> Image.Image:
        canvas = Image.new("RGBA", CELL, (0, 0, 0, 0))
        for key in parts:
            canvas.alpha_composite(named[key])
        canvas.save(ROOT / "combinacoes" / f"{out_name}.png")
        nearest_up(canvas, 2).save(ROOT / "combinacoes" / f"{out_name}_2x.png")
        return canvas

    composed = {name: compose(parts, name) for name, parts in combos.items()}

    run_light = feet_register(
        to_height(Image.open(ROOT / "movimento" / "d4_run_combo_light.jpg"), 110)
    )
    run_heavy = feet_register(
        to_height(Image.open(ROOT / "movimento" / "d4_run_combo_heavy.jpg"), 110)
    )
    run_light.save(ROOT / "movimento" / "run_light.png")
    run_heavy.save(ROOT / "movimento" / "run_heavy.png")
    nearest_up(run_light, 2).save(ROOT / "movimento" / "run_light_2x.png")
    nearest_up(run_heavy, 2).save(ROOT / "movimento" / "run_heavy_2x.png")

    gif_pair(composed["full_light"], run_light, ROOT / "previews" / "motion_light.gif")
    gif_pair(composed["full_heavy"], run_heavy, ROOT / "previews" / "motion_heavy.gif")

    d2_anchor = Image.open(refs / "d2_idle_anchor_3x.png").convert("RGBA")
    d2t = trim(d2_anchor)
    d2_52 = nearest_down(
        d2t, max(8, int(round(d2t.width * 110 / d2t.height))), 110
    )
    d2_reg = feet_register(d2_52)

    board(
        [
            ("D2 original", d2_reg),
            ("bare body", composed["bare_body"]),
            ("light", composed["light_outfit"]),
            ("heavy", composed["heavy_armor"]),
            ("cape+sword", composed["cape_sword"]),
            ("heavy+shield", composed["heavy_shield_sword"]),
        ],
        ROOT / "comparacoes" / "cmp_modular_idle.png",
        "D4 modular idle — D2 style layers (prototype)",
    )
    board(
        [
            ("idle light", composed["full_light"]),
            ("run light", run_light),
            ("idle heavy", composed["full_heavy"]),
            ("run heavy", run_heavy),
        ],
        ROOT / "comparacoes" / "cmp_motion_two_loadouts.png",
        "Motion test — two loadouts (alignment/overlap check)",
    )

    if CAUSEWAY.is_file():
        bg = Image.open(CAUSEWAY).convert("RGBA")
        comp = bg.copy()
        draw = ImageDraw.Draw(comp)
        feet_y = 515
        entries = [
            (160, d2_reg, "D2"),
            (360, composed["bare_body"], "bare"),
            (560, composed["light_outfit"], "light"),
            (760, composed["heavy_armor"], "heavy"),
            (960, composed["cape_sword"], "cape+sw"),
            (1160, composed["heavy_shield_sword"], "hvy+sh"),
        ]
        for x, sp, lab in entries:
            box = sp.split()[-1].getbbox()
            if not box:
                continue
            crop = sp.crop(box)
            target_h = 52
            tw = max(8, int(round(crop.width * target_h / crop.height)))
            scaled = nearest_down(crop, tw, target_h)
            px = x - scaled.width // 2
            py = feet_y - scaled.height
            comp.alpha_composite(scaled, (px, py))
            draw.text((x - 18, feet_y + 6), lab, fill=(230, 210, 170))
        draw.text(
            (24, 24),
            "D4 modular @ gameplay scale (causeway V2.3)",
            fill=(230, 210, 170),
        )
        comp.convert("RGB").save(ROOT / "comparacoes" / "cmp_gameplay_causeway.png")

    # QC stats
    for name, im in [("body", body), ("light", light), ("heavy", heavy)]:
        opaque = sum(
            1
            for y in range(im.height)
            for x in range(im.width)
            if im.getpixel((x, y))[3] > 0
        )
        pink = sum(
            1
            for y in range(im.height)
            for x in range(im.width)
            if (c := im.getpixel((x, y)))[3] > 0
            and c[0] > 170
            and c[1] < 100
            and (c[0] - c[1]) > 90
        )
        print(f"{name}: opaque={opaque} residual_pink={pink} bbox={im.split()[-1].getbbox()}")

    body_px = sum(
        1
        for y in range(CELL[1])
        for x in range(CELL[0])
        if body.getpixel((x, y))[3] > 200 and light.getpixel((x, y))[3] < 30
    )
    print(f"body pixels not covered by coat: {body_px}")
    print("D4 compose done →", ROOT)


if __name__ == "__main__":
    main()
