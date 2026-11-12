//! Damage calculation.
//!
//! Pure module: no Bevy types. Randomness is injected through
//! [`DamageRolls`] so every outcome is reproducible in tests.
//! Formula (documented in `docs/COMBAT_SYSTEM.md`):
//!
//! ```text
//! hit_chance = clamp(attacker.hit / 100 - defender.dodge, MIN_HIT_CHANCE, 1)
//! raw        = power * multiplier + base_damage
//! mitigated  = raw * DEFENSE_SCALE / (DEFENSE_SCALE + defense)
//! elemental  = mitigated * (1 - resistance(element))
//! critical   = elemental * critical_damage   (if crit roll < critical_chance)
//! final      = round(critical * variance), at least MIN_DAMAGE (0 if immune)
//! ```

use serde::{Deserialize, Serialize};

use crate::magic::elements::{Element, ElementalResistances};

/// Defence is divided by `DEFENSE_SCALE + defense`: 100 defence halves damage.
pub const DEFENSE_SCALE: f32 = 100.0;
/// Hits always have at least this chance to connect.
pub const MIN_HIT_CHANCE: f32 = 0.5;
/// Lowest variance multiplier (rolls map to `[MIN, MIN + SPREAD)`).
pub const VARIANCE_MIN: f32 = 0.9;
pub const VARIANCE_SPREAD: f32 = 0.2;
/// A connecting, non-immune hit always deals at least this much.
pub const MIN_DAMAGE: i32 = 1;

/// Whether damage is mitigated by physical or magical defence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageKind {
    Physical,
    Magical,
}

/// Offensive side of a hit.
#[derive(Debug, Clone, PartialEq)]
pub struct AttackProfile {
    /// ATK for physical damage or MATK for magical damage.
    pub power: i32,
    pub hit: i32,
    pub critical_chance: f32,
    pub critical_damage: f32,
}

/// Defensive side of a hit.
#[derive(Debug, Clone, PartialEq)]
pub struct DefenseProfile {
    pub defense: i32,
    pub magic_defense: i32,
    pub dodge: f32,
    pub resistances: ElementalResistances,
}

/// What the hit itself contributes (from the skill, spell or basic attack).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HitSpec {
    pub kind: DamageKind,
    pub element: Element,
    pub base_damage: i32,
    pub multiplier: f32,
}

/// Uniform random values in `[0, 1)`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DamageRolls {
    pub hit: f32,
    pub critical: f32,
    pub variance: f32,
}

impl DamageRolls {
    /// Rolls that always hit, never crit and use the average variance.
    pub const AVERAGE: DamageRolls = DamageRolls {
        hit: 0.0,
        critical: 1.0,
        variance: 0.5,
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DamageOutcome {
    Miss,
    Hit {
        amount: i32,
        critical: bool,
        element: Element,
    },
}

impl DamageOutcome {
    pub fn amount(&self) -> i32 {
        match self {
            DamageOutcome::Miss => 0,
            DamageOutcome::Hit { amount, .. } => *amount,
        }
    }
}

pub fn hit_chance(attacker: &AttackProfile, defender: &DefenseProfile) -> f32 {
    (attacker.hit as f32 / 100.0 - defender.dodge).clamp(MIN_HIT_CHANCE, 1.0)
}

/// Resolves one hit. See the module docs for the formula.
pub fn calculate_damage(
    attacker: &AttackProfile,
    defender: &DefenseProfile,
    hit: &HitSpec,
    rolls: DamageRolls,
) -> DamageOutcome {
    if rolls.hit >= hit_chance(attacker, defender) {
        return DamageOutcome::Miss;
    }

    let raw =
        attacker.power.max(0) as f32 * hit.multiplier.max(0.0) + hit.base_damage.max(0) as f32;
    let defense = match hit.kind {
        DamageKind::Physical => defender.defense,
        DamageKind::Magical => defender.magic_defense,
    }
    .max(0) as f32;
    let mitigated = raw * DEFENSE_SCALE / (DEFENSE_SCALE + defense);

    let element_multiplier = defender.resistances.damage_multiplier(hit.element);
    let elemental = mitigated * element_multiplier;

    let critical = rolls.critical < attacker.critical_chance;
    let after_crit = if critical {
        elemental * attacker.critical_damage.max(1.0)
    } else {
        elemental
    };

    let variance = VARIANCE_MIN + VARIANCE_SPREAD * rolls.variance.clamp(0.0, 1.0);
    let rounded = (after_crit * variance).round() as i32;
    let amount = if element_multiplier <= 0.0 {
        0
    } else {
        rounded.max(MIN_DAMAGE)
    };

    DamageOutcome::Hit {
        amount,
        critical,
        element: hit.element,
    }
}
