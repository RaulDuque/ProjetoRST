//! In-game HUD: HP/MP/EXP bars, Hearts, level, cooldowns, command feedback
//! and notifications. Each element is updated by its own small system.
//!
//! V1.9: a line under the MP bar shows the form (human or spectral), what
//! the transformation costs, its upkeep, the MP it has used and why it
//! cannot start. The controls hint moved from the bottom edge (where it
//! covered the floor) into the status panel, as three short lines.

use bevy::prelude::*;

use super::theme::{
    BAR_BG, EXP_COLOR, FONT_NORMAL, FONT_SMALL, HP_COLOR, MP_COLOR, PANEL_BG, PANEL_BORDER,
    TEXT_DIM, TEXT_GOLD, TEXT_MAIN, text,
};
use crate::app::states::GameState;
use crate::core::constants::NOTIFICATION_TIME;
use crate::core::events::{CommandResolved, Notification};
use crate::core::resources::GameData;
use crate::player::components::{CharacterSheet, Player, SkillBook, Vitals};
use crate::skills::execution::CommandOutcome;
use crate::transform::systems::Transformation;
use crate::world::exploration_systems::Exploration;

/// Colour of the form line while a form is active.
const FORM_ACTIVE: Color = Color::srgb(0.6, 0.88, 1.0);
/// Colour of the form line when the form cannot start or MP is low.
const FORM_WARNING: Color = Color::srgb(0.95, 0.55, 0.4);

const BAR_WIDTH: f32 = 220.0;
const BAR_HEIGHT: f32 = 14.0;
const MAX_NOTIFICATIONS: usize = 4;
const CONTROLS_HINT: &str = "Arrows move  Shift run  Z jump  X attack  C magic\n\
     A guard  S dodge  D use  T form\n\
     M map  Tab status  I equip  F5 save  F9 load";

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarKind {
    Hp,
    Mp,
    Exp,
}

/// The coloured fill of a bar.
#[derive(Component, Debug, Clone, Copy)]
pub struct BarFill(pub BarKind);

/// The numeric label next to a bar.
#[derive(Component, Debug, Clone, Copy)]
pub struct BarLabel(pub BarKind);

#[derive(Component, Debug, Clone, Copy)]
pub struct LevelText;

#[derive(Component, Debug, Clone, Copy)]
pub struct HeartsText;

#[derive(Component, Debug, Clone, Copy)]
pub struct CooldownText;

/// Current form and transformation availability (V1.9).
#[derive(Component, Debug, Clone, Copy)]
pub struct FormText;

#[derive(Component, Debug, Clone, Copy)]
pub struct CommandFeedbackText;

#[derive(Component, Debug, Clone, Copy)]
pub struct NotificationText;

/// Last command feedback and recent notifications with their timers.
#[derive(Resource, Debug, Clone, Default)]
pub struct HudMessages {
    pub command: Option<(String, f32)>,
    pub notifications: Vec<(String, f32)>,
}

fn bar(parent: &mut ChildSpawnerCommands, kind: BarKind, color: Color, label: &str) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: px(8),
            ..default()
        })
        .with_children(|row| {
            row.spawn(text(label, FONT_SMALL, TEXT_DIM));
            row.spawn((
                Node {
                    width: px(BAR_WIDTH),
                    height: px(BAR_HEIGHT),
                    ..default()
                },
                BackgroundColor(BAR_BG),
            ))
            .with_children(|track| {
                track.spawn((
                    BarFill(kind),
                    Node {
                        width: percent(100),
                        height: percent(100),
                        ..default()
                    },
                    BackgroundColor(color),
                ));
            });
            row.spawn((BarLabel(kind), text("", FONT_SMALL, TEXT_MAIN)));
        });
}

/// The column of the left HUD (status, command feedback, notifications).
#[derive(Component)]
pub struct HudColumn;

/// Background panel of a message text, hidden while the text is empty.
#[derive(Component)]
pub struct MessagePanel;

