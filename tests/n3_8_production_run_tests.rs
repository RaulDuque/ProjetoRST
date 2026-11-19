//! N3.8: the approved 15 fps run in the real game, as the **experimental
//! protagonist** (`DARK_CASTLE_PROTAGONIST=n3`). The production V2.2 art stays
//! the default and untouched.
//!
//! - the staged PNGs are byte-identical copies of the approved N3.5 idle and
//!   N3.6 run (no pixel changed);
//! - the staged manifest: 6 frames of 1/15 s (0.4 s), fixed rate, pivots,
//!   cells, scale, mirroring;
//! - the loop timing is stable under 30, 60, 144 fps and a variable frame
//!   rate (no skipped frame, no drift);
//! - in the headless game (`AssetServer`, no renderer): idle and run show the
//!   new art, the run plays at 15 fps at 310 u/s, facing left mirrors around
//!   the same pivot, and every other state holds the N3.5 pose and names the
//!   missing clip (never a V2.2 frame);
//! - gameplay (positions, velocities, states) is identical with the
//!   production art and with the experimental art;
//! - equipment: the held weapon/shield layers stay off (no measured sockets
//!   yet) while the equipment itself is unchanged.
//!
//! `assets/` and `saves/` are never written.

use std::path::{Path, PathBuf};
use std::time::Duration;

use bevy::asset::AssetPlugin;
use bevy::input::{InputPlugin as BevyInputPlugin, InputSystems};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::state::app::StatesPlugin;
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::app::plugins::GamePlugin;
use dark_castle_rpg::core::constants::PLAYER_RUN_SPEED;
use dark_castle_rpg::core::resources::GameData;
use dark_castle_rpg::core::time::clamp_delta;
use dark_castle_rpg::inventory::held_visual::{HeldEquipment, HeldLayer};
use dark_castle_rpg::player::components::{ActionState, Player};
use dark_castle_rpg::player::state::PlayerState;
use dark_castle_rpg::player::visual::art_profile::{N3_EXPERIMENTAL_STAGE, ProtagonistArt};
use dark_castle_rpg::player::visual::manifest::{AnimationManifest, ClipMode, anchor_for};
use dark_castle_rpg::player::visual::playback::loop_frame;
use dark_castle_rpg::player::visual::sockets::SOCKETS_PATH;
use dark_castle_rpg::player::visual::systems::load_sockets;
use dark_castle_rpg::player::visual::{
    MANIFEST_PATH, PlayerArtSource, PlayerSpriteSet, PlayerVisual, VisualPlayback,
};
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::save::systems::SaveSlots;
use dark_castle_rpg::world::physics::{FacingDirection, Velocity};

const FRAME: Duration = Duration::from_millis(16);
const RUN_FPS: f32 = 15.0;
const RUN_FRAMES: usize = 6;
const RUN_CYCLE: f32 = 0.4;

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn stage() -> PathBuf {
    root().join(N3_EXPERIMENTAL_STAGE)
}

fn manifest() -> AnimationManifest {
    let text = std::fs::read_to_string(stage().join(MANIFEST_PATH))
        .unwrap_or_else(|e| panic!("stage manifest: {e}"));
    AnimationManifest::from_ron(&text).unwrap_or_else(|e| panic!("{e}"))
}

fn bytes(path: &Path) -> Vec<u8> {
    std::fs::read(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn png_size(path: &Path) -> (u32, u32) {
    let b = bytes(path);
    assert_eq!(&b[..8], b"\x89PNG\r\n\x1a\n", "{}", path.display());
    let read = |at: usize| u32::from_be_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]]);
    (read(16), read(20))
}

// --- art and manifest -------------------------------------------------------

#[test]
fn the_staged_art_is_a_byte_identical_copy_of_the_approved_art() {
    for (staged, approved) in [
        (
            "sprites/player/n35_idle.png",
            "art-src/n3-5-fidelity/generated/candidate_106.png",
        ),
        (
            "sprites/player/n36_run6.png",
            "art-src/n3-6-animation-proof/generated/run6.png",
        ),
    ] {
        assert!(
            bytes(&stage().join(staged)) == bytes(&root().join(approved)),
            "{staged} differs from {approved}"
        );
    }
}

