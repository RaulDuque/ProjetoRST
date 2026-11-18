//! V1.7 stress: repeated room changes, loads, deaths and secrets in the
//! headless main game. Each scenario checks that entities, resources and
//! the room state return to the same values (no leak, no duplicate), and
//! the measurements are written to `<target>/tmp/world_stress.txt`.

use std::fmt::Write as _;
use std::time::Instant;

use bevy::prelude::*;
use dark_castle_rpg::combat::hitboxes::Hitbox;
use dark_castle_rpg::enemies::components::{Enemy, PendingRespawns};
use dark_castle_rpg::magic::projectiles::Projectile;
use dark_castle_rpg::player::components::Player;
use dark_castle_rpg::player::state::PlayerState;
use dark_castle_rpg::world::exploration::Discovery;
use dark_castle_rpg::world::maps::RoomEntity;
use dark_castle_rpg::world::transitions::{RoomChangeStats, TransitionLock};
use dark_castle_rpg::world::validation::validate;

#[path = "support/route_driver.rs"]
mod route_driver;
#[path = "support/world_app.rs"]
mod world_app;

use world_app::*;

/// Live scene and UI entities (engine-internal entities, e.g. resources
/// stored as entities and created once by the first state transition, are
/// not game content and are left out).
fn entities(app: &mut App) -> usize {
    let world = app.world_mut();
    let scene = world
        .query_filtered::<(), With<Transform>>()
        .iter(world)
        .count();
    let ui = world
        .query_filtered::<(), (With<Node>, Without<Transform>)>()
        .iter(world)
        .count();
    scene + ui
}

/// Resident memory in kB (Linux), when available.
fn rss_kb() -> Option<u64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    let line = status.lines().find(|l| l.starts_with("VmRSS:"))?;
    line.split_whitespace().nth(1)?.parse().ok()
}

fn report(line: String) {
    let path = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("world_stress.txt");
    let mut text = std::fs::read_to_string(&path).unwrap_or_default();
    let _ = writeln!(text, "{line}");
    let _ = std::fs::write(&path, text);
    println!("{line}");
}

#[test]
fn a_hundred_transitions_with_enemies_and_projectiles_leak_nothing() {
    let saves = storage("v17_stress_doors");
    let mut app = game(&saves);
    walk(
        &mut app,
        &[("to_causeway", CAUSEWAY), ("from_approach", APPROACH)],
    );
    run(&mut app, 2);
    let baseline = entities(&mut app);
    let rss_before = rss_kb();
    let start = Instant::now();
    for i in 0..50 {
        // A projectile in flight on every exit.
        app.world_mut()
            .spawn((Projectile { velocity: Vec2::X }, Transform::default()));
        walk(&mut app, &[("to_causeway", CAUSEWAY)]);
        assert_eq!(count::<With<Projectile>>(&mut app), 0, "cycle {i}");
        run(&mut app, 1);
        assert_eq!(count::<With<Enemy>>(&mut app), 1, "cycle {i}");
        walk(&mut app, &[("from_approach", APPROACH)]);
        run(&mut app, 1);
        assert_eq!(count::<With<Enemy>>(&mut app), 0, "cycle {i}");
        assert_eq!(
            entities(&mut app),
            baseline,
            "entity count drift in cycle {i}"
        );
    }
    let elapsed = start.elapsed();
    assert_eq!(count::<With<Player>>(&mut app), 1);
    assert_eq!(count::<With<Hitbox>>(&mut app), 0);
    assert!(app.world().resource::<PendingRespawns>().0.is_empty());
    assert!(!app.world().resource::<TransitionLock>().is_locked());
    report(format!(
        "100 transitions (approach<->causeway, 2 frames each): {:.1} ms total, {:.3} ms/frame; entities {baseline} -> {}; rss {:?} -> {:?} kB",
        elapsed.as_secs_f64() * 1e3,
        elapsed.as_secs_f64() * 1e3 / 200.0,
        entities(&mut app),
        rss_before,
        rss_kb()
    ));
}

#[test]
fn back_to_back_transitions_every_frame_stay_consistent() {
    let saves = storage("v17_stress_rapid");
    let mut app = game(&saves);
    walk(&mut app, &[("to_causeway", CAUSEWAY), ("to_shaft", SHAFT)]);
    let baseline = entities(&mut app);
    let before = *app.world().resource::<RoomChangeStats>();
    for _ in 0..50 {
        go_through(&mut app, "from_causeway");
        go_through(&mut app, "to_shaft");
    }
    run(&mut app, 2);
    let after = *app.world().resource::<RoomChangeStats>();
    assert_eq!(current(&app), SHAFT);
    assert_eq!(after.applied - before.applied, 100);
    // D20: shaft hosts Bone Bolt; count stays stable across rapid transitions.
    assert_eq!(count::<With<Enemy>>(&mut app), 1);
    assert_eq!(entities(&mut app), baseline);
}

