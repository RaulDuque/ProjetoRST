//! N3.8 evidence: the experimental protagonist (N3.5 idle + N3.6 run at
//! 15 fps) in the **real game** (world, room, physics, camera, HUD), in a
//! window, every move a real key press (classic controls).
//!
//! The game is built by `app::build_for_protagonist`, the same function
//! `cargo run` uses with `DARK_CASTLE_PROTAGONIST=n3`. `--production`
//! records the same script with the production V2.2 art (comparison);
//! `--view 360` switches the camera to the experimental 360 view in memory
//! (`assets/data/camera.ron` is never written).
//!
//! The game steps a fixed 20 ms per update, so one captured update is 20 ms
//! of game time and the GIFs (2 cs per frame) play in real time. Every
//! update of a sequence is captured to
//! `target/n3_8_production_run/<art>-<view>/<sequence>/NNNN.png` and logged
//! (`frames.log`: update, clip, frame, placeholder, flip, x, vx, state, y,
//! camera position).
//! `art-src/n3-8-production-run/scripts/make_evidence.py` builds the GIFs.
//!
//! Checks (exit code != 0 on failure): the run shows the six frames at
//! 15 fps, idle and run use the experimental art, other states hold the
//! idle pose with a named placeholder, facing left mirrors.
//!
//! Run with: `cargo run --example n3_8_production_run [-- --production] [-- --view 360]`.

use std::fmt::Write as _;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::time::Duration;

use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::core::events::{Arrival, EquipOp, EquipRequest, RoomRequest};
use dark_castle_rpg::core::resources::{GameData, project_root};
use dark_castle_rpg::inventory::equipment::EquipmentSlot;
use dark_castle_rpg::player::components::{ActionState, Belongings, Player};
use dark_castle_rpg::player::visual::art_profile::ProtagonistArt;
use dark_castle_rpg::player::visual::{PlayerSpriteSet, PlayerVisual, VisualPlayback};
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::save::systems::SaveSlots;
use dark_castle_rpg::world::camera::{CameraConfig, MainCamera};
use dark_castle_rpg::world::physics::Velocity;

const FRAME: Duration = Duration::from_millis(20);
const TAG: &str = "[n3_8_production_run]";

static VERDICT: OnceLock<bool> = OnceLock::new();

