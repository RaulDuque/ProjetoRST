"""N4-1-A: build visual equipment of a family from data (offline authoring).

The base is the DRESSED approved character: its layer grids (stage/parts/)
partition the approved idle and run frames, so the shirt torso is a layer
of its own without any nude body. For a family:

1. the shared silhouette masks are derived once from the base layers
   (``equipment/families/<family>.json``) and written to
   ``stage/equipment/families/<family>/<clip>_mask.png``;
2. every item recipe (``equipment/recipes/<id>.json``: palette, extent and
   pattern) is painted inside the family masks and written as sheets on the
   base canvas and pivot (``stage/equipment/art/<id>/<clip>.png`` + ``.txt``);
3. coverage is checked: with the item's HIDE rules (from its RON manifest),
   no texel opaque in the dressed base may become transparent (no hole);
   the report also shows what HIDE would break for items that do not cover
   the whole hidden layer (the reason such an item must not HIDE);
4. evidence sheets go to docs/development/n4/evidence/n41a/.

Usage (project root): ``python3 -B art-src/n4-sprite-machine/scripts/build_equipment.py``
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

from PIL import Image, ImageDraw

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(ROOT / "art-src/n3-4-original-protagonist/scripts"))

import reconstruct  # noqa: E402  (idle upper-arm spans: where the idle sleeve hangs)
import run_poses  # noqa: E402  (torso polygon and belt of each run pose)
from n34_palette import PALETTE  # noqa: E402

STAGE = ROOT / "art-src/n4-sprite-machine/stage"
AUTHORING = ROOT / "art-src/n4-sprite-machine/equipment"
EVIDENCE = ROOT / "docs/development/n4/evidence/n41a"
CLIPS = {"idle": (80, 128, 1), "run_c1": (112, 128, 3)}
ITEM_KEYS = "01234"  # tones; K = outline; a b c = trim tones
H = 128


# --- base composition -----------------------------------------------------------------


def base_composition() -> tuple[list[str], dict[str, list[tuple[str, dict[str, str]]]], list[str]]:
    """Layer order, the pieces of every base part (layer, clip -> texture)
    and the default outfit, read from protagonist.ron (small checked regex;
    the Rust tests parse the same file with the real loader)."""
    text = (STAGE / "protagonist.ron").read_text()
    layers = re.findall(r'"([a-z_]+)"', re.search(r"layers: \[(.*?)\]", text, re.S).group(1))
    parts = {}
    for m in re.finditer(r'"([a-z_]+)": \(region: \w+.*?\]\),', text, re.S):
        pieces = []
        for pm in re.finditer(r'\(layer: "([a-z_]+)", sheets: \{(.*?)\}\)', m.group(0), re.S):
            sheets = dict(re.findall(r'"([a-z_0-9]+)": \(texture: "([^"]+)"', pm.group(2)))
            pieces.append((pm.group(1), sheets))
        parts[m.group(1)] = pieces
    outfit = re.findall(r'"([a-z_]+)"', re.search(r"default_outfit: \[(.*?)\]", text, re.S).group(1))
    return layers, parts, outfit


def read_strip(path: Path, w: int, frames: int) -> list[dict[tuple[int, int], str]]:
    rows = path.read_text().splitlines()
    assert len(rows) == H and all(len(r) == w * frames for r in rows), path
    out = []
    for f in range(frames):
        out.append({(x, y): rows[y][f * w + x] for y in range(H) for x in range(w) if rows[y][f * w + x] != "."})
    return out


def base_layers(clip: str) -> dict[str, list[dict]]:
    """layer -> per-frame {(x, y): palette key} of the dressed base."""
    w, _, frames = CLIPS[clip]
    _, parts, _ = base_composition()
    out = {}
    for pieces in parts.values():
        for layer, sheets in pieces:
            if clip in sheets:
                out[layer] = read_strip(STAGE / sheets[clip].replace(".png", ".txt"), w, frames)
    return out


# --- family masks ---------------------------------------------------------------------


def inside_poly(p, poly) -> bool:
    x, y = p
    inside = False
    j = len(poly) - 1
    for i in range(len(poly)):
        (xi, yi), (xj, yj) = poly[i], poly[j]
        if (yi > y) != (yj > y) and x < (xj - xi) * (y - yi) / (yj - yi + 1e-9) + xi:
            inside = not inside
        j = i
    return inside


def dressed_cells(clip: str, frame: int) -> set:
    base = base_layers(clip)
    return set().union(*(set(t[frame]) for t in base.values()))


def family_masks(family: dict) -> dict[str, list[tuple[set, set, set]]]:
    """Per clip and frame: (mask, region, neckline). The mask is where items
    of the family are painted; the region is the whole torso, so an edge
    against the region (an arm or a sleeve over it) gets a shadow, and only
    an edge against the outside gets the outline. The neckline is the cut
    an item with a gorget fills back in."""
    out = {}
    for clip, rule in family["masks"].items():
        w, _, frames = CLIPS[clip]
        masks = []
        for f in range(frames):
            if rule.get("torso_of_pose"):
                pose = run_poses.FRAMES[rule["poses"][f]]
                region = {p for p in dressed_cells(clip, f)
                          if inside_poly(p, pose["torso"]) and p[1] < pose["belt_y"]}
                cells = set(region)
            else:
                region = set(read_strip(STAGE / rule["strip"], w, frames)[f])
                cells = set(region)
            if rule.get("minus_upper_arm"):
                # The idle sleeve hangs over the torso side: keep it in front.
                sleeve = {(x, y) for y, (a, b) in reconstruct.UPPER_ARM.items() for x in range(a - 1, b + 2)}
                cells -= sleeve
            cut = set()
            neck_rule = family.get("neckline")
            if neck_rule:
                if rule.get("torso_of_pose"):
                    nx, ny = run_poses.FRAMES[rule["poses"][f]]["neck"]
                    ny += 2
                else:
                    nx, ny = neck_rule["idle_neck"]
                r = neck_rule["radius"]
                cut = {p for p in cells if (p[0] - nx) ** 2 + ((p[1] - ny) * 1.3) ** 2 <= r * r}
                cells -= cut
                region -= cut
            masks.append((cells, region, cut))
        out[clip] = masks
    return out


# --- painting -------------------------------------------------------------------------


def item_cells(cells: set, region: set, neck: set, recipe: dict) -> tuple[set, set]:
    """An item with a gorget fills the family neckline back in."""
    if recipe.get("gorget"):
        return cells | neck, region | neck
    return cells, region


def paint(mask: set, recipe: dict, region: set | None = None) -> dict[tuple[int, int], str]:
    """Plate inside ``mask``: outline on the outer edge (against what is not
    ``region``), a shadow where an arm or sleeve of the region cuts it,
    light from the upper front, specular band, and the recipe's pattern
    (lames, rivets, ridge, trim)."""
    region = mask if region is None else region
    if not mask:
        return {}
    ys = [y for _, y in mask]
    top, bottom = min(ys), max(ys)
    cut = top + round((bottom - top) * recipe["extent"])
    cells = {p for p in mask if p[1] <= cut}
    rows: dict[int, list[int]] = {}
    for x, y in cells:
        rows.setdefault(y, []).append(x)
    height = max(1, cut - top)
    out = {}
    for x, y in cells:
        a, b = min(rows[y]), max(rows[y])
        t = (x - a) / max(1, b - a)
        level = 0 if t < 0.15 else 1 if t < 0.4 else 2 if t < 0.7 else 3 if t < 0.86 else 2
        v = (y - top) / height
        if v > 0.7:
            level = max(0, level - 1)
        if 0.62 <= t <= 0.74 and v < 0.55:
            level = 4
        out[(x, y)] = ITEM_KEYS[level]
    lames = recipe.get("lames")
    if lames:
        start = top + round(height * lames["from"])
        for (x, y) in list(out):
            if y >= start and (y - start) % lames["every"] == 0:
                out[(x, y)] = "0"
            elif y > start and (y - start) % lames["every"] == 1 and out[(x, y)] in "12":
                out[(x, y)] = ITEM_KEYS[int(out[(x, y)]) + 1]
    if recipe.get("ridge"):
        for y, xs in rows.items():
            a, b = min(xs), max(xs)
            rx = round(a + 0.58 * (b - a))
            if top + 2 <= y < cut - 1 and (rx, y) in out:
                out[(rx, y)] = "4"
                if (rx + 1, y) in out:
                    out[(rx + 1, y)] = "1"
    trim = recipe.get("trim")
    if trim:
        # A continuous band along the top and the bottom of every column.
        cols: dict[int, list[int]] = {}
        for x, y in cells:
            cols.setdefault(x, []).append(y)
        xs_all = sorted(cols)
        for x in xs_all:
            t = (x - xs_all[0]) / max(1, xs_all[-1] - xs_all[0])
            key = "a" if t < 0.3 else "b" if t < 0.75 else "c"
            ys_col = sorted(cols[x])
            for y in ys_col[1:3] + ys_col[-2:-1]:
                out[(x, y)] = key
    rivets = recipe.get("rivets")
    if rivets and lames:
        # One rivet at the back end of every lame.
        start = top + round(height * lames["from"])
        for y, xs in rows.items():
            if y > start and (y - start) % lames["every"] == 1:
                x = min(xs) + 2
                if (x, y) in out:
                    out[(x, y)] = "4"
    for (x, y) in list(out):
        around = ((x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1))
        if any(n not in region for n in around) or (x, y + 1) not in cells and y == cut:
            if out[(x, y)] not in "abc" or (x, y + 1) not in cells or (x - 1, y) not in cells:
                out[(x, y)] = "K"
        elif any(n not in cells for n in around):
            out[(x, y)] = "0"  # shadow cast by the arm or sleeve over the plate
    return out


def rgba(key: str, recipe: dict) -> tuple[int, int, int, int]:
    if key == "K":
        c = recipe["outline"]
    elif key in "abc":
        c = recipe["trim"]["tones"]["abc".index(key)]
    else:
        c = recipe["tones"][int(key)]
    return (c[0], c[1], c[2], 255)


# --- item manifests (HIDE rules) -------------------------------------------------------


def item_hides(item_id: str, family_id: str) -> dict[str, list[str]]:
    """Effective HIDE per clip: the item RON override, else the family RON."""
    item = (STAGE / f"equipment/items/{item_id}.ron").read_text()
    m = re.search(r"hide_layers: Some\(\{(.*?)\}\)", item, re.S)
    if m is None:
        fam = (STAGE / f"equipment/families/{family_id}.ron").read_text()
        m = re.search(r"hide_layers: \{(.*?)\},\n", fam, re.S)
    text = m.group(1) if m else ""
    return {clip: re.findall(r'"([a-z_]+)"', layers) for clip, layers in re.findall(r'"([a-z_0-9]+)": \[(.*?)\]', text)}


# --- composition, checks and evidence ---------------------------------------------------


def draw_order(clip: str, family_id: str = "torso_plate") -> list[str]:
    """Layer slots sorted like the Rust ``resolve``: z (index, or the
    family's frame_order for the clip), ties by the declared index."""
    layers, _, _ = base_composition()
    fam = (STAGE / f"equipment/families/{family_id}.ron").read_text()
    layer = re.search(r'layer: "([a-z_]+)"', fam).group(1)
    z = {name: i for i, name in enumerate(layers)}
    for c, value in re.findall(r'\(clip: "([a-z_0-9]+)", z: (-?\d+)\)', fam):
        if c == clip:
            z[layer] = int(value)
    return sorted(layers, key=lambda n: (z[n], layers.index(n)))


def compose(clip: str, frame: int, base: dict, item: dict | None, recipe: dict | None, hides: list[str]) -> Image.Image:
    w, _, _ = CLIPS[clip]
    im = Image.new("RGBA", (w, H), (0, 0, 0, 0))
    for layer in draw_order(clip):
        if layer == "armor_torso" and item is not None:
            for (x, y), k in item.items():
                im.putpixel((x, y), rgba(k, recipe))
            continue
        if layer in hides or layer not in base:
            continue
        for (x, y), k in base[layer][frame].items():
            r, g, b = PALETTE[k][:3]
            im.putpixel((x, y), (r, g, b, 255))
    return im


def holes(dressed: Image.Image, im: Image.Image) -> set:
    return {(x, y) for y in range(H) for x in range(im.width)
            if dressed.getpixel((x, y))[3] and im.getpixel((x, y))[3] == 0}


def sheet(rows, path: Path, scale: int = 3) -> None:
    tiles = [t for _, r in rows for _, t in r]
    tw = max(t.width for t in tiles) * scale
    th = H * scale
    cols = max(len(r) for _, r in rows)
    out = Image.new("RGB", (cols * (tw + 4) + 70, len(rows) * (th + 16)), (16, 14, 20))
    d = ImageDraw.Draw(out)
    for r, (label, items) in enumerate(rows):
        y = r * (th + 16)
        d.text((2, y + th // 2), label, fill=(235, 200, 110))
        for c, (name, im) in enumerate(items):
            base = Image.new("RGBA", im.size, (48, 44, 56, 255))
            base.alpha_composite(im)
            big = base.resize((im.width * scale, H * scale), Image.NEAREST).convert("RGB")
            x = 70 + c * (tw + 4)
            out.paste(big, (x, y + 14))
            d.text((x + 2, y + 1), name, fill=(235, 200, 110))
    out.save(path)


def overlay_mask(im: Image.Image, cells: set, color) -> Image.Image:
    out = im.copy()
    for p in cells:
        r, g, b, a = out.getpixel(p)
        out.putpixel(p, ((r + color[0]) // 2, (g + color[1]) // 2, (b + color[2]) // 2, 255))
    return out


def main() -> None:
    EVIDENCE.mkdir(parents=True, exist_ok=True)
    report = {}
    for family_path in sorted((AUTHORING / "families").glob("*.json")):
        family = json.loads(family_path.read_text())
        fid = family["family"]
        masks = family_masks(family)
        fam_dir = STAGE / "equipment/families" / fid
        fam_dir.mkdir(parents=True, exist_ok=True)
        for clip, frames in masks.items():
            w = CLIPS[clip][0]
            im = Image.new("RGBA", (w * len(frames), H), (0, 0, 0, 0))
            rows = [["."] * (w * len(frames)) for _ in range(H)]
            for f, (cells, _, neck) in enumerate(frames):
                for (x, y) in cells:
                    im.putpixel((f * w + x, y), (255, 255, 255, 255))
                    rows[y][f * w + x] = "#"
                for (x, y) in neck:
                    rows[y][f * w + x] = "n"  # neckline (filled by items with a gorget)
            im.save(fam_dir / f"{clip}_mask.png")
            (fam_dir / f"{clip}_mask.txt").write_text("\n".join("".join(r) for r in rows) + "\n")
        recipes = [json.loads(p.read_text()) for p in sorted((AUTHORING / "recipes").glob("*.json"))]
        recipes = [r for r in recipes if r["family"] == fid]
        for recipe in recipes:
            iid = recipe["id"]
            art = STAGE / "equipment/art" / iid
            art.mkdir(parents=True, exist_ok=True)
            hides = item_hides(iid, fid)
            rep = report.setdefault(iid, {"family": fid, "hide_layers": hides, "frames": {}})
            for clip, frames in masks.items():
                w = CLIPS[clip][0]
                base = base_layers(clip)
                strip = Image.new("RGBA", (w * len(frames), H), (0, 0, 0, 0))
                rows = [["."] * (w * len(frames)) for _ in range(H)]
                for f, (cells, region, neck) in enumerate(frames):
                    ic, ir = item_cells(cells, region, neck, recipe)
                    item = paint(ic, recipe, ir)
                    for (x, y), k in item.items():
                        strip.putpixel((f * w + x, y), rgba(k, recipe))
                        rows[y][f * w + x] = k
                    dressed = compose(clip, f, base, None, None, [])
                    worn = compose(clip, f, base, item, recipe, hides.get(clip, []))
                    hidden = set().union(*(set(base[l][f]) for l in hides.get(clip, []) if l in base))
                    # What HIDE of the shirt torso layer would expose: hidden
                    # texels the item does not cover would show whatever is
                    # under the shirt (nothing, or the C1 reconstruction,
                    # which equipment must never rely on).
                    torso_layer = "shirt_torso" if clip == "run_c1" else "shirt"
                    torso = set(base[torso_layer][f])
                    rep["frames"][f"{clip}#{f}"] = {
                        "item_texels": len(item),
                        "holes_with_its_rules": len(holes(dressed, worn)),
                        "hidden_texels": len(hidden),
                        "hidden_not_covered": len(hidden - set(item)),
                        "exposed_if_it_hid_the_torso_shirt": len(torso - set(item)),
                    }
                strip.save(art / f"{clip}.png")
                (art / f"{clip}.txt").write_text("\n".join("".join(r) for r in rows) + "\n")
        evidence(fid, masks, recipes)
        export(masks, recipes, fid)
    (EVIDENCE / "coverage.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=1))
    bad = [(i, k) for i, r in report.items() for k, v in r["frames"].items()
           if v["holes_with_its_rules"] or v["hidden_not_covered"]]
    if bad:
        sys.exit(f"items hide texels they do not cover (holes or exposed body): {bad}")


def evidence(fid: str, masks: dict, recipes: list) -> None:
    by_id = {r["id"]: r for r in recipes}
    for clip, frames in masks.items():
        base = base_layers(clip)
        torso_layer = "shirt_torso" if clip == "run_c1" else "shirt"
        rows = []
        for f, (cells, region, neck) in enumerate(frames):
            dressed = compose(clip, f, base, None, None, [])
            row = [("original", dressed), ("family mask", overlay_mask(dressed, cells, (60, 220, 120)))]
            for iid, recipe in by_id.items():
                ic, ir = item_cells(cells, region, neck, recipe)
                item = paint(ic, recipe, ir)
                hides = item_hides(iid, fid).get(clip, [])
                alone = Image.new("RGBA", dressed.size, (0, 0, 0, 0))
                for (x, y), k in item.items():
                    alone.putpixel((x, y), rgba(k, recipe))
                hidden = set().union(*(set(base[l][f]) for l in hides if l in base))
                worn = compose(clip, f, base, item, recipe, hides)
                row += [
                    (f"{iid}: item", alone),
                    (f"{iid}: hidden ({','.join(hides) or 'none'})", overlay_mask(dressed, hidden, (230, 60, 60))),
                    (f"{iid}: result", worn),
                ]
            rows.append((f"{clip} #{f}", row))
        sheet(rows, EVIDENCE / f"n41a_{clip}_overlay.png", scale=3 if clip == "idle" else 2)
        # Where plain overlay is not enough: an item that does not cover the
        # whole shirt torso, if it hid it, would expose what is under the
        # shirt (red): such an item must not HIDE (or needs a mask/variant).
        rows = []
        for f, (cells, region, neck) in enumerate(frames):
            row = []
            for iid, recipe in by_id.items():
                ic, ir = item_cells(cells, region, neck, recipe)
                item = paint(ic, recipe, ir)
                im = compose(clip, f, base, item, recipe, [torso_layer])
                for p in set(base[torso_layer][f]) - set(item):
                    im.putpixel(p, (255, 0, 64, 255))
                row.append((f"{iid} IF it hid {torso_layer}", im))
            rows.append((f"{clip} #{f}", row))
        sheet(rows, EVIDENCE / f"n41a_{clip}_hide_check.png", scale=3 if clip == "idle" else 2)


def export(families_masks: dict, recipes: list, fid: str = "torso_plate") -> None:
    """Offline exports (never at runtime): one composed spritesheet per item
    and clip, and GIFs of the C1 run (2/15 s per frame, 0.4 s per cycle,
    the run's cycle; GIF delays 13/13/14 cs)."""
    scale = 3
    crop = (12, 8, 100, 122)

    def framed(im: Image.Image, label: str) -> Image.Image:
        bg = Image.new("RGBA", im.size, (48, 44, 56, 255))
        bg.alpha_composite(im)
        t = bg.crop(crop).resize(((crop[2] - crop[0]) * scale, (crop[3] - crop[1]) * scale), Image.NEAREST)
        out = Image.new("RGB", (t.width, t.height + 14), (16, 14, 20))
        out.paste(t.convert("RGB"), (0, 14))
        ImageDraw.Draw(out).text((2, 1), label, fill=(235, 200, 110))
        return out

    def worn(clip: str, f: int, recipe: dict | None) -> Image.Image:
        base = base_layers(clip)
        if recipe is None:
            return compose(clip, f, base, None, None, [])
        cells, region, neck = families_masks[clip][f]
        ic, ir = item_cells(cells, region, neck, recipe)
        hides = item_hides(recipe["id"], fid).get(clip, [])
        return compose(clip, f, base, paint(ic, recipe, ir), recipe, hides)

    for recipe in recipes:
        for clip, frames in families_masks.items():
            w = CLIPS[clip][0]
            sheet_im = Image.new("RGBA", (w * len(frames), H), (0, 0, 0, 0))
            for f in range(len(frames)):
                sheet_im.alpha_composite(worn(clip, f, recipe), (f * w, 0))
            sheet_im.save(EVIDENCE / f"n41a_sheet_{recipe['id']}_{clip}.png")
    frames = []
    for f in range(3):
        row = [framed(worn("run_c1", f, None), f"no item #{2 * f}")]
        row += [framed(worn("run_c1", f, r), r["id"]) for r in recipes]
        out = Image.new("RGB", (sum(t.width for t in row) + 4 * (len(row) - 1), row[0].height), (0, 0, 0))
        x = 0
        for t in row:
            out.paste(t, (x, 0))
            x += t.width + 4
        frames.append(out.convert("P", palette=Image.ADAPTIVE, colors=128))
    frames[0].save(EVIDENCE / "n41a_run_c1_items.gif", save_all=True, append_images=frames[1:],
                   duration=[130, 130, 140], loop=0, disposal=1)


if __name__ == "__main__":
    main()
