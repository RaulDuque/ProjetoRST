//! D18: using consumables from the inventory screen (headless).

use bevy::prelude::*;
use dark_castle_rpg::app::states::GameState;
use dark_castle_rpg::core::constants::DEFAULT_SAVE_SLOT;
use dark_castle_rpg::core::events::{ItemUsed, Notification, UseItemRequest};
use dark_castle_rpg::core::resources::GameData;
use dark_castle_rpg::player::components::{Belongings, Player, Vitals};
use dark_castle_rpg::ui::inventory_screen::{self, InventoryCursor, Row};

#[path = "support/world_app.rs"]
mod world_app;

use world_app::*;

const I: KeyCode = KeyCode::KeyI;
const ENTER: KeyCode = KeyCode::Enter;
const DOWN: KeyCode = KeyCode::ArrowDown;

#[derive(Resource, Default)]
struct Notes(Vec<String>);

#[derive(Resource, Default)]
struct Used(Vec<ItemUsed>);

fn record(
    mut notes_msg: MessageReader<Notification>,
    mut used_msg: MessageReader<ItemUsed>,
    mut notes: ResMut<Notes>,
    mut used: ResMut<Used>,
) {
    notes.0.extend(notes_msg.read().map(|n| n.0.clone()));
    used.0.extend(used_msg.read().cloned());
}

fn bag_count(app: &mut App, id: &str) -> u32 {
    let world = app.world_mut();
    world
        .query_filtered::<&Belongings, With<Player>>()
        .single(world)
        .map(|b| b.inventory.count(id))
        .unwrap_or_else(|e| panic!("{e}"))
}

fn hp(app: &mut App) -> (i32, i32) {
    let world = app.world_mut();
    world
        .query_filtered::<&Vitals, With<Player>>()
        .single(world)
        .map(|v| (v.hp.current(), v.hp.max()))
        .unwrap_or_else(|e| panic!("{e}"))
}

fn mp(app: &mut App) -> (i32, i32) {
    let world = app.world_mut();
    world
        .query_filtered::<&Vitals, With<Player>>()
        .single(world)
        .map(|v| (v.mp.current(), v.mp.max()))
        .unwrap_or_else(|e| panic!("{e}"))
}

fn app_named(name: &str) -> App {
    let mut app = game(&storage(name));
    app.init_resource::<Notes>()
        .init_resource::<Used>()
        .add_systems(PostUpdate, record);
    app
}

fn select_bag_item(app: &mut App, item: &str) {
    tap(app, I);
    run(app, 2);
    assert_eq!(
        app.world().resource::<State<GameState>>().get(),
        &GameState::Inventory
    );
    for _ in 0..32 {
        let cursor = app.world().resource::<InventoryCursor>().0;
        let belongings = {
            let world = app.world_mut();
            world
                .query_filtered::<&Belongings, With<Player>>()
                .single(world)
                .cloned()
                .unwrap_or_else(|e| panic!("{e}"))
        };
        let catalog = &app.world().resource::<GameData>().items;
        let rows = inventory_screen::rows(&belongings, catalog);
        if let Some(Row::Bag(id)) = rows.get(cursor)
            && id == item
        {
            return;
        }
        tap(app, DOWN);
        run(app, 1);
    }
    panic!("could not select {item} in inventory");
}

#[test]
fn inventory_enter_uses_tonic_and_persists() {
    let saves = storage("d18_use_tonic");
    let mut app = game(&saves);
    app.init_resource::<Notes>()
        .init_resource::<Used>()
        .add_systems(PostUpdate, record);

    let before = bag_count(&mut app, "minor_tonic");
    assert!(before >= 1);
    {
        let mut q = app
            .world_mut()
            .query_filtered::<&mut Vitals, With<Player>>();
        let mut vitals = q
            .single_mut(app.world_mut())
            .unwrap_or_else(|e| panic!("{e}"));
        let drop = vitals.hp.current().saturating_sub(10);
        vitals.hp.deplete(drop);
    }
    let (hurt, max) = hp(&mut app);
    assert!(hurt < max);

    select_bag_item(&mut app, "minor_tonic");
    tap(&mut app, ENTER);
    run(&mut app, 3);

    assert_eq!(bag_count(&mut app, "minor_tonic"), before - 1);
    let (now, _) = hp(&mut app);
    assert!(now > hurt);
    assert!(
        app.world()
            .resource::<Used>()
            .0
            .iter()
            .any(|u| u.item == "minor_tonic" && u.hp_restored > 0)
    );

    {
        let mut q = app
            .world_mut()
            .query_filtered::<&mut Vitals, With<Player>>();
        let mut vitals = q
            .single_mut(app.world_mut())
            .unwrap_or_else(|e| panic!("{e}"));
        vitals.hp.deplete(20);
    }
    app.world_mut().write_message(UseItemRequest {
        item: "minor_tonic".into(),
    });
    run(&mut app, 2);
    let after_use = bag_count(&mut app, "minor_tonic");
    let (hp_after, _) = hp(&mut app);

    tap(&mut app, KeyCode::F5);
    run(&mut app, 2);
    tap(&mut app, KeyCode::F9);
    run(&mut app, 5);
    assert_eq!(bag_count(&mut app, "minor_tonic"), after_use);
    assert_eq!(hp(&mut app).0, hp_after);
    let loaded = saves
        .load(DEFAULT_SAVE_SLOT)
        .unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(loaded.player.inventory.count("minor_tonic"), after_use);
    assert_eq!(loaded.player.hp, hp_after);
}

#[test]
fn full_hp_refuses_tonic_without_consuming() {
    let mut app = app_named("d18_full_hp");
    let before = bag_count(&mut app, "minor_tonic");
    tap(&mut app, I);
    run(&mut app, 2);
    app.world_mut().write_message(UseItemRequest {
        item: "minor_tonic".into(),
    });
    run(&mut app, 3);
    assert_eq!(bag_count(&mut app, "minor_tonic"), before);
    assert!(app.world().resource::<Used>().0.is_empty());
    assert!(
        app.world()
            .resource::<Notes>()
            .0
            .iter()
            .any(|n| n.contains("full"))
    );
}

#[test]
fn ether_restores_mp() {
    let mut app = app_named("d18_ether");
    assert!(bag_count(&mut app, "minor_ether") >= 1);
    {
        let mut q = app
            .world_mut()
            .query_filtered::<&mut Vitals, With<Player>>();
        let mut vitals = q
            .single_mut(app.world_mut())
            .unwrap_or_else(|e| panic!("{e}"));
        let drop = vitals.mp.current().saturating_sub(5);
        vitals.mp.deplete(drop);
    }
    let (hurt, _) = mp(&mut app);
    tap(&mut app, I);
    run(&mut app, 2);
    app.world_mut().write_message(UseItemRequest {
        item: "minor_ether".into(),
    });
    run(&mut app, 2);
    assert_eq!(bag_count(&mut app, "minor_ether"), 0);
    assert!(mp(&mut app).0 > hurt);
}
