//! Headless smoke tests of the real ECS wiring (no window, no renderer).
//!
//! They catch what unit tests cannot: conflicting queries, missing
//! resources or messages, and broken system ordering.

use std::path::{Path, PathBuf};
use std::time::Duration;

use bevy::input::{InputPlugin as BevyInputPlugin, InputSystems};
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::app::plugins::GamePlugin;
use dark_castle_rpg::app::states::GameSet;
use dark_castle_rpg::combat::damage::DamageOutcome;
use dark_castle_rpg::combat::hitboxes::Invulnerable;
use dark_castle_rpg::core::constants::{DEFAULT_SAVE_SLOT, SAVE_DIR};
use dark_castle_rpg::core::events::DamageDealt;
use dark_castle_rpg::core::resources::GameData;
use dark_castle_rpg::enemies::components::Enemy;
use dark_castle_rpg::magic::elements::Element;
use dark_castle_rpg::magic::projectiles::Projectile;
use dark_castle_rpg::player::components::{ActionState, Player, SkillBook, Vitals};
use dark_castle_rpg::player::state::PlayerState;
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::save::systems::SaveSlots;
use dark_castle_rpg::world::physics::Grounded;

const FRAME: Duration = Duration::from_millis(16);

/// Key changes applied right after Bevy's input systems, exactly like a
/// real keyboard event would be (so `just_pressed` is visible this frame).
#[derive(Resource, Default)]
struct PendingKeys {
    press: Vec<KeyCode>,
    release: Vec<KeyCode>,
}

fn apply_pending_keys(mut pending: ResMut<PendingKeys>, mut keys: ResMut<ButtonInput<KeyCode>>) {
    // Releases first, so "release then press again" keeps the key held.
    for key in pending.release.drain(..) {
        keys.release(key);
    }
    for key in pending.press.drain(..) {
        keys.press(key);
    }
}

fn app() -> App {
    let data = GameData::load(Path::new(env!("CARGO_MANIFEST_DIR")))
        .unwrap_or_else(|e| panic!("game data failed to load: {e}"));
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatesPlugin, BevyInputPlugin))
        .insert_resource(TimeUpdateStrategy::ManualDuration(FRAME))
        .insert_resource(data)
        .init_resource::<PendingKeys>()
        .add_systems(PreUpdate, apply_pending_keys.after(InputSystems))
        .add_plugins(GamePlugin);
    app
}

