"""Draws a flat 2D stand-in of the swordsman (concept B), side view, idle pose.

This is NOT the Blender model. Blender is not installed on the machine where
V1.1 was prepared, so this vector mock exists only to compare the two finishes
(pixel art x high resolution) and the two camera scales with the *same*
character, pose and framing. It is experimental and is not final art.

Coordinates are expressed in "heads" (the figure is 7 heads tall), x grows
toward the front (the character faces right) and y grows up from the sole.
The canvas is a multiple of the 64x64 sprite frame proposed in
docs/ASSET_CONTRACT.md, with the sole on the frame line y = 63 and the
pivot at x = 32.

Usage:
    python3 art-src/player/scripts/post/mock_figure.py [--scale 16]
"""

from __future__ import annotations

import argparse
import math
from pathlib import Path

from PIL import Image, ImageDraw

from contract import FIGURE_HEIGHT_PX, FRAME, HEADS, PIVOT
from palette import CHARACTER_RGB as C

PLAYER_DIR = Path(__file__).resolve().parents[2]
DEFAULT_OUT = PLAYER_DIR / "renders" / "mock_side_master.png"


class Canvas:
    def __init__(self, scale: int) -> None:
        self.scale = scale
        self.image = Image.new("RGBA", (FRAME * scale, FRAME * scale), (0, 0, 0, 0))
        self.draw = ImageDraw.Draw(self.image)
        self.unit = FIGURE_HEIGHT_PX * scale / HEADS
        self.origin = (PIVOT[0] * scale, PIVOT[1] * scale)

    def p(self, hx: float, hy: float) -> tuple[float, float]:
        return (self.origin[0] + hx * self.unit, self.origin[1] - hy * self.unit)

    def poly(self, points: list[tuple[float, float]], color: str) -> None:
        self.draw.polygon([self.p(x, y) for x, y in points], fill=C[color])

    def ellipse(self, x0: float, y0: float, x1: float, y1: float, color: str) -> None:
        (ax, ay), (bx, by) = self.p(x0, y1), self.p(x1, y0)
        self.draw.ellipse((ax, ay, bx, by), fill=C[color])

    def rect(self, x0: float, y0: float, x1: float, y1: float, color: str) -> None:
        self.poly([(x0, y0), (x1, y0), (x1, y1), (x0, y1)], color)


def blade(
    c: Canvas,
    start: tuple[float, float],
    tip: tuple[float, float],
    width_base: float,
    width_tip: float,
    curve: float,
) -> None:
    """Slightly curved, tapering blade with a light edge on the upper side."""
    dx, dy = tip[0] - start[0], tip[1] - start[1]
    length = math.hypot(dx, dy)
    ux, uy = dx / length, dy / length
    nx, ny = -uy, ux  # normal pointing to the back/upper side
    steps = 24
    back_edge, front_edge, highlight = [], [], []
    for i in range(steps + 1):
        t = i / steps
        bend = curve * 4.0 * t * (1.0 - t)
        cx = start[0] + dx * t + nx * bend
        cy = start[1] + dy * t + ny * bend
        half = (width_base * (1.0 - t) + width_tip * t) / 2.0
        back_edge.append((cx + nx * half, cy + ny * half))
        front_edge.append((cx - nx * half, cy - ny * half))
        highlight.append((cx + nx * half * 0.1, cy + ny * half * 0.1))
    c.poly(back_edge + front_edge[::-1], "silver")
    c.poly(back_edge + highlight[::-1], "silver_light")


