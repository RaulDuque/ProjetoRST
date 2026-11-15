//! V2.3: scenery (backdrop, decoration, lights, fog) and the zoomed camera
//! in the headless main game.
//!
//! The scenery is presentation only: the room's collisions are exactly the
//! world layout, every piece leaves with its room, repeated visits never
//! accumulate entities, effects can be switched off, regions and rooms
//! without visual data keep the V2.2 look, the parallax follows its rule,
//! nothing of it reaches the save, and the zoom never scales the HUD.
//! Saves go to directories under the target dir; `saves/` is never touched.

use bevy::prelude::*;
use dark_castle_rpg::core::events::{Arrival, RoomRequest};
use dark_castle_rpg::ui::hud::HudColumn;
use dark_castle_rpg::world::camera::{CameraConfig, MainCamera};
use dark_castle_rpg::world::camera_rules::{CameraSettings, EffectToggles, REFERENCE_ASPECT};
use dark_castle_rpg::world::exploration_systems::WorldGraph;
use dark_castle_rpg::world::graph::WorldDefinition;
use dark_castle_rpg::world::physics::RoomGeometry;
use dark_castle_rpg::world::scenery::{Decor, DecorKind, DecorLayer, parallax_centre};
use dark_castle_rpg::world::scenery_systems::{ParallaxLayer, SceneryPiece};
use dark_castle_rpg::world::validation::{Issue, validate};

#[path = "support/world_app.rs"]
mod world_app;

use world_app::*;

const CAUSEWAY_PILOT: &str = "vigil_gates_causeway";
const STAIR_PILOT: &str = "fg_stair";
const SQUARE_PILOT: &str = "av_square";

fn go(app: &mut App, room: &str, at: (f32, f32)) {
    app.world_mut().write_message(RoomRequest {
        room: room.into(),
        arrival: Arrival::Position(Vec2::new(at.0, at.1)),
    });
    run(app, 3);
    assert_eq!(current(app), room);
}

fn pieces(app: &mut App) -> Vec<SceneryPiece> {
    let world = app.world_mut();
    world
        .query::<&SceneryPiece>()
        .iter(world)
        .copied()
        .collect()
}

fn count(app: &mut App, kind: SceneryPiece) -> usize {
    pieces(app).into_iter().filter(|p| *p == kind).count()
}

fn live(app: &mut App) -> usize {
    let world = app.world_mut();
    world.query::<Entity>().iter(world).count()
}

fn graph(app: &App) -> WorldDefinition {
    app.world().resource::<WorldGraph>().0.clone()
}

#[test]
fn pilot_rooms_get_their_scenery_and_the_collisions_do_not_change() {
    let mut app = game(&storage("v23_pilots"));
    let mut plain = game(&storage("v23_pilots_plain"));
    let mut off = plain.world().resource::<CameraConfig>().0;
    off.effects = EffectToggles::NONE;
    plain.world_mut().insert_resource(CameraConfig(off));
    let world = graph(&app);
    for (room, at) in [
        (CAUSEWAY_PILOT, (1000.0, 63.0)),
        (STAIR_PILOT, (640.0, 63.0)),
        (SQUARE_PILOT, (900.0, 63.0)),
    ] {
        go(&mut app, room, at);
        go(&mut plain, room, at);
        let def = world.room(room).unwrap_or_else(|| panic!("{room}"));
        assert!(!def.decor.is_empty(), "{room} is a pilot");
        assert_eq!(
            count(&mut app, SceneryPiece::Decor),
            def.decor.len(),
            "{room}"
        );
        let lights = def.decor.iter().filter(|d| d.kind.is_light()).count();
        assert_eq!(count(&mut app, SceneryPiece::Light), lights, "{room}");
        assert_eq!(count(&mut app, SceneryPiece::Backdrop), 2, "{room}");
        assert_eq!(count(&mut app, SceneryPiece::Fog), 1, "{room}");
        assert!(pieces(&mut plain).is_empty(), "{room}");
        // The physics is the same with and without the scenery (walls,
        // platforms, object and door barriers included).
        let with = app.world().resource::<RoomGeometry>().clone();
        let without = plain.world().resource::<RoomGeometry>().clone();
        assert_eq!(with.solids, without.solids, "{room}");
        assert_eq!(with.size, without.size, "{room}");
        // ...and it contains the room's own layout.
        for solid in def.to_room_definition().solids() {
            assert!(with.solids.contains(&solid), "{room}: {solid:?}");
        }
    }
}

