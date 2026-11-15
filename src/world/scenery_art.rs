//! Provisional gothic scenery art (V2.3), generated in code.
//!
//! Pure module: no Bevy types. Decorations are flat polygons with the dark
//! outline of the character art (`core::pixel_art::rasterise`), anchored at
//! the point named by each [`DecorKind`]. Backdrop layers are seamless
//! tiles (`paint_tile`): every shape stays inside its tile, so repeating it
//! leaves no visible joint. Colours come from the region palette and are
//! kept darker than the platforms (`world::maps::PLATFORM_COLOR`) so no
//! decoration reads as something to stand on. **Not final art.**
//! Details: docs/WORLD_VISUAL_V2_3.md §4-6.

use crate::core::pixel_art::{PixelArt, Rgb3, Shape, bar, paint_tile, rasterise, shape};

use super::scenery::{Backdrop, DecorKind, RegionVisual};

/// Texels per world unit of the backdrop tiles (far, soft: cheaper).
pub const BACKDROP_DENSITY: f32 = 1.0;
/// One backdrop tile, in world units (a map cell is 1280 x 720).
pub const TILE: (f32, f32) = (640.0, 720.0);
/// Brightest a decoration's stone may be, as a share of the palette stone:
/// architecture stays behind the platforms in value.
pub const DECOR_STONE_SHADE: f32 = 0.8;

const IRON: Rgb3 = [0.16, 0.16, 0.19];
const WOOD: Rgb3 = [0.3, 0.2, 0.13];
const DEAD_PLANT: Rgb3 = [0.27, 0.25, 0.2];

fn rgb(c: (f32, f32, f32)) -> Rgb3 {
    [c.0, c.1, c.2]
}

fn shade(c: Rgb3, k: f32) -> Rgb3 {
    [c[0] * k, c[1] * k, c[2] * k]
}

