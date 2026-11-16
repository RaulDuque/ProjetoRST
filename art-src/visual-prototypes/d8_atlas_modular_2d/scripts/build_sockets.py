#!/usr/bin/env python3
"""Marco B — declarative 2D sockets per idle/run frame (lab JSON).

Positions are in cell pixels, origin top-left; sole pivot at PIVOT.
Derived from assets/animations/player_sockets.ron (read-only) scaled into
the lab cell, plus anatomical estimates on D2 bbox.
"""
from __future__ import annotations

import re
from pathlib import Path

from PIL import Image, ImageDraw

from d8_common import (
    CELL,
    LAB_H,
    PIVOT,
    REPO,
    ROOT,
    board,
    canvas,
    feet_place,
    load_d2_frames,
    nearest_up,
    scale_to_height,
    write_json,
)

OUT = ROOT / "sockets"
# world units: height ~52; lab sprite ~LAB_H px → scale
WORLD_TO_PX = LAB_H / 52.0


def parse_ron_clip(text: str, clip: str) -> list[dict]:
    """Minimal extract of weapon/hand_l/head/chest/back for a clip."""
    # Find clip array roughly
    m = re.search(rf'"{clip}"\s*:\s*\[(.*?)\],\s*"', text, re.S)
    if not m:
        # last clip may not have trailing "
        m = re.search(rf'"{clip}"\s*:\s*\[(.*?)\]\s*\}}', text, re.S)
    if not m:
        return []
    body = m.group(1)
    frames = []
    for fm in re.finditer(
        r"weapon:\s*\(x:\s*([-0-9.]+),\s*y:\s*([-0-9.]+),\s*angle:\s*([-0-9.]+).*?"
        r"hand_l:\s*\(x:\s*([-0-9.]+),\s*y:\s*([-0-9.]+).*?"
        r"head:\s*\(x:\s*([-0-9.]+),\s*y:\s*([-0-9.]+)\).*?"
        r"chest:\s*\(x:\s*([-0-9.]+),\s*y:\s*([-0-9.]+)\).*?"
        r"back:\s*\(x:\s*([-0-9.]+),\s*y:\s*([-0-9.]+)\)",
        body,
        re.S,
    ):
        frames.append(
            {
                "weapon": (float(fm.group(1)), float(fm.group(2)), float(fm.group(3))),
                "hand_l": (float(fm.group(4)), float(fm.group(5))),
                "head": (float(fm.group(6)), float(fm.group(7))),
                "chest": (float(fm.group(8)), float(fm.group(9))),
                "back": (float(fm.group(10)), float(fm.group(11))),
            }
        )
    return frames


def world_to_cell(x: float, y: float) -> tuple[int, int]:
    # sole-relative: +x forward (right), +y up
    px = int(round(PIVOT[0] + x * WORLD_TO_PX * 0.85))
    py = int(round(PIVOT[1] - y * WORLD_TO_PX * 0.85))
    return px, py


