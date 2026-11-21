//! Depth layers of an approved frame, cut by the skeleton. Pure module.
//!
//! The approved art is one flat image per frame. To put equipment *between*
//! parts of the body (a sword under the fist, a pauldron over the upper
//! arm), every opaque texel is given to exactly one [`BodyLayer`] from the
//! frame's marked skeleton: texels of arm materials inside a capsule around
//! the near (far) arm bones go to the near (far) arm, hair-coloured texels
//! behind the spine or above the shoulders go to the hair, the rest to the
//! torso or the legs. It is a **partition**: the layers recompose the frame
//! texel for texel, no texel is invented, moved or recoloured.
//!
//! [`partition_by_bone`] cuts a frame into one piece per bone of the near
//! chain instead: the pieces of the fully articulated comparison.

use serde::Deserialize;

use super::skeleton::{Bone, Joint, JointPositions};
use super::xform::{RigPoint, distance, distance_to_segment};

/// What a texel of the approved art shows, from the N3.4 palette
/// (`art-src/n3-4-original-protagonist/scripts/n34_palette.py`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Material {
    Transparent,
    Outline,
    Hair,
    Skin,
    Eye,
    Shirt,
    Trousers,
    Leather,
    Metal,
    /// A colour outside the palette (kept with the torso or the legs).
    Other,
}

/// N3.4 palette colours and their material.
const PALETTE: [([u8; 3], Material); 19] = [
    ([26, 20, 26], Material::Outline),
    ([238, 238, 244], Material::Hair),
    ([204, 206, 218], Material::Hair),
    ([156, 160, 180], Material::Hair),
    ([104, 108, 130], Material::Hair),
    ([236, 200, 172], Material::Skin),
    ([204, 154, 126], Material::Skin),
    ([146, 98, 86], Material::Skin),
    ([58, 52, 64], Material::Eye),
    ([138, 134, 140], Material::Shirt),
    ([104, 100, 108], Material::Shirt),
    ([72, 68, 78], Material::Shirt),
    ([88, 70, 70], Material::Trousers),
    ([62, 48, 50], Material::Trousers),
    ([44, 34, 38], Material::Trousers),
    ([104, 84, 74], Material::Leather),
    ([70, 54, 50], Material::Leather),
    ([46, 36, 36], Material::Leather),
    ([176, 172, 168], Material::Metal),
];

/// Material of an RGBA texel (alpha 0 is transparent).
pub fn material_of(rgba: [u8; 4]) -> Material {
    if rgba[3] == 0 {
        return Material::Transparent;
    }
    let rgb = [rgba[0], rgba[1], rgba[2]];
    PALETTE
        .iter()
        .find(|(c, _)| *c == rgb)
        .map_or(Material::Other, |(_, m)| *m)
}

/// Material of a palette key of the `.txt` grids (`.` is transparent).
pub fn material_of_key(key: char) -> Material {
    match key {
        '.' => Material::Transparent,
        'K' => Material::Outline,
        'W' | 'w' | 'h' | 'H' => Material::Hair,
        'S' | 's' | 'z' => Material::Skin,
        'e' => Material::Eye,
        'G' | 'g' | 'q' => Material::Shirt,
        'P' | 'p' | 'o' => Material::Trousers,
        'L' | 'l' | 'd' => Material::Leather,
        'M' => Material::Metal,
        _ => Material::Other,
    }
}

/// A slice of the approved body.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BodyLayer {
    FarArm,
    Legs,
    Torso,
    Hair,
    NearArm,
}

impl BodyLayer {
    pub const ALL: [Self; 5] = [
        Self::FarArm,
        Self::Legs,
        Self::Torso,
        Self::Hair,
        Self::NearArm,
    ];

    pub const fn depth(self) -> DepthSlot {
        match self {
            Self::FarArm => DepthSlot::FarArm,
            Self::Legs => DepthSlot::Legs,
            Self::Torso => DepthSlot::Torso,
            Self::Hair => DepthSlot::Hair,
            Self::NearArm => DepthSlot::NearArm,
        }
    }

