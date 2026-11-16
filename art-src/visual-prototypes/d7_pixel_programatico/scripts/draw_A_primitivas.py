#!/usr/bin/env python3
"""D7-A: side-view protagonist drawn only with Pillow primitives (no D2 blit)."""
from __future__ import annotations

from pathlib import Path

from PIL import Image, ImageDraw

from d7_common import CELL, PAL, PIVOT, ROOT, board, canvas, ensure_dirs, feet_place, nearest_up, scale_game

OUT = ROOT / "estrategias" / "A_primitivas"


def _ellipse(draw: ImageDraw.ImageDraw, box, fill, outline=None) -> None:
    draw.ellipse(box, fill=fill, outline=outline)


def draw_pose() -> Image.Image:
    """Static right-facing idle-ish pose at cell resolution."""
    im = canvas()
    d = ImageDraw.Draw(im)
    ox, oy = PIVOT[0], PIVOT[1]

    # Cape back (wine lining + indigo exterior) — ragged hem
    cape_outer = [
        (ox - 10, oy - 88),
        (ox - 18, oy - 70),
        (ox - 22, oy - 40),
        (ox - 26, oy - 10),
        (ox - 20, oy - 2),
        (ox - 8, oy - 8),
        (ox - 4, oy - 50),
        (ox - 2, oy - 78),
    ]
    d.polygon(cape_outer, fill=PAL["coat"])
    cape_wine = [
        (ox - 8, oy - 86),
        (ox - 14, oy - 60),
        (ox - 18, oy - 25),
        (ox - 16, oy - 4),
        (ox - 6, oy - 12),
        (ox - 2, oy - 55),
    ]
    d.polygon(cape_wine, fill=PAL["wine"])
    # ragged tips
    for tip in ((ox - 24, oy - 6), (ox - 20, oy - 1), (ox - 14, oy - 3)):
        d.point(tip, fill=PAL["wine_hi"])
        d.point((tip[0] + 1, tip[1] - 1), fill=PAL["wine_dk"])

    # Legs / boots
    d.rectangle([ox - 4, oy - 28, ox + 2, oy - 6], fill=PAL["pants"])
    d.rectangle([ox + 1, oy - 26, ox + 7, oy - 6], fill=PAL["pants"])
    d.polygon(
        [(ox - 5, oy - 8), (ox + 1, oy - 8), (ox + 2, oy - 1), (ox - 6, oy - 1)],
        fill=PAL["boot"],
    )
    d.polygon(
        [(ox + 1, oy - 8), (ox + 8, oy - 8), (ox + 10, oy - 1), (ox + 1, oy - 1)],
        fill=PAL["boot"],
    )
    d.point((ox + 8, oy - 3), fill=PAL["boot_hi"])
    d.point((ox - 2, oy - 3), fill=PAL["boot_hi"])

    # Torso / coat
    coat = [
        (ox - 6, oy - 86),
        (ox + 8, oy - 86),
        (ox + 12, oy - 70),
        (ox + 10, oy - 40),
        (ox + 8, oy - 28),
        (ox - 4, oy - 28),
        (ox - 8, oy - 50),
        (ox - 8, oy - 74),
    ]
    d.polygon(coat, fill=PAL["coat"])
    # lapel wine
    d.polygon(
        [(ox + 2, oy - 84), (ox + 8, oy - 78), (ox + 6, oy - 60), (ox + 1, oy - 68)],
        fill=PAL["wine"],
    )
    d.point((ox + 4, oy - 72), fill=PAL["wine_hi"])
    # gold clasps
    for gy in (oy - 76, oy - 68, oy - 60):
        d.point((ox + 3, gy), fill=PAL["gold"])
        d.point((ox + 4, gy), fill=PAL["gold_dk"])
    # sleeve
    d.polygon(
        [(ox + 6, oy - 70), (ox + 14, oy - 62), (ox + 12, oy - 48), (ox + 4, oy - 52)],
        fill=PAL["coat_hi"],
    )
    # glove
    d.ellipse([ox + 10, oy - 50, ox + 16, oy - 42], fill=PAL["boot"])

    # Head
    _ellipse(d, [ox - 2, oy - 104, ox + 10, oy - 88], PAL["skin"])
    d.polygon(
        [
            (ox - 1, oy - 102),
            (ox + 9, oy - 104),
            (ox + 11, oy - 96),
            (ox + 6, oy - 92),
            (ox - 2, oy - 94),
        ],
        fill=PAL["hair"],
    )
    d.point((ox + 4, oy - 100), fill=PAL["hair_hi"])
    d.point((ox + 7, oy - 97), fill=PAL["skin_sh"])  # cheek/nose shadow
    d.point((ox + 8, oy - 95), fill=PAL["outline"])  # eye

    # Sheath behind hip
    d.line([(ox - 2, oy - 36), (ox - 10, oy - 18)], fill=PAL["gold_dk"], width=2)
    d.line([(ox - 3, oy - 36), (ox - 11, oy - 18)], fill=PAL["coat_edge"], width=1)

    # Curved saber (front)
    blade = []
    for i in range(18):
        t = i / 17
        x = ox + 12 + int(t * 22)
        y = oy - 48 + int(t * 28) + int((t * t) * 6)
        blade.append((x, y))
    for i, (x, y) in enumerate(blade):
        d.point((x, y), fill=PAL["steel_hi"] if i % 3 == 0 else PAL["steel"])
        d.point((x, y + 1), fill=PAL["steel_dk"])
    # hilt
    d.line([(ox + 10, oy - 48), (ox + 14, oy - 44)], fill=PAL["gold"], width=2)
    d.point((ox + 9, oy - 49), fill=PAL["gold_dk"])

    # 1px outline pass on silhouette edge
    return _outline_silhouette(im)


