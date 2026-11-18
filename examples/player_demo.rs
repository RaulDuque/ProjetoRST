//! Animated demonstration of the protagonist, captured from the real game (V1.5).
//!
//! Runs the real app with a window and plays two scripted sequences with the
//! real keyboard bindings, taking a screenshot every `EVERY` frames of 16 ms
//! (fixed time step, so the captures are reproducible):
//!
//! 1. idle → run → jump → fall → land → basic attack → dodge → hurt → death → revive;
//! 2. Crescent Slash → Rising Edge → Ember Bolt → Mending Light, then an air
//!    attack and a blocked hit on the guard.
//!
//! Test-only shortcuts, never used by the game (same as `visual_check`): every
//! skill is granted at start; hurt and death are forced on the player's state
//! machine (no enemy is needed); the blocked hit is a `DamageDealt` message
//! from the front while guarding, applied by the game's own `receive_damage`.
//! The revive is the real respawn key (R).
//!
//! Frames and `frames.log` (time, state, clip, player and camera positions)
//! go to `target/player_demo/seq<N>/`; `art-src/player/scripts/post/demo_v1_5.py`
//! crops them around the player and writes the GIFs. Saves are redirected to
//! `target/player_demo/saves` (the script never saves). The run fails if a
//! scripted clip or effect never appears.
//!
//! Run with: `cargo run --example player_demo` (needs a graphical session).

use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::Duration;

use bevy::input::InputSystems;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::app::states::GameSet;
use dark_castle_rpg::combat::damage::DamageOutcome;
use dark_castle_rpg::core::constants::PLAYER_HURT_TIME;
use dark_castle_rpg::core::events::DamageDealt;
use dark_castle_rpg::core::resources::{GameData, project_root};
use dark_castle_rpg::magic::elements::Element;
use dark_castle_rpg::player::components::{ActionState, Player, SkillBook};
use dark_castle_rpg::player::visual::vfx::PlayerVfx;
use dark_castle_rpg::player::visual::{PlayerVisual, VisualPlayback};
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::save::systems::SaveSlots;
use dark_castle_rpg::world::camera::MainCamera;

const FRAME: Duration = Duration::from_millis(16);
/// A screenshot every 3 frames: 48 ms, ~21 captures per second.
const EVERY: u64 = 3;

use KeyCode::{ArrowDown as D, ArrowRight as R, ArrowUp as U, KeyA, KeyC, KeyR, KeyS, KeyX, KeyZ};

#[derive(Clone, Copy)]
enum Force {
    Hurt,
    Die,
    BlockedHit,
}

struct Sequence {
    name: &'static str,
    keys: &'static [(f32, f32, KeyCode)],
    force: &'static [(f32, Force)],
    end: f32,
    /// Clips and effects that must appear at least once.
    expect: &'static [&'static str],
}

const SEQUENCES: &[Sequence] = &[
    Sequence {
        name: "seq1",
        keys: &[
            (0.70, 1.50, R),
            (1.70, 1.85, KeyZ),
            (2.80, 2.86, KeyX),
            (3.40, 3.46, KeyS),
        ],
        force: &[(4.10, Force::Hurt), (4.90, Force::Die)],
        end: 7.6,
        expect: &[
            "idle",
            "run",
            "jump_rise",
            "fall",
            "land",
            "attack_basic",
            "dodge",
            "hurt",
            "death",
            "vfx_revive",
        ],
    },
    Sequence {
        name: "seq2",
        keys: &[
            // Crescent Slash ↓ ↘ → + ATTACK
            (0.50, 0.62, D),
            (0.56, 0.70, R),
            (0.66, 0.72, KeyX),
            // Rising Edge ↓ ↑ + ATTACK
            (1.80, 1.86, D),
            (1.86, 1.94, U),
            (1.90, 1.96, KeyX),
            // Ember Bolt ↓ ↘ → + MAGIC
            (3.00, 3.12, D),
            (3.06, 3.20, R),
            (3.16, 3.22, KeyC),
            // Mending Light ↓ ↓ + MAGIC
            (4.20, 4.26, D),
            (4.32, 4.40, D),
            (4.36, 4.42, KeyC),
            // Air attack, then guard.
            (5.60, 5.75, KeyZ),
            (5.80, 5.86, KeyX),
            (7.00, 8.20, KeyA),
        ],
        force: &[(7.50, Force::BlockedHit)],
        end: 8.6,
        expect: &[
            "slash_crescent",
            "vfx_arc_white",
            "slash_rising",
            "vfx_arc_gold",
            "cast_forward",
            "cast_up",
            "vfx_light_motes",
            "attack_basic_air",
            "guard_start",
            "vfx_guard_spark",
        ],
    },
];

