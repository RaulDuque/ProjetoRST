//! D23: the Reliquary stairs are climbed with jump inputs. The room request
//! only places the player at the foot of the stair. The court is entered
//! through its door.

use std::path::{Path, PathBuf};
use std::time::Duration;

use bevy::input::InputPlugin as BevyInputPlugin;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::app::load_world;
use dark_castle_rpg::app::plugins::GamePlugin;
use dark_castle_rpg::core::constants::DEFAULT_SAVE_SLOT;
use dark_castle_rpg::core::events::{Arrival, RoomRequest};
use dark_castle_rpg::core::resources::GameData;
use dark_castle_rpg::enemies::boss_systems::BossEncounter;
use dark_castle_rpg::enemies::components::{Boss, EnemyHealth};
use dark_castle_rpg::player::components::Player;
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::save::systems::SaveSlots;
use dark_castle_rpg::world::exploration_systems::activate_world;
use dark_castle_rpg::world::maps::CurrentRoom;
use dark_castle_rpg::world::physics::Grounded;

#[path = "support/route_driver.rs"]
mod route_driver;

use route_driver::{Dir, Expect, RouteDriver, RouteStatus, Step};

const FRAME: Duration = Duration::from_millis(16);
const RELIQUARY: &str = "vigil_gates_reliquary";
const COURT: &str = "vigil_gates_ashen_court";

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn game(save_dir: &str) -> App {
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
        .insert_resource(SaveSlots(storage));
    activate_world(&mut app, world);
    app
}

fn climb() -> Vec<Step> {
    let mut steps = vec![Step::Mark("stair_foot")];
    // Five landings plus the jump through the ceiling door. Extra jumps
    // after arrival stay on the court's landing (held still).
    for _ in 0..8 {
        steps.push(Step::Jump(Dir::Still));
    }
    steps.push(Step::Wait(20));
    steps.push(Step::Check(Expect::Room(COURT)));
    steps.push(Step::Check(Expect::EnemyAlive("ashen_sentinel")));
    steps.push(Step::Mark("court"));
    steps
}

#[test]
fn jumps_up_the_reliquary_stairs_into_the_court() {
    let mut app = game("d23_stairs");
    for _ in 0..4 {
        app.update();
    }
    app.world_mut().write_message(RoomRequest {
        room: RELIQUARY.into(),
        arrival: Arrival::Position(Vec2::new(630.0, 63.0)),
    });
    for _ in 0..6 {
        app.update();
    }
    assert_eq!(app.world().resource::<CurrentRoom>().0, RELIQUARY);

    route_driver::install(&mut app, climb());
    for _ in 0..8_000 {
        app.update();
        if app.world().resource::<RouteDriver>().is_done() {
            break;
        }
    }
    let driver = app.world().resource::<RouteDriver>();
    let report = driver.report();
    assert_eq!(driver.status, RouteStatus::Passed, "\n{report}");
    assert_eq!(driver.artificial_repositions, 0, "\n{report}");
    assert!(
        app.world().resource::<BossEncounter>().active,
        "encounter did not start"
    );
    let (hp, grounded, y) = {
        let world = app.world_mut();
        let hp = world
            .query_filtered::<&EnemyHealth, With<Boss>>()
            .iter(world)
            .next()
            .map(|h| h.current())
            .unwrap_or(0);
        let mut players = world.query_filtered::<(&Transform, &Grounded), With<Player>>();
        let (transform, ground) = players.single(world).unwrap_or_else(|e| panic!("{e}"));
        (hp, ground.on_ground, transform.translation.y)
    };
    assert!(hp > 200, "boss hp {hp}");
    assert!(grounded, "arrival is not standing");
    assert!(y > 50.0 && y < 120.0, "arrival y {y} is off the landing");
}
