//! NPCs and conversations (V2.1).
//!
//! - [`definitions`] (pure): NPCs, dialogue nodes, conditions, validation;
//! - [`dialogue`] (pure): a conversation in progress;
//! - [`systems`] (Bevy): spawning per room, `D` to talk, prompts, the
//!   dialogue state. The box itself is `ui::dialogue_box`.
//!
//! Details: docs/NPC_COMMERCE_V2_1.md.

pub mod definitions;
pub mod dialogue;
pub mod systems;

use bevy::prelude::*;

use crate::app::states::{GameSet, GameState};
use crate::world::transitions::RoomStep;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (systems::spawn_room_npcs, systems::close_on_room_change).in_set(RoomStep::React),
        )
        .add_systems(
            Update,
            systems::talk_to_npcs
                .in_set(GameSet::Resolution)
                // Before the objects: it sees an object as usable in the same
                // frame the object system will use it, so one `D` never does both.
                .before(crate::world::interactions::interact_with_objects),
        )
        .add_systems(
            Update,
            (
                systems::show_talk_prompts,
                systems::run_conversation.run_if(in_state(GameState::Dialogue)),
            )
                .in_set(GameSet::Presentation),
        );
    }
}
