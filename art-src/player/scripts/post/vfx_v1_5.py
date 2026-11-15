"""Frames of the player's visual effects (V1.5), drawn procedurally with PIL.

Effects go to the ``effects`` atlas of the player manifest (clip ids
``vfx_<key>``, where ``<key>`` is the ``vfx`` field of the skill data):

| Clip | For | Frames | Length |
|---|---|---|---|
| vfx_arc_white | Crescent Slash (``vfx: "arc_white"``) | 8 | 0.40 s (stretched to the real action) |
| vfx_arc_gold | Rising Edge (``vfx: "arc_gold"``) | 9 | 0.45 s |
| vfx_light_motes | Mending Light (``vfx: "light_motes"``) | 12 | 0.60 s; the burst of frame 0 is the heal (instant, at the start) |
| vfx_guard_spark | a hit blocked by the guard | 5 | 0.15 s |
| vfx_revive | leaving the dead state | 10 | 0.50 s |

Cell: the combat cell, 128x96 world units, at density 1 (1 texel per unit,
i.e. one screen pixel at the 1:1 camera: soft glows need no more), pivot
(40, 95) on the sole line, drawn facing right (the game mirrors). Every
shape is drawn 4x larger and reduced (anti-aliasing). Straight alpha; the
transparent texels carry the glow colour (no dark fringe with the linear
filter, contract section 3). Deterministic: the motes use a fixed seed.
The Ember Bolt (``fire_bolt``) keeps its projectile: no clip.
"""

from __future__ import annotations

import math
import random
from dataclasses import dataclass

from PIL import Image, ImageChops, ImageDraw, ImageFilter

CELL = (128, 96)
PIVOT = (40.0, 95.0)
DENSITY = 1
SS = 4  # supersampling
FPS = 20  # 0.05 s per frame, except the spark


@dataclass(frozen=True)
class Effect:
    key: str
    durations: tuple[float, ...]
    glow: tuple[int, int, int]
    core: tuple[int, int, int]


EFFECTS = {
    "vfx_arc_white": Effect("vfx_arc_white", (0.05,) * 8, (150, 185, 235), (246, 248, 255)),
    "vfx_arc_gold": Effect("vfx_arc_gold", (0.05,) * 9, (235, 160, 60), (255, 240, 190)),
    "vfx_light_motes": Effect("vfx_light_motes", (0.05,) * 12, (240, 205, 120), (255, 248, 220)),
    "vfx_guard_spark": Effect("vfx_guard_spark", (0.03,) * 5, (140, 175, 235), (250, 252, 255)),
    "vfx_revive": Effect("vfx_revive", (0.05,) * 10, (200, 190, 240), (250, 246, 255)),
}


def _canvas() -> tuple[Image.Image, ImageDraw.ImageDraw]:
    mask = Image.new("L", (CELL[0] * SS, CELL[1] * SS), 0)
    return mask, ImageDraw.Draw(mask)


def _compose(effect: Effect, core: Image.Image, glow_radius: float, glow_gain: float, fade: float) -> Image.Image:
    """Core mask + blurred glow -> RGBA frame at cell size."""
    core = core.resize(CELL, Image.Resampling.LANCZOS)
    glow = core.filter(ImageFilter.GaussianBlur(glow_radius)).point(lambda v: min(255, int(v * glow_gain)))
    alpha = ImageChops.lighter(core, glow).point(lambda v: int(v * fade))
    rgb = Image.new("RGB", CELL, effect.glow)
    rgb.paste(Image.new("RGB", CELL, effect.core), (0, 0), core)
    out = rgb.convert("RGBA")
    out.putalpha(alpha)
    return out