/// V2.2: every HUD text lives in one top-left column: the status panel,
/// then the command feedback, then the notifications, each message on a
/// dark panel. Stacked by the layout, they can never overlap each other,
/// the map (top right) or the character and the talk prompts (the
/// notifications used to sit bottom-left, over the floor where the player
/// and NPCs stand, without a background).
pub fn spawn_hud(mut commands: Commands) {
    let column = commands
        .spawn((
            HudColumn,
            Node {
                position_type: PositionType::Absolute,
                top: px(COLUMN_TOP),
                left: px(12),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                row_gap: px(6),
                ..default()
            },
        ))
        .id();
    // Status panel.
    commands
        .spawn((
            ChildOf(column),
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: px(4),
                padding: UiRect::all(px(10)),
                border: UiRect::all(px(2)),
                ..default()
            },
            BackgroundColor(PANEL_BG),
            BorderColor::all(PANEL_BORDER),
        ))
        .with_children(|panel| {
            panel.spawn((LevelText, text("", FONT_NORMAL, TEXT_GOLD)));
            bar(panel, BarKind::Hp, HP_COLOR, "HP ");
            bar(panel, BarKind::Mp, MP_COLOR, "MP ");
            panel.spawn((FormText, text("", FONT_SMALL, TEXT_DIM)));
            bar(panel, BarKind::Exp, EXP_COLOR, "EXP");
            panel.spawn((HeartsText, text("", FONT_SMALL, TEXT_MAIN)));
            panel.spawn((CooldownText, text("", FONT_SMALL, TEXT_DIM)));
            // Controls hint (V1.9: moved here from the bottom edge, where it
            // covered the floor, and kept clear of the map in the top right).
            panel.spawn(text(CONTROLS_HINT, FONT_SMALL, TEXT_DIM));
        });

    // Command feedback and notifications, each on its own panel.
    message_panel(
        &mut commands,
        column,
        (CommandFeedbackText, text("", FONT_NORMAL, TEXT_GOLD)),
    );
    // The notifications are a root node: they sit right under the column
    // while playing, or at the bottom edge over the paused screens (see
    // `notification_placement`), always drawn above the overlays.
    commands
        .spawn((
            MessagePanel,
            NotificationPanel,
            GlobalZIndex(NOTIFICATION_Z),
            Node {
                position_type: PositionType::Absolute,
                left: px(12),
                padding: UiRect::axes(px(10), px(6)),
                border: UiRect::all(px(1)),
                display: Display::None,
                ..default()
            },
            BackgroundColor(PANEL_BG),
            BorderColor::all(PANEL_BORDER),
        ))
        .with_child((NotificationText, text("", FONT_NORMAL, TEXT_MAIN)));
}

fn message_panel(commands: &mut Commands, column: Entity, content: impl Bundle) {
    commands
        .spawn((
            ChildOf(column),
            MessagePanel,
            Node {
                padding: UiRect::axes(px(10), px(6)),
                border: UiRect::all(px(1)),
                display: Display::None,
                ..default()
            },
            BackgroundColor(PANEL_BG),
            BorderColor::all(PANEL_BORDER),
        ))
        .with_child(content);
}

/// The notifications panel (it moves with the screen state).
#[derive(Component)]
pub struct NotificationPanel;

/// Draw order of the notifications panel: over the paused screens' overlay.
const NOTIFICATION_Z: i32 = 10;
/// Gap below the notifications when they sit at the bottom edge, and
/// between the column and the notifications under it.
const BOTTOM_STRIP_GAP: f32 = 8.0;
const COLUMN_TOP: f32 = 12.0;
const COLUMN_GAP: f32 = 6.0;

/// Where the notifications go.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationPlacement {
    /// In the top-left column, under the status panel (playing; dialogue,
    /// whose box is at the bottom).
    Column,
    /// At the bottom-left edge, over the dimmed scene: the shop, inventory
    /// and status screens fill the middle of the screen and would cover the
    /// column, and the game is paused so no character is hidden.
    BottomStrip,
}

pub fn notification_placement(state: GameState) -> NotificationPlacement {
    match state {
        GameState::Playing | GameState::Dialogue => NotificationPlacement::Column,
        GameState::Shop | GameState::Inventory | GameState::StatusScreen => {
            NotificationPlacement::BottomStrip
        }
    }
}

pub fn update_bars(
    player: Single<(&Vitals, &CharacterSheet), With<Player>>,
    data: Res<GameData>,
    mut fills: Query<(&BarFill, &mut Node)>,
    mut labels: Query<(&BarLabel, &mut Text)>,
) {
    let (vitals, sheet) = player.into_inner();
    let progression = sheet.progression();
    let exp_needed = data
        .character
        .progression
        .curve
        .exp_to_next(progression.level());
    let exp_ratio = exp_needed.map_or(1.0, |needed| {
        progression.exp_into_level() as f32 / needed.max(1) as f32
    });
    for (fill, mut node) in &mut fills {
        let ratio = match fill.0 {
            BarKind::Hp => vitals.hp.ratio(),
            BarKind::Mp => vitals.mp.ratio(),
            BarKind::Exp => exp_ratio,
        };
        node.width = percent(ratio.clamp(0.0, 1.0) * 100.0);
    }
    for (label, mut text) in &mut labels {
        text.0 = match label.0 {
            BarKind::Hp => format!("{}/{}", vitals.hp.current(), vitals.hp.max()),
            BarKind::Mp => format!("{}/{}", vitals.mp.current(), vitals.mp.max()),
            BarKind::Exp => match exp_needed {
                Some(needed) => format!("{}/{needed}", progression.exp_into_level()),
                None => "MAX".to_owned(),
            },
        };
    }
}