    /// Whether an idle breath lowers this layer (the legs stay planted).
    pub const fn is_upper_body(self) -> bool {
        !matches!(self, Self::Legs)
    }

    /// Short name for files and overlays.
    pub const fn key(self) -> &'static str {
        match self {
            Self::FarArm => "far_arm",
            Self::Legs => "legs",
            Self::Torso => "torso",
            Self::Hair => "hair",
            Self::NearArm => "near_arm",
        }
    }
}

/// Draw order of the whole character, back to front: body layers with the
/// equipment slots between them. Mirroring keeps the order (the sprite is
/// flipped, so the near arm stays towards the camera).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub enum DepthSlot {
    /// Capes and anything behind the whole body.
    BackGear,
    FarArm,
    /// A shield or second weapon in the far hand.
    FarHandGear,
    Legs,
    Torso,
    Hair,
    /// The weapon in the near hand: over the body and the hair, under the
    /// fist that grips it.
    MainHandGear,
    NearArm,
    /// Pauldron of the near shoulder: over the upper arm.
    ShoulderGear,
}

impl DepthSlot {
    /// Position in the draw order (0 = furthest back).
    pub const fn order(self) -> u8 {
        self as u8
    }
}

/// Capsule radii (texels) of the arm partition.
const UPPER_ARM_RADIUS: f32 = 4.5;
const FOREARM_RADIUS: f32 = 4.0;
const FIST_RADIUS: f32 = 4.0;
/// Hair-coloured texels this far behind the spine line are hair.
const HAIR_BEHIND_SPINE: f32 = 3.0;
/// Hair-coloured texels above the shoulders minus this are hair.
const HAIR_ABOVE_SHOULDER: f32 = 1.0;

/// A frame's grid geometry: cell size and sole pivot (texels from the
/// top-left corner).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellGeometry {
    pub width: usize,
    pub height: usize,
    pub pivot: (f32, f32),
}

impl CellGeometry {
    /// Rig-space centre of the texel at column `x`, row `y`.
    pub fn texel_centre(&self, x: usize, y: usize) -> RigPoint {
        (
            x as f32 + 0.5 - self.pivot.0,
            self.pivot.1 - (y as f32 + 0.5),
        )
    }

    /// Rig space -> texel coordinates (y down) of a cell point.
    pub fn to_texel(&self, p: RigPoint) -> (f32, f32) {
        (p.0 + self.pivot.0, self.pivot.1 - p.1)
    }

    /// Texel coordinates (y down) -> rig space.
    pub fn to_rig(&self, t: (f32, f32)) -> RigPoint {
        (t.0 - self.pivot.0, self.pivot.1 - t.1)
    }
}

fn arm_material(m: Material) -> bool {
    matches!(
        m,
        Material::Skin | Material::Shirt | Material::Leather | Material::Outline
    )
}

fn in_arm(p: RigPoint, joints: &JointPositions, upper: Bone, fore: Bone, fist: Joint) -> bool {
    let (s, e) = joints.segment(upper);
    let (_, h) = joints.segment(fore);
    distance_to_segment(p, s, e) <= UPPER_ARM_RADIUS
        || distance_to_segment(p, e, h) <= FOREARM_RADIUS
        || distance(p, joints.get(fist)) <= FIST_RADIUS
}

/// x of the spine line (pelvis -> neck) at height `y`.
fn spine_x(joints: &JointPositions, y: f32) -> f32 {
    let (p, n) = (joints.get(Joint::Pelvis), joints.get(Joint::Neck));
    if (n.1 - p.1).abs() < f32::EPSILON {
        return p.0;
    }
    let t = (y - p.1) / (n.1 - p.1);
    p.0 + t * (n.0 - p.0)
}

