"""Skin weights of v005, computed analytically from the rest geometry.

Why not Blender's automatic (heat) weights: the v004 body is ~130 separate,
open and overlapping shells (cloth with Solidify, tubes, locks); heat
weighting fails or bleeds across them. Here every object follows an explicit
rule, so the weights are deterministic, explainable and testable:

- **rigid**: one bone, weight 1 (face pieces and hair -> head; gloves ->
  hands; sabre pieces -> ``sword``);
- **blend**: fixed mix for small rigid parts sitting on a joint (pauldron:
  upper arm + clavicle);
- **chain**: the vertex is projected on the polyline of a bone chain
  (arc-length position ``s``) and weighted between consecutive bones with a
  smoothstep window around each joint. Used for torso, neck, arms, legs and
  feet;
- **coat** / **cape**: chain weights combined with angular weights that
  pick the coat panel (front/back, right/left) or the cape chain.

Every vertex ends with at most ``MAX_INFLUENCES`` normalised weights. An
Armature modifier is inserted first in each modifier stack, before Solidify
and Subdivision, so the cloth is thickened and smoothed after it bends.
"""

from __future__ import annotations

import math
from collections.abc import Callable, Sequence

import bpy
from mathutils import Vector

import params as P
import rig_config as C
from mesh_tools import smoothstep

Weights = dict[str, float]
Rule = Callable[[Vector], Weights]


# --- chains -------------------------------------------------------------------


class Chain:
    """Bones along a polyline of joints; ``blend[i]`` = half window at joint i+1."""

    def __init__(self, joints: Sequence[Sequence[float]], bones: Sequence[str], blend: Sequence[float]):
        if len(joints) != len(bones) + 1 or len(blend) != len(bones) - 1:
            raise ValueError("a chain needs len(bones) + 1 joints and len(bones) - 1 blend widths")
        self.joints = [Vector(j) for j in joints]
        self.bones = list(bones)
        self.blend = list(blend)
        self.lengths = [(b - a).length for a, b in zip(self.joints, self.joints[1:])]
        self.starts = [sum(self.lengths[:i]) for i in range(len(self.lengths))]

    def position(self, p: Vector) -> float:
        """Arc length of the closest point of the polyline (extended at both ends)."""
        best_d, best_s = 1e9, 0.0
        last = len(self.lengths) - 1
        for i, (a, b) in enumerate(zip(self.joints, self.joints[1:])):
            ab = b - a
            t = (p - a).dot(ab) / ab.length_squared
            lo = -1e9 if i == 0 else 0.0
            hi = 1e9 if i == last else 1.0
            t_c = min(max(t, lo), hi)
            d = (p - (a + ab * t_c)).length
            if d < best_d:
                best_d, best_s = d, self.starts[i] + t_c * self.lengths[i]
        return best_s

    def weights(self, p: Vector) -> Weights:
        s = self.position(p)
        # f[k] = how far past joint k (k = 1..n-1) the point is; f[0] = 1, f[n] = 0.
        f = [1.0]
        for k, h in enumerate(self.blend, start=1):
            joint_s = self.starts[k]
            f.append(smoothstep((s - (joint_s - h)) / (2.0 * h)))
        f.append(0.0)
        return {bone: f[k] - f[k + 1] for k, bone in enumerate(self.bones) if f[k] - f[k + 1] > 0.0}


def _mix(*parts: tuple[float, Weights]) -> Weights:
    out: Weights = {}
    for factor, w in parts:
        if factor <= 0.0:
            continue
        for bone, value in w.items():
            out[bone] = out.get(bone, 0.0) + factor * value
    return out


def _bone(name: str) -> Vector:
    return bpy.data.objects[C.ARMATURE_NAME].data.bones[name].head_local.copy()


def _tail(name: str) -> Vector:
    return bpy.data.objects[C.ARMATURE_NAME].data.bones[name].tail_local.copy()


