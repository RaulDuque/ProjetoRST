"""Camera scale comparison: current 1:1 view x a study of a x2 zoom.

The scene is rebuilt from the real room data (assets/maps/entry_hall.ron and
assets/data/enemies/castle_undead.ron) with the placeholder colours used by
the game, and framed with the same rule as src/world/camera.rs
(`clamp_axis` + look-ahead), with the view size divided by the zoom. The game
camera is not changed; this is an offline study.

Limitations (also stated on the images):
- Static frame: no smoothing, no motion, no effects.
- Enemies are drawn standing on the floor at their spawn x.
- The HUD panel is cut from the real autoplay screenshot when it exists
  (target/autoplay/01_start.png); it stays in screen space in both scales.
- The player is the experimental swordsman sprite (vector stand-in),
  always facing right, so the look-ahead is always +60.
- Line comments (//) are stripped before parsing; the parser is a regex,
  not a full RON reader, and every enemy uses the first definition found.

- With ``--zooms`` (V1.2-B) any zoom, fractional included, is drawn for
  both finishes, and ``--projectile`` adds an Ember Bolt in flight (size
  from assets/data/spells/elemental.ron, launch offset and colour mirrored
  from src/magic/casting.rs). A pixel-art sprite at a fractional zoom is
  scaled with nearest neighbour on purpose: the uneven pixels it shows are
  the artefact being evaluated.

Usage:
    python3 art-src/player/scripts/post/camera_compare.py
    python3 art-src/player/scripts/post/camera_compare.py --zooms 1,1.5,2 --projectile \
        --name swordsman_base_v002_side --out-dir DIR
"""

from __future__ import annotations

import argparse
import re
from dataclasses import dataclass
from pathlib import Path

from PIL import Image, ImageDraw

from contract import FIGURE_HEIGHT_PX, HD_DENSITY, PIVOT
from labels import font
from palette import (
    SCENE_BACKGROUND_FALLBACK,
    SCENE_FIRE_BOLT,
    SCENE_ONE_WAY,
    SCENE_PLATFORM,
    srgb_float,
)
from resample import resize

PLAYER_DIR = Path(__file__).resolve().parents[2]
PROJECT = PLAYER_DIR.parents[1]
ROOM_FILE = PROJECT / "assets" / "maps" / "entry_hall.ron"
ENEMY_FILE = PROJECT / "assets" / "data" / "enemies" / "castle_undead.ron"
SPELL_FILE = PROJECT / "assets" / "data" / "spells" / "elemental.ron"
SCREENSHOT = PROJECT / "target" / "autoplay" / "01_start.png"
RENDERS = PLAYER_DIR / "renders"
OUT_DIR = PLAYER_DIR / "comparisons"
DEFAULT_SPRITE_NAME = "swordsman_idle_side"

# Mirrors src/core/constants.rs (WINDOW_*, PLAYER_SIZE, CAMERA_LOOK_AHEAD).
WINDOW = (1280, 720)
PLAYER_SIZE = (26.0, 46.0)
CAMERA_LOOK_AHEAD = 60.0
HUD_BOX = (0, 0, 362, 150)
# Mirrors src/magic/casting.rs: CAST_HEIGHT and the launch gap (radius + 16).
CAST_HEIGHT = 6.0
CAST_GAP = 16.0
PROJECTILE_TRAVEL = 300.0  # distance flown when the still is taken (study choice)

NUM = r"(-?\d+(?:\.\d+)?)"


@dataclass
class Platform:
    min: tuple[float, float]
    size: tuple[float, float]
    one_way: bool


@dataclass
class Room:
    size: tuple[float, float]
    background: tuple[int, int, int]
    platforms: list[Platform]
    enemies: list[tuple[str, float]]


def strip_comments(text: str) -> str:
    return re.sub(r"//[^\n]*", "", text)


def parse_room(text: str) -> Room:
    text = strip_comments(text)
    size = re.search(rf"size:\s*\({NUM},\s*{NUM}\)", text)
    background = re.search(rf"background:\s*\({NUM},\s*{NUM},\s*{NUM}\)", text)
    if size is None:
        raise ValueError("room size not found")
    platforms = [
        Platform((float(m[1]), float(m[2])), (float(m[3]), float(m[4])), m[5] is not None)
        for m in re.finditer(
            rf"\(min:\s*\({NUM},\s*{NUM}\),\s*size:\s*\({NUM},\s*{NUM}\)(,\s*one_way:\s*true)?",
            text,
        )
    ]
    enemies = [
        (m[1], float(m[2]))
        for m in re.finditer(rf'enemy:\s*"(\w+)",\s*position:\s*\({NUM},\s*{NUM}\)', text)
    ]
    bg = (
        srgb_float(float(background[1]), float(background[2]), float(background[3]))
        if background
        else SCENE_BACKGROUND_FALLBACK
    )
    return Room((float(size[1]), float(size[2])), bg, platforms, enemies)


