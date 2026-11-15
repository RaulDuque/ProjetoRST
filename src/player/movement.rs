//! Player locomotion: facing, walking/running, jumping, crouching, guard
//! and dodge. Physics integration itself is done by `world::physics`.
//!
//! V1.9: a transformation may give the player a [`LocomotionProfile`]; while
//! it is present, free movement follows the profile instead of the human
//! constants. Without it (the human form) the code path is the one of V1.8.

use bevy::prelude::*;

use super::action_rules::steer_in_air;
use super::components::{ActionState, CharacterSheet, JumpControl, Player};
use super::state::{Facing, PlayerAction, PlayerState};
use crate::combat::attacks::ActiveStrike;
use crate::combat::hitboxes::Invulnerable;
use crate::core::constants::{
    COYOTE_TIME, JUMP_CUT_FACTOR, PLAYER_DODGE_SPEED, PLAYER_DODGE_TIME, PLAYER_JUMP_SPEED,
    PLAYER_RUN_SPEED, PLAYER_WALK_SPEED,
};
use crate::core::time::frame_delta;
use crate::input::PlayerInput;
use crate::world::physics::{FacingDirection, Grounded, Velocity};

/// How fast horizontal speed decays while locked in an attack or skill.
const ACTION_FRICTION: f32 = 10.0;

/// Free movement of a transformed body (V1.9): a fixed horizontal speed,
/// no running, no crouching and, unless `can_jump`, no jump. Inserted and
/// removed only by the transformation module.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct LocomotionProfile {
    pub speed: f32,
    pub can_jump: bool,
}

/// Turns the character towards the pressed direction when it may move.
/// Runs before command recording so that motions are interpreted relative
/// to the direction the character faces after the input.
///
/// V2.4: also inside the weapon's `turn_window` at the start of a basic
/// attack. Armed strikes keep their side (`SideLock`).
pub fn update_facing(
    input: Res<PlayerInput>,
    player: Single<(&ActionState, &mut FacingDirection, Option<&ActiveStrike>), With<Player>>,
) {
    let (state, mut facing, strike) = player.into_inner();
    let may_turn =
        state.state().allows(PlayerAction::Move) || ActiveStrike::may_turn(state, strike);
    if !may_turn || input.axis_x == 0 {
        return;
    }
    let wanted = if input.axis_x > 0 {
        Facing::Right
    } else {
        Facing::Left
    };
    if facing.0 != wanted {
        facing.0 = wanted;
    }
}

/// Guard and dodge (action states driven directly by buttons).
pub fn guard_and_dodge(
    input: Res<PlayerInput>,
    player: Single<
        (
            &mut ActionState,
            &mut Velocity,
            &mut Invulnerable,
            &FacingDirection,
            &Grounded,
            Option<&ActiveStrike>,
        ),
        With<Player>,
    >,
) {
    let (mut state, mut velocity, mut invulnerable, facing, grounded, strike) = player.into_inner();

    if state.state() == PlayerState::Guarding && (!input.guard_held || !grounded.on_ground) {
        state.stop_guard();
    } else if input.guard_held && grounded.on_ground {
        ActiveStrike::try_cancel(&mut state, strike, PlayerAction::Guard);
        if state.start_guard().is_ok() {
            velocity.x = 0.0;
        }
    }

    if input.dodge_pressed && grounded.on_ground {
        ActiveStrike::try_cancel(&mut state, strike, PlayerAction::Dodge);
    }
    if input.dodge_pressed
        && grounded.on_ground
        && state
            .start_action(PlayerState::Dodging, PLAYER_DODGE_TIME)
            .is_ok()
    {
        invulnerable.remaining = invulnerable.remaining.max(PLAYER_DODGE_TIME);
        velocity.x = facing.sign() * PLAYER_DODGE_SPEED;
    }
}