def build_chains() -> dict[str, Chain]:
    chains = {
        "spine": Chain([_bone("pelvis"), _bone("spine_01"), _bone("spine_02"), _bone("chest"), _tail("chest")],
                       ["pelvis", "spine_01", "spine_02", "chest"], [0.05, 0.05, 0.05]),
        "neck": Chain([Vector((0.0, 0.0, 1.38)), _bone("neck"), _bone("head"), _tail("head")],
                      ["chest", "neck", "head"], [0.025, 0.03]),
    }
    for side in ("R", "L"):
        chains[f"arm.{side}"] = Chain(
            [_bone(f"clavicle.{side}"), _bone(f"upper_arm.{side}"), _bone(f"forearm.{side}"),
             _bone(f"hand.{side}"), _tail(f"hand.{side}")],
            [f"clavicle.{side}", f"upper_arm.{side}", f"forearm.{side}", f"hand.{side}"],
            [0.04, 0.045, 0.02])
        hip = _bone(f"thigh.{side}")
        chains[f"leg.{side}"] = Chain(
            [Vector((hip.x, hip.y, 1.06)), hip, _bone(f"shin.{side}"), _bone(f"foot.{side}"), _tail(f"foot.{side}")],
            ["pelvis", f"thigh.{side}", f"shin.{side}", f"foot.{side}"], [0.06, 0.045, 0.03])
        ankle = _bone(f"foot.{side}")
        chains[f"foot.{side}"] = Chain(
            [ankle + Vector((0.0, 0.0, 0.1)), ankle, _bone(f"toe.{side}"), _tail(f"toe.{side}")],
            [f"shin.{side}", f"foot.{side}", f"toe.{side}"], [0.03, 0.022])
    return chains


# --- composite rules ------------------------------------------------------------


def _shoulder_share(p: Vector) -> list[tuple[float, Weights]]:
    """Coat/vest area over each shoulder that must follow clavicle and upper arm."""
    parts = []
    for side, shoulder in (("R", Vector(P.SHOULDER_NEAR)), ("L", Vector(P.SHOULDER_FAR))):
        d = (p - shoulder).length
        w = smoothstep((0.10 - d) / 0.07) * smoothstep((p.z - 1.33) / 0.06)
        if w > 0.0:
            parts.append((w, {f"clavicle.{side}": 0.55, f"upper_arm.{side}": 0.45}))
    return parts


def upper_body(chains: dict[str, Chain], p: Vector) -> Weights:
    shoulder = _shoulder_share(p)
    rest = 1.0 - min(sum(w for w, _ in shoulder) * 0.8, 0.8)
    return _mix((rest, chains["spine"].weights(p)), *((w * 0.8, ws) for w, ws in shoulder))


def trousers(chains: dict[str, Chain], p: Vector) -> Weights:
    side = "R" if p.y < 0.0 else "L"
    w_leg = 0.7 * smoothstep((0.99 - p.z) / 0.07) * smoothstep((abs(p.y) - 0.03) / 0.05)
    return _mix((1.0 - w_leg, chains["spine"].weights(p)), (w_leg, {f"thigh.{side}": 1.0}))


WAIST_BONE_Z = 1.06
COAT_MID_Z = 0.72


def coat(chains: dict[str, Chain], p: Vector) -> Weights:
    w_skirt = smoothstep((WAIST_BONE_Z - p.z) / 0.14)
    upper = upper_body(chains, p)
    s_r = 1.0 - smoothstep((p.y + 0.02) / 0.04)
    angle = abs(math.atan2(p.y, p.x))  # 0 front .. pi back
    w_back = smoothstep((angle - 0.9) / 1.3)
    low = smoothstep((COAT_MID_Z + 0.08 - p.z) / 0.16)
    skirt: Weights = {}
    for side, s in (("R", s_r), ("L", 1.0 - s_r)):
        for part, w in (("front", 1.0 - w_back), ("back", w_back)):
            skirt[f"coat_{part}.{side}_01"] = s * w * (1.0 - low)
            skirt[f"coat_{part}.{side}_02"] = s * w * low
    return _mix((1.0 - w_skirt, upper), (w_skirt, skirt))


CAPE_TOP_Z = 1.33


def cape(chains: dict[str, Chain], p: Vector) -> Weights:
    theta = math.atan2(p.y, p.x) % (2.0 * math.pi)  # 1.2 (far shoulder) .. 4.3 (near back)
    w_top = smoothstep((p.z - CAPE_TOP_Z) / 0.1)
    arm_side = 1.0 - smoothstep((theta - 1.9) / 0.5)
    top = {"clavicle.L": arm_side, "chest": 1.0 - arm_side}
    w_back = smoothstep((theta - 2.5) / 0.6)
    low_arm = smoothstep((1.27 - p.z) / 0.1)
    low_back = smoothstep((1.30 - p.z) / 0.1)
    lower = {
        "cape_arm.L_01": (1.0 - w_back) * (1.0 - low_arm), "cape_arm.L_02": (1.0 - w_back) * low_arm,
        "cape_back_01": w_back * (1.0 - low_back), "cape_back_02": w_back * low_back,
    }
    share = C.CAPE_TORSO_SHARE * w_back  # only the back part rests on the torso
    return _mix((w_top, top), ((1.0 - w_top) * (1.0 - share), lower), ((1.0 - w_top) * share, upper_body(chains, p)))


