//! Shop screen (V2.1): shown while `GameState::Shop` (simulation paused).
//!
//! `←`/`→` switch Buy and Sell, `↑`/`↓` select, `Enter` buys or sells one
//! unit (a `TradeRequest`; `economy::systems::apply_trades` decides), `Esc`
//! closes. Shows the shop, the keeper, the coins, category, rarity, price,
//! stock or quantity, and, for equipment, the comparison with what is worn
//! (`inventory::compare`, the same numbers as the inventory screen).

use bevy::prelude::*;

use super::theme::{
    FONT_LARGE, FONT_NORMAL, FONT_SMALL, PANEL_BORDER, TEXT_DIM, TEXT_GOLD, TEXT_MAIN, text,
};
use crate::app::states::GameState;
use crate::core::events::{TradeOp, TradeRequest};
use crate::core::resources::GameData;
use crate::economy::shop::{ShopDefinition, Stock};
use crate::economy::systems::{PlayerPurse, ShopStockState};
use crate::input::ActionEdges;
use crate::input::action::Action;
use crate::inventory::compare::{Snapshot, compare};
use crate::inventory::loadout::target_slot;
use crate::npc::definitions::{DialogueFacts, all_hold};
use crate::npc::systems::{ActiveShop, dialogue_facts};
use crate::player::components::{Belongings, CharacterSheet, Player};
use crate::world::exploration_systems::{Exploration, WorldGraph};

const SCREEN_BG: Color = Color::srgb(0.05, 0.04, 0.06);
const SELECTED: Color = Color::srgb(1.0, 0.95, 0.6);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ShopTab {
    #[default]
    Buy,
    Sell,
}

/// Tab and selected row of the open shop.
#[derive(Resource, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ShopCursor {
    pub tab: ShopTab,
    pub index: usize,
}

#[derive(Component, Debug, Clone, Copy)]
pub enum ShopPart {
    Title,
    List,
    Details,
    Hint,
}

/// Rows of the Buy tab: entries whose conditions hold, in data order.
pub fn buy_rows<'a>(shop: &'a ShopDefinition, facts: &DialogueFacts) -> Vec<&'a str> {
    shop.entries
        .iter()
        .filter(|e| all_hold(&e.when, facts))
        .map(|e| e.item.as_str())
        .collect()
}

/// Rows of the Sell tab: each item of the bag once, in bag order.
pub fn sell_rows(belongings: &Belongings) -> Vec<String> {
    let mut rows: Vec<String> = Vec::new();
    for stack in belongings.inventory.stacks() {
        if !rows.contains(&stack.id) {
            rows.push(stack.id.clone());
        }
    }
    rows
}

pub fn spawn_shop_screen(mut commands: Commands, mut cursor: ResMut<ShopCursor>) {
    *cursor = ShopCursor::default();
    commands
        .spawn((
            DespawnOnExit(GameState::Shop),
            Node {
                position_type: PositionType::Absolute,
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: px(10),
                        padding: UiRect::all(px(16)),
                        border: UiRect::all(px(2)),
                        width: px(1100),
                        ..default()
                    },
                    BackgroundColor(SCREEN_BG),
                    BorderColor::all(PANEL_BORDER),
                ))
                .with_children(|panel| {
                    panel.spawn((ShopPart::Title, text("", FONT_LARGE, TEXT_GOLD)));
                    panel
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: px(30),
                            ..default()
                        })
                        .with_children(|columns| {
                            columns.spawn((
                                ShopPart::List,
                                text("", FONT_NORMAL, TEXT_MAIN),
                                Node {
                                    width: px(580),
                                    ..default()
                                },
                            ));
                            columns.spawn((
                                ShopPart::Details,
                                text("", FONT_SMALL, TEXT_MAIN),
                                Node {
                                    width: px(440),
                                    ..default()
                                },
                            ));
                        });
                    panel.spawn((ShopPart::Hint, text("", FONT_SMALL, TEXT_DIM)));
                });
        });
}

/// Keys of the shop: tabs, selection, trade, close.
pub fn navigate_shop(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    edges: Res<ActionEdges>,
    data: Res<GameData>,
    active: Option<Res<ActiveShop>>,
    world: Option<Res<WorldGraph>>,
    exploration: Option<Res<Exploration>>,
    player: Single<&Belongings, With<Player>>,
    mut cursor: ResMut<ShopCursor>,
    mut requests: MessageWriter<TradeRequest>,
    mut next: ResMut<NextState<GameState>>,
) {
    let Some(shop) = active.as_ref().and_then(|a| data.shops.get(&a.shop)) else {
        next.set(GameState::Playing);
        return;
    };
    if keys.just_pressed(KeyCode::Escape) {
        commands.remove_resource::<ActiveShop>();
        next.set(GameState::Playing);
        return;
    }
    if edges.just_pressed(Action::MoveRight) && shop.buys {
        *cursor = ShopCursor {
            tab: ShopTab::Sell,
            index: 0,
        };
    }
    if edges.just_pressed(Action::MoveLeft) {
        *cursor = ShopCursor {
            tab: ShopTab::Buy,
            index: 0,
        };
    }
    let facts = dialogue_facts(exploration.as_deref(), world.as_deref(), Some(&player));
    let rows: Vec<String> = match cursor.tab {
        ShopTab::Buy => buy_rows(shop, &facts)
            .into_iter()
            .map(str::to_owned)
            .collect(),
        ShopTab::Sell => sell_rows(&player),
    };
    let last = rows.len().saturating_sub(1);
    if edges.just_pressed(Action::AimDown) {
        cursor.index = (cursor.index + 1).min(last);
    }
    if edges.just_pressed(Action::AimUp) {
        cursor.index = cursor.index.saturating_sub(1);
    }
    cursor.index = cursor.index.min(last);
    if !edges.just_pressed(Action::Confirm) {
        return;
    }
    if let Some(item) = rows.get(cursor.index) {
        let op = match cursor.tab {
            ShopTab::Buy => TradeOp::Buy(item.clone()),
            ShopTab::Sell => TradeOp::Sell(item.clone()),
        };
        requests.write(TradeRequest {
            shop: shop.id.clone(),
            op,
        });
    }
}

