//! The V1.8 route of the Forgotten Galleries, as route-driver steps. It
//! continues the V1.7 Vigil Gates route (the galleries open with the
//! Gatekeeper's Sigil), so the whole run starts at the game's start and uses
//! real input only. Shared by `tests/world_secrets_route_tests.rs` and
//! `examples/world_secrets_route.rs`. Docs: docs/WORLD_EXPLORATION_V1_8.md §17.

use bevy::prelude::KeyCode;
use dark_castle_rpg::world::graph::Ability;

use super::route_driver::{Dir, Expect, Step};
use super::vigil_route;

const USE: KeyCode = KeyCode::KeyD;
const ATTACK: KeyCode = KeyCode::KeyX;
/// Frames to wait after entering a room before a screenshot mark, so the
/// room fade (`ROOM_FADE_TIME`, 0.22 s) is over in the capture.
const FADE_WAIT: u32 = 20;

/// Interact (`D`) with the object the player stands on, then check `expect`.
fn interact(expect: Expect) -> [Step; 3] {
    [Step::Tap(USE), Step::Wait(3), Step::Check(expect)]
}

/// Stand at `x`, jump straight up, land at `standing` (one climb step).
fn climb(route: &mut Vec<Step>, steps: &[(f32, f32)]) {
    for &(x, standing) in steps {
        route.extend([
            Step::WalkTo(x),
            Step::Jump(Dir::Still),
            Step::Check(Expect::StandingAt(standing)),
        ]);
    }
}

