#!/usr/bin/env python3
"""D12 — diagnose + refine modular protagonist frames (lab only)."""
from __future__ import annotations

import json
import math
import shutil
import struct
import zlib
from pathlib import Path

from PIL import Image, ImageDraw, ImageFilter, ImageFont

REPO = Path(__file__).resolve().parents[4]
LAB = Path(__file__).resolve().parents[1]
D10 = REPO / "art-src/visual-prototypes/d10_finalizacao_modular"
D7 = REPO / "art-src/visual-prototypes/d7_pixel_programatico"
D11_ASSETS = REPO / "assets/experimental/d11"
OUT_ASSETS = REPO / "assets/experimental/d12"
CMP = LAB / "comparacoes"
PREV = LAB / "previews"
ANIM = LAB / "animacoes"
TEST = LAB / "testes"
ATLAS = LAB / "atlas"

CELL = (128, 160)
PIVOT = (64, 156)
# Match published locomotion world height (density-2 cell 128 → 64 wu),
# keeping 128×160 aspect. D11 used 52 and looked undersized vs game art.
WORLD_H = 64.0
WORLD_W = WORLD_H * CELL[0] / CELL[1]
VIEW_H = 405.0
WIN_H = 720.0


def ensure_dirs() -> None:
    for p in (
        CMP,
        PREV,
        ANIM,
        TEST,
        ATLAS,
        LAB / "equipamentos",
        LAB / "mascaras",
        LAB / "fontes",
        LAB / "evidencias_bevy",
        OUT_ASSETS / "frames",
        OUT_ASSETS / "poses",
        OUT_ASSETS / "pipelines",
    ):
        p.mkdir(parents=True, exist_ok=True)


def load_rgba(path: Path) -> Image.Image:
    return Image.open(path).convert("RGBA")


def opaque_bbox(im: Image.Image, a_min: int = 1) -> tuple[int, int, int, int] | None:
    px = im.load()
    w, h = im.size
    xs: list[int] = []
    ys: list[int] = []
    for y in range(h):
        for x in range(w):
            if px[x, y][3] >= a_min:
                xs.append(x)
                ys.append(y)
    if not xs:
        return None
    return min(xs), min(ys), max(xs) + 1, max(ys) + 1


def luminance(r: int, g: int, b: int) -> float:
    return 0.2126 * r + 0.7152 * g + 0.0722 * b


