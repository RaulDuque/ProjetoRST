"""Native 100px study of the N3.2 hunter. Not a reduction and not a capsule.

Pixels are placed from these tables. Pixelorama was not used to paint.
The cape and the sword are separate images. The original blend is not opened.
"""

import json
import sys
import time
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "art-src/player/scripts/post"))
from resample import resize  # noqa: E402

OUT = ROOT / "art-src/n3-3-native-pixel/generated"
EVIDENCE = ROOT / "docs/development/n3_3/evidence"
FACE = ROOT / "art-src/n3-2-art-direction/references/b_face.png"

W, H = 80, 128
SOLE = 118
HEAD = (
    "..hhhhhhhhh...",
    ".hhhhhhhhhhh..",
    ".hhhhhhhhhhh..",
    "hhhhhhsssss...",
    "hhhhssssssn...",
    "hhhhsseessnn..",
    "hhhhssssssn...",
    "hhhhssssssc...",
    ".hhhssssss....",
    "..hhhhhh......",
    "...hhh........",
    "..............",
    "..............",
    "..............",
    "..............",
    "..............",
)
HEAD_X, HEAD_Y = 30, 14

INK = (18, 12, 16, 255)
HAIR = (28, 20, 24, 255)
HAIR_LIGHT = (78, 60, 50, 255)
SKIN = (214, 176, 150, 255)
SKIN_SHADOW = (150, 104, 88, 255)
EYE = (36, 28, 32, 255)
CLOTH = (32, 36, 52, 255)
CLOTH_SHADOW = (20, 22, 34, 255)
CLOTH_LIGHT = (62, 68, 90, 255)
LEATHER = (72, 44, 34, 255)
LEATHER_LIGHT = (112, 74, 48, 255)
STEEL = (132, 136, 146, 255)
STEEL_LIGHT = (214, 216, 222, 255)
GOLD = (176, 132, 48, 255)
CAPE = (120, 24, 42, 255)
CAPE_DARK = (72, 14, 28, 255)
BOOT = (24, 22, 28, 255)

GLYPH = {
    "h": HAIR,
    "s": SKIN,
    "n": SKIN,
    "e": EYE,
    "c": SKIN_SHADOW,
}


def blank() -> Image.Image:
    return Image.new("RGBA", (W, H), (0, 0, 0, 0))


def stamp(image: Image.Image, x: int, y: int, color: tuple[int, int, int, int]) -> None:
    if 0 <= x < W and 0 <= y < H and color[3]:
        image.putpixel((x, y), color)


def span(image: Image.Image, y: int, left: int, right: int, color: tuple[int, int, int, int]) -> None:
    if right < left:
        left, right = right, left
    for x in range(left, right + 1):
        stamp(image, x, y, color)


def head(image: Image.Image) -> None:
    for row, text in enumerate(HEAD):
        for col, glyph in enumerate(text):
            color = GLYPH.get(glyph)
            if color:
                stamp(image, HEAD_X + col, HEAD_Y + row, color)
    stamp(image, HEAD_X + 4, HEAD_Y + 2, HAIR_LIGHT)


def neck(image: Image.Image) -> None:
    for y in range(24, 32):
        span(image, y, 34, 40, SKIN_SHADOW if y < 28 else CLOTH)
    span(image, 28, 38, 44, CLOTH_LIGHT)
    span(image, 29, 36, 42, GOLD)


def coat(image: Image.Image, flare: int, pauldron: bool) -> None:
    neck(image)
    for y in range(32, 86):
        t = (y - 32) / 53
        left = round(28 - flare * t)
        right = round(46 - 4 * t)
        color = CLOTH_SHADOW if left <= 28 - flare * t else CLOTH
        span(image, y, left, right, CLOTH)
        span(image, y, left, left + 3, CLOTH_SHADOW)
        if y < 48:
            span(image, y, right - 4, right, CLOTH_LIGHT)
        color = color
    if pauldron:
        for y in range(30, 35):
            span(image, y, 44, 50 - (y - 30), STEEL)
        stamp(image, 48, 31, STEEL_LIGHT)


