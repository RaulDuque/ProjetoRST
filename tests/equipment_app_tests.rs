//! V2.0: equipment in the headless main game (world graph active).
//!
//! The inventory screen driven by keys, a chest reward equipped, the strike
//! of each weapon, technique compatibility, the gate (attack, death, form,
//! passage), maxima, the appearance (cape, spectral cape), save/load and a
//! room change, chests that cannot be looted twice, and timings.
//! Saves go to directories under the target dir; `saves/` is never touched.

use std::time::Instant;

use bevy::prelude::*;
use dark_castle_rpg::app::states::GameState;
use dark_castle_rpg::combat::attacks::WeaponStrike;
use dark_castle_rpg::combat::hitboxes::{CombatProfile, Hitbox};
use dark_castle_rpg::core::events::{
    Arrival, CommandResolved, EquipOp, EquipRequest, Notification, RoomRequest, SkillActivated,
};
use dark_castle_rpg::core::resources::GameData;
use dark_castle_rpg::inventory::equipment::EquipmentSlot;
use dark_castle_rpg::inventory::items::ItemCategory;
use dark_castle_rpg::inventory::systems::EquipmentAppearance;
use dark_castle_rpg::inventory::visual::CapeLayer;
use dark_castle_rpg::player::components::{Belongings, CharacterSheet, Player, Vitals};
use dark_castle_rpg::player::state::PlayerState;
use dark_castle_rpg::skills::execution::CommandOutcome;
use dark_castle_rpg::transform::systems::Transformation;
use dark_castle_rpg::ui::inventory_screen::{self, InventoryCursor, Row};
use dark_castle_rpg::world::exploration_systems::Exploration;
use dark_castle_rpg::world::graph::{Ability, Grant};
use dark_castle_rpg::world::physics::Phasing;
use dark_castle_rpg::world::transitions::TransitionLock;

#[path = "support/world_app.rs"]
mod world_app;

use world_app::*;

const I: KeyCode = KeyCode::KeyI;
const ENTER: KeyCode = KeyCode::Enter;
const DOWN: KeyCode = KeyCode::ArrowDown;
const USE: KeyCode = KeyCode::KeyD;
/// Standing on the approach's equipment chest (x 240..288).
const AT_APPROACH_CHEST: (f32, f32) = (264.0, 63.0);
/// Standing on the causeway's chest (x 1800..1848).
const AT_CAUSEWAY_CHEST: (f32, f32) = (1824.0, 63.0);

#[derive(Resource, Default)]
struct Notes(Vec<String>);

#[derive(Resource, Default)]
struct Outcomes(Vec<CommandOutcome>);

fn record(
    mut messages: MessageReader<Notification>,
    mut resolved: MessageReader<CommandResolved>,
    mut notes: ResMut<Notes>,
    mut outcomes: ResMut<Outcomes>,
) {
    notes.0.extend(messages.read().map(|n| n.0.clone()));
    outcomes
        .0
        .extend(resolved.read().map(|r| r.outcome.clone()));
}

/// Holds the room-transition lock in the frame of an equipment request.
#[derive(Resource, Default)]
struct HoldLock(bool);

fn hold_lock(hold: Res<HoldLock>, mut lock: ResMut<TransitionLock>) {
    if hold.0 {
        lock.locked = true;
    }
}

fn equip_game(name: &str) -> App {
    let mut app = game(&storage(name));
    app.init_resource::<HoldLock>().add_systems(
        Update,
        hold_lock
            .in_set(dark_castle_rpg::app::states::GameSet::Presentation)
            .before(dark_castle_rpg::inventory::systems::apply_equip_requests),
    );
    app.init_resource::<Notes>()
        .init_resource::<Outcomes>()
        .add_systems(PostUpdate, record);
    app
}

fn noted(app: &App, text: &str) -> bool {
    app.world()
        .resource::<Notes>()
        .0
        .iter()
        .any(|n| n.contains(text))
}

