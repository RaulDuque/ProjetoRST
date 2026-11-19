"""N41-B factory tests: parsing, validation, determinism, seeds, relief,
palettes, sel-outline, 1:1 grid, details inside the mask, cache, overwrite
protection and the data-only extension path.

Run (project root):
    python3 -B -m unittest discover -s art-src/n4-sprite-machine/scripts/tests

Scratch output goes to target/n41b_tests/ (inside the project's target/).
"""

from __future__ import annotations

import copy
import json
import shutil
import sys
import unittest
from pathlib import Path

from PIL import Image

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent))

import factory as fx  # noqa: E402

SCRATCH = fx.ROOT / "target/n41b_tests"


def fresh(name: str) -> Path:
    d = SCRATCH / name
    if d.exists():
        shutil.rmtree(d)
    d.mkdir(parents=True)
    return d


def library_copy(name: str) -> Path:
    """A copy of the factory library + recipes to change data safely."""
    d = fresh(name) / "factory"
    shutil.copytree(fx.FACTORY, d)
    return d


def recipe(rid: str) -> dict:
    r = json.loads((fx.FACTORY / f"recipes/{rid}.json").read_text())
    r["_path"] = fx.FACTORY / f"recipes/{rid}.json"
    return r


class Validation(unittest.TestCase):
    def setUp(self) -> None:
        self.lib = fx.load_library()

    def test_library_and_catalogue_are_valid(self) -> None:
        recipes, failures = fx.load_recipes(self.lib)
        self.assertEqual(failures, [])
        self.assertEqual(len(recipes), 40)
        cats = {}
        for r in recipes:
            cats[r["category"]] = cats.get(r["category"], 0) + 1
        self.assertEqual(cats, {"chest_armor": 16, "sword": 12, "cape": 4, "shoulder": 8})
        self.assertEqual(len({r["id"] for r in recipes}), 40)
        self.assertEqual(len({r["generation"]["seed"] for r in recipes}), 40)

    def test_every_material_has_the_four_minimum_tones_and_no_black_outline(self) -> None:
        for mid, m in self.lib.materials.items():
            for t in fx.REQUIRED_TONES:
                self.assertIn(t, m["tones"], mid)
            self.assertNotEqual(m["tones"]["outline"], [0, 0, 0], mid)

    def test_invalid_recipes_are_refused_with_a_reason(self) -> None:
        good = recipe("segmented_cuirass_iron")
        cases = {
            "unknown template": lambda r: r.update(template="nope"),
            "unknown material": lambda r: r["materials"].update(main="mithril"),
            "missing slot": lambda r: r.update(materials={}),
            "extra slot": lambda r: r["materials"].update(cape="iron"),
            "unknown detail": lambda r: r["details"].append({"detail": "dragon", "material": "iron", "place": {"at": [0, 0]}}),
            "bad place": lambda r: r["details"].append({"detail": "rivet", "material": "iron", "place": {"edge": "top"}}),
            "bad seed": lambda r: r["generation"].update(seed=-1),
            "wrong profile": lambda r: r["generation"].update(animation_profile="cape_back"),
            "wrong category": lambda r: r.update(category="sword"),
            "bad id": lambda r: r.update(id="Bad Id"),
            "unknown key": lambda r: r.update(colour="red"),
            "bad schema": lambda r: r.update(schema=9),
            "bad mirroring": lambda r: r.update(mirroring="own"),
            "bad fallback": lambda r: r.update(fallback={"missing_clip": "stretch"}),
        }
        for name, mutate in cases.items():
            r = copy.deepcopy(good)
            mutate(r)
            with self.assertRaises(fx.FactoryError, msg=name):
                fx.validate_recipe(r, self.lib, name)

    def test_invalid_library_data_is_refused(self) -> None:
        with self.assertRaises(fx.FactoryError):
            fx.validate_material({"tones": {"outline": [0, 0, 0], "shadow": [1, 1, 1], "base": [2, 2, 2], "highlight": [3, 3, 3]}}, "black")
        with self.assertRaises(fx.FactoryError):
            fx.validate_material({"tones": {"outline": [9, 9, 9], "base": [2, 2, 2], "highlight": [3, 3, 3]}}, "3 tones")
        with self.assertRaises(fx.FactoryError):
            fx.validate_detail({"op": "stamp", "pattern": ["ab", "a"]}, "ragged")
        with self.assertRaises(fx.FactoryError):
            fx.validate_detail({"op": "spray"}, "op")
        with self.assertRaises(fx.FactoryError):
            fx.validate_relief([{"op": "bands", "from": 0.5}], "missing")


