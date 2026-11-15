//! V2.0: equipment rules without the engine: the real catalogue, weapon
//! classes, strike profiles, bag <-> slot moves, gates, modifiers, maxima,
//! resistances, comparison, appearance, technique compatibility and the
//! save format. ECS flows (inventory screen, combat, form, save/load,
//! chests) are in `equipment_app_tests.rs`.

use std::collections::BTreeSet;
use std::path::Path;

use dark_castle_rpg::combat::damage::{
    AttackProfile, DamageKind, DamageRolls, DefenseProfile, HitSpec, calculate_damage,
};
use dark_castle_rpg::core::constants::{
    BASIC_ATTACK_HITBOX, BASIC_ATTACK_TIME, HITBOX_ACTIVE_FRACTION,
};
use dark_castle_rpg::core::resources::GameData;
use dark_castle_rpg::inventory::appearance::{Appearance, DEFAULT_TRAIL, ItemVisual, Rgb};
use dark_castle_rpg::inventory::compare::{Snapshot, compare};
use dark_castle_rpg::inventory::equipment::{Equipment, EquipmentSlot};
use dark_castle_rpg::inventory::inventory::Inventory;
use dark_castle_rpg::inventory::items::{CatalogError, ItemCatalog, ItemCategory, ItemDataFile};
use dark_castle_rpg::inventory::loadout::{
    EquipBlock, EquipGate, LoadoutError, equip_from_bag, target_slot, unequip_to_bag,
};
use dark_castle_rpg::inventory::weapons::{StrikeProfile, WeaponError};
use dark_castle_rpg::magic::elements::Element;
use dark_castle_rpg::player::attributes::{BaseAttributes, ResourcePool};
use dark_castle_rpg::player::state::PlayerState;
use dark_castle_rpg::save::serialization::{decode_versioned, encode};
use dark_castle_rpg::skills::execution::{CommandOutcome, check_weapon};
use dark_castle_rpg::skills::unlock::RequirementContext;

fn data() -> GameData {
    GameData::load(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap_or_else(|e| panic!("{e}"))
}

const FREE: EquipGate = EquipGate {
    state: PlayerState::Idle,
    transformed: false,
    in_transition: false,
    strike_active: false,
};

struct Ctx {
    base: BaseAttributes,
    empty: BTreeSet<String>,
}

impl Ctx {
    fn new() -> Self {
        Self {
            base: BaseAttributes {
                strength: 6,
                constitution: 6,
                intelligence: 6,
                luck: 4,
            },
            empty: BTreeSet::new(),
        }
    }

    fn get(&self) -> RequirementContext<'_> {
        RequirementContext {
            level: 1,
            attributes: &self.base,
            owned_items: &self.empty,
            progress_flags: &self.empty,
        }
    }
}

/// The starting loadout: sword and jerkin equipped, plus `bag` items.
fn start(data: &GameData, bag: &[&str]) -> (Inventory, Equipment) {
    let mut inventory = Inventory::default();
    for id in bag {
        inventory
            .add(id, 1, &data.items)
            .unwrap_or_else(|e| panic!("{e}"));
    }
    let ctx = Ctx::new();
    let mut equipment = Equipment::default();
    for (slot, id) in &data.character.start.equipped {
        let def = data.items.get(id).unwrap_or_else(|| panic!("{id}"));
        equipment
            .equip(*slot, def, &ctx.get())
            .unwrap_or_else(|e| panic!("{e}"));
    }
    (inventory, equipment)
}

fn snapshot(data: &GameData, equipment: &Equipment) -> Snapshot {
    Snapshot::of(
        &Ctx::new().base,
        1,
        &data.character.formulas,
        equipment,
        &data.items,
    )
}

// ---- 1-3: catalogue -------------------------------------------------------

