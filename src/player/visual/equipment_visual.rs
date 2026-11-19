//! Data-driven visual equipment (N4-1-A). Pure module: no Bevy types.
//!
//! The base is the **dressed** approved character (`CharacterComposition`
//! whose layers partition the approved frames). Equipment never needs a
//! nude body: an item is drawn OVER the base, may HIDE base layer slots it
//! fully covers, and may REPLACE base parts.
//!
//! - A [`VisualFamily`] holds what items of the same kind share: the layer
//!   slot they are drawn in, the animation profile (clips), the default
//!   HIDE / REPLACE rules, the regions covered, the shared masks (authoring
//!   data) and what to do when an item lacks a clip.
//! - An [`EquipmentVisual`] is one item: its sheets per clip (same canvas
//!   and pivot as the base clip), optional mirrored sheets, variants,
//!   rule overrides, palette name and per-frame order exceptions.
//! - [`equip`] validates families and items against the base and returns a
//!   composition where every item is an ordinary removable part, so the
//!   existing `resolve` and compositor draw it with the one animation clock.
//!
//! Adding an item of an existing family is data only (a RON manifest and
//! its sheets). Visual equipment is presentation: it never touches the
//! inventory, the item stats or the save. Details:
//! docs/development/n4/N41A_CONTRATO_RON.md.

use std::collections::{BTreeMap, BTreeSet};

use serde::Deserialize;
use thiserror::Error;

use super::composition::{
    CharacterComposition, CompositionError, OrderOverride, PartDef, PieceDef, Region, SheetRef,
};

/// Only version understood (families and items).
pub const EQUIPMENT_VISUAL_VERSION: u32 = 1;

/// What happens when an item has no sheet for a clip of its family profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
pub enum MissingClip {
    /// The item is not drawn in that clip (the base shows).
    #[default]
    Hide,
    /// The item is refused at load time.
    Refuse,
}

/// Rules and shared resources of a family of visual items.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct VisualFamily {
    pub version: u32,
    pub id: String,
    pub region: Region,
    /// Layer slot of the base composition the items are drawn in.
    pub layer: String,
    /// Animation profile: the clips every item is expected to cover.
    pub clips: Vec<String>,
    /// Default HIDE per clip (layer slots of the base).
    #[serde(default)]
    pub hide_layers: BTreeMap<String, Vec<String>>,
    /// Default REPLACE (base parts).
    #[serde(default)]
    pub replaces: Vec<String>,
    /// Regions the items cover (documentation and coverage checks).
    #[serde(default)]
    pub covers: Vec<Region>,
    /// Shared silhouette masks per clip (authoring data: the build tool
    /// paints every item of the family inside them; not used at runtime).
    #[serde(default)]
    pub masks: BTreeMap<String, SheetRef>,
    #[serde(default)]
    pub missing_clip: MissingClip,
    /// Draw-order exceptions shared by every item of the family (applied
    /// before the item's own `frame_order`).
    #[serde(default)]
    pub frame_order: Vec<FrameOrder>,
}

/// A per-frame draw-order exception of an item (its family slot moves to `z`).
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FrameOrder {
    pub clip: String,
    #[serde(default)]
    pub frames: Option<Vec<u32>>,
    pub z: i32,
}

