//! Tiny pixel-art rasteriser for provisional art generated in code (V2.2).
//!
//! Pure module: no Bevy types. Flat polygons in world units (y up) are
//! rasterised at [`ART_DENSITY`] texels per unit, later shapes over earlier
//! ones, with a dark 1-texel outline like the flat finish of the body
//! atlases. Used by the held equipment (`inventory::held_art`) and the NPC
//! figures (`npc::art`). The origin of the shapes becomes [`PixelArt::grip`]
//! (the sprite anchor point).

/// An sRGB colour, channels in `[0, 1]`.
pub type Rgb3 = [f32; 3];

/// Dark outline colour (RGBA8).
pub const OUTLINE: [u8; 4] = [18, 16, 22, 255];

/// Texels per world unit (the body atlases use 2).
pub const ART_DENSITY: f32 = 2.0;

/// A rasterised image and where the hand holds it.
#[derive(Debug, Clone, PartialEq)]
pub struct PixelArt {
    pub width: u32,
    pub height: u32,
    /// RGBA8, row-major from the top-left.
    pub rgba: Vec<u8>,
    /// Grip centre in texels from the top-left (the sprite anchor).
    pub grip: (f32, f32),
}

impl PixelArt {
    /// Size in world units (`Sprite::custom_size`).
    pub fn world_size(&self) -> (f32, f32) {
        (
            self.width as f32 / ART_DENSITY,
            self.height as f32 / ART_DENSITY,
        )
    }

    /// Bevy anchor of the grip (y up, -0.5..0.5); mirrored when flipped.
    pub fn anchor(&self, flip_x: bool) -> (f32, f32) {
        let x = self.grip.0 / self.width as f32 - 0.5;
        let y = 0.5 - self.grip.1 / self.height as f32;
        (if flip_x { -x } else { x }, y)
    }

    /// Number of opaque texels (tests and reports).
    pub fn opaque_texels(&self) -> usize {
        self.rgba
            .as_chunks::<4>()
            .0
            .iter()
            .filter(|p| p[3] > 0)
            .count()
    }

    /// Extent of the opaque texels in world units from the grip:
    /// `(behind, ahead, below, above)`.
    pub fn reach(&self) -> (f32, f32, f32, f32) {
        let (mut x0, mut x1, mut y0, mut y1) = (f32::MAX, f32::MIN, f32::MAX, f32::MIN);
        for (i, p) in self.rgba.as_chunks::<4>().0.iter().enumerate() {
            if p[3] == 0 {
                continue;
            }
            let tx = (i as u32 % self.width) as f32 + 0.5;
            let ty = (i as u32 / self.width) as f32 + 0.5;
            x0 = x0.min(tx);
            x1 = x1.max(tx);
            y0 = y0.min(ty);
            y1 = y1.max(ty);
        }
        (
            (self.grip.0 - x0) / ART_DENSITY,
            (x1 - self.grip.0) / ART_DENSITY,
            (y1 - self.grip.1) / ART_DENSITY,
            (self.grip.1 - y0) / ART_DENSITY,
        )
    }
}

/// A flat polygon in world units (grip at the origin, y up).
pub struct Shape {
    pub points: Vec<(f32, f32)>,
    pub color: Rgb3,
}

pub fn shape(points: &[(f32, f32)], color: impl Into<Rgb3>) -> Shape {
    Shape {
        points: points.to_vec(),
        color: color.into(),
    }
}

/// Axis-aligned bar from `x0` to `x1`, `half` thick around `y`.
pub fn bar(x0: f32, x1: f32, y: f32, half: f32, color: impl Into<Rgb3>) -> Shape {
    shape(
        &[
            (x0, y - half),
            (x1, y - half),
            (x1, y + half),
            (x0, y + half),
        ],
        color,
    )
}

fn inside(points: &[(f32, f32)], x: f32, y: f32) -> bool {
    let mut odd = false;
    let n = points.len();
    for i in 0..n {
        let (xi, yi) = points[i];
        let (xj, yj) = points[(i + n - 1) % n];
        if (yi > y) != (yj > y) && x < (xj - xi) * (y - yi) / (yj - yi) + xi {
            odd = !odd;
        }
    }
    odd
}

fn channel(v: f32) -> u8 {
    (v.clamp(0.0, 1.0) * 255.0).round() as u8
}

/// Rasterises the shapes (later ones paint over earlier ones) and adds the
/// outline. One unit of margin keeps the outline inside the image.
pub fn rasterise(shapes: &[Shape]) -> PixelArt {
    let (mut x0, mut x1, mut y0, mut y1) = (0.0f32, 0.0f32, 0.0f32, 0.0f32);
    for (x, y) in shapes.iter().flat_map(|s| s.points.iter()) {
        x0 = x0.min(*x);
        x1 = x1.max(*x);
        y0 = y0.min(*y);
        y1 = y1.max(*y);
    }
    let (x0, x1, y0, y1) = (
        x0.floor() - 1.0,
        x1.ceil() + 1.0,
        y0.floor() - 1.0,
        y1.ceil() + 1.0,
    );
    let width = ((x1 - x0) * ART_DENSITY) as u32;
    let height = ((y1 - y0) * ART_DENSITY) as u32;
    let mut rgba = vec![0u8; (width * height * 4) as usize];
    for ty in 0..height {
        for tx in 0..width {
            let x = x0 + (tx as f32 + 0.5) / ART_DENSITY;
            let y = y1 - (ty as f32 + 0.5) / ART_DENSITY;
            if let Some(s) = shapes.iter().rev().find(|s| inside(&s.points, x, y)) {
                let i = ((ty * width + tx) * 4) as usize;
                rgba[i..i + 4].copy_from_slice(&[
                    channel(s.color[0]),
                    channel(s.color[1]),
                    channel(s.color[2]),
                    255,
                ]);
            }
        }
    }
    let opaque = |rgba: &[u8], tx: i64, ty: i64| {
        tx >= 0
            && ty >= 0
            && tx < width as i64
            && ty < height as i64
            && rgba[((ty as u32 * width + tx as u32) * 4 + 3) as usize] > 0
    };
    let mut outlined = rgba.clone();
    for ty in 0..height as i64 {
        for tx in 0..width as i64 {
            if opaque(&rgba, tx, ty) {
                continue;
            }
            let near = [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .iter()
                .any(|(dx, dy)| opaque(&rgba, tx + dx, ty + dy));
            if near {
                let i = ((ty as u32 * width + tx as u32) * 4) as usize;
                outlined[i..i + 4].copy_from_slice(&OUTLINE);
            }
        }
    }
    PixelArt {
        width,
        height,
        rgba: outlined,
        grip: (-x0 * ART_DENSITY, y1 * ART_DENSITY),
    }
}
