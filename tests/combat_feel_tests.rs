//! V2.4 — characterization of the combat feel (step 0 of
//! `docs/PLANO_V2_4_COMBAT_FEEL.md`).
//!
//! These tests pin how the basic attack, its hitbox, the state locks and the
//! technique cancel behave **today**, including known limitations, so that
//! every later V2.4 step can prove its defaults change nothing. Commands,
//! costs, cooldowns, invulnerability and received damage in general are
//! already covered by `app_smoke_tests.rs` and `command_parser_tests.rs`;
//! this file only adds what those do not pin.
//!
//! Timing: fixed 16 ms frames. The basic attack starts in `GameSet::Actions`
//! of the frame its key is pressed; the state timer ticks from the next frame
//! on, and the hitbox lifetime ticks in `GameSet::Combat` of the same frame.

use std::path::{Path, PathBuf};
use std::time::Duration;

use bevy::input::{InputPlugin as BevyInputPlugin, InputSystems};
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::app::plugins::GamePlugin;
use dark_castle_rpg::app::states::GameSet;
use dark_castle_rpg::combat::attacks::{ActiveStrike, StrikeCancelPolicy, WeaponStrike};
use dark_castle_rpg::combat::command_parser::CommandTiming;
use dark_castle_rpg::combat::damage::{DamageKind, DamageOutcome, HitSpec};
use dark_castle_rpg::combat::hitboxes::{
    AttachedToOwner, CombatProfile, HitKnockback, Hitbox, Hurtbox, Invulnerable, PendingStrike,
    RehitTimers, Team,
};
use dark_castle_rpg::combat::reaction::KnockbackIntensity;
use dark_castle_rpg::core::constants::{
    BASIC_ATTACK_HITBOX, BASIC_ATTACK_TIME, HITBOX_ACTIVE_FRACTION, PLAYER_HURT_KNOCKBACK,
    PLAYER_INVULNERABILITY,
};
use dark_castle_rpg::core::events::{CommandResolved, DamageDealt, SkillActivated, SkillStarted};
use dark_castle_rpg::core::resources::GameData;
use dark_castle_rpg::inventory::items::{ItemCatalog, ItemCategory, ItemDataFile};
use dark_castle_rpg::inventory::weapons::WeaponClass;
use dark_castle_rpg::magic::elements::Element;
use dark_castle_rpg::player::attributes::RegenAccumulator;
use dark_castle_rpg::player::components::{
    ActionState, Belongings, CharacterSheet, Player, SkillBook, Vitals,
};
use dark_castle_rpg::player::state::{ActionPhase, Facing, PlayerState};
use dark_castle_rpg::skills::definitions::{
    Acquisition, SkillCategory, SkillDefinition, SkillEffect,
};
use dark_castle_rpg::skills::execution::CommandOutcome;
use dark_castle_rpg::world::physics::{Body, FacingDirection, Grounded, Velocity};

const FRAME: Duration = Duration::from_millis(16);

// ---------------------------------------------------------------- harness

#[derive(Resource, Default)]
struct PendingKeys {
    press: Vec<KeyCode>,
    release: Vec<KeyCode>,
}

fn apply_pending_keys(mut pending: ResMut<PendingKeys>, mut keys: ResMut<ButtonInput<KeyCode>>) {
    for key in pending.release.drain(..) {
        keys.release(key);
    }
    for key in pending.press.drain(..) {
        keys.press(key);
    }
}

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
            knockback: Default::default(),
        });
    }
}

#[derive(Resource, Default)]
struct Observed {
    resolved: Vec<CommandOutcome>,
    started: Vec<String>,
}

fn observe(
    mut observed: ResMut<Observed>,
    mut resolved: MessageReader<CommandResolved>,
    mut started: MessageReader<SkillStarted>,
) {
    observed
        .resolved
        .extend(resolved.read().map(|r| r.outcome.clone()));
    observed
        .started
        .extend(started.read().map(|s| s.skill_id.clone()));
}

/// Test-only technique usable while attacking, with both costs (never in RON).
const CANCEL_SKILL: &str = "test_cancel_technique";
const CANCEL_MP: i32 = 4;
const CANCEL_HEARTS: i32 = 2;

fn cancel_skill() -> SkillDefinition {
    SkillDefinition {
        id: CANCEL_SKILL.to_owned(),
        name: "Cancel Test".to_owned(),
        description: String::new(),
        category: SkillCategory::SwordTechnique,
        command: None,
        timing: CommandTiming::default(),
        priority: 0,
        mp_cost: CANCEL_MP,
        hearts_cost: CANCEL_HEARTS,
        cooldown: 1.0,
        damage_kind: DamageKind::Physical,
        element: Element::Physical,
        base_damage: 1,
        damage_multiplier: 1.0,
        range: 40.0,
        execution_time: 0.3,
        animation: String::new(),
        vfx: String::new(),
        effect: SkillEffect::MeleeStrike {
            width: 40.0,
            height: 30.0,
            lunge_speed: 0.0,
        },
        inflicts: None,
        requirements: Default::default(),
        usable_in: vec![ActionPhase::Grounded, ActionPhase::Attacking],
        acquisition: Acquisition::Discovery,
        command_revealed_on_learn: true,
        weapons: Vec::new(),
    }
}

/// The real game (single room), standing idle at the spawn after a warm-up,
/// with injectors for skill requests and damage, and the test technique
/// learned.
fn app() -> App {
    app_with(|_| {})
}

/// [`app`] with the loaded data adjusted in memory first.
fn app_with(patch: impl FnOnce(&mut GameData)) -> App {
    let mut data = GameData::load(Path::new(env!("CARGO_MANIFEST_DIR")))
        .unwrap_or_else(|e| panic!("game data failed to load: {e}"));
    patch(&mut data);
    data.skills
        .insert(cancel_skill())
        .unwrap_or_else(|e| panic!("synthetic skill rejected: {e}"));
    let mut app = App::new();
    dark_castle_rpg::input::use_legacy_controls(&mut app);
    app.add_plugins((MinimalPlugins, StatesPlugin, BevyInputPlugin))
        .insert_resource(TimeUpdateStrategy::ManualDuration(FRAME))
        .insert_resource(data)
        .init_resource::<PendingKeys>()
        .init_resource::<PendingActivations>()
        .init_resource::<PendingDamage>()
        .init_resource::<Observed>()
        .add_systems(PreUpdate, apply_pending_keys.after(InputSystems))
        .add_systems(Update, emit_pending_activations.in_set(GameSet::Commands))
        .add_systems(Update, emit_pending_damage.in_set(GameSet::Combat))
        .add_systems(Update, observe.after(GameSet::Presentation))
        .add_plugins(GamePlugin);
    run(&mut app, 60);
    with_player(&mut app, |_, book, _| book.learned.grant(&cancel_skill()));
    assert_eq!(state(&mut app), PlayerState::Idle, "warm-up must end idle");
    reset_observed(&mut app);
    app
}

fn run(app: &mut App, frames: usize) {
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
    run(app, frames);
    for key in keys {
        release(app, *key);
    }
}

fn with_player(app: &mut App, f: impl FnOnce(&mut Vitals, &mut SkillBook, &mut ActionState)) {
    let world = app.world_mut();
    let (mut vitals, mut book, mut state) = world
        .query_filtered::<(&mut Vitals, &mut SkillBook, &mut ActionState), With<Player>>()
        .single_mut(world)
        .unwrap_or_else(|e| panic!("player missing: {e}"));
    f(&mut vitals, &mut book, &mut state);
}

