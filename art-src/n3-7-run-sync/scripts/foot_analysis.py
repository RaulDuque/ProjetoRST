"""N3.7: measure the feet of the approved run, frame by frame.

Input: the keyed full-resolution frames of N3.6
(`art-src/n3-6-animation-proof/frames/video_048..059.png`, same crop window
for every frame, so x is comparable) and the reduced run6 grids.

For every video frame of the cycle (24 fps):
- the feet are the connected opaque regions in the lowest 60 px band;
- a foot is in contact when its lowest pixel is within 4 px of the ground
  row of the cycle; its contact patch is its pixels in the lowest 7 rows;
- phase: no foot in contact = air; patch covering >= 55 % of the foot =
  stance; patch only at the back = heel strike; only at the front = push-off;
- body: the hip column (centre of the opaque run at the frame-0 hip row).

The video runs in place, so the planted foot slides backward on screen at
the speed the body would travel over the ground without sliding. Between two
consecutive frames where the same foot keeps contact, the patch-centre shift
is that speed. The estimate is approximate: the video is AI-generated (with
interpolated frames), it was not authored with root motion, and the
reduction rounds to whole texels.

Writes `analysis/foot_metrics.json` and the evidence
`foot_contact_analysis.png` and `foot_sliding_metrics.png`.

    python3 -B art-src/n3-7-run-sync/scripts/foot_analysis.py
"""

from __future__ import annotations

import json
import statistics
from collections import deque
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[3]
FRAMES = ROOT / "art-src/n3-6-animation-proof/frames"
N36_METRICS = ROOT / "art-src/n3-6-animation-proof/generated/metrics.json"
HERE = ROOT / "art-src/n3-7-run-sync"
EVIDENCE = ROOT / "docs/development/n3_7/evidence"
CYCLE = list(range(48, 60))
RUN6 = [48, 50, 52, 54, 56, 58]
GHOSTED = {49, 55}
VIDEO_FPS = 24
DENSITY = 2
BAND = 60
CONTACT = 4
PATCH_ROWS = 7
MAX_STEP_PX = 150
LIGHT = (226, 224, 228, 255)
INK = (40, 32, 40)


def components(alpha, width: int, ground: int) -> list[list[tuple[int, int]]]:
    seen: set[tuple[int, int]] = set()
    found = []
    top = ground - BAND
    for y in range(top, ground + 1):
        for x in range(width):
            if alpha[x, y] and (x, y) not in seen:
                queue = deque([(x, y)])
                seen.add((x, y))
                points = []
                while queue:
                    cx, cy = queue.popleft()
                    points.append((cx, cy))
                    for nx, ny in ((cx + 1, cy), (cx - 1, cy), (cx, cy + 1), (cx, cy - 1)):
                        if (0 <= nx < width and top <= ny <= ground and alpha[nx, ny]
                                and (nx, ny) not in seen):
                            seen.add((nx, ny))
                            queue.append((nx, ny))
                if len(points) > 40:
                    found.append(points)
    return found


def foot(points: list[tuple[int, int]], ground: int) -> dict:
    xs = [p[0] for p in points]
    bottom = max(p[1] for p in points)
    patch = [p[0] for p in points if p[1] >= ground - PATCH_ROWS + 1]
    item = {"x0": min(xs), "x1": max(xs), "bottom": bottom, "pixels": len(points),
            "contact": bottom >= ground - CONTACT}
    if item["contact"] and patch:
        p0, p1 = min(patch), max(patch)
        item["patch"] = [p0, p1]
        item["patch_centre"] = (p0 + p1) / 2
        cover = (p1 - p0 + 1) / (item["x1"] - item["x0"] + 1)
        if cover >= 0.55:
            item["phase"] = "stance"
        elif (p0 + p1) / 2 < (item["x0"] + item["x1"]) / 2:
            # Facing right: the heel is the left (back) end of the foot.
            item["phase"] = "heel_strike"
        else:
            item["phase"] = "push_off"
    else:
        item["phase"] = "swing"
    return item


