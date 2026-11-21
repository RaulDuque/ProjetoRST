//! Equipment Studio: the generic, data-driven equipment model (Modular
//! Character Engine 03). Pure module: no Bevy types.
//!
//! An [`EquipmentDesign`] is a set of independent [`Part`]s drawn in one
//! design space:
//!
//! - [`DesignSpace::Torso`]: coordinates are torso-cage `(u, v)` (u from the
//!   back to the front, v from the shoulder line to the waist), so the item
//!   follows the torso of every pose (the projection approved in 02);
//! - [`DesignSpace::Hand`] / [`DesignSpace::Bone`]: coordinates are texels
//!   in the frame of a hand (grip profile) or a bone (`+x` along it).
//!
//! A part has its own geometry (a polygon, optionally rounded into a curve,
//! or a PNG image), transform, pivot, material, depth, outline, inner
//! regions, ornaments, visibility rules and perspective range. The engine
//! never knows what a "breastplate" is: `paint` interprets any design.
//! Designs are saved as RON, one file per item, independent of the
//! character.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::equipment::{BoneEnd, RigSlot};
use super::grip::HandSide;
use super::layers::{BodyRegion, DepthSlot};
use super::skeleton::Bone;

/// Only version understood.
pub const DESIGN_VERSION: u32 = 1;

/// Where the design's coordinates live.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DesignSpace {
    Torso,
    Hand {
        side: HandSide,
    },
    Bone {
        bone: Bone,
        at: BoneEnd,
        follow: f32,
    },
}

/// How a material is shaded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Finish {
    /// Hard tone bands and a specular highlight on lit bevels.
    Metal,
    /// Ordered dithering between tones (soft, grained).
    Leather,
    /// Soft bands, no specular.
    Cloth,
    /// One tone (trims, gems), bevel only.
    Flat,
}

/// A named colour ramp with its finish. Ramp: outline, deep, shadow, base,
/// light, highlight.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaintMaterial {
    pub id: String,
    pub ramp: [[u8; 3]; 6],
    pub finish: Finish,
}

/// Ramp indices.
pub const INK: usize = 0;
pub const DEEP: usize = 1;
pub const SHADOW: usize = 2;
pub const BASE: usize = 3;
pub const LIGHT: usize = 4;
pub const HIGHLIGHT: usize = 5;

/// The geometry of a part.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PartSource {
    /// A polygon in part-local coordinates; `smooth` Chaikin passes turn it
    /// into a curve.
    Polygon {
        points: Vec<(f32, f32)>,
        #[serde(default)]
        smooth: u8,
    },
    /// A PNG drawn with its own pixels. `path` is relative to the studio
    /// folder; `pivot` is the image texel placed on the part position. In
    /// the torso space an image is a decal: pixel-aligned, never stretched.
    Image { path: String, pivot: (f32, f32) },
    /// No geometry: a group that only carries a transform, visibility and
    /// depth for its children (04).
    Group,
    /// Pixels painted in the studio (04): one character per texel, `.`
    /// transparent, otherwise an index into `palette` (`0`-`9`, `a`-`z`),
    /// PixelSpec-like rows. Drawn like an image.
    Pixels {
        size: (u32, u32),
        pivot: (f32, f32),
        palette: Vec<[u8; 4]>,
        rows: Vec<String>,
    },
}

/// Palette index of a pixel-grid character (`None` for `.`).
pub fn pixel_index(c: char) -> Option<usize> {
    match c {
        '0'..='9' => Some(c as usize - '0' as usize),
        'a'..='z' => Some(10 + c as usize - 'a' as usize),
        _ => None,
    }
}

/// The character of a palette index (`.` beyond 36 entries).
pub fn pixel_char(i: usize) -> char {
    match i {
        0..=9 => (b'0' + i as u8) as char,
        10..=35 => (b'a' + (i - 10) as u8) as char,
        _ => '.',
    }
}

/// A 2D affine map (`x' = a x + c y + tx`, `y' = b x + d y + ty`): the
/// composition of part transforms down the hierarchy.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Affine {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub tx: f32,
    pub ty: f32,
}

impl Affine {
    pub const IDENTITY: Self = Self {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        tx: 0.0,
        ty: 0.0,
    };

    pub fn apply(&self, p: (f32, f32)) -> (f32, f32) {
        (
            self.a * p.0 + self.c * p.1 + self.tx,
            self.b * p.0 + self.d * p.1 + self.ty,
        )
    }

    /// `self ∘ other` (apply `other` first).
    pub fn then(&self, other: &Self) -> Self {
        Self {
            a: self.a * other.a + self.c * other.b,
            b: self.b * other.a + self.d * other.b,
            c: self.a * other.c + self.c * other.d,
            d: self.b * other.c + self.d * other.d,
            tx: self.a * other.tx + self.c * other.ty + self.tx,
            ty: self.b * other.tx + self.d * other.ty + self.ty,
        }
    }

