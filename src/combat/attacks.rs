//! Physical attacks: the basic attack and melee techniques.

use bevy::prelude::*;

use super::damage::{DamageKind, HitSpec};
use super::hitboxes::{AttachedToOwner, CombatProfile, Hitbox, Team};
use crate::core::constants::{
    BASIC_ATTACK_HITBOX, BASIC_ATTACK_TIME, HITBOX_ACTIVE_FRACTION, Z_EFFECTS,
};
use crate::core::events::{BasicAttackRequested, SkillActivated};
use crate::core::resources::GameData;
use crate::magic::elements::Element;
use crate::player::components::{ActionState, CharacterSheet};
use crate::player::state::PlayerState;
use crate::skills::definitions::SkillEffect;

const SLASH_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 0.35);
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

fn spawn_melee_hitbox(commands: &mut Commands, strike: MeleeStrike, profile: &CombatProfile) {
    let MeleeStrike {
        owner,
        team,
        size,
        offset,
        hit,
        lifetime,
        color,
    } = strike;
    commands.spawn((
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
    ));
}

/// Basic attack (the attack button without a recognised command).
pub fn basic_attack(
    mut commands: Commands,
    mut requests: MessageReader<BasicAttackRequested>,
    mut attackers: Query<(&mut ActionState, &CombatProfile, &CharacterSheet)>,
) {
    for request in requests.read() {
        let Ok((mut state, profile, sheet)) = attackers.get_mut(request.attacker) else {
            continue;
        };
        let duration = BASIC_ATTACK_TIME / sheet.derived().attack_speed;
        if state
            .start_action(PlayerState::Attacking, duration)
            .is_err()
        {
            continue;
        }
        let size = Vec2::new(BASIC_ATTACK_HITBOX.0, BASIC_ATTACK_HITBOX.1);
        let strike = MeleeStrike {
            owner: request.attacker,
            team: Team::Player,
            size,
            offset: Vec2::new(size.x / 2.0 + 8.0, 4.0),
            hit: HitSpec {
                kind: DamageKind::Physical,
                element: Element::Physical,
                base_damage: 0,
                multiplier: 1.0,
            },
            lifetime: duration * HITBOX_ACTIVE_FRACTION,
            color: SLASH_COLOR,
        };
        spawn_melee_hitbox(&mut commands, strike, profile);
    }
}

/// Melee techniques (`SkillEffect::MeleeStrike`).
pub fn melee_techniques(
    mut commands: Commands,
    data: Res<GameData>,
    mut activations: MessageReader<SkillActivated>,
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
