//! Bevy glue for platform physics shared by the player and enemies.
//!
//! V1.9: two optional components let a transformation change how a body
//! moves without touching anyone else's: [`Phasing`] (the barrier kinds it
//! passes) and [`GravityProfile`] (its gravity). A body without them, every
//! human player and enemy, runs exactly the code it ran before.

use bevy::prelude::*;

use super::collision::{Aabb, BarrierKind, Solid, move_and_collide, move_and_collide_phased};
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

/// Collision profile of a transformed body (V1.9): it passes the barriers
/// of these kinds. Inserted and removed only by the transformation module.
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct Phasing {
    pub passes: Vec<BarrierKind>,
}

/// Gravity of a transformed body (V1.9): `scale` multiplies [`GRAVITY`] and
/// the fall speed is capped at `max_fall_speed` (negative). Without it the
/// global constants apply unchanged.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct GravityProfile {
    pub scale: f32,
    pub max_fall_speed: f32,
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
    /// Barriers count: this is the question "would a human fit here?".
    pub fn blocks(&self, aabb: &Aabb) -> bool {
        self.solids
            .iter()
            .any(|s| !s.one_way && s.aabb.intersects(aabb))
    }

    /// Whether a box overlaps a barrier (V1.9).
    pub fn overlaps_barrier(&self, aabb: &Aabb) -> bool {
        self.solids
            .iter()
            .any(|s| s.barrier.is_some() && s.aabb.intersects(aabb))
    }
}

pub fn apply_gravity(
    time: Res<Time>,
    mut bodies: Query<(&mut Velocity, Option<&GravityProfile>), With<AffectedByGravity>>,
) {
    let dt = frame_delta(&time);
    for (mut velocity, profile) in &mut bodies {
        velocity.y = match profile {
            None => (velocity.y + GRAVITY * dt).max(MAX_FALL_SPEED),
            Some(p) => (velocity.y + GRAVITY * p.scale * dt).max(p.max_fall_speed),
        };
    }
}

pub fn move_bodies(
    time: Res<Time>,
    geometry: Res<RoomGeometry>,
    mut bodies: Query<(
        &Body,
        &mut Transform,
        &mut Velocity,
        &mut Grounded,
        Option<&Phasing>,
    )>,
) {
    let dt = frame_delta(&time);
    for (body, mut transform, mut velocity, mut grounded, phasing) in &mut bodies {
        let aabb = body.aabb_at(transform.translation.truncate());
        let motion = (velocity.x, velocity.y);
        let result = match phasing {
            None => move_and_collide(aabb, motion, dt, &geometry.solids),
            Some(p) => move_and_collide_phased(aabb, motion, dt, &geometry.solids, &p.passes),
        };
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