    pub fn inverse(&self) -> Option<Self> {
        let det = self.a * self.d - self.b * self.c;
        if det.abs() < 1e-9 {
            return None;
        }
        let (a, b, c, d) = (self.d / det, -self.b / det, -self.c / det, self.a / det);
        Some(Self {
            a,
            b,
            c,
            d,
            tx: -(a * self.tx + c * self.ty),
            ty: -(b * self.tx + d * self.ty),
        })
    }
}

/// Local transform of a part (or of the whole design for the mount).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PartTransform {
    pub position: (f32, f32),
    pub rotation_deg: f32,
    pub scale: (f32, f32),
    /// Point of the local geometry that rotation and scale turn around.
    pub pivot: (f32, f32),
}

impl Default for PartTransform {
    fn default() -> Self {
        Self {
            position: (0.0, 0.0),
            rotation_deg: 0.0,
            scale: (1.0, 1.0),
            pivot: (0.0, 0.0),
        }
    }
}

impl PartTransform {
    /// The transform as an affine map (local -> parent).
    pub fn affine(&self) -> Affine {
        let (sn, cs) = self.rotation_deg.to_radians().sin_cos();
        let (a, b, c, d) = (
            cs * self.scale.0,
            sn * self.scale.0,
            -sn * self.scale.1,
            cs * self.scale.1,
        );
        Affine {
            a,
            b,
            c,
            d,
            tx: self.position.0 - (a * self.pivot.0 + c * self.pivot.1),
            ty: self.position.1 - (b * self.pivot.0 + d * self.pivot.1),
        }
    }

    /// The transform (keeping `pivot`) closest to an affine map: position,
    /// rotation and per-axis scale; a shear (non-uniform parent scale under
    /// rotation) is dropped.
    pub fn from_affine(m: &Affine, pivot: (f32, f32)) -> Self {
        let sx = m.a.hypot(m.b);
        let rot = m.b.atan2(m.a);
        let det = m.a * m.d - m.b * m.c;
        let sy = if sx > 1e-9 { det / sx } else { m.c.hypot(m.d) };
        Self {
            position: m.apply(pivot),
            rotation_deg: rot.to_degrees(),
            scale: (sx, sy),
            pivot,
        }
    }

    /// Local geometry point -> design space.
    pub fn apply(&self, p: (f32, f32)) -> (f32, f32) {
        let (s, c) = self.rotation_deg.to_radians().sin_cos();
        let x = (p.0 - self.pivot.0) * self.scale.0;
        let y = (p.1 - self.pivot.1) * self.scale.1;
        (
            self.position.0 + c * x - s * y,
            self.position.1 + s * x + c * y,
        )
    }

    /// Design space -> local geometry (`None` for a zero scale).
    pub fn invert(&self, d: (f32, f32)) -> Option<(f32, f32)> {
        if self.scale.0.abs() < 1e-6 || self.scale.1.abs() < 1e-6 {
            return None;
        }
        let (s, c) = self.rotation_deg.to_radians().sin_cos();
        let (dx, dy) = (d.0 - self.position.0, d.1 - self.position.1);
        let x = c * dx + s * dy;
        let y = -s * dx + c * dy;
        Some((
            x / self.scale.0 + self.pivot.0,
            y / self.scale.1 + self.pivot.1,
        ))
    }
}

/// How a part's edge is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outline {
    None,
    /// Ink on the silhouette, a darker seam where it overlaps a lower part.
    Selective,
    /// Ink on every edge, including seams.
    Ink,
}

/// An inner area of a part: another material or a tone shift.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Region {
    pub points: Vec<(f32, f32)>,
    #[serde(default)]
    pub material: Option<String>,
    /// Ramp steps added to the shaded tone (negative = darker).
    #[serde(default)]
    pub tone: i8,
}

/// A small detail drawn on a part, in part-local coordinates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Ornament {
    /// One lit texel with a shadow texel under it.
    Rivet { at: (f32, f32) },
    /// A line of one ramp tone (dashed = every other texel: stitches).
    Line {
        points: Vec<(f32, f32)>,
        tone: u8,
        #[serde(default)]
        dashed: bool,
    },
}

/// When a part is drawn.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Visibility {
    /// Clip keys (empty = every clip).
    #[serde(default)]
    pub clips: Vec<String>,
    /// Torso turn range `[min, max]` (0 side view, 1 three-quarter): a
    /// perspective variant of the same design.
    #[serde(default)]
    pub turn: Option<(f32, f32)>,
}

impl Visibility {
    pub fn allows(&self, clip: &str, turn: f32) -> bool {
        (self.clips.is_empty() || self.clips.iter().any(|c| c == clip))
            && self.turn.is_none_or(|(a, b)| turn >= a && turn <= b)
    }
}

