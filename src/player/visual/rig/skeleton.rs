//! The invisible skeleton of the protagonist. Pure module: no Bevy types.
//!
//! **Joints** are points marked on the approved frames (the same landmarks
//! in every pose); **bones** join two joints and form a hierarchy rooted at
//! the spine. A pose is fitted from the marked joints ([`fit`]): each bone
//! gets a world frame (origin on its first joint, `+x` towards its second)
//! and a local frame relative to its parent. [`forward`] composes local
//! frames back into world frames (forward kinematics, `world = parent ·
//! local`, the order of Agent Sprite Forge `rig_animate.py`).
//!
//! "Near" is the side of the body towards the camera when the art faces
//! right (drawn over the torso); "far" is behind it. The legs are `A` and
//! `B`: they cross in the side view, so which one is near is not decided
//! (see docs/development/modular_character_engine/).

use std::collections::BTreeMap;

use serde::Deserialize;

use super::xform::{RigPoint, Xform2, direction, distance, wrap_angle};

/// A landmark marked on every frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Joint {
    /// Centre of the hips (root of the spine and of both thighs).
    Pelvis,
    /// Base of the neck, between the shoulders.
    Neck,
    /// Centre of the head.
    Head,
    ShoulderNear,
    ElbowNear,
    /// Centre of the fist.
    HandNear,
    ShoulderFar,
    ElbowFar,
    HandFar,
    KneeA,
    AnkleA,
    ToeA,
    KneeB,
    AnkleB,
    ToeB,
    /// Torso box (02): back and front of the chest at the shoulder line,
    /// front and back of the waist above the belt. They drive the torso
    /// cage of flexible chest equipment and bound the torso in partitions.
    ChestBack,
    ChestFront,
    WaistFront,
    WaistBack,
}

/// Number of joints.
pub const JOINT_COUNT: usize = 19;

impl Joint {
    pub const ALL: [Self; JOINT_COUNT] = [
        Self::Pelvis,
        Self::Neck,
        Self::Head,
        Self::ShoulderNear,
        Self::ElbowNear,
        Self::HandNear,
        Self::ShoulderFar,
        Self::ElbowFar,
        Self::HandFar,
        Self::KneeA,
        Self::AnkleA,
        Self::ToeA,
        Self::KneeB,
        Self::AnkleB,
        Self::ToeB,
        Self::ChestBack,
        Self::ChestFront,
        Self::WaistFront,
        Self::WaistBack,
    ];

    /// Joints of the upper body (moved by an idle breath, see
    /// `data::RigFrame::drop`); the legs and the pelvis stay planted.
    pub const fn is_upper_body(self) -> bool {
        matches!(
            self,
            Self::Neck
                | Self::Head
                | Self::ShoulderNear
                | Self::ElbowNear
                | Self::HandNear
                | Self::ShoulderFar
                | Self::ElbowFar
                | Self::HandFar
                | Self::ChestBack
                | Self::ChestFront
        )
    }
}

/// A rigid segment between two joints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Bone {
    Spine,
    Head,
    UpperArmNear,
    ForearmNear,
    UpperArmFar,
    ForearmFar,
    ThighA,
    ShinA,
    FootA,
    ThighB,
    ShinB,
    FootB,
}

/// Number of bones.
pub const BONE_COUNT: usize = 12;

impl Bone {
    /// Every bone, parents before children.
    pub const ALL: [Self; BONE_COUNT] = [
        Self::Spine,
        Self::Head,
        Self::UpperArmNear,
        Self::ForearmNear,
        Self::UpperArmFar,
        Self::ForearmFar,
        Self::ThighA,
        Self::ShinA,
        Self::FootA,
        Self::ThighB,
        Self::ShinB,
        Self::FootB,
    ];

    pub const fn index(self) -> usize {
        self as usize
    }

    /// First and second joint.
    pub const fn joints(self) -> (Joint, Joint) {
        match self {
            Self::Spine => (Joint::Pelvis, Joint::Neck),
            Self::Head => (Joint::Neck, Joint::Head),
            Self::UpperArmNear => (Joint::ShoulderNear, Joint::ElbowNear),
            Self::ForearmNear => (Joint::ElbowNear, Joint::HandNear),
            Self::UpperArmFar => (Joint::ShoulderFar, Joint::ElbowFar),
            Self::ForearmFar => (Joint::ElbowFar, Joint::HandFar),
            Self::ThighA => (Joint::Pelvis, Joint::KneeA),
            Self::ShinA => (Joint::KneeA, Joint::AnkleA),
            Self::FootA => (Joint::AnkleA, Joint::ToeA),
            Self::ThighB => (Joint::Pelvis, Joint::KneeB),
            Self::ShinB => (Joint::KneeB, Joint::AnkleB),
            Self::FootB => (Joint::AnkleB, Joint::ToeB),
        }
    }

