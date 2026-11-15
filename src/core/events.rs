//! Messages exchanged between feature plugins.
//!
//! Plugins communicate through these buffered messages instead of reaching
//! into each other's components, which keeps modules decoupled.

use bevy::prelude::*;

use crate::combat::damage::DamageOutcome;
use crate::enemies::definitions::DropEntry;
use crate::skills::execution::CommandOutcome;

/// A projectile stopped against level geometry and was removed (V1.8).
/// Written by `magic::projectiles::move_projectiles`; read by the world, so
/// a spell that hits a breakable wall can break it (the projectile is gone
/// before the world's systems run, so they cannot see its hitbox).
#[derive(Message, Debug, Clone, Copy)]
pub struct ProjectileBlocked {
    pub center: Vec2,
    pub size: Vec2,
    pub team: crate::combat::hitboxes::Team,
    pub kind: crate::combat::damage::DamageKind,
    pub element: crate::magic::elements::Element,
}

/// A button press was interpreted by the command system (for UI feedback).
#[derive(Message, Debug, Clone)]
pub struct CommandResolved {
    pub outcome: CommandOutcome,
}

/// Request: the recogniser matched a skill for `caster` (validated with the
/// state of the previous frame). Only `begin_skill` reads it; it re-validates
/// with the current state and either confirms ([`SkillStarted`]) or rejects.
#[derive(Message, Debug, Clone)]
pub struct SkillActivated {
    pub caster: Entity,
    pub skill_id: String,
}

/// Confirmed activation: costs paid, cooldown started and caster locked.
/// Effect executors (melee techniques, spells) react only to this message.
#[derive(Message, Debug, Clone)]
pub struct SkillStarted {
    pub caster: Entity,
    pub skill_id: String,
}

/// The attack button was pressed without forming a command.
#[derive(Message, Debug, Clone, Copy)]
pub struct BasicAttackRequested {
    pub attacker: Entity,
}

/// A hitbox connected with a hurtbox; damage already computed.
#[derive(Message, Debug, Clone, Copy)]
pub struct DamageDealt {
    pub attacker: Entity,
    pub target: Entity,
    pub outcome: DamageOutcome,
    /// Horizontal direction of the hit (+1 right, -1 left), for knockback.
    pub direction: f32,
}

/// An enemy died; carries the reward.
#[derive(Message, Debug, Clone)]
pub struct EnemyDefeated {
    pub enemy_id: String,
    pub experience: u64,
    /// Drop table; rolled by the player's side with its LCK.
    pub drops: Vec<DropEntry>,
}

/// Short on-screen message (save done, level up, errors...).
#[derive(Message, Debug, Clone)]
pub struct Notification(pub String);

/// How the player arrives in a room (V1.7 room lifecycle).
#[derive(Debug, Clone, PartialEq)]
pub enum Arrival {
    /// Through this door of the target room (normal transition): the
    /// player appears at the door's validated entry point.
    Door(String),
    /// At an already validated position (quick load): see
    /// `world::restore::resolve_placement`.
    Position(Vec2),
}

/// Request to make `room` the active room. Written by the door system and
/// by the quick load; only `world::transitions::apply_room_requests` reads
/// it, applies at most one per frame (a load wins over a door) and
/// announces the result with [`RoomChanged`].
#[derive(Message, Debug, Clone)]
pub struct RoomRequest {
    pub room: String,
    pub arrival: Arrival,
}

/// Why the active room was (re)loaded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoomChangeCause {
    Door,
    Load,
}

/// The active room was replaced (door transition or quick load). The world
/// module has already queued the despawn of the old room's entities and
/// the spawn of the new geometry; other modules clean up what they own
/// (hitboxes, projectiles, pending enemy respawns, damage numbers) and
/// spawn the new room's enemies, in the same frame (`RoomStep::React`).
#[derive(Message, Debug, Clone)]
pub struct RoomChanged {
    pub from: String,
    pub to: String,
    /// Door of `to` the player arrived through (`None` for a load).
    pub door: Option<String>,
    pub cause: RoomChangeCause,
}

/// A world pickup was collected for the first time; the owner of each
/// grant applies it (an `Item` goes to the player's inventory).
#[derive(Message, Debug, Clone)]
pub struct PickupCollected {
    pub pickup: String,
    pub grant: crate::world::graph::Grant,
}

/// What happened to the player's transformation (V1.9). Written by the
/// transformation module; the HUD and the form visuals read it.
#[derive(Message, Debug, Clone, Copy, PartialEq)]
pub struct FormChanged {
    pub form: crate::transform::definitions::FormId,
    pub change: FormChange,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FormChange {
    /// The form started (activation MP paid).
    Entered,
    /// An end was asked for; the form stays until the human body fits.
    Ending(crate::transform::state::EndReason),
    /// Back to the human form.
    Left(crate::transform::state::EndReason),
    /// The key was pressed but the form could not start.
    Refused(crate::transform::state::ActivateError),
}

/// What an [`EquipRequest`] asks for (V2.0).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EquipOp {
    /// Equip one unit of this item from the bag.
    Equip(String),
    /// Put the item of the slot back in the bag.
    Unequip,
}

/// Request to change one equipment slot (V2.0). Written by the inventory
/// screen; only `inventory::systems::apply_equip_requests` reads it, checks
/// the gate (state, form, passage, live strike) and applies it.
#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct EquipRequest {
    pub slot: crate::inventory::equipment::EquipmentSlot,
    pub op: EquipOp,
}

/// What a [`TradeRequest`] asks for (V2.1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TradeOp {
    Buy(String),
    Sell(String),
}

/// Request to buy or sell one unit in a shop (V2.1). Written by the shop
/// screen; only `economy::systems::apply_trades` reads it.
#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct TradeRequest {
    pub shop: String,
    pub op: TradeOp,
}

/// An equipment change that was applied (V2.0), for the UI and tests.
#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct EquipmentChanged {
    pub change: crate::inventory::loadout::LoadoutChange,
}
