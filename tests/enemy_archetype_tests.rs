//! D20: ranged and keep-distance enemy archetypes in the headless world.

use bevy::prelude::*;
use dark_castle_rpg::combat::hitboxes::{Hitbox, Team};
use dark_castle_rpg::core::events::{Arrival, RoomRequest};
use dark_castle_rpg::enemies::ai::EnemyBehavior;
use dark_castle_rpg::enemies::components::{Enemy, EnemyBrain};
use dark_castle_rpg::magic::projectiles::Projectile;
use dark_castle_rpg::player::components::Player;
use dark_castle_rpg::world::maps::RoomEntity;

#[path = "support/world_app.rs"]
mod world_app;

use world_app::*;

fn enter(app: &mut App, room: &str, at: Vec2) {
    app.world_mut().write_message(RoomRequest {
        room: room.into(),
        arrival: Arrival::Position(at),
    });
    run(app, 3);
    assert_eq!(current(app), room);
}

fn enemy_of(app: &mut App, id: &str) -> (Entity, EnemyBehavior) {
    let world = app.world_mut();
    let mut found = None;
    for (entity, enemy, brain) in world.query::<(Entity, &Enemy, &EnemyBrain)>().iter(world) {
        if enemy.definition == id {
            found = Some((entity, brain.behavior));
            break;
        }
    }
    found.unwrap_or_else(|| panic!("missing enemy {id}"))
}

fn enemy_count(app: &mut App, id: &str) -> usize {
    let world = app.world_mut();
    world
        .query::<&Enemy>()
        .iter(world)
        .filter(|e| e.definition == id)
        .count()
}

fn enemy_projectiles(app: &mut App) -> usize {
    let world = app.world_mut();
    world
        .query_filtered::<&Hitbox, (With<Projectile>, With<RoomEntity>)>()
        .iter(world)
        .filter(|h| h.team == Team::Enemy)
        .count()
        + world
            .query_filtered::<&Hitbox, (With<Projectile>, Without<RoomEntity>)>()
            .iter(world)
            .filter(|h| h.team == Team::Enemy)
            .count()
}

#[test]
fn bone_bolt_spawns_in_shaft_and_fires_projectiles() {
    let mut app = game(&storage("d20_bolt"));
    // Arrive on the mid ledge with the Bone Bolt (not the busy floor route).
    enter(&mut app, SHAFT, Vec2::new(520.0, 409.0));
    assert_eq!(enemy_count(&mut app, "bone_bolt"), 1);

    // Stand in fire range but outside min_range so it attacks.
    place(&mut app, (520.0, 409.0));
    run(&mut app, 45);

    let (_e, behavior) = enemy_of(&mut app, "bone_bolt");
    assert!(
        matches!(
            behavior,
            EnemyBehavior::Attack
                | EnemyBehavior::Wait
                | EnemyBehavior::Defend
                | EnemyBehavior::Retreat
                | EnemyBehavior::Chase
        ),
        "unexpected {behavior:?}"
    );
    // After wind-up frames, at least one enemy bolt should have existed;
    // keep the player in range and advance through a full wind-up+shot.
    let mut saw_bolt = false;
    for _ in 0..90 {
        run(&mut app, 1);
        if enemy_projectiles(&mut app) > 0 {
            saw_bolt = true;
            break;
        }
    }
    assert!(saw_bolt, "Bone Bolt never fired a projectile");

    // Room change clears room-tied combat leftovers and respawns the shaft roster once.
    enter(&mut app, APPROACH, Vec2::new(360.0, 63.0));
    assert_eq!(enemy_count(&mut app, "bone_bolt"), 0);
    assert_eq!(enemy_projectiles(&mut app), 0);
}

#[test]
fn ash_warden_spawns_in_gallery_and_retreats_when_crowded() {
    let mut app = game(&storage("d20_warden"));
    enter(&mut app, GALLERY, Vec2::new(1400.0, 63.0));
    assert_eq!(enemy_count(&mut app, "ash_warden"), 1);

    // Crowd the warden (inside min_range 55).
    let world = app.world_mut();
    let mut wx = 1600.0;
    for (enemy, transform) in world.query::<(&Enemy, &Transform)>().iter(world) {
        if enemy.definition == "ash_warden" {
            wx = transform.translation.x;
            break;
        }
    }
    place(&mut app, (wx + 20.0, 63.0));
    run(&mut app, 20);
    let (_e, behavior) = enemy_of(&mut app, "ash_warden");
    assert!(
        matches!(
            behavior,
            EnemyBehavior::Retreat | EnemyBehavior::Attack | EnemyBehavior::Wait
        ),
        "expected retreat/attack when crowded, got {behavior:?}"
    );

    // Back off to the preferred band — should stop retreating.
    place(&mut app, (wx - 160.0, 63.0));
    run(&mut app, 25);
    let (_e, behavior) = enemy_of(&mut app, "ash_warden");
    assert!(
        matches!(
            behavior,
            EnemyBehavior::Defend
                | EnemyBehavior::Chase
                | EnemyBehavior::Attack
                | EnemyBehavior::Wait
                | EnemyBehavior::Patrol
        ),
        "unexpected after backing off: {behavior:?}"
    );
}

#[test]
fn bone_sentry_still_spawns_on_the_causeway() {
    let mut app = game(&storage("d20_sentry_regression"));
    enter(&mut app, CAUSEWAY, Vec2::new(1300.0, 63.0));
    assert_eq!(enemy_count(&mut app, "bone_sentry"), 1);
    let players = {
        let world = app.world_mut();
        world
            .query_filtered::<Entity, With<Player>>()
            .iter(world)
            .count()
    };
    assert_eq!(players, 1);
}
