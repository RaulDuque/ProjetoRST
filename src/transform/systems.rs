//! Bevy systems of the transformation (V1.9).
//!
//! Responsibilities, each in its own system:
//!
//! | System | Set | Does |
//! |---|---|---|
//! | [`attach_form_state`] | Input | gives the player its [`Transformation`] |
//! | [`mask_form_input`] | Input | clears the inputs of blocked actions |
//! | [`toggle_form`] | Actions | the key: activation (MP paid once) or an end request |
//! | [`upkeep_form`] | Actions | MP per second; asks to end when it cannot be paid |
//! | [`settle_form`] | Actions | ends a pending end once the human body fits |
//! | [`end_form_on_death`] | Resolution | death ends it (the body is held if inside a veil) |
//! | [`end_form_on_load`] | Rooms/React | a load restores the human form |
//!
//! The form only *decides*. Movement, gravity and collision apply it through
//! [`LocomotionProfile`], [`GravityProfile`] and [`Phasing`], inserted when
//! the form starts and removed in the same system that ends it: physics and
//! state never disagree. Visuals read the same [`Transformation`].

use bevy::prelude::*;

use super::definitions::{FormAction, TransformationDefinition};
use super::state::{ActivateError, ActivationContext, EndReason, FormState, check_activation};
use crate::core::events::{FormChange, FormChanged, Notification, RoomChangeCause, RoomChanged};
use crate::core::resources::GameData;
use crate::input::PlayerInput;
use crate::player::components::{ActionState, Player, Vitals};
use crate::player::movement::LocomotionProfile;
use crate::player::state::PlayerState;
use crate::world::exploration_systems::Exploration;
use crate::world::physics::{Body, GravityProfile, Grounded, Phasing, RoomGeometry, Velocity};
use crate::world::transitions::TransitionLock;

/// The player's transformation state.
#[derive(Component, Debug, Clone, Default, Deref, DerefMut)]
pub struct Transformation(pub FormState);

/// Gravity of a dead body still inside a veil: held where it fell until it
/// is revived (a floor veil would otherwise let it sink out of the room).
const HELD: GravityProfile = GravityProfile {
    scale: 0.0,
    max_fall_speed: 0.0,
};

fn profiles(def: &TransformationDefinition) -> (LocomotionProfile, GravityProfile, Phasing) {
    (
        LocomotionProfile {
            speed: def.movement.speed,
            can_jump: def.allows(FormAction::Jump),
        },
        GravityProfile {
            scale: def.movement.gravity_scale,
            max_fall_speed: def.movement.max_fall_speed,
        },
        Phasing {
            passes: def.collision.passes.clone(),
        },
    )
}

fn remove_profiles(commands: &mut Commands, player: Entity) {
    commands
        .entity(player)
        .remove::<(LocomotionProfile, GravityProfile, Phasing)>();
}

/// Whether a human body at `transform` overlaps no solid (veils included).
fn human_fits(geometry: &RoomGeometry, body: &Body, transform: &Transform) -> bool {
    !geometry.blocks(&body.aabb_at(transform.translation.truncate()))
}

/// Text of a refused activation.
pub fn refusal_text(def: &TransformationDefinition, error: ActivateError) -> String {
    let name = &def.name;
    match error {
        ActivateError::Locked => format!("{name}: needs the {}", def.unlock_hint),
        ActivateError::Dead => format!("{name}: not while fallen"),
        ActivateError::Busy(_) => format!("{name}: cannot transform now"),
        ActivateError::Airborne => format!("{name}: stand on the ground"),
        ActivateError::InTransition => format!("{name}: wait for the passage to settle"),
        ActivateError::NotEnoughMp { need, have } => {
            format!("{name}: not enough MP ({have}/{need})")
        }
    }
}

pub fn attach_form_state(
    mut commands: Commands,
    players: Query<Entity, (With<Player>, Without<Transformation>)>,
) {
    for player in &players {
        commands.entity(player).insert(Transformation::default());
    }
}

/// Inputs of the actions the active form blocks are cleared before any
/// gameplay system reads them (commands, attacks, spells, guard, dodge,
/// objects). The human form keeps every input.
pub fn mask_form_input(
    data: Res<GameData>,
    mut input: ResMut<PlayerInput>,
    player: Query<&Transformation, With<Player>>,
) {
    let Some(def) = player
        .single()
        .ok()
        .and_then(|t| t.active())
        .and_then(|form| data.transformations.get(form))
    else {
        return;
    };
    let blocked = |a| !def.allows(a);
    if blocked(FormAction::Jump) {
        input.jump_pressed = false;
        input.jump_held = false;
    }
    if blocked(FormAction::Run) {
        input.run_held = false;
    }
    if blocked(FormAction::Attack) {
        input.attack_pressed = false;
    }
    if blocked(FormAction::Spell) {
        input.magic_pressed = false;
    }
    if blocked(FormAction::Guard) {
        input.guard_pressed = false;
        input.guard_held = false;
    }
    if blocked(FormAction::Dodge) {
        input.dodge_pressed = false;
    }
    if blocked(FormAction::Interact) {
        input.interact_pressed = false;
    }
    if blocked(FormAction::Crouch) && input.axis_y < 0 {
        input.axis_y = 0;
    }
}

