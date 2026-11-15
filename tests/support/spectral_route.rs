//! The V1.9 route of the Spectral Form, as route-driver steps. From the
//! game's start, with real input only: the form refused without the relic,
//! the V1.7 route up to the Relic of the Veil, the floor veil of the Upper
//! Gallery, the Veiled Well, the side veil, back to human, the shrine's
//! chest, F5, a room change, F9 and the way back to the main route. Shared
//! by `tests/world_spectral_route_tests.rs` and
//! `examples/world_spectral_route.rs`. Docs: docs/TRANSFORMATIONS_V1_9.md.

use bevy::prelude::KeyCode;

use super::route_driver::{Dir, Expect, Step};
use super::vigil_route;

const FORM: KeyCode = KeyCode::KeyT;
const USE: KeyCode = KeyCode::KeyD;
/// Frames after a room change before a screenshot (room fade over).
const FADE_WAIT: u32 = 20;
/// Start MP 52 minus the activation (8).
const MP_AFTER_ACTIVATION: i32 = 44;

/// The V1.7 route up to the relic (inclusive), without its later part.
fn to_the_relic() -> Vec<Step> {
    let mut route = vigil_route::main_route();
    let relic = route
        .iter()
        .position(|s| *s == Step::Mark("08_relic"))
        .unwrap_or(route.len());
    route.truncate(relic + 1);
    route
}

pub fn spectral_route() -> Vec<Step> {
    use Dir::{Left, Still};
    use Step::*;
    let mut route = vec![
        Wait(30),
        // Without the relic, T is refused (the HUD says why).
        Tap(FORM),
        Wait(3),
        Check(Expect::FormActive(false)),
        Mark("s01_refused_without_relic"),
    ];
    route.extend(to_the_relic());
    route.extend([
        // The gallery floor over the hole is a veil: a human walks on it.
        WalkTo(1920.0),
        Check(Expect::StandingAt(63.0)),
        Mark("s02_human_on_the_floor_veil"),
        Tap(FORM),
        Wait(2),
        Check(Expect::FormActive(true)),
        Check(Expect::MpAtMost(MP_AFTER_ACTIVATION)),
        Mark("s03_spectral_form_activated"),
        Wait(6),
        Mark("s04_sinking_through_the_veil"),
        WalkUntilRoom(Still, "vc_veiled_well"),
        Check(Expect::FormActive(true)),
        Wait(FADE_WAIT),
        Mark("s05_arrived_in_the_veiled_well"),
        Wait(70),
        Check(Expect::MpAtMost(MP_AFTER_ACTIVATION - 4)),
        Mark("s06_mp_draining_on_the_ledge"),
        // Off the ledge to the left (away from the floor hole), then
        // through the side veil.
        WalkTo(220.0),
        Mark("s07_before_the_side_veil"),
        WalkTo(80.0),
        Mark("s08_crossing_the_side_veil"),
        WalkUntilRoom(Left, "vc_veiled_shrine"),
        Check(Expect::FormActive(true)),
        Wait(FADE_WAIT),
        Mark("s09_through_into_the_shrine"),
        Tap(FORM),
        Wait(3),
        Check(Expect::FormActive(false)),
        Mark("s10_back_to_human"),
        WalkTo(908.0),
        Tap(USE),
        Wait(5),
        Check(Expect::Pickup("vc_shrine_chest")),
        Check(Expect::HasItem("veilwoven_band", 1)),
        Mark("s11_reward_veilwoven_band"),
        Tap(KeyCode::F5),
        Wait(3),
        Mark("s12_saved_in_the_shrine"),
        // Another room: the drop through the shrine floor (human).
        WalkUntilRoom(Left, "vigil_gates_causeway"),
        Wait(FADE_WAIT),
        Mark("s13_dropped_into_the_causeway"),
        Tap(KeyCode::F9),
        Wait(3),
        Check(Expect::Room("vc_veiled_shrine")),
        Check(Expect::FormActive(false)),
        Check(Expect::Pickup("vc_shrine_chest")),
        Check(Expect::HasItem("veilwoven_band", 1)),
        Wait(FADE_WAIT),
        // The map (always shown) now has the whole cloister.
        Mark("s14_loaded_back_in_the_shrine_with_the_map"),
        // Back to the main route: drop to the causeway, on to the sealed
        // hall (the relic opens its grate).
        WalkUntilRoom(Left, "vigil_gates_causeway"),
        WalkUntilRoom(Left, "vigil_gates_approach"),
        WalkUntilRoom(Left, "vigil_gates_sealed_hall"),
        Check(Expect::FormActive(false)),
        Wait(FADE_WAIT),
        Mark("s15_back_on_the_main_route"),
    ]);
    route
}
