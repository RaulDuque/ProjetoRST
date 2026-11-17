"""Shared D9 helpers."""
from __future__ import annotations

import json
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[1]
REPO = ROOT.parents[2]
VIS = REPO / "art-src" / "visual-prototypes"
D2 = VIS / "d2_protagonista_B"
D4 = VIS / "d4_modular_d2"
D8 = VIS / "d8_atlas_modular_2d"

CELL = (128, 160)
PIVOT = (64, 156)
GAME_H = 52
LAB_H = 110

# D2-inspired palette for reconstruction / heavy recolor
PAL = {
    "outline": (6, 2, 10, 255),
    "hair": (29, 27, 51, 255),
    "hair_hi": (48, 44, 78, 255),
    "skin": (205, 145, 109, 255),
    "skin_sh": (156, 113, 96, 255),
    "coat": (29, 26, 55, 255),
    "coat_hi": (48, 44, 78, 255),
    "wine": (64, 0, 16, 255),
    "wine_hi": (96, 24, 40, 255),
    "gold": (188, 152, 72, 255),
    "gold_dk": (120, 88, 40, 255),
    "steel": (174, 168, 180, 255),
    "steel_hi": (240, 235, 239, 255),
    "steel_dk": (88, 96, 112, 255),
    "boot": (44, 16, 15, 255),
    "boot_hi": (72, 48, 36, 255),
    "pants": (20, 18, 35, 255),
    "shirt": (40, 36, 56, 255),
    "shirt_hi": (56, 52, 72, 255),
}

PALETTE_RGB = [(c[0], c[1], c[2]) for c in PAL.values()]


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
        (max(0, x0 - pad), max(0, y0 - pad), min(im.width, x1 + pad), min(im.height, y1 + pad))
    )


def scale_to_height(im: Image.Image, th: int) -> Image.Image:
    im = trim(im.convert("RGBA"))
    if im.height <= 0:
        return Image.new("RGBA", (1, th), (0, 0, 0, 0))
    tw = max(1, int(round(im.width * th / im.height)))
    return nearest(im, tw, th)


def feet_place(sprite: Image.Image) -> Image.Image:
    out = canvas()
    sp = trim(sprite.convert("RGBA"))
    if sp.width == 0:
        return out
    x = PIVOT[0] - sp.width // 2
    y = PIVOT[1] - sp.height + 1
    out.alpha_composite(sp, (max(0, x), max(0, y)))
    return out


def scale_game(im: Image.Image) -> Image.Image:
    return scale_to_height(im, GAME_H)


def strip_chroma(im: Image.Image) -> Image.Image:
    im = im.convert("RGBA")
    px = im.load()
    for y in range(im.height):
        for x in range(im.width):
            r, g, b, a = px[x, y]
            if a < 8:
                continue
            lum = r + g + b
            if r > 200 and g < 80 and b > 100:
                px[x, y] = (0, 0, 0, 0)
            elif r > 160 and g < 110 and b > 70 and (r - g) > 70 and lum > 280:
                px[x, y] = (0, 0, 0, 0)
    return im


def nearest_pal(rgb: tuple[int, int, int]) -> tuple[int, int, int]:
    r, g, b = rgb
    best, bd = PALETTE_RGB[0], 10**9
    for pr, pg, pb in PALETTE_RGB:
        d = (r - pr) ** 2 + (g - pg) ** 2 + (b - pb) ** 2
        if d < bd:
            bd, best = d, (pr, pg, pb)
    return best


def quantize_d2(im: Image.Image) -> Image.Image:
    im = im.convert("RGBA")
    out = Image.new("RGBA", im.size)
    s, d = im.load(), out.load()
    for y in range(im.height):
        for x in range(im.width):
            r, g, b, a = s[x, y]
            if a < 16:
                d[x, y] = (0, 0, 0, 0)
            else:
                pr, pg, pb = nearest_pal((r, g, b))
                d[x, y] = (pr, pg, pb, 255)
    return out


def board(items, path: Path, title: str, bg=(12, 10, 18), zoom=1) -> None:
    imgs = []
    for lab, im in items:
        im = im.convert("RGBA")
        if zoom > 1:
            im = nearest_up(im, zoom)
        imgs.append((lab, im))
    gap = 12
    max_h = max(im.height for _, im in imgs)
    w = gap + sum(im.width + gap for _, im in imgs)
    h = max_h + 48
    out = Image.new("RGB", (w, h), bg)
    d = ImageDraw.Draw(out)
    d.text((8, 6), title, fill=(230, 210, 170))
    x = gap
    for lab, im in imgs:
        y = 28 + (max_h - im.height)
        out.paste(im.convert("RGB"), (x, y), im)
        d.text((x, h - 16), lab[:22], fill=(180, 180, 190))
        x += im.width + gap
    path.parent.mkdir(parents=True, exist_ok=True)
    out.save(path)


def save_gif(frames, path: Path, duration=90) -> None:
    if not frames:
        return
    pals = []
    for fr in frames:
        bg = Image.new("RGBA", fr.size, (12, 10, 18, 255))
        bg.alpha_composite(fr)
        pals.append(nearest_up(bg, 2).convert("P", palette=Image.ADAPTIVE))
    path.parent.mkdir(parents=True, exist_ok=True)
    pals[0].save(path, save_all=True, append_images=pals[1:], duration=duration, loop=0)


def write_json(path: Path, data) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2), encoding="utf-8")


def load_d2_frames(clip: str) -> list[Image.Image]:
    if clip == "idle":
        folder = D2 / "animacoes" / "idle" / "frames"
        names = [f"idle_f{i:02d}.png" for i in range(6)]
    else:
        folder = D2 / "animacoes" / "run" / "frames"
        names = [f"run_f{i:02d}.png" for i in range(8)]
    return [Image.open(folder / n).convert("RGBA") for n in names if (folder / n).exists()]


def load_d4(name: str) -> Image.Image | None:
    p = D4 / "camadas" / name
    if not p.exists():
        return None
    return strip_chroma(Image.open(p).convert("RGBA"))


def load_causeway() -> Image.Image | None:
    for p in (
        VIS / "07_comparacoes" / "ref_scenery_causeway.png",
        D2 / "comparacoes" / "gameplay_scale_causeway.png",
    ):
        if p.exists():
            return Image.open(p).convert("RGBA")
    return None


def opaque_count(im: Image.Image) -> int:
    return sum(1 for p in im.getdata() if p[3] > 16)


def layer_bbox(im: Image.Image):
    return im.split()[-1].getbbox()
