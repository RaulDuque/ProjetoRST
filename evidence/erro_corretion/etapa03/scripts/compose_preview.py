"""Etapa 03 (Erro-Corretion): armour previews over the APPROVED dressed protagonist.

Fit check only (the pixel-art reduction is Prompt 04): the high-resolution armour
renders are laid over the approved sprite enlarged with nearest neighbour, in the
layer order approved in Etapa 02B:

  far_low armour < far limbs < pauldron_far < trousers/boots < shirt_torso
  < torso armour < sleeve_near < pauldron_near < hand/bracer/face/hair

The dressed layers come from evidence/erro_corretion/etapa02b/out/layers (a
partition of the approved texels; nothing is repainted). Metrics at 1x: a texel
is "covered" when >= 50 % of its 8x8 block is armour.

    python3 -I -B evidence/erro_corretion/etapa03/scripts/compose_preview.py
"""

from __future__ import annotations

import json
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[4]
OUT = ROOT / "evidence/erro_corretion/etapa03/out"
LAYERS = ROOT / "evidence/erro_corretion/etapa02b/out/layers"
TARGETS = ROOT / "evidence/erro_corretion/etapa02/out/targets.json"
IDLE_PNG = ROOT / "art-src/n3-5-fidelity/generated/candidate_106.png"
RUN_PNG = ROOT / "art-src/n3-6-animation-proof/generated/run6.png"
SUPER = 8
VIEW = 4  # preview zoom
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
BG = (52, 50, 60, 255)
IDLE_GRID = ROOT / "art-src/n3-5-fidelity/source/candidate_106.txt"
REFINED = OUT / "layers_refined"
KEEP_WITH_HAND = set("SszK")  # skin and the forearm/hand outline


def refine_idle_hand() -> None:
    """Idle only: the N4-A forearm/hand boxes also hold belt, cuff and trouser texels
    behind the forearm. Split them out (below the torso armour); skin and outline stay
    in front. Derived test files; the 02B layers are not modified."""
    rows = IDLE_GRID.read_text(encoding="utf-8").splitlines()
    hand = Image.open(LAYERS / "idle" / "hand_near.png").convert("RGBA")
    front = Image.new("RGBA", hand.size, (0, 0, 0, 0))
    behind = Image.new("RGBA", hand.size, (0, 0, 0, 0))
    px = hand.load()
    for y in range(hand.height):
        for x in range(hand.width):
            if px[x, y][3]:
                (front if rows[y][x] in KEEP_WITH_HAND else behind).putpixel((x, y), px[x, y])
    (REFINED / "idle").mkdir(parents=True, exist_ok=True)
    front.save(REFINED / "idle" / "hand_near.png")
    behind.save(REFINED / "idle" / "waist_behind_forearm.png")


def layer_path(name: str, layer: str) -> Path:
    refined = REFINED / name / f"{layer}.png"
    return refined if refined.exists() else LAYERS / name / f"{layer}.png"


def sprite_of(name: str, pose: dict) -> Image.Image:
    if name == "idle":
        return Image.open(IDLE_PNG).convert("RGBA")
    cell = pose["frame"]
    return Image.open(RUN_PNG).convert("RGBA").crop((cell * 112, 0, cell * 112 + 112, 128))


