#!/usr/bin/env python3
"""Turn the N3.1 Blender renders into labeled comparison sheets.

The pixel images are an automatic downsample plus an automatic outline.
They are not Pixelorama work and not a hand-drawn frame.

Run from the project root after the Blender renders:

    python3 art-src/n3-proof/scripts/build_n3_1_compare.py
"""

from __future__ import annotations

import json
import time
from pathlib import Path

import sys

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "art-src/player/scripts/post"))
from resample import resize  # noqa: E402
RAW_ATTACK = (
    ROOT
    / "art-src/player/renders/v2_2_weaponless/v1_3/masters/attack_basic"
)
MANIFEST = ROOT / "art-src/player/renders/v2_2_weaponless/v1_3/partial_manifest.json"
OUT = ROOT / "art-src/n3-proof/n3_1"
EVIDENCE = ROOT / "docs/development/n3_1/evidence"
CONTRACT = (128, 96)
PIVOT = (40, 95)
# Integer 2× of the contract cell. The figure lands near 100 texels.
SHOW = (256, 192)


def opaque_span(image: Image.Image) -> tuple[int, int, int]:
    pixels = image.load()
    top, bottom = image.height, -1
    count = 0
    for y in range(image.height):
        for x in range(image.width):
            if pixels[x, y][3] > 16:
                top = min(top, y)
                bottom = max(bottom, y)
                count += 1
    height = 0 if bottom < 0 else bottom - top + 1
    return height, top, count


def restore_alpha(color: Image.Image, alpha_source: Image.Image) -> Image.Image:
    out = color.convert("RGBA")
    out.putalpha(alpha_source.getchannel("A"))
    return out


def outline(image: Image.Image) -> Image.Image:
    src = image.copy()
    pixels = src.load()
    ink = (12, 8, 16, 255)
    drawn = []
    for y in range(src.height):
        for x in range(src.width):
            if pixels[x, y][3] > 16:
                continue
            if any(
                0 <= x + dx < src.width
                and 0 <= y + dy < src.height
                and pixels[x + dx, y + dy][3] > 16
                for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1))
            ):
                drawn.append((x, y))
    for x, y in drawn:
        pixels[x, y] = ink
    return src


def automatic_pixel(raw: Image.Image) -> tuple[Image.Image, Image.Image]:
    """BOX resolves the 8× supersample. The outline pass is also a script."""
    contract = raw.resize(CONTRACT, Image.Resampling.BOX)
    quantized = restore_alpha(contract.convert("RGB").quantize(colors=16, method=Image.Quantize.MEDIANCUT), contract)
    edged = outline(quantized)
    show = resize(edged, SHOW, Image.Resampling.NEAREST)
    return contract, show


def pack(frames: list[Image.Image]) -> Image.Image:
    cell_w, cell_h = frames[0].size
    sheet = Image.new("RGBA", (cell_w * len(frames), cell_h), (0, 0, 0, 0))
    for index, frame in enumerate(frames):
        sheet.paste(frame, (index * cell_w, 0))
    return sheet


def on_background(frame: Image.Image, color: tuple[int, int, int, int]) -> Image.Image:
    canvas = Image.new("RGBA", frame.size, color)
    canvas.alpha_composite(frame)
    return canvas


def write_manifest(frame_count: int) -> None:
    durations = ", ".join("0.0417" for _ in range(frame_count))
    text = "\n".join(
        [
            "(",
            "    version: 1,",
            "    facing: Right,",
            "    weapon_in_body: false,",
            "    atlases: {",
            f'        "attack": (texture: "attack_pixel.png", cell: {SHOW}, columns: {frame_count}, rows: 1, padding: 0, density: 1),',
            "    },",
            "    clips: {",
            '        "idle": (atlas: "attack", first: 6, count: 1, mode: Loop(fps: 1.0),',
            f"            pivot: ({PIVOT[0] * 2}.0, {PIVOT[1] * 2}.0), fallback: None,",
            "            durations: [1.0], speed_reference: None),",
            f'        "attack": (atlas: "attack", first: 0, count: {frame_count}, mode: Once(fps: 24.0),',
            f"            pivot: ({PIVOT[0] * 2}.0, {PIVOT[1] * 2}.0), fallback: Some(\"idle\"),",
            f"            durations: [{durations}], speed_reference: None),",
            "    },",
            ")",
            "",
        ]
    )
    # The manifest needs a root clip named idle. Frame 6 is the recovery,
    # used only so the parser accepts the file. It is not an idle animation.
    (OUT / "attack.ron").write_text(text, encoding="utf-8")


