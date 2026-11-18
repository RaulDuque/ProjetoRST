//! D23: climb the Reliquary stairs with the jump key, then fight the Ashen
//! Sentinel with no scripted heal. A loss is a recorded result.
//!
//! The room request only puts the player at the foot of the stair. Saves
//! stay in `target/d23/saves/`.
//!
//! Run: `cargo run --example d23_play_probe`

use std::path::PathBuf;
use std::sync::OnceLock;
use std::time::Duration;

use bevy::input::InputSystems;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::combat::hitboxes::{Hitbox, Team};
use dark_castle_rpg::core::events::{Arrival, RoomRequest};
use dark_castle_rpg::core::resources::{GameData, project_root};
use dark_castle_rpg::enemies::boss::{BossPattern, BossPhase};
use dark_castle_rpg::enemies::components::{Boss, BossRuntime, EnemyBrain, EnemyHealth};
use dark_castle_rpg::magic::projectiles::Projectile;
use dark_castle_rpg::player::components::{ActionState, Player, Vitals};
use dark_castle_rpg::player::state::{Facing, PlayerState};
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::save::systems::SaveSlots;
use dark_castle_rpg::world::maps::CurrentRoom;
use dark_castle_rpg::world::physics::{FacingDirection, Grounded};

const FRAME: Duration = Duration::from_millis(16);
const RELIQUARY: &str = "vigil_gates_reliquary";
const COURT: &str = "vigil_gates_ashen_court";

static VERDICT: OnceLock<bool> = OnceLock::new();

fn evidence_dir() -> PathBuf {
    project_root().join("docs/development/d23/evidence")
}

fn out_dir() -> PathBuf {
    project_root().join("target/d23")
}

