//! The generic equipment painter. Pure module: no Bevy types.
//!
//! [`paint`] draws any [`EquipmentDesign`] on a pose, straight on the
//! character's texel grid (cell-sized RGBA, binary alpha, palette colours
//! only, no resampling): for every texel it maps the cell point into the
//! design space (torso cage or hand/bone frame), finds the topmost part that
//! covers it and shades it.
//!
//! Shading follows the pixelfx painter (`src/core/rig.js`, MIT, Copyright
//! (c) 2026 Patch, Hugo Duprez): a normal per texel lit by one light,
//! quantised into the ramp; ink on the silhouette and a seam (one shade
//! darker) where a part meets a lower part. Here the normal comes from the
//! part's bevel (the distance to its edge and the edge's direction) and,
//! on the torso, from the cylinder of the body (`u` back -> front).

use std::collections::VecDeque;

use super::chest::TorsoCage;
use super::design::{
    BASE, DEEP, DesignSpace, EquipmentDesign, Finish, HIGHLIGHT, INK, LIGHT, Ornament, Outline,
    PaintMaterial, Part, PartSource, SHADOW, chaikin,
};
use super::layers::{CellGeometry, point_in_polygon};
use super::xform::{RigPoint, Xform2};

/// A decoded RGBA image (row-major from the top-left).
#[derive(Debug, Clone, PartialEq)]
pub struct RgbaImage {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<[u8; 4]>,
}

impl RgbaImage {
    pub fn get(&self, x: i64, y: i64) -> Option<[u8; 4]> {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return None;
        }
        Some(self.pixels[y as usize * self.width + x as usize])
    }
}

/// How cell points map into the design space of a pose.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpaceMap {
    /// Torso cage `(u, v)`.
    Cage(TorsoCage),
    /// A hand or bone frame (texels, `+x` along it).
    Frame(Xform2),
}

impl SpaceMap {
    /// Rig point -> space coordinates.
    pub fn to_space(&self, p: RigPoint) -> Option<(f32, f32)> {
        match self {
            Self::Cage(c) => c.uv(p),
            Self::Frame(f) => Some(f.inverse().apply(p)),
        }
    }

    /// Space coordinates -> rig point.
    pub fn to_rig(&self, s: (f32, f32)) -> RigPoint {
        match self {
            Self::Cage(c) => c.point(s.0, s.1),
            Self::Frame(f) => f.apply(s),
        }
    }

    /// Space units per texel (x, y).
    pub fn texel(&self) -> (f32, f32) {
        match self {
            Self::Cage(c) => {
                let w = (c.chest_front.0 - c.chest_back.0).hypot(c.chest_front.1 - c.chest_back.1);
                let h = (c.waist_back.0 - c.chest_back.0).hypot(c.waist_back.1 - c.chest_back.1);
                (1.0 / w.max(1.0), 1.0 / h.max(1.0))
            }
            Self::Frame(_) => (1.0, 1.0),
        }
    }
}

/// What to paint: a design on one pose.
pub struct PaintJob<'a> {
    pub design: &'a EquipmentDesign,
    pub map: SpaceMap,
    pub cell: CellGeometry,
    pub clip: &'a str,
    /// Torso turn (0 side view, 1 three-quarter), see `EquipmentDesign::turn`.
    pub turn: f32,
    /// Decoded images by `PartSource::Image::path`.
    pub images: &'a dyn Fn(&str) -> Option<&'a RgbaImage>,
}

/// The painted item: cell-sized RGBA and which part owns each texel.
#[derive(Debug, Clone, PartialEq)]
pub struct Painted {
    pub rgba: Vec<[u8; 4]>,
    pub owner: Vec<Option<usize>>,
}

impl Painted {
    pub fn opaque(&self) -> usize {
        self.rgba.iter().filter(|p| p[3] > 0).count()
    }
}

/// Bayer 2x2 thresholds (ordered dithering of leather).
const BAYER: [[f32; 2]; 2] = [[0.125, 0.625], [0.875, 0.375]];

/// Per-texel intermediate result of one part.
#[derive(Clone, Copy)]
enum Fill {
    /// A shaded polygon texel: local coordinates.
    Shape((f32, f32)),
    /// An image texel colour.
    Pixel([u8; 4]),
}

fn local_of(job: &PaintJob<'_>, part: &Part, p: RigPoint) -> Option<(f32, f32)> {
    let s = job.map.to_space(p)?;
    let d = job.design.mount.invert(s)?;
    part.transform.invert(d)
}

