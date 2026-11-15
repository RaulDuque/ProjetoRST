//! Weapon classes and the strike profile of the basic attack (V2.0).
//!
//! Pure module: no Bevy types. Each weapon category (`Sword`, `Dagger`,
//! `Axe`, `Spear`, `HeavyWeapon`) has one [`WeaponClass`] in data
//! (`assets/data/items/weapon_classes.ron`): the shape and rhythm of its
//! basic attack. An item adds its own [`WeaponStats`] (element, reach) and
//! its damage through `modifiers.attack` (ATK), as before V2.0.
//!
//! [`StrikeProfile::resolve`] turns class + item + attack speed into the
//! numbers the combat system uses. Without a weapon the strike is
//! [`StrikeProfile::unarmed`], the pre-V2.0 constants. The `Sword` class
//! has exactly those values too, so the starting sword is unchanged.
//! Balance: docs/EQUIPMENT_EXPANSION_V2_0.md §21.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::items::ItemCategory;
use crate::core::constants::{BASIC_ATTACK_HITBOX, BASIC_ATTACK_TIME, HITBOX_ACTIVE_FRACTION};
use crate::magic::elements::Element;

/// Distance between the body centre and the near edge of the pre-V2.0 strike.
pub const UNARMED_GAP: f32 = 8.0;
/// Height of the pre-V2.0 strike centre above the body centre.
pub const UNARMED_RAISE: f32 = 4.0;
/// Longest reach a weapon may have (a little more than a room tile).
pub const MAX_REACH: f32 = 160.0;
/// Longest attack or recovery time a weapon may have, in seconds.
pub const MAX_STRIKE_TIME: f32 = 1.5;

/// Shape and rhythm of the basic attack of one weapon category.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WeaponClass {
    pub category: ItemCategory,
    /// Shown in the inventory ("Sword", "Spear", ...).
    pub name: String,
    /// Width of the strike, in front of the body (world units).
    pub reach: f32,
    /// Height of the strike.
    pub height: f32,
    /// Empty space between the body centre and the strike: a spear does not
    /// cover what is right against the body.
    pub gap: f32,
    /// Height of the strike centre above the body centre.
    pub raise: f32,
    /// Seconds of the swing at attack speed 1. The strike is active for
    /// the first `HITBOX_ACTIVE_FRACTION` of it.
    pub attack_time: f32,
    /// Extra seconds the character stays committed after the swing.
    pub recovery: f32,
    /// Multiplier on ATK for the basic attack.
    pub power: f32,
    /// V2.1: needs both hands (no shield in the off-hand at the same time).
    #[serde(default)]
    pub two_handed: bool,
}

/// Per-item weapon data, on top of its class.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct WeaponStats {
    /// Element of the basic attack (`Physical` for plain steel).
    pub element: Element,
    /// Added to the class reach (may be negative, the result must stay > 0).
    pub reach_bonus: f32,
}

