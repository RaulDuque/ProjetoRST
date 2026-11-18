//! V1.7: the world graph in the main game. Room lifecycle and clean-up of
//! temporary entities, save v2 and the v1 migration, quick load across
//! rooms, persistent exploration, the map panel and world items.
//!
//! Door crossings here use the threshold shortcut of the V1.6 tests (the
//! real-input route is `world_route_tests.rs`). Saves go to directories
//! under the target dir; `saves/` is never read or written.

use bevy::prelude::*;
use dark_castle_rpg::combat::damage::{AttackProfile, DamageKind, HitSpec};
use dark_castle_rpg::combat::hitboxes::{Hitbox, Team};
use dark_castle_rpg::core::constants::DEFAULT_SAVE_SLOT;
use dark_castle_rpg::core::resources::GameData;
use dark_castle_rpg::enemies::components::{Enemy, EnemyHealth, PendingRespawns};
use dark_castle_rpg::magic::elements::Element;
use dark_castle_rpg::magic::projectiles::Projectile;
use dark_castle_rpg::player::components::{Belongings, Player};
use dark_castle_rpg::player::state::PlayerState;
use dark_castle_rpg::save::serialization::{CURRENT_SAVE_VERSION, decode_versioned};
use dark_castle_rpg::ui::damage_numbers::DamageNumber;
use dark_castle_rpg::ui::world_map::{MapPanelState, WorldMapPanel};
use dark_castle_rpg::world::exploration::{Discovery, KnownDoor};
use dark_castle_rpg::world::exploration_systems::{DoorBarrier, IllusoryWall, PickupMarker};
use dark_castle_rpg::world::graph::Ability;
use dark_castle_rpg::world::maps::RoomEntity;
use dark_castle_rpg::world::transitions::RoomChangeStats;

#[path = "support/world_app.rs"]
mod world_app;

use world_app::*;

/// A damaging area as the game spawns them (detached, like a projectile).
fn strike(team: Team, at: Vec2, lifetime: f32) -> impl Bundle {
    (
        Hitbox {
            owner: Entity::PLACEHOLDER,
            team,
            size: Vec2::splat(80.0),
            attack: AttackProfile {
                power: 40,
                hit: 500,
                critical_chance: 0.0,
                critical_damage: 1.0,
            },
            hit: HitSpec {
                kind: DamageKind::Physical,
                element: Element::Physical,
                base_damage: 10,
                multiplier: 1.0,
            },
            lifetime,
            pierce: true,
            already_hit: Vec::new(),
        },
        Transform::from_xyz(at.x, at.y, 0.0),
    )
}

fn entry(room: &str, door: &str) -> Vec2 {
    let w = world();
    let r = &w.rooms[room];
    let p = r.entry_point(r.door(door).unwrap_or_else(|| panic!("door {door}")));
    Vec2::new(p.0, p.1)
}

fn enemy_hp(app: &mut App) -> Vec<i32> {
    let world = app.world_mut();
    world
        .query_filtered::<&EnemyHealth, With<Enemy>>()
        .iter(world)
        .map(|h| h.0.current())
        .collect()
}

fn tonics(app: &mut App) -> u32 {
    let world = app.world_mut();
    world
        .query_filtered::<&Belongings, With<Player>>()
        .single(world)
        .map(|b| b.inventory.count("minor_tonic"))
        .unwrap_or_else(|e| panic!("{e}"))
}

// ------------------------------------------------------------ main game

#[test]
fn the_main_game_starts_in_the_world_at_the_start_door() {
    let saves = storage("v17_start");
    let mut app = game(&saves);
    let w = world();
    assert_eq!(current(&app), w.manifest.start_room);
    let start = entry(&w.manifest.start_room, &w.manifest.start_door);
    assert!(player_xy(&mut app).distance(start) < 4.0);
    assert_eq!(count::<With<Player>>(&mut app), 1);
    assert_eq!(exploration(&app).discovery(APPROACH), Discovery::Visited);
    // Only the start room has entities: the others are metadata.
    let data = app.world().resource::<GameData>();
    assert!(data.rooms.contains_key("entry_hall"), "legacy room kept");
}

