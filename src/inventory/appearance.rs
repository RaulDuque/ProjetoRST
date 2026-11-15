//! Equipment appearance (V2.0): what the equipped items change on the
//! character's look, resolved from data without any per-combination art.
//!
//! Pure module: no Bevy types. The hybrid model of
//! docs/EQUIPMENT_VISUAL_SYSTEM.md, as far as the published atlases allow:
//! the rendered body stays the base, and items add light runtime layers:
//!
//! - [`ItemVisual::Trail`] (weapons): colour of the strike trail; its shape
//!   comes from the weapon class (reach, height, gap);
//! - [`ItemVisual::Cape`] (body pieces): a cape layer behind the body;
//! - [`ItemVisual::Tint`] (head and body pieces): a discreet colour grade of
//!   the body sprite (two tints multiply).
//!
//! Items without `visual` change attributes only. Details:
//! docs/EQUIPMENT_EXPANSION_V2_0.md §14-15.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::equipment::{Equipment, EquipmentSlot};
use super::items::{ItemCatalog, ItemCategory};

/// Colour of the pre-V2.0 strike (white): the starting sword keeps it.
pub const DEFAULT_TRAIL: Rgb = Rgb(1.0, 1.0, 1.0);
/// Shortest and longest cape (world units below the shoulders).
pub const CAPE_LENGTH_RANGE: (f32, f32) = (12.0, 44.0);
/// Darkest tint channel: a tint never blackens the sprite.
pub const MIN_TINT: f32 = 0.45;

/// An sRGB colour, each channel in `[0, 1]`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rgb(pub f32, pub f32, pub f32);

impl Rgb {
    pub const WHITE: Rgb = Rgb(1.0, 1.0, 1.0);

    fn channels(self) -> [f32; 3] {
        [self.0, self.1, self.2]
    }

    pub fn is_valid(self) -> bool {
        self.channels()
            .iter()
            .all(|c| c.is_finite() && (0.0..=1.0).contains(c))
    }

    pub fn multiply(self, other: Rgb) -> Rgb {
        Rgb(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

/// The visual of one item, in data (`visual: Some(Cape(...))`).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ItemVisual {
    Trail { color: Rgb },
    Cape { color: Rgb, length: f32 },
    Tint { color: Rgb },
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum VisualError {
    #[error("item `{item}`: visual {visual} does not fit category {category:?}")]
    WrongCategory {
        item: String,
        visual: &'static str,
        category: ItemCategory,
    },
    #[error("item `{item}`: invalid visual value ({problem})")]
    Invalid { item: String, problem: &'static str },
}

impl ItemVisual {
    fn name(self) -> &'static str {
        match self {
            ItemVisual::Trail { .. } => "Trail",
            ItemVisual::Cape { .. } => "Cape",
            ItemVisual::Tint { .. } => "Tint",
        }
    }

    /// Which categories can carry this visual, and sane values.
    pub fn validate(self, item: &str, category: ItemCategory) -> Result<(), VisualError> {
        let fits = match self {
            ItemVisual::Trail { .. } => category.is_weapon(),
            ItemVisual::Cape { .. } => category == ItemCategory::Armor,
            ItemVisual::Tint { .. } => {
                matches!(category, ItemCategory::Armor | ItemCategory::Helmet)
            }
        };
        if !fits {
            return Err(VisualError::WrongCategory {
                item: item.to_owned(),
                visual: self.name(),
                category,
            });
        }
        let invalid = |problem| {
            Err(VisualError::Invalid {
                item: item.to_owned(),
                problem,
            })
        };
        match self {
            ItemVisual::Trail { color } | ItemVisual::Cape { color, .. } if !color.is_valid() => {
                invalid("colour channel outside 0..1")
            }
            ItemVisual::Cape { length, .. }
                if !(CAPE_LENGTH_RANGE.0..=CAPE_LENGTH_RANGE.1).contains(&length) =>
            {
                invalid("cape length out of range")
            }
            ItemVisual::Tint { color }
                if !color.is_valid() || color.channels().iter().any(|c| *c < MIN_TINT) =>
            {
                invalid("tint channel outside MIN_TINT..1")
            }
            _ => Ok(()),
        }
    }
}

/// The cape layer, when a body piece has one.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CapeLook {
    pub color: Rgb,
    pub length: f32,
}

/// The resolved look of a set of equipment.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Appearance {
    /// Strike trail colour (white without a weapon visual).
    pub trail: Rgb,
    pub cape: Option<CapeLook>,
    /// Body tint (white = untouched sprite).
    pub tint: Rgb,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            trail: DEFAULT_TRAIL,
            cape: None,
            tint: Rgb::WHITE,
        }
    }
}

impl Appearance {
    /// Reads the visuals of the equipped items. Deterministic: slots are
    /// visited in [`EquipmentSlot::ALL`] order; unknown items are ignored.
    pub fn resolve(equipment: &Equipment, catalog: &ItemCatalog) -> Self {
        let mut look = Appearance::default();
        for slot in EquipmentSlot::ALL {
            let Some(def) = equipment.get(slot).and_then(|id| catalog.get(id)) else {
                continue;
            };
            match def.visual {
                Some(ItemVisual::Trail { color }) if slot == EquipmentSlot::Weapon => {
                    look.trail = color;
                }
                Some(ItemVisual::Cape { color, length }) if slot == EquipmentSlot::Body => {
                    look.cape = Some(CapeLook { color, length });
                }
                Some(ItemVisual::Tint { color })
                    if matches!(slot, EquipmentSlot::Head | EquipmentSlot::Body) =>
                {
                    look.tint = look.tint.multiply(color);
                }
                _ => {}
            }
        }
        look
    }

    /// Number of visible differences from the bare look (for reports).
    pub fn layer_count(&self) -> usize {
        usize::from(self.trail != DEFAULT_TRAIL)
            + usize::from(self.cape.is_some())
            + usize::from(self.tint != Rgb::WHITE)
    }
}
