//! The bag of items carried by the character. Pure module: no Bevy types.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::items::{ItemCatalog, ItemCategory, ItemId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemStack {
    pub id: ItemId,
    pub quantity: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum InventoryError {
    #[error("unknown item `{0}`")]
    UnknownItem(String),
    #[error("not enough `{0}`")]
    NotEnough(String),
    #[error("quest items cannot be discarded")]
    CannotDiscard,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Inventory {
    stacks: Vec<ItemStack>,
}

impl Inventory {
    pub fn stacks(&self) -> &[ItemStack] {
        &self.stacks
    }

    pub fn count(&self, id: &str) -> u32 {
        self.stacks
            .iter()
            .filter(|s| s.id == id)
            .map(|s| s.quantity)
            .sum()
    }

    /// Adds items, filling existing stacks up to `max_stack` first.
    pub fn add(
        &mut self,
        id: &str,
        quantity: u32,
        catalog: &ItemCatalog,
    ) -> Result<(), InventoryError> {
        let def = catalog
            .get(id)
            .ok_or_else(|| InventoryError::UnknownItem(id.to_owned()))?;
        let max = def.max_stack.max(1);
        let mut left = quantity;
        for stack in self.stacks.iter_mut().filter(|s| s.id == id) {
            let room = max.saturating_sub(stack.quantity);
            let moved = room.min(left);
            stack.quantity += moved;
            left -= moved;
        }
        while left > 0 {
            let moved = left.min(max);
            self.stacks.push(ItemStack {
                id: id.to_owned(),
                quantity: moved,
            });
            left -= moved;
        }
        Ok(())
    }

    /// Removes `quantity` items (used when consuming or equipping).
    pub fn remove(&mut self, id: &str, quantity: u32) -> Result<(), InventoryError> {
        if self.count(id) < quantity {
            return Err(InventoryError::NotEnough(id.to_owned()));
        }
        let mut left = quantity;
        for stack in self.stacks.iter_mut().rev().filter(|s| s.id == id) {
            let taken = stack.quantity.min(left);
            stack.quantity -= taken;
            left -= taken;
        }
        self.stacks.retain(|s| s.quantity > 0);
        Ok(())
    }

    /// Like [`Inventory::remove`] but refuses quest items.
    pub fn discard(
        &mut self,
        id: &str,
        quantity: u32,
        catalog: &ItemCatalog,
    ) -> Result<(), InventoryError> {
        let def = catalog
            .get(id)
            .ok_or_else(|| InventoryError::UnknownItem(id.to_owned()))?;
        if def.category == ItemCategory::QuestItem {
            return Err(InventoryError::CannotDiscard);
        }
        self.remove(id, quantity)
    }

    /// Sorts by category, then rarity tier (highest first), then name.
    pub fn sort(&mut self, catalog: &ItemCatalog) {
        self.stacks.sort_by_key(|s| {
            let def = catalog.get(&s.id);
            let category = def.map(|d| d.category);
            let tier = def
                .and_then(|d| catalog.rarity(&d.rarity))
                .map_or(0, |r| r.tier);
            let name = def.map(|d| d.name.clone()).unwrap_or_default();
            (category, std::cmp::Reverse(tier), name)
        });
    }
}