def estimate_anatomy(cell: Image.Image, base: dict) -> dict[str, dict]:
    """Fill remaining sockets from bbox + known points."""
    box = cell.split()[-1].getbbox() or (40, 40, 90, 156)
    x0, y0, x1, y1 = box
    cx = (x0 + x1) // 2
    hx, hy = world_to_cell(*base["head"][:2]) if "head" in base else (cx, y0 + 8)
    chx, chy = world_to_cell(*base["chest"][:2]) if "chest" in base else (cx, y0 + int((y1 - y0) * 0.35))
    bx, by = world_to_cell(*base["back"][:2]) if "back" in base else (cx - 4, chy - 4)
    wx, wy, wang = base.get("weapon", (8.0, 28.0, -66.0))
    wpx, wpy = world_to_cell(wx, wy)
    hlx, hly = world_to_cell(*base["hand_l"][:2]) if "hand_l" in base else (cx - 6, chy + 10)

    def sock(name, x, y, attaches=None):
        return {
            "name": name,
            "x": int(x),
            "y": int(y),
            "space": "cell_px_topleft",
            "mirror": False,
            "attaches": attaches or [],
        }

    return {
        "head": sock("head", hx, hy, ["helm", "hat", "hair_front"]),
        "neck": sock("neck", hx, hy + 6, ["cape_back"]),
        "shoulder_back": sock("shoulder_back", bx - 6, chy - 2, ["pauldrons"]),
        "shoulder_front": sock("shoulder_front", chx + 8, chy - 2, ["pauldrons"]),
        "elbow_back": sock("elbow_back", bx - 4, chy + 12, ["bracers"]),
        "elbow_front": sock("elbow_front", chx + 10, chy + 14, ["bracers"]),
        "hand_back": sock("hand_back", hlx, hly, ["gloves", "shield"]),
        "hand_front": sock("hand_front", wpx, wpy, ["gloves", "weapon"]),
        "hip": sock("hip", cx, chy + 22, ["scabbard", "pants"]),
        "knee_back": sock("knee_back", cx - 4, y1 - 28, []),
        "knee_front": sock("knee_front", cx + 4, y1 - 26, []),
        "foot_back": sock("foot_back", cx - 5, y1 - 2, ["boots"]),
        "foot_front": sock("foot_front", cx + 5, y1 - 2, ["boots"]),
        "back": sock("back", bx, by, ["cape_back", "shield"]),
        "scabbard": sock("scabbard", cx - 8, chy + 18, ["scabbard"]),
        "body_center": sock("body_center", cx, chy, ["armor", "coat_front", "shirt"]),
        "weapon": {
            **sock("weapon", wpx, wpy, ["weapon"]),
            "angle_deg": wang,
        },
    }


def overlay_sockets(cell: Image.Image, socks: dict) -> Image.Image:
    im = cell.convert("RGBA").copy()
    d = ImageDraw.Draw(im)
    for name, s in socks.items():
        x, y = s["x"], s["y"]
        d.ellipse([x - 2, y - 2, x + 2, y + 2], outline=(230, 200, 80, 255))
        d.point((x, y), fill=(255, 240, 120, 255))
    return im


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    ron_path = REPO / "assets" / "animations" / "player_sockets.ron"
    text = ron_path.read_text(encoding="utf-8") if ron_path.exists() else ""

    data = {
        "version": 1,
        "cell": list(CELL),
        "pivot_sole": list(PIVOT),
        "coord_space": "cell_px_topleft_y_down",
        "note": "Lab sockets; does not modify game player_sockets.ron",
        "clips": {},
    }

    preview_items = []
    for clip, count in (("idle", 6), ("run", 8)):
        ron_frames = parse_ron_clip(text, clip)
        d2_frames = load_d2_frames(clip)
        clip_out = []
        for i, fr in enumerate(d2_frames):
            cell = feet_place(scale_to_height(fr, LAB_H))
            base = ron_frames[i] if i < len(ron_frames) else (
                ron_frames[0] if ron_frames else {
                    "weapon": (8.28, 27.83, -66.23),
                    "hand_l": (-0.11, 25.31),
                    "head": (0.78, 47.59),
                    "chest": (0.49, 38.77),
                    "back": (0.18, 41.43),
                }
            )
            socks = estimate_anatomy(cell, base)
            clip_out.append({"frame": i, "sockets": socks})
            if i == 0:
                vis = overlay_sockets(cell, socks)
                vis.save(OUT / f"{clip}_f00_overlay.png")
                preview_items.append((f"{clip} f0", nearest_up(vis, 2)))
        data["clips"][clip] = clip_out
        write_json(OUT / f"{clip}_sockets.json", {"clip": clip, "frames": clip_out})

    write_json(OUT / "sockets_lab.json", data)
    if preview_items:
        board(preview_items, ROOT / "comparacoes" / "04_sockets.png", "Sockets 2D (lab)")
    print(f"Sockets -> {OUT}")


if __name__ == "__main__":
    main()
