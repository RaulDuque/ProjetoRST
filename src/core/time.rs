//! Frame timing helpers.

use bevy::prelude::*;

use super::constants::MAX_FRAME_DELTA;

/// Simulation step for this frame, clamped to [`MAX_FRAME_DELTA`].
pub fn frame_delta(time: &Time) -> f32 {
    clamp_delta(time.delta_secs())
}

pub fn clamp_delta(dt: f32) -> f32 {
    dt.clamp(0.0, MAX_FRAME_DELTA)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_frames_are_clamped() {
        assert_eq!(clamp_delta(1.0), MAX_FRAME_DELTA);
        assert_eq!(clamp_delta(0.01), 0.01);
        assert_eq!(clamp_delta(-1.0), 0.0);
    }
}
