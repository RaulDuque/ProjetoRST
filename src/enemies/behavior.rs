//! Executes AI decisions: movement, attacks, contact damage, taking damage
//! and dying. Never reads player input.

use bevy::prelude::*;

use super::ai::{EnemyBehavior, Perception, decide};
use super::components::{Enemy, EnemyBrain, EnemyHealth, PendingRespawns, SpawnOrigin};
use crate::combat::damage::{DamageKind, HitSpec, calculate_damage};
use crate::combat::hitboxes::{
    AttachedToOwner, CombatProfile, Hitbox, Hurtbox, Invulnerable, Team, roll_damage,
};
use crate::core::constants::{ENEMY_HURT_FLASH, ENEMY_KNOCKBACK, Z_EFFECTS};
use crate::core::events::{DamageDealt, EnemyDefeated};
use crate::core::resources::{GameData, GameRandom};
use crate::core::time::frame_delta;
use crate::magic::elements::Element;
use crate::player::components::{ActionState, Player};
use crate::player::state::{Facing, PlayerState};
use crate::world::collision::Aabb;
use crate::world::physics::{Body, FacingDirection, Grounded, Velocity};

/// Seconds of telegraph before an enemy attack lands.
const ATTACK_WINDUP: f32 = 0.35;
/// Active time of an enemy attack hitbox.
const ATTACK_ACTIVE: f32 = 0.15;
const WINDUP_COLOR: Color = Color::srgb(1.0, 0.55, 0.2);
const ENEMY_ATTACK_COLOR: Color = Color::srgba(1.0, 0.3, 0.2, 0.35);

type PlayerView<'a> = (&'a Transform, &'a ActionState);

pub fn think_and_move(
    mut commands: Commands,
    time: Res<Time>,
    data: Res<GameData>,
    player: Query<PlayerView, (With<Player>, Without<Enemy>)>,
    mut enemies: Query<
        (
            Entity,
            &Enemy,
            &EnemyHealth,
            &mut EnemyBrain,
            &mut Velocity,
            &mut FacingDirection,
            &Transform,
            &Grounded,
            &CombatProfile,
        ),
        Without<Player>,
    >,
) {
    let dt = frame_delta(&time);
    let target = player
        .single()
        .ok()
        .filter(|(_, state)| state.state() != PlayerState::Dead)
        .map(|(t, _)| t.translation.truncate());

    for (
        entity,
        enemy,
        health,
        mut brain,
        mut velocity,
        mut facing,
        transform,
        grounded,
        profile,
    ) in &mut enemies
    {
        let Some(def) = data.enemies.get(&enemy.definition) else {
            continue;
        };
        brain.attack_cooldown = (brain.attack_cooldown - dt).max(0.0);
        brain.flash = (brain.flash - dt).max(0.0);
        let position = transform.translation.truncate();
        // Briefly stunned after a hit: keep the knockback velocity.
        if brain.flash > 0.0 {
            continue;
        }

        // Finish a wind-up in progress before deciding anything else.
        if let Some(remaining) = brain.windup {
            velocity.x = 0.0;
            let remaining = remaining - dt;
            if remaining > 0.0 {
                brain.windup = Some(remaining);
                continue;
            }
            brain.windup = None;
            brain.attack_cooldown = def.attack_cooldown;
            let size = Vec2::new(def.attack_hitbox.0, def.attack_hitbox.1);
            commands.spawn((
                Hitbox {
                    owner: entity,
                    team: Team::Enemy,
                    size,
                    attack: profile.physical.clone(),
                    hit: HitSpec {
                        kind: DamageKind::Physical,
                        element: Element::Physical,
                        base_damage: 0,
                        multiplier: 1.0,
                    },
                    lifetime: ATTACK_ACTIVE,
                    pierce: true,
                    already_hit: Vec::new(),
                },
                AttachedToOwner {
                    offset: Vec2::new(def.size.0 / 2.0 + size.x / 2.0, 0.0),
                },
                Sprite::from_color(ENEMY_ATTACK_COLOR, size),
                Transform::from_xyz(position.x, position.y, Z_EFFECTS),
            ));
            continue;
        }

        let perception = Perception {
            distance_to_player: target.map(|t| (t.x - position.x).abs()),
            hp_ratio: health.ratio(),
            currently: brain.behavior,
            attack_on_cooldown: brain.attack_cooldown > 0.0,
        };
        brain.behavior = decide(&def.ai, &perception);
        let toward_player = target.map_or(0.0, |t| (t.x - position.x).signum());

        match brain.behavior {
            EnemyBehavior::Patrol => {
                if position.x > brain.home_x + brain.patrol_half_width {
                    brain.patrol_direction = -1.0;
                } else if position.x < brain.home_x - brain.patrol_half_width {
                    brain.patrol_direction = 1.0;
                }
                velocity.x = brain.patrol_direction * def.move_speed * 0.6;
            }
            EnemyBehavior::Chase => velocity.x = toward_player * def.move_speed,
            EnemyBehavior::Flee | EnemyBehavior::Retreat => {
                velocity.x = -toward_player * def.move_speed;
            }
            EnemyBehavior::Attack => {
                velocity.x = 0.0;
                if grounded.on_ground {
                    brain.windup = Some(ATTACK_WINDUP);
                }
            }
            EnemyBehavior::Wait | EnemyBehavior::Defend | EnemyBehavior::UseSkill => {
                velocity.x = 0.0;
            }
        }

        let look = if matches!(brain.behavior, EnemyBehavior::Attack | EnemyBehavior::Wait) {
            toward_player
        } else {
            velocity.x.signum()
        };
        if look > 0.0 {
            facing.0 = Facing::Right;
        } else if look < 0.0 {
            facing.0 = Facing::Left;
        }
    }
}

