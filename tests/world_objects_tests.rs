//! V1.8: interactive objects in the headless main game. Breakable walls and
//! real hitboxes, chests, levers, the hidden switch, the shortcut, their
//! persistence (save, load, new session) and repeated use.
//!
//! To reach a room deep in the galleries quickly, these tests ask the room
//! lifecycle for it (`RoomRequest`, the path a load takes) and, when a test
//! needs to stand somewhere, place the player (`place`). The real-input
//! proof that every room is reachable is `world_secrets_route_tests.rs`.
//! Saves go to directories under the target dir; `saves/` is never touched.

use bevy::prelude::*;
use dark_castle_rpg::combat::damage::{AttackProfile, DamageKind, HitSpec};
use dark_castle_rpg::combat::hitboxes::{Hitbox, Team};
use dark_castle_rpg::core::constants::{DEFAULT_SAVE_SLOT, ROOM_FADE_TIME};
use dark_castle_rpg::core::events::{Arrival, Notification, RoomRequest};
use dark_castle_rpg::magic::elements::Element;
use dark_castle_rpg::magic::projectiles::Projectile;
use dark_castle_rpg::player::components::{Belongings, Player};
use dark_castle_rpg::ui::room_fade::RoomFadeState;
use dark_castle_rpg::world::exploration::{DoorState, ObjectState};
use dark_castle_rpg::world::exploration_systems::{DoorBarrier, Exploration};
use dark_castle_rpg::world::graph::{Ability, Grant};
use dark_castle_rpg::world::interactions::ObjectMarker;
use dark_castle_rpg::world::maps::RoomEntity;
use dark_castle_rpg::world::physics::RoomGeometry;

#[path = "support/world_app.rs"]
mod world_app;

use world_app::*;

const STAIR: &str = "fg_stair";
const LONG_GALLERY: &str = "fg_long_gallery";
const LEVER_HALL: &str = "fg_lever_hall";
const BRIDGE: &str = "fg_return_bridge";
const WALK: &str = "fg_collapsed_walk";
const WALL: &str = "fg_cracked_wall";
const GALLERY_CHEST: &str = "fg_gallery_chest";
const BRIDGE_CHEST: &str = "fg_bridge_chest";
/// Standing in front of the cracked wall, facing it (walked there).
const AT_WALL: (f32, f32) = (113.0, 63.0);
const AT_GALLERY_CHEST: (f32, f32) = (2224.0, 63.0);
const AT_BRIDGE_CHEST: (f32, f32) = (1028.0, 63.0);
const AT_TRAPDOOR_LEVER: (f32, f32) = (268.0, 63.0);
const ON_HALL_LEDGE: (f32, f32) = (418.0, 169.0);

/// Notifications seen by the test (to count repeated events).
#[derive(Resource, Default)]
struct Notes(Vec<String>);

fn record(mut messages: MessageReader<Notification>, mut notes: ResMut<Notes>) {
    notes.0.extend(messages.read().map(|n| n.0.clone()));
}

fn objects_game(saves: &dark_castle_rpg::save::save_manager::SaveStorage) -> App {
    let mut app = game(saves);
    app.init_resource::<Notes>().add_systems(PostUpdate, record);
    app
}

fn notes_with(app: &App, text: &str) -> usize {
    app.world()
        .resource::<Notes>()
        .0
        .iter()
        .filter(|n| n.contains(text))
        .count()
}

/// Asks the room lifecycle for `room`, arriving through `door`.
fn enter(app: &mut App, room: &str, door: &str) {
    app.world_mut().write_message(RoomRequest {
        room: room.into(),
        arrival: Arrival::Door(door.into()),
    });
    run(app, 2);
    assert_eq!(current(app), room);
}

/// Grants (as if collected) what the deeper rooms need.
fn grant(app: &mut App, grants: &[Grant]) {
    let mut exploration = app.world_mut().resource_mut::<Exploration>();
    for g in grants {
        exploration.granted.push(g.clone());
    }
}

fn items(app: &mut App, id: &str) -> u32 {
    let world = app.world_mut();
    world
        .query_filtered::<&Belongings, With<Player>>()
        .single(world)
        .map(|b| b.inventory.count(id))
        .unwrap_or_else(|e| panic!("{e}"))
}

