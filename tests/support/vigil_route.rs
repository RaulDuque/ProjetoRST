//! The V1.7 main route of the Vigil Gates, as route-driver steps. Shared
//! by `tests/world_route_tests.rs` and `examples/world_main_route.rs`.

use bevy::prelude::KeyCode;
use dark_castle_rpg::world::graph::Ability;

use super::route_driver::{Dir, Expect, Step};

/// The route of docs/WORLD_INTEGRATION_V1_7.md §15. Every room change is a
/// door (or the F9 load); every height is reached by jumping.
pub fn main_route() -> Vec<Step> {
    use Dir::{Left, Right, Still};
    use Step::*;
    let mut route = vec![
        // Let the window and the sprites come up before the first capture.
        Wait(30),
        Mark("01_start_approach"),
        Check(Expect::Room("vigil_gates_approach")),
        WalkUntilRoom(Right, "vigil_gates_causeway"),
        Mark("02_causeway"),
        WalkTo(1000.0),
        Fight,
        Mark("03_enemy_defeated"),
        WalkUntilRoom(Right, "vigil_gates_shaft"),
        Mark("04_shaft_bottom"),
        WalkUntilRoom(Right, "vigil_gates_barbican"),
        WalkUntilRoom(Right, "vigil_gates_hidden_cache"),
        Check(Expect::Secret("barbican_false_wall")),
        Mark("05_hidden_cache"),
        WalkTo(900.0),
        Wait(5),
        Check(Expect::Pickup("cache_tonic_bundle")),
        WalkUntilRoom(Left, "vigil_gates_barbican"),
        Mark("06_barbican_wall_found"),
        WalkUntilRoom(Left, "vigil_gates_shaft"),
    ];
    // The Watch Shaft: stand where two steps overlap, jump straight up.
    for (x, standing) in [
        (660.0, 169.0),
        (660.0, 299.0),
        (650.0, 409.0),
        (640.0, 519.0),
        (625.0, 629.0),
        (605.0, 739.0),
        (585.0, 849.0),
        (560.0, 959.0),
        (535.0, 1069.0),
        (475.0, 1179.0),
        (405.0, 1289.0),
        (345.0, 1409.0),
    ] {
        route.extend([WalkTo(x), Jump(Still), Check(Expect::StandingAt(standing))]);
    }
    route.extend([
        Mark("07_shaft_top"),
        Jump(Left),
        Check(Expect::StandingAt(1503.0)),
        WalkUntilRoom(Left, "vigil_gates_gallery"),
        WalkTo(500.0),
        Wait(5),
        Check(Expect::Ability(Ability::VeilStep)),
        Mark("08_relic"),
        WalkUntilRoom(Left, "vigil_gates_postern"),
        Mark("09_postern"),
        WalkUntilRoom(Left, "vigil_gates_approach"),
        Wait(60),
        Check(Expect::StandingAt(63.0)),
        Mark("10_grate_open"),
        Tap(KeyCode::KeyM),
        Wait(2),
        Tap(KeyCode::KeyM),
        Wait(2),
        Tap(KeyCode::F5),
        Wait(3),
        WalkUntilRoom(Left, "vigil_gates_sealed_hall"),
        Mark("11_sealed_hall_before_load"),
        Tap(KeyCode::F9),
        Wait(3),
        Check(Expect::Room("vigil_gates_approach")),
        Check(Expect::Ability(Ability::VeilStep)),
        Check(Expect::Secret("barbican_false_wall")),
        Mark("12_loaded_back"),
        WalkUntilRoom(Left, "vigil_gates_sealed_hall"),
        WalkUntilRoom(Left, "vigil_gates_reliquary"),
        WalkTo(400.0),
        Wait(5),
        Check(Expect::Pickup("gatekeeper_sigil")),
        Mark("13_reliquary"),
        WalkUntilRoom(Right, "vigil_gates_sealed_hall"),
        Mark("14_returned"),
    ]);
    route
}
