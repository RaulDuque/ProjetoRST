"""N3.7: run synchronization study (scenarios A-E) on the approved run6.

Reads the approved N3.6 atlas (`run6.png`, never written here), the N3.5
idle candidate and `analysis/foot_metrics.json`.

A planted foot is still in the world when the body moves at
`rate * reference`, where `reference` is the measured backward speed of the
planted foot at the video's own timing and `rate` the playback factor. The
slide is `|speed - rate * reference|`.

Scenarios (all on run6, the same six frames):
- A original: 12 fps, 310 u/s (the game's PLAYER_RUN_SPEED, not changed);
- B faster clip: rate = 310 / reference, 310 u/s;
- C slower travel: 12 fps, reference u/s (isolated example only);
- D moderate: rate 1.5 (18 fps), 1.5 * reference u/s (experimental speed);
- E longer stride: documented cost only, no frame is generated.

Owner's extra test: run6 at the game's 310 u/s with 12, 15, 18 and 23.7
fps (`game_speed_steps`), so only the cadence changes.

Writes `generated/sync.ron` (read by the isolated Bevy example) and the
evidence `run_baseline.png`, `run_12fps.gif`, `run_speed_variant.gif`,
`run_speed_comparison.png`, and for the fps test `run_fps_comparison.gif`,
`run_fps_comparison.png`, `run_15fps.gif` and `run_15fps_in_place.gif`.

    python3 -B art-src/n3-7-run-sync/scripts/sync_study.py
"""

from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "art-src/player/scripts/post"))
from resample import resize  # noqa: E402

HERE = ROOT / "art-src/n3-7-run-sync"
RUN = ROOT / "art-src/n3-6-animation-proof"
N35 = ROOT / "art-src/n3-5-fidelity/generated/candidate_106.png"
EVIDENCE = ROOT / "docs/development/n3_7/evidence"
CANVAS = (112, 128)
PIVOT = (56, 118)
DENSITY = 2
RUN6 = [48, 50, 52, 54, 56, 58]
BASE_FPS = 12.0
GAME_SPEED = 310.0
MODERATE_RATE = 1.5
# Owner's extra test: playback rates at the game's speed (12 / 15 / 18 fps,
# and the no-slide rate of scenario B).
FPS_TEST = [12.0, 15.0, 18.0]
NEUTRAL = (150, 150, 156, 255)
LIGHT = (226, 224, 228, 255)
LANE_BG = (58, 52, 62, 255)
INK = (40, 32, 40)
NEAREST = Image.Resampling.NEAREST
GIF_STEP = 0.02
GIF_SECONDS = 2.0
LANE_UNITS = 360.0
TICK_UNITS = 20.0


def zoom(image: Image.Image, factor: int) -> Image.Image:
    return resize(image, (image.width * factor, image.height * factor), NEAREST)


def on(image: Image.Image, background) -> Image.Image:
    plate = Image.new("RGBA", image.size, background)
    plate.alpha_composite(image.convert("RGBA"))
    return plate


def frames() -> list[Image.Image]:
    atlas = Image.open(RUN / "generated/run6.png").convert("RGBA")
    return [atlas.crop((i * CANVAS[0], 0, (i + 1) * CANVAS[0], CANVAS[1])) for i in range(len(RUN6))]


def contact_patch(image: Image.Image) -> float | None:
    """Centre column of the opaque texels on the two lowest rows above the pivot."""
    xs = [x for y in (PIVOT[1] - 1, PIVOT[1]) for x in range(image.width) if image.getpixel((x, y))[3]]
    return (min(xs) + max(xs)) / 2 if xs else None


def scenarios(reference: float) -> list[dict]:
    return [
        {"id": "A", "name": "original", "rate": 1.0, "speed": GAME_SPEED,
         "note": "12 fps, velocidade do jogo"},
        {"id": "B", "name": "clip faster", "rate": round(GAME_SPEED / reference, 3), "speed": GAME_SPEED,
         "note": "clipe acelerado ate o pe parar, velocidade do jogo"},
        {"id": "C", "name": "slower travel", "rate": 1.0, "speed": float(reference),
         "note": "12 fps, deslocamento da passada (so no exemplo)"},
        {"id": "D", "name": "moderate", "rate": MODERATE_RATE, "speed": round(MODERATE_RATE * reference, 1),
         "note": "clipe 1,5x e deslocamento experimental"},
    ]


