//! Item definitions, categories and configurable rarities.
//!
//! Pure module: no Bevy types. Loaded from `assets/data/items/*.ron`.

use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::equipment::EquipmentSlot;
use crate::player::attributes::AttributeModifiers;
use crate::skills::definitions::Requirements;

pub type ItemId = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ItemCategory {
    Sword,
    Dagger,
    Axe,
    Spear,
    HeavyWeapon,
    Armor,
    Helmet,
    Gloves,
    Boots,
    Shield,
    Ring,
    Amulet,
    Consumable,
    Material,
    QuestItem,
}

impl ItemCategory {
    /// Slots an item of this category can be equipped in.
    pub fn slots(self) -> &'static [EquipmentSlot] {
        use EquipmentSlot as S;
        match self {
            ItemCategory::Sword
            | ItemCategory::Dagger
            | ItemCategory::Axe
            | ItemCategory::Spear
            | ItemCategory::HeavyWeapon => &[S::Weapon],
            ItemCategory::Shield => &[S::Offhand],
            ItemCategory::Helmet => &[S::Head],
            ItemCategory::Armor => &[S::Body],
            ItemCategory::Gloves => &[S::Gloves],
            ItemCategory::Boots => &[S::Boots],
            ItemCategory::Ring | ItemCategory::Amulet => &[S::Accessory1, S::Accessory2],
            ItemCategory::Consumable | ItemCategory::Material | ItemCategory::QuestItem => &[],
        }
    }

    pub fn is_equipment(self) -> bool {
        !self.slots().is_empty()
    }
}

/// A rarity tier, defined in data (`assets/data/items/rarities.ron`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RarityDefinition {
    pub id: String,
    pub name: String,
    /// Display colour (sRGB, 0..1).
    pub color: (f32, f32, f32),
    /// Sort order, lowest first.
    pub tier: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConsumableEffect {
    RestoreHp(i32),
    RestoreMp(i32),
    RestoreHearts(i32),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemDefinition {
    pub id: ItemId,
    pub name: String,
    pub description: String,
    pub category: ItemCategory,
    pub rarity: String,
    #[serde(default)]
    pub modifiers: AttributeModifiers,
    #[serde(default)]
    pub consumable: Option<ConsumableEffect>,
    /// Keys of special effects (interpreted by gameplay systems).
    #[serde(default)]
    pub special_effects: Vec<String>,
    #[serde(default)]
    pub requirements: Requirements,
    #[serde(default)]
    pub weight: f32,
    #[serde(default)]
    pub sell_value: u32,
    #[serde(default = "default_stack")]
    pub max_stack: u32,
}

fn default_stack() -> u32 {
    1
}

/// File layout of `assets/data/items/*.ron`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ItemDataFile {
    pub rarities: Vec<RarityDefinition>,
    pub items: Vec<ItemDefinition>,
}

#[derive(Debug, Error)]
pub enum CatalogError {
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
    #[error("duplicate id `{0}`")]
    DuplicateId(String),
    #[error("item `{item}` uses unknown rarity `{rarity}`")]
    UnknownRarity { item: String, rarity: String },
}

#[derive(Debug, Clone, Default)]
pub struct ItemCatalog {
    items: BTreeMap<ItemId, ItemDefinition>,
    rarities: BTreeMap<String, RarityDefinition>,
}

impl ItemCatalog {
    pub fn from_data(files: impl IntoIterator<Item = ItemDataFile>) -> Result<Self, CatalogError> {
        let mut catalog = ItemCatalog::default();
        let mut items = Vec::new();
        for file in files {
            for rarity in file.rarities {
                if catalog.rarities.contains_key(&rarity.id) {
                    return Err(CatalogError::DuplicateId(rarity.id));
                }
                catalog.rarities.insert(rarity.id.clone(), rarity);
            }
            items.extend(file.items);
        }
        for item in items {
            if !catalog.rarities.contains_key(&item.rarity) {
                return Err(CatalogError::UnknownRarity {
                    item: item.id,
                    rarity: item.rarity,
                });
            }
            if catalog.items.contains_key(&item.id) {
                return Err(CatalogError::DuplicateId(item.id));
            }
            catalog.items.insert(item.id.clone(), item);
        }
        Ok(catalog)
    }

    pub fn load_dir(dir: &Path) -> Result<Self, CatalogError> {
        let io_err = |path: &Path, source| CatalogError::Io {
            path: path.display().to_string(),
            source,
        };
        let mut files: Vec<_> = std::fs::read_dir(dir)
            .map_err(|e| io_err(dir, e))?
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|ext| ext == "ron"))
            .collect();
        files.sort();
        let mut data = Vec::new();
        for file in files {
            let text = std::fs::read_to_string(&file).map_err(|e| io_err(&file, e))?;
            let parsed: ItemDataFile =
                ron::from_str(&text).map_err(|source| CatalogError::Parse {
                    path: file.display().to_string(),
                    source: Box::new(source),
                })?;
            data.push(parsed);
        }
        let catalog = Self::from_data(data)?;
        tracing::info!(count = catalog.items.len(), "item catalog loaded");
        Ok(catalog)
    }

    pub fn get(&self, id: &str) -> Option<&ItemDefinition> {
        self.items.get(id)
    }

    pub fn rarity(&self, id: &str) -> Option<&RarityDefinition> {
        self.rarities.get(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &ItemDefinition> {
        self.items.values()
    }
}