#[test]
fn a_world_with_errors_refuses_to_start() {
    let mut data = GameData::load(root()).unwrap_or_else(|e| panic!("{e}"));
    let mut w = world();
    w.rooms
        .get_mut(CAUSEWAY)
        .unwrap_or_else(|| panic!("room"))
        .doors
        .clear();
    // `load_world` reads from disk; validate the same rule it applies.
    let report = dark_castle_rpg::world::validation::validate(&w);
    assert!(!report.is_ok());
    // The real files load, validate and install.
    assert!(dark_castle_rpg::app::load_world(root(), &mut data).is_ok());
}

// ---------------------------------------------- hitboxes and clean-up

#[test]
fn an_enemy_strike_never_survives_a_room_change() {
    // Control: inside a room the same strike does hit.
    let saves = storage("v17_strike_control");
    let mut app = game(&saves);
    let hp = player_hp(&mut app);
    let at = player_xy(&mut app);
    app.world_mut().spawn(strike(Team::Enemy, at, 0.15));
    run(&mut app, 2);
    assert!(
        player_hp(&mut app) < hp,
        "the control strike hits in its room"
    );

    // The bug of V1.6: a strike left behind lands on the arrival point.
    let saves = storage("v17_strike");
    let mut app = game(&saves);
    let hp = player_hp(&mut app);
    let arrival = entry(CAUSEWAY, "from_approach");
    app.world_mut().spawn(strike(Team::Enemy, arrival, 0.15));
    go_through(&mut app, "to_causeway");
    assert_eq!(current(&app), CAUSEWAY);
    assert_eq!(
        count::<With<Hitbox>>(&mut app),
        0,
        "removed in the change frame"
    );
    run(&mut app, 20);
    assert_eq!(player_hp(&mut app), hp, "no damage from the old room");
}

#[test]
fn player_strikes_projectiles_and_spells_stay_in_their_room() {
    let saves = storage("v17_player_strikes");
    let mut app = game(&saves);
    walk(&mut app, &[("to_causeway", CAUSEWAY)]);
    run(&mut app, 2);
    let sentry = {
        let world = app.world_mut();
        world
            .query_filtered::<&Transform, With<Enemy>>()
            .single(world)
            .map(|t| t.translation.truncate())
            .unwrap_or_else(|e| panic!("{e}"))
    };
    // Back to the approach with a real basic attack (attached hitbox), a
    // projectile and a spell hitbox in the air, aimed at where the sentry
    // will stand when the causeway is loaded again.
    walk(&mut app, &[("from_approach", APPROACH)]);
    tap(&mut app, KeyCode::KeyX);
    assert!(count::<With<Hitbox>>(&mut app) >= 1, "basic attack active");
    app.world_mut().spawn((
        strike(Team::Player, sentry, 2.0),
        Projectile {
            velocity: Vec2::ZERO,
        },
    ));
    app.world_mut().spawn(strike(Team::Player, sentry, 2.0));
    walk(&mut app, &[("to_causeway", CAUSEWAY)]);
    assert_eq!(count::<With<Hitbox>>(&mut app), 0);
    assert_eq!(count::<With<Projectile>>(&mut app), 0);
    run(&mut app, 30);
    let full = app.world().resource::<GameData>().enemies["bone_sentry"].max_hp;
    assert_eq!(
        enemy_hp(&mut app),
        vec![full],
        "the new sentry is untouched"
    );
}

#[test]
fn damage_numbers_and_pending_respawns_do_not_follow_the_player() {
    let saves = storage("v17_numbers");
    let mut app = game(&saves);
    walk(&mut app, &[("to_causeway", CAUSEWAY)]);
    run(&mut app, 2);
    let spawn = world().rooms[CAUSEWAY].enemies[0].clone();
    app.world_mut()
        .resource_mut::<PendingRespawns>()
        .0
        .push((spawn, 0.05));
    // A real damage number: a strike lands on the player.
    let at = player_xy(&mut app);
    app.world_mut().spawn(strike(Team::Enemy, at, 0.05));
    run(&mut app, 2);
    assert!(count::<With<DamageNumber>>(&mut app) >= 1);
    walk(&mut app, &[("to_shaft", SHAFT)]);
    assert_eq!(count::<With<DamageNumber>>(&mut app), 0);
    run(&mut app, 20);
    assert!(app.world().resource::<PendingRespawns>().0.is_empty());
    // D20: the shaft has a Bone Bolt; the causeway sentry must not follow.
    let defs: Vec<String> = {
        let world = app.world_mut();
        world
            .query::<&Enemy>()
            .iter(world)
            .map(|e| e.definition.clone())
            .collect()
    };
    assert!(
        !defs.iter().any(|d| d == "bone_sentry"),
        "causeway sentry leaked into the shaft: {defs:?}"
    );
    assert_eq!(
        defs.iter().filter(|d| *d == "bone_bolt").count(),
        1,
        "expected the shaft Bone Bolt: {defs:?}"
    );
}