#[test]
fn the_run_clip_is_six_frames_at_exactly_15_fps() {
    let m = manifest();
    let run = &m.clips["run"];
    assert_eq!(run.count as usize, RUN_FRAMES);
    assert_eq!(run.first, 0);
    assert_eq!(run.mode, ClipMode::Loop { fps: RUN_FPS });
    assert_eq!(run.durations.len(), RUN_FRAMES);
    for d in &run.durations {
        assert!((d - 1.0 / RUN_FPS).abs() < 1e-7, "frame lasts {d} s");
    }
    assert!((run.total_duration() - RUN_CYCLE).abs() < 1e-6);
    // Fixed rate: the speed never speeds the cycle up (owner: no automatic
    // fps increase).
    assert_eq!(run.speed_reference, None);
    assert_eq!(run.fallback.as_deref(), Some("idle"));
    // 5 steps per second: two steps per cycle.
    assert!((2.0 / run.total_duration() - 5.0).abs() < 1e-4);
}

#[test]
fn cells_pivots_and_scale_follow_the_approved_sources() {
    let m = manifest();
    assert!(!m.weapon_in_body, "no weapon drawn in the new protagonist");
    for (clip, cell, pivot, world) in [
        ("idle", (80, 128), (40.0, 118.0), (40.0, 64.0)),
        ("run", (112, 128), (56.0, 118.0), (56.0, 64.0)),
    ] {
        let c = &m.clips[clip];
        let atlas = &m.atlases[&c.atlas];
        assert_eq!(atlas.cell, cell, "{clip}: cell");
        assert_eq!(atlas.density, 2, "{clip}: density");
        assert_eq!(c.pivot, pivot, "{clip}: pivot");
        assert_eq!(atlas.world_size(), world, "{clip}: world size");
        assert_eq!(
            png_size(&stage().join(&atlas.texture)),
            atlas.image_size(),
            "{clip}: PNG size"
        );
    }
    // Only the approved clips: nothing else is presented as finished art.
    let keys: Vec<&str> = m.clips.keys().map(String::as_str).collect();
    assert_eq!(keys, ["idle", "run"]);
}

#[test]
fn mirroring_keeps_the_pivot_and_the_sole_line() {
    let m = manifest();
    let mut soles = Vec::new();
    for clip in ["idle", "run"] {
        let c = &m.clips[clip];
        let cell = m.atlases[&c.atlas].cell;
        let right = anchor_for(c.pivot, cell, false);
        let left = anchor_for(c.pivot, cell, true);
        // The pivot is the horizontal centre of both canvases: flipping
        // does not move the body sideways.
        assert_eq!(right.0, 0.0, "{clip}");
        assert_eq!(left.0, 0.0, "{clip}");
        assert_eq!(right.1, left.1, "{clip}");
        soles.push(right.1 * cell.1 as f32);
    }
    // Same sole row (10 texels above the bottom) in both canvases.
    assert_eq!(soles[0], soles[1]);
}

#[test]
fn no_socket_table_so_the_held_layers_stay_off() {
    assert!(!stage().join(SOCKETS_PATH).exists());
    assert!(load_sockets(&stage(), &manifest()).is_none());
}

#[test]
fn the_production_art_is_still_the_default_and_unchanged() {
    assert_eq!(ProtagonistArt::default(), ProtagonistArt::Production);
    let text = std::fs::read_to_string(root().join("assets").join(MANIFEST_PATH))
        .unwrap_or_else(|e| panic!("{e}"));
    let production = AnimationManifest::from_ron(&text).unwrap_or_else(|e| panic!("{e}"));
    let run = &production.clips["run"];
    assert_eq!(
        (run.atlas.as_str(), run.first, run.count),
        ("locomotion", 12, 10)
    );
    assert_eq!(run.speed_reference, Some(190.0));
    assert!(
        !production
            .atlases
            .values()
            .any(|a| a.texture.contains("n36"))
    );
}