#[test]
fn the_catalogue_loads_with_every_slot_and_weapon_category_covered() {
    let data = data();
    let equipment: Vec<_> = data
        .items
        .iter()
        .filter(|d| d.category.is_equipment())
        .collect();
    assert!(equipment.len() >= 24, "only {} items", equipment.len());
    for category in [
        ItemCategory::Sword,
        ItemCategory::Dagger,
        ItemCategory::Axe,
        ItemCategory::Spear,
        ItemCategory::HeavyWeapon,
    ] {
        let n = equipment.iter().filter(|d| d.category == category).count();
        assert!(n >= 2, "{category:?}: {n}");
        assert!(data.items.weapon_class(category).is_some(), "{category:?}");
    }
    for slot in EquipmentSlot::ALL {
        let n = equipment
            .iter()
            .filter(|d| d.category.slots().contains(&slot))
            .count();
        assert!(n >= 2, "{slot:?}: {n}");
    }
    for rarity in ["common", "uncommon", "rare", "epic", "legendary"] {
        assert!(data.items.rarity(rarity).is_some(), "{rarity}");
    }
    // Every item has a description and a sell value; weapons resolve.
    for def in &equipment {
        assert!(!def.description.is_empty(), "{}", def.id);
        assert!(def.sell_value > 0, "{}", def.id);
    }
    assert!(data.items.weapons_without_class().is_empty());
}

fn parse(text: &str) -> Result<ItemDataFile, ron::error::SpannedError> {
    ron::from_str(text)
}

const RARITY: &str = r#"rarities: [(id: "c", name: "C", color: (1.0, 1.0, 1.0), tier: 0)],"#;

#[test]
fn duplicate_items_and_weapon_classes_are_rejected() {
    let item = r#"(id: "x", name: "X", description: "d", category: Ring, rarity: "c")"#;
    let file =
        parse(&format!("({RARITY} items: [{item}, {item}])")).unwrap_or_else(|e| panic!("{e}"));
    assert!(matches!(
        ItemCatalog::from_data([file]),
        Err(CatalogError::DuplicateId(id)) if id == "x"
    ));
    let class = r#"(category: Axe, name: "Axe", reach: 50.0, height: 30.0, gap: 8.0, raise: 4.0, attack_time: 0.3, recovery: 0.1, power: 1.4)"#;
    let file =
        parse(&format!("(weapon_classes: [{class}, {class}])")).unwrap_or_else(|e| panic!("{e}"));
    assert!(matches!(
        ItemCatalog::from_data([file]),
        Err(CatalogError::Weapon(WeaponError::Duplicate(
            ItemCategory::Axe
        )))
    ));
}

