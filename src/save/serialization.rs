//! Save game format, versioning and (de)serialisation.
//!
//! Pure module: no Bevy types. Saves are RON documents with a `version`
//! field read *before* the payload, so that:
//! * newer, unknown versions are rejected without touching the file;
//! * older versions can be migrated step by step (see [`migrate`]);
//! * corrupted files produce [`SaveError::Corrupted`] instead of a panic.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::inventory::equipment::Equipment;
use crate::inventory::inventory::Inventory;
use crate::player::attributes::BaseAttributes;
use crate::player::progression::Progression;
use crate::skills::unlock::LearnedSkills;
use crate::world::progress::WorldProgress;

/// Current save format version. Bump it (and add a migration) whenever
/// [`SaveData`] changes incompatibly.
pub const CURRENT_SAVE_VERSION: u32 = 1;
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
    /// Total play time in seconds.
    #[serde(default)]
    pub play_time: f64,
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
    migrate(header.version, text)
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
        other => Err(SaveError::TooOld {
            found: other,
            oldest: MIN_SUPPORTED_SAVE_VERSION,
        }),
    }
}
