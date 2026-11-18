"""N3.7: reusable, config-driven reference-video -> sprite pipeline.

The N3.6 run was built by two one-off scripts. This module is the same
method behind one JSON config, so the next videos (jump, attack, idle) go
through exactly the reduction the owner approved:

1. read the video metadata (GStreamer's `gst-discoverer-1.0`);
2. decode every frame once into `cache/<sha12>/` (ignored by git);
3. key the green screen of the chosen frames (margin, 1 px erosion,
   spill clamp), same crop window for every frame;
4. reduce at ONE scale (box filter, alpha >= 50 %, nearest colour of the
   N3.4 palette without the hand-only accents `M`/`e`);
5. place every frame on the canvas with ONE global offset (no per-frame
   recentring, so the body's own bob stays);
6. write the text grids, one atlas per clip, an `AnimationManifest` RON,
   the opaque boxes, metrics and the per-frame provenance;
7. write a preview sheet and a GIF of every clip.

No step paints, interpolates or invents a frame. The source video stays
read only; its SHA-256 is recorded and checked.

    python3 -B art-src/n3-7-run-sync/scripts/animation_pipeline.py \
        art-src/n3-7-run-sync/configs/run_n36.json
"""

from __future__ import annotations

import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path

from PIL import Image, ImageDraw, ImageFilter

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "art-src/n3-4-original-protagonist/scripts"))
sys.path.insert(0, str(ROOT / "art-src/player/scripts/post"))
from n34_palette import PALETTE, TRANSPARENT  # noqa: E402
from resample import resize  # noqa: E402

HERE = ROOT / "art-src/n3-7-run-sync"
PIPELINE_VERSION = "n3.7-pipeline/1"
HAND_ONLY = {"M", "e"}
NEUTRAL = (150, 150, 156, 255)
INK = (40, 32, 40)
NEAREST = Image.Resampling.NEAREST


# --- small helpers -------------------------------------------------------

def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def distance(a: tuple[int, ...], b: tuple[int, ...]) -> int:
    return sum(abs(x - y) for x, y in zip(a[:3], b[:3], strict=True))


def zoom(image: Image.Image, factor: int) -> Image.Image:
    return resize(image, (image.width * factor, image.height * factor), NEAREST)


def on(image: Image.Image, background) -> Image.Image:
    plate = Image.new("RGBA", image.size, background)
    plate.alpha_composite(image.convert("RGBA"))
    return plate


# --- 1. metadata and decode ----------------------------------------------

def metadata(video: Path) -> dict:
    """Width, height, fps and duration from `gst-discoverer-1.0 -v`."""
    text = subprocess.run(["gst-discoverer-1.0", "-v", str(video)], check=True,
                          capture_output=True, text=True).stdout
    width = re.search(r"width=\(int\)(\d+)", text)
    height = re.search(r"height=\(int\)(\d+)", text)
    rate = re.search(r"framerate=\(fraction\)(\d+)/(\d+)", text)
    duration = re.search(r"(\d+):(\d+):(\d+\.\d+)", text)
    if not (width and height and rate):
        raise SystemExit(f"cannot read the metadata of {video}")
    seconds = None
    if duration:
        h, m, s = duration.groups()
        seconds = int(h) * 3600 + int(m) * 60 + float(s)
    return {"width": int(width[1]), "height": int(height[1]),
            "fps": int(rate[1]) / int(rate[2]), "duration_s": seconds,
            "codec": "H.264" if "h264" in text else "unknown"}


def decode(video: Path, digest: str) -> Path:
    cache = HERE / "cache" / digest[:12]
    if cache.is_dir() and any(cache.glob("f_*.png")):
        return cache
    cache.mkdir(parents=True, exist_ok=True)
    subprocess.run(["gst-launch-1.0", "-q", "filesrc", f"location={video}", "!", "decodebin", "!",
                    "videoconvert", "!", "pngenc", "!", "multifilesink",
                    f"location={cache}/f_%03d.png"], check=True)
    return cache


# --- 2. key ----------------------------------------------------------------

def key(frame: Image.Image, crop: tuple[int, int, int, int], margin: int, erode: int) -> Image.Image:
    """Green-screen key: background when green beats red and blue by `margin`."""
    rgb = frame.convert("RGB").crop(crop)
    px = rgb.load()
    mask = Image.new("L", rgb.size, 0)
    mp = mask.load()
    for y in range(rgb.height):
        for x in range(rgb.width):
            r, g, b = px[x, y]
            if not (g > r + margin and g > b + margin):
                mp[x, y] = 255
            px[x, y] = (r, min(g, max(r, b)), b)
    for _ in range(erode):
        mask = mask.filter(ImageFilter.MinFilter(3))
    out = rgb.convert("RGBA")
    out.putalpha(mask)
    return out


