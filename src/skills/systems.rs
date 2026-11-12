//! Bevy systems shared by every skill: costs, cooldowns, caster state and
//! automatic unlocking. Effects are applied by `combat::attacks` (physical)
//! and `magic::casting` (spells).

use std::collections::BTreeSet;

use bevy::prelude::*;

use super::definitions::{Acquisition, SkillEffect};
use super::unlock::RequirementContext;
use crate::core::events::{Notification, SkillActivated};
use crate::core::resources::GameData;
use crate::core::time::frame_delta;
use crate::player::components::{
    ActionState, Belongings, CharacterSheet, Player, SkillBook, Vitals,
};
use crate::player::state::PlayerState;
use crate::world::WorldState;
use crate::world::physics::{FacingDirection, Velocity};

pub fn tick_cooldowns(time: Res<Time>, mut books: Query<&mut SkillBook>) {
    let dt = frame_delta(&time);
    for mut book in &mut books {
        book.cooldowns.tick(dt);
    }
}

/// Pays the costs, starts the cooldown and locks the caster.
pub fn begin_skill(
    data: Res<GameData>,
    mut activations: MessageReader<SkillActivated>,
    mut casters: Query<(
        &mut Vitals,
        &mut SkillBook,
        &mut ActionState,
        &mut Velocity,
        &FacingDirection,
        &CharacterSheet,
    )>,
) {
    for activation in activations.read() {
        let Some(def) = data.skills.get(&activation.skill_id) else {
            warn!(id = %activation.skill_id, "activated unknown skill");
            continue;
        };
        let Ok((mut vitals, mut book, mut state, mut velocity, facing, sheet)) =
            casters.get_mut(activation.caster)
        else {
            continue;
        };
        // Costs were validated by the recogniser in this same frame.
        let paid = vitals.mp.try_spend(def.mp_cost) && vitals.hearts.try_spend(def.hearts_cost);
        if !paid {
            warn!(id = %def.id, "skill activated without enough resources");
            continue;
        }
        book.cooldowns.start(&def.id, def.cooldown);
        let duration = def.execution_time / sheet.derived().attack_speed;
        if let Err(error) = state.start_action(PlayerState::Casting, duration) {
            warn!(%error, id = %def.id, "skill could not lock the caster");
        }
        if let SkillEffect::MeleeStrike { lunge_speed, .. } = def.effect {
            velocity.x = facing.sign() * lunge_speed;
        }
    }
}

/// Learns every `OnRequirements` skill whose requirements are now met.
pub fn auto_learn_skills(
    data: Res<GameData>,
    world: Res<WorldState>,
    player: Single<(&CharacterSheet, &Belongings, &mut SkillBook), With<Player>>,
    mut notifications: MessageWriter<Notification>,
) {
    let (sheet, belongings, mut book) = player.into_inner();
    let owned: BTreeSet<String> = belongings
        .inventory
        .stacks()
        .iter()
        .map(|s| s.id.clone())
        .chain(belongings.equipment.iter().map(|(_, id)| id.to_owned()))
        .collect();
    let effective = sheet.derived().effective;
    let ctx = RequirementContext {
        level: sheet.progression().level(),
        attributes: &effective,
        owned_items: &owned,
        progress_flags: &world.flags,
    };
    for def in data
        .skills
        .iter()
        .filter(|d| d.acquisition == Acquisition::OnRequirements)
    {
        if book.learned.is_learned(&def.id) {
            continue;
        }
        if book.learned.learn(def, &ctx).is_ok() {
            notifications.write(Notification(format!("Learned {}!", def.name)));
        }
    }
}
