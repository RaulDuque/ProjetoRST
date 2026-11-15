"""GIFs of the V1.5 protagonist demonstration.

Two sources, always labelled:

- **Bevy capture** (``examples/player_demo.rs``): window screenshots of the
  real game in ``target/player_demo/seq<N>/`` with ``frames.log``. Each frame
  is cropped around the player (world position and camera from the log; the
  camera is 1:1, 1 world unit = 1 screen pixel) and written as a 1:1 GIF (real
  scale, no label on the image) and a x2 GIF for inspection only, with the
  state and clip under the image. Captured every 48 ms (3 frames of 16 ms).
- **Offline comparison V1.4 x V1.5**: the hurt and fall clips taken from the
  game atlases (the published V1.4 backup and the V1.5 staging), played at
  their clip timing over the room colour. These are the exact game sprites,
  but not a capture of the running game; the GIF says so.

Outputs go to ``art-src/player/comparisons/v1_5_art_polish/demo/``.

Usage:
    python3 art-src/player/scripts/post/demo_v1_5.py [--frames DIR] [--v14-assets DIR] [--v15-assets DIR]
"""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path

from PIL import Image, ImageDraw

import rig_sheets as S5
from labels import font
from resample import resize

PLAYER_DIR = Path(__file__).resolve().parents[2]
PROJECT = PLAYER_DIR.parents[1]
FRAMES = PROJECT / "target" / "player_demo"
OUT = PLAYER_DIR / "comparisons" / "v1_5_art_polish" / "demo"
CROP = (360, 220)  # world units around the player (1:1)
FRAME_MS = 48
LINE = re.compile(r"^(\d{4}) t=([\d.]+) state=(\w+) clip=(\S*) vfx=(\S+) player=\(([-\d.]+),([-\d.]+)\) "
                  r"camera=\(([-\d.]+),([-\d.]+)\)$")


def delays(count: int, step_ms: float) -> list[int]:
    """GIF delays in 10 ms units that keep the real timing on average."""
    out, carried = [], 0.0
    for _ in range(count):
        exact = step_ms + carried
        ms = max(20, int(round(exact / 10.0)) * 10)
        carried = exact - ms
        out.append(ms)
    return out