/// The transformation key: starts the form (paying its activation once) or
/// asks it to end. A refused start says why.
pub fn toggle_form(
    mut commands: Commands,
    data: Res<GameData>,
    input: Res<PlayerInput>,
    exploration: Option<Res<Exploration>>,
    lock: Res<TransitionLock>,
    player: Query<
        (
            Entity,
            &mut Transformation,
            &mut Vitals,
            &ActionState,
            &Grounded,
        ),
        With<Player>,
    >,
    mut changed: MessageWriter<FormChanged>,
    mut notifications: MessageWriter<Notification>,
) {
    if !input.transform_pressed {
        return;
    }
    let Some(def) = data.transformations.primary() else {
        return;
    };
    let mut player = player;
    let Ok((entity, mut form, mut vitals, action, grounded)) = player.single_mut() else {
        return;
    };
    if form.is_active() {
        if def.cancel.voluntary && form.request_end(EndReason::Voluntary) {
            changed.write(FormChanged {
                form: def.id,
                change: FormChange::Ending(EndReason::Voluntary),
            });
        }
        return;
    }
    let ctx = ActivationContext {
        unlocked: exploration.is_some_and(|e| def.unlock.is_met(&e.capabilities())),
        state: action.state(),
        grounded: grounded.on_ground,
        in_transition: lock.is_locked(),
        mp: vitals.mp.current(),
    };
    match check_activation(def, ctx) {
        Ok(()) => {
            let cost = form.activate(def);
            vitals.mp.deplete(cost);
            commands.entity(entity).insert(profiles(def));
            changed.write(FormChanged {
                form: def.id,
                change: FormChange::Entered,
            });
        }
        Err(error) => {
            notifications.write(Notification(refusal_text(def, error)));
            changed.write(FormChanged {
                form: def.id,
                change: FormChange::Refused(error),
            });
        }
    }
}

/// Takes the MP upkeep; when it cannot be paid, asks the form to end.
pub fn upkeep_form(
    time: Res<Time>,
    data: Res<GameData>,
    player: Query<(&mut Transformation, &mut Vitals), With<Player>>,
    mut changed: MessageWriter<FormChanged>,
) {
    let mut player = player;
    let Ok((mut form, mut vitals)) = player.single_mut() else {
        return;
    };
    let Some(def) = form.active().and_then(|f| data.transformations.get(f)) else {
        return;
    };
    let owed = form.upkeep(def, crate::core::time::frame_delta(&time));
    if owed <= 0 {
        return;
    }
    let paid = owed.min(vitals.mp.current());
    vitals.mp.deplete(paid);
    form.record_paid(paid);
    if paid < owed && def.cancel.on_mp_empty && form.request_end(EndReason::MpEmpty) {
        changed.write(FormChanged {
            form: def.id,
            change: FormChange::Ending(EndReason::MpEmpty),
        });
    }
}

/// Ends a pending end once the human body fits where the player is (never
/// inside a veil). Until then the form keeps its physics and look.
pub fn settle_form(
    mut commands: Commands,
    geometry: Res<RoomGeometry>,
    player: Query<(Entity, &mut Transformation, &Body, &Transform), With<Player>>,
    mut changed: MessageWriter<FormChanged>,
) {
    let mut player = player;
    let Ok((entity, mut form, body, transform)) = player.single_mut() else {
        return;
    };
    if form.ending().is_none() {
        return;
    }
    if let Some((id, reason)) = form.settle(human_fits(&geometry, body, transform)) {
        remove_profiles(&mut commands, entity);
        changed.write(FormChanged {
            form: id,
            change: FormChange::Left(reason),
        });
    }
}

/// Death ends the form: at once when the human body fits, otherwise the
/// body stays phased and is held still until it is revived (respawn moves
/// it to the room's spawn, where `settle_form` ends the form).
pub fn end_form_on_death(
    mut commands: Commands,
    geometry: Res<RoomGeometry>,
    player: Query<
        (
            Entity,
            &mut Transformation,
            &ActionState,
            &Body,
            &Transform,
            &mut Velocity,
        ),
        With<Player>,
    >,
    mut changed: MessageWriter<FormChanged>,
) {
    let mut player = player;
    let Ok((entity, mut form, action, body, transform, mut velocity)) = player.single_mut() else {
        return;
    };
    if action.state() != PlayerState::Dead || !form.request_end(EndReason::Death) {
        return;
    }
    let Some(id) = form.active() else {
        return;
    };
    changed.write(FormChanged {
        form: id,
        change: FormChange::Ending(EndReason::Death),
    });
    if let Some((id, reason)) = form.settle(human_fits(&geometry, body, transform)) {
        remove_profiles(&mut commands, entity);
        changed.write(FormChanged {
            form: id,
            change: FormChange::Left(reason),
        });
    } else {
        velocity.0 = Vec2::ZERO;
        commands.entity(entity).insert(HELD);
    }
}

/// A load restores the human form. The loaded position was validated
/// against the room's solids, veils included (`world::restore`), and the
/// quick save refuses positions where a human would not fit.
pub fn end_form_on_load(
    mut commands: Commands,
    mut rooms: MessageReader<RoomChanged>,
    player: Query<(Entity, &mut Transformation), With<Player>>,
    mut changed: MessageWriter<FormChanged>,
) {
    if !rooms.read().any(|r| r.cause == RoomChangeCause::Load) {
        return;
    }
    let mut player = player;
    let Ok((entity, mut form)) = player.single_mut() else {
        return;
    };
    form.request_end(EndReason::Load);
    if let Some((id, reason)) = form.settle(true) {
        remove_profiles(&mut commands, entity);
        changed.write(FormChanged {
            form: id,
            change: FormChange::Left(reason),
        });
    }
}
