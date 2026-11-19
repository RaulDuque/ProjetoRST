"""N3.8 evidence from the real game captures (no new art, no edited pixel).

Input: ``target/n3_8_production_run/<art>-<view>/`` written by
``cargo run --example n3_8_production_run`` (``n3-405``, ``production-405``,
``n3-360``): one 1280x720 window capture per 20 ms update and ``frames.log``
(clip, frame, placeholder, flip, player x/y, camera x/y).

Output: ``docs/development/n3_8/evidence/`` (GIFs at 2 cs per frame = real
time, PNG sheets) and ``measurements.json``.

Usage (from the project root): ``python3 art-src/n3-8-production-run/scripts/make_evidence.py``
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[3]
CAPTURES = ROOT / "target/n3_8_production_run"
OUT = ROOT / "docs/development/n3_8/evidence"
RUN6 = ROOT / "art-src/n3-6-animation-proof/generated/run6.png"
N37_GIF = ROOT / "docs/development/n3_7/evidence/run_15fps_in_place.gif"
WINDOW = (1280, 720)
HALF_BODY = 23.0  # PLAYER_SIZE.1 / 2: the sole line is 23 u under the body centre
CROP_W, CROP_UP, CROP_DOWN = 150.0, 78.0, 14.0  # world units around the soles
CELL, PIVOT = (112, 128), (56, 118)
GIF_CS = 2  # one 20 ms update per GIF frame

LINE = re.compile(
    r"^(?P<seq>\w+) (?P<n>\d+) update=\d+ clip=(?P<clip>\S+) frame=(?P<frame>\d+) "
    r"placeholder=(?P<ph>\S+) flip=(?P<flip>\w+) x=(?P<x>\S+) vx=(?P<vx>\S+) "
    r"state=(?P<state>\w+) y=(?P<y>\S+) cam=(?P<cx>[^,]+),(?P<cy>\S+)$"
)


def read_log(kind: str) -> list[dict]:
    rows = []
    for line in (CAPTURES / kind / "frames.log").read_text().splitlines():
        m = LINE.match(line)
        if m:
            d = m.groupdict()
            for k in ("x", "vx", "y", "cx", "cy"):
                d[k] = float(d[k])
            d["n"], d["frame"] = int(d["n"]), int(d["frame"])
            d["flip"] = d["flip"] == "true"
            rows.append(d)
    if not rows:
        sys.exit(f"no frames in {kind}/frames.log: run the example first")
    return rows


def view_height(kind: str) -> float:
    return 360.0 if kind.endswith("360") else 405.0


def to_screen(kind: str, wx: float, wy: float, cx: float, cy: float) -> tuple[float, float]:
    s = WINDOW[1] / view_height(kind)
    return WINDOW[0] / 2 + (wx - cx) * s, WINDOW[1] / 2 - (wy - cy) * s


def crop(kind: str, row: dict, scale: int = 2) -> Image.Image:
    """Window around the player's soles; the sole line is drawn as a guide."""
    im = Image.open(CAPTURES / kind / row["seq"] / f"{row['n']:04d}.png").convert("RGB")
    s = WINDOW[1] / view_height(kind)
    sole = row["y"] - HALF_BODY
    px, py = to_screen(kind, row["x"], sole, row["cx"], row["cy"])
    box = (
        round(px - CROP_W / 2 * s),
        round(py - CROP_UP * s),
        round(px + CROP_W / 2 * s),
        round(py + CROP_DOWN * s),
    )
    c = im.crop(box)
    if scale != 1:
        c = c.resize((c.width * scale // 2, c.height * scale // 2), Image.NEAREST)
    return c


def label(im: Image.Image, text: str) -> Image.Image:
    out = Image.new("RGB", (im.width, im.height + 14), (12, 10, 16))
    out.paste(im, (0, 14))
    ImageDraw.Draw(out).text((3, 1), text, fill=(235, 200, 110))
    return out


def gif(frames: list[Image.Image], name: str, cs: int = GIF_CS) -> None:
    pal = [f.convert("P", palette=Image.ADAPTIVE, colors=128) for f in frames]
    pal[0].save(OUT / name, save_all=True, append_images=pal[1:], duration=cs * 10, loop=0, disposal=1)


def sequence(kind: str, rows: list[dict], seq: str) -> list[dict]:
    return [r for r in rows if r["seq"] == seq]


def caption(r: dict) -> str:
    ph = f" placeholder:{r['ph']}" if r["ph"] != "-" else ""
    side = "L" if r["flip"] else "R"
    return f"{r['state']} {r['clip']}#{r['frame']} {side} vx={r['vx']:.0f}{ph}"


def timing(rows: list[dict]) -> dict:
    run = [r for r in rows if r["seq"] == "run_right"]
    frames = [r["frame"] for r in run]
    changes = sum(1 for a, b in zip(frames, frames[1:]) if a != b)
    seconds = (len(frames) - 1) * 0.02
    holds, count = [], 1
    for a, b in zip(frames, frames[1:]):
        if a == b:
            count += 1
        else:
            holds.append(count)
            count = 1
    inner = holds[1:]  # the first hold is cut by the start of the recording
    return {
        "updates": len(frames),
        "seconds": round(seconds, 3),
        "frame_changes": changes,
        "fps_measured": round(changes / seconds, 3) if seconds else None,
        "hold_updates_20ms": inner,
        "mean_hold_ms": round(sum(inner) / len(inner) * 20, 2) if inner else None,
        "speeds": sorted({r["vx"] for r in run}),
    }


def pixel_check_360(rows: list[dict]) -> dict:
    """At 360 one texel is one screen pixel: compare the drawn sprite with run6."""
    run6 = Image.open(RUN6).convert("RGBA")
    results = []
    for r in [r for r in rows if r["seq"] == "run_right"][:24]:
        im = Image.open(CAPTURES / "n3-360" / r["seq"] / f"{r['n']:04d}.png").convert("RGB")
        px, py = to_screen("n3-360", r["x"], r["y"] - HALF_BODY, r["cx"], r["cy"])
        best = None
        # The camera is not on whole pixels: try the four roundings.
        for ox in (int(px - PIVOT[0]), int(px - PIVOT[0]) + 1):
            for oy in (int(py - PIVOT[1]), int(py - PIVOT[1]) + 1):
                cell = run6.crop((r["frame"] * CELL[0], 0, (r["frame"] + 1) * CELL[0], CELL[1]))
                same = total = 0
                for y in range(CELL[1]):
                    for x in range(CELL[0]):
                        p = cell.getpixel((x, y))
                        if p[3] < 255:
                            continue
                        total += 1
                        q = im.getpixel((ox + x, oy + y))
                        if max(abs(p[i] - q[i]) for i in range(3)) <= 2:
                            same += 1
                if best is None or same > best[0]:
                    best = (same, total, ox, oy)
        results.append({"n": r["n"], "frame": r["frame"], "match": best[0], "opaque": best[1]})
    exact = sum(1 for x in results if x["match"] == x["opaque"])
    ratio = sum(x["match"] for x in results) / sum(x["opaque"] for x in results)
    return {"captures": len(results), "exact_frames": exact, "pixel_match_ratio": round(ratio, 5), "detail": results}


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    n3, prod, n360 = read_log("n3-405"), read_log("production-405"), read_log("n3-360")
    measurements = {"timing_n3_405": timing(n3), "timing_production_405": timing(prod)}

    # Motion GIFs (real time, 405, the game's camera).
    names = {
        "idle_to_run": "n38_idle_to_run.gif",
        "run_right": "n38_run_right_15fps.gif",
        "run_left": "n38_run_left_15fps.gif",
        "run_to_idle": "n38_run_to_idle.gif",
        "run_turn": "n38_run_turn.gif",
        "run_to_jump": "n38_run_to_jump.gif",
        "run_to_attack": "n38_run_to_attack.gif",
        "run_to_guard": "n38_run_to_guard.gif",
        "run_to_dodge": "n38_run_to_dodge.gif",
        "run_with_cape": "n38_run_with_cape.gif",
    }
    for seq, name in names.items():
        gif([label(crop("n3-405", r), caption(r)) for r in sequence("n3-405", n3, seq)], name)

    # Production V2.2 vs experimental N3, same script, same updates.
    pairs = []
    for a, b in zip(sequence("production-405", prod, "run_right"), sequence("n3-405", n3, "run_right")):
        ca, cb = label(crop("production-405", a), "production V2.2"), label(crop("n3-405", b), "experimental N3 (15 fps)")
        both = Image.new("RGB", (ca.width + cb.width + 4, ca.height), (0, 0, 0))
        both.paste(ca, (0, 0))
        both.paste(cb, (ca.width + 4, 0))
        pairs.append(both)
    gif(pairs, "n38_production_vs_n3_run.gif")

    # Camera 405 (game) vs 360 (experimental view), same update.
    a, b = sequence("n3-405", n3, "run_right")[10], sequence("n3-360", n360, "run_right")[10]
    ca = label(crop("n3-405", a, scale=2), "camera 405 (game): 0.889 px/texel")
    cb = label(crop("n3-360", b, scale=2), "camera 360 (proof only): 1 px/texel")
    sheet = Image.new("RGB", (ca.width + cb.width + 4, max(ca.height, cb.height)), (0, 0, 0))
    sheet.paste(ca, (0, 0))
    sheet.paste(cb, (ca.width + 4, 0))
    sheet.save(OUT / "n38_camera_405_vs_360.png")
    Image.open(CAPTURES / "n3-405" / "idle_right.png").save(OUT / "n38_game_405_idle.png")
    Image.open(CAPTURES / "n3-405" / "run_right" / "0010.png").save(OUT / "n38_game_405_run.png")
    Image.open(CAPTURES / "n3-405" / "run_to_jump" / "0020.png").save(OUT / "n38_game_405_jump_placeholder.png")

    # Feet: one crop per run frame (first time each is shown), sole line marked.
    seen = {}
    for r in sequence("n3-405", n3, "run_right"):
        seen.setdefault(r["frame"], r)
    tiles = []
    for f in range(6):
        c = crop("n3-405", seen[f], scale=4)
        d = ImageDraw.Draw(c)
        sole_y = round(CROP_UP / (CROP_UP + CROP_DOWN) * c.height)
        d.line([(0, sole_y), (c.width, sole_y)], fill=(80, 220, 120), width=1)
        tiles.append(label(c, f"run #{f} (video {48 + 2 * f})"))
    feet = Image.new("RGB", (sum(t.width for t in tiles) + 4 * 5, tiles[0].height), (0, 0, 0))
    x = 0
    for t in tiles:
        feet.paste(t, (x, 0))
        x += t.width + 4
    feet.save(OUT / "n38_feet_alignment.png")

    # Comparison with N3.7: the same six frames drawn by the game at 360.
    measurements["pixel_check_360_vs_run6"] = pixel_check_360(n360)
    run6 = Image.open(RUN6).convert("RGBA")
    ref = Image.new("RGBA", (run6.width, run6.height), (40, 36, 48, 255))
    ref.alpha_composite(run6)
    game = Image.new("RGB", (run6.width, run6.height), (0, 0, 0))
    seen360 = {}
    for r in sequence("n3-360", n360, "run_right"):
        seen360.setdefault(r["frame"], r)
    for f, r in sorted(seen360.items()):
        im = Image.open(CAPTURES / "n3-360" / r["seq"] / f"{r['n']:04d}.png").convert("RGB")
        px, py = to_screen("n3-360", r["x"], r["y"] - HALF_BODY, r["cx"], r["cy"])
        game.paste(im.crop((round(px - PIVOT[0]), round(py - PIVOT[1]), round(px - PIVOT[0]) + CELL[0], round(py - PIVOT[1]) + CELL[1])), (f * CELL[0], 0))
    cmp_sheet = Image.new("RGB", (run6.width, run6.height * 2 + 28), (12, 10, 16))
    d = ImageDraw.Draw(cmp_sheet)
    d.text((3, 1), "N3.6/N3.7 approved run6 (file)", fill=(235, 200, 110))
    cmp_sheet.paste(ref.convert("RGB"), (0, 14))
    d.text((3, run6.height + 15), "drawn by the game (camera 360, 1 texel = 1 px)", fill=(235, 200, 110))
    cmp_sheet.paste(game, (0, run6.height + 28))
    cmp_sheet.resize((cmp_sheet.width * 2, cmp_sheet.height * 2), Image.NEAREST).save(OUT / "n38_vs_n37_run6.png")

    (OUT / "measurements.json").write_text(json.dumps(measurements, indent=2) + "\n")
    print(json.dumps({k: v for k, v in measurements.items() if k != "pixel_check_360_vs_run6"}, indent=2))
    p = measurements["pixel_check_360_vs_run6"]
    print("pixel check 360:", {k: v for k, v in p.items() if k != "detail"})


if __name__ == "__main__":
    main()
