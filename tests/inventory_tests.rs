//! Items, rarities, inventory stacks and equipment slots.

use std::collections::BTreeSet;

use dark_castle_rpg::inventory::equipment::{EquipError, Equipment, EquipmentSlot};
use dark_castle_rpg::inventory::inventory::{Inventory, InventoryError};
use dark_castle_rpg::inventory::items::{CatalogError, ItemCatalog, ItemDataFile};
use dark_castle_rpg::player::attributes::BaseAttributes;
use dark_castle_rpg::skills::unlock::RequirementContext;

const DATA: &str = r#"(
    rarities: [
        (id: "common", name: "Common", color: (0.8, 0.8, 0.8), tier: 0),
        (id: "rare", name: "Rare", color: (0.3, 0.5, 1.0), tier: 2),
    ],
    items: [
        (id: "iron_sword", name: "Iron Sword", description: "", category: Sword,
         rarity: "common", modifiers: (attack: 8)),
        (id: "moon_ring", name: "Moon Ring", description: "", category: Ring,
         rarity: "rare", modifiers: (luck: 3, intelligence: 1)),
        (id: "heavy_blade", name: "Heavy Blade", description: "", category: HeavyWeapon,
         rarity: "rare", modifiers: (attack: 30), requirements: (min_level: 10)),
        (id: "potion", name: "Potion", description: "", category: Consumable,
         rarity: "common", consumable: Some(RestoreHp(40)), max_stack: 9),
        (id: "old_key", name: "Old Key", description: "", category: QuestItem, rarity: "common"),
    ],
)"#;

fn catalog() -> ItemCatalog {
    let file: ItemDataFile = ron::from_str(DATA).unwrap_or_else(|e| panic!("{e}"));
    ItemCatalog::from_data([file]).unwrap_or_else(|e| panic!("{e}"))
}

fn ctx_parts() -> (BaseAttributes, BTreeSet<String>, BTreeSet<String>) {
    (BaseAttributes::default(), BTreeSet::new(), BTreeSet::new())
}

#[test]
fn unknown_rarity_is_rejected() {
    let file: ItemDataFile = ron::from_str(
        r#"(items: [(id: "x", name: "X", description: "", category: Material, rarity: "mythic")])"#,
    )
    .unwrap_or_else(|e| panic!("{e}"));
    assert!(matches!(
        ItemCatalog::from_data([file]),
        Err(CatalogError::UnknownRarity { .. })
    ));
}

#[test]
fn stacks_fill_up_to_max() {
    let cat = catalog();
    let mut inv = Inventory::default();
    inv.add("potion", 12, &cat)
        .unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(inv.count("potion"), 12);
    assert_eq!(inv.stacks().len(), 2);
    inv.remove("potion", 10).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(inv.count("potion"), 2);
    assert_eq!(inv.stacks().len(), 1);
    assert_eq!(
        inv.remove("potion", 5),
        Err(InventoryError::NotEnough("potion".into()))
    );
}

#[test]
fn quest_items_cannot_be_discarded() {
    let cat = catalog();
    let mut inv = Inventory::default();
    inv.add("old_key", 1, &cat)
        .unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(
        inv.discard("old_key", 1, &cat),
        Err(InventoryError::CannotDiscard)
    );
    assert!(matches!(
        inv.add("nope", 1, &cat),
        Err(InventoryError::UnknownItem(_))
    ));
}

#[test]
fn sorting_orders_by_category_then_rarity() {
    let cat = catalog();
    let mut inv = Inventory::default();
    for id in ["potion", "moon_ring", "iron_sword"] {
        inv.add(id, 1, &cat).unwrap_or_else(|e| panic!("{e}"));
    }
    inv.sort(&cat);
    let ids: Vec<_> = inv.stacks().iter().map(|s| s.id.as_str()).collect();
    assert_eq!(ids, ["iron_sword", "moon_ring", "potion"]);
}

#[test]
fn equipping_respects_slots_and_requirements() {
    let cat = catalog();
    let (attrs, items, flags) = ctx_parts();
    let ctx = RequirementContext {
        level: 1,
        attributes: &attrs,
        owned_items: &items,
        progress_flags: &flags,
    };
    let mut eq = Equipment::default();
    let sword = cat.get("iron_sword").unwrap_or_else(|| panic!("missing"));
    let ring = cat.get("moon_ring").unwrap_or_else(|| panic!("missing"));
    let heavy = cat.get("heavy_blade").unwrap_or_else(|| panic!("missing"));

    assert!(matches!(
        eq.equip(EquipmentSlot::Head, sword, &ctx),
        Err(EquipError::WrongSlot { .. })
    ));
    assert!(matches!(
        eq.equip(EquipmentSlot::Weapon, heavy, &ctx),
        Err(EquipError::Requirement(_))
    ));
    assert_eq!(eq.equip(EquipmentSlot::Weapon, sword, &ctx), Ok(None));
    assert_eq!(eq.equip(EquipmentSlot::Accessory2, ring, &ctx), Ok(None));

    let total = eq.total_modifiers(&cat);
    assert_eq!(total.attack, 8);
    assert_eq!(total.luck, 3);

    assert_eq!(eq.unequip(EquipmentSlot::Weapon), Some("iron_sword".into()));
    assert_eq!(eq.total_modifiers(&cat).attack, 0);
}