fn freeze_regeneration(app: &mut App) {
    with_player(app, |vitals, _, _| {
        vitals.hp_regen = RegenAccumulator::default();
        vitals.mp_regen = RegenAccumulator::default();
    });
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
        .map(|s| s.0.state())
        .unwrap_or_else(|e| panic!("player missing: {e}"))
}

fn velocity(app: &mut App) -> Vec2 {
    let world = app.world_mut();
    world
        .query_filtered::<&Velocity, With<Player>>()
        .single(world)
        .map(|v| v.0)
        .unwrap_or_else(|e| panic!("player missing: {e}"))
}

fn on_ground(app: &mut App) -> bool {
    let world = app.world_mut();
    world
        .query_filtered::<&Grounded, With<Player>>()
        .single(world)
        .map(|g| g.on_ground)
        .unwrap_or_else(|e| panic!("player missing: {e}"))
}

fn invulnerability(app: &mut App) -> f32 {
    let world = app.world_mut();
    world
        .query_filtered::<&Invulnerable, With<Player>>()
        .single(world)
        .map(|i| i.remaining)
        .unwrap_or_else(|e| panic!("player missing: {e}"))
}

fn vitals(app: &mut App) -> (i32, i32, i32) {
    let world = app.world_mut();
    world
        .query_filtered::<&Vitals, With<Player>>()
        .single(world)
        .map(|v| (v.hp.current(), v.mp.current(), v.hearts.current()))
        .unwrap_or_else(|e| panic!("player missing: {e}"))
}

/// The player's basic-attack hitboxes: (entity, hitbox, attachment).
fn strikes(app: &mut App) -> Vec<(Entity, Hitbox, AttachedToOwner)> {
    let world = app.world_mut();
    world
        .query_filtered::<(Entity, &Hitbox, &AttachedToOwner), With<WeaponStrike>>()
        .iter(world)
        .map(|(e, h, a)| (e, h.clone(), *a))
        .collect()
}

/// Every hitbox owned by the player (strikes and techniques).
fn player_hitboxes(app: &mut App) -> usize {
    let owner = player(app);
    let world = app.world_mut();
    world
        .query::<&Hitbox>()
        .iter(world)
        .filter(|h| h.owner == owner)
        .count()
}

fn attack(app: &mut App) {
    hold(app, &[KeyCode::KeyX], 1);
}

/// Forgets what was observed so far (a lone ATTACK press resolves `Invalid`).
fn reset_observed(app: &mut App) {
    *app.world_mut().resource_mut::<Observed>() = Observed::default();
}

/// Frames `duration` lasts when it starts in the current frame and is first
/// ticked in the next one (the player's action timer).
fn frames_of_action(duration: f32) -> usize {
    let dt = FRAME.as_secs_f32();
    // Alive after frame 1 plus every following frame that leaves time > 0.
    1 + (duration / dt).floor() as usize
}

/// Frames a hitbox exists when it is also ticked in its first frame.
fn frames_of_hitbox(lifetime: f32) -> usize {
    let dt = FRAME.as_secs_f32();
    (lifetime / dt).floor() as usize
}

// ---------------------------------------------------------------- Iron Sword

#[test]
fn iron_sword_strikes_like_the_pre_v2_basic_attack() {
    let mut app = app();
    let world = app.world_mut();
    let (belongings, sheet) = world
        .query_filtered::<(&Belongings, &CharacterSheet), With<Player>>()
        .single(world)
        .map(|(b, s)| (b.clone(), s.derived().attack_speed))
        .unwrap_or_else(|e| panic!("player missing: {e}"));
    assert_eq!(sheet, 1.0, "the starting character attacks at speed 1");
    let data = world.resource::<GameData>();
    let strike = data.items.strike_profile(&belongings.equipment, sheet);
    assert_eq!(strike.category, Some(ItemCategory::Sword));
    assert_eq!(strike.action_time, BASIC_ATTACK_TIME);
    assert_eq!(
        strike.active_time,
        BASIC_ATTACK_TIME * HITBOX_ACTIVE_FRACTION
    );
    assert_eq!(strike.size, BASIC_ATTACK_HITBOX);
    assert_eq!(strike.multiplier, 1.0);
    assert_eq!(strike.element, Element::Physical);
}

// ---------------------------------------------------------------- basic attack

#[test]
fn basic_attack_hits_from_its_first_frame_and_locks_for_the_whole_swing() {
    let mut app = app();
    attack(&mut app);
    assert_eq!(state(&mut app), PlayerState::Attacking);
    let first = strikes(&mut app);
    assert_eq!(first.len(), 1, "the hitbox exists in the press frame");
    let (hitbox_entity, hitbox, _) = &first[0];
    assert!(hitbox.pierce, "D-009: basic strikes pierce");
    assert!(hitbox.already_hit.is_empty());

    let active = BASIC_ATTACK_TIME * HITBOX_ACTIVE_FRACTION;
    let hitbox_frames = frames_of_hitbox(active);
    let action_frames = frames_of_action(BASIC_ATTACK_TIME);
    assert_eq!((hitbox_frames, action_frames), (10, 18));

    let mut alive = 1;
    let mut attacking = 1;
    for _ in 0..30 {
        run(&mut app, 1);
        if app.world().get_entity(*hitbox_entity).is_ok() {
            alive += 1;
        }
        if state(&mut app) == PlayerState::Attacking {
            attacking += 1;
        }
    }
    assert_eq!(alive, hitbox_frames, "frames with the hitbox");
    assert_eq!(attacking, action_frames, "frames locked in Attacking");
    assert_eq!(state(&mut app), PlayerState::Idle);
}

#[test]
fn nothing_cancels_the_basic_attack_except_techniques() {
    // Jump, dodge, guard and a second attack are ignored while attacking;
    // the character is never stuck: it is idle once the timer ends.
    let mut app = app();
    attack(&mut app);
    for key in [KeyCode::KeyZ, KeyCode::KeyS, KeyCode::KeyX] {
        hold(&mut app, &[key], 1);
        assert_eq!(state(&mut app), PlayerState::Attacking, "{key:?}");
        assert!(velocity(&mut app).y <= 0.0, "{key:?} made the player jump");
        assert_eq!(invulnerability(&mut app), 0.0, "{key:?} started a dodge");
        assert_eq!(strikes(&mut app).len(), 1, "{key:?} started a new strike");
    }
    // Guard held through the rest of the attack: refused while attacking,
    // taken as soon as the attack ends.
    press(&mut app, KeyCode::KeyA);
    run(&mut app, 1);
    assert_eq!(state(&mut app), PlayerState::Attacking);
    let mut frames = 0;
    while state(&mut app) == PlayerState::Attacking {
        run(&mut app, 1);
        frames += 1;
        assert!(frames < 30, "attack never ended");
    }
    assert_eq!(state(&mut app), PlayerState::Guarding);
    release(&mut app, KeyCode::KeyA);
    run(&mut app, 1);
    assert_eq!(state(&mut app), PlayerState::Idle);
}

#[test]
fn ground_attack_brakes_and_ignores_the_direction_keys() {
    let mut app = app();
    press(&mut app, KeyCode::ArrowRight);
    run(&mut app, 10);
    let walking = velocity(&mut app).x;
    assert!(walking > 0.0);
    press(&mut app, KeyCode::KeyX);
    run(&mut app, 1);
    release(&mut app, KeyCode::KeyX);
    assert_eq!(state(&mut app), PlayerState::Attacking);
    run(&mut app, 4);
    let braking = velocity(&mut app).x;
    assert!(
        braking < walking && braking >= 0.0,
        "friction, not control: {walking} -> {braking}"
    );
    release(&mut app, KeyCode::ArrowRight);
    run(&mut app, 1);
}

