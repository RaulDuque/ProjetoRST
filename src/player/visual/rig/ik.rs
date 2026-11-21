//! Two-bone inverse kinematics in 2D. Pure module: no Bevy types.
//!
//! Law-of-cosines solver adapted from Agent Sprite Forge
//! (`skills/codeart2d/scripts/rig_animate.py`, `solve_two_bone`; MIT,
//! Copyright (c) 2026 0x0funky), with the reach clamp of pixelfx
//! (`src/core/rig.js`, `ik`; MIT, Copyright (c) 2026 Patch, Hugo Duprez)
//! and a "too close" clamp neither has. Rewritten in Rust for rig space
//! (y up, counter-clockwise radians), so the bend sides are named by
//! rotation instead of by the screen's y-down convention.

use super::xform::{RigPoint, direction, distance};

/// Which way the middle joint bends, seen from the root towards the end.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bend {
    /// The first bone turns counter-clockwise from the root-target line
    /// (a knee pointing forward when a leg hangs down and the art faces +x).
    CounterClockwise,
    Clockwise,
}

/// World angles of both bones.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TwoBoneSolution {
    pub first: f32,
    pub second: f32,
    /// Where the middle joint lands.
    pub middle: RigPoint,
    /// Where the end lands (the target, unless it was out of reach).
    pub end: RigPoint,
    /// The target was reachable without clamping.
    pub reached: bool,
}

/// Margin that keeps the chain from locking straight or folding flat.
const REACH_MARGIN: f32 = 0.02;

/// Solves a chain of lengths `l1`, `l2` rooted at `root` towards `target`.
pub fn solve_two_bone(
    root: RigPoint,
    target: RigPoint,
    l1: f32,
    l2: f32,
    bend: Bend,
) -> TwoBoneSolution {
    let base = direction(root, target);
    let wanted = distance(root, target);
    let max = (l1 + l2 - REACH_MARGIN).max(REACH_MARGIN);
    let min = ((l1 - l2).abs() + REACH_MARGIN).min(max);
    let d = wanted.clamp(min, max);
    let reached = (d - wanted).abs() < 1e-4;
    let cos = if l1 > 0.0 && d > 0.0 {
        ((l1 * l1 + d * d - l2 * l2) / (2.0 * l1 * d)).clamp(-1.0, 1.0)
    } else {
        1.0
    };
    let opening = cos.acos();
    let first = match bend {
        Bend::CounterClockwise => base + opening,
        Bend::Clockwise => base - opening,
    };
    let middle = (root.0 + l1 * first.cos(), root.1 + l1 * first.sin());
    let end = (root.0 + d * base.cos(), root.1 + d * base.sin());
    TwoBoneSolution {
        first,
        second: direction(middle, end),
        middle,
        end,
        reached,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-3
    }

    #[test]
    fn reachable_targets_are_hit_with_the_bone_lengths() {
        let s = solve_two_bone((0.0, 50.0), (6.0, 5.0), 26.0, 25.0, Bend::CounterClockwise);
        assert!(s.reached);
        assert!(close(s.end.0, 6.0) && close(s.end.1, 5.0));
        assert!(close(distance((0.0, 50.0), s.middle), 26.0));
        assert!(close(distance(s.middle, s.end), 25.0));
    }

    #[test]
    fn counter_clockwise_puts_a_hanging_knee_forward() {
        let root = (0.0, 50.0);
        let target = (0.0, 6.0);
        let ccw = solve_two_bone(root, target, 26.0, 25.0, Bend::CounterClockwise);
        let cw = solve_two_bone(root, target, 26.0, 25.0, Bend::Clockwise);
        assert!(ccw.middle.0 > 0.0, "knee forward (+x)");
        assert!(cw.middle.0 < 0.0, "knee backward");
        assert!(close(ccw.middle.1, cw.middle.1));
    }

    #[test]
    fn out_of_reach_targets_clamp_along_the_line() {
        let s = solve_two_bone((0.0, 0.0), (100.0, 0.0), 10.0, 10.0, Bend::Clockwise);
        assert!(!s.reached);
        assert!(close(s.end.1, 0.0));
        assert!(s.end.0 < 20.0 && s.end.0 > 19.9);
    }

    #[test]
    fn too_close_targets_do_not_produce_nan() {
        let s = solve_two_bone((0.0, 0.0), (0.0, 0.0), 10.0, 6.0, Bend::Clockwise);
        assert!(!s.reached);
        assert!(s.first.is_finite() && s.second.is_finite());
        assert!(s.middle.0.is_finite() && s.end.1.is_finite());
    }
}
