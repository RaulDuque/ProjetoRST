//! Headless smoke tests of the real ECS wiring (no window, no renderer).
//!
//! They catch what unit tests cannot: conflicting queries, missing
//! resources or messages, and broken system ordering.

use std::path::{Path, PathBuf};
use std::time::Duration;

use bevy::input::{InputPlugin as BevyInputPlugin, InputSystems};
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::app::plugins::GamePlugin;
use dark_castle_rpg::app::states::GameSet;
use dark_castle_rpg::combat::command_parser::CommandTiming;
use dark_castle_rpg::combat::damage::{DamageKind, DamageOutcome, HitSpec};
use dark_castle_rpg::combat::hitboxes::{
    AttachedToOwner, CombatProfile, Hitbox, Hurtbox, Invulnerable, Team,
};
use dark_castle_rpg::core::constants::{DEFAULT_SAVE_SLOT, SAVE_DIR};
use dark_castle_rpg::core::events::{
    BasicAttackRequested, CommandResolved, DamageDealt, SkillActivated, SkillStarted,
};
use dark_castle_rpg::core::resources::GameData;
use dark_castle_rpg::enemies::components::Enemy;
use dark_castle_rpg::magic::elements::Element;
use dark_castle_rpg::magic::projectiles::Projectile;
use dark_castle_rpg::player::attributes::RegenAccumulator;
use dark_castle_rpg::player::components::{ActionState, Player, SkillBook, Vitals};
use dark_castle_rpg::player::state::{ActionPhase, PlayerState};
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::save::systems::SaveSlots;
use dark_castle_rpg::skills::definitions::{
    Acquisition, SkillCategory, SkillDefinition, SkillEffect,
};
use dark_castle_rpg::skills::execution::{CommandOutcome, ResourceKind};
use dark_castle_rpg::ui::hud::HudMessages;
use dark_castle_rpg::world::maps::CurrentRoom;
use dark_castle_rpg::world::physics::{Grounded, Velocity};

const FRAME: Duration = Duration::from_millis(16);

/// Key changes applied right after Bevy's input systems, exactly like a
/// real keyboard event would be (so `just_pressed` is visible this frame).
#[derive(Resource, Default)]
struct PendingKeys {
    press: Vec<KeyCode>,
    release: Vec<KeyCode>,
}

fn apply_pending_keys(mut pending: ResMut<PendingKeys>, mut keys: ResMut<ButtonInput<KeyCode>>) {
    // Releases first, so "release then press again" keeps the key held.
    for key in pending.release.drain(..) {
        keys.release(key);
    }
    for key in pending.press.drain(..) {
        keys.press(key);
    }
}

fn app() -> App {
    let data = GameData::load(Path::new(env!("CARGO_MANIFEST_DIR")))
        .unwrap_or_else(|e| panic!("game data failed to load: {e}"));
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatesPlugin, BevyInputPlugin))
        .insert_resource(TimeUpdateStrategy::ManualDuration(FRAME))
        .insert_resource(data)
        .init_resource::<PendingKeys>()
        .add_systems(PreUpdate, apply_pending_keys.after(InputSystems))
        .add_plugins(GamePlugin);
    app
}

fn run_frames(app: &mut App, frames: usize) {
    for _ in 0..frames {
        app.update();
    }
}

fn press(app: &mut App, key: KeyCode) {
    app.world_mut()
        .resource_mut::<PendingKeys>()
        .press
        .push(key);
}

fn release(app: &mut App, key: KeyCode) {
    app.world_mut()
        .resource_mut::<PendingKeys>()
        .release
        .push(key);
}

/// Holds `keys` for `frames` frames, then releases them.
fn hold(app: &mut App, keys: &[KeyCode], frames: usize) {
    for key in keys {
        press(app, *key);
    }
    run_frames(app, frames);
    for key in keys {
        release(app, *key);
    }
}

fn player_query<D: bevy::ecs::query::QueryData>(app: &mut App) -> usize {
    let world = app.world_mut();
    world
        .query_filtered::<D, With<Player>>()
        .iter(world)
        .count()
}

#[test]
fn game_starts_and_runs_without_panicking() {
    let mut app = app();
    run_frames(&mut app, 120);
    assert_eq!(player_query::<Entity>(&mut app), 1);
    let world = app.world_mut();
    let enemies = world.query::<&Enemy>().iter(world).count();
    assert_eq!(enemies, 2);
}

#[test]
fn player_lands_on_the_floor() {
    let mut app = app();
    run_frames(&mut app, 90);
    let world = app.world_mut();
    let grounded = world
        .query_filtered::<&Grounded, With<Player>>()
        .single(world)
        .map(|g| g.on_ground)
        .unwrap_or(false);
    assert!(grounded);
}

#[test]
fn spell_command_spends_mp_and_spawns_a_projectile() {
    let mut app = app();
    run_frames(&mut app, 60);
    let mp_before = {
        let world = app.world_mut();
        world
            .query_filtered::<&Vitals, With<Player>>()
            .single(world)
            .map(|v| v.mp.current())
            .unwrap_or_default()
    };

    // ↓ ↘ → + MAGIC
    hold(&mut app, &[KeyCode::ArrowDown], 3);
    hold(&mut app, &[KeyCode::ArrowDown, KeyCode::ArrowRight], 3);
    press(&mut app, KeyCode::ArrowRight);
    run_frames(&mut app, 2);
    hold(&mut app, &[KeyCode::KeyC], 1);
    release(&mut app, KeyCode::ArrowRight);

    let world = app.world_mut();
    let projectiles = world.query::<&Projectile>().iter(world).count();
    let (mp_after, on_cooldown) = world
        .query_filtered::<(&Vitals, &SkillBook), With<Player>>()
        .single(world)
        .map(|(v, b)| (v.mp.current(), !b.cooldowns.is_ready("ember_bolt")))
        .unwrap_or_default();
    assert_eq!(projectiles, 1, "ember bolt should be in flight");
    assert_eq!(mp_before - mp_after, 8);
    assert!(on_cooldown);
}

/// Contents of the real slot files (`None` = file absent).
fn real_save_snapshot() -> Vec<Option<Vec<u8>>> {
    let storage = SaveStorage::new(Path::new(env!("CARGO_MANIFEST_DIR")).join(SAVE_DIR));
    [
        storage.slot_path(DEFAULT_SAVE_SLOT),
        storage.backup_path(DEFAULT_SAVE_SLOT),
    ]
    .iter()
    .map(|path| std::fs::read(path).ok())
    .collect()
}

