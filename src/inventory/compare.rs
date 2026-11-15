//! Equipment comparison (V2.0): the final, effective numbers before and
//! after a change, computed with the game's own formulas.
//!
//! Pure module: no Bevy types. Nothing here is estimated for display: the
//! derived stats come from [`StatFormulas::derive`] (the same call as
//! `CharacterSheet`) and the strike from [`ItemCatalog::strike_profile`]
//! (the same call as the basic attack).

use super::equipment::Equipment;
use super::items::ItemCatalog;
use super::weapons::StrikeProfile;
use crate::magic::elements::Element;
use crate::player::attributes::{BaseAttributes, DerivedStats, PrimaryAttribute, StatFormulas};

/// How a value is shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueFormat {
    Integer,
    /// Fraction shown as a percentage.
    Percent,
    /// Two decimals (seconds, multipliers, per-second rates).
    Decimal,
}

/// One compared number.
#[derive(Debug, Clone, PartialEq)]
pub struct StatLine {
    pub label: String,
    pub before: f32,
    pub after: f32,
    pub format: ValueFormat,
}

impl StatLine {
    pub fn delta(&self) -> f32 {
        self.after - self.before
    }

    pub fn changed(&self) -> bool {
        (self.after - self.before).abs() > 1e-4
    }
}

/// Derived stats and strike of one equipment set.
#[derive(Debug, Clone, PartialEq)]
pub struct Snapshot {
    pub derived: DerivedStats,
    pub strike: StrikeProfile,
}

impl Snapshot {
    pub fn of(
        base: &BaseAttributes,
        level: u32,
        formulas: &StatFormulas,
        equipment: &Equipment,
        catalog: &ItemCatalog,
    ) -> Self {
        let modifiers = equipment.total_modifiers(catalog);
        let derived = formulas.derive(base, level, &modifiers);
        let strike = catalog.strike_profile(equipment, derived.attack_speed);
        Self { derived, strike }
    }
}

/// Every compared line, in a fixed order (changed or not).
pub fn compare(before: &Snapshot, after: &Snapshot) -> Vec<StatLine> {
    use ValueFormat::{Decimal, Integer, Percent};
    let (b, a) = (&before.derived, &after.derived);
    let line = |label: &str, before: f32, after: f32, format| StatLine {
        label: label.to_owned(),
        before,
        after,
        format,
    };
    let mut lines = vec![
        line(
            "Strike power",
            before.strike.raw_power(b.attack),
            after.strike.raw_power(a.attack),
            Integer,
        ),
        line(
            "Reach",
            before.strike.far_edge(),
            after.strike.far_edge(),
            Integer,
        ),
        line(
            "Swing time",
            before.strike.action_time,
            after.strike.action_time,
            Decimal,
        ),
        line("ATK", b.attack as f32, a.attack as f32, Integer),
        line("DEF", b.defense as f32, a.defense as f32, Integer),
        line(
            "MATK",
            b.magic_attack as f32,
            a.magic_attack as f32,
            Integer,
        ),
        line(
            "MDEF",
            b.magic_defense as f32,
            a.magic_defense as f32,
            Integer,
        ),
        line("Max HP", b.max_hp as f32, a.max_hp as f32, Integer),
        line("Max MP", b.max_mp as f32, a.max_mp as f32, Integer),
    ];
    for attribute in PrimaryAttribute::ALL {
        lines.push(line(
            attribute.short_name(),
            b.effective.get(attribute) as f32,
            a.effective.get(attribute) as f32,
            Integer,
        ));
    }
    lines.extend([
        line("CRIT", b.critical_chance, a.critical_chance, Percent),
        line("DODGE", b.dodge, a.dodge, Percent),
        line("HP regen", b.hp_regen, a.hp_regen, Decimal),
        line("MP regen", b.mp_regen, a.mp_regen, Decimal),
    ]);
    for element in Element::ALL.into_iter().filter(|e| *e != Element::Physical) {
        lines.push(line(
            element.display_name(),
            b.resistances.get(element),
            a.resistances.get(element),
            Percent,
        ));
    }
    lines
}
