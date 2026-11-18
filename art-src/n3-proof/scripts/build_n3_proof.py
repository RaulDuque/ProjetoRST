#!/usr/bin/env python3
"""Isolated N3 visual proof.

Authors a gothic pixel figure (two armors, sword, spear, cape) and the
manifest/socket files the existing parsers already accept. Does not open
``white_mesh.glb``, does not write ``assets/``, and does not present a
downscale filter as hand-made pixel art.

Run from the project root:

    python3 art-src/n3-proof/scripts/build_n3_proof.py
"""

from __future__ import annotations

import json
import math
import shutil
import time
from dataclasses import dataclass
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / "art-src" / "n3-proof" / "generated"
EVIDENCE = ROOT / "docs" / "development" / "n3" / "evidence"
GLB = ROOT / "Md-usario" / "assets-user" / "prota" / "base" / "white_mesh.glb"

# Design units are scaled so the standing figure lands in the 80–110 texel band.
SCALE = 1.62
CELL_W, CELL_H = 96, 128
PIVOT = (48, 120)  # sole, texels from the cell's top-left
CAPE_W, CAPE_H = 88, 100
CAPE_PIVOT = (44, 14)
SWORD_GRIP = (12, 12)
SPEAR_GRIP = (26, 9)
COLS, ROWS = 6, 4
GOLD = (232, 176, 64, 255)

INK = (18, 12, 20, 255)
SKIN = (214, 164, 132, 255)
SKIN_DK = (156, 104, 82, 255)
HAIR = (32, 22, 36, 255)
CLOTH = (110, 32, 52, 255)
CLOTH_DK = (62, 18, 34, 255)
CLOTH_HI = (168, 64, 78, 255)
LEATHER = (104, 64, 38, 255)
BOOT = (36, 24, 28, 255)
METAL = (168, 176, 190, 255)
METAL_DK = (78, 86, 102, 255)
METAL_HI = (214, 220, 230, 255)
CAPE = (58, 24, 78, 255)
CAPE_HI = (104, 52, 124, 255)
CAPE_DK = (28, 12, 40, 255)
WOOD = (96, 62, 36, 255)
SPEAR_TIP = (196, 206, 214, 255)

CLIPS = (
    ("idle", 4, "Loop", 6.0),
    ("walk", 6, "Loop", 10.0),
    ("jump", 6, "Once", 12.0),
    ("attack", 6, "Once", 14.0),
)


@dataclass
class Pose:
    bob: float
    lean: float
    breath: float
    thigh_l: float
    thigh_r: float
    arm_r: float
    arm_l: float
    cape: float


def pose_for(clip: str, index: int) -> Pose:
    """Authored poses. Angles are degrees from straight down, forward positive."""
    if clip == "idle":
        breath = (0.0, 1.2, 2.0, 1.0)[index]
        return Pose(0, 0, breath, 5, -4, 14, -8, (-8, -2, 6, 0)[index])
    if clip == "walk":
        table = (
            (0, 1, 0, 30, -26, -24, 20, -10),
            (2, 1, 0, 14, -10, -10, 8, -4),
            (1, 0, 0, -6, 8, 6, -8, 2),
            (0, -1, 0, -28, 30, 24, -20, 8),
            (2, -1, 0, -10, 12, 8, -6, 4),
            (1, 0, 0, 8, -6, -8, 6, -2),
        )
        bob, lean, breath, tl, tr, ar, al, cape = table[index]
        return Pose(bob, lean, breath, tl, tr, ar, al, cape)
    if clip == "jump":
        # Cape swing stays negative: behind a right-facing body, not across the chest.
        table = (
            (-5, 2, 0, 24, -22, -40, -18, -8),
            (4, -2, 0, -4, -2, -155, -145, -18),
            (12, -1, 0, 6, -8, -135, -125, -16),
            (16, 0, 0, 36, 28, -110, -100, -14),
            (8, 2, 0, 12, -14, -28, -16, -12),
            (-4, 3, 0, 26, -20, 8, -6, -8),
        )
        bob, lean, breath, tl, tr, ar, al, cape = table[index]
        return Pose(bob, lean, breath, tl, tr, ar, al, cape)
    table = (
        (0, -2, 0, 6, -5, -35, 10, -6),
        (1, -4, 0, 8, -4, -75, 14, -12),
        (0, 4, 0, 4, -6, 78, -16, 10),
        (-1, 3, 0, 4, -6, 108, -8, 16),
        (0, 1, 0, 5, -4, 36, 0, 6),
        (0, 0, 0, 5, -4, 16, -4, 0),
    )
    bob, lean, breath, tl, tr, ar, al, cape = table[index]
    return Pose(bob, lean, breath, tl, tr, ar, al, cape)


