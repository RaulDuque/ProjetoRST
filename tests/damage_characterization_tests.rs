//! V2.4 B2, step 0 — characterization of how damage is applied today.
//!
//! The formula itself (physical/magical defence, resistances, criticals,
//! misses, minimum damage) is pinned by `combat_tests.rs`, and the player's
//! side of a frame with several hits (first damaging hit wins, misses and
//! zero damage do not count, death discards the rest) by
//! `app_smoke_tests.rs`. This file pins what those do not: the enemy side,
//! the producers (strike hitboxes, enemy hitboxes, contact, projectiles),
//! knockback against damage size and death through damage, so the
//! simultaneous-hit policy and knockback intensities (steps 6 and 7) can
//! prove their defaults change nothing.
//!
//! Timing: fixed 16 ms frames. Injected `DamageDealt` messages are written
//! in `GameSet::Combat`, like real ones, and applied in `GameSet::Resolution`
//! of the same frame, after physics, so velocities read after the frame are
//! the knockback itself.

use std::path::Path;
use std::time::Duration;

use bevy::input::{InputPlugin as BevyInputPlugin, InputSystems};
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::app::plugins::GamePlugin;
use dark_castle_rpg::app::states::GameSet;
use dark_castle_rpg::combat::damage::{DamageKind, DamageOutcome, HitSpec};
use dark_castle_rpg::combat::hitboxes::{CombatProfile, Hitbox, Hurtbox, Invulnerable, Team};
use dark_castle_rpg::core::constants::{
    ENEMY_HURT_FLASH, ENEMY_KNOCKBACK, GUARD_DAMAGE_REDUCTION, PLAYER_HURT_KNOCKBACK,
    PLAYER_INVULNERABILITY,
};
use dark_castle_rpg::core::events::{DamageDealt, EnemyDefeated};
use dark_castle_rpg::core::resources::GameData;
use dark_castle_rpg::enemies::components::{Enemy, EnemyBrain, EnemyHealth, PendingRespawns};
use dark_castle_rpg::magic::elements::Element;
use dark_castle_rpg::magic::projectiles::Projectile;
use dark_castle_rpg::player::attributes::RegenAccumulator;
use dark_castle_rpg::player::components::{ActionState, Player, Vitals};
use dark_castle_rpg::player::state::PlayerState;
use dark_castle_rpg::world::physics::Velocity;

const FRAME: Duration = Duration::from_millis(16);
/// Hit rating that always connects (`hit_chance` is capped at 1).
const SURE_HIT: i32 = 1000;

// ---------------------------------------------------------------- harness

#[derive(Resource, Default)]
struct PendingKeys(Vec<KeyCode>);

fn apply_pending_keys(mut pending: ResMut<PendingKeys>, mut keys: ResMut<ButtonInput<KeyCode>>) {
    for key in pending.0.drain(..) {
        keys.press(key);
    }
}

/// Hits queued by a test: `(attacker, target, amount, direction)`.
#[derive(Resource, Default)]
struct PendingHits(Vec<(Entity, Entity, i32, f32)>);

fn emit_pending_hits(mut pending: ResMut<PendingHits>, mut damage: MessageWriter<DamageDealt>) {
    for (attacker, target, amount, direction) in pending.0.drain(..) {
        damage.write(DamageDealt {
            attacker,
            target,
            outcome: hit(amount),
            direction,
            knockback: Default::default(),
        });
    }
}

/// Every `DamageDealt` of the last frame, in message order.
#[derive(Resource, Default)]
struct Seen {
    hits: Vec<DamageDealt>,
    defeats: usize,
}

fn record(
    mut seen: ResMut<Seen>,
    mut damage: MessageReader<DamageDealt>,
    mut defeated: MessageReader<EnemyDefeated>,
) {
    seen.hits = damage.read().copied().collect();
    seen.defeats += defeated.read().count();
}

