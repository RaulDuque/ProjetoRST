//! V1.7: the main route of the Vigil Gates played with real input only.
//!
//! The app is the shipped game as `app::run` builds it (world loaded,
//! validated, installed and activated), headless. A route driver
//! (`tests/support/route_driver.rs`) only presses keys: walking, jumping,
//! attacking, F5/F9. It audits every frame and counts any repositioning of
//! the player not made by the game's own room lifecycle. Saves go to a
//! temporary directory; `saves/` is never touched.

use std::path::{Path, PathBuf};
use std::time::Duration;

use bevy::input::InputPlugin as BevyInputPlugin;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::app::load_world;
use dark_castle_rpg::app::plugins::GamePlugin;
use dark_castle_rpg::core::constants::DEFAULT_SAVE_SLOT;
use dark_castle_rpg::core::resources::GameData;
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::save::systems::SaveSlots;
use dark_castle_rpg::world::exploration_systems::activate_world;

#[path = "support/route_driver.rs"]
mod route_driver;
#[path = "support/vigil_route.rs"]
mod vigil_route;

use route_driver::{Dir, RouteDriver, RouteStatus, Step};
use vigil_route::main_route;

const FRAME: Duration = Duration::from_millis(16);

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn game(save_dir: &str) -> (App, SaveStorage) {
    let mut data = GameData::load(root()).unwrap_or_else(|e| panic!("{e}"));
    let world = load_world(root(), &mut data).unwrap_or_else(|e| panic!("{e}"));
    let storage = SaveStorage::new(PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(save_dir));
    for path in [
        storage.slot_path(DEFAULT_SAVE_SLOT),
        storage.backup_path(DEFAULT_SAVE_SLOT),
    ] {
        let _ = std::fs::remove_file(path);
    }
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatesPlugin, BevyInputPlugin))
        .insert_resource(TimeUpdateStrategy::ManualDuration(FRAME))
        .insert_resource(data)
        .add_plugins(GamePlugin)
        .insert_resource(SaveSlots(storage.clone()));
    activate_world(&mut app, world);
    (app, storage)
}

#[test]
fn the_main_route_is_playable_with_real_input_and_no_shortcuts() {
    let (mut app, storage) = game("route_saves");
    route_driver::install(&mut app, main_route());
    for _ in 0..40_000 {
        app.update();
        if app.world().resource::<RouteDriver>().is_done() {
            break;
        }
    }
    let driver = app.world().resource::<RouteDriver>();
    let report = driver.report();
    let out = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("world_route.log");
    let _ = std::fs::write(&out, &report);
    assert_eq!(driver.status, RouteStatus::Passed, "\n{report}");
    assert_eq!(driver.artificial_repositions, 0, "\n{report}");
    // 9 rooms entered by doors plus one load.
    assert!(driver.room_changes >= 14, "{}", driver.room_changes);
    assert!(
        storage.load(DEFAULT_SAVE_SLOT).is_ok(),
        "F5 wrote the isolated slot"
    );
}

/// Plays the causeway fight and returns what combat produced.
fn fight_outcome() -> (Vec<(u32, String, i32, i32)>, i32, u64) {
    use dark_castle_rpg::player::components::{CharacterSheet, Player, Vitals};
    let (mut app, _) = game("route_fight_saves");
    let steps = vec![
        Step::WalkUntilRoom(Dir::Right, "vigil_gates_causeway"),
        Step::WalkTo(1000.0),
        Step::Fight,
    ];
    route_driver::install(&mut app, steps);
    for _ in 0..5_000 {
        app.update();
        if app.world().resource::<RouteDriver>().is_done() {
            break;
        }
    }
    let world = app.world_mut();
    let (hp, exp) = world
        .query_filtered::<(&Vitals, &CharacterSheet), With<Player>>()
        .single(world)
        .map(|(v, s)| (v.hp.current(), s.progression().total_exp()))
        .unwrap_or_else(|e| panic!("{e}"));
    let driver = app.world().resource::<RouteDriver>();
    assert_eq!(driver.status, RouteStatus::Passed, "{}", driver.report());
    let log = driver
        .log
        .iter()
        .map(|e| {
            (
                e.frame,
                e.action.clone(),
                e.position.x.round() as i32,
                e.position.y.round() as i32,
            )
        })
        .collect();
    (log, hp, exp)
}

#[test]
fn combat_in_the_integrated_world_is_deterministic() {
    let first = fight_outcome();
    let second = fight_outcome();
    assert_eq!(first, second);
    assert!(first.2 > 0, "the sentry was defeated and gave EXP");
}
