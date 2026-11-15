//! World graph (V1.6): regions, rooms, doors and typed access requirements.
//!
//! Pure module: no Bevy types. The world is a graph whose nodes are rooms
//! and whose edges are doors. Cycles are allowed (shortcuts, loops); the
//! validator (`world::validation`) checks reachability. Data lives in
//! `assets/world/` (`world.ron` + `regions/*.ron`), separate from the
//! single prototype room of `assets/maps/`, so the current game is not
//! affected. Details: docs/WORLD_ARCHITECTURE.md and docs/ROOM_SYSTEM.md.
//!
//! V1.8 adds interactive objects ([`WorldObject`]): breakable walls, hidden
//! switches, chests and levers. Their state is never stored on the object:
//! it is derived from the exploration ids (secrets found, rewards taken,
//! flags granted), so the save format did not change.
//!
//! Rooms are laid out on a map grid of [`CELL_SIZE`] world units. A room
//! covers `w x h` cells; its boundary walls are generated, with an opening
//! for every door, so content files only describe the interior.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::collision::BarrierKind;
use super::rooms::{EnemySpawn, Platform, RoomDefinition};
use crate::combat::damage::DamageKind;
use crate::core::constants::PLAYER_SIZE;
use crate::magic::elements::Element;

/// One map cell in world units (one screen at the 1:1 camera).
pub const CELL_SIZE: (f32, f32) = (1280.0, 720.0);
/// Thickness of the generated boundary walls, floor and ceiling.
pub const WALL: f32 = 40.0;
/// Height of a side door opening (the player is 46 tall).
pub const SIDE_DOOR_SPAN: f32 = 160.0;
/// Width of a floor or ceiling door opening.
pub const FLOOR_DOOR_SPAN: f32 = 200.0;
/// How close to the room edge the player's centre must get to go through.
pub const DOOR_TRIGGER_DEPTH: f32 = 16.0;
/// Where the player appears, measured from the edge of the target door.
pub const ENTRY_INSET: f32 = 72.0;
/// Ledge generated under a side door above the first cell (so it can be used).
const THRESHOLD_LENGTH: f32 = 220.0;
/// Deepest a side veil may reach into its room (V1.9): it must leave room
/// to stand on a raised door's ledge in front of it.
pub const MAX_VEIL_DEPTH: f32 = 120.0;
/// Gap between a veil and the body of a player arriving through its door.
const VEIL_ENTRY_MARGIN: f32 = 8.0;
const THRESHOLD_THICKNESS: f32 = 20.0;

/// Exploration abilities. `VeilStep` (Relic of the Veil, V1.6) also unlocks
/// the Spectral Form since V1.9 (`assets/data/transformations.ron`); the
/// others are the planned set (docs/TRANSFORMATION_SYSTEM.md) so data can
/// already name them and the validator can report requirements nobody grants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Ability {
    /// Relic of the Veil: opens sealed grates (V1.6) and unlocks the
    /// Spectral Form (V1.9). The form itself is a state, never an ability.
    VeilStep,
    /// Opens astral seals (relic "Astral Emblem", V1.8). No physics of its
    /// own: like `VeilStep`, it only meets door and object requirements.
    AstralKey,
    DoubleJump,
    WallBreaker,
    /// Planned transformations.
    WraithForm,
    RavenForm,
    BeastForm,
    EmberWard,
}

/// What a door or a room needs. Typed, finite and evaluated by matching:
/// no expression is parsed or executed at runtime.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Requirement {
    Ability(Ability),
    /// Item id (`assets/data/items`), e.g. a key.
    Item(String),
    /// World flag, e.g. a seal broken by an event.
    Flag(String),
    All(Vec<Requirement>),
    Any(Vec<Requirement>),
}

/// One condition of a requirement tree.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Condition {
    Ability(Ability),
    Item(String),
    Flag(String),
}

/// What the player has, as seen by requirements.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Capabilities {
    pub abilities: BTreeSet<Ability>,
    pub items: BTreeSet<String>,
    pub flags: BTreeSet<String>,
}

