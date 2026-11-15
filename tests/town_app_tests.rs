//! V2.1: NPCs, dialogue, shop and coins in the headless main game.
//!
//! Some scenes need an NPC where the shipped data has none (next to a chest,
//! in the start room): those tests add a fixture NPC to their own copy of
//! the data, never to `assets/`. Saves go under the target dir.

use std::path::{Path, PathBuf};
use std::time::Instant;

use bevy::input::{InputPlugin as BevyInputPlugin, InputSystems};
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::app::load_world;
use dark_castle_rpg::app::plugins::GamePlugin;
use dark_castle_rpg::app::states::GameState;
use dark_castle_rpg::combat::hitboxes::{Hitbox, Team};
use dark_castle_rpg::core::constants::DEFAULT_SAVE_SLOT;
use dark_castle_rpg::core::events::{Arrival, RoomRequest, TradeOp, TradeRequest};
use dark_castle_rpg::core::resources::GameData;
use dark_castle_rpg::economy::systems::PlayerPurse;
use dark_castle_rpg::npc::definitions::{DialogueNode, NpcDefinition, NpcLibrary};
use dark_castle_rpg::npc::systems::{NpcMarker, TalkPrompt};
use dark_castle_rpg::player::components::{Belongings, Player};
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::save::systems::SaveSlots;
use dark_castle_rpg::world::exploration_systems::{Exploration, activate_world};
use dark_castle_rpg::world::maps::CurrentRoom;
use dark_castle_rpg::world::physics::Velocity;

const FRAME: std::time::Duration = std::time::Duration::from_millis(16);
const D: KeyCode = KeyCode::KeyD;
const OUTFITTER: &str = "av_outfitter";
const SHOP: &str = "hessa_outfitters";

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

#[derive(Resource, Default)]
struct Pending(Vec<KeyCode>);

fn press(mut pending: ResMut<Pending>, mut keys: ResMut<ButtonInput<KeyCode>>) {
    for key in pending.0.drain(..) {
        keys.release(key);
        keys.press(key);
    }
}

fn storage(name: &str) -> SaveStorage {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = std::fs::remove_dir_all(&dir);
    SaveStorage::new(dir)
}

/// The game, with `extra` fixture NPCs (and their nodes) added to a copy of
/// the data.
fn game(name: &str, extra: Vec<(NpcDefinition, DialogueNode)>) -> App {
    let mut data = GameData::load(root()).unwrap_or_else(|e| panic!("{e}"));
    if !extra.is_empty() {
        let mut npcs: Vec<NpcDefinition> = data.npcs.npcs().cloned().collect();
        let mut nodes: Vec<DialogueNode> = Vec::new();
        for npc in data.npcs.npcs() {
            for g in &npc.greetings {
                collect(&data.npcs, &g.node, &mut nodes);
            }
        }
        for (npc, node) in extra {
            npcs.push(npc);
            nodes.push(node);
        }
        data.npcs = NpcLibrary::new(npcs, nodes).unwrap_or_else(|e| panic!("{e}"));
    }
    let world = load_world(root(), &mut data).unwrap_or_else(|e| panic!("{e}"));
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatesPlugin, BevyInputPlugin))
        .insert_resource(TimeUpdateStrategy::ManualDuration(FRAME))
        .insert_resource(data)
        .add_plugins(GamePlugin)
        .insert_resource(SaveSlots(storage(name)))
        .init_resource::<Pending>()
        .add_systems(PreUpdate, press.after(InputSystems));
    activate_world(&mut app, world);
    run(&mut app, 4);
    app
}

/// Every node reachable from `id` (to rebuild the library with fixtures).
fn collect(lib: &NpcLibrary, id: &str, out: &mut Vec<DialogueNode>) {
    if out.iter().any(|n| n.id == id) {
        return;
    }
    let Some(node) = lib.node(id) else {
        return;
    };
    out.push(node.clone());
    if let Some(next) = &node.next {
        collect(lib, next, out);
    }
    for choice in &node.choices {
        if let dark_castle_rpg::npc::definitions::ChoiceAction::Goto(t) = &choice.action {
            collect(lib, t, out);
        }
    }
}

fn fixture(id: &str, room: &str, x: f32) -> (NpcDefinition, DialogueNode) {
    let npc: NpcDefinition = ron::from_str(&format!(
        r#"(id: "{id}", name: "Fixture", kind: Villager, room: "{room}", position: ({x}, 63.0),
            size: (26.0, 46.0), color: (1.0, 1.0, 1.0), greetings: [(node: "{id}_hi")])"#
    ))
    .unwrap_or_else(|e| panic!("{e}"));
    let node: DialogueNode = ron::from_str(&format!(
        r#"(id: "{id}_hi", speaker: "{id}", lines: ["Hello there."])"#
    ))
    .unwrap_or_else(|e| panic!("{e}"));
    (npc, node)
}