class Rendering(unittest.TestCase):
    def setUp(self) -> None:
        self.lib = fx.load_library()

    def test_same_recipe_same_pixels(self) -> None:
        for rid in ("segmented_cuirass_iron", "sword_flame_ring", "cape_long_crimson", "mantle_crimson_wolf_fur"):
            a = fx.render_item(recipe(rid), self.lib)
            b = fx.render_item(recipe(rid), self.lib)
            self.assertEqual(a["strips"].keys(), b["strips"].keys())
            for clip in a["strips"]:
                self.assertEqual(a["strips"][clip][0], b["strips"][clip][0], rid)
            self.assertEqual(a.get("isolated"), b.get("isolated"), rid)

    def test_the_seed_only_moves_seeded_details(self) -> None:
        worn = recipe("segmented_cuirass_iron")  # has wear_scratches
        other = copy.deepcopy(worn)
        other["generation"]["seed"] += 1
        a = fx.render_item(worn, self.lib)["preview"][0]
        b = fx.render_item(other, self.lib)["preview"][0]
        self.assertNotEqual(a, b)
        self.assertEqual(set(a), set(b), "same silhouette")
        clean = recipe("segmented_cuirass_bronze")  # no seeded detail
        other = copy.deepcopy(clean)
        other["generation"]["seed"] += 1
        self.assertEqual(fx.render_item(clean, self.lib)["preview"][0], fx.render_item(other, self.lib)["preview"][0])

    def test_relief_is_not_flat_and_uses_at_least_four_tones(self) -> None:
        for rid in ("segmented_cuirass_iron", "scale_hauberk_bronze", "pauldron_steel", "cape_long_royal_blue"):
            out = fx.render_item(recipe(rid), self.lib)
            for name, (shapes, _, _) in out["relief"].items():
                for s in shapes:
                    self.assertGreater(max(s.height.values()) - min(s.height.values()), 20, name)
            painted = out["preview"][0] if "preview" in out else out["isolated"][0]
            main = recipe(rid)["materials"]["main"]
            tones = {t for (m, t) in painted.values() if m == main}
            self.assertGreaterEqual(len(tones), 4, f"{rid}: {tones}")

    def test_selective_outline_uses_material_colours(self) -> None:
        out = fx.render_item(recipe("cape_long_crimson"), self.lib)
        painted = out["isolated"][0]
        cells = set(painted)
        edge = [p for p in cells if any(n not in cells for n in ((p[0] + 1, p[1]), (p[0] - 1, p[1]), (p[0], p[1] + 1), (p[0], p[1] - 1)))]
        tones = {painted[p][1] for p in edge}
        self.assertIn("outline", tones)
        for p in edge:
            mid, tone = painted[p]
            self.assertNotEqual(self.lib.materials[mid]["tones"][tone], [0, 0, 0])

    def test_details_stay_inside_the_mask_and_off_the_outline(self) -> None:
        r = recipe("brigandine_coat_iron")
        bare = copy.deepcopy(r)
        bare["details"] = []
        a = fx.render_item(r, self.lib)
        b = fx.render_item(bare, self.lib)
        for clip in a["strips"]:
            for fa, fb in zip(a["strips"][clip][0], b["strips"][clip][0]):
                self.assertEqual(set(fa), set(fb), "details never add texels")
                changed = {p for p in fa if fa[p] != fb[p]}
                inner = fx.interior(set(fb))
                self.assertTrue(changed <= inner, f"{clip}: a detail touched the outline")

    def test_mask_reuse_torso_items_stay_on_the_dressed_silhouette(self) -> None:
        for rid in ("ridged_breastplate_bronze", "brigandine_coat_hardened_leather", "scale_hauberk_iron"):
            out = fx.render_item(recipe(rid), self.lib)
            for clip, (frames, _, _) in out["strips"].items():
                for f, painted in enumerate(frames):
                    top = fx.dressed_top(clip, f)
                    self.assertTrue(set(painted) <= set(top), f"{rid} {clip}#{f}")