impl Capabilities {
    pub fn grant(&mut self, grant: &Grant) -> bool {
        match grant {
            Grant::Ability(a) => self.abilities.insert(*a),
            Grant::Item(i) => self.items.insert(i.clone()),
            Grant::Flag(f) => self.flags.insert(f.clone()),
        }
    }

    fn has(&self, condition: &Condition) -> bool {
        match condition {
            Condition::Ability(a) => self.abilities.contains(a),
            Condition::Item(i) => self.items.contains(i),
            Condition::Flag(f) => self.flags.contains(f),
        }
    }
}

impl Requirement {
    pub fn is_met(&self, caps: &Capabilities) -> bool {
        match self {
            Requirement::Ability(a) => caps.has(&Condition::Ability(*a)),
            Requirement::Item(i) => caps.has(&Condition::Item(i.clone())),
            Requirement::Flag(f) => caps.has(&Condition::Flag(f.clone())),
            Requirement::All(list) => list.iter().all(|r| r.is_met(caps)),
            Requirement::Any(list) => list.iter().any(|r| r.is_met(caps)),
        }
    }

    /// Every condition named in the tree.
    pub fn conditions(&self) -> Vec<Condition> {
        match self {
            Requirement::Ability(a) => vec![Condition::Ability(*a)],
            Requirement::Item(i) => vec![Condition::Item(i.clone())],
            Requirement::Flag(f) => vec![Condition::Flag(f.clone())],
            Requirement::All(list) | Requirement::Any(list) => {
                list.iter().flat_map(Requirement::conditions).collect()
            }
        }
    }
}

/// What a pickup gives.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Grant {
    Ability(Ability),
    Item(String),
    Flag(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Side {
    Left,
    Right,
    Top,
    Bottom,
}

impl Side {
    pub fn opposite(self) -> Side {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
            Side::Top => Side::Bottom,
            Side::Bottom => Side::Top,
        }
    }
}

/// The far end of a door.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoorTarget {
    pub room: String,
    pub door: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Door {
    /// Unique inside its room.
    pub id: String,
    pub side: Side,
    /// Which cell along the side (0 = bottom for Left/Right, left for Top/Bottom).
    #[serde(default)]
    pub cell: u32,
    pub to: DoorTarget,
    /// Needed to go through (a barrier blocks the opening until it is met).
    #[serde(default)]
    pub requires: Option<Requirement>,
    /// Secret that hides this door (on the map and in the room) until found.
    #[serde(default)]
    pub hidden_by: Option<String>,
    /// Arrival only (e.g. a hole in the ceiling one drops through): it cannot
    /// be used to leave the room.
    #[serde(default)]
    pub entry_only: bool,
    /// A shortcut (V1.8): usually opened from its far side by a mechanism.
    /// Drawn in its own colour on the map once open. The validator checks
    /// that shortcuts never change what the main progression reaches.
    #[serde(default)]
    pub shortcut: bool,
}

/// Role of a room in the progression, checked by the validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoomRole {
    /// On the main path: must be reachable without secrets.
    Required,
    /// Off the main path but reachable without secrets.
    Optional,
    /// Reached only by finding a secret.
    Secret,
}

/// Placement on the map grid (cells; y grows upward like the world).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MapRect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pickup {
    pub id: String,
    pub name: String,
    pub position: (f32, f32),
    pub grants: Grant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecretKind {
    /// Looks like wall, has no collision; walking into it reveals it.
    IllusoryWall,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretDef {
    pub id: String,
    pub kind: SecretKind,
    /// Area drawn as wall (room coordinates).
    pub area: Platform,
}

/// A barrier only a transformation passes (V1.9), filling the opening of
/// a door: a side door (`depth` units into the room past the wall) or a
/// floor door (the opening itself; `depth` must be 0). Humans, enemies and
/// projectiles collide with it like a wall; the form named by the world's
/// `passages` walks or sinks through it. Never on a ceiling door (no form
/// rises).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Veil {
    /// Unique in the world.
    pub id: String,
    /// The door of this room whose opening it fills.
    pub door: String,
    pub kind: BarrierKind,
    #[serde(default)]
    pub depth: f32,
}

