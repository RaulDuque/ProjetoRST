//! V2.3 camera and scenery bench, in a window, with screenshots.
//!
//! **A bench, not a route:** the player is placed with `RoomRequest`
//! (`Arrival::Position`, the quick-load path) at fixed spots, so "before"
//! and "after" shots show the same room, position, state and resolution.
//! Modes:
//! - `before`: the V2.2 framing (view height 720, i.e. no zoom) and no
//!   scenery effects;
//! - `after`: the shipped `assets/data/camera.ron` (zoom, parallax, decor,
//!   lights, fog);
//! - `zoom<N>`: the shipped settings with a view height of N (sweep used to
//!   choose the zoom).
//!
//! Also: jump/fall framing, a window resize (1600x900 and 1024x768) with the
//! HUD, and repeated visits to check that scenery entities do not pile up.
//! Shots in `target/world_visual_showcase/`. Exit code != 0 if a check fails.
//!
//! Run with: `cargo run --example world_visual_showcase`.

use std::path::PathBuf;
use std::sync::OnceLock;
use std::time::Duration;

use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::time::TimeUpdateStrategy;
use bevy::window::{PrimaryWindow, WindowResolution};
use dark_castle_rpg::core::events::{Arrival, RoomRequest};
use dark_castle_rpg::core::resources::{GameData, project_root};
use dark_castle_rpg::player::components::Player;
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::save::systems::SaveSlots;
use dark_castle_rpg::world::camera::{CameraConfig, MainCamera, view_size};
use dark_castle_rpg::world::camera_rules::{CameraSettings, EffectToggles};
use dark_castle_rpg::world::maps::CurrentRoom;
use dark_castle_rpg::world::physics::RoomGeometry;
use dark_castle_rpg::world::scenery_systems::SceneryPiece;

const FRAME: Duration = Duration::from_millis(16);

static VERDICT: OnceLock<bool> = OnceLock::new();

/// Fixed spots: (room, position, label).
const SPOTS: [(&str, (f32, f32), &str); 4] = [
    ("vigil_gates_causeway", (1000.0, 63.0), "causeway"),
    ("fg_stair", (640.0, 63.0), "stair_bottom"),
    ("fg_stair", (820.0, 1305.0), "stair_high"),
    ("av_square", (900.0, 63.0), "square"),
];
/// A room without decoration of a region with a look (region identity).
const PLAIN: (&str, (f32, f32), &str) = ("vigil_gates_approach", (300.0, 63.0), "approach");

