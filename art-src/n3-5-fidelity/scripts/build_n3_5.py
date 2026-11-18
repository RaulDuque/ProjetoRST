"""N3.5: build the candidate from its text grid and assemble the evidence.

Inputs:
- `source/candidate_106.txt`: the candidate, in N3.4 palette keys (the 106 px
  reduction with the few hand edits that `metrics.json` lists by diff).
- `generated/reduction_{106,132,94}.png` from `reduce_fidelity.py`.
- The owner's references (read only) and the N3.4 files.
- Optional Bevy captures in `docs/development/n3_5/evidence/bevy_*.png`; when
  present, the camera sheets are cut from them (real screen pixels).

    python3 art-src/n3-5-fidelity/scripts/build_n3_5.py
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "art-src/player/scripts/post"))
sys.path.insert(0, str(ROOT / "art-src/n3-4-original-protagonist/scripts"))
from n34_palette import PALETTE  # noqa: E402
from resample import resize  # noqa: E402

HERE = ROOT / "art-src/n3-5-fidelity"
OUT = HERE / "generated"
EVIDENCE = ROOT / "docs/development/n3_5/evidence"
BASE = ROOT / "Md-usario/assets-user/prota/base"
REFERENCE = BASE / "image.png"
SHEET = BASE / "Ficha_Modular_do_Protagonista_Gotico.png"
N34 = ROOT / "art-src/n3-4-original-protagonist/generated/base_idle.png"
N34_DRAFT = ROOT / "art-src/n3-4-original-protagonist/source/base_idle_draft.txt"
SIDE_VIEW = (680, 80, 775, 418)
PROFILE_VIEW = (700, 82, 780, 158)
CANVAS = (80, 128)
PIVOT = (40, 118)
TRANSPARENT = "."
NEUTRAL = (150, 150, 156, 255)
LIGHT = (226, 224, 228, 255)
DARK = (18, 14, 22, 255)
INK = (40, 32, 40)
NEAREST = Image.Resampling.NEAREST
WINDOW_HEIGHT = 720
# World height of the figure, kept equal to N3.4 at density 2 (106 / 2).
WORLD_HEIGHT = 53.0
CAMERAS = (405, 360)
# Side-view landmarks measured on the sheet crop (rows of SIDE_VIEW, +-3 px).
SHEET_LANDMARKS = [("topo", 8), ("sobrancelha", 30), ("queixo", 50), ("ombro", 60),
                   ("cinto", 140), ("pulso", 155), ("dedos", 195), ("joelho", 240),
                   ("aba da bota", 268), ("sola", 337)]
SHEET_TOP, SHEET_SOLE = 8, 337


def zoom(image: Image.Image, factor: int) -> Image.Image:
    return resize(image, (image.width * factor, image.height * factor), NEAREST)


def on(image: Image.Image, background: tuple[int, int, int, int]) -> Image.Image:
    plate = Image.new("RGBA", image.size, background)
    plate.alpha_composite(image.convert("RGBA"))
    return plate


def fit(image: Image.Image, height: int) -> Image.Image:
    width = max(1, round(image.width * height / image.height))
    return image.convert("RGBA").resize((width, height), Image.Resampling.LANCZOS)


def sheet(panels: list[tuple[str, Image.Image]], background=LIGHT) -> Image.Image:
    gap, label = 16, 30
    width = sum(panel.width for _, panel in panels) + gap * (len(panels) + 1)
    height = max(panel.height for _, panel in panels) + label + gap * 2
    out = Image.new("RGBA", (width, height), background)
    draw = ImageDraw.Draw(out)
    x = gap
    for title, panel in panels:
        for line, text in enumerate(title.split("\n")):
            draw.text((x, 4 + 12 * line), text, fill=INK)
        out.alpha_composite(panel.convert("RGBA"), (x, gap + label))
        x += panel.width + gap
    return out


def stack(rows: list[Image.Image], background=LIGHT) -> Image.Image:
    out = Image.new("RGBA", (max(r.width for r in rows), sum(r.height for r in rows)), background)
    y = 0
    for row in rows:
        out.alpha_composite(row, (0, y))
        y += row.height
    return out


def read_grid(path: Path, palette: dict[str, tuple[int, int, int]]) -> Image.Image:
    rows = path.read_text(encoding="utf-8").rstrip("\n").split("\n")
    if len(rows) != CANVAS[1] or any(len(row) != CANVAS[0] for row in rows):
        raise SystemExit(f"{path.name}: grid must be {CANVAS[0]}x{CANVAS[1]}")
    image = Image.new("RGBA", CANVAS, (0, 0, 0, 0))
    for y, row in enumerate(rows):
        for x, key in enumerate(row):
            if key == TRANSPARENT:
                continue
            if key not in palette:
                raise SystemExit(f"{path.name}: unknown key {key!r} at {x},{y}")
            image.putpixel((x, y), (*palette[key], 255))
    return image


def n34_draft() -> Image.Image:
    """The N3.4 automatic reduction: the panel the owner called `reducao`."""
    return read_grid(N34_DRAFT, PALETTE)


def screen(art: Image.Image, figure_texels: int, camera: int) -> tuple[Image.Image, float]:
    """Nearest-sample the art to the screen size it gets under `camera`.

    The figure keeps WORLD_HEIGHT world units, so its density is
    figure_texels / WORLD_HEIGHT texels per unit and every texel covers
    (WINDOW_HEIGHT / camera) / density screen pixels.
    """
    px_per_texel = (WINDOW_HEIGHT / camera) * WORLD_HEIGHT / figure_texels
    size = (max(1, round(art.width * px_per_texel)), max(1, round(art.height * px_per_texel)))
    return resize(art, size, NEAREST), px_per_texel


def measure(image: Image.Image) -> dict:
    box = image.getchannel("A").getbbox()
    if box is None:
        raise SystemExit("empty candidate")
    left, top, right, bottom = box
    sole = [x for x in range(image.width) if image.getpixel((x, bottom - 1))[3]]
    colours = {px[:3] for _, px in image.getcolors(maxcolors=4096) or [] if px[3]}
    alphas = {a for _, a in image.getchannel("A").getcolors() or []}
    return {
        "canvas": list(CANVAS),
        "opaque_box": [left, top, right - 1, bottom - 1],
        "opaque_width": right - left,
        "opaque_height": bottom - top,
        "sole_y": bottom - 1,
        "sole_span": [sole[0], sole[-1]],
        "pivot": list(PIVOT),
        "colours_used": len(colours),
        "alpha_values": sorted(alphas),
        "source": "source/candidate_106.txt",
        "method": "reduce_fidelity.py (eroded mask, BOX, N3.4 palette) + hand edits",
        "gui_paint": False,
        "ai_generation": False,
    }


def write_manifest() -> None:
    (OUT / "candidate.ron").write_text(
        "(\n"
        "    version: 1,\n"
        "    facing: Right,\n"
        "    weapon_in_body: false,\n"
        "    atlases: {\n"
        '        "body": (texture: "candidate_106.png", cell: (80, 128), columns: 1, rows: 1,'
        " padding: 0, density: 2),\n"
        "    },\n"
        "    clips: {\n"
        '        "idle": (atlas: "body", first: 0, count: 1, mode: Loop(fps: 1.0),\n'
        f"            pivot: ({PIVOT[0]:.1f}, {PIVOT[1]:.1f}), fallback: None,\n"
        "            durations: [1.0], speed_reference: None),\n"
        "    },\n"
        ")\n",
        encoding="utf-8",
    )


def head(art: Image.Image, figure: int) -> Image.Image:
    """Top 21 % of the figure (crown to collar), cropped to the figure width."""
    left, top, right, _ = art.getchannel("A").getbbox() or (0, 0, art.width, art.height)
    return art.crop((left, top, right, top + round(figure * 0.21)))


def guides(panel: Image.Image, rows: list[tuple[str, float]], colour=(170, 30, 50)) -> Image.Image:
    out = panel.copy()
    draw = ImageDraw.Draw(out)
    for _, y in rows:
        draw.line([(0, round(y)), (out.width, round(y))], fill=colour, width=1)
    return out


def main() -> None:
    candidate_rows = (HERE / "source/candidate_106.txt").read_text(encoding="utf-8").split()
    reduction_rows = (HERE / "source/reduction_106.txt").read_text(encoding="utf-8").split()
    candidate = read_grid(HERE / "source/candidate_106.txt", PALETTE)
    reduction = read_grid(HERE / "source/reduction_106.txt", PALETTE)
    OUT.mkdir(parents=True, exist_ok=True)
    EVIDENCE.mkdir(parents=True, exist_ok=True)
    candidate.save(OUT / "candidate_106.png")
    write_manifest()
    metrics = measure(candidate)
    metrics["hand_edits"] = [
        {"x": x, "y": y, "from": a, "to": b}
        for y, (ra, rb) in enumerate(zip(reduction_rows, candidate_rows, strict=True))
        for x, (a, b) in enumerate(zip(ra, rb, strict=True))
        if a != b
    ]
    (OUT / "metrics.json").write_text(json.dumps(metrics, indent=2) + "\n", encoding="utf-8")

    reference = Image.open(REFERENCE).convert("RGBA")
    side = Image.open(SHEET).convert("RGBA").crop(SIDE_VIEW)
    profile = Image.open(SHEET).convert("RGBA").crop(PROFILE_VIEW)
    n34 = Image.open(N34).convert("RGBA")
    draft = n34_draft()
    reductions = {
        name: Image.open(OUT / f"reduction_{name}.png").convert("RGBA")
        for name in ("106", "132", "94")
    }
    # The 106 hypothesis is shown as the candidate (with its four removals);
    # 132 and 94 are the raw reductions, not cleaned.
    reductions["106"] = candidate
    face_box = (32, 12, 52, 34)

    # 1. Sources and provenance.
    tall = 424
    sheet([("image.png\n(original)", fit(reference, tall)),
           ("ficha lateral\n(recorte x680-775 y80-418)", fit(side, tall)),
           ("ficha (perfil)\n(recorte x700-780 y82-158)", fit(profile, 240)),
           ("reducao = rascunho N3.4\n(106 px, source/base_idle_draft.txt)", on(zoom(draft, 4), NEUTRAL)),
           ]).save(EVIDENCE / "reference_sources.png")

    # 2. Measurements: every figure scaled by the same rule (crown and sole
    # on the same two rows), guides at the sheet landmarks.
    figure_px = 4 * 106
    top_y = 20

    def aligned_sheet_side() -> Image.Image:
        factor = figure_px / (SHEET_SOLE - SHEET_TOP + 1)
        scaled = side.resize((round(side.width * factor), round(side.height * factor)),
                             Image.Resampling.LANCZOS)
        panel = Image.new("RGBA", (scaled.width, figure_px + 2 * top_y), LIGHT)
        panel.alpha_composite(scaled, (0, round(top_y - SHEET_TOP * factor)))
        return panel

    def aligned_sprite(art: Image.Image) -> Image.Image:
        box = art.getchannel("A").getbbox() or (0, 0, art.width, art.height)
        figure = art.crop(box)
        k = figure_px / figure.height
        scaled = resize(figure, (round(figure.width * k), figure_px), NEAREST)
        panel = Image.new("RGBA", (scaled.width + 20, figure_px + 2 * top_y), NEUTRAL)
        panel.alpha_composite(scaled, (10, top_y))
        return panel

    rule = [(name, top_y + (row - SHEET_TOP) * figure_px / (SHEET_SOLE - SHEET_TOP + 1))
            for name, row in SHEET_LANDMARKS]
    measured = sheet([(t, guides(p, rule)) for t, p in [
        ("ficha lateral", aligned_sheet_side()),
        ("reducao (N3.4 rascunho)", aligned_sprite(draft)),
        ("N3.4", aligned_sprite(n34)),
        ("N3.5 candidato", aligned_sprite(candidate)),
    ]])
    draw = ImageDraw.Draw(measured)
    for name, y in rule:
        draw.text((measured.width - 90, 16 + 30 + round(y) - 11), name, fill=(170, 30, 50))
    measured.save(EVIDENCE / "reference_measurements.png")

    # 3. Resolution hypotheses: same method, native and integer zoom.
    for name, file, title in [("106", "resolution_80x128.png", "A 80x128, figura 106"),
                              ("132", "resolution_96x160.png", "B 96x160, figura 132"),
                              ("94", "resolution_reference.png", "C 72x112, figura 94")]:
        art = reductions[name]
        sheet([(f"{title}\n1x", on(art, NEUTRAL)),
               ("4x", on(zoom(art, 4), NEUTRAL)),
               ("rosto 8x", on(zoom(head(art, int(name)), 8), NEUTRAL)),
               ]).save(EVIDENCE / file)

    # 4. Resolution x camera matrix (nearest simulation of the screen).
    cells = []
    for name, label in (("106", "80x128 (106)"), ("132", "96x160 (132)"), ("94", "72x112 (94)")):
        row = []
        for camera in CAMERAS:
            shown, ppt = screen(reductions[name], int(name), camera)
            density = int(name) / WORLD_HEIGHT
            kind = "inteiro 1:1" if abs(ppt - 1.0) < 0.005 else ("reduz" if ppt < 1 else "amplia")
            figure_screen = round(int(name) * ppt, 1)
            row.append((f"{label} camera {camera}\n{density:.2f} tx/u  {ppt:.3f} px/tx\n"
                        f"{figure_screen} px de tela, {kind}", on(zoom(shown, 3), DARK)))
        cells.append(sheet(row))
    stack(cells).save(EVIDENCE / "resolution_matrix.png")

    # 5. Face comparison, close and at screen scale.
    ref_face = fit(profile, 22 * 10)
    close_row = sheet([("ficha (perfil)", ref_face),
                       ("reducao (N3.4 rascunho) 10x", on(zoom(draft.crop((24, 11, 50, 33)), 10), NEUTRAL)),
                       ("N3.4 10x", on(zoom(n34.crop((24, 11, 50, 33)), 10), NEUTRAL)),
                       ("N3.5 10x", on(zoom(candidate.crop(face_box), 10), NEUTRAL))])
    scale_panels = []
    for title, art, box in [("N3.4", n34, (24, 11, 50, 33)), ("N3.5", candidate, face_box)]:
        for camera in CAMERAS:
            shown, ppt = screen(art, 106, camera)
            fb = tuple(round(v * ppt) for v in box)
            scale_panels.append((f"{title} camera {camera}\n(pixels de tela, 6x)",
                                 on(zoom(shown.crop(fb), 6), NEUTRAL)))
    stack([close_row, sheet(scale_panels)]).save(EVIDENCE / "face_comparison.png")

    # 6. Hair.
    hair_box = (24, 11, 52, 60)
    sheet([("N3.4 claro", on(zoom(n34.crop(hair_box), 6), LIGHT)),
           ("N3.4 escuro", on(zoom(n34.crop(hair_box), 6), DARK)),
           ("N3.5 claro", on(zoom(candidate.crop(hair_box), 6), LIGHT)),
           ("N3.5 escuro", on(zoom(candidate.crop(hair_box), 6), DARK))]).save(
        EVIDENCE / "hair_comparison.png")

    # 7. Clothing: torso+arm and legs+boots, ficha vs N3.4 vs N3.5 at 4x.
    def side_rows(r0: int, r1: int) -> Image.Image:
        factor = 106 / (SHEET_SOLE - SHEET_TOP + 1)
        y0 = round(SHEET_TOP + (r0 - 13) / factor)
        y1 = round(SHEET_TOP + (r1 - 13) / factor)
        return fit(side.crop((0, y0, side.width, y1)), 4 * (r1 - r0))

    rows = []
    for r0, r1, part in ((28, 72, "camisa, braco, cinto, bracelete"), (72, 119, "calca, joelho, bota")):
        rows.append(sheet([(f"ficha: {part}", side_rows(r0, r1)),
                           ("N3.4", on(zoom(n34.crop((20, r0, 56, r1)), 4), NEUTRAL)),
                           ("N3.5", on(zoom(candidate.crop((20, r0, 56, r1)), 4), NEUTRAL))]))
    stack(rows).save(EVIDENCE / "clothing_comparison.png")

    # 8. N3.4 vs N3.5, whole body.
    sheet([("N3.4 4x", on(zoom(n34, 4), NEUTRAL)), ("N3.5 4x", on(zoom(candidate, 4), NEUTRAL)),
           ("N3.4 1x", on(n34, NEUTRAL)), ("N3.5 1x", on(candidate, NEUTRAL))]).save(
        EVIDENCE / "n34_vs_n35.png")

    # 9. Native scale and dark background.
    sheet([("1x neutro", on(candidate, NEUTRAL)), ("1x claro", on(candidate, LIGHT)),
           ("1x escuro", on(candidate, DARK))]).save(EVIDENCE / "native_scale.png")
    on(zoom(candidate, 4), DARK).save(EVIDENCE / "dark_background.png")

    # 10. Camera sheets from the real Bevy captures, when they exist.
    for camera in CAMERAS:
        capture = EVIDENCE / f"bevy_n35_{camera}.png"
        if not capture.is_file():
            continue
        shot = Image.open(capture).convert("RGBA")
        box = (590, 240, 690, 370)
        n34_capture = EVIDENCE / f"bevy_n34_{camera}.png"
        panels = [(f"N3.5 camera {camera}\nBevy, 1x", shot.crop(box)),
                  ("N3.5 4x (pixels da captura)", zoom(shot.crop(box), 4))]
        if n34_capture.is_file():
            panels.append((f"N3.4 camera {camera}\n4x", zoom(Image.open(n34_capture).convert("RGBA").crop(box), 4)))
        sheet(panels).save(EVIDENCE / f"camera_{camera}.png")

    # 11. Palette test on the same 106 px reduction.
    variants = [("paleta N3.4 (escolhida)", reduction)] + [
        (title, Image.open(OUT / f"variant_{name}.png").convert("RGBA"))
        for name, title in (("adaptive32", "mediancut 32"),
                            ("adaptive24_skin_hair", "mediancut 24 + pele/cabelo N3.4"))
    ]
    stack([
        sheet([(t, on(zoom(v.crop(face_box), 10), NEUTRAL)) for t, v in variants]),
        sheet([(t, on(zoom(v, 3), NEUTRAL)) for t, v in variants]),
    ]).save(EVIDENCE / "palette_test.png")

    # 12. Final candidate card.
    sheet([("N3.5 candidato 80x128\nfigura 106, pivo (40,118)\nNAO APROVADO",
            on(zoom(candidate, 4), NEUTRAL)),
           ("fundo escuro", on(zoom(candidate, 4), DARK))]).save(EVIDENCE / "final_candidate.png")

    print(json.dumps(metrics, indent=2))


if __name__ == "__main__":
    main()
