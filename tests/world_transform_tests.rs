//! V1.9: the Spectral Form in the headless main game (world graph active).
//!
//! Activation, MP, cancel, the profiles of movement and collision, veils
//! (floor and side), the safe end inside a veil, combat restrictions,
//! damage, death, save/load, room changes and repeated use. Every scenario
//! of docs/TRANSFORMATIONS_V1_9.md §18 has a test here.
//!
//! To set a scene quickly, these tests ask the room lifecycle for a room
//! (`RoomRequest`, the path a load takes), place the player (`place`) and
//! grant the relic as if collected. The input-only proof that the form is
//! obtained and used in play is `world_spectral_route_tests.rs`.
//! Saves go to directories under the target dir; `saves/` is never touched.

use bevy::prelude::*;
use dark_castle_rpg::combat::damage::DamageOutcome;
use dark_castle_rpg::combat::hitboxes::Hitbox;
use dark_castle_rpg::core::events::{Arrival, DamageDealt, Notification, RoomRequest};
use dark_castle_rpg::magic::elements::Element;
use dark_castle_rpg::magic::projectiles::Projectile;
use dark_castle_rpg::player::components::{ActionState, Player, Vitals};
use dark_castle_rpg::player::movement::LocomotionProfile;
use dark_castle_rpg::player::state::PlayerState;
use dark_castle_rpg::transform::state::{EndReason, FormState};
use dark_castle_rpg::transform::systems::Transformation;
use dark_castle_rpg::transform::visual::{FormMote, FormOutline};
use dark_castle_rpg::world::collision::Aabb;
use dark_castle_rpg::world::exploration_systems::Exploration;
use dark_castle_rpg::world::graph::{Ability, Capabilities, Grant};
use dark_castle_rpg::world::physics::{GravityProfile, Grounded, Phasing, RoomGeometry, Velocity};
use dark_castle_rpg::world::transitions::TransitionLock;
use dark_castle_rpg::world::validation::{Rules, reachable_from, validate};

#[path = "support/world_app.rs"]
mod world_app;

use world_app::*;

const WELL: &str = "vc_veiled_well";
const SHRINE: &str = "vc_veiled_shrine";
const T: KeyCode = KeyCode::KeyT;
/// Standing on the gallery's floor veil.
const ON_FLOOR_VEIL: (f32, f32) = (1920.0, 63.0);
/// Standing on the well's floor, in front of its west veil (x 0..120).
const BEFORE_WEST_VEIL: (f32, f32) = (200.0, 63.0);
/// Inside the well's west veil (a human would not fit here).
const IN_WEST_VEIL: (f32, f32) = (70.0, 63.0);
/// Start MP of the character (20 + 6 INT x 5 + 2 per level).
const START_MP: i32 = 52;

#[derive(Resource, Default)]
struct Notes(Vec<String>);

fn record(mut messages: MessageReader<Notification>, mut notes: ResMut<Notes>) {
    notes.0.extend(messages.read().map(|n| n.0.clone()));
}

fn spectral_game(name: &str) -> App {
    let mut app = game(&storage(name));
    app.init_resource::<Notes>().add_systems(PostUpdate, record);
    app
}

fn noted(app: &App, text: &str) -> bool {
    app.world()
        .resource::<Notes>()
        .0
        .iter()
        .any(|n| n.contains(text))
}

fn give_relic(app: &mut App) {
    app.world_mut()
        .resource_mut::<Exploration>()
        .granted
        .push(Grant::Ability(Ability::VeilStep));
}

fn enter(app: &mut App, room: &str, door: &str) {
    app.world_mut().write_message(RoomRequest {
        room: room.into(),
        arrival: Arrival::Door(door.into()),
    });
    run(app, 2);
    assert_eq!(current(app), room);
}

/// Places the player standing at `p` and lets the ground contact settle.
fn stand(app: &mut App, p: (f32, f32)) {
    place(app, p);
    run(app, 3);
}

fn form(app: &mut App) -> FormState {
    let world = app.world_mut();
    world
        .query_filtered::<&Transformation, With<Player>>()
        .single(world)
        .map(|t| t.0.clone())
        .unwrap_or_else(|e| panic!("{e}"))
}

/// (locomotion profile, gravity profile, phasing) present on the player.
fn profiles(app: &mut App) -> (bool, bool, bool) {
    (
        count::<(With<Player>, With<LocomotionProfile>)>(app) == 1,
        count::<(With<Player>, With<GravityProfile>)>(app) == 1,
        count::<(With<Player>, With<Phasing>)>(app) == 1,
    )
}

