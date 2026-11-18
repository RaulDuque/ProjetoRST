//! Headless game with the world graph active, as `app::run` builds it
//! (V1.7). Shared by the V1.7 integration and stress tests. Saves go to a
//! directory under the target dir; `saves/` is never touched.

#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::time::Duration;

use bevy::input::{InputPlugin as BevyInputPlugin, InputSystems};
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::app::load_world;
use dark_castle_rpg::app::plugins::GamePlugin;
use dark_castle_rpg::core::constants::DEFAULT_SAVE_SLOT;
use dark_castle_rpg::core::resources::GameData;
use dark_castle_rpg::player::components::{ActionState, Player, Vitals};
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::save::systems::SaveSlots;
use dark_castle_rpg::world::exploration::ExplorationState;
use dark_castle_rpg::world::exploration_systems::{Exploration, activate_world};
use dark_castle_rpg::world::graph::{Side, WorldDefinition};
use dark_castle_rpg::world::maps::CurrentRoom;
use dark_castle_rpg::world::physics::Velocity;

pub const FRAME: Duration = Duration::from_millis(16);
pub const APPROACH: &str = "vigil_gates_approach";
pub const CAUSEWAY: &str = "vigil_gates_causeway";
pub const SHAFT: &str = "vigil_gates_shaft";
pub const BARBICAN: &str = "vigil_gates_barbican";
pub const CACHE: &str = "vigil_gates_hidden_cache";
pub const GALLERY: &str = "vigil_gates_gallery";
pub const POSTERN: &str = "vigil_gates_postern";
pub const SEALED: &str = "vigil_gates_sealed_hall";
pub const RELIQUARY: &str = "vigil_gates_reliquary";

pub fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

pub fn world() -> WorldDefinition {
    WorldDefinition::load_dir(&root().join("assets/world")).unwrap_or_else(|e| panic!("{e}"))
}

/// Key presses applied right after the input systems (`just_pressed` seen).
#[derive(Resource, Default)]
pub struct PendingPress(pub Vec<KeyCode>);

fn apply_presses(mut pending: ResMut<PendingPress>, mut keys: ResMut<ButtonInput<KeyCode>>) {
    for key in pending.0.drain(..) {
        keys.release(key);
        keys.press(key);
    }
}

/// A fresh, empty save directory under the target dir.
pub fn storage(name: &str) -> SaveStorage {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = std::fs::remove_dir_all(&dir);
    SaveStorage::new(dir)
}

/// The shipped game (world loaded, validated, installed, activated).
pub fn game(saves: &SaveStorage) -> App {
    let mut data = GameData::load(root()).unwrap_or_else(|e| panic!("{e}"));
    let world = load_world(root(), &mut data).unwrap_or_else(|e| panic!("{e}"));
    let mut app = App::new();
    dark_castle_rpg::input::use_legacy_controls(&mut app);
    app.add_plugins((MinimalPlugins, StatesPlugin, BevyInputPlugin))
        .insert_resource(TimeUpdateStrategy::ManualDuration(FRAME))
        .insert_resource(data)
        .add_plugins(GamePlugin)
        .insert_resource(SaveSlots(saves.clone()))
        .init_resource::<PendingPress>()
        .add_systems(PreUpdate, apply_presses.after(InputSystems));
    activate_world(&mut app, world);
    app.update();
    run(&mut app, 3);
    app
}

pub fn run(app: &mut App, frames: usize) {
    for _ in 0..frames {
        app.update();
    }
}

/// Presses `key` during the next frame (and releases it afterwards).
pub fn tap(app: &mut App, key: KeyCode) {
    app.world_mut().resource_mut::<PendingPress>().0.push(key);
    run(app, 1);
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .release(key);
}

pub fn current(app: &App) -> String {
    app.world().resource::<CurrentRoom>().0.clone()
}

pub fn exploration(app: &App) -> ExplorationState {
    app.world().resource::<Exploration>().0.clone()
}

pub fn count<F: bevy::ecs::query::QueryFilter>(app: &mut App) -> usize {
    let world = app.world_mut();
    world.query_filtered::<(), F>().iter(world).count()
}

pub fn player_xy(app: &mut App) -> Vec2 {
    let world = app.world_mut();
    world
        .query_filtered::<&Transform, With<Player>>()
        .single(world)
        .map(|t| t.translation.truncate())
        .unwrap_or_else(|e| panic!("{e}"))
}

pub fn player_hp(app: &mut App) -> i32 {
    let world = app.world_mut();
    world
        .query_filtered::<&Vitals, With<Player>>()
        .single(world)
        .map(|v| v.hp.current())
        .unwrap_or_else(|e| panic!("{e}"))
}

pub fn with_player_state(app: &mut App, f: impl FnOnce(&mut ActionState)) {
    let world = app.world_mut();
    let mut q = world.query_filtered::<&mut ActionState, With<Player>>();
    let mut state = q.single_mut(world).unwrap_or_else(|e| panic!("{e}"));
    f(&mut state);
}

/// Test shortcut (not used by the route test): places the player.
pub fn place(app: &mut App, p: (f32, f32)) {
    let world = app.world_mut();
    let mut q = world.query_filtered::<(&mut Transform, &mut Velocity), With<Player>>();
    let (mut t, mut v) = q.single_mut(world).unwrap_or_else(|e| panic!("{e}"));
    t.translation.x = p.0;
    t.translation.y = p.1;
    v.0 = Vec2::ZERO;
}

/// The point on `door`'s trigger in the current room.
pub fn threshold(app: &App, door: &str) -> (f32, f32) {
    let world = world();
    let room = world
        .room(&current(app))
        .unwrap_or_else(|| panic!("no room"))
        .clone();
    let d = room
        .door(door)
        .unwrap_or_else(|| panic!("no door {door}"))
        .clone();
    let o = room.opening(&d);
    let (w, h) = room.size();
    match d.side {
        Side::Left => (8.0, o.min.1 + 24.0),
        Side::Right => (w - 8.0, o.min.1 + 24.0),
        Side::Bottom => (o.min.0 + o.size.0 / 2.0, 8.0),
        Side::Top => (o.min.0 + o.size.0 / 2.0, h - 8.0),
    }
}

/// Test shortcut: puts the player on `door`'s trigger and runs a frame.
pub fn go_through(app: &mut App, door: &str) {
    let p = threshold(app, door);
    place(app, p);
    run(app, 1);
}

pub fn walk(app: &mut App, route: &[(&str, &str)]) {
    for (door, arrives) in route {
        go_through(app, door);
        assert_eq!(current(app), *arrives, "through {door}");
    }
}

pub fn slot_bytes(saves: &SaveStorage) -> Option<Vec<u8>> {
    std::fs::read(saves.slot_path(DEFAULT_SAVE_SLOT)).ok()
}
