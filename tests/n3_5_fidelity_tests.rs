//! The N3.5 fidelity candidate: one idle frame at the N3.4 canvas and pivot,
//! density 2, drawn as the reduction of the owner's sheet with only a few
//! hand edits, and the three resolution hypotheses at their own canvases.

use std::path::{Path, PathBuf};

use dark_castle_rpg::player::visual::manifest::AnimationManifest;

const CANVAS: (usize, usize) = (80, 128);
const TOP_ROW: usize = 13;
const SOLE_ROW: usize = 118;
const PIVOT_X: usize = 40;
/// Keys of `art-src/n3-4-original-protagonist/scripts/n34_palette.py` plus `.`.
const PALETTE_KEYS: &str = ".KWwhHSszeGgqPpoLldM";
/// Upper bound on hand edits: the candidate must stay the reduction.
const MAX_HAND_EDITS: usize = 16;

fn dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("art-src/n3-5-fidelity")
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

fn grid(name: &str) -> Vec<Vec<char>> {
    let text = std::fs::read_to_string(dir().join("source").join(name))
        .unwrap_or_else(|err| panic!("{name}: {err}"));
    let rows: Vec<Vec<char>> = text.lines().map(|line| line.chars().collect()).collect();
    assert_eq!(rows.len(), CANVAS.1, "{name}: rows");
    for (y, row) in rows.iter().enumerate() {
        assert_eq!(row.len(), CANVAS.0, "{name}: row {y} width");
        for key in row {
            assert!(PALETTE_KEYS.contains(*key), "{name}: row {y} key {key:?}");
        }
    }
    rows
}

fn manifest(path: &str) -> AnimationManifest {
    let text = std::fs::read_to_string(path).unwrap_or_else(|err| panic!("{path}: {err}"));
    AnimationManifest::from_ron(&text).unwrap_or_else(|err| panic!("{path}: {err}"))
}

#[test]
fn the_candidate_is_one_idle_frame_at_density_two_without_equipment() {
    let manifest = manifest(&dir().join("generated/candidate.ron").display().to_string());
    assert!(!manifest.weapon_in_body);
    assert_eq!(manifest.clips.len(), 1);
    assert_eq!(manifest.atlases.len(), 1, "no cape or weapon atlas");
    let idle = manifest.clips.get("idle").unwrap_or_else(|| panic!("idle"));
    assert_eq!(idle.count, 1);
    assert_eq!(idle.pivot, (PIVOT_X as f32, SOLE_ROW as f32));
    let atlas = manifest
        .atlases
        .get("body")
        .unwrap_or_else(|| panic!("body"));
    assert_eq!(atlas.cell, (CANVAS.0 as u32, CANVAS.1 as u32));
    assert_eq!(atlas.density, 2);
    assert_eq!(
        png_size(&dir().join("generated/candidate_106.png")),
        atlas.cell
    );
}

#[test]
fn the_candidate_keeps_the_canvas_pivot_and_stays_the_reduction() {
    let rows = grid("candidate_106.txt");
    let filled: Vec<usize> = (0..CANVAS.1)
        .filter(|&y| rows[y].iter().any(|key| *key != '.'))
        .collect();
    assert_eq!(filled.first(), Some(&TOP_ROW));
    assert_eq!(filled.last(), Some(&SOLE_ROW));
    let sole: Vec<usize> = (0..CANVAS.0)
        .filter(|&x| rows[SOLE_ROW][x] != '.')
        .collect();
    assert!(
        (sole[0]..=sole[sole.len() - 1]).contains(&PIVOT_X),
        "pivot over the feet"
    );
    let reduction = grid("reduction_106.txt");
    let edits = rows
        .iter()
        .zip(&reduction)
        .flat_map(|(a, b)| a.iter().zip(b))
        .filter(|(a, b)| a != b)
        .count();
    assert!(edits > 0, "the hand edits are recorded in the grid");
    assert!(
        edits <= MAX_HAND_EDITS,
        "{edits} edits: no longer the reduction"
    );
}

#[test]
fn the_resolution_hypotheses_have_their_own_canvases() {
    for (name, size) in [("106", (80, 128)), ("132", (96, 160)), ("94", (72, 112))] {
        let path = dir().join(format!("generated/reduction_{name}.png"));
        assert_eq!(png_size(&path), size, "{name}");
    }
}

#[test]
fn the_n3_4_base_is_still_loadable_and_unchanged_in_shape() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let base = root.join("art-src/n3-4-original-protagonist/generated");
    let manifest = manifest(&base.join("base.ron").display().to_string());
    let idle = manifest.clips.get("idle").unwrap_or_else(|| panic!("idle"));
    assert_eq!(idle.pivot, (PIVOT_X as f32, SOLE_ROW as f32));
    assert_eq!(png_size(&base.join("base_idle.png")), (80, 128));
}
