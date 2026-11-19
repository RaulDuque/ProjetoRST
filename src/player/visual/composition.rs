//! Data-driven sprite composition (N4-A). Pure module: no Bevy types.
//!
//! A character is drawn as **layers** that share one animation state: the
//! same clip, frame and facing as the body (`VisualPlayback`), on the same
//! canvas and pivot. Removable **parts** (hair, shirt, trousers, boots, ...)
//! each own one or more layers, may hide other regions while worn and may
//! offer variants. No sheet per combination: one sheet per part and clip.
//!
//! The format is RON (`CharacterComposition::from_ron`); the first file is
//! `art-src/n4-sprite-machine/stage/protagonist.ron` (isolated proof, not
//! the production art). Details: docs/development/n4/N4A_CONTRATO_DADOS.md.

use std::collections::{BTreeMap, BTreeSet};

use serde::Deserialize;
use thiserror::Error;

/// Only version understood.
pub const COMPOSITION_VERSION: u32 = 1;

/// Body regions a part covers (what other parts can hide).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub enum Region {
    /// The neutral anatomical body (never removable).
    Body,
    HairBack,
    HairFront,
    Shirt,
    Trousers,
    Boots,
    Bracers,
    Gloves,
    Coat,
    Chest,
    Shoulders,
    ArmArmor,
    LegArmor,
    Helmet,
    Hood,
    CapeBack,
    CapeFront,
    MainWeapon,
    OffHand,
    Accessory,
}

/// Canvas, pivot and frames of one clip: every layer of the clip uses them.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CompositeClip {
    /// Cell in texels (80x128 idle, 112x128 run).
    pub cell: (u32, u32),
    /// Sole-line pivot in texels from the cell's top-left corner.
    pub pivot: (f32, f32),
    /// Texels per world unit.
    pub density: u32,
    pub frames: u32,
}

/// Where the frames of one layer are for one clip: a horizontal strip (or
/// grid) of `cell`-sized frames, `first` being the clip's frame 0.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SheetRef {
    pub texture: String,
    #[serde(default)]
    pub first: u32,
    #[serde(default = "one")]
    pub columns: u32,
    #[serde(default = "one")]
    pub rows: u32,
}

fn one() -> u32 {
    1
}

/// One layer contributed by a part: the layer slot and its sheet per clip.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PieceDef {
    pub layer: String,
    pub sheets: BTreeMap<String, SheetRef>,
}

/// A removable (or base) visual part.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PartDef {
    pub region: Region,
    #[serde(default = "yes")]
    pub removable: bool,
    pub pieces: Vec<PieceDef>,
    /// Regions hidden while this part is worn and visible (helmet -> hair).
    #[serde(default)]
    pub hides: Vec<Region>,
    /// Alternative pieces (a flattened hair under a helmet, ...).
    #[serde(default)]
    pub variants: BTreeMap<String, Vec<PieceDef>>,
    /// Socket this part follows, when it is not drawn on the body canvas
    /// (weapons and shields; see `sockets.rs`).
    #[serde(default)]
    pub socket: Option<String>,
}

fn yes() -> bool {
    true
}