def arm(image: Image.Image, pose: str) -> tuple[int, int]:
    """Return the glove pixel that holds the sword, in canvas coordinates."""
    if pose == "idle":
        for y in range(34, 46):
            span(image, y, 46, 51, CLOTH)
        for y in range(46, 54):
            span(image, y, 48, 53, CLOTH_LIGHT)
        span(image, 53, 48, 54, LEATHER)
        for y in range(54, 60):
            span(image, y, 49, 54, LEATHER)
        stamp(image, 53, 55, LEATHER_LIGHT)
        return 52, 57
    if pose == "anticipation":
        for y in range(32, 40):
            span(image, y, 24, 30, CLOTH)
        for y in range(28, 34):
            span(image, y, 18, 26, CLOTH_LIGHT)
        span(image, 27, 16, 22, LEATHER)
        for y in range(24, 28):
            span(image, y, 14, 20, LEATHER)
        stamp(image, 16, 25, LEATHER_LIGHT)
        return 15, 26
    if pose == "impact":
        for x in range(46, 62):
            span(image, 40, x, x, CLOTH)
        for x in range(58, 68):
            span(image, 39, x, x, CLOTH_LIGHT)
        span(image, 38, 66, 72, LEATHER)
        for x in range(68, 74):
            span(image, 37, x, x, LEATHER)
        stamp(image, 71, 37, LEATHER_LIGHT)
        return 72, 37
    for y in range(42, 56):
        span(image, y, 50, 55, CLOTH)
    for y in range(54, 64):
        span(image, y, 52, 57, CLOTH_LIGHT)
    span(image, 63, 52, 58, LEATHER)
    for y in range(64, 70):
        span(image, y, 53, 58, LEATHER)
    stamp(image, 57, 65, LEATHER_LIGHT)
    return 56, 67


def legs(image: Image.Image) -> None:
    for y in range(84, 106):
        span(image, y, 30, 35, CLOTH_SHADOW)
    for y in range(104, 116):
        span(image, y, 28, 36, BOOT)
    span(image, 116, 30, 37, BOOT)
    span(image, 117, 32, 36, BOOT)
    span(image, 107, 28, 31, LEATHER)
    for y in range(86, 106):
        span(image, y, 42, 47, CLOTH)
    for y in range(104, 116):
        span(image, y, 42, 50, BOOT)
    span(image, 116, 44, 54, BOOT)
    span(image, 117, 46, 53, BOOT)
    span(image, 105, 48, 53, LEATHER_LIGHT)
    stamp(image, 51, 104, STEEL)


def cape_image(sway: int) -> Image.Image:
    image = blank()
    for y in range(32, 78):
        t = (y - 32) / 45
        left = round(16 + sway * t)
        right = round(27 + sway * t * 0.35)
        if y % 7 == 0:
            left += 1
        span(image, y, left, right, CAPE_DARK if t > 0.7 else CAPE)
    return image


def sword_image() -> Image.Image:
    image = Image.new("RGBA", (36, 10), (0, 0, 0, 0))
    for x in range(8, 34):
        image.putpixel((x, 4), STEEL)
        image.putpixel((x, 5), STEEL)
    image.putpixel((30, 4), STEEL_LIGHT)
    for y in range(2, 8):
        image.putpixel((8, y), GOLD)
        image.putpixel((9, y), GOLD)
    for y in range(4, 6):
        image.putpixel((4, y), LEATHER)
        image.putpixel((5, y), LEATHER)
        image.putpixel((6, y), LEATHER)
    return image


def silhouette(kind: str) -> Image.Image:
    image = blank()
    ink = (12, 10, 14, 255)
    head(image)
    flare = {"narrow": 4, "pauldrons": 7, "cape": 8}[kind]
    coat(image, flare, kind == "pauldrons")
    arm(image, "idle")
    legs(image)
    if kind == "cape":
        cape = cape_image(0)
        for y in range(H):
            for x in range(W):
                if cape.getpixel((x, y))[3]:
                    image.putpixel((x, y), ink)
    pixels = image.load()
    for y in range(H):
        for x in range(W):
            if pixels[x, y][3]:
                pixels[x, y] = ink
    return image


def body(pose: str) -> tuple[Image.Image, tuple[int, int]]:
    image = blank()
    head(image)
    coat(image, 8, True)
    hand = arm(image, pose)
    legs(image)
    return image, hand


def outline(image: Image.Image) -> Image.Image:
    """Draw the ink outside the shape so a one-pixel nose stays skin."""
    src = image.copy()
    pixels = src.load()
    out = image.copy()
    dest = out.load()
    for y in range(H):
        for x in range(W):
            if pixels[x, y][3]:
                continue
            for nx, ny in ((x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)):
                if 0 <= nx < W and 0 <= ny < H and pixels[nx, ny][3]:
                    dest[x, y] = INK
                    break
    return out


def opaque_box(image: Image.Image) -> tuple[int, int, int, int]:
    pixels = image.load()
    left, top, right, bottom = image.width, image.height, 0, 0
    for y in range(image.height):
        for x in range(image.width):
            if pixels[x, y][3] > 16:
                left = min(left, x)
                top = min(top, y)
                right = max(right, x)
                bottom = max(bottom, y)
    return left, top, right + 1, bottom + 1