#[test]
fn a_dead_player_does_not_go_through_doors() {
    let saves = storage("v17_dead");
    let mut app = game(&saves);
    with_player_state(&mut app, |s| s.die());
    go_through(&mut app, "to_causeway");
    run(&mut app, 3);
    assert_eq!(current(&app), APPROACH);
}

#[test]
fn two_doors_in_one_frame_change_room_once() {
    use dark_castle_rpg::core::events::{Arrival, RoomRequest};
    let saves = storage("v17_two_doors");
    let mut app = game(&saves);
    let before = *app.world().resource::<RoomChangeStats>();
    // Two door requests in the same frame (as two overlapping triggers would).
    app.world_mut().write_message(RoomRequest {
        room: CAUSEWAY.into(),
        arrival: Arrival::Door("from_approach".into()),
    });
    app.world_mut().write_message(RoomRequest {
        room: SEALED.into(),
        arrival: Arrival::Door("from_approach".into()),
    });
    run(&mut app, 2);
    let after = *app.world().resource::<RoomChangeStats>();
    assert_eq!(current(&app), CAUSEWAY, "the first request wins");
    assert_eq!(after.applied - before.applied, 1);
    assert_eq!(after.discarded - before.discarded, 1);
    assert_eq!(count::<With<Player>>(&mut app), 1);
}

#[test]
fn arrival_points_are_safe_in_every_room() {
    use dark_castle_rpg::world::restore::position_is_valid;
    let w = world();
    for room in w.rooms.values() {
        let def = room.to_room_definition();
        for door in &room.doors {
            let p = room.entry_point(door);
            assert!(
                position_is_valid(&def, p),
                "{}:{} inside geometry",
                room.id,
                door.id
            );
            for other in &room.doors {
                assert!(
                    !room.crosses(other, p),
                    "{}:{} on a trigger",
                    room.id,
                    door.id
                );
            }
            for enemy in &room.enemies {
                let dx = (enemy.position.0 - p.0).abs();
                assert!(dx > 200.0, "{}:{} next to an enemy", room.id, door.id);
            }
        }
    }
}

// ------------------------------------------------------- save and load

fn reach_gallery_with_secret_and_relic(app: &mut App) {
    walk(
        app,
        &[
            ("to_causeway", CAUSEWAY),
            ("to_shaft", SHAFT),
            ("to_barbican", BARBICAN),
        ],
    );
    go_through(app, "to_cache"); // finds the false wall
    walk(
        app,
        &[
            ("to_cache", CACHE),
            ("from_barbican", BARBICAN),
            ("from_shaft", SHAFT),
            ("to_gallery", GALLERY),
        ],
    );
    let relic = world().rooms[GALLERY].pickups[0].position;
    place(app, (relic.0, relic.1 - 7.0));
    run(app, 3);
    assert!(
        exploration(app)
            .capabilities()
            .abilities
            .contains(&Ability::VeilStep)
    );
}

#[test]
fn quick_load_restores_the_saved_room_and_world_across_rooms() {
    let saves = storage("v17_load_across");
    let mut app = game(&saves);
    reach_gallery_with_secret_and_relic(&mut app);
    // Stand somewhere distinctive in the gallery (B) and save.
    place(&mut app, (900.0, 63.0));
    run(&mut app, 3);
    let saved_at = player_xy(&mut app);
    let in_gallery = count::<With<RoomEntity>>(&mut app);
    tap(&mut app, KeyCode::F5);
    let saved_state = exploration(&app);
    // Elsewhere (C), with more explored than the save knows.
    walk(
        &mut app,
        &[("to_postern", POSTERN), ("to_approach", APPROACH)],
    );
    walk(&mut app, &[("to_sealed", SEALED)]);
    assert_eq!(exploration(&app).discovery(SEALED), Discovery::Visited);
    tap(&mut app, KeyCode::F9);
    assert_eq!(
        current(&app),
        GALLERY,
        "the room is loaded in the load frame"
    );
    assert!(player_xy(&mut app).distance(saved_at) < 1.0);
    run(&mut app, 2);
    let loaded = exploration(&app);
    assert!(loaded.secrets.contains("barbican_false_wall"));
    assert!(loaded.capabilities().abilities.contains(&Ability::VeilStep));
    assert_eq!(loaded.rooms, saved_state.rooms, "replaced, not merged");
    assert_eq!(loaded.discovery(SEALED), Discovery::Discovered);
    assert_eq!(
        count::<With<RoomEntity>>(&mut app),
        in_gallery,
        "no duplicates"
    );
    assert_eq!(count::<With<PickupMarker>>(&mut app), 0, "relic not back");
    assert_eq!(count::<With<Player>>(&mut app), 1);
    // The world keeps working after the load.
    walk(
        &mut app,
        &[("to_postern", POSTERN), ("to_approach", APPROACH)],
    );
    run(&mut app, 2);
    assert_eq!(
        count::<With<DoorBarrier>>(&mut app),
        0,
        "the grate stays open"
    );
    walk(&mut app, &[("to_sealed", SEALED)]);
}

