//! Map panel of the world graph (V1.6 prototype, in the game since V1.7).
//!
//! Draws, in the top-right corner, every room the player knows: visited
//! rooms filled with their region colour, discovered rooms as outlines,
//! the current room with a bright border; unknown rooms (and secret rooms
//! not found) are not drawn. Known doors (`ExplorationState::known_doors`)
//! are small marks on the room borders (blue while locked); a gold mark
//! flags a visited room whose secret was found. Below it: exploration
//! percentage and secrets. It is UI: the world camera is not touched.
//! Rebuilt only when the exploration state, the current room or the panel
//! state changes. `M` (`InputBindings::map`) opens and closes it; it stays
//! hidden while no world is active (the single-room test game).
//!
//! V1.8: chests of visited rooms are marked (bright while closed, dim once
//! opened), open shortcuts are drawn green, the secret mark also counts
//! breakable walls and hidden switches, and the label adds the percentage
//! of the current region. Nothing of an unvisited room is drawn but its
//! outline, so chests and secrets are never revealed early.
//!
//! D17: discovered save points are marked in cyan on visited rooms.

use bevy::prelude::*;

use super::theme::{FONT_SMALL, PANEL_BG, PANEL_BORDER, TEXT_MAIN, text};
use crate::app::states::GameSet;
use crate::core::resources::GameData;
use crate::input::InputBindings;
use crate::npc::definitions::NpcMapMarker;
use crate::world::WorldState;
use crate::world::exploration::{Discovery, DoorState, ObjectState};
use crate::world::exploration_systems::{Exploration, WorldGraph};
use crate::world::graph::{ObjectKind, Side};
use crate::world::maps::CurrentRoom;

/// Size of one map cell on screen, in pixels.
const CELL_PX: (f32, f32) = (26.0, 15.0);
const PANEL_PAD: f32 = 10.0;
const CURRENT_BORDER: Color = Color::srgb(1.0, 0.95, 0.75);
const DISCOVERED_BORDER: Color = Color::srgb(0.5, 0.48, 0.55);
const DOOR_COLOR: Color = Color::srgb(0.92, 0.9, 0.85);
const LOCKED_DOOR_COLOR: Color = Color::srgb(0.4, 0.55, 0.95);
const SECRET_MARK_COLOR: Color = Color::srgb(0.95, 0.78, 0.3);
const SHORTCUT_COLOR: Color = Color::srgb(0.4, 0.9, 0.5);
/// A door filled by a veil (V1.9): only a form passes it.
const VEIL_DOOR_COLOR: Color = Color::srgb(0.6, 0.88, 1.0);
const CHEST_CLOSED_COLOR: Color = Color::srgb(0.98, 0.62, 0.22);
const CHEST_OPENED_COLOR: Color = Color::srgb(0.38, 0.33, 0.3);
const SAVE_POINT_MARK_COLOR: Color = Color::srgb(0.35, 0.9, 0.85);
/// Size of a door mark and of the secret mark, in pixels.
const MARK_PX: f32 = 4.0;
/// V2.2: NPC markers (only in visited rooms, like the chests).
const SHOP_COLOR: Color = Color::srgb(0.78, 0.5, 1.0);
const NPC_COLOR: Color = Color::srgb(1.0, 0.62, 0.8);
const SHOP_MARK_PX: f32 = 6.0;
const LEGEND: &str = "violet: shop  pink: NPC  orange: chest";

/// Whether the player wants the map panel open (toggled with `M`).
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapPanelState {
    pub open: bool,
}

impl Default for MapPanelState {
    fn default() -> Self {
        Self { open: true }
    }
}

#[derive(Component)]
pub struct WorldMapPanel;

#[derive(Component)]
pub struct WorldMapCells;

#[derive(Component)]
pub struct WorldMapLabel;

pub struct WorldMapUiPlugin;

impl Plugin for WorldMapUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapPanelState>()
            .add_systems(Startup, spawn_map_panel)
            .add_systems(
                Update,
                (toggle_map, show_map_panel, redraw_map)
                    .chain()
                    .in_set(GameSet::Presentation),
            );
    }
}

pub fn spawn_map_panel(mut commands: Commands) {
    commands
        .spawn((
            WorldMapPanel,
            Node {
                position_type: PositionType::Absolute,
                top: px(12),
                right: px(12),
                flex_direction: FlexDirection::Column,
                row_gap: px(6),
                padding: UiRect::all(px(PANEL_PAD)),
                border: UiRect::all(px(1)),
                ..default()
            },
            BackgroundColor(PANEL_BG),
            BorderColor::all(PANEL_BORDER),
        ))
        .with_children(|panel| {
            panel.spawn((
                WorldMapCells,
                Node {
                    position_type: PositionType::Relative,
                    ..default()
                },
            ));
            panel.spawn((WorldMapLabel, text("", FONT_SMALL, TEXT_MAIN)));
        });
}

/// `M` opens or closes the panel (also while paused).
pub fn toggle_map(
    keys: Res<ButtonInput<KeyCode>>,
    bindings: Res<InputBindings>,
    mut panel: ResMut<MapPanelState>,
) {
    if keys.just_pressed(bindings.map) {
        panel.open = !panel.open;
    }
}