def composite(body_image: Image.Image, cape: Image.Image, sword: Image.Image, hand: tuple[int, int], angle_note: str) -> Image.Image:
    sheet = Image.new("RGBA", (W, H), (0, 0, 0, 0))
    sheet.alpha_composite(cape)
    sheet.alpha_composite(body_image)
    # Grip of the sword image is pixel (6, 5). Place that pixel on the hand.
    sheet.alpha_composite(sword, (hand[0] - 6, hand[1] - 5))
    sheet.info["note"] = angle_note
    return sheet


def plate(image: Image.Image, color: tuple[int, int, int], scale: int = 1) -> Image.Image:
    shown = image
    if scale != 1:
        shown = resize(image, (image.width * scale, image.height * scale), Image.Resampling.NEAREST)
    board = Image.new("RGBA", shown.size, (*color, 255))
    board.alpha_composite(shown)
    return board


def labeled(title: str, image: Image.Image) -> Image.Image:
    board = Image.new("RGBA", (image.width, image.height + 16), (24, 22, 28, 255))
    board.alpha_composite(image, (0, 16))
    ImageDraw.Draw(board).text((4, 2), title, fill=(230, 226, 214, 255))
    return board


def fit_box(image: Image.Image, height: int) -> Image.Image:
    scale = height / image.height
    return resize(image.convert("RGBA"), (max(1, round(image.width * scale)), height), Image.Resampling.BOX)


def write_sheets(
    body_image: Image.Image,
    cape: Image.Image,
    sword: Image.Image,
    full: Image.Image,
    hand: tuple[int, int],
) -> None:
    native_head = body_image.crop((HEAD_X - 2, HEAD_Y - 2, HEAD_X + 20, HEAD_Y + 20))
    native_big = resize(native_head, (native_head.width * 8, native_head.height * 8), Image.Resampling.NEAREST)
    original = Image.open(FACE).convert("RGBA")
    original_small = fit_box(original, native_big.height)
    reduced = resize(original, (48, 48), Image.Resampling.BOX)
    reduced_big = resize(reduced, (48 * 4, 48 * 4), Image.Resampling.NEAREST)
    row = Image.new("RGBA", (original_small.width + reduced_big.width + native_big.width + 12, native_big.height), (24, 22, 28, 255))
    row.alpha_composite(original_small, (0, 0))
    row.alpha_composite(reduced_big, (original_small.width + 4, 0))
    row.alpha_composite(native_big, (original_small.width + reduced_big.width + 8, 0))
    labeled("original | reduzido | nativo", row).save(EVIDENCE / "face_comparison.png")

    crops = [
        ("tecido", full.crop((28, 40, 46, 70))),
        ("aco", full.crop((44, 28, 54, 40))),
        ("couro", full.crop((44, 100, 58, 118))),
        ("capa", cape.crop((14, 32, 32, 78))),
    ]
    material = Image.new("RGBA", (4 * 96, 120), (24, 22, 28, 255))
    draw = ImageDraw.Draw(material)
    for index, (name, crop) in enumerate(crops):
        big = resize(crop, (crop.width * 4, crop.height * 4), Image.Resampling.NEAREST)
        material.alpha_composite(big, (index * 96 + 8, 24))
        draw.text((index * 96 + 8, 4), name, fill=(230, 226, 214, 255))
    material.save(EVIDENCE / "material_comparison.png")

    colors = [
        SKIN, HAIR, CLOTH, CLOTH_LIGHT, LEATHER, STEEL, STEEL_LIGHT, GOLD, CAPE, INK,
    ]
    swatch = Image.new("RGBA", (len(colors) * 28, 36), (24, 22, 28, 255))
    draw = ImageDraw.Draw(swatch)
    for index, color in enumerate(colors):
        draw.rectangle((index * 28 + 2, 2, index * 28 + 24, 28), fill=color)
    swatch.save(EVIDENCE / "palette_comparison.png")

    others = [
        ("N3", ROOT / "art-src/n3-proof/generated/body_light.png"),
        ("N3.1", ROOT / "art-src/n3-proof/n3_1/attack_pixel.png"),
        ("N3.2", ROOT / "art-src/n3-2-art-direction/generated/concept_b_sprite.png"),
    ]
    panels = []
    for name, path in others:
        image = Image.open(path).convert("RGBA")
        if name == "N3":
            image = image.crop((4 * 96, 2 * 128, 5 * 96, 3 * 128))
        elif name == "N3.1":
            image = image.crop((0, 0, 256, 192))
        panels.append((name, fit_box(image, 120)))
    panels.append(("N3.3", resize(full, (full.width * 2, full.height * 2), Image.Resampling.NEAREST)))
    width = sum(image.width + 8 for _name, image in panels)
    compare = Image.new("RGBA", (width, 140), (24, 22, 28, 255))
    draw = ImageDraw.Draw(compare)
    x = 4
    for name, image in panels:
        compare.alpha_composite(image, (x, 18))
        draw.text((x, 2), name, fill=(230, 226, 214, 255))
        x += image.width + 8
    compare.save(EVIDENCE / "n3_n31_n32_n33.png")

    plate(full, (186, 186, 190)).save(EVIDENCE / "native_100px.png")
    plate(full, (186, 186, 190), 2).save(EVIDENCE / "nearest_2x.png")
    plate(full, (16, 12, 18), 4).save(EVIDENCE / "dark_background.png")

    layers = Image.new("RGBA", (W * 3, H + 16), (24, 22, 28, 255))
    draw = ImageDraw.Draw(layers)
    for index, (name, image) in enumerate((("corpo", body_image), ("capa", cape), ("junto", full))):
        layers.alpha_composite(image, (index * W, 16))
        draw.text((index * W + 4, 2), name, fill=(230, 226, 214, 255))
    layers.save(EVIDENCE / "cape_separated.png")
    weapon_sheet = Image.new("RGBA", (sword.width * 6 + 80, 40), (24, 22, 28, 255))
    weapon_sheet.alpha_composite(
        resize(sword, (sword.width * 6, sword.height * 6), Image.Resampling.NEAREST),
        (4, 4),
    )
    ImageDraw.Draw(weapon_sheet).text((sword.width * 6 + 8, 8), "espada fora do corpo", fill=(230, 226, 214, 255))
    weapon_sheet.save(EVIDENCE / "weapon_separated.png")

    debug = plate(full, (24, 22, 28), 4)
    draw = ImageDraw.Draw(debug)
    hx, hy = hand[0] * 4, hand[1] * 4
    draw.ellipse((hx - 3, hy - 3, hx + 3, hy + 3), outline=(232, 176, 64, 255))
    debug.save(EVIDENCE / "socket_debug.png")

    manifest = f"""(
    version: 1,
    facing: Right,
    weapon_in_body: false,
    atlases: {{
        "body": (texture: "body_idle.png", cell: ({W}, {H}), columns: 1, rows: 1, padding: 0, density: 1),
    }},
    clips: {{
        "idle": (atlas: "body", first: 0, count: 1, mode: Loop(fps: 1.0),
            pivot: (40.0, {SOLE}.0), fallback: None,
            durations: [1.0], speed_reference: None),
    }},
)
"""
    (OUT / "native.ron").write_text(manifest)
    weapon_x = hand[0] - 40
    weapon_y = SOLE - hand[1]
    sockets = f"""(
    version: 1,
    clips: {{
        "idle": [
            (weapon: (x: {weapon_x}.0, y: {weapon_y}.0, angle: 0.0, front: true), hand_l: (x: -8.0, y: 40.0, front: false), head: (x: 2.0, y: 100.0), chest: (x: 2.0, y: 70.0), back: (x: -16.0, y: 82.0)),
        ],
    }},
)
"""
    (OUT / "sockets.ron").write_text(sockets)


