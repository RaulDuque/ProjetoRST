//! Moves projectiles and removes them when they hit level geometry.
//! Lifetime and target hits are handled by the generic hitbox systems.

use bevy::prelude::*;

use crate::combat::hitboxes::Hitbox;
use crate::core::events::{ProjectileBlocked, RoomChanged};
use crate::core::time::frame_delta;
use crate::world::collision::Aabb;
use crate::world::physics::RoomGeometry;

#[derive(Component, Debug, Clone, Copy)]
pub struct Projectile {
    pub velocity: Vec2,
}

pub fn move_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    geometry: Res<RoomGeometry>,
    mut projectiles: Query<(Entity, &Projectile, &Hitbox, &mut Transform)>,
    mut blocked: MessageWriter<ProjectileBlocked>,
) {
    let dt = frame_delta(&time);
    for (entity, projectile, hitbox, mut transform) in &mut projectiles {
        transform.translation += (projectile.velocity * dt).extend(0.0);
        let area = Aabb::new(
            (transform.translation.x, transform.translation.y),
            (hitbox.size.x, hitbox.size.y),
        );
        if geometry.blocks(&area) {
            // V1.8: tell the world where it stopped (breakable walls).
            blocked.write(ProjectileBlocked {
                center: transform.translation.truncate(),
                size: hitbox.size,
                team: hitbox.team,
                kind: hitbox.hit.kind,
                element: hitbox.hit.element,
            });
            commands.entity(entity).try_despawn();
        }
    }
}

/// Projectiles belong to the room they were cast in: a room change (door or
/// load) removes them in the same frame (`RoomStep::React`).
pub fn clear_on_room_change(
    mut commands: Commands,
    mut changes: MessageReader<RoomChanged>,
    projectiles: Query<Entity, With<Projectile>>,
) {
    if changes.read().count() == 0 {
        return;
    }
    for entity in &projectiles {
        commands.entity(entity).try_despawn();
    }
}
