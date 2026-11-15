//! Room lifecycle (V1.7): the only place where the active room is replaced.
//!
//! Door transitions and quick loads do not touch the room themselves: they
//! write a [`RoomRequest`]. [`apply_room_requests`] applies **at most one
//! request per frame**, always in the same order of the frame:
//!
//! ```text
//! GameSet::Resolution   door_transitions → RoomRequest (Arrival::Door)
//! GameSet::Rooms
//!   RoomStep::Request   save::quick_load  → RoomRequest (Arrival::Position)
//!   RoomStep::Apply     apply_room_requests
//!                         choose: a load wins over doors; the first door wins over later ones
//!                         validate the target (room definition, door entry)
//!                         despawn every RoomEntity of the old room
//!                         spawn the target room (maps::spawn_room)
//!                         place the player (entry point or validated position)
//!                         snap the camera to the new room bounds
//!                         lock the doors, write RoomChanged
//!   RoomStep::React     owners clean up what they own (hitboxes, projectiles,
//!                       pending respawns, damage numbers), enemies of the new
//!                       room spawn, barriers/secrets/pickups are prepared,
//!                       the doors are re-armed once the player stands clear
//!                       of every door trigger
//! ```
//!
//! `GameSet::Rooms` also runs while paused, so a load from the status
//! screen replaces the room in the same frame. Nothing of the old room
//! survives to the next simulation frame: every temporary combat entity is
//! despawned in `RoomStep::React`, before `GameSet::Progression` and long
//! before the next `GameSet::Combat`.

use bevy::prelude::*;

use super::camera::{MainCamera, snap_to};
use super::exploration_systems::WorldGraph;
use super::maps::{CurrentRoom, RoomEntity, spawn_room};
use super::physics::{RoomGeometry, Velocity};
use crate::app::states::GameSet;
use crate::core::events::{Arrival, RoomChangeCause, RoomChanged, RoomRequest};
use crate::core::resources::GameData;
use crate::player::components::Player;

/// Ordered steps of a room change inside `GameSet::Rooms`.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoomStep {
    /// Systems that ask for a room (quick load).
    Request,
    /// The room change itself.
    Apply,
    /// Owners react to `RoomChanged`.
    React,
}

/// Doors ignore the player from a room change until the player is seen
/// clear of every door trigger of the new room (`exploration_systems::
/// rearm_doors`). Entry points are validated to be clear of triggers, so in
/// normal play the doors re-arm in the frame of the change; the lock only
/// stops a bad entry point from bouncing the player between two rooms.
#[derive(Resource, Debug, Clone, Copy, Default)]
pub struct TransitionLock {
    pub locked: bool,
}

impl TransitionLock {
    pub fn is_locked(&self) -> bool {
        self.locked
    }
}

/// How many room changes were applied and requests discarded (stress tests
/// and the route report read it; nothing in the game depends on it).
#[derive(Resource, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomChangeStats {
    pub applied: u32,
    pub discarded: u32,
}

pub fn configure(app: &mut App) {
    app.init_resource::<TransitionLock>()
        .init_resource::<RoomChangeStats>()
        .configure_sets(
            Update,
            (RoomStep::Request, RoomStep::Apply, RoomStep::React)
                .chain()
                .in_set(GameSet::Rooms),
        )
        .add_systems(Update, apply_room_requests.in_set(RoomStep::Apply));
}

/// Picks the request to apply this frame: the first load, else the first
/// door. Deterministic: messages are read in the order they were written.
pub fn choose_request(requests: &[RoomRequest]) -> Option<&RoomRequest> {
    requests
        .iter()
        .find(|r| matches!(r.arrival, Arrival::Position(_)))
        .or_else(|| requests.first())
}

/// Applies the chosen [`RoomRequest`] of this frame (see the module docs).
pub fn apply_room_requests(
    mut commands: Commands,
    data: Res<GameData>,
    world: Option<Res<WorldGraph>>,
    current: Option<Res<CurrentRoom>>,
    mut requests: MessageReader<RoomRequest>,
    mut player: Query<(&mut Transform, &mut Velocity), (With<Player>, Without<MainCamera>)>,
    mut camera: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    room_entities: Query<Entity, With<RoomEntity>>,
    mut lock: ResMut<TransitionLock>,
    mut stats: ResMut<RoomChangeStats>,
    mut changed: MessageWriter<RoomChanged>,
) {
    let pending: Vec<RoomRequest> = requests.read().cloned().collect();
    let Some(request) = choose_request(&pending) else {
        return;
    };
    stats.discarded += pending.len() as u32 - 1;
    let Some(def) = data.rooms.get(&request.room) else {
        warn!(room = %request.room, "room request for an unknown room ignored");
        stats.discarded += 1;
        return;
    };
    let (position, door, cause) = match &request.arrival {
        Arrival::Position(p) => (*p, None, RoomChangeCause::Load),
        Arrival::Door(door_id) => {
            let entry = world
                .as_ref()
                .and_then(|w| w.0.room(&request.room))
                .and_then(|room| room.door(door_id).map(|d| room.entry_point(d)));
            let Some(entry) = entry else {
                warn!(room = %request.room, door = %door_id, "door request without an entry point ignored");
                stats.discarded += 1;
                return;
            };
            (
                Vec2::new(entry.0, entry.1),
                Some(door_id.clone()),
                RoomChangeCause::Door,
            )
        }
    };
    let Ok((mut transform, mut velocity)) = player.single_mut() else {
        return;
    };

    for entity in &room_entities {
        commands.entity(entity).try_despawn();
    }
    spawn_room(&mut commands, def);
    transform.translation.x = position.x;
    transform.translation.y = position.y;
    if cause == RoomChangeCause::Load {
        velocity.0 = Vec2::ZERO;
    }
    if let Ok(mut camera) = camera.single_mut() {
        let geometry = RoomGeometry {
            solids: Vec::new(),
            size: Vec2::new(def.size.0, def.size.1),
        };
        snap_to(&mut camera, position, &geometry);
    }
    lock.locked = true;
    stats.applied += 1;
    changed.write(RoomChanged {
        from: current.map(|c| c.0.clone()).unwrap_or_default(),
        to: request.room.clone(),
        door,
        cause,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn door(room: &str) -> RoomRequest {
        RoomRequest {
            room: room.into(),
            arrival: Arrival::Door("d".into()),
        }
    }

    #[test]
    fn a_load_wins_and_the_first_door_wins_over_later_ones() {
        let load = RoomRequest {
            room: "saved".into(),
            arrival: Arrival::Position(Vec2::ZERO),
        };
        let both = [door("a"), door("b")];
        assert_eq!(choose_request(&both).map(|r| r.room.as_str()), Some("a"));
        let with_load = [door("a"), load, door("b")];
        assert_eq!(
            choose_request(&with_load).map(|r| r.room.as_str()),
            Some("saved")
        );
        assert!(choose_request(&[]).is_none());
    }
}
