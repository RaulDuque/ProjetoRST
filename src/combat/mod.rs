//! Combat: damage formulas, command input and recognition, hitboxes and
//! physical attacks.

pub mod attacks;
pub mod command_buffer;
pub mod command_input;
pub mod command_parser;
pub mod damage;
pub mod hitboxes;

use bevy::prelude::*;

use crate::app::states::GameSet;
use crate::player::movement::update_facing;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            command_input::record_and_resolve_commands
                .after(update_facing)
                .in_set(GameSet::Commands),
        )
        .add_systems(
            Update,
            (attacks::basic_attack, attacks::melee_techniques).in_set(GameSet::Actions),
        )
        .add_systems(
            Update,
            (
                hitboxes::tick_invulnerability,
                hitboxes::follow_owners,
                hitboxes::detect_hits,
                hitboxes::expire_hitboxes,
            )
                .chain()
                .in_set(GameSet::Combat),
        )
        .add_systems(
            Update,
            hitboxes::cancel_interrupted_hitboxes
                .in_set(GameSet::Resolution)
                .after(crate::player::systems::receive_damage),
        );
    }
}