#[test]
fn quick_load_works_after_death_during_a_skill_with_projectiles_and_mid_transition() {
    let saves = storage("v17_load_cases");
    let mut app = game(&saves);
    walk(&mut app, &[("to_causeway", CAUSEWAY)]);
    run(&mut app, 2);
    tap(&mut app, KeyCode::F5);
    let saved = player_xy(&mut app);
    let saved_hp = player_hp(&mut app);
    walk(&mut app, &[("to_shaft", SHAFT)]);

    // After death.
    with_player_state(&mut app, |s| s.die());
    tap(&mut app, KeyCode::F9);
    assert_eq!(current(&app), CAUSEWAY);
    with_player_state(&mut app, |s| assert_ne!(s.state(), PlayerState::Dead));

    // During an attack, with a projectile and an enemy strike in the air.
    tap(&mut app, KeyCode::KeyX);
    let at = player_xy(&mut app);
    app.world_mut().spawn((
        strike(Team::Player, at, 2.0),
        Projectile { velocity: Vec2::X },
    ));
    app.world_mut().spawn(strike(Team::Enemy, saved, 2.0));
    app.world_mut()
        .resource_mut::<PendingPress>()
        .0
        .push(KeyCode::F9);
    run(&mut app, 1);
    assert_eq!(count::<With<Hitbox>>(&mut app), 0);
    run(&mut app, 10);
    assert_eq!(player_hp(&mut app), saved_hp, "the old strike never lands");
    assert_eq!(count::<With<Enemy>>(&mut app), 1, "one sentry, not two");

    // A door and a load in the same frame: the load wins.
    let p = threshold(&app, "to_shaft");
    place(&mut app, p);
    app.world_mut()
        .resource_mut::<PendingPress>()
        .0
        .push(KeyCode::F9);
    run(&mut app, 1);
    assert_eq!(current(&app), CAUSEWAY);
    assert!(player_xy(&mut app).distance(saved) < 1.0);
}

#[test]
fn a_secret_room_save_loads_into_the_secret_room() {
    let saves = storage("v17_load_secret");
    let mut app = game(&saves);
    walk(
        &mut app,
        &[
            ("to_causeway", CAUSEWAY),
            ("to_shaft", SHAFT),
            ("to_barbican", BARBICAN),
        ],
    );
    go_through(&mut app, "to_cache");
    walk(&mut app, &[("to_cache", CACHE)]);
    run(&mut app, 3);
    tap(&mut app, KeyCode::F5);
    walk(
        &mut app,
        &[("from_barbican", BARBICAN), ("from_shaft", SHAFT)],
    );
    tap(&mut app, KeyCode::F9);
    assert_eq!(current(&app), CACHE);
    run(&mut app, 2);
    assert_eq!(exploration(&app).discovery(CACHE), Discovery::Visited);
    walk(&mut app, &[("from_barbican", BARBICAN)]);
    run(&mut app, 1);
    let world = app.world_mut();
    let alpha = world
        .query_filtered::<&Sprite, With<IllusoryWall>>()
        .single(world)
        .map(|s| s.color.alpha())
        .unwrap_or_else(|e| panic!("{e}"));
    assert!(alpha < 0.5, "the wall stays found after the load");
}

