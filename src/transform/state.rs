//! State of the player's transformation (V1.9). Pure module: no Bevy types.
//!
//! [`FormState`] decides which form is active; it never moves the body.
//! The Bevy systems apply its decisions (profiles, MP, visuals).
//!
//! Ending a form is two steps: [`FormState::request_end`] records why, and
//! [`FormState::settle`] ends it only when the human body fits where the
//! player stands. Until then the form stays active (same physics, same
//! look, no upkeep), so the player can always walk or sink out of a veil:
//! a human is never restored inside a barrier, and nobody is teleported.

use super::definitions::{FormId, TransformationDefinition};
use crate::player::state::PlayerState;

/// Why a form ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EndReason {
    /// The transformation key.
    Voluntary,
    /// The upkeep could not be paid.
    MpEmpty,
    Death,
    /// A save was loaded (the save never stores the form).
    Load,
}

/// Why the form cannot start now.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivateError {
    /// Its unlock requirement is not met (relic not found).
    Locked,
    Dead,
    /// An action (attack, cast, guard, dodge, hurt) is in progress.
    Busy(PlayerState),
    /// The form only starts on the ground.
    Airborne,
    /// A room change has not settled yet (doors not re-armed).
    InTransition,
    NotEnoughMp {
        need: i32,
        have: i32,
    },
}

/// What the activation check needs to know about the player.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ActivationContext {
    pub unlocked: bool,
    pub state: PlayerState,
    pub grounded: bool,
    pub in_transition: bool,
    pub mp: i32,
}

/// Whether `def` may start now (the form is not active). Checks are in a
/// fixed order, so the reason shown is deterministic.
pub fn check_activation(
    def: &TransformationDefinition,
    ctx: ActivationContext,
) -> Result<(), ActivateError> {
    if !ctx.unlocked {
        return Err(ActivateError::Locked);
    }
    if ctx.state == PlayerState::Dead {
        return Err(ActivateError::Dead);
    }
    let free = matches!(
        ctx.state,
        PlayerState::Idle
            | PlayerState::Walking
            | PlayerState::Running
            | PlayerState::Crouching
            | PlayerState::Jumping
            | PlayerState::Falling
    );
    if !free {
        return Err(ActivateError::Busy(ctx.state));
    }
    if def.activation.grounded_only && (!ctx.grounded || ctx.state.is_airborne()) {
        return Err(ActivateError::Airborne);
    }
    if ctx.in_transition {
        return Err(ActivateError::InTransition);
    }
    if ctx.mp < def.costs.activation_mp {
        return Err(ActivateError::NotEnoughMp {
            need: def.costs.activation_mp,
            have: ctx.mp,
        });
    }
    Ok(())
}

/// The player's transformation: the active form, if any, and its upkeep.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FormState {
    active: Option<FormId>,
    ending: Option<EndReason>,
    /// MP owed by the upkeep, below one whole point.
    owed: f32,
    /// MP paid since the form started (activation included).
    spent: i32,
    /// Seconds since the form started.
    elapsed: f32,
}

impl FormState {
    pub fn active(&self) -> Option<FormId> {
        self.active
    }

    pub fn is_active(&self) -> bool {
        self.active.is_some()
    }

    /// Why the form is waiting to end (it ends once the human body fits).
    pub fn ending(&self) -> Option<EndReason> {
        self.ending
    }

    pub fn spent(&self) -> i32 {
        self.spent
    }

    pub fn elapsed(&self) -> f32 {
        self.elapsed
    }

    /// Enters `def` (after [`check_activation`]). Returns the MP to pay.
    /// Ignored (0) when a form is already active: no double payment.
    pub fn activate(&mut self, def: &TransformationDefinition) -> i32 {
        if self.active.is_some() {
            return 0;
        }
        *self = FormState {
            active: Some(def.id),
            spent: def.costs.activation_mp,
            ..FormState::default()
        };
        def.costs.activation_mp
    }

    /// Upkeep of one frame: whole MP points to take now. Nothing while no
    /// form is active or while it is waiting to end.
    pub fn upkeep(&mut self, def: &TransformationDefinition, dt: f32) -> i32 {
        if self.active.is_none() {
            return 0;
        }
        self.elapsed += dt;
        if self.ending.is_some() {
            return 0;
        }
        self.owed += def.costs.mp_per_second * dt;
        let whole = self.owed.floor();
        self.owed -= whole;
        whole as i32
    }

