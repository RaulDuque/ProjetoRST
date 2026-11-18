//! Equipment/inventory screen (V2.0, key `I`). The simulation is paused
//! while it is open (`GameState::Inventory`).
//!
//! Left: the eight slots, then every equipment item in the bag, then the
//! other items (not selectable). `↑`/`↓` select; `Enter` equips the selected
//! bag item (into the first free slot of its category, accessories
//! replacing Accessory 1 when both are taken) or removes the selected slot's
//! item. Middle: the selected item (category, rarity, weapon profile,
//! requirements, why it cannot be equipped). Right: the comparison, final
//! effective numbers before → after, from the game's own formulas
//! (`inventory::compare`).
//!
//! The screen only *asks*: it writes `EquipRequest`, and
//! `inventory::systems::apply_equip_requests` decides.

use bevy::prelude::*;

use super::theme::{
    FONT_LARGE, FONT_NORMAL, FONT_SMALL, PANEL_BORDER, TEXT_DIM, TEXT_GOLD, TEXT_MAIN, text,
};
use crate::app::states::GameState;
use crate::combat::hitboxes::{AttachedToOwner, Hitbox};
use crate::core::events::{EquipOp, EquipRequest, UseItemRequest};
use crate::core::resources::GameData;
use crate::input::ActionEdges;
use crate::input::action::Action;
use crate::inventory::compare::{Snapshot, StatLine, ValueFormat, compare};
use crate::inventory::equipment::EquipmentSlot;
use crate::inventory::items::{ItemCatalog, ItemDefinition};
use crate::inventory::loadout::{check_hands, target_slot};
use crate::inventory::systems::equip_gate;
use crate::player::components::{ActionState, Belongings, CharacterSheet, Player};
use crate::skills::unlock::RequirementContext;
use crate::transform::systems::Transformation;
use crate::world::transitions::TransitionLock;

/// Rows shown at once in the list (it scrolls around the cursor).
pub const VISIBLE_ROWS: usize = 22;
const SELECTED: Color = Color::srgb(1.0, 0.95, 0.6);
const BLOCKED: Color = Color::srgb(0.75, 0.38, 0.35);
const BETTER: &str = "+";
/// Opaque panel: the HUD under the screen must not show through the text.
const SCREEN_BG: Color = Color::srgb(0.05, 0.03, 0.06);

/// One selectable row of the list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Row {
    Slot(EquipmentSlot),
    Bag(String),
}

/// Selected row (kept while the screen is closed, clamped on open).
#[derive(Resource, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct InventoryCursor(pub usize);

#[derive(Component, Debug, Clone, Copy)]
pub struct InventoryRow(usize);

#[derive(Component, Debug, Clone, Copy)]
pub enum InventoryPanel {
    Details,
    Compare,
    Other,
}

/// The selectable rows: the eight slots, then bag equipment, then
/// consumables (D18). Equipment stays above potions so gear UX is unchanged.
pub fn rows(belongings: &Belongings, catalog: &ItemCatalog) -> Vec<Row> {
    let mut rows: Vec<Row> = EquipmentSlot::ALL.into_iter().map(Row::Slot).collect();
    let push_bag = |rows: &mut Vec<Row>, want_consumable: bool| {
        for stack in belongings.inventory.stacks() {
            let Some(def) = catalog.get(&stack.id) else {
                continue;
            };
            let is_consumable = def.consumable.is_some();
            let selectable = if want_consumable {
                is_consumable
            } else {
                def.category.is_equipment()
            };
            let listed = rows.iter().any(|r| *r == Row::Bag(stack.id.clone()));
            if selectable && !listed {
                rows.push(Row::Bag(stack.id.clone()));
            }
        }
    };
    push_bag(&mut rows, false);
    push_bag(&mut rows, true);
    rows
}

pub fn toggle_inventory_screen(
    keys: Res<ButtonInput<KeyCode>>,
    edges: Res<ActionEdges>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
) {
    let close = edges.just_pressed(Action::Inventory) || keys.just_pressed(KeyCode::Escape);
    match state.get() {
        GameState::Playing if edges.just_pressed(Action::Inventory) => {
            next.set(GameState::Inventory);
        }
        GameState::Inventory if close => next.set(GameState::Playing),
        _ => {}
    }
}

