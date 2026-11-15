//! World graph (V1.6): loading, validation and exploration rules, without Bevy.
//!
//! Uses the real vertical slice (`assets/world/`) and small variations of it
//! built in memory to provoke each error the validator must report.

use std::path::Path;

use dark_castle_rpg::core::resources::GameData;
use dark_castle_rpg::world::exploration::{Discovery, DoorState, ExplorationState};
use dark_castle_rpg::world::graph::{
    Ability, Door, DoorTarget, Grant, MapRect, RegionFile, Requirement, RoomRole, Side,
    WorldDefinition, WorldError, WorldRoom,
};
use dark_castle_rpg::world::validation::{Access, Issue, validate};

const APPROACH: &str = "vigil_gates_approach";
const CACHE: &str = "vigil_gates_hidden_cache";

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn slice() -> WorldDefinition {
    WorldDefinition::load_dir(&root().join("assets/world"))
        .unwrap_or_else(|e| panic!("world failed to load: {e}"))
}

fn room_mut<'a>(world: &'a mut WorldDefinition, id: &str) -> &'a mut WorldRoom {
    world
        .rooms
        .get_mut(id)
        .unwrap_or_else(|| panic!("no room {id}"))
}

fn door_mut<'a>(world: &'a mut WorldDefinition, room: &str, door: &str) -> &'a mut Door {
    room_mut(world, room)
        .doors
        .iter_mut()
        .find(|d| d.id == door)
        .unwrap_or_else(|| panic!("no door {room}.{door}"))
}

fn has(errors: &[Issue], wanted: impl Fn(&Issue) -> bool) -> bool {
    errors.iter().any(wanted)
}

#[test]
fn the_vertical_slice_is_valid_and_classified() {
    let world = slice();
    // V1.8: the slice is the Vigil Gates region; the world also has the
    // Forgotten Galleries (8 to 12 rooms, docs/WORLD_EXPLORATION_V1_8.md).
    let in_region = |region: &str| world.rooms.values().filter(|r| r.region == region).count();
    assert!(
        (6..=10).contains(&in_region("vigil_gates")),
        "{} rooms",
        in_region("vigil_gates")
    );
    assert!((8..=12).contains(&in_region("forgotten_galleries")));
    // V1.9: plus the two rooms of the Veiled Cloister.
    assert_eq!(in_region("veiled_cloister"), 2);
    // V2.1: plus the six rooms of the Village of Ashes.
    assert_eq!(in_region("ash_village"), 6);
    assert_eq!(
        world.rooms.len(),
        in_region("vigil_gates")
            + in_region("forgotten_galleries")
            + in_region("veiled_cloister")
            + in_region("ash_village")
    );
    let report = validate(&world);
    assert!(report.is_ok(), "errors: {:#?}", report.errors);
    assert!(
        report.warnings.is_empty(),
        "warnings: {:#?}",
        report.warnings
    );
    assert_eq!(
        report.access[CACHE],
        Access::Secret,
        "behind the false wall"
    );
    // V1.8: the slice's rooms (the galleries' secret rooms are checked in
    // `v1_8::the_galleries_are_valid_connected_and_classified`).
    for (id, access) in &report.access {
        if id != CACHE && world.rooms[id].region == "vigil_gates" {
            assert_eq!(*access, Access::Main, "{id}");
        }
    }
    assert!(
        report
            .final_capabilities
            .abilities
            .contains(&Ability::VeilStep)
    );
    assert!(
        report
            .final_capabilities
            .flags
            .contains("vigil_gates_cleared")
    );
}

