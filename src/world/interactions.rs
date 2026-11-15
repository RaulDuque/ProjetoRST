//! Interactive objects of the world graph (V1.8): breakable walls, hidden
//! switches, chests and levers. Bevy layer; the rules are pure
//! (`graph::WorldObject`, `exploration::ExplorationState::{break_wall,
//! interact, object_state}`).
//!
//! - **Walls** break when a hitbox of the player (strike, technique or
//!   spell; the real combat entities, nothing new) overlaps them and the
//!   wall's `BreakCondition` accepts its damage kind and element. A wall
//!   takes no damage and has no health: the first valid hit breaks it, the
//!   following ones are ignored (it is already broken). A spell's projectile
//!   is removed by the magic module the moment it touches geometry, so its
//!   impact arrives as a `ProjectileBlocked` message instead of a hitbox.
//! - **Switches, chests and levers** are used with the interact key (`D`)
//!   while the player stands on them (`Idle` or `Walking`, the states that
//!   allow `PlayerAction::Interact`). One press uses at most one object.
//! - The state change and the grant are one call of the pure rules, so a
//!   reward is never granted twice; the item itself reaches the inventory
//!   through `PickupCollected` (the player module owns the inventory).
//! - Object entities are `RoomEntity`: they live and die with their room,
//!   and are drawn again from the exploration state on every room load.

use bevy::prelude::*;

use super::WorldState;
use super::collision::Aabb;
use super::exploration::{ExplorationState, InteractError, Interaction, ObjectState};
use super::exploration_systems::{Exploration, WorldGraph, room_solids};
use super::graph::{ChestTier, Grant, ObjectKind, WorldObject, WorldRoom};
use super::maps::{CurrentRoom, PLATFORM_COLOR, RoomEntity};
use super::physics::RoomGeometry;
use crate::combat::hitboxes::{Hitbox, PendingStrike, Team, is_armed};
use crate::core::constants::{PLAYER_SIZE, Z_PLATFORMS};
use crate::core::events::{Notification, PickupCollected, ProjectileBlocked};
use crate::core::resources::GameData;
use crate::input::PlayerInput;
use crate::player::components::{ActionState, Player};
use crate::player::state::PlayerAction;

/// An object of the current room, by its stable id.
#[derive(Component, Debug, Clone)]
pub struct ObjectMarker {
    pub id: String,
}

/// How far around an object the player's body still reaches it.
const INTERACT_MARGIN: f32 = 12.0;
/// Objects are drawn above the room extras (barriers, false walls).
const Z_OBJECTS: f32 = Z_PLATFORMS + 0.6;

const CRACKED_WALL_COLOR: Color = Color::srgb(0.42, 0.36, 0.38);
const RUBBLE_ALPHA: f32 = 0.18;
const SWITCH_HIDDEN_COLOR: Color = Color::srgb(0.35, 0.31, 0.39);
const SWITCH_FOUND_COLOR: Color = Color::srgb(0.75, 0.66, 0.32);
const LEVER_OFF_COLOR: Color = Color::srgb(0.62, 0.62, 0.68);
const LEVER_ON_COLOR: Color = Color::srgb(0.35, 0.85, 0.45);
const CHEST_OPENED_COLOR: Color = Color::srgb(0.24, 0.2, 0.16);

fn chest_color(tier: ChestTier) -> Color {
    match tier {
        ChestTier::Common => Color::srgb(0.58, 0.4, 0.2),
        ChestTier::Rare => Color::srgb(0.3, 0.46, 0.88),
        ChestTier::Secret => Color::srgb(0.82, 0.66, 0.26),
        ChestTier::BossReward => Color::srgb(0.85, 0.22, 0.22),
    }
}

