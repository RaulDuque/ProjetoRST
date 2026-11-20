"""Etapa 03B (Erro-Corretion): before/after previews of the refined armour.

Same composition as Etapa 03 (`etapa03/scripts/compose_preview.py`): the armour renders
(8x supersampled, Etapa 02 side camera) are laid over the APPROVED dressed sprite,
enlarged with nearest neighbour, in the approved 02B layer order:

  far_low < far limbs < pauldron_far < trousers/boots < shirt_torso (+ idle waist split)
  < torso armour < sleeve_near < pauldron_near < hand/bracer/face/hair

"Before" = Etapa 03 renders (read only); "after" = Etapa 03B renders. Metrics at 1x
(a texel is covered when >= 50 % of its 8x8 block is armour), plus the gorget texels
that stay visible after the hair/face/hand layers.

Writes: fit_<pose>_refined.png, fit_summary_refined.png, model_sheet_before_after.png,
armor_isolated_refined.png, fit_metrics_refined.json.

    python3 -I -B evidence/erro_corretion/etapa03b/scripts/compose_refined.py
"""

from __future__ import annotations

import json
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[4]
E = ROOT / "evidence/erro_corretion"
OUT = E / "etapa03b/out"
BEFORE = E / "etapa03/out"
LAYERS = E / "etapa02b/out/layers"
REFINED_LAYERS = BEFORE / "layers_refined"  # idle hand split of Etapa 03 (read only)
TARGETS = E / "etapa02/out/targets.json"
IDLE_PNG = ROOT / "art-src/n3-5-fidelity/generated/candidate_106.png"
RUN_PNG = ROOT / "art-src/n3-6-animation-proof/generated/run6.png"
SUPER = 8
VIEW = 4
GROUP_NAMES = ("far_low", "pauldron_far", "torso", "pauldron_near")
ORDER = [
    "@far_low",
    "sleeve_far", "hand_far", "bracer_far", "trousers_far", "boots_far",
    "@pauldron_far",
    "trousers", "boots", "shirt_torso", "waist_behind_forearm",
    "@torso",
    "sleeve_near",
    "@pauldron_near",
    "hand_near", "bracer_near", "face_skin", "hair_back", "hair_front",
]
FRONT_LAYERS = ("sleeve_near", "hand_near", "bracer_near", "face_skin", "hair_back", "hair_front")
BG = (52, 50, 60, 255)
SHEET_BG = (26, 26, 32, 255)
WHITE = (255, 255, 255)


def layer_path(name: str, layer: str) -> Path:
    refined = REFINED_LAYERS / name / f"{layer}.png"
    return refined if refined.exists() else LAYERS / name / f"{layer}.png"


def sprite_of(name: str, pose: dict) -> Image.Image:
    if name == "idle":
        return Image.open(IDLE_PNG).convert("RGBA")
    cell = pose["frame"]
    return Image.open(RUN_PNG).convert("RGBA").crop((cell * 112, 0, cell * 112 + 112, 128))


