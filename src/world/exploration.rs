//! Exploration state of the world graph (V1.6). Pure module: no Bevy types.
//!
//! What the player knows and has done, by stable ids only (never by
//! position or entity), so content can be added without invalidating it:
//!
//! - each room is `Unknown`, `Discovered` (seen through a known door: drawn
//!   on the map as an outline) or `Visited` (entered);
//! - secrets found, pickups collected and the capabilities they granted.
//!
//! Exploration percentage (docs/MAP_EXPLORATION_SYSTEM.md):
//! `visited rooms / every room of the world`, secret rooms included, so 100 %
//! needs every secret room. It is recomputed from the ids, never stored:
//! adding a region lowers it instead of corrupting it. Known rooms and
//! secrets found are reported separately.
//!
//! Secrets are false walls, breakable walls and hidden switches (V1.8);
//! `secrets_total` counts all of them. [`ExplorationState::region_stats`]
//! gives the same numbers for one region.
//!
//! Interactive objects (V1.8) keep no state of their own: a wall is broken
//! and a switch found when their id is in `secrets`, a chest is open when
//! its id is in `pickups` (the one-time rewards), a lever is pulled when its
//! flag was granted. So the save v2 stores them without a new field.
//!
//! The state is saved on its own ([`ExplorationState::to_ron`]); merging it
//! into the game save is planned with a migration (docs/WORLD_PERSISTENCE.md).

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::graph::{
    Capabilities, Door, Grant, ObjectKind, WorldDefinition, WorldObject, WorldRoom,
};
use crate::combat::damage::DamageKind;
use crate::magic::elements::Element;

/// Format version of the stand-alone exploration file.
pub const EXPLORATION_VERSION: u32 = 1;

/// Map state of a room. `Completed` (every pickup and secret of the room
/// found) is planned once rooms carry enough content to define it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Discovery {
    #[default]
    Unknown,
    Discovered,
    Visited,
}

/// Whether a door can be used right now.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoorState {
    Open,
    /// Its requirement is not met: a barrier blocks it.
    Locked,
    /// Behind a secret not found yet: drawn as wall, absent from the map.
    Hidden,
    /// Arrival only.
    EntryOnly,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ExplorationState {
    pub rooms: BTreeMap<String, Discovery>,
    pub secrets: BTreeSet<String>,
    pub pickups: BTreeSet<String>,
    pub granted: Vec<Grant>,
    pub current_room: Option<String>,
    /// Room the player came from (for "return" checks and the map trail).
    pub previous_room: Option<String>,
}

/// The persistent part of [`ExplorationState`], as stored in the save v2
/// (V1.7). Only stable ids: the current and previous rooms are not stored
/// (the save's `player.room` is the current room), the percentage is
/// recomputed and door locks are derived from the grants.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ExplorationRecord {
    pub rooms: BTreeMap<String, Discovery>,
    pub secrets: BTreeSet<String>,
    pub pickups: BTreeSet<String>,
    pub granted: Vec<Grant>,
}

/// One door the map may draw (its room is known and the door is not secret).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KnownDoor {
    pub room: String,
    pub door: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExplorationStats {
    pub visited: usize,
    /// Discovered or visited.
    pub known: usize,
    pub total: usize,
    pub secrets_found: usize,
    pub secrets_total: usize,
    /// `100 * visited / total`.
    pub percent: f32,
}

/// State of an interactive object, derived from the exploration ids (V1.8).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectState {
    Intact,
    Broken,
    Hidden,
    Found,
    Closed,
    Opened,
    Off,
    On,
    /// Save altar: always usable (discovery lives in WorldProgress).
    SaveReady,
}

/// What breaking or using an object did (V1.8).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Interaction {
    /// A wall broke or a switch was found: this secret is new.
    SecretFound(String),
    /// A chest opened or a lever was pulled: what it granted (recorded).
    Granted(Grant),
    /// A save point was used (reusable; WorldProgress records discovery).
    SavePoint(String),
}