#[test]
fn quick_save_and_load_use_the_configured_save_dir() {
    let real_before = real_save_snapshot();
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("smoke_isolated_saves");
    let storage = SaveStorage::new(&dir);
    for path in [
        storage.slot_path(DEFAULT_SAVE_SLOT),
        storage.backup_path(DEFAULT_SAVE_SLOT),
    ] {
        if path.exists() {
            std::fs::remove_file(&path)
                .unwrap_or_else(|e| panic!("cannot reset {}: {e}", path.display()));
        }
    }

    let mut app = app();
    app.insert_resource(SaveSlots(storage.clone()));
    run_frames(&mut app, 60);

    // F5 writes to the configured directory.
    hold(&mut app, &[KeyCode::F5], 1);
    run_frames(&mut app, 1);
    let saved = storage
        .load(DEFAULT_SAVE_SLOT)
        .unwrap_or_else(|e| panic!("isolated save missing or invalid: {e}"));

    // Move the player away, then F9 must read the configured directory back.
    {
        let world = app.world_mut();
        let mut transform = world
            .query_filtered::<&mut Transform, With<Player>>()
            .single_mut(world)
            .unwrap_or_else(|e| panic!("player missing: {e}"));
        transform.translation.x += 100.0;
    }
    hold(&mut app, &[KeyCode::F9], 1);
    let world = app.world_mut();
    let x = world
        .query_filtered::<&Transform, With<Player>>()
        .single(world)
        .map(|t| t.translation.x)
        .unwrap_or_else(|e| panic!("player missing: {e}"));
    assert_eq!(
        x, saved.player.position.0,
        "F9 should restore the isolated save"
    );

    assert_eq!(
        real_save_snapshot(),
        real_before,
        "the real saves/ directory must not be touched"
    );
}

// ---------------------------------------------------------------- A3
// Several damage messages for the player in one frame must respect the
// invulnerability started by the first valid hit (audit A3). Messages are
// injected directly so the order of the hits is deterministic. The player
// stays at the spawn point, far from the enemies; natural HP regeneration
// is below 1 point during these short tests, so HP values are exact.

fn player_hp(app: &mut App) -> i32 {
    let world = app.world_mut();
    world
        .query_filtered::<&Vitals, With<Player>>()
        .single(world)
        .map(|v| v.hp.current())
        .unwrap_or_else(|e| panic!("player missing: {e}"))
}

fn player_state(app: &mut App) -> PlayerState {
    let world = app.world_mut();
    world
        .query_filtered::<&ActionState, With<Player>>()
        .single(world)
        .map(|s| s.0.state())
        .unwrap_or_else(|e| panic!("player missing: {e}"))
}

fn player_invulnerability(app: &mut App) -> f32 {
    let world = app.world_mut();
    world
        .query_filtered::<&Invulnerable, With<Player>>()
        .single(world)
        .map(|i| i.remaining)
        .unwrap_or_else(|e| panic!("player missing: {e}"))
}

fn hit(amount: i32) -> DamageOutcome {
    DamageOutcome::Hit {
        amount,
        critical: false,
        element: Element::Physical,
    }
}

/// Damage queued by a test, emitted on the next frame inside `GameSet::Combat`
/// (where `detect_hits` and `contact_damage` produce real hits), so the
/// messages live exactly as long as real ones do.
#[derive(Resource, Default)]
struct PendingDamage(Vec<DamageOutcome>);

fn emit_pending_damage(
    mut pending: ResMut<PendingDamage>,
    player: Query<Entity, With<Player>>,
    mut damage: MessageWriter<DamageDealt>,
) {
    let Ok(player) = player.single() else {
        return;
    };
    for outcome in pending.0.drain(..) {
        damage.write(DamageDealt {
            attacker: player,
            target: player,
            outcome,
            direction: 1.0,
        });
    }
}

/// Queues one `DamageDealt` per outcome against the player, in order.
fn inject_damage(app: &mut App, outcomes: &[DamageOutcome]) {
    app.world_mut()
        .resource_mut::<PendingDamage>()
        .0
        .extend_from_slice(outcomes);
}

/// App after the warm-up: player standing at the spawn with full HP.
fn idle_app() -> App {
    let mut app = app();
    app.init_resource::<PendingDamage>()
        .add_systems(Update, emit_pending_damage.in_set(GameSet::Combat));
    run_frames(&mut app, 60);
    app
}

#[test]
fn two_hits_in_one_frame_apply_damage_once() {
    let mut app = idle_app();
    let before = player_hp(&mut app);
    inject_damage(&mut app, &[hit(10), hit(10)]);
    run_frames(&mut app, 1);
    assert_eq!(player_hp(&mut app), before - 10);
    assert_eq!(player_state(&mut app), PlayerState::Hurt);
}

#[test]
fn three_hits_in_one_frame_apply_only_the_first() {
    let mut app = idle_app();
    let before = player_hp(&mut app);
    inject_damage(&mut app, &[hit(7), hit(10), hit(12)]);
    run_frames(&mut app, 1);
    assert_eq!(player_hp(&mut app), before - 7);
}

#[test]
fn active_invulnerability_blocks_damage_and_hurt() {
    let mut app = idle_app();
    let before = player_hp(&mut app);
    let state_before = player_state(&mut app);
    {
        let world = app.world_mut();
        let mut invulnerable = world
            .query_filtered::<&mut Invulnerable, With<Player>>()
            .single_mut(world)
            .unwrap_or_else(|e| panic!("player missing: {e}"));
        invulnerable.remaining = 0.5;
    }
    inject_damage(&mut app, &[hit(10)]);
    run_frames(&mut app, 1);
    assert_eq!(player_hp(&mut app), before);
    assert_eq!(player_state(&mut app), state_before);
}

#[test]
fn miss_then_hit_applies_the_hit() {
    let mut app = idle_app();
    let before = player_hp(&mut app);
    inject_damage(&mut app, &[DamageOutcome::Miss, hit(10)]);
    run_frames(&mut app, 1);
    assert_eq!(player_hp(&mut app), before - 10);
    assert!(player_invulnerability(&mut app) > 0.0);
}

#[test]
fn zero_damage_hit_does_not_consume_invulnerability() {
    let mut app = idle_app();
    let before = player_hp(&mut app);
    inject_damage(&mut app, &[hit(0), hit(10)]);
    run_frames(&mut app, 1);
    assert_eq!(player_hp(&mut app), before - 10);
}

#[test]
fn hit_then_miss_applies_the_hit_once() {
    let mut app = idle_app();
    let before = player_hp(&mut app);
    inject_damage(&mut app, &[hit(10), DamageOutcome::Miss]);
    run_frames(&mut app, 1);
    assert_eq!(player_hp(&mut app), before - 10);
    // The trailing Miss neither resets the protection nor the stagger.
    assert!(player_invulnerability(&mut app) > 0.0);
    assert_eq!(player_state(&mut app), PlayerState::Hurt);
}

