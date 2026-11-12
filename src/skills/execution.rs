//! Turns a recognised command into a final, validated outcome.
//!
//! Pure module: no Bevy types. The recogniser (`combat::command_parser`)
//! only says which command was input; this module checks whether the
//! character may actually perform it (learned, cooldown, MP, Hearts, state).
//! Applying the effect is done by the combat and magic systems.

use std::fmt;

use super::cooldowns::Cooldowns;
use super::definitions::{SkillDefinition, SkillId, SkillLibrary};
use super::unlock::LearnedSkills;
use crate::combat::command_buffer::TimedInput;
use crate::combat::command_parser::{Recognition, recognize};
use crate::player::state::ActionPhase;

/// The parts of the caster that activation depends on.
#[derive(Debug, Clone, Copy)]
pub struct CasterSnapshot<'a> {
    pub learned: &'a LearnedSkills,
    pub cooldowns: &'a Cooldowns,
    pub mp: i32,
    pub hearts: i32,
    /// `None` when the character cannot act (hurt, dodging, dead, casting).
    pub phase: Option<ActionPhase>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceKind {
    Mp,
    Hearts,
}

/// Every way a command input can end.
#[derive(Debug, Clone, PartialEq)]
pub enum CommandOutcome {
    /// Valid command; the skill must now be executed.
    Executed { id: SkillId },
    /// Some inputs of a command were missing.
    Incomplete {
        id: SkillId,
        matched: usize,
        required: usize,
    },
    /// The inputs do not form any command.
    Invalid,
    /// Correct inputs, but outside the time window.
    TooSlow { id: SkillId },
    /// Not enough MP or Hearts.
    InsufficientResources {
        id: SkillId,
        resource: ResourceKind,
        required: i32,
        available: i32,
    },
    /// The command exists but the character has not learned it.
    NotLearned { id: SkillId },
    /// Learned but still cooling down.
    OnCooldown { id: SkillId, remaining: f32 },
    /// Learned and affordable, but not usable in the current state.
    NotAllowedNow { id: SkillId },
}

impl CommandOutcome {
    pub fn is_executed(&self) -> bool {
        matches!(self, CommandOutcome::Executed { .. })
    }
}

impl fmt::Display for CommandOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandOutcome::Executed { id } => write!(f, "{id}!"),
            CommandOutcome::Incomplete {
                id,
                matched,
                required,
            } => write!(f, "{id}: incomplete ({matched}/{required})"),
            CommandOutcome::Invalid => write!(f, "no command"),
            CommandOutcome::TooSlow { id } => write!(f, "{id}: too slow"),
            CommandOutcome::InsufficientResources {
                id,
                resource,
                required,
                available,
            } => {
                let name = match resource {
                    ResourceKind::Mp => "MP",
                    ResourceKind::Hearts => "Hearts",
                };
                write!(f, "{id}: not enough {name} ({available}/{required})")
            }
            CommandOutcome::NotLearned { id } => write!(f, "{id}: not learned"),
            CommandOutcome::OnCooldown { id, remaining } => {
                write!(f, "{id}: cooldown {remaining:.1}s")
            }
            CommandOutcome::NotAllowedNow { id } => write!(f, "{id}: cannot use now"),
        }
    }
}

/// Validates a known skill against the caster.
pub fn validate_activation(def: &SkillDefinition, caster: &CasterSnapshot<'_>) -> CommandOutcome {
    let id = def.id.clone();
    if !caster.learned.is_learned(&def.id) {
        return CommandOutcome::NotLearned { id };
    }
    let phase_ok = caster.phase.is_some_and(|p| def.usable_in.contains(&p));
    if !phase_ok {
        return CommandOutcome::NotAllowedNow { id };
    }
    if !caster.cooldowns.is_ready(&def.id) {
        return CommandOutcome::OnCooldown {
            remaining: caster.cooldowns.remaining(&def.id),
            id,
        };
    }
    if caster.mp < def.mp_cost {
        return CommandOutcome::InsufficientResources {
            id,
            resource: ResourceKind::Mp,
            required: def.mp_cost,
            available: caster.mp,
        };
    }
    if caster.hearts < def.hearts_cost {
        return CommandOutcome::InsufficientResources {
            id,
            resource: ResourceKind::Hearts,
            required: def.hearts_cost,
            available: caster.hearts,
        };
    }
    CommandOutcome::Executed { id }
}

/// Full pipeline: recognise the newest inputs and validate the result.
///
/// When several commands match, the best *learned* one is validated; if
/// none is learned the best match is reported as [`CommandOutcome::NotLearned`].
pub fn resolve_command(
    inputs: &[TimedInput],
    library: &SkillLibrary,
    caster: &CasterSnapshot<'_>,
) -> CommandOutcome {
    match recognize(inputs, library.patterns()) {
        Recognition::Matched(ids) => {
            let learned = ids
                .iter()
                .filter_map(|id| library.get(id))
                .find(|def| caster.learned.is_learned(&def.id));
            match learned {
                Some(def) => validate_activation(def, caster),
                None => match ids.into_iter().next() {
                    Some(id) => CommandOutcome::NotLearned { id },
                    None => CommandOutcome::Invalid,
                },
            }
        }
        Recognition::Incomplete {
            id,
            matched,
            required,
        } => CommandOutcome::Incomplete {
            id,
            matched,
            required,
        },
        Recognition::TooSlow { id } => CommandOutcome::TooSlow { id },
        Recognition::Invalid => CommandOutcome::Invalid,
    }
}
