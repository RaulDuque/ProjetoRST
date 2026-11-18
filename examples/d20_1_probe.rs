//! D20.1 windowed probe of D17–D20 systems with real screenshots.
//!
//! Plays `tests/support/d20_1_route.rs` through the Vigil Gates world with
//! input-only driving. Screenshots land in `docs/development/d20_1/evidence/`
//! and a copy under `target/d20_1/`. Saves are isolated under
//! `target/d20_1/saves/` — the real `saves/` directory is never written.
//!
//! Run with: `cargo run --example d20_1_probe` (needs a graphical session).

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

#[path = "../tests/support/d20_1_route.rs"]
mod d20_1_route;
#[path = "../tests/support/route_driver.rs"]
mod route_driver;

use route_driver::{RouteDriver, RouteStatus};

const FRAME: Duration = Duration::from_millis(16);
const EXIT_DELAY: u32 = 30;

static VERDICT: OnceLock<bool> = OnceLock::new();

fn out_dir() -> PathBuf {
    project_root().join("target/d20_1")
}

fn evidence_dir() -> PathBuf {
    project_root().join("docs/development/d20_1/evidence")
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
        let evidence = evidence_dir();
        if let Err(error) = std::fs::create_dir_all(&evidence) {
            println!("[d20_1_probe] evidence dir: {error}");
        }
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(evidence.join(format!("{name}.png"))));
        let stats = exploration.stats(&graph.0);
        println!(
            "[d20_1_probe] {name}: map {:.0}% rooms {}/{}",
            stats.percent, stats.visited, stats.total
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
        println!("[d20_1_probe] cannot write route.log: {error}");
    }
    if let Err(error) = std::fs::write(evidence_dir().join("route.log"), &report) {
        println!("[d20_1_probe] cannot write evidence route.log: {error}");
    }
    print!("{report}");
    let pass = driver.status == RouteStatus::Passed && driver.artificial_repositions == 0;
    println!(
        "[d20_1_probe] RESULT: {} (room changes {}, artificial repositions {})",
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
            eprintln!("[d20_1_probe] {error}");
            return AppExit::error();
        }
    };
    let saves = out_dir().join("saves");
    let _ = std::fs::remove_dir_all(&saves);
    if let Err(error) = std::fs::create_dir_all(&saves) {
        eprintln!("[d20_1_probe] {}: {error}", saves.display());
        return AppExit::error();
    }
    if let Err(error) = std::fs::create_dir_all(evidence_dir()) {
        eprintln!("[d20_1_probe] evidence: {error}");
        return AppExit::error();
    }
    println!(
        "[d20_1_probe] saves isolated in {}; evidence → {}",
        saves.display(),
        evidence_dir().display()
    );

    let mut app = dark_castle_rpg::app::build_with_world(data, world);
    app.insert_resource(SaveSlots(SaveStorage::new(saves)))
        .insert_resource(TimeUpdateStrategy::ManualDuration(FRAME))
        .init_resource::<Capture>()
        .add_systems(Last, capture_marks);
    route_driver::install(&mut app, d20_1_route::d20_1_route());
    let exit = app.run();
    match VERDICT.get() {
        Some(true) => exit,
        _ => AppExit::error(),
    }
}
