//! Money and shops (V2.1). Pure module: no Bevy types.
//!
//! - [`Purse`]: the player's coins ("marks"), a `u32` that never goes below 0
//!   or above [`MAX_COINS`];
//! - [`ShopDefinition`] (`assets/data/shops/*.ron`): what a shop sells, at
//!   which price, with unlimited or limited stock;
//! - [`ShopStock`]: units left of limited entries;
//! - [`buy`] / [`sell`]: every check first, then every change, so a refused
//!   trade changes nothing (no item, coin or unit is lost or created).
//!
//! Prices: a shop sells at `price` (data) and buys at the item's
//! `sell_value` (catalogue). Loading refuses `price <= sell_value`, so buying
//! and selling the same item always loses coins. Details:
//! docs/NPC_COMMERCE_V2_1.md.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::inventory::equipment::Equipment;
use crate::inventory::inventory::Inventory;
use crate::inventory::items::{ItemCatalog, ItemCategory};
use crate::npc::definitions::{DialogueCondition, DialogueFacts, all_hold};

/// Most coins the player can carry.
pub const MAX_COINS: u32 = 999_999;
/// Highest price a shop may ask.
pub const MAX_PRICE: u32 = 99_999;

/// The player's coins.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Purse(u32);

impl Purse {
    pub fn new(coins: u32) -> Self {
        Self(coins.min(MAX_COINS))
    }

    pub fn coins(self) -> u32 {
        self.0
    }

    /// Adds coins if the result stays within [`MAX_COINS`].
    pub fn checked_add(self, amount: u32) -> Option<Purse> {
        self.0
            .checked_add(amount)
            .filter(|c| *c <= MAX_COINS)
            .map(Purse)
    }

    /// Removes coins if there are enough.
    pub fn checked_spend(self, amount: u32) -> Option<Purse> {
        self.0.checked_sub(amount).map(Purse)
    }
}

