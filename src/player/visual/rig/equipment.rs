//! Rig-bound equipment: a piece drawn once, placed on a bone every frame.
//! Pure module: no Bevy types.
//!
//! An item owns its image, size, pivot, bone, offset and depth; the motion
//! belongs to the skeleton. [`place`] turns the pose of one frame into the
//! item's position and rotation in rig space; [`mirror`] applies the facing
//! with the same rule as the V2.2 sockets (`sockets::place`).
//!
//! The weapon socket model (a grip pivot on the hand, the blade along an
//! axis fixed to the hand) follows pixelfx `src/core/rig.js` (MIT,
//! Copyright (c) 2026 Patch, Hugo Duprez); the partial follow of the
//! pauldron is specific to Dark Castle.

use std::collections::BTreeMap;

use serde::Deserialize;

use super::layers::DepthSlot;
use super::skeleton::{Bone, LocalPose, WorldPose};
use super::xform::{RigPoint, wrap_angle};
use crate::player::visual::sockets;

/// Equipment slot of the prototype.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub enum RigSlot {
    MainHand,
    Shoulder,
}

/// Which end of the bone the item hangs from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum BoneEnd {
    Start,
    End,
}

/// One piece of equipment, as authored in the lab RON.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct RigItem {
    pub slot: RigSlot,
    /// Asset path of the image (one frame, drawn once).
    pub texture: String,
    /// Image size in texels.
    pub size: (u32, u32),
    /// Texel of the image placed on the socket (from its top-left corner).
    pub pivot: (f32, f32),
    pub bone: Bone,
    pub at: BoneEnd,
    /// Offset of the pivot from the bone end, in the item frame (texels:
    /// along the bone, then across it counter-clockwise).
    pub offset: (f32, f32),
    /// Where the image's "up" points, in degrees counter-clockwise from the
    /// bone direction (90 = perpendicular to a forearm, blade over the
    /// thumb; 180 = back towards the bone start).
    pub up_degrees: f32,
    /// How much of the bone's swing (relative to its rest angle) the item
    /// follows: 1 = rigidly bound, 0 = keeps the parent's frame.
    pub follow: f32,
    pub depth: DepthSlot,
    /// Diagnostic or derived art that is not final.
    pub provisional: bool,
    /// Where the art came from (credits and provenance).
    pub source: String,
}

/// Where an item goes in one frame, in rig space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Placement {
    pub position: RigPoint,
    /// Radians counter-clockwise of the image (0 = upright).
    pub rotation: f32,
    pub depth: DepthSlot,
}

/// Places an item on a pose. `rest` is the pose the item was authored on
/// (the follow fraction is measured from its local angles).
pub fn place(item: &RigItem, pose: &WorldPose, rest: &LocalPose) -> Placement {
    let bone = pose.bone(item.bone);
    let frame_angle = match item.bone.parent() {
        Some(parent) => {
            let rest_local = rest.local[item.bone.index()].angle;
            let swing = wrap_angle(pose.local_angle(item.bone) - rest_local);
            pose.bone(parent).angle + rest_local + item.follow.clamp(0.0, 1.0) * swing
        }
        None => bone.angle,
    };
    let base = match item.at {
        BoneEnd::Start => bone.origin(),
        BoneEnd::End => bone.apply((pose.length(item.bone), 0.0)),
    };
    let (s, c) = frame_angle.sin_cos();
    let (ox, oy) = item.offset;
    let position = (base.0 + c * ox - s * oy, base.1 + s * ox + c * oy);
    Placement {
        position,
        rotation: wrap_angle(
            frame_angle + item.up_degrees.to_radians() - std::f32::consts::FRAC_PI_2,
        ),
        depth: item.depth,
    }
}

/// The placement seen facing left (`flip`) or right: x and the rotation
/// are mirrored, the depth stays (see `layers::DepthSlot`).
pub fn mirror(p: Placement, flip: bool) -> Placement {
    if !flip {
        return p;
    }
    let (x, y, rotation) =
        sockets::place(p.position.0, p.position.1, p.rotation.to_degrees(), -1.0);
    Placement {
        position: (x, y),
        rotation,
        depth: p.depth,
    }
}

/// What is worn: at most one item per slot. Changing it never touches the
/// animation clock (that belongs to the body, `VisualPlayback`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RigLoadout {
    worn: BTreeMap<RigSlot, String>,
}

