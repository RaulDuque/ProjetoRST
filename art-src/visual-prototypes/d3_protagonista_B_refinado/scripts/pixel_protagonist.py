#!/usr/bin/env python3
"""D3 constructed pixel protagonist (identity-locked layers).

This is NOT downsample of generated concepts. Each frame is drawn from the
same part templates + small offsets. Output stays under d3_protagonista_B_refinado/.
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import Iterable

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[1]
CELL_W, CELL_H = 64, 80
PIVOT = (32, 78)  # sole centre
CHAR_H = 52

# Documented palette (hex -> RGB)
PALETTE = {
    "outline": (13, 11, 18),
    "rim": (74, 85, 112),
    "skin": (220, 198, 180),
    "skin_s": (179, 154, 138),
    "hair": (58, 62, 75),
    "hair_l": (93, 100, 118),
    "coat": (29, 36, 52),
    "coat_s": (20, 26, 38),
    "coat_l": (44, 54, 80),
    "cape": (87, 24, 39),
    "cape_s": (59, 16, 27),
    "cape_out": (33, 28, 48),
    "metal": (185, 195, 207),
    "metal_l": (230, 237, 243),
    "metal_d": (125, 135, 148),
    "amber": (209, 143, 47),
    "amber_d": (143, 90, 28),
    "boot": (23, 22, 29),
    "boot_c": (58, 52, 70),
    "glove": (44, 35, 41),
    "trouser": (38, 42, 54),
    "blade": (230, 237, 243),
    "blade_e": (125, 135, 148),
}


def rgb(name: str) -> tuple[int, int, int]:
    return PALETTE[name]


def new_canvas() -> Image.Image:
    return Image.new("RGBA", (CELL_W, CELL_H), (0, 0, 0, 0))


def px(img: Image.Image, x: int, y: int, color: str | tuple[int, int, int]) -> None:
    if isinstance(color, str):
        color = rgb(color)
    if 0 <= x < img.width and 0 <= y < img.height:
        img.putpixel((x, y), (*color, 255))


def fill_rect(
    img: Image.Image, x0: int, y0: int, x1: int, y1: int, color: str
) -> None:
    for y in range(min(y0, y1), max(y0, y1) + 1):
        for x in range(min(x0, x1), max(x0, x1) + 1):
            px(img, x, y, color)


def hline(img: Image.Image, x0: int, x1: int, y: int, color: str) -> None:
    for x in range(min(x0, x1), max(x0, x1) + 1):
        px(img, x, y, color)


def vline(img: Image.Image, x: int, y0: int, y1: int, color: str) -> None:
    for y in range(min(y0, y1), max(y0, y1) + 1):
        px(img, x, y, color)


def outline_rect(
    img: Image.Image, x0: int, y0: int, x1: int, y1: int, color: str = "rim"
) -> None:
    hline(img, x0, x1, y0, color)
    hline(img, x0, x1, y1, color)
    vline(img, x0, y0, y1, color)
    vline(img, x1, y0, y1, color)


def nearest_up(im: Image.Image, scale: int) -> Image.Image:
    im = im.convert("RGBA")
    w, h = im.size
    out = Image.new("RGBA", (w * scale, h * scale))
    s = im.load()
    d = out.load()
    for y in range(h):
        for x in range(w):
            c = s[x, y]
            for dy in range(scale):
                for dx in range(scale):
                    d[x * scale + dx, y * scale + dy] = c
    return out


class Protagonist:
    """Side-view facing right. Origin: feet at PIVOT."""

    def __init__(self, detail: str = "B1"):
        assert detail in ("B1", "B2")
        self.detail = detail

    def draw(
        self,
        *,
        breath: int = 0,
        cape_sway: int = 0,
        leg_phase: int = 0,
        arm_phase: int = 0,
        sword_angle: str = "rest",
        lean: int = 0,
    ) -> Image.Image:
        img = new_canvas()
        # Vertical layout from sole upward (CHAR_H pixels).
        sole_y = PIVOT[1]
        cx = PIVOT[0] + lean  # body centre x
        top = sole_y - CHAR_H + breath

        # --- cape back ---
        cape_x0 = cx - 10 + cape_sway
        cape_x1 = cx - 2 + cape_sway
        fill_rect(img, cape_x0, top + 10, cape_x1, sole_y - 2, "cape_out")
        fill_rect(img, cape_x0 + 1, top + 12, cape_x1, sole_y - 4, "cape")
        if cape_sway >= 0:
            fill_rect(img, cape_x0, sole_y - 8, cape_x0 + 2, sole_y - 2, "cape_s")
        else:
            fill_rect(img, cape_x1 - 1, sole_y - 10, cape_x1 + 1, sole_y - 3, "cape")

        # --- legs ---
        # phase 0..9 for run; idle uses 0
        boot_y0 = sole_y - 8
        if leg_phase == 0:
            # both planted
            fill_rect(img, cx - 4, sole_y - 18, cx - 1, sole_y - 1, "trouser")
            fill_rect(img, cx + 1, sole_y - 18, cx + 4, sole_y - 1, "trouser")
            fill_rect(img, cx - 5, boot_y0, cx - 1, sole_y, "boot")
            fill_rect(img, cx + 1, boot_y0, cx + 5, sole_y, "boot")
            hline(img, cx - 5, cx - 1, boot_y0 + 2, "boot_c")
            hline(img, cx + 1, cx + 5, boot_y0 + 2, "boot_c")
        else:
            # run cycle: front/back offsets
            table = [
                (2, -2),
                (4, -3),
                (3, -1),
                (1, 1),
                (-1, 3),
                (-3, 4),
                (-4, 2),
                (-2, 0),
                (0, -1),
                (1, -2),
            ]
            fx, bx = table[(leg_phase - 1) % 10]
            # back leg
            fill_rect(img, cx - 3 + bx, sole_y - 17, cx + bx, sole_y - 1, "trouser")
            fill_rect(img, cx - 4 + bx, boot_y0, cx + bx, sole_y, "boot")
            # front leg
            fill_rect(img, cx + fx, sole_y - 17, cx + 3 + fx, sole_y - 1, "trouser")
            fill_rect(img, cx - 1 + fx, boot_y0, cx + 4 + fx, sole_y, "boot")

        # --- torso / coat ---
        torso_y0 = top + 14
        torso_y1 = sole_y - 16
        fill_rect(img, cx - 5, torso_y0, cx + 5, torso_y1, "coat")
        fill_rect(img, cx - 4, torso_y0 + 1, cx + 4, torso_y1 - 1, "coat_l")
        # open lining
        fill_rect(img, cx - 1, torso_y0 + 4, cx + 1, torso_y1 - 2, "cape")
        # belt
        hline(img, cx - 5, cx + 5, torso_y0 + 16, "boot")
        px(img, cx, torso_y0 + 16, "amber")

        # armor
        fill_rect(img, cx + 3, torso_y0 + 2, cx + 6, torso_y0 + 8, "metal_d")  # pauldron
        px(img, cx + 5, torso_y0 + 3, "metal_l")
        if self.detail == "B2":
            fill_rect(img, cx - 4, torso_y0 + 3, cx + 2, torso_y0 + 6, "metal_d")
            hline(img, cx - 4, cx + 2, torso_y0 + 5, "metal")
            # coat panels
            vline(img, cx - 3, torso_y0 + 8, torso_y1 - 3, "coat_s")
            vline(img, cx + 3, torso_y0 + 8, torso_y1 - 3, "coat_s")
            # cuff metal
            fill_rect(img, cx + 4, torso_y0 + 12, cx + 6, torso_y0 + 14, "metal")

        # --- head (short slate hair + fringe — not a flat cap) ---
        head_y0 = top + 2
        fill_rect(img, cx - 3, head_y0 + 2, cx + 3, head_y0 + 10, "skin")
        fill_rect(img, cx - 2, head_y0 + 3, cx + 2, head_y0 + 9, "skin")
        # hair volume on crown + nape + forward fringe
        fill_rect(img, cx - 3, head_y0, cx + 2, head_y0 + 3, "hair")
        fill_rect(img, cx - 4, head_y0 + 1, cx - 2, head_y0 + 5, "hair")  # nape
        px(img, cx + 3, head_y0 + 2, "hair")  # fringe tip
        px(img, cx + 2, head_y0 + 1, "hair_l")
        px(img, cx + 2, head_y0 + 3, "hair")
        px(img, cx + 1, head_y0 + 5, "amber")  # eye glint
        if self.detail == "B2":
            px(img, cx - 1, head_y0 + 6, "skin_s")
            px(img, cx + 3, head_y0 + 3, "hair_l")
            hline(img, cx - 3, cx + 2, head_y0 + 9, "skin_s")

        # high open collar
        fill_rect(img, cx - 4, head_y0 + 8, cx + 1, head_y0 + 12, "coat_l")
        px(img, cx - 3, head_y0 + 9, "cape")
        px(img, cx - 4, head_y0 + 7, "coat")

        # --- arm + sword ---
        arm_y = torso_y0 + 8 + arm_phase
        if sword_angle == "rest":
            # arm down-forward
            fill_rect(img, cx + 4, arm_y, cx + 7, arm_y + 10, "coat")
            fill_rect(img, cx + 5, arm_y + 8, cx + 7, arm_y + 12, "glove")
            # sword diagonal down-right from hand
            hx, hy = cx + 7, arm_y + 11
            self._sword(img, hx, hy, mode="rest")
            # scabbard on hip
            fill_rect(img, cx - 2, torso_y0 + 18, cx + 1, torso_y0 + 30, "metal_d")
            vline(img, cx - 1, torso_y0 + 18, torso_y0 + 30, "amber_d")
        elif sword_angle == "windup":
            fill_rect(img, cx + 2, arm_y - 4, cx + 6, arm_y + 4, "coat")
            fill_rect(img, cx + 4, arm_y - 6, cx + 7, arm_y - 2, "glove")
            self._sword(img, cx + 7, arm_y - 4, mode="windup")
            fill_rect(img, cx - 2, torso_y0 + 18, cx + 1, torso_y0 + 30, "metal_d")
        elif sword_angle == "active":
            fill_rect(img, cx + 3, arm_y, cx + 10, arm_y + 3, "coat")
            fill_rect(img, cx + 9, arm_y, cx + 11, arm_y + 3, "glove")
            self._sword(img, cx + 12, arm_y + 1, mode="active")
            fill_rect(img, cx - 2, torso_y0 + 18, cx + 1, torso_y0 + 30, "metal_d")
        elif sword_angle == "recover":
            fill_rect(img, cx + 4, arm_y + 2, cx + 8, arm_y + 10, "coat")
            fill_rect(img, cx + 6, arm_y + 9, cx + 8, arm_y + 12, "glove")
            self._sword(img, cx + 8, arm_y + 12, mode="recover")
            fill_rect(img, cx - 2, torso_y0 + 18, cx + 1, torso_y0 + 30, "metal_d")

        # rim outline on silhouette edges (sparse)
        self._rim_pass(img)
        return img

    def _sword(self, img: Image.Image, hx: int, hy: int, mode: str) -> None:
        # fixed blade length 18 px tip-to-guard for identity lock
        if mode == "rest":
            # down-right
            for i in range(18):
                px(img, hx + i // 2, hy + i // 2, "blade" if i > 1 else "amber")
                if i > 2:
                    px(img, hx + i // 2, hy + i // 2 - 1, "blade_e")
            px(img, hx, hy, "amber")
        elif mode == "windup":
            for i in range(18):
                px(img, hx - i // 3, hy - i // 2, "blade" if i > 1 else "amber")
                if i > 2:
                    px(img, hx - i // 3 + 1, hy - i // 2, "blade_e")
        elif mode == "active":
            for i in range(18):
                px(img, hx + i, hy - i // 6, "blade" if i > 1 else "amber")
                if i > 2:
                    px(img, hx + i, hy - i // 6 - 1, "blade_e")
        elif mode == "recover":
            for i in range(18):
                px(img, hx + i // 3, hy + i // 2, "blade" if i > 1 else "amber")

    def _rim_pass(self, img: Image.Image) -> None:
        # add rim on left silhouette where opaque meets empty
        pxmap = img.load()
        for y in range(img.height):
            for x in range(img.width):
                r, g, b, a = pxmap[x, y]
                if a == 0:
                    continue
                left = pxmap[x - 1, y][3] if x > 0 else 0
                if left == 0:
                    # soft rim only if not already bright metal/blade
                    if (r, g, b) not in (rgb("blade"), rgb("blade_e"), rgb("metal_l"), rgb("amber")):
                        px(img, x, y, "rim")


def save_frame(img: Image.Image, path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    img.save(path)
    nearest_up(img, 3).save(path.with_name(path.stem + "_3x.png"))


def make_sheet(frames: list[Image.Image], path: Path) -> None:
    w, h = frames[0].size
    sheet = Image.new("RGBA", (w * len(frames), h), (0, 0, 0, 0))
    for i, f in enumerate(frames):
        sheet.alpha_composite(f, (i * w, 0))
    sheet.save(path)
    nearest_up(sheet, 3).save(path.with_name(path.stem + "_3x.png"))


def make_gif(frames: list[Image.Image], path: Path, duration_ms: int) -> None:
    prev = []
    for f in frames:
        big = nearest_up(f, 4)
        bg = Image.new("RGBA", big.size, (16, 12, 22, 255))
        bg.alpha_composite(big)
        d = ImageDraw.Draw(bg)
        d.line([(PIVOT[0] * 4 - 6, PIVOT[1] * 4), (PIVOT[0] * 4 + 6, PIVOT[1] * 4)], fill=(255, 70, 70))
        prev.append(bg.convert("P", palette=Image.ADAPTIVE))
    path.parent.mkdir(parents=True, exist_ok=True)
    prev[0].save(path, save_all=True, append_images=prev[1:], duration=duration_ms, loop=0, disposal=2)


def build_idle(p: Protagonist) -> list[Image.Image]:
    # 12 frames @ 12 fps — breath + cape only
    frames = []
    for i in range(12):
        # sine-like via table, 0..1..0
        breath = [0, 0, 1, 1, 1, 0, 0, 0, -0, 0, 0, 0][i]
        # cape sway -1..1..-1
        cape = [0, 0, 1, 1, 1, 0, 0, -1, -1, -1, 0, 0][i]
        frames.append(p.draw(breath=breath, cape_sway=cape, leg_phase=0, sword_angle="rest"))
    return frames


def build_run(p: Protagonist) -> list[Image.Image]:
    frames = []
    for i in range(10):
        cape = 1 if i % 2 == 0 else -1
        arm = 0 if i % 2 == 0 else 1
        frames.append(
            p.draw(
                breath=0,
                cape_sway=cape,
                leg_phase=i + 1,
                arm_phase=arm,
                lean=1,
                sword_angle="rest",
            )
        )
    return frames


def build_attack(p: Protagonist) -> list[Image.Image]:
    # 7 frames visual; mechanical hitbox may start on frame 0 in game
    plan = [
        ("windup", 0, -1, 0),
        ("windup", 0, -1, 0),
        ("active", 0, 0, 1),
        ("active", 0, 0, 1),
        ("active", 0, 1, 1),
        ("recover", 0, 0, 0),
        ("recover", 0, 0, 0),
    ]
    frames = []
    for angle, breath, cape, lean in plan:
        frames.append(
            p.draw(
                breath=breath,
                cape_sway=cape,
                leg_phase=0,
                lean=lean,
                sword_angle=angle,
            )
        )
    return frames


def write_palette() -> None:
    out = ROOT / "paleta"
    out.mkdir(parents=True, exist_ok=True)
    strip = Image.new("RGB", (20 + 28 * len(PALETTE), 72), (18, 16, 26))
    d = ImageDraw.Draw(strip)
    d.text((8, 4), "D3 locked palette (constructed pixel)", fill=(230, 210, 170))
    lines = ["# D3 locked palette\n", "Source of truth for constructed frames.\n\n"]
    for i, (name, color) in enumerate(PALETTE.items()):
        x = 10 + i * 28
        d.rectangle([x, 28, x + 24, 64], fill=color)
        lines.append(f"- `{name}`: `#{color[0]:02x}{color[1]:02x}{color[2]:02x}`\n")
    strip.save(out / "palette_strip.png")
    (out / "palette.md").write_text("".join(lines))


def qc_frames(frames: list[Image.Image], label: str) -> dict:
    """Technical checks — not artistic proof."""
    report = {"label": label, "count": len(frames), "ok": True, "issues": []}
    if not frames:
        report["ok"] = False
        report["issues"].append("no frames")
        return report
    w0, h0 = frames[0].size
    heights = []
    for i, f in enumerate(frames):
        if f.size != (w0, h0):
            report["ok"] = False
            report["issues"].append(f"frame {i} size {f.size} != {w0}x{h0}")
        # magenta check
        pxmap = f.load()
        magenta = 0
        opaque = 0
        ys = []
        for y in range(f.height):
            for x in range(f.width):
                r, g, b, a = pxmap[x, y]
                if a > 10:
                    opaque += 1
                    ys.append(y)
                    if r > 165 and b > 165 and g < 155:
                        magenta += 1
        if magenta:
            report["ok"] = False
            report["issues"].append(f"frame {i} magenta-like {magenta}")
        if opaque == 0:
            report["ok"] = False
            report["issues"].append(f"frame {i} empty")
        if ys:
            heights.append(max(ys) - min(ys) + 1)
    if heights and max(heights) - min(heights) > 2:
        report["ok"] = False
        report["issues"].append(f"height span {min(heights)}..{max(heights)}")
    report["height_range"] = [min(heights), max(heights)] if heights else None
    report["cell"] = [w0, h0]
    report["pivot"] = list(PIVOT)
    return report


def overlay_equip(base: Image.Image, kind: str) -> Image.Image:
    img = base.copy()
    cx, sole = PIVOT
    if kind == "light":
        # lighter coat tint strip
        fill_rect(img, cx - 4, sole - 40, cx + 4, sole - 20, "coat_l")
    elif kind == "heavy":
        fill_rect(img, cx - 5, sole - 42, cx + 5, sole - 22, "metal_d")
        fill_rect(img, cx - 4, sole - 40, cx + 4, sole - 24, "metal")
        px(img, cx + 4, sole - 38, "metal_l")
    elif kind == "shield":
        fill_rect(img, cx - 10, sole - 36, cx - 5, sole - 22, "metal_d")
        outline_rect(img, cx - 10, sole - 36, cx - 5, sole - 22, "metal_l")
        px(img, cx - 8, sole - 30, "amber")
    elif kind == "alt_cape":
        fill_rect(img, cx - 12, sole - 48, cx - 4, sole - 4, "cape_s")
        fill_rect(img, cx - 11, sole - 46, cx - 5, sole - 6, "cape")
    return img


def build_spectral(base: Image.Image) -> list[Image.Image]:
    # study only: human, dissolve, spectral, return — not gameplay
    frames = []
    # human
    frames.append(base.copy())
    # dissolve: punch holes + cyan rim
    dissolve = base.copy()
    p = dissolve.load()
    for y in range(dissolve.height):
        for x in range(dissolve.width):
            r, g, b, a = p[x, y]
            if a and ((x + y) % 3 == 0):
                p[x, y] = (0, 0, 0, 0)
            elif a and ((x + 2 * y) % 5 == 0):
                p[x, y] = (140, 210, 230, 180)
    frames.append(dissolve)
    # spectral: recolor non-transparent toward cyan glass
    spectral = Image.new("RGBA", base.size, (0, 0, 0, 0))
    bp = base.load()
    sp = spectral.load()
    for y in range(base.height):
        for x in range(base.width):
            r, g, b, a = bp[x, y]
            if a == 0:
                continue
            # hollow eyes / glow body — not blue filter of same sprite only
            if (r, g, b) == rgb("skin") or (r, g, b) == rgb("skin_s"):
                sp[x, y] = (180, 230, 255, 120)
            elif (r, g, b) in (rgb("hair"), rgb("hair_l")):
                sp[x, y] = (120, 180, 220, 200)
            elif (r, g, b) in (rgb("cape"), rgb("cape_s"), rgb("cape_out")):
                sp[x, y] = (80, 40, 90, 140)
            else:
                sp[x, y] = (100, 170, 210, 160)
            if x > 0 and bp[x - 1, y][3] == 0:
                sp[x, y] = (200, 245, 255, 220)
    frames.append(spectral)
    frames.append(base.copy())
    return frames


def main() -> None:
    write_palette()
    reports = []
    for detail in ("B1", "B2"):
        p = Protagonist(detail)
        ref_dir = ROOT / "referencia" / ("B1_equilibrado" if detail == "B1" else "B2_detalhado")
        idle_pose = p.draw()
        save_frame(idle_pose, ref_dir / "side_idle.png")
        # front/back are schematic marks (side is authority)
        front = new_canvas()
        fill_rect(front, 24, 20, 40, 70, "coat")
        fill_rect(front, 26, 10, 38, 20, "skin")
        fill_rect(front, 25, 8, 39, 14, "hair")
        save_frame(front, ref_dir / "front_schematic.png")
        back = new_canvas()
        fill_rect(back, 22, 18, 42, 72, "cape_out")
        fill_rect(back, 24, 20, 40, 70, "cape")
        save_frame(back, ref_dir / "back_schematic.png")

        anim_root = ROOT / "animacoes"
        # IDLE first — gatekeeper
        idle = build_idle(p)
        idle_dir = anim_root / "idle" / detail
        for i, f in enumerate(idle):
            save_frame(f, idle_dir / f"f{i:02d}.png")
        make_sheet(idle, idle_dir / "sheet.png")
        make_gif(idle, ROOT / "previews" / f"idle_{detail}.gif", duration_ms=83)
        rep = qc_frames(idle, f"idle_{detail}")
        reports.append(rep)

        if not rep["ok"]:
            print(f"STOP: idle {detail} failed QC:", rep)
            continue

        run = build_run(p)
        run_dir = anim_root / "run" / detail
        for i, f in enumerate(run):
            save_frame(f, run_dir / f"f{i:02d}.png")
        make_sheet(run, run_dir / "sheet.png")
        make_gif(run, ROOT / "previews" / f"run_{detail}.gif", duration_ms=42)
        reports.append(qc_frames(run, f"run_{detail}"))

        atk = build_attack(p)
        atk_dir = anim_root / "attack" / detail
        for i, f in enumerate(atk):
            save_frame(f, atk_dir / f"f{i:02d}.png")
        make_sheet(atk, atk_dir / "sheet.png")
        make_gif(atk, ROOT / "previews" / f"attack_{detail}.gif", duration_ms=40)
        reports.append(qc_frames(atk, f"attack_{detail}"))

        # equipment POC from same base
        eq = ROOT / "equipamentos" / detail
        for kind in ("base", "light", "heavy", "shield", "alt_cape"):
            src = idle_pose if kind == "base" else overlay_equip(idle_pose, kind)
            save_frame(src, eq / f"{kind}.png")

        # spectral study
        spec_dir = ROOT / "espectral" / detail
        for i, f in enumerate(build_spectral(idle_pose)):
            save_frame(f, spec_dir / f"step{i}.png")

    (ROOT / "testes" / "qc_report.json").write_text(json.dumps(reports, indent=2))
    print(json.dumps(reports, indent=2))


if __name__ == "__main__":
    main()