fn hit(amount: i32) -> DamageOutcome {
    if amount < 0 {
        return DamageOutcome::Miss;
    }
    DamageOutcome::Hit {
        amount,
        critical: false,
        element: Element::Physical,
    }
}

/// The real game (start room, player idle at the spawn, the room's two
/// bone sentries far away) after a warm-up.
fn app() -> App {
    let data = GameData::load(Path::new(env!("CARGO_MANIFEST_DIR")))
        .unwrap_or_else(|e| panic!("game data failed to load: {e}"));
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatesPlugin, BevyInputPlugin))
        .insert_resource(TimeUpdateStrategy::ManualDuration(FRAME))
        .insert_resource(data)
        .init_resource::<PendingKeys>()
        .init_resource::<PendingHits>()
        .init_resource::<Seen>()
        .add_systems(PreUpdate, apply_pending_keys.after(InputSystems))
        .add_systems(Update, emit_pending_hits.in_set(GameSet::Combat))
        .add_systems(Update, record.after(GameSet::Presentation))
        .add_plugins(GamePlugin);
    run(&mut app, 60);
    assert_eq!(state(&mut app), PlayerState::Idle, "warm-up must end idle");
    app
}

fn run(app: &mut App, frames: usize) {
    for _ in 0..frames {
        app.update();
    }
}

fn player(app: &mut App) -> Entity {
    let world = app.world_mut();
    world
        .query_filtered::<Entity, With<Player>>()
        .single(world)
        .unwrap_or_else(|e| panic!("player missing: {e}"))
}

fn state(app: &mut App) -> PlayerState {
    let world = app.world_mut();
    world
        .query_filtered::<&ActionState, With<Player>>()
        .single(world)
        .map(|s| s.state())
        .unwrap_or_else(|e| panic!("player missing: {e}"))
}

fn player_hp(app: &mut App) -> i32 {
    let world = app.world_mut();
    let mut vitals = world
        .query_filtered::<&mut Vitals, With<Player>>()
        .single_mut(world)
        .unwrap_or_else(|e| panic!("player missing: {e}"));
    // No regeneration may blur the next frame's damage.
    vitals.hp_regen = RegenAccumulator::default();
    vitals.mp_regen = RegenAccumulator::default();
    vitals.hp.current()
}

fn player_position(app: &mut App) -> Vec3 {
    let world = app.world_mut();
    world
        .query_filtered::<&Transform, With<Player>>()
        .single(world)
        .map(|t| t.translation)
        .unwrap_or_else(|e| panic!("player missing: {e}"))
}

fn velocity_of(app: &App, entity: Entity) -> Vec2 {
    app.world()
        .get::<Velocity>(entity)
        .map(|v| v.0)
        .unwrap_or_else(|| panic!("{entity:?} has no velocity"))
}

/// The room's enemies, left to right.
fn enemies(app: &mut App) -> Vec<Entity> {
    let world = app.world_mut();
    let mut found: Vec<(Entity, f32)> = world
        .query_filtered::<(Entity, &Transform), With<Enemy>>()
        .iter(world)
        .map(|(e, t)| (e, t.translation.x))
        .collect();
    found.sort_by(|a, b| a.1.total_cmp(&b.1));
    found.into_iter().map(|(e, _)| e).collect()
}

fn enemy_hp(app: &App, enemy: Entity) -> i32 {
    app.world()
        .get::<EnemyHealth>(enemy)
        .map(|h| h.current())
        .unwrap_or_else(|| panic!("{enemy:?} has no health"))
}

fn brain(app: &App, enemy: Entity) -> EnemyBrain {
    app.world()
        .get::<EnemyBrain>(enemy)
        .cloned()
        .unwrap_or_else(|| panic!("{enemy:?} has no brain"))
}

fn inject(app: &mut App, hits: &[(Entity, Entity, i32, f32)]) {
    app.world_mut()
        .resource_mut::<PendingHits>()
        .0
        .extend_from_slice(hits);
}

