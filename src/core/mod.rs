//! Cross-cutting building blocks shared by every feature module.

pub mod constants;
pub mod events;
pub mod resources;
pub mod rng;
pub mod time;

use bevy::prelude::*;

use constants::RNG_SEED;
use events::{
    BasicAttackRequested, CommandResolved, DamageDealt, EnemyDefeated, Notification, SkillActivated,
};
use resources::GameRandom;
use rng::GameRng;

/// Registers shared messages and resources.
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameRandom(GameRng::seeded(RNG_SEED)))
            .add_message::<CommandResolved>()
            .add_message::<SkillActivated>()
            .add_message::<BasicAttackRequested>()
            .add_message::<DamageDealt>()
            .add_message::<EnemyDefeated>()
            .add_message::<Notification>();
    }
}