pub fn update_shop_screen(
    data: Res<GameData>,
    active: Option<Res<ActiveShop>>,
    cursor: Res<ShopCursor>,
    stock: Res<ShopStockState>,
    world: Option<Res<WorldGraph>>,
    exploration: Option<Res<Exploration>>,
    player: Single<(&Belongings, &CharacterSheet, Option<&PlayerPurse>), With<Player>>,
    mut parts: Query<(&ShopPart, &mut Text, &mut TextColor)>,
) {
    let Some(active) = active else {
        return;
    };
    let Some(shop) = data.shops.get(&active.shop) else {
        return;
    };
    let (belongings, sheet, purse) = player.into_inner();
    let coins = purse.map_or(0, |p| p.coins());
    let keeper = data.npcs.npc(&shop.keeper).map_or("", |n| n.name.as_str());
    let facts = dialogue_facts(exploration.as_deref(), world.as_deref(), Some(belongings));
    let catalog = &data.items;
    let rarity = |id: &str| {
        catalog
            .get(id)
            .and_then(|d| catalog.rarity(&d.rarity))
            .map_or(String::new(), |r| r.name.clone())
    };
    let tabs = match (cursor.tab, shop.buys) {
        (ShopTab::Buy, true) => "[ BUY ]   Sell",
        (ShopTab::Sell, _) => "  Buy   [ SELL ]",
        (ShopTab::Buy, false) => "[ BUY ]",
    };
    let mut list = format!("{tabs}\n\n");
    let mut selected: Option<String> = None;
    match cursor.tab {
        ShopTab::Buy => {
            for (i, item) in buy_rows(shop, &facts).into_iter().enumerate() {
                let Some(entry) = shop.entries.iter().find(|e| e.item == item) else {
                    continue;
                };
                let def = catalog.get(item);
                let left = match (entry.stock, stock.left(shop, entry)) {
                    (Stock::Unlimited, _) => "  -".to_owned(),
                    (_, Some(0)) => "sold out".to_owned(),
                    (_, Some(n)) => format!("x{n}"),
                    (_, None) => String::new(),
                };
                let marker = if i == cursor.index { ">" } else { " " };
                if i == cursor.index {
                    selected = Some(item.to_owned());
                }
                list += &format!(
                    "{marker} {:<22}{:<12}{:>6}  {left}\n",
                    def.map_or(item, |d| d.name.as_str()),
                    def.map_or("", |d| d.category.display_name()),
                    entry.price
                );
            }
        }
        ShopTab::Sell => {
            let rows = sell_rows(belongings);
            if rows.is_empty() {
                list += "  (nothing to sell)\n";
            }
            for (i, item) in rows.iter().enumerate() {
                let def = catalog.get(item);
                let marker = if i == cursor.index { ">" } else { " " };
                if i == cursor.index {
                    selected = Some(item.clone());
                }
                let value = def.map_or(0, |d| d.sell_value);
                list += &format!(
                    "{marker} {:<22}{:<12}{:>6}  x{}\n",
                    def.map_or(item.as_str(), |d| d.name.as_str()),
                    def.map_or("", |d| d.category.display_name()),
                    value,
                    belongings.inventory.count(item)
                );
            }
        }
    }
    let mut details = String::new();
    if let Some(def) = selected.as_deref().and_then(|id| catalog.get(id)) {
        details += &format!(
            "{}\n{} - {}\n{}\n",
            def.name,
            def.category.display_name(),
            rarity(&def.id),
            def.description
        );
        if catalog.is_two_handed(&def.id) {
            details += "Two-handed: no off-hand item with it.\n";
        }
        if let Some(slot) = target_slot(&def.id, &belongings.equipment, catalog) {
            let base = sheet.base();
            let level = sheet.progression().level();
            let formulas = &data.character.formulas;
            let before = Snapshot::of(base, level, formulas, &belongings.equipment, catalog);
            let preview = belongings.equipment.preview(slot, Some(&def.id));
            let after = Snapshot::of(base, level, formulas, &preview, catalog);
            let lines: Vec<String> = compare(&before, &after)
                .iter()
                .filter(|l| l.changed())
                .map(super::inventory_screen::format_line)
                .collect();
            details += &format!(
                "\nIf worn in {} (now -> then):\n{}",
                slot.display_name(),
                if lines.is_empty() {
                    "no change".to_owned()
                } else {
                    lines.join("\n")
                }
            );
        }
    }
    for (part, mut value, mut color) in &mut parts {
        match part {
            ShopPart::Title => {
                value.0 = format!("{}  -  {keeper}        Marks: {coins}", shop.name);
            }
            ShopPart::List => {
                value.0 = list.clone();
                color.0 = SELECTED;
            }
            ShopPart::Details => value.0 = details.clone(),
            ShopPart::Hint => {
                value.0 =
                    "Left/Right: buy / sell    Up/Down: select    Enter: confirm one unit    Esc: leave"
                        .to_owned();
            }
        }
    }
}
