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
    /// Label in the inventory (the slot's body part, V2.0).
    pub fn display_name(self) -> &'static str {
        match self {
            EquipmentSlot::Weapon => "Weapon",
            EquipmentSlot::Offhand => "Off-hand",
            EquipmentSlot::Head => "Head",
            EquipmentSlot::Body => "Body",
            EquipmentSlot::Gloves => "Arms",
            EquipmentSlot::Boots => "Legs",
            EquipmentSlot::Accessory1 => "Accessory 1",
            EquipmentSlot::Accessory2 => "Accessory 2",
        }
    }

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
    /// Recomputed from the slots every time (never accumulated), so equipping
    /// and removing any number of times cannot leave a bonus behind. An item
    /// in a slot its category does not fit (a hand-edited save) counts for
    /// nothing.
    pub fn total_modifiers(&self, catalog: &ItemCatalog) -> AttributeModifiers {
        AttributeModifiers::sum(
            self.slots
                .iter()
                .filter_map(|(slot, id)| {
                    catalog
                        .get(id)
                        .filter(|def| def.category.slots().contains(slot))
                })
                .map(|def| &def.modifiers),
        )
    }

    /// A copy with `slot` holding `item` (or emptied), without any check:
    /// only for previews (the comparison), never for the real equipment.
    pub fn preview(&self, slot: EquipmentSlot, item: Option<&str>) -> Equipment {
        let mut copy = self.clone();
        match item {
            Some(id) => {
                copy.slots.insert(slot, id.to_owned());
            }
            None => {
                copy.slots.remove(&slot);
            }
        }
        copy
    }

    /// Number of occupied slots.
    pub fn len(&self) -> usize {
        self.slots.len()
    }

    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }
}
