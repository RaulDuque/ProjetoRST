"""Build comparison board: D12 Pipeline A vs Blender layers vs pixelized Blender."""
from __future__ import annotations

from pathlib import Path

from PIL import Image, ImageDraw

LAB = Path(__file__).resolve().parents[1]
REPO = LAB.parents[2]
D12 = REPO / "assets/experimental/d12/pipelines/A_pixel_refined.png"
CMP = LAB / "comparacoes"
EXP = LAB / "exports"
REND = LAB / "renders"


def nearest_fit(im: Image.Image, size: tuple[int, int]) -> Image.Image:
    src = im.convert("RGBA")
    sw, sh = src.size
    dw, dh = size
    out = Image.new("RGBA", (dw, dh))
    sp, dp = src.load(), out.load()
    for y in range(dh):
        sy = min(sh - 1, y * sh // dh)
        for x in range(dw):
            sx = min(sw - 1, x * sw // dw)
            dp[x, y] = sp[sx, sy]
    return out


def nearest_scale(im: Image.Image, factor: int) -> Image.Image:
    if factor <= 1:
        return im.copy()
    src = im.convert("RGBA")
    w, h = src.size
    out = Image.new("RGBA", (w * factor, h * factor))
    sp, dp = src.load(), out.load()
    for y in range(h):
        for x in range(w):
            p = sp[x, y]
            for dy in range(factor):
                for dx in range(factor):
                    out.putpixel((x * factor + dx, y * factor + dy), p)
    return out


def posterize_pixel(im: Image.Image, cell=(128, 160)) -> Image.Image:
    small = nearest_fit(im, cell)
    # mild contrast lift
    px = small.load()
    for y in range(cell[1]):
        for x in range(cell[0]):
            r, g, b, a = px[x, y]
            if a < 16:
                px[x, y] = (0, 0, 0, 0)
                continue
            px[x, y] = (
                min(255, int(r * 1.08 + 6)),
                min(255, int(g * 1.05 + 4)),
                min(255, int(b * 1.05 + 4)),
                255 if a >= 128 else a,
            )
    return small


def main() -> None:
    CMP.mkdir(parents=True, exist_ok=True)
    d12 = Image.open(D12).convert("RGBA") if D12.exists() else Image.new("RGBA", (128, 160))
    full = Image.open(EXP / "layer_full.png").convert("RGBA") if (EXP / "layer_full.png").exists() else Image.open(REND / "materials_lights_B.png").convert("RGBA")
    pix = posterize_pixel(full)
    pix.save(LAB / "exports" / "blender_to_128x160.png")

    board = Image.new("RGBA", (920, 420), (22, 20, 26, 255))
    d = ImageDraw.Draw(board)
    d.text((16, 10), "D13 compare: D12-A | Blender B | Blender->128 | Blender 2x crop", fill=(230, 220, 210, 255))
    tiles = [
        ("D12 A", nearest_scale(d12, 2)),
        ("Blender B", nearest_fit(full, (256, 320))),
        ("pixelized", nearest_scale(pix, 2)),
    ]
    x = 16
    for label, tile in tiles:
        d.text((x, 40), label, fill=(200, 190, 180, 255))
        board.alpha_composite(tile, (x, 60))
        x += tile.size[0] + 24
    board.save(CMP / "d12_vs_blender.png")

    # real-scale mock (view 405 → ~114px tall)
    causeway = Image.new("RGBA", (1280, 720), (28, 32, 40, 255))
    cd = ImageDraw.Draw(causeway)
    cd.rectangle((0, 500, 1280, 720), fill=(50, 48, 56, 255))
    cd.line((0, 500, 1280, 500), fill=(90, 88, 96, 255))
    game = nearest_fit(pix, (91, 114))  # ~64wu at 405 view
    causeway.alpha_composite(game, (640 - 45, 500 - 114))
    causeway.alpha_composite(nearest_scale(pix, 3), (40, 40))
    cd.text((40, 16), "D13 pixelized 3x inspect", fill=(220, 210, 200, 255))
    cd.text((900, 40), "1x approx game scale", fill=(220, 210, 200, 255))
    causeway.save(CMP / "escala_real.png")

    # modularity strip
    strip = Image.new("RGBA", (1100, 360), (20, 22, 28, 255))
    sd = ImageDraw.Draw(strip)
    sd.text((12, 8), "Modularity (Blender collections)", fill=(230, 220, 210, 255))
    for i, name in enumerate(
        ["mod_no_cape.png", "mod_no_coat.png", "mod_with_armor.png", "materials_lights_B.png"]
    ):
        p = REND / name
        if not p.exists():
            continue
        im = nearest_fit(Image.open(p).convert("RGBA"), (200, 250))
        strip.alpha_composite(im, (20 + i * 260, 40))
        sd.text((20 + i * 260, 300), name.replace(".png", ""), fill=(190, 180, 170, 255))
    strip.save(CMP / "modularidade.png")
    print("OK compare_to_d12")


if __name__ == "__main__":
    main()