def _dir(angle: float) -> tuple[float, float]:
    radians = math.radians(angle)
    return math.sin(radians), -math.cos(radians)


def _limb(origin: tuple[float, float], angle: float, length: float) -> tuple[float, float]:
    dx, dy = _dir(angle)
    return origin[0] + dx * length, origin[1] + dy * length


def skeleton(pose: Pose) -> dict:
    hip_y = 24 + pose.bob + pose.breath * 0.25
    chest_y = 38 + pose.bob + pose.breath
    head_y = 50 + pose.bob + pose.breath
    hip = (pose.lean * 0.4, hip_y)
    chest = (pose.lean, chest_y)
    head = (pose.lean + 0.4, head_y)
    shoulder_r = (chest[0] + 5.5, chest_y + 1)
    shoulder_l = (chest[0] - 5.5, chest_y + 1)
    knee_l = _limb(hip, pose.thigh_l, 12)
    foot_l = _limb(knee_l, pose.thigh_l * 0.35, 12)
    knee_r = _limb(hip, pose.thigh_r, 12)
    foot_r = _limb(knee_r, pose.thigh_r * 0.35, 12)
    # Keep a planted foot on the sole line when the pose is a stance.
    lowest = min(foot_l[1], foot_r[1])
    if lowest < 2:
        lift = 2 - lowest
        points = [hip, chest, head, shoulder_r, shoulder_l, knee_l, foot_l, knee_r, foot_r]
        hip, chest, head, shoulder_r, shoulder_l, knee_l, foot_l, knee_r, foot_r = [
            (p[0], p[1] + lift) for p in points
        ]
    elbow_r = _limb(shoulder_r, pose.arm_r, 11)
    hand_r = _limb(elbow_r, pose.arm_r, 9)
    elbow_l = _limb(shoulder_l, pose.arm_l, 10)
    hand_l = _limb(elbow_l, pose.arm_l, 8)
    dx, dy = _dir(pose.arm_r)
    angle = math.degrees(math.atan2(dy, dx))
    points = {
        "hip": hip,
        "chest": chest,
        "head": head,
        "shoulder_r": shoulder_r,
        "shoulder_l": shoulder_l,
        "knee_l": knee_l,
        "foot_l": foot_l,
        "knee_r": knee_r,
        "foot_r": foot_r,
        "elbow_r": elbow_r,
        "hand_r": hand_r,
        "elbow_l": elbow_l,
        "hand_l": hand_l,
    }
    scaled = {name: (point[0] * SCALE, point[1] * SCALE) for name, point in points.items()}
    scaled["angle"] = angle
    scaled["front"] = pose.arm_r > 15
    scaled["cape"] = pose.cape * SCALE
    scaled["breath"] = pose.breath
    return scaled


class Buf:
    def __init__(self, w: int, h: int) -> None:
        self.w = w
        self.h = h
        self.px = bytearray(w * h * 4)

    def put(self, x: int, y: int, color: tuple[int, int, int, int]) -> None:
        if 0 <= x < self.w and 0 <= y < self.h:
            i = (y * self.w + x) * 4
            self.px[i : i + 4] = bytes(color)

    def opaque(self, x: int, y: int) -> bool:
        if not (0 <= x < self.w and 0 <= y < self.h):
            return False
        return self.px[(y * self.w + x) * 4 + 3] > 0

    def disc(self, x: float, y: float, radius: float, color: tuple[int, int, int, int]) -> None:
        r = int(math.ceil(radius))
        cx, cy = int(round(x)), int(round(y))
        for py in range(cy - r - 1, cy + r + 2):
            for px in range(cx - r - 1, cx + r + 2):
                if (px - x) ** 2 + (py - y) ** 2 <= radius * radius:
                    self.put(px, py, color)

    def bone(
        self,
        a: tuple[float, float],
        b: tuple[float, float],
        radius: float,
        color: tuple[int, int, int, int],
    ) -> None:
        steps = max(int(math.hypot(b[0] - a[0], b[1] - a[1]) * 2), 1)
        for step in range(steps + 1):
            t = step / steps
            self.disc(a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, radius, color)

    def outline(self) -> None:
        inked: list[tuple[int, int]] = []
        for y in range(self.h):
            for x in range(self.w):
                if self.opaque(x, y):
                    continue
                if any(self.opaque(x + dx, y + dy) for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1))):
                    inked.append((x, y))
        for x, y in inked:
            self.put(x, y, INK)

    def image(self) -> Image.Image:
        return Image.frombytes("RGBA", (self.w, self.h), bytes(self.px))


