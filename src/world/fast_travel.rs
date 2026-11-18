//! Fast travel between discovered save altars (D19). Pure rules: no Bevy types.
//!
//! Destinations come from the world graph (`ObjectKind::SavePoint`) filtered by
//! `WorldProgress::discovered_save_points`. Travel reuses `RoomRequest` +
//! `Arrival::Position`; resting/saving stays on the D17 path.

use super::graph::{ObjectKind, WALL, WorldDefinition};
use super::progress::WorldProgress;
use crate::core::constants::PLAYER_SIZE;
use crate::player::state::PlayerState;

/// One save altar as a travel site.
#[derive(Debug, Clone, PartialEq)]
pub struct SavePointSite {
    pub id: String,
    pub room_id: String,
    pub room_name: String,
    pub region_id: String,
    /// Player centre on arrival (room coordinates).
    pub arrival: (f32, f32),
}

/// Why a fast-travel request was refused. Nothing in the world changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FastTravelError {
    UnknownOrigin(String),
    UnknownDestination(String),
    DestinationNotDiscovered(String),
    SameAltar,
    Dead,
    Busy,
    TransitionLocked,
    NoDestinations,
}

impl std::fmt::Display for FastTravelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownOrigin(id) => write!(f, "unknown origin altar `{id}`"),
            Self::UnknownDestination(id) => write!(f, "unknown destination altar `{id}`"),
            Self::DestinationNotDiscovered(id) => write!(f, "altar `{id}` is not discovered"),
            Self::SameAltar => write!(f, "already at this altar"),
            Self::Dead => write!(f, "cannot travel while fallen"),
            Self::Busy => write!(f, "cannot travel right now"),
            Self::TransitionLocked => write!(f, "cannot travel during a room change"),
            Self::NoDestinations => write!(f, "no other discovered altars"),
        }
    }
}

/// Standing centre above the altar's floor contact (matches headless AT_SAVE).
pub fn arrival_at(area_min: (f32, f32), area_size: (f32, f32)) -> (f32, f32) {
    let x = area_min.0 + area_size.0 * 0.5;
    let floor = area_min.1.max(WALL);
    let y = floor + PLAYER_SIZE.1 * 0.5 + 1.0;
    (x, y)
}

/// Every `SavePoint` in the world, ordered by id (deterministic).
pub fn sites(world: &WorldDefinition) -> Vec<SavePointSite> {
    let mut out = Vec::new();
    for room in world.rooms.values() {
        for object in &room.objects {
            if !matches!(object.kind, ObjectKind::SavePoint) {
                continue;
            }
            out.push(SavePointSite {
                id: object.id.clone(),
                room_id: room.id.clone(),
                room_name: room.name.clone(),
                region_id: room.region.clone(),
                arrival: arrival_at(object.area.min, object.area.size),
            });
        }
    }
    out.sort_by(|a, b| a.id.cmp(&b.id));
    out
}

/// Looks up one altar by id.
pub fn find_site(world: &WorldDefinition, id: &str) -> Option<SavePointSite> {
    for room in world.rooms.values() {
        for object in &room.objects {
            if object.id == id && matches!(object.kind, ObjectKind::SavePoint) {
                return Some(SavePointSite {
                    id: object.id.clone(),
                    room_id: room.id.clone(),
                    room_name: room.name.clone(),
                    region_id: room.region.clone(),
                    arrival: arrival_at(object.area.min, object.area.size),
                });
            }
        }
    }
    None
}

/// Discovered altars except `origin`, sorted by id.
pub fn destinations(
    world: &WorldDefinition,
    progress: &WorldProgress,
    origin: &str,
) -> Vec<SavePointSite> {
    sites(world)
        .into_iter()
        .filter(|s| s.id != origin && progress.discovered_save_points.contains(&s.id))
        .collect()
}

/// Whether the player may open or confirm fast travel (from an altar menu).
pub fn travel_allowed(state: PlayerState, transition_locked: bool) -> bool {
    if state == PlayerState::Dead || transition_locked {
        return false;
    }
    // Menu is opened after a successful rest while idle/walking at the altar.
    state.is_locomotion()
}