fn object_state(app: &App, room: &str, id: &str) -> ObjectState {
    let w = world();
    let object = w.rooms[room]
        .object(id)
        .unwrap_or_else(|| panic!("no object {id}"))
        .clone();
    exploration(app).object_state(&object)
}

/// Faces left (walks a frame) and stands in front of the wall.
fn stand_at_wall(app: &mut App) {
    enter(app, STAIR, "to_long_gallery");
    place(app, (AT_WALL.0 + 20.0, AT_WALL.1));
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::ArrowLeft);
    run(app, 8);
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .release(KeyCode::ArrowLeft);
    run(app, 2);
}

fn wall_is_solid(app: &App) -> bool {
    let w = world();
    let wall = w.rooms[STAIR].object(WALL).map(|o| o.area);
    let geometry = app.world().resource::<RoomGeometry>();
    wall.is_some_and(|a| {
        let s = a.solid();
        geometry.solids.iter().any(|g| g.aabb == s.aabb)
    })
}

fn hitbox(team: Team, kind: DamageKind, element: Element, at: (f32, f32)) -> impl Bundle {
    (
        Hitbox {
            owner: Entity::PLACEHOLDER,
            team,
            size: Vec2::new(46.0, 26.0),
            attack: AttackProfile {
                power: 10,
                hit: 500,
                critical_chance: 0.0,
                critical_damage: 1.0,
            },
            hit: HitSpec {
                kind,
                element,
                base_damage: 5,
                multiplier: 1.0,
            },
            lifetime: 0.15,
            pierce: true,
            already_hit: Vec::new(),
        },
        Transform::from_xyz(at.0, at.1, 0.0),
    )
}

// ------------------------------------------------------ combat and objects

#[test]
fn a_real_strike_breaks_the_wall_once_and_opens_the_passage() {
    let mut app = objects_game(&storage("v18_wall_strike"));
    stand_at_wall(&mut app);
    assert!(wall_is_solid(&app));
    assert_eq!(object_state(&app, STAIR, WALL), ObjectState::Intact);
    let door = world().rooms[STAIR]
        .door("to_cracked_cache")
        .cloned()
        .unwrap_or_else(|| panic!("door"));
    assert_eq!(exploration(&app).door_state(&door), DoorState::Hidden);
    assert!(
        !exploration(&app)
            .known_doors(&world())
            .iter()
            .any(|k| k.door == "to_cracked_cache"),
        "the secret door is not on the map before the wall breaks"
    );
    tap(&mut app, KeyCode::KeyX);
    run(&mut app, 12);
    assert_eq!(object_state(&app, STAIR, WALL), ObjectState::Broken);
    assert!(!wall_is_solid(&app), "the collision is gone");
    assert_eq!(count::<With<DoorBarrier>>(&mut app), 0);
    assert_eq!(exploration(&app).door_state(&door), DoorState::Open);
    // Attacking the rubble again: no new event, nothing recorded twice.
    for _ in 0..3 {
        tap(&mut app, KeyCode::KeyX);
        run(&mut app, 20);
    }
    assert_eq!(notes_with(&app, "crumbles"), 1);
    assert_eq!(
        exploration(&app)
            .secrets
            .iter()
            .filter(|s| *s == WALL)
            .count(),
        1
    );
    // The passage works: through it to the cache and back.
    walk(&mut app, &[("to_cracked_cache", "fg_cracked_cache")]);
    walk(&mut app, &[("to_stair", STAIR)]);
    assert!(
        !wall_is_solid(&app),
        "still broken after leaving and coming back"
    );
}

