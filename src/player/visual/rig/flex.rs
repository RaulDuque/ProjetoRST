//! Flexible equipment (Modular Character Engine 05): a chain of control
//! points hung from the torso, simulated per frame. Pure module.
//!
//! A [`FlexChain`] is data (in the design's RON): where it is fixed on the
//! torso cage, how many segments, how long, how stiff, how damped, how much
//! the body's motion drags it and how far it may swing. [`FlexState`] is the
//! simulation, in the rig space of the character (`x` towards the face, `y`
//! up): every segment is a damped pendulum around a target angle, coupled to
//! the segment above. Every cape, cloak, tabard or ribbon is the same code
//! with other numbers.
//!
//! What moves each segment:
//! - gravity (the pendulum's frequency, slower for the lower segments, so
//!   the hem lags the shoulders);
//! - the air of the body's ground speed (`influence`): it lifts the target
//!   angle behind, less near the fixation (`shelter`), with a ripple running
//!   down (`flutter`, `flutter_hz`);
//! - the body's acceleration (a start throws the cloth back, a stop swings
//!   it forward) and the jumps of the fixation between poses;
//! - a spring towards the target (`stiffness`) and damping (`damping`).
//!
//! An earlier particle version (Verlet with follow-the-leader constraints)
//! flapped chaotically whenever the fixation jumped between poses; the
//! angle form is stable with the same data.
//!
//! Angles are measured from straight down, positive towards the back
//! (`-x`). No segment may pass forward of the back line of the torso.
//!
//! [`settle`] runs a clip loop until the motion repeats and returns the
//! chain on every played frame: the deterministic pose the painter uses
//! when no live simulation is given.

use serde::{Deserialize, Serialize};

use super::xform::RigPoint;

/// Longest chain accepted (segments).
pub const MAX_SEGMENTS: usize = 16;
/// Simulation step (seconds); longer steps are split.
pub const FLEX_STEP: f32 = 1.0 / 120.0;

fn default_damping() -> f32 {
    0.92
}

fn default_gravity() -> f32 {
    260.0
}

/// A chain of control points: the data of one flexible piece.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlexChain {
    pub id: String,
    /// Fixation point on the torso cage `(u, v)`.
    pub anchor: (f32, f32),
    /// Number of segments (control points = segments + 1).
    pub segments: u8,
    /// Total length in texels.
    pub length: f32,
    /// Pull towards the rest angle per second (0 = free cloth).
    #[serde(default)]
    pub stiffness: f32,
    /// Velocity kept per 1/60 s (1 = no damping).
    #[serde(default = "default_damping")]
    pub damping: f32,
    /// How strongly the air of the body's motion drags the chain.
    #[serde(default)]
    pub influence: f32,
    /// Texels per second squared.
    #[serde(default = "default_gravity")]
    pub gravity: f32,
    /// Rest angle of every segment (degrees from down, + towards the back).
    #[serde(default)]
    pub rest_deg: f32,
    /// Absolute angle range of every segment (degrees, see above).
    pub limits: (f32, f32),
    /// Largest bend between two neighbouring segments (degrees).
    pub bend: f32,
    /// Ripple of the drag along the chain (0 = none, 1 = the drag doubles
    /// and vanishes): a wave running from the fixation to the hem.
    #[serde(default)]
    pub flutter: f32,
    /// Ripples per second.
    #[serde(default = "default_flutter_hz")]
    pub flutter_hz: f32,
    /// Fraction of the chain (from the fixation) the body shelters from the
    /// air: the drag ramps from 0 to full over it, so the cloth falls close
    /// to the back and flares further down.
    #[serde(default)]
    pub shelter: f32,
}

fn default_flutter_hz() -> f32 {
    2.5
}

/// Problems of a chain's data.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum FlexError {
    #[error("chain {0}: segments must be 1..={MAX_SEGMENTS}")]
    Segments(String),
    #[error("chain {0}: length, gravity and bend must be positive")]
    Positive(String),
    #[error(
        "chain {0}: damping and flutter must be in 0..=1; stiffness, influence, flutter_hz >= 0"
    )]
    Range(String),
    #[error("chain {0}: limits must be ordered and contain the rest angle")]
    Limits(String),
}

