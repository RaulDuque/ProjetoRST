//! Executes AI decisions: movement, attacks, contact damage, taking damage
//! and dying. Never reads player input.

use bevy::prelude::*;

use super::ai::{AiProfile, EnemyBehavior, Perception, decide};
use super::boss::BossPattern;
use super::boss_systems::{advance_pattern_after_attack, combat_view};
use super::components::{
    BossRuntime, Enemy, EnemyBrain, EnemyHealth, PendingRespawns, SpawnOrigin,
};
use crate::combat::damage::{DamageKind, HitSpec, calculate_damage};
use crate::combat::hitboxes::{
    AttachedToOwner, CombatProfile, Hitbox, Hurtbox, Invulnerable, SimultaneousHitPolicy, Team,
    roll_damage,
};
use crate::combat::reaction::KnockbackIntensity;
use crate::core::constants::{ENEMY_HURT_FLASH, Z_EFFECTS};
use crate::core::events::{DamageDealt, EnemyDefeated};
use crate::core::resources::{GameData, GameRandom};
use crate::core::time::frame_delta;
use crate::magic::elements::Element;
use crate::magic::projectiles::Projectile;
use crate::player::components::{ActionState, Player};
use crate::player::state::{Facing, PlayerState};
use crate::world::collision::Aabb;
use crate::world::physics::{Body, FacingDirection, Grounded, Velocity};

/// Active time of a melee enemy attack hitbox.
const MELEE_ACTIVE: f32 = 0.15;
/// Lifetime floor for enemy projectiles (seconds).
const PROJECTILE_LIFE_MIN: f32 = 0.4;
const WINDUP_COLOR: Color = Color::srgb(1.0, 0.55, 0.2);
const WINDUP_LUNGE: Color = Color::srgb(1.0, 0.92, 0.25);
const WINDUP_WAVE: Color = Color::srgb(0.35, 0.85, 1.0);
const PHASE_LOCK_COLOR: Color = Color::srgb(0.72, 0.35, 0.95);
const ENEMY_ATTACK_COLOR: Color = Color::srgba(1.0, 0.3, 0.2, 0.35);
const LUNGE_ATTACK_COLOR: Color = Color::srgba(1.0, 0.85, 0.2, 0.55);
const ENEMY_BOLT_COLOR: Color = Color::srgba(0.55, 0.85, 1.0, 0.85);

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
            Option<&mut BossRuntime>,
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
        boss_runtime,
    ) in &mut enemies
    {
        let Some(def) = data.enemies.get(&enemy.definition) else {
            continue;
        };
        let boss_view = boss_runtime
            .as_ref()
            .and_then(|runtime| combat_view(def, runtime));
        let ai: AiProfile = boss_view.as_ref().map_or(def.ai, |v| v.ai);
        let attack_cooldown = boss_view
            .as_ref()
            .map_or(def.attack_cooldown, |v| v.attack_cooldown);
        let attack_hitbox = boss_view
            .as_ref()
            .map_or(def.attack_hitbox, |v| v.attack_hitbox);
        let uses_projectile = boss_view
            .as_ref()
            .map_or(def.ai.uses_projectile(), |v| v.uses_projectile);
        let projectile_speed = if uses_projectile {
            ai.projectile_speed.max(1.0)
        } else {
            0.0
        };

        brain.attack_cooldown = (brain.attack_cooldown - dt).max(0.0);
        brain.flash = (brain.flash - dt).max(0.0);
        let position = transform.translation.truncate();
        // A committed boss tell keeps counting through the hurt flash, so the
        // swing arrives while the tell color is still showing (D23). Ordinary
        // enemies stay stunned and keep only the knockback velocity.
        let boss_winding = boss_runtime.is_some() && brain.windup.is_some();
        if brain.flash > 0.0 && !boss_winding {
            continue;
        }
        if boss_runtime
            .as_ref()
            .is_some_and(|runtime| runtime.phase_lock > 0.0)
        {
            velocity.x = 0.0;
            continue;
        }

        // Finish a wind-up in progress before deciding anything else.
        if let Some(remaining) = brain.windup {
            let lunging = boss_runtime.as_ref().is_some_and(|runtime| runtime.lunge);
            if !lunging {
                velocity.x = 0.0;
            }
            let remaining = remaining - dt;
            if remaining > 0.0 {
                brain.windup = Some(remaining);
                continue;
            }
            brain.windup = None;
            brain.attack_cooldown = attack_cooldown;
            let size = Vec2::new(attack_hitbox.0, attack_hitbox.1);
            let facing_sign = match facing.0 {
                Facing::Right => 1.0,
                Facing::Left => -1.0,
            };
            let hit = HitSpec {
                kind: DamageKind::Physical,
                element: Element::Physical,
                base_damage: 0,
                multiplier: 1.0,
            };
            let swing_color = boss_runtime
                .as_ref()
                .and_then(|runtime| combat_view(def, runtime))
                .map(|view| match view.pattern {
                    BossPattern::Lunge => LUNGE_ATTACK_COLOR,
                    BossPattern::FrontalSlash | BossPattern::AshWave => ENEMY_ATTACK_COLOR,
                })
                .unwrap_or(ENEMY_ATTACK_COLOR);
            if uses_projectile {
                let speed = projectile_speed;
                let life = (ai.leash_range / speed).clamp(PROJECTILE_LIFE_MIN, 3.0);
                let start = position
                    + Vec2::new(facing_sign * (def.size.0 * 0.5 + size.x * 0.5 + 4.0), 4.0);
                commands.spawn((
                    Hitbox {
                        owner: entity,
                        team: Team::Enemy,
                        size,
                        attack: profile.physical.clone(),
                        hit,
                        lifetime: life,
                        pierce: false,
                        already_hit: Vec::new(),
                    },
                    Projectile {
                        velocity: Vec2::new(facing_sign * speed, 0.0),
                    },
                    Sprite::from_color(ENEMY_BOLT_COLOR, size),
                    Transform::from_translation(start.extend(Z_EFFECTS)),
                ));
            } else {
                commands.spawn((
                    Hitbox {
                        owner: entity,
                        team: Team::Enemy,
                        size,
                        attack: profile.physical.clone(),
                        hit,
                        lifetime: MELEE_ACTIVE,
                        pierce: true,
                        already_hit: Vec::new(),
                    },
                    AttachedToOwner {
                        offset: Vec2::new(def.size.0 / 2.0 + size.x / 2.0, 0.0),
                    },
                    Sprite::from_color(swing_color, size),
                    Transform::from_xyz(position.x, position.y, Z_EFFECTS),
                ));
            }
            if let Some(mut runtime) = boss_runtime {
                if let Some(view) = combat_view(def, &runtime) {
                    runtime.last_resolved = Some(view.pattern);
                    match view.pattern {
                        BossPattern::FrontalSlash => runtime.resolved_slash = true,
                        BossPattern::Lunge => runtime.resolved_lunge = true,
                        BossPattern::AshWave => {}
                    }
                }
                advance_pattern_after_attack(&mut runtime);
            }
            continue;
        }

        let perception = Perception {
            distance_to_player: target.map(|t| (t.x - position.x).abs()),
            hp_ratio: health.ratio(),
            currently: brain.behavior,
            attack_on_cooldown: brain.attack_cooldown > 0.0,
        };
        brain.behavior = decide(&ai, &perception);
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
                    let windup = ai.windup.max(0.05);
                    brain.windup = Some(windup);
                    if let Some(mut runtime) = boss_runtime {
                        let view = combat_view(def, &runtime);
                        runtime.lunge = view.as_ref().is_some_and(|v| v.lunge);
                        runtime.lunge_dir = if toward_player == 0.0 {
                            1.0
                        } else {
                            toward_player
                        };
                    }
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

/// Touching an enemy hurts the player. Under the legacy simultaneous-hit
/// policy only the first touching enemy produces a hit each frame; under
/// `HighestDamagePerFrame` every touching enemy is a candidate, so the
/// player's `receive_damage` can pick the strongest.
pub fn contact_damage(
    data: Res<GameData>,
    policy: Res<SimultaneousHitPolicy>,
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
            knockback: KnockbackIntensity::default(),
        });
        if *policy == SimultaneousHitPolicy::Legacy {
            // One contact hit per frame is enough; invulnerability follows.
            break;
        }
    }
}

