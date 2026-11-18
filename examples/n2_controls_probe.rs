//! Opens the controls screen on the hybrid preset, captures it, remaps the
//! alternate attack to G and applies. Saves stay under `target/`.
//!
//! Run: `cargo run --example n2_controls_probe`

use std::path::PathBuf;
use std::time::Duration;

use bevy::input::InputSystems;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::app::states::GameState;
use dark_castle_rpg::core::resources::{GameData, project_root};
use dark_castle_rpg::input::action::Action;
use dark_castle_rpg::input::binding::{Binding, PhysicalKey};
use dark_castle_rpg::input::profile::Slot;
use dark_castle_rpg::input::{ControlsPath, store};
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::save::systems::SaveSlots;

fn probe_dir() -> PathBuf {
    project_root().join("target/n2_controls_probe")
}

fn evidence_dir() -> PathBuf {
    project_root().join("docs/development/n2/evidence")
}

#[derive(Resource)]
struct Script {
    frame: u32,
}

fn inject(mut script: ResMut<Script>, mut keys: ResMut<ButtonInput<KeyCode>>) {
    script.frame += 1;
    // The controls screen reads Bevy's just_pressed. A key left down from
    // the previous frame would not count as a new press.
    keys.release_all();
    let key = match script.frame {
        30 => Some(KeyCode::Escape),
        60 | 62 | 64 | 66 | 68 | 70 => Some(KeyCode::ArrowDown),
        72 => Some(KeyCode::ArrowRight),
        74 => Some(KeyCode::Enter),
        78 => Some(KeyCode::KeyG),
        80 | 82 | 84 | 86 | 88 | 90 | 92 | 94 | 96 | 98 | 100 | 102 | 104 => {
            Some(KeyCode::ArrowDown)
        }
        106 => Some(KeyCode::Enter),
        _ => None,
    };
    if let Some(key) = key {
        keys.press(key);
    }
}

fn watch(
    mut commands: Commands,
    script: Res<Script>,
    state: Res<State<GameState>>,
    mut script_shot: Local<bool>,
    mut exit: MessageWriter<AppExit>,
) {
    if script.frame == 48 && *state.get() == GameState::Controls && !*script_shot {
        *script_shot = true;
        let _ = std::fs::create_dir_all(evidence_dir());
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(evidence_dir().join("controls_screen.png")));
    }
    if script.frame == 140 {
        let path = probe_dir().join("controls.ron");
        match store::load(&path) {
            Ok(profile) => {
                let attack = profile.binding(Action::Attack).get(Slot::Alternate);
                if attack == Some(Binding::Key(PhysicalKey::G)) {
                    println!("[n2_controls_probe] apply persisted alternate attack G");
                } else {
                    println!("[n2_controls_probe] unexpected attack alternate: {attack:?}");
                }
            }
            Err(error) => println!("[n2_controls_probe] controls file: {error:?}"),
        }
        exit.write(AppExit::Success);
    }
}

fn main() {
    let root = project_root();
    let loaded = GameData::load(&root)
        .map_err(|e| e.to_string())
        .and_then(|mut data| {
            dark_castle_rpg::app::load_world(&root, &mut data).map(|world| (data, world))
        });
    let (data, world) = match loaded {
        Ok(value) => value,
        Err(error) => {
            eprintln!("[n2_controls_probe] {error}");
            return;
        }
    };
    let dir = probe_dir();
    let _ = std::fs::remove_dir_all(&dir);
    let saves = dir.join("saves");
    let controls = dir.join("controls.ron");
    let _ = std::fs::create_dir_all(&saves);
    let mut app = dark_castle_rpg::app::build_with_world(data, world);
    app.insert_resource(SaveSlots(SaveStorage::new(saves)))
        .insert_resource(ControlsPath(controls))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(
            16,
        )))
        .insert_resource(Script { frame: 0 })
        .add_systems(PreUpdate, inject.after(InputSystems))
        .add_systems(
            Update,
            watch.after(dark_castle_rpg::app::states::GameSet::Presentation),
        );
    app.run();
}
