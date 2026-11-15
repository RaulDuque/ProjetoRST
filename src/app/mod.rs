//! Application assembly: window, states, system ordering and plugins.
//!
//! Since V1.7 `run` starts the game in the world graph (`assets/world/`):
//! the world is loaded, validated (errors refuse to start), its rooms are
//! installed in `GameData` and it is activated. `build` alone (no world)
//! still runs the single room of `assets/maps/`, used as a test fixture by
//! the headless tests, `autoplay` and `visual_check`.

pub mod plugins;
pub mod states;

use std::path::Path;

use bevy::prelude::*;

use crate::core::constants::{WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH};
use crate::core::resources::{GameData, project_root};
use crate::transform::crossing::crossing_mp;
use crate::world::exploration_systems::{activate_world, install_world};
use crate::world::graph::{Grant, WorldDefinition};
use crate::world::validation::validate;

/// Directory of the world graph, relative to the project root.
pub const WORLD_DIR: &str = "assets/world";

/// Loads the game data and the world and runs the game. Data errors are
/// reported before the window opens and turn into a non-zero exit code.
pub fn run() -> AppExit {
    let root = project_root();
    let loaded = GameData::load(&root)
        .map_err(|e| format!("failed to load game data: {e}"))
        .and_then(|mut data| load_world(&root, &mut data).map(|world| (data, world)));
    match loaded {
        Ok((data, world)) => build_with_world(data, world).run(),
        Err(error) => {
            eprintln!("{error}");
            AppExit::error()
        }
    }
}

/// Loads `assets/world/`, refuses it if the validator reports errors or if
/// it names enemies or items `GameData` does not have, and installs its
/// rooms in `data` (the start room becomes the game's start).
///
/// V1.9: the world's barrier `passages` must match the transformation
/// definitions, and every veil must be crossable by its form with the MP of
/// the starting character (an effective crossing, not just the relic).
pub fn load_world(root: &Path, data: &mut GameData) -> Result<WorldDefinition, String> {
    let world = WorldDefinition::load_dir(&root.join(WORLD_DIR))
        .map_err(|e| format!("failed to load the world: {e}"))?;
    let report = validate(&world);
    if !report.is_ok() {
        return Err(format!("invalid world: {:?}", report.errors));
    }
    check_transformations(&world, data)?;
    for room in world.rooms.values() {
        if let Some(spawn) = room
            .enemies
            .iter()
            .find(|s| !data.enemies.contains_key(&s.enemy))
        {
            return Err(format!(
                "room `{}` spawns unknown enemy `{}`",
                room.id, spawn.enemy
            ));
        }
        let rewards = room
            .pickups
            .iter()
            .map(|p| (p.id.as_str(), Some(p.grants.clone())))
            .chain(room.objects.iter().map(|o| (o.id.as_str(), o.grant())));
        for (id, grant) in rewards {
            if let Some(Grant::Item(item)) = &grant
                && data.items.get(item).is_none()
            {
                return Err(format!("`{id}` grants unknown item `{item}`"));
            }
        }
    }
    check_npcs(&world, data)?;
    install_world(data, &world);
    Ok(world)
}

/// V2.1: every NPC stands in a room of the world, inside its walls.
pub fn check_npcs(world: &WorldDefinition, data: &GameData) -> Result<(), String> {
    for npc in data.npcs.npcs() {
        let Some(room) = world.room(&npc.room) else {
            return Err(format!(
                "NPC `{}` is in unknown room `{}`",
                npc.id, npc.room
            ));
        };
        let (w, h) = room.size();
        let (x, y) = npc.position;
        let (hw, hh) = (npc.size.0 / 2.0, npc.size.1 / 2.0);
        if x - hw < 0.0 || x + hw > w || y - hh < 0.0 || y + hh > h {
            return Err(format!(
                "NPC `{}` stands outside room `{}`",
                npc.id, npc.room
            ));
        }
    }
    Ok(())
}

/// V1.9 checks of the world against `data.transformations` (see [`load_world`]).
pub fn check_transformations(world: &WorldDefinition, data: &GameData) -> Result<(), String> {
    let expected = data.transformations.passages();
    if world.manifest.passages != expected {
        return Err(format!(
            "world passages {:?} do not match the transformations {:?}",
            world.manifest.passages, expected
        ));
    }
    let (sheet, ..) = crate::player::systems::starting_character(data);
    let budget = sheet.derived().max_mp as f32;
    for room in world.rooms.values() {
        for veil in &room.veils {
            let Some(def) = data.transformations.passing(veil.kind) else {
                return Err(format!("veil `{}`: no transformation passes it", veil.id));
            };
            let cost = crossing_mp(def, room, veil)
                .ok_or_else(|| format!("veil `{}` cannot be crossed", veil.id))?;
            if cost > budget {
                return Err(format!(
                    "veil `{}` costs {cost:.1} MP to cross; the starting character has {budget}",
                    veil.id
                ));
            }
        }
    }
    Ok(())
}

/// Builds the full game application around already-loaded data (single
/// room of `assets/maps/`, no world graph).
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

/// The shipped game: [`build`] plus the active world graph. `data` must
/// come from [`load_world`] (rooms installed).
pub fn build_with_world(data: GameData, world: WorldDefinition) -> App {
    let mut app = build(data);
    activate_world(&mut app, world);
    app
}
