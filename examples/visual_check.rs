//! Visual check of the player's sprite animations (V1.3 + V1.4) in the real game.
//!
//! Runs the real app with a window, waits until the sprite textures are
//! loaded, then plays a fixed script with the real keyboard bindings (idle,
//! run, basic attack, dodge, crouch, guard, jump/fall/land, Crescent Slash,
//! Ember Bolt, Mending Light, Rising Edge, both facings) and takes a
//! screenshot at chosen moments. Shots marked `wait` are taken on the first
//! frame that shows the expected clip after their time (physics decides when
//! the jump peaks or the player lands). For every shot it records the
//! gameplay state and what the sprite shows (clip, atlas index, flip,
//! positions) and checks them; it exits with a non-zero code otherwise.
//!
//! Test-only shortcuts, never used by the game: every skill is granted at
//! start (Rising Edge is learned at level 3), and the hurt and dead states
//! are forced on the player's state machine at fixed times (no enemy is
//! needed). Screenshots and `shots.log` go to `target/visual_check/`; saves
//! are redirected there too (the script never saves, this is only a guard).
//!
//! Run with: `cargo run --example visual_check` (needs a graphical session).

use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::Duration;

use bevy::input::InputSystems;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::app::states::GameSet;
use dark_castle_rpg::core::constants::PLAYER_HURT_TIME;
use dark_castle_rpg::core::resources::{GameData, project_root};
use dark_castle_rpg::player::components::{ActionState, Player, SkillBook};
use dark_castle_rpg::player::visual::{PlayerVisual, VisualPlayback};
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::save::systems::SaveSlots;
use dark_castle_rpg::world::camera::MainCamera;

const FRAME: Duration = Duration::from_millis(16);

use KeyCode::{
    ArrowDown as D, ArrowLeft as L, ArrowRight as R, ArrowUp as U, KeyA, KeyC, KeyS, KeyX, KeyZ,
};

/// Keys held during `[start, end)` seconds after the sprites became visible.
const SCRIPT: &[(f32, f32, KeyCode)] = &[
    (0.50, 1.30, R),
    (1.60, 1.66, KeyX),
    (2.40, 2.46, KeyS),
    (3.20, 3.90, L),
    (4.30, 4.36, KeyX),
    // V1.4: turn right again, crouch, guard, jump.
    (5.40, 5.50, R),
    (5.80, 6.40, D),
    (6.70, 7.30, KeyA),
    (7.60, 7.75, KeyZ),
    // Crescent Slash ↓ ↘ → + ATTACK, Ember Bolt ↓ ↘ → + MAGIC,
    // Mending Light ↓ ↓ + MAGIC, Rising Edge ↓ ↑ + ATTACK (same timings as autoplay).
    (9.00, 9.12, D),
    (9.06, 9.20, R),
    (9.16, 9.22, KeyX),
    (10.20, 10.32, D),
    (10.26, 10.40, R),
    (10.36, 10.42, KeyC),
    (11.40, 11.46, D),
    (11.52, 11.60, D),
    (11.56, 11.62, KeyC),
    (12.80, 12.86, D),
    (12.86, 12.94, U),
    (12.90, 12.96, KeyX),
    // Facing left: Crescent Slash ↓ ↙ ← + ATTACK.
    (14.00, 14.10, L),
    (14.40, 14.52, D),
    (14.46, 14.60, L),
    (14.56, 14.62, KeyX),
];

/// States forced on the player (test shortcut, see the module docs).
const FORCE: &[(f32, Force)] = &[(15.60, Force::Hurt), (16.60, Force::Die)];

#[derive(Clone, Copy)]
enum Force {
    Hurt,
    Die,
}

struct Shot {
    at: f32,
    name: &'static str,
    clip: &'static str,
    flip: bool,
    /// Wait (up to 1.5 s) for the first frame showing `clip`.
    wait: bool,
}

const fn shot(at: f32, name: &'static str, clip: &'static str, flip: bool, wait: bool) -> Shot {
    Shot {
        at,
        name,
        clip,
        flip,
        wait,
    }
}

