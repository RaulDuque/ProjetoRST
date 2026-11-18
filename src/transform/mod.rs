//! Transformations (V1.9): the player may turn into another form. The first
//! and only one implemented is the Spectral Form (unlocked by the Relic of
//! the Veil), which crosses spectral veils.
//!
//! - [`definitions`] (pure): the data of each form (`assets/data/transformations.ron`);
//! - [`state`] (pure): which form is active, upkeep and the safe end;
//! - [`crossing`] (pure): whether a form can afford to cross a veil;
//! - [`systems`] (Bevy): input mask, activation, upkeep, end, death, load;
//! - [`visual`] (Bevy): tint, outline, motes, flashes and veil shimmer.
//!
//! The human form is the absence of a form: no profile component is on the
//! player, so movement and collision run exactly the human code.
//! Details: docs/TRANSFORMATIONS_V1_9.md.

pub mod crossing;
pub mod definitions;
pub mod state;
pub mod systems;
pub mod visual;

use bevy::prelude::*;

use crate::app::states::GameSet;
use crate::world::transitions::RoomStep;

pub struct TransformPlugin;

impl Plugin for TransformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (systems::attach_form_state, systems::mask_form_input)
                .chain()
                .in_set(GameSet::Input)
                .after(crate::input::publish_player_input),
        )
        .add_systems(
            Update,
            (
                systems::toggle_form,
                systems::upkeep_form,
                systems::settle_form,
            )
                .chain()
                .in_set(GameSet::Actions)
                .after(crate::player::systems::respawn)
                .before(crate::player::movement::guard_and_dodge),
        )
        .add_systems(
            Update,
            systems::end_form_on_death
                .in_set(GameSet::Resolution)
                .after(crate::player::systems::receive_damage),
        )
        .add_systems(Update, systems::end_form_on_load.in_set(RoomStep::React))
        .add_systems(Startup, visual::create_glow_image)
        .add_systems(
            Update,
            (
                visual::tint_form,
                visual::sync_outline,
                visual::sync_halo,
                visual::spawn_form_effects,
                visual::animate_form_effects,
                visual::shimmer_veils,
            )
                .chain()
                .in_set(GameSet::Presentation)
                .after(crate::player::visual::PlayerVisualSystems)
                .after(crate::player::animation::update_player_visuals),
        );
    }
}
