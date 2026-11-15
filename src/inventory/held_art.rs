//! Provisional pixel art of the held equipment (V2.2), generated in code.
//!
//! Pure module: no Bevy types. Each weapon form is a small set of flat
//! polygons (blade, guard, grip, haft, head) drawn pointing to `+x` with the
//! grip centre (the hand) at the origin, rasterised at
//! [`ART_DENSITY`] texels per world unit with a dark 1-texel outline, like
//! the flat finish of the body atlases. Proportions are in world units next
//! to a ~52 u tall character. This is **not final art**: it gives every
//! category its own silhouette until per-item art exists
//! (docs/VISUAL_EQUIPMENT_V2_2.md §6, §15).

use super::appearance::Rgb;
use super::held::WeaponForm;
pub use crate::core::pixel_art::{ART_DENSITY, PixelArt};
use crate::core::pixel_art::{Shape, bar, rasterise, shape};

const STEEL: Rgb = Rgb(0.74, 0.77, 0.82);
const EDGE: Rgb = Rgb(0.93, 0.95, 0.98);
const DARK_STEEL: Rgb = Rgb(0.42, 0.45, 0.5);
const WOOD: Rgb = Rgb(0.4, 0.27, 0.16);
const LEATHER: Rgb = Rgb(0.27, 0.18, 0.13);

/// A rasterised image and where the hand hoarting sword's family).
fn sword(accent: Rgb) -> Vec<Shape> {
    vec![
        bar(-3.5, 3.0, 0.0, 1.0, LEATHER),
        shape(&[(-5.0, 0.0), (-3.5, 1.4), (-3.5, -1.4)], accent),
        shape(&[(3.0, -3.0), (4.2, -3.2), (4.2, 3.2), (3.0, 3.0)], accent),
        shape(
            &[
                (4.2, -1.2),
                (16.0, -1.0),
                (26.0, 0.2),
                (31.0, 1.6),
                (26.5, 1.6),
                (16.0, 1.2),
                (4.2, 1.2),
            ],
            STEEL,
        ),
        shape(
            &[
                (4.2, 0.5),
                (16.0, 0.6),
                (26.5, 1.6),
                (16.0, 1.2),
                (4.2, 1.2),
            ],
            EDGE,
        ),
    ]
}

/// Short leaf blade: half the length, wider near the guard.
fn dagger(accent: Rgb) -> Vec<Shape> {
    vec![
        bar(-2.5, 2.0, 0.0, 0.9, LEATHER),
        shape(&[(-3.8, 0.0), (-2.5, 1.1), (-2.5, -1.1)], accent),
        bar(2.0, 3.0, 0.0, 2.2, accent),
        shape(
            &[
                (3.0, -1.6),
                (7.0, -1.8),
                (13.0, 0.0),
                (7.0, 1.8),
                (3.0, 1.6),
            ],
            STEEL,
        ),
        shape(
            &[(3.0, 0.4), (7.0, 0.5), (13.0, 0.0), (7.0, 1.8), (3.0, 1.6)],
            EDGE,
        ),
    ]
}

/// Wooden haft with a bearded head at the far end.
fn axe(accent: Rgb) -> Vec<Shape> {
    vec![
        bar(-7.0, 20.0, 0.0, 0.9, WOOD),
        bar(-1.5, 1.5, 0.0, 1.1, LEATHER),
        bar(13.0, 14.2, 0.0, 1.3, accent),
        shape(
            &[
                (14.5, -1.6),
                (19.5, -1.6),
                (21.5, 1.0),
                (22.5, 7.5),
                (19.0, 5.5),
                (15.5, 4.0),
                (14.5, 1.0),
            ],
            STEEL,
        ),
        shape(&[(21.2, 1.4), (22.5, 7.5), (20.8, 6.3), (20.2, 1.6)], EDGE),
        shape(&[(14.5, -1.0), (14.5, -3.2), (16.5, -1.0)], DARK_STEEL),
    ]
}

/// Long shaft held near its middle, leaf head well ahead of the hand.
fn spear(accent: Rgb) -> Vec<Shape> {
    vec![
        bar(-24.0, 33.0, 0.0, 0.75, WOOD),
        bar(-25.5, -24.0, 0.0, 1.0, DARK_STEEL),
        bar(-1.5, 1.5, 0.0, 1.0, LEATHER),
        bar(31.5, 33.5, 0.0, 1.2, accent),
        shape(
            &[
                (33.5, -1.4),
                (37.0, -2.3),
                (44.0, 0.0),
                (37.0, 2.3),
                (33.5, 1.4),
            ],
            STEEL,
        ),
        shape(
            &[
                (33.5, 0.3),
                (37.0, 0.5),
                (44.0, 0.0),
                (37.0, 2.3),
                (33.5, 1.4),
            ],
            EDGE,
        ),
    ]
}