/// What breaks a [`ObjectKind::BreakableWall`]. Checked against the real
/// hitboxes of the player (strikes, techniques and spells); a wall takes no
/// damage, one valid hit breaks it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreakCondition {
    /// Any attack of the player.
    AnyAttack,
    /// Only attacks of this kind (e.g. `Physical`: no spell breaks it).
    Kind(DamageKind),
    /// Only attacks of this element (e.g. a wall of ice and `Fire`).
    Element(Element),
}

impl BreakCondition {
    pub fn accepts(self, kind: DamageKind, element: Element) -> bool {
        match self {
            BreakCondition::AnyAttack => true,
            BreakCondition::Kind(k) => k == kind,
            BreakCondition::Element(e) => e == element,
        }
    }
}

/// Visual tier of a chest (the reward is data, not the tier).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChestTier {
    Common,
    Rare,
    /// Inside a secret room.
    Secret,
    /// Planned for boss arenas (V2.x); allowed in data already.
    BossReward,
}

/// What an interactive object is and does (V1.8).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ObjectKind {
    /// Solid until a valid attack hits it; breaking it finds the secret of
    /// the same id (doors `hidden_by` it open).
    BreakableWall { breaks_on: BreakCondition },
    /// Looks like masonry; interacting (`D`) finds the secret of the same id.
    HiddenSwitch,
    /// Interacting opens it once and grants `reward`.
    Chest { tier: ChestTier, reward: Grant },
    /// Interacting pulls it once and sets the world flag `sets`.
    Lever { sets: String },
}

/// An interactive object of a room. Its id is unique in the world: walls and
/// switches share the secret ids, chests the one-time reward (pickup) ids.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldObject {
    pub id: String,
    pub kind: ObjectKind,
    /// Area in room coordinates: drawn, solid for an intact wall, and where
    /// the player must stand (with a small margin) to interact.
    pub area: Platform,
    /// Needed to break or use it (e.g. a relic); a wall stays intact and an
    /// object says "sealed" until it is met.
    #[serde(default)]
    pub requires: Option<Requirement>,
}

impl WorldObject {
    /// Breaking or using it finds a secret (its id is the secret id).
    pub fn reveals_secret(&self) -> bool {
        matches!(
            self.kind,
            ObjectKind::BreakableWall { .. } | ObjectKind::HiddenSwitch
        )
    }

    /// Used with the interact key (walls are broken by attacks instead).
    pub fn is_interactive(&self) -> bool {
        !matches!(self.kind, ObjectKind::BreakableWall { .. })
    }

