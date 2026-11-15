//! V2.1.1: Save v3 (economy) — format, migrations from v1 and v2, backups,
//! invalid files, and loads that never write. Every file lives in its own
//! directory under the target dir; `saves/` is never used.
//! The game-side flows (buy/sell, F5/F9, stock) are in `town_app_tests.rs`
//! and `world_town_route_tests.rs`.

use std::path::{Path, PathBuf};

use dark_castle_rpg::core::resources::GameData;
use dark_castle_rpg::economy::shop::{EconomyRecord, Purse, ShopLibrary, ShopStock, buy};
use dark_castle_rpg::inventory::equipment::EquipmentSlot;
use dark_castle_rpg::npc::definitions::DialogueFacts;
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::save::serialization::{
    CURRENT_SAVE_VERSION, SaveError, SaveFileV1, SaveFileV2, decode_versioned, encode, upgrade_v1,
    upgrade_v2,
};
use dark_castle_rpg::world::graph::{Ability, Grant};

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn fixture(name: &str) -> String {
    std::fs::read_to_string(root().join("tests/fixtures").join(name))
        .unwrap_or_else(|e| panic!("{e}"))
}

/// A fresh directory under the target dir (never `saves/`).
fn temp_dir(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("save_v3_{name}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap_or_else(|e| panic!("{e}"));
    assert!(!dir.starts_with(root().join("saves")));
    dir
}

/// An economy with coins and one sold-out entry, built by real purchases.
fn economy() -> EconomyRecord {
    let data = GameData::load(root()).unwrap_or_else(|e| panic!("{e}"));
    let shops: &ShopLibrary = &data.shops;
    let shop = shops
        .get("hessa_outfitters")
        .unwrap_or_else(|| panic!("shop"));
    let mut stock = ShopStock::default();
    let mut purse = Purse::new(500);
    let mut bag = Default::default();
    buy(
        shop,
        "watch_buckler",
        &mut stock,
        &mut purse,
        &mut bag,
        &data.items,
        &DialogueFacts::default(),
    )
    .unwrap_or_else(|e| panic!("{e}"));
    EconomyRecord {
        coins: purse.coins(),
        stock,
    }
}

#[test]
fn the_current_format_is_v3() {
    assert_eq!(CURRENT_SAVE_VERSION, 3);
}

#[test]
fn v1_loads_and_migrates_to_v3_keeping_every_field() {
    let text = fixture("save_v1_entry_hall.ron");
    let v1: SaveFileV1 = ron::from_str(&text).unwrap_or_else(|e| panic!("{e}"));
    let decoded = decode_versioned(&text).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(decoded.source_version, 1);
    assert!(decoded.migrated());
    assert_eq!(decoded.data, upgrade_v1(v1.data.clone()));
    assert_eq!(
        decoded.data.player, v1.data.player,
        "inventory, equipment, progression"
    );
    assert_eq!(decoded.data.world, v1.data.world);
    assert_eq!(decoded.data.play_time, v1.data.play_time);
    assert_eq!(decoded.data.economy, None, "no coins invented in the file");
}

#[test]
fn v2_loads_and_migrates_to_v3_keeping_every_field() {
    let text = fixture("save_v2_galleries.ron");
    let v2: SaveFileV2 = ron::from_str(&text).unwrap_or_else(|e| panic!("{e}"));
    let decoded = decode_versioned(&text).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(decoded.source_version, 2);
    assert!(decoded.migrated());
    let data = decoded.data;
    assert_eq!(data, upgrade_v2(v2.data.clone()));
    // Inventory and equipment.
    assert_eq!(data.player.inventory, v2.data.player.inventory);
    assert_eq!(data.player.equipment, v2.data.player.equipment);
    assert!(data.player.equipment.get(EquipmentSlot::Weapon).is_some());
    // Relics, exploration, persistent interactions and position.
    assert!(
        data.exploration
            .granted
            .contains(&Grant::Ability(Ability::AstralKey))
    );
    assert_eq!(data.exploration, v2.data.exploration);
    assert!(data.exploration.secrets.contains("fg_cracked_wall"));
    assert!(data.exploration.pickups.contains("fg_bridge_chest"));
    assert!(
        data.exploration
            .granted
            .contains(&Grant::Flag("fg_trapdoor_open".into()))
    );
    assert_eq!(data.player.position, v2.data.player.position);
    assert_eq!(data.player.progression, v2.data.player.progression);
    assert_eq!(data.world, v2.data.world);
    assert_eq!(data.economy, None);
}

#[test]
fn v3_round_trips_with_coins_and_sold_out_stock() {
    let text = fixture("save_v2_galleries.ron");
    let mut data = decode_versioned(&text)
        .unwrap_or_else(|e| panic!("{e}"))
        .data;
    let record = economy();
    assert_eq!(record.coins, 410);
    data.economy = Some(record.clone());
    let v3 = encode(&data).unwrap_or_else(|e| panic!("{e}"));
    assert!(v3.contains("version: 3"));
    assert!(v3.contains("economy"));
    let back = decode_versioned(&v3).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(back.source_version, 3);
    assert!(!back.migrated());
    assert_eq!(back.data, data);
    assert_eq!(back.data.economy, Some(record));
}

#[test]
fn invalid_and_unknown_versions_are_refused_without_touching_the_file() {
    let dir = temp_dir("invalid");
    let storage = SaveStorage::new(&dir);
    let path = storage.slot_path(1);
    let v2 = fixture("save_v2_galleries.ron");
    let cases = [
        ("garbage", "not a save at all".to_owned(), "corrupted"),
        (
            "future",
            v2.replacen("version: 2", "version: 99", 1),
            "too new",
        ),
        (
            "ancient",
            v2.replacen("version: 2", "version: 0", 1),
            "too old",
        ),
        // A v2 with a field v2 never had: refused, not silently truncated.
        (
            "extra",
            v2.replacen("play_time:", "gold: 5,\n        play_time:", 1),
            "corrupted",
        ),
        // A v3 whose economy has an unknown field.
        (
            "bad_v3",
            "(version: 3, data: (economy: (coins: 1, gems: 2)))".to_owned(),
            "corrupted",
        ),
    ];
    for (name, text, kind) in cases {
        std::fs::write(&path, &text).unwrap_or_else(|e| panic!("{e}"));
        let result = storage.load_versioned(1);
        let ok = matches!(
            (kind, &result),
            ("corrupted", Err(SaveError::Corrupted(_)))
                | ("too new", Err(SaveError::TooNew { found: 99, .. }))
                | ("too old", Err(SaveError::TooOld { .. }))
        );
        assert!(ok, "{name}: {result:?}");
        let after = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(after, text, "{name}: the file was modified");
    }
}

#[test]
fn loading_never_writes_any_file() {
    let dir = temp_dir("readonly");
    let storage = SaveStorage::new(&dir);
    let v2 = fixture("save_v2_galleries.ron");
    std::fs::write(storage.slot_path(1), &v2).unwrap_or_else(|e| panic!("{e}"));
    let listing = |d: &Path| {
        let mut names: Vec<String> = std::fs::read_dir(d)
            .unwrap_or_else(|e| panic!("{e}"))
            .filter_map(Result::ok)
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        names
    };
    let before = listing(&dir);
    for _ in 0..3 {
        storage.load_versioned(1).unwrap_or_else(|e| panic!("{e}"));
    }
    assert_eq!(listing(&dir), before, "no file created");
    assert_eq!(
        std::fs::read_to_string(storage.slot_path(1)).unwrap_or_default(),
        v2
    );
}

#[test]
fn the_first_v3_save_keeps_the_v2_file_and_never_replaces_old_backups() {
    let dir = temp_dir("backups");
    let storage = SaveStorage::new(&dir);
    let v2 = fixture("save_v2_galleries.ron");
    let v1 = fixture("save_v1_entry_hall.ron");
    std::fs::write(storage.slot_path(1), &v2).unwrap_or_else(|e| panic!("{e}"));
    // An older permanent backup that must survive untouched.
    std::fs::write(storage.version_backup_path(1, 1), &v1).unwrap_or_else(|e| panic!("{e}"));
    let mut data = storage.load(1).unwrap_or_else(|e| panic!("{e}"));
    data.economy = Some(economy());
    storage.save(1, &data).unwrap_or_else(|e| panic!("{e}"));
    let read = |p: PathBuf| std::fs::read_to_string(p).unwrap_or_else(|e| panic!("{e}"));
    // The v2 file is kept permanently and in the rolling backup.
    assert_eq!(read(storage.version_backup_path(1, 2)), v2);
    assert_eq!(read(storage.backup_path(1)), v2);
    assert_eq!(read(storage.version_backup_path(1, 1)), v1);
    // The slot is v3 now, with the economy.
    let saved = storage.load_versioned(1).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(saved.source_version, 3);
    assert_eq!(saved.data.economy, data.economy);
    // A second save rolls `.bak` to the first v3 but keeps the v2 copy.
    data.play_time += 10.0;
    storage.save(1, &data).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(read(storage.version_backup_path(1, 2)), v2);
    let rolled = read(storage.backup_path(1));
    assert!(rolled.contains("version: 3"));
    // No temporary file is left behind.
    assert!(!storage.slot_path(1).with_extension("ron.tmp").exists());
}

#[test]
fn coins_above_the_limit_in_a_file_are_clamped_not_overflowed() {
    assert_eq!(
        Purse::new(u32::MAX).coins(),
        dark_castle_rpg::economy::shop::MAX_COINS
    );
}
