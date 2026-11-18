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
                (
                    systems::rest_at_save_point,
                    systems::offer_fast_travel,
                    systems::quick_save,
                )
                    .chain()
                    .in_set(GameSet::Presentation),
            )
            .add_systems(
                Update,
                (systems::apply_fast_travel_requests, systems::quick_load)
                    .chain()
                    .in_set(crate::world::transitions::RoomStep::Request),
            );
    }
}
