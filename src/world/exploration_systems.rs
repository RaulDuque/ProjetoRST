//! Bevy layer of the world graph (V1.6 prototype, in the main game since V1.7).
//!
//! [`WorldExplorationPlugin`] turns the room loader into a graph of rooms:
//! doors, barriers, secrets, pickups and the exploration state. It is part
//! of `GamePlugin`; its systems do nothing until a world is activated with
//! [`activate_world`] (done by `app::run`), so the single-room test game of
//! `assets/maps/` keeps working unchanged. Details: docs/ROOM_SYSTEM.md and
//! docs/WORLD_INTEGRATION_V1_7.md.
//!
//! Ownership: a door only *requests* a room change (`RoomRequest`); the
//! change itself is made by `world::transitions::apply_room_requests`, and
//! each module cleans up what it owns when it reads `RoomChanged`.

use bevy::prelude::*;

use super::WorldState;
use super::collision::Solid;
use super::exploration::{DoorState, ExplorationState, ObjectState};
use super::graph::{Door, Grant, WorldDefinition, WorldRoom};
use super::interactions;
use super::maps::{CurrentRoom, PLATFORM_COLOR, RoomEntity};
use super::physics::RoomGeometry;
use super::transitions::{RoomStep, TransitionLock};
use crate::app::states::GameSet;
use crate::core::constants::Z_PLATFORMS;
use crate::core::events::{Arrival, Notification, PickupCollected, RoomRequest};
use crate::core::resources::GameData;
use crate::player::components::{ActionState, Player};
use crate::player::state::PlayerState;

/// The world being explored.
#[derive(Resource, Debug, Clone)]
pub struct WorldGraph(pub WorldDefinition);

/// What the player has explored (saved in the save v2 since V1.7).
/// Present only while a world is active ([`activate_world`]).
#[derive(Resource, Debug, Clone, Default, Deref, DerefMut)]
pub struct Exploration(pub ExplorationState);

/// Blocks a door whose requirement is not met.
#[derive(Component, Debug, Clone)]
pub struct DoorBarrier {
    pub door: String,
}

/// Looks like wall, has no collision; walking into it finds the secret.
#[derive(Component, Debug, Clone)]
pub struct IllusoryWall {
    pub secret: String,
}

#[derive(Component, Debug, Clone)]
pub struct PickupMarker {
    pub id: String,
}

const BARRIER_COLOR: Color = Color::srgb(0.32, 0.42, 0.72);
const PICKUP_COLOR: Color = Color::srgb(0.95, 0.78, 0.3);
const PICKUP_SIZE: f32 = 18.0;
/// A pickup is taken when the player's centre gets this close.
const PICKUP_REACH: f32 = 36.0;
/// Alpha of a false wall once found (still visible, clearly passable).
const FOUND_WALL_ALPHA: f32 = 0.35;
/// Room extras above the platforms, below enemies and the player.
const Z_ROOM_EXTRAS: f32 = Z_PLATFORMS + 0.5;

/// Puts the world's rooms into `GameData` (in memory, for the existing room
/// loader, enemy spawner and respawn) and makes its start room the game's
/// start, with the player at the start door.
pub fn install_world(data: &mut GameData, world: &WorldDefinition) {
    for room in world.rooms.values() {
        let mut def = room.to_room_definition();
        if room.id == world.manifest.start_room
            && let Some(door) = room.door(&world.manifest.start_door)
        {
            def.player_spawn = room.entry_point(door);
        }
        data.rooms.insert(room.id.clone(), def);
    }
    data.character.start.room = world.manifest.start_room.clone();
}

/// How the opening of `door` is closed right now, if it is: a locked door
/// has a blue barrier; a door hidden by a breakable wall or a hidden switch
/// not found yet is masonry (V1.8). A door hidden by a false wall stays
/// open: walking into the false wall finds it.
pub fn sealed_opening(room: &WorldRoom, door: &Door, state: &ExplorationState) -> Option<Color> {
    match state.door_state(door) {
        DoorState::Locked => Some(BARRIER_COLOR),
        DoorState::Hidden
            if door
                .hidden_by
                .as_deref()
                .is_some_and(|s| room.object(s).is_some()) =>
        {
            Some(PLATFORM_COLOR)
        }
        _ => None,
    }
}

/// Solids of a room for the current exploration state: generated layout,
/// interior, a barrier in every sealed door and every intact breakable wall.
pub fn room_solids(room: &WorldRoom, state: &ExplorationState) -> Vec<Solid> {
    let mut solids = room.to_room_definition().solids();
    solids.extend(
        room.doors
            .iter()
            .filter(|d| sealed_opening(room, d, state).is_some())
            .map(|d| room.barrier(d).solid()),
    );
    solids.extend(
        room.objects
            .iter()
            .filter(|o| state.object_state(o) == ObjectState::Intact)
            .map(|o| o.area.solid()),
    );
    solids
}

