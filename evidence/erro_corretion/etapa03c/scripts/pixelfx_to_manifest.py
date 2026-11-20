"""Etapa 03C (Erro-Corretion): PROPOSAL of how a pixelfx sheet would enter the Rust/Bevy game.

The game already plays effects as data, without any new code: `assets/animations/player.ron`
has an `effects` atlas (`AtlasDef`: texture, cell, columns, rows, padding, density) and
`vfx_<key>` clips (`ClipDef`: first, count, `Once(fps)`, pivot, durations), and
`src/player/visual/vfx` spawns them from a skill's `vfx` key, scaled to the real action
length. Effects only observe gameplay: they never deal damage or decide a hit.

This script converts a pixelfx `--sheet` export (PNG + Aseprite-style JSON) into:
- an atlas PNG laid out like `AtlasDef` (padding 2, `image_size` formula checked);
- optional frame decimation (30 fps -> 15 fps, durations summed: same total time);
- a palette snap: binary alpha (>= 50 %) and every colour moved to the nearest colour of
  the effect's own ramp, because pixelfx's light buffer gives thousands of blended colours;
- a RON fragment (atlas + clip) to paste into the manifest after approval.

It does NOT edit `assets/` or `src/`. Output: etapa03c/out/bevy_proposal/.

    python3 -I -B evidence/erro_corretion/etapa03c/scripts/pixelfx_to_manifest.py
"""

from __future__ import annotations

import json
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parents[4]
STAGE = ROOT / "evidence/erro_corretion/etapa03c"
OUT = STAGE / "out/bevy_proposal"
PADDING = 2
COLUMNS = 8
# Same ramps as run_tests.sh, plus the darkest outline-ish tone; hex -> RGB.
RAMPS = {
    "sword-slash": ["#120a1c", "#2a1a44", "#4a2e74", "#7c62aa", "#b6a8d6", "#e8e2f6", "#ffffff"],
    "magic-circle-cast": ["#10061a", "#2e1048", "#5a1c80", "#8e34b4", "#c070d8", "#e8b860", "#fff0c0"],
}
JOBS = [  # (pixelfx name, game vfx key, keep every Nth frame, density)
    ("sword-slash", "pixelfx_slash_gothic", 2, 2),
    ("magic-circle-cast", "pixelfx_circle_gothic", 3, 2),
]


def rgb(hex_colour: str) -> tuple[int, int, int]:
    h = hex_colour.lstrip("#")
    return int(h[0:2], 16), int(h[2:4], 16), int(h[4:6], 16)


def snap(frame: Image.Image, ramp: list[tuple[int, int, int]]) -> Image.Image:
    out = Image.new("RGBA", frame.size, (0, 0, 0, 0))
    src, dst = frame.load(), out.load()
    for y in range(frame.height):
        for x in range(frame.width):
            r, g, b, a = src[x, y]
            if a < 128:
                continue
            dst[x, y] = (*min(ramp, key=lambda c: (c[0] - r) ** 2 + (c[1] - g) ** 2 + (c[2] - b) ** 2), 255)
    return out


def convert(name: str, key: str, step: int, density: int) -> dict:
    folder = next((STAGE / "work/pixelfx").glob(f"*/{name}"))
    meta = json.loads((folder / f"{name}.json").read_text())
    sheet = Image.open(folder / f"{name}.png").convert("RGBA")
    info = meta["meta"]["pixelfx"]
    cell = tuple(info["frameSize"])
    ramp = [rgb(c) for c in RAMPS[name]]
    frames, durations = [], []
    for i in range(0, len(meta["frames"]), step):
        group = meta["frames"][i:i + step]
        r = group[0]["frame"]
        frames.append(snap(sheet.crop((r["x"], r["y"], r["x"] + r["w"], r["y"] + r["h"])), ramp))
        durations.append(round(sum(f["duration"] for f in group) / 1000, 6))
    rows = -(-len(frames) // COLUMNS)
    size = (COLUMNS * cell[0] + (COLUMNS - 1) * PADDING, rows * cell[1] + (rows - 1) * PADDING)
    atlas = Image.new("RGBA", size, (0, 0, 0, 0))
    for i, frame in enumerate(frames):
        atlas.alpha_composite(frame, ((i % COLUMNS) * (cell[0] + PADDING), (i // COLUMNS) * (cell[1] + PADDING)))
    texture = f"sprites/player/effects_{key}.png"
    atlas.save(OUT / f"effects_{key}.png")
    before = len({p[:3] for p in sheet.get_flattened_data() if p[3]})
    after = len({p[:3] for p in atlas.get_flattened_data() if p[3]})
    fps = round(len(frames) / sum(durations), 6)
    ron = (f'        "effects_{key}": (texture: "{texture}", cell: ({cell[0]}, {cell[1]}), columns: {COLUMNS}, '
           f"rows: {rows}, padding: {PADDING}, density: {density}),\n"
           f'        "vfx_{key}": (atlas: "effects_{key}", first: 0, count: {len(frames)}, mode: Once(fps: {fps}), '
           f'pivot: ({float(info["pivot"][0])}, {float(info["pivot"][1])}), fallback: None, '
           f"durations: [{', '.join(str(d) for d in durations)}], speed_reference: None),\n")
    return {"pixelfx": info["id"], "vfx_key": key, "frames_in": len(meta["frames"]), "frames_out": len(frames),
            "duration_s": round(sum(durations), 3), "cell": cell, "atlas_size": size,
            "colours_before": before, "colours_after": after, "density": density, "ron": ron,
            "act_hits": info.get("act", {}).get("hits")}


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    results = [convert(*job) for job in JOBS]
    fragment = ["// PROPOSAL (Etapa 03C) - not loaded by the game. Atlas lines go in `atlases`, clip lines in",
                "// `clips` of assets/animations/player.ron; the PNGs would go to assets/sprites/player/.",
                "// A skill uses one by setting vfx: \"<key>\" in assets/data/skills|spells/*.ron.", ""]
    fragment += [r["ron"] for r in results]
    (OUT / "manifest_fragment.ron.txt").write_text("\n".join(fragment))
    (OUT / "conversion.json").write_text(json.dumps([{k: v for k, v in r.items() if k != "ron"} for r in results],
                                                    indent=1))
    for r in results:
        print(r["vfx_key"], r["frames_in"], "->", r["frames_out"], "frames", r["duration_s"], "s",
              "colours", r["colours_before"], "->", r["colours_after"], "atlas", r["atlas_size"])


if __name__ == "__main__":
    main()
