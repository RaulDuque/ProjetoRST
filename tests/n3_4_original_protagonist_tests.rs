//! The N3.4 base body: one idle frame, no weapon or cape, drawn from an
//! editable 80x128 text grid that keeps the N3.3 canvas and pivot.

use std::path::PathBuf;

use dark_castle_rpg::player::visual::manifest::AnimationManifest;

const CANVAS: (usize, usize) = (80, 128);
const TOP_ROW: usize = 13;
const SOLE_ROW: usize = 118;
const PIVOT_X: usize = 40;
/// Keys of `art-src/n3-4-original-protagonist/scripts/n34_palette.py` plus `.`.
const PALETTE_KEYS: &str = ".KWwhHSszeGgqPpoLldM";

fn dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("art-src/n3-4-original-protagonist")
}

fn png_size(path: &std::path::Path) -> (u32, u32) {
    let bytes = std::fs::read(path).unwrap_or_else(|err| panic!("{err}"));
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

fn opaque(row: &[char]) -> Vec<usize> {
    row.iter()
        .enumerate()
        .filter(|(_, key)| **key != '.')
        .map(|(x, _)| x)
        .collect()
}

#[test]
fn the_base_is_one_idle_frame_without_equipment() {
    let text = std::fs::read_to_string(dir().join("generated/base.ron"))
        .unwrap_or_else(|err| panic!("{err}"));
    let manifest = AnimationManifest::from_ron(&text).unwrap_or_else(|err| panic!("{err}"));
    assert!(!manifest.weapon_in_body);
    assert_eq!(manifest.clips.len(), 1);
    let idle = manifest.clips.get("idle").unwrap_or_else(|| panic!("idle"));
    assert_eq!(idle.count, 1);
    assert_eq!(idle.pivot, (PIVOT_X as f32, SOLE_ROW as f32));
    let atlas = manifest
        .atlases
        .get("body")
        .unwrap_or_else(|| panic!("body"));
    assert_eq!(atlas.cell, (CANVAS.0 as u32, CANVAS.1 as u32));
    assert_eq!(png_size(&dir().join("generated/base_idle.png")), atlas.cell);
    assert_eq!(manifest.atlases.len(), 1, "no cape or weapon atlas");
}

#[test]
fn the_source_grid_keeps_the_n3_3_canvas_and_pivot() {
    let rows = grid("base_idle.txt");
    let filled: Vec<usize> = (0..CANVAS.1)
        .filter(|&y| !opaque(&rows[y]).is_empty())
        .collect();
    assert_eq!(filled.first(), Some(&TOP_ROW));
    assert_eq!(filled.last(), Some(&SOLE_ROW));
    assert_eq!(SOLE_ROW - TOP_ROW + 1, 106);
    let sole = opaque(&rows[SOLE_ROW]);
    let (first, last) = (sole[0], sole[sole.len() - 1]);
    assert!((first..=last).contains(&PIVOT_X), "pivot over the feet");
    // Metal and eye accents are placed by hand, never by the reduction.
    let draft = grid("base_idle_draft.txt");
    let accents = |g: &[Vec<char>]| {
        g.iter()
            .flatten()
            .filter(|k| matches!(k, 'M' | 'e'))
            .count()
    };
    assert_eq!(accents(&draft), 0);
    assert!(accents(&rows) > 0);
    assert_ne!(
        rows, draft,
        "the sprite is the corrected grid, not the draft"
    );
}