/// Why an object could not be broken or used. Nothing was recorded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum InteractError {
    #[error("no such object here, or it is not used this way")]
    NotAvailable,
    #[error("already done")]
    AlreadyDone,
    #[error("sealed: its requirement is not met")]
    Sealed,
    #[error("this attack does not break it")]
    WrongAttack,
}

#[derive(Debug, Error)]
pub enum ExplorationFileError {
    #[error("cannot parse the exploration file: {0}")]
    Parse(String),
    #[error("exploration file version {0} is not supported")]
    Version(u32),
    #[error("cannot encode the exploration state: {0}")]
    Encode(String),
}

#[derive(Serialize, Deserialize)]
struct ExplorationFile {
    version: u32,
    state: ExplorationState,
}

impl ExplorationState {
    pub fn discovery(&self, room: &str) -> Discovery {
        self.rooms.get(room).copied().unwrap_or_default()
    }

    pub fn capabilities(&self) -> Capabilities {
        let mut caps = Capabilities::default();
        for grant in &self.granted {
            caps.grant(grant);
        }
        caps
    }

    pub fn door_state(&self, door: &Door) -> DoorState {
        if door
            .hidden_by
            .as_ref()
            .is_some_and(|s| !self.secrets.contains(s))
        {
            DoorState::Hidden
        } else if door.entry_only {
            DoorState::EntryOnly
        } else if door
            .requires
            .as_ref()
            .is_some_and(|r| !r.is_met(&self.capabilities()))
        {
            DoorState::Locked
        } else {
            DoorState::Open
        }
    }

    fn discover(&mut self, room: &str) {
        let entry = self.rooms.entry(room.to_owned()).or_default();
        if *entry == Discovery::Unknown {
            *entry = Discovery::Discovered;
        }
    }

    /// The player is now in `room`: it becomes visited and the rooms behind
    /// its known doors (not hidden or arrival-only ones) become discovered.
    pub fn enter_room(&mut self, world: &WorldDefinition, room: &str) {
        if self.current_room.as_deref() != Some(room) {
            self.previous_room = self.current_room.replace(room.to_owned());
        }
        self.rooms.insert(room.to_owned(), Discovery::Visited);
        let Some(def) = world.room(room) else {
            return;
        };
        let targets: Vec<String> = def
            .doors
            .iter()
            // Hidden doors reveal nothing; arrival-only doors (a hole in the
            // ceiling) neither: the shortcut is found from its other side.
            .filter(|d| !matches!(self.door_state(d), DoorState::Hidden | DoorState::EntryOnly))
            .map(|d| d.to.room.clone())
            .collect();
        for target in targets {
            self.discover(&target);
        }
    }

    /// Records a secret of `room`; reveals the doors it hid (and the rooms
    /// behind them). Returns whether it was new.
    pub fn find_secret(&mut self, world: &WorldDefinition, room: &str, secret: &str) -> bool {
        if !self.secrets.insert(secret.to_owned()) {
            return false;
        }
        let revealed: Vec<String> = world
            .room(room)
            .map(|r| {
                r.doors
                    .iter()
                    .filter(|d| d.hidden_by.as_deref() == Some(secret))
                    .map(|d| d.to.room.clone())
                    .collect()
            })
            .unwrap_or_default();
        for target in revealed {
            self.discover(&target);
        }
        true
    }

    /// Collects a pickup once; returns what it granted the first time.
    pub fn collect(&mut self, room: &WorldRoom, pickup: &str) -> Option<Grant> {
        let def = room.pickups.iter().find(|p| p.id == pickup)?;
        if !self.pickups.insert(pickup.to_owned()) {
            return None;
        }
        self.granted.push(def.grants.clone());
        Some(def.grants.clone())
    }