#[derive(Resource, Default)]
struct Demo {
    ready_at: Option<f32>,
    sequence: usize,
    started_at: f32,
    forced: usize,
    frame: u64,
    shots: usize,
    log: String,
    seen: Vec<String>,
    failures: Vec<String>,
}

static VERDICT: OnceLock<bool> = OnceLock::new();

fn out_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("target/player_demo")
}

fn local_time(time: &Time, demo: &Demo) -> Option<f32> {
    demo.ready_at.map(|_| time.elapsed_secs() - demo.started_at)
}

fn drive_keys(time: Res<Time>, demo: Res<Demo>, mut keys: ResMut<ButtonInput<KeyCode>>) {
    let (Some(now), Some(seq)) = (local_time(&time, &demo), SEQUENCES.get(demo.sequence)) else {
        return;
    };
    for (_, _, key) in seq.keys {
        let held = seq
            .keys
            .iter()
            .any(|(s, e, k)| k == key && now >= *s && now < *e);
        if held && !keys.pressed(*key) {
            keys.press(*key);
        } else if !held && keys.pressed(*key) {
            keys.release(*key);
        }
    }
    // The revive of sequence 1 uses the real respawn key.
    let revive = demo.sequence == 0 && (6.0..6.06).contains(&now);
    if revive && !keys.pressed(KeyR) {
        keys.press(KeyR);
    } else if !revive && keys.pressed(KeyR) {
        keys.release(KeyR);
    }
}

fn shortcuts(
    time: Res<Time>,
    data: Res<GameData>,
    mut demo: ResMut<Demo>,
    mut granted: Local<bool>,
    mut player: Query<(Entity, &mut ActionState, &mut SkillBook), With<Player>>,
    mut damage: MessageWriter<DamageDealt>,
) {
    let (Some(now), Some(seq)) = (local_time(&time, &demo), SEQUENCES.get(demo.sequence)) else {
        return;
    };
    let Ok((entity, mut state, mut book)) = player.single_mut() else {
        return;
    };
    if !*granted {
        for def in data.skills.iter() {
            book.learned.grant(def);
        }
        *granted = true;
    }
    if let Some((at, force)) = seq.force.get(demo.forced)
        && now >= *at
    {
        match force {
            Force::Hurt => {
                state.hurt(PLAYER_HURT_TIME);
            }
            Force::Die => state.die(),
            // Facing right, a hit travelling left comes from the front.
            Force::BlockedHit => {
                damage.write(DamageDealt {
                    attacker: entity,
                    target: entity,
                    outcome: DamageOutcome::Hit {
                        amount: 8,
                        critical: false,
                        element: Element::Physical,
                    },
                    direction: -1.0,
                    knockback: Default::default(),
                });
            }
        }
        demo.forced += 1;
    }
}