def write_sockets() -> None:
    data = json.loads(MANIFEST.read_text(encoding="utf-8"))
    frames = data["sockets"]["attack_basic"]

    def scaled(frame: dict) -> str:
        weapon = frame["weapon"]
        hand = frame["hand_l"]
        head = frame["head"]
        chest = frame["chest"]
        back = frame["back"]
        front = "true" if weapon["front"] else "false"
        hand_front = "true" if hand["front"] else "false"
        return (
            "(weapon: (x: {wx:.2f}, y: {wy:.2f}, angle: {ang:.2f}, front: {front}), "
            "hand_l: (x: {lx:.2f}, y: {ly:.2f}, front: {hf}), "
            "head: (x: {hx:.2f}, y: {hy:.2f}), chest: (x: {cx:.2f}, y: {cy:.2f}), "
            "back: (x: {bx:.2f}, y: {by:.2f}))"
        ).format(
            wx=weapon["x"] * 2,
            wy=weapon["y"] * 2,
            ang=weapon["angle"],
            front=front,
            lx=hand["x"] * 2,
            ly=hand["y"] * 2,
            hf=hand_front,
            hx=head["x"] * 2,
            hy=head["y"] * 2,
            cx=chest["x"] * 2,
            cy=chest["y"] * 2,
            bx=back["x"] * 2,
            by=back["y"] * 2,
        )

    attack = ",\n            ".join(scaled(frame) for frame in frames)
    # One dummy idle frame so the socket table matches the manifest's idle clip.
    idle = scaled(frames[-1])
    text = (
        "(\n    version: 1,\n    clips: {\n"
        f'        "attack": [\n            {attack},\n        ],\n'
        f'        "idle": [\n            {idle},\n        ],\n'
        "    },\n)\n"
    )
    (OUT / "sockets.ron").write_text(text, encoding="utf-8")


def main() -> None:
    started = time.perf_counter()
    frames = sorted(RAW_ATTACK.glob("attack_basic_W_flat_*.png"))
    if len(frames) != 7:
        raise SystemExit(f"expected 7 attack renders, found {len(frames)}")
    OUT.mkdir(parents=True, exist_ok=True)
    EVIDENCE.mkdir(parents=True, exist_ok=True)
    raws = [Image.open(path).convert("RGBA") for path in frames]
    contracts = []
    shown = []
    for raw in raws:
        contract, show = automatic_pixel(raw)
        contracts.append(contract)
        shown.append(show)
    pack(shown).save(OUT / "attack_pixel.png")
    pack(contracts).save(OUT / "attack_contract.png")
    raw_dir = OUT / "raw"
    raw_dir.mkdir(parents=True, exist_ok=True)
    for index, raw in enumerate(raws):
        raw.save(raw_dir / f"attack_{index:02}.png")
    write_manifest(len(shown))
    write_sockets()

    # Same three moments of this attack: strike, follow-through, recovery.
    # The authored clip has no windup; frame 0 is already the strike.
    moments = (0, 3, 6)
    labels = ("impacto", "seguimento", "recuperacao")
    for index, label in zip(moments, labels):
        raws[index].save(EVIDENCE / f"raw_attack_{index:02d}_{label}.png")
        shown[index].save(EVIDENCE / f"auto_attack_{index:02d}_{label}.png")

    cell_w, cell_h = SHOW
    compare = Image.new("RGBA", (cell_w * 2 + 24, cell_h * 3 + 36), (16, 12, 18, 255))
    draw = ImageDraw.Draw(compare)
    draw.text((8, 4), "bruto reduzido so para caber", fill=(232, 224, 210, 255))
    draw.text((cell_w + 16, 4), "automatico, nao e pintura", fill=(232, 224, 210, 255))
    for row, index in enumerate(moments):
        y = 28 + row * cell_h
        fitted = raws[index].resize(SHOW, Image.Resampling.BOX)
        compare.alpha_composite(fitted, (8, y))
        compare.alpha_composite(shown[index], (cell_w + 16, y))
    compare.save(EVIDENCE / "same_gesture_attack.png")

    neutral = (186, 186, 190, 255)
    dark = (18, 12, 22, 255)
    grounds = Image.new("RGBA", (cell_w * 2 + 16, cell_h), (0, 0, 0, 0))
    grounds.alpha_composite(on_background(shown[0], neutral), (0, 0))
    grounds.alpha_composite(on_background(shown[0], dark), (cell_w + 16, 0))
    grounds.save(EVIDENCE / "backgrounds_attack.png")

    height, _top, _count = opaque_span(shown[0])
    contract_h, _, _ = opaque_span(contracts[0])
    head = raws[0].crop((250, 300, 470, 520))
    head.save(EVIDENCE / "raw_face_crop.png")

    palette = shown[0].convert("RGB").quantize(colors=12, method=Image.Quantize.MEDIANCUT).getpalette()
    colors = [tuple(palette[i : i + 3]) for i in range(0, min(len(palette or []), 36), 3)]
    metrics = {
        "machine_seconds": round(time.perf_counter() - started, 4),
        "human_artist_hours": None,
        "attack_frames": 7,
        "raw_size": list(raws[0].size),
        "contract_cell": list(CONTRACT),
        "presentation_cell": list(SHOW),
        "figure_texels_contract": contract_h,
        "figure_texels_presentation": height,
        "weapon_baked": False,
        "manual_pixelorama": False,
        "independent_drawing": False,
        "windup_in_source_clip": False,
        "palette_rgb": colors,
        "note": "Automatic processing of the V2.2 attack renders. No hand painting.",
    }
    (OUT / "metrics.json").write_text(json.dumps(metrics, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(metrics, indent=2))


if __name__ == "__main__":
    main()