impl FlexChain {
    pub fn validate(&self) -> Result<(), FlexError> {
        let id = || self.id.clone();
        if self.segments == 0 || self.segments as usize > MAX_SEGMENTS {
            return Err(FlexError::Segments(id()));
        }
        if self.length <= 0.0 || self.gravity < 0.0 || self.bend <= 0.0 {
            return Err(FlexError::Positive(id()));
        }
        if !(0.0..=1.0).contains(&self.damping)
            || self.stiffness < 0.0
            || self.influence < 0.0
            || !(0.0..=1.0).contains(&self.flutter)
            || !(0.0..1.0).contains(&self.shelter)
            || self.flutter_hz < 0.0
        {
            return Err(FlexError::Range(id()));
        }
        let (lo, hi) = self.limits;
        if lo > hi || self.rest_deg < lo || self.rest_deg > hi {
            return Err(FlexError::Limits(id()));
        }
        Ok(())
    }

    fn segment_length(&self) -> f32 {
        self.length / f32::from(self.segments.max(1))
    }
}

/// Direction of an angle (degrees from down, + towards the back).
pub fn direction(deg: f32) -> (f32, f32) {
    let (s, c) = deg.to_radians().sin_cos();
    (-s, -c)
}

/// Angle of a direction (inverse of [`direction`]).
pub fn angle_of(d: (f32, f32)) -> f32 {
    (-d.0).atan2(-d.1).to_degrees()
}

/// What moves the chain on one step.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FlexInput {
    /// The fixation point this step (rig space).
    pub anchor: RigPoint,
    /// Angle of the torso's back line (shoulder -> waist), same convention:
    /// no segment may swing forward of it.
    pub back_deg: f32,
    /// The body's ground speed towards the face (texels per second).
    pub ground_speed: f32,
}

/// The live chain: one angle (and angular speed) per segment, the points
/// they give and the body speed of the last step.
#[derive(Debug, Clone, PartialEq)]
pub struct FlexState {
    pub points: Vec<RigPoint>,
    angles: Vec<f32>,
    spin: Vec<f32>,
    anchor: Option<RigPoint>,
    speed: f32,
    time: f32,
}

impl FlexState {
    /// The chain hanging at its rest angle from `anchor`.
    pub fn rest(spec: &FlexChain, anchor: RigPoint) -> Self {
        let n = usize::from(spec.segments.clamp(1, MAX_SEGMENTS as u8));
        let mut s = Self {
            points: Vec::new(),
            angles: vec![spec.rest_deg; n],
            spin: vec![0.0; n],
            anchor: Some(anchor),
            speed: 0.0,
            time: 0.0,
        };
        s.build(spec, anchor);
        s
    }

    /// A chain already in a known shape (a settled pose), moving with the
    /// body at `speed`: an item put on mid-run starts in its running shape.
    pub fn from_points(spec: &FlexChain, points: &[RigPoint], speed: f32) -> Self {
        let Some(&anchor) = points.first() else {
            return Self::rest(spec, (0.0, 0.0));
        };
        let mut s = Self::rest(spec, anchor);
        if points.len() == s.angles.len() + 1 {
            for (a, w) in s.angles.iter_mut().zip(points.windows(2)) {
                *a = angle_of((w[1].0 - w[0].0, w[1].1 - w[0].1));
            }
            s.build(spec, anchor);
        }
        s.speed = speed;
        s
    }

    fn build(&mut self, spec: &FlexChain, anchor: RigPoint) {
        let seg = spec.segment_length();
        let mut p = anchor;
        self.points.clear();
        self.points.push(p);
        for a in &self.angles {
            let d = direction(*a);
            p = (p.0 + d.0 * seg, p.1 + d.1 * seg);
            self.points.push(p);
        }
    }

