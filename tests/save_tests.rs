//! Save format round trip, versioning, corruption handling and backups.

use std::path::PathBuf;

use dark_castle_rpg::inventory::equipment::Equipment;
use dark_castle_rpg::inventory::inventory::Inventory;
use dark_castle_rpg::player::attributes::BaseAttributes;
use dark_castle_rpg::player::progression::Progression;
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::save::serialization::{
    CURRENT_SAVE_VERSION, PlayerSave, SaveData, SaveError, decode, encode,
};
use dark_castle_rpg::skills::unlock::LearnedSkills;
use dark_castle_rpg::world::progress::WorldProgress;

fn sample(level: u32) -> SaveData {
    let mut world = WorldProgress::default();
    world.unlocked_doors.insert("entry_hall/east_gate".into());
    world.set_flag("met_the_archivist");
    world.last_save_point = Some("entry_hall/altar".into());
    SaveData {
        player: PlayerSave {
            room: "entry_hall".into(),
            position: (120.5, 64.0),
            attributes: BaseAttributes::default(),
            progression: Progression::from_parts(level, 40, 400, 2),
            hp: 80,
            mp: 30,
            hearts: 12,
            learned_skills: LearnedSkills::default(),
            inventory: Inventory::default(),
            equipment: Equipment::default(),
        },
        world,
        exploration: Default::default(),
        play_time: 321.0,
        // V2.1.1 (save v3): the economy.
        economy: Some(Default::default()),
    }
}

/// A fresh, unique directory under the target dir (no extra crates needed).
fn temp_dir(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

#[test]
fn round_trip_preserves_everything() {
    let data = sample(4);
    let text = encode(&data).unwrap_or_else(|e| panic!("{e}"));
    assert!(text.contains(&format!("version: {CURRENT_SAVE_VERSION}")));
    let back = decode(&text).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(back, data);
}

#[test]
fn corrupted_file_is_reported() {
    assert!(matches!(
        decode("this is not ron"),
        Err(SaveError::Corrupted(_))
    ));
    let mut text = encode(&sample(1)).unwrap_or_else(|e| panic!("{e}"));
    text.truncate(text.len() / 2);
    assert!(matches!(decode(&text), Err(SaveError::Corrupted(_))));
}

#[test]
fn newer_version_is_rejected() {
    let text = encode(&sample(1)).unwrap_or_else(|e| panic!("{e}"));
    let future = text.replacen(
        &format!("version: {CURRENT_SAVE_VERSION}"),
        &format!("version: {}", CURRENT_SAVE_VERSION + 1),
        1,
    );
    assert!(matches!(decode(&future), Err(SaveError::TooNew { .. })));
}

#[test]
fn older_unsupported_version_is_rejected() {
    let text = encode(&sample(1)).unwrap_or_else(|e| panic!("{e}"));
    let ancient = text.replacen(&format!("version: {CURRENT_SAVE_VERSION}"), "version: 0", 1);
    assert!(matches!(decode(&ancient), Err(SaveError::TooOld { .. })));
}

#[test]
fn storage_keeps_a_backup_of_the_previous_save() {
    let storage = SaveStorage::new(temp_dir("save_backup"));
    assert!(matches!(storage.load(1), Err(SaveError::NotFound(_))));

    storage
        .save(1, &sample(2))
        .unwrap_or_else(|e| panic!("{e}"));
    storage
        .save(1, &sample(3))
        .unwrap_or_else(|e| panic!("{e}"));

    let current = storage.load(1).unwrap_or_else(|e| panic!("{e}"));
    let backup = storage.load_backup(1).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(current.player.progression.level(), 3);
    assert_eq!(backup.player.progression.level(), 2);
}

#[test]
fn corrupted_slot_is_not_overwritten_by_loading() {
    let storage = SaveStorage::new(temp_dir("save_corrupt"));
    std::fs::create_dir_all(storage.dir()).unwrap_or_else(|e| panic!("{e}"));
    std::fs::write(storage.slot_path(1), "garbage").unwrap_or_else(|e| panic!("{e}"));
    assert!(matches!(storage.load(1), Err(SaveError::Corrupted(_))));
    let still_there = std::fs::read_to_string(storage.slot_path(1)).unwrap_or_default();
    assert_eq!(still_there, "garbage");
}

// ------------------------------------------------------------- V1.7 (v2)

fn v1_fixture() -> String {
    std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/save_v1_entry_hall.ron"),
    )
    .unwrap_or_else(|e| panic!("{e}"))
}