#[test]
fn aerial_attack_keeps_its_momentum_without_air_control() {
    let mut app = app();
    press(&mut app, KeyCode::ArrowRight);
    press(&mut app, KeyCode::KeyZ);
    run(&mut app, 5);
    assert!(!on_ground(&mut app), "the player should be airborne");
    press(&mut app, KeyCode::KeyX);
    run(&mut app, 1);
    release(&mut app, KeyCode::KeyX);
    assert_eq!(state(&mut app), PlayerState::Attacking);
    assert_eq!(
        strikes(&mut app).len(),
        1,
        "the same strike as on the ground"
    );
    let launch = velocity(&mut app).x;
    assert!(launch > 0.0);

    // Reverse the direction: the attack keeps the launch speed.
    release(&mut app, KeyCode::ArrowRight);
    press(&mut app, KeyCode::ArrowLeft);
    for _ in 0..4 {
        run(&mut app, 1);
        assert!(!on_ground(&mut app));
        assert_eq!(state(&mut app), PlayerState::Attacking);
        assert_eq!(
            velocity(&mut app).x,
            launch,
            "no air control while attacking"
        );
    }
    release(&mut app, KeyCode::ArrowLeft);
    release(&mut app, KeyCode::KeyZ);
    run(&mut app, 60);
}

#[test]
fn crouching_attack_uses_the_standing_strike_and_hurtbox() {
    let mut app = app();
    attack(&mut app);
    let standing = strikes(&mut app);
    assert_eq!(standing.len(), 1);
    run(&mut app, 30);

    press(&mut app, KeyCode::ArrowDown);
    run(&mut app, 2);
    assert_eq!(state(&mut app), PlayerState::Crouching);
    let (body, hurtbox) = {
        let world = app.world_mut();
        world
            .query_filtered::<(&Body, &Hurtbox), With<Player>>()
            .single(world)
            .map(|(b, h)| (b.size, h.size))
            .unwrap_or_else(|e| panic!("player missing: {e}"))
    };
    assert_eq!(hurtbox, body, "crouching does not shrink the hurtbox");

    hold(&mut app, &[KeyCode::KeyX], 1);
    assert_eq!(state(&mut app), PlayerState::Attacking);
    let crouching = strikes(&mut app);
    assert_eq!(crouching.len(), 1);
    assert_eq!(crouching[0].1.size, standing[0].1.size);
    assert_eq!(crouching[0].2.offset, standing[0].2.offset);
    run(&mut app, 30);
    assert_eq!(state(&mut app), PlayerState::Crouching, "back to crouching");
    release(&mut app, KeyCode::ArrowDown);
    run(&mut app, 1);
}

// ---------------------------------------------------------------- techniques

#[test]
fn technique_cancels_the_basic_attack_and_pays_once() {
    let mut app = app();
    attack(&mut app);
    run(&mut app, 2);
    assert_eq!(state(&mut app), PlayerState::Attacking);
    reset_observed(&mut app);
    freeze_regeneration(&mut app);
    let (_, mp, hearts) = vitals(&mut app);
    // The same request twice in one frame and once more the next frame.
    app.world_mut()
        .resource_mut::<PendingActivations>()
        .0
        .extend([CANCEL_SKILL.to_owned(), CANCEL_SKILL.to_owned()]);
    run(&mut app, 1);
    assert_eq!(state(&mut app), PlayerState::Casting);
    freeze_regeneration(&mut app);
    app.world_mut()
        .resource_mut::<PendingActivations>()
        .0
        .push(CANCEL_SKILL.to_owned());
    run(&mut app, 1);

    let seen = app.world().resource::<Observed>();
    assert_eq!(seen.started, vec![CANCEL_SKILL.to_owned()]);
    assert_eq!(
        seen.resolved,
        vec![
            CommandOutcome::Executed {
                id: CANCEL_SKILL.to_owned()
            },
            CommandOutcome::NotAllowedNow {
                id: CANCEL_SKILL.to_owned()
            },
            CommandOutcome::NotAllowedNow {
                id: CANCEL_SKILL.to_owned()
            },
        ]
    );
    let (_, mp_after, hearts_after) = vitals(&mut app);
    assert_eq!(mp - mp_after, CANCEL_MP, "MP paid once");
    assert_eq!(hearts - hearts_after, CANCEL_HEARTS, "Hearts paid once");
    let mut cooldown = 0.0;
    with_player(&mut app, |_, book, _| {
        cooldown = book.cooldowns.remaining(CANCEL_SKILL);
    });
    assert!(cooldown > 0.0 && cooldown < 1.0, "cooldown started once");
}

#[test]
fn technique_cancel_keeps_the_basic_strike_hitbox_alive() {
    // Known limitation (pinned, not endorsed): the technique replaces the
    // Attacking state, but the basic strike's hitbox lives on until its own
    // lifetime ends, so both can hit in the same window.
    let mut app = app();
    attack(&mut app);
    let basic = strikes(&mut app)[0].0;
    run(&mut app, 1);
    app.world_mut()
        .resource_mut::<PendingActivations>()
        .0
        .push(CANCEL_SKILL.to_owned());
    run(&mut app, 1);
    assert_eq!(state(&mut app), PlayerState::Casting);
    assert!(
        app.world().get_entity(basic).is_ok(),
        "basic strike survives"
    );
    assert_eq!(player_hitboxes(&mut app), 2, "strike + technique");
    run(&mut app, 12);
    assert!(
        app.world().get_entity(basic).is_err(),
        "it expires on its own"
    );
}

#[test]
fn technique_on_cooldown_cannot_cancel_the_basic_attack() {
    let mut app = app();
    with_player(&mut app, |_, book, _| {
        book.cooldowns.start(CANCEL_SKILL, 5.0)
    });
    attack(&mut app);
    reset_observed(&mut app);
    freeze_regeneration(&mut app);
    let before = vitals(&mut app);
    app.world_mut()
        .resource_mut::<PendingActivations>()
        .0
        .push(CANCEL_SKILL.to_owned());
    run(&mut app, 1);
    assert_eq!(state(&mut app), PlayerState::Attacking);
    assert_eq!(vitals(&mut app).1, before.1, "no MP spent");
    assert!(matches!(
        app.world().resource::<Observed>().resolved.as_slice(),
        [CommandOutcome::OnCooldown { .. }]
    ));
}

#[test]
fn spells_cannot_cancel_the_basic_attack() {
    // Spells in the data are not `usable_in: [.., Attacking]`.
    let mut app = app();
    attack(&mut app);
    reset_observed(&mut app);
    freeze_regeneration(&mut app);
    let before = vitals(&mut app);
    app.world_mut()
        .resource_mut::<PendingActivations>()
        .0
        .push("ember_bolt".to_owned());
    run(&mut app, 1);
    assert_eq!(state(&mut app), PlayerState::Attacking);
    assert_eq!(vitals(&mut app).1, before.1, "no MP spent");
    assert_eq!(
        app.world().resource::<Observed>().resolved,
        vec![CommandOutcome::NotAllowedNow {
            id: "ember_bolt".to_owned()
        }]
    );
}

#[test]
fn real_crescent_slash_input_cancels_the_basic_attack() {
    // ↓ ↘ → + ATTACK pressed while the basic attack is running.
    let mut app = app();
    attack(&mut app);
    hold(&mut app, &[KeyCode::ArrowDown], 2);
    hold(&mut app, &[KeyCode::ArrowDown, KeyCode::ArrowRight], 2);
    press(&mut app, KeyCode::ArrowRight);
    run(&mut app, 1);
    assert_eq!(state(&mut app), PlayerState::Attacking);
    hold(&mut app, &[KeyCode::KeyX], 1);
    release(&mut app, KeyCode::ArrowRight);
    assert_eq!(state(&mut app), PlayerState::Casting);
    assert_eq!(
        app.world().resource::<Observed>().started,
        vec!["crescent_slash".to_owned()]
    );
    run(&mut app, 1);
}