fn belongings(app: &mut App) -> Belongings {
    let world = app.world_mut();
    world
        .query_filtered::<&Belongings, With<Player>>()
        .single(world)
        .cloned()
        .unwrap_or_else(|e| panic!("{e}"))
}

fn sheet(app: &mut App) -> CharacterSheet {
    let world = app.world_mut();
    world
        .query_filtered::<&CharacterSheet, With<Player>>()
        .single(world)
        .cloned()
        .unwrap_or_else(|e| panic!("{e}"))
}

fn vitals(app: &mut App) -> Vitals {
    let world = app.world_mut();
    world
        .query_filtered::<&Vitals, With<Player>>()
        .single(world)
        .cloned()
        .unwrap_or_else(|e| panic!("{e}"))
}

fn state(app: &App) -> GameState {
    *app.world().resource::<State<GameState>>().get()
}

fn stand(app: &mut App, p: (f32, f32)) {
    place(app, p);
    run(app, 3);
}

fn request(app: &mut App, slot: EquipmentSlot, op: EquipOp) {
    app.world_mut().write_message(EquipRequest { slot, op });
    run(app, 1);
}

fn equip(app: &mut App, slot: EquipmentSlot, item: &str) {
    request(app, slot, EquipOp::Equip(item.into()));
}

/// Puts items in the bag as if found (the reward path of chests).
fn give(app: &mut App, items: &[&str]) {
    let data = app
        .world()
        .resource::<dark_castle_rpg::core::resources::GameData>()
        .clone();
    let world = app.world_mut();
    let mut q = world.query_filtered::<&mut Belongings, With<Player>>();
    let mut b = q.single_mut(world).unwrap_or_else(|e| panic!("{e}"));
    for id in items {
        b.inventory
            .add(id, 1, &data.items)
            .unwrap_or_else(|e| panic!("{e}"));
    }
}

fn give_relic(app: &mut App) {
    app.world_mut()
        .resource_mut::<Exploration>()
        .granted
        .push(Grant::Ability(Ability::VeilStep));
}

fn player_hitboxes(app: &mut App) -> Vec<(Hitbox, Option<WeaponStrike>)> {
    let world = app.world_mut();
    world
        .query::<(&Hitbox, Option<&WeaponStrike>)>()
        .iter(world)
        .filter(|(h, _)| h.team == dark_castle_rpg::combat::hitboxes::Team::Player)
        .map(|(h, w)| (h.clone(), w.copied()))
        .collect()
}

fn player_state(app: &mut App) -> PlayerState {
    let world = app.world_mut();
    world
        .query_filtered::<&dark_castle_rpg::player::components::ActionState, With<Player>>()
        .single(world)
        .map(|s| s.state())
        .unwrap_or_else(|e| panic!("{e}"))
}

#[test]
fn a_chest_weapon_is_equipped_through_the_inventory_screen_keys() {
    let mut app = equip_game("equip_screen");
    stand(&mut app, AT_APPROACH_CHEST);
    tap(&mut app, USE);
    run(&mut app, 2);
    assert_eq!(belongings(&mut app).inventory.count("watch_pike"), 1);
    let atk_before = sheet(&mut app).derived().attack;

    tap(&mut app, I);
    run(&mut app, 1);
    assert_eq!(state(&app), GameState::Inventory);
    // Slots first, then bag rows (D18: consumables are selectable too).
    for _ in 0..32 {
        let cursor = app.world().resource::<InventoryCursor>().0;
        let b = belongings(&mut app);
        let catalog = &app.world().resource::<GameData>().items;
        let rows = inventory_screen::rows(&b, catalog);
        if let Some(Row::Bag(id)) = rows.get(cursor)
            && id == "watch_pike"
        {
            break;
        }
        tap(&mut app, DOWN);
        run(&mut app, 1);
    }
    {
        let cursor = app.world().resource::<InventoryCursor>().0;
        let b = belongings(&mut app);
        let catalog = &app.world().resource::<GameData>().items;
        let rows = inventory_screen::rows(&b, catalog);
        assert_eq!(rows.get(cursor), Some(&Row::Bag("watch_pike".into())));
    }
    tap(&mut app, ENTER);
    run(&mut app, 1);
    let b = belongings(&mut app);
    assert_eq!(b.equipment.get(EquipmentSlot::Weapon), Some("watch_pike"));
    assert_eq!(b.inventory.count("iron_sword"), 1);
    assert_eq!(b.inventory.count("watch_pike"), 0);
    // Iron Sword +8 ATK -> Watch Pike +9 ATK, already while paused.
    assert_eq!(sheet(&mut app).derived().attack, atk_before + 1);
    tap(&mut app, I);
    run(&mut app, 1);
    assert_eq!(state(&app), GameState::Playing);
    // The chest cannot be looted again.
    stand(&mut app, AT_APPROACH_CHEST);
    tap(&mut app, USE);
    run(&mut app, 2);
    let b = belongings(&mut app);
    assert_eq!(
        b.inventory.count("watch_pike")
            + u32::from(b.equipment.get(EquipmentSlot::Weapon) == Some("watch_pike")),
        1
    );
}