    /// What using it grants (chest reward, lever flag), if anything.
    pub fn grant(&self) -> Option<Grant> {
        match &self.kind {
            ObjectKind::Chest { reward, .. } => Some(reward.clone()),
            ObjectKind::Lever { sets } => Some(Grant::Flag(sets.clone())),
            ObjectKind::BreakableWall { .. } | ObjectKind::HiddenSwitch => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldRoom {
    pub id: String,
    pub name: String,
    pub region: String,
    pub role: RoomRole,
    pub map: MapRect,
    pub background: (f32, f32, f32),
    /// Interior geometry only; walls, floor and ceiling are generated.
    #[serde(default)]
    pub platforms: Vec<Platform>,
    #[serde(default)]
    pub enemies: Vec<EnemySpawn>,
    #[serde(default)]
    pub doors: Vec<Door>,
    #[serde(default)]
    pub pickups: Vec<Pickup>,
    #[serde(default)]
    pub secrets: Vec<SecretDef>,
    /// Interactive objects (V1.8).
    #[serde(default)]
    pub objects: Vec<WorldObject>,
    /// Barriers only a transformation passes (V1.9).
    #[serde(default)]
    pub veils: Vec<Veil>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegionDef {
    pub id: String,
    pub name: String,
    /// Map colour of the region.
    pub color: (f32, f32, f32),
}

/// One file of `assets/world/regions/`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegionFile {
    pub region: RegionDef,
    pub rooms: Vec<WorldRoom>,
}

/// `assets/world/world.ron`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldManifest {
    pub start_room: String,
    /// Door of the start room the player begins at.
    pub start_door: String,
    /// Room that ends the main progression (must be reachable without secrets).
    pub goal_room: String,
    /// Rooms that existed before the world graph (save v1, `assets/maps/`)
    /// and the door of the graph where a save made there resumes (V1.7).
    #[serde(default)]
    pub legacy_rooms: BTreeMap<String, DoorTarget>,
    /// What lets the player through each kind of barrier (V1.9), as the
    /// progression sees it: e.g. `Spectral: Ability(VeilStep)`, the unlock of
    /// the form that passes veils. `app::load_world` checks it against the
    /// transformation definitions, so the two never disagree.
    #[serde(default)]
    pub passages: BTreeMap<BarrierKind, Requirement>,
}

#[derive(Debug, Error)]
pub enum WorldError {
    #[error("cannot read {path}: {message}")]
    Io { path: String, message: String },
    #[error("cannot parse {path}: {message}")]
    Parse { path: String, message: String },
    #[error("duplicate room id {0:?}")]
    DuplicateRoom(String),
    #[error("duplicate region id {0:?}")]
    DuplicateRegion(String),
    #[error("room {room}: duplicate door id {door:?}")]
    DuplicateDoor { room: String, door: String },
}

#[derive(Debug, Clone, PartialEq)]
pub struct WorldDefinition {
    pub manifest: WorldManifest,
    pub regions: BTreeMap<String, RegionDef>,
    pub rooms: BTreeMap<String, WorldRoom>,
}

impl WorldDefinition {
    /// Builds the world from its files, rejecting duplicated ids (structural
    /// rules; graph rules are in `world::validation`).
    pub fn from_parts(manifest: WorldManifest, files: Vec<RegionFile>) -> Result<Self, WorldError> {
        let mut regions = BTreeMap::new();
        let mut rooms = BTreeMap::new();
        for file in files {
            if regions.contains_key(&file.region.id) {
                return Err(WorldError::DuplicateRegion(file.region.id));
            }
            for room in file.rooms {
                let mut seen = BTreeSet::new();
                for door in &room.doors {
                    if !seen.insert(door.id.clone()) {
                        return Err(WorldError::DuplicateDoor {
                            room: room.id.clone(),
                            door: door.id.clone(),
                        });
                    }
                }
                if rooms.contains_key(&room.id) {
                    return Err(WorldError::DuplicateRoom(room.id));
                }
                rooms.insert(room.id.clone(), room);
            }
            regions.insert(file.region.id.clone(), file.region);
        }
        Ok(Self {
            manifest,
            regions,
            rooms,
        })
    }

    /// Reads `<dir>/world.ron` and every `<dir>/regions/*.ron` (sorted).
    pub fn load_dir(dir: &Path) -> Result<Self, WorldError> {
        let manifest: WorldManifest = read_ron(&dir.join("world.ron"))?;
        let regions_dir = dir.join("regions");
        let entries = std::fs::read_dir(&regions_dir).map_err(|e| WorldError::Io {
            path: regions_dir.display().to_string(),
            message: e.to_string(),
        })?;
        let mut paths: Vec<_> = entries
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|x| x == "ron"))
            .collect();
        paths.sort();
        let files = paths
            .iter()
            .map(|p| read_ron(p))
            .collect::<Result<Vec<RegionFile>, _>>()?;
        Self::from_parts(manifest, files)
    }

    pub fn room(&self, id: &str) -> Option<&WorldRoom> {
        self.rooms.get(id)
    }

