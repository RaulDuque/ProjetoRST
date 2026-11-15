//! Bevy side of the equipment (V2.0).
//!
//! | System | Set | Does |
//! |---|---|---|
//! | [`apply_equip_requests`] | Presentation (also paused) | the only writer of equipment changes |
//! | [`sync_appearance`] | Presentation | keeps [`EquipmentAppearance`] in step with the slots |
//!
//! Equipment changes come only from [`EquipRequest`] messages (the inventory
//! screen writes them). The change recomputes the equipment bonus of the
//! `CharacterSheet` from the slots (never adds to it), clamps HP/MP to the
//! new maxima without restoring them, and refreshes the combat profile in
//! the same frame, so the paused inventory already shows the final numbers.

use std::collections::BTreeSet;

use bevy::prelude::*;

use super::appearance::Appearance;
use super::loadout::{EquipGate, LoadoutError, equip_from_bag, unequip_to_bag};
use crate::app::states::GameSet;
use crate::combat::hitboxes::{AttachedToOwner, CombatProfile, Hitbox};
use crate::core::events::{EquipOp, EquipRequest, EquipmentChanged, Notification};
use crate::core::resources::GameData;
use crate::player::components::{ActionState, Belongings, CharacterSheet, Player, Vitals};
use crate::skills::unlock::RequirementContext;
use crate::transform::systems::Transformation;
use crate::world::WorldState;
use crate::world::transitions::TransitionLock;

/// The resolved look of the player's equipment (presentation reads it).
#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Deref)]
pub struct EquipmentAppearance(pub Appearance);

/// The gate of the player right now (shared by the request system and the
/// inventory screen, so the screen shows exactly what will be refused).
pub fn equip_gate(
    player: Entity,
    action: &ActionState,
    form: Option<&Transformation>,
    lock: Option<&TransitionLock>,
    strikes: &Query<&Hitbox, With<AttachedToOwner>>,
) -> EquipGate {
    EquipGate {
        state: action.state(),
        transformed: form.is_some_and(|f| f.is_active() || f.ending().is_some()),
        in_transition: lock.is_some_and(|l| l.is_locked()),
        strike_active: strikes.iter().any(|h| h.owner == player),
    }
}

/// Text of a refused change, for the HUD.
pub fn refusal_text(error: &LoadoutError, data: &GameData) -> String {
    let name = |id: &str| data.items.get(id).map_or(id.to_owned(), |d| d.name.clone());
    match error {
        LoadoutError::Blocked(block) => format!("Cannot change equipment: {block}"),
        LoadoutError::Unknown(id) => format!("Unknown item {id}"),
        LoadoutError::NotOwned(id) => format!("{} is not in the bag", name(id)),
        LoadoutError::NotEquipment(id) => format!("{} cannot be equipped", name(id)),
        LoadoutError::Equip(error) => match error {
            super::equipment::EquipError::WrongSlot { item, slot } => {
                format!("{} does not fit {}", name(item), slot.display_name())
            }
            super::equipment::EquipError::Requirement(req) => {
                format!("Requirement not met: {req}")
            }
        },
        LoadoutError::EmptySlot(slot) => format!("{} is empty", slot.display_name()),
        LoadoutError::UnknownEquipped(slot) => {
            format!("The item in {} is unknown and stays", slot.display_name())
        }
        LoadoutError::OffhandInUse => "Two-handed: remove the off-hand item first".to_owned(),
        LoadoutError::TwoHandedWeapon => "The weapon needs both hands: remove it first".to_owned(),
    }
}

type EquipQuery<'a> = (
    Entity,
    &'a mut Belongings,
    &'a mut CharacterSheet,
    &'a mut Vitals,
    &'a mut CombatProfile,
    &'a ActionState,
    Option<&'a Transformation>,
);

/// Applies equipment requests, in order, with the rules of
/// [`crate::inventory::loadout`].
pub fn apply_equip_requests(
    data: Res<GameData>,
    world: Option<Res<WorldState>>,
    lock: Option<Res<TransitionLock>>,
    mut requests: MessageReader<EquipRequest>,
    mut player: Query<EquipQuery, With<Player>>,
    strikes: Query<&Hitbox, With<AttachedToOwner>>,
    mut changed: MessageWriter<EquipmentChanged>,
    mut notifications: MessageWriter<Notification>,
) {
    if requests.is_empty() {
        return;
    }
    let Ok((entity, mut belongings, mut sheet, mut vitals, mut profile, action, form)) =
        player.single_mut()
    else {
        requests.clear();
        return;
    };
    let empty = BTreeSet::new();
    let flags = world.as_ref().map_or(&empty, |w| &w.flags);
    for request in requests.read() {
        let gate = equip_gate(entity, action, form, lock.as_deref(), &strikes);
        let owned: BTreeSet<String> = belongings
            .inventory
            .stacks()
            .iter()
            .map(|s| s.id.clone())
            .collect();
        let base = *sheet.base();
        let ctx = RequirementContext {
            level: sheet.progression().level(),
            attributes: &base,
            owned_items: &owned,
            progress_flags: flags,
        };
        let Belongings {
            inventory,
            equipment,
        } = &mut *belongings;
        let result = match &request.op {
            EquipOp::Equip(item) => equip_from_bag(
                inventory,
                equipment,
                request.slot,
                item,
                &data.items,
                &ctx,
                &gate,
            ),
            EquipOp::Unequip => {
                unequip_to_bag(inventory, equipment, request.slot, &data.items, &gate)
            }
        };
        match result {
            Ok(change) => {
                let bonus = belongings.equipment.total_modifiers(&data.items);
                sheet.set_equipment_bonus(bonus, &data.character.formulas);
                vitals.sync_max(sheet.derived());
                *profile = CombatProfile::from_derived(sheet.derived());
                changed.write(EquipmentChanged { change });
            }
            Err(error) => {
                notifications.write(Notification(refusal_text(&error, &data)));
            }
        }
    }
}

/// Resolves the look whenever the belongings change (equip, pickup, load).
pub fn sync_appearance(
    mut commands: Commands,
    data: Res<GameData>,
    players: Query<(Entity, Ref<Belongings>, Option<&EquipmentAppearance>), With<Player>>,
) {
    for (entity, belongings, current) in &players {
        if current.is_some() && !belongings.is_changed() {
            continue;
        }
        let look = EquipmentAppearance(Appearance::resolve(&belongings.equipment, &data.items));
        if current != Some(&look) {
            commands.entity(entity).insert(look);
        }
    }
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (apply_equip_requests, sync_appearance)
                .chain()
                .in_set(GameSet::Presentation)
                .before(crate::player::visual::PlayerVisualSystems),
        )
        .add_systems(Startup, super::visual::create_cape_image)
        .add_systems(
            Update,
            super::visual::sync_cape_layer
                .in_set(crate::player::visual::PlayerVisualSystems)
                .after(crate::player::visual::systems::animate_player_visual),
        );
    }
}