/// One independent part.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Part {
    pub id: String,
    /// Parent part id (04): the part's transform is relative to its
    /// parent's, and it is hidden when the parent is.
    #[serde(default)]
    pub parent: Option<String>,
    /// Binding of a root part to another space than the design's (04): e.g.
    /// the far bracer of a pair on the far forearm.
    #[serde(default)]
    pub bind: Option<DesignSpace>,
    /// Depth of the part (and of its children) instead of the design's.
    #[serde(default)]
    pub depth: Option<DepthSlot>,
    /// Body regions hidden under this part's footprint (dilated one texel):
    /// e.g. the sleeve under a pauldron.
    #[serde(default)]
    pub hides: Vec<BodyRegion>,
    pub source: PartSource,
    #[serde(default)]
    pub transform: PartTransform,
    pub material: String,
    /// Draw order inside the item (higher = in front).
    pub layer: i32,
    pub outline: Outline,
    /// Bevel width in texels (the lit / shaded rim).
    #[serde(default)]
    pub bevel: f32,
    #[serde(default)]
    pub regions: Vec<Region>,
    #[serde(default)]
    pub ornaments: Vec<Ornament>,
    #[serde(default = "yes")]
    pub visible: bool,
    #[serde(default)]
    pub visibility: Visibility,
}

fn yes() -> bool {
    true
}

/// A complete item, stored in its own RON file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquipmentDesign {
    pub version: u32,
    pub id: String,
    pub name: String,
    pub slot: RigSlot,
    pub space: DesignSpace,
    pub depth: DepthSlot,
    /// Adjusts the whole design on its mount (the "socket" edit).
    #[serde(default)]
    pub mount: PartTransform,
    /// Light direction in the design's screen terms (x forward, y up).
    #[serde(default = "default_light")]
    pub light: (f32, f32),
    /// Torso cage aspect of the side and three-quarter views (turn 0..1).
    #[serde(default = "default_aspect")]
    pub aspect_range: (f32, f32),
    pub materials: Vec<PaintMaterial>,
    pub parts: Vec<Part>,
    #[serde(default)]
    pub provisional: bool,
    #[serde(default)]
    pub source: String,
}

fn default_light() -> (f32, f32) {
    (0.45, 0.8)
}

