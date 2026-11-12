//! Data-driven skill and spell definitions.
//!
//! Pure module: no Bevy types. Definitions are loaded from RON files in
//! `assets/data/skills/` and `assets/data/spells/`; adding a new technique
//! or spell never requires changing the combat engine.

use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::combat::command_parser::{CommandPattern, CommandSequence, CommandTiming};
use crate::combat::damage::DamageKind;
use crate::magic::elements::{Element, StatusEffect};
use crate::player::attributes::PrimaryAttribute;
use crate::player::state::ActionPhase;

/// Stable identifier of a skill, e.g. `"crescent_slash"`.
pub type SkillId = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkillCategory {
    SwordTechnique,
    HeavyWeaponTechnique,
    DefensiveTechnique,
    MovementTechnique,
    SpecialTechnique,
    ElementalSpell,
    SupportSpell,
    SummonSpell,
    Passive,
}

impl SkillCategory {
    pub fn is_spell(self) -> bool {
        matches!(
            self,
            SkillCategory::ElementalSpell
                | SkillCategory::SupportSpell
                | SkillCategory::SummonSpell
        )
    }
}

/// What happens when the skill is executed. Interpreted by the combat
/// (physical) or magic (spell) execution systems, never by the parser.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SkillEffect {
    /// A melee hitbox in front of the caster.
    MeleeStrike {
        /// Hitbox width × height in world units.
        width: f32,
        height: f32,
        /// Forward dash applied to the caster (units/second), 0 for none.
        lunge_speed: f32,
    },
    /// A projectile travelling forward.
    Projectile {
        speed: f32,
        radius: f32,
        lifetime: f32,
    },
    /// Instant self heal.
    Heal { amount: i32 },
    /// No active effect (passives).
    None,
}

/// How a skill is obtained.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Acquisition {
    /// Known from the start of the game.
    Starting,
    /// Learned automatically as soon as its requirements are met
    /// (level-up, attribute increase, item obtained...).
    OnRequirements,
    /// Found in the world (book, scroll, master, chest).
    #[default]
    Discovery,
}

/// Conditions to learn or use a skill.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Requirements {
    pub min_level: u32,
    pub attributes: BTreeMap<PrimaryAttribute, i32>,
    /// Item ids that must be owned (e.g. a specific weapon type).
    pub items: Vec<String>,
    /// World/story progression flags that must be set.
    pub progress_flags: Vec<String>,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkillDefinition {
    pub id: SkillId,
    pub name: String,
    pub description: String,
    pub category: SkillCategory,
    /// `None` for passives.
    #[serde(default)]
    pub command: Option<CommandSequence>,
    #[serde(default)]
    pub timing: CommandTiming,
    #[serde(default)]
    pub priority: i32,
    #[serde(default)]
    pub mp_cost: i32,
    #[serde(default)]
    pub hearts_cost: i32,
    /// Seconds before the skill can be used again.
    #[serde(default)]
    pub cooldown: f32,
    pub damage_kind: DamageKind,
    #[serde(default = "default_element")]
    pub element: Element,
    #[serde(default)]
    pub base_damage: i32,
    #[serde(default = "default_multiplier")]
    pub damage_multiplier: f32,
    /// Reach in world units (informative for UI/AI; hitboxes come from `effect`).
    #[serde(default)]
    pub range: f32,
    /// Time the caster is locked performing the skill (seconds).
    pub execution_time: f32,
    /// Animation key (resolved by the animation layer).
    #[serde(default)]
    pub animation: String,
    /// Visual effect key.
    #[serde(default)]
    pub vfx: String,
    pub effect: SkillEffect,
    /// Status inflicted on hit and its base chance.
    #[serde(default)]
    pub inflicts: Option<(StatusEffect, f32)>,
    #[serde(default)]
    pub requirements: Requirements,
    /// Phases in which the command may be performed.
    #[serde(default = "default_phases")]
    pub usable_in: Vec<ActionPhase>,
    /// How the character obtains the skill.
    #[serde(default)]
    pub acquisition: Acquisition,
    /// Whether the command is shown on the skill screen as soon as learned.
    /// When false the player must discover it (scroll, master, trial).
    #[serde(default = "default_true")]
    pub command_revealed_on_learn: bool,
}

fn default_element() -> Element {
    Element::Physical
}

fn default_multiplier() -> f32 {
    1.0
}

fn default_phases() -> Vec<ActionPhase> {
    vec![ActionPhase::Grounded, ActionPhase::Attacking]
}

impl SkillDefinition {
    /// The recogniser pattern, if the skill is triggered by a command.
    pub fn pattern(&self) -> Option<CommandPattern> {
        self.command.as_ref().map(|sequence| CommandPattern {
            id: self.id.clone(),
            sequence: sequence.clone(),
            timing: self.timing,
            priority: self.priority,
        })
    }
}

#[derive(Debug, Error)]
pub enum LibraryError {
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
    #[error("duplicate skill id `{0}`")]
    DuplicateId(String),
}

/// All known skill definitions, indexed by id.
#[derive(Debug, Clone, Default)]
pub struct SkillLibrary {
    skills: BTreeMap<SkillId, SkillDefinition>,
    patterns: Vec<CommandPattern>,
}

impl SkillLibrary {
    pub fn from_definitions(
        definitions: impl IntoIterator<Item = SkillDefinition>,
    ) -> Result<Self, LibraryError> {
        let mut library = SkillLibrary::default();
        for def in definitions {
            library.insert(def)?;
        }
        Ok(library)
    }

    pub fn insert(&mut self, def: SkillDefinition) -> Result<(), LibraryError> {
        if self.skills.contains_key(&def.id) {
            return Err(LibraryError::DuplicateId(def.id));
        }
        if let Some(pattern) = def.pattern() {
            self.patterns.push(pattern);
        }
        self.skills.insert(def.id.clone(), def);
        Ok(())
    }

    /// Parses a RON list of definitions.
    pub fn parse_ron(source: &str, origin: &str) -> Result<Vec<SkillDefinition>, LibraryError> {
        ron::from_str(source).map_err(|source| LibraryError::Parse {
            path: origin.to_owned(),
            source: Box::new(source),
        })
    }

    /// Loads every `*.ron` file of the given directories.
    pub fn load_dirs<'a>(dirs: impl IntoIterator<Item = &'a Path>) -> Result<Self, LibraryError> {
        let mut library = SkillLibrary::default();
        for dir in dirs {
            let entries = std::fs::read_dir(dir).map_err(|source| LibraryError::Io {
                path: dir.display().to_string(),
                source,
            })?;
            let mut files: Vec<_> = entries
                .filter_map(Result::ok)
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|ext| ext == "ron"))
                .collect();
            files.sort();
            for file in files {
                let origin = file.display().to_string();
                let text = std::fs::read_to_string(&file).map_err(|source| LibraryError::Io {
                    path: origin.clone(),
                    source,
                })?;
                for def in Self::parse_ron(&text, &origin)? {
                    library.insert(def)?;
                }
            }
        }
        tracing::info!(count = library.skills.len(), "skill library loaded");
        Ok(library)
    }

    pub fn get(&self, id: &str) -> Option<&SkillDefinition> {
        self.skills.get(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &SkillDefinition> {
        self.skills.values()
    }

    pub fn len(&self) -> usize {
        self.skills.len()
    }

    pub fn is_empty(&self) -> bool {
        self.skills.is_empty()
    }

    /// Patterns of every command-triggered skill, for the recogniser.
    pub fn patterns(&self) -> &[CommandPattern] {
        &self.patterns
    }
}