#[test]
fn only_player_attacks_in_reach_and_in_this_room_break_it() {
    let mut app = objects_game(&storage("v18_wall_rules"));
    stand_at_wall(&mut app);
    // An attack out of reach.
    place(&mut app, (400.0, 63.0));
    tap(&mut app, KeyCode::KeyX);
    run(&mut app, 20);
    assert_eq!(object_state(&app, STAIR, WALL), ObjectState::Intact);
    // An enemy's strike on the wall.
    app.world_mut().spawn(hitbox(
        Team::Enemy,
        DamageKind::Physical,
        Element::Physical,
        (60.0, 100.0),
    ));
    run(&mut app, 2);
    assert_eq!(object_state(&app, STAIR, WALL), ObjectState::Intact);
    // A visual effect (sprite, no hitbox) over the wall does nothing.
    app.world_mut().spawn((
        Sprite::from_color(Color::WHITE, Vec2::splat(80.0)),
        Transform::from_xyz(60.0, 100.0, 8.0),
    ));
    run(&mut app, 2);
    assert_eq!(object_state(&app, STAIR, WALL), ObjectState::Intact);
    // A player strike at the wall's coordinates, but in another room.
    enter(&mut app, LONG_GALLERY, "to_stair");
    app.world_mut().spawn(hitbox(
        Team::Player,
        DamageKind::Physical,
        Element::Physical,
        (60.0, 100.0),
    ));
    run(&mut app, 2);
    assert_eq!(object_state(&app, STAIR, WALL), ObjectState::Intact);
    // A player spell (projectile hitbox) does break it: any attack counts.
    enter(&mut app, STAIR, "to_long_gallery");
    app.world_mut().spawn((
        hitbox(
            Team::Player,
            DamageKind::Magical,
            Element::Fire,
            (60.0, 100.0),
        ),
        Projectile {
            velocity: Vec2::ZERO,
        },
    ));
    run(&mut app, 2);
    assert_eq!(object_state(&app, STAIR, WALL), ObjectState::Broken);
}

#[test]
fn an_interrupted_attack_does_not_break_twice() {
    let mut app = objects_game(&storage("v18_wall_interrupted"));
    stand_at_wall(&mut app);
    tap(&mut app, KeyCode::KeyX);
    // Hurt in the frame after the swing: the hitbox is cancelled.
    with_player_state(&mut app, |s| {
        s.hurt(0.35);
    });
    run(&mut app, 30);
    let first = object_state(&app, STAIR, WALL);
    tap(&mut app, KeyCode::KeyX);
    run(&mut app, 20);
    assert_eq!(object_state(&app, STAIR, WALL), ObjectState::Broken);
    assert!(notes_with(&app, "crumbles") <= 1, "{first:?}");
    assert_eq!(notes_with(&app, "crumbles"), 1);
}

#[test]
fn a_chest_opens_once_and_its_item_reaches_the_inventory_once() {
    let mut app = objects_game(&storage("v18_chest_once"));
    enter(&mut app, LONG_GALLERY, "to_threshold");
    let before = items(&mut app, "minor_tonic");
    // A player projectile over the chest does not open it.
    app.world_mut().spawn((
        hitbox(
            Team::Player,
            DamageKind::Magical,
            Element::Fire,
            AT_GALLERY_CHEST,
        ),
        Projectile {
            velocity: Vec2::ZERO,
        },
    ));
    run(&mut app, 2);
    assert_eq!(
        object_state(&app, LONG_GALLERY, GALLERY_CHEST),
        ObjectState::Closed
    );
    place(&mut app, AT_GALLERY_CHEST);
    run(&mut app, 3);
    for _ in 0..5 {
        tap(&mut app, KeyCode::KeyD);
        run(&mut app, 2);
    }
    assert_eq!(
        object_state(&app, LONG_GALLERY, GALLERY_CHEST),
        ObjectState::Opened
    );
    assert_eq!(items(&mut app, "minor_tonic"), before + 1);
    assert_eq!(notes_with(&app, "Found Minor Tonic"), 1);
    // Leaving and coming back: still open, nothing granted again.
    walk(&mut app, &[("to_threshold", "fg_threshold")]);
    walk(&mut app, &[("to_long_gallery", LONG_GALLERY)]);
    place(&mut app, AT_GALLERY_CHEST);
    run(&mut app, 3);
    tap(&mut app, KeyCode::KeyD);
    run(&mut app, 3);
    assert_eq!(items(&mut app, "minor_tonic"), before + 1);
}

