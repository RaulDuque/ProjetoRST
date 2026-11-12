//! Records the player's inputs into the command buffer and resolves a
//! command whenever an action button is pressed.
//!
//! This system only *classifies* the input. A recognised command becomes a
//! `SkillActivated` request; `skills::systems::begin_skill` re-validates it
//! with the current state and reports the final `CommandResolved`. Every
//! other outcome is reported here as `CommandResolved`. Effects are applied
//! by `combat::attacks` and `magic::casting`.

use bevy::prelude::*;

use super::command_buffer::{CommandButton, Motion, TimedInput};
use crate::core::events::{BasicAttackRequested, CommandResolved, SkillActivated};
use crate::core::resources::GameData;
use crate::input::PlayerInput;
use crate::player::components::{ActionState, CommandInput, Player, SkillBook, Vitals};
use crate::skills::execution::{CasterSnapshot, CommandOutcome, resolve_command};
use crate::world::physics::FacingDirection;

pub fn record_and_resolve_commands(
    time: Res<Time>,
    input: Res<PlayerInput>,
    data: Res<GameData>,
    player: Single<
        (
            Entity,
            &mut CommandInput,
            &FacingDirection,
            &ActionState,
            &SkillBook,
            &Vitals,
        ),
        With<Player>,
    >,
    mut resolved: MessageWriter<CommandResolved>,
    mut activated: MessageWriter<SkillActivated>,
    mut basic: MessageWriter<BasicAttackRequested>,
) {
    let (entity, mut buffer, facing, state, book, vitals) = player.into_inner();
    let now = time.elapsed_secs_f64();

    buffer.prune(now);
    buffer.record_motion(Motion::from_axes(input.axis_x, input.axis_y, facing.0), now);

    // Guarding cancels any half-entered command.
    if input.guard_pressed {
        buffer.clear();
    }

    let button = if input.attack_pressed {
        CommandButton::Attack
    } else if input.magic_pressed {
        CommandButton::Magic
    } else {
        return;
    };

    buffer.record_button(button, now);
    let entries: Vec<TimedInput> = buffer.entries().copied().collect();
    let caster = CasterSnapshot {
        learned: &book.learned,
        cooldowns: &book.cooldowns,
        mp: vitals.mp.current(),
        hearts: vitals.hearts.current(),
        phase: state.state().phase(),
    };
    let outcome = resolve_command(&entries, &data.skills, &caster);
    // Every press consumes the buffer: the same inputs never fire twice.
    buffer.clear();

    // `Executed` here is provisional: `begin_skill` re-validates it.
    debug!(?outcome, "command recognised");
    match &outcome {
        // Only a request: `begin_skill` re-validates it and writes the single
        // `CommandResolved` of this activation.
        CommandOutcome::Executed { id } => {
            activated.write(SkillActivated {
                caster: entity,
                skill_id: id.clone(),
            });
            return;
        }
        _ if button == CommandButton::Attack => {
            basic.write(BasicAttackRequested { attacker: entity });
        }
        _ => {}
    }
    resolved.write(CommandResolved { outcome });
}