def parse_enemy_look(text: str) -> tuple[tuple[float, float], tuple[int, int, int]]:
    text = strip_comments(text)
    size = re.search(rf"\bsize:\s*\({NUM},\s*{NUM}\)", text)
    color = re.search(rf"\bcolor:\s*\({NUM},\s*{NUM},\s*{NUM}\)", text)
    if size is None or color is None:
        raise ValueError("enemy size/color not found")
    return (
        (float(size[1]), float(size[2])),
        srgb_float(float(color[1]), float(color[2]), float(color[3])),
    )


def parse_bolt_radius(text: str) -> float:
    """Radius of the first Projectile effect (Ember Bolt) in the spell file."""
    match = re.search(rf"Projectile\(\s*speed:\s*{NUM},\s*radius:\s*{NUM}", strip_comments(text))
    if match is None:
        raise ValueError("projectile radius not found")
    return float(match[2])


def clamp_axis(target: float, room: float, view: float) -> float:
    """Same rule as src/world/camera.rs::clamp_axis."""
    if room <= view:
        return room / 2.0
    return min(max(target, view / 2.0), room - view / 2.0)


def floor_top(room: Room) -> float:
    """Top of the floor: the solid platform at y = 0 spanning the room width."""
    floors = [p for p in room.platforms
              if not p.one_way and p.min[1] == 0.0 and p.size[0] >= room.size[0] / 2]
    if not floors:
        raise ValueError("no floor platform found")
    return max(p.min[1] + p.size[1] for p in floors)


def render_world(
    room: Room,
    enemy_look: tuple[tuple[float, float], tuple[int, int, int]],
    view_min: tuple[float, float],
    view_size: tuple[int, int],
    bolts: tuple[tuple[float, float, float], ...] = (),
) -> Image.Image:
    """Draws the world region at 1 image pixel per world unit.

    ``bolts`` are projectiles as (centre x, centre y, radius) in world units.
    """
    image = Image.new("RGBA", view_size, (*room.background, 255))
    draw = ImageDraw.Draw(image)

    def rect(x: float, y: float, w: float, h: float, color: tuple[int, int, int]) -> None:
        left = x - view_min[0]
        top = view_size[1] - (y + h - view_min[1])
        draw.rectangle((round(left), round(top), round(left + w) - 1, round(top + h) - 1), fill=color)

    for p in room.platforms:
        rect(p.min[0], p.min[1], p.size[0], p.size[1], SCENE_ONE_WAY if p.one_way else SCENE_PLATFORM)
    (ew, eh), ecolor = enemy_look
    ground = floor_top(room)
    for _, x in room.enemies:
        rect(x - ew / 2, ground, ew, eh, ecolor)
    for bx, by, radius in bolts:
        rect(bx - radius, by - radius, 2 * radius, 2 * radius, SCENE_FIRE_BOLT)
    return image


def place_sprite(
    canvas: Image.Image,
    sprite: Image.Image,
    feet_screen: tuple[float, float],
    density: int,
) -> None:
    pivot = (PIVOT[0] * density, PIVOT[1] * density)
    canvas.alpha_composite(sprite, (round(feet_screen[0] - pivot[0]), round(feet_screen[1] - pivot[1])))


def caption(canvas: Image.Image, lines: list[str]) -> None:
    draw = ImageDraw.Draw(canvas)
    f = font(15)
    width = max(draw.textlength(line, font=f) for line in lines) + 20
    height = 22 * len(lines) + 12
    x0 = canvas.size[0] - width - 12
    y0 = 12
    draw.rectangle((x0, y0, x0 + width, y0 + height), fill=(8, 6, 12, 255), outline=(120, 100, 70))
    for i, line in enumerate(lines):
        draw.text((x0 + 10, y0 + 8 + 22 * i), line, fill=(230, 225, 235), font=f)


