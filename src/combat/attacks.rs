//! Physical attacks: the basic attack and melee techniques.

use bevy::prelude::*;

use super::damage::{DamageKind, HitSpec};
use super::hitboxes::{AttachedToOwner, CombatProfile, Hitbox, Team};
use crate::core::constants::{HITBOX_ACTIVE_FRACTION, Z_EFFECTS};
use crate::core::events::{BasicAttackRequested, SkillStarted};
use crate::core::resources::GameData;
use crate::inventory::systems::EquipmentAppearance;
use crate::inventory::weapons::StrikeProfile;
use crate::player::components::{ActionState, Belongings, CharacterSheet};
use crate::player::state::PlayerState;
use crate::skills::definitions::SkillEffect;

/// Alpha of the basic strike; its colour is the weapon's trail (V2.0,
/// white for the starting sword and unarmed, as before).
const SLASH_ALPHA: f32 = 0.35;
const TECHNIQUE_COLOR: Color = Color::srgba(1.0, 0.85, 0.4, 0.45);

/// Everything needed to spawn one melee hitbox.
struct MeleeStrike {
    owner: Entity,
    team: Team,
    size: Vec2,
    offset: Vec2,
    hit: HitSpec,
    lifetime: f32,
    color: Color,
}

fn spawn_melee_hitbox(
    commands: &mut Commands,
    strike: MeleeStrike,
    profile: &CombatProfile,
) -> Entity {
    let MeleeStrike {
        owner,
        team,
        size,
        offset,
        hit,
        lifetime,
        color,
    } = strike;
    commands
        .spawn((
            Hitbox {
                owner,
                team,
                size,
                attack: profile.attack(hit.kind).clone(),
                hit,
                lifetime,
                pierce: true,
                already_hit: Vec::new(),
            },
            AttachedToOwner { offset },
            Sprite::from_color(color, size),
            Transform::from_xyz(0.0, 0.0, Z_EFFECTS),
        ))
        .id()
}

/// Marks the hitbox of a basic attack with the weapon category that made it
/// (V2.0; `None` unarmed). Presentation and tests read it.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct WeaponStrike {
    pub category: Option<crate::inventory::items::ItemCategory>,
}

/// Basic attack (the attack button without a recognised command). V2.0:
/// shape, timing, power and element come from the equipped weapon
/// ([`StrikeProfile`]); without a weapon (or without belongings) it is the
/// pre-V2.0 strike, whose numbers the starting sword keeps.
pub fn basic_attack(
    mut commands: Commands,
    data: Res<GameData>,
    mut requests: MessageReader<BasicAttackRequested>,
    mut attackers: Query<(
        &mut ActionState,
        &CombatProfile,
        &CharacterSheet,
        Option<&Belongings>,
        Option<&EquipmentAppearance>,
    )>,
) {
    for request in requests.read() {
        let Ok((mut state, profile, sheet, belongings, look)) = attackers.get_mut(request.attacker)
        else {
            continue;
        };
        let speed = sheet.derived().attack_speed;
        let strike = belongings.map_or_else(
            || StrikeProfile::unarmed(speed),
            |b| data.items.strike_profile(&b.equipment, speed),
        );
        if state
            .start_action(PlayerState::Attacking, strike.action_time)
            .is_err()
        {
            continue;
        }
        let trail = look.map_or(crate::inventory::appearance::DEFAULT_TRAIL, |l| l.trail);
        let size = Vec2::new(strike.size.0, strike.size.1);
        let melee = MeleeStrike {
            owner: request.attacker,
            team: Team::Player,
            size,
            offset: Vec2::new(strike.offset.0, strike.offset.1),
            hit: HitSpec {
                kind: DamageKind::Physical,
                element: strike.element,
                base_damage: 0,
                multiplier: strike.multiplier,
            },
            lifetime: strike.active_time,
            color: Color::srgba(trail.0, trail.1, trail.2, SLASH_ALPHA),
        };
        let hitbox = spawn_melee_hitbox(&mut commands, melee, profile);
        commands.entity(hitbox).insert(WeaponStrike {
            category: strike.category,
        });
    }
}

/// Melee techniques (`SkillEffect::MeleeStrike`). Reacts only to
/// `SkillStarted`, i.e. activations already confirmed by `begin_skill`.
pub fn melee_techniques(
    mut commands: Commands,
    data: Res<GameData>,
    mut activations: MessageReader<SkillStarted>,
    casters: Query<&CombatProfile>,
) {
    for activation in activations.read() {
        let Some(def) = data.skills.get(&activation.skill_id) else {
            continue;
        };
        let SkillEffect::MeleeStrike { width, height, .. } = def.effect else {
            continue;
        };
        let Ok(profile) = casters.get(activation.caster) else {
            continue;
        };
        let size = Vec2::new(width, height);
        let strike = MeleeStrike {
            owner: activation.caster,
            team: Team::Player,
            size,
            offset: Vec2::new(size.x / 2.0 + 6.0, height / 2.0 - 20.0),
            hit: HitSpec {
                kind: def.damage_kind,
                element: def.element,
                base_damage: def.base_damage,
                multiplier: def.damage_multiplier,
            },
            lifetime: def.execution_time * HITBOX_ACTIVE_FRACTION,
            color: TECHNIQUE_COLOR,
        };
        spawn_melee_hitbox(&mut commands, strike, profile);
    }
}
