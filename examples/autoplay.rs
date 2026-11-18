//! Scripted smoke test of the real game with objective checks.
//!
//! Injects a fixed sequence of key presses (walk, special command, spell,
//! healing spell, locked skill, status screen, quick save, quick load),
//! verifies the outcome of each step and exits with a non-zero code when a
//! mandatory check fails. Screenshots go to `target/autoplay/` for visual
//! inspection, but they are never treated as proof.
//!
//! Saves are isolated: the example replaces the `SaveSlots` resource with
//! `target/autoplay/saves/` before the first frame and refuses to start if
//! that override is not in effect. The real `saves/` directory is only read,
//! to prove at the end that it was not modified.
//!
//! Run with: `cargo run --example autoplay` (needs a graphical session).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::Duration;

use bevy::input::InputSystems;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::app::states::GameSet;
use dark_castle_rpg::core::constants::{DEFAULT_SAVE_SLOT, SAVE_DIR};
use dark_castle_rpg::core::events::{
    BasicAttackRequested, CommandResolved, DamageDealt, EnemyDefeated, Notification,
};
use dark_castle_rpg::core::resources::{GameData, project_root};
use dark_castle_rpg::magic::elements::Element;
use dark_castle_rpg::player::components::{ActionState, Player, Vitals};
use dark_castle_rpg::player::state::PlayerState;
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::save::systems::SaveSlots;
use dark_castle_rpg::skills::definitions::SkillEffect;
use dark_castle_rpg::skills::execution::CommandOutcome;
use dark_castle_rpg::world::maps::CurrentRoom;

/// Fixed simulation step: game time no longer depends on the real frame
/// rate, so the timed script behaves the same on slow machines.
const FRAME: Duration = Duration::from_millis(16);

/// (start, end, key) — the key is held during `[start, end)` seconds.
const SCRIPT: &[(f32, f32, KeyCode)] = &[
    // Walk right, over the stone block and down its far edge, towards the
    // first enemy (skills below require the player to be on the floor).
    (1.0, 3.9, KeyCode::ArrowRight),
    (2.45, 2.85, KeyCode::KeyZ),
    // Ember Bolt from a distance: ↓ ↘ → + MAGIC
    (4.80, 4.92, KeyCode::ArrowDown),
    (4.86, 5.00, KeyCode::ArrowRight),
    (4.96, 5.02, KeyCode::KeyC),
    // Close in to melee range.
    (5.10, 5.50, KeyCode::ArrowRight),
    // Basic attack first: Ember Bolt + Crescent Slash alone defeat the enemy.
    (5.55, 5.61, KeyCode::KeyX),
    // Crescent Slash: ↓ ↘ → + ATTACK
    (5.95, 6.07, KeyCode::ArrowDown),
    (6.01, 6.15, KeyCode::ArrowRight),
    (6.11, 6.17, KeyCode::KeyX),
    // Rising Edge (not learned yet): ↓ ↑ + ATTACK
    (7.10, 7.16, KeyCode::ArrowDown),
    (7.16, 7.24, KeyCode::ArrowUp),
    (7.20, 7.26, KeyCode::KeyX),
    // Mending Light: ↓ N ↓ + MAGIC
    (7.60, 7.66, KeyCode::ArrowDown),
    (7.72, 7.80, KeyCode::ArrowDown),
    (7.76, 7.82, KeyCode::KeyC),
    // Status screen open/close.
    (8.20, 8.26, KeyCode::Tab),
    (8.80, 8.86, KeyCode::Tab),
    // Quick save, then quick load (both in the isolated directory).
    (9.10, 9.16, KeyCode::F5),
    (9.60, 9.66, KeyCode::F9),
];

const SHOTS: &[(f32, &str)] = &[
    (0.8, "01_start"),
    (5.10, "02_ember_bolt"),
    (6.25, "03_crescent_slash"),
    (6.95, "04_after_combat"),
    (7.40, "05_rising_edge_locked"),
    (8.50, "06_status_screen"),
    (9.40, "07_saved"),
    (9.90, "08_loaded"),
];

