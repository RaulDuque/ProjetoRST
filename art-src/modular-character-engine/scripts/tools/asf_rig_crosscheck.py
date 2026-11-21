"""Cross-check our forward kinematics with Agent Sprite Forge rig_animate.py.

Reads `target/modular_character_engine/rig_export.json` (written by
`cargo run --example modular_character_engine`): the rest bones of our
skeleton and, per approved run frame, the local rotations and our FK result
on the rest proportions. It writes an ASF `rig.v1` SVG (one capsule per bone,
a 3x3 coloured marker on every bone end, nested groups = our hierarchy) and a
`rig_anim.v1` JSON with the same rotations, runs ASF `rig_animate.py`
(MIT, Copyright (c) 2026 0x0funky) and finds the markers in its frames: the
distance to our FK joints says whether both implementations agree.

    PATH=~/.local/share/dc-art-tools/asf-venv/bin:$PATH \
        python -B art-src/modular-character-engine/scripts/tools/asf_rig_crosscheck.py OUT_DIR
"""

from __future__ import annotations

import json
import math
import shutil
import subprocess
import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parents[4]
ASF = Path.home() / ".claude/plugins/cache/agent-sprite-forge/agent-sprite-forge/0.6.0"
RIG_ANIMATE = ASF / "skills/codeart2d/scripts/rig_animate.py"
EXPORT = ROOT / "target/modular_character_engine/rig_export.json"

WIDTH = {"Spine": 4.0, "Head": 3.0, "UpperArmNear": 2.2, "ForearmNear": 1.9, "UpperArmFar": 2.0,
         "ForearmFar": 1.7, "ThighA": 3.0, "ShinA": 2.4, "FootA": 1.6, "ThighB": 2.8, "ShinB": 2.2, "FootB": 1.5}
FILL = {"near": "#c8a088", "far": "#7a6a66", "body": "#9a96a2", "leg_a": "#6a5050", "leg_b": "#4a3a3c"}
Z = {"Spine": 30, "Head": 40, "UpperArmNear": 60, "ForearmNear": 61, "UpperArmFar": 5, "ForearmFar": 6,
     "ThighA": 51, "ShinA": 52, "FootA": 53, "ThighB": 10, "ShinB": 11, "FootB": 12}


def marker_colour(i: int) -> str:
    # Distinct saturated colours (never used by the capsules).
    hues = [(255, 0, 0), (0, 255, 0), (0, 0, 255), (255, 255, 0), (255, 0, 255), (0, 255, 255),
            (255, 128, 0), (128, 0, 255), (0, 255, 128), (255, 0, 128), (128, 255, 0), (0, 128, 255),
            (255, 255, 255)]
    r, g, b = hues[i % len(hues)]
    return f"#{r:02x}{g:02x}{b:02x}"


def fill_of(bone: str) -> str:
    if bone.endswith("Near"):
        return FILL["near"]
    if bone.endswith("Far"):
        return FILL["far"]
    if bone.endswith("A"):
        return FILL["leg_a"]
    if bone.endswith("B"):
        return FILL["leg_b"]
    return FILL["body"]


def capsule(a, b, w) -> str:
    dx, dy = b[0] - a[0], b[1] - a[1]
    n = math.hypot(dx, dy) or 1.0
    ox, oy = -dy / n * w, dx / n * w
    pts = [(a[0] + ox, a[1] + oy), (b[0] + ox, b[1] + oy), (b[0] - ox, b[1] - oy), (a[0] - ox, a[1] - oy)]
    return "M" + " L".join(f"{x:.2f} {y:.2f}" for x, y in pts) + " Z"


def build_svg(export: dict) -> tuple[str, dict]:
    bones = export["bones"]
    w, h = export["canvas"]
    px, py = export["pivot"]
    children: dict[str | None, list[str]] = {}
    for name, b in bones.items():
        children.setdefault(b["parent"], []).append(name)
    markers = {}
    order = list(bones)

    def group(name: str, depth: int) -> str:
        b = bones[name]
        a, e = b["seg"]
        i = order.index(name)
        colour = marker_colour(i)
        markers[name] = colour
        pad = "  " * depth
        out = [f'{pad}<g id="{name}" data-pivot="{a[0]:.3f} {a[1]:.3f}">']
        out.append(f'{pad}  <path data-z="{Z[name]}" fill="{fill_of(name)}" d="{capsule(a, e, WIDTH[name])}"/>')
        out.append(f'{pad}  <rect data-z="{200 + i}" fill="{colour}" x="{e[0] - 1.5:.3f}" y="{e[1] - 1.5:.3f}" width="3" height="3"/>')
        for c in children.get(name, []):
            out.append(group(c, depth + 1))
        out.append(f"{pad}</g>")
        return "\n".join(out)

    body = "\n".join(group(n, 2) for n in children[None])
    svg = f'''<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}"
     data-codeart="rig.v1" data-anchor="{px} {py}" data-facing="right">
  <!-- Dark Castle Modular Character Engine skeleton (rest = approved idle), exported for an ASF cross-check. -->
  <g id="root" data-pivot="{px} {py}">
{body}
  </g>
</svg>
'''
    return svg, markers


