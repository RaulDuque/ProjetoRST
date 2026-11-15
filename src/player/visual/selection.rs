//! Which sprite clip shows a gameplay state. Pure module: no Bevy types.
//!
//! The visual state always follows the gameplay state machine
//! (`player::state`); it never changes it. The machine already guarantees
//! the priorities (dead > hurt > dodge > attack/cast > guard > locomotion:
//! an action state locks locomotion until it ends), so the selection is a
//! direct mapping, plus two facts the state alone does not carry:
//!
//! - which skill is being performed while `Casting` (its `animation` key,
//!   from the skill/spell data, read from the `SkillStarted` message);
//! - whether the player has just landed (`Falling` → `Idle`, detected by
//!   the presentation layer, see `systems`);
//! - whether a basic attack happens in the air (V1.5: the ground contact,
//!   read from the physics, picks `attack_basic_air`).
//!
//! A clip missing from the manifest falls back to the first idle frame with
//! the plan's tint, instead of pretending to perform an action.

use super::super::state::PlayerState;

/// Clip identifiers (manifest keys; docs/ASSET_CONTRACT.md, section 2).
pub const IDLE: &str = "idle";
pub const RUN: &str = "run";
pub const ATTACK_BASIC: &str = "attack_basic";
/// V1.5: the basic attack with airborne legs (same gameplay action and duration).
pub const ATTACK_BASIC_AIR: &str = "attack_basic_air";
pub const DODGE: &str = "dodge";
pub const HURT: &str = "hurt";
pub const DEATH: &str = "death";
pub const JUMP_RISE: &str = "jump_rise";
pub const FALL: &str = "fall";
pub const LAND: &str = "land";
pub const CROUCH: &str = "crouch";
pub const GUARD: &str = "guard_start";
/// Placeholder key while casting a skill whose animation is unknown.
pub const UNKNOWN_SKILL: &str = "";

/// How the chosen clip is timed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Timing {
    /// Loops; locomotion loops scale with the horizontal speed.
    Loop { speed_scaled: bool },
    /// Follows the progress of the current gameplay action.
    Action,
    /// Plays once from the moment the state began and holds the last frame.
    Once,
    /// Holds one frame (fallback for clips the manifest does not have).
    Hold,
}

/// Colour feedback used when a state has no clip of its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tint {
    None,
    Hurt,
    Casting,
    Guarding,
    Dead,
}

impl Tint {
    /// RGB multiplier applied to the sprite colour.
    pub fn rgb(self) -> (f32, f32, f32) {
        match self {
            Tint::None => (1.0, 1.0, 1.0),
            Tint::Hurt => (1.0, 0.55, 0.55),
            Tint::Casting => (0.85, 0.75, 1.0),
            Tint::Guarding => (0.75, 0.85, 1.0),
            Tint::Dead => (0.35, 0.35, 0.4),
        }
    }
}

/// Facts the gameplay state does not carry (see the module docs).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct SelectionContext<'a> {
    /// `animation` key of the skill being performed (only read while casting).
    pub skill_animation: Option<&'a str>,
    /// The player landed and is standing still: show the landing clip.
    pub landing: bool,
    /// The player is off the ground (and the manifest has the air variant).
    pub airborne_attack: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VisualPlan<'a> {
    pub clip: &'a str,
    pub timing: Timing,
    /// Tint shown on the idle fallback if the manifest lacks `clip`.
    pub missing_tint: Tint,
}

const fn plan(clip: &str, timing: Timing, missing_tint: Tint) -> VisualPlan<'_> {
    VisualPlan {
        clip,
        timing,
        missing_tint,
    }
}

