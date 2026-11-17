//! Items, inventory and equipment.
//!
//! V2.0: weapon classes and strike profiles ([`weapons`]), appearance
//! ([`appearance`]), bag <-> slot moves ([`loadout`]) and comparison
//! ([`compare`]). All pure; the Bevy side is `systems`.
//!
//! V2.2: held equipment ([`held`], pure), its procedural pixel art
//! ([`held_art`], pure) and the weapon/shield layers ([`held_visual`], Bevy).

pub mod appearance;
pub mod compare;
pub mod consume;
pub mod equipment;
pub mod held;
pub mod held_art;
pub mod held_visual;
#[allow(clippy::module_inception)]
pub mod inventory;
pub mod items;
pub mod loadout;
pub mod systems;
pub mod visual;
pub mod weapons;