impl RigLoadout {
    /// Wears `id` in `slot`, returning what was there.
    pub fn equip(&mut self, slot: RigSlot, id: &str) -> Option<String> {
        self.worn.insert(slot, id.to_owned())
    }

    pub fn unequip(&mut self, slot: RigSlot) -> Option<String> {
        self.worn.remove(&slot)
    }

    pub fn worn(&self, slot: RigSlot) -> Option<&str> {
        self.worn.get(&slot).map(String::as_str)
    }

    /// Every worn item id, in slot order.
    pub fn items(&self) -> impl Iterator<Item = (RigSlot, &str)> {
        self.worn.iter().map(|(s, id)| (*s, id.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::super::skeleton::tests::standing;
    use super::super::skeleton::{Joint, fit, forward};
    use super::*;
    use std::f32::consts::FRAC_PI_2;

    fn sword() -> RigItem {
        RigItem {
            slot: RigSlot::MainHand,
            texture: "s.png".into(),
            size: (21, 64),
            pivot: (10.0, 53.0),
            bone: Bone::ForearmNear,
            at: BoneEnd::End,
            offset: (0.0, 0.0),
            up_degrees: 90.0,
            follow: 1.0,
            depth: DepthSlot::MainHandGear,
            provisional: false,
            source: "test".into(),
        }
    }

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-3
    }

    #[test]
    fn a_hand_item_sits_on_the_fist_in_every_pose() {
        let joints = standing();
        let rest = fit(&joints);
        let world = forward(&rest);
        let p = place(&sword(), &world, &rest);
        let fist = joints.get(Joint::HandNear);
        assert!(close(p.position.0, fist.0) && close(p.position.1, fist.1));
        // A raised forearm: the item follows the new fist.
        let mut raised = joints.clone();
        raised.set(Joint::HandNear, (14.0, 78.0));
        let pose = forward(&fit(&raised));
        let p = place(&sword(), &pose, &rest);
        assert!(close(p.position.0, 14.0) && close(p.position.1, 78.0));
    }

    #[test]
    fn a_horizontal_forearm_holds_the_blade_upright() {
        let mut joints = standing();
        let elbow = joints.get(Joint::ElbowNear);
        joints.set(Joint::HandNear, (elbow.0 + 15.0, elbow.1));
        let rest = fit(&standing());
        let p = place(&sword(), &forward(&fit(&joints)), &rest);
        assert!(close(p.rotation, 0.0), "rotation {}", p.rotation);
    }

    #[test]
    fn follow_scales_the_swing_from_the_rest_angle() {
        let joints = standing();
        let rest = fit(&joints);
        let mut pauldron = sword();
        pauldron.bone = Bone::UpperArmNear;
        pauldron.at = BoneEnd::Start;
        pauldron.up_degrees = 180.0;
        pauldron.follow = 0.5;
        let still = place(&pauldron, &forward(&rest), &rest);
        let mut swung = rest.clone();
        swung.local[Bone::UpperArmNear.index()].angle += FRAC_PI_2;
        let moved = place(&pauldron, &forward(&swung), &rest);
        assert!(close(
            wrap_angle(moved.rotation - still.rotation),
            FRAC_PI_2 * 0.5
        ));
        // The shoulder does not move, so neither does the pivot.
        assert!(close(moved.position.0, still.position.0));
    }

    #[test]
    fn mirroring_negates_x_and_rotation_and_keeps_depth() {
        let p = Placement {
            position: (6.0, 40.0),
            rotation: 0.4,
            depth: DepthSlot::ShoulderGear,
        };
        let m = mirror(p, true);
        assert!(close(m.position.0, -6.0) && close(m.position.1, 40.0));
        assert!(close(m.rotation, -0.4));
        assert_eq!(m.depth, DepthSlot::ShoulderGear);
        assert_eq!(mirror(p, false), p);
    }

    #[test]
    fn loadout_swaps_one_item_per_slot() {
        let mut l = RigLoadout::default();
        assert_eq!(l.equip(RigSlot::MainHand, "a"), None);
        assert_eq!(l.equip(RigSlot::MainHand, "b"), Some("a".into()));
        assert_eq!(l.worn(RigSlot::MainHand), Some("b"));
        l.equip(RigSlot::Shoulder, "p");
        assert_eq!(l.items().count(), 2);
        assert_eq!(l.unequip(RigSlot::MainHand), Some("b".into()));
        assert_eq!(l.worn(RigSlot::MainHand), None);
    }
}