#[test]
fn interacting_uses_its_own_key_and_needs_idle_or_walking() {
    let mut app = objects_game(&storage("v18_interact_state"));
    enter(&mut app, LONG_GALLERY, "to_threshold");
    place(&mut app, AT_GALLERY_CHEST);
    run(&mut app, 3);
    let closed = |app: &App| object_state(app, LONG_GALLERY, GALLERY_CHEST) == ObjectState::Closed;
    // `↑` (part of the `↓ ↑ + ATTACK` command) never uses an object.
    tap(&mut app, KeyCode::ArrowDown);
    tap(&mut app, KeyCode::ArrowUp);
    run(&mut app, 2);
    assert!(closed(&app));
    // Crouching (`↓` held): the interact key does nothing.
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::ArrowDown);
    run(&mut app, 4);
    tap(&mut app, KeyCode::KeyD);
    run(&mut app, 2);
    assert!(closed(&app));
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .release(KeyCode::ArrowDown);
    run(&mut app, 4);
    // Standing: it opens.
    tap(&mut app, KeyCode::KeyD);
    run(&mut app, 2);
    assert!(!closed(&app));
}

#[test]
fn a_lever_is_pulled_once_and_opens_its_gate() {
    let mut app = objects_game(&storage("v18_lever_once"));
    enter(&mut app, LEVER_HALL, "to_upper_gallery");
    let gate = world().rooms[LEVER_HALL]
        .door("to_astral_gate")
        .cloned()
        .unwrap_or_else(|| panic!("door"));
    assert_eq!(exploration(&app).door_state(&gate), DoorState::Locked);
    assert_eq!(count::<With<DoorBarrier>>(&mut app), 1);
    // From the floor the lever is out of reach.
    place(&mut app, (418.0, 63.0));
    run(&mut app, 3);
    tap(&mut app, KeyCode::KeyD);
    run(&mut app, 2);
    assert_eq!(
        object_state(&app, LEVER_HALL, "fg_hall_lever"),
        ObjectState::Off
    );
    place(&mut app, ON_HALL_LEDGE);
    run(&mut app, 3);
    for _ in 0..50 {
        tap(&mut app, KeyCode::KeyD);
    }
    run(&mut app, 2);
    assert_eq!(
        object_state(&app, LEVER_HALL, "fg_hall_lever"),
        ObjectState::On
    );
    let pulls = exploration(&app)
        .granted
        .iter()
        .filter(|g| **g == Grant::Flag("fg_hall_gate_open".into()))
        .count();
    assert_eq!(pulls, 1, "50 presses, one activation");
    assert_eq!(notes_with(&app, "mechanism"), 1);
    assert_eq!(exploration(&app).door_state(&gate), DoorState::Open);
    assert_eq!(count::<With<DoorBarrier>>(&mut app), 0);
    assert!(
        app.world()
            .resource::<dark_castle_rpg::world::WorldState>()
            .has_flag("fg_hall_gate_open")
    );
}

#[test]
fn the_hidden_switch_reveals_the_oratory_only_when_used() {
    let mut app = objects_game(&storage("v18_switch"));
    enter(&mut app, WALK, "to_chapel");
    let state = exploration(&app);
    assert!(!state.rooms.contains_key("fg_hidden_oratory"));
    // Standing on it is not enough: no reveal by proximity.
    place(&mut app, (1162.0, 63.0));
    run(&mut app, 30);
    assert_eq!(
        object_state(&app, WALK, "fg_loose_brick"),
        ObjectState::Hidden
    );
    assert_eq!(
        count::<With<DoorBarrier>>(&mut app),
        1,
        "masonry in the opening"
    );
    tap(&mut app, KeyCode::KeyD);
    run(&mut app, 2);
    assert_eq!(
        object_state(&app, WALK, "fg_loose_brick"),
        ObjectState::Found
    );
    assert_eq!(count::<With<DoorBarrier>>(&mut app), 0);
    assert!(exploration(&app).rooms.contains_key("fg_hidden_oratory"));
    walk(&mut app, &[("to_oratory", "fg_hidden_oratory")]);
}

