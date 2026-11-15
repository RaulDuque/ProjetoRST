//! Where a loaded save puts the player (V1.7). Pure module: no Bevy types.
//!
//! A save names a room id and a position. Before anything of the game is
//! touched, the load resolves them against the content that is loaded now:
//!
//! 1. the room exists in the world graph → that room;
//! 2. it is a legacy room (`world.ron` `legacy_rooms`, e.g. the pre-graph
//!    `entry_hall` of a save v1) → the mapped door of the graph;
//! 3. it is unknown (removed content) → the world's start door;
//! 4. without a world graph (the single-room test game) the room must exist
//!    in `GameData`, or the load is refused (behaviour of the save v1).
//!
//! The position is kept only if the player's body fits inside the room and
//! does not overlap solid geometry; otherwise the room's spawn point is used.
//! Every substitution is reported as a [`PlacementNote`]: the load shows
//! it to the player and logs it, it never happens silently.

use std::collections::BTreeMap;

use thiserror::Error;

use super::collision::Aabb;
use super::graph::{DoorTarget, WorldDefinition};
use super::rooms::RoomDefinition;
use crate::core::constants::PLAYER_SIZE;

#[derive(Debug, Clone, PartialEq)]
pub enum PlacementNote {
    /// The saved room predates the world graph; resumed at its mapped door.
    LegacyRoom { saved: String, resumed: String },
    /// The saved room no longer exists; resumed at the world's start.
    UnknownRoom { saved: String, resumed: String },
    /// The saved position was outside the room or inside a wall.
    PositionReset { room: String },
}

impl PlacementNote {
    /// Short text for the on-screen notification.
    pub fn message(&self) -> String {
        match self {
            PlacementNote::LegacyRoom { saved, resumed } => {
                format!("Old save: `{saved}` resumes at `{resumed}`")
            }
            PlacementNote::UnknownRoom { saved, resumed } => {
                format!("Room `{saved}` no longer exists: resumed at `{resumed}`")
            }
            PlacementNote::PositionReset { room } => {
                format!("Saved position invalid: moved to the entrance of `{room}`")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Placement {
    pub room: String,
    pub position: (f32, f32),
    pub notes: Vec<PlacementNote>,
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum PlacementError {
    #[error("unknown room `{0}`")]
    UnknownRoom(String),
}

/// Whether the player's body centred at `p` is inside `room` and clear of
/// its solid (not one-way) geometry.
pub fn position_is_valid(room: &RoomDefinition, p: (f32, f32)) -> bool {
    if !p.0.is_finite() || !p.1.is_finite() {
        return false;
    }
    let half = (PLAYER_SIZE.0 / 2.0, PLAYER_SIZE.1 / 2.0);
    let inside = p.0 - half.0 >= 0.0
        && p.0 + half.0 <= room.size.0
        && p.1 - half.1 >= 0.0
        && p.1 + half.1 <= room.size.1;
    let body = Aabb::new(p, PLAYER_SIZE);
    inside
        && !room
            .solids()
            .iter()
            .any(|s| !s.one_way && s.aabb.intersects(&body))
}

fn door_entry(world: &WorldDefinition, target: &DoorTarget) -> Option<(f32, f32)> {
    let room = world.room(&target.room)?;
    Some(room.entry_point(room.door(&target.door)?))
}

/// Resolves a saved room and position (see the module docs).
pub fn resolve_placement(
    world: Option<&WorldDefinition>,
    rooms: &BTreeMap<String, RoomDefinition>,
    saved_room: &str,
    saved_position: (f32, f32),
) -> Result<Placement, PlacementError> {
    let mut notes = Vec::new();
    let (room, position) = match world {
        Some(world) if world.room(saved_room).is_some() => (saved_room.to_owned(), saved_position),
        Some(world) => {
            let (target, legacy) = match world.manifest.legacy_rooms.get(saved_room) {
                Some(target) => (target.clone(), true),
                None => (
                    DoorTarget {
                        room: world.manifest.start_room.clone(),
                        door: world.manifest.start_door.clone(),
                    },
                    false,
                ),
            };
            let entry = door_entry(world, &target)
                .ok_or_else(|| PlacementError::UnknownRoom(target.room.clone()))?;
            let saved = saved_room.to_owned();
            let resumed = target.room.clone();
            notes.push(if legacy {
                PlacementNote::LegacyRoom { saved, resumed }
            } else {
                PlacementNote::UnknownRoom { saved, resumed }
            });
            (target.room, entry)
        }
        None => (saved_room.to_owned(), saved_position),
    };
    let def = rooms
        .get(&room)
        .ok_or_else(|| PlacementError::UnknownRoom(room.clone()))?;
    let position = if position_is_valid(def, position) {
        position
    } else {
        notes.push(PlacementNote::PositionReset { room: room.clone() });
        def.player_spawn
    };
    Ok(Placement {
        room,
        position,
        notes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::rooms::Platform;

    fn room() -> RoomDefinition {
        RoomDefinition {
            id: "r".into(),
            name: "R".into(),
            size: (1000.0, 600.0),
            background: (0.0, 0.0, 0.0),
            platforms: vec![
                Platform {
                    min: (0.0, 0.0),
                    size: (1000.0, 40.0),
                    one_way: false,
                    barrier: None,
                },
                Platform {
                    min: (300.0, 100.0),
                    size: (100.0, 16.0),
                    one_way: true,
                    barrier: None,
                },
            ],
            player_spawn: (100.0, 64.0),
            enemies: vec![],
        }
    }

    #[test]
    fn positions_inside_walls_or_outside_the_room_are_invalid() {
        let r = room();
        assert!(position_is_valid(&r, (500.0, 64.0)));
        assert!(position_is_valid(&r, (350.0, 108.0)), "one-way is passable");
        assert!(!position_is_valid(&r, (500.0, 20.0)), "inside the floor");
        assert!(!position_is_valid(&r, (-50.0, 64.0)));
        assert!(!position_is_valid(&r, (500.0, 5000.0)));
        assert!(!position_is_valid(&r, (f32::NAN, 64.0)));
    }

    #[test]
    fn without_a_world_unknown_rooms_are_refused_and_bad_positions_reset() {
        let mut rooms = BTreeMap::new();
        rooms.insert("r".to_owned(), room());
        let ok =
            resolve_placement(None, &rooms, "r", (500.0, 64.0)).unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(ok.position, (500.0, 64.0));
        assert!(ok.notes.is_empty());
        let reset =
            resolve_placement(None, &rooms, "r", (500.0, -300.0)).unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(reset.position, (100.0, 64.0));
        assert_eq!(
            reset.notes,
            vec![PlacementNote::PositionReset { room: "r".into() }]
        );
        assert_eq!(
            resolve_placement(None, &rooms, "gone", (0.0, 0.0)),
            Err(PlacementError::UnknownRoom("gone".into()))
        );
    }
}