def game_speed_steps(reference: float) -> list[dict]:
    steps = [{"id": f"{fps:.0f}fps", "name": "at game speed", "rate": fps / BASE_FPS,
              "speed": GAME_SPEED, "note": "na velocidade do jogo (310 u/s)"} for fps in FPS_TEST]
    rate = round(GAME_SPEED / reference, 3)
    steps.append({"id": f"{BASE_FPS * rate:.1f}fps", "name": "at game speed", "rate": rate,
                  "speed": GAME_SPEED, "note": "na velocidade do jogo (310 u/s), = cenario B"})
    return steps


def lane(images, patches, scenario, reference, t_end, strobe=False) -> Image.Image:
    """One fixed-camera lane: ground ticks, the body at `speed`, footprints."""
    width = int(LANE_UNITS * DENSITY)
    height = CANVAS[1] + 40
    ground_y = 20 + PIVOT[1] + 1
    out = Image.new("RGBA", (width, height), LANE_BG)
    draw = ImageDraw.Draw(out)
    draw.line([(0, ground_y), (width, ground_y)], fill=(120, 112, 124))
    for i in range(int(LANE_UNITS / TICK_UNITS) + 1):
        x = i * TICK_UNITS * DENSITY
        draw.line([(x, ground_y), (x, ground_y + 6)], fill=(120, 112, 124))
    fps = BASE_FPS * scenario["rate"]
    start = 30.0

    def state(t: float) -> tuple[int, float]:
        index = int(t * fps) % len(images)
        world = (start + scenario["speed"] * t) % LANE_UNITS
        return index, world

    # Footprints: world x of the contact patch of every shown contact frame
    # since the start (one dot per displayed frame).
    t = 0.0
    shown = []
    while t <= t_end + 1e-9:
        index, world = state(t)
        if patches[index] is not None:
            foot = world + (patches[index] - PIVOT[0]) / DENSITY
            shown.append((t, foot % LANE_UNITS))
        t += 1.0 / fps if strobe else GIF_STEP
    for when, foot in shown:
        age = t_end - when
        if age > 0.6:
            continue
        x = foot * DENSITY
        shade = int(255 - age * 300)
        draw.rectangle([x - 1, ground_y + 2, x + 1, ground_y + 4], fill=(90, 230, 120, max(60, shade)))
    if strobe:
        t = 0.0
        while t <= t_end + 1e-9:
            index, world = state(t)
            ghost = images[index].copy()
            ghost.putalpha(images[index].getchannel("A").point(lambda a: 90 if a else 0))
            out.alpha_composite(ghost, (int(world * DENSITY) - PIVOT[0], 20))
            t += 1.0 / fps
    index, world = state(t_end)
    out.alpha_composite(images[index], (int(world * DENSITY) - PIVOT[0], 20))
    if not strobe:
        # Wrap-around copy so the body never pops out at the edge.
        out.alpha_composite(images[index], (int(world * DENSITY) - PIVOT[0] - width, 20))
    slip = abs(scenario["speed"] - scenario["rate"] * reference)
    phases = ["apoio", "impulsao", "voo", "apoio", "impulsao", "contato"]
    draw.text((6, 4), f"{scenario['id']} {scenario['note']}", fill=(236, 232, 238))
    draw.text((6, height - 14),
              f"fps {fps:.1f}  fator {scenario['rate']:.2f}  vel {scenario['speed']:.0f} u/s  "
              f"x {world:5.1f}  fase {phases[index]}  deslizamento {slip:.0f} u/s "
              f"({slip / scenario['speed'] * 100:.0f} %)", fill=(236, 232, 238))
    return out