#[test]
fn fatal_hit_leaves_no_pending_damage_for_the_respawn() {
    let mut app = idle_app();
    let lethal = player_hp(&mut app) + 50;
    inject_damage(&mut app, &[hit(lethal), hit(10), hit(10)]);
    run_frames(&mut app, 1);
    assert_eq!(player_state(&mut app), PlayerState::Dead);

    // Respawn on the very next frame: the hits queued behind the fatal one
    // must not land on the revived player.
    hold(&mut app, &[KeyCode::KeyR], 1);
    run_frames(&mut app, 2);
    let world = app.world_mut();
    let (hp, max) = world
        .query_filtered::<&Vitals, With<Player>>()
        .single(world)
        .map(|v| (v.hp.current(), v.hp.max()))
        .unwrap_or_else(|e| panic!("player missing: {e}"));
    assert_ne!(player_state(&mut app), PlayerState::Dead);
    assert_eq!(hp, max, "stale damage hit the revived player");
}

#[test]
fn hits_queued_behind_a_fatal_hit_are_discarded_that_frame() {
    // Isolates the drain on death from the respawn i-frames: the player is
    // revived directly (no `respawn`, no invulnerability), so any hit left
    // unread in the frame of the death would land on the next frame.
    let mut app = idle_app();
    let lethal = player_hp(&mut app) + 50;
    inject_damage(&mut app, &[hit(lethal), hit(10), hit(10)]);
    run_frames(&mut app, 1);
    assert_eq!(player_state(&mut app), PlayerState::Dead);
    let max = {
        let world = app.world_mut();
        let (mut state, mut vitals, mut invulnerable) = world
            .query_filtered::<(&mut ActionState, &mut Vitals, &mut Invulnerable), With<Player>>()
            .single_mut(world)
            .unwrap_or_else(|e| panic!("player missing: {e}"));
        state.0.revive();
        vitals.hp.refill();
        invulnerable.remaining = 0.0;
        vitals.hp.max()
    };
    run_frames(&mut app, 1);
    assert_eq!(
        player_hp(&mut app),
        max,
        "a hit queued behind the fatal one landed"
    );
}

#[test]
fn damage_applies_again_after_invulnerability_expires() {
    let mut app = idle_app();
    inject_damage(&mut app, &[hit(10)]);
    run_frames(&mut app, 1);
    // 60 frames x 16 ms = 0.96 s, longer than the 0.9 s of invulnerability.
    run_frames(&mut app, 60);
    assert_eq!(player_invulnerability(&mut app), 0.0);
    let before = player_hp(&mut app);
    inject_damage(&mut app, &[hit(10)]);
    run_frames(&mut app, 1);
    assert_eq!(player_hp(&mut app), before - 10);
}

// ---------------------------------------------------------------- A1
// A recognised command is only a request (`SkillActivated`), validated with
// the state of the previous frame. `begin_skill` must re-validate it with
// the current state and, only when it really starts, pay the costs, start
// the cooldown, lock the caster and confirm it with `SkillStarted` (audit
// A1). Effects (hitboxes, projectiles, heals) react only to `SkillStarted`,
// and every activation ends in exactly one `CommandResolved`.
//
// Requests are injected inside `GameSet::Commands` (same timing as the
// recogniser) or produced by real key presses. Regeneration accumulators
// are reset right before the frame under test, so a single 16 ms frame
// never regenerates a whole point and resource values are exact.

/// Test-only skill with both an MP and a Hearts cost (never in RON).
const DUAL_COST_SKILL: &str = "test_dual_cost";
const DUAL_COST_MP: i32 = 5;
const DUAL_COST_HEARTS: i32 = 3;

fn dual_cost_skill() -> SkillDefinition {
    SkillDefinition {
        id: DUAL_COST_SKILL.to_owned(),
        name: "Dual Cost".to_owned(),
        description: String::new(),
        category: SkillCategory::ElementalSpell,
        command: None,
        timing: CommandTiming::default(),
        priority: 0,
        mp_cost: DUAL_COST_MP,
        hearts_cost: DUAL_COST_HEARTS,
        cooldown: 1.0,
        damage_kind: DamageKind::Magical,
        element: Element::Arcane,
        base_damage: 1,
        damage_multiplier: 1.0,
        range: 0.0,
        execution_time: 0.3,
        animation: String::new(),
        vfx: String::new(),
        effect: SkillEffect::Projectile {
            speed: 100.0,
            radius: 4.0,
            lifetime: 0.5,
        },
        inflicts: None,
        requirements: Default::default(),
        usable_in: vec![ActionPhase::Grounded],
        acquisition: Acquisition::Discovery,
        command_revealed_on_learn: true,
    }
}

/// Skill ids to request on the next frame, in order.
#[derive(Resource, Default)]
struct PendingActivations(Vec<String>);

fn emit_pending_activations(
    mut pending: ResMut<PendingActivations>,
    player: Query<Entity, With<Player>>,
    mut activated: MessageWriter<SkillActivated>,
) {
    let Ok(player) = player.single() else {
        return;
    };
    for skill_id in pending.0.drain(..) {
        activated.write(SkillActivated {
            caster: player,
            skill_id,
        });
    }
}

/// Everything the command pipeline reported since the last reset.
#[derive(Resource, Default)]
struct Observed {
    resolved: Vec<CommandOutcome>,
    started: Vec<String>,
    basic: usize,
}

fn observe_command_messages(
    mut observed: ResMut<Observed>,
    mut resolved: MessageReader<CommandResolved>,
    mut started: MessageReader<SkillStarted>,
    mut basic: MessageReader<BasicAttackRequested>,
) {
    observed
        .resolved
        .extend(resolved.read().map(|r| r.outcome.clone()));
    observed
        .started
        .extend(started.read().map(|s| s.skill_id.clone()));
    observed.basic += basic.read().count();
}

/// App after the warm-up, with the injector, the observer and the synthetic
/// dual-cost skill learned. Nothing observed yet.
fn skill_app() -> App {
    let mut app = app();
    app.world_mut()
        .resource_mut::<GameData>()
        .skills
        .insert(dual_cost_skill())
        .unwrap_or_else(|e| panic!("synthetic skill rejected: {e}"));
    app.init_resource::<PendingActivations>()
        .init_resource::<Observed>()
        .add_systems(Update, emit_pending_activations.in_set(GameSet::Commands))
        .add_systems(
            Update,
            observe_command_messages.after(GameSet::Presentation),
        );
    run_frames(&mut app, 60);
    with_player(&mut app, |_, book, _| {
        book.learned.grant(&dual_cost_skill());
    });
    reset_observed(&mut app);
    app
}

fn reset_observed(app: &mut App) {
    *app.world_mut().resource_mut::<Observed>() = Observed::default();
}

fn observed(app: &App) -> &Observed {
    app.world().resource::<Observed>()
}

fn inject_activations(app: &mut App, ids: &[&str]) {
    app.world_mut()
        .resource_mut::<PendingActivations>()
        .0
        .extend(ids.iter().map(|id| (*id).to_owned()));
}