fn mp(app: &mut App) -> i32 {
    let world = app.world_mut();
    world
        .query_filtered::<&Vitals, With<Player>>()
        .single(world)
        .map(|v| v.mp.current())
        .unwrap_or_else(|e| panic!("{e}"))
}

fn set_mp(app: &mut App, value: i32) {
    let world = app.world_mut();
    let mut q = world.query_filtered::<&mut Vitals, With<Player>>();
    let mut v = q.single_mut(world).unwrap_or_else(|e| panic!("{e}"));
    let current = v.mp.current();
    v.mp.deplete(current);
    v.mp.restore(value);
}

fn state(app: &mut App) -> PlayerState {
    let world = app.world_mut();
    world
        .query_filtered::<&ActionState, With<Player>>()
        .single(world)
        .map(|s| s.state())
        .unwrap_or_else(|e| panic!("{e}"))
}

fn velocity(app: &mut App) -> Vec2 {
    let world = app.world_mut();
    world
        .query_filtered::<&Velocity, With<Player>>()
        .single(world)
        .map(|v| v.0)
        .unwrap_or_else(|e| panic!("{e}"))
}

fn grounded(app: &mut App) -> bool {
    let world = app.world_mut();
    world
        .query_filtered::<&Grounded, With<Player>>()
        .single(world)
        .map(|g| g.on_ground)
        .unwrap_or_else(|e| panic!("{e}"))
}

/// Whether a human body would overlap a solid (veils included) here.
fn human_blocked(app: &mut App) -> bool {
    let p = player_xy(app);
    let body = Aabb::new((p.x, p.y), (26.0, 46.0));
    app.world().resource::<RoomGeometry>().blocks(&body)
}

fn hold(app: &mut App, key: KeyCode, frames: usize) {
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(key);
    run(app, frames);
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .release(key);
}

/// Holds `key` until `done` (at most `limit` frames). Returns the frames.
fn hold_until(app: &mut App, key: KeyCode, limit: usize, done: impl Fn(&mut App) -> bool) -> usize {
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(key);
    let mut frames = 0;
    while frames < limit && !done(app) {
        run(app, 1);
        frames += 1;
    }
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .release(key);
    frames
}

fn hit_player(app: &mut App, amount: i32) {
    let world = app.world_mut();
    let player = world
        .query_filtered::<Entity, With<Player>>()
        .single(world)
        .unwrap_or_else(|e| panic!("{e}"));
    world.write_message(DamageDealt {
        attacker: player,
        target: player,
        outcome: DamageOutcome::Hit {
            amount,
            critical: false,
            element: Element::Physical,
        },
        direction: 1.0,
    });
}

/// In the well, spectral, in front of its west veil.
fn spectral_in_the_well(name: &str) -> App {
    let mut app = spectral_game(name);
    give_relic(&mut app);
    enter(&mut app, WELL, "to_shrine");
    stand(&mut app, BEFORE_WEST_VEIL);
    tap(&mut app, T);
    assert!(form(&mut app).is_active());
    app
}

// --- Activation, MP, cancel ---------------------------------------------

#[test]
fn without_the_relic_the_form_does_not_start() {
    let mut app = spectral_game("v19_no_relic");
    tap(&mut app, T);
    run(&mut app, 2);
    assert!(!form(&mut app).is_active());
    assert_eq!(mp(&mut app), START_MP, "nothing paid");
    assert_eq!(profiles(&mut app), (false, false, false));
    assert!(noted(&app, "needs the Relic of the Veil"));
}

#[test]
fn with_the_relic_t_starts_the_form_and_pays_once() {
    let mut app = spectral_game("v19_activate");
    give_relic(&mut app);
    tap(&mut app, T);
    let f = form(&mut app);
    assert!(f.is_active());
    assert_eq!(f.spent(), 8);
    assert!(mp(&mut app) <= START_MP - 8 + 1, "activation paid");
    assert_eq!(profiles(&mut app), (true, true, true));
    // Holding T does not toggle every frame, and nothing is paid twice.
    hold(&mut app, T, 30);
    let f = form(&mut app);
    assert!(f.is_active(), "a held key acts once");
    assert!(f.ending().is_none());
}

#[test]
fn not_enough_mp_refuses_the_form() {
    let mut app = spectral_game("v19_low_mp");
    give_relic(&mut app);
    set_mp(&mut app, 7);
    tap(&mut app, T);
    assert!(!form(&mut app).is_active());
    assert!(noted(&app, "not enough MP (7/8)"));
    set_mp(&mut app, 8);
    tap(&mut app, T);
    assert!(form(&mut app).is_active());
}