#[test]
fn every_state_resolves_to_an_approved_clip_with_a_visible_marker() {
    use dark_castle_rpg::player::visual::selection::{SelectionContext, Tint, select};
    let m = manifest();
    let ctx = SelectionContext {
        skill_animation: Some("slash_crescent"),
        ..SelectionContext::default()
    };
    for state in [
        PlayerState::Idle,
        PlayerState::Walking,
        PlayerState::Running,
        PlayerState::Crouching,
        PlayerState::Jumping,
        PlayerState::Falling,
        PlayerState::Attacking,
        PlayerState::Casting,
        PlayerState::Guarding,
        PlayerState::Dodging,
        PlayerState::Hurt,
        PlayerState::Dead,
    ] {
        let plan = select(state, &ctx);
        let (key, _) = m
            .resolve(plan.clip)
            .unwrap_or_else(|| panic!("{state:?}: nothing to show"));
        assert!(key == "idle" || key == "run", "{state:?} shows {key}");
        if key != plan.clip {
            assert_eq!(key, "idle", "{state:?}: placeholder is the N3.5 pose");
        }
    }
    // Hurt, guard, cast and death keep their tint on the placeholder.
    for state in [
        PlayerState::Hurt,
        PlayerState::Guarding,
        PlayerState::Casting,
        PlayerState::Dead,
    ] {
        assert_ne!(select(state, &ctx).missing_tint, Tint::None, "{state:?}");
    }
}

// --- timing -------------------------------------------------------------------

/// Plays the run loop with the given frame times (clamped like the game)
/// and checks every update against the exact frame `floor(t * 15) mod 6`.
fn play(deltas: impl Iterator<Item = f32>) -> (usize, usize) {
    let durations = manifest().clips["run"].durations.clone();
    let mut elapsed = 0.0_f32;
    let mut exact = 0.0_f64;
    let mut last = 0;
    let mut changes = 0;
    let mut updates = 0;
    for dt in deltas {
        let dt = clamp_delta(dt);
        elapsed += dt;
        exact += f64::from(dt);
        let frame = loop_frame(&durations, elapsed);
        let phase = (exact * f64::from(RUN_FPS)).rem_euclid(RUN_FRAMES as f64);
        let expected = phase.floor() as usize % RUN_FRAMES;
        // Away from a frame boundary the shown frame is the exact one.
        let near_edge = (phase - phase.round()).abs() < 1e-3;
        if !near_edge {
            assert_eq!(frame, expected, "t = {exact:.4} s");
        }
        // Never more than one frame per update: 1/15 s > the 1/30 s clamp.
        let step = (frame + RUN_FRAMES - last) % RUN_FRAMES;
        assert!(step <= 1, "skipped a frame at t = {exact:.4} s");
        changes += step;
        last = frame;
        updates += 1;
    }
    (changes, updates)
}

#[test]
fn the_loop_is_stable_at_30_60_and_144_fps() {
    for rate in [30.0_f32, 60.0, 144.0] {
        let seconds = 20.0;
        let n = (seconds * rate) as usize;
        let (changes, _) = play(std::iter::repeat_n(1.0 / rate, n));
        // 15 frame changes per second, whatever the display rate.
        let expected = (seconds * RUN_FPS) as usize;
        assert!(
            changes.abs_diff(expected) <= 1,
            "{rate} fps: {changes} changes in {seconds} s"
        );
    }
}

#[test]
fn the_loop_is_stable_with_a_variable_frame_rate() {
    // Deterministic jitter between 4 ms and 33 ms, plus a few long hitches
    // that the game clamps to 1/30 s.
    let deltas = (0..3000).map(|i| {
        let jitter = ((i * 7919) % 29) as f32 / 1000.0;
        if i % 500 == 499 { 0.25 } else { 0.004 + jitter }
    });
    let (changes, updates) = play(deltas);
    assert_eq!(updates, 3000);
    assert!(changes > 0);
}