    /// Angle every segment tends to: the rest angle, plus the lift of the
    /// air (sheltered near the fixation), plus a ripple running down.
    fn target(&self, spec: &FlexChain, i: usize, speed: f32, lo: f32) -> f32 {
        let n = self.angles.len() as f32;
        let reach = (i as f32 + 0.5) / n;
        let open = if spec.shelter > 0.0 {
            (reach / spec.shelter).min(1.0)
        } else {
            1.0
        };
        // Signed: air from behind (just after a turn) lifts nothing.
        let lift = (spec.influence * speed)
            .atan2(spec.gravity.max(1.0))
            .to_degrees();
        (spec.rest_deg + lift * open).clamp(lo, spec.limits.1.max(lo))
    }

    /// Angular acceleration of the ripple on segment `i`: a forced wave
    /// running down the chain, as strong as the air lifts it (its steady
    /// swing is about `flutter * lift / 2 * reach` degrees).
    fn ripple(&self, spec: &FlexChain, i: usize, speed: f32) -> f32 {
        let n = self.angles.len() as f32;
        let reach = (i as f32 + 0.5) / n;
        let lift = (spec.influence * speed.abs())
            .atan2(spec.gravity.max(1.0))
            .to_degrees();
        let w = std::f32::consts::TAU * spec.flutter_hz;
        let phase = w * self.time - i as f32 * 0.9;
        spec.flutter * lift * 0.5 * reach * w * w * phase.sin()
    }

    /// Advances `dt` seconds (0 only follows a moved anchor). Long steps are
    /// split into [`FLEX_STEP`] sub-steps.
    pub fn step(&mut self, spec: &FlexChain, input: FlexInput, dt: f32) {
        let n = usize::from(spec.segments.clamp(1, MAX_SEGMENTS as u8));
        if self.angles.len() != n {
            *self = Self::rest(spec, input.anchor);
        }
        let seg = spec.segment_length();
        let lo = spec.limits.0.max(input.back_deg);
        let hi = spec.limits.1.max(lo);
        // A moved fixation leaves the cloth where it was for an instant: the
        // angles change as a pendulum's do when its pivot jumps.
        if let Some(old) = self.anchor {
            let (dx, dy) = (input.anchor.0 - old.0, input.anchor.1 - old.1);
            for (i, a) in self.angles.iter_mut().enumerate() {
                let arm = seg * (i as f32 + 1.0);
                let r = a.to_radians();
                // Tangent of growing angle: (-cos, +sin) of the direction.
                let push = (dx * r.cos() - dy * r.sin()) / arm;
                *a += push.to_degrees() * 0.5;
            }
        }
        self.anchor = Some(input.anchor);
        if dt > 0.0 {
            let steps = (dt / FLEX_STEP).ceil().max(1.0) as usize;
            let h = dt / steps as f32;
            // Body acceleration (forward) swings the cloth back.
            let accel = (input.ground_speed - self.speed) / dt;
            let decay = -spec.damping.clamp(1e-4, 1.0).ln() * 60.0;
            for _ in 0..steps {
                self.time += h;
                for i in 0..n {
                    let arm = seg * (i as f32 + 0.5) + seg;
                    let omega2 = spec.gravity / arm + spec.stiffness * 60.0;
                    let target = self.target(spec, i, input.ground_speed, lo);
                    let r = self.angles[i].to_radians();
                    let mut acc = omega2 * (target - self.angles[i]) - decay * self.spin[i]
                        + self.ripple(spec, i, input.ground_speed);
                    acc += (accel * r.cos() / arm).to_degrees();
                    // The segment above drags this one (a chain, not rods).
                    if i > 0 {
                        acc += omega2 * 0.5 * (self.angles[i - 1] - self.angles[i]);
                    }
                    // Forward of the back line only while turning: pushed back.
                    if self.angles[i] < lo {
                        acc += omega2 * 0.8 * (lo - self.angles[i]);
                    }
                    self.spin[i] += acc * h;
                    self.angles[i] += self.spin[i] * h;
                }
            }
            self.speed = input.ground_speed;
        }
        // Limits: never past the far limit, never more than `bend` apart.
        for i in 0..n {
            if self.angles[i] > hi {
                self.angles[i] = hi;
                self.spin[i] = self.spin[i].min(0.0);
            }
            if self.angles[i] < -hi {
                self.angles[i] = -hi;
                self.spin[i] = self.spin[i].max(0.0);
            }
            if i > 0 {
                let p = self.angles[i - 1];
                self.angles[i] = self.angles[i].clamp(p - spec.bend, p + spec.bend);
            }
        }
        self.build(spec, input.anchor);
    }

