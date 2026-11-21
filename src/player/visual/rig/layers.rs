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

    /// One-character code of the label maps (`stage/layers/*.txt`).
    pub const fn code(self) -> char {
        match self {
            Self::FarArm => 'F',
            Self::Legs => 'L',
            Self::Torso => 'T',
            Self::Hair => 'H',
            Self::NearArm => 'N',
        }
    }

    pub fn from_code(c: char) -> Option<Self> {
        Self::ALL.into_iter().find(|l| l.code() == c)
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
/// flipped, so the near arm stays towards the camera). A weapon may pick
/// another slot per frame (`grip::GripProfile::depth`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub enum DepthSlot {
    /// Capes and anything behind the whole body.
    BackGear,
    /// A weapon passing behind the body (02).
    MainHandBehind,
    FarArm,
    /// A shield or second weapon in the far hand.
    FarHandGear,
    Legs,
    /// Greaves and trousers armour over the legs.
    LegGear,
    BootGear,
    Torso,
    /// Breastplate: over the shirt, under the hair and the near arm.
    ChestGear,
    BeltGear,
    Hair,
    HelmetGear,
    /// The weapon in the near hand: over the body and the hair, under the
    /// fist that grips it.
    MainHandGear,
    NearArm,
    /// Bracer of the near forearm.
    BracerGear,
    /// Pauldron of the near shoulder: over the upper arm.
    ShoulderGear,
}

impl DepthSlot {
    /// Position in the draw order (0 = furthest back).
    pub const fn order(self) -> u8 {
        self as u8
    }
}

/// Capsule radii (texels) of the arm partition (02: tighter inside the
/// torso box, where the sleeve lies over the shirt of the torso).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArmRadii {
    pub upper_arm: f32,
    pub forearm: f32,
    pub fist: f32,
    pub sleeve_over_torso: f32,
    pub skin_over_torso: f32,
}

impl Default for ArmRadii {
    fn default() -> Self {
        Self {
            upper_arm: 4.5,
            forearm: 4.0,
            fist: 4.0,
            sleeve_over_torso: 4.0,
            skin_over_torso: 3.5,
        }
    }
}
/// Hair-coloured texels above the shoulders minus this are hair.
const HAIR_ABOVE_SHOULDER: f32 = 1.0;
/// Head circle: hair inside it is hair, anything else is the face.
const HEAD_RADIUS: f32 = 7.5;

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

/// Distance from `p` to the arm chain (upper arm, forearm, fist).
fn arm_distance(
    p: RigPoint,
    joints: &JointPositions,
    upper: Bone,
    fore: Bone,
    fist: Joint,
) -> [f32; 3] {
    let (s, e) = joints.segment(upper);
    let (_, h) = joints.segment(fore);
    [
        distance_to_segment(p, s, e),
        distance_to_segment(p, e, h),
        distance(p, joints.get(fist)),
    ]
}

/// The torso box (chest back, chest front, waist front, waist back).
pub fn torso_quad(joints: &JointPositions) -> [RigPoint; 4] {
    [
        joints.get(Joint::ChestBack),
        joints.get(Joint::ChestFront),
        joints.get(Joint::WaistFront),
        joints.get(Joint::WaistBack),
    ]
}

/// Even-odd point-in-polygon test.
pub fn point_in_polygon(p: RigPoint, polygon: &[RigPoint]) -> bool {
    let mut inside = false;
    let n = polygon.len();
    for i in 0..n {
        let (a, b) = (polygon[i], polygon[(i + n - 1) % n]);
        if (a.1 > p.1) != (b.1 > p.1) {
            let x = a.0 + (p.1 - a.1) / (b.1 - a.1) * (b.0 - a.0);
            if p.0 < x {
                inside = !inside;
            }
        }
    }
    inside
}

/// Which layer one texel belongs to (`None` for transparent texels).
///
/// 02 rules (the N4-C `classify` order, adapted): the near arm is drawn
/// over the torso, but inside the torso box only texels close to the arm
/// bones belong to it; hair is hair-coloured texels inside the head circle,
/// outside the torso box or above the shoulders; the far arm only shows
/// outside the torso box.
pub fn layer_of(material: Material, p: RigPoint, joints: &JointPositions) -> Option<BodyLayer> {
    layer_with(material, p, joints, ArmRadii::default())
}

