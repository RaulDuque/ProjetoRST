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
    /// Texels per world unit (V2.3: backdrop tiles use less).
    pub density: f32,
}

impl PixelArt {
    /// A 1x1 transparent image (placeholder that is never drawn).
    pub fn empty() -> Self {
        Self {
            width: 1,
            height: 1,
            rgba: vec![0; 4],
            grip: (0.5, 0.5),
            density: 1.0,
        }
    }

    /// Size in world units (`Sprite::custom_size`).
    pub fn world_size(&self) -> (f32, f32) {
        (
            self.width as f32 / self.density,
            self.height as f32 / self.density,
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
            (self.grip.0 - x0) / self.density,
            (x1 - self.grip.0) / self.density,
            (y1 - self.grip.1) / self.density,
            (self.grip.1 - y0) / self.density,
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

/// Bounding box of each shape, to skip the polygon test early.
fn bounds(shapes: &[Shape]) -> Vec<(f32, f32, f32, f32)> {
    shapes
        .iter()
        .map(|s| {
            s.points.iter().fold(
                (f32::MAX, f32::MAX, f32::MIN, f32::MIN),
                |(a, b, c, d), (x, y)| (a.min(*x), b.min(*y), c.max(*x), d.max(*y)),
            )
        })
        .collect()
}

/// The topmost shape covering a point (later shapes paint over earlier).
fn topmost<'a>(
    shapes: &'a [Shape],
    boxes: &[(f32, f32, f32, f32)],
    x: f32,
    y: f32,
) -> Option<&'a Shape> {
    shapes
        .iter()
        .zip(boxes)
        .rev()
        .find(|(s, b)| x >= b.0 && x <= b.2 && y >= b.1 && y <= b.3 && inside(&s.points, x, y))
        .map(|(s, _)| s)
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
    let boxes = bounds(shapes);
    for ty in 0..height {
        for tx in 0..width {
            let x = x0 + (tx as f32 + 0.5) / ART_DENSITY;
            let y = y1 - (ty as f32 + 0.5) / ART_DENSITY;
            if let Some(s) = topmost(shapes, &boxes, x, y) {
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
        density: ART_DENSITY,
    }
}

/// V2.3: a fixed `width` x `height` world-unit tile (origin bottom-left)
/// at `density`, no outline: texels outside every shape stay transparent.
/// Shapes that stay inside the tile make it seamless when repeated.
pub fn paint_tile(width: f32, height: f32, density: f32, shapes: &[Shape]) -> PixelArt {
    let w = (width * density).round().max(1.0) as u32;
    let h = (height * density).round().max(1.0) as u32;
    let mut rgba = vec![0u8; (w * h * 4) as usize];
    let boxes = bounds(shapes);
    for ty in 0..h {
        for tx in 0..w {
            let x = (tx as f32 + 0.5) / density;
            let y = height - (ty as f32 + 0.5) / density;
            if let Some(s) = topmost(shapes, &boxes, x, y) {
                let i = ((ty * w + tx) * 4) as usize;
                rgba[i..i + 4].copy_from_slice(&[
                    channel(s.color[0]),
                    channel(s.color[1]),
                    channel(s.color[2]),
                    255,
                ]);
            }
        }
    }
    PixelArt {
        width: w,
        height: h,
        rgba,
        grip: (w as f32 / 2.0, h as f32 / 2.0),
        density,
    }
}

/// V2.3: a white disc whose opacity falls off from the centre (a soft
/// light, tinted by the sprite colour). `radius` in world units.
pub fn radial_glow(radius: f32, density: f32) -> PixelArt {
    let r = (radius * density).round().max(2.0) as u32;
    let size = r * 2;
    let mut rgba = vec![0u8; (size * size * 4) as usize];
    for ty in 0..size {
        for tx in 0..size {
            let dx = tx as f32 + 0.5 - r as f32;
            let dy = ty as f32 + 0.5 - r as f32;
            let t = (dx.hypot(dy) / r as f32).min(1.0);
            let a = (1.0 - t).powi(2);
            let i = ((ty * size + tx) * 4) as usize;
            rgba[i..i + 4].copy_from_slice(&[255, 255, 255, channel(a)]);
        }
    }
    PixelArt {
        width: size,
        height: size,
        rgba,
        grip: (r as f32, r as f32),
        density,
    }
}

/// V2.3: a white band whose opacity rises from the top (0) to the bottom
/// (1): low mist, tinted and scaled by the sprite colour.
pub fn vertical_fade(width: f32, height: f32, density: f32) -> PixelArt {
    let w = (width * density).round().max(1.0) as u32;
    let h = (height * density).round().max(2.0) as u32;
    let mut rgba = Vec::with_capacity((w * h * 4) as usize);
    for ty in 0..h {
        let t = ty as f32 / (h - 1) as f32;
        let a = channel(t * t);
        for _ in 0..w {
            rgba.extend([255, 255, 255, a]);
        }
    }
    PixelArt {
        width: w,
        height: h,
        rgba,
        grip: (w as f32 / 2.0, h as f32 / 2.0),
        density,
    }
}

#[cfg(test)]
mod tile_tests {
    use super::*;

    #[test]
    fn tiles_have_the_exact_size_and_transparent_gaps() {
        let t = paint_tile(
            100.0,
            50.0,
            1.0,
            &[bar(10.0, 20.0, 25.0, 25.0, [1.0, 0.0, 0.0])],
        );
        assert_eq!((t.width, t.height), (100, 50));
        assert_eq!(t.world_size(), (100.0, 50.0));
        // Column x=15 is painted on every row; x=50 nowhere.
        for ty in 0..50 {
            assert_eq!(t.rgba[((ty * 100 + 15) * 4 + 3) as usize], 255);
            assert_eq!(t.rgba[((ty * 100 + 50) * 4 + 3) as usize], 0);
        }
    }

    #[test]
    fn glow_and_fade_fall_off() {
        let g = radial_glow(10.0, 1.0);
        let centre = g.rgba[((10 * 20 + 10) * 4 + 3) as usize];
        let edge = g.rgba[((10 * 20) * 4 + 3) as usize];
        assert!(centre > 200 && edge < 10, "{centre} {edge}");
        let f = vertical_fade(4.0, 10.0, 1.0);
        assert_eq!(f.rgba[3], 0, "transparent top");
        assert_eq!(f.rgba[((9 * 4) * 4 + 3) as usize], 255, "opaque bottom");
    }
}