/// Touching an enemy hurts the player.
pub fn contact_damage(
    data: Res<GameData>,
    mut rng: ResMut<GameRandom>,
    player: Query<(Entity, &Transform, &Hurtbox, &CombatProfile, &Invulnerable), With<Player>>,
    enemies: Query<(Entity, &Enemy, &Transform, &Body, &CombatProfile)>,
    mut damage: MessageWriter<DamageDealt>,
) {
    let Ok((target, target_transform, hurtbox, target_profile, invulnerable)) = player.single()
    else {
        return;
    };
    if invulnerable.remaining > 0.0 {
        return;
    }
    let player_area = Aabb::new(
        (
            target_transform.translation.x,
            target_transform.translation.y,
        ),
        (hurtbox.size.x, hurtbox.size.y),
    );
    for (entity, enemy, transform, body, profile) in &enemies {
        let Some(def) = data.enemies.get(&enemy.definition) else {
            continue;
        };
        if def.contact_damage_multiplier <= 0.0
            || !body
                .aabb_at(transform.translation.truncate())
                .intersects(&player_area)
        {
            continue;
        }
        let hit = HitSpec {
            kind: DamageKind::Physical,
            element: Element::Physical,
            base_damage: 0,
            multiplier: def.contact_damage_multiplier,
        };
        let outcome = calculate_damage(
            &profile.physical,
            &target_profile.defense,
            &hit,
            roll_damage(&mut rng),
        );
        let direction = (target_transform.translation.x - transform.translation.x).signum();
        damage.write(DamageDealt {
            attacker: entity,
            target,
            outcome,
            direction: if direction == 0.0 { 1.0 } else { direction },
        });
        // One contact hit per frame is enough; invulnerability follows.
        break;
    }
}

pub fn receive_damage(
    mut commands: Commands,
    data: Res<GameData>,
    mut hits: MessageReader<DamageDealt>,
    mut enemies: Query<(
        &Enemy,
        &mut EnemyHealth,
        &mut EnemyBrain,
        &mut Velocity,
        &SpawnOrigin,
    )>,
    mut pending: ResMut<PendingRespawns>,
    mut defeated: MessageWriter<EnemyDefeated>,
) {
    for hit in hits.read() {
        let Ok((enemy, mut health, mut brain, mut velocity, origin)) = enemies.get_mut(hit.target)
        else {
            continue;
        };
        if health.is_empty() {
            continue;
        }
        let amount = hit.outcome.amount();
        if amount <= 0 {
            continue;
        }
        health.deplete(amount);
        brain.flash = ENEMY_HURT_FLASH;
        brain.windup = None;
        velocity.x = hit.direction * ENEMY_KNOCKBACK;

        if health.is_empty() {
            let Some(def) = data.enemies.get(&enemy.definition) else {
                continue;
            };
            defeated.write(EnemyDefeated {
                enemy_id: def.id.clone(),
                experience: def.experience,
                drops: def.drops.clone(),
            });
            if def.respawn_time > 0.0 {
                pending.0.push((origin.0.clone(), def.respawn_time));
            }
            commands.entity(hit.target).try_despawn();
        }
    }
}

pub fn update_enemy_visuals(mut enemies: Query<(&EnemyBrain, &mut Sprite)>) {
    for (brain, mut sprite) in &mut enemies {
        sprite.color = if brain.flash > 0.0 {
            Color::WHITE
        } else if brain.windup.is_some() {
            WINDUP_COLOR
        } else {
            brain.base_color
        };
    }
}