/// Makes `world` the active world of `app`: the graph and a fresh
/// exploration state. The rooms must already be in `GameData`
/// ([`install_world`]).
pub fn activate_world(app: &mut App, world: WorldDefinition) {
    app.insert_resource(WorldGraph(world))
        .insert_resource(Exploration::default());
}

pub struct WorldExplorationPlugin;

impl Plugin for WorldExplorationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                door_transitions,
                find_secrets,
                interactions::break_walls,
                interactions::interact_with_objects,
                collect_pickups,
                open_barriers,
                interactions::sync_object_visuals,
            )
                .chain()
                .in_set(GameSet::Resolution)
                .after(crate::player::systems::receive_damage),
        )
        .add_systems(
            Update,
            (prepare_room, rearm_doors).chain().in_set(RoomStep::React),
        );
    }
}

fn player_position(player: &Transform) -> (f32, f32) {
    (player.translation.x, player.translation.y)
}

/// Asks for the room behind an open door the player's centre has reached.
/// Ignored while the player is dead or the transition lock is armed; when
/// two doors are crossed in the same frame the first one of the room data
/// wins. The change itself is `transitions::apply_room_requests`.
pub fn door_transitions(
    world: Option<Res<WorldGraph>>,
    exploration: Option<Res<Exploration>>,
    current: Option<Res<CurrentRoom>>,
    lock: Res<TransitionLock>,
    player: Query<(&Transform, &ActionState), With<Player>>,
    mut requests: MessageWriter<RoomRequest>,
) {
    let (Some(world), Some(exploration), Some(current)) = (world, exploration, current) else {
        return;
    };
    if lock.is_locked() {
        return;
    }
    let Ok((transform, action)) = player.single() else {
        return;
    };
    if action.state() == PlayerState::Dead {
        return;
    }
    let Some(room) = world.0.room(&current.0) else {
        return;
    };
    let position = player_position(transform);
    let Some((target_room, target_door)) = room
        .doors
        .iter()
        .filter(|d| exploration.door_state(d) == DoorState::Open && room.crosses(d, position))
        .find_map(|d| world.0.target(d))
    else {
        return;
    };
    requests.write(RoomRequest {
        room: target_room.id.clone(),
        arrival: Arrival::Door(target_door.id.clone()),
    });
}

/// Unlocks the doors once the player stands clear of every door trigger of
/// the current room (see `transitions::TransitionLock`).
pub fn rearm_doors(
    world: Option<Res<WorldGraph>>,
    current: Option<Res<CurrentRoom>>,
    player: Query<&Transform, With<Player>>,
    mut lock: ResMut<TransitionLock>,
) {
    if !lock.is_locked() {
        return;
    }
    let room = world
        .as_ref()
        .zip(current.as_ref())
        .and_then(|(w, c)| w.0.room(&c.0));
    let on_a_trigger = match (room, player.single()) {
        (Some(room), Ok(t)) => {
            let p = player_position(t);
            room.doors.iter().any(|d| room.crosses(d, p))
        }
        _ => false,
    };
    if !on_a_trigger {
        lock.locked = false;
    }
}

/// Runs once per room load (start, transition): spawns the barriers, false
/// walls and pickups the exploration state calls for, adds the barriers to
/// the geometry and records the visit.
pub fn prepare_room(
    mut commands: Commands,
    world: Option<Res<WorldGraph>>,
    exploration: Option<ResMut<Exploration>>,
    current: Option<Res<CurrentRoom>>,
    geometry: Option<ResMut<RoomGeometry>>,
    mut prepared: Local<Option<String>>,
) {
    let (Some(world), Some(mut exploration), Some(current), Some(mut geometry)) =
        (world, exploration, current, geometry)
    else {
        return;
    };
    if prepared.as_deref() == Some(current.0.as_str()) && !current.is_changed() {
        return;
    }
    let Some(room) = world.0.room(&current.0) else {
        return;
    };
    *prepared = Some(current.0.clone());
    exploration.enter_room(&world.0, &room.id);
    for door in &room.doors {
        let Some(color) = sealed_opening(room, door, &exploration) else {
            continue;
        };
        let barrier = room.barrier(door);
        commands.spawn((
            RoomEntity,
            DoorBarrier {
                door: door.id.clone(),
            },
            Sprite::from_color(color, Vec2::new(barrier.size.0, barrier.size.1)),
            Transform::from_xyz(
                barrier.min.0 + barrier.size.0 / 2.0,
                barrier.min.1 + barrier.size.1 / 2.0,
                Z_ROOM_EXTRAS,
            ),
        ));
    }
    for secret in &room.secrets {
        let found = exploration.secrets.contains(&secret.id);
        let alpha = if found { FOUND_WALL_ALPHA } else { 1.0 };
        let area = secret.area;
        commands.spawn((
            RoomEntity,
            IllusoryWall {
                secret: secret.id.clone(),
            },
            Sprite::from_color(
                PLATFORM_COLOR.with_alpha(alpha),
                Vec2::new(area.size.0, area.size.1),
            ),
            Transform::from_xyz(
                area.min.0 + area.size.0 / 2.0,
                area.min.1 + area.size.1 / 2.0,
                Z_ROOM_EXTRAS,
            ),
        ));
    }
    for pickup in room
        .pickups
        .iter()
        .filter(|p| !exploration.pickups.contains(&p.id))
    {
        commands.spawn((
            RoomEntity,
            PickupMarker {
                id: pickup.id.clone(),
            },
            Sprite::from_color(PICKUP_COLOR, Vec2::splat(PICKUP_SIZE)),
            Transform::from_xyz(pickup.position.0, pickup.position.1, Z_ROOM_EXTRAS),
        ));
    }
    interactions::spawn_objects(&mut commands, room, &exploration);
    geometry.solids = room_solids(room, &exploration);
}

