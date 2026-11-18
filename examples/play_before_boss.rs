//! Opens the real game at the foot of the Reliquary stairs and then leaves
//! the keyboard alone. The Ashen Court is the ceiling door above.
//!
//! Saves go to `target/play_before_boss/saves/`, not the project's `saves/`.
//!
//! Run: `cargo run --example play_before_boss`

use std::path::PathBuf;

use bevy::prelude::*;
use dark_castle_rpg::core::events::{Arrival, RoomRequest};
use dark_castle_rpg::core::resources::{GameData, project_root};
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::save::systems::SaveSlots;

const RELIQUARY: &str = "vigil_gates_reliquary";

fn saves_dir() -> PathBuf {
    project_root().join("target/play_before_boss/saves")
}

fn place_at_the_stairs(mut frames: Local<u32>, mut rooms: MessageWriter<RoomRequest>) {
    if *frames > 20 {
        return;
    }
    *frames += 1;
    if *frames == 20 {
        rooms.write(RoomRequest {
            room: RELIQUARY.into(),
            arrival: Arrival::Position(Vec2::new(630.0, 63.0)),
        });
        println!(
            "[play_before_boss] Gate Reliquary, foot of the stairs. \
             Climb with Z. The Ashen Court door is in the ceiling. \
             Saves: {}",
            saves_dir().display()
        );
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
            eprintln!("[play_before_boss] {error}");
            return;
        }
    };
    let saves = saves_dir();
    let _ = std::fs::create_dir_all(&saves);
    let mut app = dark_castle_rpg::app::build_with_world(data, world);
    app.insert_resource(SaveSlots(SaveStorage::new(saves)))
        .add_systems(Update, place_at_the_stairs);
    app.run();
}
