//! Validation of the world graph (V1.6). Pure module: no Bevy types.
//!
//! Structural checks (targets, reciprocity, map alignment, references) and
//! a progression simulation: starting at the start room with nothing, the
//! player walks every door whose requirement is met, collects every pickup of
//! the rooms reached, and repeats until nothing new is reachable. It runs
//! twice, without and with secrets, which classifies every room:
//!
//! | Access | Meaning |
//! |---|---|
//! | `Main` | reachable without any secret (required and optional content) |
//! | `Secret` | reachable only after finding a secret |
//! | `Unreachable` | never reachable: an error, whatever the room's role |
//!
//! A room whose role does not match its access (a required room behind a
//! secret, a secret room reachable without it) is reported too. Rooms that
//! are unreachable at the start are **not** errors: only rooms that the
//! whole progression never reaches are.
//!
//! V1.8: chests and levers grant like pickups; breakable walls and hidden
//! switches are secrets (a door they hide needs `with_secrets` and the
//! object's own requirement). New checks: unique object ids, objects inside
//! their room, levers whose flag nothing uses (warning), shortcuts that
//! change what the main progression reaches, and rooms with no way back to
//! the start (an irreversible block, checked at every stage of the
//! progression with the capabilities of that stage).
//!
//! V1.9: a door filled by a veil is only usable with the world's passage
//! requirement for that barrier kind (the unlock of the form that passes
//! it). A relic alone is not enough unless it is that requirement; whether
//! the form can afford the crossing (MP) is checked against the
//! transformation definitions by `app::load_world`. Veils are checked here
//! for their door, side, depth, unique id and passage ([`VeilProblem`]).

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use super::graph::{
    Capabilities, Condition, Door, MAX_VEIL_DEPTH, RoomRole, Side, WorldDefinition, WorldRoom,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Issue {
    UnknownRegion {
        room: String,
        region: String,
    },
    MissingTargetRoom {
        room: String,
        door: String,
    },
    MissingTargetDoor {
        room: String,
        door: String,
    },
    NotReciprocal {
        room: String,
        door: String,
    },
    /// The two sides of a door do not touch on the map.
    Misaligned {
        room: String,
        door: String,
    },
    DoorCellOutOfRange {
        room: String,
        door: String,
    },
    BothEntryOnly {
        room: String,
        door: String,
    },
    UnknownSecret {
        room: String,
        door: String,
        secret: String,
    },
    NoDoors {
        room: String,
    },
    /// Nothing in the world grants this condition.
    ImpossibleRequirement {
        room: String,
        door: String,
        condition: Condition,
    },
    Unreachable {
        room: String,
    },
    RequiredBehindSecret {
        room: String,
    },
    SecretRoomNotHidden {
        room: String,
    },
    RegionUnreachable {
        region: String,
    },
    GoalUnreachable {
        room: String,
    },
    BadStart {
        room: String,
        door: String,
    },
    /// Warning: no door of the room can be used to leave it.
    NoExit {
        room: String,
    },
    /// A legacy room (save v1) resumes at a door that does not exist.
    BadLegacyTarget {
        legacy: String,
        room: String,
        door: String,
    },
    /// Two secrets, rewards or objects share an id (V1.8).
    DuplicateId {
        id: String,
    },
    /// An object's area leaves its room (V1.8).
    ObjectOutOfRoom {
        room: String,
        object: String,
    },
    /// Nothing in the world grants this condition of an object (V1.8).
    ImpossibleObjectRequirement {
        room: String,
        object: String,
        condition: Condition,
    },
    /// Warning: a lever sets a flag no door or object requires (V1.8).
    UnusedLever {
        room: String,
        object: String,
    },
    /// With shortcuts the main progression reaches more than without them:
    /// the shortcut skips a requirement (V1.8).
    ShortcutBypassesProgression {
        room: String,
        door: String,
    },
    /// From this room the start cannot be reached again with what the
    /// player has when first getting there: an irreversible block (V1.8).
    NoWayBack {
        room: String,
    },
    /// A veil that cannot work as written (V1.9).
    BadVeil {
        room: String,
        veil: String,
        problem: VeilProblem,
    },
    /// Decoration or region visual that cannot be drawn as written (V2.3).
    BadScenery(super::scenery::SceneryError),
}

/// What is wrong with a veil (V1.9).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VeilProblem {
    /// Its door is not a door of the room.
    UnknownDoor,
    /// On a ceiling door: no form rises through it.
    CeilingDoor,
    /// Depth below 0, beyond `MAX_VEIL_DEPTH`, or not 0 on a floor door.
    Depth,
    /// Two veils fill the same door.
    SameDoor,
    /// The world has no passage for its kind, or nothing grants it.
    Impassable,
}