const SHOTS: &[Shot] = &[
    shot(0.30, "01_idle_right", "idle", false, false),
    shot(0.90, "02_run_right_a", "run", false, false),
    shot(0.95, "03_run_right_b", "run", false, false),
    shot(1.00, "04_run_right_c", "run", false, false),
    shot(1.62, "05_attack_start", "attack_basic", false, false),
    shot(1.72, "06_attack_mid", "attack_basic", false, false),
    shot(1.84, "07_attack_late", "attack_basic", false, false),
    shot(2.42, "08_dodge_start", "dodge", false, false),
    shot(2.52, "09_dodge_mid", "dodge", false, false),
    shot(2.64, "10_dodge_late", "dodge", false, false),
    shot(3.60, "11_run_left", "run", true, false),
    shot(4.32, "12_attack_left", "attack_basic", true, false),
    shot(5.00, "13_idle_left", "idle", true, false),
    shot(6.20, "14_crouch", "crouch", false, false),
    shot(7.10, "15_guard", "guard_start", false, false),
    shot(7.62, "16_jump_rise", "jump_rise", false, true),
    shot(7.80, "17_fall", "fall", false, true),
    shot(7.90, "18_land", "land", false, true),
    shot(9.16, "19_crescent_start", "slash_crescent", false, true),
    shot(9.36, "20_crescent_mid", "slash_crescent", false, false),
    shot(10.36, "21_ember_bolt", "cast_forward", false, true),
    shot(11.56, "22_mending_start", "cast_up", false, true),
    shot(11.90, "23_mending_mid", "cast_up", false, false),
    shot(12.90, "24_rising_start", "slash_rising", false, true),
    shot(13.12, "25_rising_top", "slash_rising", false, false),
    shot(14.56, "26_crescent_left", "slash_crescent", true, true),
    shot(15.60, "27_hurt", "hurt", true, true),
    shot(15.75, "28_hurt_mid", "hurt", true, false),
    shot(16.60, "29_death_start", "death", true, true),
    shot(17.80, "30_death_final", "death", true, false),
];
const END: f32 = 18.2;

#[derive(Resource, Default)]
struct Check {
    ready_at: Option<f32>,
    taken: usize,
    log: String,
    failures: Vec<String>,
}

static VERDICT: OnceLock<bool> = OnceLock::new();

fn out_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("target/visual_check")
}

fn drive_keys(time: Res<Time>, check: Res<Check>, mut keys: ResMut<ButtonInput<KeyCode>>) {
    let Some(ready) = check.ready_at else {
        return;
    };
    let now = time.elapsed_secs() - ready;
    for (_, _, key) in SCRIPT {
        let held = SCRIPT
            .iter()
            .any(|(s, e, k)| k == key && now >= *s && now < *e);
        if held && !keys.pressed(*key) {
            keys.press(*key);
        } else if !held && keys.pressed(*key) {
            keys.release(*key);
        }
    }
}

/// Grants every skill once the sprites are visible, and forces hurt/dead at
/// the scripted times (test shortcuts, see the module docs).
fn test_shortcuts(
    time: Res<Time>,
    check: Res<Check>,
    data: Res<GameData>,
    mut granted: Local<bool>,
    mut forced: Local<usize>,
    mut player: Query<(&mut ActionState, &mut SkillBook), With<Player>>,
) {
    let Some(ready) = check.ready_at else {
        return;
    };
    let Ok((mut state, mut book)) = player.single_mut() else {
        return;
    };
    if !*granted {
        for def in data.skills.iter() {
            book.learned.grant(def);
        }
        *granted = true;
    }
    let now = time.elapsed_secs() - ready;
    if let Some((at, force)) = FORCE.get(*forced)
        && now >= *at
    {
        match force {
            Force::Hurt => {
                state.hurt(PLAYER_HURT_TIME);
            }
            Force::Die => state.die(),
        }
        *forced += 1;
    }
}