def to_px(point: tuple[float, float]) -> tuple[float, float]:
    return PIVOT[0] + point[0], PIVOT[1] - point[1]


def draw_body(skel: dict, heavy: bool) -> Image.Image:
    buf = Buf(CELL_W, CELL_H)
    hip, chest, head = to_px(skel["hip"]), to_px(skel["chest"]), to_px(skel["head"])
    sh_r, sh_l = to_px(skel["shoulder_r"]), to_px(skel["shoulder_l"])
    knee_l, foot_l = to_px(skel["knee_l"]), to_px(skel["foot_l"])
    knee_r, foot_r = to_px(skel["knee_r"]), to_px(skel["foot_r"])
    elbow_r, hand_r = to_px(skel["elbow_r"]), to_px(skel["hand_r"])
    elbow_l, hand_l = to_px(skel["elbow_l"]), to_px(skel["hand_l"])

    cloth = METAL if heavy else CLOTH
    cloth_dk = METAL_DK if heavy else CLOTH_DK
    arm = METAL if heavy else SKIN
    arm_dk = METAL_DK if heavy else SKIN_DK
    torso_r = (6.2 if heavy else 4.4) * SCALE

    for knee, foot in ((knee_l, foot_l), (knee_r, foot_r)):
        buf.bone(hip, knee, 2.3 * SCALE, cloth_dk)
        buf.bone(knee, foot, 2.0 * SCALE, LEATHER)
        buf.disc(foot[0], foot[1], 2.6 * SCALE, BOOT)
    buf.bone(hip, chest, torso_r, cloth)
    buf.bone(chest, head, 2.2 * SCALE, SKIN if not heavy else METAL)
    # Tabard or plate band, so the two armors are not a recolor of one shape.
    if heavy:
        buf.disc(sh_r[0] + SCALE, sh_r[1], 4.2 * SCALE, METAL)
        buf.disc(sh_l[0] - SCALE, sh_l[1], 4.2 * SCALE, METAL)
        buf.bone(chest, (chest[0], chest[1] + 6 * SCALE), 2.0 * SCALE, METAL_HI)
    else:
        hem = (hip[0], hip[1] + 7 * SCALE)
        buf.bone(chest, hem, 3.2 * SCALE, CLOTH)
        buf.disc(hem[0] - 3 * SCALE, hem[1], 2.2 * SCALE, CLOTH_DK)
        buf.disc(hem[0] + 3 * SCALE, hem[1], 2.2 * SCALE, CLOTH_HI)
    buf.bone(sh_r, elbow_r, 2.1 * SCALE, arm)
    buf.bone(elbow_r, hand_r, 1.7 * SCALE, arm_dk)
    buf.bone(sh_l, elbow_l, 2.0 * SCALE, arm)
    buf.bone(elbow_l, hand_l, 1.6 * SCALE, arm_dk)
    buf.disc(hand_r[0], hand_r[1], 1.8 * SCALE, SKIN)
    buf.disc(hand_l[0], hand_l[1], 1.6 * SCALE, SKIN)
    buf.disc(head[0], head[1], 4.6 * SCALE, SKIN if not heavy else METAL)
    if heavy:
        crest = (head[0], head[1] - 7 * SCALE)
        buf.bone(head, crest, 1.5 * SCALE, METAL_DK)
        buf.disc(crest[0], crest[1], 1.6 * SCALE, CLOTH_HI)
        buf.put(int(head[0]) - 2, int(head[1]), INK)
        buf.put(int(head[0]) - 1, int(head[1]), INK)
        buf.put(int(head[0]), int(head[1]), INK)
        buf.put(int(head[0]) + 1, int(head[1]), INK)
    else:
        buf.disc(head[0] - SCALE, head[1] - 3 * SCALE, 3.4 * SCALE, HAIR)
        buf.put(int(head[0]) - 1, int(head[1]) - 1, INK)
        buf.put(int(head[0]) + 1, int(head[1]) - 1, INK)
    buf.outline()
    return buf.image()


