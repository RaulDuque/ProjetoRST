//! Rig-bound equipment: a piece drawn once, placed on the skeleton every
//! frame. Pure module: no Bevy types.
//!
//! An item owns its art (a sprite with size and pivot, or a vector model),
//! how it is mounted (on a bone, in a hand through a grip profile, or on the
//! torso cage), how it deforms (rigid, articulated in rigid segments, or
//! flexible) and its depth. The motion belongs to the skeleton.
//!
//! The weapon socket model (a grip pivot in the hand, the blade along an
//! axis of the hand) follows pixelfx `src/core/rig.js` (MIT, Copyright (c)
//! 2026 Patch, Hugo Duprez); the partial follow of the pauldron, the torso
//! cage and the contract are specific to Dark Castle.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::grip::HandSide;
use super::layers::DepthSlot;
use super::skeleton::{Bone, LocalPose, WorldPose};
use super::xform::{RigPoint, Xform2, wrap_angle};
use crate::player::visual::sockets;

/// Equipment slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum RigSlot {
    /// Sword (near hand).
    MainHand,
    /// Shield (far hand).
    OffHand,
    Shoulder,
    Chest,
    Bracers,
    Cape,
    Helmet,
    Belt,
    Legs,
    Boots,
}

impl RigSlot {
    pub const ALL: [Self; 10] = [
        Self::MainHand,
        Self::OffHand,
        Self::Shoulder,
        Self::Chest,
        Self::Bracers,
        Self::Cape,
        Self::Helmet,
        Self::Belt,
        Self::Legs,
        Self::Boots,
    ];
}

/// How an item follows the body.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum DeformKind {
    /// One rigid piece (sword, shield, pauldron, helmet).
    Rigid,
    /// Several rigid segments, each on its own bone (bracers, belt plates,
    /// greaves).
    Articulated,
    /// Deformed by a cage bound to joints (breastplate; capes later).
    Flexible,
}

/// Which end of the bone an item hangs from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum BoneEnd {
    Start,
    End,
}

/// A sprite drawn once: asset path, size and pivot (texels from its
/// top-left corner).
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SpriteArt {
    pub texture: String,
    pub size: (u32, u32),
    pub pivot: (f32, f32),
}

impl SpriteArt {
    fn is_valid(&self) -> bool {
        let (w, h) = self.size;
        w > 0
            && h > 0
            && (0.0..=w as f32).contains(&self.pivot.0)
            && (0.0..=h as f32).contains(&self.pivot.1)
    }
}

/// The art source of an item.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum ItemArt {
    Sprite(SpriteArt),
    /// Id of a vector model of the lab (`RigLab::models`).
    Model(String),
}

/// How an item is attached to the skeleton.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum Mount {
    /// On a bone: end, offset (along, across), where the image's up points
    /// (degrees from the bone direction) and how much of the bone's swing it
    /// follows (1 rigid, 0 the parent's frame).
    Bone {
        bone: Bone,
        at: BoneEnd,
        offset: (f32, f32),
        up_degrees: f32,
        follow: f32,
    },
    /// In a hand: the clip's grip profile gives the hand frame; the image's
    /// up points along the blade, turned by `up_degrees`.
    Hand { side: HandSide, up_degrees: f32 },
    /// On the torso cage (vector model).
    Torso,
}

/// One rigid piece of an articulated item.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Segment {
    pub art: SpriteArt,
    pub mount: Mount,
    pub depth: DepthSlot,
}

/// One piece of equipment, as authored in the lab RON.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct RigItem {
    pub slot: RigSlot,
    pub kind: DeformKind,
    pub art: ItemArt,
    pub mount: Mount,
    /// Articulated items: their rigid segments (the main art and mount are
    /// the first segment).
    #[serde(default)]
    pub segments: Vec<Segment>,
    pub depth: DepthSlot,
    /// Diagnostic or derived art that is not final.
    pub provisional: bool,
    /// Where the art came from (credits and provenance).
    pub source: String,
}

