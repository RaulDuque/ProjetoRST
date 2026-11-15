//! Coins, shops and trades (V2.1).
//!
//! - [`shop`] (pure): purse, shop definitions, stock, buy and sell rules;
//! - [`systems`] (Bevy): the player's purse, trade requests, load reset.
//!   The screen is `ui::shop_screen`.
//!
//! Details and the pending save decision: docs/NPC_COMMERCE_V2_1.md.

pub mod shop;
pub mod systems;