def draw_cape(skel: dict) -> Image.Image:
    """Collar stays on the pivot. The cloth is the only thing that swings."""
    buf = Buf(CAPE_W, CAPE_H)
    swing = skel["cape"]
    collar = (float(CAPE_PIVOT[0]), float(CAPE_PIVOT[1]))
    mid = (collar[0] + swing * 0.45, collar[1] + 18 * SCALE)
    hem = (collar[0] + swing, collar[1] + 36 * SCALE)
    buf.bone(collar, mid, 5.0 * SCALE, CAPE)
    buf.bone(mid, hem, 6.0 * SCALE, CAPE_DK)
    buf.disc(hem[0] - 4 * SCALE, hem[1], 2.4 * SCALE, CAPE)
    buf.disc(hem[0] + 3 * SCALE, hem[1] + 2 * SCALE, 2.0 * SCALE, CAPE_HI)
    buf.disc(collar[0], collar[1], 2.2 * SCALE, CAPE_HI)
    buf.outline()
    return buf.image()


def draw_sword() -> Image.Image:
    buf = Buf(64, 24)
    for x in range(10, 58):
        buf.put(x, 11, METAL_HI)
        buf.put(x, 12, METAL)
    for y in range(6, 18):
        buf.put(SWORD_GRIP[0], y, GOLD)
        buf.put(SWORD_GRIP[0] + 1, y, GOLD)
    buf.disc(8, 12, 3.2, LEATHER)
    buf.disc(58, 11, 2.0, METAL)
    buf.outline()
    return buf.image()


def draw_spear() -> Image.Image:
    buf = Buf(88, 18)
    for x in range(6, 72):
        buf.put(x, 8, WOOD)
        buf.put(x, 9, LEATHER)
    for x in range(72, 84):
        buf.put(x, 8, SPEAR_TIP)
        buf.put(x, 9, METAL_HI)
    buf.disc(84, 8, 2.0, METAL)
    for y in range(6, 12):
        buf.put(SPEAR_GRIP[0], y, GOLD)
    buf.outline()
    return buf.image()


def pack(frames: list[Image.Image]) -> Image.Image:
    sheet = Image.new("RGBA", (COLS * CELL_W, ROWS * CELL_H), (0, 0, 0, 0))
    for index, frame in enumerate(frames):
        col, row = index % COLS, index // COLS
        sheet.paste(frame, (col * CELL_W, row * CELL_H))
    return sheet


def pack_cape(frames: list[Image.Image]) -> Image.Image:
    cell_w, cell_h = frames[0].size
    sheet = Image.new("RGBA", (COLS * cell_w, ROWS * cell_h), (0, 0, 0, 0))
    for index, frame in enumerate(frames):
        col, row = index % COLS, index // COLS
        sheet.paste(frame, (col * cell_w, row * cell_h))
    return sheet


def count_gold(image: Image.Image) -> int:
    raw = image.tobytes()
    target = bytes(GOLD)
    return sum(1 for i in range(0, len(raw), 4) if raw[i : i + 4] == target)


def opaque_height(image: Image.Image) -> int:
    pixels = image.load()
    top, bottom = image.height, -1
    for y in range(image.height):
        for x in range(image.width):
            if pixels[x, y][3]:
                top = min(top, y)
                bottom = max(bottom, y)
    return 0 if bottom < 0 else bottom - top + 1


def frame_index_map() -> list[tuple[str, int, int]]:
    cursor = 0
    mapped = []
    for name, count, _mode, _fps in CLIPS:
        for local in range(count):
            mapped.append((name, local, cursor))
            cursor += 1
    return mapped


