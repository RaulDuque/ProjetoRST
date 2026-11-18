"""Pack the N3.1 locomotion renders into strips the owner can read.

Raw renders stay raw. The small copies are a BOX fit so the page can hold
a whole clip. Nothing here is painted.
"""

import sys
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "art-src/player/scripts/post"))
from resample import resize  # noqa: E402

OUT = ROOT / "docs/development/n3_1/evidence"
JUMP = ROOT / "art-src/player/renders/v2_2_weaponless/v1_4/masters"
WALK = ROOT / "art-src/n3-proof/blender/walk"


def fit(image: Image.Image, width: int, height: int) -> Image.Image:
    fitted = resize(image.convert("RGBA"), (width, height), Image.Resampling.BOX)
    plate = Image.new("RGBA", (width, height), (18, 12, 22, 255))
    plate.alpha_composite(fitted)
    return plate


def strip(paths: list[Path], cell: tuple[int, int], name: str) -> None:
    frames = [fit(Image.open(path), *cell) for path in paths]
    sheet = Image.new("RGBA", (cell[0] * len(frames), cell[1]), (18, 12, 22, 255))
    for index, frame in enumerate(frames):
        sheet.paste(frame, (index * cell[0], 0))
    OUT.mkdir(parents=True, exist_ok=True)
    sheet.save(OUT / name)


def main() -> None:
    rise = sorted((JUMP / "jump_rise").glob("jump_rise_W_flat_*.png"))
    fall = sorted((JUMP / "fall").glob("fall_W_flat_*.png"))
    if rise and fall:
        strip(rise + fall, (160, 120), "jump_sequence.png")
        fit(Image.open(rise[min(2, len(rise) - 1)]), 256, 192).save(OUT / "cape_jump.png")
    attack = sorted((ROOT / "art-src/n3-proof/n3_1/raw").glob("attack_*.png"))
    if attack:
        strip(attack, (160, 120), "attack_sequence.png")
        fit(Image.open(attack[0]), 256, 192).save(OUT / "cape_attack.png")
    walk = sorted(WALK.glob("walk_W_flat_*.png"))
    if len(walk) >= 16:
        strip(walk, (96, 96), "walk_sequence.png")
        print(f"walk {len(walk)}")
    before_after()
    print("sequences ok")


def before_after() -> None:
    """The N3 capsules and the N3.1 render are different drawings."""
    n3 = Image.open(ROOT / "art-src/n3-proof/generated/body_light.png")
    cell_w, cell_h = 96, 128
    attack_index = 16
    column, row = attack_index % 6, attack_index // 6
    auto = Image.open(ROOT / "art-src/n3-proof/n3_1/attack_pixel.png")
    raw = Image.open(ROOT / "art-src/n3-proof/n3_1/raw/attack_00.png")
    width, height = 220, 280
    sheet = Image.new("RGBA", (width * 3, height + 28), (24, 22, 28, 255))
    draw = ImageDraw.Draw(sheet)
    panels = [
        (
            n3.crop((column * cell_w, row * cell_h, (column + 1) * cell_w, (row + 1) * cell_h)),
            "N3 geometrico",
        ),
        (resize(raw, (256, 192), Image.Resampling.BOX), "N3.1 bruto reduzido"),
        (auto.crop((0, 0, 256, 192)), "N3.1 automatico"),
    ]
    for index, (image, label) in enumerate(panels):
        fitted = fit(image, width, height - 8)
        sheet.paste(fitted, (index * width, 28))
        draw.text((index * width + 8, 6), label, fill=(230, 226, 214, 255))
    sheet.save(OUT / "before_after_n3.png")
    silhouette = auto.crop((0, 0, 256, 192)).convert("RGBA")
    ink = Image.new("RGBA", silhouette.size, (0, 0, 0, 0))
    pixels = silhouette.load()
    ink_pixels = ink.load()
    for y in range(silhouette.height):
        for x in range(silhouette.width):
            if pixels[x, y][3] > 32:
                ink_pixels[x, y] = (232, 220, 196, 255)
    plate = Image.new("RGBA", silhouette.size, (18, 12, 22, 255))
    plate.alpha_composite(ink)
    plate.save(OUT / "silhouette_attack.png")


if __name__ == "__main__":
    main()
