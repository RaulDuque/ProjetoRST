//! Headless tests of the player's sprite visual (V1.3).
//!
//! The app has an `AssetServer` but no image loader or renderer: the
//! textures never load from disk, which is exactly the "not loaded yet"
//! case; tests that need them "loaded" put an image in `Assets<Image>`
//! under the real handles. Nothing is drawn: what is checked is the ECS
//! wiring (one child, the right clip and frame, flip and anchor, the
//! placeholder box) and that gameplay is identical with and without it.

use std::path::{Path, PathBuf};
use std::time::Duration;

use bevy::asset::AssetPlugin;
use bevy::input::{InputPlugin as BevyInputPlugin, InputSystems};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::state::app::StatesPlugin;
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::app::plugins::GamePlugin;
use dark_castle_rpg::app::states::GameSet;
use dark_castle_rpg::core::constants::{
    BASIC_ATTACK_TIME, DEFAULT_SAVE_SLOT, PLAYER_DODGE_TIME, PLAYER_SIZE,
};
use dark_castle_rpg::core::events::SkillActivated;
use dark_castle_rpg::core::resources::GameData;
use dark_castle_rpg::player::components::{ActionState, Player, SkillBook};
use dark_castle_rpg::player::state::PlayerState;
use dark_castle_rpg::player::visual::manifest::anchor_for;
use dark_castle_rpg::player::visual::{PlayerSpriteSet, PlayerVisual, VisualPlayback};
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::save::systems::SaveSlots;

const FRAME: Duration = Duration::from_millis(16);

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

/// Skill ids to request on the next frame (as the command recogniser would).
#[derive(Resource, Default)]
struct PendingActivations(Vec<String>);

fn emit_pending_activations(
    mut pending: ResMut<PendingActivations>,
    player: Query<Entity, With<Player>>,
    mut activated: MessageWriter<SkillActivated>,
) {
    let Ok(player) = player.single() else {
        return;
    };
    for skill_id in pending.0.drain(..) {
        activated.write(SkillActivated {
            caster: player,
            skill_id,
        });
    }
}

/// The game app; `assets` adds an `AssetServer` and the image/atlas stores.
fn app(assets: bool) -> App {
    let data = GameData::load(Path::new(env!("CARGO_MANIFEST_DIR")))
        .unwrap_or_else(|e| panic!("game data failed to load: {e}"));
    let mut app = App::new();
    dark_castle_rpg::input::use_legacy_controls(&mut app);
    app.add_plugins((MinimalPlugins, StatesPlugin, BevyInputPlugin));
    if assets {
        app.add_plugins(AssetPlugin::default())
            .init_asset::<Image>()
            .init_asset::<TextureAtlasLayout>();
    }
    app.insert_resource(TimeUpdateStrategy::ManualDuration(FRAME))
        .insert_resource(data)
        .init_resource::<PendingKeys>()
        .init_resource::<PendingActivations>()
        .add_systems(Update, emit_pending_activations.in_set(GameSet::Commands))
        .add_systems(PreUpdate, apply_pending_keys.after(InputSystems))
        .add_plugins(GamePlugin);
    app
}

/// Puts an image under every atlas handle, as if the PNGs had loaded.
fn mark_textures_loaded(app: &mut App) {
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
            .unwrap_or_else(|e| panic!("cannot insert test image: {e}"));
    }
}

