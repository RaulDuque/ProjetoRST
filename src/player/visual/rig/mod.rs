//! Dark Castle Modular Character Engine — first prototype (pure rules).
//!
//! An invisible skeleton fitted to the **approved** frames, depth layers cut
//! from those frames by the skeleton, and equipment drawn once and bound to
//! bones. The motion belongs to the character (the frames and their
//! skeletons); an item only says which bone it follows and how.
//!
//! - [`xform`]: rigid 2D frames (rig space: texels, y up, pivot origin);
//! - [`skeleton`]: joints, bones, hierarchy, fitting and forward kinematics;
//! - [`ik`]: two-bone inverse kinematics;
//! - [`layers`]: materials, body layers, the depth order, partitions;
//! - [`equipment`]: rig-bound items, placement, mirroring, loadout;
//! - [`articulated`]: the fully articulated comparison (rest pieces + FK/IK);
//! - [`data`]: the lab RON (marked frames, clips, items).
//!
//! Only `examples/modular_character_engine.rs` uses it: not registered in
//! [`super::PlayerVisualPlugin`], the game is unchanged. Algorithms adapted
//! from pixelfx and Agent Sprite Forge (both MIT) are credited where used.
//! Details: docs/development/modular_character_engine/.

pub mod articulated;
pub mod data;
pub mod equipment;
pub mod ik;
pub mod layers;
pub mod skeleton;
pub mod xform;
