//! Scenery data (V2.3): the look of a region (backdrop theme, palette,
//! light, fog) and the decoration of a room, separate from the room's
//! functional geometry.
//!
//! Pure module: no Bevy types. Both parts are optional data in the region
//! files (`assets/world/regions/*.ron`): `region: (..., visual: Some((...)))`
//! and, per room, `decor: [...]`. Rooms and regions without them keep the
//! V2.2 look. Decoration never collides, never blocks a door and is never
//! saved; it is spawned with the room and removed with it.
//! Details: docs/WORLD_VISUAL_V2_3.md §3-6.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Backdrop family of a region (parallax layers and decor palette).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Backdrop {
    /// Monumental castle: cold stone, tall arcades, towers far away.
    Castle,
    /// Forgotten galleries: broken arches, ruins, deep murk.
    Ruins,
    /// Village: dusk sky, roofs and chimneys, warm lit windows.
    Village,
}

/// Mist drifting low in the room.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fog {
    pub color: (f32, f32, f32),
    /// Peak opacity, 0..=0.35 (never hides the floor).
    pub density: f32,
}

/// `region: (..., visual: Some((...)))`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegionVisual {
    pub backdrop: Backdrop,
    /// Stone colour of the architecture (sRGB 0..1).
    pub stone: (f32, f32, f32),
    /// Colour of the light sources (torches, windows, lanterns).
    pub light: (f32, f32, f32),
    /// Sky / far murk colour of the backdrop.
    pub sky: (f32, f32, f32),
    #[serde(default)]
    pub fog: Option<Fog>,
}

/// Where a decoration is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum DecorLayer {
    /// Behind the platforms (architecture).
    #[default]
    Back,
    /// In front of the characters, translucent; only for thin hanging
    /// things (chains, vines) so nothing of the play is hidden.
    Front,
}

/// What a decoration is. Sizes in world units.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DecorKind {
    /// Column with base and capital; `at` = bottom centre.
    Pillar { height: f32 },
    /// Pointed (gothic) arch in the wall; `at` = bottom centre.
    Arch { width: f32, height: f32 },
    /// Tall lancet window with glow; `at` = bottom centre.
    TallWindow { height: f32 },
    /// Hanging chain; `at` = top (ceiling) point.
    Chain { length: f32 },
    /// Tattered banner; `at` = top point; its own colour.
    Banner { length: f32, color: (f32, f32, f32) },
    /// Hooded statue on a pedestal; `at` = bottom centre.
    Statue,
    /// Wall torch with a flickering light; `at` = bracket point.
    Torch,
    /// Broken stones; `at` = bottom centre.
    Rubble { width: f32 },
    /// Iron grate (portcullis-like); `at` = bottom centre.
    IronGrate { width: f32, height: f32 },
    /// Dead hanging vine; `at` = top point.
    DeadVine { length: f32 },
    /// Village shop front with awning; `at` = bottom centre.
    Shopfront { width: f32, awning: (f32, f32, f32) },
    /// Hanging lantern with warm light; `at` = top point.
    Lantern,
}

impl DecorKind {
    /// Kinds thin enough for the front layer.
    pub fn may_be_front(self) -> bool {
        matches!(self, DecorKind::Chain { .. } | DecorKind::DeadVine { .. })
    }

    /// Whether it carries a light (glow sprite).
    pub fn is_light(self) -> bool {
        matches!(
            self,
            DecorKind::Torch | DecorKind::TallWindow { .. } | DecorKind::Lantern
        )
    }

    fn dimensions(self) -> Vec<f32> {
        match self {
            DecorKind::Pillar { height } => vec![height],
            DecorKind::Arch { width, height } => vec![width, height],
            DecorKind::TallWindow { height } => vec![height],
            DecorKind::Chain { length } | DecorKind::DeadVine { length } => vec![length],
            DecorKind::Banner { length, .. } => vec![length],
            DecorKind::Rubble { width } => vec![width],
            DecorKind::IronGrate { width, height } => vec![width, height],
            DecorKind::Shopfront { width, .. } => vec![width],
            DecorKind::Statue | DecorKind::Torch | DecorKind::Lantern => vec![],
        }
    }

