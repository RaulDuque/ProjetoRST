"""Etapa 03C (Erro-Corretion): evidence images, GIFs and metrics from run_tests.sh outputs.

Reads etapa03c/work/ (raw tool outputs, ignored by git) and the approved sprites (read
only) and writes etapa03c/out/. Nothing here touches the game, the approved art or the
Etapa 03B Blender pipeline.

    python3 -I -B evidence/erro_corretion/etapa03c/scripts/build_evidence.py
"""

from __future__ import annotations

import json
import shutil
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[4]
STAGE = ROOT / "evidence/erro_corretion/etapa03c"
WORK = STAGE / "work"
OUT = STAGE / "out"
IDLE = ROOT / "art-src/n3-5-fidelity/generated/candidate_106.png"
RUN6 = ROOT / "art-src/n3-6-animation-proof/generated/run6.png"
EFFECTS = ROOT / "assets/sprites/player/player_effects.png"
CELL, PIVOT = (150, 128), (80, 118)  # wide comparison cell, N3.6 pivot row
BG = (30, 26, 38, 255)
SHEET_BG = (20, 18, 26, 255)
WHITE = (240, 240, 240)
# The pixelfx pivot is the feet of its 38 px stock dummy; our hero is 106 px, so the slash is
# lifted to the hero's chest (about 40 texels above the dummy's chest line).
SLASH_LIFT = 40


def rgba(path: Path) -> Image.Image:
    return Image.open(path).convert("RGBA")


def frames(folder: Path) -> list[Image.Image]:
    return [rgba(p) for p in sorted(folder.glob("*.png"))]


def on_cell(image: Image.Image, anchor) -> Image.Image:
    """An ASF finished frame placed on the N3.6 cell with its anchor on the N3.6 pivot."""
    cell = Image.new("RGBA", CELL, (0, 0, 0, 0))
    cell.alpha_composite(image, (round(PIVOT[0] - anchor[0]), round(PIVOT[1] - anchor[1])))
    return cell


def zoom(image: Image.Image, k: int, bg=BG) -> Image.Image:
    big = image.resize((image.width * k, image.height * k), Image.Resampling.NEAREST)
    base = Image.new("RGBA", big.size, bg)
    base.alpha_composite(big)
    return base


def colours(image: Image.Image) -> int:
    return len({p[:3] for p in image.get_flattened_data() if p[3]})


def feet_x(image: Image.Image) -> float | None:
    a = image.getchannel("A")
    box = a.getbbox()
    if box is None:
        return None
    row = box[3] - 1
    xs = [x for x in range(image.width) if a.getpixel((x, row))]
    return sum(xs) / len(xs)


def strip(images: list[Image.Image], k: int, gap: int = 4) -> Image.Image:
    tiles = [zoom(i, k) for i in images]
    out = Image.new("RGBA", (sum(t.width for t in tiles) + gap * (len(tiles) - 1), tiles[0].height), SHEET_BG)
    x = 0
    for t in tiles:
        out.alpha_composite(t, (x, 0))
        x += t.width + gap
    return out


def titled(rows: list[tuple[str, Image.Image]], title: str) -> Image.Image:
    width = max(r.width for _, r in rows) + 12
    height = 30 + sum(r.height + 26 for _, r in rows)
    sheet = Image.new("RGBA", (width, height), SHEET_BG)
    draw = ImageDraw.Draw(sheet)
    draw.text((6, 6), title, fill=WHITE)
    y = 30
    for label, row in rows:
        draw.text((6, y + 4), label, fill=WHITE)
        sheet.alpha_composite(row, (6, y + 22))
        y += row.height + 26
    return sheet


def save_gif(path: Path, images: list[Image.Image], durations: list[int]) -> None:
    flat = [i.convert("RGB").convert("P", palette=Image.Palette.ADAPTIVE, colors=255) for i in images]
    flat[0].save(path, save_all=True, append_images=flat[1:], duration=durations, loop=0, disposal=2)