/// Moments at which the scenario state is sampled.
const SAMPLE_START: f32 = 0.9;
const SAMPLE_AFTER_WALK: f32 = 3.9;
const END_TIME: f32 = 10.3;

const SPELL_CRESCENT: &str = "crescent_slash";
const SPELL_EMBER: &str = "ember_bolt";
const SPELL_MENDING: &str = "mending_light";
const SKILL_LOCKED: &str = "rising_edge";

/// Set once by `finish`: `true` when every mandatory check passed.
static VERDICT: OnceLock<bool> = OnceLock::new();

#[derive(Debug, Clone, PartialEq)]
enum MeleeSource {
    Basic,
    Skill(String),
}

#[derive(Debug, Clone, Copy)]
struct VitalsSample {
    hp: i32,
    hp_max: i32,
    mp: i32,
    hearts: i32,
}

/// One verification line of the final report.
struct Check {
    name: String,
    mandatory: bool,
    result: Result<String, String>,
}

/// Everything the scenario observed, filled frame by frame.
#[derive(Resource)]
struct Verification {
    saves_dir: PathBuf,
    real_saves: Vec<(PathBuf, Option<Vec<u8>>)>,
    start_room: String,
    player_count: Option<usize>,
    room_at_start: Option<String>,
    start_x: Option<f32>,
    walked_x: Option<f32>,
    previous: Option<VitalsSample>,
    last_melee: Option<MeleeSource>,
    basic_requests: u32,
    executed: BTreeMap<String, u32>,
    hits: BTreeMap<String, u32>,
    misses: u32,
    /// Resource/heal checks computed in the activation frame.
    activation_checks: Vec<Check>,
    locked_rejected: bool,
    basic_after_rejection: Vec<String>,
    defeated: u32,
    save_result: Option<Result<String, String>>,
    load_result: Option<Result<String, String>>,
    errors: Vec<String>,
}

impl Verification {
    fn new(saves_dir: PathBuf, real_saves: Vec<(PathBuf, Option<Vec<u8>>)>, room: String) -> Self {
        Self {
            saves_dir,
            real_saves,
            start_room: room,
            player_count: None,
            room_at_start: None,
            start_x: None,
            walked_x: None,
            previous: None,
            last_melee: None,
            basic_requests: 0,
            executed: BTreeMap::new(),
            hits: BTreeMap::new(),
            misses: 0,
            activation_checks: Vec::new(),
            locked_rejected: false,
            basic_after_rejection: Vec::new(),
            defeated: 0,
            save_result: None,
            load_result: None,
            errors: Vec::new(),
        }
    }
}

// ------------------------------------------------------------ isolation

/// The only directory the autoplay may save to: inside this crate's `target/`.
fn isolated_save_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("target/autoplay/saves")
}

fn slot_files(storage: &SaveStorage) -> [PathBuf; 3] {
    let slot = storage.slot_path(DEFAULT_SAVE_SLOT);
    let tmp = slot.with_extension("ron.tmp");
    [slot, storage.backup_path(DEFAULT_SAVE_SLOT), tmp]
}

/// Refuses directories that could be the player's, then removes the slot
/// files a previous run left in the isolated directory.
fn prepare_isolated_saves(dir: &Path, real_dir: &Path) -> Result<(), String> {
    let target = Path::new(env!("CARGO_MANIFEST_DIR")).join("target");
    if !dir.starts_with(&target) {
        return Err(format!(
            "{} is not inside {}",
            dir.display(),
            target.display()
        ));
    }
    std::fs::create_dir_all(dir).map_err(|e| format!("cannot create {}: {e}", dir.display()))?;
    let isolated = dir
        .canonicalize()
        .map_err(|e| format!("cannot resolve {}: {e}", dir.display()))?;
    let real = real_dir
        .canonicalize()
        .unwrap_or_else(|_| real_dir.to_path_buf());
    if isolated == real || isolated.starts_with(&real) || real.starts_with(&isolated) {
        return Err(format!(
            "isolated dir {} overlaps the real saves {}",
            isolated.display(),
            real.display()
        ));
    }
    for path in slot_files(&SaveStorage::new(dir)) {
        if path.exists() {
            std::fs::remove_file(&path)
                .map_err(|e| format!("cannot reset {}: {e}", path.display()))?;
        }
    }
    Ok(())
}

