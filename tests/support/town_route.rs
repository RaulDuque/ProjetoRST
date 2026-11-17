//! The V2.1.1 route to the Village of Ashes, as route-driver steps. From
//! the game's start, with real input only: the V1.7 route up to the Watch
//! Shaft's step at y 849, off it onto the new step and the village door,
//! Bram's dialogue with a choice, the square on the map, Hessa's shop: buy
//! the one Watch Buckler (coins down, stock 1 -> 0), sell a tonic (coins
//! up), equip the buckler, F5 (save v3), another room, F9 (coins, stock,
//! bag and equipment restored, nothing duplicated), the buckler still sold
//! out, and the road back to the castle. Shared by
//! `tests/world_town_route_tests.rs` and `examples/world_town_route.rs`.
//! Docs: docs/NPC_COMMERCE_V2_1.md §23.

use bevy::prelude::KeyCode;
use dark_castle_rpg::inventory::equipment::EquipmentSlot;

use super::route_driver::{Dir, Expect, Step};
use super::vigil_route;

const TALK: KeyCode = KeyCode::KeyD;
const ENTER: KeyCode = KeyCode::Enter;
const DOWN: KeyCode = KeyCode::ArrowDown;
const RIGHT: KeyCode = KeyCode::ArrowRight;
const ESC: KeyCode = KeyCode::Escape;
const INVENTORY: KeyCode = KeyCode::KeyI;
const FADE_WAIT: u32 = 20;
/// Start coins (character.ron), knife price, tonic sell value.
const START_COINS: u32 = 120;
/// Watch Buckler: 90 coins, one unit; a Minor Tonic sells for 10.
const AFTER_BUY: u32 = 120 - 90;
const AFTER_SELL: u32 = 120 - 90 + 10;
const SHOP: &str = "hessa_outfitters";
const BUCKLER: &str = "watch_buckler";
const TONIC: &str = "minor_tonic";
/// Three at the start plus the hidden cache's bundle on the way.
const TONICS: u32 = 4;

fn tap(key: KeyCode) -> [Step; 2] {
    [Step::Tap(key), Step::Wait(2)]
}

fn without_marks(steps: &[Step]) -> Vec<Step> {
    steps
        .iter()
        .copied()
        .filter(|s| !matches!(s, Step::Mark(_)))
        .collect()
}

