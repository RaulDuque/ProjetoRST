//! V2.4: how hard a hit pushes its target and whether it staggers it.
//!
//! Pure module: no Bevy types. The intensity belongs to the hit (declared
//! by its source, e.g. the weapon class; never derived from the damage),
//! the stagger threshold to the target. `Medium` and a zero threshold are
//! the pre-V2.4 reaction: every damaging hit staggers with the same push.
//! Details: docs/COMBAT_FEEL_V2_4.md.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::core::constants::{
    ENEMY_KNOCKBACK, KNOCKBACK_HEAVY_SCALE, KNOCKBACK_LIGHT_SCALE, PLAYER_HURT_KNOCKBACK,
};

/// How hard a hit pushes. Ordered from the weakest to the strongest.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
pub enum KnockbackIntensity {
    Light,
    /// The pre-V2.4 knockback.
    #[default]
    Medium,
    Heavy,
}

impl KnockbackIntensity {
    /// Share of the `Medium` push.
    pub fn scale(self) -> f32 {
        match self {
            KnockbackIntensity::Light => KNOCKBACK_LIGHT_SCALE,
            KnockbackIntensity::Medium => 1.0,
            KnockbackIntensity::Heavy => KNOCKBACK_HEAVY_SCALE,
        }
    }

    /// Velocity given to a staggered player hit travelling `direction`.
    pub fn player_push(self, direction: f32) -> (f32, f32) {
        let scale = self.scale();
        (
            direction * PLAYER_HURT_KNOCKBACK.0 * scale,
            PLAYER_HURT_KNOCKBACK.1 * scale,
        )
    }

    /// Horizontal velocity given to an enemy hit travelling `direction`.
    pub fn enemy_push(self, direction: f32) -> f32 {
        direction * ENEMY_KNOCKBACK * self.scale()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Error)]
#[error("stagger threshold {0} is not a share of max HP in [0, 1]")]
pub struct InvalidStaggerThreshold(pub f32);

/// Smallest damage, as a share of the target's max HP, that staggers it
/// (hurt state and knockback). Lighter damaging hits still cost HP and
/// still start the invulnerability.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct StaggerThreshold(f32);

impl StaggerThreshold {
    /// Every damaging hit staggers (pre-V2.4).
    pub const ALWAYS: Self = Self(0.0);

    pub fn new(share: f32) -> Result<Self, InvalidStaggerThreshold> {
        if share.is_finite() && (0.0..=1.0).contains(&share) {
            Ok(Self(share))
        } else {
            Err(InvalidStaggerThreshold(share))
        }
    }

    pub fn share(self) -> f32 {
        self.0
    }

    /// Whether `damage` (already applied, > 0) staggers a target of
    /// `max_hp`. Damage exactly at the threshold staggers.
    pub fn staggers(self, damage: i32, max_hp: i32) -> bool {
        damage as f32 >= self.0 * max_hp.max(0) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn medium_is_the_pre_v2_4_knockback() {
        assert_eq!(KnockbackIntensity::default(), KnockbackIntensity::Medium);
        assert_eq!(
            KnockbackIntensity::Medium.player_push(-1.0),
            (-PLAYER_HURT_KNOCKBACK.0, PLAYER_HURT_KNOCKBACK.1)
        );
        assert_eq!(KnockbackIntensity::Medium.enemy_push(1.0), ENEMY_KNOCKBACK);
    }

    #[test]
    fn intensities_are_ordered_by_push() {
        let push = |k: KnockbackIntensity| k.player_push(1.0);
        let (light, medium, heavy) = (
            push(KnockbackIntensity::Light),
            push(KnockbackIntensity::Medium),
            push(KnockbackIntensity::Heavy),
        );
        assert!(light.0 < medium.0 && medium.0 < heavy.0);
        assert!(light.1 < medium.1 && medium.1 < heavy.1);
        assert!(KnockbackIntensity::Light < KnockbackIntensity::Heavy);
        assert!(
            KnockbackIntensity::Heavy.enemy_push(-1.0) < KnockbackIntensity::Light.enemy_push(-1.0)
        );
    }

    #[test]
    fn zero_threshold_staggers_every_damaging_hit() {
        assert_eq!(StaggerThreshold::default(), StaggerThreshold::ALWAYS);
        assert!(StaggerThreshold::ALWAYS.staggers(1, 1000));
    }

    #[test]
    fn threshold_edges() {
        let threshold = StaggerThreshold::new(0.1).unwrap_or_else(|e| panic!("{e}"));
        assert!(!threshold.staggers(9, 100), "below");
        assert!(threshold.staggers(10, 100), "exactly at the threshold");
        assert!(threshold.staggers(11, 100), "above");
        let full = StaggerThreshold::new(1.0).unwrap_or_else(|e| panic!("{e}"));
        assert!(!full.staggers(99, 100));
        assert!(full.staggers(100, 100));
    }

    #[test]
    fn invalid_thresholds_are_rejected() {
        for bad in [-0.1, 1.1, f32::NAN, f32::INFINITY] {
            assert!(StaggerThreshold::new(bad).is_err(), "{bad}");
        }
        assert!(StaggerThreshold::new(0.0).is_ok());
    }
}
