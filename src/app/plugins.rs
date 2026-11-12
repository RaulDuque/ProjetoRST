//! The root plugin that wires every feature plugin together.

use bevy::prelude::*;

use super::states::{GameSet, GameState};
use crate::combat::CombatPlugin;
use crate::core::CorePlugin;
use crate::enemies::EnemiesPlugin;
use crate::input::InputPlugin;
use crate::magic::MagicPlugin;
use crate::player::PlayerPlugin;
use crate::skills::SkillsPlugin;
use crate::ui::UiPlugin;
use crate::world::WorldPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .configure_sets(
                Update,
                (
                    GameSet::Input,
                    GameSet::Commands,
                    GameSet::Actions,
                    GameSet::Physics,
                    GameSet::Combat,
                    GameSet::Resolution,
                    GameSet::Progression,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            )
            .configure_sets(Update, GameSet::Presentation.after(GameSet::Progression))
            .add_plugins((
                CorePlugin,
                InputPlugin,
                WorldPlugin,
                PlayerPlugin,
                CombatPlugin,
                SkillsPlugin,
                MagicPlugin,
                EnemiesPlugin,
                UiPlugin,
            ));
    }
}