/// Broad two-handed greatblade: long grip, wide guard, heavy blade.
fn heavy(accent: Rgb) -> Vec<Shape> {
    vec![
        bar(-9.0, 2.0, 0.0, 1.2, LEATHER),
        shape(&[(-11.0, 0.0), (-9.0, 2.0), (-9.0, -2.0)], accent),
        shape(&[(2.0, -5.5), (3.8, -6.0), (3.8, 6.0), (2.0, 5.5)], accent),
        shape(
            &[
                (3.8, -3.0),
                (34.0, -2.6),
                (40.0, 0.0),
                (34.0, 2.6),
                (3.8, 3.0),
            ],
            STEEL,
        ),
        shape(
            &[
                (3.8, 1.2),
                (34.0, 1.1),
                (40.0, 0.0),
                (34.0, 2.6),
                (3.8, 3.0),
            ],
            EDGE,
        ),
        bar(4.5, 26.0, 0.0, 0.5, DARK_STEEL),
    ]
}

/// The art of a weapon form with its accent colour.
pub fn weapon_art(form: WeaponForm, accent: Rgb) -> PixelArt {
    let shapes = match form {
        WeaponForm::Sword => sword(accent),
        WeaponForm::Dagger => dagger(accent),
        WeaponForm::Axe => axe(accent),
        WeaponForm::Spear => spear(accent),
        WeaponForm::Heavy => heavy(accent),
    };
    rasterise(&shapes)
}

/// A heater shield facing the camera (the readable side in profile), its
/// face colour from the item, a metal rim, a cross and a boss; the arm holds
/// its centre.
pub fn shield_art(face: Rgb) -> PixelArt {
    let outer = [
        (-5.0, 7.0),
        (5.0, 7.0),
        (5.0, -1.5),
        (0.0, -8.5),
        (-5.0, -1.5),
    ];
    let inner = [
        (-3.8, 5.9),
        (3.8, 5.9),
        (3.8, -1.2),
        (0.0, -7.0),
        (-3.8, -1.2),
    ];
    rasterise(&[
        shape(&outer, DARK_STEEL),
        shape(&inner, face),
        bar(-0.6, 0.6, -0.5, 5.6, DARK_STEEL),
        bar(-3.8, 3.8, 2.0, 0.6, DARK_STEEL),
        bar(-1.2, 1.2, 2.0, 1.2, STEEL),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::pixel_art::OUTLINE;
    use crate::inventory::held::DEFAULT_ACCENT;

    fn all() -> Vec<(WeaponForm, PixelArt)> {
        WeaponForm::ALL
            .iter()
            .map(|f| (*f, weapon_art(*f, DEFAULT_ACCENT)))
            .collect()
    }

    #[test]
    fn the_grip_is_on_an_opaque_texel_inside_every_image() {
        for (form, art) in all()
            .into_iter()
            .chain([(WeaponForm::Sword, shield_art(Rgb(0.5, 0.4, 0.3)))])
        {
            let (gx, gy) = (art.grip.0 as u32, art.grip.1 as u32);
            assert!(gx < art.width && gy < art.height, "{form:?}");
            let i = ((gy * art.width + gx) * 4 + 3) as usize;
            assert_eq!(art.rgba[i], 255, "{form:?}: grip on a transparent texel");
            let (ax, ay) = art.anchor(false);
            assert!((-0.5..=0.5).contains(&ax) && (-0.5..=0.5).contains(&ay));
            assert_eq!(art.anchor(true).0, -ax);
        }
    }

    #[test]
    fn the_five_silhouettes_are_distinguishable() {
        let art = all();
        let reach = |f: WeaponForm| {
            art.iter()
                .find(|(g, _)| *g == f)
                .map(|(_, a)| a.reach())
                .unwrap_or_else(|| panic!("{f:?}"))
        };
        let (sword, dagger, axe, spear, heavy) = (
            reach(WeaponForm::Sword),
            reach(WeaponForm::Dagger),
            reach(WeaponForm::Axe),
            reach(WeaponForm::Spear),
            reach(WeaponForm::Heavy),
        );
        // Dagger: own short silhouette, not a scaled sword (wider for its length).
        assert!(dagger.1 < sword.1 * 0.6);
        // Spear: the longest, reaching far behind the hand too.
        assert!(spear.1 > heavy.1 && spear.1 > sword.1 + 10.0 && spear.0 > 20.0);
        // Axe: a tall head on one side of the haft.
        assert!(axe.3 > 7.0 && axe.3 > axe.2 + 3.0);
        // Heavy: broader and longer than the sword, long two-hand grip.
        assert!(heavy.1 > sword.1 && heavy.3 > sword.3 && heavy.0 > 10.0);
        let mut sizes: Vec<_> = art.iter().map(|(_, a)| (a.width, a.height)).collect();
        sizes.dedup();
        assert_eq!(sizes.len(), 5, "every form has its own image size");
    }

    #[test]
    fn the_accent_colour_is_painted() {
        let red = weapon_art(WeaponForm::Sword, Rgb(1.0, 0.0, 0.0));
        assert!(red.rgba.as_chunks::<4>().0.contains(&[255, 0, 0, 255]));
        assert_ne!(red, weapon_art(WeaponForm::Sword, DEFAULT_ACCENT));
    }

    #[test]
    fn the_outline_surrounds_the_shapes() {
        let art = shield_art(Rgb(0.5, 0.4, 0.3));
        assert!(art.rgba.as_chunks::<4>().0.contains(&OUTLINE));
        // Margin: the border rows stay transparent or outline only.
        let top = art.rgba[..(art.width * 4) as usize].as_chunks::<4>().0;
        assert!(top.iter().all(|p| p[3] == 0 || *p == OUTLINE));
    }
}
