//! Equipment slots and equipped items. Pure module: no Bevy types.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::items::{ItemCatalog, ItemDefinition, ItemId};
use crate::player::attributes::AttributeModifiers;
use crate::skills::unlock::{RequirementContext, RequirementError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EquipmentSlot {
    Weapon,
    Offhand,
    Head,
    Body,
    Gloves,
    Boots,
    Accessory1,
    Accessory2,
}

impl EquipmentSlot {
    pub const ALL: [EquipmentSlot; 8] = [
        EquipmentSlot::Weapon,
        EquipmentSlot::Offhand,
        EquipmentSlot::Head,
        EquipmentSlot::Body,
        EquipmentSlot::Gloves,
        EquipmentSlot::Boots,
        EquipmentSlot::Accessory1,
        EquipmentSlot::Accessory2,
    ];
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum EquipError {
    #[error("`{item}` cannot be equipped in {slot:?}")]
    WrongSlot { item: ItemId, slot: EquipmentSlot },
    #[error(transparent)]
    Requirement(#[from] RequirementError),
}

/// Items currently equipped, one per slot.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Equipment {
    slots: BTreeMap<EquipmentSlot, ItemId>,
}

impl Equipment {
    pub fn get(&self, slot: EquipmentSlot) -> Option<&str> {
        self.slots.get(&slot).map(String::as_str)
    }

    pub fn iter(&self) -> impl Iterator<Item = (EquipmentSlot, &str)> {
        self.slots.iter().map(|(s, id)| (*s, id.as_str()))
    }

    /// Equips `item` in `slot`, returning the item previously there.
    pub fn equip(
        &mut self,
        slot: EquipmentSlot,
        item: &ItemDefinition,
        ctx: &RequirementContext<'_>,
    ) -> Result<Option<ItemId>, EquipError> {
        if !item.category.slots().contains(&slot) {
            return Err(EquipError::WrongSlot {
                item: item.id.clone(),
                slot,
            });
        }
        item.requirements.check(ctx)?;
        Ok(self.slots.insert(slot, item.id.clone()))
    }

    pub fn unequip(&mut self, slot: EquipmentSlot) -> Option<ItemId> {
        self.slots.remove(&slot)
    }

    /// Sum of the modifiers of every equipped item known to the catalog.
    pub fn total_modifiers(&self, catalog: &ItemCatalog) -> AttributeModifiers {
        AttributeModifiers::sum(
            self.slots
                .values()
                .filter_map(|id| catalog.get(id))
                .map(|def| &def.modifiers),
        )
    }
}