/// The visual plan of a gameplay state.
pub fn select<'a>(state: PlayerState, ctx: &SelectionContext<'a>) -> VisualPlan<'a> {
    match state {
        PlayerState::Idle if ctx.landing => plan(LAND, Timing::Once, Tint::None),
        PlayerState::Idle => plan(
            IDLE,
            Timing::Loop {
                speed_scaled: false,
            },
            Tint::None,
        ),
        // The game's normal ground speed (190 px/s) is a run at this scale;
        // the cycle plays faster with the run button (310 px/s).
        PlayerState::Walking | PlayerState::Running => {
            plan(RUN, Timing::Loop { speed_scaled: true }, Tint::None)
        }
        PlayerState::Crouching => plan(CROUCH, Timing::Once, Tint::None),
        PlayerState::Jumping => plan(JUMP_RISE, Timing::Once, Tint::None),
        PlayerState::Falling => plan(
            FALL,
            Timing::Loop {
                speed_scaled: false,
            },
            Tint::None,
        ),
        PlayerState::Attacking if ctx.airborne_attack => {
            plan(ATTACK_BASIC_AIR, Timing::Action, Tint::None)
        }
        PlayerState::Attacking => plan(ATTACK_BASIC, Timing::Action, Tint::None),
        PlayerState::Casting => plan(
            ctx.skill_animation.unwrap_or(UNKNOWN_SKILL),
            Timing::Action,
            Tint::Casting,
        ),
        PlayerState::Guarding => plan(GUARD, Timing::Once, Tint::Guarding),
        PlayerState::Dodging => plan(DODGE, Timing::Action, Tint::None),
        PlayerState::Hurt => plan(HURT, Timing::Action, Tint::Hurt),
        PlayerState::Dead => plan(DEATH, Timing::Once, Tint::Dead),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::state::PlayerStateMachine;

    fn none() -> SelectionContext<'static> {
        SelectionContext::default()
    }

    #[test]
    fn every_state_has_its_clip() {
        for (state, clip, timing) in [
            (
                PlayerState::Idle,
                IDLE,
                Timing::Loop {
                    speed_scaled: false,
                },
            ),
            (
                PlayerState::Walking,
                RUN,
                Timing::Loop { speed_scaled: true },
            ),
            (
                PlayerState::Running,
                RUN,
                Timing::Loop { speed_scaled: true },
            ),
            (PlayerState::Crouching, CROUCH, Timing::Once),
            (PlayerState::Jumping, JUMP_RISE, Timing::Once),
            (
                PlayerState::Falling,
                FALL,
                Timing::Loop {
                    speed_scaled: false,
                },
            ),
            (PlayerState::Attacking, ATTACK_BASIC, Timing::Action),
            (PlayerState::Guarding, GUARD, Timing::Once),
            (PlayerState::Dodging, DODGE, Timing::Action),
            (PlayerState::Hurt, HURT, Timing::Action),
            (PlayerState::Dead, DEATH, Timing::Once),
        ] {
            let p = select(state, &none());
            assert_eq!((p.clip, p.timing), (clip, timing), "{state:?}");
        }
    }

    #[test]
    fn casting_shows_the_skill_animation_from_the_data() {
        let ctx = SelectionContext {
            skill_animation: Some("slash_crescent"),
            landing: false,
            airborne_attack: false,
        };
        let p = select(PlayerState::Casting, &ctx);
        assert_eq!((p.clip, p.timing), ("slash_crescent", Timing::Action));
        // Unknown skill: no clip of its own, the tinted idle fallback applies.
        let p = select(PlayerState::Casting, &none());
        assert_eq!((p.clip, p.missing_tint), (UNKNOWN_SKILL, Tint::Casting));
    }

    #[test]
    fn landing_only_replaces_idle() {
        let ctx = SelectionContext {
            skill_animation: None,
            landing: true,
            airborne_attack: false,
        };
        assert_eq!(select(PlayerState::Idle, &ctx).clip, LAND);
        assert_eq!(select(PlayerState::Running, &ctx).clip, RUN);
        assert_eq!(select(PlayerState::Attacking, &ctx).clip, ATTACK_BASIC);
    }

    #[test]
    fn an_attack_in_the_air_uses_the_air_variant_only_while_attacking() {
        let ctx = SelectionContext {
            airborne_attack: true,
            ..SelectionContext::default()
        };
        let p = select(PlayerState::Attacking, &ctx);
        assert_eq!((p.clip, p.timing), (ATTACK_BASIC_AIR, Timing::Action));
        // The flag never forces an air clip on other states (falling stays fall).
        assert_eq!(select(PlayerState::Falling, &ctx).clip, FALL);
        assert_eq!(select(PlayerState::Hurt, &ctx).clip, HURT);
        let ctx = SelectionContext {
            skill_animation: Some("slash_rising"),
            airborne_attack: true,
            ..SelectionContext::default()
        };
        assert_eq!(select(PlayerState::Casting, &ctx).clip, "slash_rising");
    }

    #[test]
    fn selection_follows_the_machine_priorities() {
        // Hurt interrupts an attack: the visual follows (hurt clip).
        let mut sm = PlayerStateMachine::default();
        assert!(sm.start_action(PlayerState::Attacking, 0.28).is_ok());
        assert_eq!(select(sm.state(), &none()).clip, ATTACK_BASIC);
        assert!(sm.hurt(0.35));
        assert_eq!(select(sm.state(), &none()).clip, HURT);
        // Locomotion cannot override an action in progress.
        let mut sm = PlayerStateMachine::default();
        assert!(sm.start_action(PlayerState::Dodging, 0.28).is_ok());
        sm.set_locomotion(PlayerState::Running);
        assert_eq!(select(sm.state(), &none()).clip, DODGE);
        // Death during a skill wins and the visual cannot revive.
        let mut sm = PlayerStateMachine::default();
        assert!(sm.start_action(PlayerState::Casting, 0.4).is_ok());
        sm.die();
        let ctx = SelectionContext {
            skill_animation: Some("slash_crescent"),
            landing: false,
            airborne_attack: false,
        };
        assert_eq!(select(sm.state(), &ctx).clip, DEATH);
        sm.revive();
        assert_eq!(select(sm.state(), &none()).clip, IDLE);
        // When an action ends the machine returns to idle, and so does the visual.
        let mut sm = PlayerStateMachine::default();
        assert!(sm.start_action(PlayerState::Attacking, 0.28).is_ok());
        assert!(sm.tick(0.3));
        assert_eq!(select(sm.state(), &none()).clip, IDLE);
    }
}