    /// The body turned around: the cloth stays where it was in the world,
    /// so in the (mirrored) rig space it flips forward, then swings back
    /// behind the body.
    pub fn turn_around(&mut self) {
        for a in &mut self.angles {
            *a = -*a;
        }
        for w in &mut self.spin {
            *w = -*w;
        }
        if let Some(&(ax, _)) = self.points.first() {
            for p in &mut self.points {
                p.0 = 2.0 * ax - p.0;
            }
        }
        self.speed = -self.speed;
    }
}

/// One played frame for [`settle`]: the input of the frame and how long it
/// lasts.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FlexFrame {
    pub input: FlexInput,
    pub duration: f32,
}

/// Runs the loop until it repeats (`loops` passes) and returns the chain
/// at the start of every frame of the last pass.
pub fn settle(spec: &FlexChain, frames: &[FlexFrame], loops: usize) -> Vec<Vec<RigPoint>> {
    let Some(first) = frames.first() else {
        return Vec::new();
    };
    let mut s = FlexState::rest(spec, first.input.anchor);
    s.speed = first.input.ground_speed;
    let mut out = Vec::new();
    for pass in 0..loops.max(1) {
        for f in frames {
            s.step(spec, f.input, 0.0);
            if pass + 1 == loops.max(1) {
                out.push(s.points.clone());
            }
            s.step(spec, f.input, f.duration);
        }
    }
    out
}

/// Points of a [`ChainMap`] after smoothing.
pub const MAP_POINTS: usize = 80;

/// The space of a chain on one frame, for the painter: `x` texels along
/// the (smoothed) chain from the fixation, `y` texels across it (+ towards
/// the body's front). Past the ends the first and last segments extend.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChainMap {
    pts: [RigPoint; MAP_POINTS],
    cum: [f32; MAP_POINTS],
    n: usize,
}

/// Open Chaikin passes that keep both ends.
fn smooth_open(points: &[RigPoint], passes: usize) -> Vec<RigPoint> {
    let mut v = points.to_vec();
    for _ in 0..passes {
        if v.len() < 3 || v.len() * 2 > MAP_POINTS {
            break;
        }
        let mut out = vec![v[0]];
        for w in v.windows(2) {
            let (a, b) = (w[0], w[1]);
            out.push((a.0 * 0.75 + b.0 * 0.25, a.1 * 0.75 + b.1 * 0.25));
            out.push((a.0 * 0.25 + b.0 * 0.75, a.1 * 0.25 + b.1 * 0.75));
        }
        out.push(v[v.len() - 1]);
        v = out;
    }
    v
}

impl ChainMap {
    /// The map of a chain's control points (at least two).
    pub fn new(points: &[RigPoint]) -> Option<Self> {
        if points.len() < 2 {
            return None;
        }
        let smooth = smooth_open(points, 2);
        let n = smooth.len().min(MAP_POINTS);
        let mut pts = [(0.0, 0.0); MAP_POINTS];
        let mut cum = [0.0; MAP_POINTS];
        for i in 0..n {
            pts[i] = smooth[i];
            if i > 0 {
                let (a, b) = (pts[i - 1], pts[i]);
                cum[i] = cum[i - 1] + (b.0 - a.0).hypot(b.1 - a.1);
            }
        }
        Some(Self { pts, cum, n })
    }

    /// The smoothed curve.
    pub fn points(&self) -> &[RigPoint] {
        &self.pts[..self.n]
    }

    /// Length of the smoothed curve.
    pub fn length(&self) -> f32 {
        self.cum[self.n - 1]
    }

