"""Practical runs of pixelfx and Agent Sprite Forge on the lab's output (offline).

Nothing here is needed by the game or by the lab. Inputs: the captures and
exports of `cargo run --example modular_character_engine` in
`target/modular_character_engine/`. Work files: `.../tools/`. A summary goes
to `tools/external_tools.json`; `make_evidence_02.py` copies the evidence.

Tools (local, no paid calls):
- ASF codeart2d `rig_animate.py`: renders our skeleton's run poses
  (`tools/asf_rig_crosscheck.py`) to cross-check our FK;
- ASF video2dsprite `pixel_motion.py measure`: feet, step and ground speed of
  the approved run and of the hybrid run with equipment;
- ASF generate2dsprite `build_animation_clips.py`: tick grid, holds, loop
  seam and mirrored-turn slide of both runs;
- ASF codeart2d `pixel_qa.py`: palette, alpha, orphans and outline gates of
  the breastplate rasters and the pauldrons;
- pixelfx `rig.js skeleton()` (`tools/pixelfx_probe.mjs`) and `tools/act.mjs`.

    PATH=~/.local/share/dc-art-tools/asf-venv/bin:$PATH \
        python -B art-src/modular-character-engine/scripts/run_external_tools.py
"""

from __future__ import annotations

import json
import re
import shutil
import subprocess
import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parents[3]
CAP = ROOT / "target/modular_character_engine"
WORK = CAP / "tools"
ASF = Path.home() / ".claude/plugins/cache/agent-sprite-forge/agent-sprite-forge/0.6.0/skills"
PFX = Path.home() / ".claude/plugins/cache/pixelfx/pixelfx/0.3.1"
SCRIPTS = Path(__file__).resolve().parent
RUN6 = ROOT / "art-src/n3-8-production-run/stage/sprites/player/n36_run6.png"
# Hybrid panel cell on screen: centre x 640, sole y 610, 3 px per texel.
SCALE, CELL, PIVOT = 3, (112, 128), (56, 118)
BG = (48, 43, 56)


def run(cmd: list[str], cwd: Path | None = None) -> dict:
    p = subprocess.run(cmd, capture_output=True, text=True, cwd=cwd)
    return {"cmd": " ".join(str(c) for c in cmd), "returncode": p.returncode,
            "stdout": p.stdout[-4000:], "stderr": p.stderr[-2000:]}


def fresh(path: Path) -> Path:
    if path.exists():
        shutil.rmtree(path)
    return path


def approved_frames(out: Path) -> list[Path]:
    strip = Image.open(RUN6).convert("RGBA")
    files = []
    for i in range(6):
        f = out / f"frame_{i:03d}.png"
        strip.crop((i * 112, 0, i * 112 + 112, 128)).save(f)
        files.append(f)
    return files


def hybrid_frames(names: list[str], out: Path) -> list[Path]:
    """1x frames of the hybrid panel, sampled from the 3x captures."""
    x0 = 640 - PIVOT[0] * SCALE
    y0 = (360 + 250) - PIVOT[1] * SCALE
    files = []
    for i, n in enumerate(names):
        cap = Image.open(CAP / f"{n}.png").convert("RGB")
        img = Image.new("RGBA", CELL, (0, 0, 0, 0))
        for y in range(CELL[1]):
            for x in range(CELL[0]):
                r, g, b = cap.getpixel((x0 + x * SCALE + 1, y0 + y * SCALE + 1))
                if max(abs(r - BG[0]), abs(g - BG[1]), abs(b - BG[2])) > 6:
                    img.putpixel((x, y), (r, g, b, 255))
        f = out / f"frame_{i:03d}.png"
        img.save(f)
        files.append(f)
    return files


def raw_to_png(src: Path, dst: Path) -> Path:
    m = re.search(r"_(\d+)x(\d+)\.rgba$", src.name)
    w, h = int(m.group(1)), int(m.group(2))
    Image.frombytes("RGBA", (w, h), src.read_bytes()).save(dst)
    return dst


def manifest(frames: list[Path], path: Path) -> None:
    path.write_text(json.dumps({
        "schema": "generate2dsprite.animation_clips.v2",
        "frames": [f.name for f in frames],
        "anchor_px": list(PIVOT),
        "clips": {"run": {"frames": list(range(len(frames))), "ticks": 4, "loop_policy": "cycle",
                          "events": [{"at": 0, "name": "step_r"}, {"at": 3, "name": "step_l"}],
                          "role": "player"}},
        "states": {"moving": "run"},
        "pixel_art": True,
    }, indent=1))


