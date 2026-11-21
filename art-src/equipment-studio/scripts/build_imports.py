"""Builds the Equipment Studio import folder (offline tool, no paid calls).

- `sources/buckle.pixelspec.json` -> Agent Sprite Forge `render_pixelspec.py`
  (PixelSpec, palette variants iron / brass) -> `import/buckle_iron.png`,
  `import/buckle_brass.png`;
- `sources/emblem.svg` -> Agent Sprite Forge `svg_render.py render --crisp`
  (pixel route: 1 unit = 1 px, 0 partial alpha) -> `import/emblem_gothic.png`;
- the N41-B factory sword (`art-src/n4-sprite-machine/stage/factory/
  isolated/sword_arming_cross.png`) copied unchanged -> `import/
  sword_arming_cross.png` (the PNG import proof).

Agent Sprite Forge is MIT, Copyright (c) 2026 0x0funky.

    PATH=~/.local/share/dc-art-tools/asf-venv/bin:$PATH \
        python -B art-src/equipment-studio/scripts/build_imports.py
"""

from __future__ import annotations

import json
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
STUDIO = ROOT / "art-src/equipment-studio"
ASF = Path.home() / ".claude/plugins/cache/agent-sprite-forge/agent-sprite-forge/0.6.0/skills/codeart2d/scripts"
WORK = ROOT / "target/equipment_studio/asf"


def run(cmd: list[str]) -> dict:
    p = subprocess.run(cmd, capture_output=True, text=True)
    if p.returncode != 0:
        raise SystemExit(f"failed: {' '.join(cmd)}\n{p.stdout}\n{p.stderr}")
    return json.loads(p.stdout.strip().splitlines()[-1])


def main() -> None:
    for d in ("buckle", "emblem"):
        if (WORK / d).exists():
            shutil.rmtree(WORK / d)
    WORK.mkdir(parents=True, exist_ok=True)
    out = STUDIO / "import"
    out.mkdir(exist_ok=True)
    report = {}
    report["render_pixelspec"] = run([sys.executable, "-B", str(ASF / "render_pixelspec.py"), "--spec",
                                      str(STUDIO / "sources/buckle.pixelspec.json"), "--output-dir",
                                      str(WORK / "buckle"), "--variants", "all", "--preview-scale", "8"])
    for variant in ("iron", "brass"):
        shutil.copyfile(WORK / "buckle" / variant / "frames/buckle.png", out / f"buckle_{variant}.png")
    report["svg_render"] = run([sys.executable, "-B", str(ASF / "svg_render.py"), "render", "--svg",
                                str(STUDIO / "sources/emblem.svg"), "--output-dir", str(WORK / "emblem"),
                                "--crisp", "--zoom", "8"])
    shutil.copyfile(WORK / "emblem/base/emblem.png", out / "emblem_gothic.png")
    shutil.copyfile(ROOT / "art-src/n4-sprite-machine/stage/factory/isolated/sword_arming_cross.png",
                    out / "sword_arming_cross.png")
    (WORK / "build_imports.json").write_text(json.dumps(report, indent=2))
    print(json.dumps({k: v.get("qa") for k, v in report.items()}))


if __name__ == "__main__":
    main()