/// Mutable access to the player's vitals, skills and state.
fn with_player(app: &mut App, f: impl FnOnce(&mut Vitals, &mut SkillBook, &mut ActionState)) {
    let world = app.world_mut();
    let (mut vitals, mut book, mut state) = world
        .query_filtered::<(&mut Vitals, &mut SkillBook, &mut ActionState), With<Player>>()
        .single_mut(world)
        .unwrap_or_else(|e| panic!("player missing: {e}"));
    f(&mut vitals, &mut book, &mut state);
}

/// Makes the next frame regenerate nothing (see the section comment).
fn freeze_regeneration(app: &mut App) {
    with_player(app, |vitals, _, _| {
        vitals.hp_regen = RegenAccumulator::default();
        vitals.mp_regen = RegenAccumulator::default();
    });
}

fn skill_def(app: &App, id: &str) -> SkillDefinition {
    app.world()
        .resource::<GameData>()
        .skills
        .get(id)
        .cloned()
        .unwrap_or_else(|| panic!("skill `{id}` missing from the data"))
}

/// What a skill start can change, captured around the frame under test.
#[derive(Debug, Clone, PartialEq)]
struct SkillSnapshot {
    hp: i32,
    mp: i32,
    hearts: i32,
    velocity_x: f32,
    state: PlayerState,
    cooldown: f32,
    player_hitboxes: usize,
    projectiles: usize,
}

fn snapshot(app: &mut App, skill: &str) -> SkillSnapshot {
    let world = app.world_mut();
    let (player, hp, mp, hearts, velocity_x, state, cooldown) = world
        .query_filtered::<(Entity, &Vitals, &Velocity, &ActionState, &SkillBook), With<Player>>()
        .single(world)
        .map(|(e, v, vel, s, b)| {
            (
                e,
                v.hp.current(),
                v.mp.current(),
                v.hearts.current(),
                vel.0.x,
                s.0.state(),
                b.cooldowns.remaining(skill),
            )
        })
        .unwrap_or_else(|e| panic!("player missing: {e}"));
    let player_hitboxes = world
        .query::<&Hitbox>()
        .iter(world)
        .filter(|h| h.owner == player)
        .count();
    let projectiles = world.query::<&Projectile>().iter(world).count();
    SkillSnapshot {
        hp,
        mp,
        hearts,
        velocity_x,
        state,
        cooldown,
        player_hitboxes,
        projectiles,
    }
}

/// Injects `ids`, runs exactly one frame and returns the snapshots taken
/// right before and right after it (for `skill`).
fn run_injected(app: &mut App, ids: &[&str], skill: &str) -> (SkillSnapshot, SkillSnapshot) {
    freeze_regeneration(app);
    let before = snapshot(app, skill);
    inject_activations(app, ids);
    run_frames(app, 1);
    (before, snapshot(app, skill))
}

/// Everything a rejected activation must leave untouched.
fn assert_rejected(
    app: &App,
    def: &SkillDefinition,
    before: &SkillSnapshot,
    after: &SkillSnapshot,
    expected: &CommandOutcome,
) {
    let seen = observed(app);
    assert!(seen.started.is_empty(), "skill started: {:?}", seen.started);
    assert_eq!(
        seen.resolved,
        vec![expected.clone()],
        "exactly one CommandResolved with the rejection"
    );
    assert!(!seen.resolved.iter().any(CommandOutcome::is_executed));
    assert_eq!(after.mp, before.mp, "MP spent by a rejected skill");
    assert_eq!(
        after.hearts, before.hearts,
        "Hearts spent by a rejected skill"
    );
    assert_eq!(after.hp, before.hp, "HP changed by a rejected skill");
    assert!(
        after.cooldown <= before.cooldown,
        "cooldown (re)started: {} -> {}",
        before.cooldown,
        after.cooldown
    );
    assert_eq!(
        after.player_hitboxes, before.player_hitboxes,
        "hitbox spawned"
    );
    assert_eq!(after.projectiles, before.projectiles, "projectile spawned");
    assert_ne!(after.state, PlayerState::Casting, "caster locked");
    if let SkillEffect::MeleeStrike { lunge_speed, .. } = def.effect
        && lunge_speed > 0.0
    {
        assert!(
            (after.velocity_x.abs() - lunge_speed).abs() > 1.0,
            "lunge applied: {}",
            after.velocity_x
        );
    }
}

/// Buffers ↓ ↘ → (→ still held when this returns).
fn input_quarter_circle_forward(app: &mut App) {
    hold(app, &[KeyCode::ArrowDown], 3);
    hold(app, &[KeyCode::ArrowDown, KeyCode::ArrowRight], 3);
    press(app, KeyCode::ArrowRight);
    run_frames(app, 2);
}

/// Buffers ↓ N ↓ (↓ still held when this returns).
fn input_down_neutral_down(app: &mut App) {
    hold(app, &[KeyCode::ArrowDown], 3);
    run_frames(app, 3);
    press(app, KeyCode::ArrowDown);
    run_frames(app, 2);
}

/// Presses `keys` together for exactly one frame; returns the snapshots
/// taken right before and right after that frame.
fn press_together(app: &mut App, keys: &[KeyCode], skill: &str) -> (SkillSnapshot, SkillSnapshot) {
    freeze_regeneration(app);
    reset_observed(app);
    let before = snapshot(app, skill);
    hold(app, keys, 1);
    (before, snapshot(app, skill))
}

#[test]
fn valid_technique_starts_in_the_same_frame() {
    let mut app = skill_app();
    let def = skill_def(&app, "crescent_slash");
    input_quarter_circle_forward(&mut app);
    let (before, after) = press_together(&mut app, &[KeyCode::KeyX], "crescent_slash");
    release(&mut app, KeyCode::ArrowRight);

    let seen = observed(&app);
    assert_eq!(seen.started, vec!["crescent_slash".to_owned()]);
    assert_eq!(
        seen.resolved,
        vec![CommandOutcome::Executed {
            id: "crescent_slash".to_owned()
        }]
    );
    assert_eq!(seen.basic, 0);
    assert_eq!(after.player_hitboxes, before.player_hitboxes + 1);
    assert_eq!(after.state, PlayerState::Casting);
    assert!(after.cooldown > 0.0);
    let SkillEffect::MeleeStrike { lunge_speed, .. } = def.effect else {
        panic!("crescent_slash should be a melee strike");
    };
    assert_eq!(
        after.velocity_x, lunge_speed,
        "lunge forward (facing right)"
    );
    assert_eq!(
        hud_command_text(&mut app).as_deref(),
        Some("Crescent Slash!")
    );
}

