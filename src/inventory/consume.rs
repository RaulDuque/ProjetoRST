//! Using consumable items (D18). Pure rules: no Bevy types.
//!
//! One call removes a single unit and applies the effect, or changes nothing.
//! Inventory quantities stay the source of truth.

use super::inventory::{Inventory, InventoryError};
use super::items::{ConsumableEffect, ItemCatalog};
use crate::player::attributes::ResourcePool;
use crate::player::components::Vitals;
use crate::player::state::PlayerState;

/// Why a consumable could not be used. Nothing was changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UseItemError {
    UnknownItem(String),
    NotConsumable(String),
    NotOwned(String),
    Dead,
    Busy,
    NoEffect,
    Inventory(InventoryError),
}

impl std::fmt::Display for UseItemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownItem(id) => write!(f, "unknown item `{id}`"),
            Self::NotConsumable(id) => write!(f, "`{id}` is not consumable"),
            Self::NotOwned(id) => write!(f, "not enough `{id}`"),
            Self::Dead => write!(f, "cannot use items while fallen"),
            Self::Busy => write!(f, "cannot use items right now"),
            Self::NoEffect => write!(f, "no effect (resource already full)"),
            Self::Inventory(error) => write!(f, "{error}"),
        }
    }
}

/// What a successful use restored (for HUD feedback).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UseItemResult {
    pub hp_restored: i32,
    pub mp_restored: i32,
    pub hearts_restored: i32,
}

/// Inventory screen pauses the world: only death blocks use there.
/// While Playing, only locomotion (idle/walk/run/crouch/jump/fall).
pub fn use_allowed(state: PlayerState, inventory_open: bool) -> bool {
    if state == PlayerState::Dead {
        return false;
    }
    inventory_open || state.is_locomotion()
}

fn effect_applies(
    effect: &ConsumableEffect,
    hp: &ResourcePool,
    mp: &ResourcePool,
    hearts: &ResourcePool,
) -> bool {
    match effect {
        ConsumableEffect::RestoreHp(_) => hp.current() < hp.max(),
        ConsumableEffect::RestoreMp(_) => mp.current() < mp.max(),
        ConsumableEffect::RestoreHearts(_) => hearts.current() < hearts.max(),
    }
}

fn apply_effect(
    effect: &ConsumableEffect,
    hp: &mut ResourcePool,
    mp: &mut ResourcePool,
    hearts: &mut ResourcePool,
) -> UseItemResult {
    match *effect {
        ConsumableEffect::RestoreHp(amount) => UseItemResult {
            hp_restored: hp.restore(amount),
            mp_restored: 0,
            hearts_restored: 0,
        },
        ConsumableEffect::RestoreMp(amount) => UseItemResult {
            hp_restored: 0,
            mp_restored: mp.restore(amount),
            hearts_restored: 0,
        },
        ConsumableEffect::RestoreHearts(amount) => UseItemResult {
            hp_restored: 0,
            mp_restored: 0,
            hearts_restored: hearts.restore(amount),
        },
    }
}