#[test]
fn the_slice_uses_existing_enemies_and_safe_entry_points() {
    let data = GameData::load(root()).unwrap_or_else(|e| panic!("{e}"));
    let world = slice();
    for room in world.rooms.values() {
        for spawn in &room.enemies {
            assert!(
                data.enemies.contains_key(&spawn.enemy),
                "{}: {}",
                room.id,
                spawn.enemy
            );
        }
        let solids = room.to_room_definition().solids();
        let (w, h) = room.size();
        for door in &room.doors {
            let (x, y) = room.entry_point(door);
            assert!(
                x > 0.0 && x < w && y > 0.0 && y < h,
                "{}.{} outside",
                room.id,
                door.id
            );
            let body = dark_castle_rpg::world::collision::Aabb::new((x, y), (26.0, 46.0));
            assert!(
                !solids
                    .iter()
                    .any(|s| !s.one_way && s.aabb.intersects(&body)),
                "{}.{}: entry inside a solid",
                room.id,
                door.id
            );
            assert!(
                !room.crosses(door, (x, y)),
                "{}.{}: entry on the trigger",
                room.id,
                door.id
            );
        }
    }
}

#[test]
fn duplicate_ids_are_rejected_when_loading() {
    let world = slice();
    let region = world
        .regions
        .values()
        .next()
        .cloned()
        .unwrap_or_else(|| panic!("no region"));
    let room = world.rooms[APPROACH].clone();
    let twice = vec![RegionFile {
        region: region.clone(),
        rooms: vec![room.clone(), room.clone()],
    }];
    assert!(matches!(
        WorldDefinition::from_parts(world.manifest.clone(), twice),
        Err(WorldError::DuplicateRoom(id)) if id == APPROACH
    ));
    let two_regions = vec![
        RegionFile {
            region: region.clone(),
            rooms: vec![],
        },
        RegionFile {
            region,
            rooms: vec![],
        },
    ];
    assert!(matches!(
        WorldDefinition::from_parts(world.manifest.clone(), two_regions),
        Err(WorldError::DuplicateRegion(_))
    ));
    let mut doubled = room;
    let copy = doubled.doors[0].clone();
    doubled.doors.push(copy);
    let file = vec![RegionFile {
        region: world
            .regions
            .values()
            .next()
            .cloned()
            .unwrap_or_else(|| panic!("no region")),
        rooms: vec![doubled],
    }];
    assert!(matches!(
        WorldDefinition::from_parts(world.manifest, file),
        Err(WorldError::DuplicateDoor { .. })
    ));
}

#[test]
fn doors_to_nowhere_are_errors() {
    let mut world = slice();
    door_mut(&mut world, APPROACH, "to_causeway").to.room = "nowhere".into();
    let report = validate(&world);
    assert!(has(
        &report.errors,
        |i| matches!(i, Issue::MissingTargetRoom { door, .. } if door == "to_causeway")
    ));

    let mut world = slice();
    door_mut(&mut world, APPROACH, "to_causeway").to.door = "no_such_door".into();
    assert!(has(&validate(&world).errors, |i| matches!(
        i,
        Issue::MissingTargetDoor { .. }
    )));

    let mut world = slice();
    door_mut(&mut world, "vigil_gates_causeway", "from_approach")
        .to
        .door = "to_sealed".into();
    assert!(has(&validate(&world).errors, |i| matches!(
        i,
        Issue::NotReciprocal { .. }
    )));
}

#[test]
fn doors_must_touch_on_the_map() {
    let mut world = slice();
    room_mut(&mut world, "vigil_gates_barbican").map.y = 1;
    let errors = validate(&world).errors;
    assert!(has(
        &errors,
        |i| matches!(i, Issue::Misaligned { room, .. } if room == "vigil_gates_barbican")
    ));
    let mut world = slice();
    door_mut(&mut world, "vigil_gates_shaft", "to_gallery").cell = 5;
    assert!(has(&validate(&world).errors, |i| matches!(
        i,
        Issue::DoorCellOutOfRange { .. }
    )));
}

fn lone_room(id: &str, x: i32) -> WorldRoom {
    WorldRoom {
        id: id.into(),
        name: id.into(),
        region: "vigil_gates".into(),
        role: RoomRole::Optional,
        map: MapRect {
            x,
            y: 10,
            w: 1,
            h: 1,
        },
        background: (0.0, 0.0, 0.0),
        platforms: vec![],
        enemies: vec![],
        doors: vec![],
        pickups: vec![],
        secrets: vec![],
        objects: vec![],
        veils: vec![],
    }
}