/// Local point -> cell texel coordinates (y down).
fn local_to_texel(job: &PaintJob<'_>, part: &Part, l: (f32, f32)) -> (f32, f32) {
    let s = job.design.mount.apply(part.transform.apply(l));
    job.cell.to_texel(job.map.to_rig(s))
}

/// Which texels a part covers and how.
fn cover(job: &PaintJob<'_>, part: &Part) -> Vec<Option<Fill>> {
    let n = job.cell.width * job.cell.height;
    let mut out = vec![None; n];
    match &part.source {
        PartSource::Polygon { points, smooth } => {
            let poly = chaikin(points, *smooth);
            for (i, o) in out.iter_mut().enumerate() {
                let p = job
                    .cell
                    .texel_centre(i % job.cell.width, i / job.cell.width);
                if let Some(l) = local_of(job, part, p)
                    && point_in_polygon(l, &poly)
                {
                    *o = Some(Fill::Shape(l));
                }
            }
        }
        PartSource::Image { path, pivot } => {
            let Some(img) = (job.images)(path) else {
                return out;
            };
            match (job.design.space, job.map) {
                (DesignSpace::Torso, _) | (_, SpaceMap::Cage(_)) => {
                    // A decal: pixel-aligned on the anchor, never stretched.
                    let anchor = local_to_texel(job, part, part.transform.pivot);
                    let (ax, ay) = (anchor.0.floor() as i64, anchor.1.floor() as i64);
                    for (i, o) in out.iter_mut().enumerate() {
                        let (x, y) = ((i % job.cell.width) as i64, (i / job.cell.width) as i64);
                        let ix = x - ax + pivot.0.floor() as i64;
                        let iy = y - ay + pivot.1.floor() as i64;
                        if let Some(px) = img.get(ix, iy)
                            && px[3] > 0
                        {
                            *o = Some(Fill::Pixel(px));
                        }
                    }
                }
                _ => {
                    for (i, o) in out.iter_mut().enumerate() {
                        let p = job
                            .cell
                            .texel_centre(i % job.cell.width, i / job.cell.width);
                        let Some(l) = local_of(job, part, p) else {
                            continue;
                        };
                        let ix = (pivot.0 + l.0).floor() as i64;
                        let iy = (pivot.1 - l.1).floor() as i64;
                        if let Some(px) = img.get(ix, iy)
                            && px[3] > 0
                        {
                            *o = Some(Fill::Pixel(px));
                        }
                    }
                }
            }
        }
    }
    out
}

/// Distance (in texels, 4-neighbour steps) from each covered texel to the
/// part's edge, capped at `cap`.
fn edge_distance(mask: &[bool], w: usize, h: usize, cap: u16) -> Vec<u16> {
    let mut dist = vec![u16::MAX; mask.len()];
    let mut queue = VecDeque::new();
    for i in 0..mask.len() {
        if !mask[i] {
            continue;
        }
        let (x, y) = (i % w, i / w);
        let edge = x == 0
            || y == 0
            || x + 1 == w
            || y + 1 == h
            || !mask[i - 1]
            || !mask[i + 1]
            || !mask[i - w]
            || !mask[i + w];
        if edge {
            dist[i] = 1;
            queue.push_back(i);
        }
    }
    while let Some(i) = queue.pop_front() {
        let d = dist[i];
        if d >= cap {
            continue;
        }
        let (x, y) = (i % w, i / w);
        let mut push = |j: usize| {
            if mask[j] && dist[j] > d + 1 {
                dist[j] = d + 1;
                queue.push_back(j);
            }
        };
        if x > 0 {
            push(i - 1);
        }
        if x + 1 < w {
            push(i + 1);
        }
        if y > 0 {
            push(i - w);
        }
        if y + 1 < h {
            push(i + w);
        }
    }
    dist
}

/// Outward direction of the edge near a texel (y up), from the uncovered
/// texels around it.
fn outward(mask: &[bool], w: usize, h: usize, i: usize, reach: i64) -> (f32, f32) {
    let (x, y) = ((i % w) as i64, (i / w) as i64);
    let (mut ox, mut oy) = (0.0f32, 0.0f32);
    for dy in -reach..=reach {
        for dx in -reach..=reach {
            let (nx, ny) = (x + dx, y + dy);
            let outside = nx < 0
                || ny < 0
                || nx >= w as i64
                || ny >= h as i64
                || !mask[ny as usize * w + nx as usize];
            if outside {
                ox += dx as f32;
                oy -= dy as f32;
            }
        }
    }
    let n = ox.hypot(oy);
    if n > 0.0 {
        (ox / n, oy / n)
    } else {
        (0.0, 0.0)
    }
}