/// Atomically uses one unit of `item_id` if valid.
pub fn use_consumable(
    inventory: &mut Inventory,
    catalog: &ItemCatalog,
    item_id: &str,
    state: PlayerState,
    inventory_open: bool,
    vitals: &mut Vitals,
) -> Result<UseItemResult, UseItemError> {
    if state == PlayerState::Dead {
        return Err(UseItemError::Dead);
    }
    if !use_allowed(state, inventory_open) {
        return Err(UseItemError::Busy);
    }
    let def = catalog
        .get(item_id)
        .ok_or_else(|| UseItemError::UnknownItem(item_id.to_owned()))?;
    let Some(effect) = def.consumable.as_ref() else {
        return Err(UseItemError::NotConsumable(item_id.to_owned()));
    };
    if inventory.count(item_id) == 0 {
        return Err(UseItemError::NotOwned(item_id.to_owned()));
    }
    if !effect_applies(effect, &vitals.hp, &vitals.mp, &vitals.hearts) {
        return Err(UseItemError::NoEffect);
    }
    // Apply first conceptually: remove only after restore succeeds so a
    // full resource never burns a potion. restore() already clamps.
    let result = apply_effect(effect, &mut vitals.hp, &mut vitals.mp, &mut vitals.hearts);
    if result.hp_restored == 0 && result.mp_restored == 0 && result.hearts_restored == 0 {
        return Err(UseItemError::NoEffect);
    }
    inventory
        .remove(item_id, 1)
        .map_err(UseItemError::Inventory)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::items::{ItemCatalog, ItemDataFile};
    use crate::player::attributes::{RegenAccumulator, ResourcePool};
    use crate::player::components::Vitals;
    use crate::player::state::PlayerState;

    fn catalog() -> ItemCatalog {
        let file: ItemDataFile = ron::from_str(
            r#"(
            rarities: [(id: "common", name: "Common", color: (1.0, 1.0, 1.0), tier: 0)],
            items: [
                (id: "potion", name: "Potion", description: "", category: Consumable,
                 rarity: "common", consumable: Some(RestoreHp(40)), max_stack: 9),
                (id: "ether", name: "Ether", description: "", category: Consumable,
                 rarity: "common", consumable: Some(RestoreMp(30)), max_stack: 9),
                (id: "sword", name: "Sword", description: "", category: Sword, rarity: "common"),
            ],
        )"#,
        )
        .unwrap_or_else(|e| panic!("{e}"));
        ItemCatalog::from_data([file]).unwrap_or_else(|e| panic!("{e}"))
    }

    fn vitals(hp: ResourcePool, mp: ResourcePool, hearts: ResourcePool) -> Vitals {
        Vitals {
            hp,
            mp,
            hearts,
            hp_regen: RegenAccumulator::default(),
            mp_regen: RegenAccumulator::default(),
        }
    }

    #[test]
    fn restores_hp_and_consumes_one() {
        let cat = catalog();
        let mut inv = Inventory::default();
        inv.add("potion", 2, &cat).unwrap_or_else(|e| panic!("{e}"));
        let mut v = vitals(
            ResourcePool::new(20, 100),
            ResourcePool::full(50),
            ResourcePool::full(3),
        );
        let result = use_consumable(&mut inv, &cat, "potion", PlayerState::Idle, true, &mut v)
            .unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(result.hp_restored, 40);
        assert_eq!(v.hp.current(), 60);
        assert_eq!(inv.count("potion"), 1);
    }

    #[test]
    fn clamps_to_max_and_refuses_when_full() {
        let cat = catalog();
        let mut inv = Inventory::default();
        inv.add("potion", 2, &cat).unwrap_or_else(|e| panic!("{e}"));
        let mut v = vitals(
            ResourcePool::new(90, 100),
            ResourcePool::full(10),
            ResourcePool::full(1),
        );
        let ok = use_consumable(&mut inv, &cat, "potion", PlayerState::Idle, true, &mut v)
            .unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(ok.hp_restored, 10);
        assert_eq!(v.hp.current(), 100);
        assert_eq!(inv.count("potion"), 1);
        assert_eq!(
            use_consumable(&mut inv, &cat, "potion", PlayerState::Idle, true, &mut v),
            Err(UseItemError::NoEffect)
        );
        assert_eq!(inv.count("potion"), 1);
    }

    #[test]
    fn restores_mp() {
        let cat = catalog();
        let mut inv = Inventory::default();
        inv.add("ether", 1, &cat).unwrap_or_else(|e| panic!("{e}"));
        let mut v = vitals(
            ResourcePool::full(10),
            ResourcePool::new(5, 40),
            ResourcePool::full(1),
        );
        let result = use_consumable(&mut inv, &cat, "ether", PlayerState::Walking, false, &mut v)
            .unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(result.mp_restored, 30);
        assert_eq!(v.mp.current(), 35);
        assert_eq!(inv.count("ether"), 0);
    }

    #[test]
    fn refuses_dead_empty_and_non_consumable() {
        let cat = catalog();
        let mut inv = Inventory::default();
        inv.add("potion", 1, &cat).unwrap_or_else(|e| panic!("{e}"));
        inv.add("sword", 1, &cat).unwrap_or_else(|e| panic!("{e}"));
        let mut v = vitals(
            ResourcePool::new(1, 10),
            ResourcePool::full(10),
            ResourcePool::full(1),
        );
        assert_eq!(
            use_consumable(&mut inv, &cat, "potion", PlayerState::Dead, true, &mut v),
            Err(UseItemError::Dead)
        );
        assert_eq!(inv.count("potion"), 1);
        assert_eq!(
            use_consumable(&mut inv, &cat, "sword", PlayerState::Idle, true, &mut v),
            Err(UseItemError::NotConsumable("sword".into()))
        );
        assert_eq!(
            use_consumable(&mut inv, &cat, "ether", PlayerState::Idle, true, &mut v),
            Err(UseItemError::NotOwned("ether".into()))
        );
        assert!(!use_allowed(PlayerState::Attacking, false));
        assert!(use_allowed(PlayerState::Attacking, true));
    }
}
