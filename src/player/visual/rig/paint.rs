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
    Affine, BASE, DEEP, DesignSpace, EquipmentDesign, Finish, HIGHLIGHT, INK, LIGHT, Ornament,
    Outline, PaintMaterial, Part, PartSource, SHADOW, chaikin, pixel_index,
};
use super::flex::{ChainMap, FlexFrame, FlexInput, angle_of, settle};
use super::layers::{BodyRegion, CellGeometry, DepthSlot, point_in_polygon};
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
#[derive(Debug, Clone, PartialEq)]
pub enum SpaceMap {
    /// Torso cage `(u, v)`.
    Cage(TorsoCage),
    /// A hand or bone frame (texels, `+x` along it).
    Frame(Xform2),
    /// A flexible chain (texels along and across it, 05; shared, it is big).
    Chain(std::sync::Arc<ChainMap>),
}

impl SpaceMap {
    /// Rig point -> space coordinates.
    pub fn to_space(&self, p: RigPoint) -> Option<(f32, f32)> {
        match self {
            Self::Cage(c) => c.uv(p),
            Self::Frame(f) => Some(f.inverse().apply(p)),
            Self::Chain(c) => Some(c.to_space(p)),
        }
    }

    /// Space coordinates -> rig point.
    pub fn to_rig(&self, s: (f32, f32)) -> RigPoint {
        match self {
            Self::Cage(c) => c.point(s.0, s.1),
            Self::Frame(f) => f.apply(s),
            Self::Chain(c) => c.to_rig(s),
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
            Self::Frame(_) | Self::Chain(_) => (1.0, 1.0),
        }
    }
}

/// What to paint: a design on one pose (`'i`: the decoded images).
pub struct PaintJob<'a, 'i> {
    pub design: &'a EquipmentDesign,
    /// The space map of a design space on this pose (the design's own and
    /// any part binding, 04).
    pub maps: &'a dyn Fn(&DesignSpace) -> Option<SpaceMap>,
    pub cell: CellGeometry,
    pub clip: &'a str,
    /// Torso turn (0 side view, 1 three-quarter), see `EquipmentDesign::turn`.
    pub turn: f32,
    /// Decoded images by `PartSource::Image::path`.
    pub images: &'a dyn Fn(&str) -> Option<&'i RgbaImage>,
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

/// The space map and local <-> design map of a part.
struct PartFrame {
    map: SpaceMap,
    world: Affine,
    inverse: Affine,
}

fn part_frame(job: &PaintJob<'_, '_>, i: usize) -> Option<PartFrame> {
    let d = job.design;
    let map = (job.maps)(&d.effective_space(i))?;
    // local -> design (hierarchy) -> mount -> space.
    let world = d.mount.affine().then(&d.world_affine(i));
    Some(PartFrame {
        map,
        inverse: world.inverse()?,
        world,
    })
}

fn local_of(f: &PartFrame, p: RigPoint) -> Option<(f32, f32)> {
    Some(f.inverse.apply(f.map.to_space(p)?))
}

/// Local point -> cell texel coordinates (y down).
fn local_to_texel(f: &PartFrame, cell: CellGeometry, l: (f32, f32)) -> (f32, f32) {
    cell.to_texel(f.map.to_rig(f.world.apply(l)))
}

/// A pixel source: an imported PNG or a grid painted in the studio.
#[allow(clippy::type_complexity)]
fn pixel_source<'a, 'i: 'a>(
    job: &PaintJob<'a, 'i>,
    part: &'a Part,
) -> Option<(Box<dyn Fn(i64, i64) -> Option<[u8; 4]> + 'a>, (f32, f32))> {
    match &part.source {
        PartSource::Image { path, pivot } => {
            let img = (job.images)(path)?;
            Some((
                Box::new(move |x, y| img.get(x, y).filter(|p| p[3] > 0)),
                *pivot,
            ))
        }
        PartSource::Pixels {
            size,
            pivot,
            palette,
            rows,
        } => {
            let (w, h) = (size.0 as i64, size.1 as i64);
            let grid: Vec<Vec<char>> = rows.iter().map(|r| r.chars().collect()).collect();
            Some((
                Box::new(move |x, y| {
                    if x < 0 || y < 0 || x >= w || y >= h {
                        return None;
                    }
                    let c = *grid.get(y as usize)?.get(x as usize)?;
                    pixel_index(c)
                        .and_then(|i| palette.get(i).copied())
                        .filter(|p| p[3] > 0)
                }),
                *pivot,
            ))
        }
        _ => None,
    }
}

