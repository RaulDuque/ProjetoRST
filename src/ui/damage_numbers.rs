//! Floating damage numbers above hit targets.

use bevy::prelude::*;

use super::theme::{FONT_NORMAL, TEXT_DIM};
use crate::combat::damage::DamageOutcome;
use crate::core::constants::Z_EFFECTS;
use crate::core::events::{DamageDealt, RoomChanged};
use crate::magic::casting::element_color;

const LIFETIME: f32 = 0.8;
const RISE_SPEED: f32 = 50.0;
const SPAWN_HEIGHT: f32 = 36.0;

#[derive(Component, Debug, Clone, Copy)]
pub struct DamageNumber {
    remaining: f32,
}

pub fn spawn_damage_numbers(
    mut commands: Commands,
    mut hits: MessageReader<DamageDealt>,
    targets: Query<&Transform>,
) {
    for hit in hits.read() {
        let Ok(target) = targets.get(hit.target) else {
            continue;
        };
        let (label, color) = match hit.outcome {
            DamageOutcome::Miss => ("MISS".to_owned(), TEXT_DIM),
            DamageOutcome::Hit {
                amount,
                critical,
                element,
            } => {
                let label = if critical {
                    format!("{amount}!")
                } else {
                    amount.to_string()
                };
                (label, element_color(element))
            }
        };
        commands.spawn((
            DamageNumber {
                remaining: LIFETIME,
            },
            Text2d::new(label),
            TextFont {
                font_size: FontSize::Px(FONT_NORMAL),
                ..default()
            },
            TextColor(color),
            Transform::from_xyz(
                target.translation.x,
                target.translation.y + SPAWN_HEIGHT,
                Z_EFFECTS + 1.0,
            ),
        ));
    }
}

/// Numbers float in world space: a room change removes them (they would
/// hang over the new room at the old coordinates).
pub fn clear_on_room_change(
    mut commands: Commands,
    mut changes: MessageReader<RoomChanged>,
    numbers: Query<Entity, With<DamageNumber>>,
) {
    if changes.read().count() == 0 {
        return;
    }
    for entity in &numbers {
        commands.entity(entity).try_despawn();
    }
}

pub fn animate_damage_numbers(
    mut commands: Commands,
    time: Res<Time>,
    mut numbers: Query<(Entity, &mut DamageNumber, &mut Transform, &mut TextColor)>,
) {
    let dt = time.delta_secs();
    for (entity, mut number, mut transform, mut color) in &mut numbers {
        number.remaining -= dt;
        if number.remaining <= 0.0 {
            commands.entity(entity).try_despawn();
            continue;
        }
        transform.translation.y += RISE_SPEED * dt;
        color.0 = color.0.with_alpha(number.remaining / LIFETIME);
    }
}