fn snapshot(storage: &SaveStorage) -> Vec<(PathBuf, Option<Vec<u8>>)> {
    slot_files(storage)
        .into_iter()
        .map(|path| {
            let bytes = std::fs::read(&path).ok();
            (path, bytes)
        })
        .collect()
}

// -------------------------------------------------------------- driving

fn drive_keys(time: Res<Time>, mut keys: ResMut<ButtonInput<KeyCode>>) {
    let now = time.elapsed_secs();
    for (start, end, key) in SCRIPT {
        let held = now >= *start && now < *end;
        if held && !keys.pressed(*key) {
            keys.press(*key);
        } else if !held && keys.pressed(*key) {
            // Another script entry may hold the same key right now.
            let still_held = SCRIPT
                .iter()
                .any(|(s, e, k)| k == key && now >= *s && now < *e);
            if !still_held {
                keys.release(*key);
            }
        }
    }
}

fn take_screenshots(mut commands: Commands, time: Res<Time>, mut taken: Local<usize>) {
    let Some((at, name)) = SHOTS.get(*taken) else {
        return;
    };
    if time.elapsed_secs() >= *at {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/autoplay");
        if let Err(error) = std::fs::create_dir_all(&dir) {
            println!(
                "[autoplay] warning: screenshot dir {}: {error}",
                dir.display()
            );
        }
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(dir.join(format!("{name}.png"))));
        *taken += 1;
    }
}

// ------------------------------------------------------------ observing

/// Logs every player state change, so a rejected command can be explained
/// (e.g. `NotAllowedNow` while airborne or hurt).
fn log_player_state(
    time: Res<Time>,
    player: Query<(&ActionState, &Transform), With<Player>>,
    mut last: Local<Option<PlayerState>>,
) {
    let Ok((state, transform)) = player.single() else {
        return;
    };
    let current = state.0.state();
    if *last != Some(current) {
        println!(
            "[autoplay {:5.2}s] state: {current:?} at ({:.1}, {:.1})",
            time.elapsed_secs(),
            transform.translation.x,
            transform.translation.y
        );
        *last = Some(current);
    }
}

type PlayerView<'a> = (Entity, &'a Transform, &'a Vitals);

