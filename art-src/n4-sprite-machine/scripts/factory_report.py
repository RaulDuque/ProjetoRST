"""N41-B: quality control, diversity, determinism, performance and evidence
of the equipment factory (offline; reads what ``factory.py`` wrote).

- qa.json          TECHNICAL errors (invalidate an item) and ARTISTIC
                   warnings (need the owner's eyes) per item;
- diversity.json   per category, aligned pairs: % of visible pixels that
                   differ, silhouette difference, colour difference, palette
                   histogram distance and dHash Hamming distance (distinct
                   measures, never mixed), and a classification
                   (DUPLICATA_EXATA / VARIACAO_MENOR / REVISAR_VISUALMENTE /
                   CANDIDATO_DISTINTO; 8 % is a triage heuristic);
- determinism.json two fresh generations in target/ compared file by file,
                   and against the stage outputs;
- performance.json generation time, memory, sizes, file counts;
- bevy_pixel_check.json  every dark-background Bevy capture of
                   ``--factory`` against the composition computed from the
                   same data (0 differing pixels expected);
- evidence sheets (PNG/GIF) for the owner.

Usage (project root): ``python3 -B art-src/n4-sprite-machine/scripts/factory_report.py``
"""

from __future__ import annotations

import hashlib
import json
import re
import shutil
import sys
import time
from pathlib import Path

from PIL import Image, ImageDraw

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))

import build_equipment as be  # noqa: E402
import factory as fx  # noqa: E402
from n34_palette import PALETTE  # noqa: E402

ROOT = fx.ROOT
OUT = fx.DEFAULT_OUT
EVIDENCE = ROOT / "docs/development/n4/evidence/n41b"
DET_DIR = ROOT / "target/n41b_determinism"
BG = (48, 44, 56, 255)
TEXT = (235, 200, 110)
MINOR = 0.08  # triage threshold (fraction of visible pixels)


# --- helpers ----------------------------------------------------------------------------


def load_rgba(p: Path) -> Image.Image:
    return Image.open(p).convert("RGBA")


def frame_of(strip: Image.Image, w: int, f: int) -> Image.Image:
    return strip.crop((f * w, 0, (f + 1) * w, strip.height))


def dressed(clip: str, f: int, item: Image.Image | None = None, hides=(), mark=None) -> Image.Image:
    """The dressed character in the family draw order, the item in its slot."""
    base = be.base_layers(clip)
    w = be.CLIPS[clip][0]
    im = Image.new("RGBA", (w, be.H), (0, 0, 0, 0))
    for layer in be.draw_order(clip):
        if layer == "armor_torso" and item is not None:
            im.alpha_composite(item)
            continue
        if layer in hides or layer not in base:
            continue
        for (x, y), k in base[layer][f].items():
            im.putpixel((x, y), PALETTE[k][:3] + (255,))
    return im


def on_bg(im: Image.Image, color=BG) -> Image.Image:
    out = Image.new("RGBA", im.size, color)
    out.alpha_composite(im)
    return out