def draw_swordsman(scale: int) -> Image.Image:
    c = Canvas(scale)

    # Half-cape over the back shoulder (asymmetric, short).
    c.poly(
        [(-0.10, 5.75), (-0.45, 5.65), (-0.80, 4.9), (-1.00, 3.85),
         (-0.70, 3.70), (-0.45, 3.95), (-0.15, 4.6)],
        "cape_wine",
    )
    c.poly([(-0.45, 5.4), (-0.75, 4.3), (-0.85, 3.85), (-0.70, 3.75), (-0.5, 4.1)],
           "cape_shadow")

    # Back leg (darker, slightly behind).
    c.poly([(-0.18, 3.6), (0.12, 3.6), (0.00, 2.1), (-0.30, 2.1)], "coat_shadow")
    c.poly([(-0.32, 2.25), (-0.02, 2.25), (-0.08, 0.35), (-0.05, 0.0),
            (-0.50, 0.0), (-0.45, 0.12), (-0.36, 0.4)], "boots")

    # Long open coat: back tail, shading, silver hem, wine lining at the edge.
    c.poly([(-0.30, 5.65), (-0.42, 4.6), (-0.55, 3.6), (-0.85, 1.40), (-0.10, 1.30),
            (0.20, 1.55), (0.30, 3.3), (0.34, 4.3), (0.36, 5.0), (0.20, 5.55),
            (0.05, 5.75)], "coat")
    c.poly([(-0.42, 4.6), (-0.55, 3.6), (-0.85, 1.40), (-0.55, 1.35), (-0.30, 3.3)],
           "coat_shadow")
    c.poly([(-0.85, 1.40), (-0.10, 1.30), (-0.10, 1.42), (-0.83, 1.52)], "silver_dark")
    c.poly([(0.26, 4.2), (0.34, 4.3), (0.30, 3.3), (0.20, 1.55), (0.10, 1.45),
            (0.20, 3.0)], "lining_wine")

    # Front leg with tall boot and cuff.
    c.poly([(-0.05, 3.6), (0.22, 3.6), (0.30, 2.15), (0.05, 2.15)], "trousers")
    c.poly([(0.02, 2.3), (0.34, 2.3), (0.30, 0.45), (0.55, 0.15), (0.58, 0.0),
            (0.05, 0.0), (0.06, 0.4)], "boots")
    c.poly([(0.0, 2.3), (0.36, 2.3), (0.35, 2.12), (0.01, 2.12)], "boot_cuff")

    # Chest under the open coat, belt and amber buckle.
    c.poly([(0.14, 5.45), (0.37, 5.0), (0.35, 4.35), (0.18, 4.35)], "coat_light")
    c.rect(-0.20, 4.22, 0.36, 4.36, "boots")
    c.rect(0.26, 4.18, 0.38, 4.40, "amber")

    # High collar with wine lining, neck, head, short hair, amber eye.
    c.poly([(-0.25, 5.6), (-0.32, 6.25), (-0.12, 6.35), (0.05, 5.9), (0.22, 5.6)],
           "coat_light")
    c.poly([(-0.20, 5.75), (-0.24, 6.15), (-0.13, 6.2), (0.0, 5.85)], "lining_wine")
    c.rect(0.0, 5.6, 0.18, 6.05, "skin_shadow")
    c.ellipse(-0.18, 6.0, 0.42, 7.0, "skin")
    c.poly([(-0.22, 6.25), (-0.25, 6.75), (-0.05, 7.02), (0.25, 7.02), (0.45, 6.8),
            (0.48, 6.6), (0.30, 6.68), (0.12, 6.55), (0.05, 6.35), (-0.05, 6.15)],
           "hair")
    c.poly([(0.0, 6.95), (0.25, 6.97), (0.35, 6.85), (0.1, 6.88)], "hair_light")
    c.rect(0.28, 6.46, 0.38, 6.56, "amber")

    # Front arm: small silver pauldron, sleeve, silver cuff.
    c.ellipse(-0.05, 5.42, 0.27, 5.72, "silver_dark")
    c.poly([(-0.02, 5.55), (0.24, 5.6), (0.40, 4.75), (0.20, 4.68)], "coat")
    c.poly([(0.20, 4.75), (0.40, 4.80), (0.66, 4.35), (0.50, 4.20)], "coat")
    c.poly([(0.50, 4.22), (0.66, 4.36), (0.72, 4.28), (0.56, 4.14)], "silver_dark")

    # Long, thin, slightly curved sword held low and forward.
    hand = (0.70, 4.20)
    tip = (2.10, 1.30)
    length = math.hypot(tip[0] - hand[0], tip[1] - hand[1])
    ux, uy = (tip[0] - hand[0]) / length, (tip[1] - hand[1]) / length
    nx, ny = -uy, ux
    pommel = (hand[0] - ux * 0.32, hand[1] - uy * 0.32)
    c.poly([(pommel[0] - nx * 0.05, pommel[1] - ny * 0.05),
            (pommel[0] + nx * 0.05, pommel[1] + ny * 0.05),
            (hand[0] + ux * 0.12 + nx * 0.05, hand[1] + uy * 0.12 + ny * 0.05),
            (hand[0] + ux * 0.12 - nx * 0.05, hand[1] + uy * 0.12 - ny * 0.05)],
           "glove")
    c.ellipse(pommel[0] - 0.08, pommel[1] - 0.08, pommel[0] + 0.08, pommel[1] + 0.08,
              "amber")
    guard = (hand[0] + ux * 0.16, hand[1] + uy * 0.16)
    c.poly([(guard[0] - nx * 0.20 - ux * 0.04, guard[1] - ny * 0.20 - uy * 0.04),
            (guard[0] + nx * 0.20 - ux * 0.04, guard[1] + ny * 0.20 - uy * 0.04),
            (guard[0] + nx * 0.20 + ux * 0.05, guard[1] + ny * 0.20 + uy * 0.05),
            (guard[0] - nx * 0.20 + ux * 0.05, guard[1] - ny * 0.20 + uy * 0.05)],
           "amber")
    blade(c, (guard[0] + ux * 0.05, guard[1] + uy * 0.05), tip,
          width_base=0.16, width_tip=0.05, curve=0.14)
    c.ellipse(hand[0] - 0.12, hand[1] - 0.11, hand[0] + 0.10, hand[1] + 0.11, "glove")

    return c.image


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--scale", type=int, default=16,
                        help="master pixels per sprite pixel (default 16)")
    parser.add_argument("--out", type=Path, default=DEFAULT_OUT)
    args = parser.parse_args()
    image = draw_swordsman(args.scale)
    args.out.parent.mkdir(parents=True, exist_ok=True)
    image.save(args.out)
    print(f"[mock_figure] {args.out} {image.size[0]}x{image.size[1]}")


if __name__ == "__main__":
    main()
