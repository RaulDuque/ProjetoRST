//! Moving items between the bag and the equipment slots (V2.0).
//!
//! Pure module: no Bevy types. The contract of the V1.x data is kept: an
//! equipped item is *not* in the bag (`starting_character` equips the
//! sword and the jerkin without adding them to the inventory). So:
//!
//! - equipping takes one unit out of the bag and puts it in the slot; the
//!   item that was there goes back to the bag;
//! - removing puts the item back in the bag.
//!
//! Every check happens before anything changes: a refused operation leaves
//! bag and slots exactly as they were (no item is lost or duplicated).
//! Whether the character may change equipment at all is [`EquipGate`].

use thiserror::Error;

use super::equipment::{EquipError, Equipment, EquipmentSlot};
use super::inventory::Inventory;
use super::items::{ItemCatalog, ItemId};
use crate::player::state::PlayerState;
use crate::skills::unlock::RequirementContext;

/// Why equipment cannot change right now.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum EquipBlock {
    #[error("not while fallen")]
    Dead,
    #[error("not in the middle of an action")]
    Busy(PlayerState),
    #[error("not while transformed")]
    Transformed,
    #[error("not during a passage")]
    InTransition,
}

/// The parts of the player that decide whether equipment may change.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EquipGate {
    pub state: PlayerState,
    /// A transformation is active (or still ending).
    pub transformed: bool,
    /// A room transition is in progress.
    pub in_transition: bool,
    /// A hitbox of the player's own melee strike still exists.
    pub strike_active: bool,
}

impl EquipGate {
    /// Equipment changes only out of actions: never fallen, attacking,
    /// casting, dodging, hurt or guarding, with a live strike, in a
    /// transformation or during a room change.
    pub fn check(&self) -> Result<(), EquipBlock> {
        if self.state == PlayerState::Dead {
            return Err(EquipBlock::Dead);
        }
        if self.transformed {
            return Err(EquipBlock::Transformed);
        }
        if self.in_transition {
            return Err(EquipBlock::InTransition);
        }
        if !self.state.is_locomotion() || self.strike_active {
            return Err(EquipBlock::Busy(self.state));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum LoadoutError {
    #[error(transparent)]
    Blocked(#[from] EquipBlock),
    #[error("unknown item `{0}`")]
    Unknown(String),
    #[error("`{0}` is not in the bag")]
    NotOwned(String),
    #[error("`{0}` is not equipment")]
    NotEquipment(String),
    #[error(transparent)]
    Equip(#[from] EquipError),
    #[error("nothing equipped in {0:?}")]
    EmptySlot(EquipmentSlot),
    /// The slot holds an item this game does not know (an old save): it is
    /// kept instead of being dropped by an exchange.
    #[error("the item in {0:?} is unknown and cannot be moved")]
    UnknownEquipped(EquipmentSlot),
    /// V2.1: a two-handed weapon and an off-hand item exclude each other.
    #[error("a two-handed weapon cannot be used with an off-hand item (remove the off-hand first)")]
    OffhandInUse,
    #[error("the off-hand is held by a two-handed weapon (remove the weapon first)")]
    TwoHandedWeapon,
}

/// V2.1: refuses a two-handed weapon while the off-hand holds something, and
/// an off-hand item while a two-handed weapon is equipped. Nothing is
/// unequipped automatically: the player chooses what to remove.
pub fn check_hands(
    slot: EquipmentSlot,
    item: &str,
    equipment: &Equipment,
    catalog: &ItemCatalog,
) -> Result<(), LoadoutError> {
    match slot {
        EquipmentSlot::Weapon
            if catalog.is_two_handed(item) && equipment.get(EquipmentSlot::Offhand).is_some() =>
        {
            Err(LoadoutError::OffhandInUse)
        }
        EquipmentSlot::Offhand
            if equipment
                .get(EquipmentSlot::Weapon)
                .is_some_and(|w| catalog.is_two_handed(w)) =>
        {
            Err(LoadoutError::TwoHandedWeapon)
        }
        _ => Ok(()),
    }
}

/// What an operation did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadoutChange {
    pub slot: EquipmentSlot,
    /// Now in the slot (`None` after a removal).
    pub equipped: Option<ItemId>,
    /// Back in the bag.
    pub returned: Option<ItemId>,
}

/// The slot an item goes to: the first free slot of its category, or the
/// first one (accessories fill `Accessory1`, then `Accessory2`, then
/// replace `Accessory1`).
pub fn target_slot(
    item: &str,
    equipment: &Equipment,
    catalog: &ItemCatalog,
) -> Option<EquipmentSlot> {
    let slots = catalog.get(item)?.category.slots();
    slots
        .iter()
        .copied()
        .find(|s| equipment.get(*s).is_none())
        .or_else(|| slots.first().copied())
}

fn returnable(
    equipment: &Equipment,
    slot: EquipmentSlot,
    catalog: &ItemCatalog,
) -> Result<Option<ItemId>, LoadoutError> {
    match equipment.get(slot) {
        None => Ok(None),
        Some(id) if catalog.get(id).is_some() => Ok(Some(id.to_owned())),
        Some(_) => Err(LoadoutError::UnknownEquipped(slot)),
    }
}

/// Equips one `item` from the bag in `slot`.
pub fn equip_from_bag(
    bag: &mut Inventory,
    equipment: &mut Equipment,
    slot: EquipmentSlot,
    item: &str,
    catalog: &ItemCatalog,
    requirements: &RequirementContext<'_>,
    gate: &EquipGate,
) -> Result<LoadoutChange, LoadoutError> {
    gate.check()?;
    let def = catalog
        .get(item)
        .ok_or_else(|| LoadoutError::Unknown(item.to_owned()))?;
    if !def.category.is_equipment() {
        return Err(LoadoutError::NotEquipment(item.to_owned()));
    }
    if bag.count(item) == 0 {
        return Err(LoadoutError::NotOwned(item.to_owned()));
    }
    let previous = returnable(equipment, slot, catalog)?;
    check_hands(slot, item, equipment, catalog)?;
    // Validates slot and requirements; on error nothing was inserted.
    equipment.equip(slot, def, requirements)?;
    bag.remove(item, 1)
        .map_err(|_| LoadoutError::NotOwned(item.to_owned()))?;
    if let Some(previous) = &previous {
        // Known to the catalog (checked above), so this cannot fail.
        bag.add(previous, 1, catalog)
            .map_err(|_| LoadoutError::Unknown(previous.clone()))?;
    }
    Ok(LoadoutChange {
        slot,
        equipped: Some(item.to_owned()),
        returned: previous,
    })
}

/// Removes the item of `slot` and puts it back in the bag.
pub fn unequip_to_bag(
    bag: &mut Inventory,
    equipment: &mut Equipment,
    slot: EquipmentSlot,
    catalog: &ItemCatalog,
    gate: &EquipGate,
) -> Result<LoadoutChange, LoadoutError> {
    gate.check()?;
    let Some(item) = returnable(equipment, slot, catalog)? else {
        return Err(LoadoutError::EmptySlot(slot));
    };
    bag.add(&item, 1, catalog)
        .map_err(|_| LoadoutError::Unknown(item.clone()))?;
    equipment.unequip(slot);
    Ok(LoadoutChange {
        slot,
        equipped: None,
        returned: Some(item),
    })
}
