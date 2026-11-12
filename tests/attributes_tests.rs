//! Derived stat formulas, equipment modifiers and resistances.

use dark_castle_rpg::magic::elements::{Element, ElementalResistances, StatusEffect};
use dark_castle_rpg::player::attributes::{
    AttributeModifiers, BaseAttributes, ResourcePool, StatFormulas,
};

fn base() -> BaseAttributes {
    BaseAttributes {
        strength: 10,
        constitution: 8,
        intelligence: 6,
        luck: 5,
    }
}

#[test]
fn derives_documented_values_at_level_one() {
    let f = StatFormulas::default();
    let d = f.derive(&base(), 1, &AttributeModifiers::default());
    // max_hp = 50 + 8*10 + 1*5
    assert_eq!(d.max_hp, 135);
    // max_mp = 20 + 6*5 + 1*2
    assert_eq!(d.max_mp, 52);
    assert_eq!(d.attack, 20);
    assert_eq!(d.defense, 8);
    assert_eq!(d.magic_attack, 12);
    // mdef = 6*1 + 8/2
    assert_eq!(d.magic_defense, 10);
    assert_eq!(d.hit, 105);
    assert!((d.critical_chance - 0.075).abs() < 1e-6);
    assert!((d.critical_damage - 1.55).abs() < 1e-6);
    assert!((d.dodge - 0.01).abs() < 1e-6);
    assert_eq!(d.attack_speed, 1.0);
    assert_eq!(d.movement_speed, 1.0);
}

#[test]
fn level_increases_hp_and_mp() {
    let f = StatFormulas::default();
    let low = f.derive(&base(), 1, &AttributeModifiers::default());
    let high = f.derive(&base(), 10, &AttributeModifiers::default());
    assert_eq!(high.max_hp - low.max_hp, 9 * 5);
    assert_eq!(high.max_mp - low.max_mp, 9 * 2);
}

#[test]
fn equipment_modifies_attributes_and_derived_stats() {
    let f = StatFormulas::default();
    let sword = AttributeModifiers {
        attack: 12,
        strength: 2,
        ..Default::default()
    };
    let ring = AttributeModifiers {
        luck: 10,
        resistances: ElementalResistances::default().with(Element::Fire, 0.3),
        ..Default::default()
    };
    let total = AttributeModifiers::sum([&sword, &ring]);
    let d = f.derive(&base(), 1, &total);
    // (10 + 2) * 2 + 12
    assert_eq!(d.attack, 36);
    assert_eq!(d.effective.strength, 12);
    assert_eq!(d.effective.luck, 15);
    assert!((d.resistances.get(Element::Fire) - 0.3).abs() < 1e-6);
    // Base attributes are untouched.
    assert_eq!(base().strength, 10);
}

#[test]
fn caps_are_respected() {
    let f = StatFormulas::default();
    let lucky = BaseAttributes {
        luck: 1000,
        ..base()
    };
    let d = f.derive(&lucky, 1, &AttributeModifiers::default());
    assert_eq!(d.critical_chance, f.max_critical_chance);
    assert_eq!(d.critical_damage, f.max_critical_damage);
    assert_eq!(d.dodge, f.max_dodge);
}

#[test]
fn negative_modifiers_never_produce_invalid_stats() {
    let f = StatFormulas::default();
    let curse = AttributeModifiers {
        max_hp: -10_000,
        attack: -10_000,
        movement_speed: -5.0,
        ..Default::default()
    };
    let d = f.derive(&base(), 1, &curse);
    assert_eq!(d.max_hp, 1);
    assert_eq!(d.attack, 0);
    assert_eq!(d.movement_speed, f.min_speed_multiplier);
}

#[test]
fn status_resistances_come_from_modifiers() {
    let f = StatFormulas::default();
    let amulet = AttributeModifiers {
        status_resistances: dark_castle_rpg::magic::elements::StatusResistances::default()
            .with(StatusEffect::Poison, 1.0),
        ..Default::default()
    };
    let d = f.derive(&base(), 1, &amulet);
    assert_eq!(
        d.status_resistances
            .application_chance(StatusEffect::Poison, 0.9),
        0.0
    );
}

#[test]
fn resource_pool_keeps_current_within_new_max() {
    let mut hp = ResourcePool::full(100);
    hp.set_max(60);
    assert_eq!(hp.current(), 60);
    hp.set_max(120);
    assert_eq!(hp.current(), 60);
    assert!((hp.ratio() - 0.5).abs() < 1e-6);
}
