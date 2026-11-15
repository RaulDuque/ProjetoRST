//! Headless exploration of the V1.6 vertical slice in the real game app.
//!
//! The app is the game (`GamePlugin`: player, physics, combat, enemies,
//! saves, and since V1.7 the world exploration), with the slice installed
//! in `GameData` and activated (`activate_world`), as `app::run` does. The player is moved by placing it on a door's
//! threshold (test shortcut); everything else runs the game's own systems.
//! Saves are redirected to a temporary directory and never written.

use std::path::{Path, PathBuf};
use std::time::Duration;

use bevy::input::{InputPlugin as BevyInputPlugin, InputSystems};
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::app::plugins::GamePlugin;
use dark_castle_rpg::core::resources::GameData;
use dark_castle_rpg::enemies::components::{Enemy, PendingRespawns};
use dark_castle_rpg::magic::projectiles::Projectile;
use dark_castle_rpg::player::components::Player;
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::save::systems::SaveSlots;
use dark_castle_rpg::world::collision::Aabb;
use dark_castle_rpg::world::exploration::{Discovery, ExplorationState};
use dark_castle_rpg::world::exploration_systems::{
    DoorBarrier, Exploration, IllusoryWall, PickupMarker, activate_world, install_world,
};
use dark_castle_rpg::world::graph::{Ability, WorldDefinition};
use dark_castle_rpg::world::maps::{CurrentRoom, RoomEntity};
use dark_castle_rpg::world::physics::{RoomGeometry, Velocity};

const FRAME: Duration = Duration::from_millis(16);
const APPROACH: &str = "vigil_gates_approach";
const CAUSEWAY: &str = "vigil_gates_causeway";
const SHAFT: &str = "vigil_gates_shaft";
const BARBICAN: &str = "vigil_gates_barbican";
const CACHE: &str = "vigil_gates_hidden_cache";
const GALLERY: &str = "vigil_gates_gallery";
const POSTERN: &str = "vigil_gates_postern";
const SEALED: &str = "vigil_gates_sealed_hall";

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn world() -> WorldDefinition {
    WorldDefinition::load_dir(&root().join("assets/world")).unwrap_or_else(|e| panic!("{e}"))
}

/// Key presses applied right after the input systems (so `just_pressed` is seen).
#[derive(Resource, Default)]
struct PendingPress(Vec<KeyCode>);

fn apply_presses(mut pending: ResMut<PendingPress>, mut keys: ResMut<ButtonInput<KeyCode>>) {
    for key in pending.0.drain(..) {
        keys.press(key);
    }
}

fn app_with(state: Option<ExplorationState>) -> App {
    let mut data = GameData::load(root()).unwrap_or_else(|e| panic!("{e}"));
    let world = world();
    install_world(&mut data, &world);
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatesPlugin, BevyInputPlugin))
        .insert_resource(TimeUpdateStrategy::ManualDuration(FRAME))
        .insert_resource(data)
        .add_plugins(GamePlugin)
        .init_resource::<PendingPress>()
        .add_systems(PreUpdate, apply_presses.after(InputSystems))
        .insert_resource(SaveSlots(SaveStorage::new(
            PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("world_exploration_saves"),
        )));
    // V1.7: the exploration plugin is part of `GamePlugin`; the world is
    // activated as the main game does it.
    activate_world(&mut app, world);
    if let Some(state) = state {
        app.insert_resource(Exploration(state));
    }
    app.update();
    run(&mut app, 3);
    app
}

fn run(app: &mut App, frames: usize) {
    for _ in 0..frames {
        app.update();
    }
}

fn current(app: &App) -> String {
    app.world().resource::<CurrentRoom>().0.clone()
}

fn exploration(app: &App) -> ExplorationState {
    app.world().resource::<Exploration>().0.clone()
}

fn count<F: bevy::ecs::query::QueryFilter>(app: &mut App) -> usize {
    let world = app.world_mut();
    world.query_filtered::<(), F>().iter(world).count()
}

fn player_xy(app: &mut App) -> Vec2 {
    let world = app.world_mut();
    world
        .query_filtered::<&Transform, With<Player>>()
        .single(world)
        .map(|t| t.translation.truncate())
        .unwrap_or_else(|e| panic!("{e}"))
}

fn place(app: &mut App, p: (f32, f32)) {
    let world = app.world_mut();
    let mut q = world.query_filtered::<(&mut Transform, &mut Velocity), With<Player>>();
    let (mut t, mut v) = q.single_mut(world).unwrap_or_else(|e| panic!("{e}"));
    t.translation.x = p.0;
    t.translation.y = p.1;
    v.0 = Vec2::ZERO;
}