/// Runs after the whole gameplay frame, so it sees this frame's messages
/// and the state they produced.
fn observe(
    time: Res<Time>,
    data: Res<GameData>,
    room: Res<CurrentRoom>,
    slots: Res<SaveSlots>,
    mut check: ResMut<Verification>,
    players: Query<PlayerView, With<Player>>,
    mut resolved: MessageReader<CommandResolved>,
    mut basic: MessageReader<BasicAttackRequested>,
    mut damage: MessageReader<DamageDealt>,
    mut defeated: MessageReader<EnemyDefeated>,
    mut notifications: MessageReader<Notification>,
) {
    let now = time.elapsed_secs();
    let player = players.single().ok();

    if now >= SAMPLE_START && check.player_count.is_none() {
        check.player_count = Some(players.iter().count());
        check.room_at_start = Some(room.0.clone());
        check.start_x = player.map(|(_, t, _)| t.translation.x);
    }
    if now >= SAMPLE_AFTER_WALK && check.walked_x.is_none() {
        check.walked_x = player.map(|(_, t, _)| t.translation.x);
    }

    let sample = player.map(|(_, _, v)| VitalsSample {
        hp: v.hp.current(),
        hp_max: v.hp.max(),
        mp: v.mp.current(),
        hearts: v.hearts.current(),
    });

    let basic_count = basic.read().count();
    check.basic_requests += basic_count as u32;
    if basic_count > 0 {
        check.last_melee = Some(MeleeSource::Basic);
    }

    let mut rejected_this_frame = Vec::new();
    for c in resolved.read() {
        println!("[autoplay {now:5.2}s] command: {:?}", c.outcome);
        match &c.outcome {
            CommandOutcome::Executed { id } => {
                *check.executed.entry(id.clone()).or_default() += 1;
                if let Some(def) = data.skills.get(id)
                    && matches!(def.effect, SkillEffect::MeleeStrike { .. })
                {
                    check.last_melee = Some(MeleeSource::Skill(id.clone()));
                }
                let checks = activation_checks(&data, id, check.previous, sample);
                check.activation_checks.extend(checks);
            }
            CommandOutcome::NotLearned { id } if id == SKILL_LOCKED => {
                check.locked_rejected = true;
                rejected_this_frame.push(format!("{:?}", c.outcome));
            }
            CommandOutcome::Invalid => {}
            other => rejected_this_frame.push(format!("{other:?}")),
        }
    }
    if basic_count > 0 {
        check.basic_after_rejection.extend(rejected_this_frame);
    }

    let player_entity = player.map(|(e, _, _)| e);
    for d in damage.read() {
        println!("[autoplay {now:5.2}s] damage: {:?}", d.outcome);
        if Some(d.attacker) != player_entity {
            continue;
        }
        match d.outcome {
            dark_castle_rpg::combat::damage::DamageOutcome::Miss => check.misses += 1,
            dark_castle_rpg::combat::damage::DamageOutcome::Hit { element, .. } => {
                let source = if element == Element::Fire {
                    Some(SPELL_EMBER.to_string())
                } else {
                    match &check.last_melee {
                        Some(MeleeSource::Basic) => Some("basic_attack".to_string()),
                        Some(MeleeSource::Skill(id)) => Some(id.clone()),
                        None => None,
                    }
                };
                if let Some(source) = source {
                    *check.hits.entry(source).or_default() += 1;
                }
            }
        }
    }
    for d in defeated.read() {
        println!(
            "[autoplay {now:5.2}s] defeated: {} (+{} EXP)",
            d.enemy_id, d.experience
        );
        check.defeated += 1;
    }

    for n in notifications.read() {
        println!("[autoplay {now:5.2}s] notification: {}", n.0);
        if n.0 == "Game saved" {
            let result = verify_saved(&slots, &check.saves_dir, &room, player);
            check.save_result = Some(result);
        } else if n.0 == "Game loaded" {
            let result = verify_loaded(&slots, &check.saves_dir, player);
            check.load_result = Some(result);
        } else if n.0.starts_with("Save failed") || n.0.starts_with("Load failed") {
            check.errors.push(n.0.clone());
        }
    }

    check.previous = sample;
}

