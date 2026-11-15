//! Visual effects of the player's skills, guard and revive (V1.5).
//!
//! - [`rules`] (pure): which clip, how long, when it stops;
//! - [`systems`] (Bevy): spawning effects from existing gameplay signals and
//!   playing them once.
//!
//! Effects are presentation only. They observe `SkillStarted`,
//! `DamageDealt` and the player's `ActionState`; they never deal damage,
//! heal, move or collide, and no gameplay system reads them. Their frames
//! are the `effects` atlas of `assets/animations/player.ron`; without it
//! (headless apps, a manifest without effects) nothing is spawned.
//! Details: docs/PLAYER_ART_POLISH_V1_5.md.

pub mod rules;
pub mod systems;

use bevy::prelude::*;

/// A running effect: a child sprite of the player.
#[derive(Component, Debug, Clone, PartialEq)]
pub struct PlayerVfx {
    /// Manifest key of the effect clip (`vfx_<key>`).
    pub clip: String,
    /// Seconds since it started.
    pub elapsed: f32,
    /// Playback stretch so it lasts the real action (see `rules::time_scale`).
    pub time_scale: f32,
    /// Removed as soon as the caster stops performing the technique.
    pub follows_action: bool,
}
