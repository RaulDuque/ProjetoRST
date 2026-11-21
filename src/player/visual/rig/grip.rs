//! Hand and wrist transform: how a held item is gripped. Pure module.
//!
//! The approved frames give the fist position (`Joint::HandNear`), not the
//! wrist's rotation. A [`GripProfile`] (one per animation state, reused by
//! every weapon) supplies that rotation: the blade direction is set in the
//! body's frame (or the forearm's, or the world's), plus a controlled
//! fraction of the forearm's swing. This is the pixelfx idea of a weapon
//! angle `sa` that is independent of the arm (`src/core/rig.js`, MIT,
//! Copyright (c) 2026 Patch, Hugo Duprez), adapted to marked frames.

use serde::{Deserialize, Serialize};

use super::layers::DepthSlot;
use super::skeleton::{Bone, Joint, JointPositions, LocalPose, WorldPose};
use super::xform::{Xform2, wrap_angle};

/// Which hand holds the item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum HandSide {
    Near,
    Far,
}

impl HandSide {
    pub const fn fist(self) -> Joint {
        match self {
            Self::Near => Joint::HandNear,
            Self::Far => Joint::HandFar,
        }
    }

    pub const fn forearm(self) -> Bone {
        match self {
            Self::Near => Bone::ForearmNear,
            Self::Far => Bone::ForearmFar,
        }
    }
}

/// The frame the grip angle is measured in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum GripReference {
    /// Relative to the upright body: the spine's rotation from the rest
    /// pose is added (a lean tilts the weapon with the body).
    Body,
    /// Relative to the forearm's direction (prototype 01 behaviour).
    Forearm,
    /// Absolute screen angle.
    World,
}

/// A reusable grip for one animation state.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct GripProfile {
    pub reference: GripReference,
    /// Direction of the blade (pommel -> tip), degrees counter-clockwise
    /// from the facing direction, in the reference frame.
    pub angle_degrees: f32,
    /// Fraction of the forearm's swing (from its rest angle) added to the
    /// blade angle: 0 = steady, 1 = rigidly bound to the forearm.
    pub follow_forearm: f32,
    /// Optional per-frame depth of the held item (one entry per frame of
    /// the clip; empty = the item's own depth). Lets a blade pass behind
    /// the body in some poses.
    #[serde(default)]
    pub depth: Vec<DepthSlot>,
}

impl GripProfile {
    /// Depth of a held item in a frame.
    pub fn depth_at(&self, frame: usize, default: DepthSlot) -> DepthSlot {
        self.depth.get(frame).copied().unwrap_or(default)
    }
}

/// World frame of the hand: origin on the fist, `+x` along the blade.
pub fn hand_frame(
    side: HandSide,
    joints: &JointPositions,
    pose: &WorldPose,
    rest: &LocalPose,
    profile: &GripProfile,
) -> Xform2 {
    let fist = joints.get(side.fist());
    let forearm = side.forearm();
    let rest_world = super::skeleton::forward(rest);
    let swing = wrap_angle(pose.bone(forearm).angle - rest_world.bone(forearm).angle);
    let base = match profile.reference {
        GripReference::Body => {
            let lean =
                wrap_angle(pose.bone(Bone::Spine).angle - rest_world.bone(Bone::Spine).angle);
            lean + profile.angle_degrees.to_radians()
        }
        GripReference::Forearm => {
            pose.bone(forearm).angle + profile.angle_degrees.to_radians()
                - profile.follow_forearm.clamp(0.0, 1.0) * swing
        }
        GripReference::World => profile.angle_degrees.to_radians(),
    };
    let angle = base + profile.follow_forearm.clamp(0.0, 1.0) * swing;
    Xform2::new(fist.0, fist.1, wrap_angle(angle))
}

#[cfg(test)]
mod tests {
    use super::super::skeleton::tests::standing;
    use super::super::skeleton::{fit, forward};
    use super::*;

    fn profile(reference: GripReference, angle: f32, follow: f32) -> GripProfile {
        GripProfile {
            reference,
            angle_degrees: angle,
            follow_forearm: follow,
            depth: Vec::new(),
        }
    }

    fn close(a: f32, b: f32) -> bool {
        wrap_angle(a - b).abs() < 1e-3
    }

    #[test]
    fn the_hand_frame_sits_on_the_fist() {
        let j = standing();
        let rest = fit(&j);
        let f = hand_frame(
            HandSide::Near,
            &j,
            &forward(&rest),
            &rest,
            &profile(GripReference::World, -50.0, 0.0),
        );
        assert_eq!(f.origin(), j.get(Joint::HandNear));
        assert!(close(f.angle, (-50f32).to_radians()));
    }

    #[test]
    fn follow_adds_a_fraction_of_the_forearm_swing_only() {
        let j = standing();
        let rest = fit(&j);
        let mut swung = rest.clone();
        swung.local[Bone::ForearmNear.index()].angle += 1.0;
        let p = profile(GripReference::Body, 200.0, 0.25);
        let still = hand_frame(HandSide::Near, &j, &forward(&rest), &rest, &p);
        let moved = hand_frame(HandSide::Near, &j, &forward(&swung), &rest, &p);
        assert!(close(moved.angle - still.angle, 0.25));
        let steady = profile(GripReference::Body, 200.0, 0.0);
        let a = hand_frame(HandSide::Near, &j, &forward(&rest), &rest, &steady);
        let b = hand_frame(HandSide::Near, &j, &forward(&swung), &rest, &steady);
        assert!(close(a.angle, b.angle), "independent of the forearm");
    }

    #[test]
    fn body_reference_tilts_with_the_spine() {
        let j = standing();
        let rest = fit(&j);
        let mut lean = rest.clone();
        lean.local[Bone::Spine.index()].angle -= 0.3;
        let p = profile(GripReference::Body, 90.0, 0.0);
        let up = hand_frame(HandSide::Near, &j, &forward(&rest), &rest, &p);
        let leaned = hand_frame(HandSide::Near, &j, &forward(&lean), &rest, &p);
        assert!(close(leaned.angle - up.angle, -0.3));
    }

    #[test]
    fn forearm_reference_reproduces_the_perpendicular_grip_of_01() {
        let j = standing();
        let rest = fit(&j);
        let pose = forward(&rest);
        let p = profile(GripReference::Forearm, 90.0, 0.0);
        let f = hand_frame(HandSide::Near, &j, &pose, &rest, &p);
        assert!(close(
            f.angle,
            pose.bone(Bone::ForearmNear).angle + std::f32::consts::FRAC_PI_2
        ));
    }

    #[test]
    fn per_frame_depth_falls_back_to_the_item_depth() {
        let mut p = profile(GripReference::World, 0.0, 0.0);
        p.depth = vec![DepthSlot::MainHandBehind, DepthSlot::MainHandGear];
        assert_eq!(
            p.depth_at(0, DepthSlot::MainHandGear),
            DepthSlot::MainHandBehind
        );
        assert_eq!(
            p.depth_at(5, DepthSlot::MainHandGear),
            DepthSlot::MainHandGear
        );
    }
}
