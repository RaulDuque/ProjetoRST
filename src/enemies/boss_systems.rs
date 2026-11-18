//! Bevy systems for boss encounters (D21): arena lock, phases, rewards, HUD hook.

use bevy::prelude::*;

use super::boss::{
    BossPattern, BossPhase, LUNGE_SPEED, next_phase1_pattern, pattern_for_phase, pattern_hitbox,
    record_defeat, should_spawn_boss,
};
use super::components::{Boss, BossRuntime, Enemy, EnemyBrain, EnemyHealth};
use super::definitions::EnemyDefinition;
use crate::core::events::{EnemyDefeated, Notification};
use crate::core::resources::GameData;
use crate::core::time::frame_delta;
use crate::magic::projectiles::Projectile;
use crate::player::components::{ActionState, Belongings, Player};
use crate::player::state::PlayerState;
use crate::world::WorldState;
use crate::world::maps::CurrentRoom;
use crate::world::transitions::TransitionLock;

/// Active boss fight in the current room (doors sealed while `active`).
#[derive(Resource, Debug, Clone, Default)]
pub struct BossEncounter {
    pub active: bool,
    pub boss_id: Option<String>,
    pub display_name: String,
}

/// Clears enemy projectiles when an encounter ends.
fn clear_enemy_projectiles(commands: &mut Commands, projectiles: &Query<Entity, With<Projectile>>) {
    for entity in projectiles.iter() {
        commands.entity(entity).try_despawn();
    }
}

pub fn begin_encounter_on_boss_spawn(
    mut encounter: ResMut<BossEncounter>,
    data: Res<GameData>,
    bosses: Query<&Boss, Added<Boss>>,
) {
    for boss in &bosses {
        let name = data
            .enemies
            .get(&boss.id)
            .map(|d| d.name.clone())
            .unwrap_or_else(|| boss.id.clone());
        encounter.active = true;
        encounter.boss_id = Some(boss.id.clone());
        encounter.display_name = name;
    }
}

/// Hold the transition lock for the whole fight (rearm_doors cannot open exits).
pub fn seal_arena_while_fighting(encounter: Res<BossEncounter>, mut lock: ResMut<TransitionLock>) {
    if encounter.active {
        lock.locked = true;
    }
}

pub fn update_boss_phase(
    time: Res<Time>,
    data: Res<GameData>,
    mut bosses: Query<(&Enemy, &EnemyHealth, &mut BossRuntime, &mut EnemyBrain)>,
    mut notifications: MessageWriter<Notification>,
) {
    let dt = frame_delta(&time);
    for (enemy, health, mut runtime, mut brain) in &mut bosses {
        let Some(def) = data.enemies.get(&enemy.definition) else {
            continue;
        };
        let Some(boss) = &def.boss else {
            continue;
        };
        runtime.phase_lock = (runtime.phase_lock - dt).max(0.0);
        let next = BossPhase::from_hp(health.ratio(), boss.phase_threshold);
        // Stay in phase 1 until both melee tells have landed, unless HP is
        // already critical (so a burst cannot skip the lunge).
        let phase_ready = runtime.resolved_slash && runtime.resolved_lunge || health.ratio() < 0.25;
        if next == BossPhase::Two && runtime.phase == BossPhase::One && phase_ready {
            runtime.phase = BossPhase::Two;
            runtime.next_pattern = BossPattern::AshWave;
            runtime.phase_lock = 0.85;
            runtime.lunge = false;
            brain.windup = None;
            brain.attack_cooldown = boss.phase2_attack_cooldown;
            brain.base_color = Color::srgb(
                boss.phase2_color.0,
                boss.phase2_color.1,
                boss.phase2_color.2,
            );
            notifications.write(Notification(format!("{} enters a second form", def.name)));
        }
    }
}

/// Effective combat numbers for the boss's current phase.
pub struct BossCombatView {
    pub ai: super::ai::AiProfile,
    pub attack_cooldown: f32,
    pub attack_hitbox: (f32, f32),
    pub pattern: BossPattern,
    pub uses_projectile: bool,
    pub lunge: bool,
}

pub fn combat_view(def: &EnemyDefinition, runtime: &BossRuntime) -> Option<BossCombatView> {
    let boss = def.boss.as_ref()?;
    let pattern = pattern_for_phase(runtime.phase, runtime.next_pattern);
    let (ai, cooldown, hitbox) = match runtime.phase {
        BossPhase::One => (def.ai, def.attack_cooldown, def.attack_hitbox),
        BossPhase::Two => (
            boss.phase2_ai,
            boss.phase2_attack_cooldown,
            boss.phase2_attack_hitbox,
        ),
    };
    let size = pattern_hitbox(pattern, def.attack_hitbox, hitbox);
    Some(BossCombatView {
        ai,
        attack_cooldown: cooldown,
        attack_hitbox: size,
        pattern,
        uses_projectile: matches!(pattern, BossPattern::AshWave) || ai.uses_projectile(),
        lunge: matches!(pattern, BossPattern::Lunge),
    })
}

