//! Character attributes: primary attributes (STR/CON/INT/LCK), equipment
//! modifiers, derived stats and resource pools (HP/MP/Hearts).
//!
//! Pure module: no Bevy types. Every formula lives in [`StatFormulas`] and
//! is documented in `docs/CHARACTER_ATTRIBUTES.md`. Derived stats are never
//! stored independently: they are always recomputed from base attributes,
//! level and modifiers via [`StatFormulas::derive`].

use serde::{Deserialize, Serialize};

use crate::magic::elements::{ElementalResistances, StatusResistances};

/// Primary attributes, raised by levelling and attribute points.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaseAttributes {
    /// STR — physical damage.
    pub strength: i32,
    /// CON — max HP, defence and HP regeneration.
    pub constitution: i32,
    /// INT — max MP, magic attack/defence and MP regeneration.
    pub intelligence: i32,
    /// LCK — critical hits, dodge, drops.
    pub luck: i32,
}

impl Default for BaseAttributes {
    fn default() -> Self {
        Self {
            strength: 6,
            constitution: 6,
            intelligence: 6,
            luck: 4,
        }
    }
}

/// A primary attribute, used to address [`BaseAttributes`] fields generically
/// (attribute point allocation, requirements).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PrimaryAttribute {
    Strength,
    Constitution,
    Intelligence,
    Luck,
}

impl PrimaryAttribute {
    pub const ALL: [PrimaryAttribute; 4] = [
        PrimaryAttribute::Strength,
        PrimaryAttribute::Constitution,
        PrimaryAttribute::Intelligence,
        PrimaryAttribute::Luck,
    ];

    pub fn short_name(self) -> &'static str {
        match self {
            PrimaryAttribute::Strength => "STR",
            PrimaryAttribute::Constitution => "CON",
            PrimaryAttribute::Intelligence => "INT",
            PrimaryAttribute::Luck => "LCK",
        }
    }
}

impl BaseAttributes {
    pub fn get(&self, attribute: PrimaryAttribute) -> i32 {
        match attribute {
            PrimaryAttribute::Strength => self.strength,
            PrimaryAttribute::Constitution => self.constitution,
            PrimaryAttribute::Intelligence => self.intelligence,
            PrimaryAttribute::Luck => self.luck,
        }
    }

    pub fn get_mut(&mut self, attribute: PrimaryAttribute) -> &mut i32 {
        match attribute {
            PrimaryAttribute::Strength => &mut self.strength,
            PrimaryAttribute::Constitution => &mut self.constitution,
            PrimaryAttribute::Intelligence => &mut self.intelligence,
            PrimaryAttribute::Luck => &mut self.luck,
        }
    }

    /// Base attributes with flat modifiers applied.
    pub fn with_modifiers(&self, modifiers: &AttributeModifiers) -> BaseAttributes {
        BaseAttributes {
            strength: self.strength + modifiers.strength,
            constitution: self.constitution + modifiers.constitution,
            intelligence: self.intelligence + modifiers.intelligence,
            luck: self.luck + modifiers.luck,
        }
    }
}

/// Flat bonuses coming from equipment, buffs or passives. Summed with
/// [`AttributeModifiers::combine`] before deriving stats.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AttributeModifiers {
    pub strength: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub luck: i32,
    pub max_hp: i32,
    pub max_mp: i32,
    pub attack: i32,
    pub defense: i32,
    pub magic_attack: i32,
    pub magic_defense: i32,
    pub hit: i32,
    pub critical_chance: f32,
    pub critical_damage: f32,
    pub attack_speed: f32,
    pub movement_speed: f32,
    pub hp_regen: f32,
    pub mp_regen: f32,
    pub dodge: f32,
    pub resistances: ElementalResistances,
    pub status_resistances: StatusResistances,
}

impl AttributeModifiers {
    pub fn combine(&mut self, other: &AttributeModifiers) {
        self.strength += other.strength;
        self.constitution += other.constitution;
        self.intelligence += other.intelligence;
        self.luck += other.luck;
        self.max_hp += other.max_hp;
        self.max_mp += other.max_mp;
        self.attack += other.attack;
        self.defense += other.defense;
        self.magic_attack += other.magic_attack;
        self.magic_defense += other.magic_defense;
        self.hit += other.hit;
        self.critical_chance += other.critical_chance;
        self.critical_damage += other.critical_damage;
        self.attack_speed += other.attack_speed;
        self.movement_speed += other.movement_speed;
        self.hp_regen += other.hp_regen;
        self.mp_regen += other.mp_regen;
        self.dodge += other.dodge;
        self.resistances.add(&other.resistances);
        self.status_resistances.add(&other.status_resistances);
    }

