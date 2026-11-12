//! Elements, negative status effects and the resistance tables for both.
//!
//! Pure module: no Bevy types. A resistance is a fraction in
//! `[MIN_RESISTANCE, MAX_RESISTANCE]`:
//! * `0.0`  → neutral, full damage;
//! * `0.5`  → resists, half damage;
//! * `1.0`  → immune;
//! * `-0.5` → weak, 150% damage.

use serde::{Deserialize, Serialize};

/// Lowest allowed resistance (a weakness that doubles damage).
pub const MIN_RESISTANCE: f32 = -1.0;
/// Highest allowed resistance (immunity).
pub const MAX_RESISTANCE: f32 = 1.0;

/// Damage element. `Physical` covers un-enchanted weapon damage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Element {
    Physical,
    Fire,
    Ice,
    Lightning,
    Water,
    Wind,
    Earth,
    /// Holy / sacred damage.
    Light,
    /// Shadow / unholy damage.
    Dark,
    Arcane,
}

impl Element {
    pub const ALL: [Element; 10] = [
        Element::Physical,
        Element::Fire,
        Element::Ice,
        Element::Lightning,
        Element::Water,
        Element::Wind,
        Element::Earth,
        Element::Light,
        Element::Dark,
        Element::Arcane,
    ];

    pub fn display_name(self) -> &'static str {
        match self {
            Element::Physical => "Physical",
            Element::Fire => "Fire",
            Element::Ice => "Ice",
            Element::Lightning => "Lightning",
            Element::Water => "Water",
            Element::Wind => "Wind",
            Element::Earth => "Earth",
            Element::Light => "Light",
            Element::Dark => "Dark",
            Element::Arcane => "Arcane",
        }
    }
}

/// Negative status effects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum StatusEffect {
    Poison,
    Curse,
    Petrify,
    Burn,
    Freeze,
    Stun,
}

impl StatusEffect {
    pub const ALL: [StatusEffect; 6] = [
        StatusEffect::Poison,
        StatusEffect::Curse,
        StatusEffect::Petrify,
        StatusEffect::Burn,
        StatusEffect::Freeze,
        StatusEffect::Stun,
    ];

    pub fn display_name(self) -> &'static str {
        match self {
            StatusEffect::Poison => "Poison",
            StatusEffect::Curse => "Curse",
            StatusEffect::Petrify => "Petrify",
            StatusEffect::Burn => "Burn",
            StatusEffect::Freeze => "Freeze",
            StatusEffect::Stun => "Stun",
        }
    }
}

fn clamp_resistance(value: f32) -> f32 {
    value.clamp(MIN_RESISTANCE, MAX_RESISTANCE)
}

/// Elemental resistances. Missing elements are neutral (`0.0`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ElementalResistances {
    #[serde(default)]
    values: Vec<(Element, f32)>,
}

impl ElementalResistances {
    pub fn get(&self, element: Element) -> f32 {
        self.values
            .iter()
            .find(|(e, _)| *e == element)
            .map_or(0.0, |(_, v)| clamp_resistance(*v))
    }

    /// Sets the raw (unclamped) value; reads are clamped.
    pub fn set(&mut self, element: Element, value: f32) {
        match self.values.iter_mut().find(|(e, _)| *e == element) {
            Some(entry) => entry.1 = value,
            None => self.values.push((element, value)),
        }
    }

    pub fn with(mut self, element: Element, value: f32) -> Self {
        self.set(element, value);
        self
    }

    /// Adds another table on top of this one (used to stack equipment).
    pub fn add(&mut self, other: &ElementalResistances) {
        for (element, value) in &other.values {
            let current = self.raw(*element);
            self.set(*element, current + value);
        }
    }

    fn raw(&self, element: Element) -> f32 {
        self.values
            .iter()
            .find(|(e, _)| *e == element)
            .map_or(0.0, |(_, v)| *v)
    }

    /// Damage multiplier for an element: `1 - resistance`.
    pub fn damage_multiplier(&self, element: Element) -> f32 {
        1.0 - self.get(element)
    }
}

/// Resistances to negative status effects, same scale as elemental ones
/// (only `[0, 1]` is meaningful: negative values increase the chance).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StatusResistances {
    #[serde(default)]
    values: Vec<(StatusEffect, f32)>,
}

impl StatusResistances {
    pub fn get(&self, status: StatusEffect) -> f32 {
        self.values
            .iter()
            .find(|(s, _)| *s == status)
            .map_or(0.0, |(_, v)| clamp_resistance(*v))
    }

    pub fn set(&mut self, status: StatusEffect, value: f32) {
        match self.values.iter_mut().find(|(s, _)| *s == status) {
            Some(entry) => entry.1 = value,
            None => self.values.push((status, value)),
        }
    }

    pub fn with(mut self, status: StatusEffect, value: f32) -> Self {
        self.set(status, value);
        self
    }

    pub fn add(&mut self, other: &StatusResistances) {
        for (status, value) in &other.values {
            let current = self
                .values
                .iter()
                .find(|(s, _)| s == status)
                .map_or(0.0, |(_, v)| *v);
            self.set(*status, current + value);
        }
    }

    /// Probability that an effect with `base_chance` sticks.
    pub fn application_chance(&self, status: StatusEffect, base_chance: f32) -> f32 {
        (base_chance * (1.0 - self.get(status))).clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_element_is_neutral() {
        let res = ElementalResistances::default();
        assert_eq!(res.get(Element::Fire), 0.0);
        assert_eq!(res.damage_multiplier(Element::Fire), 1.0);
    }

    #[test]
    fn resistance_is_clamped() {
        let res = ElementalResistances::default()
            .with(Element::Ice, 3.0)
            .with(Element::Fire, -4.0);
        assert_eq!(res.get(Element::Ice), MAX_RESISTANCE);
        assert_eq!(res.get(Element::Fire), MIN_RESISTANCE);
        assert_eq!(res.damage_multiplier(Element::Fire), 2.0);
    }

    #[test]
    fn resistances_stack() {
        let mut a = ElementalResistances::default().with(Element::Dark, 0.25);
        a.add(&ElementalResistances::default().with(Element::Dark, 0.25));
        assert!((a.get(Element::Dark) - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn status_application_chance() {
        let res = StatusResistances::default().with(StatusEffect::Poison, 0.75);
        assert!((res.application_chance(StatusEffect::Poison, 0.8) - 0.2).abs() < 1e-6);
        assert_eq!(res.application_chance(StatusEffect::Curse, 0.3), 0.3);
    }
}