/// [`layer_of`] with explicit radii.
pub fn layer_with(
    material: Material,
    p: RigPoint,
    joints: &JointPositions,
    r: ArmRadii,
) -> Option<BodyLayer> {
    if material == Material::Transparent {
        return None;
    }
    let quad = torso_quad(joints);
    let in_torso = point_in_polygon(p, &quad);
    let below_shoulders = p.1 < joints.get(Joint::ShoulderNear).1;
    // The shirt's shading reuses the cool hair greys: below the shoulders
    // they count as sleeve for the near arm.
    let sleeve_shade = material == Material::Hair && below_shoulders;
    // The sleeve's shadows also reuse the trousers browns: inside the upper
    // arm capsule they are sleeve too (not near the fist, which can lie over
    // the real trousers).
    let sleeve_brown = material == Material::Trousers && below_shoulders;
    if arm_material(material) || sleeve_shade || sleeve_brown {
        let [upper, fore, fist] = arm_distance(
            p,
            joints,
            Bone::UpperArmNear,
            Bone::ForearmNear,
            Joint::HandNear,
        );
        let near = if sleeve_brown {
            upper
                <= if in_torso {
                    r.sleeve_over_torso
                } else {
                    r.upper_arm
                }
        } else if in_torso {
            let radius = if matches!(material, Material::Shirt | Material::Hair) {
                r.sleeve_over_torso
            } else {
                r.skin_over_torso
            };
            upper.min(fore).min(fist) <= radius
        } else {
            upper <= r.upper_arm || fore <= r.forearm || fist <= r.fist
        };
        if near {
            return Some(BodyLayer::NearArm);
        }
    }
    let head = joints.get(Joint::Head);
    let in_head = distance(p, head) <= HEAD_RADIUS;
    let shoulder = joints.get(Joint::ShoulderNear).1;
    let above_shoulders = p.1 > shoulder - HAIR_ABOVE_SHOULDER;
    let hair = match material {
        Material::Hair => in_head || !in_torso || above_shoulders,
        // N4-C: the ink behind the face belongs to the hair.
        Material::Outline => in_head && p.0 < head.0,
        Material::Skin | Material::Eye => false,
        // Anything else outside the torso and above the shoulders is
        // hair (its shading and ink), unless it is the near arm (above).
        _ => !in_torso && above_shoulders && !in_head,
    };
    if hair {
        return Some(BodyLayer::Hair);
    }
    if arm_material(material) && !in_torso {
        let [upper, fore, fist] = arm_distance(
            p,
            joints,
            Bone::UpperArmFar,
            Bone::ForearmFar,
            Joint::HandFar,
        );
        if upper <= r.upper_arm || fore <= r.forearm || fist <= r.fist {
            return Some(BodyLayer::FarArm);
        }
    }
    // N4-C: below the waist (the belt) is trousers, i.e. legs.
    let (wb, wf) = (joints.get(Joint::WaistBack), joints.get(Joint::WaistFront));
    let waist_y = if (wf.0 - wb.0).abs() > f32::EPSILON {
        wb.1 + (p.0 - wb.0) / (wf.0 - wb.0) * (wf.1 - wb.1)
    } else {
        wb.1
    };
    if p.1 < waist_y {
        Some(BodyLayer::Legs)
    } else {
        Some(BodyLayer::Torso)
    }
}

/// One layer (or `None`) per texel of a frame, row-major.
pub fn partition(
    materials: &[Material],
    cell: CellGeometry,
    joints: &JointPositions,
) -> Vec<Option<BodyLayer>> {
    partition_with(materials, cell, joints, ArmRadii::default())
}

/// [`partition`] with explicit radii.
pub fn partition_with(
    materials: &[Material],
    cell: CellGeometry,
    joints: &JointPositions,
    radii: ArmRadii,
) -> Vec<Option<BodyLayer>> {
    materials
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let p = cell.texel_centre(i % cell.width, i / cell.width);
            layer_with(*m, p, joints, radii)
        })
        .collect()
}

/// A label map that does not fit its frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LabelError {
    /// Wrong number of rows or columns.
    Size,
    /// A character that is neither `.` nor a layer code.
    Code(char),
}

/// Parses a label map: one row per line, `.` transparent, a layer code
/// otherwise (authored offline, e.g. from the N4-C layers).
pub fn parse_labels(text: &str, cell: CellGeometry) -> Result<Vec<Option<BodyLayer>>, LabelError> {
    let rows: Vec<&str> = text.lines().filter(|l| !l.is_empty()).collect();
    if rows.len() != cell.height || rows.iter().any(|r| r.chars().count() != cell.width) {
        return Err(LabelError::Size);
    }
    rows.iter()
        .flat_map(|r| r.chars())
        .map(|c| match c {
            '.' => Ok(None),
            c => BodyLayer::from_code(c).map(Some).ok_or(LabelError::Code(c)),
        })
        .collect()
}

