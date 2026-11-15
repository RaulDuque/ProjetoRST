//! Item definitions, categories and configurable rarities.
//!
//! Pure module: no Bevy types. Loaded from `assets/data/items/*.ron`.

use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::appearance::{ItemVisual, VisualError};
use super::equipment::{Equipment, EquipmentSlot};
use super::weapons::{StrikeProfile, WeaponClass, WeaponError, WeaponStats};
use crate::player::attributes::AttributeModifiers;
use crate::skills::definitions::Requirements;

pub type ItemId = String;

/// Each point of LCK multiplies drop chances by `1 + LUCK_DROP_BONUS`.
pub const LUCK_DROP_BONUS: f32 = 0.02;

/// Final chance of a drop: `base * (1 + luck * LUCK_DROP_BONUS)`, in `[0, 1]`.
pub fn drop_chance(base: f32, luck: i32) -> f32 {
    (base * (1.0 + luck.max(0) as f32 * LUCK_DROP_BONUS)).clamp(0.0, 1.0)
}

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

    /// The five weapon categories (each has a `WeaponClass`, V2.0).
    pub fn is_weapon(self) -> bool {
        matches!(
            self,
            ItemCategory::Sword
                | ItemCategory::Dagger
                | ItemCategory::Axe
                | ItemCategory::Spear
                | ItemCategory::HeavyWeapon
        )
    }

    /// Short label for the inventory.
    pub fn display_name(self) -> &'static str {
        match self {
            ItemCategory::Sword => "Sword",
            ItemCategory::Dagger => "Dagger",
            ItemCategory::Axe => "Axe",
            ItemCategory::Spear => "Spear",
            ItemCategory::HeavyWeapon => "Heavy weapon",
            ItemCategory::Armor => "Body",
            ItemCategory::Helmet => "Head",
            ItemCategory::Gloves => "Arms",
            ItemCategory::Boots => "Legs",
            ItemCategory::Shield => "Off-hand",
            ItemCategory::Ring => "Ring",
            ItemCategory::Amulet => "Amulet",
            ItemCategory::Consumable => "Consumable",
            ItemCategory::Material => "Material",
            ItemCategory::QuestItem => "Key item",
        }
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
    /// V2.0: element and reach of a weapon (weapon categories only).
    #[serde(default)]
    pub weapon: Option<WeaponStats>,
    /// V2.0: what the item changes on the character's look.
    #[serde(default)]
    pub visual: Option<ItemVisual>,
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
    /// V2.0: the basic attack of each weapon category.
    pub weapon_classes: Vec<WeaponClass>,
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
    #[error(transparent)]
    Weapon(#[from] WeaponError),
    #[error(transparent)]
    Visual(#[from] VisualError),
    #[error("item `{item}`: {problem}")]
    InvalidItem { item: String, problem: &'static str },
}

#[derive(Debug, Clone, Default)]
pub struct ItemCatalog {
    items: BTreeMap<ItemId, ItemDefinition>,
    rarities: BTreeMap<String, RarityDefinition>,
    weapon_classes: BTreeMap<ItemCategory, WeaponClass>,
}

/// Checks the V2.0 parts of an item: weapon data only on weapons with a
/// sane reach, a visual that fits the category, finite modifiers.
fn validate_item(item: &ItemDefinition) -> Result<(), CatalogError> {
    if let Some(stats) = &item.weapon {
        if !item.category.is_weapon() {
            return Err(WeaponError::StatsOnNonWeapon {
                item: item.id.clone(),
            }
            .into());
        }
        if !stats.reach_bonus.is_finite() {
            return Err(WeaponError::ItemReach {
                item: item.id.clone(),
                reach: stats.reach_bonus,
            }
            .into());
        }
    }
    if let Some(visual) = item.visual {
        visual.validate(&item.id, item.category)?;
    }
    let m = &item.modifiers;
    let floats = [
        m.critical_chance,
        m.critical_damage,
        m.attack_speed,
        m.movement_speed,
        m.hp_regen,
        m.mp_regen,
        m.dodge,
    ];
    if floats.iter().any(|v| !v.is_finite()) {
        return Err(CatalogError::InvalidItem {
            item: item.id.clone(),
            problem: "non-finite modifier",
        });
    }
    if item.max_stack == 0 {
        return Err(CatalogError::InvalidItem {
            item: item.id.clone(),
            problem: "max_stack must be at least 1",
        });
    }
    Ok(())
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
            for class in file.weapon_classes {
                class.validate()?;
                if catalog.weapon_classes.contains_key(&class.category) {
                    return Err(WeaponError::Duplicate(class.category).into());
                }
                catalog.weapon_classes.insert(class.category, class);
            }
        }
        for item in items {
            validate_item(&item)?;
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

    pub fn weapon_class(&self, category: ItemCategory) -> Option<&WeaponClass> {
        self.weapon_classes.get(&category)
    }

    pub fn weapon_classes(&self) -> impl Iterator<Item = &WeaponClass> {
        self.weapon_classes.values()
    }

    /// Weapon items whose category has no class (the game data requires
    /// none; the catalog alone accepts them and they strike unarmed).
    pub fn weapons_without_class(&self) -> Vec<&str> {
        self.items
            .values()
            .filter(|d| d.category.is_weapon() && !self.weapon_classes.contains_key(&d.category))
            .map(|d| d.id.as_str())
            .collect()
    }

    /// Strike of the basic attack with this equipment: the weapon's class
    /// and stats, or the unarmed strike (no weapon, unknown weapon or a
    /// category without a class).
    pub fn strike_profile(&self, equipment: &Equipment, attack_speed: f32) -> StrikeProfile {
        equipment
            .get(EquipmentSlot::Weapon)
            .and_then(|id| self.get(id))
            .and_then(|def| {
                let class = self.weapon_classes.get(&def.category)?;
                let stats = def.weapon.clone().unwrap_or_default();
                Some(StrikeProfile::resolve(class, &stats, attack_speed))
            })
            .unwrap_or_else(|| StrikeProfile::unarmed(attack_speed))
    }

    /// V2.1: whether this item is a weapon whose class needs both hands.
    pub fn is_two_handed(&self, item: &str) -> bool {
        self.get(item)
            .and_then(|d| self.weapon_classes.get(&d.category))
            .is_some_and(|c| c.two_handed)
    }

    /// Category of the equipped weapon, if any (technique compatibility).
    pub fn weapon_category(&self, equipment: &Equipment) -> Option<ItemCategory> {
        equipment
            .get(EquipmentSlot::Weapon)
            .and_then(|id| self.get(id))
            .map(|d| d.category)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn luck_increases_drop_chance() {
        assert!((drop_chance(0.1, 0) - 0.1).abs() < 1e-6);
        assert!((drop_chance(0.1, 50) - 0.2).abs() < 1e-6);
        assert_eq!(drop_chance(0.9, 1000), 1.0);
        assert!((drop_chance(0.1, -10) - 0.1).abs() < 1e-6);
    }
}
