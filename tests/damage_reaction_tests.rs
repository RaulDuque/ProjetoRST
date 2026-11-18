//! V2.4 B2 — simultaneous-hit policy (step 6), knockback intensity and
//! stagger threshold (step 7), and how they combine.
//!
//! The defaults (`SimultaneousHitPolicy::Legacy`, `Medium`,
//! `StaggerThreshold::ALWAYS`) are pinned by
//! `damage_characterization_tests.rs`; here each new behaviour is switched
//! on in memory (never in data) and compared with the legacy one.
//!
//! Timing: fixed 16 ms frames; injected hits are written in
//! `GameSet::Combat` and applied in `GameSet::Resolution` of the same frame,
//! after physics, so velocities read after the frame are the knockback.

use std::path::Path;
use std::time::Duration;

use bevy::input::{InputPlugin as BevyInputPlugin, InputSystems};
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::app::plugins::GamePlugin;
use dark_castle_rpg::app::states::GameSet;
use dark_castle_rpg::combat::attacks::WeaponStrike;
use dark_castle_rpg::combat::damage::{DamageKind, DamageOutcome, HitSpec};
use dark_castle_rpg::combat::hitboxes::{
    CombatProfile, Hitbox, Invulnerable, SimultaneousHitPolicy, Team,
};
use dark_castle_rpg::combat::reaction::{KnockbackIntensity, StaggerThreshold};
use dark_castle_rpg::core::constants::{
    ENEMY_KNOCKBACK, PLAYER_HURT_KNOCKBACK, PLAYER_HURT_TIME, PLAYER_INVULNERABILITY,
};
use dark_castle_rpg::core::events::{DamageDealt, EnemyDefeated};
use dark_castle_rpg::core::resources::GameData;
use dark_castle_rpg::enemies::components::{Enemy, EnemyHealth};
use dark_castle_rpg::magic::elements::Element;
use dark_castle_rpg::player::attributes::RegenAccumulator;
use dark_castle_rpg::player::components::{ActionState, Player, Vitals};
use dark_castle_rpg::player::state::PlayerState;
use dark_castle_rpg::player::systems::PlayerStagger;
use dark_castle_rpg::ui::damage_numbers::DamageNumber;
use dark_castle_rpg::world::physics::Velocity;

use KnockbackIntensity::{Heavy, Light, Medium};
use SimultaneousHitPolicy::{HighestDamagePerFrame, Legacy};

const FRAME: Duration = Duration::from_millis(16);
const SURE_HIT: i32 = 1000;

// ---------------------------------------------------------------- harness

#[derive(Resource, Default)]
struct PendingKeys(Vec<KeyCode>);

fn apply_pending_keys(mut pending: ResMut<PendingKeys>, mut keys: ResMut<ButtonInput<KeyCode>>) {
    for key in pending.0.drain(..) {
        keys.press(key);
    }
}

#[derive(Resource, Default)]
struct PendingHits(Vec<DamageDealt>);

fn emit_pending_hits(mut pending: ResMut<PendingHits>, mut damage: MessageWriter<DamageDealt>) {
    for hit in pending.0.drain(..) {
        damage.write(hit);
    }
}

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

/// The real game (start room, player idle at the spawn, two bone sentries
/// far away) with `policy` and the player's `stagger` threshold.
fn app(policy: SimultaneousHitPolicy, stagger: f32) -> App {
    let data = GameData::load(Path::new(env!("CARGO_MANIFEST_DIR")))
        .unwrap_or_else(|e| panic!("game data failed to load: {e}"));
    let threshold = StaggerThreshold::new(stagger).unwrap_or_else(|e| panic!("{e}"));
    let mut app = App::new();
    dark_castle_rpg::input::use_legacy_controls(&mut app);
    app.add_plugins((MinimalPlugins, StatesPlugin, BevyInputPlugin))
        .insert_resource(TimeUpdateStrategy::ManualDuration(FRAME))
        .insert_resource(data)
        .insert_resource(policy)
        .insert_resource(PlayerStagger(threshold))
        .init_resource::<PendingKeys>()
        .init_resource::<PendingHits>()
        .init_resource::<Seen>()
        .add_systems(PreUpdate, apply_pending_keys.after(InputSystems))
        .add_systems(Update, emit_pending_hits.in_set(GameSet::Combat))
        .add_systems(Update, record.after(GameSet::Presentation))
        .add_plugins(GamePlugin);
    run(&mut app, 60);
    assert_eq!(state(&mut app), PlayerState::Idle, "warm-up must end idle");
    assert_eq!(*app.world().resource::<SimultaneousHitPolicy>(), policy);
    app
}