pub fn spawn_inventory_screen(mut commands: Commands) {
    commands
        .spawn((
            DespawnOnExit(GameState::Inventory),
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
                        row_gap: px(8),
                        padding: UiRect::all(px(16)),
                        border: UiRect::all(px(2)),
                        width: px(1180),
                        ..default()
                    },
                    BackgroundColor(SCREEN_BG),
                    BorderColor::all(PANEL_BORDER),
                ))
                .with_children(|panel| {
                    panel.spawn(text("EQUIPMENT", FONT_LARGE, TEXT_GOLD));
                    panel
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: px(28),
                            ..default()
                        })
                        .with_children(|columns| {
                            columns
                                .spawn(Node {
                                    flex_direction: FlexDirection::Column,
                                    width: px(390),
                                    ..default()
                                })
                                .with_children(|list| {
                                    for index in 0..VISIBLE_ROWS {
                                        list.spawn((
                                            InventoryRow(index),
                                            text("", FONT_SMALL, TEXT_MAIN),
                                        ));
                                    }
                                    list.spawn((
                                        InventoryPanel::Other,
                                        text("", FONT_SMALL, TEXT_DIM),
                                    ));
                                });
                            columns.spawn((
                                InventoryPanel::Details,
                                text("", FONT_NORMAL, TEXT_MAIN),
                                Node {
                                    width: px(400),
                                    ..default()
                                },
                            ));
                            columns.spawn((
                                InventoryPanel::Compare,
                                text("", FONT_SMALL, TEXT_MAIN),
                                Node {
                                    width: px(300),
                                    ..default()
                                },
                            ));
                        });
                    panel.spawn(text(
                        "Up/Down: select    Enter: equip / remove    I or Esc: close",
                        FONT_SMALL,
                        TEXT_DIM,
                    ));
                });
        });
}

/// Moves the cursor and turns `Enter` into an [`EquipRequest`] or
/// [`UseItemRequest`] (D18 consumables).
pub fn navigate_inventory(
    edges: Res<ActionEdges>,
    data: Res<GameData>,
    mut cursor: ResMut<InventoryCursor>,
    player: Single<&Belongings, With<Player>>,
    mut requests: MessageWriter<EquipRequest>,
    mut use_requests: MessageWriter<UseItemRequest>,
) {
    let rows = rows(&player, &data.items);
    let last = rows.len().saturating_sub(1);
    if edges.just_pressed(Action::AimDown) {
        cursor.0 = (cursor.0 + 1).min(last);
    }
    if edges.just_pressed(Action::AimUp) {
        cursor.0 = cursor.0.saturating_sub(1);
    }
    cursor.0 = cursor.0.min(last);
    if !edges.just_pressed(Action::Confirm) {
        return;
    }
    match rows.get(cursor.0) {
        Some(Row::Slot(slot)) if player.equipment.get(*slot).is_some() => {
            requests.write(EquipRequest {
                slot: *slot,
                op: EquipOp::Unequip,
            });
        }
        Some(Row::Bag(id)) => {
            if data.items.get(id).is_some_and(|d| d.consumable.is_some()) {
                use_requests.write(UseItemRequest { item: id.clone() });
            } else if let Some(slot) = target_slot(id, &player.equipment, &data.items) {
                requests.write(EquipRequest {
                    slot,
                    op: EquipOp::Equip(id.clone()),
                });
            }
        }
        _ => {}
    }
}

fn rarity_color(def: &ItemDefinition, catalog: &ItemCatalog) -> Color {
    catalog
        .rarity(&def.rarity)
        .map_or(TEXT_MAIN, |r| Color::srgb(r.color.0, r.color.1, r.color.2))
}

fn format_value(value: f32, format: ValueFormat) -> String {
    match format {
        ValueFormat::Integer => format!("{}", value.round() as i64),
        ValueFormat::Percent => format!("{:.0}%", value * 100.0),
        ValueFormat::Decimal => format!("{value:.2}"),
    }
}

pub fn format_line(line: &StatLine) -> String {
    let delta = line.delta();
    let sign = if delta > 0.0 { BETTER } else { "" };
    format!(
        "{:<13}{} -> {}  ({sign}{})",
        line.label,
        format_value(line.before, line.format),
        format_value(line.after, line.format),
        format_value(delta, line.format)
    )
}