fn run(app: &mut App, frames: usize) {
    for _ in 0..frames {
        app.update();
    }
}

fn tap(app: &mut App, key: KeyCode) {
    app.world_mut().resource_mut::<Pending>().0.push(key);
    run(app, 1);
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .release(key);
    run(app, 1);
}

fn state(app: &App) -> GameState {
    *app.world().resource::<State<GameState>>().get()
}

fn enter(app: &mut App, room: &str, door: &str) {
    app.world_mut().write_message(RoomRequest {
        room: room.into(),
        arrival: Arrival::Door(door.into()),
    });
    run(app, 3);
    assert_eq!(app.world().resource::<CurrentRoom>().0, room);
}

fn place(app: &mut App, x: f32) {
    let world = app.world_mut();
    let mut q = world.query_filtered::<(&mut Transform, &mut Velocity), With<Player>>();
    let (mut t, mut v) = q.single_mut(world).unwrap_or_else(|e| panic!("{e}"));
    t.translation.x = x;
    t.translation.y = 63.0;
    v.0 = Vec2::ZERO;
    run(app, 3);
}

fn coins(app: &mut App) -> u32 {
    let world = app.world_mut();
    world
        .query_filtered::<&PlayerPurse, With<Player>>()
        .single(world)
        .map(|p| p.coins())
        .unwrap_or_else(|e| panic!("{e}"))
}

fn bag_count(app: &mut App, item: &str) -> u32 {
    let world = app.world_mut();
    world
        .query_filtered::<&Belongings, With<Player>>()
        .single(world)
        .map(|b| b.inventory.count(item))
        .unwrap_or_else(|e| panic!("{e}"))
}

fn trade(app: &mut App, op: TradeOp) {
    app.world_mut().write_message(TradeRequest {
        shop: SHOP.into(),
        op,
    });
    run(app, 1);
}

fn npc_count(app: &mut App) -> usize {
    let world = app.world_mut();
    world.query::<&NpcMarker>().iter(world).count()
}

#[test]
fn only_the_current_rooms_npcs_exist_and_d_needs_reach() {
    let mut app = game("town_reach", Vec::new());
    assert_eq!(npc_count(&mut app), 0, "no NPC in the castle's start room");
    enter(&mut app, OUTFITTER, "to_merchant_row");
    assert_eq!(npc_count(&mut app), 1);
    // Far away: D does nothing.
    place(&mut app, 200.0);
    tap(&mut app, D);
    assert_eq!(state(&app), GameState::Playing);
    // In reach: the prompt shows and D talks.
    place(&mut app, 660.0);
    let shown = {
        let world = app.world_mut();
        world
            .query_filtered::<&Visibility, With<TalkPrompt>>()
            .iter(world)
            .any(|v| *v != Visibility::Hidden)
    };
    assert!(shown);
    tap(&mut app, D);
    assert_eq!(state(&app), GameState::Dialogue);
    tap(&mut app, KeyCode::Escape);
    assert_eq!(state(&app), GameState::Playing);
    // Another room: Hessa is gone with her room.
    enter(&mut app, "av_smithy", "to_outfitter");
    assert_eq!(npc_count(&mut app), 1);
    let ids: Vec<String> = {
        let world = app.world_mut();
        world
            .query::<&NpcMarker>()
            .iter(world)
            .map(|n| n.id.clone())
            .collect()
    };
    assert_eq!(ids, vec!["garrick_smith".to_owned()]);
}

#[test]
fn one_press_opens_the_chest_and_never_also_talks() {
    // A fixture NPC standing on the approach's chest (x 240..288).
    let mut app = game(
        "town_chest_and_npc",
        vec![fixture("fixture_on_chest", "vigil_gates_approach", 270.0)],
    );
    place(&mut app, 264.0);
    tap(&mut app, D);
    assert_eq!(bag_count(&mut app, "watch_pike"), 1, "the chest wins");
    assert_eq!(state(&app), GameState::Playing, "and no conversation");
    // The chest is open now: the next press talks.
    tap(&mut app, D);
    assert_eq!(state(&app), GameState::Dialogue);
}

#[test]
fn a_load_during_a_conversation_closes_it() {
    let mut app = game(
        "town_load_in_dialogue",
        vec![fixture("fixture_start", "vigil_gates_approach", 120.0)],
    );
    place(&mut app, 110.0);
    tap(&mut app, KeyCode::F5);
    tap(&mut app, D);
    assert_eq!(state(&app), GameState::Dialogue);
    tap(&mut app, KeyCode::F9);
    run(&mut app, 3);
    assert_eq!(state(&app), GameState::Playing);
    assert!(
        app.world()
            .get_resource::<dark_castle_rpg::npc::systems::ActiveConversation>()
            .is_none()
    );
}

