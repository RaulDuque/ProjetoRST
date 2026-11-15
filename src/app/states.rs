//! Global game states and the ordered system sets of a gameplay frame.

use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum GameState {
    /// The simulation runs.
    #[default]
    Playing,
    /// Status screen open; the simulation is paused.
    StatusScreen,
    /// Equipment/inventory screen open (V2.0); the simulation is paused.
    Inventory,
    /// Talking to an NPC (V2.1); the simulation is paused.
    Dialogue,
    /// A shop is open (V2.1); the simulation is paused.
    Shop,
}

/// Order of one gameplay frame. Every set except `Rooms` and `Presentation`
/// only runs in [`GameState::Playing`].
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameSet {
    /// Keyboard/gamepad → `PlayerInput`.
    Input,
    /// Command buffer recording and recognition.
    Commands,
    /// Player actions, skill execution and enemy decisions.
    Actions,
    /// Gravity, movement and collision.
    Physics,
    /// Hitboxes and projectiles produce damage.
    Combat,
    /// Damage is applied; deaths and rewards.
    Resolution,
    /// Room lifecycle (V1.7): load requests, the room change itself and
    /// the owners' clean-up, in that order (`world::transitions::RoomStep`).
    /// Runs between `Resolution` and `Progression` and, unlike the gameplay
    /// sets, also while paused (a quick load from the status screen), so a
    /// room is never half replaced when the next simulation frame starts.
    Rooms,
    /// EXP, level ups, unlocks, stat recomputation.
    Progression,
    /// Visual feedback, camera and UI (also runs while paused).
    Presentation,
}