#[test]
fn valid_spell_pays_and_fires_in_the_same_frame() {
    let mut app = skill_app();
    let def = skill_def(&app, "ember_bolt");
    input_quarter_circle_forward(&mut app);
    let (before, after) = press_together(&mut app, &[KeyCode::KeyC], "ember_bolt");
    release(&mut app, KeyCode::ArrowRight);

    let seen = observed(&app);
    assert_eq!(seen.started, vec!["ember_bolt".to_owned()]);
    assert_eq!(
        seen.resolved,
        vec![CommandOutcome::Executed {
            id: "ember_bolt".to_owned()
        }]
    );
    assert_eq!(before.mp - after.mp, def.mp_cost);
    assert_eq!(after.projectiles, before.projectiles + 1);
    assert!(after.cooldown > 0.0);
}

#[test]
fn skill_requested_in_an_incompatible_state_is_rejected() {
    let mut app = skill_app();
    let def = skill_def(&app, "crescent_slash");
    with_player(&mut app, |_, _, state| {
        state
            .start_action(PlayerState::Dodging, 0.5)
            .unwrap_or_else(|e| panic!("cannot force dodge: {e}"));
    });
    let (before, after) = run_injected(&mut app, &["crescent_slash"], "crescent_slash");
    assert_rejected(
        &app,
        &def,
        &before,
        &after,
        &CommandOutcome::NotAllowedNow {
            id: "crescent_slash".to_owned(),
        },
    );
    assert_eq!(after.state, PlayerState::Dodging);
}

#[test]
fn dodge_and_spell_in_the_same_frame_only_dodges() {
    let mut app = skill_app();
    let def = skill_def(&app, "ember_bolt");
    input_quarter_circle_forward(&mut app);
    let (before, after) = press_together(&mut app, &[KeyCode::KeyS, KeyCode::KeyC], "ember_bolt");
    release(&mut app, KeyCode::ArrowRight);
    assert_rejected(
        &app,
        &def,
        &before,
        &after,
        &CommandOutcome::NotAllowedNow {
            id: "ember_bolt".to_owned(),
        },
    );
    assert_eq!(after.state, PlayerState::Dodging);
}

#[test]
fn guard_and_spell_in_the_same_frame_only_guards() {
    // Existing rule: pressing guard clears the command buffer before the
    // MAGIC press is recorded, so the recogniser only sees a lone button
    // and never requests the spell. Observed outcome: `Invalid`.
    let mut app = skill_app();
    let def = skill_def(&app, "ember_bolt");
    input_quarter_circle_forward(&mut app);
    let (before, after) = press_together(&mut app, &[KeyCode::KeyA, KeyCode::KeyC], "ember_bolt");
    release(&mut app, KeyCode::ArrowRight);
    assert_rejected(&app, &def, &before, &after, &CommandOutcome::Invalid);
    assert_eq!(after.state, PlayerState::Guarding);
}

#[test]
fn jump_and_ground_technique_in_the_same_frame_only_jumps() {
    let mut app = skill_app();
    let def = skill_def(&app, "crescent_slash");
    input_quarter_circle_forward(&mut app);
    let (before, after) =
        press_together(&mut app, &[KeyCode::KeyZ, KeyCode::KeyX], "crescent_slash");
    release(&mut app, KeyCode::ArrowRight);
    assert_rejected(
        &app,
        &def,
        &before,
        &after,
        &CommandOutcome::NotAllowedNow {
            id: "crescent_slash".to_owned(),
        },
    );
    assert_eq!(after.state, PlayerState::Jumping);
    assert_eq!(observed(&app).basic, 0);
    // The HUD shows the real reason, never "Crescent Slash!".
    assert_eq!(
        hud_command_text(&mut app).as_deref(),
        Some("Crescent Slash - cannot use now")
    );
}

/// Command feedback line currently shown by the HUD.
fn hud_command_text(app: &mut App) -> Option<String> {
    app.world()
        .resource::<HudMessages>()
        .command
        .as_ref()
        .map(|(text, _)| text.clone())
}

#[test]
fn jump_and_ground_spell_in_the_same_frame_does_not_heal() {
    let mut app = skill_app();
    let def = skill_def(&app, "mending_light");
    with_player(&mut app, |vitals, _, _| {
        vitals.hp.deplete(40);
    });
    input_down_neutral_down(&mut app);
    let (before, after) =
        press_together(&mut app, &[KeyCode::KeyZ, KeyCode::KeyC], "mending_light");
    release(&mut app, KeyCode::ArrowDown);
    assert_rejected(
        &app,
        &def,
        &before,
        &after,
        &CommandOutcome::NotAllowedNow {
            id: "mending_light".to_owned(),
        },
    );
    assert_eq!(after.state, PlayerState::Jumping);
}

#[test]
fn unlearned_skill_request_is_rejected() {
    let mut app = skill_app();
    let def = skill_def(&app, "rising_edge");
    let (before, after) = run_injected(&mut app, &["rising_edge"], "rising_edge");
    assert_rejected(
        &app,
        &def,
        &before,
        &after,
        &CommandOutcome::NotLearned {
            id: "rising_edge".to_owned(),
        },
    );
}

#[test]
fn skill_on_cooldown_is_rejected_without_restarting_it() {
    let mut app = skill_app();
    let def = skill_def(&app, "crescent_slash");
    with_player(&mut app, |_, book, _| {
        book.cooldowns.start("crescent_slash", 5.0);
    });
    let (before, after) = run_injected(&mut app, &["crescent_slash"], "crescent_slash");
    let seen = observed(&app);
    assert!(
        matches!(
            seen.resolved.as_slice(),
            [CommandOutcome::OnCooldown { id, remaining }]
                if id == "crescent_slash" && *remaining > 4.9
        ),
        "{:?}",
        seen.resolved
    );
    let expected = seen.resolved[0].clone();
    assert_rejected(&app, &def, &before, &after, &expected);
    assert!(after.cooldown < before.cooldown, "cooldown keeps running");
}

#[test]
fn spell_without_mp_is_rejected() {
    let mut app = skill_app();
    let def = skill_def(&app, "ember_bolt");
    with_player(&mut app, |vitals, _, _| {
        let all = vitals.mp.current();
        vitals.mp.deplete(all);
    });
    let (before, after) = run_injected(&mut app, &["ember_bolt"], "ember_bolt");
    assert_rejected(
        &app,
        &def,
        &before,
        &after,
        &CommandOutcome::InsufficientResources {
            id: "ember_bolt".to_owned(),
            resource: ResourceKind::Mp,
            required: def.mp_cost,
            available: 0,
        },
    );
}

#[test]
fn technique_without_hearts_is_rejected() {
    let mut app = skill_app();
    let def = skill_def(&app, "rising_edge");
    with_player(&mut app, |vitals, book, _| {
        book.learned.grant(&def);
        let keep = def.hearts_cost - 1;
        let excess = vitals.hearts.current() - keep;
        vitals.hearts.deplete(excess);
    });
    let (before, after) = run_injected(&mut app, &["rising_edge"], "rising_edge");
    assert_rejected(
        &app,
        &def,
        &before,
        &after,
        &CommandOutcome::InsufficientResources {
            id: "rising_edge".to_owned(),
            resource: ResourceKind::Hearts,
            required: def.hearts_cost,
            available: def.hearts_cost - 1,
        },
    );
}