/// How a room is reached in the progression simulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Access {
    Main,
    Secret,
    Unreachable,
}

#[derive(Debug, Clone, Default)]
pub struct Report {
    pub errors: Vec<Issue>,
    pub warnings: Vec<Issue>,
    pub access: BTreeMap<String, Access>,
    /// Abilities, items and flags collected by the end of the simulation.
    pub final_capabilities: Capabilities,
}

impl Report {
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }
}

/// What the simulation may use besides plain doors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rules {
    /// Doors hidden by secrets can be used (the secret is found when its
    /// object's requirement is met).
    pub secrets: bool,
    /// Doors marked `shortcut` can be used.
    pub shortcuts: bool,
}

impl Rules {
    pub const MAIN: Rules = Rules {
        secrets: false,
        shortcuts: true,
    };
    pub const ALL: Rules = Rules {
        secrets: true,
        shortcuts: true,
    };
}

/// Whether the secret `id` of `room` can be found with `caps`: a false wall
/// always, a wall or switch when its requirement is met.
fn secret_findable(room: &WorldRoom, id: &str, caps: &Capabilities) -> bool {
    match room.object(id) {
        Some(object) => object.requires.as_ref().is_none_or(|r| r.is_met(caps)),
        None => room.secrets.iter().any(|s| s.id == id),
    }
}

fn door_usable(
    world: &WorldDefinition,
    room: &WorldRoom,
    door: &Door,
    caps: &Capabilities,
    rules: Rules,
) -> bool {
    !door.entry_only
        && (rules.shortcuts || !door.shortcut)
        && door
            .hidden_by
            .as_deref()
            .is_none_or(|s| rules.secrets && secret_findable(room, s, caps))
        && door.requires.as_ref().is_none_or(|r| r.is_met(caps))
        && room.veil(&door.id).is_none_or(|veil| {
            world
                .manifest
                .passages
                .get(&veil.kind)
                .is_some_and(|r| r.is_met(caps))
        })
}

/// Rooms reached from `from` with fixed capabilities (nothing collected).
/// Used for "what if" scenarios: without a relic, with a lever off...
pub fn reachable_from(
    world: &WorldDefinition,
    from: &str,
    caps: &Capabilities,
    rules: Rules,
) -> BTreeSet<String> {
    let mut seen = BTreeSet::new();
    let mut queue = VecDeque::from([from.to_owned()]);
    while let Some(id) = queue.pop_front() {
        let Some(room) = world.rooms.get(&id) else {
            continue;
        };
        if !seen.insert(id) {
            continue;
        }
        for door in &room.doors {
            if door_usable(world, room, door, caps, rules) && !seen.contains(&door.to.room) {
                queue.push_back(door.to.room.clone());
            }
        }
    }
    seen
}

/// Grants of the rooms in `rooms` whose requirement `caps` meets.
fn collect(world: &WorldDefinition, rooms: &BTreeSet<String>, caps: &mut Capabilities) {
    let before = caps.clone();
    for room in rooms.iter().filter_map(|id| world.rooms.get(id)) {
        for pickup in &room.pickups {
            caps.grant(&pickup.grants);
        }
        for object in &room.objects {
            if object.requires.as_ref().is_none_or(|r| r.is_met(&before))
                && let Some(grant) = object.grant()
            {
                caps.grant(&grant);
            }
        }
    }
}