#[test]
fn v1_migrates_to_v2_keeping_every_field() {
    use dark_castle_rpg::save::serialization::{SaveFileV1, decode_versioned, upgrade_v1};
    let text = v1_fixture();
    let v1: SaveFileV1 = ron::from_str(&text).unwrap_or_else(|e| panic!("{e}"));
    let decoded = decode_versioned(&text).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(decoded.source_version, 1);
    let expected = upgrade_v1(v1.data.clone());
    assert_eq!(decoded.data, expected);
    assert_eq!(decoded.data.player, v1.data.player);
    assert_eq!(decoded.data.world, v1.data.world);
    assert_eq!(decoded.data.play_time, v1.data.play_time);
    // Re-encoding writes the current version and reads back identically.
    let v2 = encode(&decoded.data).unwrap_or_else(|e| panic!("{e}"));
    assert!(v2.contains(&format!("version: {CURRENT_SAVE_VERSION}")));
    assert_eq!(decode(&v2).unwrap_or_else(|e| panic!("{e}")), decoded.data);
}

#[test]
fn v1_with_unknown_fields_is_refused_not_truncated() {
    let text = v1_fixture().replacen("play_time: 42.5,", "play_time: 42.5,\n        gold: 99,", 1);
    assert!(text.contains("gold"));
    assert!(matches!(decode(&text), Err(SaveError::Corrupted(_))));
}

#[test]
fn loading_never_writes_and_saving_keeps_the_old_format() {
    let storage = SaveStorage::new(temp_dir("save_v1_keep"));
    std::fs::create_dir_all(storage.dir()).unwrap_or_else(|e| panic!("{e}"));
    let v1 = v1_fixture();
    std::fs::write(storage.slot_path(1), &v1).unwrap_or_else(|e| panic!("{e}"));
    let loaded = storage.load_versioned(1).unwrap_or_else(|e| panic!("{e}"));
    assert!(loaded.migrated());
    assert_eq!(
        std::fs::read_to_string(storage.slot_path(1)).unwrap_or_default(),
        v1
    );
    assert!(!storage.version_backup_path(1, 1).exists());
    storage
        .save(1, &loaded.data)
        .unwrap_or_else(|e| panic!("{e}"));
    storage
        .save(1, &sample(5))
        .unwrap_or_else(|e| panic!("{e}"));
    let kept = std::fs::read_to_string(storage.version_backup_path(1, 1)).unwrap_or_default();
    assert_eq!(kept, v1);
    assert_eq!(
        storage
            .load(1)
            .unwrap_or_else(|e| panic!("{e}"))
            .player
            .progression
            .level(),
        5
    );
}

// ------------------------------------------------------------- V1.8

#[test]
fn a_v2_save_with_objects_reads_as_is_and_round_trips() {
    use dark_castle_rpg::save::serialization::decode_versioned;
    use dark_castle_rpg::world::graph::{Ability, Grant};
    let text = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/save_v2_galleries.ron"),
    )
    .unwrap_or_else(|e| panic!("{e}"));
    let decoded = decode_versioned(&text).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(decoded.source_version, 2);
    // V2.1.1: v2 is now an older format, converted in memory to v3.
    assert!(decoded.migrated());
    assert_eq!(decoded.data.economy, None);
    let data = decoded.data;
    // Objects are stored in the v2 fields, no new schema (V1.8):
    // broken wall → secrets, opened chest → pickups, pulled lever → granted.
    assert!(data.exploration.secrets.contains("fg_cracked_wall"));
    assert!(data.exploration.pickups.contains("fg_bridge_chest"));
    assert!(
        data.exploration
            .granted
            .contains(&Grant::Flag("fg_trapdoor_open".into()))
    );
    assert!(
        data.exploration
            .granted
            .contains(&Grant::Ability(Ability::AstralKey))
    );
    assert_eq!(data.player.inventory.count("warden_greaves"), 1);
    let again = encode(&data).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(decode(&again).unwrap_or_else(|e| panic!("{e}")), data);
}