/// Horizontal movement, jumping and the locomotion state.
pub fn locomotion(
    time: Res<Time>,
    input: Res<PlayerInput>,
    player: Single<
        (
            &mut ActionState,
            &mut Velocity,
            &mut JumpControl,
            &FacingDirection,
            &Grounded,
            &CharacterSheet,
            Option<&LocomotionProfile>,
            Option<&ActiveStrike>,
        ),
        With<Player>,
    >,
) {
    let dt = frame_delta(&time);
    let (mut state, mut velocity, mut jump, facing, grounded, sheet, profile, strike) =
        player.into_inner();
    let can_jump = grounded.on_ground || grounded.air_time < COYOTE_TIME;
    if input.jump_pressed && can_jump && velocity.y <= 0.0 {
        ActiveStrike::try_cancel(&mut state, strike, PlayerAction::Jump);
    }
    let current = state.state();

    match current {
        PlayerState::Dodging => {
            velocity.x = facing.sign() * PLAYER_DODGE_SPEED;
            return;
        }
        PlayerState::Hurt => return,
        PlayerState::Dead | PlayerState::Guarding => {
            velocity.x = 0.0;
            return;
        }
        PlayerState::Attacking | PlayerState::Casting => {
            if grounded.on_ground {
                velocity.x *= 1.0 - (ACTION_FRICTION * dt).min(1.0);
            } else if let Some(control) = ActiveStrike::air_control(&state, strike) {
                let speed = profile.map_or_else(|| human_speed(&input, sheet), |p| p.speed);
                let wanted = f32::from(input.axis_x) * speed;
                velocity.x = steer_in_air(velocity.x, wanted, control, dt);
            }
            return;
        }
        _ => {}
    }

    if let Some(profile) = profile {
        transformed_locomotion(profile, &input, &mut state, &mut velocity, grounded);
        return;
    }

    let crouching = grounded.on_ground && input.axis_y < 0;
    let speed = human_speed(&input, sheet);
    velocity.x = if crouching {
        0.0
    } else {
        f32::from(input.axis_x) * speed
    };

    if input.jump_pressed && can_jump && velocity.y <= 0.0 {
        velocity.y = PLAYER_JUMP_SPEED;
        jump.cut_applied = false;
    } else if !input.jump_held && velocity.y > 0.0 && !jump.cut_applied {
        velocity.y *= JUMP_CUT_FACTOR;
        jump.cut_applied = true;
    }

    let next = if velocity.y > 0.0 && !grounded.on_ground || velocity.y > 0.0 && input.jump_pressed
    {
        PlayerState::Jumping
    } else if !grounded.on_ground {
        PlayerState::Falling
    } else if crouching {
        PlayerState::Crouching
    } else if input.axis_x != 0 && input.run_held {
        PlayerState::Running
    } else if input.axis_x != 0 {
        PlayerState::Walking
    } else {
        PlayerState::Idle
    };
    state.set_locomotion(next);
}

/// Walking or running speed of the human form.
fn human_speed(input: &PlayerInput, sheet: &CharacterSheet) -> f32 {
    let base = if input.run_held {
        PLAYER_RUN_SPEED
    } else {
        PLAYER_WALK_SPEED
    };
    base * sheet.derived().movement_speed
}

/// Free movement under a [`LocomotionProfile`] (same states as the human
/// form, so animation, hurt and death keep working).
fn transformed_locomotion(
    profile: &LocomotionProfile,
    input: &PlayerInput,
    state: &mut ActionState,
    velocity: &mut Velocity,
    grounded: &Grounded,
) {
    velocity.x = f32::from(input.axis_x) * profile.speed;
    if profile.can_jump && input.jump_pressed && grounded.on_ground && velocity.y <= 0.0 {
        velocity.y = PLAYER_JUMP_SPEED;
    }
    let next = if !grounded.on_ground {
        if velocity.y > 0.0 {
            PlayerState::Jumping
        } else {
            PlayerState::Falling
        }
    } else if input.axis_x != 0 {
        PlayerState::Walking
    } else {
        PlayerState::Idle
    };
    state.set_locomotion(next);
}