// --- headless game ------------------------------------------------------------

#[derive(Resource, Default)]
struct PendingKeys {
    press: Vec<KeyCode>,
    release: Vec<KeyCode>,
}

fn apply_pending_keys(mut pending: ResMut<PendingKeys>, mut keys: ResMut<ButtonInput<KeyCode>>) {
    for key in pending.release.drain(..) {
        keys.release(key);
    }
    for key in pending.press.drain(..) {
        keys.press(key);
    }
}

fn tmp(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

/// The game with the experimental stage (`true`) or the published art.
fn app(experimental: bool, saves: &str) -> App {
    let data = GameData::load(root()).unwrap_or_else(|e| panic!("{e}"));
    let mut app = App::new();
    dark_castle_rpg::input::use_legacy_controls(&mut app);
    app.add_plugins((MinimalPlugins, StatesPlugin, BevyInputPlugin))
        .add_plugins(AssetPlugin::default())
        .init_asset::<Image>()
        .init_asset::<TextureAtlasLayout>();
    if experimental {
        app.insert_resource(PlayerArtSource {
            root: stage(),
            texture_prefix: String::new(),
        });
    }
    app.insert_resource(TimeUpdateStrategy::ManualDuration(FRAME))
        .insert_resource(data)
        .insert_resource(SaveSlots(SaveStorage::new(tmp(saves))))
        .init_resource::<PendingKeys>()
        .add_systems(PreUpdate, apply_pending_keys.after(InputSystems))
        .add_plugins(GamePlugin);
    app.update();
    // The atlas PNGs never load here: put images under their handles.
    let handles: Vec<Handle<Image>> = app
        .world()
        .resource::<PlayerSpriteSet>()
        .atlases
        .values()
        .map(|a| a.image.clone())
        .collect();
    let mut images = app.world_mut().resource_mut::<Assets<Image>>();
    for handle in handles {
        images
            .insert(&handle, Image::default())
            .unwrap_or_else(|e| panic!("{e}"));
    }
    run(&mut app, 30);
    app
}

fn run(app: &mut App, frames: usize) {
    for _ in 0..frames {
        app.update();
    }
}

fn press(app: &mut App, key: KeyCode) {
    app.world_mut()
        .resource_mut::<PendingKeys>()
        .press
        .push(key);
}

fn release(app: &mut App, key: KeyCode) {
    app.world_mut()
        .resource_mut::<PendingKeys>()
        .release
        .push(key);
}

#[derive(Debug, Clone, PartialEq)]
struct Shown {
    clip: Option<String>,
    frame: usize,
    placeholder: Option<String>,
    flip: bool,
    anchor: Vec2,
    atlas_image: Handle<Image>,
}

fn shown(app: &mut App) -> Shown {
    let set = app.world().resource::<PlayerSpriteSet>().clone();
    let world = app.world_mut();
    let (playback, sprite, anchor) = world
        .query_filtered::<(&VisualPlayback, &Sprite, &Anchor), With<PlayerVisual>>()
        .single(world)
        .unwrap_or_else(|e| panic!("{e}"));
    let first = playback
        .clip
        .as_deref()
        .and_then(|k| set.manifest.clips.get(k))
        .map_or(0, |c| c.first as usize);
    Shown {
        clip: playback.clip.clone(),
        frame: playback.index - first,
        placeholder: playback.placeholder_for.clone(),
        flip: sprite.flip_x,
        anchor: anchor.0,
        atlas_image: sprite.image.clone(),
    }
}

fn atlas_image(app: &App, clip: &str) -> Handle<Image> {
    let set = app.world().resource::<PlayerSpriteSet>();
    let atlas = &set.manifest.clips[clip].atlas;
    set.atlases[atlas].image.clone()
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Body {
    position: Vec3,
    velocity: Vec2,
    state: PlayerState,
}

fn body(app: &mut App) -> Body {
    let world = app.world_mut();
    let (t, v, a) = world
        .query_filtered::<(&Transform, &Velocity, &ActionState), With<Player>>()
        .single(world)
        .unwrap_or_else(|e| panic!("{e}"));
    Body {
        position: t.translation,
        velocity: v.0,
        state: a.state(),
    }
}

fn facing_left(app: &mut App) -> bool {
    let world = app.world_mut();
    world
        .query_filtered::<&FacingDirection, With<Player>>()
        .single(world)
        .map(|f| f.0 == dark_castle_rpg::player::state::Facing::Left)
        .unwrap_or_else(|e| panic!("{e}"))
}

#[test]
fn idle_and_run_show_the_new_art_at_15_fps_and_310_units_per_second() {
    let mut app = app(true, "n38_run");
    let idle = shown(&mut app);
    assert_eq!(idle.clip.as_deref(), Some("idle"));
    assert_eq!(idle.placeholder, None);
    assert_eq!(idle.atlas_image, atlas_image(&app, "idle"));

    press(&mut app, KeyCode::ShiftLeft);
    press(&mut app, KeyCode::ArrowRight);
    run(&mut app, 2);
    // Record 3 s of running, one sample per 16 ms update. The test room is
    // narrower than 3 s at 310 u/s: against the wall the state stays
    // Running and the cycle keeps its fixed 15 fps (no speed_reference).
    let mut frames = Vec::new();
    let mut free_run = 0;
    for _ in 0..188 {
        run(&mut app, 1);
        let s = shown(&mut app);
        let b = body(&mut app);
        assert_eq!(s.clip.as_deref(), Some("run"));
        assert_eq!(s.atlas_image, atlas_image(&app, "run"));
        assert!(!s.flip);
        assert_eq!(s.anchor.x, 0.0);
        assert_eq!(b.state, PlayerState::Running);
        if (b.velocity.x - PLAYER_RUN_SPEED).abs() < 1e-3 {
            free_run += 1;
        } else {
            assert_eq!(b.velocity.x, 0.0, "only a wall stops the run: {b:?}");
        }
        frames.push(s.frame);
    }
    assert!(free_run >= 30, "only {free_run} updates at 310 u/s");
    // The clip restarts once when the run starts, then never again: the
    // frame only moves forward, one step at a time, 15 times per second.
    let changes = frames.windows(2).filter(|w| w[0] != w[1]).count();
    for w in frames.windows(2) {
        let step = (w[1] + RUN_FRAMES - w[0]) % RUN_FRAMES;
        assert!(step <= 1, "skipped or reversed: {w:?}");
    }
    let seconds = frames.len() as f32 * FRAME.as_secs_f32();
    let expected = (seconds * RUN_FPS).floor() as usize;
    assert!(
        changes.abs_diff(expected) <= 1,
        "{changes} frame changes in {seconds} s (expected ~{expected})"
    );
    // Every one of the six approved frames is shown.
    for f in 0..RUN_FRAMES {
        assert!(frames.contains(&f), "frame {f} never shown");
    }

    // Turn: mirrored around the same pivot, the cycle keeps running.
    release(&mut app, KeyCode::ArrowRight);
    press(&mut app, KeyCode::ArrowLeft);
    run(&mut app, 3);
    let s = shown(&mut app);
    assert!(facing_left(&mut app));
    assert_eq!(s.clip.as_deref(), Some("run"));
    assert!(s.flip);
    assert_eq!(s.anchor.x, 0.0);
    assert!((body(&mut app).velocity.x + PLAYER_RUN_SPEED).abs() < 1e-3);

    // Run -> idle: back to the N3.5 pose, mirrored.
    release(&mut app, KeyCode::ArrowLeft);
    release(&mut app, KeyCode::ShiftLeft);
    run(&mut app, 3);
    let s = shown(&mut app);
    assert_eq!(s.clip.as_deref(), Some("idle"));
    assert!(s.flip);
    assert_eq!(s.placeholder, None);
}

#[test]
fn states_without_approved_art_hold_the_n35_pose_and_name_the_missing_clip() {
    let mut app = app(true, "n38_states");
    let idle_image = atlas_image(&app, "idle");
    // Each action is started from a run, as in play.
    let actions: [(KeyCode, PlayerState, &str); 4] = [
        (KeyCode::KeyZ, PlayerState::Jumping, "jump_rise"),
        (KeyCode::KeyX, PlayerState::Attacking, "attack_basic"),
        (KeyCode::KeyS, PlayerState::Dodging, "dodge"),
        (KeyCode::KeyA, PlayerState::Guarding, "guard_start"),
    ];
    for (key, state, wanted) in actions {
        press(&mut app, KeyCode::ShiftLeft);
        press(&mut app, KeyCode::ArrowRight);
        run(&mut app, 20);
        assert_eq!(shown(&mut app).clip.as_deref(), Some("run"));
        press(&mut app, key);
        run(&mut app, 2);
        let b = body(&mut app);
        let s = shown(&mut app);
        assert_eq!(b.state, state, "{key:?}");
        assert_eq!(s.clip.as_deref(), Some("idle"), "{key:?}");
        assert_eq!(s.frame, 0);
        assert_eq!(s.atlas_image, idle_image, "{key:?}: never a V2.2 frame");
        assert_eq!(s.placeholder.as_deref(), Some(wanted), "{key:?}");
        release(&mut app, key);
        release(&mut app, KeyCode::ArrowRight);
        release(&mut app, KeyCode::ShiftLeft);
        run(&mut app, 90);
        assert_eq!(shown(&mut app).placeholder, None, "{key:?}: back to idle");
    }
}

#[test]
fn gameplay_is_identical_with_the_production_and_the_experimental_art() {
    let script: [(usize, &[KeyCode], &[KeyCode]); 8] = [
        (10, &[KeyCode::ShiftLeft, KeyCode::ArrowRight], &[]),
        (40, &[KeyCode::KeyZ], &[]),
        (5, &[], &[KeyCode::KeyZ]),
        (60, &[KeyCode::KeyX], &[]),
        (5, &[], &[KeyCode::KeyX]),
        (30, &[KeyCode::ArrowLeft], &[KeyCode::ArrowRight]),
        (30, &[KeyCode::KeyS], &[]),
        (
            40,
            &[],
            &[KeyCode::KeyS, KeyCode::ArrowLeft, KeyCode::ShiftLeft],
        ),
    ];
    let mut traces = Vec::new();
    for (experimental, saves) in [(false, "n38_same_prod"), (true, "n38_same_exp")] {
        let mut app = app(experimental, saves);
        let mut trace = Vec::new();
        for (frames, down, up) in script {
            for k in down {
                press(&mut app, *k);
            }
            for k in up {
                release(&mut app, *k);
            }
            for _ in 0..frames {
                run(&mut app, 1);
                trace.push(body(&mut app));
            }
        }
        traces.push(trace);
    }
    assert_eq!(traces[0].len(), traces[1].len());
    for (i, (a, b)) in traces[0].iter().zip(&traces[1]).enumerate() {
        assert_eq!(a, b, "update {i}: gameplay differs");
    }
    assert!(traces[0].iter().any(|b| b.state == PlayerState::Running));
}

#[test]
fn equipment_is_kept_while_its_layers_stay_off_without_sockets() {
    let mut app = app(true, "n38_equipment");
    assert!(
        !app.world()
            .resource::<PlayerSpriteSet>()
            .held_layers_enabled()
    );
    press(&mut app, KeyCode::ShiftLeft);
    press(&mut app, KeyCode::ArrowRight);
    run(&mut app, 30);
    let world = app.world_mut();
    let held = world
        .query_filtered::<&HeldEquipment, With<Player>>()
        .single(world)
        .copied()
        .unwrap_or_else(|e| panic!("{e}"));
    assert!(held.0.weapon.is_some(), "the starting weapon is still held");
    let visible = world
        .query::<(&HeldLayer, &Visibility)>()
        .iter(world)
        .filter(|(_, v)| **v != Visibility::Hidden)
        .count();
    assert_eq!(visible, 0, "no weapon floating on unmeasured sockets");
}