/// Puts the player on `door`'s trigger in the current room and runs a frame.
fn go_through(app: &mut App, door: &str) {
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
    let p = match d.side {
        dark_castle_rpg::world::graph::Side::Left => (8.0, o.min.1 + 24.0),
        dark_castle_rpg::world::graph::Side::Right => (w - 8.0, o.min.1 + 24.0),
        dark_castle_rpg::world::graph::Side::Bottom => (o.min.0 + o.size.0 / 2.0, 8.0),
        dark_castle_rpg::world::graph::Side::Top => (o.min.0 + o.size.0 / 2.0, h - 8.0),
    };
    place(app, p);
    run(app, 1);
}

fn walk(app: &mut App, route: &[(&str, &str)]) {
    for (door, arrives) in route {
        go_through(app, door);
        assert_eq!(current(app), *arrives, "through {door}");
    }
}

#[test]
fn the_game_without_the_plugin_keeps_its_single_room() {
    let data = GameData::load(root()).unwrap_or_else(|e| panic!("{e}"));
    assert!(
        !data.rooms.contains_key(APPROACH),
        "the slice is not loaded by GameData"
    );
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatesPlugin, BevyInputPlugin))
        .insert_resource(data)
        .add_plugins(GamePlugin)
        .insert_resource(SaveSlots(SaveStorage::new(
            PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("world_exploration_saves"),
        )));
    app.update();
    assert_eq!(current(&app), "entry_hall");
    assert!(app.world().get_resource::<Exploration>().is_none());
}

#[test]
fn starts_at_the_start_door_with_the_start_room_visited() {
    let mut app = app_with(None);
    assert_eq!(current(&app), APPROACH);
    let w = world();
    let room = &w.rooms[APPROACH];
    let entry = room.entry_point(room.door("to_sealed").unwrap_or_else(|| panic!("door")));
    assert!(player_xy(&mut app).distance(Vec2::new(entry.0, entry.1)) < 4.0);
    let state = exploration(&app);
    assert_eq!(state.discovery(APPROACH), Discovery::Visited);
    assert_eq!(state.discovery(CAUSEWAY), Discovery::Discovered);
    assert_eq!(
        state.discovery(POSTERN),
        Discovery::Unknown,
        "the ceiling hole reveals nothing"
    );
}

#[test]
fn going_through_doors_and_back_keeps_one_room_loaded() {
    let mut app = app_with(None);
    let in_approach = count::<With<RoomEntity>>(&mut app);
    walk(&mut app, &[("to_causeway", CAUSEWAY)]);
    run(&mut app, 2);
    let in_causeway = count::<With<RoomEntity>>(&mut app);
    assert_eq!(
        count::<With<Enemy>>(&mut app),
        1,
        "the causeway's sentry, once"
    );
    let state = exploration(&app);
    assert_eq!(state.discovery(CAUSEWAY), Discovery::Visited);
    assert_eq!(state.previous_room.as_deref(), Some(APPROACH));
    // Arrival at the matching door, not inside geometry.
    let w = world();
    let room = &w.rooms[CAUSEWAY];
    let entry = room.entry_point(room.door("from_approach").unwrap_or_else(|| panic!("door")));
    assert!(player_xy(&mut app).distance(Vec2::new(entry.0, entry.1)) < 4.0);
    let body = Aabb::new((entry.0, entry.1), (26.0, 46.0));
    let geometry = app.world().resource::<RoomGeometry>();
    assert!(
        !geometry
            .solids
            .iter()
            .any(|s| !s.one_way && s.aabb.intersects(&body))
    );
    // Back and forth: the entity count never grows, enemies never pile up.
    for _ in 0..5 {
        walk(&mut app, &[("from_approach", APPROACH)]);
        run(&mut app, 2);
        assert_eq!(count::<With<RoomEntity>>(&mut app), in_approach);
        assert_eq!(count::<With<Enemy>>(&mut app), 0);
        walk(&mut app, &[("to_causeway", CAUSEWAY)]);
        run(&mut app, 2);
        assert_eq!(count::<With<RoomEntity>>(&mut app), in_causeway);
        assert_eq!(count::<With<Enemy>>(&mut app), 1);
    }
    assert_eq!(count::<With<Player>>(&mut app), 1);
}

