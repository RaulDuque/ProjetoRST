//! Torso equipment: one breastplate source fitted to every pose. Pure module.
//!
//! The torso of each frame is a **cage**: the quad of the four torso-box
//! joints (chest back, chest front, waist front, waist back). A point of the
//! breastplate is written in cage coordinates `(u, v)`: `u` from the back
//! (0) to the front (1), `v` from the shoulder line (0) to the waist (1);
//! values a little outside 0..1 let the plate bulge past the shirt.
//!
//! Three ways to put the same [`ChestModel`] on a pose are compared:
//!
//! - [`ChestMode::Rigid`]: rasterised once on the rest pose and moved as a
//!   rigid sprite (translation + the spine's rotation);
//! - [`ChestMode::MeshWarp`]: the rest raster deformed through the cage
//!   (a mesh whose vertices are bound to the torso joints), sampled back on
//!   the frame's texel grid;
//! - [`ChestMode::Projected`]: the vector model itself rasterised on each
//!   pose's grid (the pixelfx approach of rendering geometry straight to
//!   pixels, `src/core/rig.js`, MIT, Patch / Hugo Duprez, in 2D).
//!
//! Every output is a cell-sized RGBA image on the character's own texel
//! grid: no resampling blur, no sub-texel rotation.

use serde::Deserialize;

use super::layers::{CellGeometry, point_in_polygon};
use super::skeleton::{Joint, JointPositions};
use super::xform::RigPoint;

/// How the breastplate follows the torso.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum ChestMode {
    Rigid,
    MeshWarp,
    Projected,
}

impl ChestMode {
    pub const ALL: [Self; 3] = [Self::Rigid, Self::MeshWarp, Self::Projected];

    pub const fn next(self) -> Self {
        match self {
            Self::Rigid => Self::MeshWarp,
            Self::MeshWarp => Self::Projected,
            Self::Projected => Self::Rigid,
        }
    }
}

/// The four torso corners of one frame (rig space).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TorsoCage {
    pub chest_back: RigPoint,
    pub chest_front: RigPoint,
    pub waist_front: RigPoint,
    pub waist_back: RigPoint,
}

impl TorsoCage {
    pub fn from_joints(j: &JointPositions) -> Self {
        Self {
            chest_back: j.get(Joint::ChestBack),
            chest_front: j.get(Joint::ChestFront),
            waist_front: j.get(Joint::WaistFront),
            waist_back: j.get(Joint::WaistBack),
        }
    }

    /// Cage point of `(u, v)` (bilinear, extrapolates outside 0..1).
    pub fn point(&self, u: f32, v: f32) -> RigPoint {
        let top = lerp2(self.chest_back, self.chest_front, u);
        let bottom = lerp2(self.waist_back, self.waist_front, u);
        lerp2(top, bottom, v)
    }

    /// `(u, v)` of a point (inverse bilinear; the root nearest to the cage
    /// is chosen). `None` for a degenerate cage.
    pub fn uv(&self, p: RigPoint) -> Option<(f32, f32)> {
        inverse_bilinear(
            p,
            self.chest_back,
            self.chest_front,
            self.waist_front,
            self.waist_back,
        )
    }

    /// Average width (back -> front) over average height.
    pub fn aspect(&self) -> f32 {
        let w = (dist(self.chest_back, self.chest_front) + dist(self.waist_back, self.waist_front))
            * 0.5;
        let h = (dist(self.chest_back, self.waist_back) + dist(self.chest_front, self.waist_front))
            * 0.5;
        if h > 0.0 { w / h } else { 0.0 }
    }

    pub fn centre(&self) -> RigPoint {
        self.point(0.5, 0.5)
    }
}

fn lerp2(a: RigPoint, b: RigPoint, t: f32) -> RigPoint {
    (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t)
}

fn dist(a: RigPoint, b: RigPoint) -> f32 {
    (b.0 - a.0).hypot(b.1 - a.1)
}