/// Shows the panel only when it is open and a world is active.
pub fn show_map_panel(
    panel: Res<MapPanelState>,
    world: Option<Res<WorldGraph>>,
    mut nodes: Query<&mut Node, With<WorldMapPanel>>,
) {
    let display = if panel.open && world.is_some() {
        Display::Flex
    } else {
        Display::None
    };
    for mut node in &mut nodes {
        if node.display != display {
            node.display = display;
        }
    }
}

/// Map pixel offset of a door mark from the room's top-left corner.
fn door_mark(side: Side, cell: u32, w: u32, h: u32) -> (f32, f32) {
    let c = cell as f32 + 0.5;
    let (width, height) = (w as f32 * CELL_PX.0, h as f32 * CELL_PX.1);
    let half = MARK_PX / 2.0;
    match side {
        Side::Left => (-half, height - c * CELL_PX.1 - half),
        Side::Right => (width - half - 2.0, height - c * CELL_PX.1 - half),
        Side::Top => (c * CELL_PX.0 - half, -half),
        Side::Bottom => (c * CELL_PX.0 - half, height - half - 2.0),
    }
}

fn mark(left: f32, top: f32, color: Color) -> (Node, BackgroundColor) {
    sized_mark(left, top, MARK_PX, color)
}

fn sized_mark(left: f32, top: f32, size: f32, color: Color) -> (Node, BackgroundColor) {
    (
        Node {
            position_type: PositionType::Absolute,
            left: px(left),
            top: px(top),
            width: px(size),
            height: px(size),
            ..default()
        },
        BackgroundColor(color),
    )
}

/// Top-left of a marker of `size` px for a point of the room (room units,
/// y up) on a room box of `box_px`, kept inside the box (V2.2).
pub fn marker_spot(
    point: (f32, f32),
    room: (f32, f32),
    box_px: (f32, f32),
    size: f32,
) -> (f32, f32) {
    let left = point.0 / room.0 * box_px.0 - size / 2.0;
    let top = box_px.1 - point.1 / room.1 * box_px.1 - size;
    (
        left.clamp(0.0, (box_px.0 - size - 2.0).max(0.0)),
        top.clamp(0.0, (box_px.1 - size - 2.0).max(0.0)),
    )
}