#[derive(Resource, Default)]
struct Keys {
    jump_held: bool,
    right_held: bool,
    attack: bool,
    dodge: bool,
    respawn: bool,
    release_attack: bool,
    release_dodge: bool,
    release_respawn: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Act {
    Boot,
    Climb,
    Approach,
    Fight,
    Dead,
    Exit,
}

#[derive(Resource)]
struct Probe {
    frame: u32,
    act: Act,
    jump_hold: u32,
    airborne: bool,
    landings: u32,
    shot_mid: bool,
    shot_top: bool,
    shot_boss: bool,
    swing_wait: u32,
    attacks: u32,
    dodges: u32,
    player_start: i32,
    player_low: i32,
    boss_start: i32,
    boss_low: i32,
    saw_slash: bool,
    saw_lunge: bool,
    saw_wave: bool,
    saw_phase2: bool,
    slash_shot: bool,
    lunge_shot: bool,
    wave_shot: bool,
    flash_shot: bool,
    phase_shot: bool,
    finished: bool,
    /// Frames to keep rendering after the result so the last screenshots flush.
    drain: u32,
}

fn shot(commands: &mut Commands, name: &str) {
    let _ = std::fs::create_dir_all(evidence_dir());
    commands
        .spawn(Screenshot::primary_window())
        .observe(save_to_disk(evidence_dir().join(format!("{name}.png"))));
}

fn apply_keys(mut want: ResMut<Keys>, mut keys: ResMut<ButtonInput<KeyCode>>) {
    if want.jump_held {
        keys.press(KeyCode::KeyZ);
    } else {
        keys.release(KeyCode::KeyZ);
    }
    if want.right_held {
        keys.press(KeyCode::ArrowRight);
    } else {
        keys.release(KeyCode::ArrowRight);
    }
    let mut attack = want.attack;
    let mut release_attack = want.release_attack;
    let mut dodge = want.dodge;
    let mut release_dodge = want.release_dodge;
    let mut respawn = want.respawn;
    let mut release_respawn = want.release_respawn;
    pulse(&mut keys, KeyCode::KeyX, &mut attack, &mut release_attack);
    pulse(&mut keys, KeyCode::KeyS, &mut dodge, &mut release_dodge);
    pulse(&mut keys, KeyCode::KeyR, &mut respawn, &mut release_respawn);
    want.attack = attack;
    want.release_attack = release_attack;
    want.dodge = dodge;
    want.release_dodge = release_dodge;
    want.respawn = respawn;
    want.release_respawn = release_respawn;
}

fn pulse(keys: &mut ButtonInput<KeyCode>, key: KeyCode, pressed: &mut bool, release: &mut bool) {
    if *pressed {
        keys.release(key);
        keys.press(key);
        *pressed = false;
        *release = true;
    } else if *release {
        keys.release(key);
        *release = false;
    }
}

fn face(facing: &mut FacingDirection, boss_x: f32, player_x: f32, toward: bool) {
    let boss_on_right = boss_x > player_x;
    let look_right = if toward {
        boss_on_right
    } else {
        !boss_on_right
    };
    facing.0 = if look_right {
        Facing::Right
    } else {
        Facing::Left
    };
}

fn drive(
    mut commands: Commands,
    mut probe: ResMut<Probe>,
    mut keys: ResMut<Keys>,
    room: Res<CurrentRoom>,
    mut rooms: MessageWriter<RoomRequest>,
    player: Query<(&Transform, &Vitals, &Grounded, &ActionState), (With<Player>, Without<Boss>)>,
    mut facing: Query<&mut FacingDirection, (With<Player>, Without<Boss>)>,
    bosses: Query<
        (&EnemyHealth, &BossRuntime, &EnemyBrain, &Transform),
        (With<Boss>, Without<Player>),
    >,
    bolts: Query<&Hitbox, With<Projectile>>,
    mut exit: MessageWriter<AppExit>,
) {
    if probe.finished {
        probe.drain = probe.drain.saturating_sub(1);
        if probe.drain == 0 {
            exit.write(if VERDICT.get().copied().unwrap_or(false) {
                AppExit::Success
            } else {
                AppExit::error()
            });
        }
        return;
    }
    probe.frame += 1;
    let frame = probe.frame;
    keys.jump_held = false;
    keys.right_held = false;

    let Ok((transform, vitals, grounded, state)) = player.single() else {
        return;
    };
    let position = transform.translation.truncate();
    let hp = vitals.hp.current();
    let on_ground = grounded.on_ground;
    let dead = state.state() == PlayerState::Dead;
    let boss = bosses.iter().next().map(|(health, runtime, brain, t)| {
        (
            health.current(),
            runtime.clone(),
            brain.windup,
            brain.flash,
            t.translation.x,
        )
    });

    match probe.act {
        Act::Boot if frame >= 30 => {
            rooms.write(RoomRequest {
                room: RELIQUARY.into(),
                arrival: Arrival::Position(Vec2::new(630.0, 63.0)),
            });
            probe.act = Act::Climb;
        }
        Act::Climb => {
            if room.0 == COURT {
                shot(&mut commands, "04_court_arrival");
                probe.act = Act::Approach;
                probe.player_start = hp;
                probe.player_low = hp;
                if let Some((boss_hp, _, _, _, _)) = boss {
                    probe.boss_start = boss_hp;
                    probe.boss_low = boss_hp;
                }
                return;
            }
            if room.0 != RELIQUARY {
                return;
            }
            if frame == 50 || (frame > 50 && frame < 70 && probe.landings == 0) {
                shot(&mut commands, "01_stair_foot");
            }
            if probe.jump_hold > 0 {
                keys.jump_held = true;
                probe.jump_hold -= 1;
                if !on_ground {
                    probe.airborne = true;
                }
                return;
            }
            if !on_ground {
                probe.airborne = true;
                return;
            }
            if probe.airborne {
                probe.airborne = false;
                probe.landings += 1;
                if !probe.shot_mid && position.y > 250.0 {
                    shot(&mut commands, "02_stair_climb");
                    probe.shot_mid = true;
                }
                if !probe.shot_top && position.y > 520.0 {
                    shot(&mut commands, "03_stair_top");
                    probe.shot_top = true;
                }
            }
            if probe.landings < 8 && frame > 40 {
                keys.jump_held = true;
                probe.jump_hold = 24;
                probe.airborne = false;
            } else if frame > 2500 {
                finish(&mut probe, false, "climb did not reach the court");
            }
        }
        Act::Approach => {
            probe.player_low = probe.player_low.min(hp);
            let Some((_, _, _, _, boss_x)) = boss else {
                finish(&mut probe, false, "boss missing on arrival");
                return;
            };
            let gap = (boss_x - position.x).abs();
            if gap > 48.0 {
                keys.right_held = boss_x > position.x;
                if gap < 220.0 && !probe.shot_boss {
                    shot(&mut commands, "04b_boss_in_frame");
                    probe.shot_boss = true;
                }
            } else {
                keys.right_held = false;
                probe.act = Act::Fight;
            }
            if frame > 4000 {
                finish(&mut probe, false, "could not reach the boss");
            }
        }
        Act::Fight => {
            if dead {
                shot(&mut commands, "10_defeat");
                probe.act = Act::Dead;
                probe.jump_hold = 20;
                return;
            }
            let Some((boss_hp, runtime, windup, flash, boss_x)) = boss else {
                shot(&mut commands, "10_victory");
                finish(&mut probe, true, "boss gone");
                return;
            };
            probe.player_low = probe.player_low.min(hp);
            probe.boss_low = probe.boss_low.min(boss_hp);
            probe.saw_slash |= runtime.resolved_slash;
            probe.saw_lunge |= runtime.resolved_lunge;
            probe.saw_phase2 |= runtime.phase == BossPhase::Two;
            let bolt = bolts.iter().any(|hit| hit.team == Team::Enemy);
            probe.saw_wave |= runtime.last_resolved == Some(BossPattern::AshWave) || bolt;
            if let Ok(mut face_dir) = facing.single_mut() {
                let tell = windup.is_some_and(|left| (0.16..0.34).contains(&left));
                if tell {
                    face(&mut face_dir, boss_x, position.x, false);
                    if probe.swing_wait == 0 {
                        keys.dodge = true;
                        probe.dodges += 1;
                        probe.swing_wait = 18;
                    }
                } else {
                    face(&mut face_dir, boss_x, position.x, true);
                    let gap = (boss_x - position.x).abs();
                    if gap > 52.0 {
                        keys.right_held = boss_x > position.x;
                    } else if probe.swing_wait == 0 {
                        keys.attack = true;
                        probe.attacks += 1;
                        probe.swing_wait = 28;
                    }
                }
            }
            probe.swing_wait = probe.swing_wait.saturating_sub(1);
            if windup.is_some() && flash > 0.0 && !probe.flash_shot {
                shot(&mut commands, "08_tell_under_hit");
                probe.flash_shot = true;
            }
            if runtime.next_pattern == BossPattern::FrontalSlash
                && windup.is_some()
                && !probe.slash_shot
            {
                shot(&mut commands, "05_frontal_slash");
                probe.slash_shot = true;
            }
            if runtime.lunge && !probe.lunge_shot {
                shot(&mut commands, "06_lunge");
                probe.lunge_shot = true;
            }
            if runtime.phase_lock > 0.0 && !probe.phase_shot {
                shot(&mut commands, "09_phase2");
                probe.phase_shot = true;
            }
            if (runtime.last_resolved == Some(BossPattern::AshWave) || bolt) && !probe.wave_shot {
                shot(&mut commands, "07_ash_wave");
                probe.wave_shot = true;
            }
            if boss_hp == 0 {
                shot(&mut commands, "10_victory");
                finish(&mut probe, true, "boss defeated");
            } else if frame > 9000 {
                finish(&mut probe, true, "fight timeout");
            }
        }
        Act::Dead => {
            probe.jump_hold = probe.jump_hold.saturating_sub(1);
            if probe.jump_hold == 10 {
                keys.respawn = true;
            }
            if probe.jump_hold == 0 {
                shot(&mut commands, "11_respawn");
                finish(&mut probe, true, "player died");
            }
        }
        Act::Boot | Act::Exit => {}
    }
}

fn finish(probe: &mut Probe, ok: bool, result: &str) {
    println!(
        "[d23_play_probe] RESULT: {result} frames={} attacks={} dodges={} player {}->{} (low {}) boss {}->{} (low {}) slash={} lunge={} wave={} phase2={}",
        probe.frame,
        probe.attacks,
        probe.dodges,
        probe.player_start,
        probe.player_low,
        probe.player_low,
        probe.boss_start,
        probe.boss_low,
        probe.boss_low,
        probe.saw_slash,
        probe.saw_lunge,
        probe.saw_wave,
        probe.saw_phase2,
    );
    probe.finished = true;
    probe.act = Act::Exit;
    probe.drain = 45;
    let _ = VERDICT.set(ok);
}

fn main() -> AppExit {
    let root = project_root();
    let loaded = GameData::load(&root)
        .map_err(|e| e.to_string())
        .and_then(|mut data| {
            dark_castle_rpg::app::load_world(&root, &mut data).map(|world| (data, world))
        });
    let (data, world) = match loaded {
        Ok(value) => value,
        Err(error) => {
            eprintln!("[d23_play_probe] {error}");
            return AppExit::error();
        }
    };
    let saves = out_dir().join("saves");
    let _ = std::fs::remove_dir_all(&saves);
    let _ = std::fs::create_dir_all(&saves);
    let _ = std::fs::create_dir_all(evidence_dir());
    println!(
        "[d23_play_probe] saves {}; evidence {}",
        saves.display(),
        evidence_dir().display()
    );
    let mut app = dark_castle_rpg::app::build_with_world(data, world);
    app.insert_resource(SaveSlots(SaveStorage::new(saves)))
        .insert_resource(TimeUpdateStrategy::ManualDuration(FRAME))
        .insert_resource(Keys::default())
        .insert_resource(Probe {
            frame: 0,
            act: Act::Boot,
            jump_hold: 0,
            airborne: false,
            landings: 0,
            shot_mid: false,
            shot_top: false,
            shot_boss: false,
            swing_wait: 0,
            attacks: 0,
            dodges: 0,
            player_start: 0,
            player_low: 0,
            boss_start: 0,
            boss_low: 0,
            saw_slash: false,
            saw_lunge: false,
            saw_wave: false,
            saw_phase2: false,
            slash_shot: false,
            lunge_shot: false,
            wave_shot: false,
            flash_shot: false,
            phase_shot: false,
            finished: false,
            drain: 0,
        })
        .add_systems(PreUpdate, apply_keys.after(InputSystems))
        .add_systems(Update, drive);
    let code = app.run();
    match VERDICT.get() {
        Some(true) => code,
        _ => AppExit::error(),
    }
}
