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
//!
//! V2.4: optional fields (`startup`, `active_fraction`, `cancel_after`,
//! `air_control`, `turn_window`, `rehit_interval`, `knockback`); omitted,
//! they reproduce the pre-V2.4 strike (docs/COMBAT_FEEL_V2_4.md).

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::items::ItemCategory;
use crate::combat::reaction::KnockbackIntensity;
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
/// Shortest interval between two hits of one strike on the same target
/// (V2.4): keeps a misconfigured weapon from hitting every frame.
pub const MIN_REHIT_INTERVAL: f32 = 0.1;
/// Rounding slack of the timeline checks (seconds or fractions).
const TIMELINE_EPSILON: f32 = 1e-5;

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
    /// `active_fraction` of it, starting after `startup`.
    pub attack_time: f32,
    /// Extra seconds the character stays committed after the swing.
    pub recovery: f32,
    /// Multiplier on ATK for the basic attack.
    pub power: f32,
    /// V2.1: needs both hands (no shield in the off-hand at the same time).
    #[serde(default)]
    pub two_handed: bool,
    /// V2.4: anticipation, in seconds at attack speed 1, inside
    /// `attack_time` and before the strike can hit. 0 = hits from the
    /// first frame (as before V2.4).
    #[serde(default)]
    pub startup: f32,
    /// V2.4: share of `attack_time` the strike stays active, counted from
    /// the end of `startup`.
    #[serde(default = "default_active_fraction")]
    pub active_fraction: f32,
    /// V2.4: share of the whole action (swing + recovery) from which jump,
    /// dodge, guard and a new basic attack may cancel it. `None` = only
    /// techniques cancel it (as before V2.4).
    #[serde(default)]
    pub cancel_after: Option<f32>,
    /// V2.4: share (0..=1) of the normal horizontal control kept in the air
    /// during the basic attack. 0 = momentum kept without control (as
    /// before V2.4); 1 = steers like a normal jump.
    #[serde(default)]
    pub air_control: f32,
    /// V2.4: seconds at attack speed 1, from the start of the attack, during
    /// which the character may still turn around. Hitboxes already armed
    /// keep their side. 0 = no turning (as before V2.4).
    #[serde(default)]
    pub turn_window: f32,
    /// V2.4: seconds before the strike may hit the same target again.
    /// `None` = each target once per strike (as before V2.4).
    #[serde(default)]
    pub rehit_interval: Option<f32>,
    /// V2.4: how hard the basic attack pushes what it hits. `Medium` = the
    /// pre-V2.4 knockback.
    #[serde(default)]
    pub knockback: KnockbackIntensity,
}

