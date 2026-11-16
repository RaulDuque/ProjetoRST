"""Shared D7 helpers: palette, cell, boards, nearest scale, feet register."""
from __future__ import annotations

from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[1]
REPO = ROOT.parents[2]
VIS = REPO / "art-src" / "visual-prototypes"
D2 = VIS / "d2_protagonista_B"
D4 = VIS / "d4_modular_d2"
D5 = VIS / "d5_corrida_modular"
D6 = VIS / "d6_fidelidade_d2"

CELL = (128, 160)
PIVOT = (64, 156)  # sole center
GAME_H = 52

# Quantized D2-inspired palette (sampled from idle_f00 + identity docs)
PAL = {
    "void": (0, 0, 0, 0),
    "outline": (6, 2, 10, 255),
    "hair": (29, 27, 51, 255),
    "hair_hi": (48, 44, 78, 255),
    "skin": (205, 145, 109, 255),
    "skin_sh": (156, 113, 96, 255),
    "coat": (29, 26, 55, 255),
    "coat_hi": (48, 44, 78, 255),
    "coat_edge": (16, 12, 28, 255),
    "wine": (64, 0, 16, 255),
    "wine_hi": (96, 24, 40, 255),
    "wine_dk": (38, 0, 17, 255),
    "gold": (188, 152, 72, 255),
    "gold_dk": (120, 88, 40, 255),
    "steel": (174, 168, 180, 255),
    "steel_hi": (240, 235, 239, 255),
    "steel_dk": (88, 96, 112, 255),
    "boot": (44, 16, 15, 255),
    "boot_hi": (72, 48, 36, 255),
    "pants": (20, 18, 35, 255),
    "shirt": (32, 28, 48, 255),
    "armor": (56, 60, 72, 255),
    "armor_hi": (120, 128, 144, 255),
}

PALETTE_RGB = [
    (c[0], c[1], c[2])
    for k, c in PAL.items()
    if k != "void" and c[3] > 0
]


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


def nearest_up(im: Image.Image, scale: int) -> Image.Image:
    im = im.convert("RGBA")
    w, h = im.size
    out = Image.new("RGBA", (w * scale, h * scale))
    s, d = im.load(), out.load()
    for y in range(h):
        for x in range(w):
            c = s[x, y]
            for dy in range(scale):
                for dx in range(scale):
                    d[x * scale + dx, y * scale + dy] = c
    return out


def canvas() -> Image.Image:
    return Image.new("RGBA", CELL, (0, 0, 0, 0))


def trim(im: Image.Image, pad: int = 0) -> Image.Image:
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


def scale_to_height(im: Image.Image, th: int) -> Image.Image:
    im = trim(im.convert("RGBA"))
    if im.height == 0:
        return Image.new("RGBA", (1, th), (0, 0, 0, 0))
    tw = max(1, int(round(im.width * th / im.height)))
    return nearest(im, tw, th)


def feet_place(sprite: Image.Image, pivot: tuple[int, int] = PIVOT) -> Image.Image:
    """Place opaque sprite with bottom-center on pivot."""
    out = canvas()
    sp = trim(sprite.convert("RGBA"))
    if sp.width == 0:
        return out
    x = pivot[0] - sp.width // 2
    y = pivot[1] - sp.height + 1
    out.alpha_composite(sp, (max(0, x), max(0, y)))
    return out


def scale_game(im: Image.Image) -> Image.Image:
    return scale_to_height(im, GAME_H)


def board(
    rows: list[tuple[str, Image.Image]],
    path: Path,
    title: str,
    bg: tuple[int, int, int] = (12, 10, 18),
    zoom: int = 1,
) -> None:
    imgs: list[tuple[str, Image.Image]] = []
    for lab, im in rows:
        im = im.convert("RGBA")
        if zoom > 1:
            im = nearest_up(im, zoom)
        imgs.append((lab, im))
    gap = 14
    max_h = max(im.height for _, im in imgs)
    w = gap + sum(im.width + gap for _, im in imgs)
    h = max_h + 52
    out = Image.new("RGB", (w, h), bg)
    d = ImageDraw.Draw(out)
    d.text((8, 6), title, fill=(230, 210, 170))
    x = gap
    for lab, im in imgs:
        y = 30 + (max_h - im.height)
        out.paste(im.convert("RGB"), (x, y), im)
        d.text((x, h - 18), lab[:22], fill=(180, 180, 190))
        x += im.width + gap
    path.parent.mkdir(parents=True, exist_ok=True)
    out.save(path)


def nearest_palette(rgb: tuple[int, int, int]) -> tuple[int, int, int]:
    r, g, b = rgb
    best = PALETTE_RGB[0]
    best_d = 10**9
    for pr, pg, pb in PALETTE_RGB:
        d = (r - pr) ** 2 + (g - pg) ** 2 + (b - pb) ** 2
        if d < best_d:
            best_d = d
            best = (pr, pg, pb)
    return best


def quantize_to_d2(im: Image.Image) -> Image.Image:
    im = im.convert("RGBA")
    out = Image.new("RGBA", im.size)
    s, d = im.load(), out.load()
    for y in range(im.height):
        for x in range(im.width):
            r, g, b, a = s[x, y]
            if a < 16:
                d[x, y] = (0, 0, 0, 0)
            else:
                pr, pg, pb = nearest_palette((r, g, b))
                d[x, y] = (pr, pg, pb, 255)
    return out


def load_d2_idle() -> Image.Image:
    return Image.open(D2 / "animacoes" / "idle" / "frames" / "idle_f00.png").convert("RGBA")


def load_d2_idle_3x() -> Image.Image:
    p = ROOT / "referencias" / "d2_idle_3x.png"
    if p.exists():
        return Image.open(p).convert("RGBA")
    return nearest_up(load_d2_idle(), 3)


def load_causeway() -> Image.Image | None:
    candidates = [
        VIS / "07_comparacoes" / "ref_scenery_causeway.png",
        D2 / "comparacoes" / "gameplay_scale_causeway.png",
        REPO / "target" / "world_visual_showcase" / "after_causeway.png",
    ]
    for p in candidates:
        if p.exists():
            return Image.open(p).convert("RGBA")
    return None


def ensure_dirs() -> None:
    for rel in (
        "estrategias/A_primitivas",
        "estrategias/B_sprites_masks",
        "estrategias/C_hibrido",
        "personagem",
        "equipamentos",
        "animacoes/idle",
        "animacoes/run",
        "loadouts/leve",
        "loadouts/pesado",
        "previews",
        "comparacoes",
        "testes",
        "referencias",
    ):
        (ROOT / rel).mkdir(parents=True, exist_ok=True)
