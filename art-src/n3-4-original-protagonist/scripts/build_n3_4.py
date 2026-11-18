"""N3.4 step 2: turn the hand-edited text grid into the sprite and evidence.

Source of truth: `source/base_idle.txt` (80x128, one character per pixel,
colours in `n34_palette.py`). The grid was started from
`source/base_idle_draft.txt` (see `reduce_reference.py`) and then corrected
pixel by pixel by hand. This script only converts and compares; it does not
draw anything.

    python3 art-src/n3-4-original-protagonist/scripts/build_n3_4.py

Writes `generated/` (sprite, manifest, metrics) and
`docs/development/n3_4/evidence/`.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "art-src/player/scripts/post"))
sys.path.insert(0, str(Path(__file__).resolve().parent))
from n34_palette import PALETTE, TRANSPARENT  # noqa: E402
from resample import resize  # noqa: E402

HERE = ROOT / "art-src/n3-4-original-protagonist"
SOURCE = HERE / "source/base_idle.txt"
DRAFT = HERE / "source/base_idle_draft.txt"
OUT = HERE / "generated"
EVIDENCE = ROOT / "docs/development/n3_4/evidence"
BASE = ROOT / "Md-usario/assets-user/prota/base"
REFERENCE = BASE / "image.png"
SHEET = BASE / "Ficha_Modular_do_Protagonista_Gotico.png"
N33 = ROOT / "art-src/n3-3-native-pixel/generated/body_idle.png"
SIDE_VIEW = (680, 80, 775, 418)
FACE_VIEW = (700, 82, 780, 158)
CANVAS = (80, 128)
PIVOT = (40, 118)
NEUTRAL = (150, 150, 156, 255)
LIGHT = (226, 224, 228, 255)
DARK = (18, 14, 22, 255)
INK = (40, 32, 40)
NEAREST = Image.Resampling.NEAREST
# Sprite rows of the anatomical landmarks (crown, chin, shoulder, belt,
# bracer, knee, boot top, sole).
LANDMARKS = [("topo", 13), ("queixo", 27), ("ombro", 30), ("cinto", 51),
             ("bracelete", 59), ("joelho", 82), ("bota", 95), ("sola", 118)]


def read_grid(path: Path) -> list[str]:
    rows = path.read_text(encoding="utf-8").rstrip("\n").split("\n")
    if len(rows) != CANVAS[1] or any(len(row) != CANVAS[0] for row in rows):
        raise SystemExit(f"{path.name}: grid must be {CANVAS[0]}x{CANVAS[1]}")
    unknown = {c for row in rows for c in row} - set(PALETTE) - {TRANSPARENT}
    if unknown:
        raise SystemExit(f"{path.name}: unknown colour keys {sorted(unknown)}")
    return rows


def to_image(rows: list[str]) -> Image.Image:
    image = Image.new("RGBA", CANVAS, (0, 0, 0, 0))
    for y, row in enumerate(rows):
        for x, key in enumerate(row):
            if key != TRANSPARENT:
                image.putpixel((x, y), (*PALETTE[key], 255))
    return image


def zoom(image: Image.Image, factor: int) -> Image.Image:
    return resize(image, (image.width * factor, image.height * factor), NEAREST)


def on(image: Image.Image, background: tuple[int, int, int, int]) -> Image.Image:
    plate = Image.new("RGBA", image.size, background)
    plate.alpha_composite(image)
    return plate


def fit(image: Image.Image, height: int) -> Image.Image:
    width = max(1, round(image.width * height / image.height))
    return image.convert("RGBA").resize((width, height), Image.Resampling.LANCZOS)


def silhouette(image: Image.Image) -> Image.Image:
    alpha = image.getchannel("A").point(lambda a: 255 if a >= 128 else 0)
    out = Image.new("RGBA", image.size, (0, 0, 0, 0))
    out.paste((20, 16, 22, 255), mask=alpha)
    return out


def sheet(panels: list[tuple[str, Image.Image]], background=LIGHT) -> Image.Image:
    gap, label = 16, 22
    width = sum(panel.width for _, panel in panels) + gap * (len(panels) + 1)
    height = max(panel.height for _, panel in panels) + label + gap * 2
    out = Image.new("RGBA", (width, height), background)
    draw = ImageDraw.Draw(out)
    x = gap
    for title, panel in panels:
        draw.text((x, gap // 2), title, fill=INK)
        out.alpha_composite(panel.convert("RGBA"), (x, gap + label))
        x += panel.width + gap
    return out


def side_reference() -> Image.Image:
    return Image.open(SHEET).convert("RGBA").crop(SIDE_VIEW)


def measure(image: Image.Image) -> dict:
    box = image.getchannel("A").getbbox()
    if box is None:
        raise SystemExit("empty sprite")
    left, top, right, bottom = box
    sole = [x for x in range(CANVAS[0]) if image.getpixel((x, bottom - 1))[3]]
    colours = {px[:3] for _, px in image.getcolors(maxcolors=4096) or [] if px[3]}
    return {
        "canvas": list(CANVAS),
        "opaque_box": [left, top, right - 1, bottom - 1],
        "opaque_width": right - left,
        "opaque_height": bottom - top,
        "sole_y": bottom - 1,
        "sole_span": [sole[0], sole[-1]],
        "pivot": list(PIVOT),
        "colours_used": len(colours),
        "source": "source/base_idle.txt",
        "reduced_from_illustration": True,
        "hand_edited_pixels": None,
        "gui_paint": False,
    }


def write_manifest() -> None:
    (OUT / "base.ron").write_text(
        "(\n"
        "    version: 1,\n"
        "    facing: Right,\n"
        "    weapon_in_body: false,\n"
        "    atlases: {\n"
        '        "body": (texture: "base_idle.png", cell: (80, 128), columns: 1, rows: 1,'
        " padding: 0, density: 1),\n"
        "    },\n"
        "    clips: {\n"
        '        "idle": (atlas: "body", first: 0, count: 1, mode: Loop(fps: 1.0),\n'
        f"            pivot: ({PIVOT[0]:.1f}, {PIVOT[1]:.1f}), fallback: None,\n"
        "            durations: [1.0], speed_reference: None),\n"
        "    },\n"
        ")\n",
        encoding="utf-8",
    )


def main() -> None:
    rows = read_grid(SOURCE)
    draft_rows = read_grid(DRAFT)
    sprite = to_image(rows)
    draft = to_image(draft_rows)
    OUT.mkdir(parents=True, exist_ok=True)
    EVIDENCE.mkdir(parents=True, exist_ok=True)
    sprite.save(OUT / "base_idle.png")
    write_manifest()
    metrics = measure(sprite)
    metrics["hand_edited_pixels"] = sum(
        a != b for ra, rb in zip(rows, draft_rows, strict=True) for a, b in zip(ra, rb, strict=True)
    )
    (OUT / "metrics.json").write_text(json.dumps(metrics, indent=2) + "\n", encoding="utf-8")

    reference = Image.open(REFERENCE).convert("RGBA")
    side = side_reference()
    n33 = Image.open(N33).convert("RGBA") if N33.is_file() else Image.new("RGBA", CANVAS)
    tall = 4 * CANVAS[1]

    # 1. The owner's reference, untouched (copy for the evidence folder).
    reference.save(EVIDENCE / "original_reference.png")

    # 2. Reference analysis: the side view scaled so its crown and sole land
    # on the sprite's crown and sole rows; the lines are the sprite's own
    # landmark rows, so a line that misses the same feature on the sheet
    # means a proportion drifted.
    figure_top, figure_sole = 8, 337
    factor = 4 * metrics["opaque_height"] / (figure_sole - figure_top + 1)
    scaled = side.resize(
        (round(side.width * factor), round(side.height * factor)), Image.Resampling.LANCZOS
    )
    aligned = Image.new("RGBA", (scaled.width, 4 * CANVAS[1]), LIGHT)
    aligned.alpha_composite(scaled, (0, round(4 * metrics["opaque_box"][1] - figure_top * factor)))
    analysis = sheet([("ficha lateral (dir.)", aligned),
                      ("rascunho reduzido", on(zoom(draft, 4), NEUTRAL)),
                      ("N3.4 corrigido a mao", on(zoom(sprite, 4), NEUTRAL))])
    draw = ImageDraw.Draw(analysis)
    for name, row in LANDMARKS:
        y = 16 + 22 + 4 * row + 2
        draw.line([(0, y), (analysis.width, y)], fill=(170, 30, 50), width=1)
        draw.text((analysis.width - 70, y - 11), name, fill=(170, 30, 50))
    analysis.save(EVIDENCE / "reference_analysis.png")

    # 3. Face study.
    face_box = (24, 11, 50, 34)
    face_ref = fit(Image.open(SHEET).convert("RGBA").crop(FACE_VIEW), 14 * 23)
    sheet([("ficha (perfil)", face_ref),
           ("reducao", on(zoom(draft.crop(face_box), 14), NEUTRAL)),
           ("N3.4", on(zoom(sprite.crop(face_box), 14), NEUTRAL)),
           ("N3.3", on(zoom(n33.crop((26, 10, 52, 33)), 14), NEUTRAL))]).save(EVIDENCE / "face_study.png")

    # 4. Hair study on light and dark backgrounds.
    hair_box = (24, 11, 50, 72)
    hair = zoom(sprite.crop(hair_box), 8)
    sheet([("claro", on(hair, LIGHT)), ("neutro", on(hair, NEUTRAL)),
           ("escuro", on(hair, DARK))]).save(EVIDENCE / "hair_study.png")

    # 5. Body base alone, 4x.
    on(zoom(sprite, 4), NEUTRAL).save(EVIDENCE / "body_base.png")

    # 6. Comparison sheet required by section 13.
    game = on(zoom(sprite, 2), DARK)
    sheet([("image.png", fit(reference, tall)),
           ("ficha lateral", fit(side, tall)),
           ("N3.3", on(zoom(n33, 4), NEUTRAL)),
           ("N3.4", on(zoom(sprite, 4), NEUTRAL)),
           ("rosto 8x", on(zoom(sprite.crop(face_box), 8), NEUTRAL)),
           ("silhueta", zoom(silhouette(sprite), 4)),
           ("escala do jogo (2x)", game)]).save(EVIDENCE / "body_comparison.png")

    # 7. Silhouettes on the same canvas: the reduced side view (the draft's
    # flood-fill mask, same crown and sole rows), N3.4 and N3.3.
    sheet([("ficha reduzida (mascara)", zoom(silhouette(draft), 4)),
           ("N3.4", zoom(silhouette(sprite), 4)),
           ("N3.3", zoom(silhouette(n33), 4))]).save(EVIDENCE / "silhouette_comparison.png")

    # 8. Native scale (1:1) and nearest-neighbour previews.
    sheet([("1x neutro", on(sprite, NEUTRAL)), ("1x claro", on(sprite, LIGHT)),
           ("1x escuro", on(sprite, DARK))]).save(EVIDENCE / "native_scale.png")
    sheet([("2x", on(zoom(sprite, 2), NEUTRAL)), ("3x", on(zoom(sprite, 3), NEUTRAL)),
           ("4x", on(zoom(sprite, 4), NEUTRAL))]).save(EVIDENCE / "nearest_preview.png")
    on(zoom(sprite, 4), DARK).save(EVIDENCE / "dark_background.png")

    print(json.dumps(metrics, indent=2))


if __name__ == "__main__":
    main()
