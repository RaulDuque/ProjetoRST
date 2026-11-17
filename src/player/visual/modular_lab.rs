//! D12 experimental modular visual — **pure rules only**.
//!
//! Used by `examples/modular_visual_lab.rs`. Not registered in
//! [`super::PlayerVisualPlugin`]; the published player sprite stays default.
//!
//! D12 corrections vs D11:
//! - world height **64** (was 52) to match published locomotion height;
//! - sole [`anchor_for_pivot`] required in Bevy (D11 used center anchor);
//! - contact-shadow and after-image helpers for the lab.

#![allow(dead_code)] // exercised by the example and unit tests

/// Experimental baked loadout (matches D10 `classico` / `pesado`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModularLoadout {
    Classic,
    Heavy,
}

impl ModularLoadout {
    pub const fn asset_key(self) -> &'static str {
        match self {
            Self::Classic => "classico",
            Self::Heavy => "pesado",
        }
    }

    pub const fn toggle(self) -> Self {
        match self {
            Self::Classic => Self::Heavy,
            Self::Heavy => Self::Classic,
        }
    }
}

/// Locomotion clip for the lab (idle / run only).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModularClip {
    Idle,
    Run,
}

impl ModularClip {
    pub const fn asset_key(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Run => "run",
        }
    }

    pub const fn frame_count(self) -> usize {
        match self {
            Self::Idle => 6,
            Self::Run => 8,
        }
    }

    pub const fn fps(self) -> f32 {
        match self {
            Self::Idle => 10.0,
            Self::Run => 12.0,
        }
    }

    pub const fn frame_duration(self) -> f32 {
        1.0 / self.fps()
    }
}

/// Atlas / world sizing (D12: align with published locomotion height).
pub const CELL_W: u32 = 128;
pub const CELL_H: u32 = 160;
pub const PIVOT_SOLE: (u32, u32) = (64, 156);
/// Same world height as published `density: 2` locomotion cells (64×64).
pub const WORLD_HEIGHT: f32 = 64.0;

/// World size for the 128×160 cell at [`WORLD_HEIGHT`].
pub fn world_size() -> (f32, f32) {
    let aspect = CELL_W as f32 / CELL_H as f32;
    (WORLD_HEIGHT * aspect, WORLD_HEIGHT)
}

/// Bevy `Anchor` (x/y in −0.5..0.5) for a texel pivot from the cell top-left.
///
/// Same contract as [`super::manifest::anchor_for`].
pub fn anchor_for_pivot(pivot: (u32, u32), cell: (u32, u32), flip_x: bool) -> (f32, f32) {
    let mut x = pivot.0 as f32 / cell.0 as f32 - 0.5;
    let y = 0.5 - pivot.1 as f32 / cell.1 as f32;
    if flip_x {
        x = -x;
    }
    (x, y)
}

/// Advance a looping frame index without resetting on loadout swap.
pub fn frame_index(clip: ModularClip, elapsed: f32) -> usize {
    let n = clip.frame_count();
    if n == 0 {
        return 0;
    }
    let t = if elapsed.is_finite() && elapsed >= 0.0 {
        elapsed
    } else {
        0.0
    };
    let frame = (t / clip.frame_duration()).floor() as usize;
    frame % n
}

/// After-image B (trail): spawn a ghost every `interval` while moving.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AfterImageTrail {
    pub interval: f32,
    pub lifetime: f32,
    pub max_active: usize,
}

impl Default for AfterImageTrail {
    fn default() -> Self {
        Self {
            interval: 0.08,
            lifetime: 0.28,
            max_active: 6,
        }
    }
}

/// Wine tint for ghosts (sRGB 0–1). Variant B adds a slightly lighter edge in art.
pub const AFTERIMAGE_TINT: [f32; 4] = [0.55, 0.12, 0.22, 0.55];
pub const AFTERIMAGE_EDGE_TINT: [f32; 4] = [0.67, 0.31, 0.39, 0.58];