def coverage(image: Image.Image) -> set[tuple[int, int]]:
    small = image.getchannel("A").resize((image.width // SUPER, image.height // SUPER), Image.Resampling.BOX)
    px = small.load()
    return {(x, y) for y in range(small.height) for x in range(small.width) if px[x, y] >= 128}


def opaque(image: Image.Image) -> set[tuple[int, int]]:
    alpha = image.getchannel("A").load()
    return {(x, y) for y in range(image.height) for x in range(image.width) if alpha[x, y]}


def at_view(image: Image.Image, nearest: bool) -> Image.Image:
    div = 1 if nearest else SUPER
    size = (image.width * VIEW // div, image.height * VIEW // div)
    return image.resize(size, Image.Resampling.NEAREST if nearest else Image.Resampling.LANCZOS)


def load_armour(folder: Path, name: str) -> dict[str, Image.Image]:
    armour = {g: Image.open(folder / f"armor_{name}_{g}.png").convert("RGBA") for g in GROUP_NAMES}
    armour["all"] = Image.open(folder / f"armor_{name}_all.png").convert("RGBA")
    return armour


def compose(name: str, size, armour: dict[str, Image.Image] | None) -> Image.Image:
    canvas = Image.new("RGBA", (size[0] * VIEW, size[1] * VIEW), (0, 0, 0, 0))
    for item in ORDER:
        if item.startswith("@"):
            if armour is not None:
                canvas.alpha_composite(at_view(armour[item[1:]], nearest=False))
            continue
        path = layer_path(name, item)
        if path.exists():
            canvas.alpha_composite(at_view(Image.open(path).convert("RGBA"), nearest=True))
    return canvas


def plate(image: Image.Image) -> Image.Image:
    base = Image.new("RGBA", image.size, BG)
    base.alpha_composite(image)
    return base


def contour(draw: ImageDraw.ImageDraw, cells: set, colour, width: int = 2) -> None:
    for x, y in cells:
        X, Y = x * VIEW, y * VIEW
        if (x, y - 1) not in cells:
            draw.line([(X, Y), (X + VIEW, Y)], fill=colour, width=width)
        if (x, y + 1) not in cells:
            draw.line([(X, Y + VIEW), (X + VIEW, Y + VIEW)], fill=colour, width=width)
        if (x - 1, y) not in cells:
            draw.line([(X, Y), (X, Y + VIEW)], fill=colour, width=width)
        if (x + 1, y) not in cells:
            draw.line([(X + VIEW, Y), (X + VIEW, Y + VIEW)], fill=colour, width=width)


def metrics_for(name: str, sprite: Image.Image, armour: dict[str, Image.Image], gorget: Image.Image | None) -> dict:
    shirt = opaque(Image.open(LAYERS / name / "shirt_torso.png").convert("RGBA"))
    over_shirt = coverage(armour["torso"]) | coverage(armour["pauldron_near"])
    any_armour = coverage(armour["all"])
    front = set()
    for layer in FRONT_LAYERS:
        path = layer_path(name, layer)
        if path.exists():
            front |= opaque(Image.open(path).convert("RGBA"))
    out = {
        "shirt_torso_texels": len(shirt), "shirt_covered": len(shirt & over_shirt),
        "shirt_exposed": len(shirt - over_shirt), "armour_texels": len(any_armour),
        "armour_visible_after_front_layers": len(any_armour - front),
        "silhouette_growth_texels": len(any_armour - opaque(sprite)),
        "_exposed": sorted(shirt - over_shirt), "_shirt": shirt, "_armour": any_armour,
    }
    if gorget is not None:
        cells = coverage(gorget)
        out["gorget_texels"] = len(cells)
        out["gorget_visible_after_front_layers"] = len(cells - front)
    return out


def label(sheet: Image.Image, xy, text: str, colour=WHITE) -> None:
    ImageDraw.Draw(sheet).multiline_text(xy, text, fill=colour, spacing=3)


def side_by_side(panels: list[tuple[Image.Image, str]], gap: int = 8, head: int = 34, extra: int = 0) -> Image.Image:
    width = sum(p.width for p, _ in panels) + gap * (len(panels) - 1) + extra
    height = max(p.height for p, _ in panels) + head
    sheet = Image.new("RGBA", (width, height), SHEET_BG)
    x = 0
    for panel, title in panels:
        sheet.alpha_composite(panel, (x, head))
        label(sheet, (x + 3, 3), title)
        x += panel.width + gap
    return sheet


def public(m: dict) -> dict:
    return {k: v for k, v in m.items() if not k.startswith("_")}


def main() -> None:
    targets = json.loads(TARGETS.read_text())
    metrics: dict[str, dict] = {}
    rows_before, rows_after = [], []
    for name, pose in targets.items():
        size = tuple(pose["canvas"])
        sprite = sprite_of(name, pose)
        old, new = load_armour(BEFORE, name), load_armour(OUT, name)
        gorget = Image.open(OUT / f"armor_{name}_gorget.png").convert("RGBA")
        m_old = metrics_for(name, sprite, old, None)
        m_new = metrics_for(name, sprite, new, gorget)
        bare = compose(name, size, None)
        bare_diff = sum(1 for a, b in zip(bare.get_flattened_data(), at_view(sprite, True).get_flattened_data())
                        if a != b and (a[3] or b[3]))
        m_new["bare_reconstruction_diff_px_at_preview"] = bare_diff
        metrics[name] = {"before_etapa03": public(m_old), "after_etapa03b": public(m_new)}
        p_before, p_after = plate(compose(name, size, old)), plate(compose(name, size, new))
        p_iso = plate(at_view(new["all"], nearest=False))
        p_marks = plate(compose(name, size, new))
        draw = ImageDraw.Draw(p_marks, "RGBA")
        for x, y in m_new["_exposed"]:
            draw.rectangle([x * VIEW, y * VIEW, x * VIEW + VIEW - 1, y * VIEW + VIEW - 1], fill=(255, 40, 40, 170))
        contour(draw, m_new["_shirt"], (60, 220, 255, 255), 2)
        contour(draw, opaque(sprite) | m_new["_armour"], (255, 210, 40, 255), 2)
        panels = [(plate(at_view(sprite, True)), "original aprovado (vestido)"),
                  (p_before, "ANTES: Etapa 03"), (p_after, "DEPOIS: Etapa 03B"),
                  (p_iso, "03B: armadura isolada"),
                  (p_marks, "03B marcas: ciano = camisa do tronco\namarelo = nova silhueta\n"
                            "vermelho = camisa ainda visivel")]
        sheet = side_by_side(panels, head=48, extra=290)
        a, b = m_old, m_new
        text = [name, "", "                    antes  depois",
                f"camisa visivel:     {a['shirt_exposed']:5d}  {b['shirt_exposed']:5d}",
                f"camisa coberta:     {a['shirt_covered']:5d}  {b['shirt_covered']:5d}",
                f"armadura (texels):  {a['armour_texels']:5d}  {b['armour_texels']:5d}",
                f"visivel apos frente:{a['armour_visible_after_front_layers']:5d}  "
                f"{b['armour_visible_after_front_layers']:5d}",
                f"silhueta nova:      {a['silhouette_growth_texels']:5d}  {b['silhouette_growth_texels']:5d}",
                f"gola visivel (03B): {b['gorget_visible_after_front_layers']} de {b['gorget_texels']}",
                f"montagem sem armadura: {bare_diff} px dif."]
        label(sheet, (sheet.width - 284, 60), "\n".join(text), (235, 235, 235))
        sheet.save(OUT / f"fit_{name}_refined.png")
        rows_before.append((name, p_before))
        rows_after.append((name, p_after))
        print(name, metrics[name])
    # Summary: Etapa 03 on top, Etapa 03B below.
    width = sum(p.width for _, p in rows_after) + 10 * (len(rows_after) - 1)
    height = rows_after[0][1].height
    summary = Image.new("RGBA", (width, 2 * height + 60), SHEET_BG)
    x = 0
    for (name, before), (_, after) in zip(rows_before, rows_after, strict=True):
        summary.alpha_composite(before, (x, 24))
        summary.alpha_composite(after, (x, height + 56))
        label(summary, (x + 4, 5), f"ANTES (03) - {name}")
        label(summary, (x + 4, height + 37), f"DEPOIS (03B) - {name}")
        x += after.width + 10
    summary.save(OUT / "fit_summary_refined.png")
    # Model sheet: isolated armour, before (top) and after (bottom).
    views = (("front", "frente"), ("side_near", "lado (camera do jogo)"), ("back", "costas"), ("three_quarter", "3/4"))
    sheet = Image.new("RGBA", (900 * 4, 2 * 1000 + 70), SHEET_BG)
    for i, (view, title) in enumerate(views):
        sheet.alpha_composite(Image.open(BEFORE / f"model_{view}.png").convert("RGBA"), (i * 900, 35))
        sheet.alpha_composite(Image.open(OUT / f"model_{view}.png").convert("RGBA"), (i * 900, 1070))
        label(sheet, (i * 900 + 10, 10), f"ANTES (Etapa 03) - {title}")
        label(sheet, (i * 900 + 10, 1045), f"DEPOIS (Etapa 03B) - {title}")
    sheet.save(OUT / "model_sheet_before_after.png")
    # Isolated refined armour only (geometry without the character).
    iso = Image.new("RGBA", (900 * 4, 1035), SHEET_BG)
    for i, (view, title) in enumerate(views):
        iso.alpha_composite(Image.open(OUT / f"model_{view}.png").convert("RGBA"), (i * 900, 35))
        label(iso, (i * 900 + 10, 10), f"Etapa 03B - armadura isolada - {title}")
    iso.save(OUT / "armor_isolated_refined.png")
    (OUT / "fit_metrics_refined.json").write_text(json.dumps(metrics, indent=1))


if __name__ == "__main__":
    main()
