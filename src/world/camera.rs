//! Smooth side-scrolling camera clamped to the room bounds.

use bevy::prelude::*;

use super::physics::{FacingDirection, RoomGeometry};
use crate::core::constants::{CAMERA_FOLLOW_RATE, CAMERA_LOOK_AHEAD, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::player::components::Player;

#[derive(Component, Debug, Clone, Copy)]
pub struct MainCamera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MainCamera));
}

/// Clamps a camera centre so the view stays inside `[0, room]` on an axis.
pub fn clamp_axis(target: f32, room: f32, view: f32) -> f32 {
    if room <= view {
        room / 2.0
    } else {
        target.clamp(view / 2.0, room - view / 2.0)
    }
}

pub fn follow_player(
    time: Res<Time>,
    geometry: Res<RoomGeometry>,
    player: Single<(&Transform, &FacingDirection), (With<Player>, Without<MainCamera>)>,
    mut camera: Single<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    let (player_transform, facing) = player.into_inner();
    let look_ahead = facing.sign() * CAMERA_LOOK_AHEAD;
    let target = Vec2::new(
        clamp_axis(
            player_transform.translation.x + look_ahead,
            geometry.size.x,
            WINDOW_WIDTH as f32,
        ),
        clamp_axis(
            player_transform.translation.y,
            geometry.size.y,
            WINDOW_HEIGHT as f32,
        ),
    );
    let destination = target.extend(camera.translation.z);
    camera
        .translation
        .smooth_nudge(&destination, CAMERA_FOLLOW_RATE, time.delta_secs());
}

/// Places the camera on the player immediately (start, load, respawn).
pub fn snap_to(camera: &mut Transform, position: Vec2, geometry: &RoomGeometry) {
    camera.translation.x = clamp_axis(position.x, geometry.size.x, WINDOW_WIDTH as f32);
    camera.translation.y = clamp_axis(position.y, geometry.size.y, WINDOW_HEIGHT as f32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_stays_inside_room() {
        assert_eq!(clamp_axis(0.0, 2000.0, 1000.0), 500.0);
        assert_eq!(clamp_axis(1990.0, 2000.0, 1000.0), 1500.0);
        assert_eq!(clamp_axis(900.0, 2000.0, 1000.0), 900.0);
        assert_eq!(clamp_axis(10.0, 600.0, 1000.0), 300.0);
    }
}