#[test]
fn the_shortcut_opens_from_the_bridge_and_leads_back_to_the_gates() {
    let mut app = objects_game(&storage("v18_shortcut"));
    grant(&mut app, &[Grant::Ability(Ability::AstralKey)]);
    enter(&mut app, BRIDGE, "to_astral_gate");
    let trapdoor = world().rooms[BRIDGE]
        .door("to_sealed_hall")
        .cloned()
        .unwrap_or_else(|| panic!("door"));
    assert_eq!(exploration(&app).door_state(&trapdoor), DoorState::Locked);
    // Closed, the trapdoor is floor: standing on it does not fall.
    place(&mut app, (640.0, 63.0));
    run(&mut app, 30);
    assert_eq!(current(&app), BRIDGE);
    place(&mut app, AT_TRAPDOOR_LEVER);
    run(&mut app, 3);
    tap(&mut app, KeyCode::KeyD);
    run(&mut app, 2);
    assert_eq!(exploration(&app).door_state(&trapdoor), DoorState::Open);
    place(&mut app, (640.0, 63.0));
    run(&mut app, 40);
    assert_eq!(current(&app), SEALED, "dropped back to the Vigil Gates");
    // One way: the sealed hall's ceiling cannot be used to go back up.
    let up = world().rooms[SEALED]
        .door("from_return_bridge")
        .cloned()
        .unwrap_or_else(|| panic!("door"));
    assert_eq!(exploration(&app).door_state(&up), DoorState::EntryOnly);
}

#[test]
fn a_room_change_fades_without_moving_the_player_or_blocking_input() {
    let mut app = objects_game(&storage("v18_fade"));
    assert_eq!(app.world().resource::<RoomFadeState>().remaining, 0.0);
    go_through(&mut app, "to_causeway");
    assert_eq!(current(&app), CAUSEWAY);
    let fade = *app.world().resource::<RoomFadeState>();
    assert!(fade.alpha() > 0.8, "{fade:?}");
    let arrived = player_xy(&mut app);
    // Input works during the fade: the player walks.
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::ArrowRight);
    run(&mut app, 4);
    assert!(player_xy(&mut app).x > arrived.x);
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .release(KeyCode::ArrowRight);
    let frames = (ROOM_FADE_TIME / FRAME.as_secs_f32()).ceil() as usize + 2;
    run(&mut app, frames);
    assert_eq!(app.world().resource::<RoomFadeState>().remaining, 0.0);
    // A load that fails changes no room and starts no fade.
    tap(&mut app, KeyCode::F9);
    run(&mut app, 2);
    assert_eq!(current(&app), CAUSEWAY);
    assert_eq!(app.world().resource::<RoomFadeState>().remaining, 0.0);
}

// ------------------------------------------------------------- persistence

/// Breaks the wall, opens two chests, pulls both levers, takes the relic.
fn do_everything(app: &mut App) {
    stand_at_wall(app);
    tap(app, KeyCode::KeyX);
    run(app, 12);
    enter(app, LONG_GALLERY, "to_threshold");
    place(app, AT_GALLERY_CHEST);
    run(app, 3);
    tap(app, KeyCode::KeyD);
    run(app, 2);
    enter(app, LEVER_HALL, "to_upper_gallery");
    place(app, ON_HALL_LEDGE);
    run(app, 3);
    tap(app, KeyCode::KeyD);
    run(app, 2);
    enter(app, "fg_relic_chapel", "to_stair");
    place(app, (640.0, 63.0));
    run(app, 5);
    enter(app, BRIDGE, "to_astral_gate");
    place(app, AT_BRIDGE_CHEST);
    run(app, 3);
    tap(app, KeyCode::KeyD);
    run(app, 2);
    place(app, AT_TRAPDOOR_LEVER);
    run(app, 3);
    tap(app, KeyCode::KeyD);
    run(app, 2);
}

