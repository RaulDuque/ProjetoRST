//! Player state machine.
//!
//! Pure module: no Bevy types. It guarantees that incompatible actions never
//! overlap (e.g. attacking while dead, casting while hurt). Locomotion states
//! (idle, walking, running, crouching, jumping, falling) are chosen freely
//! from physics every frame; action states (attacking, casting, guarding,
//! dodging, hurt, dead) lock the character until they finish.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Direction the character is facing. Used to translate absolute input
/// directions into forward/back for command recognition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Facing {
    Left,
    #[default]
    Right,
}

impl Facing {
    /// `+1.0` when facing right, `-1.0` when facing left.
    pub fn sign(self) -> f32 {
        match self {
            Facing::Left => -1.0,
            Facing::Right => 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum PlayerState {
    #[default]
    Idle,
    Walking,
    Running,
    Crouching,
    Jumping,
    Falling,
    Attacking,
    Casting,
    Guarding,
    Dodging,
    Hurt,
    Dead,
}

/// Coarse phase used by data files to say when a skill may be performed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionPhase {
    /// Standing, walking, running or crouching on the ground.
    Grounded,
    /// Jumping or falling.
    Airborne,
    /// During a basic attack (lets skills cancel it).
    Attacking,
    Guarding,
}

/// High-level intents checked against the current state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerAction {
    Move,
    Jump,
    Crouch,
    Attack,
    Guard,
    Dodge,
    Interact,
}

impl PlayerState {
    pub fn is_locomotion(self) -> bool {
        matches!(
            self,
            PlayerState::Idle
                | PlayerState::Walking
                | PlayerState::Running
                | PlayerState::Crouching
                | PlayerState::Jumping
                | PlayerState::Falling
        )
    }

    pub fn is_airborne(self) -> bool {
        matches!(self, PlayerState::Jumping | PlayerState::Falling)
    }

    pub fn phase(self) -> Option<ActionPhase> {
        match self {
            PlayerState::Idle
            | PlayerState::Walking
            | PlayerState::Running
            | PlayerState::Crouching => Some(ActionPhase::Grounded),
            PlayerState::Jumping | PlayerState::Falling => Some(ActionPhase::Airborne),
            PlayerState::Attacking => Some(ActionPhase::Attacking),
            PlayerState::Guarding => Some(ActionPhase::Guarding),
            PlayerState::Casting | PlayerState::Dodging | PlayerState::Hurt | PlayerState::Dead => {
                None
            }
        }
    }

    /// Whether `action` may start in this state.
    pub fn allows(self, action: PlayerAction) -> bool {
        match action {
            PlayerAction::Move => self.is_locomotion(),
            PlayerAction::Jump => {
                matches!(
                    self,
                    PlayerState::Idle
                        | PlayerState::Walking
                        | PlayerState::Running
                        | PlayerState::Crouching
                )
            }
            PlayerAction::Crouch => {
                matches!(
                    self,
                    PlayerState::Idle | PlayerState::Walking | PlayerState::Running
                )
            }
            PlayerAction::Attack => self.is_locomotion(),
            PlayerAction::Guard | PlayerAction::Dodge => {
                self.is_locomotion() && !self.is_airborne()
            }
            PlayerAction::Interact => matches!(self, PlayerState::Idle | PlayerState::Walking),
        }
    }

    /// Damage interrupts everything except dodging (invulnerable) and death.
    pub fn can_be_hurt(self) -> bool {
        !matches!(self, PlayerState::Dodging | PlayerState::Dead)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum TransitionError {
    #[error("cannot go from {from:?} to {to:?}")]
    NotAllowed { from: PlayerState, to: PlayerState },
}

/// State plus the remaining duration of timed action states.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PlayerStateMachine {
    state: PlayerState,
    remaining: f32,
}

impl PlayerStateMachine {
    pub fn state(&self) -> PlayerState {
        self.state
    }

    /// Whether a timed action is still running (or the player is dead).
    pub fn is_locked(&self) -> bool {
        self.state == PlayerState::Dead
            || (!self.state.is_locomotion() && self.remaining > 0.0)
            || self.state == PlayerState::Guarding
    }

