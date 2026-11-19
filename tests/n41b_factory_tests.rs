//! N41-B: the data-driven equipment factory, integration side.
//!
//! The factory (`art-src/n4-sprite-machine/scripts/factory.py`, offline)
//! writes torso items as ordinary RON manifests of the existing
//! `torso_plate` family under `stage/factory/items/`. Here, with the real
//! loader and compositor rules (no Rust code per item):
//!
//! - every generated item loads next to the N4-1-A items, in the family slot;
//! - every item covers only the dressed silhouette, leaves no hole, and
//!   hides a base layer only where it fully covers it;
//! - the run C1 is limited to one family (scope of N41-B);
//! - generated items follow the one `VisualPlayback` frame and facing;
//! - swords, capes and shoulder pieces stay isolated (no game manifest);
//! - the N4-1-A data is untouched (still exactly items A and B).

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use dark_castle_rpg::player::visual::composition::{
    CharacterComposition, Outfit, resolve, resolve_facing,
};
use dark_castle_rpg::player::visual::equipment_visual::{EquipmentVisual, VisualFamily, equip};

const H: usize = 128;
/// (clip, frame width, frames).
const CLIPS: [(&str, usize, u32); 2] = [("idle", 80, 1), ("run_c1", 112, 3)];
/// Layer slots drawn above the plate slot in the run.
const ABOVE_RUN: [&str; 5] = ["arm_front", "shirt", "bracers", "hair_back", "hair_front"];

fn stage() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("art-src/n4-sprite-machine/stage")
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn files(dir: &str, ext: &str) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(stage().join(dir))
        .unwrap_or_else(|e| panic!("{dir}: {e}"))
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == ext))
        .collect();
    paths.sort();
    paths
}

fn items(dir: &str) -> Vec<EquipmentVisual> {
    files(dir, "ron")
        .iter()
        .map(|p| EquipmentVisual::from_ron(&read(p)).unwrap_or_else(|e| panic!("{e}")))
        .collect()
}

fn families() -> Vec<VisualFamily> {
    files("equipment/families", "ron")
        .iter()
        .map(|p| VisualFamily::from_ron(&read(p)).unwrap_or_else(|e| panic!("{e}")))
        .collect()
}

fn base() -> CharacterComposition {
    CharacterComposition::from_ron(&read(&stage().join("protagonist.ron")))
        .unwrap_or_else(|e| panic!("{e}"))
}

/// Recipe ids of the factory catalogue, split into torso armour (game
/// manifests) and the other categories (isolated assets).
fn catalogue() -> (BTreeSet<String>, BTreeSet<String>) {
    let dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("art-src/n4-sprite-machine/factory/recipes");
    let mut torso = BTreeSet::new();
    let mut other = BTreeSet::new();
    for entry in std::fs::read_dir(&dir).unwrap_or_else(|e| panic!("{e}")) {
        let path = entry.unwrap_or_else(|e| panic!("{e}")).path();
        let Some(id) = path.file_stem().map(|s| s.to_string_lossy().into_owned()) else {
            continue;
        };
        if read(&path).contains("\"category\": \"chest_armor\"") {
            torso.insert(id);
        } else {
            other.insert(id);
        }
    }
    (torso, other)
}

fn factory() -> Vec<EquipmentVisual> {
    items("factory/items")
}

fn equipped() -> CharacterComposition {
    let mut all = items("equipment/items");
    all.extend(factory());
    equip(&base(), &families(), &all).unwrap_or_else(|e| panic!("{e}"))
}

type Grid = Vec<Vec<char>>;

fn frame_grid(texture: &str, width: usize, frames: u32, frame: u32) -> Grid {
    let rows: Grid = read(&stage().join(texture.replace(".png", ".txt")))
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    assert_eq!(rows.len(), H, "{texture}");
    assert!(
        rows.iter().all(|r| r.len() == width * frames as usize),
        "{texture}"
    );
    let f = frame as usize;
    rows.iter()
        .map(|r| r[f * width..(f + 1) * width].to_vec())
        .collect()
}