// ---------------------------------------------------------------- damage

#[test]
fn damage_interrupts_the_basic_attack_and_removes_its_strike() {
    let mut app = app();
    attack(&mut app);
    let basic = strikes(&mut app)[0].0;
    freeze_regeneration(&mut app);
    let (hp, _, _) = vitals(&mut app);
    app.world_mut()
        .resource_mut::<PendingDamage>()
        .0
        .push(DamageOutcome::Hit {
            amount: 7,
            critical: false,
            element: Element::Physical,
        });
    run(&mut app, 1);
    assert_eq!(state(&mut app), PlayerState::Hurt);
    assert_eq!(vitals(&mut app).0, hp - 7);
    assert!(app.world().get_entity(basic).is_err(), "strike removed");
    // One knockback for every damage (V2.4 step 7 must keep it by default).
    let v = velocity(&mut app);
    assert_eq!(
        v.x, PLAYER_HURT_KNOCKBACK.0,
        "knockback x (hit from the left)"
    );
    assert!(v.y > 0.0 && v.y <= PLAYER_HURT_KNOCKBACK.1);
    let inv = invulnerability(&mut app);
    assert!(
        inv > PLAYER_INVULNERABILITY - 0.02 && inv <= PLAYER_INVULNERABILITY,
        "invulnerability {inv}"
    );
}

// ================================================================ V2.4
//
// Steps 1-3: the optional timeline fields of `WeaponClass`. The real item
// files are loaded and only the `Sword` class is patched in memory (the RON
// files are untouched), so the starting Iron Sword uses the patched class.

/// The real item catalog with the `Sword` class patched.
fn items_with_sword(patch: impl Fn(&mut WeaponClass)) -> ItemCatalog {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/data/items");
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "ron"))
        .collect();
    files.sort();
    let data = files.iter().map(|file| {
        let text =
            std::fs::read_to_string(file).unwrap_or_else(|e| panic!("{}: {e}", file.display()));
        let mut parsed: ItemDataFile =
            ron::from_str(&text).unwrap_or_else(|e| panic!("{}: {e}", file.display()));
        for class in &mut parsed.weapon_classes {
            if class.category == ItemCategory::Sword {
                patch(class);
            }
        }
        parsed
    });
    ItemCatalog::from_data(data.collect::<Vec<_>>())
        .unwrap_or_else(|e| panic!("patched catalog rejected: {e}"))
}

fn sword_app(patch: impl Fn(&mut WeaponClass)) -> App {
    let items = items_with_sword(patch);
    app_with(move |data| data.items = items)
}

/// An enemy-team target covering the whole strike area.
fn spawn_dummy_target(app: &mut App) -> Entity {
    let world = app.world_mut();
    let (center, profile) = world
        .query_filtered::<(&Transform, &CombatProfile), With<Player>>()
        .single(world)
        .map(|(t, p)| (t.translation, p.clone()))
        .unwrap_or_else(|e| panic!("player missing: {e}"));
    world
        .spawn((
            Hurtbox {
                team: Team::Enemy,
                size: Vec2::splat(400.0),
            },
            profile,
            Transform::from_translation(center),
        ))
        .id()
}

#[derive(Resource, Default)]
struct DummyHits(Vec<Entity>);

fn record_hits(mut hits: ResMut<DummyHits>, mut damage: MessageReader<DamageDealt>) {
    hits.0.extend(damage.read().map(|d| d.target));
}

fn hits_on(app: &App, target: Entity) -> usize {
    app.world()
        .resource::<DummyHits>()
        .0
        .iter()
        .filter(|t| **t == target)
        .count()
}

fn pending(app: &mut App, hitbox: Entity) -> Option<(PendingStrike, Visibility)> {
    let world = app.world_mut();
    world
        .query::<(&PendingStrike, &Visibility)>()
        .get(world, hitbox)
        .ok()
        .map(|(p, v)| (*p, *v))
}

const STARTUP: f32 = 0.07;
const CANCEL_SHARE: f32 = 0.75;

#[test]
fn default_strike_records_a_timeline_without_startup_or_cancel() {
    let mut app = app();
    attack(&mut app);
    let world = app.world_mut();
    let timeline = world
        .query_filtered::<&ActiveStrike, With<Player>>()
        .single(world)
        .map(|s| s.timeline)
        .unwrap_or_else(|e| panic!("no ActiveStrike after an attack: {e}"));
    assert_eq!(timeline.startup, 0.0);
    assert_eq!(timeline.cancel_at, None);
    assert_eq!(timeline.total, BASIC_ATTACK_TIME);
    let hitbox = strikes(&mut app)[0].0;
    assert!(
        pending(&mut app, hitbox).is_none(),
        "no startup, no pending"
    );
}

#[test]
fn startup_delays_the_first_hit_without_shortening_the_active_window() {
    let mut app = sword_app(|c| c.startup = STARTUP);
    app.init_resource::<DummyHits>()
        .add_systems(Update, record_hits.after(GameSet::Combat));
    let dummy = spawn_dummy_target(&mut app);
    attack(&mut app);
    let hitbox = strikes(&mut app)[0].0;

    let dt = FRAME.as_secs_f32();
    let pending_frames = (STARTUP / dt).ceil() as usize - 1;
    let active_frames = frames_of_hitbox(BASIC_ATTACK_TIME * HITBOX_ACTIVE_FRACTION);
    let mut frame = 1;
    let mut existed = 0;
    while app.world().get_entity(hitbox).is_ok() {
        existed += 1;
        let armed = pending(&mut app, hitbox).is_some_and(|(p, _)| p.armed());
        if frame <= pending_frames {
            assert!(!armed, "frame {frame}: armed too early");
            assert_eq!(
                pending(&mut app, hitbox).map(|(_, v)| v),
                Some(Visibility::Hidden)
            );
            assert_eq!(hits_on(&app, dummy), 0, "frame {frame}: hit during startup");
        } else {
            assert!(armed, "frame {frame}: still pending");
            assert_eq!(
                pending(&mut app, hitbox).map(|(_, v)| v),
                Some(Visibility::Inherited)
            );
            assert_eq!(hits_on(&app, dummy), 1, "frame {frame}: one hit per target");
        }
        run(&mut app, 1);
        frame += 1;
        assert!(frame < 40, "the strike never expired");
    }
    assert_eq!(existed, pending_frames + active_frames);
    assert_eq!(hits_on(&app, dummy), 1);
}

#[test]
fn damage_during_startup_removes_the_pending_strike() {
    let mut app = sword_app(|c| c.startup = STARTUP);
    app.init_resource::<DummyHits>()
        .add_systems(Update, record_hits.after(GameSet::Combat));
    let dummy = spawn_dummy_target(&mut app);
    attack(&mut app);
    let hitbox = strikes(&mut app)[0].0;
    app.world_mut()
        .resource_mut::<PendingDamage>()
        .0
        .push(DamageOutcome::Hit {
            amount: 3,
            critical: false,
            element: Element::Physical,
        });
    run(&mut app, 1);
    assert_eq!(state(&mut app), PlayerState::Hurt);
    assert!(
        app.world().get_entity(hitbox).is_err(),
        "pending strike removed"
    );
    run(&mut app, 10);
    assert_eq!(hits_on(&app, dummy), 0);
}

