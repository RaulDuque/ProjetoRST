//! The N3.1 attack sheets stay on the existing manifest contract.
//! They do not boot the game.

use std::path::PathBuf;

use dark_castle_rpg::player::visual::manifest::AnimationManifest;
use dark_castle_rpg::player::visual::sockets::SocketTable;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn dir() -> PathBuf {
    root().join("art-src/n3-proof/n3_1")
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
fn the_rendered_attack_uses_the_existing_manifest_contract() {
    let text =
        std::fs::read_to_string(dir().join("attack.ron")).unwrap_or_else(|err| panic!("{err}"));
    let manifest = AnimationManifest::from_ron(&text).unwrap_or_else(|err| panic!("{err}"));
    assert!(!manifest.weapon_in_body);
    let attack = manifest
        .clips
        .get("attack")
        .unwrap_or_else(|| panic!("attack"));
    assert_eq!(attack.count, 7);
    assert_eq!(attack.pivot, (80.0, 190.0));
    let atlas = manifest
        .atlases
        .get("attack")
        .unwrap_or_else(|| panic!("atlas"));
    assert_eq!(atlas.cell, (256, 192));
    assert_eq!(png_size(&dir().join("attack_pixel.png")), (256 * 7, 192));
    for index in 0..7 {
        let path = dir().join(format!("raw/attack_{index:02}.png"));
        assert_eq!(png_size(&path), (1024, 768), "{path:?}");
    }
}

#[test]
fn the_measured_sockets_follow_the_attack_and_mirror() {
    let text =
        std::fs::read_to_string(dir().join("attack.ron")).unwrap_or_else(|err| panic!("{err}"));
    let manifest = AnimationManifest::from_ron(&text).unwrap_or_else(|err| panic!("{err}"));
    let sockets = SocketTable::from_ron(
        &std::fs::read_to_string(dir().join("sockets.ron")).unwrap_or_else(|err| panic!("{err}")),
        &manifest,
    )
    .unwrap_or_else(|err| panic!("{err}"));
    let attack = sockets
        .clips
        .get("attack")
        .unwrap_or_else(|| panic!("attack"));
    assert_eq!(attack.len(), 7);
    let start = &attack[0].weapon;
    let later = &attack[3].weapon;
    assert!(
        (start.x - later.x).abs() > 1.0 || (start.angle - later.angle).abs() > 5.0,
        "the hand should move across the attack"
    );
    let (mirrored, _, angle) =
        dark_castle_rpg::player::visual::sockets::place(start.x, start.y, start.angle, -1.0);
    assert!(mirrored < 0.0 || start.x < 0.0);
    assert!((angle + start.angle.to_radians()).abs() < 0.001);
}