def main() -> None:
    py = sys.executable
    WORK.mkdir(parents=True, exist_ok=True)
    summary: dict[str, object] = {}

    # 1. ASF rig_animate cross-check of our FK.
    r = run([py, "-B", str(SCRIPTS / "tools/asf_rig_crosscheck.py"), str(WORK)])
    summary["asf_rig_animate"] = json.loads((WORK / "asf_rig_crosscheck.json").read_text())
    summary["asf_rig_animate"].pop("per_frame", None)

    # 2. Frames: approved run and hybrid run with all equipment, at 1x.
    sets = {}
    for label, make in (
        ("approved_run", lambda d: approved_frames(d)),
        ("hybrid_run_equipped", lambda d: hybrid_frames([f"all_run_f{i}" for i in range(6)], d)),
    ):
        d = fresh(WORK / label) / "frames"
        d.mkdir(parents=True)
        sets[label] = make(d)

    # 3. pixel_motion measurement (its functions, see tools/asf_motion_measure.py).
    summary["asf_pixel_motion"] = {}
    for label, frames in sets.items():
        out = fresh(WORK / f"motion_{label}")
        r = run([py, "-B", str(SCRIPTS / "tools/asf_motion_measure.py"), str(frames[0].parent), str(out)])
        res = {"returncode": r["returncode"], "stderr": r["stderr"][-600:]}
        if (out / "measure.json").exists():
            res["measure"] = json.loads((out / "measure.json").read_text())
        summary["asf_pixel_motion"][label] = res

    # 4. build_animation_clips.
    summary["asf_build_animation_clips"] = {}
    for label, frames in sets.items():
        mf = frames[0].parent / "clips.json"
        manifest(frames, mf)
        out = fresh(WORK / f"clips_{label}")
        r = run([py, "-B", str(ASF / "generate2dsprite/scripts/build_animation_clips.py"),
                 "--manifest", str(mf), "--output-dir", str(out), "--preview-scale", "3"])
        res = {"returncode": r["returncode"], "stdout": r["stdout"][-400:], "stderr": r["stderr"][-600:]}
        meta = out / "animation-clips.json"
        if meta.exists():
            m = json.loads(meta.read_text())
            clip = m.get("clips", {}).get("run", {})
            res["run"] = {k: clip[k] for k in clip if k in ("tick_grid", "holds", "loop_seam", "events_ms", "lints", "warnings")}
            res["top_level"] = {k: m[k] for k in m if k in ("warnings", "lints", "turn_test")}
        summary["asf_build_animation_clips"][label] = res

    # 5. pixel_qa: breastplate rasters (projected, every pose) and pauldrons.
    qa_dir = fresh(WORK / "qa_chest")
    qa_dir.mkdir(parents=True)
    pngs = [raw_to_png(f, qa_dir / f.name.replace(".rgba", ".png"))
            for f in sorted((CAP / "raw").glob("chest_projected_*.rgba"))]
    ron = (ROOT / "art-src/modular-character-engine/stage/rig_lab.ron").read_text()
    ramp = re.search(r"ramp: \(\((.*?)\)\),", ron).group(1)
    trim = re.search(r"trim: \(\((.*?)\)\),", ron).group(1)
    cols = re.findall(r"(\d+), (\d+), (\d+)", ramp + "),(" + trim)
    hexes = ["#%02x%02x%02x" % tuple(int(v) for v in c) for c in cols]
    pal = qa_dir / "plate_palette.hex"
    pal.write_text("\n".join(h.lstrip("#") for h in hexes) + "\n")
    summary["asf_pixel_qa_breastplate"] = {}
    for clip in ("run", "idle"):
        group = [p for p in pngs if f"_{clip}_" in p.name]
        rep = qa_dir / f"qa_{clip}.json"
        r = run([py, "-B", str(ASF / "codeart2d/scripts/pixel_qa.py"), "--input", *map(str, group),
                 "--palette", str(pal), "--outline", hexes[0], "--review", str(qa_dir / f"review_{clip}.png"),
                 "--scales", "1,3", "--bg", "dark", "--report", str(rep), "--once"])
        summary["asf_pixel_qa_breastplate"][clip] = {
            "returncode": r["returncode"], "frames": len(group),
            "report": json.loads(rep.read_text()).get("checks") if rep.exists() else r["stderr"][-600:]}
    pq = fresh(WORK / "qa_pauldrons")
    pq.mkdir(parents=True)
    stage = ROOT / "art-src/modular-character-engine/stage/equipment"
    r = run([py, "-B", str(ASF / "codeart2d/scripts/pixel_qa.py"), "--input",
             str(stage / "pauldron_steel_half.png"), str(stage / "pauldron_bronze_half.png"),
             "--report", str(pq / "qa.json"), "--once"])
    summary["asf_pixel_qa_pauldrons"] = {"returncode": r["returncode"],
                                         "report": json.loads((pq / "qa.json").read_text()).get("checks")
                                         if (pq / "qa.json").exists() else r["stderr"][-600:]}

    # 6. pixelfx: the weapon model of rig.js, and the sword-slash act.
    r = run(["node", str(SCRIPTS / "tools/pixelfx_probe.mjs"), str(PFX)])
    summary["pixelfx_rig_probe"] = json.loads(r["stdout"]) if r["returncode"] == 0 else r
    act_dir = fresh(WORK / "pixelfx_act")
    act_dir.mkdir(parents=True)
    r = run(["node", "tools/act.mjs", "melee/sword-slash", "--out", str(act_dir / "sword-slash-act.png"), "--zoom", "3"], cwd=PFX)
    summary["pixelfx_act"] = {"returncode": r["returncode"], "stdout": r["stdout"][-400:], "stderr": r["stderr"][-400:]}

    (WORK / "external_tools.json").write_text(json.dumps(summary, indent=2) + "\n")
    print(json.dumps({k: (v.get("returncode") if isinstance(v, dict) and "returncode" in v else "...")
                      for k, v in summary.items()}, indent=1))


if __name__ == "__main__":
    main()
