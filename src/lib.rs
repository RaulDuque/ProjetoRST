//! Dark Castle RPG — 2D action RPG / Metroidvania.
//!
//! Each feature module separates engine-independent rules (pure Rust,
//! unit-testable without Bevy) from the Bevy systems that drive them.
//! See `docs/ARCHITECTURE.md`.

pub mod app;
pub mod combat;
pub mod core;
pub mod economy;
pub mod enemies;
pub mod input;
pub mod inventory;
pub mod magic;
pub mod npc;
pub mod player;
pub mod save;
pub mod skills;
pub mod transform;
pub mod ui;
pub mod world;
