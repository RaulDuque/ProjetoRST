//! Save game format, versioning and (de)serialisation.
//!
//! Pure module: no Bevy types. Saves are RON documents with a `version`
//! field read *before* the payload, so that:
//! * newer, unknown versions are rejected without touching the file;
//! * older versions can be migrated step by step (see [`migrate`]);
//! * corrupted files produce [`SaveError::Corrupted`] instead of a panic.
//!
//! Versions:
//! * **v1** (until V1.6): player, world progress, play time.
//! * **v2** (V1.7): v1 + `exploration` (map, secrets, pickups, grants of the
//!   world graph). [`migrate`] parses a v1 document into its own frozen
//!   struct ([`SaveFileV1`], unknown fields rejected, so nothing is dropped
//!   silently) and converts it with [`upgrade_v1`]; the file itself is never
//!   modified by a load. Where a v1 room resumes in the world graph is a
//!   separate step (`world::restore`), because it depends on the content.
//! * **v3** (V2.1.1): v2 + `economy` (the player's coins and the units the
//!   shops' limited entries have sold). A v2 document is parsed into its own
//!   frozen struct ([`SaveFileV2`], unknown fields rejected) and converted by
//!   [`upgrade_v2`] with `economy: None`: the game then uses the starting
//!   coins (`assets/data/character.ron`, `start.coins`) and full stock, the
//!   same every time that file is loaded (no coins are granted twice, the
//!   file itself is never rewritten by a load). v1 chains `upgrade_v1` →
//!   `upgrade_v2`.
//! * **v4** (future): freeze today's [`SaveData`] as `SaveDataV3`, add an
//!   `upgrade_v3` and a `3 =>` arm in [`migrate`].

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::economy::shop::EconomyRecord;
use crate::inventory::equipment::Equipment;
use crate::inventory::inventory::Inventory;
use crate::player::attributes::BaseAttributes;
use crate::player::progression::Progression;
use crate::skills::unlock::LearnedSkills;
use crate::world::exploration::ExplorationRecord;
use crate::world::progress::WorldProgress;

/// Current save format version. Bump it (and add a migration) whenever
/// [`SaveData`] changes incompatibly.
pub const CURRENT_SAVE_VERSION: u32 = 3;
/// Oldest version that can still be migrated.
pub const MIN_SUPPORTED_SAVE_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerSave {
    pub room: String,
    pub position: (f32, f32),
    pub attributes: BaseAttributes,
    pub progression: Progression,
    pub hp: i32,
    pub mp: i32,
    pub hearts: i32,
    /// Techniques and spells share one table (spells are skills whose
    /// category is a spell category).
    pub learned_skills: LearnedSkills,
    pub inventory: Inventory,
    pub equipment: Equipment,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveData {
    pub player: PlayerSave,
    pub world: WorldProgress,
    /// World graph exploration (v2). Empty for a migrated v1 save.
    #[serde(default)]
    pub exploration: ExplorationRecord,
    /// Total play time in seconds.
    #[serde(default)]
    pub play_time: f64,
    /// Coins and shop stock (v3). `None` only for a save migrated from v1
    /// or v2, which never had an economy: the game uses the starting coins
    /// and full stock. Every v3 save written by the game has `Some`.
    pub economy: Option<EconomyRecord>,
}

/// Save v2 as written from V1.7 to V2.1, frozen.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SaveDataV2 {
    pub player: PlayerSave,
    pub world: WorldProgress,
    #[serde(default)]
    pub exploration: ExplorationRecord,
    #[serde(default)]
    pub play_time: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SaveFileV2 {
    pub version: u32,
    pub data: SaveDataV2,
}

/// v2 → v3: every v2 field is kept; the economy is absent (`None`).
pub fn upgrade_v2(old: SaveDataV2) -> SaveData {
    SaveData {
        player: old.player,
        world: old.world,
        exploration: old.exploration,
        play_time: old.play_time,
        economy: None,
    }
}

/// Save v1 as written until V1.6, frozen. `PlayerSave` and `WorldProgress`
/// did not change in v2; if a later version changes them, freeze copies
/// of their v1 shape here as well.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SaveDataV1 {
    pub player: PlayerSave,
    pub world: WorldProgress,
    #[serde(default)]
    pub play_time: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SaveFileV1 {
    pub version: u32,
    pub data: SaveDataV1,
}

/// v1 → v2: every v1 field is kept as is; the exploration starts empty
/// (the loaded room becomes visited when it is entered).
pub fn upgrade_v1_to_v2(old: SaveDataV1) -> SaveDataV2 {
    SaveDataV2 {
        player: old.player,
        world: old.world,
        exploration: ExplorationRecord::default(),
        play_time: old.play_time,
    }
}

/// v1 → current: `upgrade_v1_to_v2` then [`upgrade_v2`].
pub fn upgrade_v1(old: SaveDataV1) -> SaveData {
    upgrade_v2(upgrade_v1_to_v2(old))
}

/// A decoded save and the version it was read from.
#[derive(Debug, Clone, PartialEq)]
pub struct DecodedSave {
    pub data: SaveData,
    pub source_version: u32,
}

impl DecodedSave {
    /// Whether the data was converted from an older version in memory.
    pub fn migrated(&self) -> bool {
        self.source_version != CURRENT_SAVE_VERSION
    }
}

/// On-disk document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveFile {
    pub version: u32,
    pub data: SaveData,
}