/// Only the galleries part (starts in the Vigil sealed hall, where the V1.7
/// route ends, with the sigil taken).
pub fn galleries_route() -> Vec<Step> {
    use Dir::{Left, Right};
    use Step::*;
    let mut route = vec![
        WalkUntilRoom(Left, "vigil_gates_reliquary"),
        WalkUntilRoom(Left, "fg_threshold"),
        Wait(FADE_WAIT),
        Mark("15_galleries_threshold"),
        WalkUntilRoom(Left, "fg_long_gallery"),
        WalkTo(2224.0),
        Mark("16_chest_closed"),
    ];
    route.extend(interact(Expect::Pickup("fg_gallery_chest")));
    route.extend([
        Check(Expect::HasItem("minor_tonic", 1)),
        Mark("17_chest_opened_item_received"),
        Fight,
        WalkUntilRoom(Left, "fg_stair"),
        WalkTo(110.0),
        Mark("18_wall_intact"),
        Tap(ATTACK),
        Wait(20),
        Check(Expect::Secret("fg_cracked_wall")),
        Mark("19_wall_broken_passage_revealed"),
        WalkUntilRoom(Left, "fg_cracked_cache"),
        WalkTo(524.0),
    ]);
    route.extend(interact(Expect::Pickup("fg_cache_chest")));
    route.extend([
        Check(Expect::HasItem("ashen_ring", 1)),
        Mark("20_secret_chest"),
        WalkUntilRoom(Right, "fg_stair"),
    ]);
    // The Hollow Stair: to the ledge of the upper gallery door, then on to
    // the ledge of the chapel door.
    climb(
        &mut route,
        &[
            (380.0, 169.0),
            (430.0, 279.0),
            (530.0, 389.0),
            (630.0, 499.0),
            (730.0, 609.0),
            (830.0, 719.0),
        ],
    );
    route.extend([WalkTo(985.0), Jump(Right), Check(Expect::StandingAt(783.0))]);
    climb(
        &mut route,
        &[
            (1060.0, 889.0),
            (930.0, 999.0),
            (830.0, 1109.0),
            (830.0, 1219.0),
            (930.0, 1329.0),
            (930.0, 1439.0),
        ],
    );
    route.extend([
        WalkTo(985.0),
        Jump(Right),
        Check(Expect::StandingAt(1503.0)),
        Mark("21_stair_top"),
        WalkUntilRoom(Right, "fg_relic_chapel"),
        WalkTo(640.0),
        Wait(5),
        Check(Expect::Ability(Ability::AstralKey)),
        Mark("22_astral_emblem"),
        WalkUntilRoom(Right, "fg_collapsed_walk"),
        WalkTo(500.0),
        Jump(Dir::Still),
        Check(Expect::StandingAt(139.0)),
        WalkTo(1162.0),
        Mark("23_map_before_discovery"),
    ]);
    route.extend(interact(Expect::Secret("fg_loose_brick")));
    route.extend([
        Mark("24_map_after_discovery"),
        WalkUntilRoom(Right, "fg_hidden_oratory"),
        WalkTo(724.0),
    ]);
    route.extend(interact(Expect::Pickup("fg_oratory_chest")));
    route.extend([
        WalkUntilRoom(Left, "fg_collapsed_walk"),
        // The collapsed floor drops into the upper gallery: the loop.
        WalkUntilRoom(Left, "fg_upper_gallery"),
        Wait(FADE_WAIT),
        Mark("25_dropped_into_upper_gallery"),
        WalkUntilRoom(Right, "fg_lever_hall"),
        Wait(FADE_WAIT),
        Mark("26_lever_off_gate_closed"),
        WalkTo(420.0),
        Jump(Dir::Still),
        Check(Expect::StandingAt(169.0)),
    ]);
    route.extend(interact(Expect::Flag("fg_hall_gate_open")));
    route.extend([
        Mark("27_lever_on_gate_open"),
        WalkUntilRoom(Right, "fg_astral_gate"),
        Fight,
        Mark("28_astral_seal"),
        WalkUntilRoom(Right, "fg_return_bridge"),
        WalkTo(1028.0),
        Mark("29_rare_chest_closed"),
    ]);
    route.extend(interact(Expect::Pickup("fg_bridge_chest")));
    route.extend([
        Check(Expect::HasItem("warden_greaves", 1)),
        Mark("30_rare_chest_opened"),
        WalkTo(268.0),
        Mark("31_trapdoor_closed"),
    ]);
    route.extend(interact(Expect::Flag("fg_trapdoor_open")));
    route.extend([
        Mark("32_shortcut_open"),
        Tap(KeyCode::F5),
        Wait(3),
        // Through the trapdoor: back to the Vigil Gates.
        WalkUntilRoom(Right, "vigil_gates_sealed_hall"),
        Wait(FADE_WAIT),
        Mark("33_back_in_the_vigil_gates"),
        WalkUntilRoom(Right, "vigil_gates_approach"),
        Tap(KeyCode::F9),
        Wait(3),
        Check(Expect::Room("fg_return_bridge")),
        Check(Expect::Pickup("fg_bridge_chest")),
        Check(Expect::Flag("fg_trapdoor_open")),
        Check(Expect::Flag("fg_hall_gate_open")),
        Check(Expect::Secret("fg_cracked_wall")),
        Check(Expect::Secret("fg_loose_brick")),
        Check(Expect::Ability(Ability::AstralKey)),
        Check(Expect::HasItem("warden_greaves", 1)),
        Mark("34_loaded_on_the_bridge"),
        // The shortcut is still open after the load.
        WalkUntilRoom(Right, "vigil_gates_sealed_hall"),
        Wait(FADE_WAIT),
        Mark("35_shortcut_after_load"),
        // Back into the galleries: the broken wall stays broken.
        WalkUntilRoom(Left, "vigil_gates_reliquary"),
        WalkUntilRoom(Left, "fg_threshold"),
        WalkUntilRoom(Left, "fg_long_gallery"),
        Fight,
        WalkUntilRoom(Left, "fg_stair"),
        WalkUntilRoom(Left, "fg_cracked_cache"),
        Wait(FADE_WAIT),
        Mark("36_wall_still_broken"),
    ]);
    route
}

/// The whole V1.8 run: the V1.7 route, then the galleries.
pub fn full_route() -> Vec<Step> {
    let mut route = vigil_route::main_route();
    route.extend(galleries_route());
    route
}