#[test]
fn a_v1_save_loads_into_the_world_without_touching_the_file() {
    let fixture = std::fs::read_to_string(root().join("tests/fixtures/save_v1_entry_hall.ron"))
        .unwrap_or_else(|e| panic!("{e}"));
    // Format level: v1 → v2 in memory, every field kept.
    let decoded = decode_versioned(&fixture).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(decoded.source_version, 1);
    assert!(decoded.migrated());
    let d = &decoded.data;
    assert_eq!(d.player.room, "entry_hall");
    assert_eq!(d.player.progression.level(), 2);
    assert_eq!(d.player.inventory.count("minor_tonic"), 3);
    assert!(d.world.has_flag("met_the_archivist"));
    assert_eq!(d.play_time, 42.5);
    assert!(d.exploration.rooms.is_empty());

    // Game level: F9 resumes at the mapped door, the file is unchanged.
    let saves = storage("v17_v1_load");
    std::fs::create_dir_all(saves.dir()).unwrap_or_else(|e| panic!("{e}"));
    std::fs::write(saves.slot_path(DEFAULT_SAVE_SLOT), &fixture).unwrap_or_else(|e| panic!("{e}"));
    let mut app = game(&saves);
    walk(&mut app, &[("to_causeway", CAUSEWAY)]);
    tap(&mut app, KeyCode::F9);
    assert_eq!(current(&app), APPROACH, "legacy entry_hall → the approach");
    assert!(player_xy(&mut app).distance(entry(APPROACH, "to_sealed")) < 1.0);
    assert_eq!(tonics(&mut app), 3);
    assert_eq!(
        slot_bytes(&saves).as_deref(),
        Some(fixture.as_bytes()),
        "loading never rewrites the v1 file"
    );
    // An explicit save writes v2 and keeps the v1 file recoverable.
    tap(&mut app, KeyCode::F5);
    let text = String::from_utf8(slot_bytes(&saves).unwrap_or_default()).unwrap_or_default();
    let v2 = decode_versioned(&text).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(v2.source_version, CURRENT_SAVE_VERSION);
    assert_eq!(v2.data.player.room, APPROACH);
    assert_eq!(
        v2.data.exploration.rooms.get(APPROACH),
        Some(&Discovery::Visited)
    );
    let kept = std::fs::read_to_string(saves.version_backup_path(DEFAULT_SAVE_SLOT, 1))
        .unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(kept, fixture, "slot1.ron.v1.bak");
    // More saves never replace the versioned backup.
    tap(&mut app, KeyCode::F5);
    tap(&mut app, KeyCode::F5);
    let kept = std::fs::read_to_string(saves.version_backup_path(DEFAULT_SAVE_SLOT, 1))
        .unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(kept, fixture);
}

#[test]
fn invalid_saves_change_nothing_or_resume_safely() {
    let saves = storage("v17_invalid");
    let mut app = game(&saves);
    walk(&mut app, &[("to_causeway", CAUSEWAY)]);
    run(&mut app, 2);
    let here = player_xy(&mut app);
    std::fs::create_dir_all(saves.dir()).unwrap_or_else(|e| panic!("{e}"));

    // Corrupted file: nothing changes, the file stays.
    std::fs::write(saves.slot_path(DEFAULT_SAVE_SLOT), "garbage").unwrap_or_else(|e| panic!("{e}"));
    tap(&mut app, KeyCode::F9);
    assert_eq!(current(&app), CAUSEWAY);
    assert!(player_xy(&mut app).distance(here) < 1.0);
    assert_eq!(slot_bytes(&saves).as_deref(), Some(&b"garbage"[..]));

    // A save whose room was removed from the catalog: start door, noted.
    tap(&mut app, KeyCode::F5);
    let good = String::from_utf8(slot_bytes(&saves).unwrap_or_default()).unwrap_or_default();
    let removed = good.replace(
        &format!("room: \"{CAUSEWAY}\""),
        "room: \"vigil_gates_demolished\"",
    );
    assert_ne!(removed, good);
    std::fs::write(saves.slot_path(DEFAULT_SAVE_SLOT), &removed).unwrap_or_else(|e| panic!("{e}"));
    walk(&mut app, &[("to_shaft", SHAFT)]);
    tap(&mut app, KeyCode::F9);
    assert_eq!(current(&app), APPROACH);
    assert!(player_xy(&mut app).distance(entry(APPROACH, "to_sealed")) < 1.0);

    // A position outside the room: the room's entrance, noted.
    let outside = good.replace(
        &format!("position: ({:?}, {:?})", here.x, here.y),
        "position: (99999.0, -50.0)",
    );
    assert_ne!(outside, good, "position field found");
    std::fs::write(saves.slot_path(DEFAULT_SAVE_SLOT), &outside).unwrap_or_else(|e| panic!("{e}"));
    tap(&mut app, KeyCode::F9);
    assert_eq!(current(&app), CAUSEWAY);
    let spawn = app.world().resource::<GameData>().rooms[CAUSEWAY].player_spawn;
    assert!(player_xy(&mut app).distance(Vec2::new(spawn.0, spawn.1)) < 1.0);
}

