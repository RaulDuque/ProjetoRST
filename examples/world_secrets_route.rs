//! V1.8 demonstration: the shipped game (`app::run` setup) played from the
//! start through the Vigil Gates and on through the Forgotten Galleries,
//! with real input only, in a window, with screenshots.
//!
//! The route (`tests/support/galleries_route.rs`, after the V1.7 route of
//! `tests/support/vigil_route.rs`) is played by the route driver, which only
//! presses keys and audits every frame for artificial repositionings: the
//! galleries' chest, the cracked wall broken by a real attack, the secret
//! chest, the Hollow Stair climb, the Astral Emblem, the hidden switch and
//! the oratory, the drop loop, the lever and its gate, the astral seal, the
//! rare chest, the trapdoor lever, F5, the shortcut back to the gates, F9,
//! the shortcut again after the load and the wall still broken.
//!
//! Outputs in `target/world_secrets_route/`: one screenshot per route mark,
//! `route.log` and the saves of the run (`saves/` of the project is never
//! touched). Exit code != 0 if the route fails or any artificial
//! repositioning is seen.
//!
//! Run with: `cargo run --example world_secrets_route` (graphical session).

use std::path::PathBuf;
use std::sync::OnceLock;
use std::time::Duration;

use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::core::resources::{GameData, project_root};
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::save::systems::SaveSlots;
use dark_castle_rpg::world::exploration_systems::{Exploration, WorldGraph};

#[path = "../tests/support/galleries_route.rs"]
mod galleries_route;
#[path = "../tests/support/route_driver.rs"]
mod route_driver;
#[path = "../tests/support/vigil_route.rs"]
mod vigil_route;

use route_driver::{RouteDriver, RouteStatus};

const FRAME: Duration = Duration::from_millis(16);
/// Frames to wait after the route so the last screenshot reaches the disk.
const EXIT_DELAY: u32 = 30;

static VERDICT: OnceLock<bool> = OnceLock::new();

fn out_dir() -> PathBuf {
    project_root().join("target/world_secrets_route")
}

#[derive(Resource, Default)]
struct Capture {
    shots: usize,
    exit_in: Option<u32>,
}

fn capture_marks(
    mut commands: Commands,
    driver: Res<RouteDriver>,
    graph: Res<WorldGraph>,
    exploration: Res<Exploration>,
    mut capture: ResMut<Capture>,
    mut exit: MessageWriter<AppExit>,
) {
    while capture.shots < driver.marks.len() {
        let name = driver.marks[capture.shots];
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(out_dir().join(format!("{name}.png"))));
        let stats = exploration.stats(&graph.0);
        let galleries = exploration.region_stats(&graph.0, "forgotten_galleries");
        println!(
            "[world_secrets_route] {name}: map {:.0}% rooms {}/{} secrets {}/{} | galleries {:.0}% rooms {}/{} secrets {}/{}",
            stats.percent,
            stats.visited,
            stats.total,
            stats.secrets_found,
            stats.secrets_total,
            galleries.percent,
            galleries.visited,
            galleries.total,
            galleries.secrets_found,
            galleries.secrets_total
        );
        capture.shots += 1;
    }
    if !driver.is_done() {
        return;
    }
    let remaining = capture.exit_in.get_or_insert(EXIT_DELAY);
    if *remaining > 0 {
        *remaining -= 1;
        return;
    }
    if VERDICT.get().is_some() {
        return;
    }
    let report = driver.report();
    if let Err(error) = std::fs::write(out_dir().join("route.log"), &report) {
        println!("[world_secrets_route] cannot write route.log: {error}");
    }
    print!("{report}");
    let pass = driver.status == RouteStatus::Passed && driver.artificial_repositions == 0;
    println!(
        "[world_secrets_route] RESULT: {} (room changes {}, artificial repositions {})",
        if pass { "PASS" } else { "FAIL" },
        driver.room_changes,
        driver.artificial_repositions
    );
    let _ = VERDICT.set(pass);
    exit.write(AppExit::Success);
}

fn main() -> AppExit {
    let root = project_root();
    let loaded = GameData::load(&root)
        .map_err(|e| e.to_string())
        .and_then(|mut data| {
            dark_castle_rpg::app::load_world(&root, &mut data).map(|world| (data, world))
        });
    let (data, world) = match loaded {
        Ok(loaded) => loaded,
        Err(error) => {
            eprintln!("[world_secrets_route] {error}");
            return AppExit::error();
        }
    };
    let saves = out_dir().join("saves");
    let _ = std::fs::remove_dir_all(&saves);
    if let Err(error) = std::fs::create_dir_all(&saves) {
        eprintln!("[world_secrets_route] {}: {error}", saves.display());
        return AppExit::error();
    }
    let mut app = dark_castle_rpg::app::build_with_world(data, world);
    dark_castle_rpg::input::use_legacy_controls(&mut app);
    app.insert_resource(SaveSlots(SaveStorage::new(saves)))
        .insert_resource(TimeUpdateStrategy::ManualDuration(FRAME))
        .init_resource::<Capture>()
        .add_systems(Last, capture_marks);
    route_driver::install(&mut app, galleries_route::full_route());
    let exit = app.run();
    match VERDICT.get() {
        Some(true) => exit,
        _ => AppExit::error(),
    }
}