def write_manifest() -> None:
    cursor = 0
    clips = []
    for name, count, mode, fps in CLIPS:
        durations = ", ".join(f"{1 / fps:.4f}" for _ in range(count))
        mode_ron = f"Loop(fps: {fps})" if mode == "Loop" else f"Once(fps: {fps})"
        fallback = "None" if name == "idle" else 'Some("idle")'
        speed = "Some(90.0)" if name == "walk" else "None"
        clips.append(
            f'        "{name}": (atlas: "body", first: {cursor}, count: {count}, '
            f"mode: {mode_ron}, pivot: ({PIVOT[0]}.0, {PIVOT[1]}.0), "
            f"fallback: {fallback}, durations: [{durations}], speed_reference: {speed}),"
        )
        cursor += count
    text = "\n".join(
        [
            "(",
            "    version: 1,",
            "    facing: Right,",
            "    weapon_in_body: false,",
            "    atlases: {",
            f'        "body": (texture: "body_light.png", cell: ({CELL_W}, {CELL_H}), columns: {COLS}, rows: {ROWS}, padding: 0, density: 1),',
            "    },",
            "    clips: {",
            *clips,
            "    },",
            ")",
            "",
        ]
    )
    (OUT / "proof.ron").write_text(text, encoding="utf-8")


def write_sockets(frames: list[dict]) -> None:
    def num(value: float) -> str:
        return f"{value:.2f}"

    blocks = []
    cursor = 0
    for name, count, _mode, _fps in CLIPS:
        entries = []
        for local in range(count):
            skel = frames[cursor]
            hand = skel["hand_r"]
            off = skel["hand_l"]
            head = skel["head"]
            chest = skel["chest"]
            back = (skel["chest"][0] - 2.0 * SCALE, skel["chest"][1] + 1.0 * SCALE)
            front = "true" if skel["front"] else "false"
            entries.append(
                "(weapon: (x: {wx}, y: {wy}, angle: {ang}, front: {front}), "
                "hand_l: (x: {lx}, y: {ly}, front: false), "
                "head: (x: {hx}, y: {hy}), chest: (x: {cx}, y: {cy}), "
                "back: (x: {bx}, y: {by}))".format(
                    wx=num(hand[0]),
                    wy=num(hand[1]),
                    ang=num(skel["angle"]),
                    front=front,
                    lx=num(off[0]),
                    ly=num(off[1]),
                    hx=num(head[0]),
                    hy=num(head[1]),
                    cx=num(chest[0]),
                    cy=num(chest[1]),
                    bx=num(back[0]),
                    by=num(back[1]),
                )
            )
            cursor += 1
        blocks.append(f'        "{name}": [\n            ' + ",\n            ".join(entries) + ",\n        ],")
    text = "(\n    version: 1,\n    clips: {\n" + "\n".join(blocks) + "\n    },\n)\n"
    (OUT / "sockets.ron").write_text(text, encoding="utf-8")


def write_loadout() -> None:
    (OUT / "loadout.ron").write_text(
        "\n".join(
            [
                "(",
                '    body_light: "body_light.png",',
                '    body_heavy: "body_heavy.png",',
                '    cape: "cape.png",',
                '    sword: "sword.png",',
                '    spear: "spear.png",',
                f"    cell: ({CELL_W}, {CELL_H}),",
                f"    columns: {COLS},",
                f"    rows: {ROWS},",
                f"    pivot: ({PIVOT[0]}.0, {PIVOT[1]}.0),",
                f"    cape_cell: ({CAPE_W}, {CAPE_H}),",
                f"    cape_pivot: ({CAPE_PIVOT[0]}.0, {CAPE_PIVOT[1]}.0),",
                f"    sword_grip: ({SWORD_GRIP[0]}.0, {SWORD_GRIP[1]}.0),",
                f"    spear_grip: ({SPEAR_GRIP[0]}.0, {SPEAR_GRIP[1]}.0),",
                "    sword_size: (64, 24),",
                "    spear_size: (88, 18),",
                "    clips: [",
                '        (name: "idle", count: 4),',
                '        (name: "walk", count: 6),',
                '        (name: "jump", count: 6),',
                '        (name: "attack", count: 6),',
                "    ],",
                ")",
                "",
            ]
        ),
        encoding="utf-8",
    )


