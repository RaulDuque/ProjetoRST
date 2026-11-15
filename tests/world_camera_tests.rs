//! V1.8: the camera across room changes, in the headless main game.
//! V2.3: the view is the zoomed one of `CameraConfig` (no window here: the
//! reference 16:9 aspect), so every check uses that view size.
//!
//! After every change the view must lie inside the new room (nothing of an
//! unloaded area or of the outside is shown), the player must be on screen,
//! and the camera's projection (zoom) and scale must be untouched. Entries
//! by the four sides, small, large and vertical rooms, the start, a quick
//! load into another room and repeated transitions.
//!
//! Most changes are real door crossings (threshold shortcut of the V1.6
//! tests). No door of the world is entered *upwards* (rooms are climbed by
//! side doors), so the bottom entry asks the room lifecycle for it
//! (`RoomRequest` to the postern's floor door), the same path a load takes.

use bevy::prelude::*;
use dark_castle_rpg::core::events::{Arrival, RoomRequest};
use dark_castle_rpg::player::components::Player;
use dark_castle_rpg::world::camera::{CameraConfig, MainCamera};
use dark_castle_rpg::world::camera_rules::REFERENCE_ASPECT;
use dark_castle_rpg::world::physics::RoomGeometry;

#[path = "support/world_app.rs"]
mod world_app;

use world_app::*;

/// The view of the camera in use (headless: reference aspect).
fn view(app: &App) -> (f32, f32) {
    app.world()
        .resource::<CameraConfig>()
        .view_size(REFERENCE_ASPECT)
}

fn camera(app: &mut App) -> (Vec2, Vec3, String) {
    let world = app.world_mut();
    world
        .query_filtered::<(&Transform, &Projection), With<MainCamera>>()
        .single(world)
        .map(|(t, p)| (t.translation.truncate(), t.scale, format!("{p:?}")))
        .unwrap_or_else(|e| panic!("{e}"))
}

/// The view is inside the room on both axes (centred when the room is
/// smaller than the view), and the player is on screen.
fn assert_view_ok(app: &mut App, what: &str) {
    let (cam, _, _) = camera(app);
    let size = app.world().resource::<RoomGeometry>().size;
    let v = view(app);
    for (axis, c, room, view) in [("x", cam.x, size.x, v.0), ("y", cam.y, size.y, v.1)] {
        if room <= view {
            assert!((c - room / 2.0).abs() < 0.5, "{what}: {axis} centred");
        } else {
            assert!(
                c >= view / 2.0 - 0.5 && c <= room - view / 2.0 + 0.5,
                "{what}: {axis}={c} shows outside a room of {room}"
            );
        }
    }
    let player = player_xy(app);
    assert!(
        (player.x - cam.x).abs() <= v.0 / 2.0 && (player.y - cam.y).abs() <= v.1 / 2.0,
        "{what}: player {player} off screen (camera {cam})"
    );
}

fn request(app: &mut App, room: &str, door: &str) {
    app.world_mut().write_message(RoomRequest {
        room: room.into(),
        arrival: Arrival::Door(door.into()),
    });
    run(app, 1);
    assert_eq!(current(app), room);
}

#[test]
fn the_camera_starts_inside_the_start_room() {
    let mut app = game(&storage("v18_cam_start"));
    assert_eq!(current(&app), APPROACH);
    assert_view_ok(&mut app, "start");
    let (cam, _, _) = camera(&mut app);
    // V2.3: the zoomed view is smaller than a 1x1 room (1280x720), so the
    // camera frames the player inside the room instead of centring it.
    let v = view(&app);
    assert!(v.0 < 1280.0 && v.1 < 720.0, "zoomed view {v:?}");
    let player = player_xy(&mut app);
    assert!((cam.x - player.x).abs() < v.0 / 2.0, "{cam} {player}");
}