/// Colour of an object in a state (the only visual of the placeholder art).
pub fn object_color(object: &WorldObject, state: ObjectState) -> Color {
    match (&object.kind, state) {
        (ObjectKind::BreakableWall { .. }, ObjectState::Broken) => {
            PLATFORM_COLOR.with_alpha(RUBBLE_ALPHA)
        }
        (ObjectKind::BreakableWall { .. }, _) => CRACKED_WALL_COLOR,
        (ObjectKind::HiddenSwitch, ObjectState::Found) => SWITCH_FOUND_COLOR,
        (ObjectKind::HiddenSwitch, _) => SWITCH_HIDDEN_COLOR,
        (ObjectKind::Chest { .. }, ObjectState::Opened) => CHEST_OPENED_COLOR,
        (ObjectKind::Chest { tier, .. }, _) => chest_color(*tier),
        (ObjectKind::Lever { .. }, ObjectState::On) => LEVER_ON_COLOR,
        (ObjectKind::Lever { .. }, _) => LEVER_OFF_COLOR,
    }
}

fn area_aabb(object: &WorldObject) -> Aabb {
    object.area.solid().aabb
}

/// Spawns the objects of `room` as they are in `state` (called once per
/// room load by `exploration_systems::prepare_room`).
pub fn spawn_objects(commands: &mut Commands, room: &WorldRoom, state: &ExplorationState) {
    for object in &room.objects {
        let a = object.area;
        commands.spawn((
            RoomEntity,
            ObjectMarker {
                id: object.id.clone(),
            },
            Sprite::from_color(
                object_color(object, state.object_state(object)),
                Vec2::new(a.size.0, a.size.1),
            ),
            Transform::from_xyz(
                a.min.0 + a.size.0 / 2.0,
                a.min.1 + a.size.1 / 2.0,
                Z_OBJECTS,
            ),
        ));
    }
}

/// Player hitboxes break the intact walls of the current room they overlap
/// (and whose condition they meet). The geometry follows in the same frame.
pub fn break_walls(
    world: Option<Res<WorldGraph>>,
    exploration: Option<ResMut<Exploration>>,
    current: Option<Res<CurrentRoom>>,
    geometry: Option<ResMut<RoomGeometry>>,
    hitboxes: Query<(&Transform, &Hitbox, Option<&PendingStrike>)>,
    mut blocked: MessageReader<ProjectileBlocked>,
    mut notifications: MessageWriter<Notification>,
) {
    // Every player attack of this frame: live hitboxes and projectiles that
    // stopped against the geometry.
    let strikes: Vec<(Aabb, _, _)> = hitboxes
        .iter()
        .filter(|(_, h, pending)| h.team == Team::Player && is_armed(*pending))
        .map(|(t, h, _)| {
            (
                Aabb::new((t.translation.x, t.translation.y), (h.size.x, h.size.y)),
                h.hit.kind,
                h.hit.element,
            )
        })
        .chain(blocked.read().filter(|b| b.team == Team::Player).map(|b| {
            (
                Aabb::new((b.center.x, b.center.y), (b.size.x, b.size.y)),
                b.kind,
                b.element,
            )
        }))
        .collect();
    if strikes.is_empty() {
        return;
    }
    let (Some(world), Some(mut exploration), Some(current), Some(mut geometry)) =
        (world, exploration, current, geometry)
    else {
        return;
    };
    let Some(room) = world.0.room(&current.0) else {
        return;
    };
    let mut broke = false;
    for wall in room
        .objects
        .iter()
        .filter(|o| matches!(o.kind, ObjectKind::BreakableWall { .. }))
    {
        if exploration.object_state(wall) != ObjectState::Intact {
            continue;
        }
        let area = area_aabb(wall);
        for (strike, kind, element) in &strikes {
            if !strike.intersects(&area) {
                continue;
            }
            match exploration.break_wall(&world.0, room, &wall.id, *kind, *element) {
                Ok(_) => {
                    notifications.write(Notification("The wall crumbles!".into()));
                    broke = true;
                    break;
                }
                Err(InteractError::Sealed) => {
                    notifications.write(Notification("The wall does not yield.".into()));
                    break;
                }
                Err(_) => {}
            }
        }
    }
    if broke {
        geometry.solids = room_solids(room, &exploration);
    }
}