fn cells(g: &Grid) -> BTreeSet<(usize, usize)> {
    let mut s = BTreeSet::new();
    for (y, row) in g.iter().enumerate() {
        for (x, k) in row.iter().enumerate() {
            if *k != '.' {
                s.insert((x, y));
            }
        }
    }
    s
}

/// Top layer slot of every texel.
fn compose(
    c: &CharacterComposition,
    outfit: &Outfit,
    clip: &str,
    width: usize,
    frames: u32,
    frame: u32,
) -> Vec<Vec<Option<String>>> {
    let mut out = vec![vec![None; width]; H];
    for d in resolve(c, outfit, clip, frame)
        .unwrap_or_else(|e| panic!("{e}"))
        .iter()
        .filter(|d| d.visible)
    {
        for (x, y) in cells(&frame_grid(&d.texture, width, frames, d.index)) {
            out[y][x] = Some(d.layer.clone());
        }
    }
    out
}

fn opaque(g: &[Vec<Option<String>>]) -> BTreeSet<(usize, usize)> {
    let mut s = BTreeSet::new();
    for (y, row) in g.iter().enumerate() {
        for (x, t) in row.iter().enumerate() {
            if t.is_some() {
                s.insert((x, y));
            }
        }
    }
    s
}

#[test]
fn generated_items_load_with_the_real_loader_in_the_family_slot() {
    let generated = factory();
    let (torso, _) = catalogue();
    // At least the 4 torso templates x 4 materials; one manifest per recipe.
    assert!(torso.len() >= 16);
    let ids: BTreeSet<String> = generated.iter().map(|i| i.id.clone()).collect();
    assert_eq!(ids.len(), generated.len(), "unique ids");
    assert_eq!(ids, torso, "one manifest per torso recipe, nothing else");
    let c = equipped();
    for item in &generated {
        assert_eq!(item.family, "torso_plate");
        let part = &c.parts[&item.id];
        assert!(part.removable);
        assert_eq!(part.pieces.len(), 1);
        assert_eq!(part.pieces[0].layer, "armor_torso");
    }
    // Not worn by default: the dressed character stays the approved one.
    let o = Outfit::default_for(&c);
    for (clip, _, frames) in CLIPS {
        for f in 0..frames {
            let shown = resolve(&c, &o, clip, f).unwrap_or_else(|e| panic!("{e}"));
            assert!(shown.iter().all(|d| d.layer != "armor_torso"));
        }
    }
}

