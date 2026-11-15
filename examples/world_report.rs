//! Content tool (V1.6): validates `assets/world/` and writes a report.
//!
//! No window, no game: it loads the world graph, runs the validator and
//! writes to `target/world_report/`:
//! - `report.txt`: errors, warnings, the access of every room (main path,
//!   secret, unreachable) and the abilities/items/flags the progression collects;
//! - `world.dot`: the graph in Graphviz text (render it with any Graphviz
//!   viewer; nothing is installed by the project). Solid edges are free doors,
//!   dashed edges need a requirement, dotted edges are hidden by a secret,
//!   one-headed arrows are one-way (arrival-only on the other side).
//!
//! Exit code != 0 when the world has errors. Run with:
//! `cargo run --example world_report`.

use std::fmt::Write as _;
use std::path::Path;
use std::process::ExitCode;

use dark_castle_rpg::core::resources::{GameData, project_root};
use dark_castle_rpg::transform::crossing::crossing_seconds;
use dark_castle_rpg::world::graph::{RoomRole, WorldDefinition};
use dark_castle_rpg::world::validation::{Access, validate};

fn dot(world: &WorldDefinition) -> String {
    let mut out =
        String::from("digraph world {\n  rankdir=LR;\n  node [shape=box, style=filled];\n");
    for (region_id, region) in &world.regions {
        let _ = writeln!(
            out,
            "  subgraph \"cluster_{region_id}\" {{\n    label=\"{}\";",
            region.name
        );
        for room in world.rooms.values().filter(|r| &r.region == region_id) {
            let color = match room.role {
                RoomRole::Required => "#c9b8d8",
                RoomRole::Optional => "#d8d0b8",
                RoomRole::Secret => "#e8c070",
            };
            let goal = if room.id == world.manifest.goal_room {
                ", peripheries=2"
            } else {
                ""
            };
            let _ = writeln!(
                out,
                "    \"{}\" [label=\"{}\\n({}, {}) {}x{}\", fillcolor=\"{color}\"{goal}];",
                room.id, room.name, room.map.x, room.map.y, room.map.w, room.map.h
            );
        }
        out.push_str("  }\n");
    }
    for room in world.rooms.values() {
        for door in &room.doors {
            if door.entry_only {
                continue;
            }
            let back = world.target(door).is_some_and(|(_, d)| !d.entry_only);
            // Draw two-way doors once.
            if back && room.id > door.to.room {
                continue;
            }
            let style = if door.hidden_by.is_some() {
                "dotted"
            } else if door.requires.is_some() {
                "dashed"
            } else {
                "solid"
            };
            let dir = if back { "both" } else { "forward" };
            let label = door
                .requires
                .as_ref()
                .map(|r| format!(", label=\"{r:?}\""))
                .unwrap_or_default();
            let _ = writeln!(
                out,
                "  \"{}\" -> \"{}\" [style={style}, dir={dir}{label}];",
                room.id, door.to.room
            );
        }
    }
    out.push_str("}\n");
    out
}

fn main() -> ExitCode {
    let world = match WorldDefinition::load_dir(&project_root().join("assets/world")) {
        Ok(world) => world,
        Err(error) => {
            eprintln!("[world_report] {error}");
            return ExitCode::FAILURE;
        }
    };
    let report = validate(&world);
    let mut text = format!(
        "World: {} regions, {} rooms, start {}.{}, goal {}\n\n",
        world.regions.len(),
        world.rooms.len(),
        world.manifest.start_room,
        world.manifest.start_door,
        world.manifest.goal_room
    );
    let _ = writeln!(text, "Errors ({}):", report.errors.len());
    for issue in &report.errors {
        let _ = writeln!(text, "  {issue:?}");
    }
    let _ = writeln!(text, "Warnings ({}):", report.warnings.len());
    for issue in &report.warnings {
        let _ = writeln!(text, "  {issue:?}");
    }
    let _ = writeln!(text, "\nAccess:");
    for (room, access) in &report.access {
        let role = world.room(room).map(|r| r.role);
        let tag = match access {
            Access::Main => "main",
            Access::Secret => "secret",
            Access::Unreachable => "UNREACHABLE",
        };
        let _ = writeln!(text, "  {room:<32} {tag:<12} role {role:?}");
    }
    let _ = writeln!(
        text,
        "\nCollected by the progression: {:?}",
        report.final_capabilities
    );
    // V1.9: veils and what crossing each one costs the form that passes it.
    let _ = writeln!(text, "\nVeils (passages {:?}):", world.manifest.passages);
    let forms = GameData::load(&project_root()).map(|d| d.transformations);
    for room in world.rooms.values() {
        for veil in &room.veils {
            let cost = forms
                .as_ref()
                .ok()
                .and_then(|f| f.passing(veil.kind))
                .and_then(|def| crossing_seconds(def, room, veil).map(|t| (t, def.mp_for(t))));
            let cost = cost.map_or("no form passes it".to_owned(), |(t, mp)| {
                format!("crossing {t:.2} s, {mp:.1} MP with the activation")
            });
            let _ = writeln!(
                text,
                "  {:<24} {}.{} {:?} depth {}: {cost}",
                veil.id, room.id, veil.door, veil.kind, veil.depth
            );
        }
    }
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/world_report");
    let written = std::fs::create_dir_all(&dir)
        .and_then(|()| std::fs::write(dir.join("report.txt"), &text))
        .and_then(|()| std::fs::write(dir.join("world.dot"), dot(&world)));
    if let Err(error) = written {
        eprintln!("[world_report] cannot write {}: {error}", dir.display());
        return ExitCode::FAILURE;
    }
    print!("{text}");
    println!("[world_report] written to {}", dir.display());
    if report.is_ok() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