#[test]
fn disconnected_rooms_and_regions_are_errors() {
    let mut world = slice();
    world.rooms.insert("island".into(), lone_room("island", 40));
    let report = validate(&world);
    assert!(has(
        &report.errors,
        |i| matches!(i, Issue::NoDoors { room } if room == "island")
    ));
    assert!(has(
        &report.errors,
        |i| matches!(i, Issue::Unreachable { room } if room == "island")
    ));
    assert_eq!(report.access["island"], Access::Unreachable);

    // Two rooms linked to each other only: a region nobody can enter.
    let mut world = slice();
    let mut a = lone_room("far_a", 50);
    let mut b = lone_room("far_b", 51);
    a.region = "far_region".into();
    b.region = "far_region".into();
    a.doors.push(Door {
        id: "e".into(),
        side: Side::Right,
        cell: 0,
        to: DoorTarget {
            room: "far_b".into(),
            door: "w".into(),
        },
        requires: None,
        hidden_by: None,
        entry_only: false,
        shortcut: false,
    });
    b.doors.push(Door {
        id: "w".into(),
        side: Side::Left,
        cell: 0,
        to: DoorTarget {
            room: "far_a".into(),
            door: "e".into(),
        },
        requires: None,
        hidden_by: None,
        entry_only: false,
        shortcut: false,
    });
    world.rooms.insert("far_a".into(), a);
    world.rooms.insert("far_b".into(), b);
    let mut region = world.regions["vigil_gates"].clone();
    region.id = "far_region".into();
    world.regions.insert("far_region".into(), region);
    let errors = validate(&world).errors;
    assert!(has(
        &errors,
        |i| matches!(i, Issue::RegionUnreachable { region } if region == "far_region")
    ));
    assert!(
        !has(&errors, |i| matches!(i, Issue::NoDoors { .. })),
        "they do have doors"
    );
}

#[test]
fn cycles_are_valid() {
    // The slice has a loop: approach → causeway → shaft → gallery → postern → approach.
    let world = slice();
    let report = validate(&world);
    assert!(report.is_ok());
    let postern = &world.rooms["vigil_gates_postern"];
    let drop = postern
        .door("to_approach")
        .unwrap_or_else(|| panic!("no drop"));
    assert_eq!(drop.to.room, APPROACH, "the loop closes on the start room");
}

#[test]
fn requirements_nobody_grants_are_impossible() {
    let mut world = slice();
    door_mut(&mut world, APPROACH, "to_sealed").requires =
        Some(Requirement::Ability(Ability::DoubleJump));
    door_mut(&mut world, "vigil_gates_sealed_hall", "from_approach").requires = None;
    let errors = validate(&world).errors;
    assert!(has(&errors, |i| matches!(
        i,
        Issue::ImpossibleRequirement {
            condition: dark_castle_rpg::world::graph::Condition::Ability(Ability::DoubleJump),
            ..
        }
    )));
    // The rooms behind it, and the goal, become unreachable.
    assert!(has(&errors, |i| matches!(i, Issue::GoalUnreachable { .. })));
    assert!(has(
        &errors,
        |i| matches!(i, Issue::Unreachable { room } if room == "vigil_gates_reliquary")
    ));
}

#[test]
fn alternative_requirements_need_one_obtainable_branch() {
    let mut world = slice();
    let alt = Requirement::Any(vec![
        Requirement::Ability(Ability::DoubleJump),
        Requirement::Ability(Ability::VeilStep),
    ]);
    door_mut(&mut world, APPROACH, "to_sealed").requires = Some(alt.clone());
    door_mut(&mut world, "vigil_gates_sealed_hall", "from_approach").requires = Some(alt);
    let report = validate(&world);
    assert!(report.is_ok(), "{:#?}", report.errors);
    // Both together (All) is impossible: DoubleJump is never granted.
    let mut world = slice();
    door_mut(&mut world, APPROACH, "to_sealed").requires = Some(Requirement::All(vec![
        Requirement::Ability(Ability::DoubleJump),
        Requirement::Ability(Ability::VeilStep),
    ]));
    assert!(!validate(&world).is_ok());
}

