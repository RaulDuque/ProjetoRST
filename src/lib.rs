//! Dark Castle RPG — 2D action RPG / Metroidvania.
//!
//! Each feature module separates engine-independent rules (pure Rust,
//! unit-testable without Bevy) from the Bevy systems that drive them.
//! See `docs/ARCHITECTURE.md`.

pub mod combat;
pub mod core;
pub mod magic;
pub mod player;