/// A draw-order exception: `layer` moves to `z` in `clip` (all frames, or
/// only `frames`). Prepared for clips where a piece passes behind the body.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OrderOverride {
    pub clip: String,
    #[serde(default)]
    pub frames: Option<Vec<u32>>,
    pub layer: String,
    pub z: i32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CharacterComposition {
    pub version: u32,
    pub id: String,
    pub clips: BTreeMap<String, CompositeClip>,
    /// Layer slots, back to front: the index is the default z.
    pub layers: Vec<String>,
    pub parts: BTreeMap<String, PartDef>,
    /// Parts worn by default (base parts are always worn).
    pub default_outfit: Vec<String>,
    #[serde(default)]
    pub order_overrides: Vec<OrderOverride>,
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum CompositionError {
    #[error("cannot parse the composition: {0}")]
    Parse(String),
    #[error("unsupported composition version {0}")]
    Version(u32),
    #[error("layer {0} is declared twice")]
    DuplicateLayer(String),
    #[error("part {part}: unknown layer {layer}")]
    UnknownLayer { part: String, layer: String },
    #[error("part {part}: unknown clip {clip}")]
    UnknownClip { part: String, clip: String },
    #[error("part {part}, clip {clip}: frames {first}..{end} do not fit the {cells}-cell sheet")]
    FrameRange {
        part: String,
        clip: String,
        first: u32,
        end: u32,
        cells: u32,
    },
    #[error("clip {0}: invalid canvas, pivot, density or frame count")]
    Clip(String),
    #[error("part {0}: a part without pieces")]
    Empty(String),
    #[error("part {0} hides its own region")]
    HidesItself(String),
    #[error("no base part: exactly one non-removable part with region Body is required")]
    NoBase,
    #[error("outfit names unknown part {0}")]
    UnknownPart(String),
    #[error("outfit picks unknown variant {variant} of part {part}")]
    UnknownVariant { part: String, variant: String },
    #[error("order override: unknown clip {clip} or layer {layer}")]
    Override { clip: String, layer: String },
}

impl CharacterComposition {
    pub fn from_ron(text: &str) -> Result<Self, CompositionError> {
        let c: Self = ron::from_str(text).map_err(|e| CompositionError::Parse(e.to_string()))?;
        c.validate()?;
        Ok(c)
    }

    pub fn validate(&self) -> Result<(), CompositionError> {
        if self.version != COMPOSITION_VERSION {
            return Err(CompositionError::Version(self.version));
        }
        let mut seen = BTreeSet::new();
        for layer in &self.layers {
            if !seen.insert(layer.as_str()) {
                return Err(CompositionError::DuplicateLayer(layer.clone()));
            }
        }
        for (id, clip) in &self.clips {
            let pivot_inside = clip.pivot.0 >= 0.0
                && clip.pivot.1 >= 0.0
                && clip.pivot.0 <= clip.cell.0 as f32
                && clip.pivot.1 <= clip.cell.1 as f32;
            if clip.cell.0 == 0 || clip.cell.1 == 0 || clip.density == 0 || clip.frames == 0 {
                return Err(CompositionError::Clip(id.clone()));
            }
            if !pivot_inside {
                return Err(CompositionError::Clip(id.clone()));
            }
        }
        let bases = self
            .parts
            .values()
            .filter(|p| !p.removable && p.region == Region::Body)
            .count();
        if bases != 1 {
            return Err(CompositionError::NoBase);
        }
        for (id, part) in &self.parts {
            if part.pieces.is_empty() {
                return Err(CompositionError::Empty(id.clone()));
            }
            if part.hides.contains(&part.region) {
                return Err(CompositionError::HidesItself(id.clone()));
            }
            let all = part.pieces.iter().chain(part.variants.values().flatten());
            for piece in all {
                self.check_piece(id, piece)?;
            }
        }
        for id in &self.default_outfit {
            if !self.parts.contains_key(id) {
                return Err(CompositionError::UnknownPart(id.clone()));
            }
        }
        for o in &self.order_overrides {
            if !self.clips.contains_key(&o.clip) || !self.layers.contains(&o.layer) {
                return Err(CompositionError::Override {
                    clip: o.clip.clone(),
                    layer: o.layer.clone(),
                });
            }
        }
        Ok(())
    }

    fn check_piece(&self, part: &str, piece: &PieceDef) -> Result<(), CompositionError> {
        if !self.layers.contains(&piece.layer) {
            return Err(CompositionError::UnknownLayer {
                part: part.into(),
                layer: piece.layer.clone(),
            });
        }
        for (clip_id, sheet) in &piece.sheets {
            let clip = self
                .clips
                .get(clip_id)
                .ok_or_else(|| CompositionError::UnknownClip {
                    part: part.into(),
                    clip: clip_id.clone(),
                })?;
            let cells = sheet.columns * sheet.rows;
            let end = sheet.first + clip.frames;
            if sheet.columns == 0 || sheet.rows == 0 || end > cells {
                return Err(CompositionError::FrameRange {
                    part: part.into(),
                    clip: clip_id.clone(),
                    first: sheet.first,
                    end,
                    cells,
                });
            }
        }
        Ok(())
    }

    /// The base (anatomical body) part id.
    pub fn base_part(&self) -> Option<&str> {
        self.parts
            .iter()
            .find(|(_, p)| !p.removable && p.region == Region::Body)
            .map(|(id, _)| id.as_str())
    }

    /// Default z of a layer slot.
    pub fn layer_z(&self, layer: &str) -> Option<i32> {
        self.layers
            .iter()
            .position(|l| l == layer)
            .and_then(|i| i32::try_from(i).ok())
    }

    /// z of a layer in a clip and frame, after the order overrides.
    pub fn z_at(&self, layer: &str, clip: &str, frame: u32) -> Option<i32> {
        let base = self.layer_z(layer)?;
        let over = self.order_overrides.iter().rev().find(|o| {
            o.clip == clip
                && o.layer == layer
                && o.frames.as_ref().is_none_or(|f| f.contains(&frame))
        });
        Some(over.map_or(base, |o| o.z))
    }
}

/// What a character wears and shows: parts (with an optional variant) and
/// parts that are worn but switched off (inspection, cosmetics later).
/// Presentation only: never saved, never the inventory.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Outfit {
    pub parts: BTreeMap<String, Option<String>>,
    pub hidden: BTreeSet<String>,
}