/// Resource cost and healing of a skill, comparing the end of the previous
/// frame with the end of the activation frame. One point of natural
/// regeneration may land in the same frame, hence the tolerance of 1.
fn activation_checks(
    data: &GameData,
    id: &str,
    before: Option<VitalsSample>,
    after: Option<VitalsSample>,
) -> Vec<Check> {
    let mut checks = Vec::new();
    let Some(def) = data.skills.get(id) else {
        checks.push(Check {
            name: format!("{id}: definition"),
            mandatory: true,
            result: Err("executed skill missing from GameData".into()),
        });
        return checks;
    };
    let (Some(before), Some(after)) = (before, after) else {
        checks.push(Check {
            name: format!("{id}: resources"),
            mandatory: true,
            result: Err("player vitals not available around activation".into()),
        });
        return checks;
    };

    let spent_mp = before.mp - after.mp;
    let spent_hearts = before.hearts - after.hearts;
    let mp_ok = spent_mp == def.mp_cost || spent_mp == def.mp_cost - 1;
    checks.push(Check {
        name: format!("{id}: MP cost"),
        mandatory: true,
        result: if mp_ok {
            Ok(format!("spent {spent_mp} (cost {})", def.mp_cost))
        } else {
            Err(format!("spent {spent_mp}, expected {}", def.mp_cost))
        },
    });
    checks.push(Check {
        name: format!("{id}: Hearts cost"),
        mandatory: true,
        result: if spent_hearts == def.hearts_cost {
            Ok(format!("spent {spent_hearts}"))
        } else {
            Err(format!(
                "spent {spent_hearts}, expected {}",
                def.hearts_cost
            ))
        },
    });

    if let SkillEffect::Heal { amount } = def.effect {
        let expected = amount.min(before.hp_max - before.hp).max(0);
        let healed = after.hp - before.hp;
        let check = if expected == 0 {
            Check {
                name: format!("{id}: heal"),
                mandatory: false,
                result: Ok(format!("not applicable: HP already full ({})", before.hp)),
            }
        } else {
            Check {
                name: format!("{id}: heal"),
                mandatory: true,
                result: if healed == expected || healed == expected + 1 {
                    Ok(format!("{} -> {} (+{healed})", before.hp, after.hp))
                } else {
                    Err(format!("healed {healed}, expected {expected}"))
                },
            }
        };
        checks.push(check);
    }
    checks
}

fn verify_saved(
    slots: &SaveSlots,
    saves_dir: &Path,
    room: &CurrentRoom,
    player: Option<PlayerView>,
) -> Result<String, String> {
    if slots.0.dir() != saves_dir {
        return Err(format!("SaveSlots points to {}", slots.0.dir().display()));
    }
    let save = SaveStorage::new(saves_dir)
        .load(DEFAULT_SAVE_SLOT)
        .map_err(|e| format!("isolated save unreadable: {e}"))?;
    let (_, transform, vitals) = player.ok_or("player missing")?;
    let p = &save.player;
    if p.room != room.0 {
        return Err(format!("room {} != {}", p.room, room.0));
    }
    if p.position != (transform.translation.x, transform.translation.y) {
        return Err(format!("position {:?} != live", p.position));
    }
    if p.hp != vitals.hp.current() || p.mp != vitals.mp.current() {
        return Err(format!("hp/mp {}/{} != live", p.hp, p.mp));
    }
    Ok(format!(
        "{} decoded (room {}, hp {}, mp {})",
        SaveStorage::new(saves_dir)
            .slot_path(DEFAULT_SAVE_SLOT)
            .display(),
        p.room,
        p.hp,
        p.mp
    ))
}

fn verify_loaded(
    slots: &SaveSlots,
    saves_dir: &Path,
    player: Option<PlayerView>,
) -> Result<String, String> {
    if slots.0.dir() != saves_dir {
        return Err(format!("SaveSlots points to {}", slots.0.dir().display()));
    }
    let save = SaveStorage::new(saves_dir)
        .load(DEFAULT_SAVE_SLOT)
        .map_err(|e| format!("isolated save unreadable: {e}"))?;
    let (_, transform, vitals) = player.ok_or("player missing")?;
    let p = &save.player;
    let live = (transform.translation.x, transform.translation.y);
    if p.position != live {
        return Err(format!("position {live:?}, expected {:?}", p.position));
    }
    if p.hp != vitals.hp.current() || p.mp != vitals.mp.current() {
        return Err(format!(
            "hp/mp {}/{}, expected {}/{}",
            vitals.hp.current(),
            vitals.mp.current(),
            p.hp,
            p.mp
        ));
    }
    Ok("player restored from the isolated save".into())
}

// ------------------------------------------------------------ reporting