#[test]
fn each_weapon_category_shapes_the_basic_attack() {
    let mut app = equip_game("equip_strikes");
    give(
        &mut app,
        &["gutter_knife", "hearthsplitter", "watch_pike", "bell_maul"],
    );
    let mut seen = Vec::new();
    for (id, category) in [
        ("iron_sword", ItemCategory::Sword),
        ("gutter_knife", ItemCategory::Dagger),
        ("hearthsplitter", ItemCategory::Axe),
        ("watch_pike", ItemCategory::Spear),
        ("bell_maul", ItemCategory::HeavyWeapon),
    ] {
        if id != "iron_sword" {
            equip(&mut app, EquipmentSlot::Weapon, id);
        }
        run(&mut app, 30);
        tap(&mut app, KeyCode::KeyX);
        let boxes = player_hitboxes(&mut app);
        let (hitbox, strike) = boxes
            .first()
            .cloned()
            .unwrap_or_else(|| panic!("{id}: no strike"));
        assert_eq!(strike.and_then(|s| s.category), Some(category), "{id}");
        // Power of the hitbox: ATK of the sheet (with this weapon).
        let profile = {
            let world = app.world_mut();
            world
                .query_filtered::<&CombatProfile, With<Player>>()
                .single(world)
                .map(|p| p.physical.power)
                .unwrap_or_else(|e| panic!("{e}"))
        };
        assert_eq!(hitbox.attack.power, profile, "{id}");
        // Committed time: count the frames in Attacking.
        let mut frames = 0;
        while player_state(&mut app) == PlayerState::Attacking && frames < 120 {
            run(&mut app, 1);
            frames += 1;
        }
        seen.push((id, hitbox.size, frames));
    }
    let size = |id: &str| {
        seen.iter()
            .find(|s| s.0 == id)
            .map(|s| s.1)
            .unwrap_or_else(|| panic!("{id}"))
    };
    let frames = |id: &str| {
        seen.iter()
            .find(|s| s.0 == id)
            .map(|s| s.2)
            .unwrap_or_else(|| panic!("{id}"))
    };
    assert_eq!(size("iron_sword"), Vec2::new(46.0, 26.0));
    assert!(size("watch_pike").x > size("iron_sword").x);
    assert!(size("gutter_knife").x < size("iron_sword").x);
    assert!(size("bell_maul").y > size("iron_sword").y);
    assert!(frames("gutter_knife") < frames("iron_sword"));
    assert!(frames("hearthsplitter") > frames("iron_sword"));
    assert!(frames("bell_maul") > frames("hearthsplitter"));
}

