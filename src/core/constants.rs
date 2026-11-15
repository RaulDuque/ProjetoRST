//! Engine-facing tuning constants (window, physics, visuals).
//!
//! Gameplay balance (damage, EXP, skills, enemies) lives in RON data files,
//! not here. These values control "feel" and presentation.

pub const WINDOW_TITLE: &str = "Dark Castle RPG";
pub const WINDOW_WIDTH: u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 720;

/// Largest simulation step; longer frames are clamped so that a hitch
/// cannot make bodies tunnel through platforms.
pub const MAX_FRAME_DELTA: f32 = 1.0 / 30.0;

/// Seed of the gameplay RNG (fixed for reproducible prototyping).
pub const RNG_SEED: u64 = 0x5EED_CA57_1E00_0001;

// ---------------------------------------------------------------- physics

pub const GRAVITY: f32 = -1900.0;
pub const MAX_FALL_SPEED: f32 = -900.0;
pub const PLAYER_SIZE: (f32, f32) = (26.0, 46.0);
pub const PLAYER_WALK_SPEED: f32 = 190.0;
pub const PLAYER_RUN_SPEED: f32 = 310.0;
pub const PLAYER_JUMP_SPEED: f32 = 720.0;
/// Releasing jump early multiplies upward speed by this (variable jump).
pub const JUMP_CUT_FACTOR: f32 = 0.45;
/// Seconds after leaving a ledge during which a jump is still accepted.
pub const COYOTE_TIME: f32 = 0.08;
pub const PLAYER_DODGE_SPEED: f32 = 480.0;
pub const PLAYER_DODGE_TIME: f32 = 0.28;
pub const PLAYER_HURT_TIME: f32 = 0.35;
pub const PLAYER_HURT_KNOCKBACK: (f32, f32) = (220.0, 260.0);
/// Invulnerability after being hit.
pub const PLAYER_INVULNERABILITY: f32 = 0.9;
/// Fraction of damage blocked while guarding.
pub const GUARD_DAMAGE_REDUCTION: f32 = 0.7;

// ---------------------------------------------------------------- basic attack

pub const BASIC_ATTACK_TIME: f32 = 0.28;
pub const BASIC_ATTACK_HITBOX: (f32, f32) = (46.0, 26.0);
/// Hitbox lifetime (active frames) as a fraction of the attack duration.
pub const HITBOX_ACTIVE_FRACTION: f32 = 0.6;
/// V2.4: `air_control` is the share of the gap to the wanted speed closed
/// per frame of this length, so the feel does not depend on the frame rate.
pub const AIR_CONTROL_REFERENCE_FRAME: f32 = 1.0 / 60.0;

// ---------------------------------------------------------------- enemies

pub const ENEMY_KNOCKBACK: f32 = 160.0;
pub const ENEMY_HURT_FLASH: f32 = 0.15;

// ---------------------------------------------------------------- camera

// V2.3: the camera feel (zoom, follow rates, dead zones, look-ahead) is
// declarative data: assets/data/camera.ron (src/world/camera_rules.rs).

// ---------------------------------------------------------------- z layers

pub const Z_BACKGROUND: f32 = 0.0;
// V2.3 scenery (src/world/scenery_systems.rs): everything decorative stays
// behind the platforms (1.0) except thin front pieces, drawn translucent.
pub const Z_BACKDROP_FAR: f32 = 0.1;
pub const Z_BACKDROP_MID: f32 = 0.2;
pub const Z_DECOR_BACK: f32 = 0.5;
pub const Z_LIGHTS: f32 = 0.6;
pub const Z_FOG: f32 = 0.7;
pub const Z_DECOR_FRONT: f32 = 9.0;
pub const Z_PLATFORMS: f32 = 1.0;
pub const Z_ENEMIES: f32 = 5.0;
pub const Z_PLAYER: f32 = 6.0;
pub const Z_EFFECTS: f32 = 8.0;

// ---------------------------------------------------------------- save

pub const SAVE_DIR: &str = "saves";
pub const DEFAULT_SAVE_SLOT: u32 = 1;

// ---------------------------------------------------------------- UI

/// Seconds a notification stays on screen.
pub const NOTIFICATION_TIME: f32 = 2.5;
/// Seconds the black overlay takes to fade out after a room change (V1.8).
/// Presentation only: input, physics and the room lifecycle never wait.
pub const ROOM_FADE_TIME: f32 = 0.22;