/// One visual item.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct EquipmentVisual {
    pub version: u32,
    /// Usually the inventory item id it dresses (presentation only).
    pub id: String,
    pub family: String,
    pub sheets: BTreeMap<String, SheetRef>,
    /// Hand-drawn sheets for the mirrored facing (asymmetric items).
    #[serde(default)]
    pub mirrored: BTreeMap<String, SheetRef>,
    /// Alternative sheets (worn with `Outfit::wear(id, Some(name))`).
    #[serde(default)]
    pub variants: BTreeMap<String, BTreeMap<String, SheetRef>>,
    /// Overrides of the family HIDE per clip.
    #[serde(default)]
    pub hide_layers: Option<BTreeMap<String, Vec<String>>>,
    /// Overrides of the family REPLACE.
    #[serde(default)]
    pub replaces: Option<Vec<String>>,
    /// Palette name of the authoring recipe (documentation).
    #[serde(default)]
    pub palette: Option<String>,
    #[serde(default)]
    pub frame_order: Vec<FrameOrder>,
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum EquipError {
    #[error("cannot parse {what}: {message}")]
    Parse { what: String, message: String },
    #[error("{what}: unsupported version {version}")]
    Version { what: String, version: u32 },
    #[error("family {0} is declared twice")]
    DuplicateFamily(String),
    #[error("family {family}: layer slot {layer} is not in the base composition")]
    FamilyLayer { family: String, layer: String },
    #[error("family {family}: clip {clip} is not in the base composition")]
    FamilyClip { family: String, clip: String },
    #[error("item {item}: unknown family {family}")]
    UnknownFamily { item: String, family: String },
    #[error("item {item}: clip {clip} is not in its family profile")]
    ExtraClip { item: String, clip: String },
    #[error("item {item}: no sheet for clip {clip} (family refuses missing clips)")]
    MissingClip { item: String, clip: String },
    #[error("item {0} is declared twice or uses the id of a base part")]
    DuplicateItem(String),
    #[error("item {item}: {source}")]
    Composition {
        item: String,
        source: CompositionError,
    },
}

impl VisualFamily {
    pub fn from_ron(text: &str) -> Result<Self, EquipError> {
        let f: Self = ron::from_str(text).map_err(|e| EquipError::Parse {
            what: "visual family".into(),
            message: e.to_string(),
        })?;
        if f.version != EQUIPMENT_VISUAL_VERSION {
            return Err(EquipError::Version {
                what: f.id,
                version: f.version,
            });
        }
        Ok(f)
    }
}

impl EquipmentVisual {
    pub fn from_ron(text: &str) -> Result<Self, EquipError> {
        let i: Self = ron::from_str(text).map_err(|e| EquipError::Parse {
            what: "visual item".into(),
            message: e.to_string(),
        })?;
        if i.version != EQUIPMENT_VISUAL_VERSION {
            return Err(EquipError::Version {
                what: i.id,
                version: i.version,
            });
        }
        Ok(i)
    }
}