/// The progression, stage by stage: rooms reached and what was collected
/// there, until nothing new is reachable. The last stage is the result.
pub fn stages(world: &WorldDefinition, rules: Rules) -> Vec<(BTreeSet<String>, Capabilities)> {
    let mut caps = Capabilities::default();
    let mut out = Vec::new();
    loop {
        let seen = reachable_from(world, &world.manifest.start_room, &caps, rules);
        let before = caps.clone();
        collect(world, &seen, &mut caps);
        out.push((seen, before.clone()));
        if caps == before {
            return out;
        }
    }
}

/// Rooms reached from the start, collecting pickups, chests and levers
/// until a fixpoint, and the capabilities collected.
pub fn reachable(world: &WorldDefinition, with_secrets: bool) -> (BTreeSet<String>, Capabilities) {
    let rules = if with_secrets {
        Rules::ALL
    } else {
        Rules::MAIN
    };
    stages(world, rules).pop().unwrap_or_default()
}

fn structural(world: &WorldDefinition, report: &mut Report) {
    let obtainable = world.obtainable();
    // V2.3: scenery is checked like any other data (never silently dropped).
    for region in world.regions.values() {
        if let Some(Err(e)) = region.visual.map(|v| v.validate(&region.id)) {
            report.errors.push(Issue::BadScenery(e));
        }
    }
    for room in world.rooms.values() {
        if let Err(e) = super::scenery::validate_decor(&room.id, room.size(), &room.decor) {
            report.errors.push(Issue::BadScenery(e));
        }
        if !world.regions.contains_key(&room.region) {
            report.errors.push(Issue::UnknownRegion {
                room: room.id.clone(),
                region: room.region.clone(),
            });
        }
        if room.doors.is_empty() {
            report.errors.push(Issue::NoDoors {
                room: room.id.clone(),
            });
        }
        let secrets: BTreeSet<&str> = room.secret_ids().collect();
        objects(room, &obtainable, world, report);
        veils(room, &obtainable, world, report);
        for door in &room.doors {
            let at = |issue: fn(String, String) -> Issue| issue(room.id.clone(), door.id.clone());
            if door.cell >= room.side_cells(door.side) {
                report
                    .errors
                    .push(at(|room, door| Issue::DoorCellOutOfRange { room, door }));
            }
            if let Some(secret) = &door.hidden_by
                && !secrets.contains(secret.as_str())
            {
                report.errors.push(Issue::UnknownSecret {
                    room: room.id.clone(),
                    door: door.id.clone(),
                    secret: secret.clone(),
                });
            }
            for condition in door.requires.iter().flat_map(|r| r.conditions()) {
                let single = match &condition {
                    Condition::Ability(a) => obtainable.abilities.contains(a),
                    Condition::Item(i) => obtainable.items.contains(i),
                    Condition::Flag(f) => obtainable.flags.contains(f),
                };
                // An `Any` with one obtainable branch is still satisfiable.
                let tree_ok = door
                    .requires
                    .as_ref()
                    .is_some_and(|r| r.is_met(&obtainable));
                if !single && !tree_ok {
                    report.errors.push(Issue::ImpossibleRequirement {
                        room: room.id.clone(),
                        door: door.id.clone(),
                        condition,
                    });
                }
            }
            let Some(target_room) = world.rooms.get(&door.to.room) else {
                report
                    .errors
                    .push(at(|room, door| Issue::MissingTargetRoom { room, door }));
                continue;
            };
            let Some(back) = target_room.door(&door.to.door) else {
                report
                    .errors
                    .push(at(|room, door| Issue::MissingTargetDoor { room, door }));
                continue;
            };
            if back.to.room != room.id
                || back.to.door != door.id
                || back.side != door.side.opposite()
            {
                report
                    .errors
                    .push(at(|room, door| Issue::NotReciprocal { room, door }));
                continue;
            }
            if door.entry_only && back.entry_only {
                report
                    .errors
                    .push(at(|room, door| Issue::BothEntryOnly { room, door }));
            }
            // Map alignment: the door's cell touches the target door's cell.
            let (a, b) = (room.map, target_room.map);
            let aligned = match door.side {
                Side::Right => {
                    a.x + a.w as i32 == b.x && a.y + door.cell as i32 == b.y + back.cell as i32
                }
                Side::Left => {
                    b.x + b.w as i32 == a.x && a.y + door.cell as i32 == b.y + back.cell as i32
                }
                Side::Top => {
                    a.y + a.h as i32 == b.y && a.x + door.cell as i32 == b.x + back.cell as i32
                }
                Side::Bottom => {
                    b.y + b.h as i32 == a.y && a.x + door.cell as i32 == b.x + back.cell as i32
                }
            };
            if !aligned {
                report
                    .errors
                    .push(at(|room, door| Issue::Misaligned { room, door }));
            }
        }
        if room.id != world.manifest.goal_room
            && !room.doors.is_empty()
            && room.doors.iter().all(|d| d.entry_only)
        {
            report.warnings.push(Issue::NoExit {
                room: room.id.clone(),
            });
        }
    }
    let start = &world.manifest;
    let start_ok = world
        .rooms
        .get(&start.start_room)
        .is_some_and(|r| r.door(&start.start_door).is_some());
    if !start_ok {
        report.errors.push(Issue::BadStart {
            room: start.start_room.clone(),
            door: start.start_door.clone(),
        });
    }
}

