//! D22: Ashen Sentinel damaged only through the player's attack pipeline.

use bevy::prelude::*;
use dark_castle_rpg::combat::hitboxes::{Hitbox, Team};
use dark_castle_rpg::core::events::{Arrival, RoomRequest};
use dark_castle_rpg::enemies::boss::{BossPattern, BossPhase};
use dark_castle_rpg::enemies::boss_systems::BossEncounter;
use dark_castle_rpg::enemies::components::{Boss, BossRuntime, EnemyBrain, EnemyHealth};
use dark_castle_rpg::magic::projectiles::Projectile;
use dark_castle_rpg::player::components::{ActionState, Belongings, Player, Vitals};
use dark_castle_rpg::player::state::PlayerState;
use dark_castle_rpg::world::WorldState;
use dark_castle_rpg::world::physics::FacingDirection;
use dark_castle_rpg::world::physics::Grounded;
use dark_castle_rpg::world::transitions::TransitionLock;

#[path = "support/world_app.rs"]
mod world_app;

use world_app::*;

const COURT: &str = "vigil_gates_ashen_court";

fn enter_court(app: &mut App) {
    app.world_mut().write_message(RoomRequest {
        room: COURT.into(),
        arrival: Arrival::Position(Vec2::new(400.0, 63.0)),
    });
    run(app, 4);
}

fn boss_hp(app: &mut App) -> i32 {
    let world = app.world_mut();
    world
        .query_filtered::<&EnemyHealth, With<Boss>>()
        .iter(world)
        .next()
        .map(|h| h.current())
        .unwrap_or(0)
}

fn boss_xy(app: &mut App) -> Vec2 {
    let world = app.world_mut();
    world
        .query_filtered::<&Transform, With<Boss>>()
        .iter(world)
        .next()
        .map(|t| t.translation.truncate())
        .unwrap_or_else(|| panic!("boss"))
}

fn face_boss(app: &mut App) {
    let boss = boss_xy(app);
    let world = app.world_mut();
    let mut q = world.query_filtered::<(&Transform, &mut FacingDirection), With<Player>>();
    let (transform, mut facing) = q.single_mut(world).unwrap_or_else(|e| panic!("{e}"));
    let sign = (boss.x - transform.translation.x).signum();
    if sign < 0.0 {
        facing.0 = dark_castle_rpg::player::state::Facing::Left;
    } else {
        facing.0 = dark_castle_rpg::player::state::Facing::Right;
    }
}

fn swing(app: &mut App) {
    face_boss(app);
    tap(app, KeyCode::KeyX);
    run(app, 20);
}

struct Sight {
    slash: bool,
    lunge: bool,
    wave: bool,
    bolt: bool,
    phase_two: bool,
}

fn sight(app: &mut App) -> Sight {
    let mut sight = Sight {
        slash: false,
        lunge: false,
        wave: false,
        bolt: false,
        phase_two: false,
    };
    let world = app.world_mut();
    if let Some(runtime) = world.query::<&BossRuntime>().iter(world).next() {
        match runtime.last_resolved {
            Some(BossPattern::FrontalSlash) => sight.slash = true,
            Some(BossPattern::Lunge) => sight.lunge = true,
            Some(BossPattern::AshWave) => sight.wave = true,
            None => {}
        }
        // Flags survive the next pattern, so a later sample still counts.
        sight.slash |= runtime.resolved_slash;
        sight.lunge |= runtime.resolved_lunge;
        if runtime.phase == BossPhase::Two {
            sight.phase_two = true;
        }
    }
    sight.bolt = world
        .query_filtered::<&Hitbox, With<Projectile>>()
        .iter(world)
        .any(|h| h.team == Team::Enemy);
    sight
}

/// Harness heal so the script can finish the fight. Boss HP is never written.
fn top_up(app: &mut App) {
    let world = app.world_mut();
    let mut q = world.query_filtered::<&mut Vitals, With<Player>>();
    let Ok(mut vitals) = q.single_mut(world) else {
        return;
    };
    if vitals.hp.current() < vitals.hp.max() {
        vitals.hp.refill();
    }
}

fn boss_debug(app: &mut App) -> String {
    let world = app.world_mut();
    let boss = world
        .query_filtered::<(&EnemyHealth, &BossRuntime, &EnemyBrain, &Grounded, &Transform), With<Boss>>()
        .iter(world)
        .next()
        .map(|(hp, runtime, brain, grounded, t)| {
            format!(
                "hp={} phase={:?} lock={:.2} pattern={:?} last={:?} behavior={:?} windup={:?} cd={:.2} flash={:.2} grounded={} at=({:.0},{:.0})",
                hp.current(),
                runtime.phase,
                runtime.phase_lock,
                runtime.next_pattern,
                runtime.last_resolved,
                brain.behavior,
                brain.windup,
                brain.attack_cooldown,
                brain.flash,
                grounded.on_ground,
                t.translation.x,
                t.translation.y,
            )
        })
        .unwrap_or_else(|| "no boss".into());
    let player = world
        .query_filtered::<(&Vitals, &ActionState), With<Player>>()
        .single(world)
        .map(|(v, s)| format!("player hp={} state={:?}", v.hp.current(), s.state()))
        .unwrap_or_else(|_| "no player".into());
    format!("{boss} | {player}")
}