# --- 3. reduce and place ---------------------------------------------------

def reduce(frame: Image.Image, scale: float) -> list[str]:
    size = (round(frame.width * scale), round(frame.height * scale))
    small = frame.resize(size, Image.Resampling.BOX)
    colours = {k: v for k, v in PALETTE.items() if k not in HAND_ONLY}
    rows = []
    for y in range(small.height):
        row = []
        for x in range(small.width):
            r, g, b, a = small.getpixel((x, y))
            row.append(TRANSPARENT if a < 128 else
                       min(colours, key=lambda k: distance(colours[k], (r, g, b))))
        rows.append("".join(row))
    return rows


def place(rows: list[str], offset: tuple[int, int], canvas: tuple[int, int]) -> list[str]:
    grid = [[TRANSPARENT] * canvas[0] for _ in range(canvas[1])]
    for y, row in enumerate(rows):
        for x, k in enumerate(row):
            if k == TRANSPARENT:
                continue
            cx, cy = x + offset[0], y + offset[1]
            if not (0 <= cx < canvas[0] and 0 <= cy < canvas[1]):
                raise SystemExit(f"pixel {x},{y} falls outside the {canvas} canvas")
            grid[cy][cx] = k
    return ["".join(r) for r in grid]


def render(rows: list[str]) -> Image.Image:
    image = Image.new("RGBA", (len(rows[0]), len(rows)), (0, 0, 0, 0))
    for y, row in enumerate(rows):
        for x, k in enumerate(row):
            if k != TRANSPARENT:
                image.putpixel((x, y), (*PALETTE[k], 255))
    return image


def opaque_box(rows: list[str]) -> tuple[int, int, int, int]:
    xs = [x for row in rows for x, k in enumerate(row) if k != TRANSPARENT]
    ys = [y for y, row in enumerate(rows) if row.strip(TRANSPARENT)]
    return min(xs), min(ys), max(xs), max(ys)


def resolve_scale(config: dict, keyed: dict[int, Image.Image]) -> tuple[float, str]:
    rule = config["scale"]
    if "fixed" in rule:
        return float(rule["fixed"]), f"fixed {rule['fixed']}"
    frame = rule["fit_frame"]
    box = keyed[frame].getchannel("A").getbbox()
    if box is None:
        raise SystemExit(f"frame {frame} has no figure")
    return rule["figure_texels"] / (box[3] - box[1]), (
        f"frame {frame} opaque height {box[3] - box[1]} px -> {rule['figure_texels']} texels")


def resolve_offset(config: dict, keyed: dict[int, Image.Image], reduced: dict[int, list[str]],
                   scale: float) -> tuple[tuple[int, int], str]:
    rule = config["offset"]
    pivot = config["pivot"]
    if "fixed" in rule:
        return (rule["fixed"][0], rule["fixed"][1]), f"fixed {rule['fixed']}: {rule.get('why', '')}"
    stand = keyed[rule["hip_frame"]]
    box = stand.getchannel("A").getbbox()
    hip_row = box[1] + round(0.45 * (box[3] - box[1]))
    hip_xs = [x for x in range(stand.width) if stand.getpixel((x, hip_row))[3]]
    hip_x = (hip_xs[0] + hip_xs[-1]) / 2 * scale
    lowest = max(max(y for y, row in enumerate(reduced[f]) if row.strip(TRANSPARENT))
                 for f in rule["ground_frames"])
    return (pivot[0] - round(hip_x), pivot[1] - lowest), (
        f"hip of frame {rule['hip_frame']} on the pivot column, lowest sole of "
        f"{rule['ground_frames']} on the pivot row")


# --- 4. clips --------------------------------------------------------------

def clip_durations(clip: dict, fps: float) -> list[float]:
    frames = clip["frames"]
    if clip.get("video_timing"):
        ends = frames[1:] + [clip["cycle_end"]]
        return [(b - a) / fps for a, b in zip(frames, ends, strict=True)]
    if "durations" in clip:
        return [float(d) for d in clip["durations"]]
    return [1.0 / clip["fps"]] * len(frames)


