//! Rigid 2D transforms of the character rig. Pure module: no Bevy types.
//!
//! Rig space: texels, `x` towards where the art faces (right), `y` up,
//! origin on the clip's sole-line pivot; angles in radians,
//! counter-clockwise from `+x` (the convention of `sockets.rs`).

use std::f32::consts::{PI, TAU};

/// A point in rig space.
pub type RigPoint = (f32, f32);

/// Rotation then translation (no scale: bones are rigid).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Xform2 {
    pub x: f32,
    pub y: f32,
    /// Radians, counter-clockwise.
    pub angle: f32,
}

impl Xform2 {
    pub const IDENTITY: Self = Self {
        x: 0.0,
        y: 0.0,
        angle: 0.0,
    };

    pub const fn new(x: f32, y: f32, angle: f32) -> Self {
        Self { x, y, angle }
    }

    /// Position of the frame's origin.
    pub const fn origin(&self) -> RigPoint {
        (self.x, self.y)
    }

    /// A point given in this frame, expressed in the parent space.
    pub fn apply(&self, p: RigPoint) -> RigPoint {
        let (s, c) = self.angle.sin_cos();
        (self.x + c * p.0 - s * p.1, self.y + s * p.0 + c * p.1)
    }

    /// `self` is the parent's world frame, `local` a child frame given in
    /// it: the child's world frame (FK composition, `world = parent · local`).
    pub fn compose(&self, local: &Self) -> Self {
        let (x, y) = self.apply((local.x, local.y));
        Self::new(x, y, self.angle + local.angle)
    }

    /// The frame that undoes this one (`a.inverse().compose(&a)` is identity).
    pub fn inverse(&self) -> Self {
        let (s, c) = self.angle.sin_cos();
        Self::new(
            -(c * self.x + s * self.y),
            s * self.x - c * self.y,
            -self.angle,
        )
    }
}

/// Angle folded into `(-PI, PI]`.
pub fn wrap_angle(angle: f32) -> f32 {
    let a = (angle + PI).rem_euclid(TAU) - PI;
    if a <= -PI { a + TAU } else { a }
}

/// Direction from `a` to `b`, radians counter-clockwise from `+x`.
pub fn direction(a: RigPoint, b: RigPoint) -> f32 {
    (b.1 - a.1).atan2(b.0 - a.0)
}

/// Distance between two points.
pub fn distance(a: RigPoint, b: RigPoint) -> f32 {
    (b.0 - a.0).hypot(b.1 - a.1)
}

/// Distance from `p` to the segment `a`-`b`.
pub fn distance_to_segment(p: RigPoint, a: RigPoint, b: RigPoint) -> f32 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let len2 = dx * dx + dy * dy;
    if len2 <= f32::EPSILON {
        return distance(p, a);
    }
    let t = (((p.0 - a.0) * dx + (p.1 - a.1) * dy) / len2).clamp(0.0, 1.0);
    distance(p, (a.0 + t * dx, a.1 + t * dy))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-4
    }

    #[test]
    fn apply_rotates_then_translates() {
        let f = Xform2::new(10.0, 5.0, PI / 2.0);
        let (x, y) = f.apply((2.0, 0.0));
        assert!(close(x, 10.0) && close(y, 7.0));
    }

    #[test]
    fn compose_and_inverse_round_trip() {
        let parent = Xform2::new(3.0, -2.0, 0.7);
        let child = Xform2::new(-1.5, 4.0, -1.9);
        let world = parent.compose(&child);
        let back = parent.inverse().compose(&world);
        assert!(close(back.x, child.x) && close(back.y, child.y));
        assert!(close(wrap_angle(back.angle - child.angle), 0.0));
        let id = parent.inverse().compose(&parent);
        assert!(close(id.x, 0.0) && close(id.y, 0.0) && close(id.angle, 0.0));
    }

    #[test]
    fn wrap_stays_in_half_open_range() {
        assert!(close(wrap_angle(3.0 * PI), PI));
        assert!(close(wrap_angle(-PI), PI));
        assert!(close(wrap_angle(0.5), 0.5));
        assert!(close(wrap_angle(-TAU - 0.5), -0.5));
    }

    #[test]
    fn segment_distance_clamps_to_the_ends() {
        let (a, b) = ((0.0, 0.0), (10.0, 0.0));
        assert!(close(distance_to_segment((5.0, 3.0), a, b), 3.0));
        assert!(close(distance_to_segment((-4.0, 3.0), a, b), 5.0));
        assert!(close(distance_to_segment((1.0, 1.0), a, a), 2f32.sqrt()));
    }
}