/// Only the version, to decide how to read the rest.
#[derive(Debug, Deserialize)]
struct SaveHeader {
    version: u32,
}

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("save file is corrupted: {0}")]
    Corrupted(String),
    #[error("save version {found} is newer than this game supports ({supported})")]
    TooNew { found: u32, supported: u32 },
    #[error("save version {found} is too old (oldest supported: {oldest})")]
    TooOld { found: u32, oldest: u32 },
    #[error("cannot encode save: {0}")]
    Encode(String),
    #[error("i/o error on {path}: {source}")]
    Io {
        path: String,
        source: std::io::Error,
    },
    #[error("no save found at {0}")]
    NotFound(String),
}

pub fn encode(data: &SaveData) -> Result<String, SaveError> {
    let file = SaveFile {
        version: CURRENT_SAVE_VERSION,
        data: data.clone(),
    };
    ron::ser::to_string_pretty(&file, ron::ser::PrettyConfig::default())
        .map_err(|e| SaveError::Encode(e.to_string()))
}

pub fn decode(text: &str) -> Result<SaveData, SaveError> {
    decode_versioned(text).map(|d| d.data)
}

/// Reads the version of a document without decoding the rest.
pub fn peek_version(text: &str) -> Result<u32, SaveError> {
    let header: SaveHeader =
        ron::from_str(text).map_err(|e| SaveError::Corrupted(e.to_string()))?;
    Ok(header.version)
}

/// Like [`decode`], also telling which version the document had.
pub fn decode_versioned(text: &str) -> Result<DecodedSave, SaveError> {
    let header: SaveHeader =
        ron::from_str(text).map_err(|e| SaveError::Corrupted(e.to_string()))?;
    if header.version > CURRENT_SAVE_VERSION {
        return Err(SaveError::TooNew {
            found: header.version,
            supported: CURRENT_SAVE_VERSION,
        });
    }
    if header.version < MIN_SUPPORTED_SAVE_VERSION {
        return Err(SaveError::TooOld {
            found: header.version,
            oldest: MIN_SUPPORTED_SAVE_VERSION,
        });
    }
    Ok(DecodedSave {
        data: migrate(header.version, text)?,
        source_version: header.version,
    })
}

/// Reads a document of version `version` and upgrades it to the current
/// format. Each future version adds one arm that parses its own struct and
/// converts it to the next one.
fn migrate(version: u32, text: &str) -> Result<SaveData, SaveError> {
    match version {
        CURRENT_SAVE_VERSION => {
            let file: SaveFile =
                ron::from_str(text).map_err(|e| SaveError::Corrupted(e.to_string()))?;
            Ok(file.data)
        }
        2 => {
            let file: SaveFileV2 =
                ron::from_str(text).map_err(|e| SaveError::Corrupted(e.to_string()))?;
            Ok(upgrade_v2(file.data))
        }
        1 => {
            let file: SaveFileV1 =
                ron::from_str(text).map_err(|e| SaveError::Corrupted(e.to_string()))?;
            Ok(upgrade_v1(file.data))
        }
        other => Err(SaveError::TooOld {
            found: other,
            oldest: MIN_SUPPORTED_SAVE_VERSION,
        }),
    }
}