fn seen(app: &App) -> Vec<DamageDealt> {
    app.world().resource::<Seen>().hits.clone()
}

/// An enemy-team hitbox (or projectile) of `power` at `at`.
fn spawn_hitbox(app: &mut App, owner: Entity, team: Team, power: i32, at: Vec3) -> Entity {
    let world = app.world_mut();
    let mut attack = world
        .query_filtered::<&CombatProfile, With<Enemy>>()
        .iter(world)
        .next()
        .map(|p| p.physical.clone())
        .unwrap_or_else(|| panic!("no enemy in the start room"));
    attack.power = power;
    attack.hit = SURE_HIT;
    attack.critical_chance = 0.0;
    world
        .spawn((
            Hitbox {
                owner,
                team,
                size: Vec2::splat(20.0),
                attack,
                hit: HitSpec {
                    kind: DamageKind::Physical,
                    element: Element::Physical,
                    base_damage: 0,
                    multiplier: 1.0,
                },
                lifetime: 1.0,
                pierce: true,
                already_hit: Vec::new(),
            },
            Transform::from_translation(at),
        ))
        .id()
}

/// A hurtbox of the enemy team with no `Enemy` marker (no AI, no death).
fn spawn_dummy(app: &mut App, at: Vec3) -> Entity {
    let world = app.world_mut();
    let profile = world
        .query_filtered::<&CombatProfile, With<Player>>()
        .single(world)
        .cloned()
        .unwrap_or_else(|e| panic!("player missing: {e}"));
    world
        .spawn((
            Hurtbox {
                team: Team::Enemy,
                size: Vec2::splat(60.0),
            },
            profile,
            Transform::from_translation(at),
        ))
        .id()
}

// ---------------------------------------------------------------- enemy side

#[test]
fn an_enemy_takes_every_hit_of_a_frame() {
    let mut app = app();
    let attacker = player(&mut app);
    let enemy = enemies(&mut app)[0];
    let hp = enemy_hp(&app, enemy);
    inject(
        &mut app,
        &[(attacker, enemy, 10, 1.0), (attacker, enemy, 7, 1.0)],
    );
    run(&mut app, 1);
    assert_eq!(enemy_hp(&app, enemy), hp - 17, "no per-frame protection");
    let brain = brain(&app, enemy);
    assert_eq!(brain.flash, ENEMY_HURT_FLASH);
    assert_eq!(brain.windup, None);
    assert_eq!(velocity_of(&app, enemy).x, ENEMY_KNOCKBACK);
}

#[test]
fn an_enemy_has_no_invulnerability_between_frames() {
    let mut app = app();
    let attacker = player(&mut app);
    let enemy = enemies(&mut app)[0];
    assert!(app.world().get::<Invulnerable>(enemy).is_none());
    let hp = enemy_hp(&app, enemy);
    inject(&mut app, &[(attacker, enemy, 5, 1.0)]);
    run(&mut app, 1);
    inject(&mut app, &[(attacker, enemy, 5, 1.0)]);
    run(&mut app, 1);
    assert_eq!(enemy_hp(&app, enemy), hp - 10);
}

#[test]
fn enemy_knockback_ignores_damage_size_and_follows_the_last_hit() {
    let mut app = app();
    let attacker = player(&mut app);
    let [weak, strong] = enemies(&mut app)[..] else {
        panic!("the start room should have two enemies");
    };
    inject(
        &mut app,
        &[(attacker, weak, 1, 1.0), (attacker, strong, 40, -1.0)],
    );
    run(&mut app, 1);
    assert_eq!(velocity_of(&app, weak).x, ENEMY_KNOCKBACK);
    assert_eq!(velocity_of(&app, strong).x, -ENEMY_KNOCKBACK);
    // Two hits from opposite sides in one frame: the last one sets it.
    inject(
        &mut app,
        &[(attacker, weak, 1, 1.0), (attacker, weak, 1, -1.0)],
    );
    run(&mut app, 1);
    assert_eq!(velocity_of(&app, weak).x, -ENEMY_KNOCKBACK);
}

