//! D21: Ashen Sentinel encounter — spawn, lock, phase, defeat, persistence.

use bevy::prelude::*;
use dark_castle_rpg::combat::damage::DamageOutcome;
use dark_castle_rpg::combat::reaction::KnockbackIntensity;
use dark_castle_rpg::core::constants::DEFAULT_SAVE_SLOT;
use dark_castle_rpg::core::events::{Arrival, DamageDealt, RoomRequest};
use dark_castle_rpg::core::resources::GameData;
use dark_castle_rpg::enemies::boss::BossPhase;
use dark_castle_rpg::enemies::boss_systems::BossEncounter;
use dark_castle_rpg::enemies::components::{Boss, BossRuntime, Enemy, EnemyHealth};
use dark_castle_rpg::player::components::{Belongings, Player};
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::world::WorldState;
use dark_castle_rpg::world::transitions::TransitionLock;

#[path = "support/world_app.rs"]
mod world_app;

use world_app::*;

const COURT: &str = "vigil_gates_ashen_court";
const BOSS: &str = "ashen_sentinel";
const AT_COURT: Vec2 = Vec2::new(400.0, 63.0);

fn enter_court(app: &mut App) {
    app.world_mut().write_message(RoomRequest {
        room: COURT.into(),
        arrival: Arrival::Position(AT_COURT),
    });
    run(app, 4);
    assert_eq!(current(app), COURT);
}

fn boss_entity(app: &mut App) -> Entity {
    let world = app.world_mut();
    world
        .query_filtered::<Entity, With<Boss>>()
        .iter(world)
        .next()
        .unwrap_or_else(|| panic!("boss missing"))
}

fn boss_hp_ratio(app: &mut App) -> f32 {
    let world = app.world_mut();
    world
        .query_filtered::<&EnemyHealth, With<Boss>>()
        .iter(world)
        .next()
        .map(|h| h.ratio())
        .unwrap_or_else(|| panic!("boss hp"))
}

fn deal_to_boss(app: &mut App, amount: i32) {
    let target = boss_entity(app);
    let attacker = {
        let world = app.world_mut();
        world
            .query_filtered::<Entity, With<Player>>()
            .single(world)
            .unwrap_or_else(|e| panic!("{e}"))
    };
    app.world_mut().write_message(DamageDealt {
        attacker,
        target,
        outcome: DamageOutcome::Hit {
            amount,
            critical: false,
            element: dark_castle_rpg::magic::elements::Element::Physical,
        },
        direction: 1.0,
        knockback: KnockbackIntensity::default(),
    });
    run(app, 2);
}

fn bag_count(app: &mut App, item: &str) -> u32 {
    let world = app.world_mut();
    world
        .query_filtered::<&Belongings, With<Player>>()
        .single(world)
        .map(|b| b.inventory.count(item))
        .unwrap_or(0)
}

#[test]
fn ashen_court_spawns_boss_and_seals_exits() {
    let mut app = game(&storage("d21_spawn"));
    enter_court(&mut app);
    assert_eq!(
        {
            let world = app.world_mut();
            world
                .query::<&Enemy>()
                .iter(world)
                .filter(|e| e.definition == BOSS)
                .count()
        },
        1
    );
    assert!(app.world().resource::<BossEncounter>().active);
    run(&mut app, 2);
    assert!(app.world().resource::<TransitionLock>().locked);
}

#[test]
fn phase_shifts_below_half_hp() {
    let mut app = game(&storage("d21_phase"));
    enter_court(&mut app);
    let max = app
        .world()
        .resource::<GameData>()
        .enemies
        .get(BOSS)
        .map(|d| d.max_hp)
        .unwrap_or_else(|| panic!("def"));
    // Logic-only: both phase-1 tells already happened, then HP crosses 50%.
    {
        let world = app.world_mut();
        let mut q = world.query::<&mut BossRuntime>();
        let mut runtime = q.single_mut(world).unwrap_or_else(|e| panic!("{e}"));
        runtime.resolved_slash = true;
        runtime.resolved_lunge = true;
    }
    let need = (max as f32 * 0.51) as i32 + 1;
    deal_to_boss(&mut app, need);
    assert!(boss_hp_ratio(&mut app) < 0.5);
    let world = app.world_mut();
    let phase = world
        .query_filtered::<&BossRuntime, With<Boss>>()
        .iter(world)
        .next()
        .map(|r| r.phase)
        .unwrap_or_else(|| panic!("runtime"));
    assert_eq!(phase, BossPhase::Two);
}