#[test]
fn missing_hearts_do_not_spend_mp() {
    // MP is enough, Hearts are not: the old `try_spend(mp) && try_spend(hearts)`
    // spent the MP before noticing the missing Hearts.
    let mut app = skill_app();
    let def = dual_cost_skill();
    with_player(&mut app, |vitals, _, _| {
        let excess = vitals.hearts.current() - 1;
        vitals.hearts.deplete(excess);
    });
    let (before, after) = run_injected(&mut app, &[DUAL_COST_SKILL], DUAL_COST_SKILL);
    assert!(before.mp >= DUAL_COST_MP);
    assert_rejected(
        &app,
        &def,
        &before,
        &after,
        &CommandOutcome::InsufficientResources {
            id: DUAL_COST_SKILL.to_owned(),
            resource: ResourceKind::Hearts,
            required: DUAL_COST_HEARTS,
            available: 1,
        },
    );
}

#[test]
fn missing_mp_and_hearts_spend_nothing() {
    let mut app = skill_app();
    let def = dual_cost_skill();
    with_player(&mut app, |vitals, _, _| {
        let mp_excess = vitals.mp.current() - (DUAL_COST_MP - 1);
        vitals.mp.deplete(mp_excess);
        let hearts_excess = vitals.hearts.current() - 1;
        vitals.hearts.deplete(hearts_excess);
    });
    let (before, after) = run_injected(&mut app, &[DUAL_COST_SKILL], DUAL_COST_SKILL);
    assert_rejected(
        &app,
        &def,
        &before,
        &after,
        &CommandOutcome::InsufficientResources {
            id: DUAL_COST_SKILL.to_owned(),
            resource: ResourceKind::Mp,
            required: DUAL_COST_MP,
            available: DUAL_COST_MP - 1,
        },
    );
}

#[test]
fn dual_cost_skill_pays_both_costs_when_affordable() {
    let mut app = skill_app();
    let (before, after) = run_injected(&mut app, &[DUAL_COST_SKILL], DUAL_COST_SKILL);
    let seen = observed(&app);
    assert_eq!(seen.started, vec![DUAL_COST_SKILL.to_owned()]);
    assert_eq!(
        seen.resolved,
        vec![CommandOutcome::Executed {
            id: DUAL_COST_SKILL.to_owned()
        }]
    );
    assert_eq!(before.mp - after.mp, DUAL_COST_MP);
    assert_eq!(before.hearts - after.hearts, DUAL_COST_HEARTS);
    assert_eq!(after.projectiles, before.projectiles + 1);
}

#[test]
fn unknown_skill_request_is_invalid() {
    let mut app = skill_app();
    let def = skill_def(&app, "crescent_slash");
    let (before, after) = run_injected(&mut app, &["no_such_skill"], "no_such_skill");
    assert_rejected(&app, &def, &before, &after, &CommandOutcome::Invalid);
}

#[test]
fn second_activation_in_the_same_frame_is_rejected() {
    let mut app = skill_app();
    let ember = skill_def(&app, "ember_bolt");
    let (before, after) = run_injected(&mut app, &["crescent_slash", "ember_bolt"], "ember_bolt");
    let seen = observed(&app);
    assert_eq!(seen.started, vec!["crescent_slash".to_owned()]);
    assert_eq!(
        seen.resolved,
        vec![
            CommandOutcome::Executed {
                id: "crescent_slash".to_owned()
            },
            CommandOutcome::NotAllowedNow {
                id: "ember_bolt".to_owned()
            },
        ]
    );
    assert_eq!(after.mp, before.mp, "ember bolt MP spent");
    assert_eq!(after.projectiles, before.projectiles, "ember bolt fired");
    assert_eq!(after.cooldown, 0.0, "ember bolt cooldown started");
    assert_eq!(after.player_hitboxes, before.player_hitboxes + 1);
    assert_eq!(after.state, PlayerState::Casting);
    assert!(ember.mp_cost > 0);
}

#[test]
fn rejected_heal_restores_nothing() {
    let mut app = skill_app();
    let def = skill_def(&app, "mending_light");
    with_player(&mut app, |vitals, _, state| {
        vitals.hp.deplete(40);
        state
            .start_action(PlayerState::Dodging, 0.5)
            .unwrap_or_else(|e| panic!("cannot force dodge: {e}"));
    });
    let (before, after) = run_injected(&mut app, &["mending_light"], "mending_light");
    let mut max_hp = 0;
    with_player(&mut app, |vitals, _, _| max_hp = vitals.hp.max());
    assert!(
        before.hp < max_hp,
        "the heal must have something to restore"
    );
    assert_rejected(
        &app,
        &def,
        &before,
        &after,
        &CommandOutcome::NotAllowedNow {
            id: "mending_light".to_owned(),
        },
    );
}

#[test]
fn unlearned_command_still_falls_back_to_basic_attack() {
    // D-GAME-003: ↓ ↑ + ATTACK with Rising Edge not learned reports
    // NotLearned and still performs the basic attack.
    let mut app = skill_app();
    hold(&mut app, &[KeyCode::ArrowDown], 3);
    press(&mut app, KeyCode::ArrowUp);
    run_frames(&mut app, 2);
    let (before, after) = press_together(&mut app, &[KeyCode::KeyX], "rising_edge");
    release(&mut app, KeyCode::ArrowUp);
    let seen = observed(&app);
    assert_eq!(
        seen.resolved,
        vec![CommandOutcome::NotLearned {
            id: "rising_edge".to_owned()
        }]
    );
    assert_eq!(seen.basic, 1);
    assert!(seen.started.is_empty());
    assert_eq!(after.player_hitboxes, before.player_hitboxes + 1);
    assert_eq!(after.state, PlayerState::Attacking);
}

// ---------------------------------------------------------------- A2
// Hitboxes attached to the player (melee strikes) must disappear as soon as
// the action is interrupted by damage (Hurt) or death (Dead), in the same
// frame the damage is applied, so they never keep hitting nor follow the
// player to the spawn point on respawn (audit A2). Projectiles already
// launched and enemy hitboxes are not affected, and blocked damage does not
// interrupt anything. Skills are started with the A1 injector and damage is
// injected with the A3 one; the player stays at the spawn, far from enemies.

/// Every `(attacker, target)` pair of the `DamageDealt` messages seen.
#[derive(Resource, Default)]
struct DamageLog(Vec<(Entity, Entity)>);

fn log_damage(mut log: ResMut<DamageLog>, mut damage: MessageReader<DamageDealt>) {
    log.0.extend(damage.read().map(|d| (d.attacker, d.target)));
}