#[test]
fn secret_rooms_are_optional_content_not_errors() {
    let world = slice();
    let report = validate(&world);
    assert_eq!(report.access[CACHE], Access::Secret);
    assert!(!has(
        &report.errors,
        |i| matches!(i, Issue::Unreachable { room } if room == CACHE)
    ));
    // A secret room reachable without its secret is reported (warning).
    let mut world = slice();
    door_mut(&mut world, "vigil_gates_barbican", "to_cache").hidden_by = None;
    let report = validate(&world);
    assert!(report.is_ok());
    assert!(has(&report.warnings, |i| matches!(
        i,
        Issue::SecretRoomNotHidden { .. }
    )));
    // A required room behind a secret is an error: the main path never needs one.
    let mut world = slice();
    room_mut(&mut world, CACHE).role = RoomRole::Required;
    assert!(has(&validate(&world).errors, |i| matches!(
        i,
        Issue::RequiredBehindSecret { .. }
    )));
    // A door hidden by a secret the room does not define.
    let mut world = slice();
    door_mut(&mut world, "vigil_gates_barbican", "to_cache").hidden_by = Some("nope".into());
    assert!(has(&validate(&world).errors, |i| matches!(
        i,
        Issue::UnknownSecret { .. }
    )));
}

#[test]
fn the_main_progression_needs_the_relic() {
    let mut world = slice();
    room_mut(&mut world, "vigil_gates_gallery").pickups.clear();
    let report = validate(&world);
    assert!(has(&report.errors, |i| matches!(
        i,
        Issue::GoalUnreachable { .. }
    )));
    assert!(has(&report.errors, |i| matches!(
        i,
        Issue::ImpossibleRequirement { .. }
    )));
}

#[test]
fn a_room_without_exit_is_reported() {
    let mut world = slice();
    for door in &mut room_mut(&mut world, "vigil_gates_barbican").doors {
        door.entry_only = true;
    }
    // Its neighbours' doors stay usable, so it is reachable but a dead end.
    let report = validate(&world);
    assert!(has(
        &report.warnings,
        |i| matches!(i, Issue::NoExit { room } if room == "vigil_gates_barbican")
    ));
}

// --- exploration state (pure) -------------------------------------------------------------

#[test]
fn entering_rooms_discovers_neighbours_but_not_hidden_ones() {
    let world = slice();
    let mut state = ExplorationState::default();
    state.enter_room(&world, APPROACH);
    assert_eq!(state.discovery(APPROACH), Discovery::Visited);
    assert_eq!(
        state.discovery("vigil_gates_causeway"),
        Discovery::Discovered
    );
    assert_eq!(
        state.discovery("vigil_gates_sealed_hall"),
        Discovery::Discovered,
        "locked, but seen"
    );
    state.enter_room(&world, "vigil_gates_causeway");
    state.enter_room(&world, "vigil_gates_shaft");
    state.enter_room(&world, "vigil_gates_barbican");
    assert_eq!(
        state.discovery(CACHE),
        Discovery::Unknown,
        "hidden behind the false wall"
    );
    let barbican = &world.rooms["vigil_gates_barbican"];
    let hidden = barbican.door("to_cache").unwrap_or_else(|| panic!("door"));
    assert_eq!(state.door_state(hidden), DoorState::Hidden);
    assert!(state.find_secret(&world, &barbican.id, "barbican_false_wall"));
    assert!(
        !state.find_secret(&world, &barbican.id, "barbican_false_wall"),
        "only once"
    );
    assert_eq!(state.door_state(hidden), DoorState::Open);
    assert_eq!(state.discovery(CACHE), Discovery::Discovered);
    assert_eq!(state.previous_room.as_deref(), Some("vigil_gates_shaft"));
}

