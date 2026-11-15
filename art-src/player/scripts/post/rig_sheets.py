"""Review sheets of the v005 rig (V1.2-E), built from renders/v005/rig_manifest_v005.json.

Sheets (written to comparisons/v1_2e_rig_v005/):

- ``rig_validation_sheet_v005.png``: P01-P13, side (game view) and 3/4, with
  the worst severity found by the automatic checks;
- ``rig_structure_v005.png``: bones over a faded flat render, side and
  front, colour per bone collection, labels and the hierarchy;
- ``deformation_tests_v005.png``: T01-T11 close-ups with their issues;
- ``v004_vs_v005_neutral.png``: neutral contract masters, v004 and v005,
  plus the pixel difference (the rig must not change the look at rest);
- ``game_scale_v005.png``: idle, walk, run, dodge (64x64 cell) and the
  basic attack (128x96 combat cell) in finishes A, B and C at 1:1, x1.5
  and x2, sabre N (normal) and W (x1.7 width);
- ``animation_tests_v005.png``: frames of the walk, attack and dodge tests.

Also writes the A/B sprites of each game-scale master next to it (for
``camera_compare.py``) and ``sheet_metrics_v005.json``.

Usage:
    python3 art-src/player/scripts/post/rig_sheets.py
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path

from PIL import Image, ImageChops, ImageDraw, ImageStat

from contract import COMBAT_FRAME, FRAME, HD_DENSITY
from labels import font
from palette import CHARACTER_RGB, SCENE_BACKGROUND_FALLBACK
from pixelize import add_outline, add_soft_rim, high_res, pixel_art
from resample import resize

PLAYER_DIR = Path(__file__).resolve().parents[2]
RENDERS = PLAYER_DIR / "renders" / "v005"
OUT = PLAYER_DIR / "comparisons" / "v1_2e_rig_v005"
V004 = PLAYER_DIR / "renders" / "v004"
DARK = (38, 37, 44)  # dark neutral sheet background
TEXT = (232, 230, 238)
MUTED = (160, 156, 170)
PAD = 14
SEVERITY_COLOURS = {"OK": (90, 170, 100), "Cosmético": (120, 150, 120), "Baixo": (150, 160, 90),
                    "Médio": (210, 160, 60), "Alto": (220, 90, 60), "Crítico": (230, 40, 40)}
ZOOMS = (1.0, 1.5, 2.0)


def on(path: Path | str, size: tuple[int, int], bg: tuple[int, int, int] = DARK) -> Image.Image:
    image = Image.open(path).convert("RGBA")
    base = Image.new("RGBA", image.size, (*bg, 255))
    base.alpha_composite(image)
    return base.resize(size, Image.Resampling.LANCZOS)


def badge(draw: ImageDraw.ImageDraw, xy: tuple[int, int], severity: str) -> None:
    colour = SEVERITY_COLOURS.get(severity, MUTED)
    x, y = xy
    draw.rounded_rectangle((x, y, x + 92, y + 20), 5, fill=colour)
    draw.text((x + 8, y + 3), severity, fill=(20, 20, 24), font=font(12))


def header(sheet: Image.Image, title: str, subtitle: str) -> ImageDraw.ImageDraw:
    draw = ImageDraw.Draw(sheet)
    draw.text((PAD, 10), title, fill=TEXT, font=font(20))
    draw.text((PAD, 38), subtitle, fill=MUTED, font=font(13))
    return draw


def issues_text(issues: list[dict[str, object]], limit: int = 3) -> list[str]:
    keep = [i for i in issues if i["severity"] in ("Crítico", "Alto", "Médio")][:limit]
    return [f"{i['severity']}: {str(i['what']).replace('interseção ', '')} ({i['value']})" for i in keep]


def validation_sheet(m: dict) -> Path:
    poses = m["renders"]["poses"]
    cell, cols = 290, 3
    block_w, block_h = 2 * cell + PAD, cell + 70
    rows = (len(poses) + cols - 1) // cols
    sheet = Image.new("RGBA", (PAD + cols * (block_w + PAD), 64 + rows * (block_h + PAD)), (*DARK, 255))
    draw = header(sheet, "Rig v005 (experimental): poses de validação P01-P13",
                  "Lateral ortográfica (vista do jogo) e 3/4. Severidade = pior problema medido automaticamente "
                  "(interseções e volume). Testes artísticos: nenhum valor de gameplay alterado.")
    for i, (key, paths) in enumerate(poses.items()):
        x = PAD + (i % cols) * (block_w + PAD)
        y = 64 + (i // cols) * (block_h + PAD)
        info = m["poses"][key]
        sheet.alpha_composite(on(paths[0], (cell, cell)), (x, y))
        sheet.alpha_composite(on(paths[1], (cell, cell)), (x + cell + PAD, y))
        draw.text((x, y + cell + 6), info["label"], fill=TEXT, font=font(14))
        badge(draw, (x + 2 * cell + PAD - 92, y + cell + 6), info["max_severity"])
        for k, line in enumerate(issues_text(info["issues"], 2)):
            draw.text((x, y + cell + 28 + 16 * k), line, fill=MUTED, font=font(11))
    path = OUT / "rig_validation_sheet_v005.png"
    sheet.convert("RGB").save(path)
    return path


LABELLED = ("root", "pelvis", "spine_01", "chest", "neck", "head", "clavicle.R", "upper_arm.R", "forearm.R",
            "hand.R", "sword", "thigh.R", "shin.R", "foot.R", "toe.R", "coat_front.R_01", "coat_back.R_02",
            "cape_arm.L_01", "cape_back_02", "CTRL_foot_ik.R", "CTRL_knee_pole.R", "CTRL_hand_ik.R",
            "CTRL_elbow_pole.R", "upper_arm.L", "hand.L", "thigh.L")
LEGEND = (("DEF", "deformação (corpo)", (140, 158, 199)), ("COAT", "casaca", (191, 51, 71)),
          ("CAPE", "meia-capa", (242, 115, 26)), ("SWORD", "espada", (242, 191, 38)),
          ("CTRL", "controles (root, IK, polos)", (51, 191, 77)))


def tree_lines(bones: list[dict]) -> list[str]:
    children: dict[str | None, list[str]] = {}
    for b in bones:
        children.setdefault(b["parent"], []).append(b["name"])
    lines: list[str] = []

    def walk(name: str, depth: int) -> None:
        lines.append("  " * depth + name)
        for child in children.get(name, []):
            walk(child, depth + 1)

    for root in children.get(None, []):
        walk(root, 0)
    return lines


def structure_sheet(m: dict) -> Path:
    views = m["renders"]["structure"]
    size = 760
    tree = tree_lines(m["bones"])
    tree_w = 300
    sheet = Image.new("RGBA", (PAD + 2 * (size + PAD) + tree_w, 70 + size + 90), (*DARK, 255))
    draw = header(sheet, f"Rig v005: estrutura ({len(m['bones'])} ossos, {sum(b['deform'] for b in m['bones'])} "
                  "de deformação)",
                  "Ossos renderizados como octaedros sobre o personagem em cores chapadas (35 %). "
                  "Repouso = pose do v004. .R = direita do personagem = lado próximo da câmera lateral.")
    for i, view in enumerate(("side", "front")):
        x, y = PAD + i * (size + PAD), 64
        body = on(views[view]["body"], (size, size))
        faded = Image.blend(Image.new("RGBA", body.size, (*DARK, 255)), body, 0.35)
        bones = Image.open(views[view]["bones"]).convert("RGBA").resize((size, size), Image.Resampling.LANCZOS)
        faded.alpha_composite(bones)
        sheet.alpha_composite(faded, (x, y))
        scale = size / 900.0
        for name in LABELLED:
            px, py = views[view]["labels"][name]
            if 0 <= px <= 900 and 0 <= py <= 900:
                draw.text((x + px * scale + 6, y + py * scale - 6), name, fill=TEXT, font=font(10))
        draw.text((x, y + size + 4), "Lateral (vista do jogo)" if view == "side" else "Frontal", fill=TEXT,
                  font=font(14))
    tx = PAD + 2 * (size + PAD)
    draw.text((tx, 64), "Hierarquia", fill=TEXT, font=font(14))
    for k, line in enumerate(tree):
        draw.text((tx, 86 + 13 * k), line, fill=MUTED, font=font(10))
    ly = 64 + size + 30
    for k, (_, label, rgb) in enumerate(LEGEND):
        draw.rectangle((PAD + k * 260, ly, PAD + k * 260 + 16, ly + 16), fill=rgb)
        draw.text((PAD + k * 260 + 22, ly), label, fill=TEXT, font=font(12))
    path = OUT / "rig_structure_v005.png"
    sheet.convert("RGB").save(path)
    return path


def deformation_sheet(m: dict) -> Path:
    tests = m["renders"]["tests"]
    cell, cols = 330, 4
    rows = (len(tests) + cols - 1) // cols
    block_h = cell + 80
    sheet = Image.new("RGBA", (PAD + cols * (cell + PAD), 64 + rows * (block_h + PAD)), (*DARK, 255))
    draw = header(sheet, "Rig v005: testes de deformação (ombros, gola, tronco, joelhos, meia-capa)",
                  "Closes das áreas críticas. Interseções contadas em pares de triângulos acima do repouso; "
                  "volume = malhas fechadas (braços, pernas, pés, pescoço).")
    for i, (key, path) in enumerate(tests.items()):
        x = PAD + (i % cols) * (cell + PAD)
        y = 64 + (i // cols) * (block_h + PAD)
        info = m["poses"][key]
        sheet.alpha_composite(on(path, (cell, cell)), (x, y))
        draw.text((x, y + cell + 6), info["label"], fill=TEXT, font=font(13))
        badge(draw, (x + cell - 92, y + cell + 6), info["max_severity"])
        for k, line in enumerate(issues_text(info["issues"], 3)):
            draw.text((x, y + cell + 28 + 15 * k), line, fill=MUTED, font=font(11))
    path = OUT / "deformation_tests_v005.png"
    sheet.convert("RGB").save(path)
    return path


def neutral_compare(m: dict) -> tuple[Path, dict[str, float]]:
    views = ("front", "side", "back")
    crop = (232, 120, 792, 1024)
    h = 520
    w = round((crop[2] - crop[0]) * h / (crop[3] - crop[1]))
    sheet = Image.new("RGBA", (PAD + len(views) * (w + PAD), 70 + 2 * (h + 36)), (*DARK, 255))
    draw = header(sheet, "v004 × v005 em repouso (P01 Neutral): mesma câmera, escala, luz e fundo",
                  "Masters do contrato (1024 px). Diferença medida pixel a pixel abaixo de cada vista.")
    metrics = {}
    for i, view in enumerate(views):
        old = Image.open(V004 / f"swordsman_base_v004_{view}_master.png").convert("RGBA")
        new = Image.open(RENDERS / f"swordsman_rig_v005_{view}_master.png").convert("RGBA")
        diff = ImageChops.difference(old, new)
        mean = sum(ImageStat.Stat(diff).mean) / 4.0
        changed = diff.convert("L").point(lambda v: 255 if v > 8 else 0).histogram()[255]
        metrics[view] = {"mean_abs_diff": round(mean, 4), "pixels_over_8": changed}
        x = PAD + i * (w + PAD)
        for k, (label, image) in enumerate((("v004", old), ("v005 rig, P01", new))):
            y = 64 + k * (h + 36)
            base = Image.new("RGBA", image.size, (*DARK, 255))
            base.alpha_composite(image)
            sheet.alpha_composite(base.crop(crop).resize((w, h), Image.Resampling.LANCZOS), (x, y + 20))
            draw.text((x, y + 2), f"{label}, {view}", fill=TEXT, font=font(13))
        draw.text((x, 64 + 2 * (h + 36) - 14), f"Δ médio {mean:.3f}/255, {changed} px > 8", fill=MUTED, font=font(12))
    path = OUT / "v004_vs_v005_neutral.png"
    sheet.convert("RGB").save(path)
    return path, metrics


def finishes(master: Path, combat: bool) -> dict[str, Image.Image]:
    """A (pixel art + outline) and B (hd + soft rim) from one master; combat cells are padded square."""
    image = Image.open(master).convert("RGBA")
    frame = COMBAT_FRAME[0] if combat else FRAME
    pad = 0
    if combat:
        pad = image.size[0] - image.size[1]
        square = Image.new("RGBA", (image.size[0], image.size[0]), (0, 0, 0, 0))
        square.alpha_composite(image, (0, pad))
        image = square
    block = image.size[0] // frame
    a = add_outline(pixel_art(image, frame), CHARACTER_RGB["rim"])
    b = add_soft_rim(high_res(image, frame), CHARACTER_RGB["rim"])
    if combat:
        top = pad // block
        a = a.crop((0, top, a.size[0], a.size[1]))
        b = b.crop((0, top * HD_DENSITY, b.size[0], b.size[1]))
    return {"A": a, "B": b}


def screen(sprite: Image.Image, finish: str, zoom: float, density: int) -> Image.Image:
    w, h = sprite.size[0] // density, sprite.size[1] // density
    size = (round(w * zoom), round(h * zoom))
    method = Image.Resampling.NEAREST if finish == "A" else Image.Resampling.BILINEAR
    tile = Image.new("RGBA", size, (*SCENE_BACKGROUND_FALLBACK, 255))
    tile.alpha_composite(resize(sprite, size, method))
    return tile


def game_scale_sheet(m: dict) -> Path:
    rows = []
    for key, paths in m["renders"]["game"].items():
        combat = "attack" in key
        for tag in ("N", "W"):
            shaded = next(Path(p) for p in paths if f"_{tag}_shaded_" in p)
            flat = next(Path(p) for p in paths if f"_{tag}_flat_" in p)
            fs = finishes(shaded, combat)
            fc = finishes(flat, combat)
            for name, image in fs.items():
                image.save(shaded.with_name(shaded.stem.replace("_master", "") + f"_{name}"
                                            + ("_pixel_rim" if name == "A" else "_hd_rim") + ".png"))
            rows.append((f"{m['poses'][key]['label']} — lâmina {tag}", combat,
                         {"A": fs["A"], "B": fs["B"], "C": fc["B"]}))
    cell_w = {False: sum(round(FRAME * z) for z in ZOOMS), True: sum(round(COMBAT_FRAME[0] * z) for z in ZOOMS)}
    group_w = max(cell_w.values()) + 2 * PAD
    row_h = {False: round(FRAME * 2) + 30, True: round(COMBAT_FRAME[1] * 2) + 30}
    label_w = 250
    height = 90 + sum(row_h[c] + PAD for _, c, _ in rows)
    sheet = Image.new("RGBA", (label_w + 3 * group_w + PAD, height), (*DARK, 255))
    draw = header(sheet, "Rig v005 na escala do jogo: acabamentos A, B, C × câmeras 1:1, ×1,5, ×2",
                  "Fundo da sala. ~52 px de altura em 1:1 (64×64, pivô (32, 63)); ataque na célula de combate "
                  "128×96, pivô (40, 95). A = alternativa documentada; B e C priorizados. Nada é decisão.")
    for k, (name, desc) in enumerate((("A", "A pixel art (nearest)"), ("B", "B alta resolução (linear)"),
                                      ("C", "C cores chapadas (via B)"))):
        draw.text((label_w + k * group_w, 62), f"{desc}: 1:1  ×1,5  ×2", fill=TEXT, font=font(13))
    y = 90
    for label, combat, sprites in rows:
        draw.text((PAD, y + 4), label, fill=TEXT, font=font(12))
        for k, finish in enumerate(("A", "B", "C")):
            x = label_w + k * group_w
            density = 1 if finish == "A" else HD_DENSITY
            for zoom in ZOOMS:
                tile = screen(sprites[finish], "A" if finish == "A" else "B", zoom, density)
                sheet.alpha_composite(tile, (x, y + row_h[combat] - 24 - tile.size[1] + 20))
                x += tile.size[0] + 4
        y += row_h[combat] + PAD
    path = OUT / "game_scale_v005.png"
    sheet.convert("RGB").save(path)
    return path


def animation_sheet(m: dict) -> Path:
    tests = m["animation_tests"]
    cell = 200
    cols = max(len(t["frames"]) for t in tests.values())
    sheet = Image.new("RGBA", (180 + cols * (cell + 6) + PAD, 70 + len(tests) * (cell + 48)), (*DARK, 255))
    draw = header(sheet, "Rig v005: testes curtos de movimento (não são animações finais)",
                  f"{m.get('test_fps', 24)} fps só para o teste. Ataque e esquiva: 0,28 s no jogo (~7 quadros). "
                  "Caminhada: ciclo de 16 quadros (a cada 2 mostrados).")
    y = 64
    for test, data in tests.items():
        draw.text((PAD, y + cell // 2 - 10), {"walk": "Caminhada", "attack": "Ataque básico",
                                             "dodge": "Esquiva"}[test], fill=TEXT, font=font(15))
        for i, (frame, entry) in enumerate(data["frames"].items()):
            x = 180 + i * (cell + 6)
            if "image" in entry:
                sheet.alpha_composite(on(entry["image"], (cell, cell)), (x, y))
            draw.text((x + 4, y + 4), f"f{int(frame):02d}", fill=TEXT, font=font(11))
            colour = SEVERITY_COLOURS.get(entry["max_severity"], MUTED)
            draw.rectangle((x, y + cell + 4, x + cell, y + cell + 10), fill=colour)
            draw.text((x, y + cell + 14), entry["max_severity"], fill=MUTED, font=font(11))
        y += cell + 48
    path = OUT / "animation_tests_v005.png"
    sheet.convert("RGB").save(path)
    return path


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--manifest", type=Path, default=RENDERS / "rig_manifest_v005.json")
    args = parser.parse_args()
    m = json.loads(args.manifest.read_text(encoding="utf-8"))
    OUT.mkdir(parents=True, exist_ok=True)
    written = [validation_sheet(m), structure_sheet(m), deformation_sheet(m)]
    neutral, metrics = neutral_compare(m)
    written += [neutral, game_scale_sheet(m), animation_sheet(m)]
    (RENDERS / "sheet_metrics_v005.json").write_text(json.dumps({"v004_vs_v005_neutral": metrics}, indent=2) + "\n",
                                                     encoding="utf-8")
    for path in written:
        size = Image.open(path).size
        print(f"[rig_sheets] {path} {size[0]}x{size[1]}")
    print(f"[rig_sheets] neutral diff {metrics}")


if __name__ == "__main__":
    main()