fn cross(a: RigPoint, b: RigPoint) -> f32 {
    a.0 * b.1 - a.1 * b.0
}

/// Inverse of `P(u,v) = a + u·e + v·f + u·v·g` for the quad a, b, c, d
/// (closed form after Inigo Quilez, "inverse bilinear interpolation").
pub fn inverse_bilinear(
    p: RigPoint,
    a: RigPoint,
    b: RigPoint,
    c: RigPoint,
    d: RigPoint,
) -> Option<(f32, f32)> {
    let e = (b.0 - a.0, b.1 - a.1);
    let f = (d.0 - a.0, d.1 - a.1);
    let g = (a.0 - b.0 + c.0 - d.0, a.1 - b.1 + c.1 - d.1);
    let h = (p.0 - a.0, p.1 - a.1);
    let k2 = cross(g, f);
    let k1 = cross(e, f) + cross(h, g);
    let k0 = cross(h, e);
    let solve_u = |v: f32| -> Option<f32> {
        let dx = e.0 + g.0 * v;
        let dy = e.1 + g.1 * v;
        if dx.abs() > dy.abs() {
            (dx.abs() > 1e-6).then(|| (h.0 - f.0 * v) / dx)
        } else {
            (dy.abs() > 1e-6).then(|| (h.1 - f.1 * v) / dy)
        }
    };
    let candidates: Vec<f32> = if k2.abs() < 1e-6 {
        if k1.abs() < 1e-9 {
            return None;
        }
        vec![-k0 / k1]
    } else {
        let w = k1 * k1 - 4.0 * k0 * k2;
        if w < 0.0 {
            return None;
        }
        let w = w.sqrt();
        vec![(-k1 - w) / (2.0 * k2), (-k1 + w) / (2.0 * k2)]
    };
    candidates
        .into_iter()
        .filter_map(|v| Some((solve_u(v)?, v)))
        .filter(|(u, v)| u.is_finite() && v.is_finite())
        .min_by(|x, y| outside(*x).total_cmp(&outside(*y)))
}

/// How far `(u, v)` lies outside the unit square (0 inside).
fn outside((u, v): (f32, f32)) -> f32 {
    let du = (-u).max(u - 1.0).max(0.0);
    let dv = (-v).max(v - 1.0).max(0.0);
    du + dv
}

/// The breastplate source: a vector design in cage coordinates and its
/// palette. Authored once; every pose is drawn from it.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ChestModel {
    /// Silhouette polygon in `(u, v)`.
    pub silhouette: Vec<(f32, f32)>,
    /// Colours, RGB: outline, deep shadow, shadow, base, light, highlight.
    pub ramp: [[u8; 3]; 6],
    /// Trim colours, RGB: dark, light.
    pub trim: [[u8; 3]; 2],
    /// Top band (collar) height in `v`.
    pub collar: f32,
    /// `v` of the lame (horizontal plate) boundaries at the bottom.
    pub lames: Vec<f32>,
    /// `u` of the central ridge in the side view and in the three-quarter
    /// view (the cage's aspect picks between them: a perspective variant
    /// derived from the same source).
    pub ridge: (f32, f32),
    /// Cage aspect (width / height) of the side view and of the
    /// three-quarter view.
    pub aspect_range: (f32, f32),
}

impl ChestModel {
    /// How much the torso is turned towards the camera (0 side, 1 3/4).
    pub fn turn(&self, cage: &TorsoCage) -> f32 {
        let (a0, a1) = self.aspect_range;
        if (a1 - a0).abs() < 1e-6 {
            return 0.0;
        }
        ((cage.aspect() - a0) / (a1 - a0)).clamp(0.0, 1.0)
    }

