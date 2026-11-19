//! N4-A: the data-driven sprite machine, proved on the N3.5 idle.
//!
//! - the composition RON of the proof loads and validates (layers, canvas,
//!   pivot, parts, base body);
//! - recomposing the layer grids in the RON draw order gives the approved
//!   N3.5 idle **texel for texel**;
//! - every piece is independent: removing it changes only its own texels
//!   and leaves no transparent hole inside the body;
//! - the approved art and the older manifests (V2.2, N3.8) are unchanged and
//!   still load; the save version is still 3;
//! - in a headless app, every composite layer follows the one
//!   `VisualPlayback` (clip, frame, facing), and the player animation fills
//!   the shared frame and facing.
//!
//! Nothing here writes `assets/` or `saves/`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use bevy::asset::AssetPlugin;
use bevy::input::{InputPlugin as BevyInputPlugin, InputSystems};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::state::app::StatesPlugin;
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::app::plugins::GamePlugin;
use dark_castle_rpg::core::resources::GameData;
use dark_castle_rpg::player::visual::art_profile::N3_EXPERIMENTAL_STAGE;
use dark_castle_rpg::player::visual::composite::{
    CompositeCharacter, CompositeLayer, sync_composite_layers,
};
use dark_castle_rpg::player::visual::composition::{CharacterComposition, Outfit, resolve};
use dark_castle_rpg::player::visual::manifest::{AnimationManifest, anchor_for};
use dark_castle_rpg::player::visual::{
    MANIFEST_PATH, PlayerArtSource, PlayerSpriteSet, PlayerVisual, VisualPlayback,
};
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::save::serialization::CURRENT_SAVE_VERSION;
use dark_castle_rpg::save::systems::SaveSlots;

const W: usize = 80;
const H: usize = 128;
/// Keys of `art-src/n3-4-original-protagonist/scripts/n34_palette.py` plus `.`.
const PALETTE_KEYS: &str = ".KWwhHSszeGgqPpoLldM";
/// FNV-1a 64 of the approved N3.5 idle, recorded in N3.7 (`n3_7_sync_tests.rs`).
const APPROVED: [(&str, u64); 2] = [
    (
        "art-src/n3-5-fidelity/generated/candidate_106.png",
        0x63a38af5ff4fd252,
    ),
    (
        "art-src/n3-5-fidelity/source/candidate_106.txt",
        0xa241c1d3ec6b4974,
    ),
];

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn stage() -> PathBuf {
    root().join("art-src/n4-sprite-machine/stage")
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn composition() -> CharacterComposition {
    CharacterComposition::from_ron(&read(&stage().join("protagonist.ron")))
        .unwrap_or_else(|e| panic!("{e}"))
}

fn fnv1a(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
    })
}

fn png_size(path: &Path) -> (u32, u32) {
    let b = std::fs::read(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    assert!(b.starts_with(b"\x89PNG\r\n\x1a\n"), "{}", path.display());
    let word = |at: usize| u32::from_be_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]]);
    (word(16), word(20))
}

type Grid = Vec<Vec<char>>;

fn grid(path: &Path) -> Grid {
    let rows: Grid = read(path).lines().map(|l| l.chars().collect()).collect();
    assert_eq!(rows.len(), H, "{}", path.display());
    assert!(rows.iter().all(|r| r.len() == W), "{}", path.display());
    rows
}

fn original() -> Grid {
    grid(&root().join("art-src/n3-5-fidelity/source/candidate_106.txt"))
}

/// Grid of a layer texture (`parts/idle/x.png` -> `parts/idle/x.txt`).
fn layer_grid(texture: &str) -> Grid {
    grid(&stage().join(texture.replace(".png", ".txt")))
}

/// Draws the resolved idle layers back to front.
fn compose(c: &CharacterComposition, outfit: &Outfit) -> Grid {
    let mut out = vec![vec!['.'; W]; H];
    let layers = resolve(c, outfit, "idle", 0).unwrap_or_else(|e| panic!("{e}"));
    for d in layers.iter().filter(|d| d.visible) {
        let g = layer_grid(&d.texture);
        for (y, row) in g.iter().enumerate() {
            for (x, k) in row.iter().enumerate() {
                if *k != '.' {
                    out[y][x] = *k;
                }
            }
        }
    }
    out
}

