//! Room transition fade (V1.8): after every room change (door or load) a
//! black overlay covers the new room and fades out in [`ROOM_FADE_TIME`].
//!
//! The room changes in one frame, as before (`world::transitions`); the fade
//! only starts when `RoomChanged` says the change succeeded, so it never
//! hides a failed load (failures keep their notification and no fade). It
//! is presentation only: the player is not moved, input is not blocked and
//! nothing waits for it. A fade-out *before* the change would need the
//! lifecycle to defer the change by some frames; that was left out on
//! purpose (docs/WORLD_EXPLORATION_V1_8.md §15).

use bevy::prelude::*;

use crate::app::states::GameSet;
use crate::core::constants::ROOM_FADE_TIME;
use crate::core::events::RoomChanged;
use crate::core::time::frame_delta;

/// The full-screen overlay.
#[derive(Component)]
pub struct RoomFade;

/// Seconds of fade left (0 = no overlay).
#[derive(Resource, Debug, Clone, Copy, Default, PartialEq)]
pub struct RoomFadeState {
    pub remaining: f32,
}

impl RoomFadeState {
    /// Overlay opacity: 1 right after a change, 0 when done.
    pub fn alpha(&self) -> f32 {
        (self.remaining / ROOM_FADE_TIME).clamp(0.0, 1.0)
    }
}

pub struct RoomFadePlugin;

impl Plugin for RoomFadePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RoomFadeState>()
            .add_systems(Startup, spawn_room_fade)
            .add_systems(
                Update,
                (start_room_fade, update_room_fade)
                    .chain()
                    .in_set(GameSet::Presentation),
            );
    }
}

pub fn spawn_room_fade(mut commands: Commands) {
    commands.spawn((
        RoomFade,
        Node {
            position_type: PositionType::Absolute,
            left: px(0),
            top: px(0),
            width: percent(100),
            height: percent(100),
            ..default()
        },
        BackgroundColor(Color::BLACK.with_alpha(0.0)),
        // Below the HUD and the map: only the room is covered.
        GlobalZIndex(-1),
    ));
}

pub fn start_room_fade(mut changes: MessageReader<RoomChanged>, mut fade: ResMut<RoomFadeState>) {
    if changes.read().count() > 0 {
        fade.remaining = ROOM_FADE_TIME;
    }
}

pub fn update_room_fade(
    time: Res<Time>,
    mut fade: ResMut<RoomFadeState>,
    mut overlay: Query<&mut BackgroundColor, With<RoomFade>>,
) {
    let alpha = fade.alpha();
    for mut color in &mut overlay {
        let wanted = Color::BLACK.with_alpha(alpha);
        if color.0 != wanted {
            color.0 = wanted;
        }
    }
    if fade.remaining > 0.0 {
        fade.remaining = (fade.remaining - frame_delta(&time)).max(0.0);
    }
}