def blit_weapon(
    canvas: Image.Image,
    weapon: Image.Image,
    grip: tuple[int, int],
    dest: tuple[int, int],
    angle: float,
) -> None:
    pad = int(math.hypot(*weapon.size)) + 2
    big = Image.new("RGBA", (weapon.width + pad * 2, weapon.height + pad * 2), (0, 0, 0, 0))
    big.paste(weapon, (pad, pad), weapon)
    center = (grip[0] + pad, grip[1] + pad)
    rotated = big.rotate(-angle, resample=Image.Resampling.NEAREST, center=center)
    canvas.alpha_composite(rotated, (dest[0] - center[0], dest[1] - center[1]))


def composite(
    body: Image.Image,
    cape: Image.Image,
    weapon: Image.Image,
    skel: dict,
    grip: tuple[int, int],
    cape_on: bool,
) -> Image.Image:
    """Pad the cell so a blade that leaves the body atlas stays visible."""
    pad_x, pad_y = 72, 24
    canvas = Image.new("RGBA", (CELL_W + pad_x * 2, CELL_H + pad_y), (0, 0, 0, 0))
    origin = (pad_x, pad_y)
    if cape_on:
        back_x = skel["chest"][0] - 2.0 * SCALE
        back_y = skel["chest"][1] + 1.0 * SCALE
        collar = (origin[0] + int(round(PIVOT[0] + back_x)), origin[1] + int(round(PIVOT[1] - back_y)))
        canvas.alpha_composite(cape, (collar[0] - CAPE_PIVOT[0], collar[1] - CAPE_PIVOT[1]))
    canvas.alpha_composite(body, origin)
    hand = skel["hand_r"]
    dest = (origin[0] + int(round(PIVOT[0] + hand[0])), origin[1] + int(round(PIVOT[1] - hand[1])))
    if skel["front"]:
        blit_weapon(canvas, weapon, grip, dest, skel["angle"])
    else:
        under = Image.new("RGBA", canvas.size, (0, 0, 0, 0))
        blit_weapon(under, weapon, grip, dest, skel["angle"])
        under.alpha_composite(canvas)
        canvas = under
    return canvas


def caption(draw: ImageDraw.ImageDraw, text: str, xy: tuple[int, int]) -> None:
    draw.text(xy, text, fill=(232, 224, 210, 255))


def save_sheets(
    light: list[Image.Image],
    heavy: list[Image.Image],
    capes: list[Image.Image],
    sword: Image.Image,
    spear: Image.Image,
    skels: list[dict],
) -> dict:
    EVIDENCE.mkdir(parents=True, exist_ok=True)
    by_name = {name: [] for name, *_ in CLIPS}
    for (name, _local, index), skel in zip(frame_index_map(), skels):
        by_name[name].append(index)

    strike = by_name["attack"][2]
    sample = composite(light[strike], capes[strike], sword, skels[strike], SWORD_GRIP, True)
    frame_w, frame_h = sample.size

    walk = Image.new("RGBA", (6 * frame_w, frame_h), (24, 18, 28, 255))
    for slot, index in enumerate(by_name["walk"]):
        frame = composite(light[index], capes[index], sword, skels[index], SWORD_GRIP, True)
        walk.alpha_composite(frame, (slot * frame_w, 0))
    walk.save(EVIDENCE / "walk_strip.png")

    panel = Image.new("RGBA", (2 * frame_w + 16, 2 * frame_h + 16), (24, 18, 28, 255))
    combos = (
        (0, 0, light[strike], sword, SWORD_GRIP),
        (frame_w + 16, 0, light[strike], spear, SPEAR_GRIP),
        (0, frame_h + 16, heavy[strike], sword, SWORD_GRIP),
        (frame_w + 16, frame_h + 16, heavy[strike], spear, SPEAR_GRIP),
    )
    for x, y, body, weapon, grip in combos:
        panel.alpha_composite(
            composite(body, capes[strike], weapon, skels[strike], grip, True),
            (x, y),
        )
    panel.save(EVIDENCE / "combinations.png")

    cape_off = composite(light[strike], capes[strike], sword, skels[strike], SWORD_GRIP, False)
    cape_on = composite(light[strike], capes[strike], sword, skels[strike], SWORD_GRIP, True)
    pair = Image.new("RGBA", (frame_w * 2 + 8, frame_h), (24, 18, 28, 255))
    pair.alpha_composite(cape_on, (0, 0))
    pair.alpha_composite(cape_off, (frame_w + 8, 0))
    pair.save(EVIDENCE / "cape_on_off.png")

    # Same pose, two pictures. The left one is an automatic filter and is
    # labeled as such. It is not a Pixelorama finish.
    soft = Image.new("RGBA", (CELL_W * 4, CELL_H * 4), (0, 0, 0, 0))
    draw = ImageDraw.Draw(soft)
    scale = 4
    skel = skels[strike]

    def blob(point, radius, color):
        x, y = to_px(point)
        draw.ellipse(
            (
                (x - radius) * scale,
                (y - radius) * scale,
                (x + radius) * scale,
                (y + radius) * scale,
            ),
            fill=color,
        )

    blob(skel["hip"], 5 * SCALE, (90, 40, 50, 255))
    blob(skel["chest"], 6 * SCALE, (120, 50, 60, 255))
    blob(skel["head"], 5 * SCALE, (200, 160, 130, 255))
    blob(skel["hand_r"], 2 * SCALE, (200, 160, 130, 255))
    filtered = soft.resize((CELL_W, CELL_H), Image.Resampling.LANCZOS)
    authored = light[strike]
    compare = Image.new("RGBA", (CELL_W * 2 + 24, CELL_H + 28), (16, 12, 18, 255))
    compare.alpha_composite(filtered, (8, 22))
    compare.alpha_composite(authored, (CELL_W + 16, 22))
    label = ImageDraw.Draw(compare)
    caption(label, "filtro", (8, 4))
    caption(label, "pixels", (CELL_W + 16, 4))
    compare.save(EVIDENCE / "pipeline_compare.png")

    idle = light[by_name["idle"][0]]
    height = opaque_height(idle)
    return {"idle_opaque_texels_tall": height, "strike_frame": strike}


