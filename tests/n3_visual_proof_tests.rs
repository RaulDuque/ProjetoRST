//! Contract tests for the isolated N3 proof. They do not boot the game.

use std::path::PathBuf;
use std::process::Command;

use dark_castle_rpg::player::visual::manifest::AnimationManifest;
use dark_castle_rpg::player::visual::sockets::{SocketTable, place};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn generated() -> PathBuf {
    root().join("art-src/n3-proof/generated")
}

fn read(name: &str) -> String {
    std::fs::read_to_string(generated().join(name))
        .unwrap_or_else(|err| panic!("read {name}: {err}"))
}

fn manifest() -> AnimationManifest {
    AnimationManifest::from_ron(&read("proof.ron")).unwrap_or_else(|err| panic!("{err}"))
}

fn png_size(name: &str) -> (u32, u32) {
    let bytes = std::fs::read(generated().join(name)).unwrap_or_else(|err| panic!("{err}"));
    assert!(
        bytes.starts_with(b"\x89PNG\r\n\x1a\n"),
        "{name} is not a png"
    );
    let width = u32::from_be_bytes(bytes[16..20].try_into().unwrap_or_else(|_| panic!("width")));
    let height = u32::from_be_bytes(
        bytes[20..24]
            .try_into()
            .unwrap_or_else(|_| panic!("height")),
    );
    (width, height)
}

#[test]
fn the_proof_manifest_has_the_four_clips_and_no_baked_weapon() {
    let manifest = manifest();
    assert!(!manifest.weapon_in_body);
    for name in ["idle", "walk", "jump", "attack"] {
        assert!(manifest.clips.contains_key(name), "{name}");
    }
    assert_eq!(
        manifest.atlases.len(),
        1,
        "one body atlas, not one sheet per loadout"
    );
    let body = manifest
        .atlases
        .get("body")
        .unwrap_or_else(|| panic!("body"));
    assert_eq!(body.cell, (96, 128));
    assert_eq!((body.columns, body.rows), (6, 4));
    let (width, height) = png_size("body_light.png");
    assert_eq!(
        (width, height),
        (body.cell.0 * body.columns, body.cell.1 * body.rows)
    );
    assert_eq!(png_size("body_heavy.png"), (width, height));
    for (name, clip) in &manifest.clips {
        assert!(
            clip.pivot.0 >= 0.0 && clip.pivot.0 < body.cell.0 as f32,
            "{name}"
        );
        assert!(
            clip.pivot.1 >= 0.0 && clip.pivot.1 < body.cell.1 as f32,
            "{name}"
        );
        assert_eq!(clip.durations.len(), clip.count as usize, "{name}");
    }
}

#[test]
fn sockets_match_the_clips_and_the_attack_hand_moves() {
    let manifest = manifest();
    let sockets = SocketTable::from_ron(&read("sockets.ron"), &manifest)
        .unwrap_or_else(|err| panic!("{err}"));
    let attack = sockets
        .clips
        .get("attack")
        .unwrap_or_else(|| panic!("attack"));
    assert_eq!(attack.len(), 6);
    let windup = attack.first().unwrap_or_else(|| panic!("windup"));
    let strike = attack.get(2).unwrap_or_else(|| panic!("strike"));
    assert!(
        (strike.weapon.x - windup.weapon.x).abs() > 5.0,
        "the weapon socket stays with the hand, windup x {} strike x {}",
        windup.weapon.x,
        strike.weapon.x
    );
    assert!(!windup.weapon.front, "windup draws the weapon behind");
    assert!(strike.weapon.front, "the strike draws the weapon in front");
    let (mirrored_x, _, mirrored_angle) =
        place(strike.weapon.x, strike.weapon.y, strike.weapon.angle, -1.0);
    assert!(mirrored_x < 0.0, "facing left mirrors the socket");
    let expected = -strike.weapon.angle.to_radians();
    assert!((mirrored_angle - expected).abs() < 0.001);
}

#[test]
fn equipment_is_separate_files_and_the_weapon_color_is_not_in_the_body() {
    let light =
        std::fs::read(generated().join("body_light.png")).unwrap_or_else(|err| panic!("{err}"));
    let heavy =
        std::fs::read(generated().join("body_heavy.png")).unwrap_or_else(|err| panic!("{err}"));
    assert_ne!(light, heavy, "the heavy armor is a different drawing");
    assert!(generated().join("cape.png").is_file());
    assert!(generated().join("sword.png").is_file());
    assert!(generated().join("spear.png").is_file());
    assert_ne!(png_size("sword.png"), png_size("spear.png"));
    let output = Command::new("python3")
        .arg("-c")
        .arg(
            r#"
from pathlib import Path
from PIL import Image
root = Path("art-src/n3-proof/generated")
gold = (232, 176, 64, 255)
def count(name):
    return sum(pixel == gold for pixel in Image.open(root / name).getdata())
body = count("body_light.png") + count("body_heavy.png")
weapons = count("sword.png") + count("spear.png")
print(f"{body} {weapons}")
"#,
        )
        .current_dir(root())
        .output()
        .unwrap_or_else(|err| panic!("python: {err}"));
    assert!(
        output.status.success(),
        "python audit failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let line = String::from_utf8_lossy(&output.stdout);
    let mut parts = line.split_whitespace();
    let body: u32 = parts
        .next()
        .unwrap_or_else(|| panic!("{line}"))
        .parse()
        .unwrap_or_else(|err| panic!("{err}"));
    let weapons: u32 = parts
        .next()
        .unwrap_or_else(|| panic!("{line}"))
        .parse()
        .unwrap_or_else(|err| panic!("{err}"));
    assert_eq!(body, 0, "gold belongs to the weapon layer");
    assert!(weapons > 0);
}

#[test]
fn the_original_mesh_was_not_rewritten() {
    let path = root().join("Md-usario/assets-user/prota/base/white_mesh.glb");
    let meta = std::fs::metadata(&path).unwrap_or_else(|err| panic!("{err}"));
    assert_eq!(meta.len(), 3_387_148);
}