fn windup_left(app: &mut App) -> Option<f32> {
    let world = app.world_mut();
    world
        .query_filtered::<&EnemyBrain, With<Boss>>()
        .iter(world)
        .next()
        .and_then(|brain| brain.windup)
}

fn player_state(app: &mut App) -> PlayerState {
    let world = app.world_mut();
    world
        .query_filtered::<&ActionState, With<Player>>()
        .single(world)
        .map(|state| state.state())
        .unwrap_or_else(|e| panic!("{e}"))
}

/// Just outside the body, still inside the phase-1 swing.
fn beside(app: &mut App) {
    let boss = boss_xy(app);
    place(app, ((boss.x + 52.0).clamp(780.0, 1200.0), 72.0));
    face_boss(app);
}

#[test]
fn guard_chips_a_frontal_hit_without_stagger() {
    let mut app = game(&storage("d22_guard"));
    enter_court(&mut app);
    let start = player_hp(&mut app);
    let mut chipped = false;
    for _ in 0..500 {
        beside(&mut app);
        app.world_mut()
            .resource_mut::<PendingPress>()
            .0
            .push(KeyCode::KeyA);
        run(&mut app, 1);
        let hp = player_hp(&mut app);
        if hp < start {
            chipped = true;
            assert_eq!(
                player_state(&mut app),
                PlayerState::Guarding,
                "a frontal hit staggered through the guard"
            );
            assert!(hp > 0, "guard let a single hit kill");
            break;
        }
    }
    assert!(chipped, "boss never hit a guarding player");
}

#[test]
/// Dodge away while the tell still has time left. A dodge started in the
/// last ~0.14s can be clipped: i-frames are 0.28s and the swing stays
/// active for 0.15s after the wind-up.
fn dodge_iframes_cover_one_committed_swing() {
    let mut app = game(&storage("d22_dodge"));
    enter_court(&mut app);
    let mut dodged = false;
    let mut hp_at_dodge = 0;
    for _ in 0..500 {
        if dodged {
            break;
        }
        beside(&mut app);
        if windup_left(&mut app).is_some_and(|left| (0.18..0.32).contains(&left)) {
            let boss = boss_xy(&mut app);
            let player = player_xy(&mut app);
            let world = app.world_mut();
            let mut facing = world
                .query_filtered::<&mut FacingDirection, With<Player>>()
                .single_mut(world)
                .unwrap_or_else(|e| panic!("{e}"));
            facing.0 = if player.x < boss.x {
                dark_castle_rpg::player::state::Facing::Left
            } else {
                dark_castle_rpg::player::state::Facing::Right
            };
            hp_at_dodge = player_hp(&mut app);
            tap(&mut app, KeyCode::KeyS);
            dodged = true;
            assert_eq!(
                player_state(&mut app),
                PlayerState::Dodging,
                "dodge rejected from hp {hp_at_dodge}"
            );
        } else {
            run(&mut app, 1);
        }
    }
    assert!(dodged, "boss never committed a swing");
    run(&mut app, 15);
    assert_eq!(
        player_hp(&mut app),
        hp_at_dodge,
        "dodge i-frames did not cover the swing"
    );
}