/// Which layer one texel belongs to (`None` for transparent texels).
pub fn layer_of(material: Material, p: RigPoint, joints: &JointPositions) -> Option<BodyLayer> {
    if material == Material::Transparent {
        return None;
    }
    if arm_material(material)
        && in_arm(
            p,
            joints,
            Bone::UpperArmNear,
            Bone::ForearmNear,
            Joint::HandNear,
        )
    {
        return Some(BodyLayer::NearArm);
    }
    if material == Material::Hair {
        let shoulder = joints.get(Joint::ShoulderNear).1;
        let behind = p.0 < spine_x(joints, p.1) - HAIR_BEHIND_SPINE;
        if behind || p.1 > shoulder - HAIR_ABOVE_SHOULDER {
            return Some(BodyLayer::Hair);
        }
    }
    if arm_material(material)
        && in_arm(
            p,
            joints,
            Bone::UpperArmFar,
            Bone::ForearmFar,
            Joint::HandFar,
        )
    {
        return Some(BodyLayer::FarArm);
    }
    if p.1 >= joints.get(Joint::Pelvis).1 {
        Some(BodyLayer::Torso)
    } else {
        Some(BodyLayer::Legs)
    }
}

/// One layer (or `None`) per texel of a frame, row-major.
pub fn partition(
    materials: &[Material],
    cell: CellGeometry,
    joints: &JointPositions,
) -> Vec<Option<BodyLayer>> {
    materials
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let p = cell.texel_centre(i % cell.width, i / cell.width);
            layer_of(*m, p, joints)
        })
        .collect()
}

/// The near chain whose pieces the articulated comparison cuts.
pub const CUTOUT_BONES: [Bone; 7] = [
    Bone::Spine,
    Bone::Head,
    Bone::UpperArmNear,
    Bone::ForearmNear,
    Bone::ThighA,
    Bone::ShinA,
    Bone::FootA,
];

/// One bone (or `None`) per texel: the nearest bone of [`CUTOUT_BONES`].
/// The head piece also takes every hair texel above the neck.
pub fn partition_by_bone(
    materials: &[Material],
    cell: CellGeometry,
    joints: &JointPositions,
) -> Vec<Option<Bone>> {
    let neck_y = joints.get(Joint::Neck).1;
    materials
        .iter()
        .enumerate()
        .map(|(i, m)| {
            if *m == Material::Transparent {
                return None;
            }
            let p = cell.texel_centre(i % cell.width, i / cell.width);
            if *m == Material::Hair && p.1 >= neck_y - HAIR_ABOVE_SHOULDER * 6.0 {
                return Some(Bone::Head);
            }
            CUTOUT_BONES
                .iter()
                .map(|b| {
                    let (a, e) = joints.segment(*b);
                    (*b, distance_to_segment(p, a, e))
                })
                .min_by(|a, b| a.1.total_cmp(&b.1))
                .map(|(b, _)| b)
        })
        .collect()
}

/// The texels of column `column` of a horizontal strip (RGBA8, row-major,
/// `strip_width` texels wide). `None` when the strip is too small.
pub fn extract_cell(
    rgba: &[u8],
    strip_width: usize,
    column: usize,
    cell: CellGeometry,
) -> Option<Vec<[u8; 4]>> {
    let x0 = column * cell.width;
    if x0 + cell.width > strip_width || rgba.len() < strip_width * cell.height * 4 {
        return None;
    }
    let mut out = Vec::with_capacity(cell.width * cell.height);
    for y in 0..cell.height {
        for x in x0..x0 + cell.width {
            let i = (y * strip_width + x) * 4;
            out.push([rgba[i], rgba[i + 1], rgba[i + 2], rgba[i + 3]]);
        }
    }
    Some(out)
}