#[test]
fn defeat_grants_reward_once_and_blocks_respawn() {
    let saves = storage("d21_defeat");
    let mut app = game(&saves);
    enter_court(&mut app);
    assert_eq!(bag_count(&mut app, "sentinel_brand"), 0);

    let max = app
        .world()
        .resource::<GameData>()
        .enemies
        .get(BOSS)
        .map(|d| d.max_hp)
        .unwrap_or_else(|| panic!("def"));
    deal_to_boss(&mut app, max + 10);
    run(&mut app, 3);

    assert!(!app.world().resource::<BossEncounter>().active);
    assert!(!app.world().resource::<TransitionLock>().locked);
    assert!(
        app.world()
            .resource::<WorldState>()
            .0
            .defeated_bosses
            .contains(BOSS)
    );
    assert!(
        app.world()
            .resource::<WorldState>()
            .0
            .has_flag("ashen_sentinel_fallen")
    );
    assert_eq!(bag_count(&mut app, "sentinel_brand"), 1);
    assert_eq!(
        {
            let world = app.world_mut();
            world
                .query_filtered::<Entity, With<Boss>>()
                .iter(world)
                .count()
        },
        0
    );

    // Persist and reload: boss must not return; reward must not duplicate.
    tap(&mut app, KeyCode::F5);
    run(&mut app, 2);
    enter_court(&mut app);
    assert_eq!(
        {
            let world = app.world_mut();
            world
                .query::<&Enemy>()
                .iter(world)
                .filter(|e| e.definition == BOSS)
                .count()
        },
        0
    );
    assert!(!app.world().resource::<BossEncounter>().active);
    assert_eq!(bag_count(&mut app, "sentinel_brand"), 1);

    let save = SaveStorage::new(saves.dir())
        .load(DEFAULT_SAVE_SLOT)
        .unwrap_or_else(|e| panic!("{e}"));
    assert!(save.world.defeated_bosses.contains(BOSS));
}

#[test]
fn player_death_clears_lock_and_despawns_boss_for_retry() {
    let mut app = game(&storage("d21_death"));
    enter_court(&mut app);
    assert!(app.world().resource::<BossEncounter>().active);

    // Force the player dead the same way respawn expects.
    {
        let world = app.world_mut();
        let mut q = world
            .query_filtered::<&mut dark_castle_rpg::player::components::ActionState, With<Player>>(
            );
        let mut state = q.single_mut(world).unwrap_or_else(|e| panic!("{e}"));
        state.die();
    }
    run(&mut app, 3);
    assert!(!app.world().resource::<BossEncounter>().active);
    assert!(!app.world().resource::<TransitionLock>().locked);
    assert_eq!(
        {
            let world = app.world_mut();
            world
                .query_filtered::<Entity, With<Boss>>()
                .iter(world)
                .count()
        },
        0
    );

    // Leave and return: full fight available again (same-room request
    // does not re-fire enemy spawn).
    app.world_mut().write_message(RoomRequest {
        room: SEALED.into(),
        arrival: Arrival::Position(Vec2::new(400.0, 63.0)),
    });
    run(&mut app, 4);
    // Revive so Playing systems run normally.
    {
        let world = app.world_mut();
        let mut q = world
            .query_filtered::<&mut dark_castle_rpg::player::components::ActionState, With<Player>>(
            );
        let mut state = q.single_mut(world).unwrap_or_else(|e| panic!("{e}"));
        *state = dark_castle_rpg::player::components::ActionState::default();
    }
    enter_court(&mut app);
    assert_eq!(
        {
            let world = app.world_mut();
            world
                .query::<&Enemy>()
                .iter(world)
                .filter(|e| e.definition == BOSS)
                .count()
        },
        1
    );
    assert!(boss_hp_ratio(&mut app) > 0.99);
}
