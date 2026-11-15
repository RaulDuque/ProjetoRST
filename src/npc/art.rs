//! Provisional figures of the NPCs (V2.2), generated in code.
//!
//! Pure module: no Bevy types. Each NPC may declare a [`NpcLook`] in
//! `assets/data/npcs/*.ron` (`look: Some((...))`): an outfit, a headwear, a
//! prop and an accent colour. With the NPC's `size` and `color` (unchanged
//! V2.1 fields: the body colour) it becomes a small front-facing figure,
//! rasterised by `core::pixel_art` with the body centre at the anchor, so
//! positions, reach and prompts stay exactly as in V2.1. An NPC without
//! `look` keeps a plain figure of its colour. **Not final art**: it gives
//! every villager a readable silhouette and role (docs/VISUAL_EQUIPMENT_V2_2.md §9).

use serde::{Deserialize, Serialize};

use crate::core::pixel_art::{PixelArt, Rgb3, Shape, bar, rasterise, shape};

const SKIN: Rgb3 = [0.86, 0.7, 0.58];
const TROUSERS: Rgb3 = [0.2, 0.18, 0.2];
const BOOTS: Rgb3 = [0.14, 0.11, 0.1];
const STEEL: Rgb3 = [0.72, 0.75, 0.8];
const WOOD: Rgb3 = [0.42, 0.29, 0.17];
const LEATHER: Rgb3 = [0.36, 0.24, 0.15];
const FLAME: Rgb3 = [1.0, 0.82, 0.38];
const GREY_HAIR: Rgb3 = [0.78, 0.77, 0.74];

/// Main garment (sets the silhouette below the shoulders).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outfit {
    /// Plain tunic and trousers.
    Tunic,
    /// Tunic with a tabard of the accent colour and a belt (guards).
    Tabard,
    /// Long robe to the feet (pilgrims, scholars).
    Robe,
    /// Long dress flared from the waist.
    Dress,
    /// Heavy work apron of the accent colour (smiths, cooks).
    Apron,
    /// Dress with a shawl of the accent colour over the shoulders.
    Shawl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Headwear {
    Bare,
    Helm,
    Hood,
    Kerchief,
    GreyBun,
}

/// Something carried that tells the role at a glance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Prop {
    Spear,
    LanternStaff,
    Satchel,
    Hammer,
    Cane,
}

/// The look of an NPC (`look: Some((outfit: Apron, headwear: Bare, prop:
/// Some(Hammer), accent: (0.3, 0.25, 0.2)))`).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NpcLook {
    pub outfit: Outfit,
    #[serde(default = "bare")]
    pub headwear: Headwear,
    #[serde(default)]
    pub prop: Option<Prop>,
    /// Second colour (tabard, apron, shawl, hood, kerchief), sRGB 0..1.
    pub accent: (f32, f32, f32),
}

fn bare() -> Headwear {
    Headwear::Bare
}

impl NpcLook {
    pub fn is_valid(&self) -> bool {
        let (r, g, b) = self.accent;
        [r, g, b]
            .iter()
            .all(|c| c.is_finite() && (0.0..=1.0).contains(c))
    }
}

fn shade(c: Rgb3, k: f32) -> Rgb3 {
    [c[0] * k, c[1] * k, c[2] * k]
}

fn octagon(cx: f32, cy: f32, r: f32, color: Rgb3) -> Shape {
    let k = r * 0.41;
    shape(
        &[
            (cx - k, cy + r),
            (cx + k, cy + r),
            (cx + r, cy + k),
            (cx + r, cy - k),
            (cx + k, cy - r),
            (cx - k, cy - r),
            (cx - r, cy - k),
            (cx - r, cy + k),
        ],
        color,
    )
}