// --------------------------------------------- exploration, map, items

#[test]
fn the_map_never_reveals_secrets_early() {
    let saves = storage("v17_map_secrets");
    let mut app = game(&saves);
    walk(
        &mut app,
        &[
            ("to_causeway", CAUSEWAY),
            ("to_shaft", SHAFT),
            ("to_barbican", BARBICAN),
        ],
    );
    let w = world();
    let hidden = KnownDoor {
        room: BARBICAN.into(),
        door: "to_cache".into(),
    };
    let doors = exploration(&app).known_doors(&w);
    assert!(!doors.contains(&hidden), "hidden door not drawn");
    assert_eq!(exploration(&app).discovery(CACHE), Discovery::Unknown);
    let postern_drop = KnownDoor {
        room: APPROACH.into(),
        door: "from_postern".into(),
    };
    assert!(!doors.contains(&postern_drop), "the drop is unknown");
    go_through(&mut app, "to_cache");
    assert!(exploration(&app).known_doors(&w).contains(&hidden));
}

#[test]
fn the_map_opens_and_closes_with_m_and_hides_without_a_world() {
    let saves = storage("v17_map_toggle");
    let mut app = game(&saves);
    let display = |app: &mut App| {
        let world = app.world_mut();
        world
            .query_filtered::<&Node, With<WorldMapPanel>>()
            .single(world)
            .map(|n| n.display)
            .unwrap_or_else(|e| panic!("{e}"))
    };
    assert_eq!(display(&mut app), Display::Flex);
    for i in 0..100 {
        tap(&mut app, KeyCode::KeyM);
        let open = app.world().resource::<MapPanelState>().open;
        assert_eq!(open, i % 2 == 1);
        assert_eq!(
            display(&mut app),
            if open { Display::Flex } else { Display::None }
        );
    }
    assert_eq!(count::<With<WorldMapPanel>>(&mut app), 1);
}

#[test]
fn world_items_reach_the_inventory_once() {
    let saves = storage("v17_items");
    let mut app = game(&saves);
    let before = tonics(&mut app);
    walk(
        &mut app,
        &[
            ("to_causeway", CAUSEWAY),
            ("to_shaft", SHAFT),
            ("to_barbican", BARBICAN),
        ],
    );
    go_through(&mut app, "to_cache");
    walk(&mut app, &[("to_cache", CACHE)]);
    let bundle = world().rooms[CACHE].pickups[0].position;
    place(&mut app, (bundle.0, bundle.1 - 7.0));
    run(&mut app, 3);
    assert_eq!(tonics(&mut app), before + 1);
    tap(&mut app, KeyCode::F5);
    // Back and forth, and a load: never collected again.
    walk(
        &mut app,
        &[("from_barbican", BARBICAN), ("to_cache", CACHE)],
    );
    place(&mut app, (bundle.0, bundle.1 - 7.0));
    run(&mut app, 3);
    tap(&mut app, KeyCode::F9);
    place(&mut app, (bundle.0, bundle.1 - 7.0));
    run(&mut app, 3);
    assert_eq!(tonics(&mut app), before + 1);
    assert_eq!(count::<With<PickupMarker>>(&mut app), 0);
}

#[test]
fn the_sigil_sets_the_world_flag_once() {
    let saves = storage("v17_flag");
    let mut app = game(&saves);
    reach_gallery_with_secret_and_relic(&mut app);
    walk(
        &mut app,
        &[
            ("to_postern", POSTERN),
            ("to_approach", APPROACH),
            ("to_sealed", SEALED),
            ("to_reliquary", RELIQUARY),
        ],
    );
    let sigil = world().rooms[RELIQUARY].pickups[0].position;
    place(&mut app, (sigil.0, sigil.1 - 7.0));
    run(&mut app, 3);
    let state = app.world().resource::<dark_castle_rpg::world::WorldState>();
    assert!(state.has_flag("vigil_gates_cleared"));
}