pub fn advance_pattern_after_attack(runtime: &mut BossRuntime) {
    runtime.next_pattern = match runtime.phase {
        BossPhase::One => next_phase1_pattern(runtime.next_pattern),
        BossPhase::Two => BossPattern::AshWave,
    };
    runtime.lunge = false;
}

pub fn on_boss_defeated(
    mut encounter: ResMut<BossEncounter>,
    mut world: ResMut<WorldState>,
    data: Res<GameData>,
    mut defeated: MessageReader<EnemyDefeated>,
    mut player: Query<&mut Belongings, With<Player>>,
    mut notifications: MessageWriter<Notification>,
    mut commands: Commands,
    projectiles: Query<Entity, With<Projectile>>,
    mut lock: ResMut<TransitionLock>,
) {
    for event in defeated.read() {
        let Some(def) = data.enemies.get(&event.enemy_id) else {
            continue;
        };
        let Some(boss) = &def.boss else {
            continue;
        };
        let first = record_defeat(&mut world.0, &def.id, boss.reward_flag.as_deref());
        if first {
            if let Some(item) = &boss.reward_item
                && let Ok(mut belongings) = player.single_mut()
            {
                match belongings.inventory.add(item, 1, &data.items) {
                    Ok(()) => {
                        let name = data
                            .items
                            .get(item)
                            .map_or(item.as_str(), |i| i.name.as_str());
                        notifications.write(Notification(format!("Claimed {name}")));
                    }
                    Err(error) => warn!(%error, "boss reward refused"),
                }
            }
            notifications.write(Notification(format!("{} falls", def.name)));
        }
        encounter.active = false;
        encounter.boss_id = None;
        lock.locked = false;
        clear_enemy_projectiles(&mut commands, &projectiles);
    }
}

pub fn end_encounter_on_player_death(
    mut encounter: ResMut<BossEncounter>,
    player: Query<&ActionState, With<Player>>,
    mut commands: Commands,
    projectiles: Query<Entity, With<Projectile>>,
    mut lock: ResMut<TransitionLock>,
    bosses: Query<Entity, With<Boss>>,
) {
    if !encounter.active {
        return;
    }
    let Ok(state) = player.single() else {
        return;
    };
    if state.state() != PlayerState::Dead {
        return;
    }
    encounter.active = false;
    encounter.boss_id = None;
    lock.locked = false;
    clear_enemy_projectiles(&mut commands, &projectiles);
    // Remove the live boss so a fresh full-HP fight starts on re-entry.
    for entity in &bosses {
        commands.entity(entity).try_despawn();
    }
}

pub fn end_encounter_on_room_leave(
    mut encounter: ResMut<BossEncounter>,
    room: Option<Res<CurrentRoom>>,
    mut lock: ResMut<TransitionLock>,
    bosses: Query<(), With<Boss>>,
    mut last: Local<Option<String>>,
) {
    let Some(room) = room else {
        return;
    };
    if last.as_ref() != Some(&room.0) {
        // Entering the arena spawns a boss the same frame — keep the fight.
        // Only clear a stale seal when no boss entity remains after a change.
        if encounter.active && bosses.is_empty() {
            encounter.active = false;
            encounter.boss_id = None;
            lock.locked = false;
        }
        *last = Some(room.0.clone());
    }
}

/// Filter used by spawning: bosses already defeated stay gone.
pub fn may_spawn(progress: &crate::world::progress::WorldProgress, def: &EnemyDefinition) -> bool {
    should_spawn_boss(progress, &def.id, def.is_boss())
}

/// Applies lunge velocity while a boss wind-up is marked as lunge.
pub fn apply_lunge_velocity(
    mut bosses: Query<(
        &BossRuntime,
        &EnemyBrain,
        &mut crate::world::physics::Velocity,
    )>,
) {
    for (runtime, brain, mut velocity) in &mut bosses {
        if runtime.lunge && brain.windup.is_some() {
            let dir = if runtime.lunge_dir == 0.0 {
                1.0
            } else {
                runtime.lunge_dir.signum()
            };
            velocity.x = dir * LUNGE_SPEED;
        }
    }
}
