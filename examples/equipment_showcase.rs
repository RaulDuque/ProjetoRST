//! V2.0 visual bench of the equipment, in a window, with screenshots.
//!
//! **A bench, not a route:** the items are put in the bag directly (the
//! input-only proof that equipment is found and used in play is
//! `examples/world_equipment_route.rs`). Everything else is the real game:
//! each item is equipped through `EquipRequest` (the inventory's path,
//! with its gate), each strike is a real attack key press, the form is the
//! real `T` key and the inventory is the real screen.
//!
//! Shots in `target/equipment_showcase/`: the five weapon categories (two
//! swords) mid-strike, the hauberk's tint, both cloaks, accessories and
//! the comparison on the inventory screen, the Spectral Form with a cape,
//! and the item refused while transformed. Exit code != 0 if a check fails.
//!
//! Run with: `cargo run --example equipment_showcase` (graphical session).

use std::path::PathBuf;
use std::sync::OnceLock;
use std::time::Duration;

use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::combat::attacks::WeaponStrike;
use dark_castle_rpg::combat::hitboxes::Hitbox;
use dark_castle_rpg::core::events::{EquipOp, EquipRequest};
use dark_castle_rpg::core::resources::{GameData, project_root};
use dark_castle_rpg::inventory::equipment::EquipmentSlot;
use dark_castle_rpg::inventory::visual::CapeLayer;
use dark_castle_rpg::player::components::{Belongings, Player};
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::save::systems::SaveSlots;
use dark_castle_rpg::world::exploration_systems::Exploration;
use dark_castle_rpg::world::graph::{Ability, Grant};

const FRAME: Duration = Duration::from_millis(16);

static VERDICT: OnceLock<bool> = OnceLock::new();

fn out_dir() -> PathBuf {
    project_root().join("target/equipment_showcase")
}

