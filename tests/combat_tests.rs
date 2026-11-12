//! Damage formula: defence, elements, criticals, accuracy and variance.

use dark_castle_rpg::combat::damage::{
    AttackProfile, DamageKind, DamageOutcome, DamageRolls, DefenseProfile, HitSpec,
    calculate_damage, hit_chance,
};
use dark_castle_rpg::magic::elements::{Element, ElementalResistances};

fn attacker() -> AttackProfile {
    AttackProfile {
        power: 50,
        hit: 100,
        critical_chance: 0.1,
        critical_damage: 2.0,
    }
}

fn defender(defense: i32, resistances: ElementalResistances) -> DefenseProfile {
    DefenseProfile {
        defense,
        magic_defense: defense * 2,
        dodge: 0.0,
        resistances,
    }
}

fn physical(element: Element) -> HitSpec {
    HitSpec {
        kind: DamageKind::Physical,
        element,
        base_damage: 0,
        multiplier: 1.0,
    }
}

fn amount(outcome: DamageOutcome) -> i32 {
    outcome.amount()
}

#[test]
fn zero_defense_deals_full_power() {
    let d = calculate_damage(
        &attacker(),
        &defender(0, ElementalResistances::default()),
        &physical(Element::Physical),
        DamageRolls::AVERAGE,
    );
    assert_eq!(amount(d), 50);
}

#[test]
fn hundred_defense_halves_damage() {
    let d = calculate_damage(
        &attacker(),
        &defender(100, ElementalResistances::default()),
        &physical(Element::Physical),
        DamageRolls::AVERAGE,
    );
    assert_eq!(amount(d), 25);
}

#[test]
fn magic_uses_magic_defense() {
    let spell = HitSpec {
        kind: DamageKind::Magical,
        ..physical(Element::Fire)
    };
    // magic_defense = 100 → half.
    let d = calculate_damage(
        &attacker(),
        &defender(50, ElementalResistances::default()),
        &spell,
        DamageRolls::AVERAGE,
    );
    assert_eq!(amount(d), 25);
}

#[test]
fn multiplier_and_base_damage_apply() {
    let skill = HitSpec {
        base_damage: 10,
        multiplier: 1.8,
        ..physical(Element::Physical)
    };
    let d = calculate_damage(
        &attacker(),
        &defender(0, ElementalResistances::default()),
        &skill,
        DamageRolls::AVERAGE,
    );
    assert_eq!(amount(d), 100);
}

#[test]
fn elemental_weakness_resistance_and_immunity() {
    let res = ElementalResistances::default()
        .with(Element::Fire, -0.5)
        .with(Element::Ice, 0.5)
        .with(Element::Light, 1.0);
    let def = defender(0, res);
    let fire = calculate_damage(
        &attacker(),
        &def,
        &physical(Element::Fire),
        DamageRolls::AVERAGE,
    );
    let ice = calculate_damage(
        &attacker(),
        &def,
        &physical(Element::Ice),
        DamageRolls::AVERAGE,
    );
    let light = calculate_damage(
        &attacker(),
        &def,
        &physical(Element::Light),
        DamageRolls::AVERAGE,
    );
    assert_eq!(amount(fire), 75);
    assert_eq!(amount(ice), 25);
    assert_eq!(amount(light), 0);
}

#[test]
fn critical_hits_multiply_damage() {
    let rolls = DamageRolls {
        critical: 0.05,
        ..DamageRolls::AVERAGE
    };
    let d = calculate_damage(
        &attacker(),
        &defender(0, ElementalResistances::default()),
        &physical(Element::Physical),
        rolls,
    );
    assert_eq!(
        d,
        DamageOutcome::Hit {
            amount: 100,
            critical: true,
            element: Element::Physical
        }
    );
}

#[test]
fn dodge_can_cause_misses_but_hit_chance_has_a_floor() {
    let mut def = defender(0, ElementalResistances::default());
    def.dodge = 0.2;
    assert!((hit_chance(&attacker(), &def) - 0.8).abs() < 1e-6);
    let miss = DamageRolls {
        hit: 0.85,
        ..DamageRolls::AVERAGE
    };
    assert_eq!(
        calculate_damage(&attacker(), &def, &physical(Element::Physical), miss),
        DamageOutcome::Miss
    );
    def.dodge = 0.99;
    assert!((hit_chance(&attacker(), &def) - 0.5).abs() < 1e-6);
}

#[test]
fn variance_stays_within_ten_percent() {
    let def = defender(0, ElementalResistances::default());
    let low = DamageRolls {
        variance: 0.0,
        ..DamageRolls::AVERAGE
    };
    let high = DamageRolls {
        variance: 0.999,
        ..DamageRolls::AVERAGE
    };
    assert_eq!(
        amount(calculate_damage(
            &attacker(),
            &def,
            &physical(Element::Physical),
            low
        )),
        45
    );
    assert_eq!(
        amount(calculate_damage(
            &attacker(),
            &def,
            &physical(Element::Physical),
            high
        )),
        55
    );
}

#[test]
fn connecting_hits_deal_at_least_one_damage() {
    let weak = AttackProfile {
        power: 1,
        ..attacker()
    };
    let d = calculate_damage(
        &weak,
        &defender(10_000, ElementalResistances::default()),
        &physical(Element::Physical),
        DamageRolls::AVERAGE,
    );
    assert_eq!(amount(d), 1);
}