/// Applies the frame's hits to enemies, as chosen by
/// [`SimultaneousHitPolicy`] (by default all of them, added up). Every
/// damaging hit flashes the enemy, cancels its wind-up and pushes it by the
/// hit's knockback intensity. Bosses keep a committed wind-up (D22) so the
/// telegraph can finish.
pub fn receive_damage(
    mut commands: Commands,
    data: Res<GameData>,
    policy: Res<SimultaneousHitPolicy>,
    mut hits: MessageReader<DamageDealt>,
    mut enemies: Query<(
        &Enemy,
        &mut EnemyHealth,
        &mut EnemyBrain,
        &mut Velocity,
        &SpawnOrigin,
        Option<&super::components::Boss>,
    )>,
    mut pending: ResMut<PendingRespawns>,
    mut defeated: MessageWriter<EnemyDefeated>,
) {
    let frame = policy.select(hits.read().copied().collect(), |hit| hit.outcome.amount());
    for hit in frame {
        let Ok((enemy, mut health, mut brain, mut velocity, origin, boss)) =
            enemies.get_mut(hit.target)
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
        if boss.is_none() {
            brain.windup = None;
        }
        velocity.x = hit.knockback.enemy_push(hit.direction);

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

pub fn update_enemy_visuals(mut enemies: Query<(&EnemyBrain, Option<&BossRuntime>, &mut Sprite)>) {
    for (brain, boss, mut sprite) in &mut enemies {
        // A boss tell outranks the hurt flash (D23). Ordinary enemies still
        // flash white, including over their own wind-up.
        let boss_tell = boss.and_then(|runtime| {
            if runtime.phase_lock > 0.0 {
                Some(PHASE_LOCK_COLOR)
            } else if brain.windup.is_some() {
                Some(
                    match super::boss::pattern_for_phase(runtime.phase, runtime.next_pattern) {
                        BossPattern::Lunge => WINDUP_LUNGE,
                        BossPattern::AshWave => WINDUP_WAVE,
                        BossPattern::FrontalSlash => WINDUP_COLOR,
                    },
                )
            } else {
                None
            }
        });
        sprite.color = if let Some(color) = boss_tell {
            color
        } else if brain.flash > 0.0 {
            Color::WHITE
        } else if brain.windup.is_some() {
            WINDUP_COLOR
        } else {
            brain.base_color
        };
    }
}
