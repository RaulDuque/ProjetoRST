//! Cross-cutting building blocks shared by every feature module.

pub mod constants;
pub mod events;
pub mod pixel_art;
pub mod pixel_image;
pub mod resources;
pub mod rng;
pub mod time;

use bevy::prelude::*;

use constants::RNG_SEED;
use events::{
    BasicAttackRequested, CommandResolved, DamageDealt, EnemyDefeated, FormChanged, Notification,
    PickupCollected, ProjectileBlocked, RoomChanged, RoomRequest, SavePointActivated,
    SkillActivated, SkillStarted,
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
            .add_message::<SkillStarted>()
            .add_message::<BasicAttackRequested>()
            .add_message::<DamageDealt>()
            .add_message::<EnemyDefeated>()
            .add_message::<Notification>()
            .add_message::<RoomChanged>()
            .add_message::<RoomRequest>()
            .add_message::<PickupCollected>()
            .add_message::<SavePointActivated>()
            .add_message::<ProjectileBlocked>()
            .add_message::<FormChanged>()
            .add_message::<events::EquipRequest>()
            .add_message::<events::EquipmentChanged>()
            .add_message::<events::UseItemRequest>()
            .add_message::<events::ItemUsed>()
            .add_message::<events::TradeRequest>();
    }
}