def side_by_side(images: list[Image.Image], gap: int = 8) -> Image.Image:
    out = Image.new("RGBA", (sum(i.width for i in images) + gap * (len(images) - 1), max(i.height for i in images)),
                    SHEET_BG)
    x = 0
    for i in images:
        out.alpha_composite(i, (x, 0))
        x += i.width + gap
    return out


def run_comparison(metrics: dict) -> None:
    run = rgba(RUN6)
    n36 = [on_cell(run.crop((i * 112, 0, i * 112 + 112, 128)), (56, 118)) for i in range(run.width // 112)]
    asf = {}
    for mode in ("pixel", "hibit"):
        finish = json.loads((WORK / f"fin_{mode}" / "finish.json").read_text())
        asf[mode] = [on_cell(f, finish["anchor"]) for f in frames(WORK / f"fin_{mode}" / "frames")]
    loop = json.loads((WORK / "loop/selection.json").read_text())
    metrics["A1_video2dsprite"] = {
        "loop": {"start": loop["start"], "endExclusive": loop["endExclusive"], "frames": loop["endExclusive"] - loop["start"],
                 "source": "clean frames of the run video from 1.5 s (24 fps)"},
        "n36_run6": {"frames": len(n36), "colours_per_frame": [colours(f) for f in n36],
                     "feet_x": [feet_x(f) for f in n36]},
        **{f"asf_{m}": {"frames": len(v), "colours_per_frame": [colours(f) for f in v],
                        "colours_total": len({p[:3] for f in v for p in f.get_flattened_data() if p[3]}),
                        "feet_x": [feet_x(f) for f in v]} for m, v in asf.items()},
    }
    sheet = titled([
        ("N3.6 aprovado: run6 (6 desenhos, 12 fps, paleta N3.4, reducao propria)", strip(n36, 2)),
        ("ASF video2dsprite + gait_loop + finish pixel (12 quadros do video, 24 fps, 22 cores aprendidas)",
         strip(asf["pixel"], 2)),
        ("ASF finish hibit (mesmos 12 quadros, paleta aprendida, contorno automatico)", strip(asf["hibit"], 2)),
    ], "Teste A1 - mesmo video, mesma escala (0,2367 vs 0,2366): N3.6 x Agent Sprite Forge")
    sheet.save(OUT / "asf_run_comparison.png")
    # Animated, same speed in time: run6 at 12 fps (each drawing 2 ticks), ASF at 24 fps.
    gif = []
    for t in range(24):
        tiles = [zoom(n36[(t // 2) % len(n36)], 3), zoom(asf["pixel"][t % 12], 3), zoom(asf["hibit"][t % 12], 3)]
        gif.append(side_by_side(tiles))
    save_gif(OUT / "asf_run_comparison.gif", gif, [42] * len(gif))


def roundtrip(metrics: dict) -> None:
    motion = json.loads((WORK / "pm_fin/motion.json").read_text())
    back = frames(WORK / "pm_fin/clean")
    rows_a, rows_b, rows_d, per = [], [], [], []
    for k, frame in enumerate(back):
        i = (motion["videoFrames"][k] // 2) % 6
        original = rgba(WORK / f"run_{i}.png")
        a, b = frame.crop(frame.getbbox()), original.crop(original.getbbox())
        diff = Image.new("RGBA", b.size, (0, 0, 0, 0))
        changed = 0
        if a.size == b.size:
            for y in range(b.height):
                for x in range(b.width):
                    p, q = a.getpixel((x, y)), b.getpixel((x, y))
                    if (p[3] or q[3]) and p != q:
                        changed += 1
                        diff.putpixel((x, y), (255, 60, 60, 255))
                    elif q[3]:
                        diff.putpixel((x, y), (90, 90, 100, 255))
        opaque = sum(1 for p in b.get_flattened_data() if p[3])
        per.append({"drawing": i, "same_box": a.size == b.size, "changed_px": changed, "opaque_px": opaque,
                    "kept_pct": round(100 * (1 - changed / opaque), 2)})
        rows_a.append(b)
        rows_b.append(a)
        rows_d.append(diff)
    metrics["A3_pixel_motion_roundtrip"] = {"picked_video_frames": motion["videoFrames"],
                                            "cycle": motion["picked"], "drawings": per}
    titled([("original N3.6 (desenho)", strip(rows_a, 3)), ("lido de volta do video por pixel_motion", strip(rows_b, 3)),
            ("diferenca (vermelho = pixel mudou)", strip(rows_d, 3))],
           "Teste A3 - run6 -> video H.264 (sprite 5x sobre magenta) -> pixel_motion frames/finish").save(
        OUT / "asf_roundtrip.png")


def pixel_look(metrics: dict) -> None:
    original = rgba(IDLE)
    original = original.crop(original.getbbox())
    master = rgba(WORK / "look/master.png")
    master = master.crop(master.getbbox())
    best = None
    for dy in range(0, max(1, master.height - original.height) + 1):
        close = total = 0
        for y in range(original.height):
            for x in range(min(original.width, master.width)):
                p, q = original.getpixel((x, y)), master.getpixel((x, y + dy))
                if p[3] or q[3]:
                    total += 1
                    close += p[3] and q[3] and sum(abs(a - b) for a, b in zip(p[:3], q[:3], strict=True)) <= 36
        if best is None or close / total > best[0]:
            best = (close / total, dy)
    picture = Image.open(WORK / "picture.jpg").convert("RGBA")
    picture = picture.resize((picture.width * original.height * 3 // picture.height, original.height * 3))
    metrics["A4_pixel_look_master"] = {"original_size": original.size, "master_size": master.size,
                                       "colours_master": colours(master), "colours_original": colours(original),
                                       "texels_close_pct": round(100 * best[0], 2), "row_offset": best[1],
                                       "tolerance": "sum |dRGB| <= 36"}
    sheet = side_by_side([zoom(original, 3), picture, zoom(master, 3)], gap=16)
    titled([("original N3.5 | 'pintura' borrada 11x (JPEG) | pixel_look master", sheet)],
           "Teste A4 - pixel_look master recupera a grade 1x").save(OUT / "asf_pixel_look.png")


def breath() -> None:
    clean = frames(WORK / "breath/clean")
    motion = json.loads((WORK / "breath/motion.json").read_text())
    save_gif(OUT / "asf_idle_breath.gif", [zoom(f, 4) for f in clean], motion.get("durationsMs", [500] * len(clean)))


def effect_frames(name: str) -> tuple[list[Image.Image], dict]:
    folder = next((WORK / "pixelfx").glob(f"*/{name}"))
    meta = json.loads((folder / f"{name}.json").read_text())
    sheet = rgba(folder / f"{name}.png")
    out = []
    for item in meta["frames"]:
        r = item["frame"]
        out.append(sheet.crop((r["x"], r["y"], r["x"] + r["w"], r["y"] + r["h"])))
    return out, meta


def current_arc() -> list[Image.Image]:
    """vfx_arc_white of the game's effects atlas (cell 128x96, padding 2, first 0, 8 frames)."""
    atlas = rgba(EFFECTS)
    return [atlas.crop((i * 130, 0, i * 130 + 128, 96)) for i in range(8)]


def in_game_demo(metrics: dict) -> None:
    """Approved idle sprite + effects at the character's texel size (density 2, 1:1)."""
    idle = rgba(IDLE)
    slash, slash_meta = effect_frames("sword-slash")
    spell, spell_meta = effect_frames("magic-circle-cast")
    arc = current_arc()
    w, h, ground, hero_x = 240, 236, 226, 90

    def plate(effect: Image.Image | None, pivot, scale: int = 1, behind: bool = False, lift: int = 0,
              ahead: int = 20) -> Image.Image:
        canvas = Image.new("RGBA", (w, h), BG)
        hero = (hero_x - 40, ground - 118)
        if effect is not None:
            fx = effect.resize((effect.width * scale, effect.height * scale), Image.Resampling.NEAREST)
            pos = (hero_x - pivot[0] * scale + ahead, ground - lift - pivot[1] * scale)
            if behind:
                canvas.alpha_composite(fx, pos)
                canvas.alpha_composite(idle, hero)
                return canvas
            canvas.alpha_composite(idle, hero)
            canvas.alpha_composite(fx, pos)
            return canvas
        canvas.alpha_composite(idle, hero)
        return canvas

    gif = []
    ticks = 60  # 2 s at 30 fps
    for t in range(ticks):
        a = arc[t // 2] if t // 2 < len(arc) else None          # 20 fps clip on a 30 fps clock (approx.)
        b = slash[t] if t < len(slash) else None
        c = spell[min(t * 2, len(spell) - 1)]                    # spell shown at 2x speed to fit the loop
        panels = [plate(a, (40, 95), 2), plate(b, slash_meta["meta"]["pixelfx"]["pivot"], lift=SLASH_LIFT),
                  plate(c, spell_meta["meta"]["pixelfx"]["pivot"], behind=True, ahead=0)]
        gif.append(zoom(side_by_side([p for p in panels]), 2))
    save_gif(OUT / "pixelfx_in_game_demo.gif", gif, [33] * ticks)
    still = side_by_side([plate(arc[2], (40, 95), 2), plate(slash[3], slash_meta["meta"]["pixelfx"]["pivot"], lift=SLASH_LIFT),
                          plate(spell[60], spell_meta["meta"]["pixelfx"]["pivot"], behind=True, ahead=0)])
    titled([("atual: vfx_arc_white (atlas density 1 = 2x) | pixelfx sword-slash (gotico, 1:1) | "
             "pixelfx magic-circle-cast (roxo/dourado, 1:1)", zoom(still, 2))],
           "Teste B - efeitos sobre o protagonista aprovado (quadro de pico)").save(OUT / "pixelfx_in_game_demo.png")
    metrics["B_pixelfx"] = {
        name: {"frames": len(fr), "frame_size": m["meta"]["pixelfx"]["frameSize"], "fps": m["meta"]["pixelfx"]["fps"],
               "loop": m["meta"]["pixelfx"]["loop"], "pivot": m["meta"]["pixelfx"]["pivot"],
               "duration_s": round(sum(f["duration"] for f in m["frames"]) / 1000, 3),
               "colours": len({p[:3] for f in fr for p in f.get_flattened_data() if p[3]}),
               "act": m["meta"]["pixelfx"].get("act", {}).get("hits")}
        for name, fr, m in (("sword-slash", slash, slash_meta), ("magic-circle-cast", spell, spell_meta))}


def copy_pixelfx() -> None:
    for folder in sorted((WORK / "pixelfx").glob("*/*")):
        dest = OUT / "pixelfx" / folder.name
        dest.mkdir(parents=True, exist_ok=True)
        for item in ("anim.gif", "contact.png", f"{folder.name}.png", f"{folder.name}.json"):
            shutil.copy(folder / item, dest / item)


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    metrics: dict = {}
    run_comparison(metrics)
    roundtrip(metrics)
    pixel_look(metrics)
    breath()
    in_game_demo(metrics)
    copy_pixelfx()
    for name in ("doctor_video.json", "doctor_routes.txt", "pixelfx_check.txt"):
        shutil.copy(WORK / name, OUT / name)
    shutil.copy(WORK / "loop/aids/loop3x.gif", OUT / "asf_gait_loop3x.gif")
    shutil.copy(WORK / "triage/triage-sheet.png", OUT / "asf_triage_sheet.png")
    (OUT / "metrics.json").write_text(json.dumps(metrics, indent=1))
    print(json.dumps({k: v for k, v in metrics.items() if k != "A1_video2dsprite"}, indent=1)[:2500])


if __name__ == "__main__":
    main()