    /// State of an object of the world (V1.8), from the ids alone.
    pub fn object_state(&self, object: &WorldObject) -> ObjectState {
        if matches!(object.kind, ObjectKind::SavePoint) {
            return ObjectState::SaveReady;
        }
        let done = match &object.kind {
            ObjectKind::BreakableWall { .. } | ObjectKind::HiddenSwitch => {
                self.secrets.contains(&object.id)
            }
            ObjectKind::Chest { .. } => self.pickups.contains(&object.id),
            ObjectKind::Lever { sets } => self.capabilities().flags.contains(sets),
            ObjectKind::SavePoint => false,
        };
        match (&object.kind, done) {
            (ObjectKind::BreakableWall { .. }, false) => ObjectState::Intact,
            (ObjectKind::BreakableWall { .. }, true) => ObjectState::Broken,
            (ObjectKind::HiddenSwitch, false) => ObjectState::Hidden,
            (ObjectKind::HiddenSwitch, true) => ObjectState::Found,
            (ObjectKind::Chest { .. }, false) => ObjectState::Closed,
            (ObjectKind::Chest { .. }, true) => ObjectState::Opened,
            (ObjectKind::Lever { .. }, false) => ObjectState::Off,
            (ObjectKind::Lever { .. }, true) => ObjectState::On,
            (ObjectKind::SavePoint, _) => ObjectState::SaveReady,
        }
    }