#[test]
fn player_attacks_deal_real_damage_and_resolve_all_patterns() {
    let mut app = game(&storage("d22_real"));
    enter_court(&mut app);
    let start_hp = boss_hp(&mut app);
    assert!(start_hp > 200, "boss hp {start_hp}");
    // Stand in sword range, left of the sentinel.
    let boss = boss_xy(&mut app);
    // Right of the sentinel, on the floor past the exit gap.
    place(&mut app, (boss.x + 48.0, 72.0));
    run(&mut app, 5);

    let mut saw_slash = false;
    let mut saw_lunge = false;
    let mut saw_wave = false;
    let mut saw_bolt = false;
    let mut saw_phase_two = false;
    for _ in 0..40 {
        top_up(&mut app);
        swing(&mut app);
        let seen = sight(&mut app);
        saw_slash |= seen.slash;
        saw_lunge |= seen.lunge;
        saw_wave |= seen.wave;
        saw_bolt |= seen.bolt;
        saw_phase_two |= seen.phase_two;
        if saw_slash {
            break;
        }
    }
    // Let the committed lunge finish without stun-locking the tell.
    for _ in 0..240 {
        run(&mut app, 1);
        let seen = sight(&mut app);
        saw_slash |= seen.slash;
        saw_lunge |= seen.lunge;
        saw_wave |= seen.wave;
        saw_bolt |= seen.bolt;
        saw_phase_two |= seen.phase_two;
        if saw_lunge {
            break;
        }
    }
    for _ in 0..200 {
        let hp = boss_hp(&mut app);
        if hp == 0 || (saw_phase_two && hp < start_hp / 2) {
            break;
        }
        let boss = boss_xy(&mut app);
        place(&mut app, ((boss.x + 32.0).clamp(780.0, 1200.0), 72.0));
        top_up(&mut app);
        swing(&mut app);
        let seen = sight(&mut app);
        saw_slash |= seen.slash;
        saw_lunge |= seen.lunge;
        saw_wave |= seen.wave;
        saw_bolt |= seen.bolt;
        saw_phase_two |= seen.phase_two;
    }
    for _ in 0..180 {
        if saw_wave || saw_bolt || boss_hp(&mut app) == 0 {
            break;
        }
        let boss = boss_xy(&mut app);
        // Outside min_range, still on the right-hand floor.
        let x = (boss.x - 220.0).clamp(820.0, 1180.0);
        place(&mut app, (x, 72.0));
        top_up(&mut app);
        run(&mut app, 1);
        let seen = sight(&mut app);
        saw_wave |= seen.wave;
        saw_bolt |= seen.bolt;
        saw_phase_two |= seen.phase_two;
    }
    let after_wait = boss_debug(&mut app);
    for _ in 0..200 {
        if boss_hp(&mut app) == 0 {
            break;
        }
        let boss = boss_xy(&mut app);
        place(&mut app, ((boss.x + 32.0).clamp(780.0, 1200.0), 72.0));
        top_up(&mut app);
        swing(&mut app);
    }

    assert!(
        boss_hp(&mut app) < start_hp,
        "boss HP never fell through player strikes"
    );
    assert!(
        saw_slash,
        "slash={saw_slash} lunge={saw_lunge} wave={saw_wave} bolt={saw_bolt} phase2={saw_phase_two} hp {start_hp}->{}",
        boss_hp(&mut app)
    );
    assert!(saw_lunge, "lunge not resolved");
    assert!(
        saw_phase_two,
        "phase two never started (hp {})",
        boss_hp(&mut app)
    );
    assert!(
        saw_wave || saw_bolt,
        "ash wave not observed phase2={saw_phase_two} after-wait: {after_wait}"
    );
    assert_eq!(
        boss_hp(&mut app),
        0,
        "boss still alive after the scripted fight {}",
        boss_debug(&mut app)
    );
    run(&mut app, 4);
    assert!(!app.world().resource::<BossEncounter>().active);
    assert!(!app.world().resource::<TransitionLock>().locked);
    assert!(
        app.world()
            .resource::<WorldState>()
            .0
            .defeated_bosses
            .contains("ashen_sentinel")
    );
    let brand = {
        let world = app.world_mut();
        world
            .query_filtered::<&Belongings, With<Player>>()
            .single(world)
            .map(|b| b.inventory.count("sentinel_brand"))
            .unwrap_or(0)
    };
    assert_eq!(brand, 1);
}

/// D23: a committed boss tell keeps counting while the hurt flash is up.
/// Ordinary enemies still lose their wind-up on hit; this only checks the boss.
#[test]
fn boss_tell_counts_down_while_the_hurt_flash_is_up() {
    let saves = storage("d23_tell_tick");
    let mut app = game(&saves);
    app.world_mut().write_message(RoomRequest {
        room: COURT.into(),
        arrival: Arrival::Position(Vec2::new(700.0, 80.0)),
    });
    run(&mut app, 6);
    {
        let world = app.world_mut();
        let mut brains = world.query_filtered::<&mut EnemyBrain, With<Boss>>();
        let mut brain = brains.single_mut(world).unwrap_or_else(|e| panic!("{e}"));
        brain.windup = Some(0.50);
        brain.flash = 0.20;
    }
    run(&mut app, 4);
    let left = {
        let world = app.world_mut();
        world
            .query_filtered::<&EnemyBrain, With<Boss>>()
            .single(world)
            .unwrap_or_else(|e| panic!("{e}"))
            .windup
    };
    let left = left.unwrap_or_else(|| panic!("windup cleared during the flash"));
    assert!(left < 0.48, "tell froze under the hurt flash ({left})");
    assert!(left > 0.30, "tell skipped ahead ({left})");
}
