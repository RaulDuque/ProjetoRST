"""N3.7: idle -> run -> idle transition study.

Sources, all real (no frame is invented or interpolated):
- N3.5 idle candidate (80x128, pivot 40,118), placed on the 112x128 canvas
  by its pivot (never by the top-left corner or the opaque box);
- the video's standing frame 0 (N3.6 `stand.png`, 112x128, pivot 56,118);
- `run_start`: video frames 3, 5, 7, 9, 11, 13 (the real start, through
  the reusable pipeline at the N3.6 scale and offset);
- the approved run6.

There is no stop in the video, so run -> idle is a direct cut. The exit
frame chosen is the run6 frame whose silhouette is closest to the idle.

Three sequences are compared (same camera, same scale, guides for ground,
hip, head top and body centre):
1. direct: N3.5 idle -> run6 -> N3.5 idle;
2. video: frame 0 -> run_start -> run6 -> frame 0 (one source throughout);
3. mixed: N3.5 idle -> run_start -> run6 -> N3.5 idle.

Writes `analysis/transition_metrics.json`, `generated/transition.ron` and the evidence
`idle_run_transition.gif` and `idle_run_alignment.png`.

    python3 -B art-src/n3-7-run-sync/scripts/transition_study.py
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "art-src/n3-4-original-protagonist/scripts"))
sys.path.insert(0, str(ROOT / "art-src/player/scripts/post"))
from n34_palette import PALETTE  # noqa: E402
from resample import resize  # noqa: E402

HERE = ROOT / "art-src/n3-7-run-sync"
N35 = ROOT / "art-src/n3-5-fidelity/generated/candidate_106.png"
N35_PIVOT = (40, 118)
STAND = ROOT / "art-src/n3-6-animation-proof/generated/stand.png"
RUN6 = ROOT / "art-src/n3-6-animation-proof/generated/run6.png"
START = HERE / "pipeline/run_start/generated/run_start.png"
EVIDENCE = ROOT / "docs/development/n3_7/evidence"
CANVAS = (112, 128)
PIVOT = (56, 118)
HIP_FRACTION = 0.45
CHEST_FRACTION = 0.3
HAIR = {"W", "w", "h", "H"}
STEP = 1 / 24
NEUTRAL = (150, 150, 156, 255)
LIGHT = (226, 224, 228, 255)
INK = (40, 32, 40)
NEAREST = Image.Resampling.NEAREST
BY_COLOUR = {v: k for k, v in PALETTE.items()}


def zoom(image: Image.Image, factor: int) -> Image.Image:
    return resize(image, (image.width * factor, image.height * factor), NEAREST)


def cells(path: Path) -> list[Image.Image]:
    atlas = Image.open(path).convert("RGBA")
    return [atlas.crop((i * CANVAS[0], 0, (i + 1) * CANVAS[0], CANVAS[1]))
            for i in range(atlas.width // CANVAS[0])]


def on_canvas(image: Image.Image, pivot: tuple[int, int]) -> Image.Image:
    out = Image.new("RGBA", CANVAS, (0, 0, 0, 0))
    out.alpha_composite(image, (PIVOT[0] - pivot[0], PIVOT[1] - pivot[1]))
    return out


def measure(image: Image.Image) -> dict:
    box = image.getchannel("A").getbbox()
    top, bottom = box[1], box[3] - 1
    height = bottom - top + 1

    def centre(row: int) -> float | None:
        xs = [x for x in range(image.width) if image.getpixel((x, row))[3]]
        return (xs[0] + xs[-1]) / 2 if xs else None

    keys = [BY_COLOUR.get(image.getpixel((x, y))[:3]) for y in range(image.height)
            for x in range(image.width) if image.getpixel((x, y))[3]]
    hair = sum(1 for k in keys if k in HAIR)
    sole = [x for x in range(image.width) if image.getpixel((x, PIVOT[1]))[3]]
    return {"head_top": top, "bottom": bottom, "height": height,
            "hip_row": top + round(HIP_FRACTION * height), "hip_x": centre(top + round(HIP_FRACTION * height)),
            "chest_x": centre(top + round(CHEST_FRACTION * height)), "hair_texels": hair,
            "opaque_texels": len(keys), "sole_x": [min(sole), max(sole)] if sole else None,
            "palette": {k: keys.count(k) for k in sorted(set(keys))}}


def silhouette_difference(a: Image.Image, b: Image.Image) -> float:
    union = differ = 0
    for y in range(CANVAS[1]):
        for x in range(CANVAS[0]):
            oa, ob = a.getpixel((x, y))[3] > 0, b.getpixel((x, y))[3] > 0
            if oa or ob:
                union += 1
                differ += oa != ob
    return round(differ / union, 3)


def guides(panel: Image.Image, factor: int, rows: dict[str, int], column: float) -> Image.Image:
    draw = ImageDraw.Draw(panel)
    colours = {"chao": (220, 30, 40), "quadril": (60, 120, 220), "cabeca": (40, 160, 80)}
    for name, row in rows.items():
        draw.line([(0, row * factor), (panel.width, row * factor)], fill=colours[name])
    draw.line([(column * factor, 0), (column * factor, panel.height)], fill=(230, 160, 40))
    return panel


def main() -> None:
    EVIDENCE.mkdir(parents=True, exist_ok=True)
    idle = on_canvas(Image.open(N35).convert("RGBA"), N35_PIVOT)
    stand = cells(STAND)[0]
    start = cells(START)
    run = cells(RUN6)
    named = {"n35_idle": idle, "video_stand": stand,
             **{f"run_start_{i}": im for i, im in enumerate(start)},
             **{f"run6_{i}": im for i, im in enumerate(run)}}
    measures = {name: measure(image) for name, image in named.items()}
    exit_scores = {i: silhouette_difference(idle, run[i]) for i in range(len(run))}
    exit_frame = min(exit_scores, key=exit_scores.get)
    metrics = {
        "alignment": "every image on the 112x128 canvas by its own pivot; N3.5 shifted by "
                     f"({PIVOT[0] - N35_PIVOT[0]}, {PIVOT[1] - N35_PIVOT[1]})",
        "measures": measures,
        "silhouette_difference": {
            "n35_idle_vs_video_stand": silhouette_difference(idle, stand),
            "video_stand_vs_run_start_0": silhouette_difference(stand, start[0]),
            "n35_idle_vs_run_start_0": silhouette_difference(idle, start[0]),
            "run_start_last_vs_run6_0": silhouette_difference(start[-1], run[0]),
            "reference_run6_5_vs_run6_0": silhouette_difference(run[-1], run[0]),
            "run_start_last_vs_run6_5": silhouette_difference(start[-1], run[-1]),
            "n35_idle_vs_run6": exit_scores,
        },
        "exit_frame": exit_frame,
        "differences_n35_vs_video": {
            "head_top": measures["n35_idle"]["head_top"] - measures["video_stand"]["head_top"],
            "hip_x": (measures["n35_idle"]["hip_x"] or 0) - (measures["video_stand"]["hip_x"] or 0),
            "chest_x": (measures["n35_idle"]["chest_x"] or 0) - (measures["video_stand"]["chest_x"] or 0),
            "hair_texels": measures["n35_idle"]["hair_texels"] - measures["video_stand"]["hair_texels"],
            "opaque_texels": measures["n35_idle"]["opaque_texels"] - measures["video_stand"]["opaque_texels"],
        },
    }
    # Data for the isolated Bevy example (transition mode).
    (HERE / "generated/transition.ron").write_text(
        "(\n    idle_hold: 1.0,\n    run_frames: 12,\n"
        f"    exit_frame: {exit_frame},\n    start_clip: \"run_start\",\n)\n", encoding="utf-8")
    (HERE / "analysis/transition_metrics.json").write_text(json.dumps(metrics, indent=1) + "\n",
                                                           encoding="utf-8")

    # --- sequences --------------------------------------------------------
    def timeline(first_idle, use_start, last_idle) -> list[tuple[Image.Image, str]]:
        out = []
        out += [(first_idle[0], first_idle[1])] * 24
        if use_start:
            for image in start:
                out += [(image, "partida real (video 3-13)")] * 2
        cycle = list(range(len(run))) * 3
        for i in cycle[: 12 if use_start else 18]:
            out += [(run[i], f"corrida run6 #{i}")] * 2
        # Stop: run until the exit frame comes round, then a direct cut.
        last = cycle[(12 if use_start else 18) - 1]
        k = (last + 1) % len(run)
        while True:
            out += [(run[k], f"corrida run6 #{k}")] * 2
            if k == exit_frame:
                break
            k = (k + 1) % len(run)
        out += [(last_idle[0], last_idle[1] + " (corte direto)")] * 24
        return out

    lanes = [
        ("1 troca direta: N3.5 -> run6 -> N3.5", timeline((idle, "parado N3.5"), False, (idle, "parado N3.5"))),
        ("2 so o video: quadro 0 -> partida -> run6 -> quadro 0",
         timeline((stand, "parado video 0"), True, (stand, "parado video 0"))),
        ("3 misto: N3.5 -> partida do video -> run6 -> N3.5",
         timeline((idle, "parado N3.5"), True, (idle, "parado N3.5"))),
    ]
    length = max(len(t) for _, t in lanes)
    factor = 2
    pw, ph = CANVAS[0] * factor, CANVAS[1] * factor
    base_rows = {"chao": PIVOT[1] + 1, "quadril": measures["video_stand"]["hip_row"],
                 "cabeca": measures["video_stand"]["head_top"]}
    frames = []
    for n in range(length):
        sheet = Image.new("RGBA", (3 * (pw + 12) + 12, ph + 50), LIGHT)
        draw = ImageDraw.Draw(sheet)
        for c, (title, items) in enumerate(lanes):
            image, label = items[min(n, len(items) - 1)]
            panel = Image.new("RGBA", (pw, ph), NEUTRAL)
            panel.alpha_composite(zoom(image, factor))
            guides(panel, factor, base_rows, PIVOT[0])
            x = 12 + c * (pw + 12)
            sheet.alpha_composite(panel, (x, 30))
            draw.text((x, 4), title, fill=INK)
            draw.text((x, 16), label, fill=INK)
        draw.text((12, ph + 34), "verde = topo da cabeca (video 0); azul = quadril; vermelho = chao; "
                  "laranja = coluna do pivo. 24 fps de GIF.", fill=INK)
        frames.append(sheet.convert("P", palette=Image.Palette.ADAPTIVE))
    frames[0].save(EVIDENCE / "idle_run_transition.gif", save_all=True, append_images=frames[1:],
                   loop=0, duration=round(STEP * 1000), disposal=2)

    # --- alignment sheet ----------------------------------------------------
    factor = 3
    panels = []
    for name, image, title in (("n35_idle", idle, "parado N3.5\n(pivo 40,118 -> 56,118)"),
                               ("video_stand", stand, "parado video 0"),
                               ("run_start_0", start[0], "partida #0 (video 3)"),
                               ("run_start_5", start[-1], "partida #5 (video 13)"),
                               ("run6_0", run[0], "run6 #0 (video 48)"),
                               (f"run6_{exit_frame}", run[exit_frame], f"run6 #{exit_frame}\nsaida mais proxima")):
        panel = Image.new("RGBA", (pw * 3 // 2, ph * 3 // 2), NEUTRAL)
        panel.alpha_composite(zoom(image, factor))
        guides(panel, factor, base_rows, PIVOT[0])
        m = measures[name]
        panels.append((title, panel, m))
    onion = Image.new("RGBA", (CANVAS[0] * factor, CANVAS[1] * factor), NEUTRAL)
    for image, tint in ((idle, (255, 80, 80)), (stand, (80, 140, 255))):
        layer = Image.new("RGBA", CANVAS, (0, 0, 0, 0))
        for y in range(CANVAS[1]):
            for x in range(CANVAS[0]):
                if image.getpixel((x, y))[3]:
                    layer.putpixel((x, y), (*tint, 120))
        onion.alpha_composite(zoom(layer, factor))
    guides(onion, factor, base_rows, PIVOT[0])
    panels.append(("sobreposicao\nvermelho N3.5 / azul video 0", onion, None))
    gap = 12
    width = sum(p.width for _, p, _ in panels) + gap * (len(panels) + 1)
    out = Image.new("RGBA", (width, panels[0][1].height + 130), LIGHT)
    draw = ImageDraw.Draw(out)
    x = gap
    for title, panel, m in panels:
        for i, text in enumerate(title.split("\n")):
            draw.text((x, 4 + 12 * i), text, fill=INK)
        out.alpha_composite(panel, (x, 30))
        if m:
            info = [f"topo {m['head_top']}  chao {m['bottom']}", f"quadril x {m['hip_x']}  peito x {m['chest_x']}",
                    f"cabelo {m['hair_texels']}  total {m['opaque_texels']}"]
            for i, text in enumerate(info):
                draw.text((x, 36 + panel.height + 12 * i), text, fill=INK)
        x += panel.width + gap
    sd = metrics["silhouette_difference"]
    draw.text((gap, out.height - 40),
              f"diferenca de silhueta: N3.5 x video 0 = {sd['n35_idle_vs_video_stand']}; video 0 x partida #0 = "
              f"{sd['video_stand_vs_run_start_0']}; partida #5 x run6 #0 = {sd['run_start_last_vs_run6_0']}; "
              f"melhor saida run6 #{exit_frame} = {exit_scores[exit_frame]}", fill=INK)
    draw.text((gap, out.height - 26), "verde = topo da cabeca do video 0; azul = quadril; vermelho = chao; "
              "laranja = coluna do pivo (56)", fill=INK)
    out.save(EVIDENCE / "idle_run_alignment.png")
    print(json.dumps({k: metrics[k] for k in ("silhouette_difference", "exit_frame",
                                              "differences_n35_vs_video")}, indent=1))
    for name in ("n35_idle", "video_stand", "run_start_0", "run_start_5", "run6_0"):
        m = measures[name]
        print(name, {k: m[k] for k in ("head_top", "bottom", "hip_x", "chest_x", "hair_texels", "sole_x")})


if __name__ == "__main__":
    main()
