//! Frame timing of sprite clips. Pure module: no Bevy types.
//!
//! Frames may last different times (the 0.28 s action clips are six frames
//! of 1/24 s and a last one of 0.03 s), so every function works on the
//! authored per-frame durations instead of a single frame rate.

/// Frame shown `elapsed` seconds into a looping clip.
pub fn loop_frame(durations: &[f32], elapsed: f32) -> usize {
    let total: f32 = durations.iter().sum();
    if durations.is_empty() || total <= 0.0 {
        return 0;
    }
    frame_at(durations, elapsed.rem_euclid(total))
}

/// Frame shown `elapsed` seconds into a clip played once: it holds its last
/// frame afterwards (death, guard and crouch end in a held pose).
pub fn once_frame(durations: &[f32], elapsed: f32) -> usize {
    if durations.is_empty() {
        return 0;
    }
    frame_at(durations, elapsed.max(0.0))
}

/// Frame shown at `progress` (0..1) of a clip stretched over an action.
/// The authored durations keep their proportions, so a short last frame
/// stays short whatever the real action length is.
pub fn action_frame(durations: &[f32], progress: f32) -> usize {
    let total: f32 = durations.iter().sum();
    if durations.is_empty() || total <= 0.0 {
        return 0;
    }
    frame_at(durations, progress.clamp(0.0, 1.0) * total)
}

/// Frame whose interval `[start, start + duration)` contains `t`; the last
/// frame for `t` at or beyond the end.
fn frame_at(durations: &[f32], t: f32) -> usize {
    let mut start = 0.0;
    for (i, d) in durations.iter().enumerate() {
        start += d;
        if t < start - 1e-6 {
            return i;
        }
    }
    durations.len() - 1
}

/// Progress of a timed gameplay action, from the action's total duration
/// and its remaining time (both read from the gameplay state, never set).
pub fn action_progress(total: f32, remaining: f32) -> f32 {
    if total <= 0.0 {
        return 1.0;
    }
    ((total - remaining) / total).clamp(0.0, 1.0)
}

/// Playback rate of a locomotion loop: the real horizontal speed over the
/// speed the cycle was authored for, so the planted foot keeps pace with
/// the ground. Clamped so a nearly blocked run still reads as running.
pub fn locomotion_rate(speed: f32, reference: Option<f32>) -> f32 {
    match reference {
        Some(r) if r > 0.0 => (speed.abs() / r).clamp(MIN_RATE, MAX_RATE),
        _ => 1.0,
    }
}

pub const MIN_RATE: f32 = 0.5;
pub const MAX_RATE: f32 = 2.5;

#[cfg(test)]
mod tests {
    use super::*;

    const STEP: f32 = 1.0 / 24.0;
    /// The V1.3 attack and dodge clips: 6 frames of 1/24 s + 0.03 s = 0.28 s.
    const ACTION: [f32; 7] = [STEP, STEP, STEP, STEP, STEP, STEP, 0.03];

    #[test]
    fn action_clip_lasts_exactly_the_action() {
        let total: f32 = ACTION.iter().sum();
        assert!((total - 0.28).abs() < 1e-6);
        assert_eq!(action_frame(&ACTION, 0.0), 0);
        // Frame 6 starts at 0.25 s and covers only the last 0.03 s.
        assert_eq!(action_frame(&ACTION, 0.249 / 0.28), 5);
        assert_eq!(action_frame(&ACTION, 0.251 / 0.28), 6);
        assert_eq!(action_frame(&ACTION, 1.0), 6);
    }

    #[test]
    fn faster_attack_keeps_frame_proportions() {
        // attack_speed 1.4 shortens the action to 0.2 s: the frame at a given
        // progress is the same, only the real time changes.
        let total = 0.28 / 1.4;
        for remaining in [0.2_f32, 0.15, 0.1, 0.05, 0.01] {
            let p = action_progress(total, remaining);
            assert_eq!(
                action_frame(&ACTION, p),
                action_frame(&ACTION, (0.2 - remaining) / 0.2)
            );
        }
        assert_eq!(action_frame(&ACTION, action_progress(total, total)), 0);
    }

    #[test]
    fn loop_wraps_without_a_duplicate_frame() {
        let idle = [1.0 / 12.0; 12];
        assert_eq!(loop_frame(&idle, 0.0), 0);
        assert_eq!(loop_frame(&idle, 11.5 / 12.0), 11);
        assert_eq!(loop_frame(&idle, 1.0), 0);
        assert_eq!(loop_frame(&idle, 1.0 + 1.5 / 12.0), 1);
    }

    #[test]
    fn once_holds_its_last_frame() {
        let death = [1.0 / 12.0; 8];
        assert_eq!(once_frame(&death, 0.0), 0);
        assert_eq!(once_frame(&death, 3.5 / 12.0), 3);
        assert_eq!(once_frame(&death, 8.0 / 12.0), 7);
        assert_eq!(
            once_frame(&death, 60.0),
            7,
            "held while the player stays dead"
        );
        assert_eq!(once_frame(&[], 1.0), 0);
    }

    #[test]
    fn run_rate_follows_the_real_speed() {
        assert!((locomotion_rate(190.0, Some(190.0)) - 1.0).abs() < 1e-6);
        assert!((locomotion_rate(-310.0, Some(190.0)) - 310.0 / 190.0).abs() < 1e-6);
        assert_eq!(locomotion_rate(0.0, Some(190.0)), MIN_RATE);
        assert_eq!(locomotion_rate(5000.0, Some(190.0)), MAX_RATE);
        assert_eq!(locomotion_rate(100.0, None), 1.0);
    }

    #[test]
    fn progress_is_clamped_and_safe() {
        assert_eq!(action_progress(0.0, 0.0), 1.0);
        assert_eq!(action_progress(0.28, 0.4), 0.0);
        assert_eq!(action_progress(0.28, -1.0), 1.0);
    }
}
