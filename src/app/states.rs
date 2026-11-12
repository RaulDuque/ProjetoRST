//! Global game states and the ordered system sets of a gameplay frame.

use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum GameState {
    /// The simulation runs.
    #[default]
    Playing,
    /// Status screen open; the simulation is paused.
    StatusScreen,
}

/// Order of one gameplay frame. Every set except `Presentation` only runs
/// in [`GameState::Playing`].
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
    /// EXP, level ups, unlocks, stat recomputation.
    Progression,
    /// Visual feedback, camera and UI (also runs while paused).
    Presentation,
}