#[test]
fn the_shop_pauses_combat_and_trades_are_atomic() {
    let mut app = game("town_shop", Vec::new());
    enter(&mut app, OUTFITTER, "to_merchant_row");
    place(&mut app, 660.0);
    tap(&mut app, D);
    tap(&mut app, KeyCode::Enter);
    assert_eq!(state(&app), GameState::Shop);
    // X does not attack while the shop is open.
    tap(&mut app, KeyCode::KeyX);
    let strikes = {
        let world = app.world_mut();
        world
            .query::<&Hitbox>()
            .iter(world)
            .filter(|h| h.team == Team::Player)
            .count()
    };
    assert_eq!(strikes, 0);
    assert_eq!(coins(&mut app), 120);
    // Two buys of the last unit in the same frame: one succeeds.
    {
        let world = app.world_mut();
        for _ in 0..2 {
            world.write_message(TradeRequest {
                shop: SHOP.into(),
                op: TradeOp::Buy("quiet_soles".into()),
            });
        }
    }
    run(&mut app, 1);
    assert_eq!(bag_count(&mut app, "quiet_soles"), 0, "260 > 120 coins");
    trade(&mut app, TradeOp::Buy("gutter_knife".into()));
    trade(&mut app, TradeOp::Buy("gutter_knife".into()));
    assert_eq!(
        bag_count(&mut app, "gutter_knife"),
        1,
        "40 coins left after one"
    );
    assert_eq!(coins(&mut app), 40);
    // Two sells of the same unit in the same frame: one succeeds.
    {
        let world = app.world_mut();
        for _ in 0..2 {
            world.write_message(TradeRequest {
                shop: SHOP.into(),
                op: TradeOp::Sell("gutter_knife".into()),
            });
        }
    }
    run(&mut app, 1);
    assert_eq!(bag_count(&mut app, "gutter_knife"), 0);
    assert_eq!(coins(&mut app), 75);
    // Equipped items are not sold.
    trade(&mut app, TradeOp::Sell("iron_sword".into()));
    assert_eq!(coins(&mut app), 75);
    tap(&mut app, KeyCode::Escape);
    assert_eq!(state(&app), GameState::Playing);
}

#[test]
fn trades_are_saved_with_the_bag_and_restored_by_a_load() {
    // V2.1.1 (save v3): F5 after trading is allowed and keeps coins, bag
    // and stock together; F9 restores exactly that, never more.
    let mut app = game("town_economy_save", Vec::new());
    enter(&mut app, OUTFITTER, "to_merchant_row");
    trade(&mut app, TradeOp::Buy("gutter_knife".into()));
    trade(&mut app, TradeOp::Sell("minor_tonic".into()));
    assert_eq!(coins(&mut app), 120 - 80 + 10);
    tap(&mut app, KeyCode::F5);
    let saved = app
        .world()
        .resource::<SaveSlots>()
        .0
        .load(DEFAULT_SAVE_SLOT)
        .unwrap_or_else(|e| panic!("{e}"));
    let economy = saved
        .economy
        .clone()
        .unwrap_or_else(|| panic!("no economy"));
    assert_eq!(economy.coins, 50);
    assert_eq!(saved.player.inventory.count("gutter_knife"), 1);
    // Spend more after the save, change room, then load: the save wins.
    trade(&mut app, TradeOp::Buy("minor_tonic".into()));
    trade(&mut app, TradeOp::Buy("minor_tonic".into()));
    assert_eq!(coins(&mut app), 0);
    enter(&mut app, "av_smithy", "to_outfitter");
    tap(&mut app, KeyCode::F9);
    run(&mut app, 3);
    assert_eq!(app.world().resource::<CurrentRoom>().0, OUTFITTER);
    assert_eq!(coins(&mut app), 50);
    assert_eq!(bag_count(&mut app, "gutter_knife"), 1);
    assert_eq!(
        bag_count(&mut app, "minor_tonic"),
        saved.player.inventory.count("minor_tonic")
    );
    // F9 twice gives the same state (nothing is granted per load).
    tap(&mut app, KeyCode::F9);
    run(&mut app, 3);
    assert_eq!(coins(&mut app), 50);
    assert_eq!(bag_count(&mut app, "gutter_knife"), 1);
    // Loading never wrote the file.
    let again = app
        .world()
        .resource::<SaveSlots>()
        .0
        .load(DEFAULT_SAVE_SLOT)
        .unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(again, saved);
    let visited = app
        .world()
        .resource::<Exploration>()
        .0
        .rooms
        .contains_key(OUTFITTER);
    assert!(visited);
}

