//! N4-1-A: visual equipment defined by data, on the DRESSED approved
//! protagonist (no nude body needed).
//!
//! - every family and item under `stage/equipment/` loads and validates
//!   with the real loader (no Rust code per item);
//! - without equipment the composition is still the approved idle and run
//!   frames texel for texel;
//! - item A (iron cuirass) OVERLAYs the torso and HIDEs the run shirt torso,
//!   which it fully covers; item B (blackened breastplate) reuses the same
//!   family silhouette, is shorter and therefore hides nothing;
//! - no texel of the dressed character becomes transparent (no hole) and no
//!   hidden texel is left uncovered (nothing under the shirt shows);
//! - both items follow the one `VisualPlayback` frame and facing;
//! - the save format and the approved art are unchanged.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use dark_castle_rpg::player::visual::VisualPlayback;
use dark_castle_rpg::player::visual::composite::{
    CompositeCharacter, CompositeLayer, sync_composite_layers,
};
use dark_castle_rpg::player::visual::composition::{
    CharacterComposition, Outfit, resolve, resolve_facing,
};
use dark_castle_rpg::player::visual::equipment_visual::{EquipmentVisual, VisualFamily, equip};
use dark_castle_rpg::save::serialization::CURRENT_SAVE_VERSION;

const H: usize = 128;
const A: &str = "iron_cuirass";
const B: &str = "blackened_breastplate";
/// (clip, frame width, frames).
const CLIPS: [(&str, usize, u32); 2] = [("idle", 80, 1), ("run_c1", 112, 3)];

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn stage() -> PathBuf {
    root().join("art-src/n4-sprite-machine/stage")
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn ron_files(dir: &str) -> Vec<String> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(stage().join(dir))
        .unwrap_or_else(|e| panic!("{dir}: {e}"))
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "ron"))
        .collect();
    paths.sort();
    paths.iter().map(|p| read(p)).collect()
}

fn families() -> Vec<VisualFamily> {
    ron_files("equipment/families")
        .iter()
        .map(|t| VisualFamily::from_ron(t).unwrap_or_else(|e| panic!("{e}")))
        .collect()
}

fn items() -> Vec<EquipmentVisual> {
    ron_files("equipment/items")
        .iter()
        .map(|t| EquipmentVisual::from_ron(t).unwrap_or_else(|e| panic!("{e}")))
        .collect()
}

fn base() -> CharacterComposition {
    CharacterComposition::from_ron(&read(&stage().join("protagonist.ron")))
        .unwrap_or_else(|e| panic!("{e}"))
}

fn equipped() -> CharacterComposition {
    equip(&base(), &families(), &items()).unwrap_or_else(|e| panic!("{e}"))
}

type Grid = Vec<Vec<char>>;

