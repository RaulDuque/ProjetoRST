//! Pure boss rules (D21): phases, attack patterns, defeat bookkeeping.
//!
//! No Bevy types. The Bevy layer (`boss_systems`) executes these choices.

use serde::{Deserialize, Serialize};

use super::ai::AiProfile;
use crate::world::progress::WorldProgress;

/// Optional boss metadata on an [`super::definitions::EnemyDefinition`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BossDefinition {
    /// HP ratio at which phase 2 begins (`0.5` = below 50%).
    pub phase_threshold: f32,
    /// AI used while in phase 2 (typically ranged ash wave).
    pub phase2_ai: AiProfile,
    pub phase2_attack_cooldown: f32,
    pub phase2_attack_hitbox: (f32, f32),
    /// Placeholder tint after the phase shift.
    pub phase2_color: (f32, f32, f32),
    /// Guaranteed item granted once on first defeat (`None` = EXP/drops only).
    pub reward_item: Option<String>,
    /// Progress flag set on first defeat (doors / story).
    pub reward_flag: Option<String>,
}

impl Default for BossDefinition {
    fn default() -> Self {
        Self {
            phase_threshold: 0.5,
            phase2_ai: AiProfile {
                style: super::ai::AiStyle::Ranged,
                aggro_range: 520.0,
                leash_range: 640.0,
                attack_range: 380.0,
                preferred_range: 280.0,
                min_range: 90.0,
                projectile_speed: 260.0,
                windup: 0.4,
                flee_below_hp: 0.0,
            },
            phase2_attack_cooldown: 1.2,
            phase2_attack_hitbox: (22.0, 14.0),
            phase2_color: (0.75, 0.35, 0.2),
            reward_item: None,
            reward_flag: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BossPhase {
    One,
    Two,
}

/// Phase-1 melee patterns (functionally distinct).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BossPattern {
    /// Stationary frontal slash.
    FrontalSlash,
    /// Short dash then frontal hit.
    Lunge,
    /// Phase-2 ranged ash wave (projectile).
    AshWave,
}

impl BossPhase {
    pub fn from_hp(hp_ratio: f32, threshold: f32) -> Self {
        if hp_ratio < threshold {
            BossPhase::Two
        } else {
            BossPhase::One
        }
    }
}

/// Next melee pattern in the phase-1 cycle.
pub fn next_phase1_pattern(previous: BossPattern) -> BossPattern {
    match previous {
        BossPattern::FrontalSlash | BossPattern::AshWave => BossPattern::Lunge,
        BossPattern::Lunge => BossPattern::FrontalSlash,
    }
}

pub fn pattern_for_phase(phase: BossPhase, cycle: BossPattern) -> BossPattern {
    match phase {
        BossPhase::One => match cycle {
            BossPattern::AshWave => BossPattern::FrontalSlash,
            other => other,
        },
        BossPhase::Two => BossPattern::AshWave,
    }
}

/// Hitbox size for a pattern (phase-1 melee or phase-2 bolt).
pub fn pattern_hitbox(pattern: BossPattern, phase1: (f32, f32), phase2: (f32, f32)) -> (f32, f32) {
    match pattern {
        BossPattern::FrontalSlash => phase1,
        BossPattern::Lunge => (phase1.0 * 0.9, phase1.1),
        BossPattern::AshWave => phase2,
    }
}

/// Horizontal dash impulse applied at the start of a lunge wind-up.
pub const LUNGE_SPEED: f32 = 220.0;

/// Whether this enemy id should spawn given current progress.
pub fn should_spawn_boss(progress: &WorldProgress, enemy_id: &str, is_boss: bool) -> bool {
    if !is_boss {
        return true;
    }
    !progress.defeated_bosses.contains(enemy_id)
}

/// Records a first-time boss defeat. Returns `true` if newly recorded.
pub fn record_defeat(
    progress: &mut WorldProgress,
    boss_id: &str,
    reward_flag: Option<&str>,
) -> bool {
    let first = progress.defeated_bosses.insert(boss_id.to_owned());
    if first && let Some(flag) = reward_flag {
        progress.set_flag(flag);
    }
    first
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_threshold_is_strict() {
        assert_eq!(BossPhase::from_hp(0.5, 0.5), BossPhase::One);
        assert_eq!(BossPhase::from_hp(0.49, 0.5), BossPhase::Two);
        assert_eq!(BossPhase::from_hp(1.0, 0.5), BossPhase::One);
    }

    #[test]
    fn patterns_cycle_and_phase2_is_ash_wave() {
        assert_eq!(
            next_phase1_pattern(BossPattern::FrontalSlash),
            BossPattern::Lunge
        );
        assert_eq!(
            next_phase1_pattern(BossPattern::Lunge),
            BossPattern::FrontalSlash
        );
        assert_eq!(
            pattern_for_phase(BossPhase::Two, BossPattern::Lunge),
            BossPattern::AshWave
        );
    }

    #[test]
    fn defeat_is_idempotent_and_sets_flag() {
        let mut progress = WorldProgress::default();
        assert!(record_defeat(
            &mut progress,
            "ashen_sentinel",
            Some("ashen_sentinel_fallen")
        ));
        assert!(!record_defeat(
            &mut progress,
            "ashen_sentinel",
            Some("ashen_sentinel_fallen")
        ));
        assert!(progress.defeated_bosses.contains("ashen_sentinel"));
        assert!(progress.has_flag("ashen_sentinel_fallen"));
        assert!(!should_spawn_boss(&progress, "ashen_sentinel", true));
        assert!(should_spawn_boss(&progress, "bone_sentry", false));
    }
}