fn assert_everything_kept(app: &mut App) {
    assert_eq!(object_state(app, STAIR, WALL), ObjectState::Broken);
    assert_eq!(
        object_state(app, LONG_GALLERY, GALLERY_CHEST),
        ObjectState::Opened
    );
    assert_eq!(object_state(app, BRIDGE, BRIDGE_CHEST), ObjectState::Opened);
    assert_eq!(
        object_state(app, LEVER_HALL, "fg_hall_lever"),
        ObjectState::On
    );
    assert_eq!(
        object_state(app, BRIDGE, "fg_trapdoor_lever"),
        ObjectState::On
    );
    let caps = exploration(app).capabilities();
    assert!(caps.abilities.contains(&Ability::AstralKey));
    assert_eq!(items(app, "warden_greaves"), 1);
    let w = world();
    let door = |room: &str, id: &str| {
        w.rooms[room]
            .door(id)
            .cloned()
            .unwrap_or_else(|| panic!("door {id}"))
    };
    let state = exploration(app);
    for (room, id) in [
        (STAIR, "to_cracked_cache"),
        (LEVER_HALL, "to_astral_gate"),
        ("fg_astral_gate", "to_return_bridge"),
        (BRIDGE, "to_sealed_hall"),
    ] {
        assert_eq!(state.door_state(&door(room, id)), DoorState::Open, "{id}");
    }
}

#[test]
fn every_object_survives_save_load_and_a_new_session() {
    let saves = storage("v18_persist");
    let mut app = objects_game(&saves);
    do_everything(&mut app);
    assert_everything_kept(&mut app);
    tap(&mut app, KeyCode::F5);
    run(&mut app, 2);
    // Another room, then the load.
    enter(&mut app, LONG_GALLERY, "to_threshold");
    tap(&mut app, KeyCode::F9);
    run(&mut app, 3);
    assert_eq!(current(&app), BRIDGE);
    assert_everything_kept(&mut app);
    // A brand new session (game restarted) loading the same slot.
    let mut again = objects_game(&saves);
    assert_eq!(object_state(&again, STAIR, WALL), ObjectState::Intact);
    tap(&mut again, KeyCode::F9);
    run(&mut again, 3);
    assert_eq!(current(&again), BRIDGE);
    assert_everything_kept(&mut again);
    // Back in the rooms: drawn and solid as saved.
    enter(&mut again, STAIR, "to_long_gallery");
    assert!(!wall_is_solid(&again));
    enter(&mut again, LONG_GALLERY, "to_stair");
    let world = again.world_mut();
    let chests = world
        .query::<(&ObjectMarker, &Sprite)>()
        .iter(world)
        .filter(|(m, _)| m.id == GALLERY_CHEST)
        .count();
    assert_eq!(chests, 1, "the opened chest is drawn once");
}

#[test]
fn loading_an_older_save_closes_the_chest_and_takes_the_item_back() {
    let saves = storage("v18_older_save");
    let mut app = objects_game(&saves);
    enter(&mut app, LONG_GALLERY, "to_threshold");
    let before = items(&mut app, "minor_tonic");
    tap(&mut app, KeyCode::F5);
    run(&mut app, 2);
    place(&mut app, AT_GALLERY_CHEST);
    run(&mut app, 3);
    tap(&mut app, KeyCode::KeyD);
    run(&mut app, 2);
    assert_eq!(items(&mut app, "minor_tonic"), before + 1);
    tap(&mut app, KeyCode::F9);
    run(&mut app, 3);
    assert_eq!(
        object_state(&app, LONG_GALLERY, GALLERY_CHEST),
        ObjectState::Closed
    );
    assert_eq!(
        items(&mut app, "minor_tonic"),
        before,
        "state and item agree"
    );
}

#[test]
fn opening_a_chest_and_saving_in_the_same_frame_saves_both_or_neither() {
    let saves = storage("v18_same_frame");
    let mut app = objects_game(&saves);
    enter(&mut app, LONG_GALLERY, "to_threshold");
    place(&mut app, AT_GALLERY_CHEST);
    run(&mut app, 3);
    let before = items(&mut app, "minor_tonic");
    app.world_mut()
        .resource_mut::<PendingPress>()
        .0
        .extend([KeyCode::KeyD, KeyCode::F5]);
    run(&mut app, 2);
    let saved = saves
        .load(DEFAULT_SAVE_SLOT)
        .unwrap_or_else(|e| panic!("{e}"));
    let opened = saved.exploration.pickups.contains(GALLERY_CHEST);
    let count = saved.player.inventory.count("minor_tonic");
    assert_eq!(count, before + u32::from(opened), "opened={opened}");
}