/// `skill_app` plus the damage injector and the damage log.
fn combat_app() -> App {
    let mut app = skill_app();
    app.init_resource::<PendingDamage>()
        .init_resource::<DamageLog>()
        .add_systems(Update, emit_pending_damage.in_set(GameSet::Combat))
        .add_systems(Update, log_damage.after(GameSet::Presentation));
    app
}

fn player_entity(app: &mut App) -> Entity {
    let world = app.world_mut();
    world
        .query_filtered::<Entity, With<Player>>()
        .single(world)
        .unwrap_or_else(|e| panic!("player missing: {e}"))
}

fn player_position(app: &mut App) -> Vec2 {
    let world = app.world_mut();
    world
        .query_filtered::<&Transform, With<Player>>()
        .single(world)
        .map(|t| t.translation.truncate())
        .unwrap_or_else(|e| panic!("player missing: {e}"))
}

fn set_player_invulnerability(app: &mut App, remaining: f32) {
    let world = app.world_mut();
    let mut invulnerable = world
        .query_filtered::<&mut Invulnerable, With<Player>>()
        .single_mut(world)
        .unwrap_or_else(|e| panic!("player missing: {e}"));
    invulnerable.remaining = remaining;
}

/// Hitboxes owned by the player (melee strikes and projectiles).
fn player_hitboxes(app: &mut App) -> Vec<Entity> {
    let player = player_entity(app);
    let world = app.world_mut();
    world
        .query::<(Entity, &Hitbox)>()
        .iter(world)
        .filter(|(_, h)| h.owner == player)
        .map(|(e, _)| e)
        .collect()
}

fn exists(app: &App, entity: Entity) -> bool {
    app.world().get_entity(entity).is_ok()
}

/// Starts Crescent Slash with the A1 injector and runs its first frame:
/// the player is casting with exactly one melee hitbox attached.
fn start_crescent_slash(app: &mut App) -> Entity {
    freeze_regeneration(app);
    inject_activations(app, &["crescent_slash"]);
    run_frames(app, 1);
    assert_eq!(observed(app).started, vec!["crescent_slash".to_owned()]);
    assert_eq!(player_state(app), PlayerState::Casting);
    let hitboxes = player_hitboxes(app);
    assert_eq!(hitboxes.len(), 1, "crescent slash should have one hitbox");
    let world = app.world_mut();
    assert!(
        world.get::<AttachedToOwner>(hitboxes[0]).is_some(),
        "the melee hitbox follows its owner"
    );
    hitboxes[0]
}

/// A target centred on the player that any hitbox around the player
/// overlaps (no `Enemy` marker, so no AI, contact damage or death handling).
fn spawn_dummy_target(app: &mut App) -> Entity {
    let center = player_position(app);
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
                size: Vec2::splat(400.0),
            },
            profile,
            Transform::from_translation(center.extend(0.0)),
        ))
        .id()
}

