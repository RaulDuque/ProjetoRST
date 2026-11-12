//! Learning skills and checking their requirements.
//!
//! Pure module: no Bevy types. Learning a skill and knowing its command are
//! separate: a technique found in a book may still require the player to
//! discover the input sequence (shown on the skill screen once revealed).

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::definitions::{Requirements, SkillDefinition, SkillId};
use crate::player::attributes::{BaseAttributes, PrimaryAttribute};

/// Everything a requirement may be checked against.
#[derive(Debug, Clone, Copy)]
pub struct RequirementContext<'a> {
    pub level: u32,
    /// Effective attributes (base + equipment).
    pub attributes: &'a BaseAttributes,
    pub owned_items: &'a BTreeSet<String>,
    pub progress_flags: &'a BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum RequirementError {
    #[error("requires level {required} (current {current})")]
    Level { required: u32, current: u32 },
    #[error("requires {} {required} (current {current})", attribute.short_name())]
    Attribute {
        attribute: PrimaryAttribute,
        required: i32,
        current: i32,
    },
    #[error("requires item `{0}`")]
    Item(String),
    #[error("requires progress `{0}`")]
    Progress(String),
}

impl Requirements {
    /// Returns the first unmet requirement, if any.
    pub fn check(&self, ctx: &RequirementContext<'_>) -> Result<(), RequirementError> {
        if ctx.level < self.min_level {
            return Err(RequirementError::Level {
                required: self.min_level,
                current: ctx.level,
            });
        }
        for (attribute, required) in &self.attributes {
            let current = ctx.attributes.get(*attribute);
            if current < *required {
                return Err(RequirementError::Attribute {
                    attribute: *attribute,
                    required: *required,
                    current,
                });
            }
        }
        if let Some(item) = self.items.iter().find(|i| !ctx.owned_items.contains(*i)) {
            return Err(RequirementError::Item(item.clone()));
        }
        if let Some(flag) = self
            .progress_flags
            .iter()
            .find(|f| !ctx.progress_flags.contains(*f))
        {
            return Err(RequirementError::Progress(flag.clone()));
        }
        Ok(())
    }
}

/// Per-skill learning state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillProgress {
    /// Upgrade rank, starting at 1.
    pub rank: u32,
    /// Whether the command is shown to the player.
    pub command_revealed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum LearnError {
    #[error("skill already learned")]
    AlreadyLearned,
    #[error(transparent)]
    Requirement(#[from] RequirementError),
}

/// Skills the character has learned.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearnedSkills {
    skills: BTreeMap<SkillId, SkillProgress>,
}

impl LearnedSkills {
    pub fn is_learned(&self, id: &str) -> bool {
        self.skills.contains_key(id)
    }

    pub fn progress(&self, id: &str) -> Option<SkillProgress> {
        self.skills.get(id).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &SkillProgress)> {
        self.skills.iter().map(|(id, p)| (id.as_str(), p))
    }

    /// Learns a skill after validating its requirements.
    pub fn learn(
        &mut self,
        def: &SkillDefinition,
        ctx: &RequirementContext<'_>,
    ) -> Result<(), LearnError> {
        if self.is_learned(&def.id) {
            return Err(LearnError::AlreadyLearned);
        }
        def.requirements.check(ctx)?;
        self.grant(def);
        Ok(())
    }

    /// Learns a skill unconditionally (starting skills, scripted rewards,
    /// loading a save).
    pub fn grant(&mut self, def: &SkillDefinition) {
        self.skills.entry(def.id.clone()).or_insert(SkillProgress {
            rank: 1,
            command_revealed: def.command_revealed_on_learn,
        });
    }

    pub fn reveal_command(&mut self, id: &str) -> bool {
        match self.skills.get_mut(id) {
            Some(progress) => {
                progress.command_revealed = true;
                true
            }
            None => false,
        }
    }

    pub fn upgrade(&mut self, id: &str) -> Option<u32> {
        self.skills.get_mut(id).map(|progress| {
            progress.rank += 1;
            progress.rank
        })
    }
}