fn differences(a: &Grid, b: &Grid) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    for y in 0..H {
        for x in 0..W {
            if a[y][x] != b[y][x] {
                out.push((x, y));
            }
        }
    }
    out
}

// --- data -------------------------------------------------------------------

#[test]
fn the_proof_composition_loads_with_the_n35_canvas_and_pivot() {
    let c = composition();
    let idle = &c.clips["idle"];
    assert_eq!(idle.cell, (80, 128));
    assert_eq!(idle.pivot, (40.0, 118.0));
    assert_eq!(idle.density, 2);
    assert_eq!(idle.frames, 1);
    assert_eq!(c.base_part(), Some("body"));
    assert_eq!(
        c.layers,
        [
            "body",
            "trousers",
            "boots",
            "arm_front",
            "shirt",
            "bracers",
            "hair_back",
            "hair_front"
        ]
    );
    let removable: BTreeSet<&str> = c
        .parts
        .iter()
        .filter(|(_, p)| p.removable)
        .map(|(id, _)| id.as_str())
        .collect();
    assert_eq!(
        removable,
        BTreeSet::from(["boots", "bracers", "hair", "shirt", "trousers"])
    );
    // Every layer texture: same canvas as the clip, palette keys only.
    for part in c.parts.values() {
        for piece in &part.pieces {
            let sheet = &piece.sheets["idle"];
            assert_eq!(png_size(&stage().join(&sheet.texture)), (80, 128));
            let g = layer_grid(&sheet.texture);
            assert!(
                g.iter().flatten().all(|k| PALETTE_KEYS.contains(*k)),
                "{}",
                sheet.texture
            );
        }
    }
}

#[test]
fn the_layers_recompose_the_approved_idle_texel_for_texel() {
    let c = composition();
    let full = compose(&c, &Outfit::default_for(&c));
    let diff = differences(&full, &original());
    assert!(diff.is_empty(), "{} texels differ: {diff:?}", diff.len());
}

#[test]
fn each_piece_is_independent_and_leaves_no_hole() {
    let c = composition();
    let outfit = Outfit::default_for(&c);
    let full = compose(&c, &outfit);
    let mut base = Outfit::default_for(&c);
    for id in c.default_outfit.clone() {
        base.remove(&c, &id);
    }
    let body = compose(&c, &base);
    // Row extent of the base body: inside it, a transparent texel is a hole.
    let extent: BTreeMap<usize, (usize, usize)> = (0..H)
        .filter_map(|y| {
            let xs: Vec<usize> = (0..W).filter(|&x| body[y][x] != '.').collect();
            Some((y, (*xs.first()?, *xs.last()?)))
        })
        .collect();
    let holes = |g: &Grid| -> usize {
        extent
            .iter()
            .map(|(&y, &(a, b))| (a..=b).filter(|&x| g[y][x] == '.').count())
            .sum()
    };
    assert_eq!(holes(&body), 0, "base body");
    for id in &c.default_outfit {
        let mut o = outfit.clone();
        assert!(o.remove(&c, id), "{id} is removable");
        let without = compose(&c, &o);
        assert_eq!(holes(&without), 0, "hole after removing {id}");
        // Only texels of this piece's own layers change.
        let own: Vec<Grid> = c.parts[id]
            .pieces
            .iter()
            .map(|p| layer_grid(&p.sheets["idle"].texture))
            .collect();
        for (x, y) in differences(&full, &without) {
            assert!(
                own.iter().any(|g| g[y][x] != '.'),
                "removing {id} changed ({x}, {y}), which is not one of its texels"
            );
        }
        assert!(
            !differences(&full, &without).is_empty(),
            "{id} shows nothing"
        );
    }
    // The base body is a body: skin on the head, the torso, the legs and the feet.
    for (y, label) in [(20, "head"), (40, "torso"), (75, "thigh"), (117, "foot")] {
        assert!(
            body[y].iter().any(|k| "Ssz".contains(*k)),
            "no skin on the {label} row"
        );
    }
}