#[test]
fn the_upkeep_takes_four_mp_per_second() {
    let mut app = spectral_game("v19_upkeep");
    give_relic(&mut app);
    tap(&mut app, T);
    run(&mut app, 125); // 2 s at 16 ms
    let f = form(&mut app);
    assert!(
        (15..=16).contains(&f.spent()),
        "8 + 4 MP/s x 2 s, spent {}",
        f.spent()
    );
    // The global regeneration (0.8 MP/s) is unchanged and still runs.
    let left = mp(&mut app);
    assert!(
        left >= START_MP - f.spent() && left <= START_MP - f.spent() + 2,
        "{left}"
    );
}

#[test]
fn t_again_returns_to_the_human_form() {
    let mut app = spectral_game("v19_cancel");
    give_relic(&mut app);
    tap(&mut app, T);
    run(&mut app, 10);
    tap(&mut app, T);
    assert!(!form(&mut app).is_active());
    assert_eq!(profiles(&mut app), (false, false, false));
}

#[test]
fn running_out_of_mp_ends_the_form() {
    let mut app = spectral_game("v19_mp_empty");
    give_relic(&mut app);
    set_mp(&mut app, 9);
    tap(&mut app, T);
    assert!(form(&mut app).is_active());
    run(&mut app, 120);
    assert!(!form(&mut app).is_active(), "upkeep could not be paid");
    assert_eq!(profiles(&mut app), (false, false, false));
    assert!(mp(&mut app) <= 1);
}

#[test]
fn the_form_does_not_start_in_actions_in_the_air_or_while_dead() {
    let mut app = spectral_game("v19_busy");
    give_relic(&mut app);
    // Attacking.
    tap(&mut app, KeyCode::KeyX);
    assert_eq!(state(&mut app), PlayerState::Attacking);
    tap(&mut app, T);
    assert!(!form(&mut app).is_active());
    assert!(noted(&app, "cannot transform now"));
    run(&mut app, 30);
    // In the air.
    tap(&mut app, KeyCode::KeyZ);
    run(&mut app, 4);
    assert!(!grounded(&mut app));
    tap(&mut app, T);
    assert!(!form(&mut app).is_active());
    assert!(noted(&app, "stand on the ground"));
    run(&mut app, 80);
    // Dead.
    hit_player(&mut app, 10_000);
    run(&mut app, 1);
    assert_eq!(state(&mut app), PlayerState::Dead);
    tap(&mut app, T);
    assert!(!form(&mut app).is_active());
    assert!(noted(&app, "not while fallen"));
}

#[test]
fn the_form_does_not_start_during_a_room_transition() {
    let mut app = spectral_game("v19_transition");
    give_relic(&mut app);
    app.world_mut().resource_mut::<TransitionLock>().locked = true;
    // The lock re-arms once the player is seen clear of the triggers, in
    // the Rooms set: press T in a frame where it is still armed.
    app.world_mut().resource_mut::<PendingPress>().0.push(T);
    app.update();
    assert!(!form(&mut app).is_active());
    assert!(noted(&app, "wait for the passage to settle"));
}

// --- Profiles: human preserved, spectral applied ------------------------

/// Jump apex and walking speed of the human form, measured in place.
fn human_motion(app: &mut App) -> (f32, f32) {
    let start = player_xy(app);
    hold(app, KeyCode::ArrowRight, 30);
    let speed = velocity(app).x;
    run(app, 20);
    let y0 = player_xy(app).y;
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyZ);
    let mut apex = y0;
    for _ in 0..60 {
        run(app, 1);
        apex = apex.max(player_xy(app).y);
    }
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .release(KeyCode::KeyZ);
    run(app, 30);
    place(app, (start.x, start.y));
    run(app, 5);
    (speed, apex - y0)
}

#[test]
fn the_human_form_is_identical_before_and_after_a_transformation() {
    let mut app = spectral_game("v19_human_same");
    give_relic(&mut app);
    // The start room (no enemy); left of its block (420..540).
    stand(&mut app, (300.0, 63.0));
    let before = human_motion(&mut app);
    tap(&mut app, T);
    run(&mut app, 40);
    tap(&mut app, T);
    run(&mut app, 2);
    assert_eq!(profiles(&mut app), (false, false, false));
    let after = human_motion(&mut app);
    assert_eq!(before.0, 190.0, "walk speed constant");
    assert_eq!(before, after, "speed and jump height unchanged");
}

