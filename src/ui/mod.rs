//! User interface: HUD, status screen and combat feedback. Each screen is
//! its own module and each element is updated by a small dedicated system.

pub mod damage_numbers;
pub mod hud;
pub mod status_screen;
pub mod theme;

use bevy::prelude::*;

use crate::app::states::{GameSet, GameState};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<hud::HudMessages>()
            .add_systems(Startup, hud::spawn_hud)
            .add_systems(
                Update,
                (
                    hud::update_bars,
                    hud::update_level_and_hearts,
                    hud::update_cooldowns,
                    (hud::collect_messages, hud::update_messages).chain(),
                    damage_numbers::spawn_damage_numbers,
                    damage_numbers::animate_damage_numbers,
                    status_screen::toggle_status_screen,
                )
                    .in_set(GameSet::Presentation),
            )
            .add_systems(
                OnEnter(GameState::StatusScreen),
                status_screen::spawn_status_screen,
            )
            .add_systems(
                Update,
                (
                    status_screen::allocate_points,
                    status_screen::update_status_screen,
                )
                    .chain()
                    .in_set(GameSet::Presentation)
                    .run_if(in_state(GameState::StatusScreen)),
            );
    }
}