fn count_check(name: &str, count: u32, what: &str) -> Check {
    Check {
        name: name.into(),
        mandatory: true,
        result: if count > 0 {
            Ok(format!("{count} {what}"))
        } else {
            Err(format!("no {what}"))
        },
    }
}

fn option_check(name: &str, value: &Option<Result<String, String>>, missing: &str) -> Check {
    Check {
        name: name.into(),
        mandatory: true,
        result: value.clone().unwrap_or_else(|| Err(missing.into())),
    }
}

fn build_report(check: &Verification, slots: &SaveSlots) -> Vec<Check> {
    let mut report = Vec::new();
    let executed = |id: &str| check.executed.get(id).copied().unwrap_or(0);
    let hits = |id: &str| check.hits.get(id).copied().unwrap_or(0);

    report.push(Check {
        name: "player spawned".into(),
        mandatory: true,
        result: match check.player_count {
            Some(1) => Ok("exactly one player".into()),
            other => Err(format!("player count {other:?}")),
        },
    });
    report.push(Check {
        name: "start room loaded".into(),
        mandatory: true,
        result: match &check.room_at_start {
            Some(room) if *room == check.start_room => Ok(room.clone()),
            other => Err(format!("room {other:?}, expected {}", check.start_room)),
        },
    });
    report.push(Check {
        name: "player moved right".into(),
        mandatory: true,
        result: match (check.start_x, check.walked_x) {
            (Some(a), Some(b)) if b > a => Ok(format!("x {a:.1} -> {b:.1}")),
            other => Err(format!("x samples {other:?}")),
        },
    });
    report.push(count_check(
        "basic attack requested",
        check.basic_requests,
        "requests",
    ));
    report.push(count_check(
        "basic attack hit",
        hits("basic_attack"),
        "hits",
    ));
    for id in [SPELL_CRESCENT, SPELL_EMBER, SPELL_MENDING] {
        report.push(count_check(
            &format!("{id} executed"),
            executed(id),
            "activations",
        ));
    }
    report.push(count_check(
        &format!("{SPELL_CRESCENT} hit"),
        hits(SPELL_CRESCENT),
        "hits",
    ));
    report.push(count_check(
        &format!("{SPELL_EMBER} hit"),
        hits(SPELL_EMBER),
        "hits",
    ));
    report.extend(check.activation_checks.iter().map(|c| Check {
        name: c.name.clone(),
        mandatory: c.mandatory,
        result: c.result.clone(),
    }));
    report.push(Check {
        name: format!("{SKILL_LOCKED} rejected (not learned)"),
        mandatory: true,
        result: if check.locked_rejected {
            Ok("NotLearned".into())
        } else {
            Err("no NotLearned outcome".into())
        },
    });
    report.push(Check {
        name: format!("{SKILL_LOCKED} never executed"),
        mandatory: true,
        result: match executed(SKILL_LOCKED) {
            0 => Ok("0 activations".into()),
            n => Err(format!("{n} activations")),
        },
    });
    report.push(option_check(
        "quick save (isolated)",
        &check.save_result,
        "no `Game saved`",
    ));
    report.push(option_check(
        "quick load (isolated)",
        &check.load_result,
        "no `Game loaded`",
    ));
    report.push(Check {
        name: "no save/load errors".into(),
        mandatory: true,
        result: if check.errors.is_empty() {
            Ok("none".into())
        } else {
            Err(check.errors.join("; "))
        },
    });
    report.push(Check {
        name: "save dir still isolated".into(),
        mandatory: true,
        result: if slots.0.dir() == check.saves_dir {
            Ok(check.saves_dir.display().to_string())
        } else {
            Err(format!("SaveSlots points to {}", slots.0.dir().display()))
        },
    });
    let changed: Vec<String> = check
        .real_saves
        .iter()
        .filter(|(path, before)| std::fs::read(path).ok() != *before)
        .map(|(path, _)| path.display().to_string())
        .collect();
    report.push(Check {
        name: "real saves untouched".into(),
        mandatory: true,
        result: if changed.is_empty() {
            Ok(format!("{} files compared", check.real_saves.len()))
        } else {
            Err(format!("changed: {}", changed.join(", ")))
        },
    });

    // Informational only: the rule is still an open design decision.
    report.push(Check {
        name: "basic attack after rejected command (D-GAME-003, pending)".into(),
        mandatory: false,
        result: Ok(if check.basic_after_rejection.is_empty() {
            "not observed".into()
        } else {
            format!("observed after {}", check.basic_after_rejection.join(", "))
        }),
    });
    report.push(Check {
        name: "enemies defeated".into(),
        mandatory: false,
        result: Ok(format!("{} (misses: {})", check.defeated, check.misses)),
    });
    report
}

