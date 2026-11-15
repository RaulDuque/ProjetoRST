//! Sprite animation manifest of the player (`assets/animations/player.ron`).
//!
//! Pure module: no Bevy types. It describes the atlases and clips produced
//! by the art pipeline (`art-src/player/scripts/post/game_sprites_v1_3.py`)
//! and validates them structurally (docs/ASSET_CONTRACT.md, section 7). The
//! format follows the contract proposal, extended with per-frame durations
//! (the 0.28 s action clips end with a shorter frame) and a speed reference
//! for locomotion loops.

use std::collections::{BTreeMap, BTreeSet};

use serde::Deserialize;
use thiserror::Error;

/// Direction the artwork faces. The contract draws everything facing right.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum ArtFacing {
    Right,
}

/// One texture laid out as a grid (`TextureAtlasLayout::from_grid`).
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AtlasDef {
    /// Path relative to `assets/`.
    pub texture: String,
    /// Cell size in texels.
    pub cell: (u32, u32),
    pub columns: u32,
    pub rows: u32,
    /// Texels between cells (no outer border).
    pub padding: u32,
    /// Texels per world unit.
    pub density: u32,
}

impl AtlasDef {
    /// Cell size in world units (`Sprite::custom_size`).
    pub fn world_size(&self) -> (f32, f32) {
        let d = self.density as f32;
        (self.cell.0 as f32 / d, self.cell.1 as f32 / d)
    }

    /// Image size implied by the grid: `columns·cell + (columns−1)·padding`.
    pub fn image_size(&self) -> (u32, u32) {
        (
            self.columns * self.cell.0 + self.columns.saturating_sub(1) * self.padding,
            self.rows * self.cell.1 + self.rows.saturating_sub(1) * self.padding,
        )
    }
}

/// How a clip is timed.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum ClipMode {
    /// Repeats at its own rate (scaled by speed for locomotion).
    Loop { fps: f32 },
    /// Stretched over the real duration of the gameplay action.
    FitToAction,
    /// Plays once and holds the last frame.
    Once { fps: f32 },
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ClipDef {
    pub atlas: String,
    pub first: u32,
    pub count: u32,
    pub mode: ClipMode,
    /// Sole-line pivot in texels from the cell's top-left corner.
    pub pivot: (f32, f32),
    pub fallback: Option<String>,
    /// Authored duration of every frame (seconds).
    pub durations: Vec<f32>,
    /// Horizontal speed (world units per second) the loop was authored for.
    pub speed_reference: Option<f32>,
}

