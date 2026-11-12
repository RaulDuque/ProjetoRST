//! World: rooms, collision, physics, camera and persistent progress.

pub mod camera;
pub mod collision;
pub mod maps;
pub mod physics;
pub mod progress;
pub mod rooms;

use bevy::prelude::*;

use crate::app::states::GameSet;
use progress::WorldProgress;

/// Persistent world state as a resource (saved with the game).
#[derive(Resource, Debug, Clone, Default, Deref, DerefMut)]
pub struct WorldState(pub WorldProgress);

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<physics::RoomGeometry>()
            .init_resource::<WorldState>()
            .add_systems(Startup, (camera::spawn_camera, maps::spawn_start_room))
            .add_systems(
                Update,
                (physics::apply_gravity, physics::move_bodies)
                    .chain()
                    .in_set(GameSet::Physics),
            )
            .add_systems(Update, camera::follow_player.in_set(GameSet::Presentation));
    }
}