impl Default for WeaponStats {
    fn default() -> Self {
        Self {
            element: Element::Physical,
            reach_bonus: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum WeaponError {
    #[error("weapon class `{0:?}` is not a weapon category")]
    NotAWeapon(ItemCategory),
    #[error("weapon class `{category:?}`: {field} = {value} is out of range")]
    OutOfRange {
        category: ItemCategory,
        field: &'static str,
        value: f32,
    },
    #[error("weapon class `{0:?}` is defined twice")]
    Duplicate(ItemCategory),
    #[error("item `{item}`: weapon data on a non-weapon category")]
    StatsOnNonWeapon { item: String },
    #[error("item `{item}`: reach {reach} is out of range")]
    ItemReach { item: String, reach: f32 },
}

fn in_range(value: f32, min: f32, max: f32, inclusive_min: bool) -> bool {
    value.is_finite()
        && value <= max
        && if inclusive_min {
            value >= min
        } else {
            value > min
        }
}

impl WeaponClass {
    /// Rejects unknown categories, negative or absurd sizes and times.
    pub fn validate(&self) -> Result<(), WeaponError> {
        if !self.category.is_weapon() {
            return Err(WeaponError::NotAWeapon(self.category));
        }
        let checks: [(&'static str, f32, f32, f32, bool); 7] = [
            ("reach", self.reach, 0.0, MAX_REACH, false),
            ("height", self.height, 0.0, MAX_REACH, false),
            ("gap", self.gap, 0.0, MAX_REACH, true),
            ("raise", self.raise, -MAX_REACH, MAX_REACH, true),
            ("attack_time", self.attack_time, 0.0, MAX_STRIKE_TIME, false),
            ("recovery", self.recovery, 0.0, MAX_STRIKE_TIME, true),
            ("power", self.power, 0.0, 10.0, false),
        ];
        for (field, value, min, max, inclusive) in checks {
            if !in_range(value, min, max, inclusive) {
                return Err(WeaponError::OutOfRange {
                    category: self.category,
                    field,
                    value,
                });
            }
        }
        Ok(())
    }
}

/// Everything the basic attack needs, already resolved.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StrikeProfile {
    /// `None` when unarmed.
    pub category: Option<ItemCategory>,
    /// Hitbox size (width, height).
    pub size: (f32, f32),
    /// Hitbox centre relative to the body centre, facing right.
    pub offset: (f32, f32),
    /// Seconds the character is committed (swing + recovery).
    pub action_time: f32,
    /// Seconds the hitbox exists.
    pub active_time: f32,
    /// Multiplier on ATK.
    pub multiplier: f32,
    pub element: Element,
}

impl StrikeProfile {
    /// The pre-V2.0 basic attack (constants), used without a weapon.
    pub fn unarmed(attack_speed: f32) -> Self {
        let speed = attack_speed.max(f32::EPSILON);
        let (w, h) = BASIC_ATTACK_HITBOX;
        let swing = BASIC_ATTACK_TIME / speed;
        Self {
            category: None,
            size: (w, h),
            offset: (w / 2.0 + UNARMED_GAP, UNARMED_RAISE),
            action_time: swing,
            active_time: swing * HITBOX_ACTIVE_FRACTION,
            multiplier: 1.0,
            element: Element::Physical,
        }
    }

    /// Class + item at a given attack speed (`DerivedStats::attack_speed`,
    /// already floored by the formulas). Attack speed scales the swing and
    /// the recovery alike.
    pub fn resolve(class: &WeaponClass, stats: &WeaponStats, attack_speed: f32) -> Self {
        let speed = attack_speed.max(f32::EPSILON);
        let reach = (class.reach + stats.reach_bonus).max(1.0);
        let swing = class.attack_time / speed;
        Self {
            category: Some(class.category),
            size: (reach, class.height),
            offset: (reach / 2.0 + class.gap, class.raise),
            action_time: swing + class.recovery / speed,
            active_time: swing * HITBOX_ACTIVE_FRACTION,
            multiplier: class.power,
            element: stats.element,
        }
    }

    /// Farthest point the strike reaches from the body centre.
    pub fn far_edge(&self) -> f32 {
        self.offset.0 + self.size.0 / 2.0
    }

    /// Nearest point the strike covers from the body centre.
    pub fn near_edge(&self) -> f32 {
        self.offset.0 - self.size.0 / 2.0
    }

    /// Basic attacks per second when attacking without pause.
    pub fn swings_per_second(&self) -> f32 {
        if self.action_time <= 0.0 {
            0.0
        } else {
            1.0 / self.action_time
        }
    }

    /// Raw power of one strike before defence and variance:
    /// `ATK * multiplier` (the formula of `combat::damage`).
    pub fn raw_power(&self, attack: i32) -> f32 {
        attack.max(0) as f32 * self.multiplier
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sword() -> WeaponClass {
        WeaponClass {
            category: ItemCategory::Sword,
            name: "Sword".into(),
            reach: BASIC_ATTACK_HITBOX.0,
            height: BASIC_ATTACK_HITBOX.1,
            gap: UNARMED_GAP,
            raise: UNARMED_RAISE,
            attack_time: BASIC_ATTACK_TIME,
            recovery: 0.0,
            power: 1.0,
            two_handed: false,
        }
    }

    #[test]
    fn a_reference_sword_strikes_like_the_pre_v2_attack() {
        let sword = StrikeProfile::resolve(&sword(), &WeaponStats::default(), 1.0);
        let unarmed = StrikeProfile::unarmed(1.0);
        assert_eq!(sword.size, unarmed.size);
        assert_eq!(sword.offset, unarmed.offset);
        assert_eq!(sword.action_time, unarmed.action_time);
        assert_eq!(sword.active_time, unarmed.active_time);
        assert_eq!(sword.multiplier, unarmed.multiplier);
    }

    #[test]
    fn invalid_classes_are_rejected() {
        let mut bad = sword();
        bad.reach = -1.0;
        assert!(bad.validate().is_err());
        let mut bad = sword();
        bad.attack_time = f32::NAN;
        assert!(bad.validate().is_err());
        let mut bad = sword();
        bad.category = ItemCategory::Ring;
        assert_eq!(
            bad.validate(),
            Err(WeaponError::NotAWeapon(ItemCategory::Ring))
        );
        assert!(sword().validate().is_ok());
    }

    #[test]
    fn attack_speed_scales_swing_and_recovery() {
        let mut axe = sword();
        axe.recovery = 0.2;
        let slow = StrikeProfile::resolve(&axe, &WeaponStats::default(), 1.0);
        let fast = StrikeProfile::resolve(&axe, &WeaponStats::default(), 2.0);
        assert!((fast.action_time * 2.0 - slow.action_time).abs() < 1e-6);
    }
}