    pub fn sum<'a>(items: impl IntoIterator<Item = &'a AttributeModifiers>) -> AttributeModifiers {
        let mut total = AttributeModifiers::default();
        for item in items {
            total.combine(item);
        }
        total
    }
}

/// Stats computed from attributes, level and modifiers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DerivedStats {
    pub max_hp: i32,
    pub max_mp: i32,
    pub max_hearts: i32,
    /// ATK — physical attack power.
    pub attack: i32,
    /// DEF — physical defence.
    pub defense: i32,
    /// MATK — magic attack power.
    pub magic_attack: i32,
    /// MDEF — magic defence.
    pub magic_defense: i32,
    /// HIT — accuracy, 100 = baseline.
    pub hit: i32,
    /// Probability in `[0, max]`.
    pub critical_chance: f32,
    /// Damage multiplier applied on critical hits.
    pub critical_damage: f32,
    /// Multiplier on attack animation speed (1.0 = normal).
    pub attack_speed: f32,
    /// Multiplier on movement speed (1.0 = normal).
    pub movement_speed: f32,
    /// HP regenerated per second.
    pub hp_regen: f32,
    /// MP regenerated per second.
    pub mp_regen: f32,
    /// Probability of evading a hit.
    pub dodge: f32,
    pub resistances: ElementalResistances,
    pub status_resistances: StatusResistances,
    /// Effective primary attributes (base + modifiers), for display.
    pub effective: BaseAttributes,
}

/// Coefficients of every attribute formula. Centralised so balancing never
/// requires touching the formulas themselves.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StatFormulas {
    pub base_hp: i32,
    pub hp_per_constitution: i32,
    pub hp_per_level: i32,
    pub base_mp: i32,
    pub mp_per_intelligence: i32,
    pub mp_per_level: i32,
    pub max_hearts: i32,
    pub attack_per_strength: i32,
    pub defense_per_constitution: i32,
    pub magic_attack_per_intelligence: i32,
    pub magic_defense_per_intelligence: i32,
    pub base_hit: i32,
    pub hit_per_luck: i32,
    pub base_critical_chance: f32,
    pub critical_chance_per_luck: f32,
    pub max_critical_chance: f32,
    pub base_critical_damage: f32,
    pub critical_damage_per_luck: f32,
    pub max_critical_damage: f32,
    pub hp_regen_per_constitution: f32,
    pub base_mp_regen: f32,
    pub mp_regen_per_intelligence: f32,
    pub dodge_per_luck: f32,
    pub max_dodge: f32,
    pub min_speed_multiplier: f32,
}

impl Default for StatFormulas {
    fn default() -> Self {
        Self {
            base_hp: 50,
            hp_per_constitution: 10,
            hp_per_level: 5,
            base_mp: 20,
            mp_per_intelligence: 5,
            mp_per_level: 2,
            max_hearts: 99,
            attack_per_strength: 2,
            defense_per_constitution: 1,
            magic_attack_per_intelligence: 2,
            magic_defense_per_intelligence: 1,
            base_hit: 100,
            hit_per_luck: 1,
            base_critical_chance: 0.05,
            critical_chance_per_luck: 0.005,
            max_critical_chance: 0.5,
            base_critical_damage: 1.5,
            critical_damage_per_luck: 0.01,
            max_critical_damage: 3.0,
            hp_regen_per_constitution: 0.02,
            base_mp_regen: 0.5,
            mp_regen_per_intelligence: 0.05,
            dodge_per_luck: 0.002,
            max_dodge: 0.25,
            min_speed_multiplier: 0.25,
        }
    }
}

