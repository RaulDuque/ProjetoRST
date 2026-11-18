//! D21 windowed probe of the Ashen Court boss encounter.
//!
//! Teleports into the arena, waits for phase/combat cues, deals scripted
//! damage through the real pipeline where possible, and captures screenshots
//! under `docs/development/d21/evidence/`. Saves stay in `target/d21/saves/`.
//!
//! Run: `cargo run --example d21_boss_probe` (needs a graphical session).

use std::path::PathBuf;
use std::sync::OnceLock;
use std::time::Duration;

use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::combat::damage::DamageOutcome;
use dark_castle_rpg::combat::reaction::KnockbackIntensity;
use dark_castle_rpg::core::events::{Arrival, DamageDealt, RoomRequest};
use dark_castle_rpg::core::resources::{GameData, project_root};
use dark_castle_rpg::enemies::boss::BossPhase;
use dark_castle_rpg::enemies::boss_systems::BossEncounter;
use dark_castle_rpg::enemies::components::{Boss, BossRuntime, EnemyHealth};
use dark_castle_rpg::magic::elements::Element;
use dark_castle_rpg::player::components::Player;
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::save::systems::SaveSlots;
use dark_castle_rpg::world::WorldState;

const FRAME: Duration = Duration::from_millis(16);
const COURT: &str = "vigil_gates_ashen_court";

static VERDICT: OnceLock<bool> = OnceLock::new();

fn evidence_dir() -> PathBuf {
    project_root().join("docs/development/d21/evidence")
}

fn out_dir() -> PathBuf {
    project_root().join("target/d21")
}

#[derive(Resource)]
struct Probe {
    step: u32,
    shot: u32,
    exit_in: Option<u32>,
    failed: Option<String>,
}

fn shot(commands: &mut Commands, name: &str) {
    let _ = std::fs::create_dir_all(evidence_dir());
    commands
        .spawn(Screenshot::primary_window())
        .observe(save_to_disk(evidence_dir().join(format!("{name}.png"))));
}

fn drive(
    mut commands: Commands,
    time: Res<Time>,
    mut probe: ResMut<Probe>,
    encounter: Res<BossEncounter>,
    world: Res<WorldState>,
    mut rooms: MessageWriter<RoomRequest>,
    mut damage: MessageWriter<DamageDealt>,
    bosses: Query<(Entity, &EnemyHealth, &BossRuntime), With<Boss>>,
    player: Query<Entity, With<Player>>,
    mut exit: MessageWriter<AppExit>,
) {
    if probe.failed.is_some() || VERDICT.get().is_some() {
        return;
    }
    let t = time.elapsed_secs();
    match probe.step {
        0 if t >= 0.6 => {
            rooms.write(RoomRequest {
                room: COURT.into(),
                arrival: Arrival::Position(Vec2::new(400.0, 63.0)),
            });
            probe.step = 1;
        }
        1 if t >= 1.2 => {
            if !encounter.active {
                probe.failed = Some("encounter not active".into());
                return;
            }
            if probe.shot == 0 {
                shot(&mut commands, "01_arena_start");
                probe.shot = 1;
            }
            probe.step = 2;
        }
        2 if t >= 2.0 => {
            if probe.shot == 1 {
                shot(&mut commands, "02_boss_phase1");
                probe.shot = 2;
            }
            // Chunk damage toward phase 2.
            if let (Ok(player), Ok((boss, health, _))) = (player.single(), bosses.single()) {
                let amount = (health.max() as f32 * 0.12).ceil() as i32;
                damage.write(DamageDealt {
                    attacker: player,
                    target: boss,
                    outcome: DamageOutcome::Hit {
                        amount,
                        critical: false,
                        element: Element::Physical,
                    },
                    direction: 1.0,
                    knockback: KnockbackIntensity::default(),
                });
            }
            if bosses
                .single()
                .ok()
                .is_some_and(|(_, _, runtime)| runtime.phase == BossPhase::Two)
            {
                probe.step = 3;
            }
            if t > 12.0 {
                probe.failed = Some("phase 2 timeout".into());
            }
        }
        3 => {
            if probe.shot == 2 {
                shot(&mut commands, "03_phase2");
                probe.shot = 3;
            }
            // Finish the boss.
            if let (Ok(player), Ok((boss, health, _))) = (player.single(), bosses.single()) {
                damage.write(DamageDealt {
                    attacker: player,
                    target: boss,
                    outcome: DamageOutcome::Hit {
                        amount: health.current() + 5,
                        critical: false,
                        element: Element::Physical,
                    },
                    direction: 1.0,
                    knockback: KnockbackIntensity::default(),
                });
            }
            probe.step = 4;
        }
        4 if t >= 0.0 => {
            if encounter.active {
                return;
            }
            if !world.0.defeated_bosses.contains("ashen_sentinel") {
                probe.failed = Some("boss not recorded".into());
                return;
            }
            if probe.shot == 3 {
                shot(&mut commands, "04_victory");
                probe.shot = 4;
            }
            probe.exit_in.get_or_insert(40);
            if let Some(left) = probe.exit_in.as_mut() {
                *left = left.saturating_sub(1);
                if *left == 0 {
                    let _ = VERDICT.set(true);
                    println!("[d21_boss_probe] RESULT: PASS");
                    exit.write(AppExit::Success);
                }
            }
        }
        _ => {}
    }
    if let Some(reason) = &probe.failed {
        println!("[d21_boss_probe] RESULT: FAIL ({reason})");
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
        Ok(v) => v,
        Err(error) => {
            eprintln!("[d21_boss_probe] {error}");
            return AppExit::error();
        }
    };
    let saves = out_dir().join("saves");
    let _ = std::fs::remove_dir_all(&saves);
    let _ = std::fs::create_dir_all(&saves);
    let _ = std::fs::create_dir_all(evidence_dir());
    println!(
        "[d21_boss_probe] saves {}; evidence {}",
        saves.display(),
        evidence_dir().display()
    );

    let mut app = dark_castle_rpg::app::build_with_world(data, world);
    dark_castle_rpg::input::use_legacy_controls(&mut app);
    app.insert_resource(SaveSlots(SaveStorage::new(saves)))
        .insert_resource(TimeUpdateStrategy::ManualDuration(FRAME))
        .insert_resource(Probe {
            step: 0,
            shot: 0,
            exit_in: None,
            failed: None,
        })
        .add_systems(Update, drive);
    let exit = app.run();
    match VERDICT.get() {
        Some(true) => exit,
        _ => AppExit::error(),
    }
}