    /// Records that `paid` MP of the upkeep were taken.
    pub fn record_paid(&mut self, paid: i32) {
        self.spent += paid;
    }

    /// Asks for the form to end. Returns `false` when there is nothing to
    /// end or an end is already pending (the first reason is kept, except
    /// that death and loads replace a voluntary or MP end).
    pub fn request_end(&mut self, reason: EndReason) -> bool {
        if self.active.is_none() {
            return false;
        }
        match self.ending {
            None => {
                self.ending = Some(reason);
                true
            }
            Some(EndReason::Voluntary | EndReason::MpEmpty)
                if matches!(reason, EndReason::Death | EndReason::Load) =>
            {
                self.ending = Some(reason);
                true
            }
            Some(_) => false,
        }
    }

    /// Ends a pending end when the human body fits. Returns the form that
    /// ended and why.
    pub fn settle(&mut self, human_fits: bool) -> Option<(FormId, EndReason)> {
        let (Some(form), Some(reason)) = (self.active, self.ending) else {
            return None;
        };
        if !human_fits {
            return None;
        }
        *self = FormState::default();
        Some((form, reason))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transform::definitions::tests_support::spectral;

    fn ready() -> ActivationContext {
        ActivationContext {
            unlocked: true,
            state: PlayerState::Idle,
            grounded: true,
            in_transition: false,
            mp: 50,
        }
    }

    #[test]
    fn activation_needs_the_relic_ground_freedom_and_mp() {
        let def = spectral();
        assert_eq!(check_activation(&def, ready()), Ok(()));
        let with = |f: fn(&mut ActivationContext)| {
            let mut c = ready();
            f(&mut c);
            check_activation(&def, c)
        };
        assert_eq!(with(|c| c.unlocked = false), Err(ActivateError::Locked));
        assert_eq!(
            with(|c| c.state = PlayerState::Dead),
            Err(ActivateError::Dead)
        );
        for busy in [
            PlayerState::Attacking,
            PlayerState::Casting,
            PlayerState::Guarding,
            PlayerState::Dodging,
            PlayerState::Hurt,
        ] {
            let mut c = ready();
            c.state = busy;
            assert_eq!(check_activation(&def, c), Err(ActivateError::Busy(busy)));
        }
        assert_eq!(with(|c| c.grounded = false), Err(ActivateError::Airborne));
        assert_eq!(
            with(|c| c.state = PlayerState::Falling),
            Err(ActivateError::Airborne)
        );
        assert_eq!(
            with(|c| c.in_transition = true),
            Err(ActivateError::InTransition)
        );
        assert_eq!(
            with(|c| c.mp = 7),
            Err(ActivateError::NotEnoughMp { need: 8, have: 7 })
        );
        assert_eq!(with(|c| c.mp = 8), Ok(()));
    }

    #[test]
    fn activation_is_paid_once_and_upkeep_accumulates_fractions() {
        let def = spectral();
        let mut s = FormState::default();
        assert_eq!(s.activate(&def), 8);
        assert_eq!(s.activate(&def), 0, "no double payment");
        // 4 MP/s at 60 fps: 1 MP every 15 frames.
        let paid: i32 = (0..60).map(|_| s.upkeep(&def, 1.0 / 60.0)).sum();
        assert!((3..=4).contains(&paid), "{paid}");
        let paid_2s: i32 = (0..60).map(|_| s.upkeep(&def, 1.0 / 60.0)).sum();
        assert_eq!(paid + paid_2s, 8, "exactly 4 MP/s over two seconds");
    }

    #[test]
    fn ending_waits_until_the_human_body_fits() {
        let def = spectral();
        let mut s = FormState::default();
        assert!(!s.request_end(EndReason::Voluntary), "nothing active");
        s.activate(&def);
        assert!(s.request_end(EndReason::Voluntary));
        assert!(!s.request_end(EndReason::MpEmpty), "already pending");
        assert_eq!(s.upkeep(&def, 1.0), 0, "no upkeep while ending");
        assert_eq!(s.settle(false), None, "inside a veil: still spectral");
        assert!(s.is_active());
        assert!(s.request_end(EndReason::Death), "death replaces the reason");
        assert_eq!(s.settle(true), Some((FormId::Spectral, EndReason::Death)));
        assert!(!s.is_active());
        assert_eq!(s, FormState::default());
    }
}