#[test]
fn the_spectral_profile_moves_slower_cannot_jump_and_drifts_down() {
    let mut app = spectral_game("v19_profile");
    give_relic(&mut app);
    stand(&mut app, (300.0, 63.0));
    tap(&mut app, T);
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::ShiftLeft);
    hold(&mut app, KeyCode::ArrowRight, 10);
    assert_eq!(velocity(&mut app).x, 150.0, "no run in the form");
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .release(KeyCode::ShiftLeft);
    let y = player_xy(&mut app).y;
    tap(&mut app, KeyCode::KeyZ);
    run(&mut app, 10);
    assert!((player_xy(&mut app).y - y).abs() < 0.5, "no jump");
    // A drop towards the approach's ledge (one-way at y 190..206) is slow.
    place(&mut app, (930.0, 600.0));
    let mut slowest = 0.0f32;
    for _ in 0..60 {
        run(&mut app, 1);
        slowest = slowest.min(velocity(&mut app).y);
    }
    assert!(slowest >= -220.0 - 0.01, "fall capped at 220: {slowest}");
    assert!(slowest < -200.0, "it does fall: {slowest}");
}

#[test]
fn ordinary_walls_stop_the_spectral_form() {
    let mut app = spectral_game("v19_walls");
    give_relic(&mut app);
    // The approach's block (420..540, 40..90) and its left grate.
    stand(&mut app, (300.0, 63.0));
    tap(&mut app, T);
    hold(&mut app, KeyCode::ArrowRight, 90);
    let p = player_xy(&mut app);
    assert!(
        (p.x - (420.0 - 13.0)).abs() < 0.5,
        "stopped by the block: {p}"
    );
    // The shrine's left wall (no door): a solid boundary.
    enter(&mut app, SHRINE, "to_well");
    stand(&mut app, (200.0, 63.0));
    assert!(
        form(&mut app).is_active(),
        "the form survives a room change"
    );
    hold(&mut app, KeyCode::ArrowLeft, 120);
    let p = player_xy(&mut app);
    assert!((p.x - 53.0).abs() < 0.5, "stopped by the wall: {p}");
    assert_eq!(current(&app), SHRINE);
}

// --- Veils ---------------------------------------------------------------

#[test]
fn a_human_stands_on_a_floor_veil_and_the_form_sinks_through_it() {
    let mut app = spectral_game("v19_floor_veil");
    give_relic(&mut app);
    enter(&mut app, GALLERY, "from_shaft");
    stand(&mut app, ON_FLOOR_VEIL);
    run(&mut app, 30);
    assert!(grounded(&mut app), "the veil holds a human");
    assert!((player_xy(&mut app).y - 63.0).abs() < 0.5);
    tap(&mut app, T);
    let frames = hold_until(&mut app, KeyCode::ArrowDown, 240, |a| current(a) == WELL);
    assert_eq!(current(&app), WELL, "sank into the well ({frames} frames)");
    assert!(
        form(&mut app).is_active(),
        "the form survives the room change"
    );
    // It drifts down onto the catching platform (380..400).
    run(&mut app, 200);
    assert!(grounded(&mut app));
    assert!(
        (player_xy(&mut app).y - 423.0).abs() < 0.5,
        "{}",
        player_xy(&mut app)
    );
}

#[test]
fn side_veils_stop_humans_and_let_the_form_through() {
    let mut app = spectral_game("v19_side_veil");
    give_relic(&mut app);
    enter(&mut app, WELL, "to_shrine");
    stand(&mut app, BEFORE_WEST_VEIL);
    hold(&mut app, KeyCode::ArrowLeft, 90);
    let p = player_xy(&mut app);
    assert!(
        (p.x - (120.0 + 13.0)).abs() < 0.5,
        "a human is stopped: {p}"
    );
    assert_eq!(current(&app), WELL);
    tap(&mut app, T);
    let frames = hold_until(&mut app, KeyCode::ArrowLeft, 120, |a| current(a) == SHRINE);
    assert_eq!(current(&app), SHRINE, "{frames} frames");
    // Arrival past the shrine's own veil (x > 1280 - 120), where a human fits.
    let p = player_xy(&mut app);
    assert!(p.x < 1280.0 - 120.0 - 13.0, "{p}");
    tap(&mut app, T);
    assert!(!form(&mut app).is_active());
    assert!(!human_blocked(&mut app));
}