def manifest_text(config: dict, clips: dict[str, list[float]]) -> str:
    cw, ch = config["canvas"]
    px, py = config["pivot"]
    lines = ["(\n    version: 1,\n", f"    facing: {config.get('facing', 'Right')},\n",
             "    weapon_in_body: false,\n    atlases: {\n"]
    for name, durations in clips.items():
        lines.append(f'        "{name}": (texture: "{name}.png", cell: ({cw}, {ch}), '
                     f"columns: {len(durations)}, rows: 1, padding: 0, density: {config['density']}),\n")
    lines.append("    },\n    clips: {\n")
    for name, durations in clips.items():
        clip = config["clips"][name]
        loop = clip.get("loop", True)
        fps = clip.get("fps", round(len(durations) / sum(durations), 3))
        mode = f"Loop(fps: {fps:.1f})" if loop else f"Once(fps: {fps:.1f})"
        values = ", ".join(f"{d:.5f}" for d in durations)
        fallback = "None" if loop else 'Some("idle")'
        lines.append(f'        "{name}": (atlas: "{name}", first: 0, count: {len(durations)}, mode: {mode},\n'
                     f"            pivot: ({px:.1f}, {py:.1f}), fallback: {fallback},\n"
                     f"            durations: [{values}], speed_reference: None),\n")
    lines.append("    },\n)\n")
    return "".join(lines)


# --- 5. run ----------------------------------------------------------------

