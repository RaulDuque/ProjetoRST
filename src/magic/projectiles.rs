//! Moves projectiles and removes them when they hit level geometry.
//! Lifetime and target hits are handled by the generic hitbox systems.

use bevy::prelude::*;

use crate::combat::hitboxes::Hitbox;
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
) {
    let dt = frame_delta(&time);
    for (entity, projectile, hitbox, mut transform) in &mut projectiles {
        transform.translation += (projectile.velocity * dt).extend(0.0);
        let area = Aabb::new(
            (transform.translation.x, transform.translation.y),
            (hitbox.size.x, hitbox.size.y),
        );
        if geometry.blocks(&area) {
            commands.entity(entity).try_despawn();
        }
    }
}