    /// Colour of the plate at `(u, v)` before outlining, or `None` outside.
    /// `du` and `dv` are one texel in cage units (line thickness).
    fn shade(&self, u: f32, v: f32, du: f32, dv: f32, turn: f32) -> Option<[u8; 3]> {
        if !point_in_polygon((u, v), &self.silhouette) {
            return None;
        }
        let [_, deep, shadow, base, light, highlight] = self.ramp;
        if v < self.collar {
            return Some(if v < self.collar * 0.5 {
                self.trim[1]
            } else {
                self.trim[0]
            });
        }
        for b in &self.lames {
            if (v - b).abs() < dv * 0.5 {
                return Some(deep);
            }
            if v > *b && v - b < dv * 1.5 {
                return Some(light);
            }
        }
        let ridge = self.ridge.0 + (self.ridge.1 - self.ridge.0) * turn;
        if (u - ridge).abs() < du * 0.5 {
            return Some(highlight);
        }
        // Light from the front and above: the back is in shadow.
        let lit = u - ridge;
        Some(if u < 0.12 {
            deep
        } else if u < 0.35 {
            shadow
        } else if lit < -du * 0.5 {
            base
        } else if lit < du * 2.5 {
            light
        } else {
            base
        })
    }

    /// The model drawn on a cage, cell-sized RGBA (row-major).
    pub fn rasterize(&self, cage: &TorsoCage, cell: CellGeometry) -> Vec<[u8; 4]> {
        let turn = self.turn(cage);
        let (w, h) = (cell.width, cell.height);
        let width_texels = dist(cage.chest_back, cage.chest_front).max(1.0);
        let height_texels = dist(cage.chest_back, cage.waist_back).max(1.0);
        let (du, dv) = (1.0 / width_texels, 1.0 / height_texels);
        let mut colour = vec![None; w * h];
        for (i, c) in colour.iter_mut().enumerate() {
            let p = cell.texel_centre(i % w, i / w);
            if let Some((u, v)) = cage.uv(p) {
                *c = self.shade(u, v, du, dv, turn);
            }
        }
        outline(&colour, w, h, self.ramp[0])
    }
}

/// Opaque RGBA with a 1-texel outline on the silhouette's edge texels.
fn outline(colour: &[Option<[u8; 3]>], w: usize, h: usize, ink: [u8; 3]) -> Vec<[u8; 4]> {
    let at = |x: isize, y: isize| -> bool {
        x >= 0
            && y >= 0
            && (x as usize) < w
            && (y as usize) < h
            && colour[y as usize * w + x as usize].is_some()
    };
    colour
        .iter()
        .enumerate()
        .map(|(i, c)| match c {
            None => [0; 4],
            Some(rgb) => {
                let (x, y) = ((i % w) as isize, (i / w) as isize);
                let edge = !(at(x - 1, y) && at(x + 1, y) && at(x, y - 1) && at(x, y + 1));
                let c = if edge { ink } else { *rgb };
                [c[0], c[1], c[2], 255]
            }
        })
        .collect()
}