#[test]
fn different_doors_lead_to_different_entries() {
    let mut app = app_with(None);
    walk(&mut app, &[("to_causeway", CAUSEWAY), ("to_shaft", SHAFT)]);
    let w = world();
    let shaft = &w.rooms[SHAFT];
    let bottom = shaft.entry_point(
        shaft
            .door("from_causeway")
            .unwrap_or_else(|| panic!("door")),
    );
    assert!(player_xy(&mut app).distance(Vec2::new(bottom.0, bottom.1)) < 4.0);
    walk(&mut app, &[("to_gallery", GALLERY), ("from_shaft", SHAFT)]);
    let top = shaft.entry_point(shaft.door("to_gallery").unwrap_or_else(|| panic!("door")));
    assert!(
        player_xy(&mut app).distance(Vec2::new(top.0, top.1)) < 4.0,
        "came back by the upper door"
    );
}

#[test]
fn the_barrier_blocks_until_the_relic_and_then_opens() {
    let mut app = app_with(None);
    assert_eq!(count::<With<DoorBarrier>>(&mut app), 1);
    let blocked = app.world().resource::<RoomGeometry>().solids.len();
    go_through(&mut app, "to_sealed");
    run(&mut app, 3);
    assert_eq!(current(&app), APPROACH, "locked: no transition");
    // The relic, by the main route: causeway → shaft → gallery.
    walk(
        &mut app,
        &[
            ("to_causeway", CAUSEWAY),
            ("to_shaft", SHAFT),
            ("to_gallery", GALLERY),
        ],
    );
    assert_eq!(count::<With<PickupMarker>>(&mut app), 1);
    let w = world();
    let relic = w.rooms[GALLERY].pickups[0].position;
    place(&mut app, (relic.0, relic.1 - 10.0));
    run(&mut app, 2);
    assert_eq!(count::<With<PickupMarker>>(&mut app), 0);
    assert!(
        exploration(&app)
            .capabilities()
            .abilities
            .contains(&Ability::VeilStep)
    );
    // Return by the shortcut: gallery → postern → drop into the approach.
    walk(
        &mut app,
        &[("to_postern", POSTERN), ("to_approach", APPROACH)],
    );
    run(&mut app, 2);
    assert_eq!(
        count::<With<DoorBarrier>>(&mut app),
        0,
        "no barrier once the requirement is met"
    );
    assert_eq!(
        app.world().resource::<RoomGeometry>().solids.len(),
        blocked - 1
    );
    walk(
        &mut app,
        &[
            ("to_sealed", SEALED),
            ("to_reliquary", "vigil_gates_reliquary"),
        ],
    );
    // Re-entering the gallery does not respawn the relic.
    walk(
        &mut app,
        &[
            ("from_sealed", SEALED),
            ("from_approach", APPROACH),
            ("to_causeway", CAUSEWAY),
            ("to_shaft", SHAFT),
            ("to_gallery", GALLERY),
        ],
    );
    run(&mut app, 2);
    assert_eq!(count::<With<PickupMarker>>(&mut app), 0);
}

#[test]
fn the_false_wall_hides_a_room_until_walked_into() {
    let mut app = app_with(None);
    walk(
        &mut app,
        &[
            ("to_causeway", CAUSEWAY),
            ("to_shaft", SHAFT),
            ("to_barbican", BARBICAN),
        ],
    );
    assert_eq!(count::<With<IllusoryWall>>(&mut app), 1);
    assert_eq!(exploration(&app).discovery(CACHE), Discovery::Unknown);
    // The first frame on the threshold finds the secret; the next one goes through.
    go_through(&mut app, "to_cache");
    assert_eq!(current(&app), BARBICAN);
    assert!(exploration(&app).secrets.contains("barbican_false_wall"));
    assert_eq!(exploration(&app).discovery(CACHE), Discovery::Discovered);
    go_through(&mut app, "to_cache");
    assert_eq!(current(&app), CACHE);
    // Back through the passage: the wall stays found (faded) and passable.
    walk(&mut app, &[("from_barbican", BARBICAN)]);
    run(&mut app, 1);
    let world = app.world_mut();
    let alpha = world
        .query_filtered::<&Sprite, With<IllusoryWall>>()
        .single(world)
        .map(|s| s.color.alpha())
        .unwrap_or_else(|e| panic!("{e}"));
    assert!(alpha < 0.5);
    walk(&mut app, &[("to_cache", CACHE)]);
    // V1.8: the count of the Vigil Gates (the region this test explores);
    // the galleries add two more secrets to the world.
    let stats = exploration(&app).region_stats(&self::world(), "vigil_gates");
    assert_eq!((stats.secrets_found, stats.secrets_total), (1, 1));
}

