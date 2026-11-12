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
        play_time: 321.0,
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