class Batch(unittest.TestCase):
    def test_batch_is_deterministic_cached_and_does_not_overwrite(self) -> None:
        out_a, out_b = fresh("batch_a"), fresh("batch_b")
        only = ["segmented_cuirass_iron", "sword_estoc_cross", "pauldron_bronze"]
        ra = fx.build(out_a, only=only)
        rb = fx.build(out_b, only=only)
        self.assertEqual(ra["failures"], [])
        files = sorted(p.relative_to(out_a) for p in out_a.rglob("*") if p.is_file())
        self.assertTrue(files)
        for p in files:
            self.assertEqual((out_a / p).read_bytes(), (out_b / p).read_bytes(), str(p))
        # Cache: same inputs, nothing regenerated.
        again = fx.build(out_a, only=only)
        self.assertEqual({v["status"] for v in again["items"].values()}, {"cached"})
        # PNG on whole texels: binary alpha, RGBA.
        im = Image.open(out_a / "isolated/sword_estoc_cross.png")
        self.assertEqual(im.mode, "RGBA")
        self.assertTrue(set(im.getchannel("A").get_flattened_data()) <= {0, 255})

    def test_changed_recipe_refuses_to_overwrite_without_rebuild(self) -> None:
        lib_dir = library_copy("overwrite")
        out = fresh("overwrite_out")
        rid = "pauldron_bronze"
        self.assertEqual(fx.build(out, only=[rid], factory_root=lib_dir)["failures"], [])
        p = lib_dir / f"recipes/{rid}.json"
        r = json.loads(p.read_text())
        r["materials"]["main"] = "gold"
        p.write_text(json.dumps(r))
        rep = fx.build(out, only=[rid], factory_root=lib_dir)
        self.assertEqual(len(rep["failures"]), 1)
        self.assertIn("refusing to overwrite", rep["failures"][0]["error"])
        rep = fx.build(out, rebuild=True, only=[rid], factory_root=lib_dir)
        self.assertEqual(rep["failures"], [])
        self.assertEqual(rep["items"][rid]["status"], "generated")

    def test_a_new_item_material_and_detail_need_only_data(self) -> None:
        lib_dir = library_copy("data_only")
        (lib_dir / "materials/test_copper.json").write_text(json.dumps({
            "schema": 1, "id": "test_copper", "kind": "metal",
            "tones": {"outline": [40, 20, 14], "deep": [70, 34, 20], "shadow": [104, 52, 30], "base": [146, 80, 46],
                      "light": [186, 116, 70], "highlight": [222, 160, 112]},
            "gloss": 0.5, "roughness": 0.2, "contrast": 1.0, "wear": 0.0}))
        (lib_dir / "details/test_tower.json").write_text(json.dumps({
            "schema": 1, "id": "test_tower", "op": "stamp", "pattern": ["h.h", "hlh", "bsb"], "on_overflow": "skip"}))
        (lib_dir / "recipes/test_new_item.json").write_text(json.dumps({
            "schema": 1, "id": "test_new_item", "category": "chest_armor", "family": "torso_plate", "name": "t",
            "template": "segmented_cuirass", "materials": {"main": "test_copper"},
            "details": [{"detail": "test_tower", "material": "gold", "place": {"at": [0.6, 0.3]}}],
            "generation": {"seed": 5, "animation_profile": "humanoid_torso"}, "covers": ["Chest"], "hides": "auto",
            "mirroring": "flip", "fallback": {"missing_clip": "hide"}}))
        out = fresh("data_only_out")
        rep = fx.build(out, only=["test_new_item"], factory_root=lib_dir)
        self.assertEqual(rep["failures"], [])
        self.assertTrue((out / "items/test_new_item.ron").exists())
        self.assertIn('family: "torso_plate"', (out / "items/test_new_item.ron").read_text())


if __name__ == "__main__":
    unittest.main()