fn run(app: &mut App, frames: usize) {
    for _ in 0..frames {
        app.update();
    }
}

fn hit(amount: i32) -> DamageOutcome {
    DamageOutcome::Hit {
        amount,
        critical: false,
        element: Element::Physical,
    }
}

fn critical(amount: i32) -> DamageOutcome {
    DamageOutcome::Hit {
        amount,
        critical: true,
        element: Element::Physical,
    }
}

fn dealt(
    attacker: Entity,
    target: Entity,
    outcome: DamageOutcome,
    direction: f32,
    knockback: KnockbackIntensity,
) -> DamageDealt {
    DamageDealt {
        attacker,
        target,
        outcome,
        direction,
        knockback,
    }
}

fn inject(app: &mut App, hits: impl IntoIterator<Item = DamageDealt>) {
    app.world_mut().resource_mut::<PendingHits>().0.extend(hits);
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

fn with_player<C: Component<Mutability = bevy::ecs::component::Mutable>>(
    app: &mut App,
    f: impl FnOnce(&mut C),
) {
    let world = app.world_mut();
    let mut component = world
        .query_filtered::<&mut C, With<Player>>()
        .single_mut(world)
        .unwrap_or_else(|e| panic!("player missing: {e}"));
    f(&mut component);
}

/// Current and max HP, with regeneration frozen so the next frame only
/// shows damage.
fn player_hp(app: &mut App) -> (i32, i32) {
    let mut hp = (0, 0);
    with_player::<Vitals>(app, |vitals| {
        vitals.hp_regen = RegenAccumulator::default();
        vitals.mp_regen = RegenAccumulator::default();
        hp = (vitals.hp.current(), vitals.hp.max());
    });
    hp
}

fn invulnerability(app: &mut App) -> f32 {
    let mut remaining = 0.0;
    with_player::<Invulnerable>(app, |i| remaining = i.remaining);
    remaining
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

fn seen(app: &App) -> Vec<DamageDealt> {
    app.world().resource::<Seen>().hits.clone()
}

fn player_push(intensity: KnockbackIntensity, direction: f32) -> Vec2 {
    let (x, y) = intensity.player_push(direction);
    Vec2::new(x, y)
}

/// A player-team hitbox of `power` and `element` at `at`.
fn spawn_strike(app: &mut App, power: i32, element: Element, at: Vec3) -> Entity {
    let owner = player(app);
    let world = app.world_mut();
    let mut attack = world
        .query_filtered::<&CombatProfile, With<Player>>()
        .single(world)
        .map(|p| p.physical.clone())
        .unwrap_or_else(|e| panic!("player missing: {e}"));
    attack.power = power;
    attack.hit = SURE_HIT;
    attack.critical_chance = 0.0;
    world
        .spawn((
            Hitbox {
                owner,
                team: Team::Player,
                size: Vec2::splat(20.0),
                attack,
                hit: HitSpec {
                    kind: DamageKind::Physical,
                    element,
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

// ================================================================ step 6

#[test]
fn defaults_are_the_legacy_rules() {
    let mut app = App::new();
    dark_castle_rpg::input::use_legacy_controls(&mut app);
    app.add_plugins((MinimalPlugins, StatesPlugin, BevyInputPlugin))
        .insert_resource(
            GameData::load(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap_or_else(|e| panic!("{e}")),
        )
        .add_plugins(GamePlugin);
    assert_eq!(*app.world().resource::<SimultaneousHitPolicy>(), Legacy);
    assert_eq!(
        app.world().resource::<PlayerStagger>().0,
        StaggerThreshold::ALWAYS
    );
}

#[test]
fn two_hits_on_the_player_legacy_takes_the_first_highest_takes_the_strongest() {
    for (policy, expected) in [(Legacy, 5), (HighestDamagePerFrame, 30)] {
        let mut app = app(policy, 0.0);
        let target = player(&mut app);
        let (hp, _) = player_hp(&mut app);
        inject(
            &mut app,
            [
                dealt(target, target, hit(5), 1.0, Medium),
                dealt(target, target, hit(30), 1.0, Medium),
            ],
        );
        run(&mut app, 1);
        assert_eq!(player_hp(&mut app).0, hp - expected, "{policy:?}");
        assert_eq!(state(&mut app), PlayerState::Hurt);
        assert_eq!(
            invulnerability(&mut app),
            PLAYER_INVULNERABILITY,
            "{policy:?}: the invulnerability starts once, this frame"
        );
    }
}

#[test]
fn two_hits_on_an_enemy_legacy_adds_them_highest_takes_the_strongest() {
    for (policy, expected) in [(Legacy, 17), (HighestDamagePerFrame, 10)] {
        let mut app = app(policy, 0.0);
        let attacker = player(&mut app);
        let enemy = enemies(&mut app)[0];
        let hp = enemy_hp(&app, enemy);
        inject(
            &mut app,
            [
                dealt(attacker, enemy, hit(7), 1.0, Medium),
                dealt(attacker, enemy, hit(10), 1.0, Medium),
            ],
        );
        run(&mut app, 1);
        assert_eq!(enemy_hp(&app, enemy), hp - expected, "{policy:?}");
    }
}

#[test]
fn targets_are_never_mixed() {
    let mut app = app(HighestDamagePerFrame, 0.0);
    let me = player(&mut app);
    let [a, b] = enemies(&mut app)[..] else {
        panic!("the start room should have two enemies");
    };
    let (hp, _) = player_hp(&mut app);
    let (hp_a, hp_b) = (enemy_hp(&app, a), enemy_hp(&app, b));
    inject(
        &mut app,
        [
            dealt(me, a, hit(10), 1.0, Medium),
            dealt(me, b, hit(3), 1.0, Medium),
            dealt(me, me, hit(6), 1.0, Medium),
            dealt(me, a, hit(4), 1.0, Medium),
        ],
    );
    run(&mut app, 1);
    assert_eq!(enemy_hp(&app, a), hp_a - 10);
    assert_eq!(
        enemy_hp(&app, b),
        hp_b - 3,
        "a weaker hit on another target"
    );
    assert_eq!(player_hp(&mut app).0, hp - 6);
}

#[test]
fn frames_are_never_mixed() {
    let mut app = app(HighestDamagePerFrame, 0.0);
    let me = player(&mut app);
    let enemy = enemies(&mut app)[0];
    let hp = enemy_hp(&app, enemy);
    inject(&mut app, [dealt(me, enemy, hit(4), 1.0, Medium)]);
    run(&mut app, 1);
    inject(&mut app, [dealt(me, enemy, hit(9), 1.0, Medium)]);
    run(&mut app, 1);
    assert_eq!(
        enemy_hp(&app, enemy),
        hp - 13,
        "one hit per frame, both kept"
    );
}

#[test]
fn ties_go_to_the_lower_attacker_then_to_the_first_message() {
    let mut app = app(HighestDamagePerFrame, 0.0);
    let enemy = enemies(&mut app)[0];
    let first = app.world_mut().spawn_empty().id();
    let second = app.world_mut().spawn_empty().id();
    let (low, high) = if first.to_bits() < second.to_bits() {
        (first, second)
    } else {
        (second, first)
    };
    inject(
        &mut app,
        [
            dealt(high, enemy, hit(10), -1.0, Medium),
            dealt(low, enemy, hit(10), 1.0, Medium),
        ],
    );
    run(&mut app, 1);
    assert_eq!(
        velocity_of(&app, enemy).x,
        ENEMY_KNOCKBACK,
        "lower attacker"
    );
    run(&mut app, 30);
    inject(
        &mut app,
        [
            dealt(low, enemy, hit(10), -1.0, Medium),
            dealt(low, enemy, hit(10), 1.0, Medium),
        ],
    );
    run(&mut app, 1);
    assert_eq!(
        velocity_of(&app, enemy).x,
        -ENEMY_KNOCKBACK,
        "first message"
    );
}

#[test]
fn a_critical_hit_wins_only_when_it_deals_more() {
    for (crit, normal, expected) in [(15, 12, 15), (9, 12, 12)] {
        let mut app = app(HighestDamagePerFrame, 0.0);
        let me = player(&mut app);
        let enemy = enemies(&mut app)[0];
        let hp = enemy_hp(&app, enemy);
        inject(
            &mut app,
            [
                dealt(me, enemy, hit(normal), 1.0, Medium),
                dealt(me, enemy, critical(crit), 1.0, Medium),
            ],
        );
        run(&mut app, 1);
        assert_eq!(enemy_hp(&app, enemy), hp - expected);
    }
}

#[test]
fn physical_and_elemental_hits_compare_by_the_damage_after_resistances() {
    // Bone sentries are weak to fire (-0.25): the same power deals ~25 %
    // more as fire, more than the ±10 % variance can bridge.
    let mut app = app(HighestDamagePerFrame, 0.0);
    let enemy = enemies(&mut app)[0];
    let at = app
        .world()
        .get::<Transform>(enemy)
        .map(|t| t.translation)
        .unwrap_or_else(|| panic!("enemy without transform"));
    let hp = enemy_hp(&app, enemy);
    spawn_strike(&mut app, 40, Element::Physical, at);
    spawn_strike(&mut app, 40, Element::Fire, at);
    run(&mut app, 1);
    let hits: Vec<_> = seen(&app)
        .into_iter()
        .filter(|h| h.target == enemy)
        .collect();
    assert_eq!(hits.len(), 2, "both strikes are candidates: {hits:?}");
    let amount_of = |element: Element| {
        hits.iter()
            .find(|h| matches!(h.outcome, DamageOutcome::Hit { element: e, .. } if e == element))
            .map(|h| h.outcome.amount())
            .unwrap_or_else(|| panic!("no {element:?} hit"))
    };
    let (physical, fire) = (amount_of(Element::Physical), amount_of(Element::Fire));
    assert!(fire > physical, "fire {fire} vs physical {physical}");
    assert_eq!(enemy_hp(&app, enemy), hp - fire);
}

#[test]
fn the_guard_counts_in_the_comparison() {
    // Guarding, facing right: 30 from the front becomes 9, 20 from behind
    // stays 20.
    for (policy, expected) in [(Legacy, 9), (HighestDamagePerFrame, 20)] {
        let mut app = app(policy, 0.0);
        let me = player(&mut app);
        app.world_mut()
            .resource_mut::<PendingKeys>()
            .0
            .push(KeyCode::KeyA);
        run(&mut app, 5);
        assert_eq!(state(&mut app), PlayerState::Guarding);
        let (hp, _) = player_hp(&mut app);
        inject(
            &mut app,
            [
                dealt(me, me, hit(30), -1.0, Medium),
                dealt(me, me, hit(20), 1.0, Medium),
            ],
        );
        run(&mut app, 1);
        assert_eq!(player_hp(&mut app).0, hp - expected, "{policy:?}");
        assert_eq!(state(&mut app), PlayerState::Guarding, "the guard holds");
    }
}

#[test]
fn existing_invulnerability_blocks_every_candidate() {
    let mut app = app(HighestDamagePerFrame, 0.0);
    let me = player(&mut app);
    with_player::<Invulnerable>(&mut app, |i| i.remaining = 0.5);
    let (hp, _) = player_hp(&mut app);
    inject(
        &mut app,
        [
            dealt(me, me, hit(5), 1.0, Heavy),
            dealt(me, me, hit(30), 1.0, Heavy),
        ],
    );
    run(&mut app, 1);
    assert_eq!(player_hp(&mut app).0, hp);
    assert_ne!(state(&mut app), PlayerState::Hurt);
    assert_eq!(velocity_of(&app, me).x, 0.0);
}

#[test]
fn a_lethal_frame_kills_once_under_the_highest_damage_rule() {
    let mut app = app(HighestDamagePerFrame, 0.0);
    let me = player(&mut app);
    let enemy = enemies(&mut app)[0];
    inject(
        &mut app,
        [
            dealt(me, enemy, hit(10), 1.0, Medium),
            dealt(me, enemy, hit(1000), 1.0, Heavy),
            dealt(me, enemy, hit(1000), 1.0, Heavy),
        ],
    );
    run(&mut app, 1);
    assert!(app.world().get_entity(enemy).is_err());
    assert_eq!(app.world().resource::<Seen>().defeats, 1);

    let (hp, _) = player_hp(&mut app);
    inject(
        &mut app,
        [
            dealt(me, me, hit(5), 1.0, Light),
            dealt(me, me, hit(hp), 1.0, Heavy),
        ],
    );
    run(&mut app, 1);
    assert_eq!(state(&mut app), PlayerState::Dead);
    assert_eq!(player_hp(&mut app).0, 0);
    assert_eq!(velocity_of(&app, me).x, 0.0, "no knockback on death");
}

#[test]
fn contact_under_the_highest_damage_rule_offers_every_touching_enemy() {
    let mut app = app(HighestDamagePerFrame, 0.0);
    let me = player(&mut app);
    let at = player_position(&mut app);
    let all = enemies(&mut app);
    for enemy in &all {
        if let Some(mut transform) = app.world_mut().get_mut::<Transform>(*enemy) {
            transform.translation = at;
        }
    }
    let (hp, _) = player_hp(&mut app);
    run(&mut app, 1);
    let contacts: Vec<i32> = seen(&app)
        .iter()
        .filter(|h| h.target == me && all.contains(&h.attacker))
        .map(|h| h.outcome.amount())
        .collect();
    assert_eq!(contacts.len(), 2, "one candidate per touching enemy");
    let strongest = contacts.iter().copied().max().unwrap_or(0);
    assert_eq!(player_hp(&mut app).0, hp - strongest);
    run(&mut app, 1);
    assert!(
        seen(&app).iter().all(|h| h.target != me),
        "invulnerable: no contact candidates"
    );
}

// ================================================================ step 7

#[test]
fn player_knockback_follows_the_intensity() {
    for intensity in [Light, Medium, Heavy] {
        let mut app = app(Legacy, 0.0);
        let me = player(&mut app);
        inject(&mut app, [dealt(me, me, hit(5), -1.0, intensity)]);
        run(&mut app, 1);
        assert_eq!(state(&mut app), PlayerState::Hurt);
        assert_eq!(velocity_of(&app, me), player_push(intensity, -1.0));
        assert_eq!(invulnerability(&mut app), PLAYER_INVULNERABILITY);
    }
    assert_eq!(
        player_push(Medium, -1.0),
        Vec2::new(-PLAYER_HURT_KNOCKBACK.0, PLAYER_HURT_KNOCKBACK.1)
    );
    assert!(player_push(Light, 1.0).x < player_push(Heavy, 1.0).x);
}

#[test]
fn enemy_knockback_follows_the_intensity() {
    let mut app = app(Legacy, 0.0);
    let me = player(&mut app);
    let enemy = enemies(&mut app)[0];
    let mut pushes = Vec::new();
    for intensity in [Light, Medium, Heavy] {
        inject(&mut app, [dealt(me, enemy, hit(1), 1.0, intensity)]);
        run(&mut app, 1);
        let push = velocity_of(&app, enemy).x;
        assert_eq!(push, intensity.enemy_push(1.0));
        pushes.push(push);
        run(&mut app, 30);
    }
    assert_eq!(pushes[1], ENEMY_KNOCKBACK);
    assert!(pushes[0] < pushes[1] && pushes[1] < pushes[2]);
}

/// Damage at the threshold of a player with `max` HP for a share of 0.5:
/// the smallest whole damage that reaches it.
fn half_of(max: i32) -> i32 {
    (0.5 * max as f32).ceil() as i32
}

#[test]
fn a_hit_below_the_threshold_costs_hp_but_does_not_stagger() {
    let mut app = app(Legacy, 0.5);
    let me = player(&mut app);
    let (hp, max) = player_hp(&mut app);
    let below = half_of(max) - 1;
    inject(&mut app, [dealt(me, me, hit(below), -1.0, Heavy)]);
    run(&mut app, 1);
    assert_eq!(player_hp(&mut app).0, hp - below);
    assert_eq!(state(&mut app), PlayerState::Idle, "no stagger");
    assert_eq!(velocity_of(&app, me).x, 0.0, "no knockback");
    assert_eq!(
        invulnerability(&mut app),
        PLAYER_INVULNERABILITY,
        "the invulnerability is unchanged"
    );
}

#[test]
fn a_hit_at_or_above_the_threshold_staggers() {
    for extra in [0, 1] {
        let mut app = app(Legacy, 0.5);
        let me = player(&mut app);
        let (_, max) = player_hp(&mut app);
        inject(
            &mut app,
            [dealt(me, me, hit(half_of(max) + extra), -1.0, Light)],
        );
        run(&mut app, 1);
        assert_eq!(state(&mut app), PlayerState::Hurt, "+{extra}");
        assert_eq!(velocity_of(&app, me), player_push(Light, -1.0));
    }
}

#[test]
fn a_fatal_hit_below_the_threshold_still_kills() {
    let mut app = app(Legacy, 0.5);
    let me = player(&mut app);
    with_player::<Vitals>(&mut app, |v| {
        let current = v.hp.current();
        v.hp.deplete(current - 1);
    });
    inject(&mut app, [dealt(me, me, hit(1), -1.0, Heavy)]);
    run(&mut app, 1);
    assert_eq!(state(&mut app), PlayerState::Dead);
    assert_eq!(velocity_of(&app, me).x, 0.0);
}

#[test]
fn a_heavy_hit_interrupts_an_attack() {
    let mut app = app(Legacy, 0.0);
    let me = player(&mut app);
    app.world_mut()
        .resource_mut::<PendingKeys>()
        .0
        .push(KeyCode::KeyX);
    run(&mut app, 1);
    assert_eq!(state(&mut app), PlayerState::Attacking);
    inject(&mut app, [dealt(me, me, hit(5), 1.0, Heavy)]);
    run(&mut app, 1);
    assert_eq!(state(&mut app), PlayerState::Hurt);
    assert_eq!(velocity_of(&app, me), player_push(Heavy, 1.0));
    let world = app.world_mut();
    let strikes = world
        .query_filtered::<(), With<WeaponStrike>>()
        .iter(world)
        .count();
    assert_eq!(strikes, 0, "the strike is cancelled");
}

#[test]
fn a_heavy_hit_interrupts_a_cast() {
    let mut app = app(Legacy, 0.0);
    let me = player(&mut app);
    with_player::<ActionState>(&mut app, |s| {
        s.start_action(PlayerState::Casting, 1.0)
            .unwrap_or_else(|e| panic!("{e}"));
    });
    inject(&mut app, [dealt(me, me, hit(5), -1.0, Heavy)]);
    run(&mut app, 1);
    assert_eq!(state(&mut app), PlayerState::Hurt);
    assert_eq!(velocity_of(&app, me), player_push(Heavy, -1.0));
}

#[test]
fn a_heavy_hit_never_pushes_the_player_through_a_wall() {
    let mut app = app(Legacy, 0.0);
    let me = player(&mut app);
    // The west wall of the start room ends at x = 0.
    with_player::<Transform>(&mut app, |t| t.translation.x = 20.0);
    run(&mut app, 5);
    inject(&mut app, [dealt(me, me, hit(5), -1.0, Heavy)]);
    for _ in 0..40 {
        run(&mut app, 1);
        let half_width = 13.0;
        assert!(
            player_position(&mut app).x - half_width >= -0.01,
            "inside the wall at x = {}",
            player_position(&mut app).x
        );
    }
}

#[test]
fn a_stagger_ends_like_before() {
    let mut app = app(Legacy, 0.0);
    let me = player(&mut app);
    inject(&mut app, [dealt(me, me, hit(5), 1.0, Heavy)]);
    run(&mut app, 1);
    assert_eq!(state(&mut app), PlayerState::Hurt);
    let frames = (PLAYER_HURT_TIME / FRAME.as_secs_f32()).ceil() as usize + 1;
    let mut left = 0;
    while state(&mut app) == PlayerState::Hurt {
        run(&mut app, 1);
        left += 1;
        assert!(left <= frames, "hurt never ended");
    }
    assert_ne!(state(&mut app), PlayerState::Dead);
}

// ================================================================ steps 6 + 7

#[test]
fn the_selected_hit_brings_its_own_knockback() {
    for (policy, amount, intensity) in [(Legacy, 5, Heavy), (HighestDamagePerFrame, 30, Light)] {
        let mut app = app(policy, 0.0);
        let me = player(&mut app);
        let (hp, _) = player_hp(&mut app);
        inject(
            &mut app,
            [
                dealt(me, me, hit(5), -1.0, Heavy),
                dealt(me, me, hit(30), 1.0, Light),
            ],
        );
        run(&mut app, 1);
        assert_eq!(player_hp(&mut app).0, hp - amount, "{policy:?}");
        let direction = if amount == 5 { -1.0 } else { 1.0 };
        assert_eq!(
            velocity_of(&app, me),
            player_push(intensity, direction),
            "{policy:?}: damage and knockback come from the same hit"
        );
        assert_eq!(state(&mut app), PlayerState::Hurt);
        assert_eq!(invulnerability(&mut app), PLAYER_INVULNERABILITY);
    }
}

#[test]
fn an_enemy_gets_the_damage_and_knockback_of_one_hit() {
    let mut app = app(HighestDamagePerFrame, 0.0);
    let me = player(&mut app);
    let enemy = enemies(&mut app)[0];
    let hp = enemy_hp(&app, enemy);
    inject(
        &mut app,
        [
            dealt(me, enemy, hit(10), 1.0, Light),
            dealt(me, enemy, hit(25), -1.0, Heavy),
            dealt(me, enemy, hit(3), 1.0, Medium),
        ],
    );
    run(&mut app, 1);
    assert_eq!(enemy_hp(&app, enemy), hp - 25);
    assert_eq!(velocity_of(&app, enemy).x, Heavy.enemy_push(-1.0));
}

#[test]
fn a_weaker_hit_below_the_threshold_never_borrows_the_stagger() {
    // Highest rule + threshold: the selected hit alone decides the stagger.
    let mut app = app(HighestDamagePerFrame, 0.5);
    let me = player(&mut app);
    let (hp, max) = player_hp(&mut app);
    let below = half_of(max) - 1;
    inject(
        &mut app,
        [
            dealt(me, me, hit(1), 1.0, Heavy),
            dealt(me, me, hit(below), 1.0, Light),
        ],
    );
    run(&mut app, 1);
    assert_eq!(player_hp(&mut app).0, hp - below, "damage not added up");
    assert_eq!(state(&mut app), PlayerState::Idle);
    assert_eq!(velocity_of(&app, me).x, 0.0);
}

// ================================================================ known limitations
//
// Prerequisites before `HighestDamagePerFrame` can be switched on
// (docs/COMBAT_FEEL_V2_4.md §12.6): the selection happens in the targets'
// `receive_damage`, after the hit was produced and presented.

fn damage_numbers(app: &mut App) -> usize {
    let world = app.world_mut();
    world
        .query_filtered::<(), With<DamageNumber>>()
        .iter(world)
        .count()
}

fn enemy_position(app: &App, enemy: Entity) -> Vec3 {
    app.world()
        .get::<Transform>(enemy)
        .map(|t| t.translation)
        .unwrap_or_else(|| panic!("enemy without transform"))
}

/// A fire strike that wins (bone sentries are weak to fire), plus two
/// physical strikes of the same `power` that lose: one that disappears on
/// its first hit, one that pierces. Returns (winner, single-hit loser,
/// piercing loser).
fn three_strikes_on(app: &mut App, enemy: Entity, power: i32) -> (Entity, Entity, Entity) {
    let at = enemy_position(app, enemy);
    let winner = spawn_strike(app, power, Element::Fire, at);
    let single = spawn_strike(app, power, Element::Physical, at);
    let piercing = spawn_strike(app, power, Element::Physical, at);
    if let Some(mut hitbox) = app.world_mut().get_mut::<Hitbox>(single) {
        hitbox.pierce = false;
    }
    (winner, single, piercing)
}

fn enemy_hits(app: &App, enemy: Entity) -> Vec<DamageDealt> {
    seen(app)
        .into_iter()
        .filter(|h| h.target == enemy)
        .collect()
}

#[test]
fn limitation_discarded_hits_still_show_damage_numbers() {
    let mut app = app(HighestDamagePerFrame, 0.0);
    let me = player(&mut app);
    let enemy = enemies(&mut app)[0];
    let hp = enemy_hp(&app, enemy);
    assert_eq!(damage_numbers(&mut app), 0);
    inject(
        &mut app,
        [
            dealt(me, enemy, hit(7), 1.0, Medium),
            dealt(me, enemy, hit(10), 1.0, Medium),
        ],
    );
    run(&mut app, 1);
    assert_eq!(enemy_hp(&app, enemy), hp - 10, "only the strongest applies");
    assert_eq!(
        damage_numbers(&mut app),
        2,
        "the presentation reads the raw hits: the discarded 7 is shown too"
    );
}

#[test]
fn limitation_a_discarded_hit_still_spends_its_hitbox() {
    let mut app = app(HighestDamagePerFrame, 0.0);
    let enemy = enemies(&mut app)[0];
    let hp = enemy_hp(&app, enemy);
    // 40: the strongest of the three (~45) does not kill a 60 HP sentry.
    let (winner, single, piercing) = three_strikes_on(&mut app, enemy, 40);
    run(&mut app, 1);
    let hits = enemy_hits(&app, enemy);
    assert_eq!(hits.len(), 3, "all three are candidates: {hits:?}");
    let strongest = hits
        .iter()
        .map(|h| h.outcome.amount())
        .max()
        .unwrap_or_default();
    assert_eq!(enemy_hp(&app, enemy), hp - strongest, "one hit applied");
    assert!(
        app.world().get_entity(single).is_err(),
        "a single-hit hitbox (or projectile) whose hit was discarded is gone"
    );
    let spent = |entity: Entity| {
        app.world()
            .get::<Hitbox>(entity)
            .is_some_and(|h| h.already_hit.contains(&enemy))
    };
    assert!(spent(piercing), "the discarded piercing hit counts as done");
    assert!(spent(winner));
    run(&mut app, 1);
    assert!(
        enemy_hits(&app, enemy).is_empty(),
        "the discarded hit is never retried"
    );
}

#[test]
fn limitation_the_default_policy_discards_nothing_on_enemies() {
    let mut app = app(Legacy, 0.0);
    let enemy = enemies(&mut app)[0];
    let hp = enemy_hp(&app, enemy);
    // 12: the three added up (~27) do not kill a 60 HP sentry.
    three_strikes_on(&mut app, enemy, 12);
    run(&mut app, 1);
    let hits = enemy_hits(&app, enemy);
    assert_eq!(hits.len(), 3, "{hits:?}");
    let total: i32 = hits.iter().map(|h| h.outcome.amount()).sum();
    assert_eq!(enemy_hp(&app, enemy), hp - total, "every hit is applied");
    assert_eq!(
        damage_numbers(&mut app),
        3,
        "one number per applied hit: nothing shown is discarded"
    );
}
