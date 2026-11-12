//! Hitboxes, hurtboxes and the combat profile shared by every fighter.
//!
//! The overlap test produces [`DamageDealt`] messages; applying damage is
//! the responsibility of the target's own module (player or enemies).

use bevy::prelude::*;

use super::damage::{
    AttackProfile, DamageKind, DamageRolls, DefenseProfile, HitSpec, calculate_damage,
};
use crate::core::events::DamageDealt;
use crate::core::resources::GameRandom;
use crate::core::time::frame_delta;
use crate::player::attributes::DerivedStats;
use crate::world::collision::Aabb;
use crate::world::physics::FacingDirection;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Team {
    Player,
    Enemy,
}

/// Offensive and defensive numbers used when a hit connects.
#[derive(Component, Debug, Clone)]
pub struct CombatProfile {
    pub physical: AttackProfile,
    pub magical: AttackProfile,
    pub defense: DefenseProfile,
}

impl CombatProfile {
    pub fn from_derived(d: &DerivedStats) -> Self {
        let profile = |power| AttackProfile {
            power,
            hit: d.hit,
            critical_chance: d.critical_chance,
            critical_damage: d.critical_damage,
        };
        Self {
            physical: profile(d.attack),
            magical: profile(d.magic_attack),
            defense: DefenseProfile {
                defense: d.defense,
                magic_defense: d.magic_defense,
                dodge: d.dodge,
                resistances: d.resistances.clone(),
            },
        }
    }

    pub fn attack(&self, kind: DamageKind) -> &AttackProfile {
        match kind {
            DamageKind::Physical => &self.physical,
            DamageKind::Magical => &self.magical,
        }
    }
}

/// Area that can be hit, centred on the entity.
#[derive(Component, Debug, Clone, Copy)]
pub struct Hurtbox {
    pub team: Team,
    pub size: Vec2,
}

/// Remaining invulnerability (seconds). Hits are ignored while positive.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Invulnerable {
    pub remaining: f32,
}

/// Damaging area. Each target is hit at most once per hitbox.
#[derive(Component, Debug, Clone)]
pub struct Hitbox {
    pub owner: Entity,
    pub team: Team,
    pub size: Vec2,
    pub attack: AttackProfile,
    pub hit: HitSpec,
    pub lifetime: f32,
    /// Keeps hitting further targets instead of disappearing on first hit.
    pub pierce: bool,
    pub already_hit: Vec<Entity>,
}

/// Keeps a hitbox at `offset` in front of its owner (mirrored by facing).
#[derive(Component, Debug, Clone, Copy)]
pub struct AttachedToOwner {
    pub offset: Vec2,
}

pub fn tick_invulnerability(time: Res<Time>, mut query: Query<&mut Invulnerable>) {
    let dt = frame_delta(&time);
    for mut inv in &mut query {
        if inv.remaining > 0.0 {
            inv.remaining = (inv.remaining - dt).max(0.0);
        }
    }
}

pub fn follow_owners(
    mut commands: Commands,
    mut hitboxes: Query<(Entity, &Hitbox, &AttachedToOwner, &mut Transform)>,
    owners: Query<(&Transform, &FacingDirection), Without<Hitbox>>,
) {
    for (entity, hitbox, attached, mut transform) in &mut hitboxes {
        match owners.get(hitbox.owner) {
            Ok((owner, facing)) => {
                transform.translation.x = owner.translation.x + attached.offset.x * facing.sign();
                transform.translation.y = owner.translation.y + attached.offset.y;
            }
            Err(_) => commands.entity(entity).try_despawn(),
        }
    }
}

pub fn expire_hitboxes(
    mut commands: Commands,
    time: Res<Time>,
    mut hitboxes: Query<(Entity, &mut Hitbox)>,
) {
    let dt = frame_delta(&time);
    for (entity, mut hitbox) in &mut hitboxes {
        hitbox.lifetime -= dt;
        if hitbox.lifetime <= 0.0 {
            commands.entity(entity).try_despawn();
        }
    }
}

/// Draws the three random values of a damage roll.
pub fn roll_damage(rng: &mut GameRandom) -> DamageRolls {
    DamageRolls {
        hit: rng.0.next_f32(),
        critical: rng.0.next_f32(),
        variance: rng.0.next_f32(),
    }
}

pub fn detect_hits(
    mut commands: Commands,
    mut rng: ResMut<GameRandom>,
    mut hitboxes: Query<(Entity, &Transform, &mut Hitbox)>,
    hurtboxes: Query<(
        Entity,
        &Transform,
        &Hurtbox,
        &CombatProfile,
        Option<&Invulnerable>,
    )>,
    mut damage: MessageWriter<DamageDealt>,
) {
    for (hitbox_entity, hitbox_transform, mut hitbox) in &mut hitboxes {
        let hit_area = Aabb::new(
            (
                hitbox_transform.translation.x,
                hitbox_transform.translation.y,
            ),
            (hitbox.size.x, hitbox.size.y),
        );
        for (target, target_transform, hurtbox, profile, invulnerable) in &hurtboxes {
            if hurtbox.team == hitbox.team
                || hitbox.already_hit.contains(&target)
                || invulnerable.is_some_and(|i| i.remaining > 0.0)
            {
                continue;
            }
            let target_area = Aabb::new(
                (
                    target_transform.translation.x,
                    target_transform.translation.y,
                ),
                (hurtbox.size.x, hurtbox.size.y),
            );
            if !hit_area.intersects(&target_area) {
                continue;
            }
            let outcome = calculate_damage(
                &hitbox.attack,
                &profile.defense,
                &hitbox.hit,
                roll_damage(&mut rng),
            );
            let direction =
                (target_transform.translation.x - hitbox_transform.translation.x).signum();
            damage.write(DamageDealt {
                attacker: hitbox.owner,
                target,
                outcome,
                direction: if direction == 0.0 { 1.0 } else { direction },
            });
            hitbox.already_hit.push(target);
            if !hitbox.pierce {
                commands.entity(hitbox_entity).try_despawn();
                break;
            }
        }
    }
}
