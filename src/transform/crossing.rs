//! Can a form afford to cross a veil? (V1.9). Pure module: no Bevy types.
//!
//! The validator only knows that a veil needs the form's unlock. Whether
//! the player can *effectively* cross it depends on the veil's size and on
//! the form's speed and costs: [`crossing_seconds`] measures the crossing
//! (from standing in front of the veil until the door is taken) and
//! `app::load_world` refuses a world whose veils cost more MP than the
//! starting character has.

use super::definitions::TransformationDefinition;
use crate::core::constants::{GRAVITY, PLAYER_SIZE};
use crate::world::graph::{DOOR_TRIGGER_DEPTH, Side, Veil, WALL, WorldRoom};

/// Seconds the form needs to go through `veil` of `room`, from standing in
/// front of it to the door's trigger. `None` for a veil without a door or
/// on a ceiling door.
pub fn crossing_seconds(
    def: &TransformationDefinition,
    room: &WorldRoom,
    veil: &Veil,
) -> Option<f32> {
    let door = room.door(&veil.door)?;
    match door.side {
        Side::Left | Side::Right => {
            let start = WALL + veil.depth.max(0.0) + PLAYER_SIZE.0 / 2.0;
            Some((start - DOOR_TRIGGER_DEPTH).max(0.0) / def.movement.speed)
        }
        Side::Bottom => {
            let start = WALL + PLAYER_SIZE.1 / 2.0;
            Some(def.fall_time((start - DOOR_TRIGGER_DEPTH).max(0.0), GRAVITY))
        }
        Side::Top => None,
    }
}

/// MP needed to enter the form in front of the veil and cross it.
pub fn crossing_mp(def: &TransformationDefinition, room: &WorldRoom, veil: &Veil) -> Option<f32> {
    crossing_seconds(def, room, veil).map(|t| def.mp_for(t))
}