/// Presses `key` once per frame after an attack until `accepted` holds.
/// Returns the frame (the attack frame is 1) the press was accepted.
fn frame_accepted(app: &mut App, key: KeyCode, accepted: impl Fn(&mut App) -> bool) -> usize {
    attack(app);
    for frame in 2..40 {
        hold(app, &[key], 1);
        if accepted(app) {
            return frame;
        }
    }
    panic!("{key:?} was never accepted");
}

/// First frame whose systems see the cancel point (timer ticked frame - 1 times).
fn cancel_frame(share: f32) -> usize {
    let dt = FRAME.as_secs_f32();
    1 + (share * BASIC_ATTACK_TIME / dt).ceil() as usize
}

#[test]
fn dodge_cancels_the_strike_from_the_cancel_point_on() {
    let mut app = sword_app(|c| c.cancel_after = Some(CANCEL_SHARE));
    let frame = frame_accepted(&mut app, KeyCode::KeyS, |app| {
        state(app) == PlayerState::Dodging
    });
    assert_eq!(frame, cancel_frame(CANCEL_SHARE));
    assert!(
        frame < frames_of_action(BASIC_ATTACK_TIME) + 1,
        "before the end"
    );
    assert!(invulnerability(&mut app) > 0.0, "a real dodge");
    assert!(strikes(&mut app).is_empty(), "no old strike left");
}

#[test]
fn jump_cancels_the_strike_from_the_cancel_point_on() {
    let mut app = sword_app(|c| c.cancel_after = Some(CANCEL_SHARE));
    let frame = frame_accepted(&mut app, KeyCode::KeyZ, |app| {
        state(app) == PlayerState::Jumping
    });
    assert_eq!(frame, cancel_frame(CANCEL_SHARE));
    assert!(velocity(&mut app).y > 0.0);
    assert!(strikes(&mut app).is_empty(), "no old strike left");
}

#[test]
fn held_guard_takes_over_at_the_cancel_point() {
    let mut app = sword_app(|c| c.cancel_after = Some(CANCEL_SHARE));
    attack(&mut app);
    press(&mut app, KeyCode::KeyA);
    let mut frame = 1;
    while state(&mut app) == PlayerState::Attacking {
        run(&mut app, 1);
        frame += 1;
        assert!(frame < 40);
    }
    assert_eq!(state(&mut app), PlayerState::Guarding);
    assert_eq!(frame, cancel_frame(CANCEL_SHARE));
    release(&mut app, KeyCode::KeyA);
    run(&mut app, 1);
}

#[test]
fn new_attack_after_the_cancel_point_leaves_a_single_strike() {
    let mut app = sword_app(|c| c.cancel_after = Some(CANCEL_SHARE));
    attack(&mut app);
    let first = strikes(&mut app)[0].0;
    let mut seen = vec![first];
    for frame in 2..60 {
        hold(&mut app, &[KeyCode::KeyX], 1);
        let alive = strikes(&mut app);
        assert!(
            alive.len() <= 1,
            "frame {frame}: {} strikes alive",
            alive.len()
        );
        for (entity, _, _) in alive {
            if !seen.contains(&entity) {
                seen.push(entity);
            }
        }
    }
    // A strike every cancel_frame - 1 frames instead of one per full swing.
    let cycle = cancel_frame(CANCEL_SHARE) - 1;
    assert_eq!(seen.len(), 1 + 58 / cycle);
}

#[test]
fn mashing_at_the_earliest_valid_cancel_point_never_overlaps_strikes() {
    // `cancel_after` exactly at the end of the active window.
    let mut app = sword_app(|c| c.cancel_after = Some(HITBOX_ACTIVE_FRACTION));
    attack(&mut app);
    let mut started = 1;
    let mut last = strikes(&mut app)[0].0;
    for frame in 2..80 {
        hold(&mut app, &[KeyCode::KeyX], 1);
        let alive = strikes(&mut app);
        assert!(
            alive.len() <= 1,
            "frame {frame}: {} strikes alive",
            alive.len()
        );
        if let Some((entity, _, _)) = alive.first()
            && *entity != last
        {
            last = *entity;
            started += 1;
        }
    }
    assert!(started > 5, "the attack was not cancelled: {started}");
}

#[test]
fn techniques_still_cancel_at_any_moment_with_a_cancel_point() {
    let mut app = sword_app(|c| c.cancel_after = Some(CANCEL_SHARE));
    attack(&mut app);
    reset_observed(&mut app);
    freeze_regeneration(&mut app);
    let (_, mp, hearts) = vitals(&mut app);
    app.world_mut()
        .resource_mut::<PendingActivations>()
        .0
        .push(CANCEL_SKILL.to_owned());
    run(&mut app, 1);
    assert_eq!(
        state(&mut app),
        PlayerState::Casting,
        "before the cancel point"
    );
    let (_, mp_after, hearts_after) = vitals(&mut app);
    assert_eq!(
        (mp - mp_after, hearts - hearts_after),
        (CANCEL_MP, CANCEL_HEARTS)
    );
    assert_eq!(
        app.world().resource::<Observed>().started,
        vec![CANCEL_SKILL.to_owned()]
    );
}

#[test]
fn spells_and_walking_still_cannot_cancel_after_the_cancel_point() {
    let mut app = sword_app(|c| c.cancel_after = Some(CANCEL_SHARE));
    attack(&mut app);
    run(&mut app, cancel_frame(CANCEL_SHARE));
    assert_eq!(state(&mut app), PlayerState::Attacking);
    reset_observed(&mut app);
    app.world_mut()
        .resource_mut::<PendingActivations>()
        .0
        .push("ember_bolt".to_owned());
    press(&mut app, KeyCode::ArrowLeft);
    run(&mut app, 1);
    assert_eq!(state(&mut app), PlayerState::Attacking);
    assert_eq!(
        app.world().resource::<Observed>().resolved,
        vec![CommandOutcome::NotAllowedNow {
            id: "ember_bolt".to_owned()
        }]
    );
    release(&mut app, KeyCode::ArrowLeft);
    run(&mut app, 1);
}

#[test]
fn damage_after_a_cancelled_attack_still_removes_the_new_strike() {
    let mut app = sword_app(|c| c.cancel_after = Some(CANCEL_SHARE));
    attack(&mut app);
    run(&mut app, cancel_frame(CANCEL_SHARE) - 1);
    hold(&mut app, &[KeyCode::KeyX], 1);
    let alive = strikes(&mut app);
    assert_eq!(alive.len(), 1, "the new strike");
    app.world_mut()
        .resource_mut::<PendingDamage>()
        .0
        .push(DamageOutcome::Hit {
            amount: 2,
            critical: false,
            element: Element::Physical,
        });
    run(&mut app, 1);
    assert_eq!(state(&mut app), PlayerState::Hurt);
    assert!(strikes(&mut app).is_empty());
}

// ================================================================ V2.4 B1
//
// Cancel policy for the basic strike's hitbox, step 4 (air control and
// turning) and step 5 (re-hits). Defaults are pinned by the tests above.

/// Test-only skills without any hitbox of their own, usable while attacking,
/// so every hit on a dummy comes from the basic strike.
const HEAL_TECHNIQUE: &str = "test_heal_technique";
const HEAL_SPELL: &str = "test_heal_spell";

fn heal_skill(id: &str, category: SkillCategory) -> SkillDefinition {
    SkillDefinition {
        id: id.to_owned(),
        name: id.to_owned(),
        category,
        mp_cost: 0,
        hearts_cost: 0,
        effect: SkillEffect::Heal { amount: 1 },
        usable_in: vec![ActionPhase::Grounded, ActionPhase::Attacking],
        ..cancel_skill()
    }
}

