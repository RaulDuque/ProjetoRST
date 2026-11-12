//! Magic: elements, resistances and spell execution. Spells are triggered
//! by the command recogniser but executed only here.

pub mod casting;
pub mod elements;
pub mod projectiles;

use bevy::prelude::*;

use crate::app::states::GameSet;

pub struct MagicPlugin;

impl Plugin for MagicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, casting::cast_spells.in_set(GameSet::Actions))
            .add_systems(
                Update,
                projectiles::move_projectiles
                    .in_set(GameSet::Physics)
                    .after(crate::world::physics::move_bodies),
            );
    }
}
