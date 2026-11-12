//! Headless smoke tests of the real ECS wiring (no window, no renderer).
//!
//! They catch what unit tests cannot: conflicting queries, missing
//! resources or messages, and broken system ordering.

use std::path::Path;
use std::time::Duration;

use bevy::input::{InputPlugin as BevyInputPlugin, InputSystems};
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::app::plugins::GamePlugin;
use dark_castle_rpg::core::resources::GameData;
use dark_castle_rpg::enemies::components::Enemy;
use dark_castle_rpg::magic::projectiles::Projectile;
use dark_castle_rpg::player::components::{Player, SkillBook, Vitals};
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