impl Outfit {
    /// The composition's default outfit (plus the base part).
    pub fn default_for(c: &CharacterComposition) -> Self {
        let mut parts: BTreeMap<String, Option<String>> = c
            .default_outfit
            .iter()
            .map(|id| (id.clone(), None))
            .collect();
        if let Some(base) = c.base_part() {
            parts.entry(base.to_owned()).or_insert(None);
        }
        Self {
            parts,
            hidden: BTreeSet::new(),
        }
    }

    /// Removes a part (the base part cannot be removed).
    pub fn remove(&mut self, c: &CharacterComposition, part: &str) -> bool {
        if c.parts.get(part).is_some_and(|p| p.removable) {
            return self.parts.remove(part).is_some();
        }
        false
    }

    pub fn wear(&mut self, part: &str, variant: Option<&str>) {
        self.parts
            .insert(part.to_owned(), variant.map(str::to_owned));
    }

    /// Shows or hides a worn part without removing it.
    pub fn set_visible(&mut self, part: &str, visible: bool) {
        if visible {
            self.hidden.remove(part);
        } else {
            self.hidden.insert(part.to_owned());
        }
    }
}

/// One layer to draw for a clip and frame.
#[derive(Debug, Clone, PartialEq)]
pub struct DrawLayer {
    pub part: String,
    pub layer: String,
    pub z: i32,
    pub texture: String,
    /// Index in the sheet grid: `first + frame`.
    pub index: u32,
    pub visible: bool,
}