fn observe(
    mut commands: Commands,
    time: Res<Time>,
    mut check: ResMut<Check>,
    players: Query<(&Transform, &ActionState), With<Player>>,
    visuals: Query<(&VisualPlayback, &Sprite, &Visibility, &GlobalTransform), With<PlayerVisual>>,
    camera: Query<&Transform, (With<MainCamera>, Without<Player>)>,
    mut exit: MessageWriter<AppExit>,
) {
    let now = time.elapsed_secs();
    let count = visuals.iter().count();
    if count > 1 {
        check
            .failures
            .push(format!("{count} player visuals at t={now:.3}"));
    }
    let Some((playback, sprite, visibility, global)) = visuals.iter().next() else {
        if now > 10.0 {
            check.failures.push("no player visual after 10 s".into());
            finish(&mut check, &mut exit);
        }
        return;
    };
    if check.ready_at.is_none() {
        if *visibility != Visibility::Hidden {
            check.ready_at = Some(now);
            println!("[visual_check] sprites visible at t={now:.3} s");
        } else if now > 10.0 {
            check
                .failures
                .push("sprite textures not loaded after 10 s".into());
            finish(&mut check, &mut exit);
        }
        return;
    }
    let ready = check.ready_at.unwrap_or(now);
    let t = now - ready;
    let Ok((player_tf, state)) = players.single() else {
        return;
    };
    let cam = camera.single().map(|c| c.translation).unwrap_or_default();
    let shown = playback.clip.as_deref().unwrap_or("-");
    if let Some(shot) = SHOTS.get(check.taken)
        && t >= shot.at
    {
        if shot.wait && shown != shot.clip {
            if t > shot.at + 1.5 {
                check.failures.push(format!(
                    "{}: {} never shown (shows {shown})",
                    shot.name, shot.clip
                ));
                check.taken += 1;
            }
        } else {
            let dir = out_dir();
            if let Err(error) = std::fs::create_dir_all(&dir) {
                check
                    .failures
                    .push(format!("cannot create {}: {error}", dir.display()));
            }
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(dir.join(format!("{}.png", shot.name))));
            let index = sprite
                .texture_atlas
                .as_ref()
                .map_or(usize::MAX, |a| a.index);
            let sole = global.translation();
            let line = format!(
                "{} t={t:.3} state={:?} clip={shown} index={index} flip={} player=({:.1},{:.1}) sole=({:.1},{:.1}) camera=({:.1},{:.1}) color={:?}",
                shot.name,
                state.state(),
                sprite.flip_x,
                player_tf.translation.x,
                player_tf.translation.y,
                sole.x,
                sole.y,
                cam.x,
                cam.y,
                sprite.color.to_srgba(),
            );
            println!("[visual_check] {line}");
            let _ = writeln!(check.log, "{line}");
            if shown != shot.clip || sprite.flip_x != shot.flip {
                check.failures.push(format!(
                    "{}: shows {shown} flip {}, expected {} flip {}",
                    shot.name, sprite.flip_x, shot.clip, shot.flip
                ));
            }
            check.taken += 1;
        }
    }
    if t >= END {
        finish(&mut check, &mut exit);
    }
}

fn finish(check: &mut Check, exit: &mut MessageWriter<AppExit>) {
    if VERDICT.get().is_some() {
        return;
    }
    if check.taken < SHOTS.len() && check.ready_at.is_some() {
        check
            .failures
            .push(format!("only {} of {} shots", check.taken, SHOTS.len()));
    }
    let log = out_dir().join("shots.log");
    let mut text = check.log.clone();
    for failure in &check.failures {
        let _ = writeln!(text, "FAIL {failure}");
    }
    if let Err(error) = std::fs::write(&log, text) {
        println!("[visual_check] cannot write {}: {error}", log.display());
    }
    for failure in &check.failures {
        println!("[visual_check] FAIL {failure}");
    }
    let pass = check.failures.is_empty();
    println!(
        "[visual_check] RESULT: {}",
        if pass { "PASS" } else { "FAIL" }
    );
    let _ = VERDICT.set(pass);
    exit.write(AppExit::Success);
}

fn main() -> AppExit {
    let root = project_root();
    let data = match GameData::load(&root) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("[visual_check] failed to load game data: {error}");
            return AppExit::error();
        }
    };
    let mut app = dark_castle_rpg::app::build(data);
    app.insert_resource(SaveSlots(SaveStorage::new(out_dir().join("saves"))))
        .insert_resource(TimeUpdateStrategy::ManualDuration(FRAME))
        .init_resource::<Check>()
        .add_systems(PreUpdate, drive_keys.after(InputSystems))
        .add_systems(Update, test_shortcuts.in_set(GameSet::Resolution))
        .add_systems(Update, observe.after(GameSet::Presentation));
    let exit = app.run();
    match VERDICT.get() {
        Some(true) => exit,
        _ => AppExit::error(),
    }
}
