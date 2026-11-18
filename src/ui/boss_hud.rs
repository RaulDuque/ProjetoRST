//! Boss HP bar shown during an active encounter (D21).

use bevy::prelude::*;

use super::theme::{
    BAR_BG, FONT_NORMAL, FONT_SMALL, HP_COLOR, PANEL_BG, PANEL_BORDER, TEXT_GOLD, TEXT_MAIN, text,
};
use crate::enemies::boss::{BossPattern, BossPhase, pattern_for_phase};
use crate::enemies::boss_systems::BossEncounter;
use crate::enemies::components::{Boss, BossRuntime, EnemyHealth};

const BAR_WIDTH: f32 = 420.0;
const BAR_HEIGHT: f32 = 16.0;

#[derive(Component)]
pub struct BossHudRoot;

#[derive(Component)]
pub struct BossHudName;

#[derive(Component)]
pub struct BossHudFill;

#[derive(Component)]
pub struct BossHudLabel;

#[derive(Component)]
pub struct BossHudPhase;

pub fn spawn_boss_hud(mut commands: Commands) {
    commands
        .spawn((
            BossHudRoot,
            Node {
                position_type: PositionType::Absolute,
                top: px(12),
                width: percent(100),
                justify_content: JustifyContent::Center,
                display: Display::None,
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: px(4),
                    padding: UiRect::all(px(10)),
                    border: UiRect::all(px(2)),
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(PANEL_BG),
                BorderColor::all(PANEL_BORDER),
            ))
            .with_children(|panel| {
                panel.spawn((BossHudName, text("", FONT_NORMAL, TEXT_GOLD)));
                panel
                    .spawn((
                        Node {
                            width: px(BAR_WIDTH),
                            height: px(BAR_HEIGHT),
                            ..default()
                        },
                        BackgroundColor(BAR_BG),
                    ))
                    .with_children(|track| {
                        track.spawn((
                            BossHudFill,
                            Node {
                                width: percent(100),
                                height: percent(100),
                                ..default()
                            },
                            BackgroundColor(HP_COLOR),
                        ));
                    });
                panel.spawn((BossHudLabel, text("", FONT_SMALL, TEXT_MAIN)));
                panel.spawn((BossHudPhase, text("", FONT_SMALL, TEXT_MAIN)));
            });
        });
}

pub fn update_boss_hud(
    encounter: Res<BossEncounter>,
    bosses: Query<(&EnemyHealth, Option<&BossRuntime>), With<Boss>>,
    mut root: Query<&mut Node, With<BossHudRoot>>,
    mut name: Query<&mut Text, With<BossHudName>>,
    mut fill: Query<&mut Node, (With<BossHudFill>, Without<BossHudRoot>)>,
    mut label: Query<
        &mut Text,
        (
            With<BossHudLabel>,
            Without<BossHudName>,
            Without<BossHudPhase>,
        ),
    >,
    mut phase: Query<
        &mut Text,
        (
            With<BossHudPhase>,
            Without<BossHudName>,
            Without<BossHudLabel>,
        ),
    >,
) {
    let Ok(mut root_node) = root.single_mut() else {
        return;
    };
    if !encounter.active {
        root_node.display = Display::None;
        return;
    }
    let Ok((health, runtime)) = bosses.single() else {
        root_node.display = Display::None;
        return;
    };
    root_node.display = Display::Flex;
    if let Ok(mut text) = name.single_mut() {
        text.0 = encounter.display_name.clone();
    }
    let ratio = health.ratio().clamp(0.0, 1.0);
    if let Ok(mut fill_node) = fill.single_mut() {
        fill_node.width = percent(ratio * 100.0);
    }
    if let Ok(mut text) = label.single_mut() {
        text.0 = format!("{}/{}", health.current(), health.max());
    }
    let phase_label = match runtime {
        Some(runtime) if runtime.phase_lock > 0.0 => "Phase shift".to_owned(),
        Some(runtime) => {
            let tell = match pattern_for_phase(runtime.phase, runtime.next_pattern) {
                BossPattern::FrontalSlash => "slash",
                BossPattern::Lunge => "lunge",
                BossPattern::AshWave => "ash wave",
            };
            let name = match runtime.phase {
                BossPhase::Two => "Phase II",
                BossPhase::One => "Phase I",
            };
            format!("{name} · {tell}")
        }
        None => "Phase I".to_owned(),
    };
    if let Ok(mut text) = phase.single_mut() {
        text.0 = phase_label;
    }
}
