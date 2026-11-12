//! Skills (techniques and spells): definitions, learning, cooldowns and
//! command validation.

pub mod cooldowns;
pub mod definitions;
pub mod execution;
pub mod systems;
pub mod unlock;

use bevy::prelude::*;

use crate::app::states::GameSet;
use crate::combat::attacks::melee_techniques;
use crate::magic::casting::cast_spells;

pub struct SkillsPlugin;

impl Plugin for SkillsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                systems::tick_cooldowns,
                systems::begin_skill
                    .before(melee_techniques)
                    .before(cast_spells),
            )
                .in_set(GameSet::Actions),
        )
        .add_systems(
            Update,
            systems::auto_learn_skills.in_set(GameSet::Progression),
        );
    }
}