    pub const fn parent(self) -> Option<Self> {
        match self {
            Self::Spine => None,
            Self::Head | Self::UpperArmNear | Self::UpperArmFar | Self::ThighA | Self::ThighB => {
                Some(Self::Spine)
            }
            Self::ForearmNear => Some(Self::UpperArmNear),
            Self::ForearmFar => Some(Self::UpperArmFar),
            Self::ShinA => Some(Self::ThighA),
            Self::FootA => Some(Self::ShinA),
            Self::ShinB => Some(Self::ThighB),
            Self::FootB => Some(Self::ShinB),
        }
    }
}

/// Every joint of one pose, in rig space (y up, origin on the pivot).
#[derive(Debug, Clone, PartialEq)]
pub struct JointPositions {
    points: [RigPoint; JOINT_COUNT],
}

impl JointPositions {
    /// Builds from a complete map; `None` names the first missing joint.
    pub fn from_map(map: &BTreeMap<Joint, RigPoint>) -> Result<Self, Joint> {
        let mut points = [(0.0, 0.0); JOINT_COUNT];
        for (i, joint) in Joint::ALL.iter().enumerate() {
            points[i] = *map.get(joint).ok_or(*joint)?;
        }
        Ok(Self { points })
    }

    pub fn get(&self, joint: Joint) -> RigPoint {
        self.points[joint as usize]
    }

    pub fn set(&mut self, joint: Joint, p: RigPoint) {
        self.points[joint as usize] = p;
    }

    /// The upper body lowered by `drop` texels (an idle breath).
    pub fn with_drop(&self, drop: f32) -> Self {
        let mut out = self.clone();
        for joint in Joint::ALL {
            if joint.is_upper_body() {
                let (x, y) = self.get(joint);
                out.set(joint, (x, y - drop));
            }
        }
        out
    }

    /// Every bone as a segment, for overlays and partitions.
    pub fn segment(&self, bone: Bone) -> (RigPoint, RigPoint) {
        let (a, b) = bone.joints();
        (self.get(a), self.get(b))
    }
}

/// Local frames of every bone (relative to the parent's world frame; the
/// spine's is its world frame) and the bone lengths.
#[derive(Debug, Clone, PartialEq)]
pub struct LocalPose {
    pub local: [Xform2; BONE_COUNT],
    pub lengths: [f32; BONE_COUNT],
}

/// World frames of every bone.
#[derive(Debug, Clone, PartialEq)]
pub struct WorldPose {
    pub world: [Xform2; BONE_COUNT],
    pub lengths: [f32; BONE_COUNT],
}

impl WorldPose {
    pub fn bone(&self, bone: Bone) -> Xform2 {
        self.world[bone.index()]
    }

    pub fn length(&self, bone: Bone) -> f32 {
        self.lengths[bone.index()]
    }

    /// First and second joint of a bone.
    pub fn segment(&self, bone: Bone) -> (RigPoint, RigPoint) {
        let f = self.bone(bone);
        (f.origin(), f.apply((self.length(bone), 0.0)))
    }

    /// Angle of a bone relative to its parent (its own world angle for the
    /// spine).
    pub fn local_angle(&self, bone: Bone) -> f32 {
        let own = self.bone(bone).angle;
        match bone.parent() {
            Some(p) => wrap_angle(own - self.bone(p).angle),
            None => wrap_angle(own),
        }
    }
}

/// World frames straight from the marked joints.
pub fn world_from_joints(joints: &JointPositions) -> WorldPose {
    let mut world = [Xform2::IDENTITY; BONE_COUNT];
    let mut lengths = [0.0; BONE_COUNT];
    for bone in Bone::ALL {
        let (a, b) = joints.segment(bone);
        world[bone.index()] = Xform2::new(a.0, a.1, direction(a, b));
        lengths[bone.index()] = distance(a, b);
    }
    WorldPose { world, lengths }
}

/// Fits a pose to the marked joints: local frames such that [`forward`]
/// reproduces every joint.
pub fn fit(joints: &JointPositions) -> LocalPose {
    let world = world_from_joints(joints);
    let mut local = [Xform2::IDENTITY; BONE_COUNT];
    for bone in Bone::ALL {
        let w = world.bone(bone);
        local[bone.index()] = match bone.parent() {
            Some(p) => world.bone(p).inverse().compose(&w),
            None => w,
        };
    }
    LocalPose {
        local,
        lengths: world.lengths,
    }
}

/// Forward kinematics: composes the local frames down the hierarchy.
pub fn forward(pose: &LocalPose) -> WorldPose {
    let mut world = [Xform2::IDENTITY; BONE_COUNT];
    for bone in Bone::ALL {
        let local = pose.local[bone.index()];
        world[bone.index()] = match bone.parent() {
            Some(p) => world[p.index()].compose(&local),
            None => local,
        };
    }
    WorldPose {
        world,
        lengths: pose.lengths,
    }
}