/// The figure of an NPC: `size` and `body` are the V2.1 `size` and `color`.
pub fn npc_art(size: (f32, f32), body: (f32, f32, f32), look: Option<&NpcLook>) -> PixelArt {
    let (w, h) = size;
    let body: Rgb3 = [body.0, body.1, body.2];
    let accent: Rgb3 = look.map_or(shade(body, 0.7), |l| [l.accent.0, l.accent.1, l.accent.2]);
    let outfit = look.map_or(Outfit::Tunic, |l| l.outfit);
    let headwear = look.map_or(Headwear::Bare, |l| l.headwear);
    let (feet, top) = (-h / 2.0, h / 2.0);
    let head_r = (h * 0.13).max(3.0);
    let head_y = top - head_r;
    let neck = head_y - head_r;
    let shoulder = neck - 0.5;
    let waist = feet + h * 0.42;
    let half = w * 0.32;
    let mut s = vec![
        // Legs and boots (hidden by long garments).
        bar(-half * 0.8, -half * 0.15, feet + h * 0.2, h * 0.2, TROUSERS),
        bar(half * 0.15, half * 0.8, feet + h * 0.2, h * 0.2, TROUSERS),
        bar(-half * 0.9, -half * 0.1, feet + 1.0, 1.0, BOOTS),
        bar(half * 0.1, half * 0.9, feet + 1.0, 1.0, BOOTS),
        // Arms behind the torso.
        shape(
            &[
                (-half - 2.5, shoulder),
                (-half, shoulder),
                (-half, waist - 1.0),
                (-half - 2.5, waist - 1.0),
            ],
            shade(body, 0.8),
        ),
        shape(
            &[
                (half, shoulder),
                (half + 2.5, shoulder),
                (half + 2.5, waist - 1.0),
                (half, waist - 1.0),
            ],
            shade(body, 0.8),
        ),
        octagon(-half - 1.25, waist - 1.5, 1.4, SKIN),
        octagon(half + 1.25, waist - 1.5, 1.4, SKIN),
        // Torso.
        shape(
            &[
                (-half, shoulder),
                (half, shoulder),
                (half * 0.9, waist),
                (-half * 0.9, waist),
            ],
            body,
        ),
    ];
    match outfit {
        Outfit::Tunic => s.push(bar(-half * 0.9, half * 0.9, waist + 0.6, 0.6, LEATHER)),
        Outfit::Tabard => {
            s.push(shape(
                &[
                    (-half * 0.55, shoulder - 0.5),
                    (half * 0.55, shoulder - 0.5),
                    (half * 0.5, feet + h * 0.22),
                    (-half * 0.5, feet + h * 0.22),
                ],
                accent,
            ));
            s.push(bar(-half * 0.95, half * 0.95, waist + 0.6, 0.7, LEATHER));
        }
        Outfit::Robe => s.push(shape(
            &[
                (-half, shoulder),
                (half, shoulder),
                (half * 1.25, feet + 1.0),
                (-half * 1.25, feet + 1.0),
            ],
            body,
        )),
        Outfit::Dress | Outfit::Shawl => {
            s.push(shape(
                &[
                    (-half * 0.9, waist),
                    (half * 0.9, waist),
                    (half * 1.35, feet + 1.5),
                    (-half * 1.35, feet + 1.5),
                ],
                shade(body, 0.85),
            ));
            if outfit == Outfit::Shawl {
                s.push(shape(
                    &[
                        (-half - 1.0, shoulder + 0.5),
                        (half + 1.0, shoulder + 0.5),
                        (0.0, waist - 1.0),
                    ],
                    accent,
                ));
            }
        }
        Outfit::Apron => {
            s.push(shape(
                &[
                    (-half * 0.7, shoulder - 3.0),
                    (half * 0.7, shoulder - 3.0),
                    (half * 0.85, feet + h * 0.15),
                    (-half * 0.85, feet + h * 0.15),
                ],
                accent,
            ));
            s.push(bar(-half * 0.6, -half * 0.4, shoulder - 1.5, 1.5, accent));
            s.push(bar(half * 0.4, half * 0.6, shoulder - 1.5, 1.5, accent));
        }
    }
    // Head and headwear.
    s.push(octagon(0.0, head_y, head_r, SKIN));
    match headwear {
        Headwear::Bare => s.push(bar(
            -head_r,
            head_r,
            head_y + head_r * 0.7,
            head_r * 0.35,
            shade(body, 0.5),
        )),
        Headwear::Helm => {
            s.push(shape(
                &[
                    (-head_r - 0.6, head_y),
                    (head_r + 0.6, head_y),
                    (head_r * 0.7, head_y + head_r + 0.8),
                    (-head_r * 0.7, head_y + head_r + 0.8),
                ],
                STEEL,
            ));
            s.push(bar(-0.4, 0.4, head_y - head_r * 0.25, head_r * 0.35, STEEL));
        }
        Headwear::Hood => {
            s.push(shape(
                &[
                    (-head_r - 1.2, neck - 1.0),
                    (-head_r - 1.0, head_y + head_r * 0.4),
                    (0.0, head_y + head_r + 1.6),
                    (head_r + 1.0, head_y + head_r * 0.4),
                    (head_r + 1.2, neck - 1.0),
                    (head_r * 0.6, neck),
                    (head_r * 0.7, head_y + head_r * 0.2),
                    (-head_r * 0.7, head_y + head_r * 0.2),
                    (-head_r * 0.6, neck),
                ],
                accent,
            ));
            s.push(bar(
                -head_r * 0.6,
                head_r * 0.6,
                head_y - head_r * 0.1,
                head_r * 0.3,
                shade(SKIN, 0.45),
            ));
        }
        Headwear::Kerchief => s.push(shape(
            &[
                (-head_r - 0.5, head_y + head_r * 0.2),
                (head_r + 0.5, head_y + head_r * 0.2),
                (head_r * 0.6, head_y + head_r + 0.6),
                (-head_r * 0.6, head_y + head_r + 0.6),
            ],
            accent,
        )),
        Headwear::GreyBun => {
            s.push(bar(
                -head_r,
                head_r,
                head_y + head_r * 0.6,
                head_r * 0.45,
                GREY_HAIR,
            ));
            s.push(octagon(0.0, head_y + head_r + 1.2, 1.6, GREY_HAIR));
        }
    }
    // Prop, on the figure's left (screen right).
    let hand_x = half + 1.25;
    let hand_y = waist - 1.5;
    match look.and_then(|l| l.prop) {
        Some(Prop::Spear) => {
            s.push(bar(
                hand_x - 0.5,
                hand_x + 0.5,
                (feet + top + 8.0) / 2.0,
                (top + 8.0 - feet) / 2.0,
                WOOD,
            ));
            s.push(shape(
                &[
                    (hand_x - 1.4, top + 8.0),
                    (hand_x + 1.4, top + 8.0),
                    (hand_x, top + 13.0),
                ],
                STEEL,
            ));
        }
        Some(Prop::LanternStaff) => {
            s.push(bar(
                hand_x - 0.5,
                hand_x + 0.5,
                (feet + top + 4.0) / 2.0,
                (top + 4.0 - feet) / 2.0,
                WOOD,
            ));
            s.push(bar(hand_x + 0.5, hand_x + 3.0, top + 3.5, 0.4, WOOD));
            s.push(bar(hand_x + 1.8, hand_x + 3.8, top + 1.0, 1.6, FLAME));
        }
        Some(Prop::Satchel) => {
            s.push(shape(
                &[
                    (-half * 0.6, shoulder),
                    (-half * 0.3, shoulder),
                    (half + 1.0, waist - 2.0),
                    (half + 0.5, waist - 3.0),
                ],
                LEATHER,
            ));
            s.push(bar(half - 1.0, half + 3.5, waist - 3.5, 2.2, LEATHER));
        }
        Some(Prop::Hammer) => {
            s.push(bar(hand_x - 0.5, hand_x + 0.5, hand_y - 4.0, 5.0, WOOD));
            s.push(bar(hand_x - 2.2, hand_x + 2.2, hand_y - 9.5, 1.6, STEEL));
        }
        Some(Prop::Cane) => {
            s.push(bar(
                hand_x + 0.5,
                hand_x + 1.3,
                (feet + hand_y) / 2.0,
                (hand_y - feet) / 2.0,
                WOOD,
            ));
            s.push(bar(hand_x - 0.5, hand_x + 1.3, hand_y + 0.4, 0.5, WOOD));
        }
        None => {}
    }
    rasterise(&s)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn look(outfit: Outfit, headwear: Headwear, prop: Option<Prop>) -> NpcLook {
        NpcLook {
            outfit,
            headwear,
            prop,
            accent: (0.6, 0.2, 0.2),
        }
    }

    #[test]
    fn the_body_centre_is_the_anchor_and_the_feet_stay_on_the_floor() {
        let art = npc_art((26.0, 46.0), (0.5, 0.4, 0.3), None);
        // The anchor point is the body centre; the opaque art reaches the
        // floor line (23 u below) and no further than the outline.
        let (_, _, below, above) = art.reach();
        assert!((below - 23.0).abs() <= 1.0, "feet at {below}");
        assert!((22.0..=24.5).contains(&above), "head at {above}");
    }

    #[test]
    fn roles_have_distinct_silhouettes() {
        let size = (28.0, 46.0);
        let body = (0.45, 0.5, 0.6);
        let guard = npc_art(
            size,
            body,
            Some(&look(Outfit::Tabard, Headwear::Helm, Some(Prop::Spear))),
        );
        let pilgrim = npc_art(
            size,
            body,
            Some(&look(
                Outfit::Robe,
                Headwear::Hood,
                Some(Prop::LanternStaff),
            )),
        );
        let smith = npc_art(
            size,
            body,
            Some(&look(Outfit::Apron, Headwear::Bare, Some(Prop::Hammer))),
        );
        let plain = npc_art(size, body, None);
        // The spear rises well above the head; the hammer does not.
        assert!(guard.reach().3 > plain.reach().3 + 8.0);
        assert!(smith.reach().3 < guard.reach().3);
        // Every role paints a different image.
        assert_ne!(guard.rgba, pilgrim.rgba);
        assert_ne!(pilgrim.rgba, smith.rgba);
        assert_ne!(smith.rgba, plain.rgba);
    }

    #[test]
    fn accent_must_be_a_colour() {
        let mut l = look(Outfit::Shawl, Headwear::GreyBun, Some(Prop::Cane));
        assert!(l.is_valid());
        l.accent = (1.5, 0.0, 0.0);
        assert!(!l.is_valid());
    }

    #[test]
    fn look_parses_with_defaults() {
        let l: NpcLook = ron::from_str("(outfit: Apron, accent: (0.3, 0.2, 0.1))")
            .unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(l.headwear, Headwear::Bare);
        assert_eq!(l.prop, None);
    }
}