    fn usable<'a>(&self, room: &'a WorldRoom, id: &str) -> Result<&'a WorldObject, InteractError> {
        let object = room.object(id).ok_or(InteractError::NotAvailable)?;
        if matches!(
            self.object_state(object),
            ObjectState::Broken | ObjectState::Found | ObjectState::Opened | ObjectState::On
        ) {
            return Err(InteractError::AlreadyDone);
        }
        if object
            .requires
            .as_ref()
            .is_some_and(|r| !r.is_met(&self.capabilities()))
        {
            return Err(InteractError::Sealed);
        }
        Ok(object)
    }

    /// A player attack of `kind` and `element` hit the breakable wall `id`
    /// of `room`. Breaking it finds its secret, once.
    pub fn break_wall(
        &mut self,
        world: &WorldDefinition,
        room: &WorldRoom,
        id: &str,
        kind: DamageKind,
        element: Element,
    ) -> Result<Interaction, InteractError> {
        let object = self.usable(room, id)?;
        let ObjectKind::BreakableWall { breaks_on } = object.kind else {
            return Err(InteractError::NotAvailable);
        };
        if !breaks_on.accepts(kind, element) {
            return Err(InteractError::WrongAttack);
        }
        self.find_secret(world, &room.id, id);
        Ok(Interaction::SecretFound(id.to_owned()))
    }

    /// The player used the object `id` of `room` (interact key). Records the
    /// result and returns it, once: the state change and the grant are one
    /// step, so a reward can never be granted twice.
    pub fn interact(
        &mut self,
        world: &WorldDefinition,
        room: &WorldRoom,
        id: &str,
    ) -> Result<Interaction, InteractError> {
        let object = self.usable(room, id)?;
        match &object.kind {
            ObjectKind::BreakableWall { .. } => Err(InteractError::NotAvailable),
            ObjectKind::HiddenSwitch => {
                self.find_secret(world, &room.id, id);
                Ok(Interaction::SecretFound(id.to_owned()))
            }
            ObjectKind::Chest { reward, .. } => {
                self.pickups.insert(id.to_owned());
                self.granted.push(reward.clone());
                Ok(Interaction::Granted(reward.clone()))
            }
            ObjectKind::Lever { sets } => {
                let grant = Grant::Flag(sets.clone());
                self.granted.push(grant.clone());
                Ok(Interaction::Granted(grant))
            }
            ObjectKind::SavePoint => Ok(Interaction::SavePoint(id.to_owned())),
        }
    }

    /// What is saved (V1.7 save v2).
    pub fn record(&self) -> ExplorationRecord {
        ExplorationRecord {
            rooms: self.rooms.clone(),
            secrets: self.secrets.clone(),
            pickups: self.pickups.clone(),
            granted: self.granted.clone(),
        }
    }

    /// A fresh state from a saved record: replaces everything (nothing of
    /// the state before the load is kept). The current room is set when the
    /// loaded room is entered.
    pub fn from_record(record: ExplorationRecord) -> Self {
        Self {
            rooms: record.rooms,
            secrets: record.secrets,
            pickups: record.pickups,
            granted: record.granted,
            current_room: None,
            previous_room: None,
        }
    }

    /// Ids of the record that the loaded world does not have (removed
    /// content). They are kept (never silently dropped) and ignored by the
    /// statistics and the map.
    pub fn unknown_ids(&self, world: &WorldDefinition) -> Vec<String> {
        let rooms = self
            .rooms
            .keys()
            .filter(|id| world.room(id).is_none())
            .cloned();
        let all_secrets = world.secret_ids();
        let all_pickups = world.reward_ids();
        let secrets = self
            .secrets
            .iter()
            .filter(|s| !all_secrets.contains(s.as_str()))
            .cloned();
        let pickups = self
            .pickups
            .iter()
            .filter(|p| !all_pickups.contains(p.as_str()))
            .cloned();
        rooms.chain(secrets).chain(pickups).collect()
    }

    /// Doors the map may draw as connections. A door is known when its
    /// room was visited and it is neither hidden by an unfound secret nor
    /// arrival-only; an arrival-only door (a drop) becomes known once both
    /// of its rooms were visited. Secret content is never revealed early.
    pub fn known_doors(&self, world: &WorldDefinition) -> Vec<KnownDoor> {
        let mut out = Vec::new();
        for room in world.rooms.values() {
            if self.discovery(&room.id) != Discovery::Visited {
                continue;
            }
            for door in &room.doors {
                let known = match self.door_state(door) {
                    DoorState::Hidden => false,
                    DoorState::EntryOnly => self.discovery(&door.to.room) == Discovery::Visited,
                    DoorState::Open | DoorState::Locked => true,
                };
                if known {
                    out.push(KnownDoor {
                        room: room.id.clone(),
                        door: door.id.clone(),
                    });
                }
            }
        }
        out
    }

    pub fn stats(&self, world: &WorldDefinition) -> ExplorationStats {
        self.stats_of(world.rooms.values())
    }

    /// [`Self::stats`] restricted to the rooms of one region (V1.8). The
    /// global numbers keep their meaning: every room of every region.
    pub fn region_stats(&self, world: &WorldDefinition, region: &str) -> ExplorationStats {
        self.stats_of(world.rooms.values().filter(|r| r.region == region))
    }

    fn stats_of<'a>(&self, rooms: impl Iterator<Item = &'a WorldRoom> + Clone) -> ExplorationStats {
        let total = rooms.clone().count();
        let visited = rooms
            .clone()
            .filter(|r| self.discovery(&r.id) == Discovery::Visited)
            .count();
        let known = rooms
            .clone()
            .filter(|r| self.discovery(&r.id) != Discovery::Unknown)
            .count();
        let secrets: Vec<&str> = rooms.flat_map(WorldRoom::secret_ids).collect();
        let secrets_found = secrets
            .iter()
            .filter(|s| self.secrets.contains(**s))
            .count();
        ExplorationStats {
            visited,
            known,
            total,
            secrets_found,
            secrets_total: secrets.len(),
            percent: if total == 0 {
                0.0
            } else {
                100.0 * visited as f32 / total as f32
            },
        }
    }

    pub fn to_ron(&self) -> Result<String, ExplorationFileError> {
        let file = ExplorationFile {
            version: EXPLORATION_VERSION,
            state: self.clone(),
        };
        ron::ser::to_string_pretty(&file, ron::ser::PrettyConfig::default())
            .map_err(|e| ExplorationFileError::Encode(e.to_string()))
    }

    pub fn from_ron(text: &str) -> Result<Self, ExplorationFileError> {
        let file: ExplorationFile =
            ron::from_str(text).map_err(|e| ExplorationFileError::Parse(e.to_string()))?;
        if file.version != EXPLORATION_VERSION {
            return Err(ExplorationFileError::Version(file.version));
        }
        Ok(file.state)
    }
}