def frame(
    room: Room,
    enemy_look: tuple[tuple[float, float], tuple[int, int, int]],
    player_x: float,
    zoom: float,
    sprite: Image.Image,
    density: int,
    hud: Image.Image | None,
    label: str,
    bolt_radius: float | None = None,
) -> Image.Image:
    view = (round(WINDOW[0] / zoom), round(WINDOW[1] / zoom))
    ground = floor_top(room)
    center_y = ground + PLAYER_SIZE[1] / 2
    cam = (
        clamp_axis(player_x + CAMERA_LOOK_AHEAD, room.size[0], view[0]),
        clamp_axis(center_y, room.size[1], view[1]),
    )
    view_min = (cam[0] - view[0] / 2, cam[1] - view[1] / 2)
    bolts: tuple[tuple[float, float, float], ...] = ()
    if bolt_radius is not None:
        bolts = ((player_x + bolt_radius + CAST_GAP + PROJECTILE_TRAVEL, center_y + CAST_HEIGHT, bolt_radius),)
    world = render_world(room, enemy_look, view_min, view, bolts)
    if zoom != int(zoom) or (density > 1 and zoom != density):
        # Generic path (V1.2-B): fractional zoom, or a sprite whose density
        # differs from the zoom; the sprite is resampled to zoom / density.
        screen = resize(world, WINDOW, Image.Resampling.NEAREST)
        scale = zoom / density
        size = (round(sprite.size[0] * scale), round(sprite.size[1] * scale))
        smooth = Image.Resampling.LANCZOS if density > 1 else Image.Resampling.NEAREST
        scaled = resize(sprite, size, smooth)
        feet = ((player_x - view_min[0]) * zoom, WINDOW[1] - (ground - view_min[1]) * zoom)
        pivot = (PIVOT[0] * density * scale, PIVOT[1] * density * scale)
        screen.alpha_composite(scaled, (round(feet[0] - pivot[0]), round(feet[1] - pivot[1])))
    elif density == 1 and zoom > 1:
        # A 1x sprite in world space is magnified with the scene (nearest).
        place_sprite(world, sprite, (player_x - view_min[0], view[1] - (ground - view_min[1])), 1)
        screen = resize(world, WINDOW, Image.Resampling.NEAREST)
    else:
        screen = resize(world, WINDOW, Image.Resampling.NEAREST) if zoom > 1 else world
        feet = ((player_x - view_min[0]) * zoom, WINDOW[1] - (ground - view_min[1]) * zoom)
        place_sprite(screen, sprite, feet, density)
    if hud is not None:
        screen.alpha_composite(hud, (0, 0))
    visible = 100.0 * view[0] * view[1] / (room.size[0] * room.size[1])
    caption(
        screen,
        [
            label,
            f"Área visível: {view[0]} x {view[1]} u ({visible:.0f}% da sala {room.size[0]:.0f} x {room.size[1]:.0f})",
            f"Personagem na tela: ~{FIGURE_HEIGHT_PX * zoom:.0f} px de altura",
            *([f"Ember Bolt: {2 * bolt_radius:.0f} u = {2 * bolt_radius * zoom:.0f} px, a {PROJECTILE_TRAVEL:.0f} u do lançamento"]
              if bolt_radius is not None else []),
            "Composição experimental a partir de entry_hall.ron",
        ],
    )
    return screen


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--out-dir", type=Path, default=OUT_DIR)
    parser.add_argument("--name", default=DEFAULT_SPRITE_NAME, help="sprite set produced by pixelize.py")
    parser.add_argument("--zooms", help="comma-separated zooms, e.g. 1,1.5,2: both finishes per zoom")
    parser.add_argument("--projectile", action="store_true", help="draw an Ember Bolt in flight")
    args = parser.parse_args()

    room = parse_room(ROOM_FILE.read_text(encoding="utf-8"))
    enemy_look = parse_enemy_look(ENEMY_FILE.read_text(encoding="utf-8"))
    pixel = Image.open(RENDERS / f"{args.name}_A_pixel_rim.png").convert("RGBA")
    hd = Image.open(RENDERS / f"{args.name}_B_hd_rim.png").convert("RGBA")
    hud = None
    if SCREENSHOT.is_file():
        shot = Image.open(SCREENSHOT).convert("RGBA")
        if shot.size == WINDOW:
            hud = shot.crop(HUD_BOX)
        else:
            print(f"[camera_compare] warning: {SCREENSHOT} is {shot.size}, expected {WINDOW}; HUD skipped")

    scenarios = {
        "spawn": (160.0, "início da sala"),
        "combat": (780.0, "perto do primeiro inimigo"),
    }
    bolt = parse_bolt_radius(SPELL_FILE.read_text(encoding="utf-8")) if args.projectile else None
    args.out_dir.mkdir(parents=True, exist_ok=True)
    if args.zooms:
        zooms = [float(z) for z in args.zooms.split(",")]
        for key, (x, where) in scenarios.items():
            for zoom in zooms:
                tag = f"{zoom:g}".replace(".", "_") + "x"
                name = "1:1 (atual)" if zoom == 1 else f"Zoom x{zoom:g} (estudo)"
                for finish, sprite, density in (("pixel", pixel, 1), ("hd", hd, HD_DENSITY)):
                    title = "pixel art" if finish == "pixel" else "alta resolução"
                    image = frame(room, enemy_look, x, zoom, sprite, density, hud,
                                  f"Câmera {name}, {title}, {where}", bolt)
                    path = args.out_dir / f"camera_{key}_{tag}_{finish}.png"
                    image.convert("RGB").save(path)
                    print(f"[camera_compare] {path}")
        return
    for key, (x, where) in scenarios.items():
        shots = {
            "1x_pixel": frame(room, enemy_look, x, 1, pixel, 1, hud, f"Câmera 1:1 (atual), pixel art, {where}"),
            "2x_pixel": frame(room, enemy_look, x, 2, pixel, 1, hud, f"Zoom x2 (estudo), pixel art, {where}"),
            "2x_hd": frame(room, enemy_look, x, 2, hd, HD_DENSITY, hud, f"Zoom x2 (estudo), alta resolução, {where}"),
        }
        for name, image in shots.items():
            path = args.out_dir / f"camera_{key}_{name}.png"
            image.convert("RGB").save(path)
            print(f"[camera_compare] {path}")


if __name__ == "__main__":
    main()
