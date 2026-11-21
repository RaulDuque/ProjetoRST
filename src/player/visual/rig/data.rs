//! The lab's rig data (RON): marked skeletons of the approved frames, the
//! clips that play them and the rig-bound items. Pure module.
//!
//! Joints are written as the texel coordinates read on the approved frame
//! (x right, **y down** from the cell's top-left corner, as in an image
//! editor) and converted to rig space on load. File:
//! `art-src/modular-character-engine/stage/rig_lab.ron`.

use std::collections::BTreeMap;

use serde::Deserialize;
use thiserror::Error;

use super::chest::ChestModel;
use super::equipment::{ItemArt, ItemProblem, RigItem, RigSlot};
use super::grip::GripProfile;
use super::layers::{
    BodyLayer, CellGeometry, extract_cell, masked_rgba, material_of, parse_labels, partition,
};
use super::skeleton::{Joint, JointPositions};

/// Only version understood.
pub const RIG_LAB_VERSION: u32 = 1;

/// One clip: a strip of approved frames, the joints marked on each and the
/// frames played (a frame may reuse a source with the upper body lowered).
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct RigClip {
    /// Asset path of the approved strip (read only).
    pub texture: String,
    /// Cell in texels.
    pub cell: (u32, u32),
    /// Columns of the strip.
    pub columns: u32,
    /// Sole-line pivot in texels from the cell's top-left corner.
    pub pivot: (f32, f32),
    /// Marked joints of every column, texel coordinates (y down).
    pub sources: Vec<BTreeMap<Joint, (f32, f32)>>,
    pub frames: Vec<RigFrame>,
    /// Grip profile of the held items in this clip (`RigLab::grips`).
    #[serde(default)]
    pub grip: Option<String>,
    /// Optional authored label map per source (path relative to the stage,
    /// `layers::parse_labels`); empty or `None` = the rule partition.
    #[serde(default)]
    pub labels: Vec<Option<String>>,
}