#[test]
fn crescent_slash_needs_a_sword() {
    let mut app = equip_game("equip_technique");
    let player = {
        let world = app.world_mut();
        world
            .query_filtered::<Entity, With<Player>>()
            .single(world)
            .unwrap_or_else(|e| panic!("{e}"))
    };
    let slash = |app: &mut App| {
        app.world_mut().write_message(SkillActivated {
            caster: player,
            skill_id: "crescent_slash".into(),
        });
        run(app, 1);
    };
    // With the starting sword: it starts.
    slash(&mut app);
    assert_eq!(player_state(&mut app), PlayerState::Casting);
    run(&mut app, 60);
    // With an axe: refused, nothing spent.
    give(&mut app, &["hearthsplitter"]);
    equip(&mut app, EquipmentSlot::Weapon, "hearthsplitter");
    let mp = vitals(&mut app).mp.current();
    run(&mut app, 120);
    slash(&mut app);
    assert_ne!(player_state(&mut app), PlayerState::Casting);
    assert!(
        app.world()
            .resource::<Outcomes>()
            .0
            .contains(&CommandOutcome::WrongWeapon {
                id: "crescent_slash".into()
            })
    );
    assert!(vitals(&mut app).mp.current() >= mp);
}

#[test]
fn equipment_cannot_change_mid_attack_fallen_spectral_or_in_a_passage() {
    let mut app = equip_game("equip_gate");
    give(&mut app, &["hearthsplitter"]);
    run(&mut app, 5);
    // Mid attack.
    tap(&mut app, KeyCode::KeyX);
    assert_eq!(player_state(&mut app), PlayerState::Attacking);
    equip(&mut app, EquipmentSlot::Weapon, "hearthsplitter");
    assert_eq!(
        belongings(&mut app).equipment.get(EquipmentSlot::Weapon),
        Some("iron_sword")
    );
    assert!(noted(&app, "not in the middle of an action"));
    run(&mut app, 40);
    // In a passage. The lock re-arms in the Rooms set once the player is
    // clear of the triggers, so the test holds it in the frame of the request.
    app.insert_resource(HoldLock(true));
    equip(&mut app, EquipmentSlot::Weapon, "hearthsplitter");
    app.insert_resource(HoldLock(false));
    run(&mut app, 2);
    assert!(noted(&app, "not during a passage"));
    // Spectral.
    give_relic(&mut app);
    tap(&mut app, KeyCode::KeyT);
    run(&mut app, 2);
    let phasing_before = {
        let world = app.world_mut();
        world
            .query_filtered::<&Phasing, With<Player>>()
            .single(world)
            .map(|p| p.passes.clone())
            .unwrap_or_else(|e| panic!("{e}"))
    };
    equip(&mut app, EquipmentSlot::Weapon, "hearthsplitter");
    assert!(noted(&app, "not while transformed"));
    let phasing_after = {
        let world = app.world_mut();
        world
            .query_filtered::<&Phasing, With<Player>>()
            .single(world)
            .map(|p| p.passes.clone())
            .unwrap_or_else(|e| panic!("{e}"))
    };
    assert_eq!(phasing_before, phasing_after);
    tap(&mut app, KeyCode::KeyT);
    run(&mut app, 3);
    // Fallen.
    {
        let world = app.world_mut();
        let mut q = world
            .query_filtered::<&mut dark_castle_rpg::player::components::ActionState, With<Player>>(
            );
        let mut s = q.single_mut(world).unwrap_or_else(|e| panic!("{e}"));
        s.die();
    }
    equip(&mut app, EquipmentSlot::Weapon, "hearthsplitter");
    assert!(noted(&app, "not while fallen"));
    let b = belongings(&mut app);
    assert_eq!(b.equipment.get(EquipmentSlot::Weapon), Some("iron_sword"));
    assert_eq!(b.inventory.count("hearthsplitter"), 1);
}