pub fn town_route() -> Vec<Step> {
    use Dir::{Left, Right};
    use Step::*;
    let main = vigil_route::main_route();
    // The V1.7 route up to standing on the shaft's step at y 849.
    let step = main
        .iter()
        .position(|s| *s == Check(Expect::StandingAt(849.0)))
        .unwrap_or(main.len());
    let start = main
        .iter()
        .position(|s| *s == Mark("01_start_approach"))
        .map_or(0, |i| i + 1);
    let mut route = vec![Wait(30), Mark("t01_start_in_the_castle")];
    route.extend(without_marks(&main[start..=step]));
    route.extend([
        // Off the step to the right: the new step, the ledge, the door.
        WalkUntilRoom(Right, "av_gate_road"),
        Wait(FADE_WAIT),
        Mark("t02_village_gate_road"),
        WalkTo(860.0),
        Mark("t03_talk_prompt_over_bram"),
    ]);
    route.extend(tap(TALK));
    route.extend([Check(Expect::Talking(true)), Mark("t04_bram_dialogue")]);
    route.extend(tap(ENTER));
    route.extend([Mark("t05_bram_choices")]);
    // First choice: ask about the castle (two lines), back to the intro.
    route.extend(tap(ENTER));
    route.extend([Check(Expect::Talking(true)), Mark("t06_bram_answers")]);
    route.extend(tap(ENTER));
    route.extend(tap(ENTER));
    route.extend(tap(ENTER));
    // Intro again: "Farewell".
    route.extend(tap(DOWN));
    route.extend(tap(ENTER));
    route.extend([
        Check(Expect::Talking(false)),
        WalkUntilRoom(Right, "av_square"),
        Wait(FADE_WAIT),
        Mark("t07_cinder_square"),
    ]);
    // The map panel is always shown (M hides it): the village is on it.
    route.push(Mark("t08_village_on_the_map"));
    route.extend([
        WalkUntilRoom(Right, "av_merchant_row"),
        WalkUntilRoom(Right, "av_outfitter"),
        Wait(FADE_WAIT),
        WalkTo(660.0),
    ]);
    route.extend(tap(TALK));
    route.extend([Check(Expect::Talking(true)), Mark("t09_hessa_greets")]);
    // First choice: the wares.
    route.extend(tap(ENTER));
    route.extend([
        Check(Expect::ShopOpen(true)),
        Check(Expect::Coins(START_COINS)),
        Check(Expect::StockLeft(SHOP, BUCKLER, 1)),
        Mark("t10_shop_buy_tab"),
    ]);
    // Row 4: the Watch Buckler (D18: minor_ether inserted above it).
    for _ in 0..4 {
        route.extend(tap(DOWN));
    }
    route.push(Mark("t11_buckler_compared"));
    route.extend(tap(ENTER));
    route.extend([
        Check(Expect::BagCount(BUCKLER, 1)),
        Check(Expect::Coins(AFTER_BUY)),
        Check(Expect::StockLeft(SHOP, BUCKLER, 0)),
        Mark("t12_buckler_bought_stock_zero"),
    ]);
    route.extend(tap(RIGHT));
    route.push(Mark("t13_shop_sell_tab"));
    // Row 0 of the bag: the tonics.
    route.extend(tap(ENTER));
    route.extend([
        Check(Expect::Coins(AFTER_SELL)),
        Check(Expect::BagCount(TONIC, TONICS - 1)),
        Mark("t14_tonic_sold"),
    ]);
    route.extend(tap(ESC));
    route.push(Check(Expect::ShopOpen(false)));
    // Equip the buckler: eight slot rows, then the buckler (first bag item
    // that is equipment).
    route.extend(tap(INVENTORY));
    for _ in 0..EquipmentSlot::ALL.len() {
        route.extend(tap(DOWN));
    }
    route.extend(tap(ENTER));
    route.extend([
        Check(Expect::Equipped(EquipmentSlot::Offhand, BUCKLER)),
        Check(Expect::BagCount(BUCKLER, 0)),
        Mark("t15_buckler_equipped"),
    ]);
    route.extend(tap(INVENTORY));
    route.extend([
        // Save v3: coins, bag, equipment and stock in one file.
        Tap(KeyCode::F5),
        Wait(3),
        Mark("t16_saved_after_trading"),
        // Another room, then the load.
        WalkUntilRoom(Left, "av_merchant_row"),
        Wait(FADE_WAIT),
        Mark("t17_another_room"),
        Tap(KeyCode::F9),
        Wait(3),
        Check(Expect::Room("av_outfitter")),
        Check(Expect::Coins(AFTER_SELL)),
        Check(Expect::StockLeft(SHOP, BUCKLER, 0)),
        Check(Expect::Equipped(EquipmentSlot::Offhand, BUCKLER)),
        Check(Expect::BagCount(BUCKLER, 0)),
        Check(Expect::BagCount(TONIC, TONICS - 1)),
        Wait(FADE_WAIT),
        Mark("t18_loaded_coins_stock_and_buckler"),
        WalkTo(660.0),
    ]);
    // The buckler stays sold out: buying it again changes nothing.
    route.extend(tap(TALK));
    route.extend(tap(ENTER));
    route.push(Check(Expect::ShopOpen(true)));
    for _ in 0..4 {
        route.extend(tap(DOWN));
    }
    route.extend(tap(ENTER));
    route.extend([
        Check(Expect::Coins(AFTER_SELL)),
        Check(Expect::BagCount(BUCKLER, 0)),
        Check(Expect::StockLeft(SHOP, BUCKLER, 0)),
        Mark("t19_sold_out_after_the_load"),
    ]);
    route.extend(tap(ESC));
    route.extend([
        WalkUntilRoom(Left, "av_merchant_row"),
        WalkUntilRoom(Left, "av_square"),
        WalkUntilRoom(Left, "av_gate_road"),
        WalkUntilRoom(Left, "vigil_gates_shaft"),
        Wait(FADE_WAIT),
        Mark("t20_returned_to_the_castle"),
    ]);
    route
}