/// One played frame.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct RigFrame {
    /// Column of the strip.
    pub source: usize,
    /// Texels the upper body is lowered (idle breath; 0 for approved poses).
    pub drop: u32,
    /// Seconds.
    pub duration: f32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct RigLab {
    pub version: u32,
    pub clips: BTreeMap<String, RigClip>,
    /// The clip whose first frame is the rest pose items are authored on.
    pub rest_clip: String,
    pub items: BTreeMap<String, RigItem>,
    /// Items cycled per slot by the lab (variant A, variant B, ...).
    pub variants: BTreeMap<RigSlot, Vec<String>>,
    /// Reusable grip profiles, by id.
    #[serde(default)]
    pub grips: BTreeMap<String, GripProfile>,
    /// Vector models of torso equipment, by id.
    #[serde(default)]
    pub models: BTreeMap<String, ChestModel>,
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum RigLabError {
    #[error("cannot parse the rig lab: {0}")]
    Parse(String),
    #[error("unsupported rig lab version {0}")]
    Version(u32),
    #[error("clip {clip}: source {source_index} lacks joint {joint:?}")]
    MissingJoint {
        clip: String,
        source_index: usize,
        joint: Joint,
    },
    #[error("clip {clip}: {sources} marked sources for {columns} columns")]
    Sources {
        clip: String,
        sources: usize,
        columns: u32,
    },
    #[error("clip {clip}: frame {frame} is invalid (source out of range or bad duration)")]
    Frame { clip: String, frame: usize },
    #[error("clip {0}: joint outside the cell")]
    JointOutside(String),
    #[error("rest clip {0} does not exist")]
    RestClip(String),
    #[error("item {0}: {1:?}")]
    Item(String, ItemProblem),
    #[error("item {0}: unknown model {1}")]
    Model(String, String),
    #[error("clip {0}: unknown grip profile {1}")]
    Grip(String, String),
    #[error("clip {0}: grip depth or label list does not match the clip")]
    ClipLists(String),
    #[error("grip {0}: follow_forearm outside 0..=1")]
    GripFollow(String),
    #[error("variant {0} of slot {1:?}: unknown item or item of another slot")]
    Variant(String, RigSlot),
}

impl RigClip {
    pub fn geometry(&self) -> CellGeometry {
        CellGeometry {
            width: self.cell.0 as usize,
            height: self.cell.1 as usize,
            pivot: self.pivot,
        }
    }

    /// Joints of a source column in rig space.
    pub fn source_joints(&self, source: usize) -> Option<JointPositions> {
        let marked = self.sources.get(source)?;
        let g = self.geometry();
        let map: BTreeMap<Joint, (f32, f32)> =
            marked.iter().map(|(j, t)| (*j, g.to_rig(*t))).collect();
        JointPositions::from_map(&map).ok()
    }

    /// Joints of a played frame in rig space (breath applied).
    pub fn frame_joints(&self, frame: usize) -> Option<JointPositions> {
        let f = self.frames.get(frame)?;
        Some(self.source_joints(f.source)?.with_drop(f.drop as f32))
    }

    pub fn durations(&self) -> Vec<f32> {
        self.frames.iter().map(|f| f.duration).collect()
    }
}

/// The depth layers of one source column (RGBA8 per layer, cell-sized).
#[derive(Debug, Clone, PartialEq)]
pub struct SourceLayers {
    pub source: usize,
    pub layers: Vec<(BodyLayer, Vec<u8>)>,
    /// Texels where the recomposed layers differ from the approved frame
    /// (0 for a valid partition).
    pub differing: usize,
    /// The labels came from an authored map (N4-C) rather than the rules.
    pub authored: bool,
}

impl RigClip {
    /// Cuts every source column of the approved strip into its depth layers:
    /// the authored label map when the clip names one (read through
    /// `read_labels`, a path relative to the stage), the rule partition
    /// otherwise. Shared by the lab and the Equipment Studio.
    pub fn cut_layers(
        &self,
        strip: &[u8],
        strip_width: usize,
        read_labels: impl Fn(&str) -> Option<String>,
    ) -> Result<Vec<SourceLayers>, String> {
        let cell = self.geometry();
        let mut out = Vec::new();
        for source in 0..self.sources.len() {
            let texels = extract_cell(strip, strip_width, source, cell)
                .ok_or_else(|| format!("source {source}: the strip is too small"))?;
            let joints = self
                .source_joints(source)
                .ok_or_else(|| format!("source {source}: joints missing"))?;
            let materials: Vec<_> = texels.iter().map(|t| material_of(*t)).collect();
            let authored = self.labels.get(source).cloned().flatten();
            let labels = match &authored {
                Some(path) => {
                    let text = read_labels(path).ok_or_else(|| format!("cannot read {path}"))?;
                    parse_labels(&text, cell).map_err(|e| format!("{path}: {e:?}"))?
                }
                None => partition(&materials, cell, &joints),
            };
            let mut union = vec![[0u8; 4]; texels.len()];
            let mut layers = Vec::new();
            for layer in BodyLayer::ALL {
                let bytes = masked_rgba(&texels, &labels, layer);
                for (i, px) in bytes.as_chunks::<4>().0.iter().enumerate() {
                    if px[3] > 0 {
                        union[i] = *px;
                    }
                }
                layers.push((layer, bytes));
            }
            let differing = union.iter().zip(&texels).filter(|(a, b)| a != b).count();
            out.push(SourceLayers {
                source,
                layers,
                differing,
                authored: authored.is_some(),
            });
        }
        Ok(out)
    }
}

impl RigLab {
    pub fn from_ron(text: &str) -> Result<Self, RigLabError> {
        let lab: Self = ron::from_str(text).map_err(|e| RigLabError::Parse(e.to_string()))?;
        lab.validate()?;
        Ok(lab)
    }

    pub fn validate(&self) -> Result<(), RigLabError> {
        if self.version != RIG_LAB_VERSION {
            return Err(RigLabError::Version(self.version));
        }
        for (key, clip) in &self.clips {
            validate_clip(key, clip)?;
        }
        if !self.clips.contains_key(&self.rest_clip) {
            return Err(RigLabError::RestClip(self.rest_clip.clone()));
        }
        for (id, item) in &self.items {
            item.check().map_err(|p| RigLabError::Item(id.clone(), p))?;
            if let ItemArt::Model(model) = &item.art
                && !self.models.contains_key(model)
            {
                return Err(RigLabError::Model(id.clone(), model.clone()));
            }
        }
        for (id, grip) in &self.grips {
            if !(0.0..=1.0).contains(&grip.follow_forearm) {
                return Err(RigLabError::GripFollow(id.clone()));
            }
        }
        for (key, clip) in &self.clips {
            if let Some(grip) = &clip.grip {
                let profile = self
                    .grips
                    .get(grip)
                    .ok_or_else(|| RigLabError::Grip(key.clone(), grip.clone()))?;
                if !profile.depth.is_empty() && profile.depth.len() != clip.frames.len() {
                    return Err(RigLabError::ClipLists(key.clone()));
                }
            }
            if !clip.labels.is_empty() && clip.labels.len() != clip.sources.len() {
                return Err(RigLabError::ClipLists(key.clone()));
            }
        }
        for (slot, ids) in &self.variants {
            for id in ids {
                if self.items.get(id).is_none_or(|i| i.slot != *slot) {
                    return Err(RigLabError::Variant(id.clone(), *slot));
                }
            }
        }
        Ok(())
    }

    /// The grip profile of a clip.
    pub fn grip(&self, clip: &str) -> Option<&GripProfile> {
        self.grips.get(self.clips.get(clip)?.grip.as_ref()?)
    }

    /// The clip items are authored on.
    pub fn rest(&self) -> Option<&RigClip> {
        self.clips.get(&self.rest_clip)
    }
}

fn validate_clip(key: &str, clip: &RigClip) -> Result<(), RigLabError> {
    if clip.sources.len() != clip.columns as usize {
        return Err(RigLabError::Sources {
            clip: key.to_owned(),
            sources: clip.sources.len(),
            columns: clip.columns,
        });
    }
    for (i, marked) in clip.sources.iter().enumerate() {
        if let Some(joint) = Joint::ALL.iter().find(|j| !marked.contains_key(j)) {
            return Err(RigLabError::MissingJoint {
                clip: key.to_owned(),
                source_index: i,
                joint: *joint,
            });
        }
        let outside = marked.values().any(|(x, y)| {
            !(x.is_finite() && y.is_finite())
                || *x < 0.0
                || *y < 0.0
                || *x > clip.cell.0 as f32
                || *y > clip.cell.1 as f32
        });
        if outside {
            return Err(RigLabError::JointOutside(key.to_owned()));
        }
    }
    if clip.frames.is_empty() {
        return Err(RigLabError::Frame {
            clip: key.to_owned(),
            frame: 0,
        });
    }
    for (i, f) in clip.frames.iter().enumerate() {
        if f.source >= clip.sources.len() || !(f.duration.is_finite() && f.duration > 0.0) {
            return Err(RigLabError::Frame {
                clip: key.to_owned(),
                frame: i,
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const JOINTS: &str = "{ pelvis: (38, 60), neck: (39, 29), head: (40, 20), \
        shoulder_near: (39, 33), elbow_near: (40, 49), hand_near: (42, 67), \
        shoulder_far: (36, 33), elbow_far: (36, 47), hand_far: (37, 56), \
        knee_a: (38, 86), ankle_a: (37, 111), toe_a: (46, 116), \
        knee_b: (37, 86), ankle_b: (36, 111), toe_b: (44, 116), \
        chest_back: (33, 32), chest_front: (45, 32), waist_front: (44, 50), waist_back: (35, 50) }";

    fn lab() -> String {
        format!(
            r#"(
                version: 1,
                rest_clip: "idle",
                clips: {{
                    "idle": (texture: "i.png", cell: (80, 128), columns: 1, pivot: (40.0, 118.0),
                        sources: [{JOINTS}],
                        frames: [(source: 0, drop: 0, duration: 0.7), (source: 0, drop: 1, duration: 0.5)],
                        grip: Some("carry")),
                }},
                items: {{
                    "sword": (slot: MainHand, kind: Rigid,
                        art: Sprite((texture: "s.png", size: (21, 64), pivot: (10.0, 53.0))),
                        mount: Hand(side: Near, up_degrees: 0.0),
                        depth: MainHandGear, provisional: false, source: "N41-B"),
                }},
                variants: {{ MainHand: ["sword"] }},
                grips: {{ "carry": (reference: Body, angle_degrees: -60.0, follow_forearm: 0.2) }},
            )"#
        )
    }

    #[test]
    fn a_valid_lab_loads_and_converts_to_rig_space() {
        let lab = RigLab::from_ron(&lab()).unwrap_or_else(|e| panic!("{e}"));
        let clip = lab.rest().unwrap_or_else(|| panic!("rest"));
        let j = clip.source_joints(0).unwrap_or_else(|| panic!("joints"));
        // (38, 60) texels with the pivot (40, 118) -> (-2, 58) in rig space.
        assert_eq!(j.get(Joint::Pelvis), (-2.0, 58.0));
        let breath = clip.frame_joints(1).unwrap_or_else(|| panic!("frame"));
        assert_eq!(breath.get(Joint::Head).1, j.get(Joint::Head).1 - 1.0);
        assert_eq!(breath.get(Joint::Pelvis), j.get(Joint::Pelvis));
        assert_eq!(clip.durations(), vec![0.7, 0.5]);
        let grip = lab.grip("idle").unwrap_or_else(|| panic!("grip"));
        assert_eq!(grip.angle_degrees, -60.0);
    }

    #[test]
    fn layers_are_cut_from_a_strip_without_loss() {
        let lab = RigLab::from_ron(&lab()).unwrap_or_else(|e| panic!("{e}"));
        let clip = lab.rest().unwrap_or_else(|| panic!("rest"));
        // An 80x128 strip with a few opaque palette texels.
        let mut strip = vec![0u8; 80 * 128 * 4];
        for (i, px) in strip.chunks_mut(4).enumerate() {
            if i % 7 == 0 {
                px.copy_from_slice(&[138, 134, 140, 255]);
            }
        }
        let cut = clip
            .cut_layers(&strip, 80, |_| None)
            .unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(cut.len(), 1);
        assert_eq!(cut[0].differing, 0);
        assert!(!cut[0].authored);
        assert_eq!(cut[0].layers.len(), 5);
        assert!(clip.cut_layers(&strip[..100], 80, |_| None).is_err());
    }

    #[test]
    fn invalid_labs_are_refused() {
        let missing = lab().replace("toe_b: (44, 116), ", "");
        assert!(matches!(
            RigLab::from_ron(&missing),
            Err(RigLabError::MissingJoint {
                joint: Joint::ToeB,
                ..
            })
        ));
        let frame = lab().replace("(source: 0, drop: 1", "(source: 3, drop: 1");
        assert!(matches!(
            RigLab::from_ron(&frame),
            Err(RigLabError::Frame { frame: 1, .. })
        ));
        let outside = lab().replace("head: (40, 20)", "head: (40, 200)");
        assert!(matches!(
            RigLab::from_ron(&outside),
            Err(RigLabError::JointOutside(_))
        ));
        let pivot = lab().replace("pivot: (10.0, 53.0)", "pivot: (10.0, 90.0)");
        assert!(matches!(
            RigLab::from_ron(&pivot),
            Err(RigLabError::Item(_, ItemProblem::Sprite))
        ));
        let grip = lab().replace("grip: Some(\"carry\")", "grip: Some(\"swing\")");
        assert!(matches!(
            RigLab::from_ron(&grip),
            Err(RigLabError::Grip(..))
        ));
        let follow = lab().replace("follow_forearm: 0.2", "follow_forearm: 1.5");
        assert!(matches!(
            RigLab::from_ron(&follow),
            Err(RigLabError::GripFollow(_))
        ));
        let depth = lab().replace(
            "follow_forearm: 0.2)",
            "follow_forearm: 0.2, depth: [MainHandGear])",
        );
        assert!(matches!(
            RigLab::from_ron(&depth),
            Err(RigLabError::ClipLists(_))
        ));
        let slot = lab().replace("MainHand: [", "Shoulder: [");
        assert!(matches!(
            RigLab::from_ron(&slot),
            Err(RigLabError::Variant(..))
        ));
        let version = lab().replace("version: 1", "version: 2");
        assert_eq!(RigLab::from_ron(&version), Err(RigLabError::Version(2)));
        let rest = lab().replace("rest_clip: \"idle\"", "rest_clip: \"walk\"");
        assert!(matches!(
            RigLab::from_ron(&rest),
            Err(RigLabError::RestClip(_))
        ));
    }
}