/// Walking into a false wall finds its secret (and reveals the doors it hid).
pub fn find_secrets(
    world: Option<Res<WorldGraph>>,
    exploration: Option<ResMut<Exploration>>,
    current: Option<Res<CurrentRoom>>,
    player: Query<&Transform, With<Player>>,
    mut walls: Query<(&IllusoryWall, &mut Sprite)>,
    mut notifications: MessageWriter<Notification>,
) {
    let (Some(world), Some(mut exploration), Some(current)) = (world, exploration, current) else {
        return;
    };
    let (Ok(player), Some(room)) = (player.single(), world.0.room(&current.0)) else {
        return;
    };
    let position = player_position(player);
    for (wall, mut sprite) in &mut walls {
        let Some(def) = room.secrets.iter().find(|s| s.id == wall.secret) else {
            continue;
        };
        let inside = position.0 >= def.area.min.0
            && position.0 <= def.area.min.0 + def.area.size.0
            && position.1 >= def.area.min.1
            && position.1 <= def.area.min.1 + def.area.size.1;
        if inside && exploration.find_secret(&world.0, &room.id, &def.id) {
            sprite.color = sprite.color.with_alpha(FOUND_WALL_ALPHA);
            notifications.write(Notification("A hidden passage!".into()));
        }
    }
}

/// Touching a pickup collects it once (kept across visits and loads).
/// The grant is recorded in the exploration state (requirements read it);
/// a `Flag` is also set in the world flags (the one store skills read) and
/// `PickupCollected` lets the inventory receive an `Item`.
pub fn collect_pickups(
    mut commands: Commands,
    world: Option<Res<WorldGraph>>,
    exploration: Option<ResMut<Exploration>>,
    mut world_state: ResMut<WorldState>,
    current: Option<Res<CurrentRoom>>,
    player: Query<&Transform, With<Player>>,
    pickups: Query<(Entity, &PickupMarker, &Transform), Without<Player>>,
    mut collected: MessageWriter<PickupCollected>,
    mut notifications: MessageWriter<Notification>,
) {
    let (Some(world), Some(mut exploration), Some(current)) = (world, exploration, current) else {
        return;
    };
    let (Ok(player), Some(room)) = (player.single(), world.0.room(&current.0)) else {
        return;
    };
    for (entity, marker, transform) in &pickups {
        if player
            .translation
            .truncate()
            .distance(transform.translation.truncate())
            > PICKUP_REACH
        {
            continue;
        }
        commands.entity(entity).try_despawn();
        let Some(grant) = exploration.collect(room, &marker.id) else {
            continue;
        };
        let name = room
            .pickups
            .iter()
            .find(|p| p.id == marker.id)
            .map_or("item", |p| p.name.as_str());
        let what = match &grant {
            Grant::Ability(a) => format!("{name}: {a:?} learned"),
            Grant::Item(_) | Grant::Flag(_) => format!("Found {name}"),
        };
        if let Grant::Flag(flag) = &grant {
            world_state.set_flag(flag);
        }
        notifications.write(Notification(what));
        collected.write(PickupCollected {
            pickup: marker.id.clone(),
            grant,
        });
    }
}

/// When a requirement becomes met or a hidden door is found, its barrier
/// disappears; the geometry is rebuilt from the state (barriers, walls).
pub fn open_barriers(
    mut commands: Commands,
    world: Option<Res<WorldGraph>>,
    exploration: Option<Res<Exploration>>,
    current: Option<Res<CurrentRoom>>,
    geometry: Option<ResMut<RoomGeometry>>,
    barriers: Query<(Entity, &DoorBarrier)>,
) {
    let (Some(world), Some(exploration), Some(current), Some(mut geometry)) =
        (world, exploration, current, geometry)
    else {
        return;
    };
    if !exploration.is_changed() {
        return;
    }
    let Some(room) = world.0.room(&current.0) else {
        return;
    };
    for (entity, barrier) in &barriers {
        let still_sealed = room
            .door(&barrier.door)
            .is_some_and(|d| sealed_opening(room, d, &exploration).is_some());
        if !still_sealed {
            commands.entity(entity).try_despawn();
        }
    }
    geometry.solids = room_solids(room, &exploration);
}
