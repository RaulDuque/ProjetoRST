//! Fast-travel screen (D19): open after resting at a discovered altar when
//! at least one other altar is known. Simulation paused (`GameState::FastTravel`).

use bevy::prelude::*;

use super::theme::{
    FONT_LARGE, FONT_NORMAL, FONT_SMALL, PANEL_BORDER, TEXT_DIM, TEXT_GOLD, TEXT_MAIN, text,
};
use crate::app::states::GameState;
use crate::core::events::{FastTravelRequest, Notification};
use crate::input::ActionEdges;
use crate::input::action::Action;
use crate::world::WorldState;
use crate::world::exploration_systems::WorldGraph;
use crate::world::fast_travel::{self, SavePointSite};

const SCREEN_BG: Color = Color::srgb(0.05, 0.03, 0.07);

/// Altar the player just rested at (origin of the trip).
#[derive(Resource, Debug, Clone, Default)]
pub struct FastTravelSession {
    pub origin: String,
}

#[derive(Resource, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FastTravelCursor(pub usize);

#[derive(Component, Debug, Clone, Copy)]
pub enum FastTravelPart {
    Title,
    List,
    Hint,
}

pub fn spawn_fast_travel_screen(mut commands: Commands, mut cursor: ResMut<FastTravelCursor>) {
    *cursor = FastTravelCursor(0);
    commands
        .spawn((
            DespawnOnExit(GameState::FastTravel),
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
                        width: px(720),
                        ..default()
                    },
                    BackgroundColor(SCREEN_BG),
                    BorderColor::all(PANEL_BORDER),
                ))
                .with_children(|panel| {
                    panel.spawn((
                        FastTravelPart::Title,
                        text("FAST TRAVEL", FONT_LARGE, TEXT_GOLD),
                    ));
                    panel.spawn((FastTravelPart::List, text("", FONT_NORMAL, TEXT_MAIN)));
                    panel.spawn((
                        FastTravelPart::Hint,
                        text(
                            "Up/Down: select  Enter: travel  Esc: stay",
                            FONT_SMALL,
                            TEXT_DIM,
                        ),
                    ));
                });
        });
}

fn destination_rows(graph: &WorldGraph, progress: &WorldState, origin: &str) -> Vec<SavePointSite> {
    fast_travel::destinations(&graph.0, &progress.0, origin)
}

pub fn navigate_fast_travel(
    keys: Res<ButtonInput<KeyCode>>,
    edges: Res<ActionEdges>,
    session: Res<FastTravelSession>,
    graph: Option<Res<WorldGraph>>,
    progress: Res<WorldState>,
    mut cursor: ResMut<FastTravelCursor>,
    mut next: ResMut<NextState<GameState>>,
    mut requests: MessageWriter<FastTravelRequest>,
    mut notifications: MessageWriter<Notification>,
) {
    let Some(graph) = graph else {
        return;
    };
    let rows = destination_rows(&graph, &progress, &session.origin);
    if rows.is_empty() {
        notifications.write(Notification("No other discovered altars".into()));
        next.set(GameState::Playing);
        return;
    }
    let last = rows.len().saturating_sub(1);
    if edges.just_pressed(Action::AimDown) {
        cursor.0 = (cursor.0 + 1).min(last);
    }
    if edges.just_pressed(Action::AimUp) {
        cursor.0 = cursor.0.saturating_sub(1);
    }
    cursor.0 = cursor.0.min(last);

    if keys.just_pressed(KeyCode::Escape) {
        next.set(GameState::Playing);
        return;
    }
    if !edges.just_pressed(Action::Confirm) {
        return;
    }
    let Some(dest) = rows.get(cursor.0) else {
        return;
    };
    requests.write(FastTravelRequest {
        destination: dest.id.clone(),
    });
}

pub fn update_fast_travel_screen(
    session: Res<FastTravelSession>,
    cursor: Res<FastTravelCursor>,
    graph: Option<Res<WorldGraph>>,
    progress: Res<WorldState>,
    mut parts: Query<(&FastTravelPart, &mut Text)>,
) {
    let Some(graph) = graph else {
        return;
    };
    let origin_name = fast_travel::find_site(&graph.0, &session.origin)
        .map(|s| s.room_name)
        .unwrap_or_else(|| session.origin.clone());
    let rows = destination_rows(&graph, &progress, &session.origin);
    let mut list = String::new();
    for (i, site) in rows.iter().enumerate() {
        let mark = if i == cursor.0 { ">" } else { " " };
        list.push_str(&format!("{mark} {} ({})\n", site.room_name, site.id));
    }
    if list.is_empty() {
        list.push_str("(no destinations)\n");
    }
    for (part, mut text) in &mut parts {
        match *part {
            FastTravelPart::Title => {
                text.0 = format!("FAST TRAVEL — from {origin_name}");
            }
            FastTravelPart::List => {
                text.0 = list.clone();
            }
            FastTravelPart::Hint => {}
        }
    }
}