#[test]
fn barriers_follow_the_capabilities() {
    let world = slice();
    let mut state = ExplorationState::default();
    let door = world.rooms[APPROACH]
        .door("to_sealed")
        .unwrap_or_else(|| panic!("door"));
    assert_eq!(state.door_state(door), DoorState::Locked);
    let gallery = &world.rooms["vigil_gates_gallery"];
    assert_eq!(
        state.collect(gallery, "veil_relic"),
        Some(Grant::Ability(Ability::VeilStep))
    );
    assert_eq!(
        state.collect(gallery, "veil_relic"),
        None,
        "a pickup is taken once"
    );
    assert_eq!(state.door_state(door), DoorState::Open);
    let top = world.rooms[APPROACH]
        .door("from_postern")
        .unwrap_or_else(|| panic!("door"));
    assert_eq!(state.door_state(top), DoorState::EntryOnly);
}

#[test]
fn exploration_percentage_counts_every_room_and_secret() {
    let world = slice();
    let mut state = ExplorationState::default();
    // V1.8: the numbers of the Vigil Gates region (this slice); the global
    // ones keep the same formula over every region (checked below).
    let empty = state.region_stats(&world, "vigil_gates");
    assert_eq!((empty.visited, empty.total, empty.percent), (0, 9, 0.0));
    let global = state.stats(&world);
    // V1.9: 22 rooms (the Veiled Cloister adds two, no secret).
    // V2.1: 28 rooms (the Village of Ashes adds six, no secret).
    assert_eq!((global.total, global.secrets_total), (28, 3));
    state.enter_room(&world, APPROACH);
    assert!((state.stats(&world).percent - 100.0 / 28.0).abs() < 1e-4);
    let one = state.region_stats(&world, "vigil_gates");
    assert_eq!(one.visited, 1);
    assert!((one.percent - 100.0 / 9.0).abs() < 1e-4);
    assert_eq!(one.known, 3, "approach + causeway + sealed hall");
    assert_eq!((one.secrets_found, one.secrets_total), (0, 1));
    for id in world.rooms.keys() {
        state.rooms.insert(id.clone(), Discovery::Visited);
    }
    assert_eq!(state.stats(&world).percent, 100.0);
    // Ids of rooms that no longer exist do not count (content changed).
    state
        .rooms
        .insert("removed_room".into(), Discovery::Visited);
    assert_eq!(state.stats(&world).percent, 100.0);
}

#[test]
fn exploration_state_round_trips_and_checks_its_version() {
    let world = slice();
    let mut state = ExplorationState::default();
    state.enter_room(&world, APPROACH);
    state.find_secret(&world, "vigil_gates_barbican", "barbican_false_wall");
    state.collect(&world.rooms["vigil_gates_gallery"], "veil_relic");
    let text = state.to_ron().unwrap_or_else(|e| panic!("{e}"));
    let back = ExplorationState::from_ron(&text).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(back, state);
    let future = text.replace("version: 1", "version: 99");
    assert!(ExplorationState::from_ron(&future).is_err());
    assert!(ExplorationState::from_ron("not ron").is_err());
}

// ------------------------------------------------------------- V1.7

#[test]
fn legacy_rooms_resume_at_existing_doors() {
    let world = slice();
    let target = world
        .manifest
        .legacy_rooms
        .get("entry_hall")
        .unwrap_or_else(|| panic!("entry_hall is mapped"));
    assert!(
        world
            .room(&target.room)
            .and_then(|r| r.door(&target.door))
            .is_some()
    );
    let mut broken = slice();
    if let Some(t) = broken.manifest.legacy_rooms.get_mut("entry_hall") {
        t.door = "no_such_door".into();
    }
    let report = validate(&broken);
    assert!(has(&report.errors, |i| matches!(
        i,
        Issue::BadLegacyTarget { legacy, .. } if legacy == "entry_hall"
    )));
}