impl StatFormulas {
    /// Computes every derived stat. See `docs/CHARACTER_ATTRIBUTES.md`.
    pub fn derive(
        &self,
        base: &BaseAttributes,
        level: u32,
        modifiers: &AttributeModifiers,
    ) -> DerivedStats {
        let attrs = base.with_modifiers(modifiers);
        let level = i32::try_from(level).unwrap_or(i32::MAX);
        let luck = attrs.luck as f32;

        DerivedStats {
            max_hp: (self.base_hp
                + attrs.constitution * self.hp_per_constitution
                + level * self.hp_per_level
                + modifiers.max_hp)
                .max(1),
            max_mp: (self.base_mp
                + attrs.intelligence * self.mp_per_intelligence
                + level * self.mp_per_level
                + modifiers.max_mp)
                .max(0),
            max_hearts: self.max_hearts,
            attack: (attrs.strength * self.attack_per_strength + modifiers.attack).max(0),
            defense: (attrs.constitution * self.defense_per_constitution + modifiers.defense)
                .max(0),
            magic_attack: (attrs.intelligence * self.magic_attack_per_intelligence
                + modifiers.magic_attack)
                .max(0),
            magic_defense: (attrs.intelligence * self.magic_defense_per_intelligence
                + attrs.constitution / 2
                + modifiers.magic_defense)
                .max(0),
            hit: (self.base_hit + attrs.luck * self.hit_per_luck + modifiers.hit).max(0),
            critical_chance: (self.base_critical_chance
                + luck * self.critical_chance_per_luck
                + modifiers.critical_chance)
                .clamp(0.0, self.max_critical_chance),
            critical_damage: (self.base_critical_damage
                + luck * self.critical_damage_per_luck
                + modifiers.critical_damage)
                .clamp(1.0, self.max_critical_damage),
            attack_speed: (1.0 + modifiers.attack_speed).max(self.min_speed_multiplier),
            movement_speed: (1.0 + modifiers.movement_speed).max(self.min_speed_multiplier),
            hp_regen: (attrs.constitution as f32 * self.hp_regen_per_constitution
                + modifiers.hp_regen)
                .max(0.0),
            mp_regen: (self.base_mp_regen
                + attrs.intelligence as f32 * self.mp_regen_per_intelligence
                + modifiers.mp_regen)
                .max(0.0),
            dodge: (luck * self.dodge_per_luck + modifiers.dodge).clamp(0.0, self.max_dodge),
            resistances: modifiers.resistances.clone(),
            status_resistances: modifiers.status_resistances.clone(),
            effective: attrs,
        }
    }
}

/// A bounded resource such as HP, MP or Hearts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourcePool {
    current: i32,
    max: i32,
}

impl ResourcePool {
    pub fn full(max: i32) -> Self {
        let max = max.max(0);
        Self { current: max, max }
    }

    pub fn new(current: i32, max: i32) -> Self {
        let max = max.max(0);
        Self {
            current: current.clamp(0, max),
            max,
        }
    }

    pub fn current(&self) -> i32 {
        self.current
    }

    pub fn max(&self) -> i32 {
        self.max
    }

    pub fn is_empty(&self) -> bool {
        self.current == 0
    }

    pub fn ratio(&self) -> f32 {
        if self.max == 0 {
            0.0
        } else {
            self.current as f32 / self.max as f32
        }
    }

    pub fn has(&self, amount: i32) -> bool {
        self.current >= amount
    }

    /// Spends `amount` if available. Returns whether it was spent.
    pub fn try_spend(&mut self, amount: i32) -> bool {
        if amount < 0 || !self.has(amount) {
            return false;
        }
        self.current -= amount;
        true
    }

    /// Removes up to `amount`, saturating at zero. Returns the amount removed.
    pub fn deplete(&mut self, amount: i32) -> i32 {
        let removed = amount.clamp(0, self.current);
        self.current -= removed;
        removed
    }

    /// Restores up to `amount`, saturating at max. Returns the amount restored.
    pub fn restore(&mut self, amount: i32) -> i32 {
        let restored = amount.clamp(0, self.max - self.current);
        self.current += restored;
        restored
    }

    pub fn refill(&mut self) {
        self.current = self.max;
    }

    /// Changes the maximum, keeping the current value inside the new bounds.
    pub fn set_max(&mut self, max: i32) {
        self.max = max.max(0);
        self.current = self.current.min(self.max);
    }
}

/// Accumulates fractional regeneration so that small per-frame amounts are
/// not lost to integer rounding.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct RegenAccumulator {
    fraction: f32,
}

impl RegenAccumulator {
    /// Adds `rate * dt` and returns the whole points ready to be restored.
    pub fn tick(&mut self, rate_per_second: f32, dt: f32) -> i32 {
        self.fraction += (rate_per_second * dt).max(0.0);
        let whole = self.fraction.floor();
        self.fraction -= whole;
        whole as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pool_spend_and_restore() {
        let mut pool = ResourcePool::full(10);
        assert!(pool.try_spend(4));
        assert!(!pool.try_spend(7));
        assert_eq!(pool.current(), 6);
        assert_eq!(pool.restore(10), 4);
        assert_eq!(pool.deplete(50), 10);
        assert!(pool.is_empty());
    }

    #[test]
    fn regen_accumulates_fractions() {
        let mut acc = RegenAccumulator::default();
        let total: i32 = (0..10).map(|_| acc.tick(1.0, 0.25)).sum();
        assert_eq!(total, 2);
    }
}
