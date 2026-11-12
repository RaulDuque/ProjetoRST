//! Bevy glue for platform physics shared by the player and enemies.

use bevy::prelude::*;

use super::collision::{Aabb, Solid, move_and_collide};
use crate::core::constants::{GRAVITY, MAX_FALL_SPEED};
use crate::core::time::frame_delta;
use crate::player::state::Facing;

/// Collision box size, centred on the entity's translation.
#[derive(Component, Debug, Clone, Copy)]
pub struct Body {
    pub size: Vec2,
}

impl Body {
    pub fn aabb_at(&self, position: Vec2) -> Aabb {
        Aabb::new((position.x, position.y), (self.size.x, self.size.y))
    }
}

#[derive(Component, Debug, Clone, Copy, Default, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

/// Result of the last physics step.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Grounded {
    pub on_ground: bool,
    /// Seconds since the body last stood on ground.
    pub air_time: f32,
}

/// Whether gravity applies to the body.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct AffectedByGravity;

#[derive(Component, Debug, Clone, Copy, Default, Deref, DerefMut)]
pub struct FacingDirection(pub Facing);

/// Static geometry of the current room.
#[derive(Resource, Debug, Clone, Default)]
pub struct RoomGeometry {
    pub solids: Vec<Solid>,
    pub size: Vec2,
}

impl RoomGeometry {
    /// Whether a box overlaps any solid (one-way platforms excluded).
    pub fn blocks(&self, aabb: &Aabb) -> bool {
        self.solids
            .iter()
            .any(|s| !s.one_way && s.aabb.intersects(aabb))
    }
}

pub fn apply_gravity(time: Res<Time>, mut bodies: Query<&mut Velocity, With<AffectedByGravity>>) {
    let dt = frame_delta(&time);
    for mut velocity in &mut bodies {
        velocity.y = (velocity.y + GRAVITY * dt).max(MAX_FALL_SPEED);
    }
}

pub fn move_bodies(
    time: Res<Time>,
    geometry: Res<RoomGeometry>,
    mut bodies: Query<(&Body, &mut Transform, &mut Velocity, &mut Grounded)>,
) {
    let dt = frame_delta(&time);
    for (body, mut transform, mut velocity, mut grounded) in &mut bodies {
        let aabb = body.aabb_at(transform.translation.truncate());
        let result = move_and_collide(aabb, (velocity.x, velocity.y), dt, &geometry.solids);
        transform.translation.x = result.center.0;
        transform.translation.y = result.center.1;
        velocity.0 = Vec2::new(result.velocity.0, result.velocity.1);
        grounded.on_ground = result.grounded;
        grounded.air_time = if result.grounded {
            0.0
        } else {
            grounded.air_time + dt
        };
    }
}