    fn colours(self) -> Vec<(f32, f32, f32)> {
        match self {
            DecorKind::Banner { color, .. } => vec![color],
            DecorKind::Shopfront { awning, .. } => vec![awning],
            _ => vec![],
        }
    }
}

/// One decoration of a room (`decor: [(kind: Torch, at: (300.0, 160.0))]`).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Decor {
    pub kind: DecorKind,
    /// Anchor point in room coordinates (see each kind).
    pub at: (f32, f32),
    #[serde(default)]
    pub layer: DecorLayer,
    /// Mirror horizontally.
    #[serde(default)]
    pub flip: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum SceneryError {
    #[error("room `{room}` decor #{index}: anchor outside the room")]
    Outside { room: String, index: usize },
    #[error("room `{room}` decor #{index}: size not between 1 and the room size")]
    Size { room: String, index: usize },
    #[error("room `{room}` decor #{index}: only chains and vines may be in front")]
    Front { room: String, index: usize },
    #[error("colour channel outside 0..1 ({0})")]
    Colour(String),
    #[error("region `{0}`: fog density outside 0..0.35")]
    Fog(String),
}

/// Highest fog opacity: the floor stays readable.
pub const MAX_FOG_DENSITY: f32 = 0.35;

fn colour_ok(c: (f32, f32, f32)) -> bool {
    [c.0, c.1, c.2]
        .iter()
        .all(|v| v.is_finite() && (0.0..=1.0).contains(v))
}

impl RegionVisual {
    pub fn validate(&self, region: &str) -> Result<(), SceneryError> {
        for c in [self.stone, self.light, self.sky] {
            if !colour_ok(c) {
                return Err(SceneryError::Colour(format!("region {region}")));
            }
        }
        if let Some(fog) = self.fog {
            if !colour_ok(fog.color) {
                return Err(SceneryError::Colour(format!("region {region} fog")));
            }
            if !(0.0..=MAX_FOG_DENSITY).contains(&fog.density) {
                return Err(SceneryError::Fog(region.to_owned()));
            }
        }
        Ok(())
    }
}

/// Checks the decoration of a room of size `room`.
pub fn validate_decor(
    room_id: &str,
    room: (f32, f32),
    decor: &[Decor],
) -> Result<(), SceneryError> {
    for (index, d) in decor.iter().enumerate() {
        let room_s = || room_id.to_owned();
        let (x, y) = d.at;
        if !(x.is_finite()
            && y.is_finite()
            && (0.0..=room.0).contains(&x)
            && (0.0..=room.1).contains(&y))
        {
            return Err(SceneryError::Outside {
                room: room_s(),
                index,
            });
        }
        let max = room.0.max(room.1);
        if d.kind
            .dimensions()
            .iter()
            .any(|v| !(v.is_finite() && *v >= 1.0 && *v <= max))
        {
            return Err(SceneryError::Size {
                room: room_s(),
                index,
            });
        }
        if d.kind.colours().into_iter().any(|c| !colour_ok(c)) {
            return Err(SceneryError::Colour(format!(
                "room {room_id} decor #{index}"
            )));
        }
        if d.layer == DecorLayer::Front && !d.kind.may_be_front() {
            return Err(SceneryError::Front {
                room: room_s(),
                index,
            });
        }
    }
    Ok(())
}

/// Parallax placement: `factor` 1 = fixed in the world, 0 = fixed on the
/// screen. Returns the centre of the layer on one axis for a camera centre,
/// so that at the room centre every layer is centred on the room.
pub fn parallax_centre(camera: f32, room: f32, factor: f32) -> f32 {
    room / 2.0 + (camera - room / 2.0) * (1.0 - factor)
}