#[test]
fn enemies_and_spells_do_not_pass_veils() {
    let mut app = spectral_game("v19_veil_solid");
    give_relic(&mut app);
    enter(&mut app, WELL, "to_shrine");
    let geometry = app.world().resource::<RoomGeometry>().clone();
    let veil = geometry
        .solids
        .iter()
        .find(|s| s.barrier.is_some())
        .unwrap_or_else(|| panic!("no veil in the well"));
    // Projectiles and enemies use `blocks` / plain collision: a veil is solid.
    assert!(geometry.blocks(&veil.aabb));
    assert!(!veil.one_way);
}

#[test]
fn ending_inside_a_veil_waits_until_the_human_body_fits() {
    let mut app = spectral_in_the_well("v19_cancel_inside");
    hold_until(&mut app, KeyCode::ArrowLeft, 60, |a| {
        player_xy(a).x <= IN_WEST_VEIL.0
    });
    assert!(human_blocked(&mut app), "inside the veil");
    let spent = form(&mut app).spent();
    tap(&mut app, T);
    let f = form(&mut app);
    assert!(f.is_active(), "not restored inside the veil");
    assert_eq!(f.ending(), Some(EndReason::Voluntary));
    assert_eq!(profiles(&mut app), (true, true, true), "still phased");
    run(&mut app, 120);
    assert_eq!(form(&mut app).spent(), spent, "no upkeep while ending");
    // No teleport: the player stays where it was until it walks out.
    let p = player_xy(&mut app);
    assert!(human_blocked(&mut app), "{p}");
    hold_until(&mut app, KeyCode::ArrowRight, 60, |a| !form(a).is_active());
    assert!(!form(&mut app).is_active());
    assert!(!human_blocked(&mut app));
    let after = player_xy(&mut app);
    assert!(
        after.x >= 133.0 && after.x < 140.0,
        "just out of the veil: {after}"
    );
}

#[test]
fn mp_running_out_inside_a_veil_never_traps_the_player() {
    let mut app = spectral_in_the_well("v19_mp_in_veil");
    hold_until(&mut app, KeyCode::ArrowLeft, 60, |a| {
        player_xy(a).x <= IN_WEST_VEIL.0
    });
    set_mp(&mut app, 0);
    run(&mut app, 30);
    let f = form(&mut app);
    assert_eq!(f.ending(), Some(EndReason::MpEmpty));
    assert!(f.is_active(), "kept until the body fits");
    // Either way out works: forward through the door...
    let frames = hold_until(&mut app, KeyCode::ArrowLeft, 120, |a| current(a) == SHRINE);
    assert_eq!(current(&app), SHRINE, "{frames}");
    run(&mut app, 1);
    assert!(!form(&mut app).is_active(), "human once past the veil");
    assert!(!human_blocked(&mut app));
}

#[test]
fn a_room_change_mid_crossing_keeps_the_form_and_no_duplicates() {
    let mut app = spectral_in_the_well("v19_room_change");
    run(&mut app, 10);
    assert_eq!(count::<With<FormOutline>>(&mut app), 1);
    hold_until(&mut app, KeyCode::ArrowLeft, 120, |a| current(a) == SHRINE);
    run(&mut app, 2);
    assert!(form(&mut app).is_active());
    assert_eq!(count::<With<FormOutline>>(&mut app), 1);
    assert_eq!(count::<With<Player>>(&mut app), 1);
    // Back and forth through the two-sided veil, many times.
    for _ in 0..4 {
        set_mp(&mut app, 40);
        hold_until(&mut app, KeyCode::ArrowRight, 200, |a| current(a) == WELL);
        hold_until(&mut app, KeyCode::ArrowLeft, 200, |a| current(a) == SHRINE);
    }
    assert_eq!(current(&app), SHRINE);
    assert_eq!(count::<With<FormOutline>>(&mut app), 1);
    assert!(count::<With<FormMote>>(&mut app) < 20, "motes expire");
    tap(&mut app, T);
    run(&mut app, 60);
    assert_eq!(count::<With<FormOutline>>(&mut app), 0);
    assert_eq!(count::<With<FormMote>>(&mut app), 0);
}

// --- Combat, damage, death ------------------------------------------------