def coverage(image: Image.Image) -> set[tuple[int, int]]:
    alpha = image.getchannel("A").load()
    out = set()
    for y in range(image.height // SUPER):
        for x in range(image.width // SUPER):
            total = sum(alpha[x * SUPER + dx, y * SUPER + dy] for dy in range(SUPER) for dx in range(SUPER))
            if total >= 128 * SUPER * SUPER:
                out.add((x, y))
    return out


def opaque(image: Image.Image) -> set[tuple[int, int]]:
    alpha = image.getchannel("A").load()
    return {(x, y) for y in range(image.height) for x in range(image.width) if alpha[x, y]}


def at_view(image: Image.Image, nearest: bool) -> Image.Image:
    size = (image.width * VIEW // (SUPER if not nearest else 1), image.height * VIEW // (SUPER if not nearest else 1))
    return image.resize(size, Image.Resampling.NEAREST if nearest else Image.Resampling.LANCZOS)


def compose(name: str, size, armour: dict[str, Image.Image], with_armour: bool) -> Image.Image:
    canvas = Image.new("RGBA", (size[0] * VIEW, size[1] * VIEW), (0, 0, 0, 0))
    for item in ORDER:
        if item.startswith("@"):
            if with_armour:
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


def main() -> None:
    targets = json.loads(TARGETS.read_text())
    refine_idle_hand()
    metrics = {}
    overlays = []
    for name, pose in targets.items():
        size = tuple(pose["canvas"])
        sprite = sprite_of(name, pose)
        armour = {g: Image.open(OUT / f"armor_{name}_{g}.png").convert("RGBA")
                  for g in ("far_low", "pauldron_far", "torso", "pauldron_near")}
        whole = Image.open(OUT / f"armor_{name}_all.png").convert("RGBA")
        bare = compose(name, size, armour, with_armour=False)
        dressed = compose(name, size, armour, with_armour=True)
        original_view = at_view(sprite, nearest=True)
        bare_diff = sum(1 for a, b in zip(bare.get_flattened_data(), original_view.get_flattened_data())
                        if a != b and (a[3] or b[3]))
        shirt = opaque(Image.open(LAYERS / name / "shirt_torso.png").convert("RGBA"))
        over_shirt = coverage(armour["torso"]) | coverage(armour["pauldron_near"])
        any_armour = coverage(whole)
        body = opaque(sprite)
        exposed = shirt - over_shirt
        grown = any_armour - body
        # Armour hidden by the front layers (sleeve, hand, hair, face): how much of it shows.
        front = set()
        for layer in ("sleeve_near", "hand_near", "bracer_near", "face_skin", "hair_back", "hair_front"):
            path = layer_path(name, layer)
            if path.exists():
                front |= opaque(Image.open(path).convert("RGBA"))
        visible_armour = any_armour - front
        metrics[name] = {
            "shirt_torso_texels": len(shirt), "shirt_covered": len(shirt & over_shirt),
            "shirt_exposed": len(exposed), "armour_texels": len(any_armour),
            "armour_visible_after_front_layers": len(visible_armour),
            "silhouette_growth_texels": len(grown), "bare_reconstruction_diff_px_at_preview": bare_diff,
        }
        # Panels.
        p_orig = plate(original_view)
        p_iso = plate(at_view(whole, nearest=False))
        p_dress = plate(dressed)
        p_marks = plate(dressed)
        draw = ImageDraw.Draw(p_marks, "RGBA")
        for x, y in exposed:
            draw.rectangle([x * VIEW, y * VIEW, x * VIEW + VIEW - 1, y * VIEW + VIEW - 1], fill=(255, 40, 40, 150))
        contour(draw, shirt, (60, 220, 255, 255), 2)
        contour(draw, body, (255, 255, 255, 110), 1)
        contour(draw, body | any_armour, (255, 210, 40, 255), 2)
        panels = [(p_orig, "original aprovado (vestido)"), (p_iso, "armadura isolada (render 3D)"),
                  (p_dress, "armadura sobre o personagem vestido"),
                  (p_marks, "ciano: camisa coberta | amarelo: nova silhueta | branco: silhueta original | "
                            "vermelho: camisa ainda visivel")]
        m = metrics[name]
        text = [name, "",
                f"camisa (tronco): {m['shirt_torso_texels']} texels",
                f"  coberta: {m['shirt_covered']}",
                f"  ainda visivel: {m['shirt_exposed']}",
                f"armadura: {m['armour_texels']} texels",
                f"  visivel apos manga/mao/cabelo: {m['armour_visible_after_front_layers']}",
                f"silhueta nova (fora do corpo): {m['silhouette_growth_texels']} texels",
                f"montagem sem armadura: {bare_diff} px diferentes"]
        gap, text_w = 8, 300
        width = sum(p.width for p, _ in panels) + gap * len(panels) + text_w
        sheet = Image.new("RGBA", (width, p_orig.height + 34), (26, 26, 32, 255))
        sdraw = ImageDraw.Draw(sheet)
        x = 0
        for panel, title in panels:
            sheet.alpha_composite(panel, (x, 34))
            sdraw.multiline_text((x + 3, 2), title[:70] + ("\n" + title[70:] if len(title) > 70 else ""),
                                 fill=(255, 255, 255))
            x += panel.width + gap
        sdraw.multiline_text((x + 6, 40), "\n".join(text), fill=(235, 235, 235), spacing=4)
        sheet.save(OUT / f"fit_{name}.png")
        overlays.append((name, p_dress))
        print(name, metrics[name])
    # Four poses side by side (dressed + armour).
    width = sum(p.width for _, p in overlays) + 10 * len(overlays)
    summary = Image.new("RGBA", (width, overlays[0][1].height + 24), (26, 26, 32, 255))
    sdraw = ImageDraw.Draw(summary)
    x = 0
    for name, panel in overlays:
        summary.alpha_composite(panel, (x, 24))
        sdraw.text((x + 4, 5), name, fill=(255, 255, 255))
        x += panel.width + 10
    summary.save(OUT / "fit_summary.png")
    (OUT / "fit_metrics.json").write_text(json.dumps(metrics, indent=1))


if __name__ == "__main__":
    main()