#[test]
fn approved_art_and_older_manifests_are_unchanged() {
    for (path, hash) in APPROVED {
        let bytes = std::fs::read(root().join(path)).unwrap_or_else(|e| panic!("{path}: {e}"));
        assert_eq!(fnv1a(&bytes), hash, "{path} changed");
    }
    for path in [
        root().join("assets").join(MANIFEST_PATH),
        root().join(N3_EXPERIMENTAL_STAGE).join(MANIFEST_PATH),
    ] {
        AnimationManifest::from_ron(&read(&path))
            .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    }
    // A composition is not an animation manifest, and the reverse.
    assert!(AnimationManifest::from_ron(&read(&stage().join("protagonist.ron"))).is_err());
    assert!(
        CharacterComposition::from_ron(&read(&root().join("assets").join(MANIFEST_PATH))).is_err()
    );
    assert_eq!(CURRENT_SAVE_VERSION, 3, "the save format did not move");
}

// --- headless ECS -------------------------------------------------------------

const SYNC_FIXTURE: &str = r#"(
    version: 1,
    id: "sync_fixture",
    clips: { "run": (cell: (112, 128), pivot: (56.0, 118.0), density: 2, frames: 6) },
    layers: ["body", "shirt", "hair"],
    parts: {
        "body": (region: Body, removable: false, pieces: [
            (layer: "body", sheets: { "run": (texture: "body_run.png", columns: 6) }),
        ]),
        "shirt": (region: Shirt, pieces: [
            (layer: "shirt", sheets: { "run": (texture: "shirt_run.png", columns: 6) }),
        ]),
        "hair": (region: HairBack, pieces: [
            (layer: "hair", sheets: { "run": (texture: "hair_run.png", columns: 6) }),
        ]),
    },
    default_outfit: ["shirt", "hair"],
)"#;

#[derive(Debug, PartialEq)]
struct LayerState {
    slot: String,
    index: Option<usize>,
    flip: bool,
    anchor: Vec2,
    z: f32,
    visible: bool,
}

fn layer_states(app: &mut App) -> Vec<LayerState> {
    let world = app.world_mut();
    let mut q = world.query::<(&CompositeLayer, &Sprite, &Anchor, &Transform, &Visibility)>();
    let mut out: Vec<LayerState> = q
        .iter(world)
        .map(|(slot, sprite, anchor, t, v)| LayerState {
            slot: slot.0.clone(),
            index: sprite.texture_atlas.as_ref().map(|a| a.index),
            flip: sprite.flip_x,
            anchor: anchor.0,
            z: t.translation.z,
            visible: *v != Visibility::Hidden,
        })
        .collect();
    out.sort_by(|a, b| a.z.total_cmp(&b.z).then_with(|| a.slot.cmp(&b.slot)));
    out
}

