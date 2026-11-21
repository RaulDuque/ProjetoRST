//! Modular Character Engine 01/02: the rig lab against the approved art.
//!
//! - `art-src/modular-character-engine/stage/rig_lab.ron` loads and points
//!   at the approved strips and the existing items;
//! - the run plays the six approved frames at 1/15 s (0.4 s per cycle);
//! - every marked skeleton lands on the art: each fist joint is on (and
//!   centred on) fist texels, the pelvis, neck and head on the body;
//! - 02: the N4-C label maps fit the art, the rule partition labels every
//!   opaque texel once, keeps the fist on the near arm and agrees with N4-C;
//! - 02: swords are held through grip profiles: on the fist in every frame,
//!   never through the ground, mirrored, steady in the idle, with a
//!   controlled swing in the run;
//! - 02: one breastplate model covers the torso box of every pose, wider
//!   than the shirt, palette only, in the projected and warped approaches;
//! - 02: the refined marks are steadier than prototype 01's (measured);
//! - equipping and unequipping never move the clock;
//! - the articulated comparison reaches every ankle the rest leg can reach.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use dark_castle_rpg::player::visual::playback::loop_frame;
use dark_castle_rpg::player::visual::rig::articulated::articulate;
use dark_castle_rpg::player::visual::rig::chest::{ChestModel, TorsoCage, warp};
use dark_castle_rpg::player::visual::rig::data::RigLab;
use dark_castle_rpg::player::visual::rig::equipment::{
    ItemArt, Mount, RigLoadout, RigSlot, mirror, place_in_hand, place_on_bone, sprite_point,
};
use dark_castle_rpg::player::visual::rig::grip::{HandSide, hand_frame};
use dark_castle_rpg::player::visual::rig::layers::{
    BodyLayer, CellGeometry, Material, label_mismatches, layer_iou, material_of_key, parse_labels,
    partition, point_in_polygon, torso_quad,
};
use dark_castle_rpg::player::visual::rig::measure::{
    centroid_near, loop_angle_jerk, loop_jerk, spine_frame, spread, to_local,
};
use dark_castle_rpg::player::visual::rig::skeleton::{Bone, Joint, fit, forward};
use dark_castle_rpg::player::visual::rig::xform::distance_to_segment;
use serde::Deserialize;

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn stage() -> PathBuf {
    root().join("art-src/modular-character-engine/stage")
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn lab() -> RigLab {
    RigLab::from_ron(&read(&stage().join("rig_lab.ron"))).unwrap_or_else(|e| panic!("{e}"))
}

/// Palette grid of an approved frame (one char per texel).
fn grid(path: &Path, width: usize) -> Vec<Material> {
    let text = read(path);
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

fn texel_index(cell: CellGeometry, p: (f32, f32)) -> usize {
    let (x, y) = cell.to_texel(p);
    y.floor() as usize * cell.width + x.floor() as usize
}

/// The labels the lab uses for a source: the authored map, or the rules.
fn labels_for(lab: &RigLab, clip: &str, source: usize) -> Vec<Option<BodyLayer>> {
    let c = &lab.clips[clip];
    let cell = c.geometry();
    let (materials, _) = source_grid(clip, source);
    match c.labels.get(source).cloned().flatten() {
        Some(path) => {
            parse_labels(&read(&stage().join(path)), cell).unwrap_or_else(|e| panic!("{e:?}"))
        }
        None => partition(
            &materials,
            cell,
            &c.source_joints(source).unwrap_or_else(|| panic!("joints")),
        ),
    }
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
    for (id, item) in &lab.items {
        match &item.art {
            ItemArt::Sprite(s) => {
                let (source, rel) = s
                    .texture
                    .split_once("://")
                    .unwrap_or_else(|| panic!("{id}: no asset source"));
                let dir = match source {
                    "factory" => "art-src/n4-sprite-machine/stage/factory/isolated",
                    "mce" => "art-src/modular-character-engine/stage",
                    other => panic!("{id}: unknown source {other}"),
                };
                assert!(root().join(dir).join(rel).exists(), "{id}: {rel}");
                assert_eq!(item.provisional, source == "mce", "{id}");
            }
            ItemArt::Model(m) => {
                assert!(lab.models.contains_key(m), "{id}");
                assert!(
                    item.provisional,
                    "{id}: the diagnostic model is provisional"
                );
            }
        }
    }
    for slot in [RigSlot::MainHand, RigSlot::Shoulder, RigSlot::Chest] {
        assert!(!lab.variants[&slot].is_empty(), "{slot:?}");
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
            let fist = materials[texel_index(cell, j.get(Joint::HandNear))];
            assert!(
                matches!(fist, Material::Skin | Material::Leather | Material::Outline),
                "{} near fist on {fist:?}",
                path.display()
            );
            // 02: the fist joint is centred on the fist's skin.
            let seed = cell.to_texel(j.get(Joint::HandNear));
            let c = centroid_near(&materials, cell.width, seed, 3.5, |m| *m == Material::Skin)
                .unwrap_or_else(|| panic!("{} no skin at the fist", path.display()));
            let off = (c.0 - seed.0).hypot(c.1 - seed.1);
            assert!(off < 2.0, "{} fist off its skin by {off}", path.display());
            for joint in [Joint::Pelvis, Joint::Head, Joint::Neck] {
                let m = materials[texel_index(cell, j.get(joint))];
                assert_ne!(m, Material::Transparent, "{} {joint:?}", path.display());
            }
        }
    }
}

#[test]
fn n4c_label_maps_fit_the_art_and_the_rules_agree_with_them() {
    let lab = lab();
    let run = &lab.clips["run"];
    let cell = run.geometry();
    let mut ious = Vec::new();
    for source in [0, 2, 4] {
        let (materials, path) = source_grid("run", source);
        let authored = labels_for(&lab, "run", source);
        assert_eq!(
            label_mismatches(&materials, &authored),
            0,
            "{}",
            path.display()
        );
        let joints = run.source_joints(source).unwrap_or_else(|| panic!("j"));
        let rules = partition(&materials, cell, &joints);
        let near = layer_iou(&rules, &authored, BodyLayer::NearArm);
        let hair = layer_iou(&rules, &authored, BodyLayer::Hair);
        ious.push((source, near, hair));
        assert!(near > 0.6, "#{source}: near arm agreement {near}");
        assert!(hair > 0.7, "#{source}: hair agreement {hair}");
    }
    println!("rule partition vs N4-C (frame, near arm IoU, hair IoU): {ious:?}");
}

#[test]
fn every_frame_is_labelled_once_with_the_fist_on_the_near_arm() {
    let lab = lab();
    for (key, clip) in &lab.clips {
        let cell = clip.geometry();
        for source in 0..clip.sources.len() {
            let (materials, path) = source_grid(key, source);
            let j = clip.source_joints(source).unwrap_or_else(|| panic!("j"));
            let labels = labels_for(&lab, key, source);
            assert_eq!(
                label_mismatches(&materials, &labels),
                0,
                "{}",
                path.display()
            );
            for layer in BodyLayer::ALL {
                assert!(
                    labels.contains(&Some(layer)),
                    "{} has no {layer:?} texel",
                    path.display()
                );
            }
            assert_eq!(
                labels[texel_index(cell, j.get(Joint::HandNear))],
                Some(BodyLayer::NearArm),
                "{}",
                path.display()
            );
            // No shirt texel of the torso far from the arm lands on the near
            // arm (the prototype 01 over-capture).
            let quad = torso_quad(&j);
            let (s, e) = j.segment(Bone::UpperArmNear);
            let hand = j.get(Joint::HandNear);
            for (i, l) in labels.iter().enumerate() {
                let p = cell.texel_centre(i % cell.width, i / cell.width);
                if *l == Some(BodyLayer::NearArm)
                    && materials[i] == Material::Shirt
                    && point_in_polygon(p, &quad)
                {
                    let d = distance_to_segment(p, s, e).min(distance_to_segment(p, e, hand));
                    // N4-C sleeves are wide (up to ~7 texels from these bones).
                    assert!(
                        d < 7.5,
                        "{} shirt texel {i} on the near arm, {d} away",
                        path.display()
                    );
                }
            }
        }
    }
}

#[test]
fn swords_are_gripped_on_the_fist_above_the_ground_and_mirror() {
    let lab = lab();
    let rest = fit(&lab
        .rest()
        .and_then(|c| c.frame_joints(0))
        .unwrap_or_else(|| panic!("rest")));
    let sword = &lab.items["sword_arming_cross"];
    let ItemArt::Sprite(art) = &sword.art else {
        panic!("sprite")
    };
    let Mount::Hand { side, up_degrees } = sword.mount else {
        panic!("hand mount")
    };
    assert_eq!(side, HandSide::Near);
    let mut idle_angles = Vec::new();
    let mut run_angles = Vec::new();
    for (key, clip) in &lab.clips {
        let grip = lab.grip(key).unwrap_or_else(|| panic!("{key}: no grip"));
        for frame in 0..clip.frames.len() {
            let j = clip.frame_joints(frame).unwrap_or_else(|| panic!("frame"));
            let pose = forward(&fit(&j));
            let hand = hand_frame(side, &j, &pose, &rest, grip);
            let p = place_in_hand(hand, up_degrees, grip.depth_at(frame, sword.depth));
            let fist = j.get(Joint::HandNear);
            assert!((p.position.0 - fist.0).abs() < 1e-3 && (p.position.1 - fist.1).abs() < 1e-3);
            // Tip (row 4) and pommel (row 63) above the sole line.
            for texel in [(10.0, 4.0), (10.0, 63.0)] {
                let q = sprite_point(&p, art, texel);
                assert!(q.1 >= 0.0, "{key} #{frame}: sword at y {}", q.1);
            }
            // Mirror: same depth, x and rotation negated.
            let m = mirror(p, true);
            assert!((m.position.0 + p.position.0).abs() < 1e-3);
            assert!((m.rotation + p.rotation).abs() < 1e-3);
            assert_eq!(m.depth, p.depth);
            if key == "idle" {
                idle_angles.push(hand.angle);
            } else {
                run_angles.push(hand.angle);
            }
        }
    }
    // Idle: steady, diagonally down and forward.
    // (the breath frame tilts the spine by a hundredth of a radian)
    assert!(
        idle_angles
            .iter()
            .all(|a| (a - idle_angles[0]).abs() < 0.01)
    );
    assert!(idle_angles[0] < 0.0 && idle_angles[0] > -std::f32::consts::FRAC_PI_2);
    // Run: carried back (blade towards -x), swinging within 40 degrees.
    assert!(run_angles.iter().all(|a| a.cos() < -0.7), "{run_angles:?}");
    let mean = run_angles
        .iter()
        .map(|a| a.sin())
        .sum::<f32>()
        .atan2(run_angles.iter().map(|a| a.cos()).sum());
    let swing = run_angles
        .iter()
        .map(|a| dark_castle_rpg::player::visual::rig::xform::wrap_angle(a - mean).abs())
        .fold(0.0, f32::max);
    assert!(
        swing.to_degrees() < 20.0,
        "run swing +-{}",
        swing.to_degrees()
    );
}

#[test]
fn one_breastplate_model_covers_the_torso_of_every_pose() {
    let lab = lab();
    let model: &ChestModel = &lab.models["plate_cuirass"];
    let rest_clip = lab.rest().unwrap_or_else(|| panic!("rest"));
    let rest_cage =
        TorsoCage::from_joints(&rest_clip.frame_joints(0).unwrap_or_else(|| panic!("j")));
    let rest_raster = model.rasterize(&rest_cage, rest_clip.geometry());
    let mut palette: Vec<[u8; 3]> = model.ramp.to_vec();
    palette.extend(model.trim);
    for (key, clip) in &lab.clips {
        let cell = clip.geometry();
        for source in 0..clip.sources.len() {
            let (materials, path) = source_grid(key, source);
            let j = clip.source_joints(source).unwrap_or_else(|| panic!("j"));
            let cage = TorsoCage::from_joints(&j);
            let px = model.rasterize(&cage, cell);
            let quad = torso_quad(&j);
            let (mut inside, mut covered, mut beyond_shirt) = (0usize, 0usize, 0usize);
            for (i, p) in px.iter().enumerate() {
                let c = cell.texel_centre(i % cell.width, i / cell.width);
                if point_in_polygon(c, &quad) {
                    inside += 1;
                    covered += usize::from(p[3] > 0);
                }
                // The plate has its own silhouette: it also covers texels
                // that are not shirt (it is not limited by the shirt mask).
                if p[3] > 0 && materials[i] != Material::Shirt {
                    beyond_shirt += 1;
                }
                if p[3] > 0 {
                    assert!(palette.contains(&[p[0], p[1], p[2]]), "{}", path.display());
                }
            }
            let ratio = covered as f32 / inside as f32;
            assert!(
                ratio > 0.9,
                "{}: plate covers {ratio} of the torso box",
                path.display()
            );
            assert!(
                beyond_shirt > 10,
                "{}: plate bounded by the shirt",
                path.display()
            );
            // The mesh warp of the rest raster also lands on this pose.
            let warped = warp(&rest_raster, &rest_cage, rest_clip.geometry(), &cage, cell);
            let n = warped.iter().filter(|p| p[3] > 0).count();
            assert!(n > inside / 2, "{}: warp {n}", path.display());
        }
    }
}

/// Prototype 01 data, only the marked joints (history/rig_lab_01.ron).
#[derive(Deserialize)]
struct History {
    clips: BTreeMap<String, HistoryClip>,
}

#[derive(Deserialize)]
struct HistoryClip {
    pivot: (f32, f32),
    sources: Vec<BTreeMap<Joint, (f32, f32)>>,
}

/// The near shoulder in the spine frame over the run.
fn shoulders(sources: &[BTreeMap<Joint, (f32, f32)>], pivot: (f32, f32)) -> Vec<(f32, f32)> {
    let rig = |t: (f32, f32)| (t.0 - pivot.0, pivot.1 - t.1);
    sources
        .iter()
        .map(|s| {
            let f = spine_frame(rig(s[&Joint::Pelvis]), rig(s[&Joint::Neck]));
            to_local(&f, rig(s[&Joint::ShoulderNear]))
        })
        .collect()
}

#[test]
fn the_refined_marks_are_steadier_than_prototype_01() {
    let history: History = ron::from_str(&read(&stage().join("history/rig_lab_01.ron")))
        .unwrap_or_else(|e| panic!("{e}"));
    let old = &history.clips["run"];
    let rig = |t: (f32, f32)| (t.0 - old.pivot.0, old.pivot.1 - t.1);
    // 01: the blade was perpendicular to the forearm.
    let old_angles: Vec<f32> = old
        .sources
        .iter()
        .map(|s| {
            let (e, h) = (rig(s[&Joint::ElbowNear]), rig(s[&Joint::HandNear]));
            (h.1 - e.1).atan2(h.0 - e.0) + std::f32::consts::FRAC_PI_2
        })
        .collect();
    let lab = lab();
    let run = &lab.clips["run"];
    let rest = fit(&lab
        .rest()
        .and_then(|c| c.frame_joints(0))
        .unwrap_or_else(|| panic!("rest")));
    let grip = lab.grip("run").unwrap_or_else(|| panic!("grip"));
    let new_angles: Vec<f32> = (0..run.sources.len())
        .map(|i| {
            let j = run.frame_joints(i).unwrap_or_else(|| panic!("j"));
            hand_frame(HandSide::Near, &j, &forward(&fit(&j)), &rest, grip).angle
        })
        .collect();
    let (sh01, sh02) = (
        shoulders(&old.sources, old.pivot),
        shoulders(&run.sources, run.pivot),
    );
    // Spread: how far the shoulder travels (02 follows the arm swing on
    // purpose); jerk: the jitter that smooth motion does not explain.
    let (s01, s02) = (spread(&sh01), spread(&sh02));
    let (k01, k02) = (loop_jerk(&sh01), loop_jerk(&sh02));
    let (j01, j02) = (
        loop_angle_jerk(&old_angles).to_degrees(),
        loop_angle_jerk(&new_angles).to_degrees(),
    );
    println!(
        "shoulder in the spine frame: spread 01 {s01:.2} -> 02 {s02:.2} texels, jerk 01 {k01:.2} -> 02 {k02:.2} texels; sword angle jerk: 01 {j01:.1} -> 02 {j02:.1} degrees"
    );
    assert!(k02 < k01, "shoulder jitter {k01} -> {k02}");
    assert!(j02 < j01 / 2.0);
}

#[test]
fn the_pauldron_sits_on_the_shoulder_in_every_frame() {
    let lab = lab();
    let rest = fit(&lab
        .rest()
        .and_then(|c| c.frame_joints(0))
        .unwrap_or_else(|| panic!("rest")));
    let pauldron = &lab.items["pauldron_steel"];
    for clip in lab.clips.values() {
        for frame in 0..clip.frames.len() {
            let j = clip.frame_joints(frame).unwrap_or_else(|| panic!("j"));
            let p = place_on_bone(&pauldron.mount, pauldron.depth, &forward(&fit(&j)), &rest)
                .unwrap_or_else(|| panic!("bone mount"));
            let s = j.get(Joint::ShoulderNear);
            assert!((p.position.0 - s.0).abs() < 1e-3 && (p.position.1 - s.1).abs() < 1e-3);
        }
    }
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
                loadout.equip(RigSlot::Chest, "plate_cuirass");
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
        elapsed += 1.0 / 15.0 + 1e-6;
    }
}

#[test]
fn the_articulated_pose_reaches_every_ankle_within_the_rest_leg() {
    let lab = lab();
    let rest = fit(&lab
        .rest()
        .and_then(|c| c.frame_joints(0))
        .unwrap_or_else(|| panic!("rest")));
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
                let reach = (end.0 - hip.0).hypot(end.1 - hip.1);
                assert!((reach - leg).abs() < 0.1, "frame {frame} {ankle:?}");
                clamped.push((frame, ankle, wanted - leg));
            }
        }
    }
    // Measured limitation: the approved run draws some legs longer than the
    // idle leg. Recorded, not hidden.
    println!("clamped ankles (frame, joint, texels beyond the rest leg): {clamped:?}");
    assert!(clamped.len() <= 4, "{clamped:?}");
}