#[test]
fn unknown_object_ids_in_a_save_are_kept_and_ignored() {
    let saves = storage("v18_unknown_ids");
    let mut app = objects_game(&saves);
    {
        let mut e = app.world_mut().resource_mut::<Exploration>();
        e.secrets.insert("removed_wall".into());
        e.pickups.insert("removed_chest".into());
    }
    tap(&mut app, KeyCode::F5);
    run(&mut app, 2);
    tap(&mut app, KeyCode::F9);
    run(&mut app, 3);
    let state = exploration(&app);
    let unknown = state.unknown_ids(&world());
    assert!(unknown.contains(&"removed_wall".to_owned()));
    assert!(unknown.contains(&"removed_chest".to_owned()));
    let stats = state.stats(&world());
    assert_eq!(stats.secrets_found, 0, "unknown ids do not count");
    assert_eq!(object_state(&app, STAIR, WALL), ObjectState::Intact);
}

// ------------------------------------------------------------------ stress

#[test]
fn fifty_returns_to_object_rooms_leak_nothing_and_grant_nothing_again() {
    let mut app = objects_game(&storage("v18_stress_returns"));
    do_everything(&mut app);
    enter(&mut app, LONG_GALLERY, "to_threshold");
    let room_entities = count::<With<RoomEntity>>(&mut app);
    let greaves = items(&mut app, "warden_greaves");
    let tonics = items(&mut app, "minor_tonic");
    let granted = exploration(&app).granted.len();
    for _ in 0..50 {
        enter(&mut app, STAIR, "to_long_gallery");
        assert!(!wall_is_solid(&app));
        enter(&mut app, LONG_GALLERY, "to_stair");
        place(&mut app, AT_GALLERY_CHEST);
        run(&mut app, 2);
        tap(&mut app, KeyCode::KeyD);
        run(&mut app, 1);
    }
    enter(&mut app, LONG_GALLERY, "to_threshold");
    run(&mut app, 2);
    assert_eq!(count::<With<RoomEntity>>(&mut app), room_entities);
    assert_eq!(count::<With<Hitbox>>(&mut app), 0, "no stale hitboxes");
    assert_eq!(items(&mut app, "warden_greaves"), greaves);
    assert_eq!(items(&mut app, "minor_tonic"), tonics);
    assert_eq!(exploration(&app).granted.len(), granted);
}

#[test]
fn a_hundred_interactions_and_fifty_lever_pulls_change_nothing_twice() {
    let mut app = objects_game(&storage("v18_stress_interact"));
    enter(&mut app, LEVER_HALL, "to_upper_gallery");
    place(&mut app, ON_HALL_LEDGE);
    run(&mut app, 3);
    for _ in 0..50 {
        tap(&mut app, KeyCode::KeyD);
    }
    enter(&mut app, LONG_GALLERY, "to_threshold");
    place(&mut app, AT_GALLERY_CHEST);
    run(&mut app, 3);
    let before = items(&mut app, "minor_tonic");
    for _ in 0..100 {
        tap(&mut app, KeyCode::KeyD);
    }
    run(&mut app, 2);
    assert_eq!(items(&mut app, "minor_tonic"), before + 1);
    assert_eq!(exploration(&app).granted.len(), 2, "one flag, one item");
}

#[test]
fn fifty_save_load_cycles_with_objects_stay_consistent() {
    let saves = storage("v18_stress_saves");
    let mut app = objects_game(&saves);
    do_everything(&mut app);
    tap(&mut app, KeyCode::F5);
    run(&mut app, 2);
    let entities = count::<With<RoomEntity>>(&mut app);
    for i in 0..50 {
        let (room, door) = if i % 2 == 0 {
            (STAIR, "to_long_gallery")
        } else {
            (LEVER_HALL, "to_upper_gallery")
        };
        enter(&mut app, room, door);
        tap(&mut app, KeyCode::F9);
        run(&mut app, 2);
        assert_eq!(current(&app), BRIDGE);
    }
    assert_eq!(count::<With<RoomEntity>>(&mut app), entities);
    assert_eq!(count::<With<Player>>(&mut app), 1);
    assert_everything_kept(&mut app);
}