fn describe(def: &ItemDefinition, data: &GameData, equipped_in: Option<EquipmentSlot>) -> String {
    let rarity = data
        .items
        .rarity(&def.rarity)
        .map_or(def.rarity.clone(), |r| r.name.clone());
    let mut out = format!(
        "{}\n{} - {}\n{}\n",
        def.name,
        def.category.display_name(),
        rarity,
        def.description
    );
    if data.items.is_two_handed(&def.id) {
        out += "Two-handed: no off-hand item with it\n";
    }
    if let Some(slot) = equipped_in {
        out += &format!("Equipped: {}\n", slot.display_name());
    }
    if let Some(class) = data.items.weapon_class(def.category) {
        let stats = def.weapon.clone().unwrap_or_default();
        let strike = crate::inventory::weapons::StrikeProfile::resolve(class, &stats, 1.0);
        out += &format!(
            "\n{} strike: x{:.2} ATK, {}\nreach {:.0}-{:.0}, swing {:.2}s + recovery {:.2}s\n",
            class.name,
            strike.multiplier,
            strike.element.display_name(),
            strike.near_edge().max(0.0),
            strike.far_edge(),
            class.attack_time,
            class.recovery,
        );
        let techniques: Vec<&str> = data
            .skills
            .iter()
            .filter(|s| s.weapons.contains(&def.category))
            .map(|s| s.name.as_str())
            .collect();
        out += &format!(
            "Techniques: {}\n",
            if techniques.is_empty() {
                "none (sword techniques need a sword)".to_owned()
            } else {
                techniques.join(", ")
            }
        );
    }
    let req = &def.requirements;
    if req.min_level > 0 || !req.attributes.is_empty() {
        let mut parts = Vec::new();
        if req.min_level > 0 {
            parts.push(format!("level {}", req.min_level));
        }
        for (attribute, value) in &req.attributes {
            parts.push(format!("{} {value}", attribute.short_name()));
        }
        out += &format!("Requires {}\n", parts.join(", "));
    }
    out
}

type ScreenPlayer<'a> = (
    Entity,
    &'a Belongings,
    &'a CharacterSheet,
    &'a ActionState,
    Option<&'a Transformation>,
);