#[test]
fn a_sold_out_entry_stays_sold_out_after_f5_and_f9() {
    let mut app = game("town_stock_save", Vec::new());
    enter(&mut app, OUTFITTER, "to_merchant_row");
    // The last unit, twice in the same frame: one sale.
    {
        let world = app.world_mut();
        for _ in 0..2 {
            world.write_message(TradeRequest {
                shop: SHOP.into(),
                op: TradeOp::Buy("watch_buckler".into()),
            });
        }
    }
    run(&mut app, 1);
    assert_eq!(bag_count(&mut app, "watch_buckler"), 1);
    assert_eq!(coins(&mut app), 30);
    tap(&mut app, KeyCode::F5);
    let saved = app
        .world()
        .resource::<SaveSlots>()
        .0
        .load(DEFAULT_SAVE_SLOT)
        .unwrap_or_else(|e| panic!("{e}"));
    assert!(saved.economy.as_ref().is_some_and(|e| e.coins == 30));
    // Sell it back after the save, then load: sold out again, 30 coins,
    // one buckler (the save's), and a new buy is refused.
    trade(&mut app, TradeOp::Sell("watch_buckler".into()));
    assert_eq!(bag_count(&mut app, "watch_buckler"), 0);
    tap(&mut app, KeyCode::F9);
    run(&mut app, 3);
    assert_eq!(coins(&mut app), 30);
    assert_eq!(bag_count(&mut app, "watch_buckler"), 1);
    trade(&mut app, TradeOp::Buy("watch_buckler".into()));
    assert_eq!(
        bag_count(&mut app, "watch_buckler"),
        1,
        "sold out after the load"
    );
    assert_eq!(coins(&mut app), 30);
    // Not enough coins for anything above 30.
    trade(&mut app, TradeOp::Buy("sentry_coif".into()));
    assert_eq!(bag_count(&mut app, "sentry_coif"), 0);
    assert_eq!(coins(&mut app), 30);
}

#[test]
fn town_timings_are_measured() {
    let mut app = game("town_perf", Vec::new());
    let t = Instant::now();
    enter(&mut app, OUTFITTER, "to_merchant_row");
    let enter_ms = t.elapsed().as_secs_f64() * 1000.0;
    let entities = app.world().entities().len();
    place(&mut app, 660.0);
    let plain = timed(&mut app, 60);
    let t = Instant::now();
    tap(&mut app, D);
    let dialogue_ms = t.elapsed().as_secs_f64() * 1000.0;
    let t = Instant::now();
    tap(&mut app, KeyCode::Enter);
    let shop_ms = t.elapsed().as_secs_f64() * 1000.0;
    let shop_frame = timed(&mut app, 30);
    let t = Instant::now();
    trade(&mut app, TradeOp::Buy("minor_tonic".into()));
    let buy_ms = t.elapsed().as_secs_f64() * 1000.0;
    let t = Instant::now();
    trade(&mut app, TradeOp::Sell("minor_tonic".into()));
    let sell_ms = t.elapsed().as_secs_f64() * 1000.0;
    tap(&mut app, KeyCode::Escape);
    let t = Instant::now();
    tap(&mut app, KeyCode::F5);
    let save_ms = t.elapsed().as_secs_f64() * 1000.0;
    let t = Instant::now();
    tap(&mut app, KeyCode::F9);
    let load_ms = t.elapsed().as_secs_f64() * 1000.0;
    let report = format!(
        "room change with NPC {enter_ms:.3} ms\nentities in the outfitter {entities}\n\
         plain frame {plain:.3} ms\nopen dialogue (2 frames) {dialogue_ms:.3} ms\n\
         open shop (2 frames) {shop_ms:.3} ms\nshop frame {shop_frame:.3} ms\n\
         buy {buy_ms:.3} ms\nsell {sell_ms:.3} ms\n\
         F5 v3 (2 frames) {save_ms:.3} ms\nF9 v3 (2 frames) {load_ms:.3} ms\n"
    );
    let dir = root().join("target/tmp");
    let _ = std::fs::create_dir_all(&dir);
    let _ = std::fs::write(dir.join("town_v21_perf.txt"), &report);
    println!("{report}");
    assert!(dialogue_ms < 200.0 && shop_ms < 200.0);
}

fn timed(app: &mut App, frames: usize) -> f64 {
    let t = Instant::now();
    run(app, frames);
    t.elapsed().as_secs_f64() * 1000.0 / frames as f64
}
