//! D22 windowed probe: Ashen Sentinel damaged only by the attack key.
//!
//! The player is healed by the harness so the script can finish. Boss HP is
//! never written. Saves stay in `target/d22/saves/`.
//!
//! Run: `cargo run --example d22_boss_probe` (needs a graphical session).

use std::path::PathBuf;
use std::sync::OnceLock;

use bevy::input::InputSystems;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::combat::hitboxes::{Hitbox, Team};
use dark_castle_rpg::core::events::{Arrival, RoomRequest};
use dark_castle_rpg::core::resources::{GameData, project_root};
use dark_castle_rpg::enemies::boss::{BossPattern, BossPhase};
use dark_castle_rpg::enemies::boss_systems::BossEncounter;
use dark_castle_rpg::enemies::components::{Boss, BossRuntime, EnemyBrain, EnemyHealth};
use dark_castle_rpg::magic::projectiles::Projectile;
use dark_castle_rpg::player::components::{Belongings, Player, Vitals};
use dark_castle_rpg::player::state::Facing;
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::save::systems::SaveSlots;
use dark_castle_rpg::world::WorldState;
use dark_castle_rpg::world::physics::FacingDirection;
use std::time::Duration;

const FRAME: Duration = Duration::from_millis(16);
const COURT: &str = "vigil_gates_ashen_court";

static VERDICT: OnceLock<bool> = OnceLock::new();

fn evidence_dir() -> PathBuf {
    project_root().join("docs/development/d22/evidence")
}

fn out_dir() -> PathBuf {
    project_root().join("target/d22")
}

#[derive(Resource, Default)]
struct WantAttack {
    press: bool,
    release: bool,
}

#[derive(Resource)]
struct Probe {
    frame: u32,
    phase: u8,
    wait: u32,
    start_hp: i32,
    slash_shot: bool,
    saw_slash: bool,
    saw_lunge: bool,
    lunge_shot: bool,
    saw_wave: bool,
    exit_in: Option<u32>,
    failed: Option<String>,
}

fn shot(commands: &mut Commands, name: &str) {
    let _ = std::fs::create_dir_all(evidence_dir());
    commands
        .spawn(Screenshot::primary_window())
        .observe(save_to_disk(evidence_dir().join(format!("{name}.png"))));
}

fn apply_attack(mut want: ResMut<WantAttack>, mut keys: ResMut<ButtonInput<KeyCode>>) {
    if want.press {
        keys.release(KeyCode::KeyX);
        keys.press(KeyCode::KeyX);
        want.press = false;
        want.release = true;
    } else if want.release {
        keys.release(KeyCode::KeyX);
        want.release = false;
    }
}

fn stand(
    transform: &mut Transform,
    vitals: &mut Vitals,
    facing: &mut FacingDirection,
    x: f32,
    face: Facing,
) {
    transform.translation.x = x;
    transform.translation.y = 72.0;
    vitals.hp.refill();
    facing.0 = face;
}

