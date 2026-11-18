"""Turn the N3.2 generated references into comparable sheets.

The generator wrote JPEG files without alpha. Background removal, the
100-texel reduction and the palette snap are this script. None of that
is hand painting, and none of it splits the sword or the cape into layers.
"""

import json
import sys
from collections import deque
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "art-src/player/scripts/post"))
from resample import resize  # noqa: E402

SRC = Path(
    "/home/raul/.cursor/projects/home-raul-rea-de-trabalho-Andvarion-ProjetoRST/assets"
)
REF = ROOT / "art-src/n3-2-art-direction/references"
GEN = ROOT / "art-src/n3-2-art-direction/generated"
EVIDENCE = ROOT / "docs/development/n3_2/evidence"
V22 = ROOT / "art-src/n3-proof/n3_1/raw/attack_00.png"
TARGET = 100

PALETTE = [
    ((232, 206, 186), "skin"),
    ((168, 124, 108), "skin-shadow"),
    ((36, 28, 32), "hair"),
    ((92, 72, 58), "hair-light"),
    ((42, 32, 28), "leather"),
    ((78, 56, 42), "leather-light"),
    ((22, 24, 32), "cloth"),
    ((48, 52, 64), "cloth-light"),
    ((86, 90, 98), "steel"),
    ((150, 154, 162), "steel-light"),
    ((92, 68, 32), "gold"),
    ((176, 138, 64), "gold-light"),
    ((92, 22, 40), "cape"),
    ((150, 36, 58), "cape-light"),
    ((92, 48, 140), "arcane"),
    ((12, 8, 16), "outline"),
]

SOURCES = {
    "concept_a_side": "n32-concept-a-side.jpg",
    "concept_b_side": "n32-concept-b-side.jpg",
    "concept_c_side": "n32-concept-c-side.jpg",
    "b_anticipation": "n32-b-anticipation.jpg",
    "b_impact": "n32-b-impact.jpg",
    "b_recovery": "n32-b-recovery.jpg",
    "b_face": "n32-b-face.jpg",
}


def knock_out(image: Image.Image) -> Image.Image:
    """Remove the light gray field that touches the corners.

    A pixel counts as field only when it is light and nearly neutral, so the
    red cape and the skin stay. A pale sword that touches that field can be
    eaten; the metrics record the result instead of repairing it by hand.
    """
    image = image.convert("RGBA")
    pixels = image.load()
    width, height = image.size

    def field(color: tuple[int, int, int]) -> bool:
        light = max(color)
        chroma = light - min(color)
        return light >= 160 and chroma <= 26

    seen: set[tuple[int, int]] = set()
    queue = deque([(0, 0), (width - 1, 0), (0, height - 1), (width - 1, height - 1)])
    while queue:
        x, y = queue.popleft()
        if (x, y) in seen or x < 0 or y < 0 or x >= width or y >= height:
            continue
        seen.add((x, y))
        red, green, blue, _alpha = pixels[x, y]
        if not field((red, green, blue)):
            continue
        pixels[x, y] = (0, 0, 0, 0)
        queue.extend(((x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)))
    return image


def opaque_box(image: Image.Image) -> tuple[int, int, int, int]:
    pixels = image.load()
    width, height = image.size
    left, top, right, bottom = width, height, 0, 0
    found = False
    for y in range(height):
        for x in range(width):
            if pixels[x, y][3] > 16:
                found = True
                left = min(left, x)
                top = min(top, y)
                right = max(right, x)
                bottom = max(bottom, y)
    if not found:
        raise RuntimeError("no opaque pixels")
    return left, top, right + 1, bottom + 1


def crop_figure(image: Image.Image) -> Image.Image:
    left, top, right, bottom = opaque_box(image)
    return image.crop((left, top, right, bottom))


def fit_height(image: Image.Image, height: int, method: Image.Resampling) -> Image.Image:
    scale = height / image.height
    width = max(1, round(image.width * scale))
    return resize(image, (width, height), method)


