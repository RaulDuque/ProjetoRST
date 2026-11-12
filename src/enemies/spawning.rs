//! Spawning (and respawning) enemies from room data.

use bevy::prelude::*;

use super::ai::EnemyBehavior;
use super::components::{Enemy, EnemyBrain, EnemyHealth, PendingRespawns, SpawnOrigin};
use super::definitions::EnemyDefinition;
use crate::combat::damage::{AttackProfile, DefenseProfile};
use crate::combat::hitboxes::{CombatProfile, Hurtbox, Team};
use crate::core::constants::Z_ENEMIES;
use crate::core::resources::GameData;
use crate::core::time::frame_delta;
use crate::player::attributes::ResourcePool;
use crate::world::maps::RoomEntity;
use crate::world::physics::{AffectedByGravity, Body, FacingDirection, Grounded, Velocity};
use crate::world::rooms::EnemySpawn;

/// Enemies have a flat critical profile until data needs more.
const ENEMY_CRITICAL_CHANCE: f32 = 0.03;
const ENEMY_CRITICAL_DAMAGE: f32 = 1.5;

pub fn combat_profile(def: &EnemyDefinition) -> CombatProfile {
    let attack = AttackProfile {
        power: def.attack,
        hit: def.hit,
        critical_chance: ENEMY_CRITICAL_CHANCE,
        critical_damage: ENEMY_CRITICAL_DAMAGE,
    };
    CombatProfile {
        physical: attack.clone(),
        magical: attack,
        defense: DefenseProfile {
            defense: def.defense,
            magic_defense: def.magic_defense,
            dodge: def.dodge,
            resistances: def.resistances.clone(),
        },
    }
}

pub fn spawn_enemy(commands: &mut Commands, def: &EnemyDefinition, spawn: &EnemySpawn) {
    let size = Vec2::new(def.size.0, def.size.1);
    let color = Color::srgb(def.color.0, def.color.1, def.color.2);
    commands.spawn((
        (
            Enemy {
                definition: def.id.clone(),
            },
            EnemyHealth(ResourcePool::full(def.max_hp)),
            EnemyBrain {
                behavior: EnemyBehavior::Patrol,
                home_x: spawn.position.0,
                patrol_half_width: spawn.patrol_half_width,
                patrol_direction: 1.0,
                attack_cooldown: 0.0,
                windup: None,
                flash: 0.0,
                base_color: color,
            },
            SpawnOrigin(spawn.clone()),
            combat_profile(def),
            RoomEntity,
        ),
        (
            Body { size },
            Velocity::default(),
            Grounded::default(),
            AffectedByGravity,
            FacingDirection::default(),
            Hurtbox {
                team: Team::Enemy,
                size,
            },
            Sprite::from_color(color, size),
            Transform::from_xyz(spawn.position.0, spawn.position.1, Z_ENEMIES),
        ),
    ));
}

pub fn spawn_room_enemies(mut commands: Commands, data: Res<GameData>) {
    let Some(room) = data.rooms.get(&data.character.start.room) else {
        return;
    };
    for spawn in &room.enemies {
        if let Some(def) = data.enemies.get(&spawn.enemy) {
            spawn_enemy(&mut commands, def, spawn);
        }
    }
}

pub fn process_respawns(
    mut commands: Commands,
    time: Res<Time>,
    data: Res<GameData>,
    mut pending: ResMut<PendingRespawns>,
) {
    let dt = frame_delta(&time);
    let mut ready = Vec::new();
    pending.0.retain_mut(|(spawn, timer)| {
        *timer -= dt;
        if *timer <= 0.0 {
            ready.push(spawn.clone());
            false
        } else {
            true
        }
    });
    for spawn in ready {
        if let Some(def) = data.enemies.get(&spawn.enemy) {
            spawn_enemy(&mut commands, def, &spawn);
        }
    }
}