/// Validates a trip and returns the destination site on success.
pub fn plan_travel(
    world: &WorldDefinition,
    progress: &WorldProgress,
    origin: &str,
    destination: &str,
    state: PlayerState,
    transition_locked: bool,
) -> Result<SavePointSite, FastTravelError> {
    if state == PlayerState::Dead {
        return Err(FastTravelError::Dead);
    }
    if transition_locked {
        return Err(FastTravelError::TransitionLocked);
    }
    if !travel_allowed(state, false) {
        return Err(FastTravelError::Busy);
    }
    let _origin = find_site(world, origin)
        .ok_or_else(|| FastTravelError::UnknownOrigin(origin.to_owned()))?;
    if origin == destination {
        return Err(FastTravelError::SameAltar);
    }
    let dest = find_site(world, destination)
        .ok_or_else(|| FastTravelError::UnknownDestination(destination.to_owned()))?;
    if !progress.discovered_save_points.contains(destination) {
        return Err(FastTravelError::DestinationNotDiscovered(
            destination.to_owned(),
        ));
    }
    Ok(dest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::graph::{
        MapRect, ObjectKind, RegionDef, RoomRole, WorldDefinition, WorldManifest, WorldObject,
        WorldRoom,
    };
    use crate::world::progress::WorldProgress;
    use crate::world::rooms::Platform;

    fn world_two_altars() -> WorldDefinition {
        let room = |id: &str, name: &str, altar: &str, x: f32| WorldRoom {
            id: id.into(),
            name: name.into(),
            region: "vigil_gates".into(),
            role: RoomRole::Required,
            map: MapRect {
                x: 0,
                y: 0,
                w: 1,
                h: 1,
            },
            background: (0.0, 0.0, 0.0),
            platforms: vec![],
            enemies: vec![],
            doors: vec![],
            pickups: vec![],
            secrets: vec![],
            objects: vec![WorldObject {
                id: altar.into(),
                kind: ObjectKind::SavePoint,
                area: Platform {
                    min: (x, 40.0),
                    size: (40.0, 48.0),
                    one_way: false,
                    barrier: None,
                },
                requires: None,
            }],
            veils: vec![],
            decor: vec![],
        };
        let mut rooms = std::collections::BTreeMap::new();
        rooms.insert(
            "room_a".into(),
            room("room_a", "Gate Approach", "altar_a", 340.0),
        );
        rooms.insert(
            "room_b".into(),
            room("room_b", "Broken Causeway", "altar_b", 1920.0),
        );
        let mut regions = std::collections::BTreeMap::new();
        regions.insert(
            "vigil_gates".into(),
            RegionDef {
                id: "vigil_gates".into(),
                name: "Vigil Gates".into(),
                color: (0.5, 0.4, 0.6),
                visual: None,
            },
        );
        WorldDefinition {
            manifest: WorldManifest {
                start_room: "room_a".into(),
                start_door: String::new(),
                goal_room: "room_b".into(),
                legacy_rooms: Default::default(),
                passages: Default::default(),
            },
            regions,
            rooms,
        }
    }

    #[test]
    fn lists_sites_sorted_and_destinations_exclude_origin() {
        let world = world_two_altars();
        let all = sites(&world);
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].id, "altar_a");
        assert_eq!(all[1].id, "altar_b");
        let mut progress = WorldProgress::default();
        progress.discover_save_point("altar_a");
        progress.discover_save_point("altar_b");
        let dests = destinations(&world, &progress, "altar_a");
        assert_eq!(dests.len(), 1);
        assert_eq!(dests[0].id, "altar_b");
    }

    #[test]
    fn plan_travel_requires_discovery_and_refuses_same_or_dead() {
        let world = world_two_altars();
        let mut progress = WorldProgress::default();
        progress.discover_save_point("altar_a");
        assert_eq!(
            plan_travel(
                &world,
                &progress,
                "altar_a",
                "altar_b",
                PlayerState::Idle,
                false
            ),
            Err(FastTravelError::DestinationNotDiscovered("altar_b".into()))
        );
        progress.discover_save_point("altar_b");
        let ok = plan_travel(
            &world,
            &progress,
            "altar_a",
            "altar_b",
            PlayerState::Idle,
            false,
        )
        .unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(ok.room_id, "room_b");
        assert_eq!(
            plan_travel(
                &world,
                &progress,
                "altar_a",
                "altar_a",
                PlayerState::Idle,
                false
            ),
            Err(FastTravelError::SameAltar)
        );
        assert_eq!(
            plan_travel(
                &world,
                &progress,
                "altar_a",
                "altar_b",
                PlayerState::Dead,
                false
            ),
            Err(FastTravelError::Dead)
        );
        assert_eq!(
            plan_travel(
                &world,
                &progress,
                "altar_a",
                "altar_b",
                PlayerState::Idle,
                true
            ),
            Err(FastTravelError::TransitionLocked)
        );
        assert_eq!(
            plan_travel(
                &world,
                &progress,
                "altar_a",
                "altar_b",
                PlayerState::Attacking,
                false
            ),
            Err(FastTravelError::Busy)
        );
    }

    #[test]
    fn arrival_centres_on_altar_floor() {
        let (x, y) = arrival_at((340.0, 40.0), (40.0, 48.0));
        assert!((x - 360.0).abs() < f32::EPSILON);
        assert!((y - (40.0 + PLAYER_SIZE.1 * 0.5 + 1.0)).abs() < f32::EPSILON);
    }
}