/// Frame `frame` of a sheet strip, from its `.txt` grid.
fn frame_grid(texture: &str, width: usize, frames: u32, frame: u32) -> Grid {
    let text = read(&stage().join(texture.replace(".png", ".txt")));
    let rows: Grid = text.lines().map(|l| l.chars().collect()).collect();
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

/// Top layer of every texel (layer slot name, or None when transparent).
fn compose(
    c: &CharacterComposition,
    outfit: &Outfit,
    clip: &str,
    width: usize,
    frames: u32,
    frame: u32,
) -> Vec<Vec<Option<(String, char)>>> {
    let mut out = vec![vec![None; width]; H];
    for d in resolve(c, outfit, clip, frame)
        .unwrap_or_else(|e| panic!("{e}"))
        .iter()
        .filter(|d| d.visible)
    {
        let g = frame_grid(&d.texture, width, frames, d.index);
        for (y, row) in g.iter().enumerate() {
            for (x, k) in row.iter().enumerate() {
                if *k != '.' {
                    out[y][x] = Some((d.layer.clone(), *k));
                }
            }
        }
    }
    out
}

fn opaque(g: &[Vec<Option<(String, char)>>]) -> BTreeSet<(usize, usize)> {
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

fn cells_of(g: &Grid) -> BTreeSet<(usize, usize)> {
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

#[test]
fn every_family_and_item_loads_from_data_alone() {
    let fams = families();
    let its = items();
    assert_eq!(
        fams.iter().map(|f| f.id.as_str()).collect::<Vec<_>>(),
        ["torso_plate"]
    );
    let ids: BTreeSet<&str> = its.iter().map(|i| i.id.as_str()).collect();
    assert_eq!(ids, BTreeSet::from([A, B]));
    assert!(its.iter().all(|i| i.family == "torso_plate"));
    let c = equipped();
    for id in [A, B] {
        let part = &c.parts[id];
        assert!(part.removable);
        assert_eq!(part.pieces.len(), 1);
        assert_eq!(part.pieces[0].layer, "armor_torso");
    }
    // A keeps the family HIDE of the run shirt torso, B drops it.
    assert_eq!(
        c.parts[A].hide_layers.get("run_c1").map(Vec::as_slice),
        Some(["shirt_torso".to_owned()].as_slice())
    );
    assert!(c.parts[B].hide_layers.is_empty());
}

#[test]
fn without_equipment_the_approved_frames_are_unchanged() {
    let c = equipped();
    let outfit = Outfit::default_for(&c);
    for (clip, width, frames) in CLIPS {
        for f in 0..frames {
            let g = compose(&c, &outfit, clip, width, frames, f);
            assert!(
                g.iter()
                    .flatten()
                    .flatten()
                    .all(|(l, _)| l != "armor_torso"),
                "{clip}#{f}: armour drawn without being worn"
            );
            let source = if clip == "idle" {
                "art-src/n3-5-fidelity/source/candidate_106.txt".to_owned()
            } else {
                format!(
                    "art-src/n3-6-animation-proof/source/video_{:03}.txt",
                    48 + 4 * f
                )
            };
            let approved: Grid = read(&root().join(source))
                .lines()
                .map(|l| l.chars().collect())
                .collect();
            for y in 0..H {
                for x in 0..width {
                    let k = g[y][x].as_ref().map_or('.', |(_, k)| *k);
                    assert_eq!(k, approved[y][x], "{clip}#{f} ({x},{y})");
                }
            }
        }
    }
}

#[test]
fn both_items_overlay_without_holes_or_exposed_texels() {
    let c = equipped();
    let dressed = Outfit::default_for(&c);
    for id in [A, B] {
        let mut worn = dressed.clone();
        worn.wear(id, None);
        for (clip, width, frames) in CLIPS {
            let sheet = &c.parts[id].pieces[0].sheets[clip];
            for f in 0..frames {
                let before = compose(&c, &dressed, clip, width, frames, f);
                let after = compose(&c, &worn, clip, width, frames, f);
                // No hole: every texel of the dressed character stays opaque.
                assert_eq!(opaque(&before), opaque(&after), "{id} {clip}#{f}: hole");
                // The item is drawn, and only on the dressed silhouette.
                let item = cells_of(&frame_grid(&sheet.texture, width, frames, f));
                assert!(!item.is_empty());
                assert!(item.is_subset(&opaque(&before)), "{id} {clip}#{f}: floats");
                let shown = after
                    .iter()
                    .flatten()
                    .flatten()
                    .filter(|(l, _)| l == "armor_torso")
                    .count();
                assert!(shown > 0, "{id} {clip}#{f}: item not visible");
                // HIDE never uncovers: every hidden texel is under the item.
                for layer in c.parts[id].hide_layers.get(clip).into_iter().flatten() {
                    let hidden_sheet = c
                        .parts
                        .values()
                        .flat_map(|p| &p.pieces)
                        .find(|p| &p.layer == layer)
                        .and_then(|p| p.sheets.get(clip))
                        .unwrap_or_else(|| panic!("{layer}"));
                    let hidden = cells_of(&frame_grid(&hidden_sheet.texture, width, frames, f));
                    assert!(
                        hidden.is_subset(&item),
                        "{id} {clip}#{f}: hides {layer} where it does not cover"
                    );
                    // And the hidden layer is not drawn.
                    assert!(after.iter().flatten().flatten().all(|(l, _)| l != layer));
                }
                // The shirt never shows through the plate: where the item is
                // and nothing above the plate slot covers it, the plate shows.
                let above = ["arm_front", "shirt", "bracers", "hair_back", "hair_front"];
                for &(x, y) in &item {
                    let top = after[y][x].as_ref().map(|(l, _)| l.as_str());
                    if clip == "run_c1" {
                        assert!(
                            top == Some("armor_torso") || top.is_some_and(|l| above.contains(&l)),
                            "{id} {clip}#{f} ({x},{y}): {top:?} over the plate"
                        );
                    } else {
                        assert!(
                            matches!(
                                top,
                                Some("armor_torso" | "bracers" | "hair_back" | "hair_front")
                            ),
                            "{id} idle ({x},{y}): {top:?} over the plate"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn item_b_reuses_the_family_silhouette_of_item_a() {
    let c = equipped();
    for (clip, width, frames) in CLIPS {
        let mask_texture = format!("equipment/families/torso_plate/{clip}_mask.png");
        for f in 0..frames {
            let mask = frame_grid(&mask_texture, width, frames, f);
            let family: BTreeSet<_> = cells_of(&mask);
            let a = cells_of(&frame_grid(
                &c.parts[A].pieces[0].sheets[clip].texture,
                width,
                frames,
                f,
            ));
            let b = cells_of(&frame_grid(
                &c.parts[B].pieces[0].sheets[clip].texture,
                width,
                frames,
                f,
            ));
            // Both inside the shared family silhouette (A also fills the
            // neckline with its gorget); B is the shorter cut of the same
            // geometry, with its own paint.
            assert!(
                b.is_subset(&family),
                "{clip}#{f}: B outside the family mask"
            );
            let neck: BTreeSet<_> = mask
                .iter()
                .enumerate()
                .flat_map(|(y, r)| {
                    r.iter()
                        .enumerate()
                        .filter(|(_, k)| **k == 'n')
                        .map(move |(x, _)| (x, y))
                })
                .collect();
            let family_and_neck: BTreeSet<_> = family.union(&neck).copied().collect();
            assert!(
                a.is_subset(&family_and_neck),
                "{clip}#{f}: A outside the family"
            );
            assert!(
                b.is_subset(&a),
                "{clip}#{f}: B is not a cut of A's silhouette"
            );
            assert!(b.len() < a.len());
        }
    }
}

#[test]
fn the_items_mirror_with_the_body_and_keep_their_frame() {
    let c = equipped();
    let mut o = Outfit::default_for(&c);
    o.wear(A, None);
    for flip in [false, true] {
        for frame in 0..3 {
            let layers =
                resolve_facing(&c, &o, "run_c1", frame, flip).unwrap_or_else(|e| panic!("{e}"));
            let plate = layers
                .iter()
                .find(|d| d.part == A)
                .unwrap_or_else(|| panic!("plate"));
            // No hand-drawn mirrored sheet: flipped with the body.
            assert!(!plate.mirrored);
            assert!(layers.iter().all(|d| d.index == frame));
        }
    }
}

#[test]
fn worn_items_follow_the_one_visual_playback() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(AssetPlugin::default())
        .init_asset::<Image>()
        .init_asset::<TextureAtlasLayout>()
        .add_systems(Update, sync_composite_layers);
    let c = Arc::new(equipped());
    let mut outfit = Outfit::default_for(&c);
    outfit.wear(A, None);
    let character = {
        let world = app.world_mut();
        let server = world.resource::<AssetServer>().clone();
        let mut layouts = world.resource_mut::<Assets<TextureAtlasLayout>>();
        CompositeCharacter::load(c.clone(), outfit, &server, &mut layouts, "")
    };
    let root = app
        .world_mut()
        .spawn((
            VisualPlayback {
                clip: Some("run_c1".into()),
                ..default()
            },
            Transform::default(),
            Visibility::default(),
        ))
        .id();
    {
        let world = app.world_mut();
        let mut commands = world.commands();
        character.spawn_layers(&mut commands, root);
        world.flush();
    }
    app.world_mut().entity_mut(root).insert(character);
    for (frame, flip, item) in [(0, false, A), (1, true, A), (2, false, B), (1, true, B)] {
        {
            let mut e = app.world_mut().entity_mut(root);
            let mut p = e.get_mut::<VisualPlayback>().unwrap_or_else(|| panic!("p"));
            p.frame = frame;
            p.flip = flip;
            let mut ch = e
                .get_mut::<CompositeCharacter>()
                .unwrap_or_else(|| panic!("ch"));
            let comp = ch.composition.clone();
            ch.outfit = Outfit::default_for(&comp);
            ch.outfit.wear(item, None);
        }
        app.update();
        let world = app.world_mut();
        let mut q = world.query::<(&CompositeLayer, &Sprite, &Visibility)>();
        let visible: Vec<(String, usize, bool)> = q
            .iter(world)
            .filter(|(_, _, v)| **v != Visibility::Hidden)
            .map(|(l, s, _)| {
                (
                    l.0.clone(),
                    s.texture_atlas.as_ref().map_or(99, |a| a.index),
                    s.flip_x,
                )
            })
            .collect();
        assert!(
            visible.iter().any(|(l, ..)| l == "armor_torso"),
            "{item}: plate drawn"
        );
        // A hides the run shirt torso; B does not.
        let shirt_torso = visible.iter().any(|(l, ..)| l == "shirt_torso");
        assert_eq!(shirt_torso, item == B, "{item}: shirt torso visibility");
        assert!(
            visible.iter().all(|(_, i, f)| *i == frame && *f == flip),
            "{item}: a layer out of step"
        );
    }
}

#[test]
fn a_third_item_of_the_family_is_data_only() {
    // A new item: only a manifest (here reusing B's sheets with another
    // palette name and A's HIDE). No Rust code knows its id.
    let third = EquipmentVisual::from_ron(
        r#"(
            version: 1,
            id: "bronze_breastplate_test",
            family: "torso_plate",
            sheets: {
                "idle": (texture: "equipment/art/blackened_breastplate/idle.png"),
                "run_c1": (texture: "equipment/art/blackened_breastplate/run_c1.png", columns: 3),
            },
            hide_layers: Some({}),
            palette: Some("bronze"),
        )"#,
    )
    .unwrap_or_else(|e| panic!("{e}"));
    let mut all = items();
    all.push(third);
    let c = equip(&base(), &families(), &all).unwrap_or_else(|e| panic!("{e}"));
    let mut o = Outfit::default_for(&c);
    o.wear("bronze_breastplate_test", None);
    let layers = resolve(&c, &o, "run_c1", 1).unwrap_or_else(|e| panic!("{e}"));
    assert!(
        layers
            .iter()
            .any(|d| d.part == "bronze_breastplate_test" && d.visible && d.layer == "armor_torso")
    );
    // A broken manifest is a descriptive error, not a silent fallback.
    let broken = EquipmentVisual::from_ron(
        r#"(version: 1, id: "x", family: "torso_plate", sheets: { "idle": (texture: "x.png") }, hide_layers: Some({ "idle": ["cape"] }))"#,
    )
    .unwrap_or_else(|e| panic!("{e}"));
    let err = equip(&base(), &families(), &[broken])
        .err()
        .map(|e| e.to_string())
        .unwrap_or_default();
    assert!(err.contains("item x") && err.contains("cape"), "{err}");
}

#[test]
fn the_save_format_and_the_approved_art_do_not_move() {
    assert_eq!(CURRENT_SAVE_VERSION, 3);
    let fnv = |bytes: &[u8]| {
        bytes.iter().fold(0xcbf29ce484222325_u64, |h, b| {
            (h ^ u64::from(*b)).wrapping_mul(0x100000001b3)
        })
    };
    for (path, hash) in [
        (
            "art-src/n3-6-animation-proof/generated/run6.png",
            0x0798ef4f59448a15_u64,
        ),
        (
            "art-src/n3-5-fidelity/generated/candidate_106.png",
            0x63a38af5ff4fd252,
        ),
    ] {
        let bytes = std::fs::read(root().join(path)).unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(fnv(&bytes), hash, "{path}");
    }
}