    /// Rig point -> (along, across).
    pub fn to_space(&self, p: RigPoint) -> (f32, f32) {
        let mut best = (f32::MAX, 0.0, 0.0);
        let last = self.n - 2;
        for i in 0..=last {
            let (a, b) = (self.pts[i], self.pts[i + 1]);
            let d = (b.0 - a.0, b.1 - a.1);
            let len2 = (d.0 * d.0 + d.1 * d.1).max(1e-9);
            let v = (p.0 - a.0, p.1 - a.1);
            let mut t = (v.0 * d.0 + v.1 * d.1) / len2;
            if i > 0 {
                t = t.max(0.0);
            }
            if i < last {
                t = t.min(1.0);
            }
            let q = (a.0 + d.0 * t, a.1 + d.1 * t);
            let dist = (p.0 - q.0).hypot(p.1 - q.1);
            if dist < best.0 {
                let len = len2.sqrt();
                let across = (d.0 * v.1 - d.1 * v.0) / len;
                best = (dist, self.cum[i] + t * len, across);
            }
        }
        (best.1, best.2)
    }

    /// (along, across) -> rig point.
    pub fn to_rig(&self, s: (f32, f32)) -> RigPoint {
        let last = self.n - 2;
        let mut i = 0;
        while i < last && self.cum[i + 1] < s.0 {
            i += 1;
        }
        let (a, b) = (self.pts[i], self.pts[i + 1]);
        let d = (b.0 - a.0, b.1 - a.1);
        let len = d.0.hypot(d.1).max(1e-6);
        let t = (s.0 - self.cum[i]) / len;
        let left = (-d.1 / len, d.0 / len);
        (a.0 + d.0 * t + left.0 * s.1, a.1 + d.1 * t + left.1 * s.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cape() -> FlexChain {
        FlexChain {
            id: "cape".into(),
            anchor: (0.1, 0.05),
            segments: 6,
            length: 36.0,
            stiffness: 0.05,
            damping: 0.92,
            influence: 4.0,
            gravity: 260.0,
            rest_deg: 8.0,
            limits: (0.0, 100.0),
            bend: 35.0,
            flutter: 0.0,
            flutter_hz: 2.5,
            shelter: 0.0,
        }
    }

    fn input(speed: f32) -> FlexInput {
        FlexInput {
            anchor: (0.0, 60.0),
            back_deg: 0.0,
            ground_speed: speed,
        }
    }

    fn run(s: &mut FlexState, spec: &FlexChain, speed: f32, seconds: f32) {
        let n = (seconds * 60.0) as usize;
        for _ in 0..n {
            s.step(spec, input(speed), 1.0 / 60.0);
        }
    }

    fn hem_angle(s: &FlexState) -> f32 {
        let a = s.points[0];
        let b = s.points[s.points.len() - 1];
        angle_of((b.0 - a.0, b.1 - a.1))
    }

    #[test]
    fn the_chain_keeps_its_length_and_limits() {
        let spec = cape();
        let mut s = FlexState::rest(&spec, (0.0, 60.0));
        run(&mut s, &spec, 300.0, 2.0);
        let seg = spec.length / 6.0;
        for w in s.points.windows(2) {
            let d = (w[1].0 - w[0].0).hypot(w[1].1 - w[0].1);
            assert!((d - seg).abs() < 1e-3, "segment {d}");
            let a = angle_of((w[1].0 - w[0].0, w[1].1 - w[0].1));
            assert!((-1e-3..=100.001).contains(&a), "angle {a}");
        }
        assert_eq!(s.points[0], (0.0, 60.0));
    }

    #[test]
    fn standing_it_hangs_and_running_it_lifts_behind() {
        let spec = cape();
        let mut s = FlexState::rest(&spec, (0.0, 60.0));
        run(&mut s, &spec, 0.0, 2.0);
        let hanging = hem_angle(&s);
        assert!(hanging < 15.0, "hanging at {hanging}");
        run(&mut s, &spec, 110.0, 2.0);
        let running = hem_angle(&s);
        assert!(running > hanging + 15.0, "{hanging} -> {running}");
        // Behind the body: every point at or behind the anchor.
        assert!(s.points.iter().all(|p| p.0 <= 1e-3));
    }

    #[test]
    fn a_stop_swings_forward_then_settles() {
        let spec = cape();
        let mut s = FlexState::rest(&spec, (0.0, 60.0));
        run(&mut s, &spec, 110.0, 2.0);
        let running = hem_angle(&s);
        let mut lowest = running;
        for _ in 0..30 {
            s.step(&spec, input(0.0), 1.0 / 60.0);
            lowest = lowest.min(hem_angle(&s));
        }
        // It lags (does not snap down in one frame) and falls.
        let first = {
            let mut t = FlexState::rest(&spec, (0.0, 60.0));
            run(&mut t, &spec, 110.0, 2.0);
            t.step(&spec, input(0.0), 1.0 / 60.0);
            hem_angle(&t)
        };
        assert!(first > running - 15.0, "snapped: {running} -> {first}");
        assert!(lowest < running - 20.0, "never fell: {running} -> {lowest}");
        run(&mut s, &spec, 0.0, 3.0);
        assert!(hem_angle(&s) < 15.0);
    }

    #[test]
    fn turning_around_flips_then_returns_behind() {
        let spec = cape();
        let mut s = FlexState::rest(&spec, (0.0, 60.0));
        run(&mut s, &spec, 110.0, 2.0);
        s.turn_around();
        // Right after the turn the cloth is in front (it stayed in the world).
        assert!(s.points.last().is_some_and(|p| p.0 > 0.0) || hem_angle(&s) <= 0.5);
        run(&mut s, &spec, 110.0, 2.0);
        assert!(hem_angle(&s) > 15.0);
        assert!(s.points.iter().all(|p| p.0 <= 1e-3));
    }

    #[test]
    fn settle_is_periodic_and_deterministic() {
        let spec = cape();
        let frames: Vec<FlexFrame> = (0..6)
            .map(|i| FlexFrame {
                input: FlexInput {
                    anchor: (0.0, 60.0 + (i % 3) as f32),
                    back_deg: 0.0,
                    ground_speed: 110.0,
                },
                duration: 1.0 / 15.0,
            })
            .collect();
        let a = settle(&spec, &frames, 8);
        let b = settle(&spec, &frames, 9);
        assert_eq!(a.len(), 6);
        assert_eq!(a, settle(&spec, &frames, 8));
        for (x, y) in a.iter().zip(&b) {
            for (p, q) in x.iter().zip(y) {
                assert!((p.0 - q.0).abs() < 0.3 && (p.1 - q.1).abs() < 0.3);
            }
        }
    }

    #[test]
    fn the_chain_map_round_trips() {
        let pts = [(0.0, 60.0), (-2.0, 50.0), (-6.0, 41.0), (-12.0, 33.0)];
        let m = ChainMap::new(&pts).unwrap_or_else(|| panic!("map"));
        for s in [(0.0, 0.0), (5.0, -3.0), (12.0, 2.0), (20.0, -4.0)] {
            let back = m.to_space(m.to_rig(s));
            assert!(
                (back.0 - s.0).abs() < 0.2 && (back.1 - s.1).abs() < 0.2,
                "{s:?} -> {back:?}"
            );
        }
        // Down the first segment, +y is the body's front (+x).
        let front = m.to_rig((1.0, 3.0));
        assert!(front.0 > 1.0);
    }

    #[test]
    fn a_known_shape_round_trips() {
        let spec = cape();
        let mut s = FlexState::rest(&spec, (0.0, 60.0));
        run(&mut s, &spec, 110.0, 1.0);
        let t = FlexState::from_points(&spec, &s.points, 110.0);
        for (p, q) in s.points.iter().zip(&t.points) {
            assert!((p.0 - q.0).abs() < 1e-3 && (p.1 - q.1).abs() < 1e-3);
        }
    }

    #[test]
    fn bad_data_is_refused() {
        let mut c = cape();
        c.segments = 0;
        assert!(c.validate().is_err());
        let mut c = cape();
        c.limits = (50.0, 10.0);
        assert!(c.validate().is_err());
        let mut c = cape();
        c.damping = 1.5;
        assert!(c.validate().is_err());
        assert!(cape().validate().is_ok());
    }
}
