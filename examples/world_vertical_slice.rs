//! V1.6 vertical slice: the "Vigil Gates" region in the real game.
//!
//! Kept as the V1.6 reference (it uses test shortcuts). The V1.7 route in
//! the main game, with real input only, is `examples/world_main_route.rs`.
//!
//! The real app (player, physics, combat, enemies, sprite visual, VFX, HUD)
//! with the world graph installed in memory (`world::exploration_systems::
//! install_world`), `WorldExplorationPlugin` and the map panel (since V1.7
//! the same setup as `cargo run`).
//!
//! Modes:
//! - default: a scripted route through the 9 rooms with screenshots and
//!   checks (exit code != 0 if one fails). Walking, jumping, the false wall,
//!   the pickups and the postern drop use the real input and physics. Test
//!   shortcuts, logged as `SHORTCUT`: placing the player on a door's
//!   threshold (the shaft climb and a few long walks), never inside a room's
//!   geometry; every room change is still made by the game's door system;
//! - `--interactive`: play the slice by hand (arrows, Z jump, X attack,
//!   C magic, A guard, S dodge); the exploration state is written on exit.
//!
//! Outputs in `target/world_vertical_slice/`: screenshots, `route.log`,
//! `exploration.ron` (the stand-alone exploration file) and the validation
//! report. Saves are redirected there too (`saves/` is never touched).
//!
//! Run with: `cargo run --example world_vertical_slice [-- --interactive]`.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::Duration;

use bevy::input::InputSystems;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::core::resources::{GameData, project_root};
use dark_castle_rpg::player::components::Player;
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::save::systems::SaveSlots;
use dark_castle_rpg::world::exploration_systems::{Exploration, WorldGraph, install_world};
use dark_castle_rpg::world::graph::{Ability, Side, WorldDefinition};
use dark_castle_rpg::world::maps::CurrentRoom;
use dark_castle_rpg::world::physics::Velocity;
use dark_castle_rpg::world::validation::validate;

const FRAME: Duration = Duration::from_millis(16);

use KeyCode::{ArrowLeft as L, ArrowRight as R, KeyZ as Jump};