    /// The door at the far end of `door`.
    pub fn target(&self, door: &Door) -> Option<(&WorldRoom, &Door)> {
        let room = self.rooms.get(&door.to.room)?;
        let target = room.door(&door.to.door)?;
        Some((room, target))
    }

    /// Every ability, item and flag some pickup, chest or lever of the
    /// world grants.
    pub fn obtainable(&self) -> Capabilities {
        let mut caps = Capabilities::default();
        for room in self.rooms.values() {
            for grant in room.grants() {
                caps.grant(&grant);
            }
        }
        caps
    }

    /// Ids of every secret of the world (false walls, breakable walls,
    /// hidden switches).
    pub fn secret_ids(&self) -> BTreeSet<&str> {
        self.rooms
            .values()
            .flat_map(WorldRoom::secret_ids)
            .collect()
    }

    /// Ids of every one-time reward of the world (pickups and chests).
    pub fn reward_ids(&self) -> BTreeSet<&str> {
        self.rooms
            .values()
            .flat_map(WorldRoom::reward_ids)
            .collect()
    }
}

fn read_ron<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, WorldError> {
    let text = std::fs::read_to_string(path).map_err(|e| WorldError::Io {
        path: path.display().to_string(),
        message: e.to_string(),
    })?;
    ron::from_str(&text).map_err(|e| WorldError::Parse {
        path: path.display().to_string(),
        message: e.to_string(),
    })
}

/// A rectangle in room coordinates (bottom-left corner and size).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub min: (f32, f32),
    pub size: (f32, f32),
}

impl Rect {
    pub fn contains(&self, p: (f32, f32)) -> bool {
        p.0 >= self.min.0
            && p.0 <= self.min.0 + self.size.0
            && p.1 >= self.min.1
            && p.1 <= self.min.1 + self.size.1
    }

    fn platform(&self) -> Platform {
        Platform {
            min: self.min,
            size: self.size,
            one_way: false,
            barrier: None,
        }
    }
}

impl WorldRoom {
    pub fn size(&self) -> (f32, f32) {
        (
            self.map.w as f32 * CELL_SIZE.0,
            self.map.h as f32 * CELL_SIZE.1,
        )
    }

    pub fn door(&self, id: &str) -> Option<&Door> {
        self.doors.iter().find(|d| d.id == id)
    }

    pub fn object(&self, id: &str) -> Option<&WorldObject> {
        self.objects.iter().find(|o| o.id == id)
    }

    /// The veil filling the opening of door `door`, if any (V1.9).
    pub fn veil(&self, door: &str) -> Option<&Veil> {
        self.veils.iter().find(|v| v.door == door)
    }

    /// Area of `veil` (room coordinates): the door opening, extended
    /// `depth` units into the room for a side door. `None` when its door
    /// does not exist.
    pub fn veil_area(&self, veil: &Veil) -> Option<Rect> {
        let door = self.door(&veil.door)?;
        let o = self.opening(door);
        let depth = veil.depth.max(0.0);
        Some(match door.side {
            Side::Left => Rect {
                min: o.min,
                size: (o.size.0 + depth, o.size.1),
            },
            Side::Right => Rect {
                min: (o.min.0 - depth, o.min.1),
                size: (o.size.0 + depth, o.size.1),
            },
            Side::Top | Side::Bottom => o,
        })
    }

    /// Secrets of the room: false walls and objects that reveal one.
    pub fn secret_ids(&self) -> impl Iterator<Item = &str> {
        self.secrets.iter().map(|s| s.id.as_str()).chain(
            self.objects
                .iter()
                .filter(|o| o.reveals_secret())
                .map(|o| o.id.as_str()),
        )
    }

    /// One-time rewards of the room: pickups and chests.
    pub fn reward_ids(&self) -> impl Iterator<Item = &str> {
        self.pickups.iter().map(|p| p.id.as_str()).chain(
            self.objects
                .iter()
                .filter(|o| matches!(o.kind, ObjectKind::Chest { .. }))
                .map(|o| o.id.as_str()),
        )
    }