fn run_frames(app: &mut App, frames: usize) {
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

/// Holds `keys` for `frames` frames, then releases them.
fn hold(app: &mut App, keys: &[KeyCode], frames: usize) {
    for key in keys {
        press(app, *key);
    }
    run_frames(app, frames);
    for key in keys {
        release(app, *key);
    }
}

fn player_query<D: bevy::ecs::query::QueryData>(app: &mut App) -> usize {
    let world = app.world_mut();
    world
        .query_filtered::<D, With<Player>>()
        .iter(world)
        .count()
}

#[test]
fn game_starts_and_runs_without_panicking() {
    let mut app = app();
    run_frames(&mut app, 120);
    assert_eq!(player_query::<Entity>(&mut app), 1);
    let world = app.world_mut();
    let enemies = world.query::<&Enemy>().iter(world).count();
    assert_eq!(enemies, 2);
}

#[test]
fn player_lands_on_the_floor() {
    let mut app = app();
    run_frames(&mut app, 90);
    let world = app.world_mut();
    let grounded = world
        .query_filtered::<&Grounded, With<Player>>()
        .single(world)
        .map(|g| g.on_ground)
        .unwrap_or(false);
    assert!(grounded);
}

#[test]
fn spell_command_spends_mp_and_spawns_a_projectile() {
    let mut app = app();
    run_frames(&mut app, 60);
    let mp_before = {
        let world = app.world_mut();
        world
            .query_filtered::<&Vitals, With<Player>>()
            .single(world)
            .map(|v| v.mp.current())
            .unwrap_or_default()
    };

    // ↓ ↘ → + MAGIC
    hold(&mut app, &[KeyCode::ArrowDown], 3);
    hold(&mut app, &[KeyCode::ArrowDown, KeyCode::ArrowRight], 3);
    press(&mut app, KeyCode::ArrowRight);
    run_frames(&mut app, 2);
    hold(&mut app, &[KeyCode::KeyC], 1);
    release(&mut app, KeyCode::ArrowRight);

    let world = app.world_mut();
    let projectiles = world.query::<&Projectile>().iter(world).count();
    let (mp_after, on_cooldown) = world
        .query_filtered::<(&Vitals, &SkillBook), With<Player>>()
        .single(world)
        .map(|(v, b)| (v.mp.current(), !b.cooldowns.is_ready("ember_bolt")))
        .unwrap_or_default();
    assert_eq!(projectiles, 1, "ember bolt should be in flight");
    assert_eq!(mp_before - mp_after, 8);
    assert!(on_cooldown);
}

/// Contents of the real slot files (`None` = file absent).
fn real_save_snapshot() -> Vec<Option<Vec<u8>>> {
    let storage = SaveStorage::new(Path::new(env!("CARGO_MANIFEST_DIR")).join(SAVE_DIR));
    [
        storage.slot_path(DEFAULT_SAVE_SLOT),
        storage.backup_path(DEFAULT_SAVE_SLOT),
    ]
    .iter()
    .map(|path| std::fs::read(path).ok())
    .collect()
}

#[test]
fn quick_save_and_load_use_the_configured_save_dir() {
    let real_before = real_save_snapshot();
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("smoke_isolated_saves");
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

    let mut app = app();
    app.insert_resource(SaveSlots(storage.clone()));
    run_frames(&mut app, 60);

    // F5 writes to the configured directory.
    hold(&mut app, &[KeyCode::F5], 1);
    run_frames(&mut app, 1);
    let saved = storage
        .load(DEFAULT_SAVE_SLOT)
        .unwrap_or_else(|e| panic!("isolated save missing or invalid: {e}"));

    // Move the player away, then F9 must read the configured directory back.
    {
        let world = app.world_mut();
        let mut transform = world
            .query_filtered::<&mut Transform, With<Player>>()
            .single_mut(world)
            .unwrap_or_else(|e| panic!("player missing: {e}"));
        transform.translation.x += 100.0;
    }
    hold(&mut app, &[KeyCode::F9], 1);
    let world = app.world_mut();
    let x = world
        .query_filtered::<&Transform, With<Player>>()
        .single(world)
        .map(|t| t.translation.x)
        .unwrap_or_else(|e| panic!("player missing: {e}"));
    assert_eq!(
        x, saved.player.position.0,
        "F9 should restore the isolated save"
    );

    assert_eq!(
        real_save_snapshot(),
        real_before,
        "the real saves/ directory must not be touched"
    );
}

// ---------------------------------------------------------------- A3
// Several damage messages for the player in one frame must respect the
// invulnerability started by the first valid hit (audit A3). Messages are
// injected directly so the order of the hits is deterministic. The player
// stays at the spawn point, far from the enemies; natural HP regeneration
// is below 1 point during these short tests, so HP values are exact.

fn player_hp(app: &mut App) -> i32 {
    let world = app.world_mut();
    world
        .query_filtered::<&Vitals, With<Player>>()
        .single(world)
        .map(|v| v.hp.current())
        .unwrap_or_else(|e| panic!("player missing: {e}"))
}

fn player_state(app: &mut App) -> PlayerState {
    let world = app.world_mut();
    world
        .query_filtered::<&ActionState, With<Player>>()
        .single(world)
        .map(|s| s.0.state())
        .unwrap_or_else(|e| panic!("player missing: {e}"))
}

fn player_invulnerability(app: &mut App) -> f32 {
    let world = app.world_mut();
    world
        .query_filtered::<&Invulnerable, With<Player>>()
        .single(world)
        .map(|i| i.remaining)
        .unwrap_or_else(|e| panic!("player missing: {e}"))
}

fn hit(amount: i32) -> DamageOutcome {
    DamageOutcome::Hit {
        amount,
        critical: false,
        element: Element::Physical,
    }
}

/// Damage queued by a test, emitted on the next frame inside `GameSet::Combat`
/// (where `detect_hits` and `contact_damage` produce real hits), so the
/// messages live exactly as long as real ones do.
#[derive(Resource, Default)]
struct PendingDamage(Vec<DamageOutcome>);

fn emit_pending_damage(
    mut pending: ResMut<PendingDamage>,
    player: Query<Entity, With<Player>>,
    mut damage: MessageWriter<DamageDealt>,
) {
    let Ok(player) = player.single() else {
        return;
    };
    for outcome in pending.0.drain(..) {
        damage.write(DamageDealt {
            attacker: player,
            target: player,
            outcome,
            direction: 1.0,
        });
    }
}

/// Queues one `DamageDealt` per outcome against the player, in order.
fn inject_damage(app: &mut App, outcomes: &[DamageOutcome]) {
    app.world_mut()
        .resource_mut::<PendingDamage>()
        .0
        .extend_from_slice(outcomes);
}

/// App after the warm-up: player standing at the spawn with full HP.
fn idle_app() -> App {
    let mut app = app();
    app.init_resource::<PendingDamage>()
        .add_systems(Update, emit_pending_damage.in_set(GameSet::Combat));
    run_frames(&mut app, 60);
    app
}

#[test]
fn two_hits_in_one_frame_apply_damage_once() {
    let mut app = idle_app();
    let before = player_hp(&mut app);
    inject_damage(&mut app, &[hit(10), hit(10)]);
    run_frames(&mut app, 1);
    assert_eq!(player_hp(&mut app), before - 10);
    assert_eq!(player_state(&mut app), PlayerState::Hurt);
}

#[test]
fn three_hits_in_one_frame_apply_only_the_first() {
    let mut app = idle_app();
    let before = player_hp(&mut app);
    inject_damage(&mut app, &[hit(7), hit(10), hit(12)]);
    run_frames(&mut app, 1);
    assert_eq!(player_hp(&mut app), before - 7);
}

#[test]
fn active_invulnerability_blocks_damage_and_hurt() {
    let mut app = idle_app();
    let before = player_hp(&mut app);
    let state_before = player_state(&mut app);
    {
        let world = app.world_mut();
        let mut invulnerable = world
            .query_filtered::<&mut Invulnerable, With<Player>>()
            .single_mut(world)
            .unwrap_or_else(|e| panic!("player missing: {e}"));
        invulnerable.remaining = 0.5;
    }
    inject_damage(&mut app, &[hit(10)]);
    run_frames(&mut app, 1);
    assert_eq!(player_hp(&mut app), before);
    assert_eq!(player_state(&mut app), state_before);
}

#[test]
fn miss_then_hit_applies_the_hit() {
    let mut app = idle_app();
    let before = player_hp(&mut app);
    inject_damage(&mut app, &[DamageOutcome::Miss, hit(10)]);
    run_frames(&mut app, 1);
    assert_eq!(player_hp(&mut app), before - 10);
    assert!(player_invulnerability(&mut app) > 0.0);
}

#[test]
fn zero_damage_hit_does_not_consume_invulnerability() {
    let mut app = idle_app();
    let before = player_hp(&mut app);
    inject_damage(&mut app, &[hit(0), hit(10)]);
    run_frames(&mut app, 1);
    assert_eq!(player_hp(&mut app), before - 10);
}

#[test]
fn hit_then_miss_applies_the_hit_once() {
    let mut app = idle_app();
    let before = player_hp(&mut app);
    inject_damage(&mut app, &[hit(10), DamageOutcome::Miss]);
    run_frames(&mut app, 1);
    assert_eq!(player_hp(&mut app), before - 10);
    // The trailing Miss neither resets the protection nor the stagger.
    assert!(player_invulnerability(&mut app) > 0.0);
    assert_eq!(player_state(&mut app), PlayerState::Hurt);
}

#[test]
fn fatal_hit_leaves_no_pending_damage_for_the_respawn() {
    let mut app = idle_app();
    let lethal = player_hp(&mut app) + 50;
    inject_damage(&mut app, &[hit(lethal), hit(10), hit(10)]);
    run_frames(&mut app, 1);
    assert_eq!(player_state(&mut app), PlayerState::Dead);

    // Respawn on the very next frame: the hits queued behind the fatal one
    // must not land on the revived player.
    hold(&mut app, &[KeyCode::KeyR], 1);
    run_frames(&mut app, 2);
    let world = app.world_mut();
    let (hp, max) = world
        .query_filtered::<&Vitals, With<Player>>()
        .single(world)
        .map(|v| (v.hp.current(), v.hp.max()))
        .unwrap_or_else(|e| panic!("player missing: {e}"));
    assert_ne!(player_state(&mut app), PlayerState::Dead);
    assert_eq!(hp, max, "stale damage hit the revived player");
}

#[test]
fn hits_queued_behind_a_fatal_hit_are_discarded_that_frame() {
    // Isolates the drain on death from the respawn i-frames: the player is
    // revived directly (no `respawn`, no invulnerability), so any hit left
    // unread in the frame of the death would land on the next frame.
    let mut app = idle_app();
    let lethal = player_hp(&mut app) + 50;
    inject_damage(&mut app, &[hit(lethal), hit(10), hit(10)]);
    run_frames(&mut app, 1);
    assert_eq!(player_state(&mut app), PlayerState::Dead);
    let max = {
        let world = app.world_mut();
        let (mut state, mut vitals, mut invulnerable) = world
            .query_filtered::<(&mut ActionState, &mut Vitals, &mut Invulnerable), With<Player>>()
            .single_mut(world)
            .unwrap_or_else(|e| panic!("player missing: {e}"));
        state.0.revive();
        vitals.hp.refill();
        invulnerable.remaining = 0.0;
        vitals.hp.max()
    };
    run_frames(&mut app, 1);
    assert_eq!(
        player_hp(&mut app),
        max,
        "a hit queued behind the fatal one landed"
    );
}

#[test]
fn damage_applies_again_after_invulnerability_expires() {
    let mut app = idle_app();
    inject_damage(&mut app, &[hit(10)]);
    run_frames(&mut app, 1);
    // 60 frames x 16 ms = 0.96 s, longer than the 0.9 s of invulnerability.
    run_frames(&mut app, 60);
    assert_eq!(player_invulnerability(&mut app), 0.0);
    let before = player_hp(&mut app);
    inject_damage(&mut app, &[hit(10)]);
    run_frames(&mut app, 1);
    assert_eq!(player_hp(&mut app), before - 10);
}