def build_anim(export: dict) -> dict:
    frames = export["frames"]
    n = len(frames)
    tracks: dict[str, dict] = {}
    for name in export["bones"]:
        keys = [[i / n, frames[i]["rotate"][name]] for i in range(n)] + [[1.0, frames[0]["rotate"][name]]]
        tracks[name] = {"rotate": keys, "ease": "step"}
    tracks["Spine"]["translate"] = [[i / n, frames[i]["translate"]] for i in range(n)] + [[1.0, frames[0]["translate"]]]
    return {
        "schema": "codeart2d.rig_anim.v1",
        "rig": "skeleton.rig.svg",
        "states": {"moving": "run"},
        "clips": {"run": {"frames": n, "loop": True, "duration_ms": 67, "ease": "step", "tracks": tracks}},
    }


def find(img: Image.Image, colour: str) -> tuple[float, float] | None:
    rgb = tuple(int(colour[i:i + 2], 16) for i in (1, 3, 5))
    px = img.load()
    pts = [(x + 0.5, y + 0.5) for y in range(img.height) for x in range(img.width)
           if px[x, y][3] > 0 and tuple(px[x, y][:3]) == rgb]
    if not pts:
        return None
    return (sum(p[0] for p in pts) / len(pts), sum(p[1] for p in pts) / len(pts))


def main() -> None:
    out = Path(sys.argv[1])
    work = out / "asf_rig"
    if work.exists():
        shutil.rmtree(work)
    work.mkdir(parents=True)
    export = json.loads(EXPORT.read_text())
    svg, markers = build_svg(export)
    (work / "skeleton.rig.svg").write_text(svg)
    (work / "skeleton.anim.json").write_text(json.dumps(build_anim(export), indent=1))
    cmd = [sys.executable, "-B", str(RIG_ANIMATE), "--anim", str(work / "skeleton.anim.json"),
           "--output-dir", str(work / "render"), "--route", "vector"]
    run = subprocess.run(cmd, capture_output=True, text=True)
    (work / "rig_animate.log").write_text(run.stdout + "\n" + run.stderr)
    result = {"command": " ".join(cmd[1:]), "returncode": run.returncode}
    report = work / "render" / "rig-report.json"
    zoom = 1.0
    if report.exists():
        rep = json.loads(report.read_text())
        zoom = float(rep.get("zoom", 1))
        result["asf_zoom"] = zoom
        result["asf_ground_failures"] = rep.get("ground_failures")
        result["asf_qa"] = rep.get("qa")
    # Exit 4 = frames written but ASF's own QA gate failed (it expects feet
    # planted on its ground row; this skeleton has no feet geometry there).
    if run.returncode not in (0, 4) or not (work / "render" / "frames").exists():
        result["error"] = (run.stderr or run.stdout)[-2000:]
        (out / "asf_rig_crosscheck.json").write_text(json.dumps(result, indent=2))
        print(json.dumps(result, indent=2))
        sys.exit(1)
    frames = sorted((work / "render" / "frames").glob("run-*.png"))
    result["frames_found"] = [str(f.relative_to(work)) for f in frames]
    per_frame = []
    for i, f in enumerate(frames[: len(export["frames"])]):
        img = Image.open(f).convert("RGBA")
        errors = {}
        for name, colour in markers.items():
            got = find(img, colour)
            if got is not None:
                got = (got[0] / zoom, got[1] / zoom)
            want = export["frames"][i]["fk"][name][1]
            errors[name] = None if got is None else round(math.hypot(got[0] - want[0], got[1] - want[1]), 3)
        per_frame.append(errors)
    seen = [e for f in per_frame for e in f.values() if e is not None]
    result["markers_found"] = len(seen)
    result["markers_expected"] = len(per_frame) * len(markers)
    result["max_error_texels"] = max(seen) if seen else None
    result["mean_error_texels"] = round(sum(seen) / len(seen), 3) if seen else None
    result["per_frame"] = per_frame
    (out / "asf_rig_crosscheck.json").write_text(json.dumps(result, indent=2))
    print(json.dumps({k: v for k, v in result.items() if k != "per_frame"}, indent=2))


if __name__ == "__main__":
    main()
