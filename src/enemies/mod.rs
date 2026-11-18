//! Enemies: data, AI decisions and behaviour execution.

pub mod ai;
pub mod behavior;
pub mod boss;
pub mod boss_systems;
pub mod components;
pub mod definitions;
pub mod spawning;

use bevy::prelude::*;

use crate::app::states::GameSet;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<components::PendingRespawns>()
            .init_resource::<boss_systems::BossEncounter>()
            .add_systems(Startup, spawning::spawn_room_enemies)
            .add_systems(
                Update,
                (
                    spawning::process_respawns,
                    behavior::think_and_move,
                    boss_systems::apply_lunge_velocity,
                )
                    .chain()
                    .in_set(GameSet::Actions),
            )
            .add_systems(Update, behavior::contact_damage.in_set(GameSet::Combat))
            .add_systems(
                Update,
                (
                    behavior::receive_damage,
                    boss_systems::on_boss_defeated,
                    boss_systems::end_encounter_on_player_death,
                )
                    .chain()
                    .in_set(GameSet::Resolution),
            )
            .add_systems(
                Update,
                spawning::enter_new_room.in_set(crate::world::transitions::RoomStep::React),
            )
            .add_systems(
                Update,
                (
                    boss_systems::begin_encounter_on_boss_spawn,
                    boss_systems::update_boss_phase,
                    boss_systems::seal_arena_while_fighting,
                    boss_systems::end_encounter_on_room_leave,
                    behavior::update_enemy_visuals,
                )
                    .in_set(GameSet::Presentation),
            );
    }
}
