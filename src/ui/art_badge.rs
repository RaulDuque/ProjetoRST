//! N3.8: badge of the experimental N3 protagonist.
//!
//! Only spawned when the game runs with `ExperimentalProtagonist`
//! (`DARK_CASTLE_PROTAGONIST=n3`). It says that the art is experimental and,
//! when a state has no approved clip, which clip the idle pose stands in for,
//! so a placeholder is never mistaken for a finished animation.

use bevy::prelude::*;

use super::theme::{FONT_SMALL, PANEL_BG, TEXT_DIM, TEXT_GOLD, text};
use crate::player::visual::{PlayerVisual, VisualPlayback};

/// First line of the badge.
pub const BADGE_TITLE: &str = "EXPERIMENTAL PROTAGONIST (N3.8): idle N3.5 + run N3.6 15 fps";

/// Root node of the badge.
#[derive(Component, Debug, Clone, Copy)]
pub struct ArtBadge;

/// Line naming the placeholder (empty when the wanted clip is shown).
#[derive(Component, Debug, Clone, Copy)]
pub struct ArtBadgePlaceholder;

/// Second line of the badge for the clip the idle pose stands in for.
pub fn placeholder_line(wanted: Option<&str>) -> String {
    match wanted {
        Some(clip) => format!("PLACEHOLDER: N3.5 pose for \"{clip}\" (no approved clip)"),
        None => String::new(),
    }
}

pub fn spawn_art_badge(mut commands: Commands) {
    commands
        .spawn((
            ArtBadge,
            Node {
                position_type: PositionType::Absolute,
                right: px(12),
                bottom: px(12),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexEnd,
                row_gap: px(2),
                padding: UiRect::all(px(6)),
                ..default()
            },
            BackgroundColor(PANEL_BG),
        ))
        .with_children(|badge| {
            badge.spawn(text(BADGE_TITLE, FONT_SMALL, TEXT_GOLD));
            badge.spawn((
                ArtBadgePlaceholder,
                text(String::new(), FONT_SMALL, TEXT_DIM),
            ));
        });
}

pub fn update_art_badge(
    visuals: Query<&VisualPlayback, With<PlayerVisual>>,
    mut lines: Query<&mut Text, With<ArtBadgePlaceholder>>,
) {
    let wanted = visuals.iter().find_map(|p| p.placeholder_for.as_deref());
    let line = placeholder_line(wanted);
    for mut text in &mut lines {
        if text.0 != line {
            text.0.clone_from(&line);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_placeholder_line_names_the_missing_clip() {
        assert_eq!(placeholder_line(None), "");
        let line = placeholder_line(Some("jump_rise"));
        assert!(line.contains("PLACEHOLDER") && line.contains("\"jump_rise\""));
    }
}
