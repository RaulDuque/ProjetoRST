//! Application assembly: window, states, system ordering and plugins.

pub mod plugins;
pub mod states;

use bevy::prelude::*;

use crate::core::constants::{WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH};
use crate::core::resources::{GameData, project_root};

/// Loads the game data and runs the game. Data errors are reported before
/// the window opens and turn into a non-zero exit code.
pub fn run() -> AppExit {
    match GameData::load(&project_root()) {
        Ok(data) => build(data).run(),
        Err(error) => {
            eprintln!("failed to load game data: {error}");
            AppExit::error()
        }
    }
}

/// Builds the full game application around already-loaded data.
pub fn build(data: GameData) -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: WINDOW_TITLE.into(),
            resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
            ..default()
        }),
        ..default()
    }))
    .insert_resource(data)
    .add_plugins(plugins::GamePlugin);
    app
}