/// Object checks of one room (V1.8).
fn objects(
    room: &WorldRoom,
    obtainable: &Capabilities,
    world: &WorldDefinition,
    report: &mut Report,
) {
    let (w, h) = room.size();
    for object in &room.objects {
        let a = object.area;
        if a.min.0 < 0.0 || a.min.1 < 0.0 || a.min.0 + a.size.0 > w || a.min.1 + a.size.1 > h {
            report.errors.push(Issue::ObjectOutOfRoom {
                room: room.id.clone(),
                object: object.id.clone(),
            });
        }
        if let Some(requires) = &object.requires
            && !requires.is_met(obtainable)
        {
            for condition in requires.conditions() {
                report.errors.push(Issue::ImpossibleObjectRequirement {
                    room: room.id.clone(),
                    object: object.id.clone(),
                    condition,
                });
            }
        }
        if let super::graph::ObjectKind::Lever { sets } = &object.kind {
            let flag = Condition::Flag(sets.clone());
            let used = world.rooms.values().any(|r| {
                r.doors
                    .iter()
                    .filter_map(|d| d.requires.as_ref())
                    .chain(r.objects.iter().filter_map(|o| o.requires.as_ref()))
                    .any(|req| req.conditions().contains(&flag))
            });
            if !used {
                report.warnings.push(Issue::UnusedLever {
                    room: room.id.clone(),
                    object: object.id.clone(),
                });
            }
        }
    }
}

/// Veil checks of one room (V1.9).
fn veils(
    room: &WorldRoom,
    obtainable: &Capabilities,
    world: &WorldDefinition,
    report: &mut Report,
) {
    let mut doors = BTreeSet::new();
    for veil in &room.veils {
        let mut problem = |problem| {
            report.errors.push(Issue::BadVeil {
                room: room.id.clone(),
                veil: veil.id.clone(),
                problem,
            });
        };
        if !doors.insert(veil.door.as_str()) {
            problem(VeilProblem::SameDoor);
        }
        match room.door(&veil.door).map(|d| d.side) {
            None => problem(VeilProblem::UnknownDoor),
            Some(Side::Top) => problem(VeilProblem::CeilingDoor),
            Some(Side::Bottom) if veil.depth != 0.0 => problem(VeilProblem::Depth),
            Some(_) if !(0.0..=MAX_VEIL_DEPTH).contains(&veil.depth) => {
                problem(VeilProblem::Depth);
            }
            Some(_) => {}
        }
        let passable = world
            .manifest
            .passages
            .get(&veil.kind)
            .is_some_and(|r| r.is_met(obtainable));
        if !passable {
            problem(VeilProblem::Impassable);
        }
    }
}