def _crescent(draw: ImageDraw.ImageDraw, centre: tuple[float, float], radius: float, head: float, length: float,
              width: float, sign: int = 1) -> None:
    """Crescent trail ending at angle ``head`` (deg, y down), ``length`` deg long,
    thickest near the head and tapering to the tail. ``sign`` = sweep direction."""
    cx, cy = centre
    steps = 48
    outer, inner = [], []
    for i in range(steps + 1):
        s = i / steps  # 0 tail .. 1 head
        a = math.radians(head - sign * length * (1.0 - s))
        w = width * (math.sin(math.pi * min(1.0, s * 1.15)) ** 0.7) if s < 1 else width * 0.35
        outer.append(((cx + (radius + w / 2) * math.cos(a)) * SS, (cy + (radius + w / 2) * math.sin(a)) * SS))
        inner.append(((cx + (radius - w / 2) * math.cos(a)) * SS, (cy + (radius - w / 2) * math.sin(a)) * SS))
    draw.polygon(outer + inner[::-1], fill=255)


def _dot(draw: ImageDraw.ImageDraw, x: float, y: float, r: float, value: int = 255) -> None:
    draw.ellipse(((x - r) * SS, (y - r) * SS, (x + r) * SS, (y + r) * SS), fill=value)


def arc_white() -> list[Image.Image]:
    """Crescent Slash: a wide arc from high-front to low-front, around the chest.
    Frame 0 already shows the cut (the hitbox exists from the first frame)."""
    effect = EFFECTS["vfx_arc_white"]
    heads = (-25.0, 15.0, 45.0, 60.0, 64.0, 66.0, 67.0, 68.0)
    lengths = (70.0, 105.0, 130.0, 130.0, 118.0, 100.0, 80.0, 60.0)
    fades = (1.0, 1.0, 1.0, 0.85, 0.65, 0.45, 0.28, 0.14)
    frames = []
    for head, length, fade in zip(heads, lengths, fades):
        mask, draw = _canvas()
        width = 9.0 if fade > 0.8 else 9.0 * (0.6 + 0.4 * fade)
        _crescent(draw, (46.0, 62.0), 40.0 + (1.0 - fade) * 6.0, head, length, width)
        frames.append(_compose(effect, mask, 2.2, 1.3, fade))
    return frames


def arc_gold() -> list[Image.Image]:
    """Rising Edge: a narrower arc climbing from low-front to above the head,
    in gold (Light element); unlike the crescent it sweeps upward."""
    effect = EFFECTS["vfx_arc_gold"]
    heads = (30.0, -15.0, -60.0, -95.0, -110.0, -115.0, -118.0, -120.0, -121.0)
    lengths = (60.0, 95.0, 125.0, 140.0, 130.0, 110.0, 90.0, 70.0, 50.0)
    fades = (1.0, 1.0, 1.0, 1.0, 0.8, 0.6, 0.42, 0.26, 0.12)
    frames = []
    for head, length, fade in zip(heads, lengths, fades):
        mask, draw = _canvas()
        _crescent(draw, (48.0, 58.0), 34.0 + (1.0 - fade) * 4.0, head, length, 7.5 * (0.6 + 0.4 * fade), sign=-1)
        frames.append(_compose(effect, mask, 2.4, 1.4, fade))
    return frames


def light_motes() -> list[Image.Image]:
    """Mending Light: a pale burst around the body on frame 0 (the heal is
    instant when the spell starts), then motes rising and fading, and a faint
    halo over the raised hand. Kept dim: candlelight, not a modern flash."""
    effect = EFFECTS["vfx_light_motes"]
    rng = random.Random(15)
    motes = [(rng.uniform(22.0, 60.0), rng.uniform(70.0, 94.0), rng.uniform(26.0, 48.0), rng.uniform(0.8, 1.6),
              rng.uniform(0.0, 0.25)) for _ in range(16)]
    frames = []
    n = len(effect.durations)
    for k in range(n):
        t = k / n  # the last frame still shows the last motes fading
        mask, draw = _canvas()
        if k < 3:  # the heal: a soft ring growing from the chest
            r = 12.0 + 6.0 * k  # stays inside the cell (bottom at most at the sole line)
            draw.ellipse(((40 - r) * SS, (64 - r * 1.3) * SS, (40 + r) * SS, (64 + r * 1.3) * SS),
                         outline=int(220 * (1.0 - k / 3)), width=int(2.2 * SS))
        for x, y0, rise, size, delay in motes:
            local = (t - delay) / (1.0 - delay)
            if local < 0.0:
                continue
            y = y0 - rise * local
            sway = 1.5 * math.sin(6.0 * local + x)
            value = int(255 * min(1.0, 3.0 * local) * (1.0 - local) ** 0.6)
            _dot(draw, x + sway, y, size, value)
        halo = 0.6 * math.sin(math.pi * min(1.0, t * 1.4))
        if halo > 0.05:
            _dot(draw, 46.0, 30.0, 3.5, int(255 * halo))
        frames.append(_compose(effect, mask, 2.0, 1.8, 0.9))
    return frames


