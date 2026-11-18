//! Bevy components of enemies.

use bevy::prelude::*;

use super::ai::EnemyBehavior;
use super::boss::{BossPattern, BossPhase};
use crate::player::attributes::ResourcePool;
use crate::world::rooms::EnemySpawn;

#[derive(Component, Debug, Clone)]
pub struct Enemy {
    pub definition: String,
}

#[derive(Component, Debug, Clone, Copy, Deref, DerefMut)]
pub struct EnemyHealth(pub ResourcePool);

/// Marks a live boss entity (D21).
#[derive(Component, Debug, Clone)]
pub struct Boss {
    pub id: String,
}

/// Per-boss combat runtime (phase + attack cycle).
#[derive(Component, Debug, Clone)]
pub struct BossRuntime {
    pub phase: BossPhase,
    pub next_pattern: BossPattern,
    /// When set, apply a horizontal lunge during wind-up.
    pub lunge: bool,
    /// Facing used for the current lunge (`±1`).
    pub lunge_dir: f32,
    /// Brief pause after a phase transition (no attacks).
    pub phase_lock: f32,
    /// Last pattern whose hitbox or bolt actually spawned (D22).
    pub last_resolved: Option<BossPattern>,
    pub resolved_slash: bool,
    pub resolved_lunge: bool,
}

impl Default for BossRuntime {
    fn default() -> Self {
        Self {
            phase: BossPhase::One,
            next_pattern: BossPattern::FrontalSlash,
            lunge: false,
            lunge_dir: 1.0,
            phase_lock: 0.0,
            last_resolved: None,
            resolved_slash: false,
            resolved_lunge: false,
        }
    }
}

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