#[derive(Debug, Clone, Copy)]
enum Act {
    Wait(u32),
    Mode(&'static str, f32, bool),
    Goto(&'static str, (f32, f32)),
    Shot(&'static str),
    Press(KeyCode),
    Hold(KeyCode),
    Release(KeyCode),
    Resize(f32, f32),
    /// The view is inside the room and the player on screen.
    Framed,
    /// Scenery entities of the current room (none when effects are off).
    Scenery(bool),
    RememberCount,
    SameCount,
}

fn script() -> Vec<Act> {
    use Act::*;
    let mut acts = vec![Wait(60)];
    let modes: [(&'static str, f32, bool); 2] = [("before", 720.0, false), ("after", 0.0, true)];
    for (mode, height, effects) in modes {
        acts.push(Mode(mode, height, effects));
        for (room, at, label) in SPOTS.iter().copied().chain([PLAIN]) {
            acts.extend([Goto(room, at), Wait(50), Framed, Scenery(effects)]);
            acts.push(Shot(shot_name(mode, label)));
        }
    }
    // Zoom sweep (scenery on) in three rooms.
    for (name, height) in [
        ("zoom720", 720.0),
        ("zoom540", 540.0),
        ("zoom480", 480.0),
        ("zoom405", 405.0),
        ("zoom360", 360.0),
    ] {
        acts.push(Mode(name, height, true));
        for (room, at, label) in [SPOTS[0], SPOTS[2], SPOTS[3]] {
            acts.extend([
                Goto(room, at),
                Wait(50),
                Framed,
                Shot(shot_name(name, label)),
            ]);
        }
    }
    // Framing in motion with the shipped camera.
    acts.extend([
        Mode("after", 0.0, true),
        Goto("vigil_gates_causeway", (900.0, 63.0)),
        Wait(40),
        Hold(KeyCode::ArrowRight),
        Wait(20),
        Shot("motion_run"),
        Press(KeyCode::KeyZ),
        Wait(14),
        Framed,
        Shot("motion_jump_apex"),
        Wait(30),
        Release(KeyCode::ArrowRight),
        Wait(20),
        Goto("fg_stair", (820.0, 1305.0)),
        Wait(40),
        Hold(KeyCode::ArrowLeft),
        Wait(70),
        Release(KeyCode::ArrowLeft),
        Wait(4),
        Shot("motion_fall"),
        Wait(80),
        Framed,
        Shot("motion_landed"),
        // Window sizes: the zoom keeps the world height; the HUD keeps its
        // pixel size.
        Goto("av_square", (900.0, 63.0)),
        Wait(30),
        Resize(1600.0, 900.0),
        Wait(30),
        Framed,
        Shot("window_1600x900"),
        Resize(1024.0, 768.0),
        Wait(30),
        Framed,
        Shot("window_1024x768"),
        Resize(1280.0, 720.0),
        Wait(30),
        // Repeated visits: the scenery is replaced, never accumulated.
        RememberCount,
    ]);
    for _ in 0..6 {
        acts.extend([
            Goto("vigil_gates_causeway", (1000.0, 63.0)),
            Wait(5),
            Goto("av_square", (900.0, 63.0)),
            Wait(5),
        ]);
    }
    acts.extend([Wait(20), SameCount, Wait(20)]);
    acts
}

fn shot_name(mode: &str, label: &str) -> &'static str {
    Box::leak(format!("{mode}_{label}").into_boxed_str())
}

#[derive(Resource)]
struct Bench {
    acts: Vec<Act>,
    index: usize,
    waited: u32,
    release: Option<KeyCode>,
    failures: Vec<String>,
    exit_in: u32,
    out: PathBuf,
    shipped: CameraSettings,
    remembered: Option<usize>,
}

fn run_bench(
    mut commands: Commands,
    mut bench: ResMut<Bench>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut config: ResMut<CameraConfig>,
    mut requests: MessageWriter<RoomRequest>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    geometry: Res<RoomGeometry>,
    current: Res<CurrentRoom>,
    player: Query<&Transform, With<Player>>,
    camera: Query<&Transform, With<MainCamera>>,
    scenery: Query<&SceneryPiece>,
    all: Query<Entity>,
    mut exit: MessageWriter<AppExit>,
) {
    if let Some(key) = bench.release.take() {
        keys.release(key);
    }
    let Some(act) = bench.acts.get(bench.index).copied() else {
        if bench.exit_in > 0 {
            bench.exit_in -= 1;
            return;
        }
        if VERDICT.get().is_none() {
            for failure in &bench.failures {
                println!("[world_visual_showcase] FAIL {failure}");
            }
            let pass = bench.failures.is_empty();
            println!(
                "[world_visual_showcase] RESULT: {}",
                if pass { "PASS" } else { "FAIL" }
            );
            let _ = VERDICT.set(pass);
            exit.write(AppExit::Success);
        }
        return;
    };
    let at = bench.index;
    let mut next = true;
    match act {
        Act::Wait(frames) => {
            bench.waited += 1;
            next = bench.waited >= frames;
            if next {
                bench.waited = 0;
            }
        }
        Act::Mode(name, height, effects) => {
            let mut s = bench.shipped;
            if height > 0.0 {
                s.view_height = height;
            }
            if !effects {
                s.effects = EffectToggles::NONE;
            }
            config.0 = s;
            println!(
                "[world_visual_showcase] mode {name}: view height {}",
                s.view_height
            );
        }
        Act::Goto(room, (x, y)) => {
            requests.write(RoomRequest {
                room: room.into(),
                arrival: Arrival::Position(Vec2::new(x, y)),
            });
        }
        Act::Shot(name) => {
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(bench.out.join(format!("{name}.png"))));
            println!("[world_visual_showcase] {name} ({})", current.0);
        }
        Act::Press(key) => {
            keys.release(key);
            keys.press(key);
            bench.release = Some(key);
        }
        Act::Hold(key) => keys.press(key),
        Act::Release(key) => keys.release(key),
        Act::Resize(w, h) => {
            if let Ok(mut win) = window.single_mut() {
                win.resolution = WindowResolution::new(w as u32, h as u32);
            }
        }
        Act::Framed => {
            let view = view_size(&config.0, window.single().ok());
            if let (Ok(cam), Ok(p)) = (camera.single(), player.single()) {
                let c = cam.translation.truncate();
                let room = geometry.size;
                for (axis, c, room, view) in
                    [("x", c.x, room.x, view.x), ("y", c.y, room.y, view.y)]
                {
                    let ok = if room <= view {
                        (c - room / 2.0).abs() < 1.0
                    } else {
                        c >= view / 2.0 - 1.0 && c <= room - view / 2.0 + 1.0
                    };
                    if !ok {
                        bench
                            .failures
                            .push(format!("#{at} {}: view {axis} outside the room", current.0));
                    }
                }
                let d = p.translation.truncate() - c;
                if d.x.abs() > view.x / 2.0 || d.y.abs() > view.y / 2.0 {
                    bench
                        .failures
                        .push(format!("#{at} {}: player off screen", current.0));
                }
            }
        }
        Act::Scenery(expected) => {
            let n = scenery.iter().count();
            if (n > 0) != expected {
                bench.failures.push(format!(
                    "#{at} {}: {n} scenery pieces, expected any: {expected}",
                    current.0
                ));
            }
        }
        Act::RememberCount => bench.remembered = Some(all.iter().count()),
        Act::SameCount => {
            let n = all.iter().count();
            let before = bench.remembered;
            if before != Some(n) {
                bench
                    .failures
                    .push(format!("entities {before:?} -> {n} after repeated visits"));
            }
            println!(
                "[world_visual_showcase] entities after 6 round trips: {n} (before {:?})",
                bench.remembered
            );
        }
    }
    if next {
        bench.index += 1;
    }
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
            eprintln!("[world_visual_showcase] {error}");
            return AppExit::error();
        }
    };
    let shipped = match std::fs::read_to_string(root.join("assets/data/camera.ron"))
        .map_err(|e| e.to_string())
        .and_then(|t| CameraSettings::from_ron(&t).map_err(|e| e.to_string()))
    {
        Ok(s) => s,
        Err(error) => {
            eprintln!("[world_visual_showcase] camera.ron: {error}");
            return AppExit::error();
        }
    };
    let out = root.join("target/world_visual_showcase");
    let saves = out.join("saves");
    let _ = std::fs::remove_dir_all(&saves);
    if let Err(error) = std::fs::create_dir_all(&saves) {
        eprintln!("[world_visual_showcase] {}: {error}", saves.display());
        return AppExit::error();
    }
    let mut app = dark_castle_rpg::app::build_with_world(data, world);
    dark_castle_rpg::input::use_legacy_controls(&mut app);
    app.insert_resource(SaveSlots(SaveStorage::new(saves)))
        .insert_resource(CameraConfig(shipped))
        .insert_resource(TimeUpdateStrategy::ManualDuration(FRAME))
        .insert_resource(Bench {
            acts: script(),
            index: 0,
            waited: 0,
            release: None,
            failures: Vec::new(),
            exit_in: 30,
            out,
            shipped,
            remembered: None,
        })
        .add_systems(PreUpdate, run_bench.after(bevy::input::InputSystems));
    let exit = app.run();
    match VERDICT.get() {
        Some(true) => exit,
        _ => AppExit::error(),
    }
}