    /// What the room can grant: pickups, chests and levers.
    pub fn grants(&self) -> Vec<Grant> {
        self.pickups
            .iter()
            .map(|p| p.grants.clone())
            .chain(self.objects.iter().filter_map(WorldObject::grant))
            .collect()
    }

    /// Cells along a side.
    pub fn side_cells(&self, side: Side) -> u32 {
        match side {
            Side::Left | Side::Right => self.map.h,
            Side::Top | Side::Bottom => self.map.w,
        }
    }

    /// The gap a door leaves in the boundary (room coordinates).
    pub fn opening(&self, door: &Door) -> Rect {
        let (w, h) = self.size();
        let cell = door.cell as f32;
        match door.side {
            Side::Left | Side::Right => {
                let x = if door.side == Side::Left {
                    0.0
                } else {
                    w - WALL
                };
                Rect {
                    min: (x, cell * CELL_SIZE.1 + WALL),
                    size: (WALL, SIDE_DOOR_SPAN),
                }
            }
            Side::Top | Side::Bottom => {
                let y = if door.side == Side::Bottom {
                    0.0
                } else {
                    h - WALL
                };
                Rect {
                    min: (
                        cell * CELL_SIZE.0 + (CELL_SIZE.0 - FLOOR_DOOR_SPAN) / 2.0,
                        y,
                    ),
                    size: (FLOOR_DOOR_SPAN, WALL),
                }
            }
        }
    }

    /// Whether the player's centre at `p` is going through `door`.
    pub fn crosses(&self, door: &Door, p: (f32, f32)) -> bool {
        let (w, h) = self.size();
        let o = self.opening(door);
        let along_y = p.1 >= o.min.1 && p.1 <= o.min.1 + o.size.1;
        let along_x = p.0 >= o.min.0 && p.0 <= o.min.0 + o.size.0;
        match door.side {
            Side::Left => along_y && p.0 <= DOOR_TRIGGER_DEPTH,
            Side::Right => along_y && p.0 >= w - DOOR_TRIGGER_DEPTH,
            Side::Bottom => along_x && p.1 <= DOOR_TRIGGER_DEPTH,
            Side::Top => along_x && p.1 >= h - DOOR_TRIGGER_DEPTH,
        }
    }

    /// Where the player stands after arriving through `door` (centre of the
    /// body), clear of the door's trigger.
    pub fn entry_point(&self, door: &Door) -> (f32, f32) {
        let (w, h) = self.size();
        let o = self.opening(door);
        let half_h = PLAYER_SIZE.1 / 2.0;
        // V1.9: a veiled side door puts the player past its veil.
        let inset = self.veil(&door.id).map_or(ENTRY_INSET, |v| {
            ENTRY_INSET.max(WALL + v.depth.max(0.0) + PLAYER_SIZE.0 / 2.0 + VEIL_ENTRY_MARGIN)
        });
        match door.side {
            Side::Left => (inset, o.min.1 + half_h + 1.0),
            Side::Right => (w - inset, o.min.1 + half_h + 1.0),
            Side::Bottom => (o.min.0 + o.size.0 / 2.0, ENTRY_INSET),
            Side::Top => (o.min.0 + o.size.0 / 2.0, h - ENTRY_INSET),
        }
    }

