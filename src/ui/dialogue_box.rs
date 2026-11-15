//! Dialogue box (V2.1): shown while `GameState::Dialogue`. Bottom of the
//! screen, above nothing of the HUD (the HUD panels are at the top), with
//! the speaker's name, the current line, the choices and what the keys do.
//! It only reads `ActiveConversation`; `npc::systems::run_conversation`
//! handles the keys.

use bevy::prelude::*;

use super::theme::{FONT_LARGE, FONT_NORMAL, FONT_SMALL, PANEL_BORDER, TEXT_DIM, TEXT_GOLD, text};
use crate::app::states::GameState;
use crate::core::resources::GameData;
use crate::npc::systems::{ActiveConversation, dialogue_facts};
use crate::player::components::{Belongings, Player};
use crate::world::exploration_systems::{Exploration, WorldGraph};

const BOX_BG: Color = Color::srgb(0.06, 0.04, 0.07);
const SELECTED: Color = Color::srgb(1.0, 0.95, 0.6);

#[derive(Component, Debug, Clone, Copy)]
pub enum DialoguePart {
    Speaker,
    Line,
    Choices,
    Hint,
}

pub fn spawn_dialogue_box(mut commands: Commands) {
    commands
        .spawn((
            DespawnOnExit(GameState::Dialogue),
            Node {
                position_type: PositionType::Absolute,
                left: px(140),
                right: px(140),
                bottom: px(24),
                flex_direction: FlexDirection::Column,
                row_gap: px(8),
                padding: UiRect::all(px(16)),
                border: UiRect::all(px(2)),
                ..default()
            },
            BackgroundColor(BOX_BG),
            BorderColor::all(PANEL_BORDER),
        ))
        .with_children(|panel| {
            panel.spawn((DialoguePart::Speaker, text("", FONT_LARGE, TEXT_GOLD)));
            panel.spawn((DialoguePart::Line, text("", FONT_NORMAL, Color::WHITE)));
            panel.spawn((DialoguePart::Choices, text("", FONT_NORMAL, SELECTED)));
            panel.spawn((DialoguePart::Hint, text("", FONT_SMALL, TEXT_DIM)));
        });
}

pub fn update_dialogue_box(
    data: Res<GameData>,
    conversation: Option<Res<ActiveConversation>>,
    world: Option<Res<WorldGraph>>,
    exploration: Option<Res<Exploration>>,
    player: Query<&Belongings, With<Player>>,
    mut parts: Query<(&DialoguePart, &mut Text)>,
) {
    let Some(conversation) = conversation else {
        return;
    };
    let c = &conversation.0;
    let facts = dialogue_facts(
        exploration.as_deref(),
        world.as_deref(),
        player.single().ok(),
    );
    let speaker = data.npcs.npc(&c.npc).map_or("", |n| n.name.as_str());
    let line = c.text(&data.npcs).unwrap_or("");
    let choices = c.choices(&data.npcs, &facts);
    let listed: Vec<String> = choices
        .iter()
        .enumerate()
        .map(|(i, choice)| {
            let marker = if i == c.cursor { "> " } else { "  " };
            format!("{marker}{}", choice.text)
        })
        .collect();
    let hint = if !choices.is_empty() {
        "Up/Down: choose    Enter or D: confirm    Esc: leave"
    } else if c.at_last_line(&data.npcs) {
        "Enter or D: close    Esc: leave"
    } else {
        "Enter or D: continue  >>    Esc: leave"
    };
    for (part, mut value) in &mut parts {
        value.0 = match part {
            DialoguePart::Speaker => speaker.to_owned(),
            DialoguePart::Line => line.to_owned(),
            DialoguePart::Choices => listed.join("\n"),
            DialoguePart::Hint => hint.to_owned(),
        };
    }
}