pub fn update_inventory_screen(
    data: Res<GameData>,
    cursor: Res<InventoryCursor>,
    lock: Option<Res<TransitionLock>>,
    world: Option<Res<crate::world::WorldState>>,
    player: Single<ScreenPlayer, With<Player>>,
    strikes: Query<&Hitbox, With<AttachedToOwner>>,
    mut list: Query<(&InventoryRow, &mut Text, &mut TextColor), Without<InventoryPanel>>,
    mut panels: Query<(&InventoryPanel, &mut Text), Without<InventoryRow>>,
) {
    let (entity, belongings, sheet, action, form) = player.into_inner();
    let catalog = &data.items;
    let rows = rows(belongings, catalog);
    let gate = equip_gate(entity, action, form, lock.as_deref(), &strikes);
    let blocked = gate.check().err();
    let base = *sheet.base();
    let level = sheet.progression().level();
    let empty = std::collections::BTreeSet::new();
    let owned: std::collections::BTreeSet<String> = belongings
        .inventory
        .stacks()
        .iter()
        .map(|s| s.id.clone())
        .collect();
    let ctx = RequirementContext {
        level,
        attributes: &base,
        owned_items: &owned,
        progress_flags: world.as_ref().map_or(&empty, |w| &w.flags),
    };
    let equipment = &belongings.equipment;

    // List, scrolled so the cursor stays visible.
    let first = cursor.0.saturating_sub(VISIBLE_ROWS - 1);
    for (row, mut value, mut color) in &mut list {
        let index = first + row.0;
        let selected = index == cursor.0;
        let marker = if selected { "> " } else { "  " };
        let (line, tint) = match rows.get(index) {
            None => (String::new(), TEXT_MAIN),
            Some(Row::Slot(slot)) => match equipment.get(*slot).and_then(|id| catalog.get(id)) {
                Some(def) => (
                    format!("{marker}{:<12}{}", slot.display_name(), def.name),
                    rarity_color(def, catalog),
                ),
                None => (format!("{marker}{:<12}-", slot.display_name()), TEXT_DIM),
            },
            Some(Row::Bag(id)) => {
                let Some(def) = catalog.get(id) else {
                    continue;
                };
                let count = belongings.inventory.count(id);
                let qty = if count > 1 || def.consumable.is_some() {
                    format!(" x{count}")
                } else {
                    String::new()
                };
                let (flag, tint) = if def.consumable.is_some() {
                    ("  [use]", rarity_color(def, catalog))
                } else {
                    let usable = def.requirements.check(&ctx).is_ok();
                    (
                        if usable { "" } else { "  (cannot equip)" },
                        if usable {
                            rarity_color(def, catalog)
                        } else {
                            BLOCKED
                        },
                    )
                };
                (format!("{marker}  {}{qty}{flag}", def.name), tint)
            }
        };
        if index == EquipmentSlot::ALL.len() && rows.len() > index {
            value.0 = format!("-- bag --\n{line}");
        } else {
            value.0 = line;
        }
        color.0 = if selected { SELECTED } else { tint };
    }

    // Details and comparison of the selected row.
    let before = Snapshot::of(&base, level, &data.character.formulas, equipment, catalog);
    let (details, after) = match rows.get(cursor.0) {
        Some(Row::Slot(slot)) => match equipment.get(*slot).and_then(|id| catalog.get(id)) {
            Some(def) => (
                describe(def, &data, Some(*slot)) + "\nEnter: remove",
                Some(equipment.preview(*slot, None)),
            ),
            None => (format!("{}: empty", slot.display_name()), None),
        },
        Some(Row::Bag(id)) => match catalog.get(id) {
            Some(def) if def.consumable.is_some() => {
                let mut out = describe(def, &data, None);
                out += "\nEnter: use one";
                (out, None)
            }
            Some(def) => {
                let slot = target_slot(id, equipment, catalog);
                let mut out = describe(def, &data, None);
                if let Err(error) = def.requirements.check(&ctx) {
                    out += &format!("\nCannot equip: {error}");
                }
                // V2.1: say it before Enter, not only after (two hands).
                if let Some(Err(error)) = slot.map(|s| check_hands(s, id, equipment, catalog)) {
                    out += &format!("\nCannot equip: {error}");
                }
                match slot {
                    Some(slot) => out += &format!("\nEnter: equip in {}", slot.display_name()),
                    None => out += "\nNot equipment",
                }
                (out, slot.map(|s| equipment.preview(s, Some(id))))
            }
            None => (format!("Unknown item {id}"), None),
        },
        None => (String::new(), None),
    };
    let details = match blocked {
        Some(block) => format!("{details}\n\nCannot change equipment now: {block}"),
        None => details,
    };
    let comparison = match after {
        Some(after) => {
            let after = Snapshot::of(&base, level, &data.character.formulas, &after, catalog);
            let lines: Vec<String> = compare(&before, &after)
                .iter()
                .filter(|l| l.changed())
                .map(format_line)
                .collect();
            if lines.is_empty() {
                "Comparison\n\nNo change".to_owned()
            } else {
                format!("Comparison (now -> then)\n\n{}", lines.join("\n"))
            }
        }
        None => current_summary(&before),
    };
    let other: Vec<String> = belongings
        .inventory
        .stacks()
        .iter()
        .filter(|s| {
            catalog
                .get(&s.id)
                .is_some_and(|d| !d.category.is_equipment())
        })
        .map(|s| {
            let name = catalog
                .get(&s.id)
                .map_or(s.id.as_str(), |d| d.name.as_str());
            format!("  {name} x{}", s.quantity)
        })
        .collect();
    for (panel, mut value) in &mut panels {
        value.0 = match panel {
            InventoryPanel::Details => details.clone(),
            InventoryPanel::Compare => comparison.clone(),
            InventoryPanel::Other => {
                if other.is_empty() {
                    String::new()
                } else {
                    format!("\n-- other items --\n{}", other.join("\n"))
                }
            }
        };
    }
}

fn current_summary(now: &Snapshot) -> String {
    let d = &now.derived;
    format!(
        "Current\n\nStrike power {:.0}\nReach {:.0}\nSwing time {:.2}\nATK {}  DEF {}\nMATK {}  MDEF {}\nHP {}  MP {}",
        now.strike.raw_power(d.attack),
        now.strike.far_edge(),
        now.strike.action_time,
        d.attack,
        d.defense,
        d.magic_attack,
        d.magic_defense,
        d.max_hp,
        d.max_mp
    )
}
