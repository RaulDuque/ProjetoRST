//! The fully articulated comparison: the rest pose's pieces driven to a
//! target pose. Pure module: no Bevy types.
//!
//! The target pose's local angles are retargeted onto the rest proportions
//! (forward kinematics); then each leg is solved with two-bone IK so the
//! ankle lands where the target frame has it (planted feet do not follow
//! from the angles alone once the proportions differ), and the foot keeps
//! the target's world angle (as Agent Sprite Forge `rig_animate.py` does).

use super::ik::{Bend, solve_two_bone};
use super::skeleton::{Bone, Joint, JointPositions, LocalPose, WorldPose, fit, forward, retarget};
use super::xform::Xform2;

/// A piece of the rest pose placed by the articulated pose.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PieceTransform {
    /// Where the bone starts in the target pose.
    pub origin: (f32, f32),
    /// Rotation from the rest bone to the target bone (radians).
    pub rotation: f32,
}

/// World pose of the rest proportions in the target's attitude, legs by IK.
pub fn articulate(rest: &LocalPose, target: &JointPositions) -> WorldPose {
    let mut world = forward(&retarget(rest, &fit(target)));
    for (thigh, shin, foot, ankle, toe) in [
        (
            Bone::ThighA,
            Bone::ShinA,
            Bone::FootA,
            Joint::AnkleA,
            Joint::ToeA,
        ),
        (
            Bone::ThighB,
            Bone::ShinB,
            Bone::FootB,
            Joint::AnkleB,
            Joint::ToeB,
        ),
    ] {
        let hip = world.bone(thigh).origin();
        let knee_target = world.segment(thigh).1;
        let goal = target.get(ankle);
        let (l1, l2) = (world.length(thigh), world.length(shin));
        // Keep the knee on the side the retargeted pose bends it to.
        let side =
            (goal.0 - hip.0) * (knee_target.1 - hip.1) - (goal.1 - hip.1) * (knee_target.0 - hip.0);
        let bend = if side > 0.0 {
            Bend::CounterClockwise
        } else {
            Bend::Clockwise
        };
        let s = solve_two_bone(hip, goal, l1, l2, bend);
        world.world[thigh.index()] = Xform2::new(hip.0, hip.1, s.first);
        world.world[shin.index()] = Xform2::new(s.middle.0, s.middle.1, s.second);
        let toe_angle = super::xform::direction(target.get(ankle), target.get(toe));
        world.world[foot.index()] = Xform2::new(s.end.0, s.end.1, toe_angle);
    }
    world
}

/// How to move a rest piece onto the articulated pose.
pub fn piece_transform(rest: &WorldPose, pose: &WorldPose, bone: Bone) -> PieceTransform {
    PieceTransform {
        origin: pose.bone(bone).origin(),
        rotation: pose.bone(bone).angle - rest.bone(bone).angle,
    }
}

#[cfg(test)]
mod tests {
    use super::super::skeleton::tests::standing;
    use super::super::xform::distance;
    use super::*;

    #[test]
    fn the_rest_pose_articulates_onto_itself() {
        let j = standing();
        let rest = fit(&j);
        let world = articulate(&rest, &j);
        for bone in Bone::ALL {
            let (a, b) = world.segment(bone);
            let (ea, eb) = j.segment(bone);
            assert!(distance(a, ea) < 1e-2 && distance(b, eb) < 0.1, "{bone:?}");
        }
    }

    #[test]
    fn ankles_reach_the_target_with_rest_lengths() {
        let rest = fit(&standing());
        let mut target = standing();
        target.set(Joint::KneeA, (12.0, 36.0));
        target.set(Joint::AnkleA, (4.0, 12.0));
        target.set(Joint::ToeA, (12.0, 8.0));
        let world = articulate(&rest, &target);
        let ankle = world.segment(Bone::ShinA).1;
        assert!(distance(ankle, (4.0, 12.0)) < 1e-2);
        let thigh = world.length(Bone::ThighA);
        let (h, k) = world.segment(Bone::ThighA);
        assert!((distance(h, k) - thigh).abs() < 1e-3);
        assert!(k.0 > 0.0, "the knee stays forward");
    }

    #[test]
    fn piece_rotation_is_relative_to_the_rest_bone() {
        let j = standing();
        let rest = forward(&fit(&j));
        let t = piece_transform(&rest, &rest, Bone::ForearmNear);
        assert!(t.rotation.abs() < 1e-6);
        assert!(distance(t.origin, j.get(Joint::ElbowNear)) < 1e-4);
    }
}
