//! Rules of the player's visual effects. Pure module: no Bevy types.
//!
//! Effects are clips of the `effects` atlas of the player manifest
//! (`vfx_<key>`). Which effect a skill shows comes from the `vfx` key of its
//! data (`assets/data/skills/*.ron`, `assets/data/spells/*.ron`), so a new
//! skill with an existing key needs no code. Effects only observe gameplay:
//! they never deal damage, heal, move anything or decide a hit.

use crate::player::state::PlayerState;

/// Manifest prefix of every effect clip.
pub const CLIP_PREFIX: &str = "vfx_";
/// Spark shown when a guard blocks a hit from the front.
pub const GUARD_SPARK: &str = "vfx_guard_spark";
/// Light shown when the player leaves the dead state (respawn or load).
pub const REVIVE: &str = "vfx_revive";

/// Manifest clip of a skill's `vfx` key (`None` for an empty key).
pub fn skill_clip(vfx_key: &str) -> Option<String> {
    (!vfx_key.is_empty()).then(|| format!("{CLIP_PREFIX}{vfx_key}"))
}

/// How much slower (> 1) or faster (< 1) an effect plays so that it lasts
/// the real gameplay action (`attack_speed` shortens skills). Without an
/// action (guard spark, revive) it plays at its authored rate.
pub fn time_scale(action_seconds: f32, clip_seconds: f32) -> f32 {
    if action_seconds > 0.0 && clip_seconds > 0.0 && action_seconds.is_finite() {
        action_seconds / clip_seconds
    } else {
        1.0
    }
}

/// Frame shown `elapsed` seconds after the effect started, or `None` once it
/// has played (the effect is then removed: nothing stays on screen).
pub fn effect_frame(durations: &[f32], elapsed: f32, scale: f32) -> Option<usize> {
    let scale = if scale > 0.0 { scale } else { 1.0 };
    let mut start = 0.0;
    for (index, duration) in durations.iter().enumerate() {
        start += duration * scale;
        if elapsed < start {
            return Some(index);
        }
    }
    None
}

/// A technique's effect follows the strike: it disappears as soon as the
/// caster stops performing it (hurt, death, load). Spell effects (the heal
/// already happened when the spell started) play to the end.
pub fn interrupted(follows_action: bool, state: PlayerState) -> bool {
    follows_action && state != PlayerState::Casting
}

/// The player has just left the dead state (respawn with R or a quick load).
pub fn revived(previous: Option<PlayerState>, now: PlayerState) -> bool {
    previous == Some(PlayerState::Dead) && now != PlayerState::Dead
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skill_effects_come_from_the_data_keys() {
        assert_eq!(skill_clip("arc_white").as_deref(), Some("vfx_arc_white"));
        assert_eq!(
            skill_clip("light_motes").as_deref(),
            Some("vfx_light_motes")
        );
        assert_eq!(skill_clip(""), None);
    }

    #[test]
    fn effects_last_the_real_action() {
        // Crescent Slash 0.40 s with attack_speed 1.25 lasts 0.32 s.
        let durations = [0.05; 8];
        let scale = time_scale(0.32, 0.40);
        assert!((scale - 0.8).abs() < 1e-6);
        assert_eq!(effect_frame(&durations, 0.0, scale), Some(0));
        assert_eq!(effect_frame(&durations, 0.319, scale), Some(7));
        assert_eq!(effect_frame(&durations, 0.321, scale), None);
        // No action: authored rate; nonsense inputs never divide by zero.
        assert_eq!(time_scale(0.0, 0.4), 1.0);
        assert_eq!(time_scale(0.3, 0.0), 1.0);
        assert_eq!(effect_frame(&durations, 0.39, 0.0), Some(7));
        assert_eq!(effect_frame(&[], 0.0, 1.0), None);
    }

    #[test]
    fn technique_effects_stop_with_the_action() {
        assert!(!interrupted(true, PlayerState::Casting));
        for state in [PlayerState::Hurt, PlayerState::Dead, PlayerState::Idle] {
            assert!(interrupted(true, state), "{state:?}");
            assert!(!interrupted(false, state), "{state:?}");
        }
    }

    #[test]
    fn revive_is_leaving_the_dead_state() {
        assert!(revived(Some(PlayerState::Dead), PlayerState::Idle));
        assert!(revived(Some(PlayerState::Dead), PlayerState::Falling));
        assert!(!revived(Some(PlayerState::Dead), PlayerState::Dead));
        assert!(!revived(None, PlayerState::Idle));
        assert!(!revived(Some(PlayerState::Hurt), PlayerState::Idle));
    }
}
