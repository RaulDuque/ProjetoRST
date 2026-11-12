//! Temporary visuals: until sprites exist, the player is a coloured box
//! whose colour, squash and "visor" show the current state and facing.

use bevy::prelude::*;

use super::components::{ActionState, Player};
use super::state::PlayerState;
use crate::combat::hitboxes::Invulnerable;
use crate::world::physics::FacingDirection;

/// Small marker showing which way the character faces.
#[derive(Component, Debug, Clone, Copy)]
pub struct Visor;

const VISOR_SIZE: Vec2 = Vec2::new(8.0, 6.0);
const VISOR_OFFSET: Vec2 = Vec2::new(7.0, 12.0);
/// Blink frequency while invulnerable (Hz).
const BLINK_RATE: f32 = 16.0;

fn state_color(state: PlayerState) -> Color {
    match state {
        PlayerState::Idle | PlayerState::Walking => Color::srgb(0.75, 0.2, 0.25),
        PlayerState::Running => Color::srgb(0.85, 0.3, 0.3),
        PlayerState::Crouching => Color::srgb(0.6, 0.15, 0.2),
        PlayerState::Jumping | PlayerState::Falling => Color::srgb(0.8, 0.35, 0.4),
        PlayerState::Attacking => Color::srgb(0.95, 0.9, 0.9),
        PlayerState::Casting => Color::srgb(0.7, 0.5, 1.0),
        PlayerState::Guarding => Color::srgb(0.4, 0.55, 0.85),
        PlayerState::Dodging => Color::srgb(0.95, 0.75, 0.35),
        PlayerState::Hurt => Color::srgb(1.0, 1.0, 1.0),
        PlayerState::Dead => Color::srgb(0.25, 0.25, 0.25),
    }
}

pub fn attach_visor(mut commands: Commands, players: Query<Entity, Added<Player>>) {
    for player in &players {
        commands.entity(player).with_child((
            Visor,
            Sprite::from_color(Color::srgb(1.0, 0.9, 0.5), VISOR_SIZE),
            Transform::from_xyz(VISOR_OFFSET.x, VISOR_OFFSET.y, 0.1),
        ));
    }
}

pub fn update_player_visuals(
    time: Res<Time>,
    players: Query<
        (
            &ActionState,
            &FacingDirection,
            &Invulnerable,
            &Children,
            &mut Sprite,
            &mut Transform,
        ),
        (With<Player>, Without<Visor>),
    >,
    mut visors: Query<&mut Transform, With<Visor>>,
) {
    let blink_on = (time.elapsed_secs() * BLINK_RATE).sin() > 0.0;
    for (state, facing, invulnerable, children, mut sprite, mut transform) in players {
        let mut color = state_color(state.state());
        if invulnerable.remaining > 0.0 && state.state() != PlayerState::Dead && blink_on {
            color = color.with_alpha(0.35);
        }
        sprite.color = color;
        let squash = match state.state() {
            PlayerState::Crouching => 0.65,
            PlayerState::Dead => 0.3,
            _ => 1.0,
        };
        transform.scale = Vec3::new(1.0, squash, 1.0);
        for child in children.iter() {
            if let Ok(mut visor) = visors.get_mut(child) {
                visor.translation.x = VISOR_OFFSET.x * facing.sign();
            }
        }
    }
}