#[test]
fn entries_from_the_four_sides_keep_the_view_in_the_room() {
    let mut app = game(&storage("v18_cam_sides"));
    let (_, scale, projection) = camera(&mut app);
    // From the left side (approach → causeway, a 2x1 room).
    walk(&mut app, &[("to_causeway", CAUSEWAY)]);
    assert_view_ok(&mut app, "left entry");
    let v = view(&app);
    assert_eq!(camera(&mut app).0.x, v.0 / 2.0, "left end of a wide room");
    // From the right side (shaft → causeway).
    walk(
        &mut app,
        &[("to_shaft", SHAFT), ("from_causeway", CAUSEWAY)],
    );
    assert_view_ok(&mut app, "right entry");
    assert_eq!(
        camera(&mut app).0.x,
        2560.0 - v.0 / 2.0,
        "right end of a wide room"
    );
    // From the top (postern drop into the approach's ceiling door).
    request(&mut app, APPROACH, "from_postern");
    assert_view_ok(&mut app, "top entry");
    // From the bottom (the postern's floor door, a 1x2 room).
    request(&mut app, POSTERN, "to_approach");
    assert_view_ok(&mut app, "bottom entry");
    assert_eq!(camera(&mut app).0.y, v.1 / 2.0, "bottom of a tall room");
    let (_, after_scale, after_projection) = camera(&mut app);
    assert_eq!(scale, after_scale);
    assert_eq!(projection, after_projection, "zoom unchanged");
}

#[test]
fn small_large_and_vertical_rooms() {
    let mut app = game(&storage("v18_cam_sizes"));
    // Small (1x1).
    request(&mut app, "fg_threshold", "to_long_gallery");
    assert_view_ok(&mut app, "small");
    // Large (2x1), entering at its right end.
    request(&mut app, "fg_long_gallery", "to_threshold");
    assert_view_ok(&mut app, "large");
    // Vertical (1x3), entering at the top door (cell 2).
    request(&mut app, "fg_stair", "to_chapel");
    assert_view_ok(&mut app, "vertical, top");
    let (cam, _, _) = camera(&mut app);
    assert!(cam.y > 1000.0, "the camera is up with the player: {cam}");
    // ...and at the bottom door.
    request(&mut app, "fg_stair", "to_long_gallery");
    assert_view_ok(&mut app, "vertical, bottom");
    assert_eq!(camera(&mut app).0.y, view(&app).1 / 2.0);
    // The follow keeps it inside while the player stands still for a while.
    run(&mut app, 120);
    assert_view_ok(&mut app, "vertical, settled");
}

#[test]
fn a_quick_load_into_another_room_snaps_the_camera() {
    let mut app = game(&storage("v18_cam_load"));
    request(&mut app, "fg_stair", "to_chapel");
    run(&mut app, 20);
    tap(&mut app, KeyCode::F5);
    run(&mut app, 2);
    walk(&mut app, &[("to_chapel", "fg_relic_chapel")]);
    assert_view_ok(&mut app, "chapel");
    tap(&mut app, KeyCode::F9);
    assert_eq!(current(&app), "fg_stair");
    assert_view_ok(&mut app, "loaded in the stair (same frame)");
    let player = player_xy(&mut app);
    assert!(player.y > 1400.0, "{player}");
}

#[test]
fn repeated_transitions_never_show_outside_a_room() {
    let mut app = game(&storage("v18_cam_repeat"));
    let (_, scale, projection) = camera(&mut app);
    for i in 0..20 {
        walk(&mut app, &[("to_causeway", CAUSEWAY)]);
        assert_view_ok(&mut app, &format!("causeway #{i}"));
        walk(&mut app, &[("from_approach", APPROACH)]);
        assert_view_ok(&mut app, &format!("approach #{i}"));
    }
    let world = app.world_mut();
    assert_eq!(
        world
            .query_filtered::<(), With<MainCamera>>()
            .iter(world)
            .count(),
        1
    );
    assert_eq!(
        world
            .query_filtered::<(), With<Player>>()
            .iter(world)
            .count(),
        1
    );
    let (_, after_scale, after_projection) = camera(&mut app);
    assert_eq!((scale, projection), (after_scale, after_projection));
}