def main() -> None:
    started = time.perf_counter()
    OUT.mkdir(parents=True, exist_ok=True)
    EVIDENCE.mkdir(parents=True, exist_ok=True)
    kinds = ("narrow", "pauldrons", "cape")
    silhouettes = [silhouette(kind) for kind in kinds]
    cell_w = W
    study = Image.new("RGBA", (cell_w * 3, H * 2 + 8), (0, 0, 0, 0))
    for index, image in enumerate(silhouettes):
        study.alpha_composite(plate(image, (214, 210, 200)), (index * cell_w, 0))
        study.alpha_composite(plate(image, (16, 12, 18)), (index * cell_w, H + 8))
    study.save(EVIDENCE / "silhouette_study.png")

    idle, hand = body("idle")
    idle = outline(idle)
    cape = outline(cape_image(0))
    sword = sword_image()
    full = composite(idle, cape, sword, hand, "idle")
    full.save(OUT / "idle_composite.png")
    idle.save(OUT / "body_idle.png")
    cape.save(OUT / "cape_idle.png")
    sword.save(OUT / "sword.png")
    plate(full, (186, 186, 190), 4).save(EVIDENCE / "native_idle.png")
    write_sheets(idle, cape, sword, full, hand)

    left, top, right, bottom = opaque_box(full)
    metrics = {
        "opaque_width": right - left,
        "opaque_height": bottom - top,
        "canvas": [W, H],
        "sole_y": SOLE,
        "hand": list(hand),
        "pivot": [40, SOLE],
        "machine_seconds": round(time.perf_counter() - started, 4),
        "human_artist_hours": None,
        "pixelorama_paint": False,
        "reduced_from_illustration": False,
    }
    (OUT / "metrics.json").write_text(json.dumps(metrics, indent=2))
    print(json.dumps(metrics))


if __name__ == "__main__":
    main()