/// Stock of one entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Stock {
    Unlimited,
    /// Units for sale; once bought they are gone.
    Limited(u32),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShopEntry {
    /// Item id of the catalogue (`assets/data/items/`).
    pub item: String,
    /// Price in coins (> the item's `sell_value`).
    pub price: u32,
    pub stock: Stock,
    /// Shown and sold only when these hold.
    #[serde(default)]
    pub when: Vec<DialogueCondition>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShopDefinition {
    /// Unique in the game.
    pub id: String,
    pub name: String,
    /// NPC who runs it (its `shop` must name this shop).
    pub keeper: String,
    pub entries: Vec<ShopEntry>,
    /// Whether the shop buys the player's items (at their `sell_value`).
    #[serde(default)]
    pub buys: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ShopError {
    #[error("duplicate shop `{0}`")]
    DuplicateId(String),
    #[error("shop `{shop}`: {problem}")]
    Invalid { shop: String, problem: String },
}

/// Every shop, validated against the catalogue.
#[derive(Debug, Clone, Default)]
pub struct ShopLibrary {
    shops: BTreeMap<String, ShopDefinition>,
}

impl ShopLibrary {
    /// Unique ids; every item exists and is not a key item; `0 < price <=
    /// MAX_PRICE`; `price > sell_value` (no coin loop); no `Limited(0)`; an
    /// item appears once per shop.
    pub fn new(
        shops: impl IntoIterator<Item = ShopDefinition>,
        catalog: &ItemCatalog,
    ) -> Result<Self, ShopError> {
        let mut library = ShopLibrary::default();
        for shop in shops {
            let invalid = |problem: String| ShopError::Invalid {
                shop: shop.id.clone(),
                problem,
            };
            if library.shops.contains_key(&shop.id) {
                return Err(ShopError::DuplicateId(shop.id));
            }
            if shop.entries.is_empty() {
                return Err(invalid("no entries".into()));
            }
            for (i, entry) in shop.entries.iter().enumerate() {
                let Some(def) = catalog.get(&entry.item) else {
                    return Err(invalid(format!("unknown item `{}`", entry.item)));
                };
                if def.category == ItemCategory::QuestItem {
                    return Err(invalid(format!("`{}` is a key item", entry.item)));
                }
                if entry.price == 0 || entry.price > MAX_PRICE {
                    return Err(invalid(format!("`{}`: price out of range", entry.item)));
                }
                if entry.price <= def.sell_value {
                    return Err(invalid(format!(
                        "`{}`: price {} must be above its sell value {}",
                        entry.item, entry.price, def.sell_value
                    )));
                }
                if entry.stock == Stock::Limited(0) {
                    return Err(invalid(format!("`{}`: Limited(0)", entry.item)));
                }
                if shop.entries[..i].iter().any(|e| e.item == entry.item) {
                    return Err(invalid(format!("`{}` listed twice", entry.item)));
                }
            }
            library.shops.insert(shop.id.clone(), shop);
        }
        Ok(library)
    }

    pub fn get(&self, id: &str) -> Option<&ShopDefinition> {
        self.shops.get(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &ShopDefinition> {
        self.shops.values()
    }
}

/// The economy as stored in the save (v3, V2.1.1): the coins and the units
/// sold by the shops' limited entries. Only stable ids (shop and item ids).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EconomyRecord {
    pub coins: u32,
    pub stock: ShopStock,
}

/// Units left of the limited entries, per shop and item.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShopStock {
    sold: BTreeMap<String, BTreeMap<String, u32>>,
}

impl ShopStock {
    /// Units still for sale (`None` = unlimited).
    pub fn left(&self, shop: &ShopDefinition, entry: &ShopEntry) -> Option<u32> {
        match entry.stock {
            Stock::Unlimited => None,
            Stock::Limited(total) => {
                let sold = self
                    .sold
                    .get(&shop.id)
                    .and_then(|s| s.get(&entry.item))
                    .copied()
                    .unwrap_or(0);
                Some(total.saturating_sub(sold))
            }
        }
    }

    fn record_sale(&mut self, shop: &str, item: &str) {
        let count = self
            .sold
            .entry(shop.to_owned())
            .or_default()
            .entry(item.to_owned())
            .or_insert(0);
        *count = count.saturating_add(1);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum TradeError {
    #[error("not sold here")]
    NotForSale,
    #[error("sold out")]
    SoldOut,
    #[error("not enough coins ({have}/{need})")]
    NotEnoughCoins { have: u32, need: u32 },
    #[error("this shop does not buy")]
    DoesNotBuy,
    #[error("not in the bag (remove equipped items first)")]
    NotOwned,
    #[error("cannot be sold")]
    NotSellable,
    #[error("your last weapon cannot be sold")]
    LastWeapon,
    #[error("purse full")]
    PurseFull,
    #[error("unknown item `{0}`")]
    UnknownItem(String),
}

/// Buys one unit of `item` from `shop`.
pub fn buy(
    shop: &ShopDefinition,
    item: &str,
    stock: &mut ShopStock,
    purse: &mut Purse,
    bag: &mut Inventory,
    catalog: &ItemCatalog,
    facts: &DialogueFacts,
) -> Result<u32, TradeError> {
    let entry = shop
        .entries
        .iter()
        .find(|e| e.item == item && all_hold(&e.when, facts))
        .ok_or(TradeError::NotForSale)?;
    if stock.left(shop, entry) == Some(0) {
        return Err(TradeError::SoldOut);
    }
    let paid = purse
        .checked_spend(entry.price)
        .ok_or(TradeError::NotEnoughCoins {
            have: purse.coins(),
            need: entry.price,
        })?;
    // The only fallible change first; the others cannot fail afterwards.
    bag.add(item, 1, catalog)
        .map_err(|_| TradeError::UnknownItem(item.to_owned()))?;
    *purse = paid;
    if matches!(entry.stock, Stock::Limited(_)) {
        stock.record_sale(&shop.id, item);
    }
    Ok(entry.price)
}

/// Sells one unit of `item` from the bag to `shop` at its `sell_value`.
/// Equipped items are not in the bag, so they cannot be sold. Key items,
/// items without value and the last weapon (nothing equipped, no other
/// weapon in the bag) are refused.
pub fn sell(
    shop: &ShopDefinition,
    item: &str,
    purse: &mut Purse,
    bag: &mut Inventory,
    equipment: &Equipment,
    catalog: &ItemCatalog,
) -> Result<u32, TradeError> {
    if !shop.buys {
        return Err(TradeError::DoesNotBuy);
    }
    let def = catalog
        .get(item)
        .ok_or_else(|| TradeError::UnknownItem(item.to_owned()))?;
    if bag.count(item) == 0 {
        return Err(TradeError::NotOwned);
    }
    if def.category == ItemCategory::QuestItem || def.sell_value == 0 {
        return Err(TradeError::NotSellable);
    }
    if def.category.is_weapon() {
        let armed = catalog.weapon_category(equipment).is_some();
        let weapons_in_bag: u32 = bag
            .stacks()
            .iter()
            .filter(|s| catalog.get(&s.id).is_some_and(|d| d.category.is_weapon()))
            .map(|s| s.quantity)
            .sum();
        if !armed && weapons_in_bag <= 1 {
            return Err(TradeError::LastWeapon);
        }
    }
    let earned = purse
        .checked_add(def.sell_value)
        .ok_or(TradeError::PurseFull)?;
    bag.remove(item, 1).map_err(|_| TradeError::NotOwned)?;
    *purse = earned;
    Ok(def.sell_value)
}
