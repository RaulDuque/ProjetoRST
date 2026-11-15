//! V2.4: timeline of one basic attack (anticipation, active window,
//! recovery and cancel point).
//!
//! Pure module: no Bevy types. The timeline only *describes* a strike; the
//! timers that run the game stay where they were (the player's
//! `ActionState` and the hitbox lifetime). Elapsed time is derived from the
//! action timer, so there is no second clock to keep in sync and nothing
//! depends on animation frames. Docs: docs/COMBAT_FEEL_V2_4.md.

use crate::inventory::weapons::StrikeProfile;

/// Sub-phase of `PlayerState::Attacking`. It does not replace the state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StrikePhase {
    /// Anticipation: committed, the strike cannot hit yet.
    Startup,
    /// The hitbox can hit.
    Active,
    /// After the active window until the action ends.
    Recovery,
}

/// Seconds of one strike, already scaled by attack speed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StrikeTimeline {
    /// Seconds before the strike can hit.
    pub startup: f32,
    /// Seconds the strike can hit, after `startup`.
    pub active: f32,
    /// Seconds of the whole action (swing + recovery).
    pub total: f32,
    /// Seconds from which jump, dodge, guard and a new attack may cancel
    /// the action (`None` = never).
    pub cancel_at: Option<f32>,
    /// Seconds from the start during which the character may turn.
    pub turn_window: f32,
}

impl StrikeTimeline {
    pub fn from_profile(profile: &StrikeProfile) -> Self {
        Self {
            startup: profile.startup,
            active: profile.active_time,
            total: profile.action_time,
            cancel_at: profile
                .cancel_after
                .map(|share| share * profile.action_time),
            turn_window: profile.turn_window,
        }
    }

    /// True while the turn window is open (never with a window of 0).
    pub fn can_turn(&self, elapsed: f32) -> bool {
        elapsed < self.turn_window
    }

    /// End of the active window, in seconds since the start.
    pub fn active_end(&self) -> f32 {
        self.startup + self.active
    }

    /// Seconds since the start, from the action timer's remaining time.
    pub fn elapsed(&self, remaining: f32) -> f32 {
        (self.total - remaining).max(0.0)
    }

    pub fn phase(&self, elapsed: f32) -> StrikePhase {
        if elapsed < self.startup {
            StrikePhase::Startup
        } else if elapsed < self.active_end() {
            StrikePhase::Active
        } else {
            StrikePhase::Recovery
        }
    }

    /// True once the cancel point has been reached.
    pub fn cancelable(&self, elapsed: f32) -> bool {
        self.cancel_at.is_some_and(|at| elapsed >= at)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::constants::{BASIC_ATTACK_TIME, HITBOX_ACTIVE_FRACTION};

    fn timeline(startup: f32, cancel: Option<f32>) -> StrikeTimeline {
        let mut profile = StrikeProfile::unarmed(1.0);
        profile.startup = startup;
        profile.cancel_after = cancel;
        StrikeTimeline::from_profile(&profile)
    }

    #[test]
    fn the_default_strike_is_active_from_the_start_and_never_cancelable() {
        let t = timeline(0.0, None);
        assert_eq!(t.total, BASIC_ATTACK_TIME);
        assert_eq!(t.active, BASIC_ATTACK_TIME * HITBOX_ACTIVE_FRACTION);
        assert_eq!(t.phase(0.0), StrikePhase::Active);
        assert_eq!(t.phase(t.active_end() - 1e-4), StrikePhase::Active);
        assert_eq!(t.phase(t.active_end()), StrikePhase::Recovery);
        assert_eq!(t.phase(t.total), StrikePhase::Recovery);
        for step in 0..=28 {
            assert!(!t.cancelable(step as f32 * 0.01));
        }
    }

    #[test]
    fn phases_follow_startup_active_and_recovery() {
        let t = timeline(0.05, None);
        assert_eq!(t.phase(0.0), StrikePhase::Startup);
        assert_eq!(t.phase(0.0499), StrikePhase::Startup);
        assert_eq!(t.phase(0.05), StrikePhase::Active);
        assert_eq!(t.phase(0.05 + t.active), StrikePhase::Recovery);
    }

    #[test]
    fn cancel_point_is_a_share_of_the_whole_action() {
        let t = timeline(0.0, Some(0.75));
        let at = 0.75 * BASIC_ATTACK_TIME;
        assert_eq!(t.cancel_at, Some(at));
        assert!(!t.cancelable(at - 1e-4));
        assert!(t.cancelable(at));
    }

    #[test]
    fn turn_window_is_closed_by_default_and_open_only_at_the_start() {
        assert!(!timeline(0.0, None).can_turn(0.0));
        let mut profile = StrikeProfile::unarmed(1.0);
        profile.turn_window = 0.1;
        let t = StrikeTimeline::from_profile(&profile);
        assert!(t.can_turn(0.0));
        assert!(t.can_turn(0.0999));
        assert!(!t.can_turn(0.1));
    }

    #[test]
    fn elapsed_comes_from_the_action_timer() {
        let t = timeline(0.0, None);
        assert_eq!(t.elapsed(t.total), 0.0);
        assert!((t.elapsed(0.08) - (BASIC_ATTACK_TIME - 0.08)).abs() < 1e-6);
        assert_eq!(t.elapsed(t.total + 1.0), 0.0);
    }
}