/// One rectangle per known room, positioned from its map cells.
pub fn redraw_map(
    mut commands: Commands,
    world: Option<Res<WorldGraph>>,
    exploration: Option<Res<Exploration>>,
    current: Option<Res<CurrentRoom>>,
    world_state: Option<Res<WorldState>>,
    data: Option<Res<GameData>>,
    cells: Query<(Entity, Option<&Children>), With<WorldMapCells>>,
    mut label: Query<&mut Text, With<WorldMapLabel>>,
) {
    let (Some(world), Some(exploration), Some(current)) = (world, exploration, current) else {
        return;
    };
    if !exploration.is_changed()
        && !current.is_changed()
        && !world.is_changed()
        && !world_state.as_ref().is_some_and(|w| w.is_changed())
    {
        return;
    }
    let Ok((container, children)) = cells.single() else {
        return;
    };
    for child in children.into_iter().flatten() {
        commands.entity(*child).try_despawn();
    }
    let mut npc_marks = 0;
    let rooms = world.0.rooms.values();
    let min_x = rooms.clone().map(|r| r.map.x).min().unwrap_or(0);
    let max_x = rooms
        .clone()
        .map(|r| r.map.x + r.map.w as i32)
        .max()
        .unwrap_or(1);
    let max_y = rooms
        .clone()
        .map(|r| r.map.y + r.map.h as i32)
        .max()
        .unwrap_or(1);
    let min_y = rooms.clone().map(|r| r.map.y).min().unwrap_or(0);
    commands.entity(container).insert(Node {
        position_type: PositionType::Relative,
        width: px((max_x - min_x) as f32 * CELL_PX.0),
        height: px((max_y - min_y) as f32 * CELL_PX.1),
        ..default()
    });
    for room in rooms {
        let state = exploration.discovery(&room.id);
        if state == Discovery::Unknown {
            continue;
        }
        let region = world
            .0
            .regions
            .get(&room.region)
            .map_or((0.5, 0.5, 0.5), |r| r.color);
        let fill = match state {
            Discovery::Visited => Color::srgb(region.0, region.1, region.2),
            Discovery::Discovered | Discovery::Unknown => Color::NONE,
        };
        let border = if room.id == current.0 {
            CURRENT_BORDER
        } else {
            DISCOVERED_BORDER
        };
        let node = commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: px((room.map.x - min_x) as f32 * CELL_PX.0),
                    top: px((max_y - room.map.y - room.map.h as i32) as f32 * CELL_PX.1),
                    width: px(room.map.w as f32 * CELL_PX.0 - 2.0),
                    height: px(room.map.h as f32 * CELL_PX.1 - 2.0),
                    border: UiRect::all(px(if room.id == current.0 { 2 } else { 1 })),
                    ..default()
                },
                BackgroundColor(fill),
                BorderColor::all(border),
            ))
            .id();
        commands.entity(container).add_child(node);
        if state != Discovery::Visited {
            continue;
        }
        let found_secret = room.secret_ids().any(|s| exploration.secrets.contains(s));
        if found_secret {
            let secret = commands.spawn(mark(2.0, 2.0, SECRET_MARK_COLOR)).id();
            commands.entity(node).add_child(secret);
        }
        let (w, h) = room.size();
        let (px_w, px_h) = (room.map.w as f32 * CELL_PX.0, room.map.h as f32 * CELL_PX.1);
        for chest in room
            .objects
            .iter()
            .filter(|o| matches!(o.kind, ObjectKind::Chest { .. }))
        {
            let color = if exploration.object_state(chest) == ObjectState::Opened {
                CHEST_OPENED_COLOR
            } else {
                CHEST_CLOSED_COLOR
            };
            let a = chest.area;
            let left = (a.min.0 + a.size.0 / 2.0) / w * px_w - MARK_PX / 2.0;
            let top = px_h - (a.min.1 + a.size.1 / 2.0) / h * px_h - MARK_PX;
            let chest_mark = commands.spawn(mark(left, top, color)).id();
            commands.entity(node).add_child(chest_mark);
        }
        if let Some(progress) = world_state.as_ref() {
            for altar in room
                .objects
                .iter()
                .filter(|o| matches!(o.kind, ObjectKind::SavePoint))
            {
                if !progress.0.discovered_save_points.contains(&altar.id) {
                    continue;
                }
                let a = altar.area;
                let left = (a.min.0 + a.size.0 / 2.0) / w * px_w - MARK_PX / 2.0;
                let top = px_h - (a.min.1 + a.size.1 / 2.0) / h * px_h - MARK_PX;
                let save_mark = commands.spawn(mark(left, top, SAVE_POINT_MARK_COLOR)).id();
                commands.entity(node).add_child(save_mark);
            }
        }
        // V2.2: shops and NPCs of the room (one marker each, inside the box).
        for npc in data.iter().flat_map(|d| d.npcs.in_room(&room.id)) {
            let (color, size) = match npc.map_marker() {
                NpcMapMarker::Shop => (SHOP_COLOR, SHOP_MARK_PX),
                NpcMapMarker::Npc => (NPC_COLOR, MARK_PX),
            };
            let (left, top) = marker_spot(npc.position, (w, h), (px_w, px_h), size);
            let npc_mark = commands.spawn(sized_mark(left, top, size, color)).id();
            commands.entity(node).add_child(npc_mark);
            npc_marks += 1;
        }
    }
    for known in exploration.known_doors(&world.0) {
        let (Some(room), Some(door)) = (
            world.0.room(&known.room),
            world.0.room(&known.room).and_then(|r| r.door(&known.door)),
        ) else {
            continue;
        };
        let (dx, dy) = door_mark(door.side, door.cell, room.map.w, room.map.h);
        let left = (room.map.x - min_x) as f32 * CELL_PX.0 + dx;
        let top = (max_y - room.map.y - room.map.h as i32) as f32 * CELL_PX.1 + dy;
        let color = match exploration.door_state(door) {
            _ if room.veil(&door.id).is_some() => VEIL_DOOR_COLOR,
            DoorState::Locked => LOCKED_DOOR_COLOR,
            DoorState::Open if door.shortcut => SHORTCUT_COLOR,
            _ => DOOR_COLOR,
        };
        let node = commands.spawn(mark(left, top, color)).id();
        commands.entity(container).add_child(node);
    }
    let stats = exploration.stats(&world.0);
    let region = world.0.room(&current.0).and_then(|r| {
        let def = world.0.regions.get(&r.region)?;
        Some((
            def.name.clone(),
            exploration.region_stats(&world.0, &def.id),
        ))
    });
    if let Ok(mut text) = label.single_mut() {
        let mut line = format!(
            "Map {:.0}%  rooms {}/{}  secrets {}/{}",
            stats.percent, stats.visited, stats.total, stats.secrets_found, stats.secrets_total
        );
        if let Some((name, r)) = region {
            line.push_str(&format!("\n{name} {:.0}%", r.percent));
        }
        if npc_marks > 0 {
            line.push('\n');
            line.push_str(LEGEND);
        }
        **text = line;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markers_stay_inside_the_room_box() {
        // Middle of a 1280x720 room on a 52x30 px box.
        let (l, t) = marker_spot((640.0, 360.0), (1280.0, 720.0), (52.0, 30.0), 4.0);
        assert_eq!((l, t), (24.0, 11.0));
        // Points on or past the edges are clamped inside.
        for p in [(0.0, 0.0), (1280.0, 720.0), (-50.0, 900.0), (2000.0, -10.0)] {
            let (l, t) = marker_spot(p, (1280.0, 720.0), (52.0, 30.0), 6.0);
            assert!(
                (0.0..=44.0).contains(&l) && (0.0..=22.0).contains(&t),
                "{p:?}"
            );
        }
    }
}
