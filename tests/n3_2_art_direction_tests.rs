//! The N3.2 concept sheet stays a reference, not a modular attack clip.

use std::path::PathBuf;

use dark_castle_rpg::player::visual::manifest::AnimationManifest;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn png_size(path: &std::path::Path) -> (u32, u32) {
    let bytes = std::fs::read(path).unwrap_or_else(|err| panic!("{err}"));
    assert!(bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
    let width = u32::from_be_bytes(bytes[16..20].try_into().unwrap_or_else(|_| panic!("width")));
    let height = u32::from_be_bytes(
        bytes[20..24]
            .try_into()
            .unwrap_or_else(|_| panic!("height")),
    );
    (width, height)
}

#[test]
fn the_concept_sheet_is_one_frame_and_not_a_separate_weapon() {
    let dir = root().join("art-src/n3-2-art-direction/generated");
    let text =
        std::fs::read_to_string(dir.join("concept_b.ron")).unwrap_or_else(|err| panic!("{err}"));
    let manifest = AnimationManifest::from_ron(&text).unwrap_or_else(|err| panic!("{err}"));
    assert!(manifest.weapon_in_body);
    let idle = manifest.clips.get("idle").unwrap_or_else(|| panic!("idle"));
    assert_eq!(idle.count, 1);
    let atlas = manifest
        .atlases
        .get("concept")
        .unwrap_or_else(|| panic!("atlas"));
    assert_eq!(atlas.cell.1, 100);
    assert_eq!(png_size(&dir.join("concept_b_sprite.png")), atlas.cell);
    assert!(!dir.join("sword.png").exists());
    assert!(!dir.join("cape.png").exists());
}

#[test]
fn the_generated_attack_poses_do_not_share_a_frame() {
    let text =
        std::fs::read_to_string(root().join("art-src/n3-2-art-direction/generated/metrics.json"))
            .unwrap_or_else(|err| panic!("{err}"));
    let widths: Vec<u64> = ["b_anticipation", "b_impact", "b_recovery"]
        .into_iter()
        .map(|name| width_after(&text, name))
        .collect();
    let spread = widths.iter().max().unwrap_or_else(|| panic!("max"))
        - widths.iter().min().unwrap_or_else(|| panic!("min"));
    assert!(
        spread > 100,
        "the pose frames should not share a width, spread {spread}"
    );
}

fn width_after(text: &str, name: &str) -> u64 {
    let key = format!("\"{name}\"");
    let start = text.find(&key).unwrap_or_else(|| panic!("{name}"));
    let rest = &text[start..];
    let marker = "\"width\": ";
    let width_at = rest
        .find(marker)
        .unwrap_or_else(|| panic!("width of {name}"));
    let digits: String = rest[width_at + marker.len()..]
        .chars()
        .take_while(|character| character.is_ascii_digit())
        .collect();
    digits.parse().unwrap_or_else(|err| panic!("{err}"))
}