#[test]
fn the_form_blocks_attacks_spells_guard_dodge_and_objects() {
    let mut app = spectral_game("v19_combat");
    give_relic(&mut app);
    enter(&mut app, SHRINE, "to_well");
    stand(&mut app, (908.0 + 60.0, 63.0));
    tap(&mut app, T);
    let mp_before = mp(&mut app);
    tap(&mut app, KeyCode::KeyX);
    run(&mut app, 2);
    assert_ne!(state(&mut app), PlayerState::Attacking);
    assert_eq!(count::<With<Hitbox>>(&mut app), 0);
    tap(&mut app, KeyCode::KeyC);
    run(&mut app, 2);
    assert_eq!(count::<With<Projectile>>(&mut app), 0);
    assert!(mp(&mut app) >= mp_before - 1, "no spell paid");
    hold(&mut app, KeyCode::KeyA, 5);
    assert_ne!(state(&mut app), PlayerState::Guarding);
    tap(&mut app, KeyCode::KeyS);
    assert_ne!(state(&mut app), PlayerState::Dodging);
    // The chest does not open for the form.
    place(&mut app, (908.0, 63.0));
    run(&mut app, 2);
    tap(&mut app, KeyCode::KeyD);
    run(&mut app, 2);
    assert!(!exploration(&app).pickups.contains("vc_shrine_chest"));
    // Back to human: the same keys work again.
    tap(&mut app, T);
    run(&mut app, 2);
    tap(&mut app, KeyCode::KeyD);
    run(&mut app, 2);
    assert!(exploration(&app).pickups.contains("vc_shrine_chest"));
    tap(&mut app, KeyCode::KeyX);
    assert_eq!(state(&mut app), PlayerState::Attacking);
}

#[test]
fn the_form_takes_damage_like_the_human() {
    let mut app = spectral_in_the_well("v19_damage");
    hold_until(&mut app, KeyCode::ArrowLeft, 60, |a| {
        player_xy(a).x <= IN_WEST_VEIL.0
    });
    let hp = player_hp(&mut app);
    hit_player(&mut app, 7);
    run(&mut app, 1);
    assert_eq!(player_hp(&mut app), hp - 7, "no immunity");
    assert_eq!(state(&mut app), PlayerState::Hurt);
    assert!(form(&mut app).is_active(), "damage does not end the form");
    // A hit while ending (inside the veil) changes nothing of the rule.
    tap(&mut app, T);
    run(&mut app, 60);
    hit_player(&mut app, 3);
    run(&mut app, 1);
    let f = form(&mut app);
    assert!(
        f.is_active() == human_blocked(&mut app),
        "spectral only while inside"
    );
}

#[test]
fn death_inside_a_floor_veil_holds_the_body_until_it_rises() {
    let mut app = spectral_game("v19_death");
    give_relic(&mut app);
    enter(&mut app, GALLERY, "from_shaft");
    stand(&mut app, ON_FLOOR_VEIL);
    tap(&mut app, T);
    run(&mut app, 8);
    assert!(human_blocked(&mut app), "sinking inside the floor veil");
    hit_player(&mut app, 10_000);
    run(&mut app, 1);
    assert_eq!(state(&mut app), PlayerState::Dead);
    let held = player_xy(&mut app);
    run(&mut app, 120);
    assert_eq!(player_xy(&mut app), held, "held where it fell, not sinking");
    assert_eq!(current(&app), GALLERY);
    assert_eq!(form(&mut app).ending(), Some(EndReason::Death));
    tap(&mut app, KeyCode::KeyR);
    run(&mut app, 2);
    assert_eq!(state(&mut app), PlayerState::Idle);
    assert!(!form(&mut app).is_active());
    assert_eq!(profiles(&mut app), (false, false, false));
    assert!(!human_blocked(&mut app));
}

#[test]
fn death_outside_a_veil_ends_the_form_at_once() {
    let mut app = spectral_game("v19_death_out");
    give_relic(&mut app);
    tap(&mut app, T);
    hit_player(&mut app, 10_000);
    run(&mut app, 1);
    assert!(!form(&mut app).is_active());
    assert_eq!(profiles(&mut app), (false, false, false));
}

// --- Save and load ------------------------------------------------------