/// Mesh warp: samples `source` (drawn on `source_cage`) through the cage
/// mapping onto `target_cage`, nearest texel.
pub fn warp(
    source: &[[u8; 4]],
    source_cage: &TorsoCage,
    source_cell: CellGeometry,
    target_cage: &TorsoCage,
    target_cell: CellGeometry,
) -> Vec<[u8; 4]> {
    let mut out = vec![[0u8; 4]; target_cell.width * target_cell.height];
    for (i, px) in out.iter_mut().enumerate() {
        let p = target_cell.texel_centre(i % target_cell.width, i / target_cell.width);
        let Some((u, v)) = target_cage.uv(p) else {
            continue;
        };
        let q = source_cell.to_texel(source_cage.point(u, v));
        let (x, y) = (q.0.floor(), q.1.floor());
        if x < 0.0 || y < 0.0 {
            continue;
        }
        let (x, y) = (x as usize, y as usize);
        if x < source_cell.width && y < source_cell.height {
            *px = source[y * source_cell.width + x];
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cage(w: f32) -> TorsoCage {
        TorsoCage {
            chest_back: (-w * 0.5, 86.0),
            chest_front: (w * 0.5, 86.0),
            waist_front: (w * 0.5 - 1.0, 66.0),
            waist_back: (-w * 0.5 + 1.0, 66.0),
        }
    }

    fn model() -> ChestModel {
        ChestModel {
            silhouette: vec![
                (-0.05, 0.0),
                (1.05, 0.0),
                (1.1, 0.5),
                (1.0, 1.0),
                (0.0, 1.0),
            ],
            ramp: [
                [30, 26, 36],
                [52, 56, 70],
                [84, 90, 108],
                [122, 130, 148],
                [168, 176, 192],
                [220, 226, 236],
            ],
            trim: [[120, 90, 40], [210, 170, 80]],
            collar: 0.1,
            lames: vec![0.75, 0.88],
            ridge: (0.75, 0.6),
            aspect_range: (0.65, 1.2),
        }
    }

    const CELL: CellGeometry = CellGeometry {
        width: 80,
        height: 128,
        pivot: (40.0, 118.0),
    };

    #[test]
    fn inverse_bilinear_round_trips_inside_and_slightly_outside() {
        let c = TorsoCage {
            chest_back: (-6.0, 86.0),
            chest_front: (7.0, 87.0),
            waist_front: (5.0, 66.0),
            waist_back: (-4.0, 67.0),
        };
        for (u, v) in [(0.0, 0.0), (0.3, 0.7), (1.0, 1.0), (1.1, 0.5), (-0.05, 0.2)] {
            let p = c.point(u, v);
            let (bu, bv) = c.uv(p).unwrap_or_else(|| panic!("uv"));
            assert!((bu - u).abs() < 1e-3 && (bv - v).abs() < 1e-3, "{u},{v}");
        }
    }

    #[test]
    fn a_wider_cage_turns_the_model_and_widens_the_raster() {
        let m = model();
        let narrow = cage(13.0);
        let wide = cage(24.0);
        assert!(m.turn(&narrow) < m.turn(&wide));
        let count = |px: &[[u8; 4]]| px.iter().filter(|p| p[3] > 0).count();
        let a = m.rasterize(&narrow, CELL);
        let b = m.rasterize(&wide, CELL);
        assert!(count(&b) > count(&a) * 3 / 2);
        // Binary alpha, palette colours only.
        let mut palette: Vec<[u8; 3]> = m.ramp.to_vec();
        palette.extend(m.trim);
        for p in a.iter().chain(&b).filter(|p| p[3] > 0) {
            assert_eq!(p[3], 255);
            assert!(palette.contains(&[p[0], p[1], p[2]]), "{p:?}");
        }
    }

    #[test]
    fn the_raster_is_outlined_and_inside_the_cage_band() {
        let m = model();
        let c = cage(16.0);
        let px = m.rasterize(&c, CELL);
        let ink = [m.ramp[0][0], m.ramp[0][1], m.ramp[0][2], 255];
        let opaque: Vec<usize> = (0..px.len()).filter(|i| px[*i][3] > 0).collect();
        assert!(!opaque.is_empty());
        for i in &opaque {
            let p = CELL.texel_centre(i % CELL.width, i / CELL.width);
            assert!(p.1 <= 87.0 && p.1 >= 65.0, "row {}", p.1);
        }
        // Every edge texel is ink.
        for i in &opaque {
            let (x, y) = (i % CELL.width, i / CELL.width);
            let edge = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                .iter()
                .any(|(a, b)| px[b * CELL.width + a][3] == 0);
            if edge {
                assert_eq!(px[*i], ink);
            }
        }
    }

    #[test]
    fn warping_onto_the_same_cage_is_the_identity() {
        let m = model();
        let c = cage(16.0);
        let src = m.rasterize(&c, CELL);
        let same = warp(&src, &c, CELL, &c, CELL);
        assert_eq!(src, same);
    }

    #[test]
    fn modes_cycle() {
        let mut m = ChestMode::Rigid;
        for _ in 0..3 {
            m = m.next();
        }
        assert_eq!(m, ChestMode::Rigid);
    }
}