    /// Generated boundary (with door openings and ledges under raised side
    /// doors) plus the interior platforms.
    pub fn layout(&self) -> Vec<Platform> {
        let (w, h) = self.size();
        let mut out = Vec::new();
        for side in [Side::Left, Side::Right, Side::Top, Side::Bottom] {
            let mut gaps: Vec<(f32, f32)> = self
                .doors
                .iter()
                .filter(|d| d.side == side)
                .map(|d| {
                    let o = self.opening(d);
                    match side {
                        Side::Left | Side::Right => (o.min.1, o.min.1 + o.size.1),
                        Side::Top | Side::Bottom => (o.min.0, o.min.0 + o.size.0),
                    }
                })
                .collect();
            gaps.sort_by(|a, b| a.0.total_cmp(&b.0));
            let length = match side {
                Side::Left | Side::Right => h,
                Side::Top | Side::Bottom => w,
            };
            let mut start = 0.0;
            for (a, b) in gaps
                .iter()
                .copied()
                .chain(std::iter::once((length, length)))
            {
                if a > start {
                    out.push(
                        match side {
                            Side::Left => Rect {
                                min: (0.0, start),
                                size: (WALL, a - start),
                            },
                            Side::Right => Rect {
                                min: (w - WALL, start),
                                size: (WALL, a - start),
                            },
                            Side::Bottom => Rect {
                                min: (start, 0.0),
                                size: (a - start, WALL),
                            },
                            Side::Top => Rect {
                                min: (start, h - WALL),
                                size: (a - start, WALL),
                            },
                        }
                        .platform(),
                    );
                }
                start = start.max(b);
            }
        }
        for door in &self.doors {
            if matches!(door.side, Side::Left | Side::Right) && door.cell > 0 {
                let o = self.opening(door);
                let x = if door.side == Side::Left {
                    WALL
                } else {
                    w - WALL - THRESHOLD_LENGTH
                };
                out.push(Platform {
                    min: (x, o.min.1 - THRESHOLD_THICKNESS),
                    size: (THRESHOLD_LENGTH, THRESHOLD_THICKNESS),
                    one_way: false,
                    barrier: None,
                });
            }
        }
        out.extend(self.platforms.iter().copied());
        // V1.9: veils are part of the layout (always there, no state).
        out.extend(self.veils.iter().filter_map(|v| {
            let area = self.veil_area(v)?;
            Some(Platform {
                barrier: Some(v.kind),
                ..area.platform()
            })
        }));
        out
    }

    /// Solid that fills a door opening while its requirement is not met.
    pub fn barrier(&self, door: &Door) -> Platform {
        self.opening(door).platform()
    }

    /// Spawn point used by respawn: the entry of the first usable door.
    pub fn respawn_point(&self) -> (f32, f32) {
        self.doors
            .iter()
            .find(|d| !d.entry_only && d.hidden_by.is_none())
            .or(self.doors.first())
            .map_or((CELL_SIZE.0 / 2.0, WALL + PLAYER_SIZE.1), |d| {
                self.entry_point(d)
            })
    }