#[test]
fn max_mp_follows_the_band_without_free_restores() {
    let mut app = equip_game("equip_maxima");
    give(&mut app, &["veilwoven_band", "bloodstone_pendant"]);
    let before = vitals(&mut app);
    equip(&mut app, EquipmentSlot::Accessory1, "veilwoven_band");
    equip(&mut app, EquipmentSlot::Accessory2, "bloodstone_pendant");
    let with = vitals(&mut app);
    assert_eq!(with.mp.max(), before.mp.max() + 6);
    assert_eq!(with.hp.max(), before.hp.max() + 15);
    assert_eq!(with.mp.current(), before.mp.current(), "no free MP");
    assert_eq!(with.hp.current(), before.hp.current(), "no free HP");
    // Fill MP to the new max, remove the band: clamped to the old max.
    {
        let world = app.world_mut();
        let mut q = world.query_filtered::<&mut Vitals, With<Player>>();
        let mut v = q.single_mut(world).unwrap_or_else(|e| panic!("{e}"));
        v.mp.refill();
    }
    request(&mut app, EquipmentSlot::Accessory1, EquipOp::Unequip);
    let after = vitals(&mut app);
    assert_eq!(after.mp.max(), before.mp.max());
    assert_eq!(after.mp.current(), before.mp.max());
    // Regeneration of the band is part of the derived stats while worn.
    let regen_without = sheet(&mut app).derived().mp_regen;
    equip(&mut app, EquipmentSlot::Accessory1, "veilwoven_band");
    assert!((sheet(&mut app).derived().mp_regen - regen_without - 0.2).abs() < 1e-5);
}

fn cape_alpha(app: &mut App) -> Option<f32> {
    let world = app.world_mut();
    world
        .query_filtered::<&Sprite, With<CapeLayer>>()
        .iter(world)
        .next()
        .map(|s| s.color.alpha())
}

#[test]
fn the_cape_appears_turns_spectral_and_survives_save_and_load() {
    let saves = storage("equip_saveload");
    let mut app = game(&saves);
    give(&mut app, &["emberlined_cloak", "lantern_glaive"]);
    assert_eq!(cape_alpha(&mut app), None);
    equip(&mut app, EquipmentSlot::Body, "emberlined_cloak");
    equip(&mut app, EquipmentSlot::Weapon, "lantern_glaive");
    run(&mut app, 2);
    assert_eq!(cape_alpha(&mut app), Some(1.0));
    // Spectral: the cape is translucent like the body.
    give_relic(&mut app);
    tap(&mut app, KeyCode::KeyT);
    run(&mut app, 2);
    assert!(
        app.world_mut()
            .query_filtered::<&Transformation, With<Player>>()
            .single(app.world())
            .is_ok_and(|t| t.is_active())
    );
    let spectral = cape_alpha(&mut app).unwrap_or(1.0);
    assert!(spectral < 1.0, "cape alpha {spectral}");
    tap(&mut app, KeyCode::KeyT);
    run(&mut app, 3);
    assert_eq!(cape_alpha(&mut app), Some(1.0));
    let atk = sheet(&mut app).derived().attack;
    let saved = belongings(&mut app);
    tap(&mut app, KeyCode::F5);
    run(&mut app, 2);
    // Change everything, then another room.
    request(&mut app, EquipmentSlot::Body, EquipOp::Unequip);
    equip(&mut app, EquipmentSlot::Weapon, "iron_sword");
    app.world_mut().write_message(RoomRequest {
        room: CAUSEWAY.into(),
        arrival: Arrival::Door("from_approach".into()),
    });
    run(&mut app, 3);
    assert_eq!(current(&app), CAUSEWAY);
    assert_eq!(cape_alpha(&mut app), None);
    tap(&mut app, KeyCode::F9);
    run(&mut app, 3);
    assert_eq!(current(&app), APPROACH);
    let loaded = belongings(&mut app);
    assert_eq!(loaded.equipment, saved.equipment);
    assert_eq!(loaded.inventory, saved.inventory);
    assert_eq!(sheet(&mut app).derived().attack, atk);
    assert_eq!(cape_alpha(&mut app), Some(1.0));
    let look = {
        let world = app.world_mut();
        world
            .query_filtered::<&EquipmentAppearance, With<Player>>()
            .single(world)
            .map(|l| **l)
            .unwrap_or_else(|e| panic!("{e}"))
    };
    assert!(look.cape.is_some());
    assert_ne!(
        look.trail,
        dark_castle_rpg::inventory::appearance::DEFAULT_TRAIL
    );
}

