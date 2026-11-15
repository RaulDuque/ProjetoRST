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

/// Width and height from a PNG header (IHDR), without decoding the image.
fn png_size(path: &Path) -> (u32, u32) {
    let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    assert_eq!(
        &bytes[..8],
        b"\x89PNG\r\n\x1a\n",
        "{} is not a PNG",
        path.display()
    );
    let read =
        |at: usize| u32::from_be_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]]);
    (read(16), read(20))
}

#[test]
fn player_sprite_manifest_matches_its_textures_and_the_contract() {
    use dark_castle_rpg::core::constants::{BASIC_ATTACK_TIME, PLAYER_DODGE_TIME};
    use dark_castle_rpg::player::visual::MANIFEST_PATH;
    use dark_castle_rpg::player::visual::manifest::{AnimationManifest, ClipMode};

    let assets = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
    let text = std::fs::read_to_string(assets.join(MANIFEST_PATH))
        .unwrap_or_else(|e| panic!("player sprite manifest missing: {e}"));
    let manifest = AnimationManifest::from_ron(&text).unwrap_or_else(|e| panic!("{e}"));
    for (id, atlas) in &manifest.atlases {
        let size = png_size(&assets.join(&atlas.texture));
        assert_eq!(
            size,
            atlas.image_size(),
            "atlas {id}: PNG size differs from its grid"
        );
        assert!(size.0 <= 2048 && size.1 <= 2048, "atlas {id} above 2048 px");
    }
    // Contract cells (world units) and sole-line pivots.
    let cell = |clip: &str| manifest.atlases[&manifest.clips[clip].atlas].world_size();
    for clip in ["idle", "run", "dodge"] {
        assert_eq!(cell(clip), (64.0, 64.0), "{clip}: locomotion cell");
        let c = &manifest.clips[clip];
        let d = manifest.atlases[&c.atlas].density as f32;
        assert_eq!(
            (c.pivot.0 / d, c.pivot.1 / d),
            (32.0, 63.0),
            "{clip}: pivot"
        );
    }
    assert_eq!(cell("attack_basic"), (128.0, 96.0), "attack: combat cell");
    let attack = &manifest.clips["attack_basic"];
    let d = manifest.atlases[&attack.atlas].density as f32;
    assert_eq!((attack.pivot.0 / d, attack.pivot.1 / d), (40.0, 95.0));
    // Action clips are authored for the real gameplay durations.
    for (clip, seconds) in [
        ("attack_basic", BASIC_ATTACK_TIME),
        ("dodge", PLAYER_DODGE_TIME),
    ] {
        let c = &manifest.clips[clip];
        assert_eq!(c.mode, ClipMode::FitToAction);
        assert!(
            (c.total_duration() - seconds).abs() < 1e-4,
            "{clip}: {} s",
            c.total_duration()
        );
    }
}

/// V1.4: every skill/spell `animation` key has a clip lasting its execution
/// time, hurt lasts the stagger, and every clip sits in the cell of its group.
#[test]
fn player_sprite_clips_cover_the_skills_and_states() {
    use dark_castle_rpg::core::constants::PLAYER_HURT_TIME;
    use dark_castle_rpg::player::visual::MANIFEST_PATH;
    use dark_castle_rpg::player::visual::manifest::{AnimationManifest, ClipMode};

    let assets = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
    let text = std::fs::read_to_string(assets.join(MANIFEST_PATH))
        .unwrap_or_else(|e| panic!("player sprite manifest missing: {e}"));
    let manifest = AnimationManifest::from_ron(&text).unwrap_or_else(|e| panic!("{e}"));
    let data = data();
    for def in data.skills.iter() {
        let clip = manifest
            .clips
            .get(&def.animation)
            .unwrap_or_else(|| panic!("{}: no clip {:?}", def.id, def.animation));
        assert_eq!(clip.mode, ClipMode::FitToAction, "{}", def.id);
        assert!(
            (clip.total_duration() - def.execution_time).abs() < 1e-4,
            "{}: clip {} s, execution {} s",
            def.id,
            clip.total_duration(),
            def.execution_time
        );
        let atlas = &manifest.atlases[&clip.atlas];
        assert_eq!(atlas.world_size(), (128.0, 96.0), "{}: combat cell", def.id);
    }
    let hurt = &manifest.clips["hurt"];
    assert!((hurt.total_duration() - PLAYER_HURT_TIME).abs() < 1e-4);
    for clip in [
        "hurt",
        "death",
        "jump_rise",
        "fall",
        "land",
        "crouch",
        "guard_start",
    ] {
        let c = &manifest.clips[clip];
        let atlas = &manifest.atlases[&c.atlas];
        assert_eq!(atlas.world_size(), (64.0, 64.0), "{clip}: locomotion cell");
        let d = atlas.density as f32;
        assert_eq!(
            (c.pivot.0 / d, c.pivot.1 / d),
            (32.0, 63.0),
            "{clip}: pivot"
        );
    }
}

/// V1.5: the shipped sprites were published by `publish_guard.py`. Its lock
/// (`assets/animations/player.publish.json`) lists every clip of the manifest
/// and every published file; the guard refuses any later publication that
/// drops one of these clips. The SHA-256 of each file is checked by the
/// guard's own test (`art-src/player/scripts/post/tests`), as the game has no
/// hashing dependency.
#[test]
fn player_sprites_match_the_publication_lock() {
    use std::collections::BTreeMap;

    use dark_castle_rpg::player::visual::MANIFEST_PATH;
    use dark_castle_rpg::player::visual::manifest::AnimationManifest;

    let assets = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
    let manifest = AnimationManifest::from_ron(
        &std::fs::read_to_string(assets.join(MANIFEST_PATH)).unwrap_or_else(|e| panic!("{e}")),
    )
    .unwrap_or_else(|e| panic!("{e}"));
    // JSON objects, arrays, strings and numbers are valid RON values.
    let lock: BTreeMap<String, ron::Value> = ron::from_str(
        &std::fs::read_to_string(assets.join("animations/player.publish.json"))
            .unwrap_or_else(|e| panic!("publication lock missing: {e}")),
    )
    .unwrap_or_else(|e| panic!("publication lock: {e}"));
    let keys = |field: &str| -> Vec<String> {
        let value = lock
            .get(field)
            .cloned()
            .unwrap_or_else(|| panic!("lock without {field}"));
        let map: BTreeMap<String, ron::Value> = value
            .into_rust()
            .unwrap_or_else(|e| panic!("lock {field}: {e}"));
        map.into_keys().collect()
    };
    let clips: Vec<String> = manifest.clips.keys().cloned().collect();
    assert_eq!(
        keys("clips"),
        clips,
        "lock and manifest list different clips"
    );
    let files = keys("files");
    assert!(
        files.iter().any(|f| f == MANIFEST_PATH),
        "manifest not in the lock"
    );
    for atlas in manifest.atlases.values() {
        assert!(
            files.contains(&atlas.texture),
            "{} not in the lock",
            atlas.texture
        );
    }
    assert!(
        !assets
            .join("animations/.player.publish.journal.json")
            .exists(),
        "an interrupted publication is pending (publish_guard.py recover)"
    );
}