#[test]
fn the_v2_fixture_loads_into_the_galleries_with_its_objects() {
    let saves = storage("v18_v2_fixture");
    std::fs::create_dir_all(saves.dir()).unwrap_or_else(|e| panic!("{e}"));
    let fixture = root().join("tests/fixtures/save_v2_galleries.ron");
    let text = std::fs::read_to_string(&fixture).unwrap_or_else(|e| panic!("{e}"));
    std::fs::write(saves.slot_path(DEFAULT_SAVE_SLOT), &text).unwrap_or_else(|e| panic!("{e}"));
    let mut app = objects_game(&saves);
    tap(&mut app, KeyCode::F9);
    run(&mut app, 3);
    assert_eq!(current(&app), BRIDGE);
    assert_eq!(object_state(&app, STAIR, WALL), ObjectState::Broken);
    assert_eq!(
        object_state(&app, BRIDGE, BRIDGE_CHEST),
        ObjectState::Opened
    );
    assert_eq!(
        object_state(&app, BRIDGE, "fg_trapdoor_lever"),
        ObjectState::On
    );
    assert_eq!(
        object_state(&app, LONG_GALLERY, GALLERY_CHEST),
        ObjectState::Closed,
        "not opened in that save"
    );
    assert_eq!(items(&mut app, "warden_greaves"), 1);
    // Loading never wrote the file.
    assert_eq!(
        std::fs::read_to_string(saves.slot_path(DEFAULT_SAVE_SLOT)).unwrap_or_default(),
        text
    );
}

// ------------------------------------------------------------- performance

/// Measures (written to `<target>/tmp/world_v18_perf.txt`, compared with
/// the V1.7 numbers in docs/WORLD_EXPLORATION_V1_8.md §19): entities per
/// room, room change, save, load, a map redraw and one interaction.
#[test]
fn performance_of_rooms_objects_and_saves() {
    use std::fmt::Write as _;
    use std::time::Instant;
    let saves = storage("v18_perf");
    let mut app = objects_game(&saves);
    let w = world();
    let mut out = String::new();
    let objects: usize = w.rooms.values().map(|r| r.objects.len()).sum();
    let _ = writeln!(
        out,
        "rooms {}; interactive objects {objects}; secrets {}; one-time rewards {}",
        w.rooms.len(),
        w.secret_ids().len(),
        w.reward_ids().len()
    );
    let mut worst = (0usize, String::new());
    let mut change_total = 0.0;
    for room in w.rooms.values() {
        let Some(door) = room.doors.first() else {
            continue;
        };
        let t = Instant::now();
        enter(&mut app, &room.id, &door.id);
        change_total += t.elapsed().as_secs_f64();
        let n = count::<With<RoomEntity>>(&mut app);
        if n > worst.0 {
            worst = (n, room.id.clone());
        }
        assert!(n < 64, "{}: {n} room entities", room.id);
    }
    let _ = writeln!(
        out,
        "room change (request + 2 frames): {:.3} ms average over {} rooms; most entities: {} in {}",
        1000.0 * change_total / w.rooms.len() as f64,
        w.rooms.len(),
        worst.0,
        worst.1
    );
    let all = count::<()>(&mut app);
    let _ = writeln!(out, "entities alive (whole app): {all}");
    enter(&mut app, LONG_GALLERY, "to_threshold");
    place(&mut app, AT_GALLERY_CHEST);
    run(&mut app, 3);
    let t = Instant::now();
    tap(&mut app, KeyCode::KeyD);
    let interact = t.elapsed();
    let t = Instant::now();
    run(&mut app, 1);
    let idle = t.elapsed();
    let t = Instant::now();
    tap(&mut app, KeyCode::F5);
    let save = t.elapsed();
    let t = Instant::now();
    tap(&mut app, KeyCode::F9);
    let load = t.elapsed();
    let _ = writeln!(
        out,
        "frame with an interaction (+ map redraw): {:.3} ms; plain frame: {:.3} ms; F5 frame: {:.3} ms; F9 frame: {:.3} ms",
        interact.as_secs_f64() * 1000.0,
        idle.as_secs_f64() * 1000.0,
        save.as_secs_f64() * 1000.0,
        load.as_secs_f64() * 1000.0
    );
    let path = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("world_v18_perf.txt");
    let _ = std::fs::write(&path, &out);
    print!("{out}");
}
