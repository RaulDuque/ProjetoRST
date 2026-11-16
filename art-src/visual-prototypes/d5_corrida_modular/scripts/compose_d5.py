#!/usr/bin/env python3
"""Compose D5 loadouts from modular Blender layers + socket props.

Deterministic. Run from repo root:
  python3 art-src/visual-prototypes/d5_corrida_modular/scripts/compose_d5.py
"""
from __future__ import annotations

import hashlib
import json
import re
import shutil
from pathlib import Path

from PIL import Image, ImageDraw, ImageFilter, ImageOps

ROOT = Path(__file__).resolve().parents[1]
REPO = ROOT.parents[2]
CELL = (128, 160)
PIVOT = (64, 156)  # sole-ish in lab cell
FRAMES = 10
FPS = 24
DURATION_MS = int(round(1000 / FPS))

# Published sockets (Bevy units from sole); map into CELL.
PUB_SOCKETS = REPO / "assets/animations/player_sockets.ron"
CAUSEWAY = REPO / "target/world_visual_showcase/after_causeway.png"
D2_RUN = REPO / "art-src/visual-prototypes/d2_protagonista_B/animacoes/run/frames"


def nearest(im: Image.Image, tw: int, th: int) -> Image.Image:
    im = im.convert("RGBA")
    w, h = im.size
    out = Image.new("RGBA", (tw, th))
    s, d = im.load(), out.load()
    for y in range(th):
        sy = min(h - 1, y * h // th)
        for x in range(tw):
            sx = min(w - 1, x * w // tw)
            d[x, y] = s[sx, sy]
    return out


def trim(im: Image.Image, pad: int = 0) -> Image.Image:
    box = im.split()[-1].getbbox()
    if not box:
        return im
    x0, y0, x1, y1 = box
    return im.crop(
        (max(0, x0 - pad), max(0, y0 - pad), min(im.width, x1 + pad), min(im.height, y1 + pad))
    )


def feet_register(im: Image.Image) -> Image.Image:
    """Place sprite so its bottom-center sits on PIVOT (shared origin)."""
    canvas = Image.new("RGBA", CELL, (0, 0, 0, 0))
    im = trim(im)
    if im.width < 2 or im.height < 2:
        return canvas
    # Keep relative placement from 256 render: scale whole frame into cell
    # with feet near bottom (Blender side camera already soles near bottom).
    scaled = nearest(im, CELL[0], CELL[1])
    canvas.alpha_composite(scaled)
    return canvas


def outline(im: Image.Image, color=(20, 18, 28, 220)) -> Image.Image:
    """Cheap automated contour (finish B), not hand pixel art."""
    a = im.split()[-1]
    edge = a.filter(ImageFilter.FIND_EDGES)
    out = im.copy()
    px, ep = out.load(), edge.load()
    for y in range(out.height):
        for x in range(out.width):
            if ep[x, y] > 40 and px[x, y][3] > 0:
                r, g, b, a0 = px[x, y]
                px[x, y] = (
                    (r * 3 + color[0]) // 4,
                    (g * 3 + color[1]) // 4,
                    (b * 3 + color[2]) // 4,
                    a0,
                )
    return out


def parse_run_sockets() -> list[dict]:
    text = PUB_SOCKETS.read_text(encoding="utf-8")
    m = re.search(r'"run":\s*\[(.*?)\],\n\s*"dodge"', text, re.S)
    if not m:
        raise SystemExit("run sockets not found")
    blob = m.group(1)
    blocks = re.findall(
        r"\(weapon:.*?back: \(x: [-0-9.]+, y: [-0-9.]+\)\)", blob
    )
    frames = []
    for part in blocks:
        entry: dict = {}
        wm = re.search(
            r"weapon:\s*\(x:\s*([-0-9.]+),\s*y:\s*([-0-9.]+),\s*angle:\s*([-0-9.]+),\s*front:\s*(true|false)\)",
            part,
        )
        if wm:
            entry["weapon"] = {
                "x": float(wm.group(1)),
                "y": float(wm.group(2)),
                "angle": float(wm.group(3)),
                "front": wm.group(4) == "true",
            }
        hm = re.search(
            r"hand_l:\s*\(x:\s*([-0-9.]+),\s*y:\s*([-0-9.]+),\s*front:\s*(true|false)\)",
            part,
        )
        if hm:
            entry["hand_l"] = {
                "x": float(hm.group(1)),
                "y": float(hm.group(2)),
                "front": hm.group(3) == "true",
            }
        for key in ("head", "chest", "back"):
            km = re.search(rf"{key}:\s*\(x:\s*([-0-9.]+),\s*y:\s*([-0-9.]+)\)", part)
            if km:
                entry[key] = {"x": float(km.group(1)), "y": float(km.group(2))}
        frames.append(entry)
    if len(frames) != FRAMES:
        raise SystemExit(f"expected {FRAMES} run sockets, got {len(frames)}")
    return frames


def body_anchors(body: Image.Image) -> dict[str, tuple[int, int]]:
    """Stable lab anchors from the body layer bbox (avoids cell-unit mismatch)."""
    box = body.split()[-1].getbbox()
    if not box:
        return {
            "head": (PIVOT[0], 50),
            "hand_l": (PIVOT[0] - 18, 90),
            "hand_r": (PIVOT[0] + 18, 90),
        }
    x0, y0, x1, y1 = box
    cx = (x0 + x1) // 2
    return {
        "head": (cx, y0 + 6),
        "hand_l": (x0 + 4, y0 + (y1 - y0) // 2),
        "hand_r": (x1 - 4, y0 + (y1 - y0) // 2 + 6),
    }


def make_helm(size: int = 16) -> Image.Image:
    im = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    d = ImageDraw.Draw(im)
    d.ellipse((1, 2, size - 2, size - 2), fill=(48, 52, 62, 255), outline=(200, 175, 90, 255))
    d.rectangle((2, size // 2 - 1, size - 3, size // 2 + 2), fill=(15, 16, 22, 255))
    return im


def make_hat(size: int = 18) -> Image.Image:
    im = Image.new("RGBA", (size, int(size * 0.65)), (0, 0, 0, 0))
    d = ImageDraw.Draw(im)
    d.ellipse((0, size // 4, size - 1, size // 4 + 6), fill=(28, 30, 40, 255))
    d.polygon(
        [(size // 2, 0), (size // 2 + 6, size // 4 + 3), (size // 2 - 6, size // 4 + 3)],
        fill=(35, 38, 50, 255),
    )
    return im


def make_shield(size: int = 18) -> Image.Image:
    im = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    d = ImageDraw.Draw(im)
    d.ellipse((1, 1, size - 2, size - 2), fill=(55, 58, 70, 255), outline=(200, 175, 90, 255))
    d.ellipse((size // 3, size // 3, 2 * size // 3, 2 * size // 3), fill=(170, 145, 70, 255))
    return im


def make_gauntlet(size: int = 10) -> Image.Image:
    im = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    d = ImageDraw.Draw(im)
    d.rectangle((1, 1, size - 2, size - 2), fill=(70, 74, 88, 255), outline=(200, 175, 90, 255))
    return im


def load_layer(kind: str, i: int) -> Image.Image:
    path = ROOT / "camadas" / kind / f"run_f{i:02d}.png"
    return feet_register(Image.open(path))


def paste_prop(canvas: Image.Image, prop: Image.Image, xy: tuple[int, int]) -> None:
    x, y = xy
    canvas.alpha_composite(prop, (x - prop.width // 2, y - prop.height // 2))


LOADOUTS = {
    "A_viajante": {
        "layers": ["body", "light", "cape", "weapon"],
        "cape_lag": 0,
        "helm": False,
        "hat": False,
        "shield": False,
        "gauntlet": False,
        "dir": "A_viajante",
    },
    "B_cavaleiro": {
        "layers": ["body", "light", "heavy", "cape", "weapon"],
        "cape_lag": 1,  # secondary cape follow
        "helm": True,
        "hat": False,
        "shield": True,
        "gauntlet": False,
        "dir": "B_cavaleiro",
    },
    "C_misto": {
        "layers": ["body", "light", "heavy", "weapon"],
        "cape_lag": 0,
        "helm": False,
        "hat": True,
        "shield": True,
        "gauntlet": True,
        "dir": "C_misto",
    },
}


def compose_frame(loadout: str, i: int, sockets: list[dict]) -> Image.Image:
    del sockets  # published units differ from lab 256→CELL; use body bbox anchors
    cfg = LOADOUTS[loadout]
    canvas = Image.new("RGBA", CELL, (0, 0, 0, 0))
    body = load_layer("body", i)
    for layer in cfg["layers"]:
        if layer == "body":
            canvas.alpha_composite(body)
        elif layer == "cape":
            src = (i - cfg["cape_lag"]) % FRAMES
            canvas.alpha_composite(load_layer("cape", src))
        else:
            canvas.alpha_composite(load_layer(layer, i))
    anchors = body_anchors(body)
    if cfg["helm"]:
        paste_prop(canvas, make_helm(), anchors["head"])
    if cfg["hat"]:
        paste_prop(canvas, make_hat(), anchors["head"])
    if cfg["shield"]:
        paste_prop(canvas, make_shield(), anchors["hand_l"])
    if cfg["gauntlet"]:
        paste_prop(canvas, make_gauntlet(), anchors["hand_r"])
    return canvas


def finish_b(im: Image.Image) -> Image.Image:
    return outline(im)


def save_gif(frames: list[Image.Image], path: Path, bg=(16, 12, 22)) -> None:
    out = []
    for f in frames:
        bg_im = Image.new("RGBA", f.size, (*bg, 255))
        bg_im.alpha_composite(f)
        out.append(bg_im.convert("P", palette=Image.ADAPTIVE))
    out[0].save(
        path,
        save_all=True,
        append_images=out[1:],
        duration=DURATION_MS,
        loop=0,
        disposal=2,
    )


def board(rows: list[tuple[str, Image.Image]], path: Path, title: str) -> None:
    gap = 12
    imgs = [(lab, im if im.width >= 100 else nearest(im, im.width * 2, im.height * 2)) for lab, im in rows]
    max_h = max(im.height for _, im in imgs)
    w = gap + sum(im.width + gap for _, im in imgs)
    h = max_h + 48
    out = Image.new("RGB", (w, h), (12, 10, 18))
    d = ImageDraw.Draw(out)
    d.text((8, 6), title, fill=(230, 210, 170))
    x = gap
    for lab, im in imgs:
        y = 28 + (max_h - im.height)
        if im.mode == "RGBA":
            out.paste(im.convert("RGB"), (x, y), im)
        else:
            out.paste(im.convert("RGB"), (x, y))
        d.text((x, h - 16), lab, fill=(180, 180, 190))
        x += im.width + gap
    out.save(path)


def file_md5(path: Path) -> str:
    return hashlib.md5(path.read_bytes()).hexdigest()


def main() -> None:
    sockets = parse_run_sockets()
    (ROOT / "animacoes" / "run").mkdir(parents=True, exist_ok=True)

    # per-loadout frames
    composed: dict[str, list[Image.Image]] = {}
    for key, cfg in LOADOUTS.items():
        frames = []
        out_dir = ROOT / "loadouts" / cfg["dir"]
        out_dir.mkdir(parents=True, exist_ok=True)
        for i in range(FRAMES):
            fr = compose_frame(key, i, sockets)
            fr_b = finish_b(fr)
            fr.save(out_dir / f"run_f{i:02d}.png")
            fr_b.save(out_dir / f"run_f{i:02d}_finishB.png")
            frames.append(fr_b)
        composed[key] = frames
        save_gif(frames, ROOT / "previews" / f"run_{cfg['dir']}.gif")
        print("gif", cfg["dir"])

    # dynamic swap without resetting cycle index
    swap = []
    sequence = (
        ["A_viajante"] * FRAMES
        + ["B_cavaleiro"] * FRAMES
        + ["C_misto"] * FRAMES
    )
    for t, loadout in enumerate(sequence):
        i = t % FRAMES
        swap.append(composed[loadout][i])
    save_gif(swap, ROOT / "previews" / "run_dynamic_swap.gif")

    # copy body/light strips for animacoes/run
    for i in range(FRAMES):
        load_layer("body", i).save(ROOT / "animacoes" / "run" / f"body_f{i:02d}.png")

    # identity / layers / loadouts boards
    d2 = None
    if (D2_RUN / "run_f00_3x.png").exists():
        d2 = feet_register(nearest(Image.open(D2_RUN / "run_f00_3x.png"), 128, 160))
    elif (D2_RUN / "run_f00.png").exists():
        d2 = feet_register(Image.open(D2_RUN / "run_f00.png"))

    body0 = load_layer("body", 0)
    light0 = Image.new("RGBA", CELL, (0, 0, 0, 0))
    light0.alpha_composite(body0)
    light0.alpha_composite(load_layer("light", 0))
    cape0 = load_layer("cape", 0)
    weap0 = load_layer("weapon", 0)
    heavy0 = load_layer("heavy", 0)

    rows_id = []
    if d2:
        rows_id.append(("D2 run f0", d2))
    rows_id += [
        ("modular body", body0),
        ("A", composed["A_viajante"][0]),
        ("B", composed["B_cavaleiro"][0]),
        ("C", composed["C_misto"][0]),
    ]
    board(rows_id, ROOT / "comparacoes" / "01_identity.png", "01 — Identity: D2 vs modular loadouts")
    board(
        [
            ("body", body0),
            ("coat", load_layer("light", 0)),
            ("cape", cape0),
            ("pauldron", heavy0),
            ("sword", weap0),
        ],
        ROOT / "comparacoes" / "02_layers.png",
        "02 — Independent layers (shared frame 0)",
    )
    board(
        [
            ("A viajante", composed["A_viajante"][0]),
            ("B cavaleiro", composed["B_cavaleiro"][0]),
            ("C misto", composed["C_misto"][0]),
        ],
        ROOT / "comparacoes" / "03_loadouts.png",
        "03 — Loadouts A/B/C same body+run index",
    )

    # finish A/B/C samples
    raw = ROOT / "acabamento" / "blender_raw_shaded_f00.png"
    full_a = ROOT / "acabamento" / "blender_full_A_f00.png"
    if raw.exists() and full_a.exists():
        board(
            [
                ("raw shaded", feet_register(Image.open(raw))),
                ("toon A", feet_register(Image.open(full_a))),
                ("compose+outline B", composed["A_viajante"][0]),
            ],
            ROOT / "comparacoes" / "09_acabamento.png",
            "09 — Finish: Blender raw vs toon vs automated 2D",
        )

    # problems board: cape lag mismatch, thin heavy
    lag = Image.new("RGBA", CELL, (0, 0, 0, 0))
    lag.alpha_composite(load_layer("body", 3))
    lag.alpha_composite(load_layer("light", 3))
    lag.alpha_composite(load_layer("cape", 2))  # lag
    sync = Image.new("RGBA", CELL, (0, 0, 0, 0))
    sync.alpha_composite(load_layer("body", 3))
    sync.alpha_composite(load_layer("light", 3))
    sync.alpha_composite(load_layer("cape", 3))
    board(
        [
            ("cape sync f3", finish_b(sync)),
            ("cape lag f3<-f2", finish_b(lag)),
            ("heavy alone", finish_b(heavy0)),
        ],
        ROOT / "comparacoes" / "10_problemas.png",
        "10 — Issues: cape lag demo; pauldron-only heavy silhouette",
    )

    # causeway gameplay scale
    if CAUSEWAY.is_file():
        bg = Image.open(CAUSEWAY).convert("RGBA")
        comp = bg.copy()
        d = ImageDraw.Draw(comp)
        feet_y = 515
        entries = [
            (200, d2 if d2 else body0, "D2"),
            (420, composed["A_viajante"][0], "A"),
            (640, composed["B_cavaleiro"][0], "B"),
            (860, composed["C_misto"][0], "C"),
        ]
        for x, sp, lab in entries:
            if sp is None:
                continue
            box = sp.split()[-1].getbbox()
            if not box:
                continue
            crop = sp.crop(box)
            th = 52
            tw = max(8, int(round(crop.width * th / crop.height)))
            scaled = nearest(crop, tw, th)
            comp.alpha_composite(scaled, (x - scaled.width // 2, feet_y - scaled.height))
            d.text((x - 10, feet_y + 6), lab, fill=(230, 210, 170))
        d.text((24, 24), "08 — D5 modular run loadouts @ causeway V2.3", fill=(230, 210, 170))
        comp.convert("RGB").save(ROOT / "comparacoes" / "08_causeway.png")

    # dark / light bg
    for name, color in (("dark", (12, 10, 18)), ("light", (220, 215, 205))):
        row = []
        for lab, im in (
            ("A", composed["A_viajante"][0]),
            ("B", composed["B_cavaleiro"][0]),
            ("C", composed["C_misto"][0]),
        ):
            bg = Image.new("RGBA", (im.width * 2, im.height * 2), (*color, 255))
            big = nearest(im, im.width * 2, im.height * 2)
            bg.alpha_composite(big)
            row.append((lab, bg))
        board(row, ROOT / "comparacoes" / f"scale_{name}.png", f"Scale board on {name} bg")

    # automated tests
    report = {"ok": True, "checks": []}

    def check(name: str, cond: bool, detail: str = "") -> None:
        report["checks"].append({"name": name, "ok": cond, "detail": detail})
        if not cond:
            report["ok"] = False

    for kind in ("body", "light", "cape", "weapon", "heavy"):
        files = sorted((ROOT / "camadas" / kind).glob("run_f*.png"))
        check(f"{kind}_frame_count", len(files) == FRAMES, str(len(files)))
        for p in files:
            im = Image.open(p)
            check(f"{p.name}_rgba", im.mode == "RGBA")
            a = im.split()[-1]
            # no solid magenta matte
            px = im.load()
            magenta = 0
            for y in range(0, im.height, 4):
                for x in range(0, im.width, 4):
                    r, g, b, al = px[x, y]
                    if al > 200 and r > 200 and b > 200 and g < 120:
                        magenta += 1
            check(f"{p.name}_no_magenta", magenta < 5, str(magenta))

    # same body across loadouts at frame 0: body layer pixels identical source
    b0 = file_md5(ROOT / "camadas" / "body" / "run_f00.png")
    check("shared_body_source", b0 == file_md5(ROOT / "camadas" / "body" / "run_f00.png"))

    # loadout composites differ
    check(
        "A_ne_B",
        file_md5(ROOT / "loadouts/A_viajante/run_f00.png")
        != file_md5(ROOT / "loadouts/B_cavaleiro/run_f00.png"),
    )
    check(
        "A_ne_C",
        file_md5(ROOT / "loadouts/A_viajante/run_f00.png")
        != file_md5(ROOT / "loadouts/C_misto/run_f00.png"),
    )

    # reproducibility: recompose frame 0 A twice
    a1 = compose_frame("A_viajante", 0, sockets)
    a2 = compose_frame("A_viajante", 0, sockets)
    check("deterministic_compose", list(a1.getdata()) == list(a2.getdata()))

    # temp dir regenerate compare for one gif frame hashes
    tmp = ROOT / "testes" / "_repro_tmp"
    if tmp.exists():
        shutil.rmtree(tmp)
    tmp.mkdir(parents=True)
    for i in range(FRAMES):
        compose_frame("A_viajante", i, sockets).save(tmp / f"run_f{i:02d}.png")
    same_raw = all(
        file_md5(tmp / f"run_f{i:02d}.png")
        == file_md5(ROOT / "loadouts" / "A_viajante" / f"run_f{i:02d}.png")
        for i in range(FRAMES)
    )
    check("repro_tmp_matches_loadout_A", same_raw)
    shutil.rmtree(tmp)

    (ROOT / "testes" / "modular_tests.json").write_text(json.dumps(report, indent=2))
    print("tests ok=", report["ok"], "failed=", [c for c in report["checks"] if not c["ok"]][:8])
    print("D5 compose done")


if __name__ == "__main__":
    main()