def snap(image: Image.Image) -> Image.Image:
    image = image.convert("RGBA")
    colors = [color for color, _name in PALETTE if _name != "outline"]
    pixels = image.load()
    for y in range(image.height):
        for x in range(image.width):
            red, green, blue, alpha = pixels[x, y]
            if alpha < 16:
                pixels[x, y] = (0, 0, 0, 0)
                continue
            nearest = min(
                colors,
                key=lambda color: (color[0] - red) ** 2
                + (color[1] - green) ** 2
                + (color[2] - blue) ** 2,
            )
            pixels[x, y] = (*nearest, 255)
    return outline(image)


def outline(image: Image.Image) -> Image.Image:
    pixels = image.load()
    width, height = image.size
    ink = (12, 8, 16, 255)
    marks: list[tuple[int, int]] = []
    for y in range(height):
        for x in range(width):
            if pixels[x, y][3] == 0:
                continue
            for nx, ny in ((x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)):
                if nx < 0 or ny < 0 or nx >= width or ny >= height or pixels[nx, ny][3] == 0:
                    marks.append((x, y))
                    break
    for x, y in marks:
        pixels[x, y] = ink
    return image


def plate(image: Image.Image, color: tuple[int, int, int]) -> Image.Image:
    board = Image.new("RGBA", image.size, (*color, 255))
    board.alpha_composite(image)
    return board


def measure(image: Image.Image) -> dict[str, int]:
    left, top, right, bottom = opaque_box(image)
    return {
        "width": right - left,
        "height": bottom - top,
        "image_width": image.width,
        "image_height": image.height,
    }


def label_row(frames: list[tuple[str, Image.Image]], cell_h: int) -> Image.Image:
    pad = 16
    width = sum(frame.width + pad for _label, frame in frames) + pad
    sheet = Image.new("RGBA", (width, cell_h + 36), (24, 22, 28, 255))
    draw = ImageDraw.Draw(sheet)
    x = pad
    for label, frame in frames:
        fitted = fit_height(frame, cell_h, Image.Resampling.BOX)
        sheet.alpha_composite(fitted, (x, 32))
        draw.text((x, 8), label, fill=(230, 226, 214, 255))
        x += fitted.width + pad
    return sheet