#[test]
fn saving_inside_a_veil_is_refused_and_a_load_restores_the_human_form() {
    let saves = storage("v19_save");
    let mut app = game(&saves);
    app.init_resource::<Notes>().add_systems(PostUpdate, record);
    give_relic(&mut app);
    enter(&mut app, WELL, "to_shrine");
    stand(&mut app, BEFORE_WEST_VEIL);
    tap(&mut app, T);
    hold_until(&mut app, KeyCode::ArrowLeft, 60, |a| {
        player_xy(a).x <= IN_WEST_VEIL.0
    });
    tap(&mut app, KeyCode::F5);
    run(&mut app, 1);
    assert!(noted(&app, "Cannot save inside a veil"));
    assert!(slot_bytes(&saves).is_none(), "nothing written");
    // Out of the veil, still spectral: the save works.
    hold_until(&mut app, KeyCode::ArrowRight, 60, |a| {
        player_xy(a).x >= 200.0
    });
    assert!(form(&mut app).is_active());
    let saved_at = player_xy(&mut app);
    tap(&mut app, KeyCode::F5);
    run(&mut app, 1);
    assert!(slot_bytes(&saves).is_some());
    // Somewhere else, spectral: the load comes back human at the same spot.
    hold_until(&mut app, KeyCode::ArrowLeft, 200, |a| current(a) == SHRINE);
    tap(&mut app, KeyCode::F9);
    assert_eq!(current(&app), WELL);
    assert!(!form(&mut app).is_active());
    assert_eq!(profiles(&mut app), (false, false, false));
    let p = player_xy(&mut app);
    assert!((p.x - saved_at.x).abs() < 0.5, "{p} vs {saved_at}");
    assert!(!human_blocked(&mut app));
    assert!(
        exploration(&app)
            .capabilities()
            .abilities
            .contains(&Ability::VeilStep),
        "the unlock is kept (it is the relic)"
    );
}

#[test]
fn transforming_and_loading_in_the_same_frame_ends_human() {
    let saves = storage("v19_t_and_load");
    let mut app = game(&saves);
    give_relic(&mut app);
    tap(&mut app, KeyCode::F5);
    run(&mut app, 2);
    let mp_saved = mp(&mut app);
    app.world_mut()
        .resource_mut::<PendingPress>()
        .0
        .extend([T, KeyCode::F9]);
    app.update();
    assert!(!form(&mut app).is_active());
    assert_eq!(profiles(&mut app), (false, false, false));
    assert!(
        (mp(&mut app) - mp_saved).abs() <= 1,
        "the loaded MP, nothing lost twice"
    );
}

// --- Progression -----------------------------------------------------------

#[test]
fn the_validator_needs_the_form_unlock_for_veiled_doors() {
    let w = world();
    let report = validate(&w);
    assert!(report.is_ok(), "{:#?}", report.errors);
    let gallery = "vigil_gates_gallery";
    let without = reachable_from(&w, gallery, &Capabilities::default(), Rules::ALL);
    assert!(!without.contains(WELL) && !without.contains(SHRINE));
    let mut caps = Capabilities::default();
    caps.abilities.insert(Ability::VeilStep);
    let with = reachable_from(&w, gallery, &caps, Rules::ALL);
    assert!(with.contains(WELL) && with.contains(SHRINE));
    // Another ability (a relic that is not the form's unlock) does not do.
    let mut other = Capabilities::default();
    other.abilities.insert(Ability::AstralKey);
    assert!(!reachable_from(&w, gallery, &other, Rules::ALL).contains(WELL));
    // Without a passage for spectral veils, the cloister is unreachable.
    let mut no_passage = w.clone();
    no_passage.manifest.passages.clear();
    let report = validate(&no_passage);
    assert!(
        report
            .errors
            .iter()
            .any(|e| matches!(e, dark_castle_rpg::world::validation::Issue::Unreachable { room } if room == WELL))
    );
}

#[test]
fn every_veil_can_be_crossed_with_the_starting_mp() {
    use dark_castle_rpg::transform::crossing::crossing_mp;
    let data =
        dark_castle_rpg::core::resources::GameData::load(root()).unwrap_or_else(|e| panic!("{e}"));
    let w = world();
    assert!(dark_castle_rpg::app::check_transformations(&w, &data).is_ok());
    let def = data
        .transformations
        .primary()
        .unwrap_or_else(|| panic!("no form"));
    let mut veils = 0;
    for room in w.rooms.values() {
        for veil in &room.veils {
            let cost = crossing_mp(def, room, veil).unwrap_or_else(|| panic!("{}", veil.id));
            assert!(cost < START_MP as f32, "{} costs {cost}", veil.id);
            veils += 1;
        }
    }
    assert_eq!(veils, 3);
    // A veil far too deep for the MP is refused at load.
    let mut deep = w.clone();
    let well = deep.rooms.get_mut(WELL).unwrap_or_else(|| panic!("well"));
    well.veils[0].depth = 120.0;
    let mut poor = data.clone();
    poor.character.start.attributes.intelligence = 0;
    poor.character.formulas.base_mp = 0;
    poor.character.formulas.mp_per_level = 0;
    assert!(dark_castle_rpg::app::check_transformations(&deep, &poor).is_err());
}

// --- Performance ----------------------------------------------------------