fn drive(
    mut commands: Commands,
    mut probe: ResMut<Probe>,
    mut want: ResMut<WantAttack>,
    encounter: Res<BossEncounter>,
    world: Res<WorldState>,
    mut rooms: MessageWriter<RoomRequest>,
    bosses: Query<(&EnemyHealth, &BossRuntime, &Transform), (With<Boss>, Without<Player>)>,
    brains: Query<&EnemyBrain, (With<Boss>, Without<Player>)>,
    mut player: Query<
        (
            &mut Transform,
            &mut Vitals,
            &mut FacingDirection,
            &Belongings,
        ),
        (With<Player>, Without<Boss>),
    >,
    bolts: Query<&Hitbox, With<Projectile>>,
    mut exit: MessageWriter<AppExit>,
) {
    if probe.failed.is_some() || VERDICT.get().is_some() {
        return;
    }
    probe.frame += 1;
    let frame = probe.frame;
    let boss = bosses.iter().next().map(|(hp, runtime, transform)| {
        (
            hp.current(),
            runtime.clone(),
            transform.translation.truncate(),
        )
    });

    match probe.phase {
        0 if frame >= 40 => {
            rooms.write(RoomRequest {
                room: COURT.into(),
                arrival: Arrival::Position(Vec2::new(400.0, 63.0)),
            });
            probe.phase = 1;
        }
        1 if frame >= 70 => {
            if !encounter.active {
                probe.failed = Some("encounter not active".into());
                return;
            }
            let Some((hp, _, _)) = boss else {
                probe.failed = Some("boss missing".into());
                return;
            };
            probe.start_hp = hp;
            shot(&mut commands, "01_arena");
            probe.phase = 2;
            probe.wait = 0;
        }
        2 => {
            let Some((hp, runtime, pos)) = boss else {
                probe.failed = Some("boss lost before slash".into());
                return;
            };
            if probe.wait == 0 {
                if let Ok((mut transform, mut vitals, mut facing, _)) = player.single_mut() {
                    stand(
                        &mut transform,
                        &mut vitals,
                        &mut facing,
                        (pos.x + 32.0).clamp(780.0, 1200.0),
                        Facing::Left,
                    );
                }
                want.press = true;
                probe.wait = 21;
            } else {
                probe.wait -= 1;
            }
            let winding = brains
                .iter()
                .next()
                .is_some_and(|brain| brain.windup.is_some());
            if winding && runtime.next_pattern == BossPattern::FrontalSlash && !probe.slash_shot {
                shot(&mut commands, "02_frontal_slash");
                probe.slash_shot = true;
            }
            if runtime.resolved_slash || runtime.last_resolved == Some(BossPattern::FrontalSlash) {
                probe.saw_slash = true;
                probe.phase = 3;
                probe.wait = 0;
            } else if frame > 900 {
                probe.failed = Some(format!("no slash hp={hp}"));
            }
        }
        3 => {
            let Some((_, runtime, pos)) = boss else {
                probe.failed = Some("boss lost before lunge".into());
                return;
            };
            if let Ok((mut transform, mut vitals, mut facing, _)) = player.single_mut() {
                stand(
                    &mut transform,
                    &mut vitals,
                    &mut facing,
                    (pos.x + 52.0).clamp(780.0, 1200.0),
                    Facing::Left,
                );
            }
            probe.wait += 1;
            if runtime.lunge && !probe.lunge_shot {
                shot(&mut commands, "03_lunge");
                probe.lunge_shot = true;
            }
            if runtime.resolved_lunge {
                probe.saw_lunge = true;
                probe.phase = 4;
                probe.wait = 0;
            } else if probe.wait > 400 {
                probe.failed = Some("no lunge".into());
            }
        }
        4 => {
            let Some((hp, runtime, pos)) = boss else {
                probe.failed = Some("boss lost before phase 2".into());
                return;
            };
            if probe.wait == 0 {
                if let Ok((mut transform, mut vitals, mut facing, _)) = player.single_mut() {
                    stand(
                        &mut transform,
                        &mut vitals,
                        &mut facing,
                        (pos.x + 32.0).clamp(780.0, 1200.0),
                        Facing::Left,
                    );
                }
                want.press = true;
                probe.wait = 21;
            } else {
                probe.wait -= 1;
            }
            if runtime.phase == BossPhase::Two && hp < probe.start_hp / 2 {
                shot(&mut commands, "04_phase2");
                probe.phase = 5;
                probe.wait = 0;
            } else if hp == 0 {
                probe.failed = Some("boss died before phase 2".into());
            } else if frame > 4000 {
                probe.failed = Some(format!("phase 2 timeout hp={hp}"));
            }
        }
        5 => {
            let Some((hp, runtime, pos)) = &boss else {
                probe.failed = Some("boss lost before ash wave".into());
                return;
            };
            if let Ok((mut transform, mut vitals, mut facing, _)) = player.single_mut() {
                transform.translation.x = (pos.x - 220.0).clamp(820.0, 1180.0);
                transform.translation.y = 72.0;
                vitals.hp.refill();
                facing.0 = if transform.translation.x < pos.x {
                    Facing::Right
                } else {
                    Facing::Left
                };
            }
            let bolt = bolts.iter().any(|hit| hit.team == Team::Enemy);
            if runtime.last_resolved == Some(BossPattern::AshWave) || bolt {
                shot(&mut commands, "05_ash_wave");
                probe.saw_wave = true;
                probe.phase = 6;
                probe.wait = 0;
            } else if *hp == 0 || probe.wait > 260 {
                probe.failed = Some(format!("no ash wave hp={hp}"));
            }
            probe.wait += 1;
        }
        6 => {
            let Some((hp, _, pos)) = boss else {
                probe.phase = 7;
                probe.wait = 0;
                return;
            };
            if hp == 0 {
                probe.phase = 7;
                probe.wait = 0;
                return;
            }
            if probe.wait == 0 {
                if let Ok((mut transform, mut vitals, mut facing, _)) = player.single_mut() {
                    // Left of the sentinel so phase-2 retreat walks toward the wall.
                    stand(
                        &mut transform,
                        &mut vitals,
                        &mut facing,
                        (pos.x - 32.0).clamp(780.0, 1200.0),
                        Facing::Right,
                    );
                }
                want.press = true;
                probe.wait = 21;
            } else {
                probe.wait -= 1;
            }
            if frame > 12000 {
                probe.failed = Some(format!(
                    "kill timeout hp={hp} at ({:.0},{:.0})",
                    pos.x, pos.y
                ));
            }
        }
        7 => {
            probe.wait += 1;
            if probe.wait < 8 {
                return;
            }
            if encounter.active || !world.0.defeated_bosses.contains("ashen_sentinel") {
                if probe.wait > 40 {
                    probe.failed = Some("victory not recorded".into());
                }
                return;
            }
            let count = player
                .single()
                .map(|(_, _, _, bag)| bag.inventory.count("sentinel_brand"))
                .unwrap_or(0);
            if count != 1 {
                probe.failed = Some(format!("brand count {count}"));
                return;
            }
            if !probe.saw_slash || !probe.saw_lunge || !probe.saw_wave {
                probe.failed = Some(format!(
                    "patterns slash={} lunge={} wave={}",
                    probe.saw_slash, probe.saw_lunge, probe.saw_wave
                ));
                return;
            }
            shot(&mut commands, "06_victory");
            probe.exit_in.get_or_insert(30);
            if let Some(left) = probe.exit_in.as_mut() {
                *left = left.saturating_sub(1);
                if *left == 0 {
                    let _ = VERDICT.set(true);
                    println!("[d22_boss_probe] RESULT: PASS");
                    exit.write(AppExit::Success);
                }
            }
        }
        _ => {}
    }

    if let Some(reason) = &probe.failed {
        println!("[d22_boss_probe] RESULT: FAIL ({reason})");
        let _ = VERDICT.set(false);
        exit.write(AppExit::error());
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
        Ok(value) => value,
        Err(error) => {
            eprintln!("[d22_boss_probe] {error}");
            return AppExit::error();
        }
    };
    let saves = out_dir().join("saves");
    let _ = std::fs::remove_dir_all(&saves);
    let _ = std::fs::create_dir_all(&saves);
    let _ = std::fs::create_dir_all(evidence_dir());
    println!(
        "[d22_boss_probe] saves {}; evidence {}",
        saves.display(),
        evidence_dir().display()
    );

    let mut app = dark_castle_rpg::app::build_with_world(data, world);
    dark_castle_rpg::input::use_legacy_controls(&mut app);
    app.insert_resource(SaveSlots(SaveStorage::new(saves)))
        .insert_resource(TimeUpdateStrategy::ManualDuration(FRAME))
        .insert_resource(WantAttack::default())
        .insert_resource(Probe {
            frame: 0,
            phase: 0,
            wait: 0,
            start_hp: 0,
            slash_shot: false,
            saw_slash: false,
            saw_lunge: false,
            lunge_shot: false,
            saw_wave: false,
            exit_in: None,
            failed: None,
        })
        .add_systems(PreUpdate, apply_attack.after(InputSystems))
        .add_systems(Update, drive);
    let exit = app.run();
    match VERDICT.get() {
        Some(true) => exit,
        _ => AppExit::error(),
    }
}