pub fn update_level_and_hearts(
    player: Single<
        (
            &Vitals,
            &CharacterSheet,
            Option<&crate::economy::systems::PlayerPurse>,
        ),
        With<Player>,
    >,
    mut level: Single<&mut Text, (With<LevelText>, Without<HeartsText>)>,
    mut hearts: Single<&mut Text, (With<HeartsText>, Without<LevelText>)>,
) {
    let (vitals, sheet, purse) = player.into_inner();
    let points = sheet.progression().unspent_points();
    level.0 = if points > 0 {
        format!(
            "Level {}  (+{points} points - Tab)",
            sheet.progression().level()
        )
    } else {
        format!("Level {}", sheet.progression().level())
    };
    // V2.1: the coins ("marks") share the Hearts line.
    let coins = purse.map_or(0, |p| p.coins());
    hearts.0 = format!(
        "Hearts {}/{}   Marks {coins}",
        vitals.hearts.current(),
        vitals.hearts.max()
    );
}

pub fn update_cooldowns(
    data: Res<GameData>,
    book: Single<&SkillBook, With<Player>>,
    mut text: Single<&mut Text, With<CooldownText>>,
) {
    let lines: Vec<String> = book
        .cooldowns
        .active()
        .map(|(id, left)| {
            let name = data.skills.get(id).map_or(id, |d| d.name.as_str());
            format!("{name} {left:.1}s")
        })
        .collect();
    text.0 = lines.join("\n");
}

fn describe(outcome: &CommandOutcome, data: &GameData) -> Option<String> {
    let name = |id: &str| {
        data.skills
            .get(id)
            .map_or_else(|| id.to_owned(), |d| d.name.clone())
    };
    let text = match outcome {
        CommandOutcome::Invalid => return None,
        CommandOutcome::Executed { id } => format!("{}!", name(id)),
        CommandOutcome::Incomplete {
            id,
            matched,
            required,
        } => format!("{} - incomplete ({matched}/{required})", name(id)),
        CommandOutcome::TooSlow { id } => format!("{} - too slow", name(id)),
        CommandOutcome::InsufficientResources {
            id,
            resource,
            required,
            available,
        } => {
            let resource = match resource {
                crate::skills::execution::ResourceKind::Mp => "MP",
                crate::skills::execution::ResourceKind::Hearts => "Hearts",
            };
            format!(
                "{} - not enough {resource} ({available}/{required})",
                name(id)
            )
        }
        CommandOutcome::NotLearned { id } => format!("{} - not learned yet", name(id)),
        CommandOutcome::OnCooldown { id, remaining } => {
            format!("{} - cooldown {remaining:.1}s", name(id))
        }
        CommandOutcome::NotAllowedNow { id } => format!("{} - cannot use now", name(id)),
        CommandOutcome::WrongWeapon { id } => {
            let needs = data
                .skills
                .get(id)
                .map(|d| {
                    d.weapons
                        .iter()
                        .map(|c| c.display_name().to_lowercase())
                        .collect::<Vec<_>>()
                        .join(" or ")
                })
                .unwrap_or_default();
            format!("{} - needs a {needs}", name(id))
        }
    };
    Some(text)
}

pub fn collect_messages(
    data: Res<GameData>,
    mut messages: ResMut<HudMessages>,
    mut commands_resolved: MessageReader<CommandResolved>,
    mut notifications: MessageReader<Notification>,
) {
    for resolved in commands_resolved.read() {
        if let Some(text) = describe(&resolved.outcome, &data) {
            messages.command = Some((text, NOTIFICATION_TIME));
        }
    }
    for notification in notifications.read() {
        messages
            .notifications
            .push((notification.0.clone(), NOTIFICATION_TIME));
    }
    let overflow = messages
        .notifications
        .len()
        .saturating_sub(MAX_NOTIFICATIONS);
    messages.notifications.drain(..overflow);
}

pub fn update_messages(
    time: Res<Time>,
    mut messages: ResMut<HudMessages>,
    mut feedback: Single<&mut Text, (With<CommandFeedbackText>, Without<NotificationText>)>,
    mut notifications: Single<&mut Text, (With<NotificationText>, Without<CommandFeedbackText>)>,
) {
    let dt = time.delta_secs();
    if let Some((_, timer)) = messages.command.as_mut() {
        *timer -= dt;
    }
    if messages.command.as_ref().is_some_and(|(_, t)| *t <= 0.0) {
        messages.command = None;
    }
    messages.notifications.retain_mut(|(_, timer)| {
        *timer -= dt;
        *timer > 0.0
    });
    feedback.0 = messages
        .command
        .as_ref()
        .map(|(text, _)| text.clone())
        .unwrap_or_default();
    notifications.0 = messages
        .notifications
        .iter()
        .map(|(text, _)| text.as_str())
        .collect::<Vec<_>>()
        .join("\n");
}