#[test]
fn the_start_is_valid_and_the_goal_needs_the_relic() {
    use dark_castle_rpg::world::validation::reachable;
    let world = slice();
    let start = world
        .room(&world.manifest.start_room)
        .unwrap_or_else(|| panic!("start room"));
    let door = start
        .door(&world.manifest.start_door)
        .unwrap_or_else(|| panic!("start door"));
    assert!(!start.crosses(door, start.entry_point(door)));
    // Without the relic's pickup the goal is not reachable: the grate is a
    // real progression barrier, not decoration.
    let mut no_relic = slice();
    room_mut(&mut no_relic, "vigil_gates_gallery")
        .pickups
        .clear();
    let (main, _) = reachable(&no_relic, true);
    assert!(!main.contains(&world.manifest.goal_room));
    let (main, caps) = reachable(&world, false);
    assert!(main.contains(&world.manifest.goal_room));
    assert!(caps.abilities.contains(&Ability::VeilStep));
}

// ------------------------------------------------------------- V1.8

mod v1_8 {
    use super::*;
    use dark_castle_rpg::combat::damage::DamageKind;
    use dark_castle_rpg::magic::elements::Element;
    use dark_castle_rpg::world::exploration::{InteractError, Interaction, ObjectState};
    use dark_castle_rpg::world::graph::{
        BreakCondition, Capabilities, Grant, ObjectKind, Requirement, WorldObject,
    };
    use dark_castle_rpg::world::validation::{Rules, reachable_from};

    const START: &str = "vigil_gates_approach";

    fn caps(abilities: &[Ability], flags: &[&str]) -> Capabilities {
        Capabilities {
            abilities: abilities.iter().copied().collect(),
            items: Default::default(),
            flags: flags.iter().map(|f| (*f).to_owned()).collect(),
        }
    }

    /// What the player has after the Vigil Gates and the lever hall.
    fn before_the_astral_seal() -> Capabilities {
        caps(
            &[Ability::VeilStep],
            &["vigil_gates_cleared", "fg_hall_gate_open"],
        )
    }