#[test]
fn repeated_save_and_load_keeps_one_room_and_one_player() {
    let saves = storage("v17_stress_load");
    let mut app = game(&saves);
    walk(&mut app, &[("to_causeway", CAUSEWAY)]);
    run(&mut app, 2);
    tap(&mut app, KeyCode::F5);
    let in_causeway = entities(&mut app);
    let save_start = Instant::now();
    for _ in 0..20 {
        tap(&mut app, KeyCode::F5);
    }
    let save_ms = save_start.elapsed().as_secs_f64() * 1e3 / 20.0;
    let load_start = Instant::now();
    for i in 0..50 {
        walk(&mut app, &[("to_shaft", SHAFT)]);
        tap(&mut app, KeyCode::F9);
        run(&mut app, 1);
        assert_eq!(current(&app), CAUSEWAY, "load {i}");
        assert_eq!(count::<With<Enemy>>(&mut app), 1, "load {i}");
        assert_eq!(entities(&mut app), in_causeway, "load {i}");
    }
    let load_ms = load_start.elapsed().as_secs_f64() * 1e3 / 50.0;
    report(format!(
        "save: {save_ms:.2} ms each (20); door + load + 1 frame: {load_ms:.2} ms each (50)"
    ));
}

#[test]
fn death_and_revive_in_different_rooms() {
    let saves = storage("v17_stress_death");
    let mut app = game(&saves);
    for (door, room) in [
        ("to_causeway", CAUSEWAY),
        ("to_shaft", SHAFT),
        ("to_barbican", BARBICAN),
        ("from_shaft", SHAFT),
    ] {
        walk(&mut app, &[(door, room)]);
        run(&mut app, 2);
        with_player_state(&mut app, |s| s.die());
        run(&mut app, 2);
        tap(&mut app, KeyCode::KeyR);
        run(&mut app, 2);
        with_player_state(&mut app, |s| assert_ne!(s.state(), PlayerState::Dead));
        assert_eq!(current(&app), room, "revived in the same room");
        let spawn = app
            .world()
            .resource::<dark_castle_rpg::core::resources::GameData>()
            .rooms[room]
            .player_spawn;
        assert!(player_xy(&mut app).distance(Vec2::new(spawn.0, spawn.1)) < 30.0);
        assert_eq!(count::<With<Player>>(&mut app), 1);
    }
}

#[test]
fn finding_a_secret_then_loading_an_older_save_forgets_it_consistently() {
    let saves = storage("v17_stress_secret");
    let mut app = game(&saves);
    walk(
        &mut app,
        &[
            ("to_causeway", CAUSEWAY),
            ("to_shaft", SHAFT),
            ("to_barbican", BARBICAN),
        ],
    );
    tap(&mut app, KeyCode::F5); // before the secret
    for i in 0..20 {
        go_through(&mut app, "to_cache");
        assert!(
            exploration(&app).secrets.contains("barbican_false_wall"),
            "{i}"
        );
        walk(&mut app, &[("to_cache", CACHE)]);
        tap(&mut app, KeyCode::F9);
        run(&mut app, 1);
        let state = exploration(&app);
        assert_eq!(current(&app), BARBICAN);
        assert!(!state.secrets.contains("barbican_false_wall"), "{i}");
        assert_eq!(state.discovery(CACHE), Discovery::Unknown, "{i}");
    }
}

#[test]
fn the_route_audit_detects_artificial_repositioning() {
    // Negative control of the route test: a single test placement is seen.
    let saves = storage("v17_stress_audit");
    let mut app = game(&saves);
    route_driver::install(&mut app, vec![route_driver::Step::Wait(30)]);
    run(&mut app, 5);
    place(&mut app, (900.0, 63.0));
    run(&mut app, 40);
    let driver = app.world().resource::<route_driver::RouteDriver>();
    assert_eq!(driver.artificial_repositions, 1, "{}", driver.report());
}

#[test]
fn graph_validation_and_world_loading_cost() {
    let start = Instant::now();
    let w = world();
    let load = start.elapsed();
    let start = Instant::now();
    for _ in 0..100 {
        assert!(validate(&w).is_ok());
    }
    let validate_each = start.elapsed() / 100;
    let mut app = game(&storage("v17_stress_rooms"));
    let mut per_room = Vec::new();
    for (door, room) in [
        ("to_causeway", CAUSEWAY),
        ("to_shaft", SHAFT),
        ("to_barbican", BARBICAN),
    ] {
        let t = Instant::now();
        walk(&mut app, &[(door, room)]);
        let change = t.elapsed();
        run(&mut app, 1);
        per_room.push(format!(
            "{room}: {} room entities, change frame {:.2} ms",
            count::<With<RoomEntity>>(&mut app),
            change.as_secs_f64() * 1e3
        ));
    }
    report(format!(
        "world load {:.2} ms; validate {:.3} ms; {}",
        load.as_secs_f64() * 1e3,
        validate_each.as_secs_f64() * 1e3,
        per_room.join("; ")
    ));
}
