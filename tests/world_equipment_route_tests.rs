//! V2.0: the equipment route played with real input only, from the
//! game's start (`tests/support/equipment_route.rs`). The route driver only
//! presses keys and counts every repositioning not made by the game's own
//! room lifecycle (must be 0). Saves go to a temporary directory; `saves/`
//! is never touched.

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

#[path = "support/equipment_route.rs"]
mod equipment_route;
#[path = "support/route_driver.rs"]
mod route_driver;
#[path = "support/vigil_route.rs"]
mod vigil_route;

use route_driver::{RouteDriver, RouteStatus};

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
    dark_castle_rpg::input::use_legacy_controls(&mut app);
    app.add_plugins((MinimalPlugins, StatesPlugin, BevyInputPlugin))
        .insert_resource(TimeUpdateStrategy::ManualDuration(FRAME))
        .insert_resource(data)
        .add_plugins(GamePlugin)
        .insert_resource(SaveSlots(storage.clone()));
    activate_world(&mut app, world);
    (app, storage)
}

#[test]
fn the_equipment_route_is_playable_with_real_input_and_no_shortcuts() {
    let (mut app, storage) = game("equipment_route_saves");
    route_driver::install(&mut app, equipment_route::equipment_route());
    for _ in 0..60_000 {
        app.update();
        if app.world().resource::<RouteDriver>().is_done() {
            break;
        }
    }
    let driver = app.world().resource::<RouteDriver>();
    let report = driver.report();
    let out = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("world_equipment_route.log");
    let _ = std::fs::write(&out, &report);
    assert_eq!(driver.status, RouteStatus::Passed, "\n{report}");
    assert_eq!(driver.artificial_repositions, 0, "\n{report}");
    assert!(
        storage.load(DEFAULT_SAVE_SLOT).is_ok(),
        "F5 wrote the isolated slot"
    );
}