fn default_aspect() -> (f32, f32) {
    (0.67, 0.78)
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum DesignError {
    #[error("cannot parse the design: {0}")]
    Parse(String),
    #[error("cannot write the design: {0}")]
    Write(String),
    #[error("unsupported design version {0}")]
    Version(u32),
    #[error("part {0}: unknown material {1}")]
    Material(String, String),
    #[error("region of part {0}: unknown material {1}")]
    RegionMaterial(String, String),
    #[error("part {0}: a polygon needs at least 3 points")]
    Polygon(String),
    #[error("part {0}: bad transform (zero scale or not finite)")]
    Transform(String),
    #[error("two parts are called {0}")]
    DuplicatePart(String),
    #[error("two materials are called {0}")]
    DuplicateMaterial(String),
    #[error("the design has no part")]
    Empty,
    #[error("part {0}: unknown parent {1}")]
    Parent(String, String),
    #[error("part {0}: the parent chain loops")]
    Cycle(String),
    #[error("part {0}: only a root part may bind to another space")]
    BindOnChild(String),
    #[error("part {0}: the pixel rows do not match the size or the palette")]
    Pixels(String),
}

impl EquipmentDesign {
    pub fn from_ron(text: &str) -> Result<Self, DesignError> {
        let d: Self = ron::from_str(text).map_err(|e| DesignError::Parse(e.to_string()))?;
        d.validate()?;
        Ok(d)
    }

    pub fn to_ron(&self) -> Result<String, DesignError> {
        let cfg = ron::ser::PrettyConfig::default().struct_names(false);
        ron::ser::to_string_pretty(self, cfg).map_err(|e| DesignError::Write(e.to_string()))
    }

    pub fn material(&self, id: &str) -> Option<&PaintMaterial> {
        self.materials.iter().find(|m| m.id == id)
    }

    pub fn validate(&self) -> Result<(), DesignError> {
        if self.version != DESIGN_VERSION {
            return Err(DesignError::Version(self.version));
        }
        if self.parts.is_empty() {
            return Err(DesignError::Empty);
        }
        for (i, m) in self.materials.iter().enumerate() {
            if self.materials[..i].iter().any(|o| o.id == m.id) {
                return Err(DesignError::DuplicateMaterial(m.id.clone()));
            }
        }
        for (i, p) in self.parts.iter().enumerate() {
            if self.parts[..i].iter().any(|o| o.id == p.id) {
                return Err(DesignError::DuplicatePart(p.id.clone()));
            }
            if self.material(&p.material).is_none() {
                return Err(DesignError::Material(p.id.clone(), p.material.clone()));
            }
            for r in &p.regions {
                if let Some(m) = &r.material
                    && self.material(m).is_none()
                {
                    return Err(DesignError::RegionMaterial(p.id.clone(), m.clone()));
                }
            }
            if let PartSource::Polygon { points, .. } = &p.source
                && points.len() < 3
            {
                return Err(DesignError::Polygon(p.id.clone()));
            }
            if let PartSource::Pixels {
                size,
                palette,
                rows,
                ..
            } = &p.source
            {
                let ok = rows.len() == size.1 as usize
                    && rows.iter().all(|r| r.chars().count() == size.0 as usize)
                    && rows
                        .iter()
                        .flat_map(|r| r.chars())
                        .all(|c| c == '.' || pixel_index(c).is_some_and(|i| i < palette.len()));
                if !ok {
                    return Err(DesignError::Pixels(p.id.clone()));
                }
            }
            if let Some(parent) = &p.parent {
                if !self.parts.iter().any(|o| &o.id == parent) {
                    return Err(DesignError::Parent(p.id.clone(), parent.clone()));
                }
                if p.bind.is_some() {
                    return Err(DesignError::BindOnChild(p.id.clone()));
                }
            }
            let t = &p.transform;
            let finite = [
                t.position.0,
                t.position.1,
                t.rotation_deg,
                t.scale.0,
                t.scale.1,
            ]
            .iter()
            .all(|v| v.is_finite());
            if !finite || t.scale.0.abs() < 1e-6 || t.scale.1.abs() < 1e-6 {
                return Err(DesignError::Transform(p.id.clone()));
            }
        }
        for i in 0..self.parts.len() {
            if self.ancestors(i).is_none() {
                return Err(DesignError::Cycle(self.parts[i].id.clone()));
            }
        }
        Ok(())
    }

    /// Index of a part by id.
    pub fn index_of(&self, id: &str) -> Option<usize> {
        self.parts.iter().position(|p| p.id == id)
    }

    /// Parent index of a part.
    pub fn parent_of(&self, i: usize) -> Option<usize> {
        self.index_of(self.parts.get(i)?.parent.as_deref()?)
    }

    /// The ancestors of a part, nearest first; `None` when the chain loops
    /// or names a missing part.
    pub fn ancestors(&self, i: usize) -> Option<Vec<usize>> {
        let mut out = Vec::new();
        let mut cur = i;
        while let Some(parent) = self.parts.get(cur)?.parent.as_deref() {
            let p = self.index_of(parent)?;
            if p == i || out.contains(&p) {
                return None;
            }
            out.push(p);
            cur = p;
        }
        Some(out)
    }

    /// Direct children of a part (in list order).
    pub fn children(&self, i: usize) -> Vec<usize> {
        let id = &self.parts[i].id;
        (0..self.parts.len())
            .filter(|j| self.parts[*j].parent.as_deref() == Some(id.as_str()))
            .collect()
    }

    /// A part and every descendant (depth first).
    pub fn subtree(&self, i: usize) -> Vec<usize> {
        let mut out = vec![i];
        let mut k = 0;
        while k < out.len() {
            let fresh: Vec<usize> = self
                .children(out[k])
                .into_iter()
                .filter(|j| !out.contains(j))
                .collect();
            out.extend(fresh);
            k += 1;
        }
        out
    }

    /// Local -> design-space map of a part through its parents.
    pub fn world_affine(&self, i: usize) -> Affine {
        let mut m = self.parts[i].transform.affine();
        for a in self.ancestors(i).unwrap_or_default() {
            m = self.parts[a].transform.affine().then(&m);
        }
        m
    }

    /// Visible itself and through every ancestor.
    pub fn effective_visible(&self, i: usize) -> bool {
        self.parts[i].visible
            && self
                .ancestors(i)
                .unwrap_or_default()
                .iter()
                .all(|a| self.parts[*a].visible)
    }

    /// The space of a part: its root's binding or the design's.
    pub fn effective_space(&self, i: usize) -> DesignSpace {
        let root = self
            .ancestors(i)
            .and_then(|a| a.last().copied())
            .unwrap_or(i);
        self.parts[root].bind.unwrap_or(self.space)
    }

    /// The depth of a part: its own, the nearest ancestor's, or `default`.
    pub fn effective_depth(&self, i: usize, default: DepthSlot) -> DepthSlot {
        if let Some(d) = self.parts[i].depth {
            return d;
        }
        self.ancestors(i)
            .unwrap_or_default()
            .iter()
            .find_map(|a| self.parts[*a].depth)
            .unwrap_or(default)
    }

    /// Changes a part's parent (`None` = root). With `keep_global` the part
    /// stays where it is on screen (its local transform is recomputed);
    /// otherwise it keeps its local transform and moves with the new parent.
    /// Refuses a parent inside the part's own subtree.
    pub fn reparent(
        &mut self,
        i: usize,
        parent: Option<usize>,
        keep_global: bool,
    ) -> Result<(), DesignError> {
        if let Some(p) = parent
            && self.subtree(i).contains(&p)
        {
            return Err(DesignError::Cycle(self.parts[i].id.clone()));
        }
        let world = self.world_affine(i);
        self.parts[i].parent = parent.map(|p| self.parts[p].id.clone());
        if parent.is_some() {
            self.parts[i].bind = None;
        }
        if keep_global {
            let base = parent.map_or(Affine::IDENTITY, |p| self.world_affine(p));
            if let Some(inv) = base.inverse() {
                let local = inv.then(&world);
                let pivot = self.parts[i].transform.pivot;
                self.parts[i].transform = PartTransform::from_affine(&local, pivot);
            }
        }
        Ok(())
    }

    /// Duplicates a part and its subtree (new ids, same parent); returns
    /// the copy of the root.
    pub fn duplicate_subtree(&mut self, i: usize) -> usize {
        let nodes = self.subtree(i);
        let mut renamed: Vec<(String, String)> = Vec::new();
        for n in &nodes {
            let old = self.parts[*n].id.clone();
            let mut new = self.free_id(&old);
            while renamed.iter().any(|(_, r)| *r == new) {
                new = format!("{new}_");
            }
            renamed.push((old, new));
        }
        let first = self.parts.len();
        for n in &nodes {
            let mut copy = self.parts[*n].clone();
            let rename = |id: &str| {
                renamed
                    .iter()
                    .find(|(o, _)| o == id)
                    .map(|(_, r)| r.clone())
            };
            copy.id = rename(&copy.id).unwrap_or(copy.id.clone());
            if *n != i {
                copy.parent = copy.parent.as_deref().and_then(rename);
            }
            self.parts.push(copy);
        }
        first
    }

    /// Puts parts under a new group (at their first part's parent); the
    /// parts keep their place on screen. Returns the group's index.
    pub fn group(&mut self, members: &[usize], name: &str) -> usize {
        let parent = members.first().and_then(|m| self.parent_of(*m));
        let id = self.free_id(name);
        let layer = members
            .iter()
            .map(|m| self.parts[*m].layer)
            .max()
            .unwrap_or(0);
        self.parts.push(Part {
            id,
            parent: parent.map(|p| self.parts[p].id.clone()),
            bind: None,
            depth: None,
            hides: Vec::new(),
            source: PartSource::Group,
            transform: PartTransform::default(),
            material: self
                .materials
                .first()
                .map_or_else(String::new, |m| m.id.clone()),
            layer,
            outline: Outline::None,
            bevel: 0.0,
            regions: Vec::new(),
            ornaments: Vec::new(),
            visible: true,
            visibility: Visibility::default(),
        });
        let g = self.parts.len() - 1;
        for m in members {
            if *m != g {
                let _ = self.reparent(*m, Some(g), true);
            }
        }
        g
    }

    /// Removes a part and its subtree.
    pub fn remove_subtree(&mut self, i: usize) {
        let mut nodes = self.subtree(i);
        nodes.sort_unstable_by(|a, b| b.cmp(a));
        for n in nodes {
            self.parts.remove(n);
        }
    }

    /// Turn (0 side, 1 three-quarter) of a torso cage aspect.
    pub fn turn(&self, aspect: f32) -> f32 {
        let (a, b) = self.aspect_range;
        if (b - a).abs() < 1e-6 {
            0.0
        } else {
            ((aspect - a) / (b - a)).clamp(0.0, 1.0)
        }
    }

    /// A free part id (`base`, `base_2`, ...).
    pub fn free_id(&self, base: &str) -> String {
        let mut id = base.to_owned();
        let mut n = 2;
        while self.parts.iter().any(|p| p.id == id) {
            id = format!("{base}_{n}");
            n += 1;
        }
        id
    }
}

/// Chaikin corner cutting: `passes` rounds of a closed polygon.
pub fn chaikin(points: &[(f32, f32)], passes: u8) -> Vec<(f32, f32)> {
    let mut pts = points.to_vec();
    for _ in 0..passes {
        if pts.len() < 3 {
            break;
        }
        let n = pts.len();
        let mut out = Vec::with_capacity(n * 2);
        for i in 0..n {
            let (a, b) = (pts[i], pts[(i + 1) % n]);
            out.push((0.75 * a.0 + 0.25 * b.0, 0.75 * a.1 + 0.25 * b.1));
            out.push((0.25 * a.0 + 0.75 * b.0, 0.25 * a.1 + 0.75 * b.1));
        }
        pts = out;
    }
    pts
}

/// The adapter from the prototype 02 [`super::chest::ChestModel`]: one
/// silhouette part with the collar and lames as regions and the ridge as a
/// line, so the old breastplate is drawn by the generic painter.
pub fn from_chest_model(id: &str, model: &super::chest::ChestModel) -> EquipmentDesign {
    let [ink, deep, shadow, base, light, highlight] = model.ramp;
    let plate = PaintMaterial {
        id: "plate".into(),
        ramp: [ink, deep, shadow, base, light, highlight],
        finish: Finish::Metal,
    };
    let trim = PaintMaterial {
        id: "trim".into(),
        ramp: [
            ink,
            model.trim[0],
            model.trim[0],
            model.trim[1],
            model.trim[1],
            model.trim[1],
        ],
        finish: Finish::Flat,
    };
    let mut regions = vec![Region {
        points: vec![
            (-0.2, -0.2),
            (1.3, -0.2),
            (1.3, model.collar),
            (-0.2, model.collar),
        ],
        material: Some("trim".into()),
        tone: 0,
    }];
    let mut ornaments = Vec::new();
    for b in &model.lames {
        regions.push(Region {
            points: vec![(-0.2, *b), (1.3, *b), (1.3, b + 0.2), (-0.2, b + 0.2)],
            material: None,
            tone: -1,
        });
        ornaments.push(Ornament::Line {
            points: vec![(-0.1, *b), (1.2, *b)],
            tone: DEEP as u8,
            dashed: false,
        });
    }
    let ridge = (model.ridge.0 + model.ridge.1) * 0.5;
    ornaments.push(Ornament::Line {
        points: vec![
            (ridge, model.collar),
            (ridge, model.lames.first().copied().unwrap_or(0.7)),
        ],
        tone: HIGHLIGHT as u8,
        dashed: false,
    });
    EquipmentDesign {
        version: DESIGN_VERSION,
        id: id.to_owned(),
        name: format!("{id} (adapted from ChestModel)"),
        slot: RigSlot::Chest,
        space: DesignSpace::Torso,
        depth: DepthSlot::ChestGear,
        mount: PartTransform::default(),
        light: default_light(),
        aspect_range: model.aspect_range,
        materials: vec![plate, trim],
        parts: vec![Part {
            id: "plate".into(),
            parent: None,
            bind: None,
            depth: None,
            hides: Vec::new(),
            source: PartSource::Polygon {
                points: model.silhouette.clone(),
                smooth: 0,
            },
            transform: PartTransform::default(),
            material: "plate".into(),
            layer: 0,
            outline: Outline::Selective,
            bevel: 1.0,
            regions,
            ornaments,
            visible: true,
            visibility: Visibility::default(),
        }],
        provisional: true,
        source: "adapter of the 02 ChestModel".into(),
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn sample() -> EquipmentDesign {
        let steel = PaintMaterial {
            id: "steel".into(),
            ramp: [
                [30, 26, 36],
                [52, 56, 70],
                [84, 90, 108],
                [122, 130, 148],
                [168, 176, 192],
                [220, 226, 236],
            ],
            finish: Finish::Metal,
        };
        let gold = PaintMaterial {
            id: "gold".into(),
            ramp: [
                [40, 24, 16],
                [96, 62, 24],
                [140, 96, 36],
                [196, 150, 60],
                [228, 192, 96],
                [250, 232, 160],
            ],
            finish: Finish::Flat,
        };
        EquipmentDesign {
            version: DESIGN_VERSION,
            id: "test_plate".into(),
            name: "Test plate".into(),
            slot: RigSlot::Chest,
            space: DesignSpace::Torso,
            depth: DepthSlot::ChestGear,
            mount: PartTransform::default(),
            light: default_light(),
            aspect_range: default_aspect(),
            materials: vec![steel, gold],
            parts: vec![
                Part {
                    id: "body".into(),
                    parent: None,
                    bind: None,
                    depth: None,
                    hides: Vec::new(),
                    source: PartSource::Polygon {
                        points: vec![(0.0, 0.0), (1.05, 0.0), (1.1, 0.5), (1.0, 1.0), (0.0, 1.0)],
                        smooth: 0,
                    },
                    transform: PartTransform::default(),
                    material: "steel".into(),
                    layer: 0,
                    outline: Outline::Selective,
                    bevel: 1.5,
                    regions: vec![Region {
                        points: vec![(0.0, 0.7), (1.2, 0.7), (1.2, 1.1), (0.0, 1.1)],
                        material: None,
                        tone: -1,
                    }],
                    ornaments: vec![Ornament::Rivet { at: (0.8, 0.3) }],
                    visible: true,
                    visibility: Visibility::default(),
                },
                Part {
                    id: "collar".into(),
                    parent: None,
                    bind: None,
                    depth: None,
                    hides: Vec::new(),
                    source: PartSource::Polygon {
                        points: vec![(0.1, -0.05), (1.0, -0.05), (0.95, 0.12), (0.15, 0.12)],
                        smooth: 1,
                    },
                    transform: PartTransform::default(),
                    material: "gold".into(),
                    layer: 1,
                    outline: Outline::Selective,
                    bevel: 1.0,
                    regions: Vec::new(),
                    ornaments: Vec::new(),
                    visible: true,
                    visibility: Visibility::default(),
                },
            ],
            provisional: true,
            source: "test".into(),
        }
    }

    #[test]
    fn a_design_round_trips_through_ron() {
        let d = sample();
        let text = d.to_ron().unwrap_or_else(|e| panic!("{e}"));
        let back = EquipmentDesign::from_ron(&text).unwrap_or_else(|e| panic!("{e}\n{text}"));
        assert_eq!(back, d);
    }

    #[test]
    fn invalid_designs_are_refused() {
        let mut d = sample();
        d.parts[0].material = "mithril".into();
        assert!(matches!(d.validate(), Err(DesignError::Material(..))));
        let mut d = sample();
        d.parts[1].id = "body".into();
        assert!(matches!(d.validate(), Err(DesignError::DuplicatePart(_))));
        let mut d = sample();
        d.parts[0].source = PartSource::Polygon {
            points: vec![(0.0, 0.0), (1.0, 0.0)],
            smooth: 0,
        };
        assert!(matches!(d.validate(), Err(DesignError::Polygon(_))));
        let mut d = sample();
        d.parts[0].transform.scale = (0.0, 1.0);
        assert!(matches!(d.validate(), Err(DesignError::Transform(_))));
        let mut d = sample();
        d.parts[0].regions[0].material = Some("x".into());
        assert!(matches!(d.validate(), Err(DesignError::RegionMaterial(..))));
        let mut d = sample();
        d.parts.clear();
        assert_eq!(d.validate(), Err(DesignError::Empty));
        let mut d = sample();
        d.version = 9;
        assert_eq!(d.validate(), Err(DesignError::Version(9)));
    }

    #[test]
    fn transforms_invert() {
        let t = PartTransform {
            position: (3.0, -2.0),
            rotation_deg: 33.0,
            scale: (1.5, 0.75),
            pivot: (1.0, 1.0),
        };
        for p in [(0.0, 0.0), (2.0, 5.0), (-3.0, 1.5)] {
            let back = t.invert(t.apply(p)).unwrap_or_else(|| panic!("invert"));
            assert!((back.0 - p.0).abs() < 1e-4 && (back.1 - p.1).abs() < 1e-4);
        }
        // The pivot stays put under rotation and scale.
        let q = t.apply(t.pivot);
        assert!((q.0 - 3.0).abs() < 1e-5 && (q.1 + 2.0).abs() < 1e-5);
    }

    #[test]
    fn chaikin_rounds_a_square_inside_its_hull() {
        let sq = [(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)];
        let c = chaikin(&sq, 2);
        assert_eq!(c.len(), 16);
        assert!(
            c.iter()
                .all(|p| p.0 >= 0.0 && p.0 <= 4.0 && p.1 >= 0.0 && p.1 <= 4.0)
        );
        // The corners are cut away.
        assert!(sq.iter().all(|corner| !c.contains(corner)));
        assert_eq!(chaikin(&sq, 0).len(), 4);
    }

    #[test]
    fn visibility_rules_and_turn() {
        let v = Visibility {
            clips: vec!["run".into()],
            turn: Some((0.5, 1.0)),
        };
        assert!(v.allows("run", 0.7));
        assert!(!v.allows("idle", 0.7));
        assert!(!v.allows("run", 0.2));
        let d = sample();
        assert_eq!(d.turn(0.6), 0.0);
        assert_eq!(d.turn(0.9), 1.0);
        assert_eq!(d.free_id("body"), "body_2");
        assert_eq!(d.free_id("strap"), "strap");
    }

    #[test]
    fn affine_and_transform_agree_and_invert() {
        let t = PartTransform {
            position: (3.0, -2.0),
            rotation_deg: 33.0,
            scale: (1.5, 0.75),
            pivot: (1.0, 1.0),
        };
        let m = t.affine();
        for p in [(0.0, 0.0), (2.0, 5.0)] {
            let (a, b) = (m.apply(p), t.apply(p));
            assert!((a.0 - b.0).abs() < 1e-4 && (a.1 - b.1).abs() < 1e-4);
        }
        let back = PartTransform::from_affine(&m, t.pivot);
        assert!((back.rotation_deg - 33.0).abs() < 1e-3);
        assert!((back.scale.0 - 1.5).abs() < 1e-4 && (back.scale.1 - 0.75).abs() < 1e-4);
        let inv = m.inverse().unwrap_or_else(|| panic!("inverse"));
        let q = inv.apply(m.apply((7.0, -3.0)));
        assert!((q.0 - 7.0).abs() < 1e-3 && (q.1 + 3.0).abs() < 1e-3);
    }

    /// body <- collar <- (gem); the collar is a child of the body.
    fn family() -> EquipmentDesign {
        let mut d = sample();
        d.parts[1].parent = Some("body".into());
        let mut gem = d.parts[1].clone();
        gem.id = "gem".into();
        gem.parent = Some("collar".into());
        d.parts.push(gem);
        d
    }

    #[test]
    fn children_follow_their_parent_and_inherit_visibility() {
        let mut d = family();
        assert_eq!(d.validate(), Ok(()));
        let before = d.world_affine(2).apply((0.5, 0.5));
        d.parts[0].transform.position = (1.0, 2.0);
        d.parts[0].transform.rotation_deg = 90.0;
        let after = d.world_affine(2).apply((0.5, 0.5));
        let moved = (after.0 - before.0).hypot(after.1 - before.1);
        assert!(moved > 0.1, "the grandchild moved ({moved})");
        assert_eq!(d.subtree(0), vec![0, 1, 2]);
        assert_eq!(d.children(0), vec![1]);
        d.parts[0].visible = false;
        assert!(!d.effective_visible(2));
        d.parts[0].visible = true;
        d.parts[0].depth = Some(DepthSlot::ShoulderGear);
        assert_eq!(
            d.effective_depth(2, DepthSlot::ChestGear),
            DepthSlot::ShoulderGear
        );
    }

    #[test]
    fn cycles_and_bad_parents_are_refused() {
        let mut d = family();
        d.parts[0].parent = Some("gem".into());
        assert!(matches!(d.validate(), Err(DesignError::Cycle(_))));
        let mut d = family();
        d.parts[1].parent = Some("ghost".into());
        assert!(matches!(d.validate(), Err(DesignError::Parent(..))));
        let mut d = family();
        d.parts[1].bind = Some(DesignSpace::Torso);
        assert!(matches!(d.validate(), Err(DesignError::BindOnChild(_))));
        let mut d = family();
        assert!(
            d.reparent(0, Some(2), false).is_err(),
            "a part under its own grandchild"
        );
    }

    #[test]
    fn reparenting_can_keep_the_global_transform() {
        let mut d = family();
        d.parts[0].transform.position = (2.0, 1.0);
        d.parts[0].transform.rotation_deg = 30.0;
        let p = (0.4, 0.2);
        let before = d.world_affine(2).apply(p);
        d.reparent(2, None, true).unwrap_or_else(|e| panic!("{e}"));
        let kept = d.world_affine(2).apply(p);
        assert!((kept.0 - before.0).abs() < 1e-3 && (kept.1 - before.1).abs() < 1e-3);
        d.reparent(2, Some(1), false)
            .unwrap_or_else(|e| panic!("{e}"));
        let moved = d.world_affine(2).apply(p);
        assert!((moved.0 - kept.0).abs() > 1e-3 || (moved.1 - kept.1).abs() > 1e-3);
    }

    #[test]
    fn groups_duplicate_and_remove_whole_subtrees() {
        let mut d = family();
        let copy = d.duplicate_subtree(1);
        assert_eq!(d.parts.len(), 5);
        assert_eq!(d.parts[copy].parent.as_deref(), Some("body"));
        assert_eq!(
            d.parts[copy + 1].parent.as_deref(),
            Some(d.parts[copy].id.as_str())
        );
        assert_eq!(d.validate(), Ok(()));
        let g = d.group(&[1, copy], "trims");
        assert!(matches!(d.parts[g].source, PartSource::Group));
        assert_eq!(d.parts[1].parent.as_deref(), Some("trims"));
        assert_eq!(d.validate(), Ok(()));
        let n = d.parts.len();
        let removed = d.subtree(g).len();
        d.remove_subtree(g);
        assert_eq!(d.parts.len(), n - removed);
        assert_eq!(d.validate(), Ok(()));
    }

    #[test]
    fn pixel_rows_are_checked() {
        let mut d = sample();
        d.parts[1].source = PartSource::Pixels {
            size: (2, 2),
            pivot: (1.0, 1.0),
            palette: vec![[1, 2, 3, 255]],
            rows: vec!["0.".into(), ".0".into()],
        };
        assert_eq!(d.validate(), Ok(()));
        d.parts[1].source = PartSource::Pixels {
            size: (2, 2),
            pivot: (1.0, 1.0),
            palette: vec![[1, 2, 3, 255]],
            rows: vec!["01".into(), ".0".into()],
        };
        assert!(matches!(d.validate(), Err(DesignError::Pixels(_))));
        assert_eq!(pixel_index(pixel_char(17)), Some(17));
        assert_eq!(pixel_index('.'), None);
    }

    #[test]
    fn the_02_chest_model_is_adapted() {
        let model = super::super::chest::ChestModel {
            silhouette: vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            ramp: sample().materials[0].ramp,
            trim: [[1, 2, 3], [4, 5, 6]],
            collar: 0.1,
            lames: vec![0.7, 0.85],
            ridge: (0.8, 0.6),
            aspect_range: (0.67, 0.78),
        };
        let d = from_chest_model("old", &model);
        assert_eq!(d.validate(), Ok(()));
        assert_eq!(d.parts.len(), 1);
        assert_eq!(d.parts[0].regions.len(), 3);
        assert_eq!(d.space, DesignSpace::Torso);
    }
}
