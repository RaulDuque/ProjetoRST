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
//! - [`equipment`]: the equipment contract (10 slots; rigid, articulated,
//!   flexible), placement, mirroring, loadout;
//! - [`grip`] (02): hand/wrist frame from reusable grip profiles;
//! - [`chest`] (02): torso cage and one breastplate model on every pose
//!   (rigid, mesh warp or projected);
//! - [`measure`] (02): socket stability metrics;
//! - [`articulated`]: the fully articulated comparison (rest pieces + FK/IK);
//! - [`data`]: the lab RON (marked frames, clips, items).
//!
//! Only `examples/modular_character_engine.rs` uses it: not registered in
//! [`super::PlayerVisualPlugin`], the game is unchanged. Algorithms adapted
//! from pixelfx and Agent Sprite Forge (both MIT) are credited where used.
//! Details: docs/development/modular_character_engine/.

pub mod articulated;
pub mod chest;
pub mod data;
pub mod equipment;
pub mod grip;
pub mod ik;
pub mod layers;
pub mod measure;
pub mod skeleton;
pub mod xform;