def main() -> None:
    REF.mkdir(parents=True, exist_ok=True)
    GEN.mkdir(parents=True, exist_ok=True)
    EVIDENCE.mkdir(parents=True, exist_ok=True)
    knocked: dict[str, Image.Image] = {}
    for name, filename in SOURCES.items():
        image = knock_out(Image.open(SRC / filename))
        image.save(REF / f"{name}.png")
        knocked[name] = crop_figure(image)

    standing = ["concept_a_side", "concept_b_side", "concept_c_side"]
    adapted: dict[str, Image.Image] = {}
    automatic: dict[str, Image.Image] = {}
    metrics: dict[str, dict[str, int]] = {}
    for name, image in knocked.items():
        metrics[name] = measure(image)
        if name == "b_face":
            continue
        automatic[name] = fit_height(image, TARGET, Image.Resampling.BOX)
        adapted[name] = snap(fit_height(image, TARGET, Image.Resampling.NEAREST))
        automatic[name].save(GEN / f"{name}_auto.png")
        adapted[name].save(GEN / f"{name}_adapted.png")

    concepts = label_row(
        [(name[-6], adapted[name]) for name in standing],
        TARGET,
    )
    concepts.save(EVIDENCE / "concepts_panel.png")
    for name, letter in zip(standing, "abc", strict=True):
        plate(adapted[name], (186, 186, 190)).save(EVIDENCE / f"side_{letter}.png")

    silhouettes = []
    for name in standing:
        ink = Image.new("RGBA", adapted[name].size, (0, 0, 0, 0))
        src = adapted[name].load()
        dst = ink.load()
        for y in range(ink.height):
            for x in range(ink.width):
                if src[x, y][3] > 16:
                    dst[x, y] = (232, 220, 196, 255)
        silhouettes.append((name[-6], ink))
    label_row(silhouettes, TARGET).save(EVIDENCE / "silhouettes.png")

    swatch = Image.new("RGBA", (len(PALETTE) * 48, 72), (24, 22, 28, 255))
    draw = ImageDraw.Draw(swatch)
    for index, (color, name) in enumerate(PALETTE):
        draw.rectangle((index * 48 + 4, 4, index * 48 + 44, 44), fill=(*color, 255))
        draw.text((index * 48 + 4, 50), name[:4], fill=(230, 226, 214, 255))
    swatch.save(EVIDENCE / "palette.png")

    mannequin = crop_figure(Image.open(V22).convert("RGBA"))
    mannequin = fit_height(mannequin, TARGET, Image.Resampling.BOX)
    versus = label_row(
        [
            ("V2.2", mannequin),
            ("A", adapted["concept_a_side"]),
            ("B", adapted["concept_b_side"]),
            ("C", adapted["concept_c_side"]),
        ],
        TARGET,
    )
    versus.save(EVIDENCE / "versus_v22.png")
    label_row(
        [(name[-6], automatic[name]) for name in standing],
        TARGET,
    ).save(EVIDENCE / "auto_100.png")
    label_row(
        [(name[-6], adapted[name]) for name in standing],
        TARGET,
    ).save(EVIDENCE / "adapted_100.png")

    poses = ["b_anticipation", "b_impact", "b_recovery"]
    label_row(
        [("antecipacao", knocked["b_anticipation"]), ("impacto", knocked["b_impact"]), ("recuperacao", knocked["b_recovery"])],
        220,
    ).save(EVIDENCE / "pose_consistency.png")
    for name, label in zip(poses, ("anticipation", "impact", "recovery"), strict=True):
        plate(adapted[name], (18, 12, 22)).save(EVIDENCE / f"{label}.png")
        resize(adapted[name], (adapted[name].width * 2, adapted[name].height * 2), Image.Resampling.NEAREST).save(
            EVIDENCE / f"{label}_nearest2x.png"
        )

    dark = plate(adapted["concept_b_side"], (18, 12, 22))
    neutral = plate(adapted["concept_b_side"], (186, 186, 190))
    castle = plate(adapted["concept_b_side"], (48, 40, 46))
    backgrounds = Image.new("RGBA", (dark.width * 3 + 24, dark.height + 28), (24, 22, 28, 255))
    draw = ImageDraw.Draw(backgrounds)
    for index, (image, caption) in enumerate(
        ((neutral, "neutro"), (dark, "escuro"), (castle, "pedra plana"))
    ):
        backgrounds.paste(image, (8 + index * (dark.width + 4), 24))
        draw.text((8 + index * (dark.width + 4), 4), caption, fill=(230, 226, 214, 255))
    backgrounds.save(EVIDENCE / "backgrounds_b.png")

    before = label_row(
        [("N3.1 ataque", mannequin), ("N3.2 B em pe", adapted["concept_b_side"])],
        TARGET,
    )
    before.save(EVIDENCE / "before_after_n31.png")
    knocked["b_face"].save(EVIDENCE / "face_b.png")

    chosen = adapted["concept_b_side"]
    chosen.save(GEN / "concept_b_sprite.png")
    manifest = f"""(
    version: 1,
    facing: Right,
    weapon_in_body: true,
    atlases: {{
        "concept": (texture: "concept_b_sprite.png", cell: ({chosen.width}, {chosen.height}), columns: 1, rows: 1, padding: 0, density: 1),
    }},
    clips: {{
        "idle": (atlas: "concept", first: 0, count: 1, mode: Loop(fps: 1.0),
            pivot: ({chosen.width / 2:.1f}, {chosen.height - 1:.1f}), fallback: None,
            durations: [1.0], speed_reference: None),
    }},
)
"""
    (GEN / "concept_b.ron").write_text(manifest)
    (GEN / "metrics.json").write_text(json.dumps({"full": metrics, "target_height": TARGET}, indent=2))
    print(json.dumps({name: metrics[name] for name in poses}, indent=2))
    print(f"sprite {chosen.size}")


if __name__ == "__main__":
    main()