fn capture(
    mut commands: Commands,
    time: Res<Time>,
    mut demo: ResMut<Demo>,
    players: Query<(&Transform, &ActionState), With<Player>>,
    visuals: Query<(&VisualPlayback, &Visibility), With<PlayerVisual>>,
    effects: Query<&PlayerVfx>,
    camera: Query<&Transform, (With<MainCamera>, Without<Player>)>,
    mut exit: MessageWriter<AppExit>,
) {
    let now = time.elapsed_secs();
    let Ok((playback, visibility)) = visuals.single() else {
        if now > 10.0 {
            demo.failures.push("no player visual after 10 s".into());
            finish(&mut demo, &mut exit);
        }
        return;
    };
    if demo.ready_at.is_none() {
        if *visibility != Visibility::Hidden {
            demo.ready_at = Some(now);
            demo.started_at = now + 0.3;
        } else if now > 10.0 {
            demo.failures.push("sprite textures not loaded".into());
            finish(&mut demo, &mut exit);
        }
        return;
    }
    let Some(seq) = SEQUENCES.get(demo.sequence) else {
        return;
    };
    let t = now - demo.started_at;
    if t < 0.0 {
        return;
    }
    let Ok((player_tf, state)) = players.single() else {
        return;
    };
    let clip = playback.clip.clone().unwrap_or_default();
    let vfx: Vec<String> = effects.iter().map(|e| e.clip.clone()).collect();
    for name in std::iter::once(&clip).chain(vfx.iter()) {
        if !demo.seen.contains(name) {
            demo.seen.push(name.clone());
        }
    }
    if demo.frame.is_multiple_of(EVERY) {
        let dir = out_dir().join(seq.name);
        if let Err(error) = std::fs::create_dir_all(&dir) {
            demo.failures
                .push(format!("cannot create {}: {error}", dir.display()));
        }
        let shot = demo.shots;
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(dir.join(format!("{shot:04}.png"))));
        let cam = camera.single().map(|c| c.translation).unwrap_or_default();
        let line = format!(
            "{shot:04} t={t:.3} state={:?} clip={clip} vfx={} player=({:.1},{:.1}) camera=({:.1},{:.1})",
            state.state(),
            if vfx.is_empty() {
                "-".to_owned()
            } else {
                vfx.join("+")
            },
            player_tf.translation.x,
            player_tf.translation.y,
            cam.x,
            cam.y,
        );
        let _ = writeln!(demo.log, "{line}");
        demo.shots += 1;
    }
    demo.frame += 1;
    if t >= seq.end {
        let dir = out_dir().join(seq.name);
        if let Err(error) = std::fs::write(dir.join("frames.log"), &demo.log) {
            demo.failures.push(format!("cannot write the log: {error}"));
        }
        for wanted in seq.expect {
            if !demo.seen.iter().any(|s| s == wanted) {
                demo.failures
                    .push(format!("{}: {wanted} never shown", seq.name));
            }
        }
        println!(
            "[player_demo] {}: {} captures, seen {:?}",
            seq.name, demo.shots, demo.seen
        );
        demo.sequence += 1;
        demo.started_at = now + 0.6;
        demo.forced = 0;
        demo.frame = 0;
        demo.shots = 0;
        demo.log.clear();
        demo.seen.clear();
        if demo.sequence >= SEQUENCES.len() {
            finish(&mut demo, &mut exit);
        }
    }
}

fn finish(demo: &mut Demo, exit: &mut MessageWriter<AppExit>) {
    if VERDICT.get().is_some() {
        return;
    }
    for failure in &demo.failures {
        println!("[player_demo] FAIL {failure}");
    }
    let pass = demo.failures.is_empty();
    println!(
        "[player_demo] RESULT: {}",
        if pass { "PASS" } else { "FAIL" }
    );
    let _ = VERDICT.set(pass);
    exit.write(AppExit::Success);
}

fn main() -> AppExit {
    let data = match GameData::load(&project_root()) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("[player_demo] failed to load game data: {error}");
            return AppExit::error();
        }
    };
    let mut app = dark_castle_rpg::app::build(data);
    dark_castle_rpg::input::use_legacy_controls(&mut app);
    app.insert_resource(SaveSlots(SaveStorage::new(out_dir().join("saves"))))
        .insert_resource(TimeUpdateStrategy::ManualDuration(FRAME))
        .init_resource::<Demo>()
        .add_systems(PreUpdate, drive_keys.after(InputSystems))
        .add_systems(Update, shortcuts.in_set(GameSet::Commands))
        .add_systems(Update, capture.after(GameSet::Presentation));
    let exit = app.run();
    match VERDICT.get() {
        Some(true) => exit,
        _ => AppExit::error(),
    }
}
