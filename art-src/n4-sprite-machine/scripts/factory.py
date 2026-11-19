"""N41-B: data-driven equipment factory (offline authoring, never at runtime).

Five parts, all driven by JSON under ``art-src/n4-sprite-machine/factory/``:

A. templates/  reusable shapes (torso cuts on the family masks, blades,
               hilts, shoulder pieces) with their relief (height map) ops;
B. materials/  tone ramps (outline, deep, shadow, base, light, highlight,
               optional specular) plus gloss, roughness, contrast, wear;
C. details/    reusable ornaments by generic ops: ``stamp`` (pixel pattern),
               ``edge_band`` (trim / embroidery), ``line`` (seams,
               stitches), ``scatter`` (seeded wear);
D. generator   this file: validates every reference, builds the shape, the
               height map, lights it, quantizes into the material ramp,
               applies selective outlines and the details, and writes PNG +
               TXT grids on integer texels (no scaling, no blending);
E. integrator  torso items become RON manifests of the existing
               ``torso_plate`` family (read by equipment_visual.rs, drawn by
               the one compositor and VisualPlayback); swords, capes and
               shoulder pieces stay isolated (no validated socket/slot).

A recipe that only uses existing templates, materials and details needs no
code. Deterministic: explicit seeds (string-seeded local RNG), sorted
inputs, a provenance manifest per item (recipe, input and output SHA-256,
generator version) and a cache keyed on all of them.

Usage (project root):
    python3 -B art-src/n4-sprite-machine/scripts/factory.py [--out DIR] [--rebuild] [--only ID ...]
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import random
import re
import resource
import sys
import time
from dataclasses import dataclass, field
from pathlib import Path

from PIL import Image

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
sys.path.insert(0, str(HERE))

import build_equipment as be  # noqa: E402  (family masks, base layers, draw order)

GENERATOR_VERSION = "n41b-1"
SCHEMA = 1
FACTORY = ROOT / "art-src/n4-sprite-machine/factory"
STAGE = ROOT / "art-src/n4-sprite-machine/stage"
DEFAULT_OUT = STAGE / "factory"
LIGHT = (0.55, -0.65, 0.55)  # upper front (the character faces +x)
TONE_ORDER = ["deep", "shadow", "base", "light", "highlight"]
REQUIRED_TONES = ["outline", "shadow", "base", "highlight"]
TONE_CHARS = {"o": "outline", "d": "deep", "s": "shadow", "b": "base", "l": "light", "h": "highlight", "p": "specular"}
BAYER4 = [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]]
ID_RE = re.compile(r"^[a-z][a-z0-9_]*$")
CATEGORY_KIND = {"chest_armor": "torso", "sword": "blade+hilt", "cape": "shoulder", "shoulder": "shoulder"}
RELIEF_OPS = {
    "cylinder": {"amp"}, "dome": {"u", "v", "r", "amp"}, "bands": {"from", "every", "depth"},
    "ridge": {"u", "amp", "width"}, "rim": {"edge", "width", "amp"}, "grid": {"every_x", "every_y", "depth"},
    "skirt_segments": {"depth"}, "scales": {"w", "h", "depth"}, "folds": {"count", "amp"}, "drape": {"amp"},
    "blade_bevel": {"amp"}, "fuller": {"from", "to", "width", "depth"},
}
DETAIL_OPS = {"stamp": {"pattern"}, "edge_band": {"width"}, "line": {"dash", "tone"}, "scatter": {"tones"}}
RECIPE_KEYS = {"schema", "id", "category", "family", "name", "template", "templates", "materials", "details",
               "generation", "covers", "hides", "mirroring", "fallback", "variants"}
SWORD_CANVAS = (21, 64)


class FactoryError(Exception):
    """Invalid data (never silently accepted)."""


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def canonical(obj) -> bytes:
    return json.dumps(obj, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()


def rel(p: Path) -> str:
    return str(p.resolve().relative_to(ROOT))


# --- library and validation ------------------------------------------------------------


@dataclass
class Library:
    root: Path
    templates: dict = field(default_factory=dict)
    materials: dict = field(default_factory=dict)
    details: dict = field(default_factory=dict)
    profiles: dict = field(default_factory=dict)
    paths: dict = field(default_factory=dict)  # (kind, id) -> Path


def _need(cond: bool, msg: str) -> None:
    if not cond:
        raise FactoryError(msg)


def _rgb(c, where: str) -> None:
    _need(isinstance(c, list) and len(c) == 3 and all(isinstance(v, int) and 0 <= v <= 255 for v in c),
          f"{where}: expected [r, g, b] integers 0..255, got {c!r}")


def validate_material(m: dict, where: str) -> None:
    tones = m.get("tones")
    _need(isinstance(tones, dict), f"{where}: missing tones")
    for t in REQUIRED_TONES:
        _need(t in tones, f"{where}: tone {t!r} is required (outline, shadow, base, highlight)")
    for name, c in tones.items():
        _need(name in TONE_CHARS.values(), f"{where}: unknown tone {name!r}")
        _rgb(c, f"{where}.{name}")
    _need(tones["outline"] != [0, 0, 0], f"{where}: pure black outline is not allowed (use a coloured sel-outline)")
    for k in ("gloss", "roughness", "wear"):
        _need(0.0 <= float(m.get(k, 0)) <= 1.0, f"{where}: {k} must be within 0..1")
    _need(0.5 <= float(m.get("contrast", 1)) <= 1.5, f"{where}: contrast must be within 0.5..1.5")


def validate_detail(d: dict, where: str) -> None:
    op = d.get("op")
    _need(op in DETAIL_OPS, f"{where}: unknown op {op!r} (known: {sorted(DETAIL_OPS)})")
    missing = DETAIL_OPS[op] - set(d) - ({"width"} if op == "edge_band" else set())
    _need(not missing, f"{where}: missing {sorted(missing)}")
    if op == "stamp":
        pat = d["pattern"]
        _need(pat and all(len(r) == len(pat[0]) for r in pat), f"{where}: pattern rows must have equal length")
        bad = {c for r in pat for c in r if c != "." and c.lower() not in TONE_CHARS}
        _need(not bad, f"{where}: unknown pattern chars {sorted(bad)}")
        _need(d.get("on_overflow", "skip") in ("skip", "error"), f"{where}: on_overflow must be skip or error")
    if op == "edge_band":
        _need(int(d["width"]) in (1, 2), f"{where}: width must be 1 or 2")
        for t in d.get("tones", []) + ([d["second_row"]] if "second_row" in d else []):
            _need(t in TONE_CHARS.values(), f"{where}: unknown tone {t!r}")
        for r in d.get("pattern", []):
            _need(all(c.lower() in TONE_CHARS for c in r), f"{where}: bad pattern {r!r}")
        _need("tones" in d or "pattern" in d, f"{where}: edge_band needs tones or pattern")
    if op == "line":
        _need(d["tone"] in TONE_CHARS.values(), f"{where}: unknown tone {d['tone']!r}")
    if op == "scatter":
        _need(all(c.lower() in TONE_CHARS for c in d["tones"]), f"{where}: bad scatter tones")


def validate_relief(ops: list, where: str) -> None:
    _need(isinstance(ops, list), f"{where}: relief must be a list")
    for i, op in enumerate(ops):
        name = op.get("op")
        _need(name in RELIEF_OPS, f"{where}.relief[{i}]: unknown op {name!r} (known: {sorted(RELIEF_OPS)})")
        missing = RELIEF_OPS[name] - set(op)
        _need(not missing, f"{where}.relief[{i}] ({name}): missing {sorted(missing)}")


def validate_template(t: dict, where: str, profiles: dict) -> None:
    kind = t.get("kind")
    _need(kind in ("torso", "blade", "hilt", "shoulder"), f"{where}: unknown kind {kind!r}")
    _need(t.get("profile") in profiles, f"{where}: unknown animation profile {t.get('profile')!r}")
    _need(isinstance(t.get("material_slots"), list) and t["material_slots"], f"{where}: material_slots required")
    if kind == "torso":
        s = t["shape"]
        _need(0.3 <= s["extent"] <= 1.0, f"{where}: extent must be within 0.3..1")
        _need(s["bottom"] in ("straight", "pointed", "scalloped"), f"{where}: bottom must be straight|pointed|scalloped")
        _need(s["neck"] in ("cut", "gorget"), f"{where}: neck must be cut|gorget")
        _need(set(t["clips"]) <= set(be.CLIPS), f"{where}: unknown clip in {t['clips']}")
        validate_relief(t["relief"], where)
    elif kind == "blade":
        _need(10 <= t["length"] <= 44, f"{where}: blade length must fit the 64-texel canvas (10..44)")
        _need(t["half_width"][0][0] == 0 and t["half_width"][-1][0] == 1, f"{where}: half_width must span v 0..1")
        validate_relief(t["relief"], where)
    elif kind == "hilt":
        for part in ("guard", "grip", "pommel"):
            pat = t[part]["pattern"]
            _need(all(len(r) == len(pat[0]) for r in pat), f"{where}.{part}: pattern rows must have equal length")
            _need(t[part]["slot"] in t["material_slots"], f"{where}.{part}: slot not in material_slots")
            _need(all(c == "." or c in TONE_CHARS for r in pat for c in r), f"{where}.{part}: bad pattern char")
    else:
        _need(len(t["polygon"]) >= 3, f"{where}: polygon needs 3+ points")
        _need(set(t["regions"]) == set(t["material_slots"]), f"{where}: regions must match material_slots")
        validate_relief(t["relief"], where)


def load_library(root: Path = FACTORY) -> Library:
    lib = Library(root)
    for kind, target in (("profiles", lib.profiles), ("materials", lib.materials),
                         ("details", lib.details), ("templates", lib.templates)):
        for p in sorted((root / kind).glob("*.json")):
            try:
                obj = json.loads(p.read_text())
            except json.JSONDecodeError as e:
                raise FactoryError(f"{rel(p)}: invalid JSON: {e}") from e
            where = rel(p)
            _need(obj.get("schema") == SCHEMA, f"{where}: unsupported schema {obj.get('schema')!r}")
            _need(obj.get("id") == p.stem, f"{where}: id must equal the file name")
            _need(obj["id"] not in target, f"{where}: duplicate id")
            if kind == "materials":
                validate_material(obj, where)
            elif kind == "details":
                validate_detail(obj, where)
            elif kind == "templates":
                validate_template(obj, where, lib.profiles)
            target[obj["id"]] = obj
            lib.paths[(kind, obj["id"])] = p
    return lib


def validate_recipe(r: dict, lib: Library, where: str) -> None:
    unknown = set(r) - RECIPE_KEYS
    _need(not unknown, f"{where}: unknown keys {sorted(unknown)}")
    _need(r.get("schema") == SCHEMA, f"{where}: unsupported schema {r.get('schema')!r}")
    rid = r.get("id", "")
    _need(bool(ID_RE.match(rid)), f"{where}: id {rid!r} must be lower_snake_case")
    cat = r.get("category")
    _need(cat in CATEGORY_KIND, f"{where}: unknown category {cat!r}")
    gen = r.get("generation", {})
    _need(isinstance(gen.get("seed"), int) and gen["seed"] >= 0, f"{where}: generation.seed must be a non-negative int")
    slots: list[str] = []
    if cat == "sword":
        tpls = r.get("templates", {})
        _need(set(tpls) == {"blade", "hilt"}, f"{where}: a sword needs templates.blade and templates.hilt")
        for k, tid in tpls.items():
            _need(tid in lib.templates and lib.templates[tid]["kind"] == k, f"{where}: unknown {k} template {tid!r}")
            slots += lib.templates[tid]["material_slots"]
        profile = lib.templates[tpls["blade"]]["profile"]
    else:
        tid = r.get("template")
        _need(tid in lib.templates, f"{where}: unknown template {tid!r}")
        _need(lib.templates[tid]["kind"] == CATEGORY_KIND[cat], f"{where}: template {tid} is not a {CATEGORY_KIND[cat]}")
        slots = lib.templates[tid]["material_slots"]
        profile = lib.templates[tid]["profile"]
    _need(gen.get("animation_profile") == profile, f"{where}: animation_profile must be {profile!r} (template profile)")
    mats = r.get("materials", {})
    _need(set(mats) == set(slots), f"{where}: materials must fill exactly the slots {sorted(set(slots))}, got {sorted(mats)}")
    for slot, mid in mats.items():
        _need(mid in lib.materials, f"{where}: unknown material {mid!r} for slot {slot}")
    for i, d in enumerate(r.get("details", [])):
        w = f"{where}.details[{i}]"
        _need(d.get("detail") in lib.details, f"{w}: unknown detail {d.get('detail')!r}")
        _need(d.get("material") in lib.materials, f"{w}: unknown material {d.get('material')!r}")
        _need(isinstance(d.get("place"), dict), f"{w}: place is required")
        op = lib.details[d["detail"]]["op"]
        place = d["place"]
        allowed = {"stamp": {"part", "at", "along", "u", "v", "step"}, "edge_band": {"part", "edge"},
                   "line": {"part", "axis", "u", "v"}, "scatter": {"part", "density"}}[op]
        _need(set(place) <= allowed, f"{w}: place keys {sorted(set(place) - allowed)} not valid for {op}")
        if op == "stamp":
            _need(("at" in place) != ("along" in place), f"{w}: stamp needs exactly one of at / along")
            if "along" in place:
                _need(place["along"] in ("seams", "grid", "column", "row"), f"{w}: unknown along {place['along']!r}")
        if op == "edge_band":
            _need(place.get("edge") in ("top", "bottom"), f"{w}: edge must be top or bottom")
        if op == "line":
            _need(place.get("axis") in ("vertical", "horizontal"), f"{w}: axis must be vertical or horizontal")
        if op == "scatter":
            _need(0 < float(place.get("density", 0)) <= 0.3, f"{w}: density must be within (0, 0.3]")
    _need(r.get("hides") in ("auto", "none"), f"{where}: hides must be auto or none")
    _need(r.get("mirroring") == "flip", f"{where}: mirroring supports only 'flip' (the body flips the item)")
    _need(r.get("fallback", {}).get("missing_clip") in ("hide", "refuse"), f"{where}: fallback.missing_clip must be hide|refuse")
    _need(r.get("variants", {}) == {}, f"{where}: variants are not generated yet (declare none)")


def load_recipes(lib: Library, root: Path = FACTORY, only: list[str] | None = None) -> tuple[list[dict], list[dict]]:
    """Valid recipes (sorted by id) and failures (path + error)."""
    ok, failed, seen = [], [], set()
    for p in sorted((root / "recipes").glob("*.json")):
        try:
            r = json.loads(p.read_text())
            validate_recipe(r, lib, rel(p))
            _need(r["id"] == p.stem, f"{rel(p)}: id must equal the file name")
            _need(r["id"] not in seen, f"{rel(p)}: duplicate id")
        except (FactoryError, json.JSONDecodeError, KeyError, TypeError, ValueError) as e:
            failed.append({"recipe": rel(p), "error": str(e)})
            continue
        seen.add(r["id"])
        if only is None or r["id"] in only:
            r["_path"] = p
            ok.append(r)
    return ok, failed


# --- raster helpers ---------------------------------------------------------------------


def bayer(x: int, y: int) -> float:
    return BAYER4[y % 4][x % 4] / 16.0 - 0.5


def ramp(mat: dict) -> list[str]:
    return [t for t in TONE_ORDER if t in mat["tones"]]


def interp(points: list, v: float) -> float:
    for (a, fa), (b, fb) in zip(points, points[1:]):
        if a <= v <= b:
            return fa + (fb - fa) * (v - a) / max(1e-9, b - a)
    return points[-1][1]


def rows_of(cells: set) -> dict[int, tuple[int, int]]:
    rows: dict[int, list[int]] = {}
    for x, y in cells:
        rows.setdefault(y, []).append(x)
    return {y: (min(xs), max(xs)) for y, xs in rows.items()}


def uv(cells: set) -> dict:
    """Row-relative u (0 back .. 1 front) and bbox-relative v per cell."""
    rows = rows_of(cells)
    top, bottom = min(rows), max(rows)
    out = {}
    for x, y in cells:
        a, b = rows[y]
        out[(x, y)] = ((x - a) / max(1, b - a), (y - top) / max(1, bottom - top))
    return out


def interior(cells: set) -> set:
    return {(x, y) for x, y in cells if all(n in cells for n in ((x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)))}


@dataclass
class Shape:
    """One frame of an item before painting."""
    cells: set
    slot_of: dict  # cell -> material slot
    occluders: set = field(default_factory=set)  # neighbours that cast a shadow instead of an outline
    parts: dict = field(default_factory=dict)  # part name -> cells (detail targets)
    seams: list = field(default_factory=list)  # rows of band seams
    nodes: list = field(default_factory=list)  # grid nodes
    seam_curve: float = 0.0  # bend of the band seams (texels at the middle)
    height: dict = field(default_factory=dict)


# --- relief -----------------------------------------------------------------------------


def apply_relief(shape: Shape, ops: list, salt: str) -> None:
    cells = shape.cells
    coords = uv(cells)
    rows = rows_of(cells)
    xs = [x for x, _ in cells]
    ys = [y for _, y in cells]
    minx, maxx, top, bottom = min(xs), max(xs), min(ys), max(ys)
    aspect = (maxx - minx + 1) / max(1, bottom - top + 1)
    h = {p: 0.0 for p in cells}
    skirt = shape.parts.get("skirt", set())
    for op in ops:
        name = op["op"]
        for p in sorted(cells):
            x, y = p
            u, v = coords[p]
            if name == "cylinder":
                h[p] += op["amp"] * math.sin(math.pi * u) ** op.get("power", 1.0)
            elif name == "drape":
                h[p] += op["amp"] * math.sin(math.pi * u) * (0.4 + 0.6 * v)
            elif name == "dome":
                d2 = ((u - op["u"]) * aspect) ** 2 + (v - op["v"]) ** 2
                h[p] += op["amp"] * max(0.0, 1 - d2 / (op["r"] ** 2))
            elif name == "bands" and v >= op["from"] and p not in skirt:
                # ``curve`` > 0 bends every lame down in the middle (convex plates).
                start = top + round((bottom - top) * op["from"])
                k = (y - round(op.get("curve", 0) * math.sin(math.pi * u)) - start) % op["every"]
                h[p] += op["depth"] * (k / max(1, op["every"] - 1)) - (op["depth"] * 0.6 if k == 0 else 0)
            elif name == "ridge" and op.get("from", 0) <= v <= op.get("to", 1):
                h[p] += op["amp"] * max(0.0, 1 - abs(u - op["u"]) / op["width"])
            elif name == "grid" and p not in skirt:
                if (x - minx) % op["every_x"] == 0 or (y - top) % op["every_y"] == 0:
                    h[p] -= op["depth"]
            elif name == "skirt_segments" and p in skirt:
                seg = 4
                lx = (x - minx) % seg
                h[p] += op["depth"] * math.sin(math.pi * (lx + 0.5) / seg) - (op["depth"] if lx == 0 else 0)
            elif name == "scales":
                r = (y - top) // op["h"]
                lx = (x - minx + (r % 2) * (op["w"] // 2)) % op["w"]
                ly = (y - top) % op["h"]
                # Overlapping scales: the row under the previous scale and the
                # gap between two scales are low; each scale bulges at its centre.
                gap = ly == 0 or lx == 0
                bulge = math.sin(math.pi * (lx + 0.5) / op["w"]) * (0.4 + 0.6 * ly / max(1, op["h"] - 1))
                h[p] += op["depth"] * bulge - (op["depth"] * 0.8 if gap else 0)
            elif name == "folds" and v >= op.get("start", 0):
                phase = int(sha256(f"{salt}:folds".encode())[:4], 16) / 65535 * math.tau
                ramp_in = min(1.0, (v - op.get("start", 0)) / 0.25)
                h[p] += op["amp"] * ramp_in * 0.5 * (1 + math.sin(math.tau * op["count"] * u + phase + v * 1.3))
            elif name == "blade_bevel":
                a, b = rows[y]
                if op.get("spine", 0) < 0:
                    h[p] += op["amp"] * (1 - (x - a) / max(1, b - a + 1))
                else:
                    c = (a + b) / 2
                    h[p] += op["amp"] * (1 - abs(x - c) / ((b - a) / 2 + 0.75))
            elif name == "fuller" and op["from"] <= 1 - v <= op["to"]:
                a, b = rows[y]
                if abs(x - (a + b) / 2) <= op["width"] / 2 and b - a >= 3:
                    h[p] -= op["depth"]
            elif name == "rim":
                edge = op["edge"]
                w = op["width"]
                near = False
                for d in range(1, w + 1):
                    if edge in ("top", "all") and (x, y - d) not in cells:
                        near = True
                    if edge == "all" and any(n not in cells for n in ((x - d, y), (x + d, y), (x, y + d))):
                        near = True
                if near:
                    h[p] += op["amp"]
        if name == "bands":
            start = top + round((bottom - top) * op["from"])
            shape.seams = [y for y in range(start, bottom + 1) if (y - start) % op["every"] == 0 and y > start]
            shape.seam_curve = float(op.get("curve", 0))
        if name == "grid":
            ex, ey = op["every_x"], op["every_y"]
            shape.nodes = sorted(p for p in cells if p not in skirt
                                 and (p[0] - minx) % ex == ex // 2 and (p[1] - top) % ey == ey // 2)
    shape.height = h


def shade(shape: Shape, mats: dict) -> dict:
    """Light the height map and quantize every cell into its material ramp:
    cell -> (material id, tone)."""
    h = shape.height
    cells = shape.cells
    lx, ly, lz = LIGHT
    ln = math.sqrt(lx * lx + ly * ly + lz * lz)
    lx, ly, lz = lx / ln, ly / ln, lz / ln
    hs = sorted(h.values())
    lo, hi = hs[0], hs[-1]
    out = {}
    for p in sorted(cells):
        x, y = p
        mat = mats[shape.slot_of[p]]
        get = lambda q: h[q] if q in h and shape.slot_of.get(q) == shape.slot_of[p] else h[p]  # noqa: E731
        dx = (get((x + 1, y)) - get((x - 1, y))) / 2
        dy = (get((x, y + 1)) - get((x, y - 1))) / 2
        nx, ny, nz = -dx / 40, -dy / 40, 1.0
        nn = math.sqrt(nx * nx + ny * ny + nz * nz)
        lambert = max(0.0, (nx * lx + ny * ly + nz * lz) / nn)
        ao = (h[p] - lo) / max(1e-6, hi - lo) - 0.5
        i = 0.12 + 0.78 * lambert + 0.22 * ao
        i = 0.5 + (i - 0.5) * float(mat.get("contrast", 1.0))
        i += bayer(x, y) * float(mat.get("roughness", 0.2)) * 0.3
        tones = ramp(mat)
        k = min(len(tones) - 1, max(0, int(i * len(tones))))
        tone = tones[k]
        if "specular" in mat["tones"] and lambert > 0.985 - 0.05 * float(mat.get("gloss", 0)) and ao > 0.15:
            tone = "specular"
        out[p] = (mat["id"], tone)
    return out


def outline(shape: Shape, painted: dict, mats: dict) -> None:
    """Selective outline: the outer edge takes the material outline, except
    where the light hits it (front or top edge: the material's darkest ramp
    tone); edges cut by an occluder (arm, sleeve) take a cast shadow."""
    for p in sorted(shape.cells):
        x, y = p
        mat = mats[shape.slot_of[p]]
        outer = [(n, d) for n, d in (((x + 1, y), "front"), ((x, y - 1), "top"), ((x - 1, y), "back"), ((x, y + 1), "bottom"))
                 if n not in shape.cells]
        if not outer:
            continue
        if all(n in shape.occluders for n, _ in outer):
            painted[p] = (mat["id"], ramp(mat)[0])
            continue
        lit = all(d in ("front", "top") for n, d in outer if n not in shape.occluders)
        painted[p] = (mat["id"], ramp(mat)[0] if lit and len(outer) == 1 else "outline")
    # Boundaries between two material regions (collar over cloth, lining):
    # the lower/back region gets its deepest tone.
    for p in sorted(shape.cells):
        x, y = p
        up = (x, y - 1)
        if up in shape.cells and shape.slot_of[up] != shape.slot_of[p] and painted[p][1] != "outline":
            painted[p] = (painted[p][0], ramp(mats[shape.slot_of[p]])[0])


# --- details ----------------------------------------------------------------------------


def tone_of(ch: str, detail_mat: dict, host_mat: dict) -> tuple[str, str]:
    mat = host_mat if ch.isupper() else detail_mat
    tone = TONE_CHARS[ch.lower()]
    if tone not in mat["tones"]:
        tone = {"specular": "highlight", "deep": "shadow", "light": "base"}[tone]
    return mat["id"], tone


def apply_details(shape: Shape, painted: dict, recipe: dict, lib: Library, mats: dict, salt: str, log: list) -> None:
    """Details only touch interior cells of their part (never the outline,
    never outside the mask). Overflowing stamps are skipped and logged."""
    for i, use in enumerate(recipe.get("details", [])):
        d = lib.details[use["detail"]]
        dm = lib.materials[use["material"]]
        place = use["place"]
        part = shape.parts.get(place.get("part", "main"), shape.cells)
        inner = interior(part) & shape.cells
        if not inner:
            log.append(f"{salt}: {use['detail']}: no interior in part {place.get('part', 'main')}")
            continue
        host = lambda p: mats[shape.slot_of[p]]  # noqa: E731
        rows = rows_of(inner)
        xs = [x for x, _ in inner]
        ys = [y for _, y in inner]
        minx, maxx, top, bottom = min(xs), max(xs), min(ys), max(ys)
        op = d["op"]
        if op == "stamp":
            pat = d["pattern"]
            ph, pw = len(pat), len(pat[0])
            anchors = []
            if "at" in place:
                u, v = place["at"]
                anchors = [(minx + round(u * (maxx - minx)), top + round(v * (bottom - top)))]
            elif place["along"] == "seams":
                us = place.get("u", [0.15, 0.15])
                for y in shape.seams:
                    if y + 1 in rows:
                        a, b = rows[y + 1]
                        for u in sorted(set(us)):
                            bend = round(shape.seam_curve * math.sin(math.pi * u))
                            anchors.append((a + round(u * (b - a)), y + 1 + bend))
            elif place["along"] == "grid":
                anchors = list(shape.nodes)
            elif place["along"] == "column":
                v0, v1 = place["v"]
                for y in range(top + round(v0 * (bottom - top)), top + round(v1 * (bottom - top)) + 1, place.get("step", 2)):
                    if y in rows:
                        a, b = rows[y]
                        anchors.append((a + round(place["u"] * (b - a)), y))
            elif place["along"] == "row":
                y = top + round(place["v"] * (bottom - top))
                if y in rows:
                    a, b = rows[y]
                    u0, u1 = place["u"]
                    anchors = [(x, y) for x in range(a + round(u0 * (b - a)), a + round(u1 * (b - a)) + 1, place.get("step", 2))]
            skipped = 0
            for ax, ay in anchors:
                x0, y0 = ax - pw // 2, ay - ph // 2 if "at" in place else ay
                pixels = [((x0 + c, y0 + r), ch) for r, row in enumerate(pat) for c, ch in enumerate(row) if ch != "."]
                if not all(q in inner for q, _ in pixels):
                    skipped += 1
                    if d.get("on_overflow", "skip") == "error":
                        raise FactoryError(f"{recipe['id']}: detail {use['detail']} does not fit at {(ax, ay)}")
                    continue
                for q, ch in pixels:
                    painted[q] = tone_of(ch, dm, host(q))
            if skipped:
                log.append(f"{salt}: {use['detail']}: {skipped}/{len(anchors)} stamp(s) skipped (do not fit inside the mask)")
            if not anchors:
                log.append(f"{salt}: {use['detail']}: no anchor in this frame")
        elif op == "edge_band":
            cols: dict[int, list[int]] = {}
            for x, y in inner:
                cols.setdefault(x, []).append(y)
            for x in sorted(cols):
                ys_col = sorted(cols[x])
                t = (x - minx) / max(1, maxx - minx)
                edge_rows = ys_col[:d["width"]] if place["edge"] == "top" else ys_col[::-1][:d["width"]]
                for r, y in enumerate(edge_rows):
                    if "pattern" in d:
                        ch = d["pattern"][r % len(d["pattern"])][x % len(d["pattern"][0])]
                        painted[(x, y)] = tone_of(ch, dm, host((x, y)))
                    elif r == 1 and "second_row" in d:
                        painted[(x, y)] = tone_of(next(k for k, v in TONE_CHARS.items() if v == d["second_row"]), dm, host((x, y)))
                    else:
                        tones = d["tones"]
                        tone = tones[min(len(tones) - 1, int(t * len(tones)))]
                        painted[(x, y)] = tone_of(next(k for k, v in TONE_CHARS.items() if v == tone), dm, host((x, y)))
        elif op == "line":
            dash = d["dash"]
            period = sum(dash) or 1
            tone_ch = next(k for k, v in TONE_CHARS.items() if v == d["tone"])
            glint = d.get("glint")
            if place["axis"] == "vertical":
                v0, v1 = place["v"]
                for n, y in enumerate(range(top + round(v0 * (bottom - top)), top + round(v1 * (bottom - top)) + 1)):
                    if y not in rows or n % period >= dash[0]:
                        continue
                    a, b = rows[y]
                    x = a + round(place["u"] * (b - a))
                    if (x, y) in inner:
                        painted[(x, y)] = tone_of(tone_ch, dm, host((x, y)))
                        if glint and (x + 1, y) in inner:
                            painted[(x + 1, y)] = tone_of(next(k for k, v in TONE_CHARS.items() if v == glint), dm, host((x + 1, y)))
            else:
                y = top + round(place["v"] * (bottom - top))
                if y in rows:
                    a, b = rows[y]
                    u0, u1 = place["u"]
                    for n, x in enumerate(range(a + round(u0 * (b - a)), a + round(u1 * (b - a)) + 1)):
                        if n % period < dash[0] and (x, y) in inner:
                            painted[(x, y)] = tone_of(tone_ch, dm, host((x, y)))
        elif op == "scatter":
            rng = random.Random(f"{recipe['generation']['seed']}:{salt}:{i}")
            density = float(place["density"])
            for p in sorted(inner):
                if rng.random() < density:
                    ch = d["tones"][rng.randrange(len(d["tones"]))]
                    for k in range(d.get("length", 1)):
                        q = (p[0] + k, p[1] + k)
                        if q in inner:
                            painted[q] = tone_of(ch, dm, host(q))


# --- shapes per category ----------------------------------------------------------------


_MASKS: dict = {}
_TOP: dict = {}


def torso_masks() -> dict:
    if not _MASKS:
        family = json.loads((ROOT / "art-src/n4-sprite-machine/equipment/families/torso_plate.json").read_text())
        _MASKS.update(be.family_masks(family))
    return _MASKS


def dressed_top(clip: str, frame: int) -> dict:
    """Top base layer of every opaque texel of the dressed character."""
    key = (clip, frame)
    if key not in _TOP:
        base = be.base_layers(clip)
        top = {}
        for layer in be.draw_order(clip):
            if layer in base:
                for p in base[layer][frame]:
                    top[p] = layer
        _TOP[key] = top
    return _TOP[key]


def torso_shape(t: dict, clip: str, frame: int) -> Shape:
    cells, region, neck = torso_masks()[clip][frame]
    s = t["shape"]
    body = set(cells) | (set(neck) if s["neck"] == "gorget" else set())
    occluders = set(region) - set(cells) - set(neck)
    ys = [y for _, y in body]
    top, bottom = min(ys), max(ys)
    base_cut = top + round((bottom - top) * s["extent"])
    rows = rows_of(body)
    xa = min(x for x, _ in body)
    xb = max(x for x, _ in body)

    def cut_at(x: int, cut: int) -> int:
        t_ = (x - xa) / max(1, xb - xa)
        if s["bottom"] == "pointed":
            return cut + round(s.get("point_depth", 2) * max(0.0, 1 - abs(t_ - s.get("point_u", 0.6)) / 0.45))
        if s["bottom"] == "scalloped":
            return cut - (1 if (x // s.get("scallop", 3)) % 2 else 0)
        return cut

    shaped = {p for p in body if p[1] <= (cut_at(p[0], base_cut) if s["skirt"] is None else base_cut)}
    skirt: set = set()
    if s["skirt"]:
        top_layer = dressed_top(clip, frame)
        lows = [rows[y] for y in range(bottom - 2, bottom + 1) if y in rows]
        a = min(r[0] for r in lows) - s["skirt"].get("flare", 0)
        b = max(r[1] for r in lows) + s["skirt"].get("flare", 0)
        for y in range(base_cut + 1, base_cut + 1 + s["skirt"]["rows"]):
            for x in range(a, b + 1):
                if y <= cut_at(x, base_cut + s["skirt"]["rows"]) and (
                        top_layer.get((x, y)) == "trousers" or (x, y) in body):
                    skirt.add((x, y))
        # Keep only the skirt connected to the torso (no stray texels).
        keep, frontier = set(), [p for p in skirt if (p[0], p[1] - 1) in shaped]
        while frontier:
            p = frontier.pop()
            if p in keep or p not in skirt:
                continue
            keep.add(p)
            frontier += [(p[0] + 1, p[1]), (p[0] - 1, p[1]), (p[0], p[1] + 1)]
        skirt = keep
        shaped |= skirt
    return Shape(cells=shaped, slot_of={p: "main" for p in shaped}, occluders=occluders,
                 parts={"main": shaped - skirt, "skirt": skirt})


def raster_polygon(poly: list) -> set:
    xs = [p[0] for p in poly]
    ys = [p[1] for p in poly]
    return {(x, y) for y in range(min(ys), max(ys) + 1) for x in range(min(xs), max(xs) + 1)
            if be.inside_poly((x + 0.5, y + 0.5), poly)}


def shoulder_shape(t: dict) -> Shape:
    cells = raster_polygon(t["polygon"])
    hem = t.get("hem")
    if hem:
        rng = random.Random(f"{t['id']}:hem")
        rows = rows_of(cells)
        bottom = max(rows)
        cols: dict[int, int] = {}
        for x, y in cells:
            cols[x] = max(cols.get(x, 0), y)
        x = min(cols)
        while x <= max(cols):
            width = rng.randint(2, 4)
            depth = rng.randint(0, hem["depth"])
            for xx in range(x, x + width):
                for d in range(depth):
                    cells.discard((xx, cols.get(xx, bottom) - d))
            x += width
    slot_of = {p: "main" for p in cells}
    coords = uv(cells)
    rows = rows_of(cells)
    top = min(rows)
    for slot, rule in t["regions"].items():
        if rule.get("all"):
            continue
        for p in cells:
            u, v = coords[p]
            if "front_edge" in rule and v >= rule.get("from", 0) and p[0] > rows[p[1]][1] - rule["front_edge"]:
                slot_of[p] = slot
            if "top_rows" in rule and p[1] < top + rule["top_rows"]:
                slot_of[p] = slot
    parts = {slot: {p for p in cells if slot_of[p] == slot} for slot in t["regions"]}
    return Shape(cells=cells, slot_of=slot_of, parts=parts)


def sword_shape(blade: dict, hilt: dict) -> tuple[Shape, dict]:
    """Blade (tip up) and hilt stamps on the isolated canvas; returns the
    shape and the fixed hilt texels (pattern-painted, not lit)."""
    w, h = SWORD_CANVAS
    cx = w // 2
    y = h - 2
    fixed: dict = {}
    parts: dict = {}

    def stamp(name: str, pat: list, bottom_y: int) -> int:
        ph, pw = len(pat), len(pat[0])
        x0 = cx - pw // 2
        y0 = bottom_y - ph + 1
        cells = set()
        for r, row in enumerate(pat):
            for c, ch in enumerate(row):
                if ch != ".":
                    fixed[(x0 + c, y0 + r)] = (hilt[name]["slot"], ch)
                    cells.add((x0 + c, y0 + r))
        parts[name] = cells
        return y0 - 1

    y = stamp("pommel", hilt["pommel"]["pattern"], y)
    grip = hilt["grip"]
    rows = [grip["pattern"][i % len(grip["pattern"])] for i in range(grip["length"])]
    y = stamp("grip", rows, y)
    y = stamp("guard", hilt["guard"]["pattern"], y)
    length = blade["length"]
    cells = set()
    for i in range(length):
        v = i / max(1, length - 1)
        hw = interp(blade["half_width"], v)
        c = cx + interp(blade["curve"], v)
        wave = blade.get("wave")
        if wave and v >= wave["from"]:
            c += wave["amp"] * (1 if math.sin(math.tau * i / wave["period"]) > 0.3 else -1 if math.sin(math.tau * i / wave["period"]) < -0.3 else 0)
        a, b = round(c - hw), round(c + hw)
        if b - a < 0:
            b = a
        for x in range(a, b + 1):
            cells.add((x, y - i))
    shape = Shape(cells=cells, slot_of={p: "blade" for p in cells}, parts={"blade": cells, **parts})
    return shape, fixed


# --- rendering --------------------------------------------------------------------------


def render_frame(shape: Shape, relief: list, recipe: dict, lib: Library, salt: str, log: list,
                 fixed: dict | None = None, texture: dict | None = None) -> dict:
    mats = {slot: lib.materials[mid] for slot, mid in recipe["materials"].items()}
    apply_relief(shape, relief, salt)
    if texture and texture["op"] == "fur":
        rng = random.Random(f"{salt}:fur")
        for p in sorted(shape.parts.get("collar", set())):
            if rng.random() < texture["density"]:
                shape.height[p] += 70
                if (p[0], p[1] + 1) in shape.height:
                    shape.height[(p[0], p[1] + 1)] -= 40
    painted = shade(shape, mats)
    outline(shape, painted, mats)
    for p, (slot, ch) in (fixed or {}).items():
        mat = mats[slot]
        tone = TONE_CHARS[ch]
        if tone not in mat["tones"]:
            tone = {"specular": "highlight", "deep": "shadow", "light": "base"}[tone]
        painted[p] = (mat["id"], tone)
    if fixed:
        shape.cells = shape.cells | set(fixed)
        for p, (slot, _) in fixed.items():
            shape.slot_of[p] = slot
    apply_details(shape, painted, recipe, lib, mats, salt, log)
    return painted


def color(lib: Library, mid: str, tone: str) -> tuple[int, int, int, int]:
    r, g, b = lib.materials[mid]["tones"][tone]
    return (r, g, b, 255)


def write_strip(frames: list[dict], w: int, h: int, lib: Library, png: Path, txt: Path) -> None:
    im = Image.new("RGBA", (w * len(frames), h), (0, 0, 0, 0))
    keys: dict = {}
    alphabet = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
    rows = [["."] * (w * len(frames)) for _ in range(h)]
    for f, painted in enumerate(frames):
        for (x, y), (mid, tone) in sorted(painted.items()):
            _need(0 <= x < w and 0 <= y < h, f"{png.name}: texel {(x, y)} outside the {w}x{h} canvas")
            im.putpixel((f * w + x, y), color(lib, mid, tone))
            k = keys.setdefault((mid, tone), alphabet[len(keys) % len(alphabet)])
            rows[y][f * w + x] = k
    png.parent.mkdir(parents=True, exist_ok=True)
    im.save(png, optimize=False)
    txt.write_text("\n".join("".join(r) for r in rows) + "\n")


def write_relief(shapes: list[Shape], w: int, h: int, path: Path) -> None:
    """Height map 0..255 (0 = transparent / lowest); a height map, not a normal map."""
    im = Image.new("L", (w * len(shapes), h), 0)
    for f, s in enumerate(shapes):
        vals = list(s.height.values())
        lo, hi = min(vals), max(vals)
        for (x, y), v in sorted(s.height.items()):
            im.putpixel((f * w + x, y), 1 + round(254 * (v - lo) / max(1e-6, hi - lo)))
    path.parent.mkdir(parents=True, exist_ok=True)
    im.save(path)


def item_ron(recipe: dict, clips: list[str], hide_run: bool) -> str:
    sheets = []
    for clip in clips:
        cols = ", columns: 3" if be.CLIPS[clip][2] > 1 else ""
        sheets.append(f'        "{clip}": (texture: "factory/art/{recipe["id"]}/{clip}.png"{cols}),')
    hide = "" if hide_run and "run_c1" in clips else "    hide_layers: Some({}),\n"
    return (
        f"// N41-B factory item (GENERATED by scripts/factory.py {GENERATOR_VERSION}; do not edit by hand).\n"
        f"// Recipe: art-src/n4-sprite-machine/factory/recipes/{recipe['id']}.json\n"
        f"// {recipe['name']} — template {recipe['template']}, materials {recipe['materials']}.\n"
        "(\n    version: 1,\n"
        f'    id: "{recipe["id"]}",\n    family: "{recipe["family"]}",\n    sheets: {{\n'
        + "\n".join(sheets) + "\n    },\n" + hide
        + f'    palette: Some("{recipe["materials"]["main"]}"),\n)\n'
    )


# --- one item ----------------------------------------------------------------------------


def input_files(recipe: dict, lib: Library) -> list[Path]:
    files = [recipe["_path"], Path(__file__), HERE / "build_equipment.py"]
    tids = [recipe["template"]] if "template" in recipe else list(recipe["templates"].values())
    for tid in tids:
        files.append(lib.paths[("templates", tid)])
        files.append(lib.paths[("profiles", lib.templates[tid]["profile"])])
    files += [lib.paths[("materials", m)] for m in recipe["materials"].values()]
    for d in recipe.get("details", []):
        files += [lib.paths[("details", d["detail"])], lib.paths[("materials", d["material"])]]
    if recipe["category"] == "chest_armor":
        files += [ROOT / "art-src/n4-sprite-machine/equipment/families/torso_plate.json", STAGE / "protagonist.ron",
                  STAGE / "equipment/families/torso_plate.ron", HERE / "run_poses.py", HERE / "reconstruct.py"]
        files += sorted((STAGE / "parts").glob("*/*.txt"))
    return sorted(set(files))


def render_item(recipe: dict, lib: Library) -> dict:
    """Pure: recipe -> outputs in memory {name: (kind, payload)} plus notes."""
    log: list[str] = []
    out: dict = {"strips": {}, "relief": {}, "log": log, "meta": {}}
    cat = recipe["category"]
    if cat == "chest_armor":
        t = lib.templates[recipe["template"]]
        clips = list(t["clips"])
        for clip in clips:
            w, h, n = be.CLIPS[clip]
            frames, shapes = [], []
            for f in range(n):
                s = torso_shape(t, clip, f)
                frames.append(render_frame(s, t["relief"], recipe, lib, f"{t['id']}:{clip}:{f}", log))
                shapes.append(s)
            out["strips"][clip] = (frames, w, h)
            out["relief"][f"{t['id']}_{clip}"] = (shapes, w, h)
        pc, pf = t["preview_pose"]["clip"], t["preview_pose"]["frame"]
        w, h, _ = be.CLIPS[pc]
        s = torso_shape(t, pc, pf)
        out["preview"] = (render_frame(s, t["relief"], recipe, lib, f"{t['id']}:{pc}:{pf}", log), w, h)
        out["relief"][f"{t['id']}_preview"] = ([s], w, h)
        hide_ok = True
        if recipe["hides"] == "auto" and "run_c1" in clips:
            torso = be.base_layers("run_c1")["shirt_torso"]
            hide_ok = all(set(torso[f]) <= set(out["strips"]["run_c1"][0][f]) for f in range(3))
        else:
            hide_ok = False
        out["meta"] = {"clips": clips, "hide_run_shirt_torso": hide_ok, "integration": "composer"}
        out["ron"] = item_ron(recipe, clips, hide_ok)
    elif cat == "sword":
        blade = lib.templates[recipe["templates"]["blade"]]
        hilt = lib.templates[recipe["templates"]["hilt"]]
        s, fixed = sword_shape(blade, hilt)
        w, h = SWORD_CANVAS
        painted = render_frame(s, blade["relief"], recipe, lib, f"{blade['id']}:{hilt['id']}", log, fixed=fixed)
        grip = s.parts["grip"]
        out["isolated"] = (painted, w, h)
        out["relief"][f"{blade['id']}"] = ([s], w, h)
        gx = sorted(x for x, _ in grip)
        gy = sorted(y for _, y in grip)
        out["meta"] = {"canvas": [w, h], "grip_pivot": [gx[len(gx) // 2], gy[len(gy) // 2]], "integration": "isolated",
                       "blocked_by": lib.profiles["weapon_main_hand"]["blocked_by"]}
    else:
        t = lib.templates[recipe["template"]]
        s = shoulder_shape(t)
        w, h = t["canvas"]
        painted = render_frame(s, t["relief"], recipe, lib, t["id"], log, texture=t.get("collar_texture"))
        out["isolated"] = (painted, w, h)
        out["relief"][t["id"]] = ([s], w, h)
        out["meta"] = {"canvas": [w, h], "pivot": t["pivot"], "rigid": t["rigid"], "integration": "isolated",
                       "blocked_by": lib.profiles[t["profile"]]["blocked_by"]}
    return out


def outputs_of(recipe: dict, rendered: dict, out_root: Path) -> list[Path]:
    rid = recipe["id"]
    paths = []
    for clip in rendered["strips"]:
        paths += [out_root / f"art/{rid}/{clip}.png", out_root / f"art/{rid}/{clip}.txt"]
    if "preview" in rendered:
        paths += [out_root / f"preview/{rid}.png", out_root / f"preview/{rid}.txt"]
    if "isolated" in rendered:
        paths += [out_root / f"isolated/{rid}.png", out_root / f"isolated/{rid}.txt"]
    if "ron" in rendered:
        paths.append(out_root / f"items/{rid}.ron")
    return paths


def write_item(recipe: dict, rendered: dict, lib: Library, out_root: Path) -> None:
    rid = recipe["id"]
    for clip, (frames, w, h) in rendered["strips"].items():
        write_strip(frames, w, h, lib, out_root / f"art/{rid}/{clip}.png", out_root / f"art/{rid}/{clip}.txt")
    if "preview" in rendered:
        painted, w, h = rendered["preview"]
        write_strip([painted], w, h, lib, out_root / f"preview/{rid}.png", out_root / f"preview/{rid}.txt")
    if "isolated" in rendered:
        painted, w, h = rendered["isolated"]
        write_strip([painted], w, h, lib, out_root / f"isolated/{rid}.png", out_root / f"isolated/{rid}.txt")
    if "ron" in rendered:
        p = out_root / f"items/{rid}.ron"
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_text(rendered["ron"])
    for name, (shapes, w, h) in rendered["relief"].items():
        write_relief(shapes, w, h, out_root / f"relief/{name}.png")


def build(out_root: Path = DEFAULT_OUT, rebuild: bool = False, only: list[str] | None = None,
          factory_root: Path = FACTORY) -> dict:
    """Validate the library and the catalog, generate every recipe (cache,
    no silent overwrite), and return the batch report."""
    t0 = time.perf_counter()
    lib = load_library(factory_root)
    recipes, failures = load_recipes(lib, factory_root, only)
    report: dict = {"generator_version": GENERATOR_VERSION, "items": {}, "failures": failures}
    for recipe in recipes:
        rid = recipe["id"]
        t1 = time.perf_counter()
        inputs = {rel(p): sha256(p.read_bytes()) for p in input_files(recipe, lib)}
        clean = {k: v for k, v in recipe.items() if not k.startswith("_")}
        key = sha256(canonical({"generator": GENERATOR_VERSION, "recipe": clean, "inputs": inputs}))
        prov_path = out_root / f"provenance/{rid}.json"
        old = json.loads(prov_path.read_text()) if prov_path.exists() else None
        if old and old["cache_key"] == key and all(
                (out_root / p).exists() and sha256((out_root / p).read_bytes()) == h for p, h in old["outputs"].items()):
            report["items"][rid] = {"status": "cached", "category": recipe["category"], **old["summary"]}
            continue
        try:
            rendered = render_item(recipe, lib)
        except (FactoryError, KeyError, ValueError) as e:
            report["failures"].append({"recipe": rid, "error": str(e)})
            continue
        paths = outputs_of(recipe, rendered, out_root)
        if not rebuild and (old or any(p.exists() for p in paths)) and (not old or old["cache_key"] != key):
            report["failures"].append({"recipe": rid, "error": "outputs exist from another recipe/generator state; "
                                                                "refusing to overwrite (use --rebuild)"})
            continue
        write_item(recipe, rendered, lib, out_root)
        outputs = {str(p.relative_to(out_root)): sha256(p.read_bytes()) for p in paths}
        # No timing in the provenance: it must be identical between runs.
        summary = {"integration": rendered["meta"]["integration"], "notes": sorted(set(rendered["log"])),
                   "meta": rendered["meta"]}
        prov = {"id": rid, "generator_version": GENERATOR_VERSION, "recipe": rel(recipe["_path"]),
                "recipe_sha256": sha256(canonical(clean)), "seed": recipe["generation"]["seed"],
                "inputs": inputs, "cache_key": key, "outputs": outputs, "summary": summary}
        prov_path.parent.mkdir(parents=True, exist_ok=True)
        prov_path.write_text(json.dumps(prov, indent=2, sort_keys=True) + "\n")
        report["items"][rid] = {"status": "generated", "category": recipe["category"], **summary,
                                "seconds": round(time.perf_counter() - t1, 4)}
    report["seconds"] = round(time.perf_counter() - t0, 3)
    report["peak_rss_mib"] = round(resource.getrusage(resource.RUSAGE_SELF).ru_maxrss / 1024, 1)
    return report


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--out", type=Path, default=DEFAULT_OUT)
    ap.add_argument("--rebuild", action="store_true", help="overwrite outputs whose provenance is stale")
    ap.add_argument("--only", nargs="*", default=None)
    ap.add_argument("--factory", type=Path, default=FACTORY, help="library + recipes root (tests use a copy)")
    args = ap.parse_args()
    report = build(args.out.resolve(), args.rebuild, args.only, args.factory.resolve())
    counts: dict = {}
    for it in report["items"].values():
        counts[it["status"]] = counts.get(it["status"], 0) + 1
    print(json.dumps({"counts": counts, "failures": report["failures"], "seconds": report["seconds"],
                      "peak_rss_mib": report["peak_rss_mib"]}, indent=1))
    out = args.out.resolve() / "batch_report.json"
    out.write_text(json.dumps(report, indent=1, sort_keys=True) + "\n")
    if report["failures"]:
        sys.exit(1)


if __name__ == "__main__":
    main()
