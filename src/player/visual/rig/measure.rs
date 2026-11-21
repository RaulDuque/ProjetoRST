//! Stability metrics of sockets and held items across a looping clip.
//! Pure module: no Bevy types.
//!
//! The approved frames really move (the torso turns, the arms swing), so a
//! socket is not expected to stand still. What must be small is the
//! **jitter**: the frame-to-frame change that is not explained by smooth
//! motion. Metrics:
//!
//! - [`loop_jerk`]: mean length of the second difference of a point series
//!   on a loop (0 for a point moving at constant velocity);
//! - [`loop_angle_jerk`]: the same for angles (wrapped);
//! - [`spread`]: standard deviation of a point series (how far a socket
//!   wanders, e.g. the shoulder measured in the spine's frame).

use super::xform::{RigPoint, Xform2, direction, wrap_angle};

/// The spine's frame of a pose: origin on the pelvis, `+x` towards the
/// neck. A socket expressed in it is free of the body's lean and bob.
pub fn spine_frame(pelvis: RigPoint, neck: RigPoint) -> Xform2 {
    Xform2::new(pelvis.0, pelvis.1, direction(pelvis, neck))
}

/// `p` in the frame `f`.
pub fn to_local(f: &Xform2, p: RigPoint) -> RigPoint {
    f.inverse().apply(p)
}

/// Mean length of `p[i-1] - 2 p[i] + p[i+1]` around the loop.
pub fn loop_jerk(points: &[RigPoint]) -> f32 {
    let n = points.len();
    if n < 3 {
        return 0.0;
    }
    let total: f32 = (0..n)
        .map(|i| {
            let (a, b, c) = (points[(i + n - 1) % n], points[i], points[(i + 1) % n]);
            (a.0 - 2.0 * b.0 + c.0).hypot(a.1 - 2.0 * b.1 + c.1)
        })
        .sum();
    total / n as f32
}

/// Mean absolute second difference of angles (radians) around the loop.
pub fn loop_angle_jerk(angles: &[f32]) -> f32 {
    let n = angles.len();
    if n < 3 {
        return 0.0;
    }
    let total: f32 = (0..n)
        .map(|i| {
            let (a, b, c) = (angles[(i + n - 1) % n], angles[i], angles[(i + 1) % n]);
            (wrap_angle(c - b) - wrap_angle(b - a)).abs()
        })
        .sum();
    total / n as f32
}

/// Standard deviation (root mean square distance to the mean).
pub fn spread(points: &[RigPoint]) -> f32 {
    let n = points.len();
    if n == 0 {
        return 0.0;
    }
    let mx = points.iter().map(|p| p.0).sum::<f32>() / n as f32;
    let my = points.iter().map(|p| p.1).sum::<f32>() / n as f32;
    let var = points
        .iter()
        .map(|p| (p.0 - mx).powi(2) + (p.1 - my).powi(2))
        .sum::<f32>()
        / n as f32;
    var.sqrt()
}

/// Centroid of the texels selected by `keep` within `radius` of `seed`
/// (cell texel coordinates, y down); `None` when none is selected.
pub fn centroid_near<T>(
    labels: &[T],
    width: usize,
    seed: (f32, f32),
    radius: f32,
    keep: impl Fn(&T) -> bool,
) -> Option<(f32, f32)> {
    let (mut sx, mut sy, mut n) = (0.0, 0.0, 0usize);
    for (i, l) in labels.iter().enumerate() {
        let (x, y) = ((i % width) as f32 + 0.5, (i / width) as f32 + 0.5);
        if keep(l) && (x - seed.0).hypot(y - seed.1) <= radius {
            sx += x;
            sy += y;
            n += 1;
        }
    }
    (n > 0).then(|| (sx / n as f32, sy / n as f32))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_velocity_on_a_circle_has_small_jerk_and_noise_raises_it() {
        let smooth: Vec<RigPoint> = (0..6)
            .map(|i| {
                let a = i as f32 / 6.0 * std::f32::consts::TAU;
                (a.cos() * 2.0, a.sin() * 2.0)
            })
            .collect();
        let mut noisy = smooth.clone();
        noisy[2].0 += 2.0;
        assert!(loop_jerk(&noisy) > loop_jerk(&smooth) + 0.5);
        assert_eq!(loop_jerk(&[(0.0, 0.0), (1.0, 1.0)]), 0.0);
    }

    #[test]
    fn angle_jerk_wraps() {
        let steady = [3.1, -3.1, -3.0, -2.9, -2.8, 3.0];
        let still = [1.0; 6];
        assert_eq!(loop_angle_jerk(&still), 0.0);
        assert!(loop_angle_jerk(&steady) < 1.0);
    }

    #[test]
    fn spread_and_local_frames() {
        assert_eq!(spread(&[(1.0, 1.0); 4]), 0.0);
        assert!((spread(&[(0.0, 0.0), (2.0, 0.0)]) - 1.0).abs() < 1e-6);
        let f = spine_frame((0.0, 0.0), (0.0, 10.0));
        let l = to_local(&f, (-2.0, 5.0));
        assert!((l.0 - 5.0).abs() < 1e-5 && (l.1 - 2.0).abs() < 1e-5);
    }

    #[test]
    fn centroid_of_selected_texels() {
        // 4x1 grid: texels 1 and 2 selected.
        let labels = [false, true, true, false];
        let c = centroid_near(&labels, 4, (2.0, 0.5), 3.0, |l| *l)
            .unwrap_or_else(|| panic!("centroid"));
        assert!((c.0 - 2.0).abs() < 1e-6 && (c.1 - 0.5).abs() < 1e-6);
        assert!(centroid_near(&labels, 4, (2.0, 0.5), 3.0, |_| false).is_none());
    }
}