fn policy_app(policy: StrikeCancelPolicy) -> App {
    let skills = [
        heal_skill(HEAL_TECHNIQUE, SkillCategory::SwordTechnique),
        heal_skill(HEAL_SPELL, SkillCategory::ElementalSpell),
    ];
    let to_insert = skills.clone();
    let mut app = app_with(move |data| {
        for skill in to_insert {
            data.skills
                .insert(skill)
                .unwrap_or_else(|e| panic!("synthetic skill rejected: {e}"));
        }
    });
    with_player(&mut app, |_, book, _| {
        for skill in &skills {
            book.learned.grant(skill);
        }
    });
    assert_eq!(
        *app.world().resource::<StrikeCancelPolicy>(),
        StrikeCancelPolicy::Preserve,
        "the game default is Preserve"
    );
    app.insert_resource(policy)
        .init_resource::<DummyHits>()
        .add_systems(Update, record_hits.after(GameSet::Combat));
    app
}

fn request_skill(app: &mut App, id: &str) {
    app.world_mut()
        .resource_mut::<PendingActivations>()
        .0
        .push(id.to_owned());
}

/// Attack, hit `early` in the first frame, then cancel with `skill` in the
/// frame `late` appears. Returns (early, late, the strike).
fn cancel_with(app: &mut App, skill: &str) -> (Entity, Entity, Entity) {
    let early = spawn_dummy_target(app);
    attack(app);
    let strike = strikes(app)[0].0;
    assert_eq!(hits_on(app, early), 1, "hit before the cancel");
    run(app, 1);
    let late = spawn_dummy_target(app);
    request_skill(app, skill);
    run(app, 1);
    assert_eq!(
        state(app),
        PlayerState::Casting,
        "{skill} cancelled the attack"
    );
    (early, late, strike)
}

#[test]
fn preserve_policy_keeps_the_strike_hitting_after_a_technique_cancel() {
    let mut app = policy_app(StrikeCancelPolicy::Preserve);
    let (early, late, strike) = cancel_with(&mut app, HEAL_TECHNIQUE);
    assert!(app.world().get_entity(strike).is_ok(), "strike kept");
    assert_eq!(hits_on(&app, early), 1);
    assert_eq!(hits_on(&app, late), 1, "the kept strike still hits");
}

#[test]
fn remove_policy_drops_the_strike_when_a_technique_cancels() {
    let mut app = policy_app(StrikeCancelPolicy::RemoveOnCancel);
    let (early, late, strike) = cancel_with(&mut app, HEAL_TECHNIQUE);
    assert!(app.world().get_entity(strike).is_err(), "strike removed");
    assert_eq!(hits_on(&app, early), 1, "damage already dealt stays");
    assert_eq!(hits_on(&app, late), 0, "removed before GameSet::Combat");
    run(&mut app, 20);
    assert_eq!(hits_on(&app, late), 0);
}

#[test]
fn remove_policy_also_applies_to_spells_allowed_while_attacking() {
    let mut app = policy_app(StrikeCancelPolicy::RemoveOnCancel);
    let (_, late, strike) = cancel_with(&mut app, HEAL_SPELL);
    assert!(app.world().get_entity(strike).is_err());
    assert_eq!(hits_on(&app, late), 0);
}

#[test]
fn remove_policy_never_touches_other_hitboxes() {
    let mut app = policy_app(StrikeCancelPolicy::RemoveOnCancel);
    // A hitbox of someone else, far from the player.
    let foe = app.world_mut().spawn(Transform::default()).id();
    let foe_hitbox = {
        let world = app.world_mut();
        let attack = world
            .query_filtered::<&CombatProfile, With<Player>>()
            .single(world)
            .map(|p| p.physical.clone())
            .unwrap_or_else(|e| panic!("player missing: {e}"));
        world
            .spawn((
                Hitbox {
                    owner: foe,
                    team: Team::Enemy,
                    size: Vec2::splat(4.0),
                    attack,
                    hit: HitSpec {
                        kind: DamageKind::Physical,
                        element: Element::Physical,
                        base_damage: 1,
                        multiplier: 1.0,
                    },
                    lifetime: 5.0,
                    pierce: true,
                    already_hit: Vec::new(),
                },
                Transform::from_xyz(-5000.0, -5000.0, 0.0),
            ))
            .id()
    };
    attack(&mut app);
    run(&mut app, 1);
    // The technique's own melee hitbox must survive its own start.
    request_skill(&mut app, CANCEL_SKILL);
    run(&mut app, 1);
    assert_eq!(state(&mut app), PlayerState::Casting);
    assert!(strikes(&mut app).is_empty(), "basic strike removed");
    assert_eq!(player_hitboxes(&mut app), 1, "technique hitbox kept");
    assert!(
        app.world().get_entity(foe_hitbox).is_ok(),
        "other owner kept"
    );
}

#[test]
fn remove_policy_changes_nothing_without_a_cancel() {
    let mut app = policy_app(StrikeCancelPolicy::RemoveOnCancel);
    // A skill started from Idle: no strike to remove, the skill runs.
    request_skill(&mut app, CANCEL_SKILL);
    run(&mut app, 1);
    assert_eq!(state(&mut app), PlayerState::Casting);
    assert_eq!(player_hitboxes(&mut app), 1);
    run(&mut app, 40);
    // A plain attack keeps its strike for the usual 10 frames.
    attack(&mut app);
    let strike = strikes(&mut app)[0].0;
    run(&mut app, 9);
    assert!(app.world().get_entity(strike).is_ok());
}

// ---------------------------------------------------------------- step 4

fn facing(app: &mut App) -> Facing {
    let world = app.world_mut();
    world
        .query_filtered::<&FacingDirection, With<Player>>()
        .single(world)
        .map(|f| f.0)
        .unwrap_or_else(|e| panic!("player missing: {e}"))
}

fn player_x(app: &mut App) -> f32 {
    let world = app.world_mut();
    world
        .query_filtered::<&Transform, With<Player>>()
        .single(world)
        .map(|t| t.translation.x)
        .unwrap_or_else(|e| panic!("player missing: {e}"))
}

/// Horizontal side (+1 right, -1 left) of the strike relative to the player.
fn strike_side(app: &mut App, strike: Entity) -> f32 {
    let x = player_x(app);
    let world = app.world_mut();
    let hitbox_x = world
        .query::<&Transform>()
        .get(world, strike)
        .map(|t| t.translation.x)
        .unwrap_or_else(|e| panic!("strike missing: {e}"));
    (hitbox_x - x).signum()
}

/// Jumps right and waits for the fall; returns the walking speed.
fn jump_right_until_falling(app: &mut App) -> f32 {
    press(app, KeyCode::ArrowRight);
    run(app, 5);
    let walking = velocity(app).x;
    press(app, KeyCode::KeyZ);
    run(app, 1);
    release(app, KeyCode::KeyZ);
    let mut frames = 0;
    while state(app) != PlayerState::Falling {
        run(app, 1);
        frames += 1;
        assert!(frames < 60, "never started falling");
    }
    assert!(!on_ground(app));
    assert_eq!(state(app), PlayerState::Falling);
    walking
}

#[test]
fn falling_attack_without_air_control_keeps_its_speed() {
    let mut app = app();
    let walking = jump_right_until_falling(&mut app);
    hold(&mut app, &[KeyCode::KeyX], 1);
    assert_eq!(state(&mut app), PlayerState::Attacking);
    release(&mut app, KeyCode::ArrowRight);
    press(&mut app, KeyCode::ArrowLeft);
    run(&mut app, 4);
    assert!(!on_ground(&mut app));
    assert_eq!(velocity(&mut app).x, walking, "momentum kept");
    assert_eq!(facing(&mut app), Facing::Right, "no turning by default");
    release(&mut app, KeyCode::ArrowLeft);
    run(&mut app, 60);
}

