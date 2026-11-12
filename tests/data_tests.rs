//! Validates the real data files shipped in `assets/`, so a typo in a RON
//! file fails `cargo test` instead of the game at startup.

use std::path::Path;

use dark_castle_rpg::core::resources::GameData;
use dark_castle_rpg::skills::definitions::Acquisition;

fn data() -> GameData {
    GameData::load(Path::new(env!("CARGO_MANIFEST_DIR")))
        .unwrap_or_else(|e| panic!("game data failed to load: {e}"))
}

#[test]
fn all_data_files_load_and_cross_validate() {
    let data = data();
    assert!(data.rooms.contains_key(&data.character.start.room));
    assert!(!data.enemies.is_empty());
    assert!(!data.skills.is_empty());
}

#[test]
fn prototype_content_is_present() {
    let data = data();
    for id in [
        "crescent_slash",
        "rising_edge",
        "ember_bolt",
        "mending_light",
    ] {
        assert!(data.skills.get(id).is_some(), "missing skill {id}");
    }
    let starting = data
        .skills
        .iter()
        .filter(|s| s.acquisition == Acquisition::Starting)
        .count();
    assert!(starting >= 2);
    assert!(
        data.skills
            .get("ember_bolt")
            .is_some_and(|s| s.category.is_spell())
    );
}

#[test]
fn skill_commands_are_unique_per_button() {
    let data = data();
    let patterns = data.skills.patterns();
    for (i, a) in patterns.iter().enumerate() {
        for b in &patterns[i + 1..] {
            assert_ne!(
                a.sequence, b.sequence,
                "{} and {} share a command",
                a.id, b.id
            );
        }
    }
}