/// Shows a message panel only while its text has something to say, and
/// places the notifications for the current screen.
pub fn show_message_panels(
    state: Option<Res<State<GameState>>>,
    column: Query<&ComputedNode, With<HudColumn>>,
    texts: Query<(&Text, &ChildOf), Or<(With<CommandFeedbackText>, With<NotificationText>)>>,
    mut panels: Query<(&mut Node, Has<NotificationPanel>), With<MessagePanel>>,
) {
    let placement = notification_placement(state.map_or(GameState::Playing, |s| *s.get()));
    // Logical height of the column as laid out last frame.
    let column_height = column
        .single()
        .map_or(0.0, |c| c.size().y * c.inverse_scale_factor());
    for (text, parent) in &texts {
        let Ok((mut node, is_notification)) = panels.get_mut(parent.parent()) else {
            continue;
        };
        let display = if text.0.is_empty() {
            Display::None
        } else {
            Display::Flex
        };
        if node.display != display {
            node.display = display;
        }
        if !is_notification {
            continue;
        }
        let (top, bottom) = match placement {
            NotificationPlacement::Column => {
                (px(COLUMN_TOP + column_height + COLUMN_GAP), Val::Auto)
            }
            NotificationPlacement::BottomStrip => (Val::Auto, px(BOTTOM_STRIP_GAP)),
        };
        if node.top != top || node.bottom != bottom {
            node.top = top;
            node.bottom = bottom;
        }
    }
}

/// The form line (V1.9): `Form: Human` plus what `T` would do, or the
/// active form with its upkeep, the MP it has used and how long it lasts.
pub fn update_form_text(
    data: Res<GameData>,
    exploration: Option<Res<Exploration>>,
    player: Single<(&Vitals, Option<&Transformation>), With<Player>>,
    mut line: Single<(&mut Text, &mut TextColor), With<FormText>>,
) {
    let (vitals, form) = player.into_inner();
    let Some(def) = data.transformations.primary() else {
        line.0.0 = String::new();
        return;
    };
    let mp = vitals.mp.current();
    let (text, color) = match form.filter(|f| f.is_active()) {
        Some(form) => {
            let rate = def.costs.mp_per_second;
            let left = if rate > 0.0 {
                mp as f32 / rate
            } else {
                f32::INFINITY
            };
            let status = if form.ending().is_some() {
                "returning...".to_owned()
            } else {
                format!("-{rate:.0} MP/s  ~{left:.0}s")
            };
            let color = if form.ending().is_none() && left < def.visual.low_mp_seconds {
                FORM_WARNING
            } else {
                FORM_ACTIVE
            };
            (
                format!("Form: {}  {status}  (used {} MP)", def.name, form.spent()),
                color,
            )
        }
        None => {
            let unlocked = exploration
                .as_ref()
                .is_some_and(|e| def.unlock.is_met(&e.capabilities()));
            let cost = def.costs.activation_mp;
            if !unlocked {
                (format!("Form: Human  T: {} (locked)", def.name), TEXT_DIM)
            } else if mp < cost {
                (
                    format!("Form: Human  T: {} needs {cost} MP", def.name),
                    FORM_WARNING,
                )
            } else {
                (
                    format!(
                        "Form: Human  T: {} ({cost} MP, {:.0} MP/s)",
                        def.name, def.costs.mp_per_second
                    ),
                    TEXT_MAIN,
                )
            }
        }
    };
    let (mut current, mut current_color) = line.into_inner();
    if current.0 != text {
        current.0 = text;
    }
    current_color.0 = color;
}

#[cfg(test)]
mod placement_tests {
    use super::*;

    #[test]
    fn notifications_leave_the_column_only_over_the_paused_screens() {
        assert_eq!(
            notification_placement(GameState::Playing),
            NotificationPlacement::Column
        );
        // The dialogue box is at the bottom edge: the column stays clear of it.
        assert_eq!(
            notification_placement(GameState::Dialogue),
            NotificationPlacement::Column
        );
        for state in [
            GameState::Shop,
            GameState::Inventory,
            GameState::StatusScreen,
        ] {
            assert_eq!(
                notification_placement(state),
                NotificationPlacement::BottomStrip
            );
        }
    }
}