def collar(chains: dict[str, Chain], p: Vector) -> Weights:
    w_neck = 0.35 * smoothstep((p.z - 1.5) / 0.12)
    return _mix((1.0 - w_neck, upper_body(chains, p)), (w_neck, {"neck": 1.0}))


# --- object -> rule ---------------------------------------------------------------


def rule_for(name: str, chains: dict[str, Chain]) -> tuple[str, Rule]:
    """(rule kind, function) for a v004 object name."""
    def rigid(bone: str) -> tuple[str, Rule]:
        return ("rigid", lambda p: {bone: 1.0})

    def chain(key: str) -> tuple[str, Rule]:
        return ("chain", lambda p: chains[key].weights(p))

    head_parts = ("head", "eye_", "lid_", "brow_", "ear_", "mouth", "hair_")
    if name.startswith(head_parts):
        return rigid("head")
    if name.startswith("glove_near"):
        return rigid("hand.R")
    if name.startswith("glove_far"):
        return rigid("hand.L")
    if name.startswith(("blade", "sword_")):
        return rigid("sword")
    if name.startswith("pauldron"):
        return ("blend", lambda p: {"upper_arm.R": 0.55, "clavicle.R": 0.45})
    if name == "neck":
        return chain("neck")
    if name == "collar":
        return ("collar", lambda p: collar(chains, p))
    if name == "coat":
        return ("coat", lambda p: coat(chains, p))
    if name in ("half_cape", "cape_clasp"):
        return ("cape", lambda p: cape(chains, p))
    if name in ("cape_cord", "cape_stud"):
        return ("chain", lambda p: upper_body(chains, p))
    if name == "torso_trousers" or name == "trouser_fly":
        return ("trousers", lambda p: trousers(chains, p))
    if name.startswith(("torso_vest", "vest_", "belt")):
        return ("chain", lambda p: upper_body(chains, p))
    for side, tag in (("R", "near"), ("L", "far")):
        if name in (f"sleeve_{tag}", f"cuff_{tag}", f"cuff_band_{tag}"):
            return chain(f"arm.{side}")
        if name in (f"leg_{tag}", f"boot_shaft_{tag}", f"boot_seam_{tag}"):
            return chain(f"leg.{side}")
        if name in (f"boot_foot_{tag}", f"boot_sole_{tag}", f"boot_welt_{tag}", f"boot_heel_{tag}",
                    f"boot_toe_seam_{tag}"):
            return chain(f"foot.{side}")
    raise KeyError(f"no skinning rule for {name}")


PRESERVE_VOLUME = ("sleeve_", "cuff", "leg_", "boot_shaft", "neck")


def clean(weights: Weights) -> Weights:
    kept = sorted(((b, w) for b, w in weights.items() if w >= C.MIN_WEIGHT), key=lambda bw: -bw[1])
    kept = kept[: C.MAX_INFLUENCES]
    total = sum(w for _, w in kept)
    if total <= 0.0:
        raise ValueError("vertex without influence")
    return {b: w / total for b, w in kept}


def _armature_first(obj: bpy.types.Object, mod: bpy.types.Modifier) -> None:
    with bpy.context.temp_override(object=obj, active_object=obj):
        bpy.ops.object.modifier_move_to_index(modifier=mod.name, index=0)
    if obj.modifiers[0] != mod:
        raise RuntimeError(f"{obj.name}: armature modifier is not first")


def skin(objects: Sequence[bpy.types.Object], arm: bpy.types.Object) -> dict[str, str]:
    """Writes vertex groups and the Armature modifier; returns object -> rule kind."""
    chains = build_chains()
    kinds: dict[str, str] = {}
    for obj in objects:
        kind, rule = rule_for(obj.name, chains)
        kinds[obj.name] = kind
        groups: dict[str, bpy.types.VertexGroup] = {}
        for v in obj.data.vertices:
            for bone, w in clean(rule(v.co)).items():
                vg = groups.get(bone)
                if vg is None:
                    vg = groups[bone] = obj.vertex_groups.new(name=bone)
                vg.add([v.index], w, "REPLACE")
        mod = obj.modifiers.new("rig", "ARMATURE")
        mod.object = arm
        mod.use_vertex_groups = True
        mod.use_bone_envelopes = False
        mod.use_deform_preserve_volume = obj.name.startswith(PRESERVE_VOLUME)
        _armature_first(obj, mod)
    return kinds
