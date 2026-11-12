//! Bevy components of enemies.

use bevy::prelude::*;

use super::ai::EnemyBehavior;
use crate::player::attributes::ResourcePool;
use crate::world::rooms::EnemySpawn;

#[derive(Component, Debug, Clone)]
pub struct Enemy {
    pub definition: String,
}

#[derive(Component, Debug, Clone, Copy, Deref, DerefMut)]
pub struct EnemyHealth(pub ResourcePool);

/// Runtime AI state.
#[derive(Component, Debug, Clone)]
pub struct EnemyBrain {
    pub behavior: EnemyBehavior,
    pub home_x: f32,
    pub patrol_half_width: f32,
    /// Current patrol direction (+1 / -1).
    pub patrol_direction: f32,
    /// Seconds until the next attack may start.
    pub attack_cooldown: f32,
    /// Remaining wind-up before the attack hitbox appears.
    pub windup: Option<f32>,
    /// Remaining hit-flash time.
    pub flash: f32,
    pub base_color: Color,
}

/// Where the enemy came from, so it can respawn there.
#[derive(Component, Debug, Clone)]
pub struct SpawnOrigin(pub EnemySpawn);

/// Defeated enemies waiting to reappear.
#[derive(Resource, Debug, Clone, Default)]
pub struct PendingRespawns(pub Vec<(EnemySpawn, f32)>);
