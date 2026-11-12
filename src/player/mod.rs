//! The playable character: attributes, progression, state machine and the
//! Bevy systems that drive them.

pub mod animation;
pub mod attributes;
pub mod components;
pub mod movement;
pub mod progression;
pub mod state;
pub mod systems;

use bevy::prelude::*;

use crate::app::states::GameSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::spawn_player)
            .add_systems(Update, movement::update_facing.in_set(GameSet::Commands))
            .add_systems(
                Update,
                (
                    systems::tick_action_state,
                    systems::respawn,
                    movement::guard_and_dodge,
                    movement::locomotion,
                    systems::regenerate,
                )
                    .chain()
                    .in_set(GameSet::Actions)
                    .before(crate::combat::attacks::basic_attack)
                    .before(crate::skills::systems::begin_skill),
            )
            .add_systems(Update, systems::receive_damage.in_set(GameSet::Resolution))
            .add_systems(
                Update,
                (systems::collect_rewards, systems::sync_derived_stats)
                    .chain()
                    .in_set(GameSet::Progression),
            )
            .add_systems(
                Update,
                (animation::attach_visor, animation::update_player_visuals)
                    .in_set(GameSet::Presentation),
            );
    }
}
