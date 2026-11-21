//! Modular Character Engine 01: the rig lab against the approved art.
//!
//! - `art-src/modular-character-engine/stage/rig_lab.ron` loads and points
//!   at the approved strips and the existing items;
//! - the run plays the six approved frames at 1/15 s (0.4 s per cycle);
//! - every marked skeleton lands on the art: each fist joint is on fist
//!   texels, the pelvis on the body;
//! - the depth partition of every approved frame (from the palette grids)
//!   labels every opaque texel exactly once and keeps the fist on the near
//!   arm layer;
//! - the same sword and pauldron, drawn once, sit on the fist and the
//!   shoulder of every frame and mirror with the facing;
//! - equipping and unequipping never move the clock;
//! - the articulated comparison reaches every marked ankle the rest leg can
//!   reach by IK, and stretches straight towards the others (measured).

use std::path::{Path, PathBuf};

use dark_castle_rpg::player::visual::playback::loop_frame;
use dark_castle_rpg::player::visual::rig::articulated::articulate;
use dark_castle_rpg::player::visual::rig::data::RigLab;
use dark_castle_rpg::player::visual::rig::equipment::{RigLoadout, RigSlot, mirror, place};
use dark_castle_rpg::player::visual::rig::layers::{
    BodyLayer, CellGeometry, Material, material_of_key, partition,
};
use dark_castle_rpg::player::visual::rig::skeleton::{Bone, Joint, fit, forward};

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn lab() -> RigLab {
    let path = root().join("art-src/modular-character-engine/stage/rig_lab.ron");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    RigLab::from_ron(&text).unwrap_or_else(|e| panic!("{e}"))
}

/// Palette grid of an approved frame (one char per texel).
fn grid(path: &Path, width: usize) -> Vec<Material> {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let rows: Vec<Vec<char>> = text.lines().map(|l| l.chars().collect()).collect();
    assert_eq!(rows.len(), 128, "{}", path.display());
    assert!(rows.iter().all(|r| r.len() == width), "{}", path.display());
    rows.iter().flatten().map(|c| material_of_key(*c)).collect()
}

/// (clip, source) -> palette grid of the approved frame.
fn source_grid(clip: &str, source: usize) -> (Vec<Material>, PathBuf) {
    let path = if clip == "idle" {
        root().join("art-src/n3-5-fidelity/source/candidate_106.txt")
    } else {
        // run6 = video frames 48, 50, ..., 58.
        root().join(format!(
            "art-src/n3-6-animation-proof/source/video_{:03}.txt",
            48 + 2 * source
        ))
    };
    let width = if clip == "idle" { 80 } else { 112 };
    (grid(&path, width), path)
}

fn texel_material(materials: &[Material], cell: CellGeometry, p: (f32, f32)) -> Material {
    let (x, y) = cell.to_texel(p);
    let (x, y) = (x.floor() as usize, y.floor() as usize);
    materials[y * cell.width + x]
}

#[test]
fn the_lab_reads_the_approved_strips_and_the_existing_items() {
    let lab = lab();
    let run = &lab.clips["run"];
    assert_eq!(run.texture, "approved://sprites/player/n36_run6.png");
    assert_eq!(
        (run.cell, run.columns, run.pivot),
        ((112, 128), 6, (56.0, 118.0))
    );
    let idle = &lab.clips["idle"];
    assert_eq!((idle.cell, idle.pivot), ((80, 128), (40.0, 118.0)));
    for path in [
        "art-src/n3-8-production-run/stage/sprites/player/n36_run6.png",
        "art-src/n3-8-production-run/stage/sprites/player/n35_idle.png",
    ] {
        assert!(root().join(path).exists(), "{path}");
    }
    for (id, item) in &lab.items {
        let (source, rel) = item
            .texture
            .split_once("://")
            .unwrap_or_else(|| panic!("{id}: no asset source"));
        let dir = match source {
            "factory" => "art-src/n4-sprite-machine/stage/factory/isolated",
            "mce" => "art-src/modular-character-engine/stage",
            other => panic!("{id}: unknown source {other}"),
        };
        assert!(root().join(dir).join(rel).exists(), "{id}: {rel}");
        // Derived art must say so.
        assert_eq!(item.provisional, source == "mce", "{id}");
    }
}

#[test]
fn the_run_plays_the_six_approved_frames_at_fifteen_fps() {
    let lab = lab();
    let run = &lab.clips["run"];
    let sources: Vec<usize> = run.frames.iter().map(|f| f.source).collect();
    assert_eq!(sources, vec![0, 1, 2, 3, 4, 5]);
    assert!(run.frames.iter().all(|f| f.drop == 0));
    let total: f32 = run.durations().iter().sum();
    assert!((total - 0.4).abs() < 1e-4);
}

#[test]
fn every_marked_skeleton_lands_on_the_art() {
    let lab = lab();
    for (key, clip) in &lab.clips {
        let cell = clip.geometry();
        for source in 0..clip.sources.len() {
            let (materials, path) = source_grid(key, source);
            let j = clip
                .source_joints(source)
                .unwrap_or_else(|| panic!("{key} #{source}"));
            let fist = texel_material(&materials, cell, j.get(Joint::HandNear));
            assert!(
                matches!(fist, Material::Skin | Material::Leather | Material::Outline),
                "{} near fist on {fist:?}",
                path.display()
            );
            let pelvis = texel_material(&materials, cell, j.get(Joint::Pelvis));
            assert_ne!(pelvis, Material::Transparent, "{} pelvis", path.display());
            let head = texel_material(&materials, cell, j.get(Joint::Head));
            assert_ne!(head, Material::Transparent, "{} head", path.display());
        }
    }
}