fn mix(a: Rgb3, b: Rgb3, t: f32) -> Rgb3 {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

/// A pointed (gothic) arch outline: a rectangle of `w` x `h_side` topped by
/// two circular arcs meeting at `h_side + w * 0.8`. Bottom centre at
/// `(cx, y0)`.
fn pointed_arch(cx: f32, y0: f32, w: f32, h_side: f32) -> Vec<(f32, f32)> {
    let half = w / 2.0;
    let rise = w * 0.8;
    let mut pts = vec![(cx - half, y0), (cx + half, y0), (cx + half, y0 + h_side)];
    let steps = 8;
    for i in 1..steps {
        let t = i as f32 / steps as f32;
        // Right arc: from the right spring to the apex.
        let a = t * std::f32::consts::FRAC_PI_2;
        pts.push((cx + half * a.cos(), y0 + h_side + rise * a.sin()));
    }
    pts.push((cx, y0 + h_side + rise));
    for i in (1..steps).rev() {
        let t = i as f32 / steps as f32;
        let a = t * std::f32::consts::FRAC_PI_2;
        pts.push((cx - half * a.cos(), y0 + h_side + rise * a.sin()));
    }
    pts.push((cx - half, y0 + h_side));
    pts
}

/// The band of a pointed arch (outer minus inner outline) as quads, so the
/// opening stays transparent.
fn arch_rib(cx: f32, y0: f32, w: f32, thickness: f32, color: Rgb3) -> Vec<Shape> {
    let outer = pointed_arch(cx, y0, w, 0.0);
    let inner = pointed_arch(cx, y0, w - 2.0 * thickness, 0.0);
    // Skip the two base points; walk the arc points pairwise.
    let o: Vec<_> = outer.iter().skip(2).copied().collect();
    let i: Vec<_> = inner.iter().skip(2).copied().collect();
    o.windows(2)
        .zip(i.windows(2))
        .map(|(a, b)| shape(&[a[0], a[1], b[1], b[0]], color))
        .collect()
}

/// The art of one decoration, anchored where its kind says.
pub fn decor_art(kind: DecorKind, v: &RegionVisual) -> PixelArt {
    let stone = shade(rgb(v.stone), DECOR_STONE_SHADE);
    let dark = shade(stone, 0.55);
    let light = rgb(v.light);
    let shapes: Vec<Shape> = match kind {
        DecorKind::Pillar { height } => {
            let h = height.max(40.0);
            vec![
                bar(-16.0, 16.0, 6.0, 6.0, dark),
                bar(-11.0, 11.0, h / 2.0, h / 2.0 - 12.0, stone),
                bar(-11.0, -7.0, h / 2.0, h / 2.0 - 12.0, shade(stone, 1.15)),
                bar(5.0, 11.0, h / 2.0, h / 2.0 - 12.0, shade(stone, 0.75)),
                bar(-1.0, 1.0, h / 2.0, h / 2.0 - 14.0, shade(stone, 0.6)),
                shape(
                    &[
                        (-17.0, h - 12.0),
                        (17.0, h - 12.0),
                        (13.0, h - 4.0),
                        (-13.0, h - 4.0),
                    ],
                    stone,
                ),
                bar(-15.0, 15.0, h - 2.0, 2.0, dark),
            ]
        }
        DecorKind::Arch { width, height } => {
            let w = width.max(40.0);
            let side = (height - w * 0.8).max(10.0);
            let outer = pointed_arch(0.0, 0.0, w, side);
            let inner = pointed_arch(0.0, 0.0, w - 18.0, side - 4.0);
            // A blind arch (wall niche): the recess shows masonry courses,
            // so it never reads as an opening the player could go through.
            let recess = shade(stone, 0.62);
            let mut v = vec![shape(&outer, stone), shape(&inner, recess)];
            let courses = ((side / 28.0) as usize).max(1);
            for k in 1..=courses {
                let y = k as f32 * 28.0;
                v.push(bar(
                    -w / 2.0 + 10.0,
                    w / 2.0 - 10.0,
                    y,
                    1.0,
                    shade(stone, 0.5),
                ));
            }
            v.push(bar(-4.0, 4.0, side + w * 0.8 - 6.0, 6.0, shade(stone, 1.1)));
            v
        }
        DecorKind::TallWindow { height } => {
            let side = (height - 34.0).max(20.0);
            vec![
                shape(&pointed_arch(0.0, 0.0, 44.0, side), stone),
                shape(
                    &pointed_arch(0.0, 4.0, 32.0, side - 6.0),
                    shade(light, 0.55),
                ),
                bar(-1.5, 1.5, side / 2.0 + 10.0, side / 2.0 + 4.0, dark),
                bar(-16.0, 16.0, side * 0.55, 1.5, dark),
                bar(-26.0, 26.0, 2.0, 3.0, shade(stone, 1.1)),
            ]
        }
        DecorKind::Chain { length } => {
            let n = (length / 8.0).max(1.0) as usize;
            (0..n)
                .map(|i| {
                    let y = -(i as f32) * 8.0 - 4.0;
                    if i % 2 == 0 {
                        bar(-2.5, 2.5, y, 3.5, IRON)
                    } else {
                        bar(-1.0, 1.0, y, 4.0, shade(IRON, 1.6))
                    }
                })
                .collect()
        }
        DecorKind::Banner { length, color } => {
            let c = rgb(color);
            let l = length.max(30.0);
            vec![
                bar(-19.0, 19.0, -2.0, 2.0, IRON),
                shape(
                    &[
                        (-15.0, -4.0),
                        (15.0, -4.0),
                        (15.0, -l),
                        (0.0, -l + 10.0),
                        (-15.0, -l),
                    ],
                    c,
                ),
                bar(-15.0, 15.0, -10.0, 2.0, shade(c, 0.6)),
                shape(
                    &[
                        (0.0, -l * 0.35),
                        (6.0, -l * 0.5),
                        (0.0, -l * 0.65),
                        (-6.0, -l * 0.5),
                    ],
                    shade(c, 1.5),
                ),
            ]
        }
        DecorKind::Statue => vec![
            bar(-24.0, 24.0, 14.0, 14.0, dark),
            bar(-27.0, 27.0, 30.0, 3.0, stone),
            shape(
                &[(-16.0, 33.0), (16.0, 33.0), (11.0, 92.0), (-11.0, 92.0)],
                stone,
            ),
            shape(
                &[
                    (-12.0, 88.0),
                    (12.0, 88.0),
                    (8.0, 108.0),
                    (0.0, 114.0),
                    (-8.0, 108.0),
                ],
                shade(stone, 1.12),
            ),
            shape(
                &[(-5.0, 92.0), (5.0, 92.0), (4.0, 102.0), (-4.0, 102.0)],
                shade(dark, 0.5),
            ),
            bar(-2.0, 2.0, 60.0, 22.0, shade(stone, 0.7)),
        ],
        DecorKind::Torch => vec![
            bar(-5.0, 5.0, 0.0, 2.0, IRON),
            shape(&[(-3.0, 0.0), (3.0, 0.0), (2.0, 16.0), (-2.0, 16.0)], WOOD),
            shape(
                &[
                    (-5.0, 16.0),
                    (5.0, 16.0),
                    (3.0, 26.0),
                    (0.0, 34.0),
                    (-3.0, 26.0),
                ],
                light,
            ),
            shape(
                &[(-2.0, 17.0), (2.0, 17.0), (0.0, 27.0)],
                mix(light, [1.0, 1.0, 0.9], 0.6),
            ),
        ],
        DecorKind::Rubble { width } => {
            let w = width.max(20.0);
            let n = ((w / 18.0) as usize).max(2);
            (0..n)
                .map(|i| {
                    let x = -w / 2.0 + (i as f32 + 0.5) * w / n as f32;
                    let h = 6.0 + ((i * 7) % 5) as f32 * 2.0;
                    let s = if i % 2 == 0 { stone } else { shade(stone, 0.8) };
                    shape(
                        &[
                            (x - 9.0, 0.0),
                            (x + 9.0, 0.0),
                            (x + 6.0, h),
                            (x - 4.0, h + 2.0),
                        ],
                        s,
                    )
                })
                .collect()
        }
        DecorKind::IronGrate { width, height } => {
            let n = ((width / 10.0) as usize).max(2);
            let mut v: Vec<Shape> = (0..=n)
                .map(|i| {
                    let x = -width / 2.0 + i as f32 * width / n as f32;
                    bar(x - 1.5, x + 1.5, height / 2.0, height / 2.0, IRON)
                })
                .collect();
            for k in 1..4 {
                v.push(bar(
                    -width / 2.0,
                    width / 2.0,
                    height * k as f32 / 4.0,
                    1.5,
                    IRON,
                ));
            }
            v
        }
        DecorKind::DeadVine { length } => {
            let n = ((length / 12.0) as usize).max(1);
            (0..n)
                .map(|i| {
                    let y = -(i as f32) * 12.0;
                    let dx = if i % 2 == 0 { 3.0 } else { -3.0 };
                    shape(
                        &[
                            (-1.0, y),
                            (1.0, y),
                            (dx + 1.0, y - 12.0),
                            (dx - 1.0, y - 12.0),
                        ],
                        DEAD_PLANT,
                    )
                })
                .chain((0..n).step_by(2).map(|i| {
                    let y = -(i as f32) * 12.0 - 6.0;
                    shape(&[(0.0, y), (7.0, y - 3.0), (7.0, y - 1.0)], DEAD_PLANT)
                }))
                .collect()
        }
        DecorKind::Shopfront { width, awning } => {
            let w = width.max(80.0);
            let a = rgb(awning);
            let mut v = vec![
                bar(-w / 2.0, w / 2.0, 55.0, 55.0, shade(WOOD, 1.3)),
                bar(-w / 2.0 + 6.0, -w / 2.0 + 10.0, 55.0, 55.0, WOOD),
                bar(w / 2.0 - 10.0, w / 2.0 - 6.0, 55.0, 55.0, WOOD),
                bar(-w * 0.35, -w * 0.05, 48.0, 22.0, shade(light, 0.6)),
                bar(-w * 0.2 - 1.0, -w * 0.2 + 1.0, 48.0, 22.0, WOOD),
                bar(w * 0.08, w * 0.32, 32.0, 32.0, shade(WOOD, 0.8)),
                bar(-w * 0.25, w * 0.25, 122.0, 7.0, shade(WOOD, 0.9)),
            ];
            let stripes = ((w / 16.0) as usize).max(3);
            for i in 0..stripes {
                let x0 = -w / 2.0 - 6.0 + i as f32 * (w + 12.0) / stripes as f32;
                let x1 = x0 + (w + 12.0) / stripes as f32;
                let c = if i % 2 == 0 { a } else { shade(a, 0.65) };
                v.push(shape(
                    &[(x0, 100.0), (x1, 100.0), (x1 - 3.0, 86.0), (x0 - 3.0, 86.0)],
                    c,
                ));
            }
            v
        }
        DecorKind::Lantern => vec![
            bar(-0.8, 0.8, -6.0, 6.0, IRON),
            bar(-6.0, 6.0, -13.0, 1.5, IRON),
            bar(-5.0, 5.0, -21.0, 7.0, shade(light, 0.85)),
            bar(-1.0, 1.0, -21.0, 7.0, IRON),
            bar(-6.0, 6.0, -29.0, 1.5, IRON),
        ],
    };
    rasterise(&shapes)
}

/// Glow radius of a light decoration (world units), if it has one, and the
/// offset of the light from the decoration's anchor.
pub fn light_of(kind: DecorKind) -> Option<(f32, (f32, f32))> {
    match kind {
        DecorKind::Torch => Some((70.0, (0.0, 26.0))),
        DecorKind::TallWindow { height } => Some((90.0, (0.0, height * 0.5))),
        DecorKind::Lantern => Some((55.0, (0.0, -21.0))),
        _ => None,
    }
}

/// One parallax layer of a backdrop.
#[derive(Debug, Clone, PartialEq)]
pub struct BackdropLayer {
    pub art: PixelArt,
    /// 1 = fixed in the world, 0 = fixed on the screen.
    pub factor: f32,
    /// Repeat vertically too (the design is vertically periodic).
    pub tile_y: bool,
}

/// Number of steps of the stepped sky gradient.
const SKY_BANDS: usize = 12;

fn sky(v: &RegionVisual) -> Vec<Shape> {
    let top = shade(rgb(v.sky), 0.6);
    let horizon = shade(rgb(v.sky), 1.6);
    (0..SKY_BANDS)
        .map(|i| {
            let y0 = TILE.1 * i as f32 / SKY_BANDS as f32;
            let y1 = TILE.1 * (i + 1) as f32 / SKY_BANDS as f32;
            let t = 1.0 - i as f32 / (SKY_BANDS - 1) as f32;
            shape(
                &[(0.0, y0), (TILE.0, y0), (TILE.0, y1), (0.0, y1)],
                mix(top, horizon, t * t),
            )
        })
        .collect()
}

fn far_silhouettes(v: &RegionVisual) -> Vec<Shape> {
    let far = mix(shade(rgb(v.sky), 1.1), shade(rgb(v.stone), 0.35), 0.5);
    let window = shade(rgb(v.light), 0.5);
    match v.backdrop {
        Backdrop::Castle => {
            let mut s = vec![shape(
                &[(0.0, 0.0), (TILE.0, 0.0), (TILE.0, 150.0), (0.0, 150.0)],
                far,
            )];
            for (x, w, h) in [
                (60.0, 50.0, 330.0),
                (210.0, 70.0, 260.0),
                (400.0, 44.0, 380.0),
                (520.0, 60.0, 230.0),
            ] {
                s.push(bar(x, x + w, h / 2.0, h / 2.0, far));
                s.push(shape(
                    &[(x - 6.0, h), (x + w + 6.0, h), (x + w / 2.0, h + w * 1.3)],
                    far,
                ));
                s.push(bar(
                    x + w / 2.0 - 3.0,
                    x + w / 2.0 + 3.0,
                    h * 0.7,
                    7.0,
                    window,
                ));
            }
            // A pale moon, inside the tile.
            s.push(shape(
                &[
                    (300.0, 560.0),
                    (318.0, 552.0),
                    (326.0, 534.0),
                    (318.0, 516.0),
                    (300.0, 508.0),
                    (282.0, 516.0),
                    (274.0, 534.0),
                    (282.0, 552.0),
                ],
                mix(rgb(v.sky), [0.85, 0.85, 0.9], 0.55),
            ));
            s
        }
        Backdrop::Ruins => {
            let mut s = vec![shape(
                &[(0.0, 0.0), (TILE.0, 0.0), (TILE.0, 90.0), (0.0, 90.0)],
                far,
            )];
            for (x, h) in [(40.0, 300.0), (230.0, 210.0), (420.0, 340.0)] {
                s.push(bar(x, x + 26.0, h / 2.0, h / 2.0, far));
                s.push(bar(x + 120.0, x + 146.0, h * 0.35, h * 0.35, far));
                s.push(shape(
                    &[
                        (x, h),
                        (x + 70.0, h + 40.0),
                        (x + 72.0, h + 30.0),
                        (x + 26.0, h - 6.0),
                    ],
                    far,
                ));
            }
            // Hanging stalactite-like shapes from the top.
            for x in [100.0, 330.0, 520.0] {
                s.push(shape(
                    &[(x - 20.0, TILE.1), (x + 20.0, TILE.1), (x, TILE.1 - 110.0)],
                    far,
                ));
            }
            s
        }
        Backdrop::Village => {
            let mut s = vec![shape(
                &[(0.0, 0.0), (TILE.0, 0.0), (TILE.0, 120.0), (0.0, 120.0)],
                far,
            )];
            // Castle on a hill far away, then roofs and chimneys.
            s.push(shape(
                &[
                    (380.0, 120.0),
                    (620.0, 120.0),
                    (560.0, 220.0),
                    (440.0, 220.0),
                ],
                far,
            ));
            s.push(bar(470.0, 490.0, 300.0, 80.0, far));
            s.push(shape(
                &[(462.0, 380.0), (498.0, 380.0), (480.0, 430.0)],
                far,
            ));
            for (x, w, h) in [
                (20.0, 110.0, 190.0),
                (150.0, 90.0, 230.0),
                (260.0, 120.0, 180.0),
            ] {
                s.push(bar(x, x + w, h / 2.0, h / 2.0, far));
                s.push(shape(
                    &[(x - 8.0, h), (x + w + 8.0, h), (x + w / 2.0, h + 60.0)],
                    far,
                ));
                s.push(bar(x + w * 0.7, x + w * 0.7 + 12.0, h + 40.0, 30.0, far));
                s.push(bar(x + 20.0, x + 32.0, h * 0.55, 9.0, window));
            }
            s
        }
    }
}

fn mid_layer(v: &RegionVisual) -> Vec<Shape> {
    let stone = shade(rgb(v.stone), 0.42);
    let deep = shade(rgb(v.stone), 0.3);
    let warm = shade(rgb(v.light), 0.45);
    match v.backdrop {
        Backdrop::Castle => {
            // An arcade: pillars at 0/320/640 (the edge pillar is split
            // across the tile joint), open pointed arches between them.
            let mut s = vec![
                bar(0.0, 30.0, TILE.1 / 2.0, TILE.1 / 2.0, stone),
                bar(290.0, 350.0, TILE.1 / 2.0, TILE.1 / 2.0, stone),
                bar(610.0, 640.0, TILE.1 / 2.0, TILE.1 / 2.0, stone),
                bar(298.0, 304.0, TILE.1 / 2.0, TILE.1 / 2.0, deep),
            ];
            for cx in [160.0, 480.0] {
                s.extend(arch_rib(cx, 470.0, 260.0, 16.0, stone));
            }
            s
        }
        Backdrop::Ruins => {
            let mut s = Vec::new();
            for (x, h) in [(60.0, 520.0), (330.0, 300.0), (500.0, 640.0)] {
                s.push(bar(x, x + 46.0, h / 2.0, h / 2.0, stone));
                s.push(shape(
                    &[
                        (x, h),
                        (x + 46.0, h),
                        (x + 30.0, h + 24.0),
                        (x + 12.0, h + 10.0),
                    ],
                    stone,
                ));
            }
            s.push(shape(
                &[(150.0, 0.0), (260.0, 0.0), (230.0, 40.0), (180.0, 30.0)],
                deep,
            ));
            s
        }
        Backdrop::Village => {
            let mut s = Vec::new();
            // Houses across the street: tall facades with two rows of lit
            // windows well above the floor line, gabled roofs.
            for (x, w) in [(20.0, 260.0), (340.0, 260.0)] {
                s.push(bar(x, x + w, 200.0, 200.0, stone));
                s.push(shape(
                    &[
                        (x - 14.0, 400.0),
                        (x + w + 14.0, 400.0),
                        (x + w / 2.0, 500.0),
                    ],
                    deep,
                ));
                for row in [190.0, 310.0] {
                    for k in 0..3 {
                        let wx = x + 30.0 + k as f32 * 80.0;
                        s.push(bar(wx, wx + 34.0, row, 26.0, warm));
                        s.push(bar(wx + 16.0, wx + 18.0, row, 26.0, deep));
                        s.push(bar(wx - 3.0, wx + 37.0, row - 28.0, 2.0, deep));
                    }
                }
                s.push(bar(x, x + w, 250.0, 4.0, deep));
                s.push(bar(x + 6.0, x + 12.0, 200.0, 200.0, deep));
                s.push(bar(x + w - 12.0, x + w - 6.0, 200.0, 200.0, deep));
            }
            s
        }
    }
}

/// The parallax layers of a region, farthest first.
pub fn backdrop_layers(v: &RegionVisual) -> Vec<BackdropLayer> {
    let mut far = sky(v);
    far.extend(far_silhouettes(v));
    vec![
        BackdropLayer {
            art: paint_tile(TILE.0, TILE.1, BACKDROP_DENSITY, &far),
            factor: 0.12,
            tile_y: false,
        },
        BackdropLayer {
            art: paint_tile(TILE.0, TILE.1, BACKDROP_DENSITY, &mid_layer(v)),
            factor: 0.4,
            tile_y: true,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::scenery::Fog;

    fn visual(backdrop: Backdrop) -> RegionVisual {
        RegionVisual {
            backdrop,
            stone: (0.34, 0.33, 0.38),
            light: (1.0, 0.7, 0.4),
            sky: (0.06, 0.06, 0.1),
            fog: Some(Fog {
                color: (0.5, 0.5, 0.55),
                density: 0.2,
            }),
        }
    }

    /// Vertically repeated layers have no full-width horizontal band: repeated
    /// in a tall room it would cross the screen and read as a ledge.
    fn has_full_width_band(art: &PixelArt) -> bool {
        (0..art.height).any(|ty| {
            (0..art.width).all(|tx| art.rgba[((ty * art.width + tx) * 4 + 3) as usize] > 0)
        })
    }

    fn all_kinds() -> Vec<DecorKind> {
        vec![
            DecorKind::Pillar { height: 300.0 },
            DecorKind::Arch {
                width: 160.0,
                height: 260.0,
            },
            DecorKind::TallWindow { height: 200.0 },
            DecorKind::Chain { length: 120.0 },
            DecorKind::Banner {
                length: 140.0,
                color: (0.5, 0.1, 0.12),
            },
            DecorKind::Statue,
            DecorKind::Torch,
            DecorKind::Rubble { width: 90.0 },
            DecorKind::IronGrate {
                width: 80.0,
                height: 160.0,
            },
            DecorKind::DeadVine { length: 100.0 },
            DecorKind::Shopfront {
                width: 200.0,
                awning: (0.6, 0.2, 0.15),
            },
            DecorKind::Lantern,
        ]
    }

    #[test]
    fn every_decor_kind_draws_something_at_its_anchor() {
        let v = visual(Backdrop::Castle);
        for kind in all_kinds() {
            let art = decor_art(kind, &v);
            assert!(art.opaque_texels() > 20, "{kind:?}");
            let (ax, ay) = art.anchor(false);
            assert!(
                (-0.5..=0.5).contains(&ax) && (-0.5..=0.5).contains(&ay),
                "{kind:?}"
            );
        }
    }

    #[test]
    fn standing_decor_rests_on_its_anchor_and_hanging_decor_hangs_from_it() {
        let v = visual(Backdrop::Castle);
        let (_, _, below, above) = decor_art(DecorKind::Pillar { height: 300.0 }, &v).reach();
        assert!(
            below <= 1.0 && (above - 300.0).abs() <= 2.0,
            "{below} {above}"
        );
        let (_, _, below, above) = decor_art(DecorKind::Chain { length: 120.0 }, &v).reach();
        assert!(
            above <= 1.0 && (below - 120.0).abs() <= 2.0,
            "{below} {above}"
        );
    }

    #[test]
    fn decor_stone_stays_darker_than_the_platforms() {
        // PLATFORM_COLOR = (0.32, 0.29, 0.36): luminance about 0.31.
        let lum = |c: Rgb3| 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2];
        for backdrop in [Backdrop::Castle, Backdrop::Ruins, Backdrop::Village] {
            let v = visual(backdrop);
            let stone = shade(rgb(v.stone), DECOR_STONE_SHADE);
            assert!(lum(stone) < 0.31, "{backdrop:?}");
        }
    }

    #[test]
    fn backdrop_tiles_are_seamless_and_regions_differ() {
        let mut firsts = Vec::new();
        for backdrop in [Backdrop::Castle, Backdrop::Ruins, Backdrop::Village] {
            let layers = backdrop_layers(&visual(backdrop));
            assert_eq!(layers.len(), 2);
            for layer in &layers {
                let a = &layer.art;
                assert_eq!(a.world_size(), TILE);
                // Seamless: the left and right columns match row by row
                // (shapes never cross the tile edges).
                for ty in 0..a.height {
                    let l = ((ty * a.width) * 4) as usize;
                    let r = ((ty * a.width + a.width - 1) * 4) as usize;
                    assert_eq!(a.rgba[l..l + 4], a.rgba[r..r + 4], "{backdrop:?} row {ty}");
                }
            }
            assert!(layers[0].factor < layers[1].factor, "far moves less");
            for layer in layers.iter().filter(|l| l.tile_y) {
                assert!(
                    !has_full_width_band(&layer.art),
                    "{backdrop:?}: ledge-like band"
                );
            }
            firsts.push(layers[0].art.rgba.clone());
        }
        assert_ne!(firsts[0], firsts[1]);
        assert_ne!(firsts[1], firsts[2]);
    }

    #[test]
    fn lights_belong_to_light_kinds_only() {
        for kind in all_kinds() {
            assert_eq!(light_of(kind).is_some(), kind.is_light(), "{kind:?}");
        }
    }
}