fn tone_gain(finish: Finish) -> f32 {
    match finish {
        Finish::Metal => 5.0,
        Finish::Leather => 3.5,
        Finish::Cloth => 3.0,
        Finish::Flat => 2.2,
    }
}

fn colour(m: &PaintMaterial, tone: usize) -> [u8; 4] {
    let c = m.ramp[tone.min(HIGHLIGHT)];
    [c[0], c[1], c[2], 255]
}

/// Paints a design on a pose.
pub fn paint(job: &PaintJob<'_>) -> Painted {
    let (w, h) = (job.cell.width, job.cell.height);
    let n = w * h;
    let d = job.design;
    // Visible parts in draw order (back to front).
    let mut order: Vec<usize> = (0..d.parts.len())
        .filter(|i| d.parts[*i].visible && d.parts[*i].visibility.allows(job.clip, job.turn))
        .collect();
    order.sort_by_key(|i| d.parts[*i].layer);
    let covers: Vec<Vec<Option<Fill>>> = order.iter().map(|i| cover(job, &d.parts[*i])).collect();
    let mut owner: Vec<Option<usize>> = vec![None; n];
    for (k, c) in covers.iter().enumerate() {
        for (t, f) in c.iter().enumerate() {
            if f.is_some() {
                owner[t] = Some(k);
            }
        }
    }
    let (lx, ly) = d.light;
    let ln = (lx * lx + ly * ly + 0.36).sqrt();
    let light = (lx / ln, ly / ln, 0.6 / ln);
    let mut rgba = vec![[0u8; 4]; n];
    for (k, &pi) in order.iter().enumerate() {
        let part = &d.parts[pi];
        let Some(mat) = d.material(&part.material) else {
            continue;
        };
        let mask: Vec<bool> = covers[k].iter().map(Option::is_some).collect();
        let bevel = part.bevel.max(0.0);
        let dist = edge_distance(&mask, w, h, bevel.ceil() as u16 + 2);
        let regions: Vec<Vec<(f32, f32)>> = part.regions.iter().map(|r| r.points.clone()).collect();
        let mut tones: Vec<Option<(usize, usize)>> = vec![None; n]; // (material index in design, tone)
        let mat_index = |id: &str| d.materials.iter().position(|m| m.id == id);
        let own = mat_index(&part.material).unwrap_or(0);
        for t in 0..n {
            if owner[t] != Some(k) {
                continue;
            }
            match covers[k][t] {
                Some(Fill::Pixel(px)) => rgba[t] = px,
                Some(Fill::Shape(l)) => {
                    // Normal: the body's cylinder (torso) plus the bevel.
                    let (mut nx, mut ny, mut nz) = (0.0f32, 0.0f32, 1.0f32);
                    if let SpaceMap::Cage(_) = job.map
                        && let Some(s) = job.map.to_space(job.cell.texel_centre(t % w, t / w))
                    {
                        let theta = (s.0 - 0.55) * 1.6;
                        nx = theta.sin() * 0.8;
                        nz = theta.cos();
                    }
                    // The bevel starts inside the 1-texel outline ring.
                    let ring = if part.outline == Outline::None {
                        0.0
                    } else {
                        1.0
                    };
                    let dd = dist[t] as f32 - ring;
                    if bevel > 0.0 && dd >= 1.0 && dd <= bevel {
                        let weight = 1.0 - (dd - 1.0) / bevel.max(1.0);
                        let (ox, oy) = outward(&mask, w, h, t, 2);
                        nx += ox * weight * 1.4;
                        ny += oy * weight * 1.4;
                        nz *= 1.0 - weight * 0.5;
                    }
                    let len = (nx * nx + ny * ny + nz * nz).sqrt().max(1e-6);
                    let dot = (nx * light.0 + ny * light.1 + nz * light.2) / len;
                    let mut m = own;
                    let mut shift = 0i32;
                    for (r, poly) in part.regions.iter().zip(&regions) {
                        if point_in_polygon(l, poly) {
                            if let Some(id) = &r.material
                                && let Some(mi) = mat_index(id)
                            {
                                m = mi;
                            }
                            shift += i32::from(r.tone);
                        }
                    }
                    let finish = d.materials[m].finish;
                    let level = BASE as f32 + (dot - light.2) * tone_gain(finish) + shift as f32;
                    let level = level.clamp(DEEP as f32, HIGHLIGHT as f32);
                    let tone = match finish {
                        // Grain only near a tone change: a narrow band of
                        // ordered dithering, flat tones elsewhere.
                        Finish::Leather => {
                            let (x, y) = (t % w, t / w);
                            let frac = level - level.floor();
                            let lo = level.floor() as usize;
                            if (0.35..0.65).contains(&frac) {
                                let th = 0.35 + 0.3 * BAYER[y % 2][x % 2];
                                if frac > th { lo + 1 } else { lo }
                            } else {
                                level.round() as usize
                            }
                        }
                        Finish::Metal if level > 4.4 => HIGHLIGHT,
                        Finish::Metal => level.round().min(LIGHT as f32) as usize,
                        Finish::Flat => level.round().clamp(SHADOW as f32, LIGHT as f32) as usize,
                        Finish::Cloth => level.round().min(LIGHT as f32) as usize,
                    };
                    tones[t] = Some((m, tone.clamp(DEEP, HIGHLIGHT)));
                }
                None => {}
            }
        }
        // Ornaments, in local coordinates, only on this part's texels.
        for o in &part.ornaments {
            match o {
                Ornament::Rivet { at } => {
                    let (x, y) = local_to_texel(job, part, *at);
                    let t = texel_index(w, h, x, y);
                    if let Some(t) = t.filter(|t| owner[*t] == Some(k)) {
                        tones[t] = Some((own, HIGHLIGHT));
                        if let Some(b) =
                            texel_index(w, h, x, y + 1.0).filter(|b| owner[*b] == Some(k))
                        {
                            tones[b] = Some((own, DEEP));
                        }
                    }
                }
                Ornament::Line {
                    points,
                    tone,
                    dashed,
                } => {
                    let tex: Vec<(f32, f32)> = points
                        .iter()
                        .map(|p| local_to_texel(job, part, *p))
                        .collect();
                    let mut step = 0usize;
                    for seg in tex.windows(2) {
                        for (x, y) in line_texels(seg[0], seg[1]) {
                            step += 1;
                            if *dashed && step.is_multiple_of(2) {
                                continue;
                            }
                            if let Some(t) = texel_index(w, h, x as f32 + 0.5, y as f32 + 0.5)
                                .filter(|t| owner[*t] == Some(k))
                            {
                                tones[t] = Some((own, (*tone as usize).clamp(INK, HIGHLIGHT)));
                            }
                        }
                    }
                }
            }
        }
        // Edges: ink on the silhouette, a seam over lower parts.
        for t in 0..n {
            if owner[t] != Some(k) {
                continue;
            }
            let (x, y) = (t % w, t / w);
            let mut outside = false;
            let mut over_lower = false;
            for (nx, ny) in [
                (x.wrapping_sub(1), y),
                (x + 1, y),
                (x, y.wrapping_sub(1)),
                (x, y + 1),
            ] {
                if nx >= w || ny >= h {
                    outside = true;
                    continue;
                }
                match owner[ny * w + nx] {
                    None => outside = true,
                    Some(o) if o < k => over_lower = true,
                    _ => {}
                }
            }
            let image = matches!(covers[k][t], Some(Fill::Pixel(_)));
            match part.outline {
                Outline::None => {}
                _ if outside && !image => tones[t] = Some((own, INK)),
                Outline::Ink if over_lower && !image => tones[t] = Some((own, INK)),
                Outline::Selective if over_lower && !image => {
                    let base = tones[t].map_or(BASE, |(_, tone)| tone);
                    tones[t] = Some((own, base.saturating_sub(2).max(DEEP)));
                }
                _ => {}
            }
        }
        for t in 0..n {
            if let Some((m, tone)) = tones[t] {
                rgba[t] = colour(&d.materials[m], tone);
            }
        }
        // Outlined images: ink around their opaque pixels on the silhouette.
        if matches!(part.source, PartSource::Image { .. }) && part.outline != Outline::None {
            for t in 0..n {
                if owner[t] != Some(k) {
                    continue;
                }
                let (x, y) = (t % w, t / w);
                let edge = [
                    (x.wrapping_sub(1), y),
                    (x + 1, y),
                    (x, y.wrapping_sub(1)),
                    (x, y + 1),
                ]
                .iter()
                .any(|(nx, ny)| *nx >= w || *ny >= h || owner[ny * w + nx].is_none());
                if edge {
                    rgba[t] = colour(mat, INK);
                }
            }
        }
    }
    let owner = owner.iter().map(|o| o.map(|k| order[k])).collect();
    Painted { rgba, owner }
}