    /// Advances timers. Returns `true` when a timed action just ended.
    pub fn tick(&mut self, dt: f32) -> bool {
        if self.state.is_locomotion()
            || self.state == PlayerState::Dead
            || self.state == PlayerState::Guarding
        {
            return false;
        }
        self.remaining -= dt;
        if self.remaining <= 0.0 {
            self.remaining = 0.0;
            self.state = PlayerState::Idle;
            return true;
        }
        false
    }

    /// Sets a locomotion state; ignored while an action is in progress.
    pub fn set_locomotion(&mut self, state: PlayerState) {
        if state.is_locomotion() && !self.is_locked() {
            self.state = state;
        }
    }

    /// Starts a timed action (attack, cast, dodge) if allowed.
    pub fn start_action(&mut self, to: PlayerState, duration: f32) -> Result<(), TransitionError> {
        let allowed = match to {
            PlayerState::Attacking => self.state.allows(PlayerAction::Attack),
            PlayerState::Dodging => self.state.allows(PlayerAction::Dodge),
            // Skills and spells validate their own phase list beforehand;
            // here we only forbid interrupting hurt/dead/dodge/cast.
            PlayerState::Casting => {
                self.state.is_locomotion()
                    || matches!(self.state, PlayerState::Attacking | PlayerState::Guarding)
            }
            _ => false,
        };
        if !allowed {
            return Err(TransitionError::NotAllowed {
                from: self.state,
                to,
            });
        }
        self.state = to;
        self.remaining = duration.max(0.0);
        Ok(())
    }

    /// Begins guarding (held, untimed).
    pub fn start_guard(&mut self) -> Result<(), TransitionError> {
        if !self.state.allows(PlayerAction::Guard) {
            return Err(TransitionError::NotAllowed {
                from: self.state,
                to: PlayerState::Guarding,
            });
        }
        self.state = PlayerState::Guarding;
        self.remaining = 0.0;
        Ok(())
    }

    pub fn stop_guard(&mut self) {
        if self.state == PlayerState::Guarding {
            self.state = PlayerState::Idle;
        }
    }

    /// Enters the hurt state (stagger). Returns whether it took effect.
    pub fn hurt(&mut self, stagger: f32) -> bool {
        if !self.state.can_be_hurt() || self.state == PlayerState::Guarding {
            return false;
        }
        self.state = PlayerState::Hurt;
        self.remaining = stagger.max(0.0);
        true
    }

    pub fn die(&mut self) {
        self.state = PlayerState::Dead;
        self.remaining = 0.0;
    }

    pub fn revive(&mut self) {
        self.state = PlayerState::Idle;
        self.remaining = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dead_player_cannot_attack() {
        let mut sm = PlayerStateMachine::default();
        sm.die();
        assert!(sm.start_action(PlayerState::Attacking, 0.3).is_err());
        sm.set_locomotion(PlayerState::Walking);
        assert_eq!(sm.state(), PlayerState::Dead);
    }

    #[test]
    fn attack_locks_until_finished() {
        let mut sm = PlayerStateMachine::default();
        assert!(sm.start_action(PlayerState::Attacking, 0.3).is_ok());
        sm.set_locomotion(PlayerState::Walking);
        assert_eq!(sm.state(), PlayerState::Attacking);
        assert!(sm.start_action(PlayerState::Attacking, 0.3).is_err());
        assert!(!sm.tick(0.2));
        assert!(sm.tick(0.2));
        assert_eq!(sm.state(), PlayerState::Idle);
    }

    #[test]
    fn dodge_is_invulnerable() {
        let mut sm = PlayerStateMachine::default();
        assert!(sm.start_action(PlayerState::Dodging, 0.3).is_ok());
        assert!(!sm.hurt(0.4));
    }

    #[test]
    fn hurt_interrupts_attack() {
        let mut sm = PlayerStateMachine::default();
        assert!(sm.start_action(PlayerState::Attacking, 0.3).is_ok());
        assert!(sm.hurt(0.4));
        assert_eq!(sm.state(), PlayerState::Hurt);
        assert!(sm.start_action(PlayerState::Casting, 0.5).is_err());
    }

    #[test]
    fn cannot_jump_in_air_or_dodge_in_air() {
        assert!(!PlayerState::Falling.allows(PlayerAction::Jump));
        assert!(!PlayerState::Jumping.allows(PlayerAction::Dodge));
        assert!(PlayerState::Falling.allows(PlayerAction::Attack));
    }
}