impl ClipDef {
    pub fn total_duration(&self) -> f32 {
        self.durations.iter().sum()
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AnimationManifest {
    pub version: u32,
    pub facing: ArtFacing,
    pub atlases: BTreeMap<String, AtlasDef>,
    pub clips: BTreeMap<String, ClipDef>,
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ManifestError {
    #[error("cannot parse the animation manifest: {0}")]
    Parse(String),
    #[error("unsupported manifest version {0}")]
    Version(u32),
    #[error("invalid identifier {0:?} (ASCII snake_case expected)")]
    Identifier(String),
    #[error("clip {clip}: unknown atlas {atlas:?}")]
    UnknownAtlas { clip: String, atlas: String },
    #[error("atlas {0}: empty texture path, zero-sized cell or grid, or zero density")]
    Atlas(String),
    #[error("clip {clip}: frames {first}..{end} outside the {cells} cells of the atlas")]
    FrameRange {
        clip: String,
        first: u32,
        end: u32,
        cells: u32,
    },
    #[error("clip {0}: pivot outside the cell")]
    Pivot(String),
    #[error("clip {0}: fps, durations or speed reference not positive")]
    Timing(String),
    #[error("clip {clip}: {durations} durations for {count} frames")]
    DurationCount {
        clip: String,
        durations: usize,
        count: u32,
    },
    #[error("clip {clip}: fallback {fallback:?} does not exist")]
    UnknownFallback { clip: String, fallback: String },
    #[error("clip {0}: the fallback chain loops")]
    FallbackCycle(String),
    #[error("required clip {0:?} missing")]
    MissingClip(String),
}

/// Clip every manifest must provide: the root of every fallback chain.
pub const ROOT_CLIP: &str = "idle";
pub const MANIFEST_VERSION: u32 = 1;

fn is_identifier(id: &str) -> bool {
    let mut chars = id.chars();
    matches!(chars.next(), Some(c) if c.is_ascii_lowercase())
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

impl AnimationManifest {
    /// Parses and validates a manifest written in RON.
    pub fn from_ron(text: &str) -> Result<Self, ManifestError> {
        let manifest: Self =
            ron::from_str(text).map_err(|e| ManifestError::Parse(e.to_string()))?;
        manifest.validate()?;
        Ok(manifest)
    }

    /// Structural rules of docs/ASSET_CONTRACT.md, section 7 (2–5 and 7);
    /// the existence of the textures is checked by a data test.
    pub fn validate(&self) -> Result<(), ManifestError> {
        if self.version != MANIFEST_VERSION {
            return Err(ManifestError::Version(self.version));
        }
        for (id, atlas) in &self.atlases {
            if !is_identifier(id) {
                return Err(ManifestError::Identifier(id.clone()));
            }
            if atlas.texture.is_empty()
                || atlas.cell.0 == 0
                || atlas.cell.1 == 0
                || atlas.columns == 0
                || atlas.rows == 0
                || atlas.density == 0
            {
                return Err(ManifestError::Atlas(id.clone()));
            }
        }
        for (id, clip) in &self.clips {
            if !is_identifier(id) {
                return Err(ManifestError::Identifier(id.clone()));
            }
            let atlas =
                self.atlases
                    .get(&clip.atlas)
                    .ok_or_else(|| ManifestError::UnknownAtlas {
                        clip: id.clone(),
                        atlas: clip.atlas.clone(),
                    })?;
            let cells = atlas.columns * atlas.rows;
            let end = clip.first.saturating_add(clip.count);
            if clip.count == 0 || end > cells {
                return Err(ManifestError::FrameRange {
                    clip: id.clone(),
                    first: clip.first,
                    end,
                    cells,
                });
            }
            let (px, py) = clip.pivot;
            if !(0.0..=atlas.cell.0 as f32).contains(&px)
                || !(0.0..=atlas.cell.1 as f32).contains(&py)
            {
                return Err(ManifestError::Pivot(id.clone()));
            }
            if clip.durations.len() != clip.count as usize {
                return Err(ManifestError::DurationCount {
                    clip: id.clone(),
                    durations: clip.durations.len(),
                    count: clip.count,
                });
            }
            let fps_ok = match clip.mode {
                ClipMode::Loop { fps } | ClipMode::Once { fps } => fps > 0.0 && fps.is_finite(),
                ClipMode::FitToAction => true,
            };
            let durations_ok = clip.durations.iter().all(|d| *d > 0.0 && d.is_finite());
            let speed_ok = clip
                .speed_reference
                .is_none_or(|s| s > 0.0 && s.is_finite());
            if !(fps_ok && durations_ok && speed_ok) {
                return Err(ManifestError::Timing(id.clone()));
            }
            if let Some(fallback) = &clip.fallback
                && !self.clips.contains_key(fallback)
            {
                return Err(ManifestError::UnknownFallback {
                    clip: id.clone(),
                    fallback: fallback.clone(),
                });
            }
        }
        if !self.clips.contains_key(ROOT_CLIP) {
            return Err(ManifestError::MissingClip(ROOT_CLIP.into()));
        }
        for id in self.clips.keys() {
            let mut seen = BTreeSet::new();
            let mut current = Some(id.as_str());
            while let Some(key) = current {
                if !seen.insert(key) {
                    return Err(ManifestError::FallbackCycle(id.clone()));
                }
                current = self.clips.get(key).and_then(|c| c.fallback.as_deref());
            }
        }
        Ok(())
    }

    /// The clip to show for `wanted`: itself when the manifest has it,
    /// otherwise the root clip (every chain ends there, see `validate`).
    pub fn resolve(&self, wanted: &str) -> Option<(&str, &ClipDef)> {
        self.clips
            .get_key_value(wanted)
            .or_else(|| self.clips.get_key_value(ROOT_CLIP))
            .map(|(k, c)| (k.as_str(), c))
    }
}

/// Bevy `Anchor` value (y up, −0.5..0.5) of a pivot given in texels from the
/// cell's top-left corner, for a sprite that may be mirrored. `flip_x` only
/// mirrors the texture, so the anchor's x changes sign with it (contract,
/// section 5, rule 3).
pub fn anchor_for(pivot: (f32, f32), cell: (u32, u32), flip_x: bool) -> (f32, f32) {
    let x = pivot.0 / cell.0 as f32 - 0.5;
    let y = 0.5 - pivot.1 / cell.1 as f32;
    (if flip_x { -x } else { x }, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> AnimationManifest {
        AnimationManifest::from_ron(
            r#"(
                version: 1,
                facing: Right,
                atlases: {
                    "locomotion": (texture: "a.png", cell: (128, 128), columns: 8, rows: 2, padding: 2, density: 2),
                    "combat": (texture: "b.png", cell: (256, 192), columns: 7, rows: 1, padding: 2, density: 2),
                },
                clips: {
                    "idle": (atlas: "locomotion", first: 0, count: 2, mode: Loop(fps: 12.0), pivot: (64.0, 126.0),
                             fallback: None, durations: [0.083333, 0.083333], speed_reference: None),
                    "attack_basic": (atlas: "combat", first: 0, count: 2, mode: FitToAction, pivot: (80.0, 190.0),
                             fallback: Some("idle"), durations: [0.25, 0.03], speed_reference: None),
                },
            )"#,
        )
        .unwrap_or_else(|e| panic!("sample manifest: {e}"))
    }

    fn edit(m: &mut AnimationManifest, key: &str, f: impl FnOnce(&mut ClipDef)) {
        if let Some(clip) = m.clips.get_mut(key) {
            f(clip);
        }
    }

    #[test]
    fn parses_and_reports_sizes() {
        let m = sample();
        assert_eq!(m.atlases["locomotion"].world_size(), (64.0, 64.0));
        assert_eq!(m.atlases["combat"].world_size(), (128.0, 96.0));
        assert_eq!(m.atlases["locomotion"].image_size(), (1038, 258));
        assert!((m.clips["attack_basic"].total_duration() - 0.28).abs() < 1e-6);
    }

    #[test]
    fn rejects_frames_outside_the_atlas() {
        let mut m = sample();
        edit(&mut m, "idle", |c| c.first = 15);
        assert!(matches!(
            m.validate(),
            Err(ManifestError::FrameRange { .. })
        ));
    }

    #[test]
    fn rejects_bad_pivot_durations_and_fallbacks() {
        let mut m = sample();
        edit(&mut m, "idle", |c| c.pivot = (200.0, 10.0));
        assert!(matches!(m.validate(), Err(ManifestError::Pivot(_))));
        let mut m = sample();
        edit(&mut m, "idle", |c| {
            c.durations.pop();
        });
        assert!(matches!(
            m.validate(),
            Err(ManifestError::DurationCount { .. })
        ));
        let mut m = sample();
        edit(&mut m, "attack_basic", |c| c.fallback = Some("nope".into()));
        assert!(matches!(
            m.validate(),
            Err(ManifestError::UnknownFallback { .. })
        ));
        let mut m = sample();
        edit(&mut m, "idle", |c| c.fallback = Some("attack_basic".into()));
        assert!(matches!(m.validate(), Err(ManifestError::FallbackCycle(_))));
    }

    #[test]
    fn requires_the_root_clip_and_snake_case_ids() {
        let mut m = sample();
        let idle = m.clips.remove("idle");
        edit(&mut m, "attack_basic", |c| c.fallback = None);
        assert!(matches!(m.validate(), Err(ManifestError::MissingClip(_))));
        if let Some(idle) = idle {
            m.clips.insert("Idle".into(), idle);
        }
        assert!(matches!(m.validate(), Err(ManifestError::Identifier(_))));
    }

    #[test]
    fn resolve_falls_back_to_the_root_clip() {
        let m = sample();
        assert_eq!(m.resolve("attack_basic").map(|r| r.0), Some("attack_basic"));
        assert_eq!(m.resolve("hurt").map(|r| r.0), Some("idle"));
    }

    #[test]
    fn anchor_puts_the_pivot_on_the_sole_and_flips_with_the_sprite() {
        // Contract example: (32, 63) in 64x64 -> (0, -0.484).
        let (x, y) = anchor_for((32.0, 63.0), (64, 64), false);
        assert!(x.abs() < 1e-6 && (y + 0.484_375).abs() < 1e-6);
        // Same pivot at density 2 gives the same anchor.
        assert_eq!(anchor_for((64.0, 126.0), (128, 128), false), (x, y));
        // Combat pivot (40, 95) in 128x96 is left of centre and mirrors.
        let (cx, cy) = anchor_for((80.0, 190.0), (256, 192), false);
        assert!((cx + 0.1875).abs() < 1e-6 && (cy + 0.489_583).abs() < 1e-4);
        assert_eq!(anchor_for((80.0, 190.0), (256, 192), true), (-cx, cy));
    }
}