def sequence_gifs(seq: Path, out: Path) -> dict[str, object]:
    rows = [LINE.match(line) for line in (seq / "frames.log").read_text(encoding="utf-8").splitlines()]
    rows = [r for r in rows if r]
    small, large = [], []
    for r in rows:
        path = seq / f"{r[1]}.png"
        if not path.is_file():
            continue
        with Image.open(path) as shot:
            shot = shot.convert("RGB")
        scale_x, scale_y = shot.size[0] / 1280.0, shot.size[1] / 720.0
        px, py, cx, cy = (float(r[i]) for i in (6, 7, 8, 9))
        sx = shot.size[0] / 2 + (px - cx) * scale_x
        sy = shot.size[1] / 2 - (py - cy - 10.0) * scale_y  # a little above the body centre
        w, h = CROP[0] * scale_x, CROP[1] * scale_y
        box = tuple(int(round(v)) for v in (sx - w / 2, sy - h / 2, sx + w / 2, sy + h / 2))
        crop = shot.crop(box)
        if crop.size != CROP:
            crop = resize(crop, CROP, Image.Resampling.BILINEAR)
        small.append(crop)
        big = Image.new("RGB", (CROP[0] * 2, CROP[1] * 2 + 26), S5.DARK)
        big.paste(resize(crop, (CROP[0] * 2, CROP[1] * 2), Image.Resampling.NEAREST), (0, 0))
        vfx = "" if r[5] == "-" else f"  efeito: {r[5]}"
        ImageDraw.Draw(big).text((8, CROP[1] * 2 + 5), f"t={float(r[2]):.2f}s  {r[3]}  clip: {r[4]}{vfx}",
                                 fill=S5.TEXT, font=font(13))
        large.append(big)
    if not small:
        raise SystemExit(f"no frames in {seq}")
    d = delays(len(small), FRAME_MS)
    paths = {}
    for tag, frames in (("1x", small), ("2x_inspecao", large)):
        path = out / f"{seq.name}_bevy_{tag}.gif"
        frames[0].save(path, save_all=True, append_images=frames[1:], duration=d, loop=0, disposal=1)
        paths[tag] = path.name
    # Reference stills at 1:1: the first frame of every clip change.
    stills, last = [], None
    for r, frame in zip(rows, small):
        if r[4] != last:
            stills.append((r[4], frame))
            last = r[4]
    sheet = Image.new("RGB", (min(6, len(stills)) * (CROP[0] + 6) + 6,
                              ((len(stills) + 5) // 6) * (CROP[1] + 24) + 6), S5.DARK)
    draw = ImageDraw.Draw(sheet)
    for i, (clip, frame) in enumerate(stills):
        x, y = 6 + (i % 6) * (CROP[0] + 6), 6 + (i // 6) * (CROP[1] + 24)
        sheet.paste(frame, (x, y))
        draw.text((x, y + CROP[1] + 4), clip, fill=S5.TEXT, font=font(12))
    still = out / f"{seq.name}_bevy_stills_1x.png"
    sheet.save(still)
    return {"frames": len(small), "duration_s": round(sum(d) / 1000.0, 2), "gifs": paths, "stills": still.name}


def atlas_frames(assets: Path, clip: str) -> tuple[list[Image.Image], list[float], int]:
    ron = (assets / "animations" / "player.ron").read_text(encoding="utf-8")
    m = re.search(rf'"{clip}": \(atlas: "(\w+)", first: (\d+), count: (\d+),.*?durations: \[([^\]]*)\]', ron)
    if not m:
        raise SystemExit(f"{clip} not in {assets}")
    atlas_id, first, count = m[1], int(m[2]), int(m[3])
    a = re.search(rf'"{atlas_id}": \(texture: "([^"]+)", cell: \((\d+), (\d+)\), columns: (\d+), rows: \d+, '
                  rf"padding: (\d+), density: (\d+)\)", ron)
    if not a:
        raise SystemExit(f"atlas {atlas_id} not in {assets}")
    cw, ch, cols, pad, density = (int(a[i]) for i in range(2, 7))
    with Image.open(assets / a[1]) as img:
        img = img.convert("RGBA")
    frames = []
    for k in range(first, first + count):
        x, y = (k % cols) * (cw + pad), (k // cols) * (ch + pad)
        frames.append(img.crop((x, y, x + cw, y + ch)))
    return frames, [float(v) for v in m[4].split(",")], density


def offline_comparison(v14: Path, v15: Path, out: Path) -> dict[str, str]:
    """Hurt and fall, V1.4 left and V1.5 right, from the game atlases (not a capture)."""
    paths = {}
    for clip, loops in (("hurt", 3), ("fall", 3)):
        a, da, dens = atlas_frames(v14, clip)
        b, db, _ = atlas_frames(v15, clip)
        for zoom in (1, 3):
            w, h = a[0].size[0] // dens * zoom, a[0].size[1] // dens * zoom
            label_h = 18 if zoom == 1 else 30
            frames, durations = [], []
            # Both clips have the same length; step through the union of their frame starts.
            starts_a = [sum(da[:i]) for i in range(len(da))]
            starts_b = [sum(db[:i]) for i in range(len(db))]
            cuts = sorted(set(round(s, 4) for s in starts_a + starts_b))
            total = sum(da)
            for _ in range(loops):
                for n, t in enumerate(cuts):
                    ia = max(i for i, s in enumerate(starts_a) if s <= t + 1e-6)
                    ib = max(i for i, s in enumerate(starts_b) if s <= t + 1e-6)
                    canvas = Image.new("RGBA", (2 * w + 12, h + label_h), (*S5.DARK, 255))
                    for col, sprite in enumerate((a[ia], b[ib])):
                        canvas.alpha_composite(S5.screen(sprite, "B", zoom, dens), (col * (w + 12), label_h))
                    draw = ImageDraw.Draw(canvas)
                    size = 10 if zoom == 1 else 14
                    draw.text((2, 2), "V1.4", fill=S5.TEXT, font=font(size))
                    draw.text((w + 14, 2), "V1.5", fill=S5.TEXT, font=font(size))
                    if zoom > 1:
                        draw.text((80, 6), "offline: sprites do atlas, não é captura do Bevy", fill=S5.MUTED,
                                  font=font(11))
                    frames.append(canvas.convert("RGB"))
                    end = cuts[n + 1] if n + 1 < len(cuts) else total
                    durations.append((end - t) * 1000.0)
            d, carried = [], 0.0
            for ms in durations:
                exact = ms + carried
                r = max(20, int(round(exact / 10.0)) * 10)
                carried = exact - r
                d.append(r)
            path = out / f"offline_{clip}_v1_4_vs_v1_5_{zoom}x.gif"
            frames[0].save(path, save_all=True, append_images=frames[1:], duration=d, loop=0, disposal=1)
            paths[f"{clip}_{zoom}x"] = path.name
    return paths


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--frames", type=Path, default=FRAMES)
    parser.add_argument("--v14-assets", type=Path, required=True, help="backup of the V1.4 publication")
    parser.add_argument("--v15-assets", type=Path, default=PROJECT / "assets")
    args = parser.parse_args()
    OUT.mkdir(parents=True, exist_ok=True)
    report = {"source": "examples/player_demo.rs (Bevy window screenshots)", "frame_ms": FRAME_MS,
              "crop_world_units": CROP, "sequences": {}}
    for seq in sorted(p for p in args.frames.glob("seq*") if p.is_dir()):
        report["sequences"][seq.name] = sequence_gifs(seq, OUT)
    report["offline_comparison"] = offline_comparison(args.v14_assets, args.v15_assets, OUT)
    (OUT / "demo_report.json").write_text(json.dumps(report, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(json.dumps(report, indent=2, ensure_ascii=False))


if __name__ == "__main__":
    main()