/// Where a design sits on one played frame of a clip: the space map, the
/// depth (a hand item takes the grip profile's per-frame depth) and the
/// torso turn. `None` when the clip, frame or grip is missing.
pub fn pose_map(
    rig: &super::data::RigLab,
    rest: &super::skeleton::LocalPose,
    design: &EquipmentDesign,
    clip: &str,
    frame: usize,
) -> Option<(SpaceMap, super::layers::DepthSlot, f32)> {
    use super::equipment::{Mount, place_on_bone};
    let c = rig.clips.get(clip)?;
    let joints = c.frame_joints(frame)?;
    let pose = super::skeleton::forward(&super::skeleton::fit(&joints));
    let cage = TorsoCage::from_joints(&joints);
    let turn = design.turn(cage.aspect());
    Some(match design.space {
        DesignSpace::Torso => (SpaceMap::Cage(cage), design.depth, turn),
        DesignSpace::Hand { side } => {
            let grip = rig.grip(clip)?;
            let hand = super::grip::hand_frame(side, &joints, &pose, rest, grip);
            (
                SpaceMap::Frame(hand),
                grip.depth_at(frame, design.depth),
                turn,
            )
        }
        DesignSpace::Bone { bone, at, follow } => {
            let mount = Mount::Bone {
                bone,
                at,
                offset: (0.0, 0.0),
                up_degrees: 90.0,
                follow,
            };
            let p = place_on_bone(&mount, design.depth, &pose, rest)?;
            (
                SpaceMap::Frame(Xform2::new(p.position.0, p.position.1, p.rotation)),
                design.depth,
                turn,
            )
        }
    })
}

