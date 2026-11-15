"""Review sheets of the v006 rig (V1.2-F), built from renders/v006/rig_manifest_v006.json.

Sheets (written to comparisons/v1_2f_rig_v006/):

- ``rig_validation_sheet_v006.png``: P01-P13 in three views (side = game
  view, near 3/4, far-back 3/4 where the half-cape is), severity v005 -> v006;
- ``rig_structure_v006.png``: bones over a faded flat render, side and front;
- ``deformation_tests_v006.png``: T01-T11 close-ups, severity v005 -> v006;
- ``v005_vs_v006_neutral.png``: neutral contract masters and pixel difference;
- ``v005_vs_v006_problem_poses.png``: the poses and animation frames that
  failed in V1.2-E, v005 and v006 side by side, same cameras and light;
- ``stress_tests_v006.png``: T10 (deep squat) and T11 (arm raised + twist),
  v005 and v006 side by side;
- ``animation_tests_v006.png``: walk, attack and dodge, side and far-back;
- ``game_scale_v006.png``: finishes A/B/C x cameras 1:1, x1.5, x2, sabre N/W.

The helpers (finishes, screen, badges) are imported from the v005 module
``rig_sheets`` unchanged. Usage:

    python3 art-src/player/scripts/post/rig_sheets_v006.py
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path

from PIL import Image, ImageChops, ImageDraw, ImageStat

import rig_sheets as S5  # v005 sheets: helpers reused, nothing changed there
from contract import COMBAT_FRAME, FRAME, HD_DENSITY
from labels import font

PLAYER_DIR = Path(__file__).resolve().parents[2]
RENDERS = PLAYER_DIR / "renders" / "v006"
V005 = PLAYER_DIR / "renders" / "v005"
OUT = PLAYER_DIR / "comparisons" / "v1_2f_rig_v006"
DARK, TEXT, MUTED, PAD = S5.DARK, S5.TEXT, S5.MUTED, S5.PAD
VIEW_LABELS = {"side": "lateral (jogo)", "three_quarter": "3/4 frente", "far_back": "3/4 costas, lado da capa",
               "close_cape": "close capa (costas)", "close_cape_front": "close capa (frente)",
               "close_coat_side": "close casaca (lateral)", "close_coat_3q": "close casaca (3/4)",
               "close_collar": "close gola (frente)", "close_collar_back": "close gola (costas)",
               "close_ankle": "close tornozelo"}


def badge_pair(draw: ImageDraw.ImageDraw, xy: tuple[int, int], before: str, after: str) -> None:
    x, y = xy
    S5.badge(draw, (x, y), before)
    draw.text((x + 97, y + 2), "→", fill=TEXT, font=font(14))
    S5.badge(draw, (x + 116, y), after)


def save(sheet: Image.Image, name: str) -> Path:
    path = OUT / name
    sheet.convert("RGB").save(path)
    return path


def validation_sheet(m: dict) -> Path:
    poses = m["renders"]["poses"]
    cell, cols, views = 250, 2, ("side", "three_quarter", "far_back")
    block_w = len(views) * cell + (len(views) - 1) * 6
    block_h = cell + 78
    rows = (len(poses) + cols - 1) // cols
    sheet = Image.new("RGBA", (PAD + cols * (block_w + PAD), 70 + rows * (block_h + PAD)), (*DARK, 255))
    draw = S5.header(sheet, "Rig v006 (experimental): poses de validação P01-P13, severidade v005 → v006",
                     "Lateral (vista do jogo), 3/4 frontal e 3/4 de costas pelo lado da meia-capa (escondido na "
                     "lateral). Mesmas câmeras, luz e materiais. Nenhum valor de gameplay alterado.")
    for i, (key, paths) in enumerate(poses.items()):
        x = PAD + (i % cols) * (block_w + PAD)
        y = 66 + (i // cols) * (block_h + PAD)
        for k, view in enumerate(views):
            sheet.alpha_composite(S5.on(paths[view], (cell, cell)), (x + k * (cell + 6), y))
            draw.text((x + k * (cell + 6) + 4, y + 4), VIEW_LABELS[view], fill=MUTED, font=font(10))
        after, before = m["poses"][key], m["poses_v005"][key]
        draw.text((x, y + cell + 6), after["label"], fill=TEXT, font=font(14))
        badge_pair(draw, (x + block_w - 208, y + cell + 6), before["max_severity"], after["max_severity"])
        lines = S5.issues_text(after["issues"] + after["extra_issues"], 2) or ["sem problema Médio ou pior"]
        for k, line in enumerate(lines):
            draw.text((x, y + cell + 30 + 16 * k), "v006 " + line, fill=MUTED, font=font(11))
    return save(sheet, "rig_validation_sheet_v006.png")


def structure_sheet(m: dict) -> Path:
    views = m["renders"]["structure"]
    size, tree_w = 760, 320
    tree = S5.tree_lines(m["bones"])
    sheet = Image.new("RGBA", (PAD + 2 * (size + PAD) + tree_w, 70 + size + 90), (*DARK, 255))
    n_def = sum(b["deform"] for b in m["bones"])
    draw = S5.header(sheet, f"Rig v006: estrutura ({len(m['bones'])} ossos, {n_def} de deformação)",
                     "Mudanças sobre o v005: collar_back (novo, filho de chest); cape_arm.L_01/_02 com pivô no "
                     "ombro e o referencial de upper_arm.L. .R = direita = lado próximo da câmera lateral.")
    labelled = (*S5.LABELLED, "collar_back", "cape_arm.L_02")
    for i, view in enumerate(("side", "front")):
        x, y = PAD + i * (size + PAD), 64
        body = S5.on(views[view]["body"], (size, size))
        faded = Image.blend(Image.new("RGBA", body.size, (*DARK, 255)), body, 0.35)
        bones = Image.open(views[view]["bones"]).convert("RGBA").resize((size, size), Image.Resampling.LANCZOS)
        faded.alpha_composite(bones)
        sheet.alpha_composite(faded, (x, y))
        scale = size / 900.0
        for name in labelled:
            px, py = views[view]["labels"][name]
            colour = (250, 210, 120) if name in ("collar_back", "cape_arm.L_01", "cape_arm.L_02") else TEXT
            if 0 <= px <= 900 and 0 <= py <= 900:
                draw.text((x + px * scale + 6, y + py * scale - 6), name, fill=colour, font=font(10))
        draw.text((x, y + size + 4), "Lateral (vista do jogo)" if view == "side" else "Frontal", fill=TEXT,
                  font=font(14))
    tx = PAD + 2 * (size + PAD)
    draw.text((tx, 64), "Hierarquia", fill=TEXT, font=font(14))
    for k, line in enumerate(tree):
        draw.text((tx, 86 + 13 * k), line, fill=MUTED, font=font(10))
    ly = 64 + size + 30
    for k, (_, label, rgb) in enumerate(S5.LEGEND):
        draw.rectangle((PAD + k * 260, ly, PAD + k * 260 + 16, ly + 16), fill=rgb)
        draw.text((PAD + k * 260 + 22, ly), label, fill=TEXT, font=font(12))
    return save(sheet, "rig_structure_v006.png")


def deformation_sheet(m: dict) -> Path:
    tests = m["renders"]["tests"]
    cell, cols = 330, 4
    rows = (len(tests) + cols - 1) // cols
    block_h = cell + 84
    sheet = Image.new("RGBA", (PAD + cols * (cell + PAD), 66 + rows * (block_h + PAD)), (*DARK, 255))
    draw = S5.header(sheet, "Rig v006: testes de deformação T01-T11, severidade v005 → v006",
                     "Mesmos closes do v005 (T11 visto de costas, lado da capa). Interseções em pares de triângulos "
                     "acima do repouso; volume das malhas fechadas.")
    for i, (key, path) in enumerate(tests.items()):
        x = PAD + (i % cols) * (cell + PAD)
        y = 64 + (i // cols) * (block_h + PAD)
        after, before = m["poses"][key], m["poses_v005"][key]
        sheet.alpha_composite(S5.on(path, (cell, cell)), (x, y))
        draw.text((x, y + cell + 6), after["label"], fill=TEXT, font=font(13))
        badge_pair(draw, (x + cell - 208, y + cell + 30), before["max_severity"], after["max_severity"])
        for k, line in enumerate(S5.issues_text(after["issues"] + after["extra_issues"], 2)):
            draw.text((x, y + cell + 54 + 14 * k), "v006 " + line, fill=MUTED, font=font(11))
    return save(sheet, "deformation_tests_v006.png")


def neutral_compare() -> tuple[Path, dict[str, dict[str, float]]]:
    views = ("front", "side", "back")
    crop = (232, 120, 792, 1024)
    h = 520
    w = round((crop[2] - crop[0]) * h / (crop[3] - crop[1]))
    sheet = Image.new("RGBA", (PAD + len(views) * (w + PAD), 70 + 2 * (h + 36)), (*DARK, 255))
    draw = S5.header(sheet, "v005 × v006 em repouso (P01 Neutral): mesma câmera, escala, luz e fundo",
                     "Masters do contrato (1024 px). As correções só agem em pose: em repouso o modelo não muda.")
    metrics = {}
    for i, view in enumerate(views):
        old = Image.open(V005 / f"swordsman_rig_v005_{view}_master.png").convert("RGBA")
        new = Image.open(RENDERS / f"swordsman_rig_v006_{view}_master.png").convert("RGBA")
        diff = ImageChops.difference(old, new)
        mean = sum(ImageStat.Stat(diff).mean) / 4.0
        changed = diff.convert("L").point(lambda v: 255 if v > 8 else 0).histogram()[255]
        metrics[view] = {"mean_abs_diff": round(mean, 4), "pixels_over_8": changed}
        x = PAD + i * (w + PAD)
        for k, (label, image) in enumerate((("v005", old), ("v006", new))):
            y = 64 + k * (h + 36)
            base = Image.new("RGBA", image.size, (*DARK, 255))
            base.alpha_composite(image)
            sheet.alpha_composite(base.crop(crop).resize((w, h), Image.Resampling.LANCZOS), (x, y + 20))
            draw.text((x, y + 2), f"{label}, {view}", fill=TEXT, font=font(13))
        draw.text((x, 64 + 2 * (h + 36) - 14), f"Δ médio {mean:.3f}/255, {changed} px > 8", fill=MUTED,
                  font=font(12))
    return save(sheet, "v005_vs_v006_neutral.png"), metrics


def _compare_rows(m: dict, keys: tuple[str, ...], anim: bool, title: str, subtitle: str, name: str) -> Path:
    cell = 230
    rows: list[tuple[str, str, str, list[tuple[str, str, str]]]] = []
    for key in keys:
        imgs = m["compare_images"]
        views = list(imgs["v006"][key])
        rows.append((m["poses"][key]["label"], m["poses_v005"][key]["max_severity"], m["poses"][key]["max_severity"],
                     [(v, imgs["v005"][key][v], imgs["v006"][key][v]) for v in views]))
    if anim:
        for test, frames in (("attack", (2, 3)), ("dodge", (5,))):
            for frame in frames:
                old = m["animation_tests_v005"][test]["frames"][str(frame)]
                new = m["animation_tests"][test]["frames"][str(frame)]
                views = [v for v in old["images"] if v in new.get("images", {})]
                label = {"attack": "Teste de ataque", "dodge": "Teste de esquiva"}[test] + f", quadro {frame}"
                rows.append((label, old["max_severity"], new["max_severity"],
                             [(v, old["images"][v], new["images"][v]) for v in views]))
    max_views = max(len(r[3]) for r in rows)
    label_w = 230
    pair_w = 2 * cell + 6
    width = label_w + max_views * (pair_w + 2 * PAD) + PAD
    row_h = cell + 40
    sheet = Image.new("RGBA", (width, 70 + len(rows) * (row_h + PAD)), (*DARK, 255))
    draw = S5.header(sheet, title, subtitle)
    y = 70
    for label, before, after, views in rows:
        draw.text((PAD, y + 20), label, fill=TEXT, font=font(14))
        badge_pair(draw, (PAD, y + 48), before, after)
        x = label_w
        for view, old, new in views:
            draw.text((x, y), VIEW_LABELS.get(view, view), fill=MUTED, font=font(11))
            sheet.alpha_composite(S5.on(old, (cell, cell)), (x, y + 18))
            sheet.alpha_composite(S5.on(new, (cell, cell)), (x + cell + 6, y + 18))
            draw.text((x + 4, y + 22), "v005", fill=TEXT, font=font(12))
            draw.text((x + cell + 10, y + 22), "v006", fill=(250, 210, 120), font=font(12))
            x += pair_w + 2 * PAD
        y += row_h + PAD
    return save(sheet, name)


def problem_poses_sheet(m: dict) -> Path:
    keys = ("P05_jump_fall", "P07_crescent_slash", "P11_hurt", "P09_dodge", "P13_death", "T06_head_up",
            "T08_head_turn")
    return _compare_rows(m, keys, True, "v005 × v006: poses e quadros com problema na V1.2-E (antes | depois)",
                         "Mesmas câmeras, luz, materiais, fundo e pose. A meia-capa é mostrada pelo lado dela "
                         "(3/4 de costas e close), não pela lateral do jogo, que a esconde.",
                         "v005_vs_v006_problem_poses.png")


def stress_sheet(m: dict) -> Path:
    return _compare_rows(m, ("T10_squat", "T11_cape_arm_up"), False,
                         "Testes de estresse v005 × v006: T10 agachamento profundo, T11 braço erguido com tronco torcido",
                         "Além do que as poses do jogo pedem: coxas a 90°, joelhos a 120°; braço esquerdo a 120° com "
                         "o peito torcido 15° para o lado da espada.", "stress_tests_v006.png")


def animation_sheet(m: dict) -> Path:
    tests = m["animation_tests"]
    old = m["animation_tests_v005"]
    cell = 170
    cols = max(len(t["frames"]) for t in tests.values())
    rows = [(test, view) for test in tests for view in ("side", "far_back")]
    sheet = Image.new("RGBA", (200 + cols * (cell + 6) + PAD, 70 + len(rows) * (cell + 52)), (*DARK, 255))
    draw = S5.header(sheet, "Rig v006: testes curtos de movimento (não são animações finais)",
                     f"{m.get('test_fps', 24)} fps só para o teste, mesmas durações do v005 (ataque e esquiva: "
                     "0,28 s = 7 quadros; caminhada: 16 quadros, a cada 2). Barra = severidade v006; texto = v005 → v006.")
    names = {"walk": "Caminhada", "attack": "Ataque básico", "dodge": "Esquiva"}
    y = 64
    for test, view in rows:
        draw.text((PAD, y + cell // 2 - 18), names[test], fill=TEXT, font=font(15))
        draw.text((PAD, y + cell // 2 + 2), VIEW_LABELS[view], fill=MUTED, font=font(11))
        for i, (frame, entry) in enumerate(tests[test]["frames"].items()):
            if view not in entry.get("images", {}):
                continue
            x = 200 + i * (cell + 6)
            sheet.alpha_composite(S5.on(entry["images"][view], (cell, cell)), (x, y))
            draw.text((x + 4, y + 4), f"f{int(frame):02d}", fill=TEXT, font=font(11))
            colour = S5.SEVERITY_COLOURS.get(entry["max_severity"], MUTED)
            draw.rectangle((x, y + cell + 4, x + cell, y + cell + 10), fill=colour)
            before = old[test]["frames"].get(str(frame), {}).get("max_severity", "?")
            draw.text((x, y + cell + 14), f"{before} → {entry['max_severity']}", fill=MUTED, font=font(11))
        y += cell + 52
    return save(sheet, "animation_tests_v006.png")


def game_scale_sheet(m: dict) -> Path:
    rows = []
    for key, paths in m["renders"]["game"].items():
        combat = "attack" in key
        for tag in ("N", "W"):
            shaded = next(Path(p) for p in paths if f"_{tag}_shaded_" in p)
            flat = next(Path(p) for p in paths if f"_{tag}_flat_" in p)
            fs, fc = S5.finishes(shaded, combat), S5.finishes(flat, combat)
            for name, image in fs.items():
                image.save(shaded.with_name(shaded.stem.replace("_master", "") + f"_{name}"
                                            + ("_pixel_rim" if name == "A" else "_hd_rim") + ".png"))
            rows.append((f"{m['poses'][key]['label']} — lâmina {tag}", combat,
                         {"A": fs["A"], "B": fs["B"], "C": fc["B"]}))
    cell_w = {False: sum(round(FRAME * z) for z in S5.ZOOMS),
              True: sum(round(COMBAT_FRAME[0] * z) for z in S5.ZOOMS)}
    group_w = max(cell_w.values()) + 2 * PAD
    row_h = {False: round(FRAME * 2) + 30, True: round(COMBAT_FRAME[1] * 2) + 30}
    label_w = 250
    height = 90 + sum(row_h[c] + PAD for _, c, _ in rows)
    sheet = Image.new("RGBA", (label_w + 3 * group_w + PAD, height), (*DARK, 255))
    draw = S5.header(sheet, "Rig v006 na escala do jogo: acabamentos A, B, C × câmeras 1:1, ×1,5, ×2",
                     "Fundo da sala. ~52 px de altura em 1:1 (64×64, pivô (32, 63)); ataque na célula de combate "
                     "128×96, pivô (40, 95). Comparação externa ao jogo: a câmera do Bevy não muda. Nada é decisão.")
    for k, (_, desc) in enumerate((("A", "A pixel art (nearest)"), ("B", "B alta resolução (linear)"),
                                   ("C", "C cores chapadas (via B)"))):
        draw.text((label_w + k * group_w, 62), f"{desc}: 1:1  ×1,5  ×2", fill=TEXT, font=font(13))
    y = 90
    for label, combat, sprites in rows:
        draw.text((PAD, y + 4), label, fill=TEXT, font=font(12))
        for k, finish in enumerate(("A", "B", "C")):
            x = label_w + k * group_w
            density = 1 if finish == "A" else HD_DENSITY
            for zoom in S5.ZOOMS:
                tile = S5.screen(sprites[finish], "A" if finish == "A" else "B", zoom, density)
                sheet.alpha_composite(tile, (x, y + row_h[combat] - 24 - tile.size[1] + 20))
                x += tile.size[0] + 4
        y += row_h[combat] + PAD
    return save(sheet, "game_scale_v006.png")


def check_png(path: Path) -> dict[str, object]:
    image = Image.open(path)
    return {"size": image.size, "mode": image.mode}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--manifest", type=Path, default=RENDERS / "rig_manifest_v006.json")
    args = parser.parse_args()
    m = json.loads(args.manifest.read_text(encoding="utf-8"))
    OUT.mkdir(parents=True, exist_ok=True)
    written = [validation_sheet(m), structure_sheet(m), deformation_sheet(m)]
    neutral, metrics = neutral_compare()
    written += [neutral, problem_poses_sheet(m), stress_sheet(m), animation_sheet(m), game_scale_sheet(m)]
    (RENDERS / "sheet_metrics_v006.json").write_text(json.dumps({"v005_vs_v006_neutral": metrics}, indent=2) + "\n",
                                                     encoding="utf-8")
    for path in written:
        info = check_png(path)
        print(f"[rig_sheets_v006] {path} {info['size'][0]}x{info['size'][1]} {info['mode']}")
    print(f"[rig_sheets_v006] neutral diff {metrics}")


if __name__ == "__main__":
    main()