/// Length a parallax layer needs on one axis so the view never sees past
/// its ends for any camera centre the room limits allow (plus a margin).
pub fn parallax_extent(room: f32, view: f32, factor: f32, margin: f32) -> f32 {
    view + factor * (room - view).max(0.0) + 2.0 * margin
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::camera_rules::clamp_axis;

    fn torch(at: (f32, f32)) -> Decor {
        Decor {
            kind: DecorKind::Torch,
            at,
            layer: DecorLayer::Back,
            flip: false,
        }
    }

    #[test]
    fn decor_parses_with_defaults() {
        let d: Decor = ron::from_str("(kind: Pillar(height: 300.0), at: (200.0, 40.0))")
            .unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(d.layer, DecorLayer::Back);
        assert!(!d.flip);
        let v: RegionVisual = ron::from_str(
            "(backdrop: Castle, stone: (0.3, 0.3, 0.35), light: (1.0, 0.7, 0.4), sky: (0.05, 0.05, 0.1))",
        )
        .unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(v.fog, None);
    }

    #[test]
    fn invalid_decor_is_refused() {
        let room = (1280.0, 720.0);
        assert!(validate_decor("r", room, &[torch((640.0, 300.0))]).is_ok());
        assert!(matches!(
            validate_decor("r", room, &[torch((1400.0, 300.0))]),
            Err(SceneryError::Outside { .. })
        ));
        let huge = Decor {
            kind: DecorKind::Pillar { height: 5000.0 },
            ..torch((10.0, 40.0))
        };
        assert!(matches!(
            validate_decor("r", room, &[huge]),
            Err(SceneryError::Size { .. })
        ));
        let front_statue = Decor {
            kind: DecorKind::Statue,
            layer: DecorLayer::Front,
            ..torch((10.0, 40.0))
        };
        assert!(matches!(
            validate_decor("r", room, &[front_statue]),
            Err(SceneryError::Front { .. })
        ));
        let front_chain = Decor {
            kind: DecorKind::Chain { length: 120.0 },
            layer: DecorLayer::Front,
            ..torch((10.0, 680.0))
        };
        assert!(validate_decor("r", room, &[front_chain]).is_ok());
    }

    #[test]
    fn fog_and_colours_are_bounded() {
        let mut v = RegionVisual {
            backdrop: Backdrop::Ruins,
            stone: (0.3, 0.3, 0.3),
            light: (0.6, 0.9, 0.8),
            sky: (0.0, 0.05, 0.05),
            fog: Some(Fog {
                color: (0.5, 0.6, 0.6),
                density: 0.2,
            }),
        };
        assert!(v.validate("g").is_ok());
        v.fog = Some(Fog {
            color: (0.5, 0.6, 0.6),
            density: 0.8,
        });
        assert_eq!(v.validate("g"), Err(SceneryError::Fog("g".into())));
        v.fog = None;
        v.sky = (2.0, 0.0, 0.0);
        assert!(matches!(v.validate("g"), Err(SceneryError::Colour(_))));
    }

    #[test]
    fn parallax_layers_always_cover_the_view() {
        // For every camera position the room limits allow, the layer's
        // ends are outside the view (no background leak at the edges).
        for (room, view) in [
            (2560.0, 720.0),
            (1280.0, 720.0),
            (2160.0, 405.0),
            (600.0, 720.0),
        ] {
            for factor in [0.0, 0.15, 0.4, 0.8, 1.0] {
                let len = parallax_extent(room, view, factor, 8.0);
                for i in 0..=50 {
                    let cam = clamp_axis(room * i as f32 / 50.0, room, view);
                    let c = parallax_centre(cam, room, factor);
                    assert!(
                        c - len / 2.0 <= cam - view / 2.0 + 1e-3,
                        "{room} {factor} {cam}"
                    );
                    assert!(
                        c + len / 2.0 >= cam + view / 2.0 - 1e-3,
                        "{room} {factor} {cam}"
                    );
                }
            }
        }
    }

    #[test]
    fn parallax_moves_less_for_far_layers() {
        let near = parallax_centre(1500.0, 2560.0, 0.8) - parallax_centre(1000.0, 2560.0, 0.8);
        let far = parallax_centre(1500.0, 2560.0, 0.2) - parallax_centre(1000.0, 2560.0, 0.2);
        // Relative to the camera (500 u), the far layer drifts less.
        assert!((500.0 - far) < (500.0 - near));
        assert_eq!(
            parallax_centre(1280.0, 2560.0, 0.3),
            1280.0,
            "centred at the room centre"
        );
    }
}