fn texel_index(w: usize, h: usize, x: f32, y: f32) -> Option<usize> {
    if x < 0.0 || y < 0.0 {
        return None;
    }
    let (x, y) = (x.floor() as usize, y.floor() as usize);
    (x < w && y < h).then_some(y * w + x)
}

/// Texels of a segment between two cell points (DDA, both ends included).
fn line_texels(a: (f32, f32), b: (f32, f32)) -> Vec<(i64, i64)> {
    let (x0, y0) = (a.0.floor(), a.1.floor());
    let (x1, y1) = (b.0.floor(), b.1.floor());
    let steps = (x1 - x0).abs().max((y1 - y0).abs()).max(1.0) as i64;
    (0..=steps)
        .map(|i| {
            let t = i as f32 / steps as f32;
            (
                (x0 + (x1 - x0) * t).round() as i64,
                (y0 + (y1 - y0) * t).round() as i64,
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::super::design::tests::sample;
    use super::super::design::{DESIGN_VERSION, PartTransform, Visibility};
    use super::*;

    const CELL: CellGeometry = CellGeometry {
        width: 80,
        height: 128,
        pivot: (40.0, 118.0),
    };

    fn cage(width: f32) -> TorsoCage {
        TorsoCage {
            chest_back: (-width * 0.5, 86.0),
            chest_front: (width * 0.5, 86.0),
            waist_front: (width * 0.5 - 1.0, 66.0),
            waist_back: (-width * 0.5 + 1.0, 66.0),
        }
    }

    fn no_images<'a>(_: &str) -> Option<&'a RgbaImage> {
        None
    }

    fn job<'a>(
        d: &'a EquipmentDesign,
        map: SpaceMap,
        images: &'a dyn Fn(&str) -> Option<&'a RgbaImage>,
    ) -> PaintJob<'a> {
        PaintJob {
            design: d,
            map,
            cell: CELL,
            clip: "run",
            turn: 1.0,
            images,
        }
    }

    fn palette(d: &EquipmentDesign) -> Vec<[u8; 3]> {
        d.materials.iter().flat_map(|m| m.ramp).collect()
    }

    #[test]
    fn a_design_paints_binary_alpha_palette_texels_on_the_cage() {
        let d = sample();
        let p = paint(&job(&d, SpaceMap::Cage(cage(16.0)), &no_images));
        let pal = palette(&d);
        assert!(p.opaque() > 200);
        for px in p.rgba.iter().filter(|px| px[3] > 0) {
            assert_eq!(px[3], 255);
            assert!(pal.contains(&[px[0], px[1], px[2]]), "{px:?}");
        }
        // Silhouette edges are ink.
        let ink = d.materials[0].ramp[INK];
        let w = CELL.width;
        for (t, px) in p.rgba.iter().enumerate() {
            if px[3] == 0 {
                continue;
            }
            let edge = [t - 1, t + 1, t - w, t + w]
                .iter()
                .any(|n| p.rgba[*n][3] == 0);
            if edge {
                let inks = [ink, d.materials[1].ramp[INK]];
                assert!(inks.contains(&[px[0], px[1], px[2]]), "{px:?}");
            }
        }
    }

    #[test]
    fn the_same_source_follows_a_wider_torso() {
        let d = sample();
        let narrow = paint(&job(&d, SpaceMap::Cage(cage(12.0)), &no_images));
        let wide = paint(&job(&d, SpaceMap::Cage(cage(22.0)), &no_images));
        assert!(wide.opaque() > narrow.opaque() * 3 / 2);
    }

    #[test]
    fn the_topmost_part_owns_its_texels_and_hidden_parts_vanish() {
        let mut d = sample();
        let p = paint(&job(&d, SpaceMap::Cage(cage(16.0)), &no_images));
        assert!(p.owner.contains(&Some(1)), "collar on top");
        assert!(p.owner.contains(&Some(0)));
        d.parts[1].visible = false;
        let q = paint(&job(&d, SpaceMap::Cage(cage(16.0)), &no_images));
        assert!(!q.owner.contains(&Some(1)));
        d.parts[1].visible = true;
        d.parts[1].visibility = Visibility {
            clips: vec!["idle".into()],
            turn: None,
        };
        let r = paint(&job(&d, SpaceMap::Cage(cage(16.0)), &no_images));
        assert!(!r.owner.contains(&Some(1)), "idle-only part in the run");
    }

    #[test]
    fn moving_a_vertex_changes_the_silhouette() {
        let mut d = sample();
        let before = paint(&job(&d, SpaceMap::Cage(cage(16.0)), &no_images)).opaque();
        if let PartSource::Polygon { points, .. } = &mut d.parts[0].source {
            points[2] = (1.6, 0.5);
        }
        let after = paint(&job(&d, SpaceMap::Cage(cage(16.0)), &no_images)).opaque();
        assert!(after > before + 5, "{before} -> {after}");
    }

    #[test]
    fn an_image_part_keeps_its_pixels_in_a_frame() {
        let img = RgbaImage {
            width: 3,
            height: 5,
            pixels: (0..15).map(|i| [10 * i as u8, 0, 0, 255]).collect(),
        };
        let mut d = sample();
        d.space = DesignSpace::Hand {
            side: super::super::grip::HandSide::Near,
        };
        d.parts = vec![Part {
            id: "blade".into(),
            source: PartSource::Image {
                path: "x.png".into(),
                pivot: (1.0, 4.0),
            },
            transform: PartTransform::default(),
            material: "steel".into(),
            layer: 0,
            outline: Outline::None,
            bevel: 0.0,
            regions: Vec::new(),
            ornaments: Vec::new(),
            visible: true,
            visibility: Visibility::default(),
        }];
        let images = |p: &str| (p == "x.png").then_some(&img);
        // Frame on the texel centre (0.5, 50.5): identity rotation.
        let frame = Xform2::new(0.0, 50.0, 0.0);
        let p = paint(&job(&d, SpaceMap::Frame(frame), &images));
        assert_eq!(p.opaque(), 15, "every image pixel, unstretched");
        let colours: Vec<u8> = p
            .rgba
            .iter()
            .filter(|px| px[3] > 0)
            .map(|px| px[0])
            .collect();
        for c in (0..15).map(|i| 10 * i as u8) {
            assert!(colours.contains(&c));
        }
        assert_eq!(d.version, DESIGN_VERSION);
    }

    #[test]
    fn mount_transform_moves_the_whole_item() {
        let mut d = sample();
        let a = paint(&job(&d, SpaceMap::Cage(cage(16.0)), &no_images));
        d.mount.position = (0.0, 0.3);
        let b = paint(&job(&d, SpaceMap::Cage(cage(16.0)), &no_images));
        let top = |p: &Painted| p.rgba.iter().position(|px| px[3] > 0).unwrap_or(0) / CELL.width;
        assert!(top(&b) > top(&a), "the item moved down the torso");
    }

    #[test]
    fn line_texels_include_both_ends() {
        let l = line_texels((0.5, 0.5), (3.5, 0.5));
        assert_eq!(l.first(), Some(&(0, 0)));
        assert_eq!(l.last(), Some(&(3, 0)));
        assert_eq!(l.len(), 4);
    }
}
