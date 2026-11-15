//! Items, inventory and equipment.
//!
//! V2.0: weapon classes and strike profiles ([`weapons`]), appearance
//! ([`appearance`]), bag <-> slot moves ([`loadout`]) and comparison
//! ([`compare`]). All pure; the Bevy side is `systems`.

pub mod appearance;
pub mod compare;
pub mod equipment;
#[allow(clippy::module_inception)]
pub mod inventory;
pub mod items;
pub mod loadout;
pub mod systems;
pub mod visual;
pub mod weapons;
