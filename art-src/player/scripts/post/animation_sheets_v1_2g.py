"""Sprites, experimental spritesheets, previews and review sheets of V1.2-G.

Reads the masters and the manifest written by bpy/build_animations_v007.py
(renders/v1_2g_animation_tests/, or ``--run-dir``) and the v007 build
manifest (shoulder before/after). Writes:

- ``<out-dir>/sprites/<finish>/<clip>/<clip>_<NNN>.png``: finishes A (pixel art,
  nearest, binary alpha, outline), B (high resolution, soft rim) and C (flat
  colours through B), contract naming (docs/ASSET_CONTRACT.md, section 2);
- ``<sheet-dir>/spritesheets/<clip>_<finish>.png`` + ``.json``: the contract
  grid (8 columns in locomotion, 7 in combat, row-major), 2 px padding between
  cells only (contract, section 6), at most 2048 px per side, frame times
  and durations at 24 fps and the derived 12 fps selection;
- ``<sheet-dir>/previews/<clip>_<finish>_<fps>fps.gif``: local animated previews;
- review sheets (``shoulder_before_after.png``, ``<clip>_contact_sheet.png``,
  ``<clip>_12_vs_24.png``, ``finish_abc_animation_comparison.png``,
  ``game_scale_animation_comparison.png``) and ``animation_validation_report.json``.

Not integrated into the game: nothing is written to ``assets/``.

Usage:
    python3 art-src/player/scripts/post/animation_sheets_v1_2g.py [--run-dir DIR] [--out-dir DIR] [--sheet-dir DIR]
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
from pathlib import Path

from PIL import Image, ImageDraw

import rig_sheets as S5  # helpers of the v005 sheets, unchanged
from contract import COMBAT_FRAME, COMBAT_PIVOT, FRAME, HD_DENSITY, PIVOT
from labels import font
from palette import SCENE_BACKGROUND_FALLBACK
from resample import resize

PLAYER_DIR = Path(__file__).resolve().parents[2]
RUN_DIR = PLAYER_DIR / "renders" / "v1_2g_animation_tests"
V007 = PLAYER_DIR / "renders" / "v007"
SHEETS = PLAYER_DIR / "comparisons" / "v1_2g_animation_tests"
DARK, TEXT, MUTED, PAD = S5.DARK, S5.TEXT, S5.MUTED, S5.PAD
ACCENT = (250, 210, 120)
PADDING = 2  # contract: between cells only, no outer border
COLUMNS = {"locomotion": 8, "combat": 7}  # grid columns of the contract atlases (section 6)
NAME = re.compile(r"^[a-z][a-z0-9_]*_\d{3}\.png$")
FINISHES = ("A", "B", "C")
SEVERITY_ORDER = ("Crítico", "Alto", "Médio", "Baixo", "Cosmético", "OK")
LABELS = {"walk": "Walk (caminhada)", "attack_basic": "Basic Attack", "dodge": "Dodge (esquiva)"}


# --- sprites ---------------------------------------------------------------------------------


def cell_of(clip: dict) -> tuple[tuple[int, int], tuple[float, float]]:
    return (COMBAT_FRAME, COMBAT_PIVOT) if clip["group"] == "combat" else ((FRAME, FRAME), PIVOT)


def edge_touch(master: Path) -> dict[str, int]:
    """Opaque master pixels on the left, right and top borders (a cut sprite).

    The bottom border is the floor under the pivot line, not a cut.
    """
    alpha = Image.open(master).getchannel("A")
    w, h = alpha.size
    px = alpha.load()
    return {"left": sum(1 for y in range(h) if px[0, y] > 0), "right": sum(1 for y in range(h) if px[w - 1, y] > 0),
            "top": sum(1 for x in range(w) if px[x, 0] > 0)}


def lowest_opaque_row(image: Image.Image, threshold: int = 128) -> int:
    alpha = image.getchannel("A")
    bbox = alpha.point(lambda a: 255 if a >= threshold else 0).getbbox()
    return -1 if bbox is None else bbox[3] - 1


def make_sprites(manifest: dict, out: Path) -> dict[str, dict[str, list[Path]]]:
    sprites: dict[str, dict[str, list[Path]]] = {}
    for key, clip in manifest["clips"].items():
        combat = clip["group"] == "combat"
        sprites[key] = {f: [] for f in FINISHES}
        for i in range(clip["frames"]):
            masters = manifest["renders"][key]["masters"][str(i)]
            shaded = S5.finishes(Path(masters["N_shaded"]), combat)
            flat = S5.finishes(Path(masters["N_flat"]), combat)
            for finish, image in (("A", shaded["A"]), ("B", shaded["B"]), ("C", flat["B"])):
                path = out / "sprites" / finish / key / f"{key}_{i:03d}.png"
                path.parent.mkdir(parents=True, exist_ok=True)
                image.save(path)
                sprites[key][finish].append(path)
            if "W_shaded" in masters:  # wide blade variant of the combat clip, finish B only
                path = out / "sprites" / "B_wide_blade" / key / f"{key}_{i:03d}.png"
                path.parent.mkdir(parents=True, exist_ok=True)
                S5.finishes(Path(masters["W_shaded"]), combat)["B"].save(path)
    return sprites


def spritesheet(key: str, clip: dict, finish: str, frames: list[Path], out_dir: Path) -> tuple[Path, dict]:
    density = 1 if finish == "A" else HD_DENSITY
    (cw, ch), (px, py) = cell_of(clip)
    cw, ch = cw * density, ch * density
    n = len(frames)
    columns = min(n, COLUMNS[clip["group"]])
    rows = (n + columns - 1) // columns
    sheet = Image.new("RGBA", (columns * cw + (columns - 1) * PADDING, rows * ch + (rows - 1) * PADDING),
                      (0, 0, 0, 0))
    for i, path in enumerate(frames):  # row-major, as TextureAtlasLayout::from_grid indexes cells
        col, row = i % columns, i // columns
        sheet.alpha_composite(Image.open(path).convert("RGBA"), (col * (cw + PADDING), row * (ch + PADDING)))
    path = out_dir / "spritesheets" / f"{key}_{finish}.png"
    path.parent.mkdir(parents=True, exist_ok=True)
    sheet.save(path)
    meta = {
        "id": key, "finish": finish, "group": clip["group"], "texture": path.name,
        "cell": [cw, ch], "density": density, "pivot": [px * density, py * density], "columns": columns, "rows": rows,
        "padding": PADDING, "offset": None, "first": 0, "count": n, "facing": "Right",
        "loop": clip["loop"], "mode": "Loop" if clip["loop"] else "FitToAction",
        "production_fps": clip["production_fps"], "frame_times_s": clip["times_s"],
        "frame_durations_s": clip["durations_s"], "duration_s": clip["duration_s"],
        "derived_12fps": clip["derived"]["12"], "root_motion": clip["root_motion"],
        "colour": "sRGB, straight alpha", "alpha": "binary" if finish == "A" else "soft",
        "note": "Experimental (V1.2-G). Not loaded by the game; the atlas per group of the contract is V1.3+.",
    }
    path.with_suffix(".json").write_text(json.dumps(meta, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    return path, meta


# --- previews ------------------------------------------------------------------------------


def on_room(sprite: Image.Image, density: int, zoom: float, finish: str) -> Image.Image:
    return S5.screen(sprite, "A" if finish == "A" else "B", zoom, density)


def gif(key: str, clip: dict, finish: str, frames: list[Path], fps: int, out_dir: Path) -> dict[str, object]:
    density = 1 if finish == "A" else HD_DENSITY
    zoom = 4 if finish == "A" else 3
    if fps == clip["production_fps"]:
        picked = [(i, d) for i, d in enumerate(clip["durations_s"])]
    else:
        picked = [(f["source_frame"], f["duration_s"]) for f in clip["derived"][str(fps)]]
    images = [on_room(Image.open(frames[i]).convert("RGBA"), density, zoom, finish).convert("RGB") for i, _ in picked]
    # GIF delays are whole centiseconds: 1/24 s (41.7 ms) is written as 40 ms. The
    # sum is kept close to the clip by carrying the rounding error forward.
    delays, carried = [], 0.0
    for _, d in picked:
        exact = d * 1000.0 + carried
        ms = max(10, int(round(exact / 10.0)) * 10)
        carried = exact - ms
        delays.append(ms)
    path = out_dir / "previews" / f"{key}_{finish}_{fps}fps.gif"
    path.parent.mkdir(parents=True, exist_ok=True)
    images[0].save(path, save_all=True, append_images=images[1:], duration=delays, loop=0, disposal=1)
    return {"file": path.name, "frames": len(images), "delays_ms": delays, "total_ms": sum(delays),
            "clip_ms": round(clip["duration_s"] * 1000.0, 1)}


# --- review sheets ------------------------------------------------------------------------------


def severity_of(frame: dict) -> str:
    return frame["max_severity"]


def contact_sheet(key: str, clip: dict, manifest: dict, out_dir: Path) -> Path:
    frames = clip["validation"]["frames"]
    review = manifest["renders"][key]["review"]
    cell = 210
    n = len(frames)
    cols = min(n, 8)
    rows = (n + cols - 1) // cols
    block_h = 2 * cell + 64
    sheet = Image.new("RGBA", (PAD + cols * (cell + 6), 76 + rows * (block_h + PAD)), (*DARK, 255))
    hit = ""
    if key == "attack_basic":
        hit = f" Hitbox ativa de 0 a {clip['hitbox_window_s'][1]:.3f} s (borda dourada)."
    draw = S5.header(sheet, f"{LABELS[key]}: {n} quadros a 24 fps, {clip['duration_s']:.3f} s"
                     + (" em loop" if clip["loop"] else ""),
                     "Lateral (vista do jogo) e 3/4 de costas (lado da meia-capa). t = início do quadro; "
                     "barra = pior severidade medida no quadro." + hit)
    hit_end = clip.get("hitbox_window_s", [0, -1])[1]
    for i, f in enumerate(frames):
        x = PAD + (i % cols) * (cell + 6)
        y = 70 + (i // cols) * (block_h + PAD)
        for k, view in enumerate(("side", "far_back")):
            sheet.alpha_composite(S5.on(review[str(i)][view], (cell, cell)), (x, y + k * cell))
        if f["t"] < hit_end:
            draw.rectangle((x, y, x + cell - 1, y + 2 * cell - 1), outline=ACCENT, width=2)
        draw.text((x + 4, y + 4), f"f{i:02d}  t={f['t']:.3f}s", fill=TEXT, font=font(11))
        colour = S5.SEVERITY_COLOURS.get(severity_of(f), MUTED)
        draw.rectangle((x, y + 2 * cell + 4, x + cell, y + 2 * cell + 10), fill=colour)
        shell = max(f["shell_residual_m"].values())
        draw.text((x, y + 2 * cell + 14), f"{severity_of(f)}; dura {f['duration'] * 1000:.1f} ms", fill=MUTED,
                  font=font(11))
        draw.text((x, y + 2 * cell + 30), f"casca {shell * 1000:.3f} mm; espada {f['sword_offset_drift_m']:.0e} m",
                  fill=MUTED, font=font(10))
    path = out_dir / f"{key}_contact_sheet.png"
    sheet.convert("RGB").save(path)
    return path


def timeline_sheet(key: str, clip: dict, sprites: list[Path], out_dir: Path) -> Path:
    """24 fps and derived 12 fps on one time axis: each frame is as wide as it lasts."""
    density = HD_DENSITY
    zoom = 1.5 if clip["group"] == "combat" else 2.0
    tiles = [on_room(Image.open(p).convert("RGBA"), density, zoom, "B") for p in sprites]
    tile_w, tile_h = tiles[0].size
    scale = (tile_w + 4) * 24.0  # px per second: one 24 fps frame = one tile
    width = 140 + round(clip["duration_s"] * scale) + tile_w + PAD
    sheet = Image.new("RGBA", (int(width), 80 + 2 * (tile_h + 60)), (*DARK, 255))
    draw = S5.header(sheet, f"{LABELS[key]}: 24 fps × 12 fps derivado (eixo de tempo comum)",
                     "Cada quadro começa no seu instante; a barra mostra quanto dura. 12 fps = um quadro sim, um não, "
                     f"dos 24 fps. Duração do clipe: {clip['duration_s']:.3f} s nos dois.")
    rows = (("24 fps", [(i, d) for i, d in enumerate(clip["durations_s"])]),
            ("12 fps", [(f["source_frame"], f["duration_s"]) for f in clip["derived"]["12"]]))
    for r, (label, picked) in enumerate(rows):
        y = 74 + r * (tile_h + 60)
        draw.text((PAD, y + tile_h // 2 - 8), label, fill=TEXT, font=font(15))
        draw.text((PAD, y + tile_h // 2 + 12), f"{len(picked)} quadros", fill=MUTED, font=font(11))
        t = 0.0
        for n, (i, d) in enumerate(picked):
            x = 140 + round(t * scale)
            sheet.alpha_composite(tiles[i], (x, y))
            bar_colour = ACCENT if n % 2 == 0 else (120, 160, 210)
            draw.rectangle((x, y + tile_h + 6, x + max(2, round(d * scale)) - 2, y + tile_h + 14), fill=bar_colour)
            draw.text((x, y + tile_h + 18), f"f{i:02d} {t:.3f}s", fill=MUTED, font=font(10))
            t += d
    path = out_dir / f"{key}_12_vs_24.png"
    sheet.convert("RGB").save(path)
    return path


def finish_sheet(manifest: dict, sprites: dict, out_dir: Path) -> Path:
    zoom = 2.0
    rows = []
    for key in manifest["clips"]:
        for finish in FINISHES:
            rows.append((key, finish, sprites[key][finish]))
        if "B_wide_blade" in sprites.get(key, {}):
            rows.append((key, "B, lâmina ×1,7", sprites[key]["B_wide_blade"]))
    widths = []
    for key, finish, frames in rows:
        density = 1 if finish == "A" else HD_DENSITY
        tile = on_room(Image.open(frames[0]).convert("RGBA"), density, zoom, finish[0])
        widths.append(len(frames) * (tile.size[0] + 3))
    row_h = {k: on_room(Image.open(f[0]).convert("RGBA"), 1 if fi == "A" else HD_DENSITY, zoom, fi[0]).size[1]
             for k, fi, f in rows}
    sheet = Image.new("RGBA", (230 + max(widths) + PAD, 74 + sum(row_h[k] + 26 for k, _, _ in rows)), (*DARK, 255))
    draw = S5.header(sheet, "Acabamentos A, B e C nas três animações (×2, fundo da sala)",
                     "A: pixel art (nearest, alfa binário, contorno). B: alta resolução (2 texels/u, borda suave). "
                     "C: cores chapadas via B. Nenhum foi escolhido.")
    y = 70
    for key, finish, frames in rows:
        density = 1 if finish[0] == "A" else HD_DENSITY
        draw.text((PAD, y + 6), LABELS[key], fill=TEXT, font=font(13))
        draw.text((PAD, y + 24), f"acabamento {finish}", fill=MUTED, font=font(12))
        x = 230
        for p in frames:
            tile = on_room(Image.open(p).convert("RGBA"), density, zoom, finish[0])
            sheet.alpha_composite(tile, (x, y))
            x += tile.size[0] + 3
        y += row_h[key] + 26
    path = out_dir / "finish_abc_animation_comparison.png"
    sheet.convert("RGB").save(path)
    return path


KEY_FRAMES = {"walk": (0, 4), "attack_basic": (0, 3), "dodge": (2, 6)}


def game_scale_sheet(manifest: dict, sprites: dict, out_dir: Path) -> Path:
    zooms = S5.ZOOMS
    items = [(k, i) for k, fs in KEY_FRAMES.items() for i in fs]
    tiles = {}
    for key, i in items:
        for finish in FINISHES:
            density = 1 if finish == "A" else HD_DENSITY
            spr = Image.open(sprites[key][finish][i]).convert("RGBA")
            tiles[(key, i, finish)] = [on_room(spr, density, z, finish) for z in zooms]
    group_w = max(sum(t.size[0] + 4 for t in tiles[(k, i, f)]) for k, i in items for f in FINISHES) + PAD
    row_h = {(k, i): max(t.size[1] for f in FINISHES for t in tiles[(k, i, f)]) + 30 for k, i in items}
    sheet = Image.new("RGBA", (220 + 3 * group_w, 96 + sum(row_h.values())), (*DARK, 255))
    draw = S5.header(sheet, "Animações na escala do jogo: A, B, C × câmeras 1:1, ×1,5, ×2 (quadros-chave)",
                     "Comparação externa; a câmera do Bevy não muda. 1:1 = ~52 px de altura (célula 64×64, pivô "
                     "(32, 63); ataque 128×96, pivô (40, 95)). D2 e D3 continuam pendentes.")
    for k, finish in enumerate(FINISHES):
        draw.text((220 + k * group_w, 64), f"{finish}: 1:1  ×1,5  ×2", fill=TEXT, font=font(13))
    y = 90
    for key, i in items:
        draw.text((PAD, y + 4), f"{LABELS[key]} f{i:02d}", fill=TEXT, font=font(12))
        for k, finish in enumerate(FINISHES):
            x = 220 + k * group_w
            for t in tiles[(key, i, finish)]:
                sheet.alpha_composite(t, (x, y + row_h[(key, i)] - 28 - t.size[1] + 20))
                x += t.size[0] + 4
        y += row_h[(key, i)]
    path = out_dir / "game_scale_animation_comparison.png"
    sheet.convert("RGB").save(path)
    return path


def shoulder_sheet(v007: dict, out_dir: Path) -> Path:
    images = v007["shoulder_images"]
    poses = list(images["v007"])
    cell = 190
    views = ("close_pauldron", "close_pauldron_back")
    cols = 2
    block_w = 4 * cell + 3 * 4
    rows = (len(poses) + cols - 1) // cols
    sheet = Image.new("RGBA", (PAD + cols * (block_w + PAD), 96 + rows * (cell + 56)), (*DARK, 255))
    draw = S5.header(sheet, "Ombreira v006 × v007: antes | depois (mesma câmera, luz e pose)",
                     "Resíduo = quanto a casca final salta quando o rig é empurrado 1 µm / 1 µrad (estável ≈ 0,002 mm). "
                     "v006: polo da placa colapsado em 37 vértices (36 faces de área zero).")
    draw.text((PAD, 60), "Em repouso a casca nova fica a ≤ " + ", ".join(
        f"{n} {v * 1000:.1f} mm" for n, v in v007["rest_shell_change_m"].items()) + " da antiga.", fill=MUTED,
        font=font(12))
    for n, key in enumerate(poses):
        x = PAD + (n % cols) * (block_w + PAD)
        y = 84 + (n // cols) * (cell + 56)
        for k, (tag, view) in enumerate(((t, v) for v in views for t in ("v006", "v007"))):
            sheet.alpha_composite(S5.on(images[tag][key][view], (cell, cell)), (x + k * (cell + 4), y + 18))
            draw.text((x + k * (cell + 4) + 4, y + 22), tag, fill=ACCENT if tag == "v007" else TEXT, font=font(11))
        before = max(v007["poses_v006"][key]["shell_residual_m"][p] for p in ("pauldron", "pauldron_lame"))
        after = max(v007["poses"][key]["shell_residual_m"][p] for p in ("pauldron", "pauldron_lame"))
        draw.text((x, y), f"{key}: resíduo {before * 1000:.3f} mm → {after * 1000:.4f} mm", fill=TEXT, font=font(12))
    path = out_dir / "shoulder_before_after.png"
    sheet.convert("RGB").save(path)
    return path


# --- validation -------------------------------------------------------------------------------


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def validate(manifest: dict, sprites: dict, sheets: dict) -> dict[str, object]:
    report: dict[str, object] = {"clips": {}, "failures": []}
    fails: list[str] = report["failures"]  # type: ignore[assignment]
    for key, clip in manifest["clips"].items():
        (cw, ch), (px, py) = cell_of(clip)
        frames = clip["validation"]["frames"]
        entry: dict[str, object] = {}
        # Timing.
        entry["duration_s"] = clip["duration_s"]
        if key in ("attack_basic", "dodge"):
            target = manifest["game_constants"]["BASIC_ATTACK_TIME" if key == "attack_basic" else "PLAYER_DODGE_TIME"]
            entry["duration_matches_game"] = abs(clip["duration_s"] - target) < 1e-6
            if not entry["duration_matches_game"]:
                fails.append(f"{key}: {clip['duration_s']} s, game {target} s")
        derived_total = sum(f["duration_s"] for f in clip["derived"]["12"])
        entry["derived_12fps_duration_s"] = round(derived_total, 6)
        if abs(derived_total - clip["duration_s"]) > 1e-5:
            fails.append(f"{key}: 12 fps selection lasts {derived_total} s")
        # Rig and deformation per frame.
        entry["worst_severity"] = min((f["max_severity"] for f in frames), key=SEVERITY_ORDER.index)
        entry["frames_medio_or_worse"] = [f["index"] for f in frames if f["max_severity"] in ("Crítico", "Alto", "Médio")]
        entry["max_sword_drift_m"] = max(f["sword_offset_drift_m"] for f in frames)
        entry["max_shell_residual_m"] = max(max(f["shell_residual_m"].values()) for f in frames)
        entry["scale_failures"] = [f["index"] for f in frames if f["scale_failures"]]
        entry["pops"] = {f["index"]: f["step_from_previous"]["pops"] for f in frames[1:] if f["step_from_previous"]["pops"]}
        entry["max_body_step_m"] = max(f["step_from_previous"]["max_body_m"] for f in frames[1:])
        entry["lowest_z_range_m"] = [min(f["lowest_z_m"] for f in frames), max(f["lowest_z_m"] for f in frames)]
        entry["min_boot_volume_ratio"] = min(min(f["boot_volume_ratio"].values()) for f in frames)
        if clip["loop"]:
            v = clip["validation"]
            entry["loop_closure_max_m"] = v["loop_closure_max_m"]
            entry["loop_step_max_body_m"] = v["loop_step"]["max_body_m"]
            steps = [f["step_from_previous"]["max_body_m"] for f in frames[1:]]
            entry["loop_step_vs_mean_step"] = round(v["loop_step"]["max_body_m"] / (sum(steps) / len(steps)), 3)
            if v["loop_closure_max_m"] > 1e-4:
                fails.append(f"{key}: loop does not close ({v['loop_closure_max_m']} m)")
        if entry["max_sword_drift_m"] > 1e-5:
            fails.append(f"{key}: sabre drifts {entry['max_sword_drift_m']} m")
        if entry["max_shell_residual_m"] > 1e-4:
            fails.append(f"{key}: unstable shell {entry['max_shell_residual_m']} m")
        if entry["pops"]:
            fails.append(f"{key}: pops {entry['pops']}")
        if entry["scale_failures"]:
            fails.append(f"{key}: pose scale changed")
        if entry["frames_medio_or_worse"]:
            fails.append(f"{key}: Médio or worse in frames {entry['frames_medio_or_worse']}")
        # Masters: cuts and pivot line.
        masters = manifest["renders"][key]["masters"]
        touches = {}
        for i, m in masters.items():
            for variant, path in m.items():
                t = edge_touch(Path(path))
                if any(t.values()):
                    touches[f"{i}:{variant}"] = t
        entry["masters_touching_cell_edges"] = touches
        if touches:
            fails.append(f"{key}: the figure reaches the cell edge in {sorted(touches)} (possible cut, not cropped)")
        # Sprites: names, sizes, alpha, pivot line, duplicates.
        sprite_info = {}
        for finish, paths in sprites[key].items():
            density = 1 if finish == "A" else HD_DENSITY
            size = (cw * density, ch * density)
            lows, hashes, bad = [], [], []
            for p in paths:
                img = Image.open(p)
                if not NAME.match(p.name):
                    bad.append(f"name {p.name}")
                if img.size != size or img.mode != "RGBA":
                    bad.append(f"{p.name} {img.size} {img.mode}")
                if "icc_profile" in img.info:
                    bad.append(f"{p.name} has an ICC profile")
                partial = sum(img.getchannel("A").histogram()[1:255])
                if finish == "A" and partial:
                    bad.append(f"{p.name} alpha not binary")
                lows.append(lowest_opaque_row(img))
                hashes.append(sha(p))
            dups = [i for i in range(1, len(hashes)) if hashes[i] == hashes[i - 1]]
            if clip["loop"] and hashes and hashes[0] == hashes[-1]:
                dups.append(0)
            expected = py * density
            sprite_info[finish] = {"size": list(size), "frames": len(paths), "lowest_opaque_row": [min(lows), max(lows)],
                                   "pivot_y": expected, "duplicates": dups, "problems": bad}
            if bad:
                fails.append(f"{key} {finish}: {bad[:3]}")
            if dups:
                fails.append(f"{key} {finish}: duplicated consecutive frames {dups}")
            # The sole sits on the pivot line: the lowest opaque row is the pivot row (A: the
            # outline may add the row below it, which is still inside the cell).
            if not (expected - 2 * density <= min(lows) and max(lows) <= expected + density):
                fails.append(f"{key} {finish}: lowest opaque rows {min(lows)}-{max(lows)}, pivot y {expected}")
        entry["sprites"] = sprite_info
        report["clips"][key] = entry
    report["sheets"] = {k: v.name for k, v in sheets.items()}  # names only: the report is path-independent
    return report


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--run-dir", type=Path, default=RUN_DIR)
    parser.add_argument("--out-dir", type=Path, default=None, help="sprites (default: <run-dir>)")
    parser.add_argument("--sheet-dir", type=Path, default=SHEETS)
    args = parser.parse_args()
    out = args.out_dir or args.run_dir
    sheet_dir = args.sheet_dir
    sheet_dir.mkdir(parents=True, exist_ok=True)
    manifest = json.loads((args.run_dir / "animation_manifest.json").read_text(encoding="utf-8"))
    v007 = json.loads((V007 / "rig_manifest_v007.json").read_text(encoding="utf-8"))
    sprites = make_sprites(manifest, out)
    sheets: dict[str, Path] = {"shoulder": shoulder_sheet(v007, sheet_dir)}
    spritesheets, previews = {}, {}
    for key, clip in manifest["clips"].items():
        sheets[f"{key}_contact"] = contact_sheet(key, clip, manifest, sheet_dir)
        sheets[f"{key}_12_vs_24"] = timeline_sheet(key, clip, sprites[key]["B"], sheet_dir)
        for finish in FINISHES:
            path, meta = spritesheet(key, clip, finish, sprites[key][finish], sheet_dir)
            spritesheets[f"{key}_{finish}"] = {"file": path.name, "size": Image.open(path).size, "sha256": sha(path)}
            for fps in (24, 12):
                previews[f"{key}_{finish}_{fps}"] = gif(key, clip, finish, sprites[key][finish], fps, sheet_dir)
    sheets["finish_abc"] = finish_sheet(manifest, sprites, sheet_dir)
    sheets["game_scale"] = game_scale_sheet(manifest, sprites, sheet_dir)
    report = validate(manifest, sprites, sheets)
    report["spritesheets"] = spritesheets
    for name, info in spritesheets.items():  # contract: conservative texture limit of 2048 px per side
        if max(info["size"]) > 2048:
            report["failures"].append(f"spritesheet {name} is {info['size']}, above 2048 px")
    report["previews"] = previews
    report["sprite_sha256"] = {f"{k}/{f}/{p.name}": sha(p) for k, fs in sprites.items() for f, ps in fs.items()
                               for p in ps}
    report["walk_speed"] = {"animation_px_s": manifest["clips"]["walk"]["walk_speed_px_s"],
                            "game_px_s": manifest["clips"]["walk"]["game_walk_speed_px_s"]}
    (sheet_dir / "animation_validation_report.json").write_text(
        json.dumps(report, indent=2, ensure_ascii=False, default=str) + "\n", encoding="utf-8")
    for name, path in sheets.items():
        size = Image.open(path).size
        print(f"[anim_sheets] {name}: {path.name} {size[0]}x{size[1]}")
    print(f"[anim_sheets] failures: {report['failures']}")


if __name__ == "__main__":
    main()
