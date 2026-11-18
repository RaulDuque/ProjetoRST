"""N3.6: evidence sheets for the reference audit (no sprite is made here).

Reads the owner's sheets (read only), `analysis/poses.json` from
`segment_sheets.py`, the keyed video frames and the reduced frames of
`build_run.py`, and writes to `docs/development/n3_6/evidence/`:

- `references_overview.png`: every new reference, numbered poses, sizes.
- `character_identity.png`: N3.5 candidate, video frame 0 and one standing
  pose of each sheet, side by side.
- `run_pose_analysis.png`: every run pose of every source.
- `jump_pose_analysis.png`, `attack_pose_analysis.png`,
  `idle_pose_analysis.png`: the poses of each study, numbered.

    python3 art-src/n3-6-animation-proof/scripts/analyze_references.py
"""

from __future__ import annotations

import json
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[3]
BASE = ROOT / "Md-usario/assets-user/prota/base"
HERE = ROOT / "art-src/n3-6-animation-proof"
EVIDENCE = ROOT / "docs/development/n3_6/evidence"
N35 = ROOT / "art-src/n3-5-fidelity/generated/candidate_106.png"
LIGHT = (226, 224, 228, 255)
NEUTRAL = (150, 150, 156, 255)
INK = (40, 32, 40)
RED = (200, 20, 30)


def poses() -> dict:
    return json.loads((HERE / "analysis/poses.json").read_text(encoding="utf-8"))


def sheet_image(info: dict) -> Image.Image:
    return Image.open(BASE / info["file"]).convert("RGBA")


def fit(image: Image.Image, height: int) -> Image.Image:
    width = max(1, round(image.width * height / image.height))
    return image.resize((width, height), Image.Resampling.LANCZOS)


def labelled(info: dict, scale: float, title: str) -> Image.Image:
    """The whole sheet, scaled, with every pose box and number drawn."""
    image = sheet_image(info)
    small = image.resize((round(image.width * scale), round(image.height * scale)), Image.Resampling.LANCZOS)
    out = Image.new("RGBA", (small.width, small.height + 18), LIGHT)
    out.alpha_composite(small, (0, 18))
    draw = ImageDraw.Draw(out)
    draw.text((4, 3), title, fill=INK)
    for pose in info["poses"]:
        x0, y0, x1, y1 = (round(v * scale) for v in pose["box"])
        draw.rectangle([x0, y0 + 18, x1, y1 + 18], outline=RED, width=1)
        draw.rectangle([x0, y0 + 18, x0 + 18, y0 + 30], fill=RED)
        draw.text((x0 + 2, y0 + 19), str(pose["id"]), fill=(255, 255, 255))
    return out


def stack(rows: list[Image.Image]) -> Image.Image:
    out = Image.new("RGBA", (max(r.width for r in rows), sum(r.height for r in rows) + 8 * len(rows)), LIGHT)
    y = 0
    for row in rows:
        out.alpha_composite(row, (0, y))
        y += row.height + 8
    return out


def strip(crops: list[tuple[str, Image.Image]], height: int) -> Image.Image:
    fitted = [(t, fit(c, height)) for t, c in crops]
    width = sum(c.width for _, c in fitted) + 8 * (len(fitted) + 1)
    out = Image.new("RGBA", (width, height + 30), LIGHT)
    draw = ImageDraw.Draw(out)
    x = 8
    for title, crop in fitted:
        out.alpha_composite(crop, (x, 26))
        for line, text in enumerate(title.split("\n")):
            draw.text((x, 2 + 11 * line), text, fill=INK)
        x += crop.width + 8
    return out


def crop(info: dict, pose_id: int) -> Image.Image:
    box = next(p["box"] for p in info["poses"] if p["id"] == pose_id)
    return sheet_image(info).crop(tuple(box))


def video_frame(index: int) -> Image.Image:
    image = Image.open(HERE / f"frames/video_{index:03d}.png").convert("RGBA")
    box = image.getchannel("A").getbbox() or (0, 0, image.width, image.height)
    plate = Image.new("RGBA", image.size, NEUTRAL)
    plate.alpha_composite(image)
    return plate.crop(box)