#[test]
fn every_layer_follows_the_one_visual_playback() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(AssetPlugin::default())
        .init_asset::<Image>()
        .init_asset::<TextureAtlasLayout>()
        .add_systems(Update, sync_composite_layers);
    let c =
        Arc::new(CharacterComposition::from_ron(SYNC_FIXTURE).unwrap_or_else(|e| panic!("{e}")));
    let outfit = Outfit::default_for(&c);
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
                clip: Some("run".into()),
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
    let pivot_anchor = |flip: bool| {
        let (x, y) = anchor_for((56.0, 118.0), (112, 128), flip);
        Vec2::new(x, y)
    };
    for (frame, flip) in [(0, false), (3, false), (5, true), (2, true)] {
        {
            let mut e = app.world_mut().entity_mut(root);
            let mut p = e
                .get_mut::<VisualPlayback>()
                .unwrap_or_else(|| panic!("playback"));
            p.frame = frame;
            p.flip = flip;
        }
        app.update();
        let states = layer_states(&mut app);
        assert_eq!(states.len(), 3);
        for s in &states {
            assert!(s.visible, "{}", s.slot);
            assert_eq!(s.index, Some(frame), "{}: frame", s.slot);
            assert_eq!(s.flip, flip, "{}: facing", s.slot);
            assert_eq!(s.anchor, pivot_anchor(flip), "{}: pivot", s.slot);
        }
        assert!(states.windows(2).all(|w| w[0].z < w[1].z), "draw order");
    }
    // Removing a part hides its layer; the others keep the shared frame.
    {
        let mut e = app.world_mut().entity_mut(root);
        let mut ch = e
            .get_mut::<CompositeCharacter>()
            .unwrap_or_else(|| panic!("character"));
        let comp = ch.composition.clone();
        assert!(ch.outfit.remove(&comp, "shirt"));
    }
    app.update();
    let states = layer_states(&mut app);
    let shirt = states
        .iter()
        .find(|s| s.slot == "shirt")
        .unwrap_or_else(|| panic!("shirt"));
    assert!(!shirt.visible);
    assert!(
        states
            .iter()
            .filter(|s| s.slot != "shirt")
            .all(|s| s.visible && s.index == Some(2))
    );
}

#[derive(Resource, Default)]
struct PendingKeys {
    press: Vec<KeyCode>,
}

fn apply_pending_keys(mut pending: ResMut<PendingKeys>, mut keys: ResMut<ButtonInput<KeyCode>>) {
    for key in pending.press.drain(..) {
        keys.press(key);
    }
}

#[test]
fn the_player_animation_fills_the_shared_frame_and_facing() {
    let data = GameData::load(root()).unwrap_or_else(|e| panic!("{e}"));
    let saves = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("n4a_shared_state");
    let _ = std::fs::remove_dir_all(&saves);
    let mut app = App::new();
    dark_castle_rpg::input::use_legacy_controls(&mut app);
    app.add_plugins((MinimalPlugins, StatesPlugin, BevyInputPlugin))
        .add_plugins(AssetPlugin::default())
        .init_asset::<Image>()
        .init_asset::<TextureAtlasLayout>()
        .insert_resource(PlayerArtSource {
            root: root().join(N3_EXPERIMENTAL_STAGE),
            texture_prefix: String::new(),
        })
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(
            16,
        )))
        .insert_resource(data)
        .insert_resource(SaveSlots(SaveStorage::new(saves)))
        .init_resource::<PendingKeys>()
        .add_systems(PreUpdate, apply_pending_keys.after(InputSystems))
        .add_plugins(GamePlugin);
    app.update();
    let handles: Vec<Handle<Image>> = app
        .world()
        .resource::<PlayerSpriteSet>()
        .atlases
        .values()
        .map(|a| a.image.clone())
        .collect();
    for handle in handles {
        app.world_mut()
            .resource_mut::<Assets<Image>>()
            .insert(&handle, Image::default())
            .unwrap_or_else(|e| panic!("{e}"));
    }
    app.world_mut()
        .resource_mut::<PendingKeys>()
        .press
        .extend([KeyCode::ShiftLeft, KeyCode::ArrowLeft]);
    let mut seen = BTreeSet::new();
    for _ in 0..60 {
        app.update();
        let set = app.world().resource::<PlayerSpriteSet>().clone();
        let world = app.world_mut();
        let (p, sprite) = world
            .query_filtered::<(&VisualPlayback, &Sprite), With<PlayerVisual>>()
            .single(world)
            .unwrap_or_else(|e| panic!("{e}"));
        let first = p
            .clip
            .as_deref()
            .and_then(|k| set.manifest.clips.get(k))
            .map_or(0, |c| c.first as usize);
        assert_eq!(p.frame, p.index - first, "shared frame");
        assert_eq!(p.flip, sprite.flip_x, "shared facing");
        if p.clip.as_deref() == Some("run") {
            seen.insert(p.frame);
            assert!(p.flip, "running left");
        }
    }
    assert!(seen.len() >= 5, "run frames seen: {seen:?}");
}
