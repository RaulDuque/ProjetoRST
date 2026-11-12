//! Experience, levels and attribute points.
//!
//! Pure module: no Bevy types. The curve and growth rules are data
//! ([`ProgressionRules`]) so they can be loaded from
//! `assets/data/progression.ron` and rebalanced without code changes.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::attributes::{BaseAttributes, PrimaryAttribute};

/// Experience curve: `exp_to_next(level) = round(base * level ^ exponent)`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExperienceCurve {
    pub base: f64,
    pub exponent: f64,
    pub max_level: u32,
}

impl Default for ExperienceCurve {
    fn default() -> Self {
        Self {
            base: 30.0,
            exponent: 1.6,
            max_level: 99,
        }
    }
}

impl ExperienceCurve {
    /// EXP needed to go from `level` to `level + 1`. `None` at max level.
    pub fn exp_to_next(&self, level: u32) -> Option<u64> {
        if level == 0 || level >= self.max_level {
            return None;
        }
        let value = (self.base * f64::from(level).powf(self.exponent)).round();
        // Always at least 1 so a misconfigured curve cannot loop forever.
        Some((value as u64).max(1))
    }
}

/// Attributes gained automatically each level, plus free points to allocate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LevelGrowth {
    pub automatic: BaseAttributes,
    pub points_per_level: u32,
}

impl Default for LevelGrowth {
    fn default() -> Self {
        Self {
            automatic: BaseAttributes {
                strength: 1,
                constitution: 1,
                intelligence: 1,
                luck: 0,
            },
            points_per_level: 2,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProgressionRules {
    pub curve: ExperienceCurve,
    pub growth: LevelGrowth,
}

/// Result of [`Progression::gain_experience`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LevelUpSummary {
    pub levels_gained: u32,
    pub points_gained: u32,
    pub new_level: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum AllocationError {
    #[error("no unspent attribute points")]
    NoPointsAvailable,
}

/// Level, EXP and unspent attribute points of the character.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Progression {
    level: u32,
    /// EXP accumulated towards the next level.
    exp_into_level: u64,
    /// Lifetime EXP, for display.
    total_exp: u64,
    unspent_points: u32,
}

impl Default for Progression {
    fn default() -> Self {
        Self {
            level: 1,
            exp_into_level: 0,
            total_exp: 0,
            unspent_points: 0,
        }
    }
}

impl Progression {
    /// Rebuilds a progression from saved values (level clamped to `>= 1`).
    pub fn from_parts(
        level: u32,
        exp_into_level: u64,
        total_exp: u64,
        unspent_points: u32,
    ) -> Self {
        Self {
            level: level.max(1),
            exp_into_level,
            total_exp,
            unspent_points,
        }
    }

    pub fn level(&self) -> u32 {
        self.level
    }

    pub fn exp_into_level(&self) -> u64 {
        self.exp_into_level
    }

    pub fn total_exp(&self) -> u64 {
        self.total_exp
    }

    pub fn unspent_points(&self) -> u32 {
        self.unspent_points
    }

    /// Adds EXP, levelling up as many times as needed and applying
    /// automatic attribute growth to `attributes`.
    pub fn gain_experience(
        &mut self,
        amount: u64,
        rules: &ProgressionRules,
        attributes: &mut BaseAttributes,
    ) -> LevelUpSummary {
        let mut summary = LevelUpSummary {
            new_level: self.level,
            ..Default::default()
        };
        self.total_exp = self.total_exp.saturating_add(amount);
        self.exp_into_level = self.exp_into_level.saturating_add(amount);

        while let Some(needed) = rules.curve.exp_to_next(self.level) {
            if self.exp_into_level < needed {
                break;
            }
            self.exp_into_level -= needed;
            self.level += 1;
            self.unspent_points += rules.growth.points_per_level;
            for attribute in PrimaryAttribute::ALL {
                *attributes.get_mut(attribute) += rules.growth.automatic.get(attribute);
            }
            summary.levels_gained += 1;
            summary.points_gained += rules.growth.points_per_level;
        }

        // At max level surplus EXP is kept only in the lifetime total.
        if rules.curve.exp_to_next(self.level).is_none() {
            self.exp_into_level = 0;
        }

        summary.new_level = self.level;
        if summary.levels_gained > 0 {
            tracing::info!(level = self.level, "level up");
        }
        summary
    }

    /// Spends one free point on `attribute`.
    pub fn allocate_point(
        &mut self,
        attribute: PrimaryAttribute,
        attributes: &mut BaseAttributes,
    ) -> Result<(), AllocationError> {
        if self.unspent_points == 0 {
            return Err(AllocationError::NoPointsAvailable);
        }
        self.unspent_points -= 1;
        *attributes.get_mut(attribute) += 1;
        Ok(())
    }
}