def main() -> None:
    EVIDENCE.mkdir(parents=True, exist_ok=True)
    (HERE / "analysis").mkdir(parents=True, exist_ok=True)
    scale = json.loads(N36_METRICS.read_text(encoding="utf-8"))["video_scale"]
    images = {f: Image.open(FRAMES / f"video_{f:03d}.png").convert("RGBA") for f in [0, *CYCLE]}
    ground = max(images[f].getchannel("A").getbbox()[3] - 1 for f in CYCLE)
    stand_box = images[0].getchannel("A").getbbox()
    hip_row = stand_box[1] + round(0.45 * (stand_box[3] - stand_box[1]))

    frames = {}
    for f in CYCLE:
        image = images[f]
        alpha = image.getchannel("A").load()
        feet = sorted((foot(points, ground) for points in components(alpha, image.width, ground)),
                      key=lambda item: item["x0"])
        hip = [x for x in range(image.width) if alpha[x, hip_row]]
        planted = [item for item in feet if item["contact"]]
        phase = "air" if not planted else planted[0]["phase"]
        frames[f] = {"feet": feet, "hip_x": (hip[0] + hip[-1]) / 2 if hip else None,
                     "phase": phase, "ghosted": f in GHOSTED,
                     "bottom": image.getchannel("A").getbbox()[3] - 1}

    # Planted-foot shift between consecutive frames that keep the same foot
    # in contact (the patch moves back by less than MAX_STEP_PX).
    pairs = []
    order = CYCLE + [CYCLE[0]]
    for a, b in zip(order, order[1:], strict=False):
        fa = [x for x in frames[a]["feet"] if x["contact"]]
        fb = [x for x in frames[b]["feet"] if x["contact"]]
        if not fa or not fb:
            continue
        shift = fa[0]["patch_centre"] - fb[0]["patch_centre"]
        if 0 < shift <= MAX_STEP_PX:
            pairs.append({"from": a, "to": b, "shift_px": shift,
                          "phases": [frames[a]["phase"], frames[b]["phase"]]})
    to_units = scale / DENSITY * VIDEO_FPS
    shifts = [p["shift_px"] for p in pairs]
    # Both frames flat on the ground: the cleanest per-frame shifts.
    flat = [p["shift_px"] for p in pairs if p["phases"] == ["stance", "stance"]]
    median = statistics.median(flat)
    # Whole contact, heel strike to push-off, of each step.
    steps = []
    for start, end in ((58, 50), (53, 56)):
        if end < start:
            span = (CYCLE.index(end) + len(CYCLE)) - CYCLE.index(start)
        else:
            span = CYCLE.index(end) - CYCLE.index(start)
        travel = (frames[start]["feet"][[x["contact"] for x in frames[start]["feet"]].index(True)]
                  ["patch_centre"] - [x for x in frames[end]["feet"] if x["contact"]][0]["patch_centre"])
        steps.append({"strike": start, "push_off": end, "frames": span, "travel_px": travel,
                      "speed_units_s": round(travel / span * to_units, 1)})
    hips = [frames[f]["hip_x"] for f in CYCLE]
    metrics = {
        "method": "connected feet in the lowest 60 px; contact within 4 px of the ground row; "
                  "patch = lowest 7 rows; same foot kept between consecutive frames",
        "video_scale": scale,
        "ground_row_px": ground,
        "hip_row_px": hip_row,
        "units_per_px_per_frame": round(to_units, 4),
        "frames": {str(f): frames[f] for f in CYCLE},
        "planted_pairs": pairs,
        "stance_shift_px": {"median": median, "min": min(flat), "max": max(flat),
                            "all_pairs_min": min(shifts), "all_pairs_max": max(shifts)},
        "steps": steps,
        "reference_speed_units_s": round(statistics.mean(s["speed_units_s"] for s in steps)),
        "stance_speed_units_s": {"median": round(median * to_units, 1),
                                 "min": round(min(flat) * to_units, 1),
                                 "max": round(max(flat) * to_units, 1)},
        "hip_x_px": {"min": min(hips), "max": max(hips)},
        "hip_drift_units": round((max(hips) - min(hips)) * scale / DENSITY, 2),
        "run6_phase": {str(f): frames[f]["phase"] for f in RUN6},
        "n36_estimate_units_s": json.loads(N36_METRICS.read_text(encoding="utf-8"))[
            "planted_sole_speed_units_per_s"],
        "limits": ["AI-generated video with interpolated frames (49 and 55 ghosted)",
                   "in-place run, no root motion authored",
                   "1 video px = %.3f texel; the sprite rounds to whole texels" % scale,
                   "the foot rolls (heel to toe), so the patch centre moves even without sliding"],
    }
    (HERE / "analysis/foot_metrics.json").write_text(json.dumps(metrics, indent=1) + "\n",
                                                     encoding="utf-8")

    # --- evidence: contact analysis -------------------------------------
    crop_top = ground - 200
    panels = []
    for f in CYCLE:
        image = images[f].crop((40, crop_top, 440, ground + 6))
        plate = Image.new("RGBA", image.size, (150, 150, 156, 255))
        plate.alpha_composite(image)
        draw = ImageDraw.Draw(plate)
        draw.line([(0, ground - crop_top), (plate.width, ground - crop_top)], fill=(220, 30, 40))
        for item in frames[f]["feet"]:
            colour = (40, 200, 90) if item["contact"] else (240, 150, 40)
            draw.rectangle([item["x0"] - 40, item["bottom"] - crop_top - 45,
                            item["x1"] - 40, item["bottom"] - crop_top], outline=colour, width=2)
            if "patch" in item:
                draw.line([(item["patch"][0] - 40, ground - crop_top + 3),
                           (item["patch"][1] - 40, ground - crop_top + 3)], fill=(40, 200, 90), width=4)
        if frames[f]["hip_x"] is not None:
            hx = frames[f]["hip_x"] - 40
            draw.line([(hx, 0), (hx, 30)], fill=(60, 120, 220), width=2)
        small = plate.resize((plate.width // 2, plate.height // 2), Image.Resampling.BOX)
        panels.append((f, small))
    gap, label = 8, 30
    pw, ph = panels[0][1].size
    cols = 6
    sheet = Image.new("RGBA", (cols * (pw + gap) + gap, 2 * (ph + label + gap) + 60), LIGHT)
    draw = ImageDraw.Draw(sheet)
    names = {"stance": "apoio", "heel_strike": "contato (calcanhar)", "push_off": "impulsao (ponta)",
             "air": "voo"}
    for i, (f, panel) in enumerate(panels):
        x = gap + (i % cols) * (pw + gap)
        y = gap + (i // cols) * (ph + label + gap)
        tag = names[frames[f]["phase"]] + (" FANTASMA" if f in GHOSTED else "")
        draw.text((x, y), f"video {f}" + ("  run6" if f in RUN6 else ""), fill=INK)
        draw.text((x, y + 12), tag, fill=INK)
        sheet.alpha_composite(panel, (x, y + label))
    draw.text((gap, sheet.height - 52),
              "verde = pe em contato (barra = area de contato no chao); laranja = pe livre; "
              "azul = coluna do quadril; vermelho = chao do ciclo", fill=INK)
    draw.text((gap, sheet.height - 38),
              f"pe de apoio, passo inteiro (contato -> impulsao): {steps[0]['speed_units_s']:.0f} e "
              f"{steps[1]['speed_units_s']:.0f} u/s, referencia {metrics['reference_speed_units_s']} u/s; "
              f"quadros apoiados: {min(flat) * to_units:.0f}-{max(flat) * to_units:.0f} u/s; "
              f"quadril varia {metrics['hip_drift_units']} u (corrida no lugar)", fill=INK)
    draw.text((gap, sheet.height - 24), "estimativa aproximada: video gerado por IA, sem root motion, "
              "pe rola do calcanhar para a ponta", fill=INK)
    sheet.save(EVIDENCE / "foot_contact_analysis.png")

    # --- evidence: sliding metrics chart --------------------------------
    width, height = 900, 420
    chart = Image.new("RGBA", (width, height), LIGHT)
    draw = ImageDraw.Draw(chart)
    left, top, plot_w, plot_h = 70, 40, 520, 300
    draw.text((left, 10), "Centro da area de contato do pe de apoio (px de video), por quadro", fill=INK)
    xmin, xmax = 80, 400
    draw.rectangle([left, top, left + plot_w, top + plot_h], outline=INK)
    for v in range(xmin, xmax + 1, 80):
        y = top + plot_h - (v - xmin) / (xmax - xmin) * plot_h
        draw.text((10, y - 6), f"{v}", fill=INK)
        draw.line([(left, y), (left + plot_w, y)], fill=(200, 198, 204))
    step_x = plot_w / (len(CYCLE) - 1)
    previous = None
    for i, f in enumerate(CYCLE):
        x = left + i * step_x
        draw.text((x - 8, top + plot_h + 6), str(f), fill=INK)
        planted = [item for item in frames[f]["feet"] if item["contact"]]
        if not planted:
            draw.text((x - 8, top + plot_h - 16), "voo", fill=(160, 60, 60))
            previous = None
            continue
        value = planted[0]["patch_centre"]
        y = top + plot_h - (value - xmin) / (xmax - xmin) * plot_h
        if previous is not None and 0 < previous[2] - value <= MAX_STEP_PX:
            draw.line([previous[:2], (x, y)], fill=(60, 120, 220), width=2)
        draw.ellipse([x - 4, y - 4, x + 4, y + 4], fill=(40, 160, 80) if f not in GHOSTED else (220, 120, 40))
        previous = (x, y, value)
    draw.text((left, top + plot_h + 24), "laranja = quadro com fantasma (49, 55); linhas = mesmo pe em contato; "
              "a queda do 58 ao 50 e do 53 ao 56 e o pe recuando", fill=INK)
    draw.text((left, top + plot_h + 38), "a velocidade de jogo que nao desliza e a inclinacao dessas quedas",
              fill=INK)
    bx = 640
    draw.text((bx, 10), "Velocidade do pe de apoio (u/s)", fill=INK)
    bars = [("referencia", float(metrics["reference_speed_units_s"])),
            ("apoio min", min(flat) * to_units), ("apoio max", max(flat) * to_units)] + [(f"passo {s['strike']}-{s['push_off']}",
                                                   s["speed_units_s"]) for s in steps] + [
        ("N3.6 (antiga)", metrics["n36_estimate_units_s"]), ("jogo 310", 310.0)]
    for i, (name, value) in enumerate(bars):
        y = 40 + i * 40
        draw.text((bx, y), name, fill=INK)
        colour = (220, 30, 40) if name == "jogo 310" else (150, 150, 156) if "N3.6" in name else (60, 120, 220)
        draw.rectangle([bx, y + 14, bx + value * 0.7, y + 28], fill=colour)
        draw.text((bx + value * 0.7 + 4, y + 14), f"{value:.0f}", fill=INK)
    chart.save(EVIDENCE / "foot_sliding_metrics.png")
    print(json.dumps({k: metrics[k] for k in ("stance_shift_px", "stance_speed_units_s", "steps",
                                              "reference_speed_units_s",
                                              "hip_drift_units", "run6_phase")}, indent=1))
    for f in CYCLE:
        print(f, frames[f]["phase"], [(x["x0"], x["x1"], x.get("patch")) for x in frames[f]["feet"]])


if __name__ == "__main__":
    main()
