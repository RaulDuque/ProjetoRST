//! The V2.0 equipment route, as route-driver steps. From the game's start,
//! with real input only: the inventory with the starting gear, the
//! approach's chest (Watch Pike), the inventory screen (select, compare,
//! equip), the pike's longer strike, the causeway's sentry fought with it,
//! the causeway's chest (Emberlined Cloak) equipped and shown as a cape,
//! the V1.7 route up to the Relic of the Veil, the Spectral Form with the
//! equipment (and equipment locked while transformed), back to human, F5,
//! another room with a different weapon, F9 and the equipment and look
//! restored. Shared by `tests/world_equipment_route_tests.rs` and
//! `examples/world_equipment_route.rs`. Docs:
//! docs/EQUIPMENT_EXPANSION_V2_0.md §22.

use bevy::prelude::KeyCode;
use dark_castle_rpg::inventory::equipment::EquipmentSlot;

use super::route_driver::{Expect, Step};
use super::vigil_route;

const INVENTORY: KeyCode = KeyCode::KeyI;
const ENTER: KeyCode = KeyCode::Enter;
const DOWN: KeyCode = KeyCode::ArrowDown;
const UP: KeyCode = KeyCode::ArrowUp;
const USE: KeyCode = KeyCode::KeyD;
const FORM: KeyCode = KeyCode::KeyT;
const ATTACK: KeyCode = KeyCode::KeyX;
/// Frames after a room change before a screenshot (room fade over).
const FADE_WAIT: u32 = 20;
/// Approach chest (x 240..288) and causeway chest (x 1800..1848).
const APPROACH_CHEST_X: f32 = 264.0;
const CAUSEWAY_CHEST_X: f32 = 1824.0;
/// Width of the Watch Pike's strike (Spear class reach 78).
const PIKE_STRIKE: f32 = 78.0;
/// Start ATK: STR 6 x 2 = 12, + Iron Sword 8 = 20; the pike gives +9.
const ATK_WITH_PIKE: i32 = 21;
/// Start DEF: CON 6 + Leather Jerkin 4 = 10; the cloak gives 5.
const DEF_WITH_CLOAK: i32 = 11;

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