#[test]
fn unknown_categories_and_invalid_weapon_data_are_rejected() {
    // An unknown category does not even parse.
    assert!(
        parse(&format!(
            r#"({RARITY} items: [(id: "x", name: "X", description: "d", category: Whip, rarity: "c")])"#
        ))
        .is_err()
    );
    // A class for a non-weapon category.
    let file = parse(r#"(weapon_classes: [(category: Ring, name: "R", reach: 50.0, height: 30.0, gap: 8.0, raise: 4.0, attack_time: 0.3, recovery: 0.1, power: 1.4)])"#)
        .unwrap_or_else(|e| panic!("{e}"));
    assert!(matches!(
        ItemCatalog::from_data([file]),
        Err(CatalogError::Weapon(WeaponError::NotAWeapon(
            ItemCategory::Ring
        )))
    ));
    // Negative reach, zero time.
    for bad in ["reach: -5.0, height: 30.0", "reach: 50.0, height: 0.0"] {
        let file = parse(&format!(
            r#"(weapon_classes: [(category: Axe, name: "A", {bad}, gap: 8.0, raise: 4.0, attack_time: 0.3, recovery: 0.1, power: 1.4)])"#
        ))
        .unwrap_or_else(|e| panic!("{e}"));
        assert!(
            matches!(
                ItemCatalog::from_data([file]),
                Err(CatalogError::Weapon(WeaponError::OutOfRange { .. }))
            ),
            "{bad}"
        );
    }
    let file = parse(r#"(weapon_classes: [(category: Axe, name: "A", reach: 50.0, height: 30.0, gap: 8.0, raise: 4.0, attack_time: 0.0, recovery: 0.1, power: 1.4)])"#)
        .unwrap_or_else(|e| panic!("{e}"));
    assert!(ItemCatalog::from_data([file]).is_err());
    // Weapon data on a ring; a cape on a weapon; a black tint.
    for (category, extra) in [
        ("Ring", "weapon: Some((element: Fire))"),
        (
            "Sword",
            "visual: Some(Cape(color: (1.0, 0.0, 0.0), length: 30.0))",
        ),
        ("Armor", "visual: Some(Tint(color: (0.0, 0.0, 0.0)))"),
        (
            "Armor",
            "visual: Some(Cape(color: (1.0, 0.0, 0.0), length: 300.0))",
        ),
    ] {
        let file = parse(&format!(
            r#"({RARITY} items: [(id: "x", name: "X", description: "d", category: {category}, rarity: "c", {extra})])"#
        ))
        .unwrap_or_else(|e| panic!("{e}"));
        assert!(
            ItemCatalog::from_data([file]).is_err(),
            "{category} {extra}"
        );
    }
}

// ---- 4-10: equip, remove, swap ------------------------------------------

#[test]
fn equipping_moves_one_unit_from_the_bag_and_returns_the_old_item() {
    let data = data();
    let ctx = Ctx::new();
    let (mut bag, mut eq) = start(&data, &["watch_pike"]);
    let change = equip_from_bag(
        &mut bag,
        &mut eq,
        EquipmentSlot::Weapon,
        "watch_pike",
        &data.items,
        &ctx.get(),
        &FREE,
    )
    .unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(change.returned.as_deref(), Some("iron_sword"));
    assert_eq!(eq.get(EquipmentSlot::Weapon), Some("watch_pike"));
    assert_eq!(bag.count("watch_pike"), 0);
    assert_eq!(bag.count("iron_sword"), 1);
    // Swap back: still exactly one of each, nothing lost or duplicated.
    equip_from_bag(
        &mut bag,
        &mut eq,
        EquipmentSlot::Weapon,
        "iron_sword",
        &data.items,
        &ctx.get(),
        &FREE,
    )
    .unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(bag.count("watch_pike") + bag.count("iron_sword"), 1);
    assert_eq!(eq.get(EquipmentSlot::Weapon), Some("iron_sword"));
    // Remove: back to the bag.
    let removed = unequip_to_bag(&mut bag, &mut eq, EquipmentSlot::Weapon, &data.items, &FREE)
        .unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(removed.returned.as_deref(), Some("iron_sword"));
    assert_eq!(eq.get(EquipmentSlot::Weapon), None);
    assert_eq!(bag.count("iron_sword"), 1);
    assert_eq!(
        unequip_to_bag(&mut bag, &mut eq, EquipmentSlot::Weapon, &data.items, &FREE),
        Err(LoadoutError::EmptySlot(EquipmentSlot::Weapon))
    );
}

#[test]
fn refused_changes_leave_bag_and_slots_untouched() {
    let data = data();
    let ctx = Ctx::new();
    let (bag0, eq0) = start(&data, &["watch_pike", "ossuary_greatblade", "minor_tonic"]);
    let cases: Vec<(EquipmentSlot, &str, EquipGate)> = vec![
        // Wrong slot, item not owned, unknown, not equipment, requirement.
        (EquipmentSlot::Head, "watch_pike", FREE),
        (EquipmentSlot::Weapon, "bell_maul", FREE),
        (EquipmentSlot::Weapon, "no_such_item", FREE),
        (EquipmentSlot::Weapon, "minor_tonic", FREE),
        (EquipmentSlot::Weapon, "ossuary_greatblade", FREE),
    ];
    for (slot, item, gate) in cases {
        let (mut bag, mut eq) = (bag0.clone(), eq0.clone());
        let result = equip_from_bag(
            &mut bag,
            &mut eq,
            slot,
            item,
            &data.items,
            &ctx.get(),
            &gate,
        );
        assert!(result.is_err(), "{item} in {slot:?}");
        assert_eq!(bag, bag0, "{item}");
        assert_eq!(eq, eq0, "{item}");
    }
}

#[test]
fn the_gate_blocks_death_actions_forms_passages_and_live_strikes() {
    let blocked = [
        (PlayerState::Dead, false, false, false, EquipBlock::Dead),
        (
            PlayerState::Attacking,
            false,
            false,
            false,
            EquipBlock::Busy(PlayerState::Attacking),
        ),
        (
            PlayerState::Casting,
            false,
            false,
            false,
            EquipBlock::Busy(PlayerState::Casting),
        ),
        (
            PlayerState::Dodging,
            false,
            false,
            false,
            EquipBlock::Busy(PlayerState::Dodging),
        ),
        (
            PlayerState::Guarding,
            false,
            false,
            false,
            EquipBlock::Busy(PlayerState::Guarding),
        ),
        (
            PlayerState::Idle,
            true,
            false,
            false,
            EquipBlock::Transformed,
        ),
        (
            PlayerState::Idle,
            false,
            true,
            false,
            EquipBlock::InTransition,
        ),
        (
            PlayerState::Idle,
            false,
            false,
            true,
            EquipBlock::Busy(PlayerState::Idle),
        ),
    ];
    let data = data();
    let ctx = Ctx::new();
    for (state, transformed, in_transition, strike_active, expected) in blocked {
        let gate = EquipGate {
            state,
            transformed,
            in_transition,
            strike_active,
        };
        assert_eq!(gate.check(), Err(expected));
        let (mut bag, mut eq) = start(&data, &["watch_pike"]);
        let before = (bag.clone(), eq.clone());
        let result = equip_from_bag(
            &mut bag,
            &mut eq,
            EquipmentSlot::Weapon,
            "watch_pike",
            &data.items,
            &ctx.get(),
            &gate,
        );
        assert_eq!(result, Err(LoadoutError::Blocked(expected)));
        assert_eq!((bag, eq), before);
    }
    for state in [
        PlayerState::Idle,
        PlayerState::Walking,
        PlayerState::Running,
        PlayerState::Crouching,
        PlayerState::Jumping,
        PlayerState::Falling,
    ] {
        let gate = EquipGate { state, ..FREE };
        assert_eq!(gate.check(), Ok(()), "{state:?}");
    }
}

#[test]
fn two_accessories_combine_and_a_single_copy_cannot_fill_both_slots() {
    let data = data();
    let ctx = Ctx::new();
    let (mut bag, mut eq) = start(
        &data,
        &["veilwoven_band", "bloodstone_pendant", "ashen_ring"],
    );
    let base = snapshot(&data, &eq).derived;
    for id in ["veilwoven_band", "bloodstone_pendant"] {
        let slot = target_slot(id, &eq, &data.items).unwrap_or_else(|| panic!("{id}"));
        equip_from_bag(&mut bag, &mut eq, slot, id, &data.items, &ctx.get(), &FREE)
            .unwrap_or_else(|e| panic!("{e}"));
    }
    assert_eq!(eq.get(EquipmentSlot::Accessory1), Some("veilwoven_band"));
    assert_eq!(
        eq.get(EquipmentSlot::Accessory2),
        Some("bloodstone_pendant")
    );
    let both = snapshot(&data, &eq).derived;
    // Veilwoven Band: +6 max MP and +0.2 MP/s; pendant: +15 max HP.
    assert_eq!(both.max_mp, base.max_mp + 6);
    assert!((both.mp_regen - base.mp_regen - 0.2).abs() < 1e-5);
    assert_eq!(both.max_hp, base.max_hp + 15);
    // Both full: a third ring replaces Accessory 1 deterministically.
    assert_eq!(
        target_slot("ashen_ring", &eq, &data.items),
        Some(EquipmentSlot::Accessory1)
    );
    // The band is no longer in the bag: it cannot go to the second slot too.
    assert_eq!(
        equip_from_bag(
            &mut bag,
            &mut eq,
            EquipmentSlot::Accessory2,
            "veilwoven_band",
            &data.items,
            &ctx.get(),
            &FREE,
        ),
        Err(LoadoutError::NotOwned("veilwoven_band".into()))
    );
}

#[test]
fn repeated_equip_and_remove_never_accumulates_a_bonus() {
    let data = data();
    let ctx = Ctx::new();
    let (mut bag, mut eq) = start(&data, &["chain_hauberk"]);
    let original = snapshot(&data, &eq).derived;
    for _ in 0..50 {
        equip_from_bag(
            &mut bag,
            &mut eq,
            EquipmentSlot::Body,
            "chain_hauberk",
            &data.items,
            &ctx.get(),
            &FREE,
        )
        .unwrap_or_else(|e| panic!("{e}"));
        equip_from_bag(
            &mut bag,
            &mut eq,
            EquipmentSlot::Body,
            "leather_jerkin",
            &data.items,
            &ctx.get(),
            &FREE,
        )
        .unwrap_or_else(|e| panic!("{e}"));
    }
    assert_eq!(snapshot(&data, &eq).derived, original);
    assert_eq!(bag.count("chain_hauberk"), 1);
    assert_eq!(bag.count("leather_jerkin"), 0);
    assert_eq!(eq.len(), 2);
}

#[test]
fn modifiers_reach_the_final_stats_and_vanish_when_removed() {
    let data = data();
    let (_, eq) = start(&data, &[]);
    let base = snapshot(&data, &eq).derived;
    let with = |slot, id: &str| snapshot(&data, &eq.preview(slot, Some(id))).derived;
    let hauberk = with(EquipmentSlot::Body, "chain_hauberk");
    // Jerkin DEF 4 -> hauberk DEF 8 and +10 max HP.
    assert_eq!(hauberk.defense, base.defense + 4);
    assert_eq!(hauberk.max_hp, base.max_hp + 10);
    let bracers = with(EquipmentSlot::Gloves, "scribes_bracers");
    // +2 INT: +10 max MP (5 per INT), +4 MATK, MDEF +2 (INT) +2 (item).
    assert_eq!(
        bracers.effective.intelligence,
        base.effective.intelligence + 2
    );
    assert_eq!(bracers.max_mp, base.max_mp + 10);
    assert_eq!(bracers.magic_defense, base.magic_defense + 4);
    let soles = with(EquipmentSlot::Boots, "quiet_soles");
    assert!((soles.dodge - base.dodge - 0.03).abs() < 1e-5);
    let removed = snapshot(&data, &eq.preview(EquipmentSlot::Body, None)).derived;
    assert_eq!(removed.defense, base.defense - 4);
}

#[test]
fn lower_maxima_clamp_current_hp_and_mp_without_restoring_them() {
    let mut hp = ResourcePool::new(60, 125);
    hp.set_max(110);
    assert_eq!(hp.current(), 60, "below the new max: unchanged");
    hp.set_max(140);
    assert_eq!(hp.current(), 60, "a higher max restores nothing");
    let mut mp = ResourcePool::full(58);
    mp.set_max(52);
    assert_eq!(mp.current(), 52, "above the new max: clamped");
    mp.set_max(-5);
    assert_eq!((mp.current(), mp.max()), (0, 0), "never negative");
}

#[test]
fn stacked_resistances_are_capped_and_never_heal() {
    let data = data();
    let (_, eq) = start(&data, &[]);
    // Emberlined Cloak (Fire 0.2) + Ashen Ring (Fire 0.25) = 0.45.
    let eq = eq
        .preview(EquipmentSlot::Body, Some("emberlined_cloak"))
        .preview(EquipmentSlot::Accessory1, Some("ashen_ring"));
    let derived = snapshot(&data, &eq).derived;
    assert!((derived.resistances.get(Element::Fire) - 0.45).abs() < 1e-5);
    assert!((derived.resistances.get(Element::Ice) - 0.1).abs() < 1e-5);
    // A hand-made extreme set is capped at the formulas' maximum.
    let mut mods = eq.total_modifiers(&data.items);
    mods.resistances.set(Element::Fire, 9.0);
    let capped = data
        .character
        .formulas
        .derive(&Ctx::new().base, 1, &mods)
        .resistances
        .get(Element::Fire);
    assert_eq!(capped, data.character.formulas.max_resistance);
    assert!(capped < 1.0);
    // A capped fire hit still deals damage (no immunity, no healing).
    let attacker = AttackProfile {
        power: 40,
        hit: 200,
        critical_chance: 0.0,
        critical_damage: 1.5,
    };
    let mut resistances = mods.resistances.capped(-1.0, capped);
    resistances.set(Element::Fire, capped);
    let defender = DefenseProfile {
        defense: 0,
        magic_defense: 0,
        dodge: 0.0,
        resistances,
    };
    let hit = HitSpec {
        kind: DamageKind::Magical,
        element: Element::Fire,
        base_damage: 0,
        multiplier: 1.0,
    };
    let amount = calculate_damage(&attacker, &defender, &hit, DamageRolls::AVERAGE).amount();
    assert!(amount > 0);
}

// ---- 14-19: weapons in combat --------------------------------------------

#[test]
fn the_starting_sword_keeps_the_pre_v2_strike_exactly() {
    let data = data();
    let (_, eq) = start(&data, &[]);
    let strike = data.items.strike_profile(&eq, 1.0);
    assert_eq!(strike.category, Some(ItemCategory::Sword));
    assert_eq!(strike.size, BASIC_ATTACK_HITBOX);
    assert_eq!(strike.offset, (BASIC_ATTACK_HITBOX.0 / 2.0 + 8.0, 4.0));
    assert_eq!(strike.action_time, BASIC_ATTACK_TIME);
    assert_eq!(
        strike.active_time,
        BASIC_ATTACK_TIME * HITBOX_ACTIVE_FRACTION
    );
    assert_eq!(strike.multiplier, 1.0);
    assert_eq!(strike.element, Element::Physical);
    let unarmed = StrikeProfile::unarmed(1.0);
    assert_eq!((strike.size, strike.offset), (unarmed.size, unarmed.offset));
}

#[test]
fn the_five_categories_differ_in_damage_reach_speed_and_recovery() {
    let data = data();
    let (_, eq) = start(&data, &[]);
    let profile = |id: &str| {
        let eq = eq.preview(EquipmentSlot::Weapon, Some(id));
        let snap = snapshot(&data, &eq);
        (snap.strike, snap.strike.raw_power(snap.derived.attack))
    };
    let (sword, sword_power) = profile("iron_sword");
    let (dagger, dagger_power) = profile("gutter_knife");
    let (axe, axe_power) = profile("hearthsplitter");
    let (spear, _) = profile("watch_pike");
    let (heavy, heavy_power) = profile("bell_maul");
    // Damage per hit.
    assert!(dagger_power < sword_power);
    assert!(axe_power > sword_power);
    assert!(heavy_power > axe_power);
    // Reach: the spear goes farthest but leaves the body uncovered.
    assert!(spear.far_edge() > heavy.far_edge());
    assert!(spear.far_edge() > sword.far_edge() + 30.0);
    assert!(spear.near_edge() > sword.near_edge() + 10.0);
    assert!(dagger.far_edge() < sword.far_edge());
    // Speed and recovery.
    assert!(dagger.action_time < sword.action_time);
    assert!(axe.action_time > sword.action_time);
    assert!(heavy.action_time > axe.action_time);
    assert!(heavy.action_time - heavy.active_time > sword.action_time - sword.active_time);
    // Elements of the item, not the class.
    let (saber, _) = profile("duskwater_saber");
    assert_eq!(saber.element, Element::Ice);
    assert!(saber.far_edge() > sword.far_edge());
}

#[test]
fn sword_techniques_refuse_other_weapons_and_no_weapon() {
    let data = data();
    for id in ["crescent_slash", "rising_edge"] {
        let def = data.skills.get(id).unwrap_or_else(|| panic!("{id}"));
        assert_eq!(check_weapon(def, Some(ItemCategory::Sword)), None);
        for other in [
            Some(ItemCategory::Dagger),
            Some(ItemCategory::Axe),
            Some(ItemCategory::Spear),
            Some(ItemCategory::HeavyWeapon),
            None,
        ] {
            assert_eq!(
                check_weapon(def, other),
                Some(CommandOutcome::WrongWeapon { id: id.into() }),
                "{id} {other:?}"
            );
        }
    }
    // Spells have no weapon restriction.
    for def in data.skills.iter().filter(|d| d.category.is_spell()) {
        assert!(def.weapons.is_empty(), "{}", def.id);
        assert_eq!(check_weapon(def, Some(ItemCategory::Axe)), None);
    }
}

// ---- comparison, appearance -----------------------------------------------

#[test]
fn the_comparison_reports_the_formulas_own_numbers() {
    let data = data();
    let (_, eq) = start(&data, &[]);
    let before = snapshot(&data, &eq);
    let after = snapshot(
        &data,
        &eq.preview(EquipmentSlot::Weapon, Some("hearthsplitter")),
    );
    let lines = compare(&before, &after);
    let line = |label: &str| {
        lines
            .iter()
            .find(|l| l.label == label)
            .unwrap_or_else(|| panic!("{label}"))
    };
    assert_eq!(line("ATK").before, before.derived.attack as f32);
    assert_eq!(line("ATK").after, after.derived.attack as f32);
    assert_eq!(line("Swing time").after, after.strike.action_time);
    assert_eq!(
        line("Strike power").after,
        after.strike.raw_power(after.derived.attack)
    );
    assert!(!line("DEF").changed());
    // Same equipment: nothing changes.
    assert!(compare(&before, &before).iter().all(|l| !l.changed()));
}

#[test]
fn appearance_resolves_trail_cape_and_tint_from_the_slots() {
    let data = data();
    let (_, eq) = start(&data, &[]);
    assert_eq!(Appearance::resolve(&eq, &data.items), Appearance::default());
    let eq2 = eq
        .preview(EquipmentSlot::Weapon, Some("lantern_glaive"))
        .preview(EquipmentSlot::Body, Some("emberlined_cloak"));
    let look = Appearance::resolve(&eq2, &data.items);
    assert_ne!(look.trail, DEFAULT_TRAIL);
    let cape = look.cape.unwrap_or_else(|| panic!("no cape"));
    assert_eq!(cape.length, 40.0);
    assert_eq!(look.tint, Rgb::WHITE);
    let eq3 = eq.preview(EquipmentSlot::Body, Some("chain_hauberk"));
    let look = Appearance::resolve(&eq3, &data.items);
    assert!(look.cape.is_none());
    assert_ne!(look.tint, Rgb::WHITE);
    assert_eq!(look.layer_count(), 1);
    // Visuals only apply from their slot (a mis-slotted cape is ignored).
    let odd = eq.preview(EquipmentSlot::Offhand, Some("pilgrim_mantle"));
    assert!(Appearance::resolve(&odd, &data.items).cape.is_none());
    // Every item visual fits its category.
    for def in data.items.iter() {
        if let Some(visual) = def.visual {
            assert!(visual.validate(&def.id, def.category).is_ok(), "{}", def.id);
            if let ItemVisual::Trail { .. } = visual {
                assert!(def.category.is_weapon());
            }
        }
    }
}

// ---- 22-23: save format ---------------------------------------------------

#[test]
fn a_v2_save_keeps_equipment_as_ids_and_a_v1_save_still_loads_it() {
    let data = data();
    let ctx = Ctx::new();
    let (mut bag, mut eq) = start(&data, &["watch_pike", "veilwoven_band"]);
    equip_from_bag(
        &mut bag,
        &mut eq,
        EquipmentSlot::Weapon,
        "watch_pike",
        &data.items,
        &ctx.get(),
        &FREE,
    )
    .unwrap_or_else(|e| panic!("{e}"));
    equip_from_bag(
        &mut bag,
        &mut eq,
        EquipmentSlot::Accessory1,
        "veilwoven_band",
        &data.items,
        &ctx.get(),
        &FREE,
    )
    .unwrap_or_else(|e| panic!("{e}"));
    let text = ron::ser::to_string(&eq).unwrap_or_else(|e| panic!("{e}"));
    assert!(text.contains("\"watch_pike\"") && text.contains("Accessory1"));
    let back: Equipment = ron::from_str(&text).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(back, eq);
    // The V1.6-era v1 fixture (no V2.0 field anywhere) decodes with its
    // equipment intact, and the V2.0 catalogue derives its stats.
    let fixture = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/save_v1_entry_hall.ron"),
    )
    .unwrap_or_else(|e| panic!("{e}"));
    let save = decode_versioned(&fixture).unwrap_or_else(|e| panic!("{e}"));
    assert!(save.migrated());
    let old = &save.data.player.equipment;
    assert!(!old.is_empty());
    for (slot, id) in old.iter() {
        let def = data.items.get(id).unwrap_or_else(|| panic!("{id}"));
        assert!(def.category.slots().contains(&slot));
    }
    let strike = data.items.strike_profile(old, 1.0);
    assert_eq!(strike.action_time, BASIC_ATTACK_TIME);
    // Re-encoded as v2: same equipment.
    let text = encode(&save.data).unwrap_or_else(|e| panic!("{e}"));
    let again = decode_versioned(&text).unwrap_or_else(|e| panic!("{e}"));
    assert!(!again.migrated());
    assert_eq!(&again.data.player.equipment, old);
}