/// RGBA8 bytes of the texels labelled `keep` (every other texel cleared).
pub fn masked_rgba<T: PartialEq + Copy>(
    texels: &[[u8; 4]],
    labels: &[Option<T>],
    keep: T,
) -> Vec<u8> {
    texels
        .iter()
        .zip(labels)
        .flat_map(|(t, l)| if *l == Some(keep) { *t } else { [0; 4] })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::super::skeleton::tests::standing;
    use super::*;

    const CELL: CellGeometry = CellGeometry {
        width: 80,
        height: 128,
        pivot: (40.0, 118.0),
    };

    #[test]
    fn palette_colours_map_to_their_material() {
        assert_eq!(material_of([238, 238, 244, 255]), Material::Hair);
        assert_eq!(material_of([236, 200, 172, 255]), Material::Skin);
        assert_eq!(material_of([1, 2, 3, 255]), Material::Other);
        assert_eq!(material_of([238, 238, 244, 0]), Material::Transparent);
        assert_eq!(material_of([0, 0, 0, 255]), Material::Other);
        assert_eq!(material_of_key('q'), Material::Shirt);
        assert_eq!(material_of_key('.'), Material::Transparent);
    }

    #[test]
    fn texel_and_rig_coordinates_round_trip() {
        let p = CELL.texel_centre(40, 117);
        assert_eq!(p, (0.5, 0.5));
        assert_eq!(CELL.to_rig(CELL.to_texel((3.0, 7.0))), (3.0, 7.0));
    }

    #[test]
    fn skin_at_the_near_fist_is_near_arm_and_trousers_never_are() {
        let j = standing();
        let fist = j.get(Joint::HandNear);
        assert_eq!(layer_of(Material::Skin, fist, &j), Some(BodyLayer::NearArm));
        assert_eq!(
            layer_of(Material::Trousers, fist, &j),
            Some(BodyLayer::Legs)
        );
        assert_eq!(layer_of(Material::Transparent, fist, &j), None);
    }

    #[test]
    fn hair_behind_the_back_or_above_the_shoulders_is_hair() {
        let j = standing();
        assert_eq!(
            layer_of(Material::Hair, (-10.0, 70.0), &j),
            Some(BodyLayer::Hair)
        );
        assert_eq!(
            layer_of(Material::Hair, (0.0, 95.0), &j),
            Some(BodyLayer::Hair)
        );
        // Hair-coloured shading in front of the spine, below the shoulders,
        // stays with the shirt.
        assert_eq!(
            layer_of(Material::Hair, (3.0, 70.0), &j),
            Some(BodyLayer::Torso)
        );
    }

    #[test]
    fn partition_covers_exactly_the_opaque_texels() {
        let j = standing();
        let mut materials = vec![Material::Transparent; CELL.width * CELL.height];
        for (i, m) in materials.iter_mut().enumerate() {
            if i % 3 == 0 {
                *m = Material::Shirt;
            }
        }
        let layers = partition(&materials, CELL, &j);
        for (m, l) in materials.iter().zip(&layers) {
            assert_eq!(*m == Material::Transparent, l.is_none());
        }
        let bones = partition_by_bone(&materials, CELL, &j);
        for (m, b) in materials.iter().zip(&bones) {
            assert_eq!(*m == Material::Transparent, b.is_none());
        }
    }

    #[test]
    fn cells_are_cut_from_the_strip_and_masks_keep_only_their_label() {
        let cell = CellGeometry {
            width: 2,
            height: 1,
            pivot: (1.0, 1.0),
        };
        // A 4x1 strip: two cells.
        let strip: Vec<u8> = (0..16).collect();
        let c1 = extract_cell(&strip, 4, 1, cell).unwrap_or_else(|| panic!("cell"));
        assert_eq!(c1, vec![[8, 9, 10, 11], [12, 13, 14, 15]]);
        assert!(extract_cell(&strip, 4, 2, cell).is_none());
        let labels = [Some(BodyLayer::Hair), Some(BodyLayer::Torso)];
        assert_eq!(
            masked_rgba(&c1, &labels, BodyLayer::Torso),
            vec![0, 0, 0, 0, 12, 13, 14, 15]
        );
    }

    #[test]
    fn the_draw_order_puts_the_weapon_under_the_fist_and_the_pauldron_over_it() {
        assert!(DepthSlot::Hair < DepthSlot::MainHandGear);
        assert!(DepthSlot::MainHandGear < DepthSlot::NearArm);
        assert!(DepthSlot::NearArm < DepthSlot::ShoulderGear);
        assert!(DepthSlot::BackGear < DepthSlot::FarArm);
        assert!(DepthSlot::FarHandGear < DepthSlot::Torso);
        for l in BodyLayer::ALL {
            assert!(l.depth() != DepthSlot::MainHandGear);
        }
    }
}
