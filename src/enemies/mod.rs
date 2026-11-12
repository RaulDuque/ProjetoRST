//! Enemies: data, AI decisions and behaviour execution.

pub mod ai;
pub mod behavior;
pub mod components;
pub mod definitions;
pub mod spawning;

use bevy::prelude::*;

use crate::app::states::GameSet;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<components::PendingRespawns>()
            .add_systems(Startup, spawning::spawn_room_enemies)
            .add_systems(
                Update,
                (spawning::process_respawns, behavior::think_and_move)
                    .chain()
                    .in_set(GameSet::Actions),
            )
            .add_systems(Update, behavior::contact_damage.in_set(GameSet::Combat))
            .add_systems(Update, behavior::receive_damage.in_set(GameSet::Resolution))
            .add_systems(
                Update,
                behavior::update_enemy_visuals.in_set(GameSet::Presentation),
            );
    }
}