def save(image: Image.Image, name: str) -> None:
    """Photographic sheets: 256-colour PNG keeps them small enough to publish."""
    image.convert("RGB").quantize(colors=256, method=Image.Quantize.MEDIANCUT).save(EVIDENCE / name)


def main() -> None:
    EVIDENCE.mkdir(parents=True, exist_ok=True)
    p = poses()
    titles = {name: f"{info['file']}  {info['size'][0]}x{info['size'][1]}  {len(info['poses'])} poses"
              for name, info in p.items()}

    side = Image.open(BASE / "side.png").convert("RGBA")
    video = [(f"video {i}", video_frame(i)) for i in (0, 48, 51, 54, 57)]
    save(stack([labelled(info, 0.22, titles[name]) for name, info in p.items()]
          + [strip([("side.png 78x280", side)] + video, 160)]), "references_overview.png")

    n35 = Image.open(N35).convert("RGBA")
    n35 = n35.crop(n35.getchannel("A").getbbox() or (0, 0, n35.width, n35.height))
    stand = Image.open(HERE / "generated/stand.png").convert("RGBA")
    stand = stand.crop(stand.getchannel("A").getbbox() or (0, 0, stand.width, stand.height))
    identity = [
        ("N3.5 candidato\n(sprite 106 px)", n35.resize((n35.width * 3, n35.height * 3), Image.Resampling.NEAREST)),
        ("video 0 reduzido\n(sprite 106 px)", stand.resize((stand.width * 3, stand.height * 3), Image.Resampling.NEAREST)),
        ("side.png", side),
        ("video 0", video_frame(0)),
        ("run_a #2", crop(p["run_a"], 2)),
        ("run_jump #19", crop(p["run_jump"], 19)),
        ("attack #1", crop(p["attack"], 1)),
        ("idle_run_jump #1\n(3/4, outra roupa)", crop(p["idle_run_jump"], 1)),
        ("fight_left #8\n(olha para a esquerda)", crop(p["fight_left"], 8)),
    ]
    save(strip(identity, 318), "character_identity.png")

    run_rows = [
        strip([(f"run_a #{i}", crop(p["run_a"], i)) for i in range(1, 9)], 200),
        strip([(f"run_jump #{i}", crop(p["run_jump"], i)) for i in range(1, 9)], 200),
        strip([(f"idle_run_jump #{i}", crop(p["idle_run_jump"], i)) for i in range(7, 15)], 200),
        strip([(f"video {i}", video_frame(i)) for i in range(48, 60)], 200),
    ]
    save(stack(run_rows), "run_pose_analysis.png")

    jump_rows = [
        strip([(f"run_jump #{i}", crop(p["run_jump"], i)) for i in range(9, 13)], 200),
        strip([(f"run_jump #{i}", crop(p["run_jump"], i)) for i in range(13, 20)], 200),
        strip([(f"idle_run_jump #{i}", crop(p["idle_run_jump"], i)) for i in range(15, 20)], 200),
        strip([(f"fight_left #{i}", crop(p["fight_left"], i)) for i in range(28, 32)], 200),
    ]
    save(stack(jump_rows), "jump_pose_analysis.png")

    attack_rows = [
        strip([(f"attack #{i}", crop(p["attack"], i)) for i in range(1, 7)], 220),
        strip([(f"attack #{i}", crop(p["attack"], i)) for i in range(7, 13)], 220),
        strip([(f"fight_left #{i}", crop(p["fight_left"], i)) for i in range(1, 13)], 170),
        strip([(f"fight_left #{i}", crop(p["fight_left"], i)) for i in range(13, 28)], 120),
    ]
    save(stack(attack_rows), "attack_pose_analysis.png")

    idle_rows = [
        strip([(f"idle_run_jump #{i}", crop(p["idle_run_jump"], i)) for i in range(1, 7)], 260),
        strip([("side.png", side), ("video 0", video_frame(0)), ("run_jump #19", crop(p["run_jump"], 19)),
               ("attack #1", crop(p["attack"], 1)), ("attack #7", crop(p["attack"], 7))], 260),
    ]
    save(stack(idle_rows), "idle_pose_analysis.png")
    print("reference evidence written")


if __name__ == "__main__":
    main()
