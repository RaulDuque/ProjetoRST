//! World: rooms, collision, physics, camera and persistent progress.
//!
//! V1.6 added the world graph (`graph`, `validation`, `exploration`: pure)
//! and its Bevy layer (`exploration_systems::WorldExplorationPlugin`).
//! V1.7 puts it in the main game: the room lifecycle (`transitions`) is the
//! only place that replaces the active room, and `restore` (pure) resolves
//! where a loaded save resumes. See docs/WORLD_INTEGRATION_V1_7.md.

pub mod camera;
pub mod collision;
pub mod exploration;
pub mod exploration_systems;
pub mod graph;
pub mod interactions;
pub mod maps;
pub mod physics;
pub mod progress;
pub mod restore;
pub mod rooms;
pub mod transitions;
pub mod validation;

use bevy::prelude::*;

use crate::app::states::GameSet;
use progress::WorldProgress;

/// Persistent world state as a resource (saved with the game).
#[derive(Resource, Debug, Clone, Default, Deref, DerefMut)]
pub struct WorldState(pub WorldProgress);

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        transitions::configure(app);
        app.init_resource::<physics::RoomGeometry>()
            .init_resource::<WorldState>()
            .add_plugins(exploration_systems::WorldExplorationPlugin)
            .add_systems(Startup, (camera::spawn_camera, maps::spawn_start_room))
            .add_systems(
                Update,
                (physics::apply_gravity, physics::move_bodies)
                    .chain()
                    .in_set(GameSet::Physics),
            )
            .add_systems(
                Update,
                (camera::snap_on_start, camera::follow_player)
                    .chain()
                    .in_set(GameSet::Presentation),
            );
    }
}
