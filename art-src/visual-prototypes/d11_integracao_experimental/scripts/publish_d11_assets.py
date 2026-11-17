#!/usr/bin/env python3
"""Publish D10 lab frames into assets/experimental/d11/ (reversible experimental)."""
from __future__ import annotations

import json
import shutil
from pathlib import Path

REPO = Path(__file__).resolve().parents[4]
D10 = REPO / "art-src" / "visual-prototypes" / "d10_finalizacao_modular"
OUT = REPO / "assets" / "experimental" / "d11"


def copy_frames(clip: str, loadout: str, count: int) -> list[str]:
    src_dir = D10 / "animacoes" / clip
    dst = OUT / "frames" / loadout / clip
    dst.mkdir(parents=True, exist_ok=True)
    files = []
    for i in range(count):
        name = f"{loadout}_f{i:02d}.png"
        src = src_dir / name
        if not src.exists():
            raise SystemExit(f"missing {src}")
        dest = dst / f"f{i:02d}.png"
        shutil.copy2(src, dest)
        files.append(f"experimental/d11/frames/{loadout}/{clip}/f{i:02d}.png")
    return files


def main() -> None:
    if not D10.exists():
        raise SystemExit("D10 lab missing — abort publish")
    OUT.mkdir(parents=True, exist_ok=True)
    classic_idle = copy_frames("idle", "classico", 6)
    classic_run = copy_frames("run", "classico", 8)
    heavy_idle = copy_frames("idle", "pesado", 6)
    heavy_run = copy_frames("run", "pesado", 8)
    # static poses
    for lo in ("classico", "pesado"):
        src = D10 / "loadouts" / lo / "pose.png"
        dst = OUT / "poses" / f"{lo}.png"
        dst.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(src, dst)
    # antifake proof
    shutil.copy2(D10 / "testes" / "antifake_report.json", OUT / "antifake_report.json")

    contract = {
        "version": 1,
        "id": "d11_modular_experimental",
        "cell_px": [128, 160],
        "pivot_sole_px": [64, 156],
        "game_height_px": 52,
        "world_height": 52.0,
        "filter": "nearest",
        "default_visual_untouched": True,
        "clips": {
            "classico": {
                "idle": {"frames": classic_idle, "fps": 10},
                "run": {"frames": classic_run, "fps": 12},
            },
            "pesado": {
                "idle": {"frames": heavy_idle, "fps": 10},
                "run": {"frames": heavy_run, "fps": 12},
            },
        },
        "poses": {
            "classico": "experimental/d11/poses/classico.png",
            "pesado": "experimental/d11/poses/pesado.png",
        },
        "equipment_visual_slots": [
            "weapon",
            "offhand",
            "head",
            "body",
            "gloves",
            "boots",
            "cape",
            "scabbard",
        ],
        "functional_slots_unchanged": [
            "Weapon",
            "Offhand",
            "Head",
            "Body",
            "Gloves",
            "Boots",
            "Accessory1",
            "Accessory2",
        ],
        "notes": [
            "Classic motion frames still use D2 as pose guide (D10 limitation).",
            "Do not treat as final approved art for replacing assets/animations/player.ron.",
        ],
    }
    (OUT / "contract.json").write_text(json.dumps(contract, indent=2), encoding="utf-8")
    print(f"Published experimental assets -> {OUT}")


if __name__ == "__main__":
    main()
