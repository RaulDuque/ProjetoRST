//! User interface: HUD, status screen and combat feedback. Each screen is
//! its own module and each element is updated by a small dedicated system.

pub mod damage_numbers;
pub mod dialogue_box;
pub mod hud;
pub mod inventory_screen;
pub mod room_fade;
pub mod shop_screen;
pub mod status_screen;
pub mod theme;
pub mod world_map;

use bevy::prelude::*;

use crate::app::states::{GameSet, GameState};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<hud::HudMessages>()
            .init_resource::<inventory_screen::InventoryCursor>()
            .init_resource::<shop_screen::ShopCursor>()
            .add_systems(
                OnEnter(GameState::Dialogue),
                dialogue_box::spawn_dialogue_box,
            )
            .add_systems(OnEnter(GameState::Shop), shop_screen::spawn_shop_screen)
            .add_systems(
                Update,
                dialogue_box::update_dialogue_box
                    .in_set(GameSet::Presentation)
                    .after(crate::npc::systems::run_conversation)
                    .run_if(in_state(GameState::Dialogue)),
            )
            .add_systems(
                Update,
                (
                    shop_screen::navigate_shop.before(crate::economy::systems::apply_trades),
                    shop_screen::update_shop_screen.after(crate::economy::systems::apply_trades),
                )
                    .in_set(GameSet::Presentation)
                    .run_if(in_state(GameState::Shop)),
            )
            .add_systems(Startup, hud::spawn_hud)
            .add_systems(
                Update,
                (
                    hud::update_bars,
                    hud::update_level_and_hearts,
                    hud::update_cooldowns,
                    hud::update_form_text,
                    (
                        hud::collect_messages,
                        hud::update_messages,
                        hud::show_message_panels,
                    )
                        .chain(),
                    damage_numbers::spawn_damage_numbers,
                    damage_numbers::animate_damage_numbers,
                    status_screen::toggle_status_screen,
                    inventory_screen::toggle_inventory_screen,
                )
                    .in_set(GameSet::Presentation),
            )
            .add_systems(
                Update,
                damage_numbers::clear_on_room_change
                    .in_set(crate::world::transitions::RoomStep::React),
            )
            .add_plugins(world_map::WorldMapUiPlugin)
            .add_plugins(room_fade::RoomFadePlugin)
            .add_systems(
                OnEnter(GameState::Inventory),
                inventory_screen::spawn_inventory_screen,
            )
            .add_systems(
                Update,
                (
                    inventory_screen::navigate_inventory
                        .before(crate::inventory::systems::apply_equip_requests),
                    inventory_screen::update_inventory_screen
                        .after(crate::inventory::systems::sync_appearance),
                )
                    .in_set(GameSet::Presentation)
                    .run_if(in_state(GameState::Inventory)),
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