/// Ids shared by secrets, rewards and objects must be unique in the world.
fn unique_ids(world: &WorldDefinition, report: &mut Report) {
    let mut seen = BTreeSet::new();
    for room in world.rooms.values() {
        let ids = room
            .secrets
            .iter()
            .map(|s| s.id.as_str())
            .chain(room.pickups.iter().map(|p| p.id.as_str()))
            .chain(room.objects.iter().map(|o| o.id.as_str()))
            .chain(room.veils.iter().map(|v| v.id.as_str()));
        for id in ids {
            if !seen.insert(id) {
                report.errors.push(Issue::DuplicateId { id: id.to_owned() });
            }
        }
    }
}

/// Shortcuts must not change what the main progression reaches, and from
/// every room reached at any stage the start must stay reachable.
fn progression_safety(world: &WorldDefinition, report: &mut Report) {
    let with = stages(world, Rules::MAIN).pop().unwrap_or_default().0;
    let without = stages(
        world,
        Rules {
            secrets: false,
            shortcuts: false,
        },
    )
    .pop()
    .unwrap_or_default()
    .0;
    if with != without {
        for room in world.rooms.values() {
            for door in room.doors.iter().filter(|d| d.shortcut) {
                report.errors.push(Issue::ShortcutBypassesProgression {
                    room: room.id.clone(),
                    door: door.id.clone(),
                });
            }
        }
    }
    let start = &world.manifest.start_room;
    let mut stuck = BTreeSet::new();
    for (rooms, caps) in stages(world, Rules::ALL) {
        for room in &rooms {
            if !reachable_from(world, room, &caps, Rules::ALL).contains(start) {
                stuck.insert(room.clone());
            }
        }
    }
    for room in stuck {
        report.errors.push(Issue::NoWayBack { room });
    }
}

/// Every check; `report.is_ok()` when the world can be shipped.
pub fn validate(world: &WorldDefinition) -> Report {
    let mut report = Report::default();
    structural(world, &mut report);
    unique_ids(world, &mut report);
    progression_safety(world, &mut report);
    let (main, _) = reachable(world, false);
    let (all, caps) = reachable(world, true);
    report.final_capabilities = caps;
    for room in world.rooms.values() {
        let access = if main.contains(&room.id) {
            Access::Main
        } else if all.contains(&room.id) {
            Access::Secret
        } else {
            Access::Unreachable
        };
        report.access.insert(room.id.clone(), access);
        let id = room.id.clone();
        match (room.role, access) {
            (_, Access::Unreachable) => report.errors.push(Issue::Unreachable { room: id }),
            (RoomRole::Required | RoomRole::Optional, Access::Secret) => {
                report.errors.push(Issue::RequiredBehindSecret { room: id });
            }
            (RoomRole::Secret, Access::Main) => {
                report
                    .warnings
                    .push(Issue::SecretRoomNotHidden { room: id });
            }
            _ => {}
        }
    }
    for region in world.regions.keys() {
        let any = world
            .rooms
            .values()
            .any(|r| &r.region == region && all.contains(&r.id));
        if !any {
            report.errors.push(Issue::RegionUnreachable {
                region: region.clone(),
            });
        }
    }
    for (legacy, target) in &world.manifest.legacy_rooms {
        let ok = world
            .room(&target.room)
            .and_then(|r| r.door(&target.door))
            .is_some();
        if !ok {
            report.errors.push(Issue::BadLegacyTarget {
                legacy: legacy.clone(),
                room: target.room.clone(),
                door: target.door.clone(),
            });
        }
    }
    let goal = &world.manifest.goal_room;
    if !main.contains(goal) {
        report
            .errors
            .push(Issue::GoalUnreachable { room: goal.clone() });
    }
    report.errors.sort();
    report.errors.dedup();
    report
}
