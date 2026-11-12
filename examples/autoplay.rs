//! Scripted smoke test of the real game: injects a fixed sequence of key
//! presses (walk, special command, spell, locked skill, status screen,
//! quick save), logs every command outcome and saves screenshots to
//! `target/autoplay/`. Exits by itself.
//!
//! Run with: `cargo run --example autoplay`

use std::path::PathBuf;

use bevy::input::InputSystems;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use dark_castle_rpg::core::events::{CommandResolved, DamageDealt, EnemyDefeated, Notification};
use dark_castle_rpg::core::resources::{GameData, project_root};

/// (start, end, key) — the key is held during `[start, end)` seconds.
const SCRIPT: &[(f32, f32, KeyCode)] = &[
    // Walk right, jumping over the stone block, towards the first enemy.
    (1.0, 3.6, KeyCode::ArrowRight),
    (2.45, 2.85, KeyCode::KeyZ),
    // Crescent Slash: ↓ ↘ → + ATTACK
    (3.80, 3.92, KeyCode::ArrowDown),
    (3.86, 4.00, KeyCode::ArrowRight),
    (3.96, 4.02, KeyCode::KeyX),
    // Ember Bolt: ↓ ↘ → + MAGIC
    (4.80, 4.92, KeyCode::ArrowDown),
    (4.86, 5.00, KeyCode::ArrowRight),
    (4.96, 5.02, KeyCode::KeyC),
    // Close in and finish the enemy with basic attacks.
    (5.10, 5.50, KeyCode::ArrowRight),
    (5.55, 5.61, KeyCode::KeyX),
    (5.95, 6.01, KeyCode::KeyX),
    // Rising Edge (not learned yet): ↓ ↑ + ATTACK
    (6.50, 6.56, KeyCode::ArrowDown),
    (6.56, 6.64, KeyCode::ArrowUp),
    (6.60, 6.66, KeyCode::KeyX),
    // Status screen open/close.
    (7.50, 7.56, KeyCode::Tab),
    (8.50, 8.56, KeyCode::Tab),
    // Quick save.
    (9.00, 9.06, KeyCode::F5),
];

const SHOTS: &[(f32, &str)] = &[
    (0.8, "01_start"),
    (4.05, "02_crescent_slash"),
    (5.12, "03_ember_bolt"),
    (6.10, "04_after_combat"),
    (6.80, "05_rising_edge_locked"),
    (8.00, "06_status_screen"),
    (9.30, "07_saved"),
];

const END_TIME: f32 = 10.0;

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
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/autoplay");
        let _ = std::fs::create_dir_all(&dir);
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(dir.join(format!("{name}.png"))));
        *taken += 1;
    }
}

fn log_events(
    time: Res<Time>,
    mut commands_resolved: MessageReader<CommandResolved>,
    mut defeated: MessageReader<EnemyDefeated>,
    mut notifications: MessageReader<Notification>,
    mut damage: MessageReader<DamageDealt>,
) {
    let now = time.elapsed_secs();
    for d in damage.read() {
        println!("[autoplay {now:5.2}s] damage: {:?}", d.outcome);
    }
    for c in commands_resolved.read() {
        println!("[autoplay {now:5.2}s] command: {:?}", c.outcome);
    }
    for d in defeated.read() {
        println!(
            "[autoplay {now:5.2}s] defeated: {} (+{} EXP)",
            d.enemy_id, d.experience
        );
    }
    for n in notifications.read() {
        println!("[autoplay {now:5.2}s] notification: {}", n.0);
    }
}

fn finish(time: Res<Time>, mut exit: MessageWriter<AppExit>) {
    if time.elapsed_secs() >= END_TIME {
        println!("[autoplay] done");
        exit.write(AppExit::Success);
    }
}

fn main() -> AppExit {
    let data = match GameData::load(&project_root()) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("failed to load game data: {error}");
            return AppExit::error();
        }
    };
    let mut app = dark_castle_rpg::app::build(data);
    app.add_systems(PreUpdate, drive_keys.after(InputSystems))
        .add_systems(Update, (take_screenshots, log_events, finish))
        .run()
}