/// Resolves an outfit into the layers of one clip and frame, sorted back to
/// front. Every layer uses the same `frame`: there is no per-layer clock.
/// A part is invisible when switched off or when a visible worn part hides
/// its region. Parts without a sheet for the clip are skipped.
pub fn resolve(
    c: &CharacterComposition,
    outfit: &Outfit,
    clip: &str,
    frame: u32,
) -> Result<Vec<DrawLayer>, CompositionError> {
    let Some(clip_def) = c.clips.get(clip) else {
        return Err(CompositionError::UnknownClip {
            part: c.id.clone(),
            clip: clip.into(),
        });
    };
    let frame = frame.min(clip_def.frames.saturating_sub(1));
    let mut worn = outfit.parts.clone();
    if let Some(base) = c.base_part() {
        worn.entry(base.to_owned()).or_insert(None);
    }
    let mut hidden_regions = BTreeSet::new();
    for id in worn.keys() {
        let part = c
            .parts
            .get(id)
            .ok_or_else(|| CompositionError::UnknownPart(id.clone()))?;
        if !outfit.hidden.contains(id) {
            hidden_regions.extend(part.hides.iter().copied());
        }
    }
    let mut out = Vec::new();
    for (id, variant) in &worn {
        let part = &c.parts[id];
        let pieces = match variant {
            None => &part.pieces,
            Some(v) => part
                .variants
                .get(v)
                .ok_or_else(|| CompositionError::UnknownVariant {
                    part: id.clone(),
                    variant: v.clone(),
                })?,
        };
        let visible = !outfit.hidden.contains(id)
            && (part.region == Region::Body || !hidden_regions.contains(&part.region));
        for piece in pieces {
            let Some(sheet) = piece.sheets.get(clip) else {
                continue;
            };
            let z = c.z_at(&piece.layer, clip, frame).unwrap_or(0);
            out.push(DrawLayer {
                part: id.clone(),
                layer: piece.layer.clone(),
                z,
                texture: sheet.texture.clone(),
                index: sheet.first + frame,
                visible,
            });
        }
    }
    out.sort_by(|a, b| a.z.cmp(&b.z).then_with(|| a.layer.cmp(&b.layer)));
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"(
        version: 1,
        id: "sample",
        clips: {
            "idle": (cell: (80, 128), pivot: (40.0, 118.0), density: 2, frames: 1),
            "run": (cell: (112, 128), pivot: (56.0, 118.0), density: 2, frames: 6),
        },
        layers: ["body", "trousers", "shirt", "hair"],
        parts: {
            "body": (region: Body, removable: false, pieces: [
                (layer: "body", sheets: {
                    "idle": (texture: "body_idle.png"),
                    "run": (texture: "body_run.png", columns: 6),
                }),
            ]),
            "trousers": (region: Trousers, pieces: [
                (layer: "trousers", sheets: { "idle": (texture: "trousers.png") }),
            ]),
            "shirt": (region: Shirt, pieces: [
                (layer: "shirt", sheets: { "idle": (texture: "shirt.png") }),
            ]),
            "hair": (region: HairBack, pieces: [
                (layer: "hair", sheets: {
                    "idle": (texture: "hair.png"),
                    "run": (texture: "hair_run.png", columns: 6),
                }),
            ], variants: {
                "short": [(layer: "hair", sheets: { "idle": (texture: "hair_short.png") })],
            }),
            "helmet": (region: Helmet, pieces: [
                (layer: "hair", sheets: { "idle": (texture: "helmet.png") }),
            ], hides: [HairBack]),
        },
        default_outfit: ["trousers", "shirt", "hair"],
        order_overrides: [(clip: "run", frames: Some([3]), layer: "shirt", z: -1)],
    )"#;

    fn sample() -> CharacterComposition {
        CharacterComposition::from_ron(SAMPLE).unwrap_or_else(|e| panic!("{e}"))
    }

    fn names(layers: &[DrawLayer]) -> Vec<(&str, bool)> {
        layers
            .iter()
            .map(|l| (l.part.as_str(), l.visible))
            .collect()
    }

    #[test]
    fn the_default_outfit_draws_back_to_front() {
        let c = sample();
        let l = resolve(&c, &Outfit::default_for(&c), "idle", 0).unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(
            names(&l),
            [
                ("body", true),
                ("trousers", true),
                ("shirt", true),
                ("hair", true)
            ]
        );
        assert!(l.windows(2).all(|w| w[0].z <= w[1].z));
    }

    #[test]
    fn every_layer_uses_the_same_frame() {
        let c = sample();
        for frame in 0..6 {
            let l = resolve(&c, &Outfit::default_for(&c), "run", frame)
                .unwrap_or_else(|e| panic!("{e}"));
            // Parts without a run sheet are skipped, never drawn on another clock.
            assert_eq!(names(&l), [("body", true), ("hair", true)]);
            assert!(l.iter().all(|d| d.index == frame));
        }
        // Past the end: clamped to the last frame, not another frame per layer.
        let l = resolve(&c, &Outfit::default_for(&c), "run", 40).unwrap_or_else(|e| panic!("{e}"));
        assert!(l.iter().all(|d| d.index == 5));
    }

    #[test]
    fn removing_and_hiding_parts() {
        let c = sample();
        let mut o = Outfit::default_for(&c);
        assert!(o.remove(&c, "shirt"));
        assert!(!o.remove(&c, "body"), "the base body cannot be removed");
        o.set_visible("hair", false);
        let l = resolve(&c, &o, "idle", 0).unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(
            names(&l),
            [("body", true), ("trousers", true), ("hair", false)]
        );
    }

    #[test]
    fn a_worn_part_hides_the_regions_it_covers() {
        let c = sample();
        let mut o = Outfit::default_for(&c);
        o.wear("helmet", None);
        let l = resolve(&c, &o, "idle", 0).unwrap_or_else(|e| panic!("{e}"));
        let hair = l
            .iter()
            .find(|d| d.part == "hair")
            .unwrap_or_else(|| panic!("hair"));
        assert!(!hair.visible, "hair under the helmet");
        // A switched-off helmet hides nothing.
        o.set_visible("helmet", false);
        let l = resolve(&c, &o, "idle", 0).unwrap_or_else(|e| panic!("{e}"));
        assert!(l.iter().any(|d| d.part == "hair" && d.visible));
    }

    #[test]
    fn variants_swap_the_pieces() {
        let c = sample();
        let mut o = Outfit::default_for(&c);
        o.wear("hair", Some("short"));
        let l = resolve(&c, &o, "idle", 0).unwrap_or_else(|e| panic!("{e}"));
        assert!(l.iter().any(|d| d.texture == "hair_short.png"));
        o.wear("hair", Some("long"));
        assert!(matches!(
            resolve(&c, &o, "idle", 0),
            Err(CompositionError::UnknownVariant { .. })
        ));
    }

    #[test]
    fn order_overrides_apply_to_their_clip_and_frames() {
        let c = sample();
        assert_eq!(c.z_at("shirt", "idle", 0), Some(2));
        assert_eq!(c.z_at("shirt", "run", 3), Some(-1));
        assert_eq!(c.z_at("shirt", "run", 2), Some(2));
    }

    #[test]
    fn invalid_compositions_are_refused() {
        let cases: [(&str, &str); 7] = [
            ("version: 1,", "version: 2,"),
            (
                r#"layers: ["body", "trousers", "shirt", "hair"]"#,
                r#"layers: ["body", "body", "shirt", "hair"]"#,
            ),
            (r#"(layer: "shirt", sheets"#, r#"(layer: "cape", sheets"#),
            (r#"removable: false"#, r#"removable: true"#),
            (r#"hides: [HairBack]"#, r#"hides: [Helmet]"#),
            (
                r#""run": (texture: "hair_run.png", columns: 6)"#,
                r#""run": (texture: "hair_run.png", columns: 4)"#,
            ),
            (
                r#"default_outfit: ["trousers""#,
                r#"default_outfit: ["gloves""#,
            ),
        ];
        for (from, to) in cases {
            assert!(SAMPLE.contains(from), "{from}");
            let text = SAMPLE.replacen(from, to, 1);
            assert!(
                CharacterComposition::from_ron(&text).is_err(),
                "accepted after {from} -> {to}"
            );
        }
        assert!(matches!(
            CharacterComposition::from_ron("("),
            Err(CompositionError::Parse(_))
        ));
    }

    #[test]
    fn unknown_clips_and_parts_are_errors_not_silent() {
        let c = sample();
        let o = Outfit::default_for(&c);
        assert!(resolve(&c, &o, "swim", 0).is_err());
        let mut o = o;
        o.wear("wings", None);
        assert!(matches!(
            resolve(&c, &o, "idle", 0),
            Err(CompositionError::UnknownPart(_))
        ));
    }
}