/// Why an item is refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemProblem {
    /// Pivot outside the image or empty image.
    Sprite,
    /// `follow` outside 0..=1.
    Follow,
    /// A vector model needs the torso mount and the torso mount a model.
    TorsoModel,
    /// Flexible items need the torso cage (the only cage implemented).
    FlexibleWithoutCage,
    /// Articulated items need segments on bones; others have none.
    Segments,
    /// Hand mounts only for the hand slots.
    HandSlot,
}

fn mount_ok(mount: &Mount) -> bool {
    match mount {
        Mount::Bone { follow, .. } => (0.0..=1.0).contains(follow),
        _ => true,
    }
}

impl RigItem {
    /// Checks the combination of art, mount, kind and slot.
    pub fn check(&self) -> Result<(), ItemProblem> {
        if let ItemArt::Sprite(s) = &self.art
            && !s.is_valid()
        {
            return Err(ItemProblem::Sprite);
        }
        if !mount_ok(&self.mount) {
            return Err(ItemProblem::Follow);
        }
        let model = matches!(self.art, ItemArt::Model(_));
        let torso = matches!(self.mount, Mount::Torso);
        if model != torso {
            return Err(ItemProblem::TorsoModel);
        }
        if self.kind == DeformKind::Flexible && !torso {
            return Err(ItemProblem::FlexibleWithoutCage);
        }
        let articulated = self.kind == DeformKind::Articulated;
        if articulated == self.segments.is_empty() {
            return Err(ItemProblem::Segments);
        }
        for s in &self.segments {
            if !s.art.is_valid() {
                return Err(ItemProblem::Sprite);
            }
            if !matches!(s.mount, Mount::Bone { .. }) || !mount_ok(&s.mount) {
                return Err(ItemProblem::Segments);
            }
        }
        if matches!(self.mount, Mount::Hand { .. })
            && !matches!(self.slot, RigSlot::MainHand | RigSlot::OffHand)
        {
            return Err(ItemProblem::HandSlot);
        }
        Ok(())
    }

    /// The sprite of a sprite item.
    pub fn sprite(&self) -> Option<&SpriteArt> {
        match &self.art {
            ItemArt::Sprite(s) => Some(s),
            ItemArt::Model(_) => None,
        }
    }
}

/// Where a piece goes in one frame, in rig space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Placement {
    pub position: RigPoint,
    /// Radians counter-clockwise of the image (0 = upright).
    pub rotation: f32,
    pub depth: DepthSlot,
}

/// Places a bone mount on a pose. `rest` is the pose the item was authored
/// on (the follow fraction is measured from its local angles). `None` for
/// a mount that is not on a bone.
pub fn place_on_bone(
    mount: &Mount,
    depth: DepthSlot,
    pose: &WorldPose,
    rest: &LocalPose,
) -> Option<Placement> {
    let Mount::Bone {
        bone,
        at,
        offset,
        up_degrees,
        follow,
    } = mount
    else {
        return None;
    };
    let frame = pose.bone(*bone);
    let frame_angle = match bone.parent() {
        Some(parent) => {
            let rest_local = rest.local[bone.index()].angle;
            let swing = wrap_angle(pose.local_angle(*bone) - rest_local);
            pose.bone(parent).angle + rest_local + follow.clamp(0.0, 1.0) * swing
        }
        None => frame.angle,
    };
    let base = match at {
        BoneEnd::Start => frame.origin(),
        BoneEnd::End => frame.apply((pose.length(*bone), 0.0)),
    };
    let (s, c) = frame_angle.sin_cos();
    let (ox, oy) = *offset;
    Some(Placement {
        position: (base.0 + c * ox - s * oy, base.1 + s * ox + c * oy),
        rotation: wrap_angle(frame_angle + up_degrees.to_radians() - std::f32::consts::FRAC_PI_2),
        depth,
    })
}

/// Places a hand-held item on the hand frame (`grip::hand_frame`): the
/// image's up follows the blade direction.
pub fn place_in_hand(hand: Xform2, up_degrees: f32, depth: DepthSlot) -> Placement {
    Placement {
        position: hand.origin(),
        rotation: wrap_angle(hand.angle + up_degrees.to_radians() - std::f32::consts::FRAC_PI_2),
        depth,
    }
}