/// Whether a new ghost should spawn this tick.
pub fn should_spawn_ghost(
    moving: bool,
    since_last: f32,
    active: usize,
    cfg: AfterImageTrail,
) -> bool {
    moving && since_last >= cfg.interval && active < cfg.max_active
}

/// Alpha of a ghost given age in seconds (linear fade).
pub fn ghost_alpha(age: f32, lifetime: f32, peak: f32) -> f32 {
    if lifetime <= 0.0 || age >= lifetime {
        return 0.0;
    }
    let t = (1.0 - age / lifetime).clamp(0.0, 1.0);
    peak * t
}

/// Contact shadow ellipse size in world units (width, height).
pub fn contact_shadow_size(body_world_width: f32) -> (f32, f32) {
    (body_world_width * 0.72, body_world_width * 0.14)
}

/// Shadow opacity while grounded vs airborne (jump).
pub fn contact_shadow_alpha(airborne: bool) -> f32 {
    if airborne { 0.22 } else { 0.45 }
}

/// Shadow scale factor while airborne (stays on ground, shrinks).
pub fn contact_shadow_air_scale(airborne: bool) -> f32 {
    if airborne { 0.65 } else { 1.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_loops_and_survives_loadout_concept() {
        let clip = ModularClip::Idle;
        assert_eq!(frame_index(clip, 0.0), 0);
        assert_eq!(frame_index(clip, 0.05), 0);
        assert_eq!(frame_index(clip, 0.10), 1);
        assert_eq!(frame_index(clip, 0.60), 0);
        let before = frame_index(clip, 0.35);
        let after = frame_index(clip, 0.35);
        assert_eq!(before, after);
        assert_eq!(before, 3);
    }

    #[test]
    fn run_has_eight_frames() {
        assert_eq!(ModularClip::Run.frame_count(), 8);
        assert_eq!(frame_index(ModularClip::Run, 7.5 / 12.0), 7);
        assert_eq!(frame_index(ModularClip::Run, 8.5 / 12.0), 0);
    }

    #[test]
    fn afterimage_spawn_gates() {
        let cfg = AfterImageTrail::default();
        assert!(!should_spawn_ghost(false, 1.0, 0, cfg));
        assert!(!should_spawn_ghost(true, 0.01, 0, cfg));
        assert!(should_spawn_ghost(true, 0.09, 0, cfg));
        assert!(!should_spawn_ghost(true, 0.09, cfg.max_active, cfg));
    }

    #[test]
    fn ghost_fades_out() {
        assert!((ghost_alpha(0.0, 0.28, 0.55) - 0.55).abs() < 1e-5);
        assert!(ghost_alpha(0.28, 0.28, 0.55) == 0.0);
        assert!(ghost_alpha(0.14, 0.28, 0.55) < 0.55);
    }

    #[test]
    fn world_size_matches_d12_contract() {
        let (w, h) = world_size();
        assert!((h - 64.0).abs() < 1e-5);
        assert!((w - 64.0 * 128.0 / 160.0).abs() < 1e-5);
    }

    #[test]
    fn sole_anchor_matches_manifest_contract() {
        let (x, y) = anchor_for_pivot(PIVOT_SOLE, (CELL_W, CELL_H), false);
        assert!((x - 0.0).abs() < 1e-5);
        assert!((y - (0.5 - 156.0 / 160.0)).abs() < 1e-5);
        let (xf, _) = anchor_for_pivot(PIVOT_SOLE, (CELL_W, CELL_H), true);
        assert!((xf - 0.0).abs() < 1e-5); // pivot x centered → flip is still 0
    }

    #[test]
    fn contact_shadow_shrinks_in_air() {
        assert!(contact_shadow_air_scale(true) < contact_shadow_air_scale(false));
        assert!(contact_shadow_alpha(true) < contact_shadow_alpha(false));
    }
}
