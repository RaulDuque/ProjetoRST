//! V2.4: what the player may do during each phase of a basic attack.
//!
//! Pure module: no Bevy types. One table, [`while_attacking`], answers for
//! every intent during `PlayerState::Attacking`; outside an attack
//! `PlayerState::allows` keeps answering as before. With the default data
//! (no `cancel_after`) the table reproduces the pre-V2.4 rules exactly:
//! only skills whose `usable_in` lists `Attacking` interrupt a basic
//! attack, at any moment. Docs: docs/COMBAT_FEEL_V2_4.md.

use super::state::PlayerAction;
use crate::combat::strike_timeline::StrikePhase;
use crate::core::constants::AIR_CONTROL_REFERENCE_FRAME;

/// Something the player tries to do while a basic attack runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttackIntent {
    /// Movement-type intents of the state machine.
    Action(PlayerAction),
    /// A technique, validated by `begin_skill`.
    Technique,
    /// A spell, validated by `begin_skill`.
    Spell,
}

/// Answer of the table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    Denied,
    /// Allowed: the attack is cancelled first.
    Allowed,
    /// Decided by the skill's own data (`usable_in` contains `Attacking`),
    /// in every phase, as before V2.4.
    ByData,
}

/// What `intent` may do during `phase` of a basic attack, given whether
/// the weapon's cancel point has been reached.
///
/// After the cancel point (and never before the active window has ended),
/// jump, dodge, guard and a new basic attack cancel the strike. Walking,
/// crouching and interacting stay locked for the whole action.
pub fn while_attacking(phase: StrikePhase, cancelable: bool, intent: AttackIntent) -> Permission {
    match intent {
        AttackIntent::Technique | AttackIntent::Spell => Permission::ByData,
        AttackIntent::Action(action) => match action {
            PlayerAction::Jump
            | PlayerAction::Dodge
            | PlayerAction::Guard
            | PlayerAction::Attack
                if cancelable && phase == StrikePhase::Recovery =>
            {
                Permission::Allowed
            }
            _ => Permission::Denied,
        },
    }
}

/// Shorthand: true when `while_attacking` answers `Allowed`.
pub fn cancels_attack(phase: StrikePhase, cancelable: bool, action: PlayerAction) -> bool {
    while_attacking(phase, cancelable, AttackIntent::Action(action)) == Permission::Allowed
}

/// Horizontal speed after one frame of air control during an attack.
///
/// `air_control` 0 keeps `current` (momentum, as before V2.4); 1 reaches
/// `wanted` at once, like a normal jump; values in between close that share
/// of the gap per [`AIR_CONTROL_REFERENCE_FRAME`].
pub fn steer_in_air(current: f32, wanted: f32, air_control: f32, dt: f32) -> f32 {
    let control = air_control.clamp(0.0, 1.0);
    if control <= 0.0 || dt <= 0.0 {
        return current;
    }
    if control >= 1.0 {
        return wanted;
    }
    let share = 1.0 - (1.0 - control).powf(dt / AIR_CONTROL_REFERENCE_FRAME);
    current + (wanted - current) * share
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::state::{PlayerState, PlayerStateMachine};

    const PHASES: [StrikePhase; 3] = [
        StrikePhase::Startup,
        StrikePhase::Active,
        StrikePhase::Recovery,
    ];
    const ACTIONS: [PlayerAction; 7] = [
        PlayerAction::Move,
        PlayerAction::Jump,
        PlayerAction::Crouch,
        PlayerAction::Attack,
        PlayerAction::Guard,
        PlayerAction::Dodge,
        PlayerAction::Interact,
    ];

    #[test]
    fn without_a_cancel_point_the_table_matches_the_state_machine() {
        let mut attacking = PlayerStateMachine::default();
        assert!(attacking.start_action(PlayerState::Attacking, 0.28).is_ok());
        for phase in PHASES {
            for action in ACTIONS {
                let table = while_attacking(phase, false, AttackIntent::Action(action));
                let machine = attacking.state().allows(action);
                assert_eq!(table, Permission::Denied, "{phase:?} {action:?}");
                assert!(!machine, "{action:?} was allowed by allows()");
            }
            // Skills keep deciding by `usable_in`, and the machine accepts
            // Casting from Attacking at any moment.
            for intent in [AttackIntent::Technique, AttackIntent::Spell] {
                assert_eq!(while_attacking(phase, false, intent), Permission::ByData);
            }
            let mut copy = attacking;
            assert!(copy.start_action(PlayerState::Casting, 0.4).is_ok());
        }
    }

    #[test]
    fn the_cancel_point_frees_jump_dodge_guard_and_attack_only() {
        for action in ACTIONS {
            let freed = matches!(
                action,
                PlayerAction::Jump
                    | PlayerAction::Dodge
                    | PlayerAction::Guard
                    | PlayerAction::Attack
            );
            assert_eq!(
                cancels_attack(StrikePhase::Recovery, true, action),
                freed,
                "{action:?}"
            );
        }
    }

    #[test]
    fn air_control_zero_keeps_momentum_and_one_steers_fully() {
        let frame = AIR_CONTROL_REFERENCE_FRAME;
        assert_eq!(steer_in_air(190.0, -190.0, 0.0, frame), 190.0);
        assert_eq!(steer_in_air(190.0, -190.0, 1.0, frame), -190.0);
        assert_eq!(steer_in_air(190.0, -190.0, 0.5, 0.0), 190.0);
        let half = steer_in_air(100.0, 0.0, 0.5, frame);
        assert!((half - 50.0).abs() < 1e-4);
        // Two half frames close the same gap as one full frame.
        let twice = steer_in_air(
            steer_in_air(100.0, 0.0, 0.5, frame / 2.0),
            0.0,
            0.5,
            frame / 2.0,
        );
        assert!((twice - half).abs() < 1e-3);
    }

    #[test]
    fn nothing_but_skills_cancels_before_the_active_window_ends() {
        for phase in [StrikePhase::Startup, StrikePhase::Active] {
            for action in ACTIONS {
                assert!(!cancels_attack(phase, true, action), "{phase:?} {action:?}");
            }
        }
    }
}