#[test]
fn misses_and_zero_damage_leave_an_enemy_untouched() {
    let mut app = app();
    let attacker = player(&mut app);
    let enemy = enemies(&mut app)[0];
    let hp = enemy_hp(&app, enemy);
    inject(
        &mut app,
        &[(attacker, enemy, -1, 1.0), (attacker, enemy, 0, 1.0)],
    );
    run(&mut app, 1);
    assert_eq!(enemy_hp(&app, enemy), hp);
    assert_eq!(brain(&app, enemy).flash, 0.0);
}

#[test]
fn a_lethal_frame_defeats_an_enemy_once() {
    let mut app = app();
    let attacker = player(&mut app);
    let enemy = enemies(&mut app)[0];
    let hp = enemy_hp(&app, enemy);
    let respawns = app.world().resource::<PendingRespawns>().0.len();
    inject(
        &mut app,
        &[(attacker, enemy, hp, 1.0), (attacker, enemy, hp, 1.0)],
    );
    run(&mut app, 1);
    assert!(app.world().get_entity(enemy).is_err(), "despawned");
    assert_eq!(app.world().resource::<Seen>().defeats, 1);
    assert_eq!(
        app.world().resource::<PendingRespawns>().0.len(),
        respawns + 1
    );
}

#[test]
fn a_strike_hitbox_damages_an_enemy_by_the_reported_amount() {
    let mut app = app();
    let owner = player(&mut app);
    let enemy = enemies(&mut app)[0];
    let hp = enemy_hp(&app, enemy);
    let at = app
        .world()
        .get::<Transform>(enemy)
        .map(|t| t.translation)
        .unwrap_or_else(|| panic!("enemy without transform"));
    spawn_hitbox(&mut app, owner, Team::Player, 30, at);
    run(&mut app, 1);
    let hits: Vec<_> = seen(&app)
        .into_iter()
        .filter(|h| h.target == enemy)
        .collect();
    assert_eq!(hits.len(), 1, "one hit per target per hitbox");
    assert!(hits[0].outcome.amount() > 0);
    assert_eq!(enemy_hp(&app, enemy), hp - hits[0].outcome.amount());
    run(&mut app, 5);
    assert_eq!(
        enemy_hp(&app, enemy),
        hp - hits[0].outcome.amount(),
        "a hitbox never hits the same target twice"
    );
}

// ---------------------------------------------------------------- player side

#[test]
fn two_enemy_hitboxes_on_the_player_in_one_frame_apply_only_the_first() {
    let mut app = app();
    let target = player(&mut app);
    let [first_enemy, second_enemy] = enemies(&mut app)[..] else {
        panic!("the start room should have two enemies");
    };
    let at = player_position(&mut app);
    spawn_hitbox(&mut app, first_enemy, Team::Enemy, 10, at);
    spawn_hitbox(&mut app, second_enemy, Team::Enemy, 60, at);
    let hp = player_hp(&mut app);
    run(&mut app, 1);
    let hits: Vec<_> = seen(&app)
        .into_iter()
        .filter(|h| h.target == target)
        .collect();
    assert_eq!(hits.len(), 2, "both hitboxes connect: {hits:?}");
    assert_ne!(hits[0].outcome.amount(), hits[1].outcome.amount());
    assert_eq!(
        player_hp(&mut app),
        hp - hits[0].outcome.amount(),
        "the first message wins, whatever its size"
    );
    assert_eq!(state(&mut app), PlayerState::Hurt);
}