#[test]
fn the_depth_partition_labels_every_opaque_texel_once() {
    let lab = lab();
    for (key, clip) in &lab.clips {
        let cell = clip.geometry();
        for source in 0..clip.sources.len() {
            let (materials, path) = source_grid(key, source);
            let j = clip
                .source_joints(source)
                .unwrap_or_else(|| panic!("{key} #{source}"));
            let labels = partition(&materials, cell, &j);
            for (m, l) in materials.iter().zip(&labels) {
                assert_eq!(
                    *m == Material::Transparent,
                    l.is_none(),
                    "{}",
                    path.display()
                );
            }
            for layer in BodyLayer::ALL {
                let n = labels.iter().filter(|l| **l == Some(layer)).count();
                assert!(n > 0, "{} has no {layer:?} texel", path.display());
            }
            // The fist joint's texel is drawn on the near arm layer.
            let (x, y) = cell.to_texel(j.get(Joint::HandNear));
            let i = y.floor() as usize * cell.width + x.floor() as usize;
            assert_eq!(labels[i], Some(BodyLayer::NearArm), "{}", path.display());
        }
    }
}

#[test]
fn one_sword_and_one_pauldron_follow_every_frame() {
    let lab = lab();
    let rest_clip = lab.rest().unwrap_or_else(|| panic!("rest"));
    let rest = fit(&rest_clip
        .frame_joints(0)
        .unwrap_or_else(|| panic!("rest pose")));
    let sword = &lab.items["sword_arming_cross"];
    let pauldron = &lab.items["pauldron_steel"];
    let mut rotations = Vec::new();
    for clip in lab.clips.values() {
        for frame in 0..clip.frames.len() {
            let j = clip.frame_joints(frame).unwrap_or_else(|| panic!("frame"));
            let pose = forward(&fit(&j));
            let s = place(sword, &pose, &rest);
            let fist = j.get(Joint::HandNear);
            assert!((s.position.0 - fist.0).abs() < 1e-3 && (s.position.1 - fist.1).abs() < 1e-3);
            let p = place(pauldron, &pose, &rest);
            let shoulder = j.get(Joint::ShoulderNear);
            assert!((p.position.0 - shoulder.0).abs() < 1e-3);
            assert!((p.position.1 - shoulder.1).abs() < 1e-3);
            rotations.push(s.rotation);
            // Facing left: mirrored about the pivot, same depth.
            let m = mirror(s, true);
            assert!((m.position.0 + s.position.0).abs() < 1e-3);
            assert!((m.rotation + s.rotation).abs() < 1e-3);
            assert_eq!(m.depth, s.depth);
        }
    }
    // The same piece really takes different attitudes across the poses.
    let spread = rotations.iter().cloned().fold(f32::MIN, f32::max)
        - rotations.iter().cloned().fold(f32::MAX, f32::min);
    assert!(spread > 1.0, "sword rotations barely change: {spread}");
}

#[test]
fn equipping_never_moves_the_clock() {
    let lab = lab();
    let durations = lab.clips["run"].durations();
    let mut loadout = RigLoadout::default();
    let mut elapsed = 0.0;
    for step in 0..24 {
        let before = loop_frame(&durations, elapsed);
        match step % 4 {
            0 => {
                loadout.equip(RigSlot::MainHand, "sword_arming_cross");
            }
            1 => {
                loadout.equip(RigSlot::MainHand, "sword_estoc_ring");
            }
            2 => {
                loadout.unequip(RigSlot::MainHand);
            }
            _ => {
                loadout.equip(RigSlot::Shoulder, "pauldron_bronze");
            }
        }
        assert_eq!(loop_frame(&durations, elapsed), before);
        assert_eq!(before, step % 6);
        elapsed += 1.0 / 15.0;
        elapsed += 1e-6;
    }
}

#[test]
fn the_articulated_pose_reaches_every_ankle_within_the_rest_leg() {
    let lab = lab();
    let rest_clip = lab.rest().unwrap_or_else(|| panic!("rest"));
    let rest = fit(&rest_clip
        .frame_joints(0)
        .unwrap_or_else(|| panic!("rest pose")));
    let run = &lab.clips["run"];
    let mut clamped = Vec::new();
    for frame in 0..run.frames.len() {
        let j = run.frame_joints(frame).unwrap_or_else(|| panic!("frame"));
        let world = articulate(&rest, &j);
        for (thigh, shin, ankle) in [
            (Bone::ThighA, Bone::ShinA, Joint::AnkleA),
            (Bone::ThighB, Bone::ShinB, Joint::AnkleB),
        ] {
            let hip = world.segment(thigh).0;
            let end = world.segment(shin).1;
            let goal = j.get(ankle);
            let leg = world.length(thigh) + world.length(shin);
            let wanted = (goal.0 - hip.0).hypot(goal.1 - hip.1);
            let miss = (end.0 - goal.0).hypot(end.1 - goal.1);
            if wanted < leg - 0.05 {
                assert!(miss < 0.5, "frame {frame} {ankle:?} misses by {miss}");
            } else {
                // Out of reach: the leg stretches straight towards the
                // target and stops at its rest length (no NaN, no flip).
                let reach = (end.0 - hip.0).hypot(end.1 - hip.1);
                assert!((reach - leg).abs() < 0.1, "frame {frame} {ankle:?}");
                assert!((miss - (wanted - reach)).abs() < 0.1);
                clamped.push((frame, ankle, wanted - leg));
            }
        }
    }
    // Measured limitation (report 01, section 20): the approved run draws
    // some legs longer than the idle leg. Recorded, not hidden.
    println!("clamped ankles (frame, joint, texels beyond the rest leg): {clamped:?}");
    assert!(clamped.len() <= 4, "{clamped:?}");
}