#[derive(Debug, Clone, Copy)]
enum Act {
    Wait(u32),
    Give(&'static [&'static str]),
    Equip(EquipmentSlot, &'static str),
    Unequip(EquipmentSlot),
    Press(KeyCode),
    /// Keeps a key down until `Release`.
    Hold(KeyCode),
    Release(KeyCode),
    Shot(&'static str),
    StrikeWidth(f32),
    Cape(bool),
    Equipped(EquipmentSlot, &'static str),
    Relic,
}

fn script() -> Vec<Act> {
    use Act::*;
    use EquipmentSlot as S;
    let mut acts = vec![
        Wait(60),
        Give(&[
            "duskwater_saber",
            "gutter_knife",
            "wrens_needle",
            "hearthsplitter",
            "grave_hatchet",
            "watch_pike",
            "lantern_glaive",
            "bell_maul",
            "ossuary_greatblade",
            "sentry_coif",
            "chain_hauberk",
            "pilgrim_mantle",
            "emberlined_cloak",
            "wrapped_gauntlets",
            "quiet_soles",
            "watch_buckler",
            "veilwoven_band",
            "still_water_amulet",
        ]),
        Wait(5),
        Shot("v01_starting_iron_sword_idle"),
    ];
    let weapons: [(&str, &str, f32); 6] = [
        ("iron_sword", "v02_strike_sword_iron", 46.0),
        ("duskwater_saber", "v03_strike_sword_duskwater_saber", 50.0),
        ("gutter_knife", "v04_strike_dagger_gutter_knife", 28.0),
        ("hearthsplitter", "v05_strike_axe_hearthsplitter", 50.0),
        ("lantern_glaive", "v06_strike_spear_lantern_glaive", 86.0),
        ("bell_maul", "v07_strike_heavy_bell_maul", 62.0),
    ];
    for (id, shot, width) in weapons {
        // The Iron Sword is already equipped at the start.
        if id != "iron_sword" {
            acts.push(Equip(S::Weapon, id));
        }
        acts.extend([
            Wait(30),
            Press(KeyCode::KeyX),
            Wait(3),
            StrikeWidth(width),
            Shot(shot),
            Wait(50),
        ]);
    }
    acts.extend([
        // V2.1: one-handed, so the buckler below can be held with it.
        Equip(S::Weapon, "duskwater_saber"),
        Equip(S::Body, "chain_hauberk"),
        Equip(S::Head, "sentry_coif"),
        Wait(10),
        Cape(false),
        Shot("v08_chain_hauberk_tint"),
        Equip(S::Body, "pilgrim_mantle"),
        Wait(10),
        Cape(true),
        Shot("v09_pilgrim_mantle_cape"),
        Equip(S::Body, "emberlined_cloak"),
        Wait(10),
        Cape(true),
        Shot("v10_emberlined_cloak_cape"),
        Hold(KeyCode::ArrowRight),
        Wait(25),
        Shot("v11_cloak_running"),
        Release(KeyCode::ArrowRight),
        Wait(40),
        Equip(S::Gloves, "wrapped_gauntlets"),
        Equip(S::Boots, "quiet_soles"),
        Equip(S::Offhand, "watch_buckler"),
        Equip(S::Accessory1, "veilwoven_band"),
        Equip(S::Accessory2, "still_water_amulet"),
        Wait(5),
        Equipped(S::Accessory1, "veilwoven_band"),
        Equipped(S::Accessory2, "still_water_amulet"),
        // The inventory: slots, then the bag; select the Ossuary Greatblade
        // (STR 10 needed) and another weapon for comparisons.
        Press(KeyCode::KeyI),
        Wait(5),
        Shot("v12_inventory_full_loadout"),
    ]);
    for _ in 0..8 {
        acts.extend([Press(KeyCode::ArrowDown), Wait(2)]);
    }
    acts.extend([Wait(3), Shot("v13_compare_first_bag_item")]);
    for _ in 0..7 {
        acts.extend([Press(KeyCode::ArrowDown), Wait(2)]);
    }
    acts.extend([
        Wait(3),
        Shot("v14_compare_further_bag_item"),
        Press(KeyCode::KeyI),
        Wait(5),
        Relic,
        Press(KeyCode::KeyT),
        Wait(20),
        Shot("v15_spectral_form_with_cloak"),
        Equip(S::Body, "pilgrim_mantle"),
        Wait(3),
        Equipped(S::Body, "emberlined_cloak"),
        Press(KeyCode::KeyI),
        Wait(5),
        Shot("v16_inventory_locked_while_spectral"),
        Press(KeyCode::KeyI),
        Wait(3),
        Press(KeyCode::KeyT),
        Wait(15),
        Shot("v17_human_again_with_cloak"),
        Unequip(S::Body),
        Wait(5),
        Cape(false),
        Shot("v18_cloak_removed"),
        Wait(30),
    ]);
    acts
}

#[derive(Resource)]
struct Bench {
    acts: Vec<Act>,
    index: usize,
    waited: u32,
    release: Option<KeyCode>,
    failures: Vec<String>,
    exit_in: u32,
}

fn run_bench(
    mut commands: Commands,
    mut bench: ResMut<Bench>,
    data: Res<GameData>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut requests: MessageWriter<EquipRequest>,
    mut player: Query<&mut Belongings, With<Player>>,
    strikes: Query<&Hitbox, With<WeaponStrike>>,
    capes: Query<&Visibility, With<CapeLayer>>,
    exploration: Option<ResMut<Exploration>>,
    mut exit: MessageWriter<AppExit>,
) {
    if let Some(key) = bench.release.take() {
        keys.release(key);
    }
    let Some(act) = bench.acts.get(bench.index).copied() else {
        if bench.exit_in > 0 {
            bench.exit_in -= 1;
            return;
        }
        if VERDICT.get().is_none() {
            for failure in &bench.failures {
                println!("[equipment_showcase] FAIL {failure}");
            }
            let pass = bench.failures.is_empty();
            println!(
                "[equipment_showcase] RESULT: {}",
                if pass { "PASS" } else { "FAIL" }
            );
            let _ = VERDICT.set(pass);
            exit.write(AppExit::Success);
        }
        return;
    };
    let mut failures: Vec<String> = Vec::new();
    let mut next = true;
    match act {
        Act::Wait(frames) => {
            bench.waited += 1;
            next = bench.waited >= frames;
            if next {
                bench.waited = 0;
            }
        }
        Act::Give(items) => {
            if let Ok(mut b) = player.single_mut() {
                for id in items {
                    if let Err(error) = b.inventory.add(id, 1, &data.items) {
                        failures.push(format!("give {id}: {error}"));
                    }
                }
            }
        }
        Act::Equip(slot, id) => {
            requests.write(EquipRequest {
                slot,
                op: EquipOp::Equip(id.into()),
            });
        }
        Act::Unequip(slot) => {
            requests.write(EquipRequest {
                slot,
                op: EquipOp::Unequip,
            });
        }
        Act::Press(key) => {
            keys.release(key);
            keys.press(key);
            bench.release = Some(key);
        }
        Act::Hold(key) => keys.press(key),
        Act::Release(key) => keys.release(key),
        Act::Shot(name) => {
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(out_dir().join(format!("{name}.png"))));
            println!("[equipment_showcase] {name}");
        }
        Act::StrikeWidth(width) => {
            if !strikes.iter().any(|h| (h.size.x - width).abs() < 0.5) {
                failures.push(format!("no strike of width {width}"));
            }
        }
        Act::Cape(shown) => {
            let visible = capes.iter().any(|v| *v != Visibility::Hidden);
            if visible != shown {
                failures.push(format!("cape shown {visible}, expected {shown}"));
            }
        }
        Act::Equipped(slot, id) => {
            let ok = player
                .single_mut()
                .is_ok_and(|b| b.equipment.get(slot) == Some(id));
            if !ok {
                failures.push(format!("{id} not in {slot:?}"));
            }
        }
        Act::Relic => {
            if let Some(mut exploration) = exploration {
                exploration.granted.push(Grant::Ability(Ability::VeilStep));
            }
        }
    }
    bench.failures.extend(failures);
    if next {
        bench.index += 1;
    }
}

fn main() -> AppExit {
    let root = project_root();
    let loaded = GameData::load(&root)
        .map_err(|e| e.to_string())
        .and_then(|mut data| {
            dark_castle_rpg::app::load_world(&root, &mut data).map(|world| (data, world))
        });
    let (data, world) = match loaded {
        Ok(loaded) => loaded,
        Err(error) => {
            eprintln!("[equipment_showcase] {error}");
            return AppExit::error();
        }
    };
    let saves = out_dir().join("saves");
    let _ = std::fs::remove_dir_all(&saves);
    if let Err(error) = std::fs::create_dir_all(&saves) {
        eprintln!("[equipment_showcase] {}: {error}", saves.display());
        return AppExit::error();
    }
    let mut app = dark_castle_rpg::app::build_with_world(data, world);
    dark_castle_rpg::input::use_legacy_controls(&mut app);
    app.insert_resource(SaveSlots(SaveStorage::new(saves)))
        .insert_resource(TimeUpdateStrategy::ManualDuration(FRAME))
        .insert_resource(Bench {
            acts: script(),
            index: 0,
            waited: 0,
            release: None,
            failures: Vec::new(),
            exit_in: 30,
        })
        .add_systems(PreUpdate, run_bench.after(bevy::input::InputSystems));
    let exit = app.run();
    match VERDICT.get() {
        Some(true) => exit,
        _ => AppExit::error(),
    }
}