#[test]
fn contact_damage_is_one_hit_per_frame() {
    let mut app = app();
    let target = player(&mut app);
    let at = player_position(&mut app);
    let all = enemies(&mut app);
    for enemy in &all {
        if let Some(mut transform) = app.world_mut().get_mut::<Transform>(*enemy) {
            transform.translation = at;
        }
    }
    let hp = player_hp(&mut app);
    run(&mut app, 1);
    let contacts: Vec<_> = seen(&app)
        .into_iter()
        .filter(|h| h.target == target && all.contains(&h.attacker))
        .collect();
    assert_eq!(contacts.len(), 1, "two enemies touching, one hit");
    assert_eq!(player_hp(&mut app), hp - contacts[0].outcome.amount());
    // Invulnerable now: contact produces nothing at all.
    run(&mut app, 1);
    assert!(
        seen(&app).iter().all(|h| h.target != target),
        "no hit while invulnerable"
    );
}

#[test]
fn player_knockback_ignores_damage_size() {
    for amount in [1, 40] {
        let mut app = app();
        let target = player(&mut app);
        inject(&mut app, &[(target, target, amount, -1.0)]);
        run(&mut app, 1);
        assert_eq!(state(&mut app), PlayerState::Hurt);
        assert_eq!(
            velocity_of(&app, target),
            Vec2::new(-PLAYER_HURT_KNOCKBACK.0, PLAYER_HURT_KNOCKBACK.1)
        );
    }
}

#[test]
fn a_guarded_hit_from_the_front_is_reduced_and_does_not_stagger() {
    let mut app = app();
    let target = player(&mut app);
    app.world_mut()
        .resource_mut::<PendingKeys>()
        .0
        .push(KeyCode::KeyA);
    run(&mut app, 5);
    assert_eq!(state(&mut app), PlayerState::Guarding);
    let hp = player_hp(&mut app);
    // Facing right: a hit travelling left comes from the front.
    inject(&mut app, &[(target, target, 20, -1.0)]);
    run(&mut app, 1);
    let blocked = (20.0 * (1.0 - GUARD_DAMAGE_REDUCTION)).round() as i32;
    assert_eq!(player_hp(&mut app), hp - blocked);
    assert_eq!(state(&mut app), PlayerState::Guarding);
    assert_eq!(velocity_of(&app, target).x, 0.0, "no knockback");
    let inv = app
        .world()
        .get::<Invulnerable>(target)
        .map_or(0.0, |i| i.remaining);
    assert_eq!(inv, PLAYER_INVULNERABILITY, "a blocked hit still protects");
}

#[test]
fn a_fatal_hit_kills_without_knockback() {
    let mut app = app();
    let target = player(&mut app);
    let hp = player_hp(&mut app);
    inject(&mut app, &[(target, target, hp, 1.0)]);
    run(&mut app, 1);
    assert_eq!(state(&mut app), PlayerState::Dead);
    assert_eq!(velocity_of(&app, target).x, 0.0);
}

// ---------------------------------------------------------------- projectiles

#[test]
fn a_projectile_hits_one_target_and_disappears() {
    let mut app = app();
    let owner = player(&mut app);
    let at = player_position(&mut app) + Vec3::new(120.0, 0.0, 0.0);
    let near = spawn_dummy(&mut app, at);
    let also = spawn_dummy(&mut app, at);
    let projectile = spawn_hitbox(&mut app, owner, Team::Player, 20, at);
    if let Some(mut hitbox) = app.world_mut().get_mut::<Hitbox>(projectile) {
        hitbox.pierce = false;
    }
    app.world_mut().entity_mut(projectile).insert(Projectile {
        velocity: Vec2::ZERO,
    });
    run(&mut app, 1);
    let hits: Vec<_> = seen(&app)
        .into_iter()
        .filter(|h| h.attacker == owner && (h.target == near || h.target == also))
        .collect();
    assert_eq!(hits.len(), 1, "a projectile stops at its first target");
    assert!(app.world().get_entity(projectile).is_err(), "consumed");
}