def main() -> None:
    EVIDENCE.mkdir(parents=True, exist_ok=True)
    (HERE / "generated").mkdir(parents=True, exist_ok=True)
    metrics = json.loads((HERE / "analysis/foot_metrics.json").read_text(encoding="utf-8"))
    reference = float(metrics["reference_speed_units_s"])
    images = frames()
    patches = [contact_patch(image) if metrics["run6_phase"][str(f)] != "air" else None
               for image, f in zip(images, RUN6, strict=True)]
    plan = scenarios(reference)
    fps_steps = game_speed_steps(reference)

    # --- data for the isolated Bevy example -----------------------------
    phase_keys = [metrics["run6_phase"][str(f)] for f in RUN6]
    lines = ["(\n", f"    reference_speed: {reference:.1f},\n", f"    game_speed: {GAME_SPEED:.1f},\n",
             f"    base_fps: {BASE_FPS:.1f},\n", "    scenarios: [\n"]
    for s in plan:
        lines.append(f'        (id: "{s["id"]}", name: "{s["name"]}", rate: {s["rate"]:.3f}, '
                     f"speed: {s['speed']:.1f}),\n")
    lines.append("    ],\n    game_speed_steps: [\n")
    for s in fps_steps:
        lines.append(f'        (id: "{s["id"]}", name: "{s["name"]}", rate: {s["rate"]:.3f}, '
                     f"speed: {s['speed']:.1f}),\n")
    lines.append("    ],\n    run6_phase: [" + ", ".join(f'"{p}"' for p in phase_keys) + "],\n)\n")
    (HERE / "generated/sync.ron").write_text("".join(lines), encoding="utf-8")

    # --- baseline sheet --------------------------------------------------
    gap, label = 12, 40
    n35 = Image.open(N35).convert("RGBA")
    panels = [("N3.5 parado 80x128\npivo (40,118)", on(zoom(n35, 3), NEUTRAL))]
    panels += [(f"run6 #{i} (video {f})\n{phase_keys[i]}", on(zoom(img, 3), NEUTRAL))
               for i, (img, f) in enumerate(zip(images, RUN6, strict=True))]
    width = sum(p.width for _, p in panels) + gap * (len(panels) + 1)
    sheet = Image.new("RGBA", (width, panels[0][1].height + label + gap * 2 + 60), LIGHT)
    draw = ImageDraw.Draw(sheet)
    x = gap
    for title, panel in panels:
        for i, text in enumerate(title.split("\n")):
            draw.text((x, 4 + 12 * i), text, fill=INK)
        sheet.alpha_composite(panel, (x, label))
        x += panel.width + gap
    files = [N35, RUN / "generated/run6.png", RUN / "generated/run.ron"]
    for i, path in enumerate(files):
        digest = hashlib.sha256(path.read_bytes()).hexdigest()
        draw.text((gap, sheet.height - 54 + 14 * i), f"{path.relative_to(ROOT)}  sha256 {digest}", fill=INK)
    sheet.save(EVIDENCE / "run_baseline.png")

    # --- baseline GIF at 12 fps -------------------------------------------
    shown = [on(zoom(img, 3), NEUTRAL).convert("P", palette=Image.Palette.ADAPTIVE) for img in images]
    shown[0].save(EVIDENCE / "run_12fps.gif", save_all=True, append_images=shown[1:], loop=0,
                  duration=round(1000 / BASE_FPS), disposal=2)

    # --- four lanes, same camera, same scale, same frames -----------------
    gif_frames = []
    steps = round(GIF_SECONDS / GIF_STEP)
    for n in range(steps):
        t = n * GIF_STEP
        lanes = [lane(images, patches, s, reference, t) for s in plan]
        canvas = Image.new("RGBA", (lanes[0].width, sum(l.height for l in lanes) + 4 * len(lanes)), LIGHT)
        y = 0
        for l in lanes:
            canvas.alpha_composite(l, (0, y))
            y += l.height + 4
        gif_frames.append(canvas.convert("P", palette=Image.Palette.ADAPTIVE))
    gif_frames[0].save(EVIDENCE / "run_speed_variant.gif", save_all=True, append_images=gif_frames[1:],
                       loop=0, duration=round(GIF_STEP * 1000), disposal=2)

    # --- strobe comparison: one cycle of every scenario -------------------
    rows = []
    for s in plan:
        cycle = len(images) / (BASE_FPS * s["rate"])
        rows.append(lane(images, patches, s, reference, cycle - 1e-6, strobe=True))
    notes = [
        f"Referencia medida: o pe de apoio recua {reference:.0f} u/s no video (N3.6 estimava 78).",
        "Pegadas verdes: posicao no mundo da area de contato em cada quadro mostrado. Pegadas juntas = pe parado;",
        "pegadas espalhadas = pe deslizando. Mesma camera fixa, mesma escala (1 texel = 1 px), mesmos 6 quadros.",
        f"E (passada maior): a 12 fps e 310 u/s cada passo cobre {GAME_SPEED * 0.25:.0f} u ({GAME_SPEED * 0.25 * DENSITY:.0f} texels);",
        f"hoje cobre {reference * 0.25:.0f} u. Exigiria um novo video de corrida com passada ~2x, nao gerado nesta etapa.",
    ]
    out = Image.new("RGBA", (rows[0].width, sum(r.height + 6 for r in rows) + 16 * len(notes) + 10), LIGHT)
    y = 0
    for r in rows:
        out.alpha_composite(r, (0, y))
        y += r.height + 6
    draw = ImageDraw.Draw(out)
    for i, text in enumerate(notes):
        draw.text((6, y + 4 + 16 * i), text, fill=INK)
    out.save(EVIDENCE / "run_speed_comparison.png")
    # --- owner's fps test: the same frames at 310 u/s, only the cadence ---
    def lanes_gif(items, seconds, path) -> None:
        shown = []
        for n in range(round(seconds / GIF_STEP)):
            t = n * GIF_STEP
            rows = [lane(images, patches, s, reference, t) for s in items]
            canvas = Image.new("RGBA", (rows[0].width, sum(r.height + 4 for r in rows)), LIGHT)
            y = 0
            for r in rows:
                canvas.alpha_composite(r, (0, y))
                y += r.height + 4
            shown.append(canvas.convert("P", palette=Image.Palette.ADAPTIVE))
        shown[0].save(path, save_all=True, append_images=shown[1:], loop=0,
                      duration=round(GIF_STEP * 1000), disposal=2)

    lanes_gif(fps_steps, GIF_SECONDS, EVIDENCE / "run_fps_comparison.gif")
    fifteen = [s for s in fps_steps if s["id"] == "15fps"]
    lanes_gif(fifteen, 4.0, EVIDENCE / "run_15fps.gif")
    strobes = [lane(images, patches, s, reference, len(images) / (BASE_FPS * s["rate"]) - 1e-6, strobe=True)
               for s in fps_steps]
    notes = ["Mesmos 6 quadros aprovados, mesma velocidade (310 u/s), mesma camera e escala: so muda a cadencia.",
             "Exposicao multipla de um ciclo; pegadas verdes espalhadas = pe deslizando.",
             "Nenhuma velocidade foi escolhida: avaliacao do proprietario."]
    out = Image.new("RGBA", (strobes[0].width, sum(r.height + 6 for r in strobes) + 16 * len(notes) + 10), LIGHT)
    y = 0
    for r in strobes:
        out.alpha_composite(r, (0, y))
        y += r.height + 6
    draw = ImageDraw.Draw(out)
    for i, text in enumerate(notes):
        draw.text((6, y + 4 + 16 * i), text, fill=INK)
    out.save(EVIDENCE / "run_fps_comparison.png")
    # In place at exactly 15 fps: GIF delays are whole centiseconds, so the
    # six frames alternate 70/70/60 ms (400 ms per cycle = 15 fps on average).
    in_place = [on(zoom(img, 3), NEUTRAL).convert("P", palette=Image.Palette.ADAPTIVE) for img in images]
    in_place[0].save(EVIDENCE / "run_15fps_in_place.gif", save_all=True, append_images=in_place[1:], loop=0,
                     duration=[70, 70, 60, 70, 70, 60], disposal=2)

    print(json.dumps({"steps": [{**s, "fps": round(BASE_FPS * s["rate"], 2),
                                 "slip": round(abs(s["speed"] - s["rate"] * reference), 1),
                                 "slip_pct": round(abs(s["speed"] - s["rate"] * reference) / s["speed"] * 100, 1),
                                 "cycle_s": round(len(images) / (BASE_FPS * s["rate"]), 3)}
                                for s in fps_steps]}, indent=1))
    print(json.dumps({"reference": reference, "patches": patches,
                      "scenarios": [{**s, "fps": BASE_FPS * s["rate"],
                                     "slip": round(abs(s["speed"] - s["rate"] * reference), 1)}
                                    for s in plan]}, indent=1))


if __name__ == "__main__":
    main()