#[derive(Debug, Clone, Copy)]
enum Act {
    Wait(u32),
    Hold(KeyCode),
    Release(KeyCode),
    /// Press for one update.
    Press(KeyCode),
    /// Start capturing every update into a sequence.
    Record(&'static str),
    Stop,
    Shot(&'static str),
    Equip(EquipmentSlot, &'static str),
    Give(&'static str),
    /// Move to a room through one of its doors (the normal transition).
    Room(&'static str, &'static str),
}

fn script() -> Vec<Act> {
    use Act::*;
    use KeyCode::{ArrowLeft as L, ArrowRight as R, KeyA, KeyS, KeyX, KeyZ, ShiftLeft as Run};
    // The causeway floor is free from the door (x ~ 70) to a block at x 700;
    // one update at 310 u/s is 6.2 u, so the script turns around to stay
    // inside (and away from the left door).
    vec![
        Wait(40),
        // A long room: the start room has a block 300 u from the spawn.
        Room("vigil_gates_causeway", "from_approach"),
        Wait(60),
        Shot("idle_right"),
        // Idle -> run (right), then a free run.
        Record("idle_to_run"),
        Wait(15),
        Hold(Run),
        Hold(R),
        Wait(35),
        Stop,
        Record("run_right"),
        Wait(40),
        Stop,
        // Run -> idle.
        Record("run_to_idle"),
        Wait(5),
        Release(R),
        Release(Run),
        Wait(30),
        Stop,
        // Run left, then turn right while running.
        Hold(Run),
        Hold(L),
        Wait(10),
        Record("run_left"),
        Wait(45),
        Stop,
        Record("run_turn"),
        Wait(10),
        Release(L),
        Hold(R),
        Wait(30),
        Stop,
        // Run -> jump (right).
        Record("run_to_jump"),
        Wait(10),
        Press(KeyZ),
        Wait(35),
        Stop,
        Release(R),
        Hold(L),
        Wait(10),
        // Run -> attack (left).
        Record("run_to_attack"),
        Wait(10),
        Press(KeyX),
        Wait(30),
        Stop,
        // Run -> guard (left).
        Record("run_to_guard"),
        Wait(10),
        Hold(KeyA),
        Wait(25),
        Release(KeyA),
        Wait(10),
        Stop,
        // Run -> dodge (right).
        Release(L),
        Hold(R),
        Record("run_to_dodge"),
        Wait(10),
        Press(KeyS),
        Wait(30),
        Stop,
        Release(R),
        Release(Run),
        Wait(30),
        // Existing equipment while running: the cape (procedural layer).
        Give("emberlined_cloak"),
        Equip(EquipmentSlot::Body, "emberlined_cloak"),
        Wait(10),
        Hold(Run),
        Hold(L),
        Wait(15),
        Record("run_with_cape"),
        Wait(45),
        Stop,
        Release(L),
        Release(Run),
        Wait(20),
        Shot("idle_cape"),
    ]
}

#[derive(Resource)]
struct Bench {
    acts: Vec<Act>,
    index: usize,
    waited: u32,
    release: Option<KeyCode>,
    recording: Option<&'static str>,
    sequence_frame: u32,
    update: u64,
    log: String,
    out: PathBuf,
    experimental: bool,
    failures: Vec<String>,
    /// Per sequence: (clip, frame) of every captured update.
    samples: Vec<(
        &'static str,
        Option<String>,
        usize,
        Option<String>,
        bool,
        f32,
    )>,
    exit_in: u32,
}

fn run_bench(
    mut commands: Commands,
    mut bench: ResMut<Bench>,
    data: Res<GameData>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut requests: MessageWriter<EquipRequest>,
    mut rooms: MessageWriter<RoomRequest>,
    mut player: Query<&mut Belongings, With<Player>>,
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
            finish(&mut bench);
            let pass = bench.failures.is_empty();
            for failure in &bench.failures {
                println!("{TAG} FAIL {failure}");
            }
            println!("{TAG} RESULT: {}", if pass { "PASS" } else { "FAIL" });
            let _ = VERDICT.set(pass);
            exit.write(AppExit::Success);
        }
        return;
    };
    let mut next = true;
    match act {
        Act::Wait(frames) => {
            bench.waited += 1;
            next = bench.waited >= frames;
            if next {
                bench.waited = 0;
            }
        }
        Act::Hold(key) => keys.press(key),
        Act::Release(key) => keys.release(key),
        Act::Press(key) => {
            keys.release(key);
            keys.press(key);
            bench.release = Some(key);
        }
        Act::Record(name) => {
            bench.recording = Some(name);
            bench.sequence_frame = 0;
            let _ = std::fs::create_dir_all(bench.out.join(name));
        }
        Act::Stop => bench.recording = None,
        Act::Shot(name) => {
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(bench.out.join(format!("{name}.png"))));
        }
        Act::Give(id) => {
            if let Ok(mut b) = player.single_mut()
                && let Err(error) = b.inventory.add(id, 1, &data.items)
            {
                bench.failures.push(format!("give {id}: {error}"));
            }
        }
        Act::Room(room, door) => {
            rooms.write(RoomRequest {
                room: room.into(),
                arrival: Arrival::Door(door.into()),
            });
        }
        Act::Equip(slot, id) => {
            requests.write(EquipRequest {
                slot,
                op: EquipOp::Equip(id.into()),
            });
        }
    }
    if next {
        bench.index += 1;
    }
}

/// After the frame's animation: log the shown clip and capture the window.
fn capture(
    mut commands: Commands,
    mut bench: ResMut<Bench>,
    set: Option<Res<PlayerSpriteSet>>,
    players: Query<(&Transform, &Velocity, &ActionState), With<Player>>,
    visuals: Query<(&VisualPlayback, &Sprite), With<PlayerVisual>>,
    cameras: Query<&Transform, With<MainCamera>>,
) {
    bench.update += 1;
    let Some(name) = bench.recording else {
        return;
    };
    let (Some(set), Ok((t, v, a)), Ok((p, sprite))) = (set, players.single(), visuals.single())
    else {
        return;
    };
    let first = p
        .clip
        .as_deref()
        .and_then(|k| set.manifest.clips.get(k))
        .map_or(0, |c| c.first as usize);
    let frame = p.index.saturating_sub(first);
    let n = bench.sequence_frame;
    let cam = cameras.single().map_or(Vec3::ZERO, |c| c.translation);
    let line = format!(
        "{name} {n:04} update={} clip={} frame={frame} placeholder={} flip={} x={:.1} vx={:.1} state={:?} y={:.1} cam={:.1},{:.1}\n",
        bench.update,
        p.clip.as_deref().unwrap_or("-"),
        p.placeholder_for.as_deref().unwrap_or("-"),
        sprite.flip_x,
        t.translation.x,
        v.0.x,
        a.state(),
        t.translation.y,
        cam.x,
        cam.y,
    );
    bench.log.push_str(&line);
    bench.samples.push((
        name,
        p.clip.clone(),
        frame,
        p.placeholder_for.clone(),
        sprite.flip_x,
        v.0.x,
    ));
    let path = bench.out.join(name).join(format!("{n:04}.png"));
    commands
        .spawn(Screenshot::primary_window())
        .observe(save_to_disk(path));
    bench.sequence_frame += 1;
}

/// Objective checks on the recorded samples, then the log file.
fn finish(bench: &mut Bench) {
    let mut failures = Vec::new();
    let seq = |name: &str| -> Vec<_> {
        bench
            .samples
            .iter()
            .filter(|s| s.0 == name)
            .cloned()
            .collect()
    };
    let run_right = seq("run_right");
    if bench.experimental {
        // 40 updates of 20 ms = 0.8 s = 12 frame changes at 15 fps.
        let frames: Vec<usize> = run_right.iter().map(|s| s.2).collect();
        let changes = frames.windows(2).filter(|w| w[0] != w[1]).count();
        let seconds = frames.len().saturating_sub(1) as f32 * FRAME.as_secs_f32();
        let expected = seconds * 15.0;
        if (changes as f32 - expected).abs() > 1.5 {
            failures.push(format!(
                "run_right: {changes} frame changes in {seconds:.2} s (15 fps: {expected:.1})"
            ));
        }
        if (0..6).any(|f| !frames.contains(&f)) {
            failures.push("run_right: not all six frames shown".into());
        }
        if run_right
            .iter()
            .any(|s| s.1.as_deref() != Some("run") || s.4)
        {
            failures.push("run_right: not the run clip facing right".into());
        }
        if seq("run_left")
            .iter()
            .any(|s| s.1.as_deref() != Some("run") || !s.4)
        {
            failures.push("run_left: not the mirrored run clip".into());
        }
        for (name, wanted) in [
            ("run_to_jump", "jump_rise"),
            ("run_to_attack", "attack_basic"),
            ("run_to_guard", "guard_start"),
            ("run_to_dodge", "dodge"),
        ] {
            let samples = seq(name);
            if !samples.iter().any(|s| s.3.as_deref() == Some(wanted)) {
                failures.push(format!("{name}: no placeholder for {wanted}"));
            }
            if samples
                .iter()
                .any(|s| s.3.is_some() && s.1.as_deref() != Some("idle"))
            {
                failures.push(format!("{name}: a placeholder that is not the idle pose"));
            }
        }
    }
    if run_right
        .iter()
        .any(|s| s.5.abs() > 0.0 && (s.5 - 310.0).abs() > 1e-3)
    {
        failures.push("run_right: speed other than 310 u/s".into());
    }
    let _ = writeln!(bench.log, "# failures: {}", failures.len());
    let log = std::mem::take(&mut bench.log);
    if let Err(error) = std::fs::write(bench.out.join("frames.log"), log) {
        failures.push(format!("frames.log: {error}"));
    }
    bench.failures.extend(failures);
}

fn main() -> AppExit {
    let args: Vec<String> = std::env::args().collect();
    let production = args.iter().any(|a| a == "--production");
    let view_360 = args.windows(2).any(|w| w[0] == "--view" && w[1] == "360");
    let root = project_root();
    let loaded = GameData::load(&root)
        .map_err(|e| e.to_string())
        .and_then(|mut data| {
            dark_castle_rpg::app::load_world(&root, &mut data).map(|world| (data, world))
        });
    let (data, world) = match loaded {
        Ok(loaded) => loaded,
        Err(error) => {
            eprintln!("{TAG} {error}");
            return AppExit::error();
        }
    };
    let art = if production {
        ProtagonistArt::Production
    } else {
        ProtagonistArt::N3Experimental
    };
    let kind = format!(
        "{}-{}",
        if production { "production" } else { "n3" },
        if view_360 { "360" } else { "405" }
    );
    let out = root.join("target/n3_8_production_run").join(&kind);
    let _ = std::fs::remove_dir_all(&out);
    let saves = out.join("saves");
    if let Err(error) = std::fs::create_dir_all(&saves) {
        eprintln!("{TAG} {}: {error}", saves.display());
        return AppExit::error();
    }
    println!("{TAG} art: {art:?}, view: {kind}");
    let mut app = dark_castle_rpg::app::build_for_protagonist(data, world, &root, art);
    dark_castle_rpg::input::use_legacy_controls(&mut app);
    app.insert_resource(SaveSlots(SaveStorage::new(saves)))
        .insert_resource(TimeUpdateStrategy::ManualDuration(FRAME))
        .insert_resource(Bench {
            acts: script(),
            index: 0,
            waited: 0,
            release: None,
            recording: None,
            sequence_frame: 0,
            update: 0,
            log: String::new(),
            out,
            experimental: !production,
            failures: Vec::new(),
            samples: Vec::new(),
            exit_in: 30,
        })
        .add_systems(PreUpdate, run_bench.after(bevy::input::InputSystems))
        .add_systems(Last, capture);
    if view_360 {
        app.add_systems(PostStartup, |config: Option<ResMut<CameraConfig>>| {
            if let Some(mut config) = config {
                config.0.view_height = 360.0;
            }
        });
    }
    let exit = app.run();
    match VERDICT.get() {
        Some(true) => exit,
        _ => AppExit::error(),
    }
}