fn finish(
    time: Res<Time>,
    check: Res<Verification>,
    slots: Res<SaveSlots>,
    mut exit: MessageWriter<AppExit>,
    mut done: Local<bool>,
) {
    if *done || time.elapsed_secs() < END_TIME {
        return;
    }
    *done = true;
    let report = build_report(&check, &slots);
    let mut failed = 0;
    let mut mandatory = 0;
    for c in &report {
        let tag = match (&c.result, c.mandatory) {
            (Ok(_), true) => "PASS",
            (Err(_), true) => "FAIL",
            (_, false) => "INFO",
        };
        if c.mandatory {
            mandatory += 1;
            if c.result.is_err() {
                failed += 1;
            }
        }
        let detail = match &c.result {
            Ok(d) | Err(d) => d,
        };
        println!("[autoplay] {tag} {}: {detail}", c.name);
    }
    let passed = failed == 0;
    let _ = VERDICT.set(passed);
    if passed {
        println!("[autoplay] RESULT: PASS ({mandatory}/{mandatory} mandatory checks)");
        exit.write(AppExit::Success);
    } else {
        println!("[autoplay] RESULT: FAIL ({failed} of {mandatory} mandatory checks failed)");
        exit.write(AppExit::error());
    }
}

fn main() -> AppExit {
    let root = project_root();
    let data = match GameData::load(&root) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("[autoplay] failed to load game data: {error}");
            return AppExit::error();
        }
    };

    // The directory the game would use on its own (`SaveSlots::default`).
    let real_dir = root.join(SAVE_DIR);
    let saves_dir = isolated_save_dir();
    if let Err(error) = prepare_isolated_saves(&saves_dir, &real_dir) {
        eprintln!("[autoplay] refusing to run, saves not isolated: {error}");
        return AppExit::error();
    }
    let real_saves = snapshot(&SaveStorage::new(&real_dir));
    let start_room = data.character.start.room.clone();

    let mut app = dark_castle_rpg::app::build(data);
    dark_castle_rpg::input::use_legacy_controls(&mut app);
    app.insert_resource(SaveSlots(SaveStorage::new(&saves_dir)))
        .insert_resource(TimeUpdateStrategy::ManualDuration(FRAME))
        .insert_resource(Verification::new(saves_dir.clone(), real_saves, start_room));

    // Prove the override is in effect before the first frame runs.
    let active = app.world().resource::<SaveSlots>().0.dir().to_path_buf();
    if active != saves_dir {
        eprintln!(
            "[autoplay] refusing to run: SaveSlots points to {}",
            active.display()
        );
        return AppExit::error();
    }
    println!("[autoplay] saves isolated in {}", saves_dir.display());

    app.add_systems(PreUpdate, drive_keys.after(InputSystems))
        .add_systems(
            Update,
            (
                take_screenshots,
                log_player_state.after(GameSet::Presentation),
                observe,
                finish,
            )
                .chain(),
        );
    let exit = app.run();

    match VERDICT.get() {
        Some(true) => exit,
        Some(false) => AppExit::error(),
        None => {
            eprintln!("[autoplay] FAIL: the app exited before the checks ran ({exit:?})");
            AppExit::error()
        }
    }
}
