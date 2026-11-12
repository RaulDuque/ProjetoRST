//! Messages exchanged between feature plugins.
//!
//! Plugins communicate through these buffered messages instead of reaching
//! into each other's components, which keeps modules decoupled.

use bevy::prelude::*;

use crate::combat::damage::DamageOutcome;
use crate::enemies::definitions::DropEntry;
use crate::skills::execution::CommandOutcome;

/// A button press was interpreted by the command system (for UI feedback).
#[derive(Message, Debug, Clone)]
pub struct CommandResolved {
    pub outcome: CommandOutcome,
}

/// Request: the recogniser matched a skill for `caster` (validated with the
/// state of the previous frame). Only `begin_skill` reads it; it re-validates
/// with the current state and either confirms ([`SkillStarted`]) or rejects.
#[derive(Message, Debug, Clone)]
pub struct SkillActivated {
    pub caster: Entity,
    pub skill_id: String,
}

/// Confirmed activation: costs paid, cooldown started and caster locked.
/// Effect executors (melee techniques, spells) react only to this message.
#[derive(Message, Debug, Clone)]
pub struct SkillStarted {
    pub caster: Entity,
    pub skill_id: String,
}

/// The attack button was pressed without forming a command.
#[derive(Message, Debug, Clone, Copy)]
pub struct BasicAttackRequested {
    pub attacker: Entity,
}

/// A hitbox connected with a hurtbox; damage already computed.
#[derive(Message, Debug, Clone, Copy)]
pub struct DamageDealt {
    pub attacker: Entity,
    pub target: Entity,
    pub outcome: DamageOutcome,
    /// Horizontal direction of the hit (+1 right, -1 left), for knockback.
    pub direction: f32,
}

/// An enemy died; carries the reward.
#[derive(Message, Debug, Clone)]
pub struct EnemyDefeated {
    pub enemy_id: String,
    pub experience: u64,
    /// Drop table; rolled by the player's side with its LCK.
    pub drops: Vec<DropEntry>,
}

/// Short on-screen message (save done, level up, errors...).
#[derive(Message, Debug, Clone)]
pub struct Notification(pub String);
