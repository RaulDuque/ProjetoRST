//! Enemy archetypes loaded from `assets/data/enemies/*.ron`.
//! Pure module: no Bevy types.

use serde::{Deserialize, Serialize};

use super::ai::AiProfile;
use crate::magic::elements::{ElementalResistances, StatusResistances};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DropEntry {
    pub item: String,
    /// Base chance in `[0, 1]`, increased by the player's LCK.
    pub chance: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnemyDefinition {
    pub id: String,
    pub name: String,
    pub max_hp: i32,
    #[serde(default)]
    pub max_mp: i32,
    pub attack: i32,
    pub defense: i32,
    #[serde(default)]
    pub magic_defense: i32,
    #[serde(default = "default_hit")]
    pub hit: i32,
    #[serde(default)]
    pub dodge: f32,
    /// Resistances below zero are weaknesses.
    #[serde(default)]
    pub resistances: ElementalResistances,
    #[serde(default)]
    pub status_resistances: StatusResistances,
    pub move_speed: f32,
    /// Body size in world units.
    pub size: (f32, f32),
    /// Seconds between attacks.
    pub attack_cooldown: f32,
    /// Melee hitbox (width, height) spawned in front of the enemy.
    pub attack_hitbox: (f32, f32),
    /// Damage multiplier of touching the enemy (0 disables contact damage).
    #[serde(default)]
    pub contact_damage_multiplier: f32,
    pub experience: u64,
    #[serde(default)]
    pub drops: Vec<DropEntry>,
    #[serde(default)]
    pub ai: AiProfile,
    /// Placeholder body colour (sRGB, 0..1) until sprites exist.
    pub color: (f32, f32, f32),
    /// Seconds before a defeated enemy reappears (0 = never).
    #[serde(default)]
    pub respawn_time: f32,
}

fn default_hit() -> i32 {
    100
}
