"""Engine 05 checks with the Agent Sprite Forge tools (offline, MIT,
Copyright (c) 2026 0x0funky). No paid call.

1. `pixel_qa.py` on the 1x renders of `cargo run --example equipment_preview`:
   - every new item isolated, against a palette made of its own material
     ramps: `off_palette` and `partial_alpha` must be 0;
   - the full set with the cape on the six run frames, `--onion`: the loop
     seam (last -> first change over the median step) must stay under 2.0.
2. `rig_animate.spring(k, d)` (ASF's damped-spring ease) as an independent
   reference for the cape's motion: the hem angle after a stop
   (`equipment_preview --trace gothic_cape`) is normalised and fitted with
   the best spring; the fit error and the overshoot are reported.

    PATH=~/.local/share/dc-art-tools/asf-venv/bin:$PATH python -B \
        art-src/equipment-studio/scripts/run_tools_05.py

Writes `target/equipment_studio/tools05/` and `tools05.json` there.
"""

from __future__ import annotations

import json
import os
import re
import subprocess
import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parents[3]
ITEMS = ROOT / "art-src/equipment-studio/items"
PREVIEW = ROOT / "target/equipment_preview"
OUT = ROOT / "target/equipment_studio/tools05"
ASF = Path.home() / ".claude/plugins/cache/agent-sprite-forge/agent-sprite-forge/0.6.0/skills/codeart2d/scripts"
NEW = ["gothic_v2_cuirass", "gothic_v2_pauldron", "gothic_v2_bracers", "gothic_v2_faulds",
       "gothic_cape", "gothic_cape_short", "leather_v2_jerkin"]
FULL = ["gothic_v2_cuirass", "gothic_v2_pauldron", "gothic_v2_bracers", "gothic_v2_faulds",
        "gothic_cape", "arming_sword"]
PREVIEW_BIN = ROOT / "target/debug/examples/equipment_preview"


def preview(*args: str) -> str:
    env = dict(os.environ, CARGO_MANIFEST_DIR=str(ROOT))
    res = subprocess.run([str(PREVIEW_BIN), *args], cwd=ROOT, env=env, capture_output=True, text=True)
    if res.returncode != 0:
        raise SystemExit(res.stderr)
    return res.stdout


def frames_png(name: str, clip: str, count: int) -> list[str]:
    out = []
    for f in range(count):
        hit = sorted(PREVIEW.glob(f"{name}_{clip}_f{f}_*x*.rgba"))[0]
        w, h = map(int, re.search(r"_(\d+)x(\d+)\.rgba$", hit.name).groups())
        p = OUT / f"{name}_{clip}_f{f}.png"
        Image.frombytes("RGBA", (w, h), hit.read_bytes()).save(p)
        out.append(str(p))
    return out


def ramp_palette(item: str) -> Path:
    text = (ITEMS / f"{item}.ron").read_text()
    colours = set()
    for ramp in re.findall(r"ramp: \(([^\n]*?)\), finish", text):
        for r, g, b in re.findall(r"\((\d+), (\d+), (\d+)\)", ramp):
            colours.add(f"#{int(r):02x}{int(g):02x}{int(b):02x}")
    # Imported PNG parts keep their own pixels: their colours join the palette.
    for path in re.findall(r'Image\(path: "([^"]+)"', text):
        img = Image.open(ROOT / "art-src/equipment-studio" / path).convert("RGBA")
        for r, g, b, a in img.getdata():
            if a:
                colours.add(f"#{r:02x}{g:02x}{b:02x}")
    p = OUT / f"{item}.hex"
    p.write_text("\n".join(sorted(colours)) + "\n")
    return p


def qa(inputs: list[str], report: Path, *extra: str) -> dict:
    # pixel_qa only writes new files: drop this script's previous outputs.
    report.unlink(missing_ok=True)
    if "--review" in extra:
        Path(extra[extra.index("--review") + 1]).unlink(missing_ok=True)
    cmd = [sys.executable, "-B", str(ASF / "pixel_qa.py"), "--input", *inputs, "--report", str(report), *extra]
    res = subprocess.run(cmd, cwd=ROOT, capture_output=True, text=True)
    data = json.loads(report.read_text()) if report.exists() else {}
    return {"exit": res.returncode, "checks": {c["id"]: [c["status"], c.get("value")] for c in data.get("checks", [])}}


def spring_fit() -> dict:
    sys.path.insert(0, str(ASF))
    from rig_animate import spring  # ASF, MIT

    rows = json.loads(preview("--trace", "gothic_cape"))
    stop = next(i for i, r in enumerate(rows) if r["clip"] == "idle" and i > 60)
    seg = rows[stop:]
    start, end = seg[0]["hem"], seg[-1]["hem"]
    t0 = seg[0]["t"]
    data = [((r["t"] - t0), (r["hem"] - end) / (start - end)) for r in seg]
    best = None
    for k in [x * 2.0 for x in range(5, 120)]:
        for d in [x * 0.5 for x in range(1, 40)]:
            ease = spring(k, d)
            err = sum((1.0 - ease(t) - y) ** 2 for t, y in data) / len(data)
            if best is None or err < best[0]:
                best = (err, k, d)
    err, k, d = best
    overshoot = min(y for _, y in data)
    return {
        "stop_frame": stop,
        "hem_running_deg": round(start, 2),
        "hem_rest_deg": round(end, 2),
        "best_spring": {"k": k, "d": d, "rms": round(err ** 0.5, 4)},
        "underdamped": d * d < 4 * k,
        "forward_overshoot_fraction": round(-overshoot, 3),
        "settle_time_s": next((round(t, 3) for t, y in data if abs(y) < 0.1 and all(abs(v) < 0.1 for tt, v in data if tt >= t)), None),
        "trace_points": len(data),
    }


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    result: dict = {"isolated_palette": {}, "loop": {}}
    for item in NEW:
        preview(f"iso_{item}", item, "--isolated")
        pngs = frames_png(f"iso_{item}", "run", 6) + frames_png(f"iso_{item}", "idle", 2)
        result["isolated_palette"][item] = qa(pngs, OUT / f"qa_{item}.json", "--palette", str(ramp_palette(item)))
    preview("full_set", *FULL)
    pngs = frames_png("full_set", "run", 6)
    result["loop"]["full_set_run"] = qa(pngs, OUT / "qa_full_run.json", "--onion",
                                        "--review", str(OUT / "qa_full_run_review.png"), "--scales", "1,3")
    preview("cape_only", "gothic_cape")
    result["loop"]["cape_run"] = qa(frames_png("cape_only", "run", 6), OUT / "qa_cape_run.json", "--onion")
    result["spring"] = spring_fit()
    (OUT / "tools05.json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result, indent=1))


if __name__ == "__main__":
    main()
