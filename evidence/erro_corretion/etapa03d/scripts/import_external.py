"""Etapa 03D (Erro-Corretion): import an EXTERNAL armour picture (made in ChatGPT from
request_pose/reference.png + prompt.txt) and read it back at the game's exact size.

Agent Sprite Forge route (generate2dsprite / pixel_look, no generation here):
  pixel_look.py pose --image <picture> --request request_pose --name armour --output-dir <run>/pose
The picture is read on the approved N3.5 sprite's own lattice (cell 9 px, master at
(45, 5) of the 1024x1024 reference); only the changed cells are taken, every other cell
stays the approved sprite's own pixel.

Then this script measures, against the approved sprite (read only):
- where the result sits on the 80x128 canvas (pivot 40,118), by best overlap;
- changed texels per N3.4 material of the original grid (hair, face skin, other skin,
  shirt, trousers, leather/boots, outline), so we see whether hair, face, trousers and
  boots kept their original pixels;
- new silhouette texels and new colours.
Writes <run>/comparison.png, <run>/preview_1x_2x.png, <run>/metrics.json and keeps a copy
of the source picture. Nothing in art-src/, assets/, src/ or Blender is touched.

    python3 -I -B evidence/erro_corretion/etapa03d/scripts/import_external.py \
        --image evidence/erro_corretion/etapa03d/inbox/armour_master.png --run take01
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import subprocess
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[4]
STAGE = ROOT / "evidence/erro_corretion/etapa03d"
REQUEST = STAGE / "request_pose"
IDLE = ROOT / "art-src/n3-5-fidelity/generated/candidate_106.png"
GRID = ROOT / "art-src/n3-5-fidelity/source/candidate_106.txt"
ASF = Path(os.environ.get("ASF", Path.home() / ".claude/plugins/cache/agent-sprite-forge/agent-sprite-forge/0.6.0/skills"))
VENV = Path(os.environ.get("VENV", Path.home() / ".local/share/dc-art-tools/asf-venv"))
FACE_ROWS = 36  # skin above this row of the 80x128 canvas is the face/neck
MATERIALS = {
    "hair": set("WwhH"), "skin": set("Ssze"), "shirt": set("Ggq"), "trousers": set("Ppo"),
    "leather_boots": set("LldM"), "outline": set("K"),
}
PROTECTED = ("hair", "face", "trousers", "leather_boots")
BG = (30, 26, 38, 255)
SHEET_BG = (20, 18, 26, 255)
WHITE = (240, 240, 240)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def material(key: str, y: int) -> str:
    for name, keys in MATERIALS.items():
        if key in keys:
            return "face" if name == "skin" and y < FACE_ROWS else name
    return "other"


def place(sprite: Image.Image, original: Image.Image) -> tuple[Image.Image, tuple[int, int], float]:
    """The read-back sprite on the 80x128 canvas, at the offset where most cells equal the original."""
    orig = original.load()
    best = None
    # The feet are not part of the change, so the bottoms line up within a few rows.
    sprite_bottom = (sprite.getbbox() or (0, 0, 0, sprite.height))[3]
    original_bottom = (original.getbbox() or (0, 0, 0, original.height))[3]
    base_dy = original_bottom - sprite_bottom
    for dx in range(-sprite.width, original.width + 1):
        for dy in range(base_dy - 4, base_dy + 5):
            same = total = 0
            for y in range(0, sprite.height, 2):
                oy = y + dy
                if not 0 <= oy < original.height:
                    continue
                for x in range(0, sprite.width, 2):
                    ox = x + dx
                    if not 0 <= ox < original.width:
                        continue
                    p, q = sprite.getpixel((x, y)), orig[ox, oy]
                    if q[3]:
                        total += 1
                        same += p == q
            if total > 200 and (best is None or same / total > best[0]):
                best = (same / total, dx, dy)
    if best is None:
        raise SystemExit("the read-back sprite does not overlap the approved sprite")
    share, dx, dy = best
    canvas = Image.new("RGBA", original.size, (0, 0, 0, 0))
    canvas.alpha_composite(sprite.crop((max(0, -dx), max(0, -dy), sprite.width, sprite.height)),
                           (max(0, dx), max(0, dy)))
    return canvas, (dx, dy), share


def zoom(image: Image.Image, k: int) -> Image.Image:
    big = image.resize((image.width * k, image.height * k), Image.Resampling.NEAREST)
    base = Image.new("RGBA", big.size, BG)
    base.alpha_composite(big)
    return base


def row(images: list[Image.Image], titles: list[str], gap: int = 12) -> Image.Image:
    head = 22
    out = Image.new("RGBA", (sum(i.width for i in images) + gap * (len(images) - 1),
                             max(i.height for i in images) + head), SHEET_BG)
    draw = ImageDraw.Draw(out)
    x = 0
    for image, title in zip(images, titles, strict=True):
        out.alpha_composite(image, (x, head))
        draw.text((x + 2, 4), title, fill=WHITE)
        x += image.width + gap
    return out


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--image", required=True, type=Path, help="the external picture (PNG/JPG/WebP)")
    parser.add_argument("--run", required=True, help="name of a new folder under etapa03d/runs/")
    parser.add_argument("--new-colours", type=int, default=12, help="most colours the armour may add")
    parser.add_argument("--smallest", type=int, default=10, help="smallest changed group kept (cells)")
    args = parser.parse_args()
    run = STAGE / "runs" / args.run
    if run.exists():
        raise SystemExit(f"run folder exists: {run}")
    run.mkdir(parents=True)
    source = run / f"source{args.image.suffix.lower()}"
    shutil.copy(args.image, source)
    picture = Image.open(source)
    env = dict(os.environ, PATH=f"{VENV / 'bin'}:{os.environ.get('PATH', '')}")
    result = subprocess.run(
        [str(VENV / "bin/python"), str(ASF / "generate2dsprite/scripts/pixel_look.py"), "pose", "--image", str(source),
         "--request", str(REQUEST), "--name", "armour", "--new-colours", str(args.new_colours),
         "--smallest", str(args.smallest), "--output-dir", str(run / "pose")],
        capture_output=True, text=True, env=env, check=False)
    (run / "pixel_look_pose.log").write_text(result.stdout + result.stderr)
    if result.returncode != 0:
        print(result.stdout[-2000:], result.stderr[-2000:])
        raise SystemExit(f"pixel_look pose failed (exit {result.returncode}); see {run / 'pixel_look_pose.log'}")
    pose_line = json.loads(result.stdout.strip().splitlines()[-1])
    sprite = Image.open(run / "pose/armour.png").convert("RGBA")
    original = Image.open(IDLE).convert("RGBA")
    placed, offset, share = place(sprite, original)
    placed.save(run / "armour_on_canvas_80x128.png")
    grid = GRID.read_text(encoding="utf-8").splitlines()
    per: dict[str, dict[str, int]] = {}
    diff = Image.new("RGBA", original.size, (0, 0, 0, 0))
    grown = 0
    for y in range(original.height):
        for x in range(original.width):
            p, q = placed.getpixel((x, y)), original.getpixel((x, y))
            if q[3]:
                name = material(grid[y][x], y)
                stat = per.setdefault(name, {"texels": 0, "changed": 0})
                stat["texels"] += 1
                if p != q:
                    stat["changed"] += 1
                    diff.putpixel((x, y), (255, 60, 60, 255) if name in PROTECTED else (255, 200, 60, 255))
                else:
                    diff.putpixel((x, y), (80, 80, 92, 255))
            elif p[3]:
                grown += 1
                diff.putpixel((x, y), (60, 200, 255, 255))
    added = {c[:3] for c in placed.get_flattened_data() if c[3]} - {c[:3] for c in original.get_flattened_data() if c[3]}
    metrics = {
        "source": {"file": source.name, "sha256": sha256(source), "size": picture.size, "from": str(args.image)},
        "pixel_look_pose": pose_line,
        "placement": {"offset_on_80x128": offset, "equal_cells_share_at_offset": round(share, 4)},
        "per_material": {k: {**v, "changed_pct": round(100 * v["changed"] / v["texels"], 2)} for k, v in per.items()},
        "protected_changed_texels": sum(per.get(k, {}).get("changed", 0) for k in PROTECTED),
        "new_silhouette_texels": grown, "new_colours": len(added),
    }
    (run / "metrics.json").write_text(json.dumps(metrics, indent=1, default=str))
    view = picture.convert("RGBA").resize((original.height * 4 * picture.width // picture.height, original.height * 4))
    row([zoom(original, 4), view, zoom(placed, 4), zoom(diff, 4)],
        ["aprovado N3.5 (intocado)", "imagem externa (fonte)", "recuperada 1x (pixel_look pose)",
         "vermelho: cabelo/rosto/calca/botas mudou | amarelo: outro mudou | azul: silhueta nova"]).save(
        run / "comparison.png")
    row([zoom(original, 1), zoom(placed, 1), zoom(original, 2), zoom(placed, 2)],
        ["1x aprovado", "1x armadura", "2x aprovado", "2x armadura"]).save(run / "preview_1x_2x.png")
    print(json.dumps({k: metrics[k] for k in ("placement", "protected_changed_texels", "new_silhouette_texels",
                                              "new_colours")}, indent=1))
    print("per material:", {k: v["changed_pct"] for k, v in metrics["per_material"].items()})
    print("outputs:", run)


if __name__ == "__main__":
    main()