fn default_active_fraction() -> f32 {
    HITBOX_ACTIVE_FRACTION
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
        self.validate_timeline()
    }

    /// V2.4: the active window fits inside the swing, and the cancel point
    /// comes after it (never two strikes of the same weapon alive at once).
    fn validate_timeline(&self) -> Result<(), WeaponError> {
        let out_of_range = |field, value| WeaponError::OutOfRange {
            category: self.category,
            field,
            value,
        };
        if !in_range(self.active_fraction, 0.0, 1.0, false) {
            return Err(out_of_range("active_fraction", self.active_fraction));
        }
        let active_end = self.startup + self.attack_time * self.active_fraction;
        if !in_range(self.startup, 0.0, MAX_STRIKE_TIME, true)
            || active_end > self.attack_time + TIMELINE_EPSILON
        {
            return Err(out_of_range("startup", self.startup));
        }
        if let Some(cancel) = self.cancel_after {
            let earliest = active_end / (self.attack_time + self.recovery);
            if !in_range(cancel, 0.0, 1.0, false) || cancel + TIMELINE_EPSILON < earliest {
                return Err(out_of_range("cancel_after", cancel));
            }
        }
        if !in_range(self.air_control, 0.0, 1.0, true) {
            return Err(out_of_range("air_control", self.air_control));
        }
        if !in_range(self.turn_window, 0.0, self.attack_time, true) {
            return Err(out_of_range("turn_window", self.turn_window));
        }
        if let Some(interval) = self.rehit_interval
            && !in_range(interval, MIN_REHIT_INTERVAL, MAX_STRIKE_TIME, true)
        {
            return Err(out_of_range("rehit_interval", interval));
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
    /// Seconds the hitbox can hit.
    pub active_time: f32,
    /// V2.4: seconds before the hitbox can hit (0 = from the first frame).
    pub startup: f32,
    /// V2.4: share of `action_time` from which the strike may be cancelled
    /// by jump, dodge, guard or a new attack (`None` = never).
    pub cancel_after: Option<f32>,
    /// V2.4: share of horizontal control kept in the air (0 = none).
    pub air_control: f32,
    /// V2.4: seconds from the start during which the character may turn.
    pub turn_window: f32,
    /// V2.4: seconds before the same target can be hit again (`None` =
    /// once). Not scaled by attack speed.
    pub rehit_interval: Option<f32>,
    /// V2.4: knockback intensity of the strike's hits.
    pub knockback: KnockbackIntensity,
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
            startup: 0.0,
            cancel_after: None,
            air_control: 0.0,
            turn_window: 0.0,
            rehit_interval: None,
            knockback: KnockbackIntensity::Medium,
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
            active_time: swing * class.active_fraction,
            startup: class.startup / speed,
            cancel_after: class.cancel_after,
            air_control: class.air_control,
            turn_window: class.turn_window / speed,
            rehit_interval: class.rehit_interval,
            knockback: class.knockback,
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
            startup: 0.0,
            active_fraction: HITBOX_ACTIVE_FRACTION,
            cancel_after: None,
            air_control: 0.0,
            turn_window: 0.0,
            rehit_interval: None,
            knockback: KnockbackIntensity::Medium,
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

    #[test]
    fn omitted_timeline_fields_reproduce_the_pre_v2_4_strike() {
        let text = r#"(category: Sword, name: "Sword", reach: 46.0, height: 26.0,
            gap: 8.0, raise: 4.0, attack_time: 0.28, recovery: 0.0, power: 1.0)"#;
        let class: WeaponClass =
            ron::from_str(text).unwrap_or_else(|e| panic!("class did not parse: {e}"));
        assert_eq!(class, sword());
        let profile = StrikeProfile::resolve(&class, &WeaponStats::default(), 1.0);
        assert_eq!(profile.startup, 0.0);
        assert_eq!(profile.cancel_after, None);
        assert_eq!(
            profile.active_time,
            BASIC_ATTACK_TIME * HITBOX_ACTIVE_FRACTION
        );
        let mut unarmed = StrikeProfile::unarmed(1.0);
        unarmed.category = Some(ItemCategory::Sword);
        assert_eq!(profile, unarmed);
    }

    #[test]
    fn attack_speed_scales_startup_but_not_the_cancel_share() {
        let mut class = sword();
        class.startup = 0.06;
        class.cancel_after = Some(0.9);
        let slow = StrikeProfile::resolve(&class, &WeaponStats::default(), 1.0);
        let fast = StrikeProfile::resolve(&class, &WeaponStats::default(), 2.0);
        assert!((fast.startup * 2.0 - slow.startup).abs() < 1e-6);
        assert_eq!(fast.cancel_after, slow.cancel_after);
    }

    #[test]
    fn timelines_that_do_not_fit_are_rejected() {
        let field_of = |class: WeaponClass| match class.validate() {
            Err(WeaponError::OutOfRange { field, .. }) => field,
            other => panic!("expected OutOfRange, got {other:?}"),
        };
        let mut bad = sword();
        bad.startup = -0.01;
        assert_eq!(field_of(bad), "startup");
        // 0.12 + 0.28 * 0.6 = 0.288 > 0.28: the window leaves the swing.
        let mut bad = sword();
        bad.startup = 0.12;
        assert_eq!(field_of(bad), "startup");
        let mut bad = sword();
        bad.active_fraction = 0.0;
        assert_eq!(field_of(bad), "active_fraction");
        let mut bad = sword();
        bad.active_fraction = 1.2;
        assert_eq!(field_of(bad), "active_fraction");
        // The cancel point must come after the active window (0.6 here).
        let mut bad = sword();
        bad.cancel_after = Some(0.5);
        assert_eq!(field_of(bad), "cancel_after");
        let mut bad = sword();
        bad.cancel_after = Some(1.5);
        assert_eq!(field_of(bad), "cancel_after");

        let mut good = sword();
        good.startup = 0.1;
        good.cancel_after = Some(0.97);
        assert_eq!(good.validate(), Ok(()));
        let mut edge = sword();
        edge.cancel_after = Some(HITBOX_ACTIVE_FRACTION);
        assert_eq!(edge.validate(), Ok(()));
    }

    #[test]
    fn control_turn_and_rehit_default_to_the_pre_v2_4_strike() {
        let profile = StrikeProfile::resolve(&sword(), &WeaponStats::default(), 1.0);
        assert_eq!(profile.knockback, KnockbackIntensity::Medium);
        assert_eq!(profile.air_control, 0.0);
        assert_eq!(profile.turn_window, 0.0);
        assert_eq!(profile.rehit_interval, None);
        let unarmed = StrikeProfile::unarmed(1.0);
        assert_eq!(
            (
                unarmed.air_control,
                unarmed.turn_window,
                unarmed.rehit_interval
            ),
            (0.0, 0.0, None)
        );
    }

    #[test]
    fn control_turn_and_rehit_are_validated_and_scaled() {
        let field_of = |class: WeaponClass| match class.validate() {
            Err(WeaponError::OutOfRange { field, .. }) => field,
            other => panic!("expected OutOfRange, got {other:?}"),
        };
        for bad_control in [-0.1, 1.1, f32::NAN] {
            let mut bad = sword();
            bad.air_control = bad_control;
            assert_eq!(field_of(bad), "air_control");
        }
        for bad_window in [-0.01, BASIC_ATTACK_TIME + 0.01] {
            let mut bad = sword();
            bad.turn_window = bad_window;
            assert_eq!(field_of(bad), "turn_window");
        }
        for bad_interval in [0.0, MIN_REHIT_INTERVAL - 0.01, MAX_STRIKE_TIME + 0.1] {
            let mut bad = sword();
            bad.rehit_interval = Some(bad_interval);
            assert_eq!(field_of(bad), "rehit_interval");
        }

        let mut good = sword();
        good.air_control = 1.0;
        good.turn_window = 0.1;
        good.rehit_interval = Some(MIN_REHIT_INTERVAL);
        assert_eq!(good.validate(), Ok(()));
        let slow = StrikeProfile::resolve(&good, &WeaponStats::default(), 1.0);
        let fast = StrikeProfile::resolve(&good, &WeaponStats::default(), 2.0);
        assert!((fast.turn_window * 2.0 - slow.turn_window).abs() < 1e-6);
        assert_eq!(fast.air_control, slow.air_control);
        assert_eq!(fast.rehit_interval, slow.rehit_interval, "not scaled");
    }
}