#[test]
fn every_generated_item_overlays_without_holes_floating_texels_or_unsafe_hide() {
    let c = equipped();
    let dressed = Outfit::default_for(&c);
    for item in factory() {
        let mut worn = dressed.clone();
        worn.wear(&item.id, None);
        for (clip, width, frames) in CLIPS {
            let Some(sheet) = c.parts[&item.id].pieces[0].sheets.get(clip) else {
                continue;
            };
            for f in 0..frames {
                let before = compose(&c, &dressed, clip, width, frames, f);
                let after = compose(&c, &worn, clip, width, frames, f);
                let id = &item.id;
                assert_eq!(opaque(&before), opaque(&after), "{id} {clip}#{f}: hole");
                let texels = cells(&frame_grid(&sheet.texture, width, frames, f));
                assert!(!texels.is_empty(), "{id} {clip}#{f}: empty");
                assert!(
                    texels.is_subset(&opaque(&before)),
                    "{id} {clip}#{f}: floats outside the dressed silhouette"
                );
                assert!(
                    after.iter().flatten().flatten().any(|l| l == "armor_torso"),
                    "{id} {clip}#{f}: not visible"
                );
                for layer in c.parts[id].hide_layers.get(clip).into_iter().flatten() {
                    let hidden = c
                        .parts
                        .values()
                        .flat_map(|p| &p.pieces)
                        .find(|p| &p.layer == layer)
                        .and_then(|p| p.sheets.get(clip))
                        .unwrap_or_else(|| panic!("{layer}"));
                    let hidden = cells(&frame_grid(&hidden.texture, width, frames, f));
                    assert!(
                        hidden.is_subset(&texels),
                        "{id} {clip}#{f}: hides {layer} where it does not cover"
                    );
                }
                if clip == "run_c1" {
                    // Occlusion by layer order: where the plate is, only the
                    // near arm, its sleeve, the bracers or the hair cover it.
                    for &(x, y) in &texels {
                        let top = after[y][x].as_deref();
                        assert!(
                            top == Some("armor_torso")
                                || top.is_some_and(|l| ABOVE_RUN.contains(&l)),
                            "{id} run ({x},{y}): {top:?} over the plate"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn the_run_c1_is_limited_to_one_family_of_generated_armour() {
    let generated = factory();
    let run: Vec<&str> = generated
        .iter()
        .filter(|i| i.sheets.contains_key("run_c1"))
        .map(|i| i.id.as_str())
        .collect();
    // One torso template (four materials) in the run; every item in the idle.
    assert_eq!(run.len(), 4, "{run:?}");
    assert!(generated.iter().all(|i| i.sheets.contains_key("idle")));
    // The run family covers the whole run shirt torso, so it keeps the
    // family HIDE; idle-only items never hide anything.
    let c = equipped();
    for i in &generated {
        let hides = &c.parts[&i.id].hide_layers;
        if run.contains(&i.id.as_str()) {
            assert_eq!(
                hides.get("run_c1").map(Vec::as_slice),
                Some(["shirt_torso".to_owned()].as_slice()),
                "{}",
                i.id
            );
        } else {
            assert!(hides.is_empty(), "{}", i.id);
        }
    }
}

#[test]
fn generated_items_follow_the_one_playback_frame_and_facing() {
    let c = equipped();
    for item in factory().iter().filter(|i| i.sheets.contains_key("run_c1")) {
        let mut o = Outfit::default_for(&c);
        o.wear(&item.id, None);
        for flip in [false, true] {
            for frame in 0..3 {
                let layers =
                    resolve_facing(&c, &o, "run_c1", frame, flip).unwrap_or_else(|e| panic!("{e}"));
                let plate = layers
                    .iter()
                    .find(|d| d.part == item.id)
                    .unwrap_or_else(|| panic!("{}", item.id));
                assert!(!plate.mirrored, "flipped with the body, no own sheet");
                assert!(layers.iter().all(|d| d.index == frame));
            }
        }
    }
}

#[test]
fn isolated_categories_have_no_game_manifest_and_n41a_is_untouched() {
    let manifests: BTreeSet<String> = factory().into_iter().map(|i| i.id).collect();
    let isolated: Vec<String> = files("factory/isolated", "png")
        .iter()
        .filter_map(|p| p.file_stem().map(|s| s.to_string_lossy().into_owned()))
        .collect();
    // Swords, capes and shoulder pieces (at least 12 + 4 + 8), none with a
    // game manifest (no validated hand socket, no shoulder or back slot yet).
    let (_, other) = catalogue();
    assert!(other.len() >= 24);
    assert_eq!(isolated.iter().cloned().collect::<BTreeSet<_>>(), other);
    assert!(isolated.iter().all(|id| !manifests.contains(id)));
    let n41a: BTreeSet<String> = items("equipment/items").into_iter().map(|i| i.id).collect();
    assert_eq!(
        n41a,
        BTreeSet::from([
            "blackened_breastplate".to_owned(),
            "iron_cuirass".to_owned()
        ])
    );
    assert_eq!(
        families().iter().map(|f| f.id.as_str()).collect::<Vec<_>>(),
        ["torso_plate"],
        "the factory reuses the family; it adds none"
    );
}
