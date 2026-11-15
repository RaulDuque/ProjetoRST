//! Side-scrolling camera (V2.3): a zoomed orthographic view of fixed world
//! height, framed by the pure rules of [`super::camera_rules`] and clamped
//! to the room bounds.
//!
//! The zoom only changes the projection (`ScalingMode::FixedVertical`):
//! physics, collisions, room sizes and the UI (which has its own pixel
//! layout) are unchanged. Window resizes keep the visible height and adapt
//! the width to the aspect ratio.

use bevy::camera::ScalingMode;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::camera_rules::{CAMERA_SETTINGS_PATH, CameraRig, CameraSettings, FollowInput};
use super::physics::{FacingDirection, Grounded, RoomGeometry, Velocity};
use crate::core::resources::project_root;
use crate::player::components::Player;

pub use super::camera_rules::clamp_axis;

#[derive(Component, Debug, Clone, Copy)]
pub struct MainCamera;

/// The camera's follow state.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct CameraFollow(pub CameraRig);

/// The camera settings in use (`assets/data/camera.ron`, or the defaults).
#[derive(Resource, Debug, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct CameraConfig(pub CameraSettings);

/// Reads `assets/data/camera.ron` once, unless a config was inserted
/// already (tests, showcases). Missing: defaults (info). Invalid: defaults
/// (error), so a typo never breaks the game.
pub fn load_camera_config(mut commands: Commands, existing: Option<Res<CameraConfig>>) {
    if existing.is_some() {
        return;
    }
    let path = project_root().join("assets").join(CAMERA_SETTINGS_PATH);
    let settings = match std::fs::read_to_string(&path) {
        Ok(text) => CameraSettings::from_ron(&text).unwrap_or_else(|error| {
            error!(%error, "invalid camera settings; defaults used");
            CameraSettings::default()
        }),
        Err(error) => {
            info!(%error, path = %path.display(), "no camera settings; defaults used");
            CameraSettings::default()
        }
    };
    commands.insert_resource(CameraConfig(settings));
}

fn projection(view_height: f32) -> Projection {
    Projection::Orthographic(OrthographicProjection {
        scaling_mode: ScalingMode::FixedVertical {
            viewport_height: view_height,
        },
        ..OrthographicProjection::default_2d()
    })
}

pub fn spawn_camera(mut commands: Commands, config: Option<Res<CameraConfig>>) {
    let settings = config.map(|c| c.0).unwrap_or_default();
    commands.spawn((
        Camera2d,
        MainCamera,
        CameraFollow::default(),
        projection(settings.view_height),
    ));
}

/// Applies a zoom change made at run time (showcases, diagnostics).
pub fn apply_zoom(
    config: Option<Res<CameraConfig>>,
    mut cameras: Query<&mut Projection, With<MainCamera>>,
) {
    let Some(config) = config.filter(|c| c.is_changed()) else {
        return;
    };
    for mut p in &mut cameras {
        let wanted = projection(config.view_height);
        if format!("{:?}", *p) != format!("{wanted:?}") {
            *p = wanted;
        }
    }
}

/// Visible world size: the configured height, the window's aspect (the
/// reference 16:9 without a window).
pub fn view_size(settings: &CameraSettings, window: Option<&Window>) -> Vec2 {
    let aspect = window.map_or(f32::NAN, |w| w.width() / w.height());
    let (w, h) = settings.view_size(aspect);
    Vec2::new(w, h)
}

fn follow_input(
    player: (
        &Transform,
        &FacingDirection,
        Option<&Velocity>,
        Option<&Grounded>,
    ),
    geometry: &RoomGeometry,
    view: Vec2,
    dt: f32,
) -> FollowInput {
    let (t, facing, velocity, grounded) = player;
    FollowInput {
        position: (t.translation.x, t.translation.y),
        velocity: velocity.map_or((0.0, 0.0), |v| (v.0.x, v.0.y)),
        grounded: grounded.is_none_or(|g| g.on_ground),
        facing: facing.sign(),
        room: (geometry.size.x, geometry.size.y),
        view: (view.x, view.y),
        dt,
    }
}

type PlayerView<'a> = (
    &'a Transform,
    &'a FacingDirection,
    Option<&'a Velocity>,
    Option<&'a Grounded>,
);

pub fn follow_player(
    time: Res<Time>,
    config: Option<Res<CameraConfig>>,
    geometry: Res<RoomGeometry>,
    window: Query<&Window, With<PrimaryWindow>>,
    player: Single<PlayerView, (With<Player>, Without<MainCamera>)>,
    camera: Single<(&mut Transform, &mut CameraFollow), (With<MainCamera>, Without<Player>)>,
) {
    if geometry.size == Vec2::ZERO {
        return;
    }
    let settings = config.map(|c| c.0).unwrap_or_default();
    let view = view_size(&settings, window.single().ok());
    let input = follow_input(player.into_inner(), &geometry, view, time.delta_secs());
    let (mut transform, mut follow) = camera.into_inner();
    let (x, y) = follow.0.step(&settings, &input);
    transform.translation.x = x;
    transform.translation.y = y;
}

/// Once, on the first frame with a loaded room: puts the camera on the
/// player (V1.7), so the area outside the room is never shown.
pub fn snap_on_start(
    mut done: Local<bool>,
    config: Option<Res<CameraConfig>>,
    geometry: Res<RoomGeometry>,
    window: Query<&Window, With<PrimaryWindow>>,
    player: Single<PlayerView, (With<Player>, Without<MainCamera>)>,
    camera: Single<(&mut Transform, &mut CameraFollow), (With<MainCamera>, Without<Player>)>,
) {
    if *done || geometry.size == Vec2::ZERO {
        return;
    }
    let settings = config.map(|c| c.0).unwrap_or_default();
    let view = view_size(&settings, window.single().ok());
    let input = follow_input(player.into_inner(), &geometry, view, 0.0);
    let (mut transform, mut follow) = camera.into_inner();
    follow.0.snap(&settings, &input);
    transform.translation.x = follow.0.centre.0;
    transform.translation.y = follow.0.centre.1;
    *done = true;
}

/// Places the camera on a position immediately (room change, load).
pub fn snap_to(
    camera: &mut Transform,
    follow: &mut CameraFollow,
    settings: &CameraSettings,
    position: Vec2,
    facing: f32,
    room: Vec2,
    view: Vec2,
) {
    let input = FollowInput {
        position: (position.x, position.y),
        velocity: (0.0, 0.0),
        grounded: true,
        facing,
        room: (room.x, room.y),
        view: (view.x, view.y),
        dt: 0.0,
    };
    follow.0.snap(settings, &input);
    camera.translation.x = follow.0.centre.0;
    camera.translation.y = follow.0.centre.1;
}