def labelled_grid(tiles: list[tuple[str, Image.Image]], cols: int, scale: int, path: Path, title: str = "") -> None:
    tw = max(t.width for _, t in tiles) * scale
    th = max(t.height for _, t in tiles) * scale
    rows = (len(tiles) + cols - 1) // cols
    top = 18 if title else 0
    out = Image.new("RGB", (cols * (tw + 6) + 6, top + rows * (th + 18) + 6), (16, 14, 20))
    d = ImageDraw.Draw(out)
    if title:
        d.text((6, 3), title, fill=TEXT)
    for i, (label, t) in enumerate(tiles):
        x = 6 + (i % cols) * (tw + 6)
        y = top + (i // cols) * (th + 18)
        d.text((x, y + 2), label[: max(8, tw // 6)], fill=TEXT)
        out.paste(on_bg(t).convert("RGB").resize((t.width * scale, t.height * scale), Image.NEAREST), (x, y + 16))
    path.parent.mkdir(parents=True, exist_ok=True)
    out.save(path)


def item_hides(rid: str) -> dict:
    text = (OUT / f"items/{rid}.ron").read_text()
    if "hide_layers: Some({})" in text:
        return {}
    fam = (fx.STAGE / "equipment/families/torso_plate.ron").read_text()
    m = re.search(r"hide_layers: \{(.*?)\},\n", fam, re.S)
    return {c: re.findall(r'"([a-z_]+)"', ls) for c, ls in re.findall(r'"([a-z_0-9]+)": \[(.*?)\]', m.group(1))} if m else {}


# --- QA ---------------------------------------------------------------------------------


def allowed_colors(recipe: dict, lib: fx.Library) -> set:
    mats = set(recipe["materials"].values()) | {d["material"] for d in recipe.get("details", [])}
    return {tuple(c) + (255,) for m in mats for c in lib.materials[m]["tones"].values()}


def qa_item(recipe: dict, lib: fx.Library, prov: dict) -> dict:
    errors: list[str] = []
    warnings: list[str] = []
    rid = recipe["id"]
    allowed = allowed_colors(recipe, lib)
    sheets = []
    if recipe["category"] == "chest_armor":
        t = lib.templates[recipe["template"]]
        for clip in t["clips"]:
            w, h, n = be.CLIPS[clip]
            sheets.append((clip, OUT / f"art/{rid}/{clip}.png", w, h, n))
        sheets.append(("preview", OUT / f"preview/{rid}.png", *be.CLIPS["run_c1"][:2], 1))
    else:
        meta = prov["summary"]["meta"]
        sheets.append(("isolated", OUT / f"isolated/{rid}.png", meta["canvas"][0], meta["canvas"][1], 1))
    stats = {}
    for clip, path, w, h, n in sheets:
        if not path.exists():
            errors.append(f"{clip}: missing {path.name}")
            continue
        raw = Image.open(path)
        if raw.mode != "RGBA":
            errors.append(f"{clip}: PNG mode {raw.mode}, expected RGBA")
        im = raw.convert("RGBA")
        if im.size != (w * n, h):
            errors.append(f"{clip}: size {im.size}, expected {(w * n, h)}")
            continue
        txt = (path.with_suffix(".txt")).read_text().splitlines()
        px = im.load()
        alphas, colors, cells = set(), {}, []
        for y in range(h):
            for x in range(w * n):
                c = px[x, y]
                alphas.add(c[3])
                if c[3]:
                    colors[c] = colors.get(c, 0) + 1
                    cells.append((x, y))
                if (c[3] == 0) != (txt[y][x] == "."):
                    errors.append(f"{clip}: TXT grid and PNG alpha disagree at {(x, y)}")
                    break
        if alphas - {0, 255}:
            errors.append(f"{clip}: partial alpha {sorted(alphas - {0, 255})} (anti-aliasing/blending)")
        foreign = [c for c in colors if c not in allowed]
        if foreign:
            errors.append(f"{clip}: {len(foreign)} colour(s) outside the declared material palettes (blending/scaling)")
        if (0, 0, 0, 255) in colors:
            errors.append(f"{clip}: pure black texels")
        if not cells:
            errors.append(f"{clip}: empty sheet")
            continue
        if any(x % w in (0, w - 1) or y in (0, h - 1) for x, y in cells):
            errors.append(f"{clip}: touches the canvas border (cropped)")
        # Per-frame checks.
        for f in range(n):
            fc = {(x - f * w, y) for x, y in cells if f * w <= x < (f + 1) * w}
            if not fc:
                errors.append(f"{clip}#{f}: empty frame")
                continue
            edge = [p for p in fc if any(q not in fc for q in ((p[0] + 1, p[1]), (p[0] - 1, p[1]), (p[0], p[1] + 1), (p[0], p[1] - 1)))]
            dark = 0
            for (x, y) in edge:
                c = px[x + f * w, y][:3]
                if sum(c) < 3 * 90:
                    dark += 1
            if edge and dark / len(edge) < 0.6:
                warnings.append(f"{clip}#{f}: weak silhouette outline ({dark}/{len(edge)} edge texels dark)")
            if clip in ("idle", "run_c1", "preview"):
                rc = "run_c1" if clip == "preview" else clip
                rf = 1 if clip == "preview" else f
                body = dressed(rc, rf)
                opaque = {(x, y) for y in range(h) for x in range(w) if body.getpixel((x, y))[3]}
                floating = fc - opaque
                if floating:
                    errors.append(f"{clip}#{f}: {len(floating)} texel(s) outside the dressed silhouette (floating)")
                if len(fc) < 40:
                    warnings.append(f"{clip}#{f}: only {len(fc)} visible texels (low legibility at this pose)")
            if clip in ("idle", "run_c1"):
                hides = item_hides(rid).get(clip, [])
                item_im = frame_of(im, w, f)
                before = dressed(clip, f)
                after = dressed(clip, f, item_im, hides)
                holes = sum(1 for y in range(h) for x in range(w) if before.getpixel((x, y))[3] and not after.getpixel((x, y))[3])
                if holes:
                    errors.append(f"{clip}#{f}: HIDE leaves {holes} hole(s)")
        main_mat = lib.materials[recipe["materials"][next(iter(recipe["materials"]))]]
        main_cols = {tuple(c) + (255,) for c in main_mat["tones"].values()}
        tones_used = len([c for c in colors if c in main_cols])
        if tones_used < 4:
            warnings.append(f"{clip}: only {tones_used} tones of the main material (volume may read flat)")
        top = max(colors.values()) / max(1, sum(colors.values()))
        if top > 0.4:
            warnings.append(f"{clip}: one colour covers {top:.0%} of the item (flat fill)")
        stats[clip] = {"texels": len(cells), "colours": len(colors), "main_tones": tones_used, "top_colour_share": round(top, 3)}
    for note in prov["summary"]["notes"]:
        warnings.append(f"detail skipped: {note}")
    tids = [recipe["template"]] if "template" in recipe else sorted(recipe["templates"].values())
    for tid in tids:
        warnings += [f"known issue: {w_}" for w_ in lib.templates[tid].get("known_issues", [])]
    return {"technical_errors": errors, "artistic_warnings": warnings, "stats": stats,
            "verdict": "TECHNICAL_ERROR" if errors else ("REVIEW" if warnings else "TECHNICALLY_VALID")}


# --- diversity --------------------------------------------------------------------------


def dhash(im: Image.Image) -> int:
    g = on_bg(im, (0, 0, 0, 255)).convert("L").resize((9, 8), Image.BOX)
    px = list(g.get_flattened_data())
    bits = 0
    for y in range(8):
        for x in range(8):
            bits = (bits << 1) | (px[y * 9 + x] > px[y * 9 + x + 1])
    return bits


def histogram(im: Image.Image) -> dict:
    h: dict = {}
    n = 0
    for c in im.get_flattened_data():
        if c[3]:
            k = (c[0] >> 4, c[1] >> 4, c[2] >> 4)
            h[k] = h.get(k, 0) + 1
            n += 1
    return {k: v / max(1, n) for k, v in h.items()}


def compare(a: Image.Image, b: Image.Image) -> dict:
    pa, pb = a.load(), b.load()
    union = inter = sil = diff = 0
    cdist = 0.0
    for y in range(a.height):
        for x in range(a.width):
            ca, cb = pa[x, y], pb[x, y]
            va, vb = ca[3] > 0, cb[3] > 0
            if va or vb:
                union += 1
                if va != vb:
                    sil += 1
                    diff += 1
                else:
                    inter += 1
                    if ca != cb:
                        diff += 1
                    cdist += sum(abs(ca[i] - cb[i]) for i in range(3)) / (3 * 255)
    ha, hb = histogram(a), histogram(b)
    hist = sum(abs(ha.get(k, 0) - hb.get(k, 0)) for k in set(ha) | set(hb)) / 2
    return {"visible_union": union, "pixel_diff": round(diff / max(1, union), 4),
            "silhouette_diff": round(sil / max(1, union), 4), "colour_diff": round(cdist / max(1, inter), 4),
            "palette_hist_l1": round(hist, 4), "dhash_hamming": bin(dhash(a) ^ dhash(b)).count("1"),
            "identical": a.tobytes() == b.tobytes()}


def classify(m: dict) -> str:
    if m["identical"]:
        return "DUPLICATA_EXATA"
    if m["pixel_diff"] < MINOR:
        return "VARIACAO_MENOR"
    if m["silhouette_diff"] < 0.03 and m["dhash_hamming"] <= 8:
        return "REVISAR_VISUALMENTE"  # same shape, other colours (a recolour)
    return "CANDIDATO_DISTINTO"


RANK = ["DUPLICATA_EXATA", "VARIACAO_MENOR", "REVISAR_VISUALMENTE", "CANDIDATO_DISTINTO"]


def diversity(recipes: list[dict]) -> dict:
    """Aligned comparisons inside each category (same canvas and pivot, same
    pose: the torso preview pose, the sword canvas, the shoulder canvas)."""
    images = {}
    for r in recipes:
        p = OUT / ("preview" if r["category"] == "chest_armor" else "isolated") / f"{r['id']}.png"
        images[r["id"]] = (r["category"] if r["category"] in ("chest_armor", "sword") else "cape_shoulder", load_rgba(p))
    pairs, per_item = [], {}
    ids = sorted(images)
    for i, a in enumerate(ids):
        for b in ids[i + 1:]:
            if images[a][0] != images[b][0] or images[a][1].size != images[b][1].size:
                continue
            m = compare(images[a][1], images[b][1])
            m["class"] = classify(m)
            pairs.append({"a": a, "b": b, **m})
            for x, y in ((a, b), (b, a)):
                cur = per_item.get(x)
                if cur is None or RANK.index(m["class"]) < RANK.index(cur["class"]) or (
                        m["class"] == cur["class"] and m["pixel_diff"] < cur["pixel_diff"]):
                    per_item[x] = {"class": m["class"], "nearest": y, "pixel_diff": m["pixel_diff"],
                                   "silhouette_diff": m["silhouette_diff"], "dhash_hamming": m["dhash_hamming"]}
    counts: dict = {}
    for v in per_item.values():
        counts[v["class"]] = counts.get(v["class"], 0) + 1
    return {"threshold_minor": MINOR, "note": "pixel_diff and dhash_hamming are different measures; 8 % is a triage "
            "heuristic, not proof of originality; artistic approval stays with the owner",
            "counts": counts, "items": per_item, "pairs": pairs}


# --- determinism, performance, Bevy -----------------------------------------------------


def tree_hashes(root: Path) -> dict:
    return {str(p.relative_to(root)): hashlib.sha256(p.read_bytes()).hexdigest()
            for p in sorted(root.rglob("*")) if p.is_file() and p.name != "batch_report.json"}


def determinism() -> dict:
    runs = []
    for name in ("a", "b"):
        d = DET_DIR / name
        if d.exists():
            shutil.rmtree(d)  # only our own scratch run inside target/
        t0 = time.perf_counter()
        rep = fx.build(d, rebuild=True)
        runs.append((d, rep, round(time.perf_counter() - t0, 3)))
    ha, hb = tree_hashes(runs[0][0]), tree_hashes(runs[1][0])
    hs = tree_hashes(OUT)
    differ_ab = sorted(k for k in set(ha) | set(hb) if ha.get(k) != hb.get(k))
    # Provenance files mention their own output root only through relative paths.
    differ_stage = sorted(k for k in ha if hs.get(k) != ha[k])
    return {"files": len(ha), "run_a_seconds": runs[0][2], "run_b_seconds": runs[1][2],
            "differ_between_runs": differ_ab, "differ_from_stage": differ_stage,
            "identical": not differ_ab and not differ_stage, "generator_version": fx.GENERATOR_VERSION}


def performance(batch: dict, det: dict) -> dict:
    files = [p for p in OUT.rglob("*") if p.is_file()]
    by_kind: dict = {}
    for p in files:
        k = p.relative_to(OUT).parts[0]
        e = by_kind.setdefault(k, {"files": 0, "bytes": 0})
        e["files"] += 1
        e["bytes"] += p.stat().st_size
    runtime_png = [p for p in OUT.glob("art/*/*.png")]
    return {
        "generation_seconds_full_batch_fresh": det["run_a_seconds"],
        "generation_seconds_per_item_mean": round(det["run_a_seconds"] / max(1, len(batch["items"])), 4),
        "generator_peak_rss_mib": batch.get("peak_rss_mib"),
        "files": len(files), "bytes": sum(p.stat().st_size for p in files), "by_kind": by_kind,
        "runtime_textures": {"files": len(runtime_png), "bytes": sum(p.stat().st_size for p in runtime_png)},
        "composition_cost": "+1 sprite (the armor_torso slot) per worn item; switching items = Outfit::wear + one "
                            "texture handle swap in sync_composite_layers (no generation at runtime)",
        "not_measured": "Bevy load time and frame cost were not profiled (isolated proof, 16 + 3x4 captures ran without "
                        "frame drops observed; no profiler attached)",
    }


IDLE_BOX = (700, 118, 1020, 630)
RUN_BOX = (656, 118, 1104, 630)


def bevy_check() -> dict:
    out = {}
    for path in sorted(EVIDENCE.glob("bevy_n41b_*.png")):
        name = path.stem
        run = name.startswith("bevy_n41b_run")
        clip = "run_c1" if run else "idle"
        m = re.search(r"_f(\d)_", name)
        frame = int(m.group(1)) // 2 if m else (2 if run else 0)
        rid = re.sub(r"^bevy_n41b_(idle|run_f\d|run_c1|idle)_(mirrored_)?", "", name)
        if rid == "none":
            expected = dressed(clip, frame)
        else:
            w = be.CLIPS[clip][0]
            item = frame_of(load_rgba(OUT / f"art/{rid}/{clip}.png"), w, frame)
            expected = dressed(clip, frame, item, item_hides(rid).get(clip, []))
        if "_mirrored_" in name:
            expected = expected.transpose(Image.FLIP_LEFT_RIGHT)
        shot = Image.open(path).convert("RGB").crop(RUN_BOX if run else IDLE_BOX)
        want = on_bg(expected, shot.getpixel((0, 0)) + (255,)).convert("RGB").resize(shot.size, Image.NEAREST)
        pa, pb = shot.load(), want.load()
        diff = sum(1 for y in range(shot.height) for x in range(shot.width) if pa[x, y] != pb[x, y])
        out[name] = {"pixels": shot.width * shot.height, "different": diff}
    return out


# --- evidence sheets --------------------------------------------------------------------


def evidence(recipes: list[dict], lib: fx.Library, qa: dict, div: dict) -> None:
    by_id = {r["id"]: r for r in recipes}
    torso = [r for r in recipes if r["category"] == "chest_armor"]
    # 1. Catalogue: every item (torso on the character at the preview pose).
    tiles = []
    for r in recipes:
        if r["category"] == "chest_armor":
            im = dressed("run_c1", 1, load_rgba(OUT / f"preview/{r['id']}.png")).crop((36, 20, 88, 72))
        else:
            im = load_rgba(OUT / f"isolated/{r['id']}.png")
            if im.height > 52:
                im = im.crop((0, 4, im.width, im.height))
        tag = {"TECHNICALLY_VALID": "ok", "REVIEW": "rev", "TECHNICAL_ERROR": "ERR"}[qa[r["id"]]["verdict"]]
        tiles.append((f"{r['id'][:22]} [{tag}]", im))
    labelled_grid(tiles, 8, 3, EVIDENCE / "n41b_catalog.png", "N41-B catalogue: 40 recipes (torso shown at run C1 #2 preview pose; others isolated)")
    # 2. Four torso templates (same material), alone and worn, idle + preview pose.
    tiles = []
    for tpl in ("segmented_cuirass", "ridged_breastplate", "brigandine_coat", "scale_hauberk"):
        rid = f"{tpl}_iron"
        prev = load_rgba(OUT / f"preview/{rid}.png")
        idle = load_rgba(OUT / f"art/{rid}/idle.png")
        tiles += [(f"{tpl} alone", prev.crop((36, 20, 88, 72))),
                  (f"{tpl} worn", dressed("run_c1", 1, prev).crop((36, 20, 88, 72))),
                  (f"{tpl} idle", dressed("idle", 0, idle).crop((14, 14, 66, 66)))]
    labelled_grid(tiles, 6, 4, EVIDENCE / "n41b_torso_templates.png", "Four torso templates, same material (iron)")
    # 3. Materials: one template x 4 materials + every material ramp.
    tiles = [(r["id"].replace("segmented_cuirass_", ""), dressed("run_c1", 1, load_rgba(OUT / f"preview/{r['id']}.png")).crop((40, 24, 84, 68)))
             for r in torso if r["template"] == "segmented_cuirass"]
    tiles += [(r["id"].replace("scale_hauberk_", ""), dressed("run_c1", 1, load_rgba(OUT / f"preview/{r['id']}.png")).crop((40, 24, 84, 68)))
              for r in torso if r["template"] == "scale_hauberk"]
    labelled_grid(tiles, 4, 4, EVIDENCE / "n41b_materials_items.png", "Same template, four materials (segmented cuirass, scale hauberk)")
    sw = Image.new("RGB", (420, 18 * len(lib.materials) + 6), (16, 14, 20))
    d = ImageDraw.Draw(sw)
    for i, (mid, m) in enumerate(sorted(lib.materials.items())):
        d.text((4, 6 + i * 18), mid, fill=TEXT)
        for j, name in enumerate(["outline"] + fx.TONE_ORDER + ["specular"]):
            if name in m["tones"]:
                d.rectangle((160 + j * 34, 4 + i * 18, 190 + j * 34, 18 + i * 18), fill=tuple(m["tones"][name]))
    sw.save(EVIDENCE / "n41b_material_ramps.png")
    # 4. Details: the library stamps and items differing only by details.
    tiles = []
    for did, dd in sorted(lib.details.items()):
        if dd["op"] != "stamp":
            continue
        pat = dd["pattern"]
        im = Image.new("RGBA", (len(pat[0]) + 2, len(pat) + 2), (0, 0, 0, 0))
        host, dm = lib.materials["iron"], lib.materials["gold"]
        for y, row in enumerate(pat):
            for x, ch in enumerate(row):
                if ch != ".":
                    mid, tone = fx.tone_of(ch, dm, host)
                    im.putpixel((x + 1, y + 1), fx.color(lib, mid, tone))
        tiles.append((did, im))
    labelled_grid(tiles, 10, 8, EVIDENCE / "n41b_detail_stamps.png", "Detail stamps (gold detail on iron host)")
    tiles = [(r["id"].replace("sword_", ""), load_rgba(OUT / f"isolated/{r['id']}.png").crop((0, 0, 21, 64))) for r in recipes if r["category"] == "sword"]
    labelled_grid(tiles, 12, 3, EVIDENCE / "n41b_swords.png", "Swords: 4 blades x 3 hilts (isolated, grip pivot; no validated hand socket)")
    tiles = [(r["id"], load_rgba(OUT / f"isolated/{r['id']}.png")) for r in recipes if r["category"] in ("cape", "shoulder")]
    labelled_grid(tiles, 6, 3, EVIDENCE / "n41b_capes_shoulders.png", "Capes (flexible), pauldrons (rigid), mantles (accessory): isolated")
    # 5. Relief maps next to the lit result.
    tiles = []
    for p in sorted((OUT / "relief").glob("*.png")):
        rl = Image.open(p).convert("L")
        rgba = Image.merge("RGBA", (rl, rl, rl, rl.point(lambda v: 255 if v else 0)))
        if p.stem.endswith("_preview") or "run_c1" in p.stem:
            rgba = rgba.crop((36, 20, 88, 72)) if p.stem.endswith("_preview") else rgba.crop((36 + 112, 20, 88 + 112, 72))
        elif p.stem.endswith("_idle"):
            rgba = rgba.crop((14, 14, 66, 66))
        tiles.append((p.stem, rgba))
    labelled_grid(tiles, 6, 3, EVIDENCE / "n41b_relief_maps.png", "Height maps 0..255 per template (height, not normals; light from the upper front)")
    # 6. Idle sample on the protagonist (one per template x material).
    tiles = [(r["id"], dressed("idle", 0, load_rgba(OUT / f"art/{r['id']}/idle.png")).crop((14, 14, 66, 82))) for r in torso]
    labelled_grid(tiles, 8, 3, EVIDENCE / "n41b_idle_all_torso.png", "Idle N3.5: the 16 torso items (the idle shirt+sleeve is one layer: no HIDE)")
    # 7. Occlusion: run C1, the item texels drawn under the near arm / sleeve.
    tiles = []
    run_items = [r for r in torso if "run_c1" in lib.templates[r["template"]]["clips"]]
    sample = run_items[0]["id"] if run_items else None
    if sample:
        strip = load_rgba(OUT / f"art/{sample}/run_c1.png")
        for f in range(3):
            item = frame_of(strip, 112, f)
            worn = dressed("run_c1", f, item, item_hides(sample).get("run_c1", []))
            occl = worn.copy()
            for y in range(128):
                for x in range(112):
                    if item.getpixel((x, y))[3] and worn.getpixel((x, y)) != item.getpixel((x, y)):
                        occl.putpixel((x, y), (255, 40, 80, 255))
            tiles += [(f"#{2 * f} worn", worn.crop((30, 16, 96, 72))), (f"#{2 * f} occluded (red)", occl.crop((30, 16, 96, 72)))]
        labelled_grid(tiles, 6, 3, EVIDENCE / "n41b_occlusion.png", f"Run C1 occlusion: {sample} under the near arm/sleeve (layer order, no shader)")
        # 8. Mirroring: the compositor flips the item with the body.
        tiles = []
        for f in range(3):
            worn = dressed("run_c1", f, frame_of(strip, 112, f), item_hides(sample).get("run_c1", []))
            tiles += [(f"#{2 * f}", worn.crop((24, 10, 100, 124))), (f"#{2 * f} mirrored", worn.transpose(Image.FLIP_LEFT_RIGHT).crop((12, 10, 88, 124)))]
        labelled_grid(tiles, 6, 2, EVIDENCE / "n41b_mirroring.png", "Mirroring: the item flips with the body (light stays at the front)")
        # 9. Run C1 sheets + GIF for the run family.
        rows = []
        for r in run_items:
            st = load_rgba(OUT / f"art/{r['id']}/run_c1.png")
            for f in range(3):
                rows.append((f"{r['id'].replace('segmented_cuirass_', '')} #{2 * f}",
                             dressed("run_c1", f, frame_of(st, 112, f), item_hides(r["id"]).get("run_c1", [])).crop((12, 8, 100, 124))))
        labelled_grid(rows, 6, 2, EVIDENCE / "n41b_run_c1_family.png", "Run C1 (#0 #2 #4 only): the segmented_cuirass family, four materials")
        frames = []
        for f in range(3):
            row = [dressed("run_c1", f).crop((12, 8, 100, 124))]
            for r in run_items:
                st = load_rgba(OUT / f"art/{r['id']}/run_c1.png")
                row.append(dressed("run_c1", f, frame_of(st, 112, f), item_hides(r["id"]).get("run_c1", [])).crop((12, 8, 100, 124)))
            im = Image.new("RGB", (len(row) * 88 * 3 + 4 * len(row), 116 * 3), (16, 14, 20))
            for i, t in enumerate(row):
                im.paste(on_bg(t).convert("RGB").resize((88 * 3, 116 * 3), Image.NEAREST), (i * (88 * 3 + 4), 0))
            frames.append(im.convert("P", palette=Image.ADAPTIVE, colors=255))
        frames[0].save(EVIDENCE / "n41b_run_c1_family.gif", save_all=True, append_images=frames[1:],
                       duration=[130, 130, 140], loop=0, disposal=1)
    # 10. Diversity: nearest neighbour of every item.
    rows = []
    for rid, v in sorted(div["items"].items()):
        a = OUT / ("preview" if by_id[rid]["category"] == "chest_armor" else "isolated") / f"{rid}.png"
        b = OUT / ("preview" if by_id[v["nearest"]]["category"] == "chest_armor" else "isolated") / f"{v['nearest']}.png"
        ia, ib = load_rgba(a), load_rgba(b)
        if by_id[rid]["category"] == "chest_armor":
            ia, ib = ia.crop((40, 24, 84, 68)), ib.crop((40, 24, 84, 68))
        rows += [(f"{rid[:18]}", ia), (f"~{v['nearest'][:12]} {v['pixel_diff']:.0%} {v['class'][:9]}", ib)]
    labelled_grid(rows, 8, 2, EVIDENCE / "n41b_diversity_nearest.png", "Each item next to its nearest neighbour (pixel diff, class)")


def spotlight(ids: list[str]) -> None:
    """Sheet for chosen items: alone at the preview pose, worn there, worn
    in the idle and the real Bevy capture (e.g. the automation proof)."""
    tiles = []
    for rid in ids:
        prev = load_rgba(OUT / f"preview/{rid}.png")
        tiles += [(f"{rid} alone", prev.crop((36, 20, 88, 72))),
                  ("worn (run #2 pose)", dressed("run_c1", 1, prev).crop((36, 20, 88, 72))),
                  ("idle", dressed("idle", 0, load_rgba(OUT / f"art/{rid}/idle.png")).crop((14, 14, 66, 66)))]
        cap = EVIDENCE / f"bevy_n41b_idle_{rid}.png"
        if cap.exists():
            shot = Image.open(cap).convert("RGBA").crop(IDLE_BOX)
            tiles.append(("Bevy capture", shot.resize((shot.width // 4, shot.height // 4), Image.NEAREST).crop((14, 14, 66, 66))))
    labelled_grid(tiles, 4, 4, EVIDENCE / "n41b_spotlight.png", "Items made from data only: " + ", ".join(ids))


def main() -> None:
    if "--spotlight" in sys.argv:
        spotlight(sys.argv[sys.argv.index("--spotlight") + 1:])
        return
    EVIDENCE.mkdir(parents=True, exist_ok=True)
    lib = fx.load_library()
    recipes, failures = fx.load_recipes(lib)
    batch = json.loads((OUT / "batch_report.json").read_text())
    qa = {}
    for r in recipes:
        prov = json.loads((OUT / f"provenance/{r['id']}.json").read_text())
        qa[r["id"]] = qa_item(r, lib, prov)
    div = diversity(recipes)
    det = determinism()
    perf = performance(batch, det)
    summary = {
        "recipes_valid": len(recipes), "recipes_failed": failures,
        "pngs_produced": len([r for r in recipes if (OUT / ("art" if r["category"] == "chest_armor" else "isolated")).exists()]),
        "technically_valid": sum(1 for v in qa.values() if not v["technical_errors"]),
        "technical_errors": sum(1 for v in qa.values() if v["technical_errors"]),
        "with_artistic_warnings": sum(1 for v in qa.values() if v["artistic_warnings"]),
        "diversity": div["counts"], "deterministic": det["identical"],
        "integrated_idle": sum(1 for r in recipes if r["category"] == "chest_armor"),
        "integrated_run_c1": sum(1 for r in recipes if r["category"] == "chest_armor"
                                 and "run_c1" in lib.templates[r["template"]]["clips"]),
        "isolated": sum(1 for r in recipes if r["category"] != "chest_armor"),
    }
    (EVIDENCE / "qa.json").write_text(json.dumps({"summary": summary, "items": qa}, indent=1, sort_keys=True) + "\n")
    (EVIDENCE / "diversity.json").write_text(json.dumps(div, indent=1, sort_keys=True) + "\n")
    (EVIDENCE / "determinism.json").write_text(json.dumps(det, indent=1, sort_keys=True) + "\n")
    (EVIDENCE / "performance.json").write_text(json.dumps(perf, indent=1, sort_keys=True) + "\n")
    if any(EVIDENCE.glob("bevy_n41b_*.png")):
        bev = bevy_check()
        (EVIDENCE / "bevy_pixel_check.json").write_text(json.dumps(bev, indent=1, sort_keys=True) + "\n")
        summary["bevy_captures"] = len(bev)
        summary["bevy_captures_with_differences"] = sorted(k for k, v in bev.items() if v["different"])
    evidence(recipes, lib, qa, div)
    print(json.dumps(summary, indent=1))


if __name__ == "__main__":
    main()