/// Rig-space point of a texel of a placed sprite (e.g. the blade tip),
/// before mirroring.
pub fn sprite_point(placement: &Placement, art: &SpriteArt, texel: (f32, f32)) -> RigPoint {
    let local = (texel.0 - art.pivot.0, art.pivot.1 - texel.1);
    Xform2::new(
        placement.position.0,
        placement.position.1,
        placement.rotation,
    )
    .apply(local)
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

    fn sprite() -> SpriteArt {
        SpriteArt {
            texture: "s.png".into(),
            size: (21, 64),
            pivot: (10.0, 53.5),
        }
    }

    fn bone_mount(bone: Bone, at: BoneEnd, up: f32, follow: f32) -> Mount {
        Mount::Bone {
            bone,
            at,
            offset: (0.0, 0.0),
            up_degrees: up,
            follow,
        }
    }

    fn item(slot: RigSlot, kind: DeformKind, art: ItemArt, mount: Mount) -> RigItem {
        RigItem {
            slot,
            kind,
            art,
            mount,
            segments: Vec::new(),
            depth: DepthSlot::MainHandGear,
            provisional: false,
            source: "test".into(),
        }
    }

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-3
    }

    #[test]
    fn a_bone_item_sits_on_its_joint_in_every_pose() {
        let joints = standing();
        let rest = fit(&joints);
        let m = bone_mount(Bone::ForearmNear, BoneEnd::End, 90.0, 1.0);
        let p = place_on_bone(&m, DepthSlot::MainHandGear, &forward(&rest), &rest)
            .unwrap_or_else(|| panic!("bone"));
        let fist = joints.get(Joint::HandNear);
        assert!(close(p.position.0, fist.0) && close(p.position.1, fist.1));
        let mut raised = joints.clone();
        raised.set(Joint::HandNear, (14.0, 78.0));
        let p = place_on_bone(&m, DepthSlot::MainHandGear, &forward(&fit(&raised)), &rest)
            .unwrap_or_else(|| panic!("bone"));
        assert!(close(p.position.0, 14.0) && close(p.position.1, 78.0));
        assert!(
            place_on_bone(&Mount::Torso, DepthSlot::ChestGear, &forward(&rest), &rest).is_none()
        );
    }

    #[test]
    fn follow_scales_the_swing_from_the_rest_angle() {
        let rest = fit(&standing());
        let m = bone_mount(Bone::UpperArmNear, BoneEnd::Start, 180.0, 0.5);
        let still = place_on_bone(&m, DepthSlot::ShoulderGear, &forward(&rest), &rest)
            .unwrap_or_else(|| panic!("bone"));
        let mut swung = rest.clone();
        swung.local[Bone::UpperArmNear.index()].angle += FRAC_PI_2;
        let moved = place_on_bone(&m, DepthSlot::ShoulderGear, &forward(&swung), &rest)
            .unwrap_or_else(|| panic!("bone"));
        assert!(close(
            wrap_angle(moved.rotation - still.rotation),
            FRAC_PI_2 * 0.5
        ));
        assert!(close(moved.position.0, still.position.0));
    }

    #[test]
    fn a_hand_item_points_its_image_up_along_the_blade() {
        let hand = Xform2::new(3.0, 50.0, 0.0);
        let p = place_in_hand(hand, 0.0, DepthSlot::MainHandGear);
        assert_eq!(p.position, (3.0, 50.0));
        // Blade along +x -> the upright image turned -90 degrees.
        assert!(close(p.rotation, -FRAC_PI_2));
        // The tip (texel row 4, 49.5 texels above the pivot) lies along +x.
        let tip = sprite_point(&p, &sprite(), (10.0, 4.0));
        assert!(close(tip.0, 3.0 + 49.5) && close(tip.1, 50.0));
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
    fn the_contract_accepts_every_slot_with_a_coherent_mount() {
        let rigid = |slot, mount| item(slot, DeformKind::Rigid, ItemArt::Sprite(sprite()), mount);
        let ok = [
            rigid(
                RigSlot::MainHand,
                Mount::Hand {
                    side: HandSide::Near,
                    up_degrees: 0.0,
                },
            ),
            rigid(
                RigSlot::OffHand,
                Mount::Hand {
                    side: HandSide::Far,
                    up_degrees: 0.0,
                },
            ),
            rigid(
                RigSlot::Shoulder,
                bone_mount(Bone::UpperArmNear, BoneEnd::Start, 180.0, 0.5),
            ),
            rigid(
                RigSlot::Helmet,
                bone_mount(Bone::Head, BoneEnd::End, 90.0, 1.0),
            ),
            item(
                RigSlot::Chest,
                DeformKind::Flexible,
                ItemArt::Model("plate".into()),
                Mount::Torso,
            ),
            item(
                RigSlot::Chest,
                DeformKind::Rigid,
                ItemArt::Model("plate".into()),
                Mount::Torso,
            ),
        ];
        for i in &ok {
            assert_eq!(i.check(), Ok(()), "{:?}", i.slot);
        }
        let mut bracers = rigid(
            RigSlot::Bracers,
            bone_mount(Bone::ForearmNear, BoneEnd::Start, 90.0, 1.0),
        );
        bracers.kind = DeformKind::Articulated;
        bracers.segments = vec![Segment {
            art: sprite(),
            mount: bone_mount(Bone::ForearmFar, BoneEnd::Start, 90.0, 1.0),
            depth: DepthSlot::FarArm,
        }];
        assert_eq!(bracers.check(), Ok(()));
        for slot in [RigSlot::Cape, RigSlot::Belt, RigSlot::Legs, RigSlot::Boots] {
            assert!(RigSlot::ALL.contains(&slot));
        }
    }

    #[test]
    fn the_contract_refuses_incoherent_items() {
        let bad_pivot = item(
            RigSlot::Shoulder,
            DeformKind::Rigid,
            ItemArt::Sprite(SpriteArt {
                pivot: (30.0, 1.0),
                ..sprite()
            }),
            bone_mount(Bone::UpperArmNear, BoneEnd::Start, 0.0, 1.0),
        );
        assert_eq!(bad_pivot.check(), Err(ItemProblem::Sprite));
        let follow = item(
            RigSlot::Shoulder,
            DeformKind::Rigid,
            ItemArt::Sprite(sprite()),
            bone_mount(Bone::UpperArmNear, BoneEnd::Start, 0.0, 2.0),
        );
        assert_eq!(follow.check(), Err(ItemProblem::Follow));
        let model_on_bone = item(
            RigSlot::Chest,
            DeformKind::Rigid,
            ItemArt::Model("plate".into()),
            bone_mount(Bone::Spine, BoneEnd::Start, 0.0, 1.0),
        );
        assert_eq!(model_on_bone.check(), Err(ItemProblem::TorsoModel));
        let flexible_bone = item(
            RigSlot::Cape,
            DeformKind::Flexible,
            ItemArt::Sprite(sprite()),
            bone_mount(Bone::Spine, BoneEnd::Start, 0.0, 1.0),
        );
        assert_eq!(flexible_bone.check(), Err(ItemProblem::FlexibleWithoutCage));
        let no_segments = item(
            RigSlot::Belt,
            DeformKind::Articulated,
            ItemArt::Sprite(sprite()),
            bone_mount(Bone::ThighA, BoneEnd::Start, 0.0, 1.0),
        );
        assert_eq!(no_segments.check(), Err(ItemProblem::Segments));
        let helmet_in_hand = item(
            RigSlot::Helmet,
            DeformKind::Rigid,
            ItemArt::Sprite(sprite()),
            Mount::Hand {
                side: HandSide::Near,
                up_degrees: 0.0,
            },
        );
        assert_eq!(helmet_in_hand.check(), Err(ItemProblem::HandSlot));
    }

    #[test]
    fn loadout_swaps_one_item_per_slot() {
        let mut l = RigLoadout::default();
        assert_eq!(l.equip(RigSlot::MainHand, "a"), None);
        assert_eq!(l.equip(RigSlot::MainHand, "b"), Some("a".into()));
        assert_eq!(l.worn(RigSlot::MainHand), Some("b"));
        l.equip(RigSlot::Chest, "p");
        assert_eq!(l.items().count(), 2);
        assert_eq!(l.unequip(RigSlot::MainHand), Some("b".into()));
        assert_eq!(l.worn(RigSlot::MainHand), None);
    }
}
