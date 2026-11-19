//! Sprite visual of the player (V1.3): a child entity that shows the
//! rendered protagonist (model v007) and follows the gameplay state.
//!
//! - [`art_profile`] (pure, N3.8): production art or the experimental
//!   N3 protagonist (`DARK_CASTLE_PROTAGONIST=n3`);
//! - [`manifest`] (pure): atlases and clips of `assets/animations/player.ron`;
//! - [`playback`] (pure): frame timing (loops, actions, per-frame durations);
//! - [`selection`] (pure): which clip shows a gameplay state, and fallbacks;
//! - [`sockets`] (pure, V2.2): per-frame equipment sockets (hands, head, back);
//! - [`systems`] (Bevy): loading, attaching the child, animating it;
//! - [`vfx`] (V1.5): skill, guard and revive effects (presentation only).
//!
//! The visual only reads gameplay components (`ActionState`,
//! `FacingDirection`, `Velocity`, `Invulnerable`); it never writes them.
//! Until the textures are loaded (or when the manifest is missing, or in
//! headless apps without an `AssetServer`) the placeholder box of
//! `player::animation` stays visible. Details: docs/PLAYER_VISUAL_ANIMATION_V1_3.md.

pub mod art_profile;
pub mod manifest;
pub mod modular_lab;
pub mod playback;
pub mod selection;
pub mod sockets;
pub mod systems;
pub mod vfx;

use std::collections::HashMap;

use bevy::prelude::*;

use crate::app::states::GameSet;
use manifest::AnimationManifest;

/// Manifest path, relative to `assets/`.
pub const MANIFEST_PATH: &str = "animations/player.ron";

/// Effects sit in front of the character sprite (`systems::VISUAL_Z`).
pub const VISUAL_Z_EFFECTS: f32 = 0.1;

/// The sprite child of the player.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct PlayerVisual;

/// Playback bookkeeping of the sprite child (presentation only).
#[derive(Component, Debug, Clone, Default)]
pub struct VisualPlayback {
    /// Manifest key of the clip being shown.
    pub clip: Option<String>,
    /// Gameplay state seen last frame.
    pub state: Option<crate::player::state::PlayerState>,
    /// Seconds into a loop (already scaled by the playback rate).
    pub elapsed: f32,
    /// Total length of the current action, read when it started.
    pub action_total: f32,
    /// Remaining action time seen last frame (a rise means a new action).
    pub last_remaining: f32,
    /// Atlas index currently shown.
    pub index: usize,
    /// `animation` key of the skill being performed (from `SkillStarted`).
    pub skill_clip: Option<String>,
    /// Time left of the landing clip (0 when not landing).
    pub landing_left: f32,
    /// Player position seen last frame (a teleport is not a landing).
    pub last_position: Option<Vec2>,
    /// Seconds spent in `Falling` so far (a landing needs a real fall).
    pub falling_time: f32,
    /// N3.8: the clip the state wanted when the manifest lacks it (the idle
    /// pose is shown instead); `None` when the wanted clip is shown.
    pub placeholder_for: Option<String>,
}

/// N3.8: the player atlases are sampled with nearest filtering (crisp
/// texels, as in the approved N3.5/N3.7 proofs). Absent: Bevy's default
/// sampler, so the production art keeps exactly the look it has always had.
#[derive(Resource, Debug, Clone, Copy, Default)]
pub struct NearestPlayerArt;

/// N3.8: the game runs with the experimental N3 protagonist (not the
/// production art). Only presentation reads it (the experimental badge).
#[derive(Resource, Debug, Clone, Copy, Default)]
pub struct ExperimentalProtagonist;

/// Image and grid of one atlas.
#[derive(Debug, Clone)]
pub struct AtlasHandles {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    /// Cell in texels.
    pub cell: (u32, u32),
    /// Cell in world units (`Sprite::custom_size`).
    pub world_size: Vec2,
}

/// Loaded manifest and its handles, shared by every visual.
#[derive(Resource, Debug, Clone)]
pub struct PlayerSpriteSet {
    pub manifest: AnimationManifest,
    pub atlases: HashMap<String, AtlasHandles>,
    /// V2.2: per-frame equipment sockets (`None` without a valid table: the
    /// held equipment layers stay off).
    pub sockets: Option<sockets::SocketTable>,
}

impl PlayerSpriteSet {
    /// Whether held equipment can be drawn as layers: the body frames have
    /// no weapon baked in and the sockets are known.
    pub fn held_layers_enabled(&self) -> bool {
        !self.manifest.weapon_in_body && self.sockets.is_some()
    }
}

/// Where the player art is read from. The default is the published art in
/// `assets/`; a showcase can point it at a staged candidate (V2.2) that is
/// registered as another asset source, without touching `assets/`.
#[derive(Resource, Debug, Clone)]
pub struct PlayerArtSource {
    /// Directory holding `animations/player.ron` (and the sockets).
    pub root: std::path::PathBuf,
    /// Prefix of the texture asset paths (e.g. `"candidate://"`).
    pub texture_prefix: String,
}

impl Default for PlayerArtSource {
    fn default() -> Self {
        Self {
            root: crate::core::resources::project_root().join("assets"),
            texture_prefix: String::new(),
        }
    }
}

impl PlayerSpriteSet {
    /// Whether every atlas texture is in memory (sprites can be shown).
    pub fn ready(&self, images: &Assets<Image>) -> bool {
        self.atlases.values().all(|a| images.contains(&a.image))
    }
}

/// Systems of the sprite visual, inside [`GameSet::Presentation`].
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerVisualSystems;

pub struct PlayerVisualPlugin;

impl Plugin for PlayerVisualPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::load_player_sprites)
            .configure_sets(
                Update,
                PlayerVisualSystems
                    .in_set(GameSet::Presentation)
                    // F9 also runs in Presentation: show the loaded state in the
                    // same frame instead of one frame later.
                    .after(crate::save::systems::quick_load),
            )
            .add_systems(
                Update,
                (
                    systems::attach_player_visual,
                    systems::animate_player_visual,
                    vfx::systems::spawn_skill_vfx,
                    vfx::systems::spawn_guard_vfx,
                    vfx::systems::spawn_revive_vfx,
                    vfx::systems::animate_player_vfx,
                )
                    .chain()
                    .in_set(PlayerVisualSystems),
            );
    }
}
