//! Shared resources: the loaded game data and the gameplay RNG.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::rng::GameRng;
use crate::enemies::definitions::EnemyDefinition;
use crate::inventory::equipment::EquipmentSlot;
use crate::inventory::items::{CatalogError, ItemCatalog};
use crate::player::attributes::{BaseAttributes, StatFormulas};
use crate::player::progression::ProgressionRules;
use crate::skills::definitions::{LibraryError, SkillLibrary};
use crate::world::rooms::RoomDefinition;

/// Gameplay random source.
#[derive(Resource, Debug, Clone)]
pub struct GameRandom(pub GameRng);

/// Character creation and balance settings (`assets/data/character.ron`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterConfig {
    pub progression: ProgressionRules,
    pub formulas: StatFormulas,
    pub start: StartingCharacter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartingCharacter {
    pub room: String,
    pub attributes: BaseAttributes,
    pub hearts: i32,
    #[serde(default)]
    pub items: Vec<(String, u32)>,
    #[serde(default)]
    pub equipped: Vec<(EquipmentSlot, String)>,
}

#[derive(Debug, Error)]
pub enum DataError {
    #[error(transparent)]
    Skills(#[from] LibraryError),
    #[error(transparent)]
    Items(#[from] CatalogError),
    #[error("cannot read {path}: {source}")]
    Io {
        path: String,
        source: std::io::Error,
    },
    #[error("invalid data in {path}: {source}")]
    Parse {
        path: String,
        source: Box<ron::error::SpannedError>,
    },
    #[error("starting room `{0}` does not exist")]
    MissingStartRoom(String),
    #[error("room `{room}` spawns unknown enemy `{enemy}`")]
    UnknownEnemy { room: String, enemy: String },
    #[error("starting item `{0}` does not exist")]
    UnknownStartingItem(String),
}

/// Every data file, validated and indexed. Inserted once at startup.
#[derive(Resource, Debug, Clone)]
pub struct GameData {
    pub skills: SkillLibrary,
    pub items: ItemCatalog,
    pub enemies: BTreeMap<String, EnemyDefinition>,
    pub rooms: BTreeMap<String, RoomDefinition>,
    pub character: CharacterConfig,
}

/// Directory containing `assets/`: `BEVY_ASSET_ROOT`, then
/// `CARGO_MANIFEST_DIR` (during `cargo run`), then the executable's folder.
pub fn project_root() -> PathBuf {
    if let Some(dir) = std::env::var_os("BEVY_ASSET_ROOT") {
        return PathBuf::from(dir);
    }
    if let Some(dir) = std::env::var_os("CARGO_MANIFEST_DIR") {
        return PathBuf::from(dir);
    }
    std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(Path::to_path_buf))
        .unwrap_or_else(|| PathBuf::from("."))
}

fn read_ron<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, DataError> {
    let text = std::fs::read_to_string(path).map_err(|source| DataError::Io {
        path: path.display().to_string(),
        source,
    })?;
    ron::from_str(&text).map_err(|source| DataError::Parse {
        path: path.display().to_string(),
        source: Box::new(source),
    })
}

fn ron_files(dir: &Path) -> Result<Vec<PathBuf>, DataError> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(dir)
        .map_err(|source| DataError::Io {
            path: dir.display().to_string(),
            source,
        })?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "ron"))
        .collect();
    files.sort();
    Ok(files)
}

impl GameData {
    /// Loads and cross-validates everything under `<root>/assets`.
    pub fn load(root: &Path) -> Result<Self, DataError> {
        let assets = root.join("assets");
        let data = assets.join("data");

        let skills = SkillLibrary::load_dirs([
            data.join("skills").as_path(),
            data.join("spells").as_path(),
        ])?;
        let items = ItemCatalog::load_dir(&data.join("items"))?;

        let mut enemies = BTreeMap::new();
        for file in ron_files(&data.join("enemies"))? {
            let list: Vec<EnemyDefinition> = read_ron(&file)?;
            for def in list {
                enemies.insert(def.id.clone(), def);
            }
        }

        let mut rooms = BTreeMap::new();
        for file in ron_files(&assets.join("maps"))? {
            let room: RoomDefinition = read_ron(&file)?;
            rooms.insert(room.id.clone(), room);
        }

        let character: CharacterConfig = read_ron(&data.join("character.ron"))?;

        let game_data = Self {
            skills,
            items,
            enemies,
            rooms,
            character,
        };
        game_data.validate()?;
        Ok(game_data)
    }

    fn validate(&self) -> Result<(), DataError> {
        let start = &self.character.start;
        if !self.rooms.contains_key(&start.room) {
            return Err(DataError::MissingStartRoom(start.room.clone()));
        }
        for room in self.rooms.values() {
            if let Some(spawn) = room
                .enemies
                .iter()
                .find(|s| !self.enemies.contains_key(&s.enemy))
            {
                return Err(DataError::UnknownEnemy {
                    room: room.id.clone(),
                    enemy: spawn.enemy.clone(),
                });
            }
        }
        let starting_ids = start
            .items
            .iter()
            .map(|(id, _)| id)
            .chain(start.equipped.iter().map(|(_, id)| id));
        for id in starting_ids {
            if self.items.get(id).is_none() {
                return Err(DataError::UnknownStartingItem(id.clone()));
            }
        }
        Ok(())
    }
}