fn loaded_app() -> App {
    let mut app = app(true);
    app.update();
    mark_textures_loaded(&mut app);
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

fn player(app: &mut App) -> Entity {
    let world = app.world_mut();
    world
        .query_filtered::<Entity, With<Player>>()
        .single(world)
        .unwrap_or_else(|e| panic!("player missing: {e}"))
}

fn state(app: &mut App) -> PlayerState {
    let world = app.world_mut();
    world
        .query_filtered::<&ActionState, With<Player>>()
        .single(world)
        .map(|s| s.state())
        .unwrap_or_else(|e| panic!("player missing: {e}"))
}

struct Seen {
    count: usize,
    parent: Entity,
    clip: String,
    index: usize,
    flip: bool,
    anchor: Vec2,
    visible: bool,
    color: Color,
    offset: Vec3,
}

fn visual(app: &mut App) -> Seen {
    let world = app.world_mut();
    let mut q = world.query_filtered::<(
        &ChildOf,
        &VisualPlayback,
        &Sprite,
        &Anchor,
        &Visibility,
        &Transform,
    ), With<PlayerVisual>>();
    let all: Vec<_> = q.iter(world).collect();
    let count = all.len();
    let (parent, playback, sprite, anchor, visibility, transform) = all
        .first()
        .copied()
        .unwrap_or_else(|| panic!("no player visual"));
    Seen {
        count,
        parent: parent.parent(),
        clip: playback.clip.clone().unwrap_or_default(),
        index: sprite
            .texture_atlas
            .as_ref()
            .map_or(usize::MAX, |a| a.index),
        flip: sprite.flip_x,
        anchor: anchor.0,
        visible: *visibility != Visibility::Hidden,
        color: sprite.color,
        offset: transform.translation,
    }
}

fn visual_count(app: &mut App) -> usize {
    let world = app.world_mut();
    world
        .query_filtered::<(), With<PlayerVisual>>()
        .iter(world)
        .count()
}

fn box_alpha(app: &mut App) -> f32 {
    let world = app.world_mut();
    world
        .query_filtered::<&Sprite, With<Player>>()
        .single(world)
        .map(|s| s.color.alpha())
        .unwrap_or_else(|e| panic!("player missing: {e}"))
}

fn first_of(app: &App, clip: &str) -> usize {
    app.world().resource::<PlayerSpriteSet>().manifest.clips[clip].first as usize
}

#[test]
fn headless_app_without_asset_server_keeps_the_placeholder() {
    let mut app = app(false);
    run(&mut app, 20);
    assert_eq!(visual_count(&mut app), 0);
    assert!(app.world().get_resource::<PlayerSpriteSet>().is_none());
    assert!(box_alpha(&mut app) > 0.0, "the coloured box stays visible");
}

#[test]
fn one_visual_is_attached_and_waits_for_the_textures() {
    let mut app = app(true);
    run(&mut app, 20);
    let seen = visual(&mut app);
    assert_eq!(seen.count, 1);
    assert_eq!(seen.parent, player(&mut app));
    assert!(!seen.visible, "hidden while the textures are not loaded");
    assert!(box_alpha(&mut app) > 0.0, "the box covers until then");
    mark_textures_loaded(&mut app);
    run(&mut app, 1);
    let seen = visual(&mut app);
    assert!(seen.visible);
    assert_eq!(box_alpha(&mut app), 0.0, "box hidden in the same frame");
    // Feet on the floor: the pivot sits at the bottom of the 26x46 body.
    assert_eq!(seen.offset.y, -PLAYER_SIZE.1 / 2.0);
    run(&mut app, 200);
    assert_eq!(visual_count(&mut app), 1, "never duplicated");
}

#[test]
fn idle_run_and_facing() {
    let mut app = loaded_app();
    assert_eq!(visual(&mut app).clip, "idle");
    press(&mut app, KeyCode::ArrowRight);
    run(&mut app, 20);
    assert!(matches!(state(&mut app), PlayerState::Walking));
    let seen = visual(&mut app);
    assert_eq!((seen.clip.as_str(), seen.flip), ("run", false));
    let frames: std::collections::BTreeSet<usize> = (0..12)
        .map(|_| {
            run(&mut app, 1);
            visual(&mut app).index
        })
        .collect();
    assert!(frames.len() > 3, "the run cycle advances: {frames:?}");
    release(&mut app, KeyCode::ArrowRight);
    press(&mut app, KeyCode::ArrowLeft);
    run(&mut app, 5);
    let seen = visual(&mut app);
    assert!(seen.flip, "mirrored when facing left");
    release(&mut app, KeyCode::ArrowLeft);
    run(&mut app, 10);
    let seen = visual(&mut app);
    assert_eq!(seen.clip, "idle");
    assert!(seen.flip, "keeps facing left at rest");
}

/// Presses `key` once and records (state, clip, index) every frame until the
/// action ends.
fn action_trace(app: &mut App, key: KeyCode) -> Vec<(PlayerState, String, usize)> {
    press(app, key);
    let mut trace = Vec::new();
    for _ in 0..40 {
        run(app, 1);
        release(app, key);
        let s = state(app);
        let seen = visual(app);
        trace.push((s, seen.clip, seen.index));
    }
    trace
}

fn check_action(app: &mut App, key: KeyCode, wanted: PlayerState, clip: &str, seconds: f32) {
    let trace = action_trace(app, key);
    let active: Vec<_> = trace.iter().filter(|(s, _, _)| *s == wanted).collect();
    // The gameplay timer is untouched: the action lasts its own time.
    let frames = (seconds / FRAME.as_secs_f32()).ceil() as usize;
    assert!(
        active.len().abs_diff(frames) <= 1,
        "{clip}: {} frames in {wanted:?}, expected ~{frames}",
        active.len()
    );
    // The visual shows the clip exactly while the state lasts, from its
    // impact frame (frame 0) onwards, never going back.
    let first = first_of(app, clip);
    assert!(active.iter().all(|(_, c, _)| c == clip));
    assert_eq!(active[0].2, first, "{clip} starts on its impact frame");
    assert!(active.windows(2).all(|w| w[0].2 <= w[1].2));
    assert_eq!(
        active.last().map(|a| a.2),
        Some(first + 6),
        "{clip} reaches its last frame"
    );
    let after = trace
        .iter()
        .skip_while(|(s, _, _)| *s != wanted)
        .find(|(s, _, _)| *s != wanted)
        .unwrap_or_else(|| panic!("{clip} never ended"));
    assert_eq!(after.1, "idle", "back to idle with the gameplay state");
}

#[test]
fn basic_attack_and_dodge_follow_the_gameplay_timers() {
    let mut app = loaded_app();
    check_action(
        &mut app,
        KeyCode::KeyX,
        PlayerState::Attacking,
        "attack_basic",
        BASIC_ATTACK_TIME,
    );
    run(&mut app, 30);
    check_action(
        &mut app,
        KeyCode::KeyS,
        PlayerState::Dodging,
        "dodge",
        PLAYER_DODGE_TIME,
    );
}

#[test]
fn combat_cell_anchor_mirrors_with_the_sprite() {
    let mut app = loaded_app();
    let (pivot, cell) = {
        let set = app.world().resource::<PlayerSpriteSet>();
        let clip = &set.manifest.clips["attack_basic"];
        (clip.pivot, set.manifest.atlases[&clip.atlas].cell)
    };
    press(&mut app, KeyCode::ArrowLeft);
    run(&mut app, 3);
    release(&mut app, KeyCode::ArrowLeft);
    run(&mut app, 10);
    press(&mut app, KeyCode::KeyX);
    run(&mut app, 2);
    release(&mut app, KeyCode::KeyX);
    let seen = visual(&mut app);
    assert_eq!(seen.clip, "attack_basic");
    let (ax, ay) = anchor_for(pivot, cell, true);
    assert_eq!(seen.anchor, Vec2::new(ax, ay));
    assert!(
        seen.anchor.x > 0.0,
        "left-facing combat pivot is right of centre"
    );
    // Idle and attack keep the same sole point: same transform, anchors on the sole line.
    assert_eq!(seen.offset.y, -PLAYER_SIZE.1 / 2.0);
}

/// Fallback path (V1.3 test, adapted in V1.4): since V1.4 every state has a
/// clip, so the "state without sprites" case is produced by removing the
/// death and fall clips from the loaded manifest, as an older art export
/// without them would be.
#[test]
fn states_without_sprites_hold_idle_with_a_tint_and_respawn_returns() {
    let mut app = loaded_app();
    {
        let mut set = app.world_mut().resource_mut::<PlayerSpriteSet>();
        set.manifest.clips.remove("death");
        set.manifest.clips.remove("fall");
    }
    let entity = player(&mut app);
    app.world_mut()
        .get_mut::<ActionState>(entity)
        .unwrap_or_else(|| panic!("player state missing"))
        .die();
    run(&mut app, 2);
    let seen = visual(&mut app);
    assert_eq!(
        (seen.clip.as_str(), seen.index),
        ("idle", first_of(&app, "idle"))
    );
    assert!(seen.color.to_srgba().red < 0.5, "dead tint");
    press(&mut app, KeyCode::KeyR);
    run(&mut app, 2);
    release(&mut app, KeyCode::KeyR);
    run(&mut app, 2);
    // Respawn puts the player at the room's spawn point, in the air: the
    // visual follows (no fall clip here: idle frame, no dead tint).
    assert_ne!(state(&mut app), PlayerState::Dead);
    let seen = visual(&mut app);
    assert_eq!(seen.clip, "idle");
    assert!(seen.color.to_srgba().red > 0.9, "dead tint removed");
    run(&mut app, 120);
    assert_eq!(state(&mut app), PlayerState::Idle, "landed");
    let seen = visual(&mut app);
    assert_eq!(seen.clip, "idle");
    assert_eq!(seen.count, 1, "respawn does not duplicate the visual");
}

/// Same inputs with and without the sprite visual: identical gameplay.
#[test]
fn gameplay_is_identical_with_and_without_sprites() {
    let script: &[(usize, KeyCode, bool)] = &[
        (10, KeyCode::ArrowRight, true),
        (40, KeyCode::KeyZ, true),
        (46, KeyCode::KeyZ, false),
        (80, KeyCode::ArrowRight, false),
        (90, KeyCode::KeyX, true),
        (92, KeyCode::KeyX, false),
        (130, KeyCode::KeyS, true),
        (132, KeyCode::KeyS, false),
        (160, KeyCode::ArrowLeft, true),
        (161, KeyCode::ShiftLeft, true),
        (200, KeyCode::ShiftLeft, false),
        (200, KeyCode::ArrowLeft, false),
    ];
    let trace = |app: &mut App| {
        let mut out = Vec::new();
        for frame in 0..240 {
            for (at, key, down) in script {
                if *at == frame {
                    if *down {
                        press(app, *key);
                    } else {
                        release(app, *key);
                    }
                }
            }
            run(app, 1);
            let world = app.world_mut();
            let (t, s) = world
                .query_filtered::<(&Transform, &ActionState), With<Player>>()
                .single(world)
                .map(|(t, s)| (t.translation, s.state()))
                .unwrap_or_else(|e| panic!("player missing: {e}"));
            out.push((t.x.to_bits(), t.y.to_bits(), s));
        }
        out
    };
    let mut plain = app(false);
    let mut with_sprites = app(true);
    with_sprites.update();
    plain.update();
    mark_textures_loaded(&mut with_sprites);
    let a = trace(&mut plain);
    let b = trace(&mut with_sprites);
    assert_eq!(visual_count(&mut with_sprites), 1);
    assert_eq!(a, b, "position and state must not depend on the visual");
}

#[test]
fn quick_load_keeps_one_visual_matching_the_state() {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("visual_isolated_saves");
    let storage = SaveStorage::new(&dir);
    for path in [
        storage.slot_path(DEFAULT_SAVE_SLOT),
        storage.backup_path(DEFAULT_SAVE_SLOT),
    ] {
        if path.exists() {
            std::fs::remove_file(&path)
                .unwrap_or_else(|e| panic!("cannot reset {}: {e}", path.display()));
        }
    }
    let mut app = loaded_app();
    app.insert_resource(SaveSlots(storage));
    press(&mut app, KeyCode::F5);
    run(&mut app, 1);
    release(&mut app, KeyCode::F5);
    press(&mut app, KeyCode::ArrowRight);
    run(&mut app, 20);
    release(&mut app, KeyCode::ArrowRight);
    press(&mut app, KeyCode::F9);
    run(&mut app, 1);
    release(&mut app, KeyCode::F9);
    run(&mut app, 5);
    let seen = visual(&mut app);
    assert_eq!(seen.count, 1);
    assert_eq!(seen.parent, player(&mut app));
    assert_eq!(seen.clip, "idle", "matches the loaded (idle) state");
    assert!(seen.visible);
}

// ------------------------------------------------------------------ V1.4

fn clip_frames(app: &App, clip: &str) -> (usize, usize) {
    let c = &app.world().resource::<PlayerSpriteSet>().manifest.clips[clip];
    (c.first as usize, c.count as usize)
}

fn with_state(app: &mut App, f: impl FnOnce(&mut ActionState)) {
    let entity = player(app);
    let mut state = app
        .world_mut()
        .get_mut::<ActionState>(entity)
        .unwrap_or_else(|| panic!("player state missing"));
    f(&mut state);
}

/// Runs frames and records (state, clip, index) until `done` or `max` frames.
fn trace_until(
    app: &mut App,
    max: usize,
    done: impl Fn(PlayerState) -> bool,
) -> Vec<(PlayerState, String, usize)> {
    let mut out = Vec::new();
    for _ in 0..max {
        run(app, 1);
        let s = state(app);
        let seen = visual(app);
        out.push((s, seen.clip, seen.index));
        if done(s) {
            break;
        }
    }
    out
}

/// A timed clip: starts on frame 0, never goes back, reaches its last frame,
/// and lasts as long as the gameplay state (± one 16 ms frame).
fn check_timed(
    app: &App,
    trace: &[(PlayerState, String, usize)],
    wanted: PlayerState,
    clip: &str,
    seconds: f32,
) {
    let (first, count) = clip_frames(app, clip);
    let active: Vec<_> = trace.iter().filter(|(s, _, _)| *s == wanted).collect();
    let frames = (seconds / FRAME.as_secs_f32()).ceil() as usize;
    assert!(
        active.len().abs_diff(frames) <= 1,
        "{clip}: {} frames, expected ~{frames}",
        active.len()
    );
    assert!(
        active.iter().all(|(_, c, _)| c == clip),
        "{clip}: {active:?}"
    );
    assert_eq!(active[0].2, first, "{clip} starts on its impact frame");
    assert!(
        active.windows(2).all(|w| w[0].2 <= w[1].2),
        "{clip} never goes back"
    );
    assert_eq!(
        active.last().map(|a| a.2),
        Some(first + count - 1),
        "{clip} reaches its last frame"
    );
}

#[test]
fn hurt_follows_the_gameplay_stagger() {
    let mut app = loaded_app();
    with_state(&mut app, |s| {
        assert!(s.hurt(dark_castle_rpg::core::constants::PLAYER_HURT_TIME));
    });
    let trace = trace_until(&mut app, 60, |s| s != PlayerState::Hurt);
    // The first traced frame already ran one frame of the stagger.
    let seen: Vec<_> = trace.iter().filter(|t| t.0 == PlayerState::Hurt).collect();
    assert!(seen.iter().all(|t| t.1 == "hurt"));
    let (first, count) = clip_frames(&app, "hurt");
    assert_eq!(seen.last().map(|t| t.2), Some(first + count - 1));
    assert!(seen.windows(2).all(|w| w[0].2 <= w[1].2));
    assert_ne!(
        trace.last().map(|t| t.1.as_str()),
        Some("hurt"),
        "returns with the state"
    );
}

#[test]
fn attack_interrupted_by_hurt_shows_hurt() {
    let mut app = loaded_app();
    press(&mut app, KeyCode::KeyX);
    run(&mut app, 3);
    release(&mut app, KeyCode::KeyX);
    assert_eq!(visual(&mut app).clip, "attack_basic");
    with_state(&mut app, |s| {
        assert!(s.hurt(0.35));
    });
    run(&mut app, 1);
    let seen = visual(&mut app);
    assert_eq!(
        (seen.clip.as_str(), seen.index),
        ("hurt", clip_frames(&app, "hurt").0)
    );
}

#[test]
fn death_plays_once_holds_and_respawn_returns() {
    let mut app = loaded_app();
    with_state(&mut app, |s| s.die());
    run(&mut app, 1);
    let (first, count) = clip_frames(&app, "death");
    assert_eq!(
        (visual(&mut app).clip, visual(&mut app).index),
        ("death".into(), first)
    );
    run(&mut app, 120);
    let seen = visual(&mut app);
    assert_eq!(
        (seen.clip.as_str(), seen.index),
        ("death", first + count - 1),
        "holds the final pose"
    );
    assert!(
        seen.color.to_srgba().red > 0.9,
        "no tint: death has its own clip"
    );
    press(&mut app, KeyCode::KeyR);
    run(&mut app, 2);
    release(&mut app, KeyCode::KeyR);
    run(&mut app, 2);
    assert_ne!(visual(&mut app).clip, "death");
    run(&mut app, 120);
    let seen = visual(&mut app);
    assert_eq!((seen.clip.as_str(), seen.count), ("idle", 1));
}

#[test]
fn jump_fall_and_land_follow_the_physics() {
    let mut app = loaded_app();
    press(&mut app, KeyCode::KeyZ);
    let trace = trace_until(&mut app, 200, |_| false);
    release(&mut app, KeyCode::KeyZ);
    for (s, clip, _) in &trace {
        let ok = match s {
            PlayerState::Jumping => clip == "jump_rise",
            PlayerState::Falling => clip == "fall",
            PlayerState::Idle => clip == "land" || clip == "idle",
            _ => true,
        };
        assert!(ok, "{s:?} shown as {clip}");
    }
    let states: Vec<_> = trace.iter().map(|t| t.0).collect();
    assert!(states.contains(&PlayerState::Jumping) && states.contains(&PlayerState::Falling));
    // Right after the landing the land clip plays once, then idle.
    let landed = trace
        .iter()
        .position(|t| t.0 == PlayerState::Idle)
        .unwrap_or_else(|| panic!("never landed"));
    assert_eq!(trace[landed].1, "land");
    let (first, count) = clip_frames(&app, "land");
    assert_eq!(trace[landed].2, first);
    assert_eq!(trace.last().map(|t| t.1.as_str()), Some("idle"));
    let lands = trace.iter().filter(|t| t.1 == "land").count();
    assert!(
        lands >= count && lands <= count * 3,
        "land lasts about its 0.167 s ({lands} frames)"
    );
}

#[test]
fn air_attack_shows_the_attack() {
    let mut app = loaded_app();
    press(&mut app, KeyCode::KeyZ);
    run(&mut app, 6);
    assert_eq!(state(&mut app), PlayerState::Jumping);
    press(&mut app, KeyCode::KeyX);
    run(&mut app, 1);
    release(&mut app, KeyCode::KeyX);
    release(&mut app, KeyCode::KeyZ);
    assert_eq!(state(&mut app), PlayerState::Attacking);
    // V1.5: the attack still wins over the jump, now with its air variant
    // (V1.4 reused the ground clip; tests/player_vfx_tests.rs checks the rest).
    assert_eq!(
        visual(&mut app).clip,
        "attack_basic_air",
        "attack wins over jump"
    );
}

#[test]
fn crouch_and_guard_hold_their_pose() {
    let mut app = loaded_app();
    press(&mut app, KeyCode::ArrowDown);
    run(&mut app, 30);
    assert_eq!(state(&mut app), PlayerState::Crouching);
    let (first, count) = clip_frames(&app, "crouch");
    let seen = visual(&mut app);
    assert_eq!(
        (seen.clip.as_str(), seen.index),
        ("crouch", first + count - 1)
    );
    assert_eq!(
        seen.offset.y,
        -PLAYER_SIZE.1 / 2.0,
        "sole still on the body bottom"
    );
    release(&mut app, KeyCode::ArrowDown);
    run(&mut app, 5);
    press(&mut app, KeyCode::KeyA);
    run(&mut app, 30);
    assert_eq!(state(&mut app), PlayerState::Guarding);
    let (first, count) = clip_frames(&app, "guard_start");
    let seen = visual(&mut app);
    assert_eq!(
        (seen.clip.as_str(), seen.index),
        ("guard_start", first + count - 1)
    );
    release(&mut app, KeyCode::KeyA);
    run(&mut app, 3);
    assert_eq!(visual(&mut app).clip, "idle");
}

fn grant_all_skills(app: &mut App) {
    let defs: Vec<_> = app
        .world()
        .resource::<GameData>()
        .skills
        .iter()
        .cloned()
        .collect();
    let entity = player(app);
    let mut book = app
        .world_mut()
        .get_mut::<SkillBook>(entity)
        .unwrap_or_else(|| panic!("skill book missing"));
    for def in &defs {
        book.learned.grant(def);
    }
}

#[test]
fn each_skill_shows_its_own_clip_for_its_whole_duration() {
    let mut app = loaded_app();
    grant_all_skills(&mut app);
    let skills: Vec<(String, String, f32)> = app
        .world()
        .resource::<GameData>()
        .skills
        .iter()
        .map(|d| (d.id.clone(), d.animation.clone(), d.execution_time))
        .collect();
    assert_eq!(skills.len(), 4);
    for (id, clip, seconds) in skills {
        app.world_mut()
            .resource_mut::<PendingActivations>()
            .0
            .push(id.clone());
        let trace = trace_until(&mut app, 80, |s| s != PlayerState::Casting);
        let casting: Vec<_> = trace
            .iter()
            .take_while(|t| t.0 == PlayerState::Casting)
            .cloned()
            .collect();
        assert!(!casting.is_empty(), "{id} did not start");
        check_timed(&app, &casting, PlayerState::Casting, &clip, seconds);
        run(&mut app, 90); // cooldowns and recovery
    }
}

#[test]
fn death_during_a_skill_wins() {
    let mut app = loaded_app();
    app.world_mut()
        .resource_mut::<PendingActivations>()
        .0
        .push("crescent_slash".into());
    run(&mut app, 3);
    assert_eq!(visual(&mut app).clip, "slash_crescent");
    with_state(&mut app, |s| s.die());
    run(&mut app, 1);
    assert_eq!(visual(&mut app).clip, "death");
}

#[test]
fn quick_load_while_falling_is_not_a_landing() {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("visual_land_saves");
    let storage = SaveStorage::new(&dir);
    for path in [
        storage.slot_path(DEFAULT_SAVE_SLOT),
        storage.backup_path(DEFAULT_SAVE_SLOT),
    ] {
        if path.exists() {
            std::fs::remove_file(&path)
                .unwrap_or_else(|e| panic!("cannot reset {}: {e}", path.display()));
        }
    }
    let mut app = loaded_app();
    app.insert_resource(SaveSlots(storage));
    press(&mut app, KeyCode::F5);
    run(&mut app, 1);
    release(&mut app, KeyCode::F5);
    // Lift the player: it falls (a teleport upwards is not a landing either).
    let entity = player(&mut app);
    if let Some(mut t) = app.world_mut().get_mut::<Transform>(entity) {
        t.translation.y += 150.0;
    }
    run(&mut app, 8);
    assert_eq!(state(&mut app), PlayerState::Falling);
    press(&mut app, KeyCode::F9);
    let trace = trace_until(&mut app, 12, |_| false);
    release(&mut app, KeyCode::F9);
    assert!(
        trace.iter().any(|t| t.0 == PlayerState::Idle),
        "back on the floor after loading"
    );
    assert!(
        trace.iter().all(|t| t.1 != "land"),
        "no landing after a quick load: {trace:?}"
    );
}