/// The base composition with every item added as a removable part (not
/// worn by default: `Outfit::wear(item_id, None)` puts it on). Families and
/// items are validated against the base first; the result is validated as
/// a whole, so a bad sheet, layer or rule is a descriptive error.
pub fn equip(
    base: &CharacterComposition,
    families: &[VisualFamily],
    items: &[EquipmentVisual],
) -> Result<CharacterComposition, EquipError> {
    let mut by_id: BTreeMap<&str, &VisualFamily> = BTreeMap::new();
    for f in families {
        if by_id.insert(f.id.as_str(), f).is_some() {
            return Err(EquipError::DuplicateFamily(f.id.clone()));
        }
        if !base.layers.contains(&f.layer) {
            return Err(EquipError::FamilyLayer {
                family: f.id.clone(),
                layer: f.layer.clone(),
            });
        }
        if let Some(clip) = f.clips.iter().find(|c| !base.clips.contains_key(*c)) {
            return Err(EquipError::FamilyClip {
                family: f.id.clone(),
                clip: clip.clone(),
            });
        }
    }
    let mut out = base.clone();
    let mut seen = BTreeSet::new();
    for item in items {
        if !seen.insert(item.id.as_str()) || base.parts.contains_key(&item.id) {
            return Err(EquipError::DuplicateItem(item.id.clone()));
        }
        let family = by_id
            .get(item.family.as_str())
            .ok_or_else(|| EquipError::UnknownFamily {
                item: item.id.clone(),
                family: item.family.clone(),
            })?;
        if let Some(clip) = item.sheets.keys().find(|c| !family.clips.contains(c)) {
            return Err(EquipError::ExtraClip {
                item: item.id.clone(),
                clip: clip.clone(),
            });
        }
        if family.missing_clip == MissingClip::Refuse
            && let Some(clip) = family.clips.iter().find(|c| !item.sheets.contains_key(*c))
        {
            return Err(EquipError::MissingClip {
                item: item.id.clone(),
                clip: clip.clone(),
            });
        }
        let piece =
            |sheets: &BTreeMap<String, SheetRef>, mirrored: &BTreeMap<String, SheetRef>| PieceDef {
                layer: family.layer.clone(),
                sheets: sheets.clone(),
                mirrored: mirrored.clone(),
            };
        let part = PartDef {
            region: family.region,
            removable: true,
            pieces: vec![piece(&item.sheets, &item.mirrored)],
            hides: Vec::new(),
            variants: item
                .variants
                .iter()
                .map(|(name, sheets)| (name.clone(), vec![piece(sheets, &BTreeMap::new())]))
                .collect(),
            socket: None,
            hide_layers: item
                .hide_layers
                .clone()
                .unwrap_or_else(|| family.hide_layers.clone()),
            replaces: item
                .replaces
                .clone()
                .unwrap_or_else(|| family.replaces.clone()),
        };
        out.parts.insert(item.id.clone(), part);
        out.order_overrides
            .extend(
                family
                    .frame_order
                    .iter()
                    .chain(&item.frame_order)
                    .map(|o| OrderOverride {
                        clip: o.clip.clone(),
                        frames: o.frames.clone(),
                        layer: family.layer.clone(),
                        z: o.z,
                    }),
            );
        out.validate().map_err(|source| EquipError::Composition {
            item: item.id.clone(),
            source,
        })?;
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::visual::composition::{Outfit, resolve, resolve_facing};

    const BASE: &str = r#"(
        version: 1,
        id: "base",
        clips: {
            "idle": (cell: (80, 128), pivot: (40.0, 118.0), density: 2, frames: 1),
            "run": (cell: (112, 128), pivot: (56.0, 118.0), density: 2, frames: 3),
        },
        layers: ["body", "shirt_torso", "shirt", "armor", "boots", "hair"],
        parts: {
            "body": (region: Body, removable: false, pieces: [
                (layer: "body", sheets: { "idle": (texture: "b.png"), "run": (texture: "br.png", columns: 3) }),
            ]),
            "shirt": (region: Shirt, pieces: [
                (layer: "shirt_torso", sheets: { "run": (texture: "st.png", columns: 3) }),
                (layer: "shirt", sheets: { "idle": (texture: "s.png"), "run": (texture: "ss.png", columns: 3) }),
            ]),
            "boots": (region: Boots, pieces: [
                (layer: "boots", sheets: { "idle": (texture: "bo.png") }),
            ]),
        },
        default_outfit: ["shirt", "boots"],
    )"#;

    const FAMILY: &str = r#"(
        version: 1,
        id: "torso_plate",
        region: Chest,
        layer: "armor",
        clips: ["idle", "run"],
        hide_layers: { "run": ["shirt_torso"] },
        covers: [Chest],
        masks: { "idle": (texture: "mask_idle.png"), "run": (texture: "mask_run.png", columns: 3) },
    )"#;

    fn item(id: &str, extra: &str) -> EquipmentVisual {
        EquipmentVisual::from_ron(&format!(
            r#"(version: 1, id: "{id}", family: "torso_plate",
                sheets: {{ "idle": (texture: "{id}_idle.png"), "run": (texture: "{id}_run.png", columns: 3) }},
                {extra})"#
        ))
        .unwrap_or_else(|e| panic!("{e}"))
    }

    fn base() -> CharacterComposition {
        CharacterComposition::from_ron(BASE).unwrap_or_else(|e| panic!("{e}"))
    }

    fn family() -> VisualFamily {
        VisualFamily::from_ron(FAMILY).unwrap_or_else(|e| panic!("{e}"))
    }

    fn shown(c: &CharacterComposition, o: &Outfit, clip: &str) -> Vec<(String, bool)> {
        resolve(c, o, clip, 0)
            .unwrap_or_else(|e| panic!("{e}"))
            .into_iter()
            .map(|d| (d.layer, d.visible))
            .collect()
    }

    #[test]
    fn two_items_of_one_family_need_only_data() {
        let c = equip(
            &base(),
            &[family()],
            &[
                item("iron", ""),
                item("bronze", "palette: Some(\"bronze\"),"),
            ],
        )
        .unwrap_or_else(|e| panic!("{e}"));
        for id in ["iron", "bronze"] {
            let mut o = Outfit::default_for(&c);
            o.wear(id, None);
            // OVERLAY in the family slot, HIDE of the run torso shirt only.
            let run = shown(&c, &o, "run");
            assert!(run.contains(&("armor".into(), true)));
            assert!(run.contains(&("shirt_torso".into(), false)), "{run:?}");
            assert!(run.contains(&("shirt".into(), true)), "sleeves stay");
            let idle = shown(&c, &o, "idle");
            assert!(idle.contains(&("shirt".into(), true)), "no idle HIDE");
        }
        // Not worn by default.
        let o = Outfit::default_for(&c);
        assert!(!shown(&c, &o, "run").iter().any(|(l, _)| l == "armor"));
    }

    #[test]
    fn replace_hides_the_base_part() {
        let mut f = family();
        f.replaces = vec!["boots".into()];
        let c = equip(&base(), &[f], &[item("greaves", "")]).unwrap_or_else(|e| panic!("{e}"));
        let mut o = Outfit::default_for(&c);
        o.wear("greaves", None);
        assert!(shown(&c, &o, "idle").contains(&("boots".into(), false)));
        // The item can override the family rule.
        let mut f = family();
        f.replaces = vec!["boots".into()];
        let c = equip(&base(), &[f], &[item("greaves", "replaces: Some([]),")])
            .unwrap_or_else(|e| panic!("{e}"));
        let mut o = Outfit::default_for(&c);
        o.wear("greaves", None);
        assert!(shown(&c, &o, "idle").contains(&("boots".into(), true)));
    }

    #[test]
    fn mirrored_sheets_are_used_when_the_body_is_flipped() {
        let c = equip(
            &base(),
            &[family()],
            &[item(
                "shield",
                r#"mirrored: { "run": (texture: "shield_run_left.png", columns: 3) },"#,
            )],
        )
        .unwrap_or_else(|e| panic!("{e}"));
        let mut o = Outfit::default_for(&c);
        o.wear("shield", None);
        let pick = |flip| {
            resolve_facing(&c, &o, "run", 1, flip)
                .unwrap_or_else(|e| panic!("{e}"))
                .into_iter()
                .find(|d| d.part == "shield")
                .unwrap_or_else(|| panic!("shield"))
        };
        let right = pick(false);
        assert_eq!(
            (right.texture.as_str(), right.mirrored),
            ("shield_run.png", false)
        );
        let left = pick(true);
        assert_eq!(
            (left.texture.as_str(), left.mirrored, left.index),
            ("shield_run_left.png", true, 1)
        );
    }

    #[test]
    fn missing_clips_follow_the_family_rule() {
        let partial = EquipmentVisual::from_ron(
            r#"(version: 1, id: "vest", family: "torso_plate", sheets: { "idle": (texture: "v.png") })"#,
        )
        .unwrap_or_else(|e| panic!("{e}"));
        // Hide (default): not drawn in the run, drawn in the idle.
        let c = equip(&base(), &[family()], std::slice::from_ref(&partial))
            .unwrap_or_else(|e| panic!("{e}"));
        let mut o = Outfit::default_for(&c);
        o.wear("vest", None);
        assert!(shown(&c, &o, "idle").contains(&("armor".into(), true)));
        assert!(!shown(&c, &o, "run").iter().any(|(l, _)| l == "armor"));
        // Refuse: a load error naming the item and the clip.
        let mut f = family();
        f.missing_clip = MissingClip::Refuse;
        assert_eq!(
            equip(&base(), &[f], &[partial]),
            Err(EquipError::MissingClip {
                item: "vest".into(),
                clip: "run".into()
            })
        );
    }

    #[test]
    fn frame_order_exceptions_move_the_family_slot() {
        let c = equip(
            &base(),
            &[family()],
            &[item(
                "iron",
                r#"frame_order: [(clip: "run", frames: Some([2]), z: 1)],"#,
            )],
        )
        .unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(c.z_at("armor", "run", 2), Some(1));
        assert_eq!(c.z_at("armor", "run", 1), Some(3));
    }

    #[test]
    fn family_frame_order_is_shared_and_ties_keep_the_layer_order() {
        let mut f = family();
        // Onto the z of "boots" (4): the declared order puts armor (3) first.
        f.frame_order = vec![FrameOrder {
            clip: "idle".into(),
            frames: None,
            z: 4,
        }];
        let c = equip(&base(), &[f], &[item("iron", ""), item("bronze", "")])
            .unwrap_or_else(|e| panic!("{e}"));
        let mut o = Outfit::default_for(&c);
        o.wear("iron", None);
        let order: Vec<String> = resolve(&c, &o, "idle", 0)
            .unwrap_or_else(|e| panic!("{e}"))
            .into_iter()
            .map(|d| d.layer)
            .collect();
        assert_eq!(order, ["body", "shirt", "armor", "boots"]);
    }

    #[test]
    fn invalid_data_is_refused_with_a_descriptive_error() {
        let b = base();
        let f = family();
        let bad_layer = VisualFamily {
            layer: "cape".into(),
            ..f.clone()
        };
        assert!(matches!(
            equip(&b, &[bad_layer], &[]),
            Err(EquipError::FamilyLayer { .. })
        ));
        let bad_clip = VisualFamily {
            clips: vec!["swim".into()],
            ..f.clone()
        };
        assert!(matches!(
            equip(&b, &[bad_clip], &[]),
            Err(EquipError::FamilyClip { .. })
        ));
        assert!(matches!(
            equip(&b, &[f.clone(), f.clone()], &[]),
            Err(EquipError::DuplicateFamily(_))
        ));
        let orphan = EquipmentVisual {
            family: "helmet".into(),
            ..item("x", "")
        };
        assert!(matches!(
            equip(&b, std::slice::from_ref(&f), &[orphan]),
            Err(EquipError::UnknownFamily { .. })
        ));
        assert!(matches!(
            equip(
                &b,
                std::slice::from_ref(&f),
                &[item("x", ""), item("x", "")]
            ),
            Err(EquipError::DuplicateItem(_))
        ));
        assert!(matches!(
            equip(&b, std::slice::from_ref(&f), &[item("boots", "")]),
            Err(EquipError::DuplicateItem(_))
        ));
        // HIDE of a layer the base does not have, or of the item's own slot.
        for hide in [
            r#"hide_layers: Some({ "run": ["cape"] }),"#,
            r#"hide_layers: Some({ "run": ["armor"] }),"#,
        ] {
            assert!(matches!(
                equip(&b, std::slice::from_ref(&f), &[item("x", hide)]),
                Err(EquipError::Composition { .. })
            ));
        }
        // REPLACE of the base body.
        assert!(matches!(
            equip(
                &b,
                std::slice::from_ref(&f),
                &[item("x", r#"replaces: Some(["body"]),"#)]
            ),
            Err(EquipError::Composition { .. })
        ));
        // Sheets past the strip.
        let short = EquipmentVisual::from_ron(
            r#"(version: 1, id: "x", family: "torso_plate", sheets: { "run": (texture: "x.png", columns: 2) })"#,
        )
        .unwrap_or_else(|e| panic!("{e}"));
        assert!(matches!(
            equip(&b, std::slice::from_ref(&f), &[short]),
            Err(EquipError::Composition { .. })
        ));
        assert!(matches!(
            VisualFamily::from_ron(&FAMILY.replace("version: 1", "version: 4")),
            Err(EquipError::Version { .. })
        ));
        assert!(matches!(
            EquipmentVisual::from_ron("("),
            Err(EquipError::Parse { .. })
        ));
    }
}
