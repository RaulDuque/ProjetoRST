//! Bevy side of the economy (V2.1).
//!
//! | System | Set | Does |
//! |---|---|---|
//! | [`attach_purse`] | Presentation | gives the player its [`PlayerPurse`] (starting coins) |
//! | [`apply_trades`] | Presentation (also paused) | the only writer of coins and shop stock |
//!
//! **Persistence (save v3, V2.1.1):** `save::systems::quick_save` writes the
//! coins and the shop stock into the same file as the bag
//! (`SaveData.economy`), and `quick_load` restores both. A purchase and its
//! payment are therefore always saved or loaded together: no duplication,
//! no coin loop across F5/F9. A v1/v2 save has no economy and loads with
//! the starting coins and full stock. Details: docs/NPC_COMMERCE_V2_1.md §10.

use bevy::prelude::*;

use super::shop::{Purse, ShopStock, buy, sell};
use crate::app::states::GameSet;
use crate::core::events::{Notification, TradeOp, TradeRequest};
use crate::core::resources::GameData;
use crate::npc::systems::dialogue_facts;
use crate::player::components::{Belongings, Player};
use crate::world::exploration_systems::{Exploration, WorldGraph};

/// The player's coins.
#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq, Deref, DerefMut)]
pub struct PlayerPurse(pub Purse);

/// Units sold by the shops' limited entries.
#[derive(Resource, Debug, Clone, Default, PartialEq, Eq, Deref, DerefMut)]
pub struct ShopStockState(pub ShopStock);

pub fn attach_purse(
    mut commands: Commands,
    data: Res<GameData>,
    players: Query<Entity, (With<Player>, Without<PlayerPurse>)>,
) {
    for player in &players {
        commands
            .entity(player)
            .insert(PlayerPurse(Purse::new(data.character.start.coins)));
    }
}

/// Applies buy and sell requests in order. A refused trade changes nothing.
pub fn apply_trades(
    data: Res<GameData>,
    world: Option<Res<WorldGraph>>,
    exploration: Option<Res<Exploration>>,
    mut requests: MessageReader<TradeRequest>,
    mut stock: ResMut<ShopStockState>,
    mut player: Query<(&mut PlayerPurse, &mut Belongings), With<Player>>,
    mut notifications: MessageWriter<Notification>,
) {
    if requests.is_empty() {
        return;
    }
    let Ok((mut purse, mut belongings)) = player.single_mut() else {
        requests.clear();
        return;
    };
    for request in requests.read() {
        let Some(shop) = data.shops.get(&request.shop) else {
            continue;
        };
        let facts = dialogue_facts(exploration.as_deref(), world.as_deref(), Some(&belongings));
        let name = |id: &str| data.items.get(id).map_or(id.to_owned(), |d| d.name.clone());
        let Belongings {
            inventory,
            equipment,
        } = &mut *belongings;
        let result = match &request.op {
            TradeOp::Buy(item) => buy(
                shop,
                item,
                &mut stock.0,
                &mut purse.0,
                inventory,
                &data.items,
                &facts,
            )
            .map(|price| format!("Bought {} for {price}", name(item))),
            TradeOp::Sell(item) => {
                sell(shop, item, &mut purse.0, inventory, equipment, &data.items)
                    .map(|value| format!("Sold {} for {value}", name(item)))
            }
        };
        match result {
            Ok(text) => {
                notifications.write(Notification(text));
            }
            Err(error) => {
                notifications.write(Notification(format!("Cannot trade: {error}")));
            }
        }
    }
}

pub struct EconomyPlugin;

impl Plugin for EconomyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShopStockState>().add_systems(
            Update,
            (attach_purse, apply_trades)
                .chain()
                .in_set(GameSet::Presentation),
        );
    }
}