pub fn equipment_route() -> Vec<Step> {
    use Step::*;
    let main = vigil_route::main_route();
    let at = |name| {
        main.iter()
            .position(|s| *s == Mark(name))
            .unwrap_or(main.len())
    };
    // The V1.7 route, split after the sentry fight and cut after the relic.
    let start = at("01_start_approach") + 1;
    let fought = at("03_enemy_defeated");
    let relic = at("08_relic");
    let to_the_sentry = without_marks(&main[start..fought]);
    let to_the_relic = without_marks(&main[fought..relic]);

    let mut route = vec![Wait(30), Mark("e01_start_with_the_iron_sword")];
    route.extend(tap(INVENTORY));
    route.extend([
        Check(Expect::InventoryOpen(true)),
        Mark("e02_inventory_starting_gear"),
    ]);
    route.extend(tap(INVENTORY));
    route.extend([
        Check(Expect::InventoryOpen(false)),
        // The approach's chest, a step from the start.
        WalkTo(APPROACH_CHEST_X),
        Tap(USE),
        Wait(5),
        Check(Expect::Pickup("vg_approach_chest")),
        Check(Expect::HasItem("watch_pike", 1)),
        Mark("e03_chest_watch_pike"),
    ]);
    route.extend(tap(INVENTORY));
    // Eight slot rows, then the bag: the pike is the first bag row.
    for _ in 0..EquipmentSlot::ALL.len() {
        route.extend(tap(DOWN));
    }
    route.push(Mark("e04_compare_watch_pike"));
    route.extend(tap(ENTER));
    route.extend([
        Check(Expect::Equipped(EquipmentSlot::Weapon, "watch_pike")),
        Check(Expect::AttackAtLeast(ATK_WITH_PIKE)),
        Mark("e05_watch_pike_equipped"),
    ]);
    route.extend(tap(INVENTORY));
    route.extend([
        Check(Expect::InventoryOpen(false)),
        Tap(ATTACK),
        Wait(3),
        Check(Expect::StrikeWidth(PIKE_STRIKE)),
        Mark("e06_the_pike_strikes_farther"),
        Wait(30),
    ]);
    route.extend(to_the_sentry);
    route.extend([
        Mark("e07_sentry_defeated_with_the_pike"),
        WalkTo(CAUSEWAY_CHEST_X),
        Tap(USE),
        Wait(5),
        Check(Expect::Pickup("vg_causeway_chest")),
        Check(Expect::HasItem("emberlined_cloak", 1)),
    ]);
    route.extend(tap(INVENTORY));
    // Bag rows now: Iron Sword (row 8), Emberlined Cloak (row 9).
    route.extend(tap(DOWN));
    route.push(Mark("e08_compare_emberlined_cloak"));
    route.extend(tap(ENTER));
    route.extend([
        Check(Expect::Equipped(EquipmentSlot::Body, "emberlined_cloak")),
        Check(Expect::DefenseAtLeast(DEF_WITH_CLOAK)),
        Mark("e09_cloak_equipped"),
    ]);
    route.extend(tap(INVENTORY));
    route.extend([
        Check(Expect::Cape(true)),
        Wait(10),
        Mark("e10_the_cloak_is_worn"),
    ]);
    route.extend(to_the_relic);
    route.extend([
        Mark("e11_relic_with_pike_and_cloak"),
        Tap(FORM),
        Wait(3),
        Check(Expect::FormActive(true)),
        Wait(12),
        Mark("e12_spectral_form_with_the_equipment"),
    ]);
    // Equipment is locked while transformed: Enter changes nothing.
    route.extend(tap(INVENTORY));
    route.extend(tap(ENTER));
    route.extend([
        Check(Expect::Equipped(EquipmentSlot::Body, "emberlined_cloak")),
        Check(Expect::Equipped(EquipmentSlot::Weapon, "watch_pike")),
        Mark("e13_inventory_locked_while_spectral"),
    ]);
    route.extend(tap(INVENTORY));
    route.extend([
        Tap(FORM),
        Wait(3),
        Check(Expect::FormActive(false)),
        Wait(10),
        Mark("e14_human_again_with_the_cape"),
        Tap(KeyCode::F5),
        Wait(3),
        // Another room (the gallery's left door), then another weapon.
        Step::WalkUntilRoom(super::route_driver::Dir::Left, "vigil_gates_postern"),
        Wait(FADE_WAIT),
    ]);
    route.extend(tap(INVENTORY));
    // Bag rows: Iron Sword (8), Leather Jerkin (9); the cursor is on 9.
    route.extend(tap(UP));
    route.extend(tap(ENTER));
    route.extend([
        Check(Expect::Equipped(EquipmentSlot::Weapon, "iron_sword")),
        Mark("e15_postern_back_to_the_sword"),
    ]);
    route.extend(tap(INVENTORY));
    route.extend([
        Tap(KeyCode::F9),
        Wait(3),
        Check(Expect::Room("vigil_gates_gallery")),
        Check(Expect::FormActive(false)),
        Check(Expect::Equipped(EquipmentSlot::Weapon, "watch_pike")),
        Check(Expect::Equipped(EquipmentSlot::Body, "emberlined_cloak")),
        Check(Expect::AttackAtLeast(ATK_WITH_PIKE)),
        Wait(FADE_WAIT),
        Check(Expect::Cape(true)),
        Mark("e16_loaded_equipment_and_look_restored"),
    ]);
    route.extend(tap(INVENTORY));
    route.push(Mark("e17_inventory_after_the_load"));
    route.extend(tap(INVENTORY));
    route.extend([
        Tap(ATTACK),
        Wait(3),
        Check(Expect::StrikeWidth(PIKE_STRIKE)),
        Mark("e18_pike_strike_after_the_load"),
        Wait(30),
    ]);
    route
}
