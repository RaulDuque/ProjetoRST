//! N4-1 (C1): the run frames #0, #2 and #4 as modular layers.
//!
//! - the `run_c1` clip uses the run canvas (112x128) and pivot (56, 118),
//!   like the approved N3.6/N3.8 run;
//! - every frame recomposed from its layers is the approved frame texel for
//!   texel (run6 #0, #2, #4 = video 48, 52, 56);
//! - each piece is independent in every frame and its removal opens no
//!   cavity the dressed frame does not have;
//! - in a headless app, the layers follow one `VisualPlayback` frame and
//!   facing, at 30, 60 and 144 fps and with a variable frame time, and the
//!   C1 cycle keeps the run's 0.4 s;
//! - the idle of N4-A is unchanged (tests/n4_sprite_machine_tests.rs).

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use dark_castle_rpg::core::time::clamp_delta;
use dark_castle_rpg::player::visual::VisualPlayback;
use dark_castle_rpg::player::visual::art_profile::N3_EXPERIMENTAL_STAGE;
use dark_castle_rpg::player::visual::composite::{
    CompositeCharacter, CompositeLayer, sync_composite_layers,
};
use dark_castle_rpg::player::visual::composition::{CharacterComposition, Outfit, resolve};
use dark_castle_rpg::player::visual::manifest::{AnimationManifest, anchor_for};
use dark_castle_rpg::player::visual::playback::loop_frame;

const W: usize = 112;
const H: usize = 128;
const CLIP: &str = "run_c1";
/// run6 frame shown by each C1 frame.
const SOURCES: [&str; 3] = ["video_048", "video_052", "video_056"];
const PALETTE_KEYS: &str = ".KWwhHSszeGgqPpoLldM";
/// The run cycle (N3.8): 6 frames of 1/15 s. C1 shows every other frame,
/// each for 2/15 s, so the cycle stays 0.4 s.
const C1_FRAME_TIME: f32 = 2.0 / 15.0;

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

type Grid = Vec<Vec<char>>;

fn grid(path: &Path, width: usize) -> Grid {
    let rows: Grid = read(path).lines().map(|l| l.chars().collect()).collect();
    assert_eq!(rows.len(), H, "{}", path.display());
    assert!(rows.iter().all(|r| r.len() == width), "{}", path.display());
    rows
}

fn png_size(path: &Path) -> (u32, u32) {
    let b = std::fs::read(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    assert!(b.starts_with(b"\x89PNG\r\n\x1a\n"), "{}", path.display());
    let word = |at: usize| u32::from_be_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]]);
    (word(16), word(20))
}

/// Frame `frame` (a 112-wide column) of a layer strip.
fn layer_frame(texture: &str, frame: usize) -> Grid {
    let strip = grid(&stage().join(texture.replace(".png", ".txt")), W * 3);
    strip
        .iter()
        .map(|row| row[frame * W..(frame + 1) * W].to_vec())
        .collect()
}