/// Retargets a pose onto other proportions: the rest pose's offsets and
/// lengths with the pose's local angles (and the pose's root frame).
pub fn retarget(rest: &LocalPose, pose: &LocalPose) -> LocalPose {
    let mut local = rest.local;
    for bone in Bone::ALL {
        let i = bone.index();
        local[i].angle = pose.local[i].angle;
    }
    local[Bone::Spine.index()] = pose.local[Bone::Spine.index()];
    LocalPose {
        local,
        lengths: rest.lengths,
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// A plausible standing pose (rig space).
    pub(crate) fn standing() -> JointPositions {
        let map: BTreeMap<Joint, RigPoint> = [
            (Joint::Pelvis, (-2.0, 58.0)),
            (Joint::Neck, (-1.0, 89.0)),
            (Joint::Head, (0.0, 98.0)),
            (Joint::ShoulderNear, (-1.0, 85.0)),
            (Joint::ElbowNear, (0.0, 69.0)),
            (Joint::HandNear, (2.0, 51.0)),
            (Joint::ShoulderFar, (-4.0, 85.0)),
            (Joint::ElbowFar, (-4.0, 71.0)),
            (Joint::HandFar, (-3.0, 62.0)),
            (Joint::KneeA, (1.0, 32.0)),
            (Joint::AnkleA, (-3.0, 7.0)),
            (Joint::ToeA, (6.0, 2.0)),
            (Joint::KneeB, (0.0, 32.0)),
            (Joint::AnkleB, (-4.0, 7.0)),
            (Joint::ToeB, (4.0, 2.0)),
            (Joint::ChestBack, (-7.0, 86.0)),
            (Joint::ChestFront, (5.0, 86.0)),
            (Joint::WaistFront, (4.0, 68.0)),
            (Joint::WaistBack, (-5.0, 68.0)),
        ]
        .into_iter()
        .collect();
        JointPositions::from_map(&map).unwrap_or_else(|j| panic!("missing {j:?}"))
    }

    fn close(a: RigPoint, b: RigPoint) -> bool {
        (a.0 - b.0).abs() < 1e-3 && (a.1 - b.1).abs() < 1e-3
    }

    #[test]
    fn every_bone_has_its_parent_listed_first() {
        for (i, bone) in Bone::ALL.iter().enumerate() {
            assert_eq!(bone.index(), i);
            if let Some(p) = bone.parent() {
                assert!(p.index() < i, "{bone:?} before its parent");
            }
        }
    }

    #[test]
    fn a_missing_joint_is_named() {
        let mut map = BTreeMap::new();
        map.insert(Joint::Pelvis, (0.0, 0.0));
        assert_eq!(JointPositions::from_map(&map), Err(Joint::Neck));
    }

    #[test]
    fn fit_then_forward_reproduces_every_joint() {
        let joints = standing();
        let world = forward(&fit(&joints));
        for bone in Bone::ALL {
            let (a, b) = world.segment(bone);
            let (ea, eb) = joints.segment(bone);
            assert!(close(a, ea) && close(b, eb), "{bone:?}");
        }
    }

    #[test]
    fn rotating_a_parent_carries_its_children() {
        let mut pose = fit(&standing());
        let before = forward(&pose).segment(Bone::ForearmNear).1;
        pose.local[Bone::UpperArmNear.index()].angle += 0.5;
        let world = forward(&pose);
        let after = world.segment(Bone::ForearmNear).1;
        let shoulder = world.segment(Bone::UpperArmNear).0;
        assert!(!close(before, after), "the hand moved");
        // The hand turned 0.5 rad around the shoulder, at the same distance.
        let d0 = distance(shoulder, before);
        let d1 = distance(shoulder, after);
        assert!((d0 - d1).abs() < 1e-3);
        // Unrelated bones stay.
        assert_eq!(
            world.segment(Bone::ThighA),
            forward(&fit(&standing())).segment(Bone::ThighA)
        );
    }

    #[test]
    fn drop_lowers_only_the_upper_body() {
        let joints = standing().with_drop(1.0);
        assert_eq!(joints.get(Joint::Head).1, 97.0);
        assert_eq!(joints.get(Joint::HandNear).1, 50.0);
        assert_eq!(joints.get(Joint::Pelvis).1, 58.0);
        assert_eq!(joints.get(Joint::ToeA).1, 2.0);
    }

    #[test]
    fn retarget_keeps_rest_lengths_and_pose_angles() {
        let rest = fit(&standing());
        let mut other = standing();
        other.set(Joint::ElbowNear, (10.0, 80.0));
        other.set(Joint::HandNear, (20.0, 90.0));
        let pose = fit(&other);
        let world = forward(&retarget(&rest, &pose));
        let (a, b) = world.segment(Bone::ForearmNear);
        assert!((distance(a, b) - rest.lengths[Bone::ForearmNear.index()]).abs() < 1e-3);
        assert!(
            (world.local_angle(Bone::UpperArmNear)
                - forward(&pose).local_angle(Bone::UpperArmNear))
            .abs()
                < 1e-3
        );
    }
}