/// A melee hitbox attached to an enemy, like the ones enemy attacks spawn.
fn spawn_enemy_hitbox(app: &mut App) -> (Entity, Entity) {
    let world = app.world_mut();
    let (enemy, attack) = world
        .query_filtered::<(Entity, &CombatProfile), With<Enemy>>()
        .iter(world)
        .next()
        .map(|(e, p)| (e, p.physical.clone()))
        .unwrap_or_else(|| panic!("no enemy in the start room"));
    let hitbox = world
        .spawn((
            Hitbox {
                owner: enemy,
                team: Team::Enemy,
                size: Vec2::new(10.0, 10.0),
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
            AttachedToOwner { offset: Vec2::ZERO },
            Transform::default(),
        ))
        .id();
    (enemy, hitbox)
}

/// No attached hitbox may survive on an owner that is Hurt or Dead.
fn assert_no_interrupted_hitboxes(app: &mut App) {
    let world = app.world_mut();
    let owners: Vec<Entity> = world
        .query_filtered::<&Hitbox, With<AttachedToOwner>>()
        .iter(world)
        .map(|h| h.owner)
        .collect();
    for owner in owners {
        let state = world.get::<ActionState>(owner).map(|s| s.0.state());
        assert!(
            !matches!(state, Some(PlayerState::Hurt | PlayerState::Dead)),
            "attached hitbox survives on an interrupted owner ({state:?})"
        );
    }
}

#[test]
fn damage_interrupts_the_attack_and_removes_its_hitbox() {
    let mut app = combat_app();
    let hitbox = start_crescent_slash(&mut app);
    inject_damage(&mut app, &[hit(10)]);
    run_frames(&mut app, 1);
    assert_eq!(player_state(&mut app), PlayerState::Hurt);
    assert!(!exists(&app, hitbox), "interrupted hitbox still alive");
    assert!(player_hitboxes(&mut app).is_empty());
    assert_no_interrupted_hitboxes(&mut app);
}

#[test]
fn cancelled_hitbox_never_hits_afterwards() {
    let mut app = combat_app();
    let player = player_entity(&mut app);
    // Control: a target overlapping the slash is hit while it is active.
    let control = spawn_dummy_target(&mut app);
    start_crescent_slash(&mut app);
    assert!(
        app.world()
            .resource::<DamageLog>()
            .0
            .contains(&(player, control)),
        "the slash should hit a target overlapping it"
    );

    inject_damage(&mut app, &[hit(10)]);
    run_frames(&mut app, 1);
    assert_eq!(player_state(&mut app), PlayerState::Hurt);
    // A fresh target where the slash would still be: nothing may hit it for
    // the rest of the hitbox lifetime (0.24 s = 15 frames; run 30).
    let late = spawn_dummy_target(&mut app);
    app.world_mut().resource_mut::<DamageLog>().0.clear();
    for _ in 0..30 {
        run_frames(&mut app, 1);
        assert!(player_hitboxes(&mut app).is_empty());
    }
    let log = &app.world().resource::<DamageLog>().0;
    assert!(
        !log.iter().any(|(attacker, _)| *attacker == player),
        "cancelled hitbox dealt damage: {log:?}"
    );
    assert!(!log.contains(&(player, late)));
}

#[test]
fn same_frame_trade_keeps_the_hit_already_landed() {
    // Activation, the slash hitting a target (Combat) and the player being
    // hit (Combat → Resolution) all happen in one frame: the slash's hit
    // stands and only afterwards its hitbox is cancelled.
    let mut app = combat_app();
    let player = player_entity(&mut app);
    let target = spawn_dummy_target(&mut app);
    freeze_regeneration(&mut app);
    app.world_mut().resource_mut::<DamageLog>().0.clear();
    inject_activations(&mut app, &["crescent_slash"]);
    inject_damage(&mut app, &[hit(10)]);
    run_frames(&mut app, 1);
    assert_eq!(observed(&app).started, vec!["crescent_slash".to_owned()]);
    assert!(
        app.world()
            .resource::<DamageLog>()
            .0
            .contains(&(player, target)),
        "the hit landed in the same frame must stand"
    );
    assert_eq!(player_state(&mut app), PlayerState::Hurt);
    assert!(player_hitboxes(&mut app).is_empty());
}

#[test]
fn death_during_crescent_slash_removes_its_hitbox() {
    let mut app = combat_app();
    let hitbox = start_crescent_slash(&mut app);
    let lethal = player_hp(&mut app) + 50;
    inject_damage(&mut app, &[hit(lethal)]);
    run_frames(&mut app, 1);
    assert_eq!(player_state(&mut app), PlayerState::Dead);
    assert!(!exists(&app, hitbox), "hitbox survived the death");
    assert!(player_hitboxes(&mut app).is_empty());
    assert_no_interrupted_hitboxes(&mut app);
}

#[test]
fn respawn_does_not_bring_the_old_hitbox() {
    let mut app = combat_app();
    let hitbox = start_crescent_slash(&mut app);
    let lethal = player_hp(&mut app) + 50;
    inject_damage(&mut app, &[hit(lethal)]);
    run_frames(&mut app, 1);
    assert_eq!(player_state(&mut app), PlayerState::Dead);

    // R on the very next frame, while the slash would still be active.
    hold(&mut app, &[KeyCode::KeyR], 1);
    assert_ne!(player_state(&mut app), PlayerState::Dead);
    let spawn = {
        let world = app.world();
        let room = &world.resource::<CurrentRoom>().0;
        let def = world
            .resource::<GameData>()
            .rooms
            .get(room)
            .unwrap_or_else(|| panic!("current room `{room}` missing"));
        Vec2::new(def.player_spawn.0, def.player_spawn.1)
    };
    assert_eq!(player_position(&mut app).x, spawn.x, "respawned at spawn");
    assert!(!exists(&app, hitbox), "old hitbox carried to the respawn");
    assert!(player_hitboxes(&mut app).is_empty());
    let world = app.world_mut();
    let near_spawn: Vec<Vec2> = world
        .query_filtered::<&Transform, With<Hitbox>>()
        .iter(world)
        .map(|t| t.translation.truncate())
        .filter(|p| p.distance(spawn) < 200.0)
        .collect();
    assert!(near_spawn.is_empty(), "hitboxes near spawn: {near_spawn:?}");
}

#[test]
fn blocked_damage_does_not_interrupt_the_attack() {
    let mut app = combat_app();
    let hitbox = start_crescent_slash(&mut app);
    let hp = player_hp(&mut app);
    set_player_invulnerability(&mut app, 0.5);
    inject_damage(&mut app, &[hit(10)]);
    run_frames(&mut app, 1);
    assert_eq!(player_hp(&mut app), hp, "blocked hit dealt damage");
    assert_eq!(player_state(&mut app), PlayerState::Casting);
    assert!(exists(&app, hitbox), "blocked hit cancelled the attack");
    assert_eq!(player_hitboxes(&mut app), vec![hitbox]);
}

#[test]
fn dead_character_starts_no_attacks() {
    let mut app = combat_app();
    let lethal = player_hp(&mut app) + 50;
    inject_damage(&mut app, &[hit(lethal)]);
    run_frames(&mut app, 1);
    assert_eq!(player_state(&mut app), PlayerState::Dead);
    reset_observed(&mut app);
    let before = snapshot(&mut app, "ember_bolt");

    hold(&mut app, &[KeyCode::KeyX], 1);
    input_quarter_circle_forward(&mut app);
    hold(&mut app, &[KeyCode::KeyC], 1);
    release(&mut app, KeyCode::ArrowRight);
    for key in [KeyCode::KeyZ, KeyCode::KeyS, KeyCode::KeyA] {
        hold(&mut app, &[key], 1);
    }
    run_frames(&mut app, 1);

    let after = snapshot(&mut app, "ember_bolt");
    let seen = observed(&app);
    assert!(seen.started.is_empty(), "skill started: {:?}", seen.started);
    assert!(!seen.resolved.iter().any(CommandOutcome::is_executed));
    assert_eq!(after.player_hitboxes, 0, "hitbox spawned while dead");
    assert_eq!(after.projectiles, before.projectiles, "projectile spawned");
    assert_eq!(after.mp, before.mp, "MP spent while dead");
    assert_eq!(after.state, PlayerState::Dead);
}

#[test]
fn launched_projectile_survives_damage_to_its_owner() {
    let mut app = combat_app();
    freeze_regeneration(&mut app);
    inject_activations(&mut app, &["ember_bolt"]);
    run_frames(&mut app, 1);
    assert_eq!(observed(&app).started, vec!["ember_bolt".to_owned()]);
    let world = app.world_mut();
    let projectiles: Vec<Entity> = world
        .query_filtered::<Entity, With<Projectile>>()
        .iter(world)
        .collect();
    assert_eq!(projectiles.len(), 1, "ember bolt should be in flight");

    inject_damage(&mut app, &[hit(10)]);
    run_frames(&mut app, 1);
    assert_eq!(player_state(&mut app), PlayerState::Hurt);
    assert!(exists(&app, projectiles[0]), "projectile removed by damage");
}

#[test]
fn enemy_hitboxes_survive_when_the_player_is_hurt() {
    let mut app = combat_app();
    let slash = start_crescent_slash(&mut app);
    let (enemy, enemy_hitbox) = spawn_enemy_hitbox(&mut app);
    inject_damage(&mut app, &[hit(10)]);
    run_frames(&mut app, 1);
    assert_eq!(player_state(&mut app), PlayerState::Hurt);
    assert!(!exists(&app, slash), "player hitbox should be cancelled");
    assert!(exists(&app, enemy), "enemy should still exist");
    assert!(exists(&app, enemy_hitbox), "enemy hitbox removed");
}

#[test]
fn no_attached_hitbox_survives_on_an_interrupted_owner() {
    let mut app = combat_app();
    start_crescent_slash(&mut app);
    let (_, enemy_hitbox) = spawn_enemy_hitbox(&mut app);
    let lethal = player_hp(&mut app) + 50;
    inject_damage(&mut app, &[hit(lethal)]);
    run_frames(&mut app, 1);
    assert_eq!(player_state(&mut app), PlayerState::Dead);
    assert_no_interrupted_hitboxes(&mut app);
    // Whatever stays attached belongs to an owner that still exists and is
    // able to act (here, the enemy's hitbox).
    let world = app.world_mut();
    let survivors: Vec<(Entity, Entity)> = world
        .query_filtered::<(Entity, &Hitbox), With<AttachedToOwner>>()
        .iter(world)
        .map(|(e, h)| (e, h.owner))
        .collect();
    assert_eq!(
        survivors.iter().map(|(e, _)| *e).collect::<Vec<_>>(),
        vec![enemy_hitbox]
    );
    for (_, owner) in survivors {
        assert!(app.world().get_entity(owner).is_ok(), "orphan hitbox");
    }
}