def main() -> None:
    started = time.perf_counter()
    OUT.mkdir(parents=True, exist_ok=True)
    skels = []
    light_frames = []
    heavy_frames = []
    cape_frames = []
    for name, count, _mode, _fps in CLIPS:
        for index in range(count):
            skel = skeleton(pose_for(name, index))
            skels.append(skel)
            light_frames.append(draw_body(skel, False))
            heavy_frames.append(draw_body(skel, True))
            cape_frames.append(draw_cape(skel))
    if len(light_frames) > COLS * ROWS:
        raise SystemExit(f"atlas holds {COLS * ROWS} cells, got {len(light_frames)} frames")

    body_light = pack(light_frames)
    body_heavy = pack(heavy_frames)
    cape = pack_cape(cape_frames)
    sword = draw_sword()
    spear = draw_spear()
    body_light.save(OUT / "body_light.png")
    body_heavy.save(OUT / "body_heavy.png")
    cape.save(OUT / "cape.png")
    sword.save(OUT / "sword.png")
    spear.save(OUT / "spear.png")
    write_manifest()
    write_sockets(skels)
    write_loadout()

    stats = save_sheets(light_frames, heavy_frames, cape_frames, sword, spear, skels)
    glb = GLB.stat()
    metrics = {
        "generator_seconds": round(time.perf_counter() - started, 4),
        "pixelorama_on_path": shutil.which("pixelorama") is not None,
        "blender_on_path": shutil.which("blender") is not None,
        "white_mesh_bytes": glb.st_size,
        "white_mesh_mtime_ns": glb.st_mtime_ns,
        "body_light_bytes": (OUT / "body_light.png").stat().st_size,
        "body_heavy_bytes": (OUT / "body_heavy.png").stat().st_size,
        "cape_bytes": (OUT / "cape.png").stat().st_size,
        "sword_bytes": (OUT / "sword.png").stat().st_size,
        "spear_bytes": (OUT / "spear.png").stat().st_size,
        "gold_in_body_light": count_gold(body_light),
        "gold_in_body_heavy": count_gold(body_heavy),
        "gold_in_sword": count_gold(sword),
        "gold_in_spear": count_gold(spear),
        "frames": len(light_frames),
        "cell": [CELL_W, CELL_H],
        "pivot": list(PIVOT),
        **stats,
        "note": "Authored pixels. The filter image is not a manual finish and not a Blender render.",
    }
    (OUT / "metrics.json").write_text(json.dumps(metrics, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(metrics, indent=2))


if __name__ == "__main__":
    main()