    /// The room as the existing room loader sees it (`world::maps::spawn_room`).
    pub fn to_room_definition(&self) -> RoomDefinition {
        RoomDefinition {
            id: self.id.clone(),
            name: self.name.clone(),
            size: self.size(),
            background: self.background,
            platforms: self.layout(),
            player_spawn: self.respawn_point(),
            enemies: self.enemies.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn door(id: &str, side: Side, cell: u32) -> Door {
        Door {
            id: id.into(),
            side,
            cell,
            to: DoorTarget {
                room: "x".into(),
                door: "y".into(),
            },
            requires: None,
            hidden_by: None,
            entry_only: false,
            shortcut: false,
        }
    }

    fn room(doors: Vec<Door>, w: u32, h: u32) -> WorldRoom {
        WorldRoom {
            id: "r".into(),
            name: "R".into(),
            region: "g".into(),
            role: RoomRole::Required,
            map: MapRect { x: 0, y: 0, w, h },
            background: (0.0, 0.0, 0.0),
            platforms: vec![],
            enemies: vec![],
            doors,
            pickups: vec![],
            secrets: vec![],
            objects: vec![],
            veils: vec![],
        }
    }

    #[test]
    fn requirements_combine_with_all_and_any() {
        let mut caps = Capabilities::default();
        let req = Requirement::All(vec![
            Requirement::Ability(Ability::WraithForm),
            Requirement::Flag("seal_broken".into()),
        ]);
        let alt = Requirement::Any(vec![
            Requirement::Ability(Ability::DoubleJump),
            Requirement::Ability(Ability::RavenForm),
        ]);
        assert!(!req.is_met(&caps) && !alt.is_met(&caps));
        caps.abilities.insert(Ability::WraithForm);
        assert!(!req.is_met(&caps), "All needs every part");
        caps.flags.insert("seal_broken".into());
        assert!(req.is_met(&caps));
        caps.abilities.insert(Ability::RavenForm);
        assert!(alt.is_met(&caps), "Any needs one branch");
        assert_eq!(req.conditions().len(), 2);
    }

    #[test]
    fn walls_leave_openings_exactly_at_the_doors() {
        let r = room(
            vec![door("e", Side::Right, 0), door("u", Side::Left, 1)],
            1,
            2,
        );
        let layout = r.layout();
        let o = r.opening(&r.doors[0]);
        // No wall overlaps the right opening.
        let blocked = layout.iter().any(|p| {
            p.min.0 < o.min.0 + o.size.0
                && p.min.0 + p.size.0 > o.min.0
                && p.min.1 < o.min.1 + o.size.1
                && p.min.1 + p.size.1 > o.min.1
        });
        assert!(!blocked);
        // The raised left door gets a ledge right below its opening.
        let up = r.opening(&r.doors[1]);
        assert!(
            layout
                .iter()
                .any(|p| p.min.0 == WALL && p.min.1 + p.size.1 == up.min.1)
        );
        // Entry points are clear of the triggers.
        for d in &r.doors {
            assert!(!r.crosses(d, r.entry_point(d)), "{}", d.id);
        }
    }

    #[test]
    fn crossing_is_only_through_the_opening() {
        let r = room(
            vec![door("e", Side::Right, 0), door("b", Side::Bottom, 0)],
            1,
            1,
        );
        let (w, _) = r.size();
        assert!(r.crosses(&r.doors[0], (w - 5.0, WALL + 30.0)));
        assert!(
            !r.crosses(&r.doors[0], (w - 5.0, 500.0)),
            "above the opening"
        );
        assert!(r.crosses(&r.doors[1], (640.0, 5.0)));
        assert!(!r.crosses(&r.doors[1], (100.0, 5.0)));
    }

    #[test]
    fn veils_fill_their_door_and_arrivals_land_past_them() {
        let mut r = room(
            vec![door("l", Side::Left, 0), door("b", Side::Bottom, 0)],
            1,
            1,
        );
        let plain_entry = r.entry_point(&r.doors[0]);
        r.veils = vec![
            Veil {
                id: "side".into(),
                door: "l".into(),
                kind: BarrierKind::Spectral,
                depth: 80.0,
            },
            Veil {
                id: "floor".into(),
                door: "b".into(),
                kind: BarrierKind::Spectral,
                depth: 0.0,
            },
        ];
        let side = r.veil_area(&r.veils[0]).unwrap_or_else(|| panic!("area"));
        assert_eq!(
            (side.min, side.size),
            ((0.0, WALL), (WALL + 80.0, SIDE_DOOR_SPAN))
        );
        let floor = r.veil_area(&r.veils[1]).unwrap_or_else(|| panic!("area"));
        assert_eq!(floor, r.opening(&r.doors[1]));
        // In the layout, as barriers.
        let barriers: Vec<Platform> = r
            .layout()
            .into_iter()
            .filter(|p| p.barrier.is_some())
            .collect();
        assert_eq!(barriers.len(), 2);
        // The arrival is clear of the veil and of the trigger.
        let entry = r.entry_point(&r.doors[0]);
        assert!(entry.0 - PLAYER_SIZE.0 / 2.0 > WALL + 80.0, "{entry:?}");
        assert!(entry.0 > plain_entry.0);
        assert!(!r.crosses(&r.doors[0], entry));
        let below = r.entry_point(&r.doors[1]);
        assert!(below.1 - PLAYER_SIZE.1 / 2.0 > WALL);
    }
}