#[test]
fn the_causeway_chest_gives_the_cloak_once_even_after_a_load() {
    let saves = storage("equip_chest_once");
    let mut app = game(&saves);
    app.world_mut().write_message(RoomRequest {
        room: CAUSEWAY.into(),
        arrival: Arrival::Door("from_approach".into()),
    });
    run(&mut app, 3);
    stand(&mut app, AT_CAUSEWAY_CHEST);
    tap(&mut app, USE);
    run(&mut app, 2);
    assert_eq!(belongings(&mut app).inventory.count("emberlined_cloak"), 1);
    tap(&mut app, KeyCode::F5);
    run(&mut app, 2);
    tap(&mut app, USE);
    run(&mut app, 2);
    tap(&mut app, KeyCode::F9);
    run(&mut app, 3);
    stand(&mut app, AT_CAUSEWAY_CHEST);
    tap(&mut app, USE);
    run(&mut app, 2);
    assert_eq!(belongings(&mut app).inventory.count("emberlined_cloak"), 1);
}

#[test]
fn equipment_timings_are_measured() {
    let mut app = equip_game("equip_perf");
    give(
        &mut app,
        &["hearthsplitter", "chain_hauberk", "emberlined_cloak"],
    );
    run(&mut app, 30);
    let plain = timed(&mut app, 60);
    let t = Instant::now();
    equip(&mut app, EquipmentSlot::Weapon, "hearthsplitter");
    let equip_ms = t.elapsed().as_secs_f64() * 1000.0;
    let t = Instant::now();
    request(&mut app, EquipmentSlot::Weapon, EquipOp::Unequip);
    let unequip_ms = t.elapsed().as_secs_f64() * 1000.0;
    equip(&mut app, EquipmentSlot::Body, "emberlined_cloak");
    run(&mut app, 2);
    let layers = count::<With<CapeLayer>>(&mut app);
    let t = Instant::now();
    tap(&mut app, I);
    run(&mut app, 1);
    let open_ms = t.elapsed().as_secs_f64() * 1000.0;
    let open_frame = timed(&mut app, 30);
    tap(&mut app, I);
    run(&mut app, 1);
    // Recompute of the stats alone (pure), 10 000 times.
    let data = app
        .world()
        .resource::<dark_castle_rpg::core::resources::GameData>()
        .clone();
    let b = belongings(&mut app);
    let s = sheet(&mut app);
    let t = Instant::now();
    for _ in 0..10_000 {
        let m = b.equipment.total_modifiers(&data.items);
        std::hint::black_box(data.character.formulas.derive(s.base(), 1, &m));
    }
    let recompute_us = t.elapsed().as_secs_f64() * 1e6 / 10_000.0;
    let report = format!(
        "plain frame {plain:.3} ms\nequip frame {equip_ms:.3} ms\nunequip frame {unequip_ms:.3} ms\n\
         recompute {recompute_us:.2} us\ninventory open frame {open_ms:.3} ms\n\
         inventory frame {open_frame:.3} ms\ncape layer entities {layers}\n"
    );
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("target/tmp");
    let _ = std::fs::create_dir_all(&dir);
    let _ = std::fs::write(dir.join("equipment_v20_perf.txt"), &report);
    println!("{report}");
    assert_eq!(layers, 1, "one cape child, nothing else");
    assert!(equip_ms < 100.0 && open_ms < 100.0);
}

fn timed(app: &mut App, frames: usize) -> f64 {
    let t = Instant::now();
    run(app, frames);
    t.elapsed().as_secs_f64() * 1000.0 / frames as f64
}