/// Texels where a label map disagrees with the art's opacity (a label on a
/// transparent texel or an opaque texel without label).
pub fn label_mismatches(materials: &[Material], labels: &[Option<BodyLayer>]) -> usize {
    materials
        .iter()
        .zip(labels)
        .filter(|(m, l)| (**m == Material::Transparent) != l.is_none())
        .count()
}

/// Intersection over union of one layer in two labelings (1 when both are
/// empty).
pub fn layer_iou(a: &[Option<BodyLayer>], b: &[Option<BodyLayer>], layer: BodyLayer) -> f32 {
    let (mut inter, mut union) = (0usize, 0usize);
    for (x, y) in a.iter().zip(b) {
        let (ix, iy) = (*x == Some(layer), *y == Some(layer));
        inter += usize::from(ix && iy);
        union += usize::from(ix || iy);
    }
    if union == 0 {
        1.0
    } else {
        inter as f32 / union as f32
    }
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
    fn hair_outside_the_torso_box_or_above_the_shoulders_is_hair() {
        let j = standing();
        assert_eq!(
            layer_of(Material::Hair, (-10.0, 70.0), &j),
            Some(BodyLayer::Hair)
        );
        assert_eq!(
            layer_of(Material::Hair, (0.0, 95.0), &j),
            Some(BodyLayer::Hair)
        );
        // Hair-coloured shading inside the torso box, below the shoulders:
        // the sleeve's shading when it is close to the near arm, the shirt's
        // otherwise.
        assert_eq!(
            layer_of(Material::Hair, (0.0, 76.0), &j),
            Some(BodyLayer::NearArm)
        );
        assert_eq!(
            layer_of(Material::Hair, (-5.0, 76.0), &j),
            Some(BodyLayer::Torso)
        );
    }

    #[test]
    fn inside_the_torso_box_only_texels_close_to_the_arm_are_near_arm() {
        let j = standing();
        // The upper arm runs from (-1, 85) to (0, 69): x = 3.8 is within the
        // old 4.5 capsule but beyond the 4.0 sleeve radius over the torso.
        assert_eq!(
            layer_of(Material::Shirt, (3.8, 77.0), &j),
            Some(BodyLayer::Torso)
        );
        assert_eq!(
            layer_of(Material::Shirt, (1.5, 77.0), &j),
            Some(BodyLayer::NearArm)
        );
        // The far arm never shows through the torso box.
        let far = j.get(Joint::ElbowFar);
        assert!(point_in_polygon(far, &torso_quad(&j)) || far.0 < -6.0);
    }

    #[test]
    fn label_maps_parse_and_are_checked_against_the_art() {
        let cell = CellGeometry {
            width: 3,
            height: 2,
            pivot: (1.0, 2.0),
        };
        let labels = parse_labels("N.T\nHHF\n", cell).unwrap_or_else(|e| panic!("{e:?}"));
        assert_eq!(labels[0], Some(BodyLayer::NearArm));
        assert_eq!(labels[1], None);
        assert_eq!(parse_labels("N.T\n", cell), Err(LabelError::Size));
        assert_eq!(parse_labels("N.X\nHHF\n", cell), Err(LabelError::Code('X')));
        let materials = [
            Material::Skin,
            Material::Transparent,
            Material::Shirt,
            Material::Hair,
            Material::Hair,
            Material::Transparent,
        ];
        assert_eq!(label_mismatches(&materials, &labels), 1);
        assert!((layer_iou(&labels, &labels, BodyLayer::Hair) - 1.0).abs() < 1e-6);
        let mut other = labels.clone();
        other[3] = Some(BodyLayer::Torso);
        assert!((layer_iou(&labels, &other, BodyLayer::Hair) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn polygon_test_is_even_odd() {
        let square = [(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)];
        assert!(point_in_polygon((2.0, 2.0), &square));
        assert!(!point_in_polygon((5.0, 2.0), &square));
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
        assert!(DepthSlot::Torso < DepthSlot::ChestGear);
        assert!(DepthSlot::ChestGear < DepthSlot::Hair);
        assert!(DepthSlot::MainHandBehind < DepthSlot::Legs);
        for l in BodyLayer::ALL {
            assert!(l.depth() != DepthSlot::MainHandGear);
        }
    }
}