def _outline_silhouette(im: Image.Image) -> Image.Image:
    out = im.copy()
    s = im.load()
    d = out.load()
    w, h = im.size
    for y in range(h):
        for x in range(w):
            if s[x, y][3] < 128:
                continue
            edge = False
            for nx, ny in ((x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)):
                if not (0 <= nx < w and 0 <= ny < h) or s[nx, ny][3] < 128:
                    edge = True
                    break
            if edge:
                # darken edge pixel slightly rather than replace all
                r, g, b, a = s[x, y]
                d[x, y] = (max(0, r // 3), max(0, g // 3), max(0, b // 3), a)
    return out


def layers_from_primitives() -> dict[str, Image.Image]:
    """Export approximate independent layers for architecture proof (prototype)."""
    full = draw_pose()
    # crude vertical splits for docs — not production cuts
    head = canvas()
    body = canvas()
    cape = canvas()
    sword = canvas()
    boots = canvas()
    s = full.load()
    for y in range(CELL[1]):
        for x in range(CELL[0]):
            r, g, b, a = s[x, y]
            if a < 16:
                continue
            # sword: bright steel region right-forward
            if r > 150 and abs(r - g) < 40 and x > PIVOT[0] + 8:
                sword.putpixel((x, y), (r, g, b, a))
            elif y > PIVOT[1] - 12:
                boots.putpixel((x, y), (r, g, b, a))
            elif y < PIVOT[1] - 88:
                head.putpixel((x, y), (r, g, b, a))
            elif b > r + 10 and r < 50:  # indigo cape/coat back-biased
                if x < PIVOT[0] - 2:
                    cape.putpixel((x, y), (r, g, b, a))
                else:
                    body.putpixel((x, y), (r, g, b, a))
            else:
                body.putpixel((x, y), (r, g, b, a))
    return {
        "full": full,
        "head": head,
        "body": body,
        "cape": cape,
        "sword": sword,
        "boots": boots,
    }


def main() -> None:
    ensure_dirs()
    OUT.mkdir(parents=True, exist_ok=True)
    layers = layers_from_primitives()
    full = layers["full"]
    full.save(OUT / "pose_estatica.png")
    nearest_up(full, 3).save(OUT / "pose_estatica_3x.png")
    scale_game(full).save(OUT / "pose_escala_jogo.png")
    for name, im in layers.items():
        if name == "full":
            continue
        im.save(OUT / f"layer_{name}.png")
    board(
        [
            ("D7-A cell", full),
            ("D7-A 3x", nearest_up(full, 3)),
            ("D7-A game", nearest_up(scale_game(full), 3)),
        ],
        OUT / "preview_board.png",
        "D7-A primitivas (Pillow only)",
    )
    print(f"D7-A wrote {OUT}")


if __name__ == "__main__":
    main()