#[test]
fn full_air_control_steers_a_falling_attack_like_a_jump() {
    let mut app = sword_app(|c| c.air_control = 1.0);
    let walking = jump_right_until_falling(&mut app);
    hold(&mut app, &[KeyCode::KeyX], 1);
    assert_eq!(state(&mut app), PlayerState::Attacking);
    release(&mut app, KeyCode::ArrowRight);
    press(&mut app, KeyCode::ArrowLeft);
    run(&mut app, 1);
    assert_eq!(velocity(&mut app).x, -walking, "steered at walking speed");
    assert_eq!(state(&mut app), PlayerState::Attacking);
    assert_eq!(facing(&mut app), Facing::Right, "steering does not turn");
    release(&mut app, KeyCode::ArrowLeft);
    run(&mut app, 1);
    assert_eq!(velocity(&mut app).x, 0.0, "no key, no drift");
    run(&mut app, 60);
}

#[test]
fn air_control_does_not_change_the_ground_brake() {
    let mut walking_default = app();
    let mut walking_control = sword_app(|c| c.air_control = 1.0);
    let mut speeds = Vec::new();
    for app in [&mut walking_default, &mut walking_control] {
        press(app, KeyCode::ArrowRight);
        run(app, 10);
        hold(app, &[KeyCode::KeyX], 1);
        press(app, KeyCode::ArrowLeft);
        run(app, 4);
        speeds.push(velocity(app).x);
    }
    assert_eq!(speeds[0], speeds[1]);
}

#[test]
fn attacks_against_a_wall_never_cross_it() {
    let mut app = sword_app(|c| {
        c.air_control = 1.0;
        c.turn_window = 0.1;
    });
    press(&mut app, KeyCode::ArrowLeft);
    run(&mut app, 120);
    let wall_x = player_x(&mut app);
    run(&mut app, 5);
    assert_eq!(player_x(&mut app), wall_x, "stopped by the wall");
    // Ground attack into the wall.
    hold(&mut app, &[KeyCode::KeyX], 1);
    for _ in 0..20 {
        run(&mut app, 1);
        assert!(player_x(&mut app) >= wall_x - 1e-3);
    }
    // Jumping attack steered into the wall.
    press(&mut app, KeyCode::KeyZ);
    run(&mut app, 3);
    release(&mut app, KeyCode::KeyZ);
    hold(&mut app, &[KeyCode::KeyX], 1);
    assert_eq!(state(&mut app), PlayerState::Attacking);
    for _ in 0..40 {
        run(&mut app, 1);
        assert!(player_x(&mut app) >= wall_x - 1e-3, "inside the wall");
    }
    release(&mut app, KeyCode::ArrowLeft);
    run(&mut app, 30);
    assert!(on_ground(&mut app));
}

#[test]
fn turn_window_lets_the_character_turn_only_at_the_start() {
    let window = 0.1;
    // Inside the window.
    let mut app = sword_app(|c| c.turn_window = window);
    attack(&mut app);
    run(&mut app, 1);
    hold(&mut app, &[KeyCode::ArrowLeft], 1);
    assert_eq!(facing(&mut app), Facing::Left, "turned inside the window");
    assert_eq!(state(&mut app), PlayerState::Attacking, "still attacking");
    assert_eq!(velocity(&mut app).x, 0.0, "turning is not walking");

    // After the window.
    let mut app = sword_app(|c| c.turn_window = window);
    attack(&mut app);
    let after = (window / FRAME.as_secs_f32()).ceil() as usize + 1;
    run(&mut app, after);
    assert_eq!(state(&mut app), PlayerState::Attacking);
    hold(&mut app, &[KeyCode::ArrowLeft], 1);
    assert_eq!(facing(&mut app), Facing::Right, "window closed");
}

#[test]
fn turning_never_flips_an_armed_strike() {
    let mut app = sword_app(|c| c.turn_window = 0.1);
    attack(&mut app);
    let strike = strikes(&mut app)[0].0;
    assert_eq!(strike_side(&mut app, strike), 1.0);
    hold(&mut app, &[KeyCode::ArrowLeft], 1);
    assert_eq!(facing(&mut app), Facing::Left);
    for _ in 0..5 {
        assert_eq!(
            strike_side(&mut app, strike),
            1.0,
            "the strike keeps its side"
        );
        run(&mut app, 1);
    }
}

#[test]
fn a_pending_strike_follows_the_turn_and_then_keeps_its_side() {
    let mut app = sword_app(|c| {
        c.startup = 0.1;
        c.turn_window = 0.05;
    });
    attack(&mut app);
    let strike = strikes(&mut app)[0].0;
    hold(&mut app, &[KeyCode::ArrowLeft], 1);
    assert_eq!(facing(&mut app), Facing::Left);
    assert_eq!(strike_side(&mut app, strike), -1.0, "aimed while pending");
    // Let the 0.05 s window close (still pending until 0.1 s).
    run(&mut app, 3);
    assert!(pending(&mut app, strike).is_some_and(|(p, _)| !p.armed()));
    // Window over: right is ignored, the strike arms and stays left.
    press(&mut app, KeyCode::ArrowRight);
    for _ in 0..10 {
        run(&mut app, 1);
        if app.world().get_entity(strike).is_err() {
            break;
        }
        assert_eq!(facing(&mut app), Facing::Left);
        assert_eq!(strike_side(&mut app, strike), -1.0);
    }
    release(&mut app, KeyCode::ArrowRight);
    run(&mut app, 1);
}

#[test]
fn crouching_attacks_turn_like_standing_ones() {
    let mut app = sword_app(|c| c.turn_window = 0.1);
    press(&mut app, KeyCode::ArrowDown);
    run(&mut app, 2);
    assert_eq!(state(&mut app), PlayerState::Crouching);
    hold(&mut app, &[KeyCode::KeyX], 1);
    hold(&mut app, &[KeyCode::ArrowLeft], 1);
    assert_eq!(facing(&mut app), Facing::Left);
    assert_eq!(state(&mut app), PlayerState::Attacking);
    release(&mut app, KeyCode::ArrowDown);
    run(&mut app, 1);
}

#[test]
fn techniques_and_other_states_never_turn() {
    let mut app = sword_app(|c| c.turn_window = 0.28);
    attack(&mut app);
    request_skill(&mut app, CANCEL_SKILL);
    run(&mut app, 1);
    assert_eq!(state(&mut app), PlayerState::Casting);
    hold(&mut app, &[KeyCode::ArrowLeft], 1);
    assert_eq!(facing(&mut app), Facing::Right, "casting does not turn");
    run(&mut app, 40);
    hold(&mut app, &[KeyCode::KeyS], 1);
    assert_eq!(state(&mut app), PlayerState::Dodging);
    hold(&mut app, &[KeyCode::ArrowLeft], 1);
    assert_eq!(facing(&mut app), Facing::Right, "dodging does not turn");
}

#[test]
fn commands_still_read_the_direction_with_a_turn_window() {
    let mut app = sword_app(|c| c.turn_window = 0.1);
    attack(&mut app);
    hold(&mut app, &[KeyCode::ArrowDown], 2);
    hold(&mut app, &[KeyCode::ArrowDown, KeyCode::ArrowRight], 2);
    press(&mut app, KeyCode::ArrowRight);
    run(&mut app, 1);
    hold(&mut app, &[KeyCode::KeyX], 1);
    release(&mut app, KeyCode::ArrowRight);
    assert_eq!(facing(&mut app), Facing::Right);
    assert_eq!(state(&mut app), PlayerState::Casting);
    assert_eq!(
        app.world().resource::<Observed>().started,
        vec!["crescent_slash".to_owned()]
    );
}