#[test]
fn rooms_and_regions_without_visual_data_keep_the_old_look() {
    let mut app = game(&storage("v23_plain"));
    // A room of a region with a look but no decoration: backdrop only.
    go(&mut app, "vigil_gates_approach", (300.0, 63.0));
    assert_eq!(count(&mut app, SceneryPiece::Decor), 0);
    assert_eq!(count(&mut app, SceneryPiece::Backdrop), 2);
    // A region without a look (Veiled Cloister): nothing at all.
    let world = graph(&app);
    let cloister = world
        .rooms
        .values()
        .find(|r| r.region == "veiled_cloister")
        .unwrap_or_else(|| panic!("no cloister room"));
    assert!(world.regions["veiled_cloister"].visual.is_none());
    let at = cloister.respawn_point();
    go(&mut app, &cloister.id.clone(), at);
    assert!(pieces(&mut app).is_empty());
}

#[test]
fn scenery_leaves_with_its_room_and_never_piles_up() {
    let mut app = game(&storage("v23_unload"));
    // One full cycle first: the map discovers each room once.
    go(&mut app, CAUSEWAY_PILOT, (1000.0, 63.0));
    go(&mut app, STAIR_PILOT, (640.0, 63.0));
    go(&mut app, SQUARE_PILOT, (900.0, 63.0));
    run(&mut app, 30);
    let square = pieces(&mut app).len();
    let entities = live(&mut app);
    for _ in 0..8 {
        go(&mut app, CAUSEWAY_PILOT, (1000.0, 63.0));
        go(&mut app, STAIR_PILOT, (640.0, 63.0));
        go(&mut app, SQUARE_PILOT, (900.0, 63.0));
    }
    run(&mut app, 30);
    assert_eq!(pieces(&mut app).len(), square, "only the square's scenery");
    assert_eq!(live(&mut app), entities, "no entity growth");
    // Generated images are shared: one per distinct piece, not per visit.
    let images = app
        .world()
        .resource::<dark_castle_rpg::world::scenery_systems::SceneryCache>()
        .len();
    assert!(images > 0 && images < 60, "{images}");
}

#[test]
fn effects_can_be_switched_off() {
    let mut app = game(&storage("v23_off"));
    let mut settings = app.world().resource::<CameraConfig>().0;
    settings.effects = EffectToggles::NONE;
    app.world_mut().insert_resource(CameraConfig(settings));
    go(&mut app, CAUSEWAY_PILOT, (1000.0, 63.0));
    assert!(pieces(&mut app).is_empty());
    // Only the lights back on.
    settings.effects.lights = true;
    app.world_mut().insert_resource(CameraConfig(settings));
    go(&mut app, SQUARE_PILOT, (900.0, 63.0));
    let p = pieces(&mut app);
    assert!(!p.is_empty() && p.iter().all(|k| *k == SceneryPiece::Light));
}

#[test]
fn parallax_layers_follow_the_camera_rule() {
    let mut app = game(&storage("v23_parallax"));
    go(&mut app, CAUSEWAY_PILOT, (300.0, 63.0));
    for x in [300.0, 1280.0, 2300.0] {
        go(&mut app, CAUSEWAY_PILOT, (x, 63.0));
        run(&mut app, 60);
        let world = app.world_mut();
        let cam = world
            .query_filtered::<&Transform, With<MainCamera>>()
            .single(world)
            .map(|t| t.translation)
            .unwrap_or_else(|e| panic!("{e}"));
        let layers: Vec<(ParallaxLayer, Vec3)> = world
            .query::<(&ParallaxLayer, &Transform)>()
            .iter(world)
            .map(|(l, t)| (*l, t.translation))
            .collect();
        assert_eq!(layers.len(), 2);
        for (layer, t) in layers {
            assert!((t.x - parallax_centre(cam.x, layer.room.x, layer.factor)).abs() < 1e-3);
            assert!((t.y - parallax_centre(cam.y, layer.room.y, layer.factor)).abs() < 1e-3);
        }
    }
}

#[test]
fn the_save_carries_no_scenery_and_a_load_rebuilds_it() {
    let saves = storage("v23_save");
    let mut app = game(&saves);
    go(&mut app, STAIR_PILOT, (640.0, 63.0));
    let before = pieces(&mut app).len();
    tap(&mut app, KeyCode::F5);
    run(&mut app, 3);
    let bytes = slot_bytes(&saves).unwrap_or_else(|| panic!("no save"));
    let text = String::from_utf8_lossy(&bytes);
    for word in ["decor", "parallax", "Pillar", "view_height"] {
        assert!(!text.contains(word), "save mentions {word}");
    }
    go(&mut app, SQUARE_PILOT, (900.0, 63.0));
    tap(&mut app, KeyCode::F9);
    run(&mut app, 3);
    assert_eq!(current(&app), STAIR_PILOT);
    assert_eq!(pieces(&mut app).len(), before);
}

