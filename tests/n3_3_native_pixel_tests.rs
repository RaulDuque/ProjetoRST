//! The N3.3 sheet is a one-frame construction, not an attack clip.

use std::path::PathBuf;

use dark_castle_rpg::player::visual::manifest::AnimationManifest;
use dark_castle_rpg::player::visual::sockets::{SocketTable, place};

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
fn the_construction_is_one_idle_frame_with_separate_layers() {
    let dir = root().join("art-src/n3-3-native-pixel/generated");
    let text =
        std::fs::read_to_string(dir.join("native.ron")).unwrap_or_else(|err| panic!("{err}"));
    let manifest = AnimationManifest::from_ron(&text).unwrap_or_else(|err| panic!("{err}"));
    assert!(!manifest.weapon_in_body);
    assert!(!manifest.clips.contains_key("attack"));
    let idle = manifest.clips.get("idle").unwrap_or_else(|| panic!("idle"));
    assert_eq!(idle.count, 1);
    assert_eq!(idle.pivot, (40.0, 118.0));
    let atlas = manifest
        .atlases
        .get("body")
        .unwrap_or_else(|| panic!("body"));
    assert_eq!(png_size(&dir.join("body_idle.png")), atlas.cell);
    assert!(dir.join("cape_idle.png").is_file());
    assert!(dir.join("sword.png").is_file());
    let body = std::fs::read(dir.join("body_idle.png")).unwrap_or_else(|err| panic!("{err}"));
    let sword = std::fs::read(dir.join("sword.png")).unwrap_or_else(|err| panic!("{err}"));
    assert_ne!(body, sword);
}

#[test]
fn the_sword_socket_mirrors_and_the_figure_is_near_one_hundred_pixels() {
    let dir = root().join("art-src/n3-3-native-pixel/generated");
    let text =
        std::fs::read_to_string(dir.join("native.ron")).unwrap_or_else(|err| panic!("{err}"));
    let manifest = AnimationManifest::from_ron(&text).unwrap_or_else(|err| panic!("{err}"));
    let sockets = SocketTable::from_ron(
        &std::fs::read_to_string(dir.join("sockets.ron")).unwrap_or_else(|err| panic!("{err}")),
        &manifest,
    )
    .unwrap_or_else(|err| panic!("{err}"));
    let frame = sockets.frame("idle", 0).unwrap_or_else(|| panic!("frame"));
    let (mirrored, _, angle) = place(frame.weapon.x, frame.weapon.y, frame.weapon.angle, -1.0);
    assert!(mirrored < 0.0);
    assert!(angle.abs() < 0.001);
    let metrics =
        std::fs::read_to_string(dir.join("metrics.json")).unwrap_or_else(|err| panic!("{err}"));
    let height = metrics
        .split("\"opaque_height\": ")
        .nth(1)
        .unwrap_or_else(|| panic!("height"))
        .chars()
        .take_while(|character| character.is_ascii_digit())
        .collect::<String>()
        .parse::<u32>()
        .unwrap_or_else(|err| panic!("{err}"));
    assert!((96..=112).contains(&height), "{height}");
}
