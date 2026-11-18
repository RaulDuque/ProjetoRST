//! D20.1 visual/systems route: altars, consumables, fast travel, enemy archetypes.
//! Shared by `examples/d20_1_probe.rs` (windowed screenshots). Input-only.

use bevy::prelude::KeyCode;
use dark_castle_rpg::inventory::equipment::EquipmentSlot;

use super::route_driver::{Dir, Expect, Step};

const USE: KeyCode = KeyCode::KeyD;
const INVENTORY: KeyCode = KeyCode::KeyI;
const ENTER: KeyCode = KeyCode::Enter;
const DOWN: KeyCode = KeyCode::ArrowDown;
const UP: KeyCode = KeyCode::ArrowUp;
const MAP: KeyCode = KeyCode::KeyM;
const FADE_WAIT: u32 = 20;

fn tap(key: KeyCode) -> [Step; 2] {
    [Step::Tap(key), Step::Wait(2)]
}

fn climb(route: &mut Vec<Step>, steps: &[(f32, f32)]) {
    for &(x, standing) in steps {
        route.extend([
            Step::WalkTo(x),
            Step::Jump(Dir::Still),
            Step::Check(Expect::StandingAt(standing)),
        ]);
    }
}

/// Reset the persistent inventory cursor, then land on `minor_tonic`
/// (8 equipment slots, then first bag consumable).
fn select_first_consumable(route: &mut Vec<Step>) {
    for _ in 0..16 {
        route.extend(tap(UP));
    }
    for _ in 0..EquipmentSlot::ALL.len() {
        route.extend(tap(DOWN));
    }
}

/// Windowed D20.1 probe through Vigil Gates (D17–D20 systems).
pub fn d20_1_route() -> Vec<Step> {
    use Dir::{Left, Right};
    use Step::*;

    let mut route = vec![
        Wait(30),
        Mark("01_start_approach"),
        Check(Expect::Room("vigil_gates_approach")),
    ];

    // D18: inventory + consumable row (no use yet — HP still full).
    route.extend(tap(INVENTORY));
    route.extend([Check(Expect::InventoryOpen(true))]);
    select_first_consumable(&mut route);
    route.extend([Wait(5), Mark("02_inventory_consumable")]);
    route.extend(tap(INVENTORY));
    route.push(Check(Expect::InventoryOpen(false)));

    // D17: first altar (no fast-travel menu — only one known).
    route.extend([
        WalkTo(360.0),
        Tap(USE),
        Wait(8),
        Check(Expect::LastSavePoint("vg_approach_save")),
        Check(Expect::FastTravelOpen(false)),
        Mark("03_altar_approach"),
    ]);

    // D20: Bone Sentry — step into melee so contact/attack lands, then tonic.
    route.extend([
        WalkUntilRoom(Right, "vigil_gates_causeway"),
        Wait(FADE_WAIT),
        WalkTo(1280.0),
        Wait(150),
        Check(Expect::EnemyAlive("bone_sentry")),
        Mark("04_bone_sentry"),
        Check(Expect::HpBelowMax),
    ]);
    route.extend(tap(INVENTORY));
    route.push(Check(Expect::InventoryOpen(true)));
    select_first_consumable(&mut route);
    route.extend([
        Wait(3),
        Mark("05_consumable_selected"),
        Tap(ENTER),
        Wait(6),
        Check(Expect::BagCount("minor_tonic", 2)),
        Mark("06_tonic_used"),
    ]);
    route.extend(tap(INVENTORY));
    route.push(Check(Expect::InventoryOpen(false)));
    route.extend([
        Fight,
        Mark("07_sentry_defeated"),
        WalkTo(1940.0),
        Tap(USE),
        Wait(8),
        Check(Expect::FastTravelOpen(true)),
        Check(Expect::LastSavePoint("vg_causeway_save")),
        Mark("08_fast_travel_menu"),
        Tap(ENTER),
        Wait(12),
        Check(Expect::Room("vigil_gates_approach")),
        Check(Expect::FastTravelOpen(false)),
        // Travel must not rewrite last_save_point (still causeway rest).
        Check(Expect::LastSavePoint("vg_causeway_save")),
        Mark("09_after_fast_travel"),
        Tap(MAP),
        Wait(4),
        Mark("10_map_altars"),
        Tap(MAP),
        Wait(2),
        WalkUntilRoom(Right, "vigil_gates_causeway"),
        Wait(FADE_WAIT),
        WalkUntilRoom(Right, "vigil_gates_shaft"),
        Wait(FADE_WAIT),
        Mark("11_shaft_bottom"),
    ]);

    // Climb to the Bone Bolt ledge (y ≈ 409).
    climb(
        &mut route,
        &[(660.0, 169.0), (660.0, 299.0), (650.0, 409.0)],
    );
    // Observe Bone Bolt on the ledge (do not fight — falling would reset the climb).
    route.extend([
        WalkTo(680.0),
        Wait(60),
        Check(Expect::EnemyAlive("bone_bolt")),
        Mark("12_bone_bolt"),
    ]);

    // Finish the shaft climb into the Upper Gallery for Ash Warden.
    climb(
        &mut route,
        &[
            (640.0, 519.0),
            (625.0, 629.0),
            (605.0, 739.0),
            (585.0, 849.0),
            (560.0, 959.0),
            (535.0, 1069.0),
            (475.0, 1179.0),
            (405.0, 1289.0),
            (345.0, 1409.0),
        ],
    );
    route.extend([
        Jump(Left),
        Check(Expect::StandingAt(1503.0)),
        WalkUntilRoom(Left, "vigil_gates_gallery"),
        Wait(FADE_WAIT),
        WalkTo(1500.0),
        Wait(50),
        Check(Expect::EnemyAlive("ash_warden")),
        Mark("13_ash_warden"),
        Fight,
        Mark("14_ash_warden_defeated"),
        Wait(10),
        Mark("15_done"),
    ]);
    route
}