// ---------------------------------------------------------------- step 5

/// A long strike: the whole 0.5 s swing is active.
fn long_strike(class: &mut WeaponClass, rehit: Option<f32>) {
    class.attack_time = 0.5;
    class.active_fraction = 1.0;
    class.rehit_interval = rehit;
}

/// Attacks once and runs until the strike is gone; checks that no target
/// is hit twice in one frame. Returns the strike entity.
fn strike_until_gone(app: &mut App, targets: &[Entity]) -> Entity {
    attack(app);
    let strike = strikes(app)[0].0;
    let mut before: Vec<usize> = targets.iter().map(|t| hits_on(app, *t)).collect();
    for _ in 0..60 {
        if app.world().get_entity(strike).is_err() {
            return strike;
        }
        run(app, 1);
        for (i, target) in targets.iter().enumerate() {
            let now = hits_on(app, *target);
            assert!(now - before[i] <= 1, "two hits in one frame");
            before[i] = now;
        }
    }
    panic!("the strike never expired");
}

fn rehit_app(rehit: Option<f32>) -> App {
    let mut app = sword_app(move |c| long_strike(c, rehit));
    app.init_resource::<DummyHits>()
        .add_systems(Update, record_hits.after(GameSet::Combat));
    app
}

#[test]
fn without_rehit_each_target_is_hit_once_per_strike() {
    let mut app = rehit_app(None);
    let targets = [spawn_dummy_target(&mut app), spawn_dummy_target(&mut app)];
    strike_until_gone(&mut app, &targets);
    for target in targets {
        assert_eq!(hits_on(&app, target), 1);
    }
}

#[test]
fn rehit_interval_hits_each_target_again_independently() {
    let mut app = rehit_app(Some(0.2));
    let targets = [spawn_dummy_target(&mut app), spawn_dummy_target(&mut app)];
    strike_until_gone(&mut app, &targets);
    // Hits at 0 s, ~0.2 s and ~0.4 s of a 0.5 s strike.
    for target in targets {
        assert_eq!(hits_on(&app, target), 3);
    }
}

#[test]
fn two_strikes_each_hit_the_same_target() {
    for rehit in [None, Some(1.0)] {
        let mut app = rehit_app(rehit);
        let target = spawn_dummy_target(&mut app);
        let first = strike_until_gone(&mut app, &[target]);
        run(&mut app, 20);
        let second = strike_until_gone(&mut app, &[target]);
        assert_ne!(first, second);
        assert_eq!(hits_on(&app, target), 2, "rehit {rehit:?}: one per strike");
    }
}

#[test]
fn rehit_records_are_dropped_with_the_wait_and_the_hitbox() {
    let mut app = rehit_app(Some(0.2));
    let target = spawn_dummy_target(&mut app);
    attack(&mut app);
    let strike = strikes(&mut app)[0].0;
    let tracked = |app: &mut App| {
        let world = app.world_mut();
        world
            .query::<&RehitTimers>()
            .get(world, strike)
            .map(RehitTimers::tracked)
            .unwrap_or_else(|e| panic!("record missing: {e}"))
    };
    assert_eq!(tracked(&mut app), 1, "one target waiting");
    // The target leaves; its wait ends and the entry goes away.
    app.world_mut().despawn(target);
    run(&mut app, 14);
    assert_eq!(tracked(&mut app), 0, "finished wait dropped");
    run(&mut app, 30);
    assert!(app.world().get_entity(strike).is_err());
    let world = app.world_mut();
    assert_eq!(world.query::<&RehitTimers>().iter(world).count(), 0);
}

#[test]
fn projectiles_and_techniques_keep_one_hit_per_target() {
    let mut app = rehit_app(Some(0.2));
    run(&mut app, 1);
    request_skill(&mut app, CANCEL_SKILL);
    run(&mut app, 1);
    let world = app.world_mut();
    assert_eq!(
        world.query::<&RehitTimers>().iter(world).count(),
        0,
        "only basic strikes of a weapon with rehit_interval get re-hits"
    );
}

// ---------------------------------------------------------------- checkpoint review

/// A basic-strike hitbox with a long lifetime, owned by `owner`.
fn spawn_lingering_strike(app: &mut App, owner: Entity) -> Entity {
    let world = app.world_mut();
    let attack = world
        .query_filtered::<&CombatProfile, With<Player>>()
        .single(world)
        .map(|p| p.physical.clone())
        .unwrap_or_else(|e| panic!("player missing: {e}"));
    world
        .spawn((
            Hitbox {
                owner,
                team: Team::Player,
                size: Vec2::splat(4.0),
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
            WeaponStrike { category: None },
            Transform::from_xyz(-5000.0, -5000.0, 0.0),
        ))
        .id()
}

#[test]
fn a_new_strike_from_idle_removes_a_lingering_strike_of_the_same_attacker() {
    // Models a strike whose last active frame coincides with the end of its
    // attack (possible when timings are exact multiples of the frame).
    let mut app = app();
    let owner = player(&mut app);
    let lingering = spawn_lingering_strike(&mut app, owner);
    let someone_else = app.world_mut().spawn(Transform::default()).id();
    let foreign = spawn_lingering_strike(&mut app, someone_else);
    assert_eq!(state(&mut app), PlayerState::Idle);
    attack(&mut app);
    assert!(
        app.world().get_entity(lingering).is_err(),
        "lingering strike removed"
    );
    assert!(app.world().get_entity(foreign).is_ok(), "other owner kept");
    let mine: Vec<_> = strikes(&mut app)
        .into_iter()
        .filter(|(_, h, _)| h.owner == owner)
        .collect();
    assert_eq!(mine.len(), 1, "exactly the new strike");
}

// ---------------------------------------------------------------- B2: weapon knockback

#[derive(Resource, Default)]
struct KnockbackSeen(Vec<(Entity, KnockbackIntensity)>);

fn record_knockback(mut seen: ResMut<KnockbackSeen>, mut damage: MessageReader<DamageDealt>) {
    seen.0
        .extend(damage.read().map(|d| (d.target, d.knockback)));
}

/// Strikes a dummy once with the sword patched by `patch`; returns the
/// knockback of its hit and whether the strike hitbox carried `HitKnockback`.
fn strike_knockback(patch: impl Fn(&mut WeaponClass)) -> (KnockbackIntensity, bool) {
    let mut app = sword_app(patch);
    app.init_resource::<KnockbackSeen>()
        .add_systems(Update, record_knockback.after(GameSet::Combat));
    let target = spawn_dummy_target(&mut app);
    attack(&mut app);
    let world = app.world_mut();
    let tagged = world
        .query_filtered::<&HitKnockback, With<WeaponStrike>>()
        .iter(world)
        .count()
        > 0;
    let seen = &app.world().resource::<KnockbackSeen>().0;
    let hits: Vec<_> = seen.iter().filter(|(t, _)| *t == target).collect();
    assert_eq!(hits.len(), 1, "one hit on the dummy: {seen:?}");
    (hits[0].1, tagged)
}

#[test]
fn the_iron_sword_hits_with_the_legacy_knockback() {
    assert_eq!(
        strike_knockback(|_| {}),
        (KnockbackIntensity::Medium, false),
        "default class: Medium and no extra component"
    );
}

#[test]
fn a_weapon_class_knockback_reaches_its_hits() {
    for intensity in [KnockbackIntensity::Light, KnockbackIntensity::Heavy] {
        assert_eq!(
            strike_knockback(move |c| c.knockback = intensity),
            (intensity, true)
        );
    }
}