def run(config_path: Path) -> dict:
    config = json.loads(config_path.read_text(encoding="utf-8"))
    name = config["name"]
    out = HERE / "pipeline" / name
    for sub in ("source", "generated", "preview"):
        (out / sub).mkdir(parents=True, exist_ok=True)
    video = ROOT / config["video"]
    digest = sha256(video)
    if config.get("video_sha256") and config["video_sha256"] != digest:
        raise SystemExit(f"{video.name}: SHA-256 differs from the config")
    meta = metadata(video)
    cache = decode(video, digest)
    decoded = len(list(cache.glob("f_*.png")))
    crop = tuple(config["crop"])
    keying = config["key"]
    keyed = {f: key(Image.open(cache / f"f_{f:03d}.png"), crop, keying["margin"], keying["erode"])
             for f in config["frames"]}
    keyed_dir = cache / f"keyed_{name}"
    keyed_dir.mkdir(exist_ok=True)
    for f, image in keyed.items():
        image.save(keyed_dir / f"video_{f:03d}.png")

    scale, scale_why = resolve_scale(config, keyed)
    reduced = {f: reduce(image, scale) for f, image in keyed.items()}
    offset, offset_why = resolve_offset(config, keyed, reduced, scale)
    canvas = tuple(config["canvas"])
    grids = {f: place(rows, offset, canvas) for f, rows in reduced.items()}
    for f, rows in grids.items():
        (out / "source" / f"video_{f:03d}.txt").write_text("\n".join(rows) + "\n", encoding="utf-8")
    images = {f: render(rows) for f, rows in grids.items()}
    boxes = {f: opaque_box(rows) for f, rows in grids.items()}

    clips = {clip: clip_durations(spec, meta["fps"]) for clip, spec in config["clips"].items()}
    for clip, spec in config["clips"].items():
        sheet = Image.new("RGBA", (canvas[0] * len(spec["frames"]), canvas[1]), (0, 0, 0, 0))
        for i, f in enumerate(spec["frames"]):
            sheet.alpha_composite(images[f], (i * canvas[0], 0))
        sheet.save(out / "generated" / f"{clip}.png")
    (out / "generated" / f"{name}.ron").write_text(manifest_text(config, clips), encoding="utf-8")
    (out / "generated" / "boxes.ron").write_text(
        "{\n" + "".join(f'    "{clip}": [{", ".join(str(boxes[f]) for f in spec["frames"])}],\n'
                        for clip, spec in config["clips"].items()) + "}\n", encoding="utf-8")

    excluded = {int(k): v for k, v in config.get("excluded", {}).items()}
    provenance = {
        "pipeline_version": PIPELINE_VERSION,
        "config": str(config_path.relative_to(ROOT)),
        "source": {"file": config["video"], "sha256": digest, **meta, "decoded_frames": decoded,
                   "authorship": config.get("source_authorship", "")},
        "extraction": "gst-launch-1.0 decodebin ! videoconvert ! pngenc (every frame, lossless PNG)",
        "background_removal": (f"green key: background when g > r+{keying['margin']} and g > b+"
                               f"{keying['margin']}; mask eroded {keying['erode']} px; green spill "
                               "clamped to max(r, b)"),
        "crop": list(crop),
        "scale": round(scale, 6),
        "scale_rule": scale_why,
        "reduction": "PIL BOX filter, alpha >= 128 opaque, nearest N3.4 colour without M/e",
        "offset": list(offset),
        "offset_rule": offset_why,
        "canvas": list(canvas),
        "pivot": config["pivot"],
        "density": config["density"],
        "frames": {
            str(f): {
                "video_index": f,
                "timestamp_s": round(f / meta["fps"], 5),
                "keyed_intermediate": f"cache/{digest[:12]}/keyed_{name}/video_{f:03d}.png (ignored)",
                "grid": f"pipeline/{name}/source/video_{f:03d}.txt",
                "opaque_box": list(boxes[f]),
                "excluded": excluded.get(f),
                "adjustments": [],
                "manual_edit": False,
                "agent_generated": False,
                "source_ai_generated": bool(config.get("source_ai_generated", True)),
            } for f in config["frames"]
        },
        "clips": {clip: {"frames": spec["frames"], "durations": [round(d, 5) for d in clips[clip]],
                         "total_s": round(sum(clips[clip]), 5), "loop": spec.get("loop", True)}
                  for clip, spec in config["clips"].items()},
        "approval": "NOT APPROVED (experimental)",
    }
    (out / "generated" / "provenance.json").write_text(json.dumps(provenance, indent=1) + "\n",
                                                       encoding="utf-8")

    # Validation: every clip frame fits the canvas, stays above the ground
    # row, and every atlas has the declared size.
    problems = []
    for clip, spec in config["clips"].items():
        atlas = Image.open(out / "generated" / f"{clip}.png")
        if atlas.size != (canvas[0] * len(spec["frames"]), canvas[1]):
            problems.append(f"{clip}: atlas size {atlas.size}")
        for f in spec["frames"]:
            if boxes[f][3] > config["pivot"][1]:
                problems.append(f"{clip}: frame {f} below the ground row")
    if problems:
        raise SystemExit("; ".join(problems))

    # Preview: every frame at 2x with its id, box and pivot; one GIF per clip.
    gap, label = 8, 24
    width = len(config["frames"]) * (canvas[0] * 2 + gap) + gap
    preview = Image.new("RGBA", (width, canvas[1] * 2 + label + gap * 2), (226, 224, 228, 255))
    draw = ImageDraw.Draw(preview)
    for i, f in enumerate(config["frames"]):
        x = gap + i * (canvas[0] * 2 + gap)
        draw.text((x, 4), f"{f}" + (f" {excluded[f]}" if f in excluded else ""), fill=INK)
        panel = on(zoom(images[f], 2), NEUTRAL)
        pd = ImageDraw.Draw(panel)
        x0, y0, x1, y1 = boxes[f]
        pd.rectangle([x0 * 2, y0 * 2, x1 * 2 + 1, y1 * 2 + 1], outline=(60, 120, 220))
        px, py = config["pivot"]
        pd.line([(px * 2 - 8, py * 2 + 1), (px * 2 + 8, py * 2 + 1)], fill=(220, 30, 40), width=2)
        preview.alpha_composite(panel, (x, label))
    preview.save(out / "preview" / "frames.png")
    for clip, spec in config["clips"].items():
        if len(spec["frames"]) < 2:
            continue
        shown = [on(zoom(images[f], 3), NEUTRAL).convert("P", palette=Image.Palette.ADAPTIVE)
                 for f in spec["frames"]]
        shown[0].save(out / "preview" / f"{clip}.gif", save_all=True, append_images=shown[1:],
                      loop=0, duration=[round(d * 1000) for d in clips[clip]], disposal=2)
    return provenance


def main() -> None:
    if len(sys.argv) < 2:
        raise SystemExit(__doc__)
    for arg in sys.argv[1:]:
        provenance = run(ROOT / arg if not Path(arg).is_absolute() else Path(arg))
        print(json.dumps({"config": provenance["config"], "scale": provenance["scale"],
                          "offset": provenance["offset"], "frames": len(provenance["frames"]),
                          "clips": {k: v["total_s"] for k, v in provenance["clips"].items()}}))


if __name__ == "__main__":
    main()
