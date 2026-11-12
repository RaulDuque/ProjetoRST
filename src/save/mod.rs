//! Save games: versioned format, crash-safe slot storage and the quick
//! save/load systems.

pub mod save_manager;
pub mod serialization;
pub mod systems;

use bevy::prelude::*;

use crate::app::states::{GameSet, GameState};

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<systems::SaveSlots>()
            .init_resource::<systems::PlayTime>()
            .add_systems(
                Update,
                systems::track_play_time.run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (systems::quick_save, systems::quick_load).in_set(GameSet::Presentation),
            );
    }
}
