//! Entry point. All game logic lives in the library crate.

use bevy::prelude::AppExit;

fn main() -> AppExit {
    dark_castle_rpg::app::run()
}
