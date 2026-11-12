//! Bevy components of the playable character.

use bevy::prelude::*;

use super::attributes::{
    AttributeModifiers, BaseAttributes, DerivedStats, PrimaryAttribute, RegenAccumulator,
    ResourcePool, StatFormulas,
};
use super::progression::{AllocationError, LevelUpSummary, Progression, ProgressionRules};
use super::state::PlayerStateMachine;
use crate::combat::command_buffer::CommandBuffer;
use crate::inventory::equipment::Equipment;
use crate::inventory::inventory::Inventory;
use crate::skills::cooldowns::Cooldowns;
use crate::skills::unlock::LearnedSkills;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Player;

/// Current action/locomotion state.
#[derive(Component, Debug, Clone, Copy, Default, Deref, DerefMut)]
pub struct ActionState(pub PlayerStateMachine);

/// Attributes, level and equipment bonus. Derived stats are recomputed by
/// every mutating method, so they can never be stale or duplicated.
#[derive(Component, Debug, Clone)]
pub struct CharacterSheet {
    base: BaseAttributes,
    progression: Progression,
    equipment_bonus: AttributeModifiers,
    derived: DerivedStats,
}

impl CharacterSheet {
    pub fn new(
        base: BaseAttributes,
        progression: Progression,
        equipment_bonus: AttributeModifiers,
        formulas: &StatFormulas,
    ) -> Self {
        let derived = formulas.derive(&base, progression.level(), &equipment_bonus);
        Self {
            base,
            progression,
            equipment_bonus,
            derived,
        }
    }

    pub fn base(&self) -> &BaseAttributes {
        &self.base
    }

    pub fn progression(&self) -> &Progression {
        &self.progression
    }

    pub fn derived(&self) -> &DerivedStats {
        &self.derived
    }

    fn recompute(&mut self, formulas: &StatFormulas) {
        self.derived = formulas.derive(&self.base, self.progression.level(), &self.equipment_bonus);
    }

    pub fn gain_experience(
        &mut self,
        amount: u64,
        rules: &ProgressionRules,
        formulas: &StatFormulas,
    ) -> LevelUpSummary {
        let summary = self
            .progression
            .gain_experience(amount, rules, &mut self.base);
        if summary.levels_gained > 0 {
            self.recompute(formulas);
        }
        summary
    }

    pub fn allocate_point(
        &mut self,
        attribute: PrimaryAttribute,
        formulas: &StatFormulas,
    ) -> Result<(), AllocationError> {
        self.progression.allocate_point(attribute, &mut self.base)?;
        self.recompute(formulas);
        Ok(())
    }

    pub fn set_equipment_bonus(&mut self, bonus: AttributeModifiers, formulas: &StatFormulas) {
        self.equipment_bonus = bonus;
        self.recompute(formulas);
    }
}

/// HP, MP and Hearts plus fractional regeneration.
#[derive(Component, Debug, Clone)]
pub struct Vitals {
    pub hp: ResourcePool,
    pub mp: ResourcePool,
    pub hearts: ResourcePool,
    pub hp_regen: RegenAccumulator,
    pub mp_regen: RegenAccumulator,
}

impl Vitals {
    pub fn full(derived: &DerivedStats, hearts: i32) -> Self {
        Self {
            hp: ResourcePool::full(derived.max_hp),
            mp: ResourcePool::full(derived.max_mp),
            hearts: ResourcePool::new(hearts, derived.max_hearts),
            hp_regen: RegenAccumulator::default(),
            mp_regen: RegenAccumulator::default(),
        }
    }

    /// Applies new maxima after a stat change.
    pub fn sync_max(&mut self, derived: &DerivedStats) {
        self.hp.set_max(derived.max_hp);
        self.mp.set_max(derived.max_mp);
        self.hearts.set_max(derived.max_hearts);
    }
}

/// Learned skills and their cooldowns.
#[derive(Component, Debug, Clone, Default)]
pub struct SkillBook {
    pub learned: LearnedSkills,
    pub cooldowns: Cooldowns,
}

/// Recent inputs used for command recognition.
#[derive(Component, Debug, Clone, Default, Deref, DerefMut)]
pub struct CommandInput(pub CommandBuffer);

#[derive(Component, Debug, Clone, Default)]
pub struct Belongings {
    pub inventory: Inventory,
    pub equipment: Equipment,
}

/// Variable-height jump bookkeeping.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct JumpControl {
    pub cut_applied: bool,
}