#[test]
fn the_zoom_never_scales_the_hud() {
    let mut app = game(&storage("v23_hud"));
    let hud = |app: &mut App| {
        let world = app.world_mut();
        world
            .query_filtered::<&Node, With<HudColumn>>()
            .single(world)
            .map(|n| format!("{:?} {:?} {:?} {:?}", n.left, n.top, n.width, n.height))
            .unwrap_or_else(|e| panic!("{e}"))
    };
    let before = hud(&mut app);
    let mut settings = app.world().resource::<CameraConfig>().0;
    for height in [720.0, 480.0, 360.0] {
        settings.view_height = height;
        app.world_mut().insert_resource(CameraConfig(settings));
        run(&mut app, 2);
        assert_eq!(hud(&mut app), before, "view height {height}");
        // The zoom changed the camera projection only.
        let world = app.world_mut();
        let projection = world
            .query_filtered::<&Projection, With<MainCamera>>()
            .single(world)
            .map(|p| format!("{p:?}"))
            .unwrap_or_else(|e| panic!("{e}"));
        assert!(projection.contains(&format!("{height:?}")), "{projection}");
    }
}

#[test]
fn shipped_camera_settings_are_the_documented_defaults() {
    let text = std::fs::read_to_string(root().join("assets/data/camera.ron"))
        .unwrap_or_else(|e| panic!("{e}"));
    let shipped = CameraSettings::from_ron(&text).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(shipped, CameraSettings::default());
    assert!(shipped.zoom() > 1.5, "the V2.3 view is zoomed in");
    let (w, h) = shipped.view_size(REFERENCE_ASPECT);
    assert!(w >= 640.0 && h >= 360.0, "never a cramped view: {w}x{h}");
}

#[test]
fn invalid_scenery_data_is_refused_and_old_data_still_loads() {
    let mut world = world();
    let report = validate(&world);
    assert!(report.errors.is_empty(), "{:?}", report.errors);
    // A statue in the front layer, and a torch outside its room.
    let room = world
        .rooms
        .get_mut(CAUSEWAY_PILOT)
        .unwrap_or_else(|| panic!("causeway"));
    room.decor.push(Decor {
        kind: DecorKind::Statue,
        at: (500.0, 40.0),
        layer: DecorLayer::Front,
        flip: false,
    });
    let report = validate(&world);
    assert!(
        report
            .errors
            .iter()
            .any(|e| matches!(e, Issue::BadScenery(_)))
    );
    // Region and room files written before V2.3 (no visual, no decor) parse.
    let old = r#"(
        region: (id: "old", name: "Old", color: (0.5, 0.5, 0.5)),
        rooms: [(id: "old_room", name: "Old", region: "old", role: Optional,
            map: (x: 40, y: 40, w: 1, h: 1), background: (0.0, 0.0, 0.0))],
    )"#;
    let file: dark_castle_rpg::world::graph::RegionFile =
        ron::from_str(old).unwrap_or_else(|e| panic!("{e}"));
    assert!(file.region.visual.is_none() && file.rooms[0].decor.is_empty());
}

#[test]
fn scenery_cost_is_measured_against_the_same_rooms_without_it() {
    use std::time::Instant;

    fn measure(app: &mut App) -> String {
        // First visit (art generated and cached), then a later visit.
        let t = Instant::now();
        go(app, CAUSEWAY_PILOT, (1000.0, 63.0));
        let first = t.elapsed().as_secs_f64() * 1000.0;
        go(app, SQUARE_PILOT, (900.0, 63.0));
        let t = Instant::now();
        go(app, CAUSEWAY_PILOT, (1000.0, 63.0));
        let again = t.elapsed().as_secs_f64() * 1000.0;
        run(app, 60);
        let frames = 300;
        let mut worst = 0.0f64;
        let t = Instant::now();
        for _ in 0..frames {
            let f = Instant::now();
            app.update();
            worst = worst.max(f.elapsed().as_secs_f64() * 1000.0);
        }
        let avg = t.elapsed().as_secs_f64() * 1000.0 / frames as f64;
        let sprites = {
            let world = app.world_mut();
            world.query::<&Sprite>().iter(world).count()
        };
        let images = app
            .world()
            .resource::<dark_castle_rpg::world::scenery_systems::SceneryCache>()
            .len();
        format!(
            "avg {avg:.4} ms/frame, worst {worst:.4} ms, entities {}, sprites {sprites}, \
             scenery pieces {}, generated images {images}, room change first {first:.2} ms, \
             again {again:.2} ms",
            live(app),
            pieces(app).len()
        )
    }
    let mut on = game(&storage("v23_perf_on"));
    let mut off = game(&storage("v23_perf_off"));
    let mut s = off.world().resource::<CameraConfig>().0;
    s.effects = EffectToggles::NONE;
    off.world_mut().insert_resource(CameraConfig(s));
    let with = measure(&mut on);
    let without = measure(&mut off);
    let report = format!(
        "causeway pilot, headless dev profile, 300 frames\nwith scenery:    {with}\nwithout scenery: {without}\n"
    );
    let dir = root().join("target/tmp");
    let _ = std::fs::create_dir_all(&dir);
    let _ = std::fs::write(dir.join("scenery_v23_perf.txt"), &report);
    println!("{report}");
    assert_eq!(pieces(&mut off).len(), 0);
}
