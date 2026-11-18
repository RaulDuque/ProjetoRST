//! The N3.6 run proof: frames reduced from the owner's run video, one
//! manifest with a stand frame and two run cycles on a 112x128 canvas, and
//! the N3.5 candidate still loadable beside it.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use dark_castle_rpg::player::visual::manifest::{AnimationManifest, ClipMode};

const CANVAS: (usize, usize) = (112, 128);
const PIVOT: (f32, f32) = (56.0, 118.0);
const SOLE_ROW: usize = 118;
/// Keys of `art-src/n3-4-original-protagonist/scripts/n34_palette.py` plus `.`.
const PALETTE_KEYS: &str = ".KWwhHSszeGgqPpoLldM";
const RUN6: [u32; 6] = [48, 50, 52, 54, 56, 58];
const RUN10: [u32; 10] = [48, 50, 51, 52, 53, 54, 56, 57, 58, 59];
/// One video cycle: 12 frames at 24 fps.
const CYCLE_SECONDS: f32 = 0.5;

fn dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("art-src/n3-6-animation-proof")
}

fn png_size(path: &Path) -> (u32, u32) {
    let bytes = std::fs::read(path).unwrap_or_else(|err| panic!("{}: {err}", path.display()));
    assert!(bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
    let word = |at: usize| {
        u32::from_be_bytes(
            bytes[at..at + 4]
                .try_into()
                .unwrap_or_else(|_| panic!("png header")),
        )
    };
    (word(16), word(20))
}

fn manifest() -> AnimationManifest {
    let text = std::fs::read_to_string(dir().join("generated/run.ron"))
        .unwrap_or_else(|err| panic!("run.ron: {err}"));
    AnimationManifest::from_ron(&text).unwrap_or_else(|err| panic!("run.ron: {err}"))
}

fn grid(frame: u32) -> Vec<Vec<char>> {
    let path = dir().join(format!("source/video_{frame:03}.txt"));
    let text = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("{frame}: {err}"));
    let rows: Vec<Vec<char>> = text.lines().map(|line| line.chars().collect()).collect();
    assert_eq!(rows.len(), CANVAS.1, "frame {frame}: rows");
    for (y, row) in rows.iter().enumerate() {
        assert_eq!(row.len(), CANVAS.0, "frame {frame}: row {y}");
        assert!(
            row.iter().all(|key| PALETTE_KEYS.contains(*key)),
            "frame {frame}: row {y} has a key outside the palette"
        );
    }
    rows
}

fn opaque_rows(rows: &[Vec<char>]) -> (usize, usize) {
    let filled: Vec<usize> = (0..rows.len())
        .filter(|&y| rows[y].iter().any(|key| *key != '.'))
        .collect();
    (filled[0], filled[filled.len() - 1])
}

#[test]
fn the_manifest_has_a_stand_frame_and_two_run_cycles() {
    let manifest = manifest();
    assert!(!manifest.weapon_in_body);
    for (name, count) in [("idle", 1), ("run6", 6), ("run10", 10)] {
        let clip = manifest.clips.get(name).unwrap_or_else(|| panic!("{name}"));
        assert_eq!(clip.count, count, "{name}");
        assert_eq!(clip.pivot, PIVOT, "{name}");
        assert!(matches!(clip.mode, ClipMode::Loop { .. }), "{name}");
        let atlas = manifest
            .atlases
            .get(&clip.atlas)
            .unwrap_or_else(|| panic!("{name} atlas"));
        assert_eq!(atlas.cell, (CANVAS.0 as u32, CANVAS.1 as u32));
        assert_eq!(atlas.density, 2);
        assert_eq!(
            png_size(&dir().join("generated").join(&atlas.texture)),
            atlas.image_size(),
            "{name} texture"
        );
    }
}

#[test]
fn both_run_cycles_last_one_video_cycle() {
    let manifest = manifest();
    for name in ["run6", "run10"] {
        let clip = manifest.clips.get(name).unwrap_or_else(|| panic!("{name}"));
        let total = clip.total_duration();
        assert!((total - CYCLE_SECONDS).abs() < 0.001, "{name}: {total}");
    }
    let run6 = manifest.clips.get("run6").unwrap_or_else(|| panic!("run6"));
    assert!(
        run6.durations
            .iter()
            .all(|d| (d - run6.durations[0]).abs() < 1e-6),
        "run6 is evenly timed"
    );
}

#[test]
fn every_kept_frame_shares_canvas_ground_and_head_height() {
    let mut tops = Vec::new();
    let mut touches_ground = 0;
    for frame in RUN10 {
        let rows = grid(frame);
        let (top, bottom) = opaque_rows(&rows);
        assert!(bottom <= SOLE_ROW, "frame {frame} sinks below the ground");
        if bottom == SOLE_ROW {
            touches_ground += 1;
        }
        tops.push(top);
    }
    assert!(touches_ground >= 4, "contact frames reach the pivot row");
    let (low, high) = (
        tops.iter().min().copied().unwrap_or(0),
        tops.iter().max().copied().unwrap_or(0),
    );
    assert!(
        high - low <= 4,
        "head top varies {low}..{high}: not one scale"
    );
    for frame in RUN6 {
        assert!(RUN10.contains(&frame), "run6 frame {frame} is a kept frame");
    }
}

#[test]
fn the_overlay_boxes_match_the_clips() {
    let manifest = manifest();
    let text = std::fs::read_to_string(dir().join("generated/boxes.ron"))
        .unwrap_or_else(|err| panic!("boxes.ron: {err}"));
    let boxes: BTreeMap<String, Vec<(u32, u32, u32, u32)>> =
        ron::from_str(&text).unwrap_or_else(|err| panic!("boxes.ron: {err}"));
    for (name, clip) in &manifest.clips {
        let list = boxes.get(name).unwrap_or_else(|| panic!("{name} boxes"));
        assert_eq!(list.len(), clip.count as usize, "{name}");
        for &(x0, y0, x1, y1) in list {
            assert!(x0 <= x1 && y0 <= y1);
            assert!((x1 as usize) < CANVAS.0 && (y1 as usize) < CANVAS.1);
        }
    }
}

#[test]
fn the_n3_5_candidate_still_loads() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("art-src/n3-5-fidelity/generated/candidate.ron");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("{err}"));
    let manifest = AnimationManifest::from_ron(&text).unwrap_or_else(|err| panic!("{err}"));
    assert_eq!(
        manifest.clips.get("idle").map(|clip| clip.pivot),
        Some((40.0, 118.0))
    );
}
