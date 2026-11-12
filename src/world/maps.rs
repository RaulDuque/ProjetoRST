//! Spawns the current room: background, platform visuals and geometry.

use bevy::prelude::*;

use super::physics::RoomGeometry;
use super::rooms::RoomDefinition;
use crate::core::constants::{Z_BACKGROUND, Z_PLATFORMS};
use crate::core::resources::GameData;

/// Id of the room currently loaded.
#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct CurrentRoom(pub String);

/// Marks every entity belonging to the loaded room (despawned on change).
#[derive(Component, Debug, Clone, Copy)]
pub struct RoomEntity;

const PLATFORM_COLOR: Color = Color::srgb(0.32, 0.29, 0.36);
const ONE_WAY_COLOR: Color = Color::srgb(0.45, 0.36, 0.25);

pub fn spawn_start_room(mut commands: Commands, data: Res<GameData>) {
    let room_id = data.character.start.room.clone();
    // GameData::validate guarantees the start room exists.
    if let Some(room) = data.rooms.get(&room_id) {
        spawn_room(&mut commands, room);
    }
}

pub fn spawn_room(commands: &mut Commands, room: &RoomDefinition) {
    let size = Vec2::new(room.size.0, room.size.1);
    let (r, g, b) = room.background;
    commands.spawn((
        RoomEntity,
        Sprite::from_color(Color::srgb(r, g, b), size),
        Transform::from_xyz(size.x / 2.0, size.y / 2.0, Z_BACKGROUND),
    ));

    for platform in &room.platforms {
        let solid = platform.solid();
        let color = if platform.one_way {
            ONE_WAY_COLOR
        } else {
            PLATFORM_COLOR
        };
        commands.spawn((
            RoomEntity,
            Sprite::from_color(color, Vec2::new(platform.size.0, platform.size.1)),
            Transform::from_xyz(solid.aabb.center.0, solid.aabb.center.1, Z_PLATFORMS),
        ));
    }

    commands.insert_resource(RoomGeometry {
        solids: room.solids(),
        size,
    });
    commands.insert_resource(CurrentRoom(room.id.clone()));
    info!(room = %room.id, "room loaded");
}
