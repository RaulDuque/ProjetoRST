//! Executes spells confirmed by `skills::systems::begin_skill`. The parser
//! never applies magical effects: it only emits the `SkillActivated`
//! request; this module reacts to the confirmed `SkillStarted`.

use bevy::prelude::*;

use super::projectiles::Projectile;
use crate::combat::damage::HitSpec;
use crate::combat::hitboxes::{CombatProfile, Hitbox, Team};
use crate::core::constants::Z_EFFECTS;
use crate::core::events::{Notification, SkillStarted};
use crate::core::resources::GameData;
use crate::magic::elements::Element;
use crate::player::components::Vitals;
use crate::skills::definitions::SkillEffect;
use crate::world::physics::FacingDirection;

/// Vertical offset of projectiles relative to the caster's centre.
const CAST_HEIGHT: f32 = 6.0;

pub fn element_color(element: Element) -> Color {
    match element {
        Element::Fire => Color::srgb(1.0, 0.45, 0.1),
        Element::Ice => Color::srgb(0.6, 0.9, 1.0),
        Element::Lightning => Color::srgb(1.0, 1.0, 0.4),
        Element::Water => Color::srgb(0.2, 0.5, 1.0),
        Element::Wind => Color::srgb(0.6, 1.0, 0.7),
        Element::Earth => Color::srgb(0.6, 0.45, 0.25),
        Element::Light => Color::srgb(1.0, 0.95, 0.7),
        Element::Dark => Color::srgb(0.5, 0.2, 0.7),
        Element::Arcane => Color::srgb(0.8, 0.4, 1.0),
        Element::Physical => Color::srgb(0.9, 0.9, 0.9),
    }
}

/// Applies the effect of every confirmed spell (`SkillStarted`).
pub fn cast_spells(
    mut commands: Commands,
    data: Res<GameData>,
    mut activations: MessageReader<SkillStarted>,
    mut casters: Query<(
        &Transform,
        &FacingDirection,
        &CombatProfile,
        Option<&mut Vitals>,
    )>,
    mut notifications: MessageWriter<Notification>,
) {
    for activation in activations.read() {
        let Some(def) = data.skills.get(&activation.skill_id) else {
            continue;
        };
        if !def.category.is_spell() {
            continue;
        }
        let Ok((transform, facing, profile, vitals)) = casters.get_mut(activation.caster) else {
            continue;
        };
        match def.effect {
            SkillEffect::Projectile {
                speed,
                radius,
                lifetime,
            } => {
                let hit = HitSpec {
                    kind: def.damage_kind,
                    element: def.element,
                    base_damage: def.base_damage,
                    multiplier: def.damage_multiplier,
                };
                let size = Vec2::splat(radius * 2.0);
                let start = transform.translation.truncate()
                    + Vec2::new(facing.sign() * (radius + 16.0), CAST_HEIGHT);
                commands.spawn((
                    Hitbox {
                        owner: activation.caster,
                        team: Team::Player,
                        size,
                        attack: profile.attack(def.damage_kind).clone(),
                        hit,
                        lifetime,
                        pierce: false,
                        already_hit: Vec::new(),
                    },
                    Projectile {
                        velocity: Vec2::new(facing.sign() * speed, 0.0),
                    },
                    Sprite::from_color(element_color(def.element), size),
                    Transform::from_translation(start.extend(Z_EFFECTS)),
                ));
            }
            SkillEffect::Heal { amount } => {
                if let Some(mut vitals) = vitals {
                    let restored = vitals.hp.restore(amount);
                    notifications.write(Notification(format!("{}: +{restored} HP", def.name)));
                }
            }
            SkillEffect::MeleeStrike { .. } | SkillEffect::None => {}
        }
    }
}