#[derive(Clone, Copy)]
enum Step {
    /// Hold a key for some frames (real input).
    Hold(KeyCode, u32),
    /// Hold a key until the given room is loaded (at most 900 frames).
    HoldUntilRoom(KeyCode, &'static str),
    /// Hold a key until the named secret is found.
    HoldUntilSecret(KeyCode, &'static str),
    /// Hold a direction and the jump key (a full jump: releasing early cuts it).
    HoldJump(KeyCode, u32),
    Wait(u32),
    /// Test shortcut: put the player on this door's threshold of the current room.
    Threshold(&'static str),
    /// Test shortcut: put the player at a point of the current room, standing.
    Place(f32, f32),
    Shot(&'static str),
    ExpectRoom(&'static str),
}

use Step::*;

const ROUTE: &[Step] = &[
    Wait(40),
    Shot("01_approach_sealed_grate"),
    Hold(R, 100),
    HoldJump(R, 14),
    HoldUntilRoom(R, "vigil_gates_causeway"),
    Hold(R, 120),
    Wait(20),
    Shot("02_causeway"),
    Threshold("to_shaft"),
    ExpectRoom("vigil_gates_shaft"),
    Wait(30),
    Shot("03_watch_shaft_bottom"),
    Threshold("to_barbican"),
    ExpectRoom("vigil_gates_barbican"),
    Wait(30),
    Shot("04_barbican_wall"),
    Place(1080.0, 63.0),
    HoldUntilSecret(R, "barbican_false_wall"),
    Shot("05_false_wall_found"),
    HoldUntilRoom(R, "vigil_gates_hidden_cache"),
    Wait(10),
    Shot("06_hidden_cache"),
    Hold(R, 280),
    Shot("07_cache_reward_taken"),
    Threshold("from_barbican"),
    ExpectRoom("vigil_gates_barbican"),
    Wait(30),
    Shot("08_back_through_the_passage"),
    Threshold("from_shaft"),
    ExpectRoom("vigil_gates_shaft"),
    Threshold("to_gallery"),
    ExpectRoom("vigil_gates_gallery"),
    Wait(30),
    Shot("09_upper_gallery"),
    Place(700.0, 63.0),
    Hold(L, 90),
    Shot("10_relic_of_the_veil"),
    Threshold("to_postern"),
    ExpectRoom("vigil_gates_postern"),
    Hold(L, 30),
    Shot("11_postern_drop"),
    HoldUntilRoom(L, "vigil_gates_approach"),
    Wait(60),
    Shot("12_back_at_the_gate_grate_open"),
    Hold(L, 50),
    HoldJump(L, 14),
    HoldUntilRoom(L, "vigil_gates_sealed_hall"),
    Wait(20),
    Shot("13_sealed_hall"),
    HoldUntilRoom(L, "vigil_gates_reliquary"),
    Hold(L, 280),
    Wait(30),
    Shot("14_reliquary_goal_and_map"),
    // Screenshots are saved asynchronously: let the last one land before exiting.
    Wait(30),
];

#[derive(Resource, Default)]
struct Script {
    step: usize,
    frames: u32,
    log: String,
    failures: Vec<String>,
    shots: usize,
}

static VERDICT: OnceLock<bool> = OnceLock::new();

fn out_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("target/world_vertical_slice")
}

fn world() -> Result<WorldDefinition, String> {
    WorldDefinition::load_dir(&project_root().join("assets/world")).map_err(|e| e.to_string())
}

/// Held keys of the current step, applied after the input systems.
fn drive_keys(script: Res<Script>, mut keys: ResMut<ButtonInput<KeyCode>>) {
    let (held, jump) = match ROUTE.get(script.step) {
        Some(Hold(k, _) | HoldUntilRoom(k, _) | HoldUntilSecret(k, _)) => (Some(*k), false),
        Some(HoldJump(k, _)) => (Some(*k), true),
        _ => (None, false),
    };
    for key in [L, R, Jump] {
        if Some(key) == held || (key == Jump && jump) {
            keys.press(key);
        } else if keys.pressed(key) {
            keys.release(key);
        }
    }
}

fn threshold(world: &WorldDefinition, room: &str, door: &str) -> Option<(f32, f32)> {
    let room = world.room(room)?;
    let d = room.door(door)?;
    let o = room.opening(d);
    let (w, h) = room.size();
    Some(match d.side {
        Side::Left => (8.0, o.min.1 + 24.0),
        Side::Right => (w - 8.0, o.min.1 + 24.0),
        Side::Bottom => (o.min.0 + o.size.0 / 2.0, 8.0),
        Side::Top => (o.min.0 + o.size.0 / 2.0, h - 8.0),
    })
}

fn run_script(
    mut commands: Commands,
    mut script: ResMut<Script>,
    graph: Res<WorldGraph>,
    exploration: Res<Exploration>,
    current: Res<CurrentRoom>,
    mut player: Query<(&mut Transform, &mut Velocity), With<Player>>,
    mut exit: MessageWriter<AppExit>,
) {
    let Some(step) = ROUTE.get(script.step).copied() else {
        finish(&mut script, &graph.0, &exploration, &mut exit);
        return;
    };
    script.frames += 1;
    let room = current.0.clone();
    let mut done;
    match step {
        Hold(_, n) | HoldJump(_, n) | Wait(n) => done = script.frames >= n,
        HoldUntilRoom(_, target) => {
            done = room == target;
            if !done && script.frames > 900 {
                script
                    .failures
                    .push(format!("never reached {target} (in {room})"));
                done = true;
            }
        }
        HoldUntilSecret(_, secret) => {
            done = exploration.secrets.contains(secret);
            if !done && script.frames > 600 {
                script.failures.push(format!("secret {secret} never found"));
                done = true;
            }
        }
        Threshold(door) => {
            let Some(p) = threshold(&graph.0, &room, door) else {
                script.failures.push(format!("{room}: no door {door}"));
                script.step += 1;
                return;
            };
            if let Ok((mut t, mut v)) = player.single_mut() {
                t.translation.x = p.0;
                t.translation.y = p.1;
                v.0 = Vec2::ZERO;
            }
            let _ = writeln!(script.log, "SHORTCUT threshold {room}.{door}");
            done = true;
        }
        Place(x, y) => {
            if let Ok((mut t, mut v)) = player.single_mut() {
                t.translation.x = x;
                t.translation.y = y;
                v.0 = Vec2::ZERO;
            }
            let _ = writeln!(script.log, "SHORTCUT place {room} ({x}, {y})");
            done = true;
        }
        ExpectRoom(target) => {
            // The transition happens in the frame after the threshold.
            done = room == target || script.frames > 3;
            if done && room != target {
                script
                    .failures
                    .push(format!("expected {target}, in {room}"));
            }
        }
        Shot(name) => {
            let dir = out_dir();
            if let Err(error) = std::fs::create_dir_all(&dir) {
                script.failures.push(format!("{}: {error}", dir.display()));
            }
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(dir.join(format!("{name}.png"))));
            let stats = exploration.stats(&graph.0);
            let pos = player
                .single()
                .map(|(t, _)| t.translation.truncate())
                .unwrap_or_default();
            let line = format!(
                "{name} room={room} player=({:.0},{:.0}) map={:.1}% visited={}/{} secrets={}/{} abilities={:?}",
                pos.x,
                pos.y,
                stats.percent,
                stats.visited,
                stats.total,
                stats.secrets_found,
                stats.secrets_total,
                exploration.capabilities().abilities
            );
            println!("[world_slice] {line}");
            let _ = writeln!(script.log, "{line}");
            script.shots += 1;
            done = true;
        }
    }
    if done {
        script.step += 1;
        script.frames = 0;
    }
}

fn write_outputs(log: &str, world: &WorldDefinition, exploration: &Exploration) -> Vec<String> {
    let mut failures = Vec::new();
    let dir = out_dir();
    let report = validate(world);
    let mut text = format!(
        "errors: {:#?}\nwarnings: {:#?}\naccess: {:#?}\n",
        report.errors, report.warnings, report.access
    );
    let _ = writeln!(text, "{:#?}", exploration.stats(world));
    for (path, body) in [("route.log", log.to_owned()), ("validation.txt", text)] {
        if let Err(error) = std::fs::write(dir.join(path), body) {
            failures.push(format!("cannot write {path}: {error}"));
        }
    }
    match exploration.to_ron() {
        Ok(ron) => {
            if let Err(error) = std::fs::write(dir.join("exploration.ron"), ron) {
                failures.push(format!("cannot write exploration.ron: {error}"));
            }
        }
        Err(error) => failures.push(error.to_string()),
    }
    failures
}

fn finish(
    script: &mut Script,
    world: &WorldDefinition,
    exploration: &Exploration,
    exit: &mut MessageWriter<AppExit>,
) {
    if VERDICT.get().is_some() {
        return;
    }
    // V1.8: this slice covers the Vigil Gates region; the world also has
    // the Forgotten Galleries (examples/world_secrets_route.rs).
    let stats = exploration.region_stats(world, "vigil_gates");
    if stats.visited != stats.total {
        script
            .failures
            .push(format!("visited {}/{} rooms", stats.visited, stats.total));
    }
    if stats.secrets_found != stats.secrets_total {
        script.failures.push("secret not found".into());
    }
    let caps = exploration.capabilities();
    if !caps.abilities.contains(&Ability::VeilStep) || !caps.flags.contains("vigil_gates_cleared") {
        script.failures.push(format!("missing rewards: {caps:?}"));
    }
    if script.shots != ROUTE.iter().filter(|s| matches!(s, Shot(_))).count() {
        script.failures.push("missing screenshots".into());
    }
    let log = script.log.clone();
    script
        .failures
        .extend(write_outputs(&log, world, exploration));
    for failure in &script.failures {
        println!("[world_slice] FAIL {failure}");
    }
    let pass = script.failures.is_empty();
    println!(
        "[world_slice] RESULT: {}",
        if pass { "PASS" } else { "FAIL" }
    );
    let _ = VERDICT.set(pass);
    exit.write(AppExit::Success);
}

/// Interactive mode: write the exploration file when the window closes.
fn save_on_exit(
    mut exits: MessageReader<AppExit>,
    graph: Res<WorldGraph>,
    exploration: Res<Exploration>,
) {
    if exits.read().next().is_some() {
        if let Err(error) = std::fs::create_dir_all(out_dir()) {
            println!("[world_slice] {error}");
        }
        for failure in write_outputs("interactive session\n", &graph.0, &exploration) {
            println!("[world_slice] {failure}");
        }
    }
}

fn main() -> AppExit {
    let interactive = std::env::args().any(|a| a == "--interactive");
    let root = project_root();
    let (mut data, world) = match (GameData::load(&root), world()) {
        (Ok(data), Ok(world)) => (data, world),
        (Err(error), _) => {
            eprintln!("[world_slice] game data: {error}");
            return AppExit::error();
        }
        (_, Err(error)) => {
            eprintln!("[world_slice] world: {error}");
            return AppExit::error();
        }
    };
    let report = validate(&world);
    if !report.is_ok() {
        eprintln!("[world_slice] invalid world: {:#?}", report.errors);
        return AppExit::error();
    }
    install_world(&mut data, &world);
    // V1.7: the exploration and map plugins are part of `GamePlugin`.
    let mut app = dark_castle_rpg::app::build_with_world(data, world);
    app.insert_resource(SaveSlots(SaveStorage::new(out_dir().join("saves"))));
    if interactive {
        app.add_systems(Last, save_on_exit);
        return app.run();
    }
    app.insert_resource(TimeUpdateStrategy::ManualDuration(FRAME))
        .init_resource::<Script>()
        .add_systems(PreUpdate, drive_keys.after(InputSystems))
        .add_systems(
            Update,
            run_script.after(dark_castle_rpg::app::states::GameSet::Presentation),
        );
    let exit = app.run();
    match VERDICT.get() {
        Some(true) => exit,
        _ => AppExit::error(),
    }
}