fn compose(c: &CharacterComposition, outfit: &Outfit, frame: u32) -> Grid {
    let mut out = vec![vec!['.'; W]; H];
    for d in resolve(c, outfit, CLIP, frame)
        .unwrap_or_else(|e| panic!("{e}"))
        .iter()
        .filter(|d| d.visible)
    {
        let g = layer_frame(&d.texture, d.index as usize);
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

fn approved(frame: usize) -> Grid {
    grid(
        &root().join(format!(
            "art-src/n3-6-animation-proof/source/{}.txt",
            SOURCES[frame]
        )),
        W,
    )
}

/// Transparent texels with no transparent path to the canvas border.
fn cavities(g: &Grid) -> BTreeSet<(usize, usize)> {
    let clear: BTreeSet<(usize, usize)> = (0..H)
        .flat_map(|y| (0..W).map(move |x| (x, y)))
        .filter(|&(x, y)| g[y][x] == '.')
        .collect();
    let mut stack: Vec<(usize, usize)> = clear
        .iter()
        .copied()
        .filter(|&(x, y)| x == 0 || y == 0 || x == W - 1 || y == H - 1)
        .collect();
    let mut seen: BTreeSet<(usize, usize)> = stack.iter().copied().collect();
    while let Some((x, y)) = stack.pop() {
        let mut next = vec![(x + 1, y), (x, y + 1)];
        if x > 0 {
            next.push((x - 1, y));
        }
        if y > 0 {
            next.push((x, y - 1));
        }
        for q in next {
            if clear.contains(&q) && seen.insert(q) {
                stack.push(q);
            }
        }
    }
    clear.difference(&seen).copied().collect()
}

#[test]
fn the_c1_clip_uses_the_approved_run_canvas_and_pivot() {
    let c = composition();
    let clip = &c.clips[CLIP];
    assert_eq!(clip.cell, (112, 128));
    assert_eq!(clip.pivot, (56.0, 118.0));
    assert_eq!(clip.density, 2);
    assert_eq!(clip.frames, 3);
    // Same canvas and pivot as the run the game plays (N3.8 stage).
    let n38 = AnimationManifest::from_ron(&read(
        &root()
            .join(N3_EXPERIMENTAL_STAGE)
            .join("animations/player.ron"),
    ))
    .unwrap_or_else(|e| panic!("{e}"));
    let run = &n38.clips["run"];
    assert_eq!(run.pivot, clip.pivot);
    assert_eq!(n38.atlases[&run.atlas].cell, clip.cell);
    // Every run_c1 sheet: a 3-frame strip, palette keys only.
    let mut sheets = 0;
    for part in c.parts.values() {
        for piece in &part.pieces {
            if let Some(sheet) = piece.sheets.get(CLIP) {
                sheets += 1;
                assert_eq!(sheet.columns, 3);
                assert_eq!(png_size(&stage().join(&sheet.texture)), (336, 128));
                for f in 0..3 {
                    let g = layer_frame(&sheet.texture, f);
                    assert!(g.iter().flatten().all(|k| PALETTE_KEYS.contains(*k)));
                }
            }
        }
    }
    assert_eq!(sheets, 14, "one sheet per layer slot");
}

#[test]
fn each_c1_frame_recomposes_the_approved_frame_texel_for_texel() {
    let c = composition();
    let outfit = Outfit::default_for(&c);
    for frame in 0..3 {
        let full = compose(&c, &outfit, frame as u32);
        let original = approved(frame);
        let diff: Vec<(usize, usize)> = (0..H)
            .flat_map(|y| (0..W).map(move |x| (x, y)))
            .filter(|&(x, y)| full[y][x] != original[y][x])
            .collect();
        assert!(diff.is_empty(), "#{frame}: {} texels differ", diff.len());
        // Ground contact: the lowest opaque row is the same.
        let lowest = |g: &Grid| (0..H).rev().find(|&y| g[y].iter().any(|k| *k != '.'));
        assert_eq!(lowest(&full), lowest(&original));
    }
}

#[test]
fn each_piece_is_independent_and_opens_no_cavity_in_any_frame() {
    let c = composition();
    let outfit = Outfit::default_for(&c);
    let mut base = outfit.clone();
    for id in c.default_outfit.clone() {
        base.remove(&c, &id);
    }
    for frame in 0..3u32 {
        let full = compose(&c, &outfit, frame);
        let dressed_cavities = cavities(&full);
        let body = compose(&c, &base, frame);
        assert!(
            cavities(&body).is_subset(&dressed_cavities),
            "#{frame}: the base body opens a cavity"
        );
        for id in &c.default_outfit {
            let mut o = outfit.clone();
            assert!(o.remove(&c, id));
            let without = compose(&c, &o, frame);
            assert!(
                cavities(&without).is_subset(&dressed_cavities),
                "#{frame}: removing {id} opens a cavity"
            );
            let own: Vec<Grid> = c.parts[id]
                .pieces
                .iter()
                .filter_map(|p| p.sheets.get(CLIP))
                .map(|s| layer_frame(&s.texture, frame as usize))
                .collect();
            let mut changed = 0;
            for y in 0..H {
                for x in 0..W {
                    if full[y][x] != without[y][x] {
                        changed += 1;
                        assert!(
                            own.iter().any(|g| g[y][x] != '.'),
                            "#{frame}: removing {id} changed ({x}, {y})"
                        );
                    }
                }
            }
            assert!(changed > 0, "#{frame}: {id} shows nothing");
        }
        // The base body has skin on the head, the torso, both legs and feet.
        let skin_rows = (0..H)
            .filter(|&y| body[y].iter().any(|k| "Ssz".contains(*k)))
            .count();
        assert!(skin_rows > 90, "#{frame}: only {skin_rows} rows with skin");
    }
}

// --- headless ECS ---------------------------------------------------------------

fn app_with_character() -> (App, Entity) {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(AssetPlugin::default())
        .init_asset::<Image>()
        .init_asset::<TextureAtlasLayout>()
        .add_systems(Update, sync_composite_layers);
    let c = Arc::new(composition());
    let character = {
        let world = app.world_mut();
        let server = world.resource::<AssetServer>().clone();
        let mut layouts = world.resource_mut::<Assets<TextureAtlasLayout>>();
        CompositeCharacter::load(
            c.clone(),
            Outfit::default_for(&c),
            &server,
            &mut layouts,
            "",
        )
    };
    let root = app
        .world_mut()
        .spawn((
            VisualPlayback {
                clip: Some(CLIP.into()),
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
    (app, root)
}

/// (slot, atlas index, flip, anchor) of every visible layer.
fn visible_layers(app: &mut App) -> Vec<(String, usize, bool, Vec2)> {
    let world = app.world_mut();
    let mut q = world.query::<(&CompositeLayer, &Sprite, &Anchor, &Visibility)>();
    q.iter(world)
        .filter(|(_, _, _, v)| **v != Visibility::Hidden)
        .map(|(slot, sprite, anchor, _)| {
            (
                slot.0.clone(),
                sprite
                    .texture_atlas
                    .as_ref()
                    .map_or(usize::MAX, |a| a.index),
                sprite.flip_x,
                anchor.0,
            )
        })
        .collect()
}

#[test]
fn the_layers_change_frame_together_at_any_render_rate() {
    let durations = [C1_FRAME_TIME; 3];
    let rates: [(&str, Vec<f32>); 4] = [
        ("30 fps", vec![1.0 / 30.0; 120]),
        ("60 fps", vec![1.0 / 60.0; 240]),
        ("144 fps", vec![1.0 / 144.0; 576]),
        (
            "variable",
            (0..300)
                .map(|i| {
                    if i % 97 == 96 {
                        0.25
                    } else {
                        0.004 + ((i * 7919) % 29) as f32 / 1000.0
                    }
                })
                .collect(),
        ),
    ];
    for (label, deltas) in rates {
        let (mut app, root) = app_with_character();
        let mut elapsed = 0.0_f32;
        let mut exact = 0.0_f64;
        let mut changes = 0;
        let mut last = 0;
        for (i, dt) in deltas.iter().enumerate() {
            let dt = clamp_delta(*dt);
            elapsed += dt;
            exact += f64::from(dt);
            // The one clock: the same loop function the player animation uses.
            let frame = loop_frame(&durations, elapsed);
            let flip = (i / 50) % 2 == 1;
            {
                let mut e = app.world_mut().entity_mut(root);
                let mut p = e
                    .get_mut::<VisualPlayback>()
                    .unwrap_or_else(|| panic!("playback"));
                p.frame = frame;
                p.flip = flip;
            }
            app.update();
            let layers = visible_layers(&mut app);
            assert_eq!(layers.len(), 14, "{label}: all layers drawn");
            let (ax, ay) = anchor_for((56.0, 118.0), (112, 128), flip);
            for (slot, index, f, anchor) in &layers {
                assert_eq!(*index, frame, "{label}: {slot} out of step");
                assert_eq!(*f, flip, "{label}: {slot} facing");
                assert_eq!(*anchor, Vec2::new(ax, ay), "{label}: {slot} pivot");
            }
            if frame != last {
                assert_eq!((last + 1) % 3, frame, "{label}: skipped a frame");
                changes += 1;
            }
            last = frame;
        }
        // 7.5 C1 frames per second = one 0.4 s cycle every 3 changes.
        let expected = (exact / f64::from(C1_FRAME_TIME)).floor() as i64;
        assert!(
            (changes as i64 - expected).abs() <= 1,
            "{label}: {changes} changes in {exact:.2} s, expected {expected}"
        );
    }
}

#[test]
fn removing_a_piece_during_the_run_hides_only_its_layers() {
    let (mut app, root) = app_with_character();
    for (frame, piece, gone) in [
        (0, "hair", ["hair_back", "hair_front"].as_slice()),
        (
            1,
            "shirt",
            ["sleeve_back", "shirt_torso", "shirt"].as_slice(),
        ),
        (2, "trousers", ["trousers_back", "trousers"].as_slice()),
        (0, "boots", ["boots_back", "boots"].as_slice()),
        (1, "bracers", ["bracers_back", "bracers"].as_slice()),
    ] {
        {
            let mut e = app.world_mut().entity_mut(root);
            e.get_mut::<VisualPlayback>()
                .unwrap_or_else(|| panic!("playback"))
                .frame = frame;
            let mut ch = e
                .get_mut::<CompositeCharacter>()
                .unwrap_or_else(|| panic!("character"));
            let comp = ch.composition.clone();
            ch.outfit = Outfit::default_for(&comp);
            assert!(ch.outfit.remove(&comp, piece));
        }
        app.update();
        let layers = visible_layers(&mut app);
        assert_eq!(layers.len(), 14 - gone.len(), "{piece}");
        for (slot, index, ..) in &layers {
            assert!(
                !gone.contains(&slot.as_str()),
                "{slot} still drawn without {piece}"
            );
            assert_eq!(*index, frame, "{slot}: frame");
        }
    }
}