def analyze_png(path: Path) -> dict:
    im = load_rgba(path)
    w, h = im.size
    px = im.load()
    semi = rgb0 = opaque = 0
    lums: list[float] = []
    for y in range(h):
        for x in range(w):
            r, g, b, a = px[x, y]
            if a == 0:
                if r | g | b:
                    rgb0 += 1
                continue
            if a < 255:
                semi += 1
            if a >= 250:
                opaque += 1
                lums.append(luminance(r, g, b))
    bb = opaque_bbox(im)
    sole = (bb[3] - 1) if bb else None
    lums.sort()
    return {
        "file": str(path.relative_to(REPO)) if path.is_relative_to(REPO) else str(path),
        "cell": [w, h],
        "bbox": list(bb) if bb else None,
        "bbox_h": (bb[3] - bb[1]) if bb else 0,
        "sole_y": sole,
        "sole_delta_vs_156": (sole - 156) if sole is not None else None,
        "opaque": opaque,
        "semi_alpha": semi,
        "rgb_in_clear": rgb0,
        "lum_p50": round(lums[len(lums) // 2], 1) if lums else None,
        "lum_p90": round(lums[int(0.9 * (len(lums) - 1))], 1) if lums else None,
        "fill_h": round((bb[3] - bb[1]) / h, 3) if bb else 0,
    }


def nearest_scale(im: Image.Image, factor: int) -> Image.Image:
    """Nearest upscale without PIL.resize (broken on some system Pillow builds)."""
    if factor <= 1:
        return im.copy()
    src = im.convert("RGBA")
    w, h = src.size
    out = Image.new("RGBA", (w * factor, h * factor))
    sp = src.load()
    dp = out.load()
    for y in range(h):
        for x in range(w):
            p = sp[x, y]
            x0, y0 = x * factor, y * factor
            for dy in range(factor):
                for dx in range(factor):
                    dp[x0 + dx, y0 + dy] = p
    return out


def nearest_fit(im: Image.Image, size: tuple[int, int]) -> Image.Image:
    """Nearest resize to exact size via integer box sampling."""
    src = im.convert("RGBA")
    sw, sh = src.size
    dw, dh = size
    out = Image.new("RGBA", (dw, dh))
    sp = src.load()
    dp = out.load()
    for y in range(dh):
        sy = min(sh - 1, y * sh // dh)
        for x in range(dw):
            sx = min(sw - 1, x * sw // dw)
            dp[x, y] = sp[sx, sy]
    return out


def place_on(bg: Image.Image, sprite: Image.Image, cx: int, sole_y: int, pivot=PIVOT) -> None:
    """Paste sprite so its pivot sole lands on (cx, sole_y)."""
    x = cx - pivot[0]
    y = sole_y - pivot[1]
    bg.alpha_composite(sprite, (x, y))


def board(cols: list[tuple[str, Image.Image]], cell_scale: int = 3, pad: int = 12) -> Image.Image:
    tiles = [(label, nearest_scale(im, cell_scale)) for label, im in cols]
    tw = max(t.size[0] for _, t in tiles)
    th = max(t.size[1] for _, t in tiles)
    label_h = 22
    w = pad + len(tiles) * (tw + pad)
    h = pad + label_h + th + pad
    out = Image.new("RGBA", (w, h), (28, 26, 32, 255))
    draw = ImageDraw.Draw(out)
    for i, (label, tile) in enumerate(tiles):
        x = pad + i * (tw + pad)
        draw.text((x, pad), label, fill=(220, 210, 200, 255))
        out.alpha_composite(tile, (x, pad + label_h))
    return out


# --- Pipeline A: programmatic refine of D10 ---


def clamp(v: float) -> int:
    return int(max(0, min(255, round(v))))


def refine_contrast(im: Image.Image, variant: str) -> Image.Image:
    """A=preserve, B=readable gothic, C=dramatic (no neon)."""
    src = im.load()
    out = Image.new("RGBA", im.size, (0, 0, 0, 0))
    dst = out.load()
    w, h = im.size
    if variant == "A":
        gain, lift, metal, wine, face = 1.05, 4, 1.08, 1.04, 1.10
    elif variant == "B":
        gain, lift, metal, wine, face = 1.14, 10, 1.22, 1.10, 1.22
    else:  # C
        gain, lift, metal, wine, face = 1.22, 14, 1.35, 1.14, 1.30

    for y in range(h):
        for x in range(w):
            r, g, b, a = src[x, y]
            if a == 0:
                continue
            # Soft rim: lift edge pixels slightly (neighbors with a==0).
            edge = False
            for dy, dx in ((-1, 0), (1, 0), (0, -1), (0, 1)):
                nx, ny = x + dx, y + dy
                if 0 <= nx < w and 0 <= ny < h and src[nx, ny][3] == 0:
                    edge = True
                    break
            rr, gg, bb = float(r), float(g), float(b)
            lum = luminance(r, g, b)
            # Face-ish: pale midtones in upper third.
            if y < 90 and lum > 35 and abs(r - g) < 35 and b < r + 20:
                rr *= face
                gg *= face
                bb *= face * 0.98
            # Wine coat: red-dominant mid darks.
            if r > g + 8 and r > b + 5 and 15 < lum < 90:
                rr *= wine
                gg *= 0.98
                bb *= 0.97
            # Metal: high-g grey or cool highlights.
            if lum > 55 and abs(r - g) < 18 and abs(g - b) < 22:
                rr *= metal
                gg *= metal
                bb *= metal * 1.02
            rr = rr * gain + lift
            gg = gg * gain + lift
            bb = bb * gain + lift
            if edge and variant != "A":
                # Controlled rim (cool), not neon outline.
                rr += 18 if variant == "B" else 28
                gg += 20 if variant == "B" else 30
                bb += 28 if variant == "B" else 40
            dst[x, y] = (clamp(rr), clamp(gg), clamp(bb), 255 if a >= 128 else a)
    return out


def boost_sword(im: Image.Image) -> Image.Image:
    """Local metal lift on elongated cool/grey pixels (blade heuristic)."""
    src = im.load()
    out = im.copy()
    dst = out.load()
    w, h = im.size
    for y in range(h):
        for x in range(w):
            r, g, b, a = src[x, y]
            if a < 200:
                continue
            lum = luminance(r, g, b)
            # Blade-ish: mid-upper grey streak, often right of body for classic idle.
            if lum > 40 and abs(r - g) < 25 and abs(g - b) < 30 and x > 55:
                dst[x, y] = (
                    clamp(r * 1.25 + 12),
                    clamp(g * 1.25 + 14),
                    clamp(b * 1.28 + 18),
                    a,
                )
    return out


def contact_shadow_layer(width: int = 48, height: int = 10, alpha: int = 110) -> Image.Image:
    sh = Image.new("RGBA", (width, height), (0, 0, 0, 0))
    px = sh.load()
    cx, cy = width / 2, height / 2
    for y in range(height):
        for x in range(width):
            nx = (x - cx) / (width / 2)
            ny = (y - cy) / (height / 2)
            d = nx * nx + ny * ny
            if d <= 1.0:
                a = int(alpha * (1.0 - d) ** 1.4)
                px[x, y] = (8, 6, 10, a)
    return sh


def with_contact_shadow(sprite: Image.Image) -> Image.Image:
    out = Image.new("RGBA", sprite.size, (0, 0, 0, 0))
    sh = contact_shadow_layer()
    sx = PIVOT[0] - sh.size[0] // 2
    sy = PIVOT[1] - sh.size[1] // 2 + 2
    out.alpha_composite(sh, (sx, sy))
    out.alpha_composite(sprite, (0, 0))
    return out


def silhouette(im: Image.Image, fg: tuple[int, int, int, int], bg: tuple[int, int, int, int]) -> Image.Image:
    out = Image.new("RGBA", im.size, bg)
    src = im.load()
    dst = out.load()
    for y in range(im.size[1]):
        for x in range(im.size[0]):
            if src[x, y][3] > 0:
                dst[x, y] = fg
    return out


def afterimage_variant(im: Image.Image, kind: str) -> Image.Image:
    """A wine, B wine+edge, C wine+magenta."""
    src = im.load()
    out = Image.new("RGBA", im.size, (0, 0, 0, 0))
    dst = out.load()
    w, h = im.size
    for y in range(h):
        for x in range(w):
            r, g, b, a = src[x, y]
            if a == 0:
                continue
            edge = any(
                0 <= x + dx < w
                and 0 <= y + dy < h
                and src[x + dx, y + dy][3] == 0
                for dx, dy in ((-1, 0), (1, 0), (0, -1), (0, 1))
            )
            if kind == "A":
                rr, gg, bb, aa = 140, 30, 55, 140
            elif kind == "B":
                rr, gg, bb, aa = (170, 80, 100, 150) if edge else (140, 30, 55, 140)
            else:
                rr, gg, bb, aa = (160, 40, 90, 145) if not edge else (200, 70, 140, 160)
            # Preserve a bit of luminance from source so gear silhouette reads.
            f = 0.35
            dst[x, y] = (
                clamp(rr * (1 - f) + r * f),
                clamp(gg * (1 - f) + g * f),
                clamp(bb * (1 - f) + b * f),
                aa,
            )
    return out


def gif_from_frames(frames: list[Image.Image], path: Path, duration_ms: int) -> None:
    if not frames:
        return
    rgb = []
    for f in frames:
        bg = Image.new("RGBA", f.size, (18, 16, 22, 255))
        bg.alpha_composite(f)
        rgb.append(bg.convert("P", palette=Image.ADAPTIVE, colors=64))
    rgb[0].save(
        path,
        save_all=True,
        append_images=rgb[1:],
        duration=duration_ms,
        loop=0,
        disposal=2,
    )


def scale_math() -> dict:
    px_per_wu = WIN_H / VIEW_H
    return {
        "cell_px": list(CELL),
        "pivot_sole_px": list(PIVOT),
        "world_size_d11_wrong": [52.0 * CELL[0] / CELL[1], 52.0],
        "world_size_d12": [WORLD_W, WORLD_H],
        "published_locomotion_world": [64.0, 64.0],
        "view_height": VIEW_H,
        "window_h": WIN_H,
        "px_per_world_unit": round(px_per_wu, 4),
        "sprite_screen_h_d11": round(52.0 * px_per_wu, 2),
        "sprite_screen_h_d12": round(WORLD_H * px_per_wu, 2),
        "sprite_screen_h_published": round(64.0 * px_per_wu, 2),
        "d11_missing_anchor": True,
        "anchor_formula": "ax = px/cw - 0.5; ay = 0.5 - py/ch",
        "anchor_d12": [
            PIVOT[0] / CELL[0] - 0.5,
            0.5 - PIVOT[1] / CELL[1],
        ],
    }


def process_clip(loadout: str, clip: str, count: int, variant: str) -> list[Image.Image]:
    frames: list[Image.Image] = []
    dst_dir = OUT_ASSETS / "frames" / loadout / clip
    dst_dir.mkdir(parents=True, exist_ok=True)
    lab_dir = ANIM / clip / loadout
    lab_dir.mkdir(parents=True, exist_ok=True)
    for i in range(count):
        src = D10 / "animacoes" / clip / f"{loadout}_f{i:02d}.png"
        im = load_rgba(src)
        refined = boost_sword(refine_contrast(im, variant))
        refined.save(dst_dir / f"f{i:02d}.png")
        refined.save(lab_dir / f"f{i:02d}.png")
        frames.append(refined)
    return frames


def try_blender_pipeline_b() -> Image.Image | None:
    """Optional: if a prior Blender PNG exists in lab, use it; else synthetic stub note."""
    # Prefer any manual drop-in; otherwise leave None — script documents absence.
    candidates = list((LAB / "blender").glob("*.png"))
    if candidates:
        return nearest_fit(load_rgba(candidates[0]), CELL)
    return None


def hybrid_pipeline_c(base: Image.Image, blender_ref: Image.Image | None) -> Image.Image:
    """Depth/rim cue from Blender luminance into D10-refined base."""
    out = base.copy()
    if blender_ref is None:
        # Soft programmatic rim already in B; mark as hybrid-lite.
        return refine_contrast(boost_sword(base), "B")
    bl = nearest_fit(blender_ref, CELL).convert("RGBA")
    sp = out.load()
    bp = bl.load()
    for y in range(CELL[1]):
        for x in range(CELL[0]):
            r, g, b, a = sp[x, y]
            if a == 0:
                continue
            br, bg, bb, ba = bp[x, y]
            if ba < 10:
                continue
            blum = luminance(br, bg, bb)
            # Add cool rim where Blender is bright.
            if blum > 90:
                sp[x, y] = (
                    clamp(r + 12),
                    clamp(g + 16),
                    clamp(b + 28),
                    a,
                )
    return out


def main() -> None:
    ensure_dirs()
    report: dict = {"scale": scale_math(), "sources": {}, "pipelines": {}}

    # Audit sources
    for key, path in {
        "d10_classico": D10 / "loadouts/classico/pose.png",
        "d10_pesado": D10 / "loadouts/pesado/pose.png",
        "d7b_leve": D7 / "loadouts/leve/pose.png",
        "d11_classico": D11_ASSETS / "poses/classico.png",
    }.items():
        report["sources"][key] = analyze_png(path)

    d10c = load_rgba(D10 / "loadouts/classico/pose.png")
    d10p = load_rgba(D10 / "loadouts/pesado/pose.png")
    d7 = load_rgba(D7 / "loadouts/leve/pose.png")

    # Contrast variants
    va = refine_contrast(d10c, "A")
    vb = boost_sword(refine_contrast(d10c, "B"))
    vc = boost_sword(refine_contrast(d10c, "C"))
    for name, im in (("A", va), ("B", vb), ("C", vc)):
        im.save(OUT_ASSETS / "pipelines" / f"contrast_{name}.png")
        im.save(LAB / "equipamentos" / f"contrast_{name}.png")

    # Chosen default refine for publish: B (readable gothic)
    chosen = "B"
    classic_pose = boost_sword(refine_contrast(d10c, chosen))
    heavy_pose = boost_sword(refine_contrast(d10p, chosen))
    classic_pose.save(OUT_ASSETS / "poses/classico.png")
    heavy_pose.save(OUT_ASSETS / "poses/pesado.png")
    classic_pose.save(LAB / "atlas/classico_pose.png")
    heavy_pose.save(LAB / "atlas/pesado_pose.png")

    blender_ref = try_blender_pipeline_b()
    pipe_a = classic_pose
    pipe_b = blender_ref if blender_ref is not None else silhouette(
        d10c, (180, 180, 190, 255), (20, 20, 24, 255)
    )
    # If no Blender render, annotate placeholder so it's not mistaken for Bevy.
    if blender_ref is None:
        pipe_b = classic_pose.copy()
        d = ImageDraw.Draw(pipe_b)
        d.rectangle((2, 2, 125, 20), fill=(40, 20, 20, 220))
        d.text((6, 5), "B: no blend render", fill=(240, 200, 180, 255))
    pipe_c = hybrid_pipeline_c(d10c, blender_ref)
    pipe_a.save(OUT_ASSETS / "pipelines/A_pixel_refined.png")
    pipe_b.save(OUT_ASSETS / "pipelines/B_blender.png")
    pipe_c.save(OUT_ASSETS / "pipelines/C_hybrid.png")

    # Evidence boards (Python)
    diag = Image.new("RGBA", (900, 420), (22, 20, 26, 255))
    draw = ImageDraw.Draw(diag)
    draw.text((16, 12), "D11 diagnosis (lab)", fill=(230, 220, 210, 255))
    lines = [
        f"D11 world_h=52 → screen ~{report['scale']['sprite_screen_h_d11']}px (undersized vs published 64wu/~{report['scale']['sprite_screen_h_published']}px)",
        "D11 example used default Sprite Anchor (center), not sole pivot (64,156)",
        f"Classic opaque fill_h={report['sources']['d10_classico']['fill_h']}, lum_p50={report['sources']['d10_classico']['lum_p50']} (too dark on dark BG)",
        f"Alpha: semi={report['sources']['d10_classico']['semi_alpha']} rgb0={report['sources']['d10_classico']['rgb_in_clear']} (PNG clean; 'soft pixels' often from eval/scale)",
        "Equipment disproportion: heavy bbox wider; classic sword low-contrast vs coat",
    ]
    for i, line in enumerate(lines):
        draw.text((16, 48 + i * 28), line, fill=(200, 190, 180, 255))
    for i, (label, im) in enumerate(
        (("D7-B", d7), ("D10", d10c), ("D12-B", vb), ("D12-C", vc))
    ):
        t = nearest_scale(im, 2)
        diag.alpha_composite(t, (16 + i * 220, 200))
        draw.text((16 + i * 220, 180), label, fill=(210, 200, 190, 255))
    diag.save(CMP / "01_D11_diagnostico.png")

    # Scale / pivots
    piv = Image.new("RGBA", (640, 360), (24, 22, 28, 255))
    pd = ImageDraw.Draw(piv)
    for i, (lab, im) in enumerate((("D10", d10c), ("D12", classic_pose))):
        x0 = 40 + i * 300
        piv.alpha_composite(nearest_scale(im, 2), (x0, 40))
        # pivot mark at 2x
        px = x0 + PIVOT[0] * 2
        py = 40 + PIVOT[1] * 2
        pd.ellipse((px - 3, py - 3, px + 3, py + 3), outline=(255, 80, 80, 255))
        pd.line((x0, py, x0 + 256, py), fill=(120, 120, 140, 255))
        pd.text((x0, 12), f"{lab} pivot (64,156)", fill=(230, 220, 210, 255))
    piv.save(CMP / "02_escala_e_pivos.png")

    grey_bg = Image.new("RGBA", CELL, (120, 118, 122, 255))
    grey_bg.alpha_composite(classic_pose)
    dark_bg = Image.new("RGBA", CELL, (12, 10, 14, 255))
    dark_bg.alpha_composite(classic_pose)
    board(
        [
            ("sil A", silhouette(classic_pose, (0, 0, 0, 255), (255, 255, 255, 255))),
            ("sil B", silhouette(classic_pose, (255, 255, 255, 255), (0, 0, 0, 255))),
            ("near-black", dark_bg),
            ("grey", grey_bg),
        ],
        2,
    ).save(CMP / "03_silhueta.png")

    board(
        [("A preserve", va), ("B readable", vb), ("C dramatic", vc)],
        3,
    ).save(CMP / "04_contraste_A_B_C.png")

    board(
        [("D10 sword", d10c), ("D12 sword+", classic_pose)],
        4,
    ).save(CMP / "05_espada_e_maos.png")

    # Alpha / contours diagnostic zoom
    alpha_board = Image.new("RGBA", (800, 360), (16, 16, 18, 255))
    ad = ImageDraw.Draw(alpha_board)
    crop = d10c.crop((50, 40, 100, 100))
    alpha_board.alpha_composite(nearest_scale(crop, 5), (20, 40))
    alpha_board.alpha_composite(nearest_scale(classic_pose.crop((50, 40, 100, 100)), 5), (420, 40))
    ad.text((20, 10), "D10 edge 5x", fill=(220, 210, 200, 255))
    ad.text((420, 10), "D12-B edge 5x", fill=(220, 210, 200, 255))
    ad.text(
        (20, 320),
        f"semi_alpha={report['sources']['d10_classico']['semi_alpha']} rgb_clear={report['sources']['d10_classico']['rgb_in_clear']}",
        fill=(180, 170, 160, 255),
    )
    alpha_board.save(CMP / "06_alfa_e_contornos.png")

    # Blender lights/materials placeholders (filled when blender script runs)
    for name in ("07_blender_luzes.png", "08_blender_materiais.png"):
        img = Image.new("RGBA", (640, 280), (20, 22, 28, 255))
        d = ImageDraw.Draw(img)
        d.text((20, 20), name.replace(".png", "") + " — see ESTUDO_BLENDER.md", fill=(210, 200, 190, 255))
        d.text((20, 50), "Blender 5.2.4 LTS lab copy; EEVEE not auto-chosen", fill=(160, 150, 140, 255))
        if blender_ref:
            img.alpha_composite(nearest_scale(blender_ref, 2), (20, 90))
        img.save(CMP / name)

    board(
        [("Pipe A", pipe_a), ("Pipe B", pipe_b), ("Pipe C", pipe_c)],
        3,
    ).save(CMP / "09_pipeline_A_B_C.png")

    board([("classic refined", classic_pose)], 4).save(CMP / "10_classico_refinado.png")
    board([("heavy refined", heavy_pose)], 4).save(CMP / "11_pesado_refinado.png")

    board(
        [("no shadow", classic_pose), ("contact", with_contact_shadow(classic_pose))],
        3,
    ).save(CMP / "12_sombra_contato.png")

    ai_a = afterimage_variant(classic_pose, "A")
    ai_b = afterimage_variant(classic_pose, "B")
    ai_c = afterimage_variant(classic_pose, "C")
    board([("wine A", ai_a), ("edge B", ai_b), ("magenta C", ai_c)], 3).save(
        CMP / "13_after_image_B.png"
    )

    # Process animation frames (chosen contrast B)
    idle_c = process_clip("classico", "idle", 6, chosen)
    run_c = process_clip("classico", "run", 8, chosen)
    idle_p = process_clip("pesado", "idle", 6, chosen)
    run_p = process_clip("pesado", "run", 8, chosen)

    strip = Image.new("RGBA", (128 * 8 + 24, 360), (18, 16, 22, 255))
    sd = ImageDraw.Draw(strip)
    sd.text((8, 4), "idle classic", fill=(220, 210, 200, 255))
    for i, f in enumerate(idle_c):
        strip.alpha_composite(f, (8 + i * 128, 24))
    sd.text((8, 190), "run classic", fill=(220, 210, 200, 255))
    for i, f in enumerate(run_c):
        strip.alpha_composite(f, (8 + i * 128, 210))
    strip.save(CMP / "14_idle_run.png")

    # Causeway / real-scale mock (view 405): character ~114px tall
    sh = int(round(WORLD_H * WIN_H / VIEW_H))
    sw = int(round(WORLD_W * WIN_H / VIEW_H))
    scaled = nearest_fit(classic_pose, (sw, sh))
    sole_off = int(sh * PIVOT[1] / CELL[1])
    causeway = Image.new("RGBA", (1280, 720), (28, 32, 40, 255))
    cd = ImageDraw.Draw(causeway)
    cd.rectangle((0, 500, 1280, 720), fill=(50, 48, 56, 255))
    cd.line((0, 500, 1280, 500), fill=(90, 88, 96, 255))
    causeway.alpha_composite(scaled, (640 - PIVOT[0] * sw // CELL[0], 500 - sole_off))
    causeway.alpha_composite(nearest_scale(classic_pose, 2), (40, 40))
    causeway.alpha_composite(nearest_scale(classic_pose, 4), (320, 40))
    cd.text((40, 20), "2x inspect", fill=(220, 210, 200, 255))
    cd.text((320, 20), "4x inspect", fill=(220, 210, 200, 255))
    cd.text((900, 40), "1x real scale (view 405)", fill=(220, 210, 200, 255))
    causeway.save(CMP / "15_causeway_escala_real.png")

    final = board(
        [
            ("D7-B", d7),
            ("D10", d10c),
            ("D12-A", pipe_a),
            ("D12-C", pipe_c),
            ("heavy", heavy_pose),
        ],
        2,
    )
    final.save(CMP / "16_resultado_final.png")

    # GIFs
    gif_from_frames(idle_c, PREV / "idle_classico.gif", 100)
    gif_from_frames(idle_p, PREV / "idle_pesado.gif", 100)
    gif_from_frames(run_c, PREV / "run_classico.gif", 83)
    gif_from_frames(run_p, PREV / "run_pesado.gif", 83)
    # dynamic swap: classic run then heavy run keeping phase
    swap = []
    for i in range(8):
        swap.append(run_c[i] if i < 4 else run_p[i])
    gif_from_frames(swap, PREV / "troca_dinamica.gif", 83)
    trail = []
    for i, f in enumerate(run_c):
        canvas = Image.new("RGBA", CELL, (14, 12, 18, 255))
        if i > 0:
            canvas.alpha_composite(ai_b, (-6, 0))
        if i > 1:
            ghost = ai_b.copy()
            canvas.alpha_composite(ghost, (-12, 0))
        canvas.alpha_composite(f, (0, 0))
        trail.append(canvas)
    gif_from_frames(trail, PREV / "after_image_vinho.gif", 83)

    # Tests
    failures = []
    for path in OUT_ASSETS.rglob("*.png"):
        info = analyze_png(path)
        if info["cell"] != list(CELL) and "pipelines" not in str(path) and "contrast" not in path.name:
            # poses/frames must be 128x160
            if "frames" in str(path) or "poses" in str(path):
                if info["cell"] != list(CELL):
                    failures.append(f"bad cell {path}")
        if "frames" in str(path) or path.parent.name == "poses":
            if info["semi_alpha"] > 50:
                failures.append(f"too much semi {path}")
            if info["sole_delta_vs_156"] not in (0, None) and abs(info["sole_delta_vs_156"] or 0) > 2:
                failures.append(f"sole drift {path}: {info['sole_delta_vs_156']}")

    # contract
    contract = {
        "version": 1,
        "id": "d12_refined_experimental",
        "cell_px": list(CELL),
        "pivot_sole_px": list(PIVOT),
        "world_height": WORLD_H,
        "world_width": WORLD_W,
        "filter": "nearest",
        "anchor": "sole_pivot",
        "contrast_variant_default": chosen,
        "d11_world_height_was": 52.0,
        "notes": [
            "World height raised 52→64 to match published locomotion height.",
            "Bevy lab must use Anchor from pivot; nearest sampler per image.",
            "Pipeline A (D10 refine B) recommended pending owner choice.",
        ],
    }
    (OUT_ASSETS / "contract.json").write_text(json.dumps(contract, indent=2), encoding="utf-8")
    (TEST / "audit_report.json").write_text(json.dumps(report, indent=2), encoding="utf-8")
    (TEST / "test_results.json").write_text(
        json.dumps({"failures": failures, "ok": not failures}, indent=2),
        encoding="utf-8",
    )
    if failures:
        raise SystemExit(f"D12 tests failed: {failures}")
    print("D12 pipeline OK")
    print(json.dumps(report["scale"], indent=2))


if __name__ == "__main__":
    main()
