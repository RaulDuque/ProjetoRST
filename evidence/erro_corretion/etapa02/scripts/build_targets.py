"""Etapa 02 (Erro-Corretion): landmarks of the four approved poses, from EXISTING data only.

- C1 #0/#2/#4: art-src/n4-sprite-machine/scripts/run_poses.py (hand-marked skeletons, N4-1).
- idle: there is no idle skeleton file. Landmarks are derived mechanically from the
  anatomy tables of art-src/n4-sprite-machine/scripts/reconstruct.py and the region
  boxes of partition.py (both N4-A). Each rule is recorded in "derivation".

Coordinates are texel indices of the approved canvas (x right, y down). Read only:
nothing in art-src is written.

    python3 -I evidence/erro_corretion/etapa02/scripts/build_targets.py
"""

from __future__ import annotations

import importlib.util
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
SCRIPTS = ROOT / "art-src/n4-sprite-machine/scripts"
OUT = ROOT / "evidence/erro_corretion/etapa02/out/targets.json"


def load(name: str):
    spec = importlib.util.spec_from_file_location(name, SCRIPTS / f"{name}.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def centre(table: dict[int, tuple[int, int]], rows) -> tuple[float, float]:
    rows = [r for r in rows if r in table]
    xs = [(table[r][0] + table[r][1]) / 2 for r in rows]
    return (sum(xs) / len(xs), sum(rows) / len(rows))


def idle_targets() -> dict:
    rc = load("reconstruct")
    pt = load("partition")
    head = centre(rc.SKULL, range(15, 27))
    neck = centre(rc.NECK, range(27, 31))
    shoulder = centre(rc.UPPER_ARM, range(31, 36))
    elbow = centre(rc.UPPER_ARM, [51])
    wrist = centre(rc.WRIST, range(59, 64))
    hand_rows, hand_cols = pt.HAND
    fist = ((hand_cols[0] + hand_cols[-1]) / 2, (hand_rows[0] + hand_rows[-1]) / 2)
    hip = centre(rc.LEGS, [61])
    knee = centre(rc.LEGS, range(80, 85))
    ankle = centre(rc.LEGS, [112])
    toe = (max(v[1] for v in rc.FEET.values()), 117.0)
    arm = [shoulder, elbow, wrist, fist]
    leg = [hip, knee, ankle, toe]
    return {
        "canvas": [80, 128], "pivot": [40, 118],
        "sprite": "art-src/n3-5-fidelity/generated/candidate_106.png",
        "head": head, "neck": neck, "hip": hip, "belt_y": pt.TROUSERS_FROM_ROW,
        "near_arm": arm, "near_leg": leg,
        # Profile idle: the far limbs are behind the near ones and not discernible.
        "far_arm": arm, "far_leg": leg, "far_hidden": True,
        "derivation": {
            "head": "centre of reconstruct.SKULL rows 15..26",
            "neck": "centre of reconstruct.NECK rows 27..30",
            "shoulder": "centre of reconstruct.UPPER_ARM rows 31..35",
            "elbow": "last row (51) of reconstruct.UPPER_ARM, above the visible forearm",
            "wrist": "centre of reconstruct.WRIST rows 59..63 (under the bracer)",
            "fist": "centre of partition.HAND box",
            "hip": "first row (61) of reconstruct.LEGS (legs separate from the hips)",
            "knee": "reconstruct.LEGS rows 80..84 ('knee at rows 80..84')",
            "ankle": "last shin row (112) of reconstruct.LEGS",
            "toe": "front of reconstruct.FEET, row 117",
            "waist": "partition.TROUSERS_FROM_ROW (belt)",
            "far limbs": "hidden in profile: same targets as the near limbs (not measured)",
        },
    }


def run_targets() -> dict:
    rp = load("run_poses")
    out = {}
    for index, frame in rp.FRAMES.items():
        out[f"run_c1_{index}"] = {
            "canvas": [112, 128], "pivot": [56, 118], "frame": index,
            "sprite": "art-src/n3-6-animation-proof/generated/run6.png",
            "sprite_cell": index,
            "head": frame["head"], "neck": frame["neck"], "hip": frame["hip"],
            "belt_y": frame["belt_y"],
            "near_arm": frame["near_arm"], "far_arm": frame["far_arm"],
            "near_leg": frame["near_leg"], "far_leg": frame["far_leg"],
            "far_hidden": False,
            "derivation": {"all": f"run_poses.FRAMES[{index}] ({frame['source']})"},
        }
    return out


def main() -> None:
    OUT.parent.mkdir(parents=True, exist_ok=True)
    data = {"idle": idle_targets(), **run_targets()}
    OUT.write_text(json.dumps(data, indent=1), encoding="utf-8")
    for name, pose in data.items():
        print(name, "head", pose["head"], "hip", pose["hip"], "near_arm", pose["near_arm"])


if __name__ == "__main__":
    main()