fn reaches(player: &Transform, object: &WorldObject) -> bool {
    let body = Aabb::new(
        (player.translation.x, player.translation.y),
        (
            PLAYER_SIZE.0 + 2.0 * INTERACT_MARGIN,
            PLAYER_SIZE.1 + 2.0 * INTERACT_MARGIN,
        ),
    );
    body.intersects(&area_aabb(object))
}

/// The interactive object the player stands at that can still be used
/// (V2.1: shared with NPC conversations, so `D` never does both). Objects
/// win over NPCs: the first usable object in data order is the target.
pub fn usable_object<'a>(
    room: &'a WorldRoom,
    exploration: &ExplorationState,
    player: &Transform,
) -> Option<&'a WorldObject> {
    room.objects
        .iter()
        .filter(|o| o.is_interactive() && reaches(player, o))
        .find(|o| {
            !matches!(
                exploration.object_state(o),
                ObjectState::Found | ObjectState::Opened | ObjectState::On
            )
        })
}

fn reward_name(data: &GameData, grant: &Grant) -> String {
    match grant {
        Grant::Item(id) => data
            .items
            .get(id)
            .map_or_else(|| id.clone(), |item| item.name.clone()),
        Grant::Ability(a) => format!("{a:?}"),
        Grant::Flag(f) => f.clone(),
    }
}

/// The interact key uses the first object of the current room the player
/// stands on: a hidden switch is found, a chest opens, a lever is pulled.
pub fn interact_with_objects(
    input: Res<PlayerInput>,
    data: Res<GameData>,
    world: Option<Res<WorldGraph>>,
    exploration: Option<ResMut<Exploration>>,
    mut world_state: ResMut<WorldState>,
    current: Option<Res<CurrentRoom>>,
    player: Query<(&Transform, &ActionState), With<Player>>,
    mut collected: MessageWriter<PickupCollected>,
    mut notifications: MessageWriter<Notification>,
) {
    if !input.interact_pressed {
        return;
    }
    let (Some(world), Some(mut exploration), Some(current)) = (world, exploration, current) else {
        return;
    };
    let (Ok((transform, action)), Some(room)) = (player.single(), world.0.room(&current.0)) else {
        return;
    };
    if !action.state().allows(PlayerAction::Interact) {
        return;
    }
    let Some(object) = usable_object(room, &exploration, transform) else {
        return;
    };
    match exploration.interact(&world.0, room, &object.id) {
        Ok(Interaction::SecretFound(_)) => {
            notifications.write(Notification("A hidden passage!".into()));
        }
        Ok(Interaction::Granted(grant)) => {
            let text = match (&object.kind, &grant) {
                (ObjectKind::Lever { .. }, _) => "A mechanism clanks somewhere.".to_owned(),
                (_, Grant::Ability(a)) => format!("Relic found: {a:?} learned"),
                (_, other) => format!("Found {}", reward_name(&data, other)),
            };
            if let Grant::Flag(flag) = &grant {
                world_state.set_flag(flag);
            }
            notifications.write(Notification(text));
            collected.write(PickupCollected {
                pickup: object.id.clone(),
                grant,
            });
        }
        Err(InteractError::Sealed) => {
            notifications.write(Notification("It is sealed.".into()));
        }
        Err(_) => {}
    }
}

/// Redraws the objects of the room when the exploration state changes
/// (broken, found, opened, pulled; also after a quick load).
pub fn sync_object_visuals(
    world: Option<Res<WorldGraph>>,
    exploration: Option<Res<Exploration>>,
    current: Option<Res<CurrentRoom>>,
    mut objects: Query<(&ObjectMarker, &mut Sprite)>,
) {
    let (Some(world), Some(exploration), Some(current)) = (world, exploration, current) else {
        return;
    };
    if !exploration.is_changed() {
        return;
    }
    let Some(room) = world.0.room(&current.0) else {
        return;
    };
    for (marker, mut sprite) in &mut objects {
        if let Some(object) = room.object(&marker.id) {
            let color = object_color(object, exploration.object_state(object));
            if sprite.color != color {
                sprite.color = color;
            }
        }
    }
}