/// Measures (written to `<target>/tmp/world_v19_perf.txt`, compared with
/// the V1.8 numbers in docs/TRANSFORMATIONS_V1_9.md §20): activation and
/// end frames, the phased collision against the plain one, entities of the
/// form, a room change while spectral, save and load frames.
#[test]
fn performance_of_the_spectral_form() {
    use dark_castle_rpg::world::collision::BarrierKind;
    use dark_castle_rpg::world::collision::{move_and_collide, move_and_collide_phased};
    use std::fmt::Write as _;
    use std::time::Instant;
    let saves = storage("v19_perf");
    let mut app = game(&saves);
    give_relic(&mut app);
    enter(&mut app, WELL, "to_shrine");
    stand(&mut app, BEFORE_WEST_VEIL);
    let mut out = String::new();
    let frames = |app: &mut App, n: usize| {
        let t = Instant::now();
        run(app, n);
        t.elapsed().as_secs_f64() * 1000.0 / n as f64
    };
    let plain = frames(&mut app, 60);
    let entities_human = count::<()>(&mut app);
    let t = Instant::now();
    tap(&mut app, T);
    let activation = t.elapsed().as_secs_f64() * 1000.0;
    let spectral = frames(&mut app, 60);
    let entities_spectral = count::<()>(&mut app);
    let motes = count::<With<FormMote>>(&mut app);
    let t = Instant::now();
    hold_until(&mut app, KeyCode::ArrowLeft, 200, |a| current(a) == SHRINE);
    let crossing_frames = t.elapsed().as_secs_f64() * 1000.0;
    let t = Instant::now();
    tap(&mut app, T);
    let end = t.elapsed().as_secs_f64() * 1000.0;
    assert!(!form(&mut app).is_active());
    run(&mut app, 60);
    let entities_after = count::<()>(&mut app);
    let t = Instant::now();
    tap(&mut app, KeyCode::F5);
    let save = t.elapsed().as_secs_f64() * 1000.0;
    let t = Instant::now();
    tap(&mut app, KeyCode::F9);
    let load = t.elapsed().as_secs_f64() * 1000.0;
    // Collision: the plain and the phased step over the well's solids.
    enter(&mut app, WELL, "to_shrine");
    let solids = app.world().resource::<RoomGeometry>().solids.clone();
    let body = Aabb::new((200.0, 63.0), (26.0, 46.0));
    let n = 100_000;
    // Warm-up, so the first loop is not penalised.
    for i in 0..n {
        std::hint::black_box(move_and_collide_phased(
            body,
            (150.0, -(i % 7) as f32),
            0.016,
            &solids,
            &[],
        ));
    }
    let t = Instant::now();
    for i in 0..n {
        std::hint::black_box(move_and_collide(
            body,
            (150.0, -(i % 7) as f32),
            0.016,
            &solids,
        ));
    }
    let plain_step = t.elapsed().as_secs_f64() * 1e9 / n as f64;
    let t = Instant::now();
    for i in 0..n {
        std::hint::black_box(move_and_collide_phased(
            body,
            (150.0, -(i % 7) as f32),
            0.016,
            &solids,
            &[BarrierKind::Spectral],
        ));
    }
    let phased_step = t.elapsed().as_secs_f64() * 1e9 / n as f64;
    let _ = writeln!(
        out,
        "plain frame {plain:.3} ms; spectral frame {spectral:.3} ms; activation frame {activation:.3} ms; end frame {end:.3} ms"
    );
    let _ = writeln!(
        out,
        "entities (whole app): human in the well {entities_human}, spectral in the well {entities_spectral} (motes {motes}), human in the shrine after the end {entities_after}"
    );
    let _ = writeln!(
        out,
        "veil crossing + room change while spectral: {crossing_frames:.2} ms in total; F5 frame {save:.3} ms; F9 frame {load:.3} ms"
    );
    let _ = writeln!(
        out,
        "collision step over {} solids: plain {plain_step:.1} ns, phased {phased_step:.1} ns",
        solids.len()
    );
    let path = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("world_v19_perf.txt");
    let _ = std::fs::write(&path, &out);
    print!("{out}");
    // `entities_after` is measured in the shrine (it has a chest): what must
    // be gone are the form's own entities.
    assert_eq!(count::<With<FormMote>>(&mut app), 0);
    assert_eq!(count::<With<FormOutline>>(&mut app), 0);
    assert_eq!(
        count::<With<dark_castle_rpg::transform::visual::FormHalo>>(&mut app),
        0
    );
}