#[test]
fn leaving_a_room_drops_its_respawns_and_projectiles() {
    let mut app = app_with(None);
    walk(&mut app, &[("to_causeway", CAUSEWAY)]);
    run(&mut app, 2);
    // A pending respawn of the causeway's sentry and a projectile in flight.
    let spawn = world().rooms[CAUSEWAY].enemies[0].clone();
    app.world_mut()
        .resource_mut::<PendingRespawns>()
        .0
        .push((spawn, 0.05));
    app.world_mut()
        .spawn((Projectile { velocity: Vec2::X }, Transform::default()));
    walk(&mut app, &[("from_approach", APPROACH)]);
    run(&mut app, 20);
    assert!(app.world().resource::<PendingRespawns>().0.is_empty());
    assert_eq!(
        count::<With<Enemy>>(&mut app),
        0,
        "no sentry respawned in the approach"
    );
    assert_eq!(count::<With<Projectile>>(&mut app), 0);
}

#[test]
fn exploration_survives_a_restart_through_its_file() {
    let mut app = app_with(None);
    walk(
        &mut app,
        &[
            ("to_causeway", CAUSEWAY),
            ("to_shaft", SHAFT),
            ("to_barbican", BARBICAN),
        ],
    );
    go_through(&mut app, "to_cache"); // finds the secret
    walk(
        &mut app,
        &[
            ("to_cache", CACHE),
            ("from_barbican", BARBICAN),
            ("from_shaft", SHAFT),
            ("to_gallery", GALLERY),
        ],
    );
    let w = world();
    let relic = w.rooms[GALLERY].pickups[0].position;
    place(&mut app, (relic.0, relic.1 - 10.0));
    run(&mut app, 2);
    let saved = exploration(&app).to_ron().unwrap_or_else(|e| panic!("{e}"));
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("world_exploration_state.ron");
    std::fs::write(&path, &saved).unwrap_or_else(|e| panic!("{e}"));
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{e}"));
    let restored = ExplorationState::from_ron(&text).unwrap_or_else(|e| panic!("{e}"));
    let mut app = app_with(Some(restored));
    assert_eq!(current(&app), APPROACH);
    assert_eq!(count::<With<DoorBarrier>>(&mut app), 0, "the relic is kept");
    let state = exploration(&app);
    assert_eq!(state.discovery(CACHE), Discovery::Visited);
    assert!(state.secrets.contains("barbican_false_wall"));
    walk(&mut app, &[("to_sealed", SEALED)]);
}

#[test]
fn walking_into_a_wall_does_not_change_room() {
    let mut app = app_with(None);
    // The approach's right wall is solid above its door: no transition there.
    let w = world();
    let (width, _) = w.rooms[APPROACH].size();
    place(&mut app, (width - 8.0, 500.0));
    run(&mut app, 1);
    assert_eq!(current(&app), APPROACH);
}

fn hold(app: &mut App, key: KeyCode, down: bool) {
    let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
    if down {
        keys.press(key);
    } else {
        keys.release(key);
    }
}

/// Runs until the current room is `room` (or `max` frames); returns the frames used.
fn until_room(app: &mut App, room: &str, max: usize) -> usize {
    for frame in 0..max {
        if current(app) == room {
            return frame;
        }
        run(app, 1);
    }
    panic!("never reached {room} (in {})", current(app));
}

#[test]
fn real_movement_walks_through_a_door_and_drops_down_the_postern() {
    let mut app = app_with(None);
    // Walking right with the real input and physics, from the start door,
    // jumping over the block in front of the exit (50 units: a walk stops there).
    hold(&mut app, KeyCode::ArrowRight, true);
    run(&mut app, 100);
    app.world_mut()
        .resource_mut::<PendingPress>()
        .0
        .push(KeyCode::KeyZ);
    run(&mut app, 12);
    hold(&mut app, KeyCode::KeyZ, false);
    let frames = 112 + until_room(&mut app, CAUSEWAY, 600);
    assert!(frames > 60, "it took a real walk ({frames} frames)");
    hold(&mut app, KeyCode::ArrowRight, false);
    // The shortcut: from the postern's upper door, walk left off the ledge,
    // fall onto the funnel, off it and through the floor opening.
    walk(
        &mut app,
        &[
            ("to_shaft", SHAFT),
            ("to_gallery", GALLERY),
            ("to_postern", POSTERN),
        ],
    );
    hold(&mut app, KeyCode::ArrowLeft, true);
    until_room(&mut app, APPROACH, 600);
    hold(&mut app, KeyCode::ArrowLeft, false);
    // Arrived through the ceiling, then landed on the approach's floor.
    run(&mut app, 90);
    let p = player_xy(&mut app);
    assert!(
        (p.y - (40.0 + 23.0)).abs() < 2.0,
        "standing on the floor: {p:?}"
    );
    assert_eq!(exploration(&app).discovery(POSTERN), Discovery::Visited);
}