def guard_spark() -> list[Image.Image]:
    """A short burst on the guard, in front of the chest: the blocked hit."""
    effect = EFFECTS["vfx_guard_spark"]
    rays = (-70.0, -35.0, -5.0, 25.0, 60.0, 180.0 - 30.0, 180.0 + 35.0)
    frames = []
    for k in range(len(effect.durations)):
        mask, draw = _canvas()
        cx, cy = 58.0, 64.0
        r0, r1 = 2.0 + 3.0 * k, 7.0 + 5.0 * k
        for angle in rays:
            a = math.radians(angle)
            draw.line(((cx + r0 * math.cos(a)) * SS, (cy + r0 * math.sin(a)) * SS,
                       (cx + r1 * math.cos(a)) * SS, (cy + r1 * math.sin(a)) * SS),
                      fill=255, width=int(max(1.0, 2.2 - 0.4 * k) * SS))
        if k < 2:
            _dot(draw, cx, cy, 3.5 - k)
        frames.append(_compose(effect, mask, 1.6, 1.5, 1.0 - 0.18 * k))
    return frames


def revive() -> list[Image.Image]:
    """Revive: a pale column rising over the body and a ring on the floor."""
    effect = EFFECTS["vfx_revive"]
    frames = []
    n = len(effect.durations)
    for k in range(n):
        t = k / n
        mask, draw = _canvas()
        top = 95.0 - 70.0 * min(1.0, t * 1.8)
        half = 9.0 * (1.0 - 0.5 * t)
        value = 150.0 * (1.0 - t) ** 0.8
        rows = max(1, int((95.0 - top) * SS))
        for row in range(rows):  # brighter at the floor, fading toward the top
            y = 95.0 * SS - row
            shade = int(value * (1.0 - row / rows) ** 1.5)
            w = half * (1.0 - 0.4 * row / rows)
            draw.line(((40 - w) * SS, y, (40 + w) * SS, y), fill=shade)
        r = 10.0 + 20.0 * t
        draw.ellipse(((40 - r) * SS, (94 - r * 0.18) * SS, (40 + r) * SS, (94 + r * 0.18) * SS),
                     outline=int(255 * (1.0 - t)), width=int(1.6 * SS))
        frames.append(_compose(effect, mask.filter(ImageFilter.GaussianBlur(2.0 * SS)), 3.0, 1.2, 1.0))
    return frames


BUILDERS = {"vfx_arc_white": arc_white, "vfx_arc_gold": arc_gold, "vfx_light_motes": light_motes,
            "vfx_guard_spark": guard_spark, "vfx_revive": revive}


def all_frames() -> dict[str, list[Image.Image]]:
    out = {key: build() for key, build in BUILDERS.items()}
    for key, frames in out.items():
        if len(frames) != len(EFFECTS[key].durations):
            raise ValueError(f"{key}: {len(frames)} frames for {len(EFFECTS[key].durations)} durations")
        for i, frame in enumerate(frames):
            if frame.size != CELL or frame.getchannel("A").getbbox() is None:
                raise ValueError(f"{key} frame {i}: size {frame.size} or empty")
    return out