/// Which texels a part covers and how.
fn cover(job: &PaintJob<'_, '_>, i: usize) -> Vec<Option<Fill>> {
    let cell = job.cell;
    let n = cell.width * cell.height;
    let mut out = vec![None; n];
    let part = &job.design.parts[i];
    let Some(f) = part_frame(job, i) else {
        return out;
    };
    match &part.source {
        PartSource::Group => {}
        PartSource::Polygon { points, smooth } => {
            let poly = chaikin(points, *smooth);
            for (t, o) in out.iter_mut().enumerate() {
                let p = cell.texel_centre(t % cell.width, t / cell.width);
                if let Some(l) = local_of(&f, p)
                    && point_in_polygon(l, &poly)
                {
                    *o = Some(Fill::Shape(l));
                }
            }
        }
        PartSource::Image { .. } | PartSource::Pixels { .. } => {
            let Some((sample, pivot)) = pixel_source(job, part) else {
                return out;
            };
            match f.map {
                SpaceMap::Cage(_) => {
                    // A decal: pixel-aligned on the anchor, never stretched.
                    let anchor = local_to_texel(&f, cell, (0.0, 0.0));
                    let (ax, ay) = (anchor.0.floor() as i64, anchor.1.floor() as i64);
                    for (t, o) in out.iter_mut().enumerate() {
                        let (x, y) = ((t % cell.width) as i64, (t / cell.width) as i64);
                        if let Some(px) = sample(
                            x - ax + pivot.0.floor() as i64,
                            y - ay + pivot.1.floor() as i64,
                        ) {
                            *o = Some(Fill::Pixel(px));
                        }
                    }
                }
                SpaceMap::Frame(_) | SpaceMap::Chain(_) => {
                    for (t, o) in out.iter_mut().enumerate() {
                        let p = cell.texel_centre(t % cell.width, t / cell.width);
                        let Some(l) = local_of(&f, p) else {
                            continue;
                        };
                        let ix = (pivot.0 + l.0).floor() as i64;
                        let iy = (pivot.1 - l.1).floor() as i64;
                        if let Some(px) = sample(ix, iy) {
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
pub fn paint(job: &PaintJob<'_, '_>) -> Painted {
    let (w, h) = (job.cell.width, job.cell.height);
    let n = w * h;
    let d = job.design;
    // Visible parts in draw order (back to front).
    // A part shows when it and every ancestor are visible and allowed in
    // this clip and turn; groups have no texels of their own.
    let shown = |i: usize| {
        d.effective_visible(i)
            && std::iter::once(i)
                .chain(d.ancestors(i).unwrap_or_default())
                .all(|a| d.parts[a].visibility.allows(job.clip, job.turn))
            && !matches!(d.parts[i].source, PartSource::Group)
    };
    let mut order: Vec<usize> = (0..d.parts.len()).filter(|i| shown(*i)).collect();
    order.sort_by_key(|i| d.parts[*i].layer);
    let covers: Vec<Vec<Option<Fill>>> = order.iter().map(|i| cover(job, *i)).collect();
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
        let Some(frame) = part_frame(job, pi) else {
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
                    if let SpaceMap::Cage(_) = frame.map
                        && let Some(s) = frame.map.to_space(job.cell.texel_centre(t % w, t / w))
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
                    let (x, y) = local_to_texel(&frame, job.cell, *at);
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
                    material,
                } => {
                    let m = material.as_deref().and_then(mat_index).unwrap_or(own);
                    let tex: Vec<(f32, f32)> = points
                        .iter()
                        .map(|p| local_to_texel(&frame, job.cell, *p))
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
                                tones[t] = Some((m, (*tone as usize).clamp(INK, HIGHLIGHT)));
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
        if matches!(
            part.source,
            PartSource::Image { .. } | PartSource::Pixels { .. }
        ) && part.outline != Outline::None
        {
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

/// The space map of a design space on one played frame, and the grip
/// profile's depth for hand spaces. `None` when the clip, frame or grip is
/// missing.
pub fn space_map(
    rig: &super::data::RigLab,
    rest: &super::skeleton::LocalPose,
    space: &DesignSpace,
    clip: &str,
    frame: usize,
) -> Option<(SpaceMap, Option<DepthSlot>)> {
    use super::equipment::{Mount, place_on_bone};
    let c = rig.clips.get(clip)?;
    let joints = c.frame_joints(frame)?;
    let pose = super::skeleton::forward(&super::skeleton::fit(&joints));
    Some(match *space {
        DesignSpace::Torso => (SpaceMap::Cage(TorsoCage::from_joints(&joints)), None),
        DesignSpace::Hand { side } => {
            let grip = rig.grip(clip)?;
            let hand = super::grip::hand_frame(side, &joints, &pose, rest, grip);
            let depth = grip.depth.get(frame).copied();
            (SpaceMap::Frame(hand), depth)
        }
        DesignSpace::Bone { bone, at, follow } => {
            let mount = Mount::Bone {
                bone,
                at,
                offset: (0.0, 0.0),
                up_degrees: 90.0,
                follow,
            };
            let p = place_on_bone(&mount, DepthSlot::Torso, &pose, rest)?;
            (
                SpaceMap::Frame(Xform2::new(p.position.0, p.position.1, p.rotation)),
                None,
            )
        }
        // A chain needs the design's data and a simulation: `chain_maps`.
        DesignSpace::Flex { .. } => return None,
    })
}

/// Seconds-per-loop passes [`settle`] runs before reading a clip.
const SETTLE_LOOPS: usize = 6;

/// What a flexible chain receives on every played frame of a clip: the
/// fixation on the torso cage, the back line and the clip's ground speed.
pub fn flex_frames(
    rig: &super::data::RigLab,
    chain: &super::flex::FlexChain,
    clip: &str,
) -> Option<Vec<FlexFrame>> {
    let c = rig.clips.get(clip)?;
    (0..c.frames.len())
        .map(|f| {
            Some(FlexFrame {
                input: flex_input(rig, chain, clip, f)?,
                duration: c.frames[f].duration,
            })
        })
        .collect()
}

/// The input of a chain on one played frame.
pub fn flex_input(
    rig: &super::data::RigLab,
    chain: &super::flex::FlexChain,
    clip: &str,
    frame: usize,
) -> Option<FlexInput> {
    let c = rig.clips.get(clip)?;
    let cage = TorsoCage::from_joints(&c.frame_joints(frame)?);
    let (b, w) = (cage.chest_back, cage.waist_back);
    Some(FlexInput {
        anchor: cage.point(chain.anchor.0, chain.anchor.1),
        back_deg: angle_of((w.0 - b.0, w.1 - b.1)),
        ground_speed: c.ground_speed,
    })
}

/// Every chain of a design on one frame, in the steady loop of the clip
/// (the deterministic pose; the studio passes its live simulation instead).
pub fn settled_chains(
    rig: &super::data::RigLab,
    design: &EquipmentDesign,
    clip: &str,
    frame: usize,
) -> Vec<Option<Vec<RigPoint>>> {
    design
        .flex
        .iter()
        .map(|chain| {
            let frames = flex_frames(rig, chain, clip)?;
            settle(chain, &frames, SETTLE_LOOPS).get(frame).cloned()
        })
        .collect()
}

/// Where a design sits on one played frame of a clip: the map of its own
/// space, its default depth (a hand item takes the grip profile's per-frame
/// depth) and the torso turn.
pub fn pose_map(
    rig: &super::data::RigLab,
    rest: &super::skeleton::LocalPose,
    design: &EquipmentDesign,
    clip: &str,
    frame: usize,
) -> Option<(SpaceMap, DepthSlot, f32)> {
    let joints = rig.clips.get(clip)?.frame_joints(frame)?;
    let turn = design.turn(TorsoCage::from_joints(&joints).aspect());
    let own = match design.space {
        DesignSpace::Flex { .. } => DesignSpace::Torso,
        s => s,
    };
    let (map, grip_depth) = space_map(rig, rest, &own, clip, frame)?;
    Some((map, grip_depth.unwrap_or(design.depth), turn))
}

/// Paints a design on one frame of a clip, every part in its own space.
/// Returns the painted item and the design's default depth on that frame.
pub fn paint_on_frame<'i>(
    rig: &super::data::RigLab,
    rest: &super::skeleton::LocalPose,
    design: &EquipmentDesign,
    clip: &str,
    frame: usize,
    images: &dyn Fn(&str) -> Option<&'i RgbaImage>,
) -> Option<(Painted, DepthSlot)> {
    let chains = settled_chains(rig, design, clip, frame);
    paint_on_frame_with(rig, rest, design, clip, frame, images, &chains)
}

/// [`paint_on_frame`] with the chains given (one entry per
/// `EquipmentDesign::flex`, `None` = settled): the studio's live cloth.
pub fn paint_on_frame_with<'i>(
    rig: &super::data::RigLab,
    rest: &super::skeleton::LocalPose,
    design: &EquipmentDesign,
    clip: &str,
    frame: usize,
    images: &dyn Fn(&str) -> Option<&'i RgbaImage>,
    chains: &[Option<Vec<RigPoint>>],
) -> Option<(Painted, DepthSlot)> {
    let (_, depth, turn) = pose_map(rig, rest, design, clip, frame)?;
    let cell = rig.clips.get(clip)?.geometry();
    let settled = if chains.iter().any(Option::is_none) && !design.flex.is_empty() {
        settled_chains(rig, design, clip, frame)
    } else {
        Vec::new()
    };
    let chain_maps: Vec<Option<ChainMap>> = (0..design.flex.len())
        .map(|k| {
            let pts = chains
                .get(k)
                .cloned()
                .flatten()
                .or_else(|| settled.get(k).cloned().flatten())?;
            ChainMap::new(&pts)
        })
        .collect();
    let maps = |space: &DesignSpace| match space {
        DesignSpace::Flex { chain } => chain_maps
            .get(usize::from(*chain))
            .copied()
            .flatten()
            .map(|c| SpaceMap::Chain(std::sync::Arc::new(c))),
        _ => space_map(rig, rest, space, clip, frame).map(|(m, _)| m),
    };
    let job = PaintJob {
        design,
        maps: &maps,
        cell,
        clip,
        turn,
        images,
    };
    Some((paint(&job), depth))
}

/// Splits a painted item by the depth of the part that owns each texel
/// (part depth, else an ancestor's, else `default`).
pub fn split_by_depth(
    design: &EquipmentDesign,
    painted: &Painted,
    default: DepthSlot,
) -> Vec<(DepthSlot, Vec<[u8; 4]>)> {
    let mut out: Vec<(DepthSlot, Vec<[u8; 4]>)> = Vec::new();
    for (t, o) in painted.owner.iter().enumerate() {
        let Some(i) = *o else {
            continue;
        };
        let depth = design.effective_depth(i, default);
        let k = match out.iter().position(|(d, _)| *d == depth) {
            Some(k) => k,
            None => {
                out.push((depth, vec![[0; 4]; painted.rgba.len()]));
                out.len() - 1
            }
        };
        out[k].1[t] = painted.rgba[t];
    }
    out.sort_by_key(|(d, _)| *d);
    out
}

/// Texels of `region` hidden by a painted item: under the footprint
/// (dilated one texel) of every part whose `hides` names the region.
pub fn hide_mask(
    design: &EquipmentDesign,
    painted: &Painted,
    width: usize,
    region: BodyRegion,
) -> Vec<bool> {
    let n = painted.owner.len();
    let core: Vec<bool> = painted
        .owner
        .iter()
        .map(|o| o.is_some_and(|i| design.parts[i].hides.contains(&region)))
        .collect();
    let mut out = core.clone();
    for (t, _) in core.iter().enumerate().filter(|(_, c)| **c) {
        let (x, y) = (t % width, t / width);
        for (nx, ny) in [
            (x.wrapping_sub(1), y),
            (x + 1, y),
            (x, y.wrapping_sub(1)),
            (x, y + 1),
        ] {
            if nx < width && ny * width + nx < n {
                out[ny * width + nx] = true;
            }
        }
    }
    out
}

/// One layer of the final composite: its depth, a sub-order and its texels.
pub struct CompositeLayer<'a> {
    pub depth: DepthSlot,
    pub sub: u8,
    pub texels: &'a [[u8; 4]],
}

/// The final image: every layer over the ones behind it (depth, then
/// sub-order). The editor shows this one image at every zoom, so 1x, 2x,
/// 4x and 8x are the same composition.
pub fn compose(len: usize, layers: &mut [CompositeLayer<'_>]) -> Vec<[u8; 4]> {
    layers.sort_by_key(|l| (l.depth, l.sub));
    let mut out = vec![[0u8; 4]; len];
    for l in layers.iter() {
        for (o, t) in out.iter_mut().zip(l.texels) {
            if t[3] > 0 {
                *o = *t;
            }
        }
    }
    out
}

/// The cell of a pixel part under a rig point (the brush of the studio),
/// with the same mapping the painter draws it with; `None` off the grid.
pub fn pixel_cell_at(
    design: &EquipmentDesign,
    i: usize,
    map: &SpaceMap,
    cell: CellGeometry,
    p: RigPoint,
) -> Option<(usize, usize)> {
    let PartSource::Pixels { size, pivot, .. } = &design.parts.get(i)?.source else {
        return None;
    };
    let world = design.mount.affine().then(&design.world_affine(i));
    let (x, y) = match map {
        SpaceMap::Cage(_) => {
            let anchor = cell.to_texel(map.to_rig(world.apply((0.0, 0.0))));
            let t = cell.to_texel(p);
            (
                t.0.floor() as i64 - anchor.0.floor() as i64 + pivot.0.floor() as i64,
                t.1.floor() as i64 - anchor.1.floor() as i64 + pivot.1.floor() as i64,
            )
        }
        SpaceMap::Frame(_) | SpaceMap::Chain(_) => {
            let l = world.inverse()?.apply(map.to_space(p)?);
            (
                (pivot.0 + l.0).floor() as i64,
                (pivot.1 - l.1).floor() as i64,
            )
        }
    };
    (x >= 0 && y >= 0 && (x as u32) < size.0 && (y as u32) < size.1)
        .then_some((x as usize, y as usize))
}

/// One worn item on a frame: the design, what the painter drew and its
/// default depth on that frame.
pub struct WornItem<'a> {
    pub design: &'a EquipmentDesign,
    pub painted: &'a Painted,
    pub depth: DepthSlot,
}

/// The final frame (04): the body regions (RGBA8 bytes, cell-sized; the
/// upper ones lowered by `drop` texels for the idle breath) with the texels
/// hidden by the worn items removed, and every item part at its own depth.
/// The same pure function feeds every view of the studio and the tests, so
/// 1x, 2x, 4x and 8x show one composition.
pub fn compose_frame(
    body: &[(BodyRegion, Vec<u8>)],
    drop: usize,
    width: usize,
    items: &[WornItem<'_>],
) -> Vec<[u8; 4]> {
    let len = body.first().map_or(0, |(_, b)| b.len() / 4);
    let mut regions: Vec<(BodyRegion, Vec<[u8; 4]>)> = Vec::new();
    for (region, bytes) in body {
        let mut px: Vec<[u8; 4]> = bytes.as_chunks::<4>().0.to_vec();
        if region.is_upper_body() && drop > 0 {
            let shift = drop * width;
            let mut moved = vec![[0u8; 4]; len];
            let keep = len.saturating_sub(shift);
            moved[shift.min(len)..shift.min(len) + keep].copy_from_slice(&px[..keep]);
            px = moved;
        }
        for item in items {
            let mask = hide_mask(item.design, item.painted, width, *region);
            for (p, m) in px.iter_mut().zip(&mask) {
                if *m {
                    *p = [0; 4];
                }
            }
        }
        regions.push((*region, px));
    }
    let gear: Vec<(DepthSlot, Vec<[u8; 4]>)> = items
        .iter()
        .flat_map(|i| split_by_depth(i.design, i.painted, i.depth))
        .collect();
    let mut layers: Vec<CompositeLayer<'_>> = regions
        .iter()
        .map(|(r, px)| CompositeLayer {
            depth: r.depth(),
            sub: 0,
            texels: px,
        })
        .chain(gear.iter().map(|(d, px)| CompositeLayer {
            depth: *d,
            sub: 1,
            texels: px,
        }))
        .collect();
    compose(len, &mut layers)
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

    fn paint_at<'i>(
        d: &EquipmentDesign,
        map: SpaceMap,
        images: &dyn Fn(&str) -> Option<&'i RgbaImage>,
    ) -> Painted {
        let maps = move |_: &DesignSpace| Some(map.clone());
        paint(&PaintJob {
            design: d,
            maps: &maps,
            cell: CELL,
            clip: "run",
            turn: 1.0,
            images,
        })
    }

    fn palette(d: &EquipmentDesign) -> Vec<[u8; 3]> {
        d.materials.iter().flat_map(|m| m.ramp).collect()
    }

    #[test]
    fn a_design_paints_binary_alpha_palette_texels_on_the_cage() {
        let d = sample();
        let p = paint_at(&d, SpaceMap::Cage(cage(16.0)), &no_images);
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
        let narrow = paint_at(&d, SpaceMap::Cage(cage(12.0)), &no_images);
        let wide = paint_at(&d, SpaceMap::Cage(cage(22.0)), &no_images);
        assert!(wide.opaque() > narrow.opaque() * 3 / 2);
    }

    #[test]
    fn the_topmost_part_owns_its_texels_and_hidden_parts_vanish() {
        let mut d = sample();
        let p = paint_at(&d, SpaceMap::Cage(cage(16.0)), &no_images);
        assert!(p.owner.contains(&Some(1)), "collar on top");
        assert!(p.owner.contains(&Some(0)));
        d.parts[1].visible = false;
        let q = paint_at(&d, SpaceMap::Cage(cage(16.0)), &no_images);
        assert!(!q.owner.contains(&Some(1)));
        d.parts[1].visible = true;
        d.parts[1].visibility = Visibility {
            clips: vec!["idle".into()],
            turn: None,
        };
        let r = paint_at(&d, SpaceMap::Cage(cage(16.0)), &no_images);
        assert!(!r.owner.contains(&Some(1)), "idle-only part in the run");
    }

    #[test]
    fn moving_a_vertex_changes_the_silhouette() {
        let mut d = sample();
        let before = paint_at(&d, SpaceMap::Cage(cage(16.0)), &no_images).opaque();
        if let PartSource::Polygon { points, .. } = &mut d.parts[0].source {
            points[2] = (1.6, 0.5);
        }
        let after = paint_at(&d, SpaceMap::Cage(cage(16.0)), &no_images).opaque();
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
            parent: None,
            bind: None,
            depth: None,
            hides: Vec::new(),
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
        let p = paint_at(&d, SpaceMap::Frame(frame), &images);
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
        let a = paint_at(&d, SpaceMap::Cage(cage(16.0)), &no_images);
        d.mount.position = (0.0, 0.3);
        let b = paint_at(&d, SpaceMap::Cage(cage(16.0)), &no_images);
        let top = |p: &Painted| p.rgba.iter().position(|px| px[3] > 0).unwrap_or(0) / CELL.width;
        assert!(top(&b) > top(&a), "the item moved down the torso");
    }

    #[test]
    fn a_child_turns_with_its_parent_and_depths_split() {
        let mut d = sample();
        d.parts[1].parent = Some("body".into());
        d.parts[1].depth = Some(DepthSlot::ShoulderGear);
        let a = paint_at(&d, SpaceMap::Cage(cage(16.0)), &no_images);
        let parts = split_by_depth(&d, &a, DepthSlot::ChestGear);
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0].0, DepthSlot::ChestGear);
        assert_eq!(parts[1].0, DepthSlot::ShoulderGear);
        // Moving the parent moves the child's texels.
        let collar = |p: &Painted| p.owner.iter().position(|o| *o == Some(1));
        d.parts[0].transform.position = (0.0, 0.25);
        let b = paint_at(&d, SpaceMap::Cage(cage(16.0)), &no_images);
        assert_ne!(collar(&a), collar(&b));
        // Hiding the parent hides the child.
        d.parts[0].visible = false;
        let c = paint_at(&d, SpaceMap::Cage(cage(16.0)), &no_images);
        assert_eq!(c.opaque(), 0);
    }

    #[test]
    fn pixel_parts_draw_their_grid_and_hide_masks_dilate() {
        let mut d = sample();
        d.parts[1].source = PartSource::Pixels {
            size: (3, 2),
            pivot: (1.0, 1.0),
            palette: vec![[200, 10, 10, 255], [10, 200, 10, 255]],
            rows: vec!["010".into(), "1.1".into()],
        };
        d.parts[1].hides = vec![BodyRegion::NearUpperArm];
        d.parts[1].outline = Outline::None;
        let p = paint_at(&d, SpaceMap::Cage(cage(16.0)), &no_images);
        let reds = p
            .rgba
            .iter()
            .filter(|px| **px == [200, 10, 10, 255])
            .count();
        let greens = p
            .rgba
            .iter()
            .filter(|px| **px == [10, 200, 10, 255])
            .count();
        assert_eq!((reds, greens), (2, 3));
        let mask = hide_mask(&d, &p, CELL.width, BodyRegion::NearUpperArm);
        let core = p.owner.iter().filter(|o| **o == Some(1)).count();
        assert!(mask.iter().filter(|m| **m).count() > core);
        assert!(
            !hide_mask(&d, &p, CELL.width, BodyRegion::Hair)
                .iter()
                .any(|m| *m)
        );
    }

    #[test]
    fn the_brush_cell_matches_the_painted_texel() {
        let mut d = sample();
        d.parts[1].source = PartSource::Pixels {
            size: (3, 2),
            pivot: (1.0, 1.0),
            palette: vec![[200, 10, 10, 255]],
            rows: vec!["0..".into(), "...".into()],
        };
        d.parts[1].outline = Outline::None;
        let map = SpaceMap::Cage(cage(16.0));
        let p = paint_at(&d, map.clone(), &no_images);
        let t = p
            .rgba
            .iter()
            .position(|px| *px == [200, 10, 10, 255])
            .unwrap_or_else(|| panic!("red"));
        let centre = CELL.texel_centre(t % CELL.width, t / CELL.width);
        assert_eq!(pixel_cell_at(&d, 1, &map, CELL, centre), Some((0, 0)));
    }

    #[test]
    fn compose_stacks_by_depth() {
        let back = vec![[1, 1, 1, 255]; 2];
        let front = vec![[9, 9, 9, 255], [0, 0, 0, 0]];
        let mut layers = [
            CompositeLayer {
                depth: DepthSlot::Hair,
                sub: 0,
                texels: &front,
            },
            CompositeLayer {
                depth: DepthSlot::Torso,
                sub: 0,
                texels: &back,
            },
        ];
        assert_eq!(
            compose(2, &mut layers),
            vec![[9, 9, 9, 255], [1, 1, 1, 255]]
        );
    }

    #[test]
    fn line_texels_include_both_ends() {
        let l = line_texels((0.5, 0.5), (3.5, 0.5));
        assert_eq!(l.first(), Some(&(0, 0)));
        assert_eq!(l.last(), Some(&(3, 0)));
        assert_eq!(l.len(), 4);
    }
}