    fn object_mut<'a>(world: &'a mut WorldDefinition, room: &str, id: &str) -> &'a mut WorldObject {
        room_mut(world, room)
            .objects
            .iter_mut()
            .find(|o| o.id == id)
            .unwrap_or_else(|| panic!("no object {id}"))
    }

    #[test]
    fn the_galleries_are_valid_connected_and_classified() {
        let world = slice();
        let report = validate(&world);
        assert!(report.is_ok(), "{:#?}", report.errors);
        assert!(report.warnings.is_empty(), "{:#?}", report.warnings);
        for (room, access) in [
            ("fg_threshold", Access::Main),
            ("fg_return_bridge", Access::Main),
            ("fg_collapsed_walk", Access::Main),
            ("fg_cracked_cache", Access::Secret),
            ("fg_hidden_oratory", Access::Secret),
        ] {
            assert_eq!(report.access[room], access, "{room}");
        }
        assert_eq!(world.manifest.goal_room, "fg_return_bridge");
        let fc = &report.final_capabilities;
        assert!(fc.abilities.contains(&Ability::AstralKey));
        assert!(fc.flags.contains("fg_trapdoor_open"));
        assert!(fc.items.contains("warden_greaves"));
    }

    #[test]
    fn progression_scenarios_relic_secret_lever_and_shortcut() {
        let world = slice();
        // Without the relic the astral seal stops the main path; with it the
        // goal is reached.
        let no_relic = reachable_from(&world, START, &before_the_astral_seal(), Rules::MAIN);
        assert!(no_relic.contains("fg_astral_gate"));
        assert!(!no_relic.contains("fg_return_bridge"));
        let mut with_relic = before_the_astral_seal();
        with_relic.abilities.insert(Ability::AstralKey);
        assert!(
            reachable_from(&world, START, &with_relic, Rules::MAIN).contains("fg_return_bridge")
        );
        // Secret not found / found.
        let main = reachable_from(&world, START, &with_relic, Rules::MAIN);
        let all = reachable_from(&world, START, &with_relic, Rules::ALL);
        for secret_room in ["fg_cracked_cache", "fg_hidden_oratory"] {
            assert!(
                !main.contains(secret_room) && all.contains(secret_room),
                "{secret_room}"
            );
        }
        // Mechanism off / on.
        let lever_off = caps(
            &[Ability::VeilStep, Ability::AstralKey],
            &["vigil_gates_cleared"],
        );
        let off = reachable_from(&world, START, &lever_off, Rules::ALL);
        assert!(off.contains("fg_lever_hall") && !off.contains("fg_astral_gate"));
        assert!(
            off.contains("fg_relic_chapel"),
            "the relic does not need the lever"
        );
        // Shortcut closed / open, from the bridge with nothing but its flag.
        let closed = reachable_from(&world, "fg_return_bridge", &caps(&[], &[]), Rules::ALL);
        assert_eq!(closed.len(), 1);
        let open = reachable_from(
            &world,
            "fg_return_bridge",
            &caps(&[], &["fg_trapdoor_open"]),
            Rules::ALL,
        );
        assert!(open.contains("vigil_gates_sealed_hall"));
        // Without shortcuts the main progression reaches exactly the same.
        let no_shortcuts = Rules {
            secrets: false,
            shortcuts: false,
        };
        assert_eq!(
            reachable_from(&world, START, &with_relic, no_shortcuts),
            reachable_from(&world, START, &with_relic, Rules::MAIN)
        );
    }

    #[test]
    fn the_validator_catches_broken_objects_and_progression() {
        // Duplicate id.
        let mut w = slice();
        object_mut(&mut w, "fg_cracked_cache", "fg_cache_chest").id = "fg_gallery_chest".into();
        assert!(has(
            &validate(&w).errors,
            |i| matches!(i, Issue::DuplicateId { id } if id == "fg_gallery_chest")
        ));
        // Object outside its room.
        let mut w = slice();
        object_mut(&mut w, "fg_long_gallery", "fg_gallery_chest")
            .area
            .min
            .0 = 9000.0;
        assert!(has(&validate(&w).errors, |i| matches!(
            i,
            Issue::ObjectOutOfRoom { .. }
        )));
        // A chest sealed by something nobody grants.
        let mut w = slice();
        object_mut(&mut w, "fg_long_gallery", "fg_gallery_chest").requires =
            Some(Requirement::Ability(Ability::WraithForm));
        assert!(has(&validate(&w).errors, |i| matches!(
            i,
            Issue::ImpossibleObjectRequirement { .. }
        )));
        // A lever whose flag nothing uses.
        let mut w = slice();
        object_mut(&mut w, "fg_return_bridge", "fg_trapdoor_lever").kind = ObjectKind::Lever {
            sets: "nothing_uses_this".into(),
        };
        let report = validate(&w);
        assert!(has(&report.warnings, |i| matches!(
            i,
            Issue::UnusedLever { .. }
        )));
        assert!(has(&report.errors, |i| matches!(
            i,
            Issue::ImpossibleRequirement { .. }
        )));
        // A "shortcut" that is in fact the only way on.
        let mut w = slice();
        door_mut(&mut w, "vigil_gates_approach", "to_sealed").shortcut = true;
        door_mut(&mut w, "vigil_gates_sealed_hall", "from_approach").shortcut = true;
        assert!(has(&validate(&w).errors, |i| matches!(
            i,
            Issue::ShortcutBypassesProgression { .. }
        )));
        // A room with no way back.
        let mut w = slice();
        door_mut(&mut w, "fg_hidden_oratory", "to_collapsed_walk").entry_only = true;
        assert!(has(&validate(&w).errors, |i| matches!(
            i,
            Issue::NoWayBack { room } if room == "fg_hidden_oratory"
        )));
        // A door hidden by a secret that does not exist.
        let mut w = slice();
        door_mut(&mut w, "fg_stair", "to_cracked_cache").hidden_by = Some("no_such_wall".into());
        assert!(has(&validate(&w).errors, |i| matches!(
            i,
            Issue::UnknownSecret { .. }
        )));
    }

    #[test]
    fn break_conditions_and_object_rules() {
        use DamageKind::{Magical, Physical};
        assert!(BreakCondition::AnyAttack.accepts(Magical, Element::Fire));
        assert!(BreakCondition::Kind(Physical).accepts(Physical, Element::Physical));
        assert!(!BreakCondition::Kind(Physical).accepts(Magical, Element::Fire));
        assert!(BreakCondition::Element(Element::Fire).accepts(Magical, Element::Fire));
        assert!(!BreakCondition::Element(Element::Fire).accepts(Physical, Element::Physical));

        let mut world = slice();
        object_mut(&mut world, "fg_stair", "fg_cracked_wall").kind = ObjectKind::BreakableWall {
            breaks_on: BreakCondition::Kind(Physical),
        };
        let stair = world.rooms["fg_stair"].clone();
        let mut state = ExplorationState::default();
        assert_eq!(
            state.break_wall(&world, &stair, "fg_cracked_wall", Magical, Element::Fire),
            Err(InteractError::WrongAttack)
        );
        assert_eq!(
            state.break_wall(
                &world,
                &stair,
                "fg_cracked_wall",
                Physical,
                Element::Physical
            ),
            Ok(Interaction::SecretFound("fg_cracked_wall".into()))
        );
        assert_eq!(
            state.break_wall(
                &world,
                &stair,
                "fg_cracked_wall",
                Physical,
                Element::Physical
            ),
            Err(InteractError::AlreadyDone)
        );
        assert_eq!(
            state.interact(&world, &stair, "fg_cracked_wall"),
            Err(InteractError::AlreadyDone),
            "a wall is never used with the interact key"
        );

        // A sealed chest opens only with its requirement, once.
        object_mut(&mut world, "fg_long_gallery", "fg_gallery_chest").requires =
            Some(Requirement::Ability(Ability::AstralKey));
        let gallery = world.rooms["fg_long_gallery"].clone();
        assert_eq!(
            state.interact(&world, &gallery, "fg_gallery_chest"),
            Err(InteractError::Sealed)
        );
        assert!(state.pickups.is_empty(), "nothing recorded when sealed");
        state.granted.push(Grant::Ability(Ability::AstralKey));
        assert_eq!(
            state.interact(&world, &gallery, "fg_gallery_chest"),
            Ok(Interaction::Granted(Grant::Item("minor_tonic".into())))
        );
        assert_eq!(
            state.interact(&world, &gallery, "fg_gallery_chest"),
            Err(InteractError::AlreadyDone)
        );
        let chest = gallery
            .object("fg_gallery_chest")
            .cloned()
            .unwrap_or_else(|| panic!("chest"));
        assert_eq!(state.object_state(&chest), ObjectState::Opened);
    }

    #[test]
    fn region_stats_and_the_global_percentage() {
        let world = slice();
        let mut state = ExplorationState::default();
        for room in world
            .rooms
            .values()
            .filter(|r| r.region == "forgotten_galleries")
        {
            state.rooms.insert(room.id.clone(), Discovery::Visited);
        }
        let galleries = state.region_stats(&world, "forgotten_galleries");
        assert_eq!(
            (galleries.visited, galleries.total, galleries.percent),
            (11, 11, 100.0)
        );
        assert_eq!(galleries.secrets_total, 2);
        let global = state.stats(&world);
        // V1.9: 22 rooms in the world (Veiled Cloister).
        // V2.1: 28 rooms (Village of Ashes).
        assert_eq!((global.visited, global.total), (11, 28));
        assert!((global.percent - 100.0 * 11.0 / 28.0).abs() < 1e-4);
        assert_eq!(state.region_stats(&world, "vigil_gates").visited, 0);
    }
}
