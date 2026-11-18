//! V2.2 visual bench of the held equipment, in a window, with screenshots.
//!
//! **A bench, not a route:** the items are put in the bag directly. Each
//! item is equipped through `EquipRequest` (the inventory's path, with its
//! gate and the two-hands rules); every move is a real key press.
//!
//! Art: by default the **staged V2.2 candidate** (weaponless body +
//! sockets, `art-src/player/renders/v2_2_weaponless/stage/`), read through
//! a separate asset source: `assets/` is neither read for the player art
//! nor written. `--published` uses the published art in `assets/`; the held
//! layers are then expected exactly when its manifest says the body has no
//! weapon drawn in (`weapon_in_body: false`, published since V2.2), and
//! expected off for older art with the sabre baked in.
//!
//! Shots in `target/equipment_visual_showcase/<candidate|published>/`:
//! unarmed, the five weapon categories (idle and mid-strike), shield, armour
//! and cape, running, jumping, guard, dodge, facing left, the two-hands
//! refusals, unequipping and F5/F9. Exit code != 0 if a check fails.
//!
//! Run with: `cargo run --example equipment_visual_showcase [-- --published]`.

use std::path::PathBuf;
use std::sync::OnceLock;
use std::time::Duration;

use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::core::events::{EquipOp, EquipRequest};
use dark_castle_rpg::core::pixel_image::pixel_image;
use dark_castle_rpg::core::resources::{GameData, project_root};
use dark_castle_rpg::inventory::equipment::EquipmentSlot;
use dark_castle_rpg::inventory::held::{ShieldPlacement, WeaponForm};
use dark_castle_rpg::inventory::held_visual::{HeldEquipment, HeldLayer};
use dark_castle_rpg::npc::art::npc_art;
use dark_castle_rpg::player::components::{Belongings, Player};
use dark_castle_rpg::player::visual::manifest::AnimationManifest;
use dark_castle_rpg::player::visual::{MANIFEST_PATH, PlayerSpriteSet};
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::save::systems::SaveSlots;

const FRAME: Duration = Duration::from_millis(16);
const STAGE: &str = "art-src/player/renders/v2_2_weaponless/stage";

static VERDICT: OnceLock<bool> = OnceLock::new();

#[derive(Debug, Clone, Copy)]
enum Act {
    Wait(u32),
    Give(&'static [&'static str]),
    Equip(EquipmentSlot, &'static str),
    Unequip(EquipmentSlot),
    Press(KeyCode),
    Hold(KeyCode),
    Release(KeyCode),
    Shot(&'static str),
    /// The weapon layer: `Some(form)` visible, `None` absent or hidden.
    Weapon(Option<WeaponForm>),
    Shield(Option<ShieldPlacement>),
    /// The visible weapon layer is mirrored (facing left).
    Flipped(bool),
    Equipped(EquipmentSlot, Option<&'static str>),
    /// Presentation only: the figures of every NPC in the data, side by
    /// side next to the player (no NPC entity, no interaction).
    NpcLineup,
}

fn script() -> Vec<Act> {
    use Act::*;
    use EquipmentSlot as S;
    use WeaponForm as F;
    let mut acts = vec![
        Wait(60),
        Give(&[
            "gutter_knife",
            "hearthsplitter",
            "watch_pike",
            "bell_maul",
            "duskwater_saber",
            "watch_buckler",
            "emberlined_cloak",
            "chain_hauberk",
            "sentry_coif",
        ]),
        Wait(5),
        Weapon(Some(F::Sword)),
        Shot("s02_sword_iron_idle"),
        Unequip(S::Weapon),
        Wait(10),
        Weapon(None),
        Equipped(S::Weapon, None),
        Shot("s01_unarmed_idle"),
        Equip(S::Weapon, "iron_sword"),
        Wait(10),
    ];
    let weapons: [(&str, F, &str, &str); 5] = [
        (
            "iron_sword",
            F::Sword,
            "s03_sword_iron_strike",
            "s03b_sword_idle",
        ),
        (
            "gutter_knife",
            F::Dagger,
            "s04_dagger_strike",
            "s04b_dagger_idle",
        ),
        ("hearthsplitter", F::Axe, "s05_axe_strike", "s05b_axe_idle"),
        (
            "watch_pike",
            F::Spear,
            "s06_spear_strike",
            "s06b_spear_idle",
        ),
        ("bell_maul", F::Heavy, "s07_heavy_strike", "s07b_heavy_idle"),
    ];
    for (id, form, strike, idle) in weapons {
        acts.extend([
            Equip(S::Weapon, id),
            Wait(30),
            Weapon(Some(form)),
            Shot(idle),
            Press(KeyCode::KeyX),
            Wait(4),
            Weapon(Some(form)),
            Shot(strike),
            Wait(50),
        ]);
    }
    acts.extend([
        // Two hands: a shield with the greatblade is refused; the shield
        // does not appear.
        Equip(S::Offhand, "watch_buckler"),
        Wait(10),
        Equipped(S::Offhand, None),
        Shield(None),
        Shot("s08_two_handed_refuses_shield"),
        // One-handed sword + shield on the left arm.
        Equip(S::Weapon, "duskwater_saber"),
        Equip(S::Offhand, "watch_buckler"),
        Wait(10),
        Equipped(S::Offhand, Some("watch_buckler")),
        Shield(Some(ShieldPlacement::LeftHand)),
        Shot("s09_sword_and_shield"),
        // Dagger + shield.
        Equip(S::Weapon, "gutter_knife"),
        Wait(10),
        Weapon(Some(F::Dagger)),
        Shield(Some(ShieldPlacement::LeftHand)),
        Shot("s10_dagger_and_shield"),
        // Axe + shield (the axe class is one-handed in weapon_classes.ron).
        Equip(S::Weapon, "hearthsplitter"),
        Wait(10),
        Shield(Some(ShieldPlacement::LeftHand)),
        Shot("s11_axe_and_shield"),
        // The spear needs both hands while the shield is held: refused.
        Equip(S::Weapon, "watch_pike"),
        Wait(10),
        Equipped(S::Weapon, Some("hearthsplitter")),
        Weapon(Some(F::Axe)),
        Shot("s12_spear_refused_while_shield"),
        // Armour, helmet and cape.
        Equip(S::Weapon, "duskwater_saber"),
        Equip(S::Body, "emberlined_cloak"),
        Equip(S::Head, "sentry_coif"),
        Wait(15),
        Shot("s13_cape_shield_sword"),
        Equip(S::Body, "chain_hauberk"),
        Wait(10),
        Shot("s14_hauberk_shield_sword"),
        Equip(S::Body, "emberlined_cloak"),
        Wait(10),
        // Movement: run, jump, fall, guard, dodge, facing left.
        Hold(KeyCode::ArrowRight),
        Wait(25),
        Shot("s15_running"),
        Press(KeyCode::KeyZ),
        Wait(8),
        Shot("s16_jumping"),
        Wait(22),
        Shot("s17_falling_or_landing"),
        Release(KeyCode::ArrowRight),
        Wait(30),
        Hold(KeyCode::KeyA),
        Wait(12),
        Shot("s18_guard"),
        Release(KeyCode::KeyA),
        Wait(20),
        Press(KeyCode::KeyS),
        Wait(6),
        Shot("s19_dodge"),
        Wait(40),
        Hold(KeyCode::ArrowLeft),
        Wait(20),
        Release(KeyCode::ArrowLeft),
        Wait(15),
        Flipped(true),
        Weapon(Some(F::Sword)),
        Shot("s20_facing_left"),
        Press(KeyCode::KeyX),
        Wait(4),
        Flipped(true),
        Shot("s21_facing_left_strike"),
        Wait(40),
        // F5 saves sword + shield; swap to the axe, F9 brings the sword back.
        Press(KeyCode::F5),
        Wait(20),
        Unequip(S::Offhand),
        Equip(S::Weapon, "hearthsplitter"),
        Wait(10),
        Weapon(Some(F::Axe)),
        Shield(None),
        Shot("s22_changed_before_load"),
        Press(KeyCode::F9),
        Wait(30),
        Weapon(Some(F::Sword)),
        Shield(Some(ShieldPlacement::LeftHand)),
        Shot("s23_after_quick_load"),
        // Unequip everything held.
        Unequip(S::Offhand),
        Unequip(S::Weapon),
        Wait(10),
        Weapon(None),
        Shield(None),
        Shot("s24_unequipped"),
        NpcLineup,
        Wait(5),
        Shot("s25_npc_lineup_presentation_only"),
        Wait(20),
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
    /// Whether the art can show held layers (candidate) or not (published).
    layers_expected: bool,
    out: PathBuf,
}

fn layer_visible(v: &InheritedVisibility, own: &Visibility) -> bool {
    *own != Visibility::Hidden && v.get()
}

fn run_bench(
    mut commands: Commands,
    mut bench: ResMut<Bench>,
    data: Res<GameData>,
    set: Option<Res<PlayerSpriteSet>>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut requests: MessageWriter<EquipRequest>,
    mut player: Query<(&mut Belongings, Option<&HeldEquipment>), With<Player>>,
    layers: Query<(&HeldLayer, &Sprite, &Visibility, &InheritedVisibility)>,
    player_at: Query<&Transform, With<Player>>,
    mut images: ResMut<Assets<Image>>,
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
                println!("[equipment_visual_showcase] FAIL {failure}");
            }
            let pass = bench.failures.is_empty();
            println!(
                "[equipment_visual_showcase] RESULT: {}",
                if pass { "PASS" } else { "FAIL" }
            );
            let _ = VERDICT.set(pass);
            exit.write(AppExit::Success);
        }
        return;
    };
    let mut failures: Vec<String> = Vec::new();
    let mut next = true;
    let shown = |kind: HeldLayer| {
        layers
            .iter()
            .any(|(k, _, own, inherited)| *k == kind && layer_visible(inherited, own))
    };
    let at = bench.index;
    match act {
        Act::Wait(frames) => {
            bench.waited += 1;
            next = bench.waited >= frames;
            if next {
                bench.waited = 0;
            }
        }
        Act::Give(items) => {
            if let Ok((mut b, _)) = player.single_mut() {
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
                .observe(save_to_disk(bench.out.join(format!("{name}.png"))));
            println!("[equipment_visual_showcase] {name}");
        }
        Act::Weapon(form) => {
            let held = player
                .single()
                .ok()
                .and_then(|(_, h)| h.and_then(|h| h.weapon).map(|w| w.form));
            if held != form {
                failures.push(format!("#{at}: held weapon {held:?}, expected {form:?}"));
            }
            let visible = shown(HeldLayer::Weapon);
            let expected = form.is_some() && bench.layers_expected;
            if visible != expected {
                failures.push(format!(
                    "#{at}: weapon layer visible {visible}, expected {expected}"
                ));
            }
        }
        Act::Shield(placement) => {
            let held = player
                .single()
                .ok()
                .and_then(|(_, h)| h.and_then(|h| h.shield).map(|s| s.placement));
            if held != placement {
                failures.push(format!("#{at}: shield {held:?}, expected {placement:?}"));
            }
            let visible = shown(HeldLayer::Shield);
            let expected = placement.is_some() && bench.layers_expected;
            if visible != expected {
                failures.push(format!(
                    "#{at}: shield layer visible {visible}, expected {expected}"
                ));
            }
        }
        Act::Flipped(flipped) => {
            if bench.layers_expected {
                let ok = layers
                    .iter()
                    .filter(|(k, ..)| **k == HeldLayer::Weapon)
                    .all(|(_, s, ..)| s.flip_x == flipped);
                if !ok {
                    failures.push(format!("#{at}: weapon layer flip != {flipped}"));
                }
            }
        }
        Act::NpcLineup => {
            let origin = player_at.single().map_or(Vec3::ZERO, |t| t.translation);
            for (i, npc) in data.npcs.npcs().enumerate() {
                let art = npc_art(npc.size, npc.color, npc.look.as_ref());
                let (w, h) = art.world_size();
                let (ax, ay) = art.anchor(false);
                let mut sprite = Sprite::from_image(images.add(pixel_image(art)));
                sprite.custom_size = Some(Vec2::new(w, h));
                // Body centres on the player's floor line.
                let y = origin.y - 23.0 + npc.size.1 / 2.0;
                commands.spawn((
                    sprite,
                    bevy::sprite::Anchor(Vec2::new(ax, ay)),
                    Transform::from_xyz(origin.x + 50.0 + 40.0 * i as f32, y, 50.0),
                ));
                println!("[equipment_visual_showcase] lineup {}: {}", i + 1, npc.name);
            }
        }
        Act::Equipped(slot, id) => {
            let current = player
                .single()
                .ok()
                .and_then(|(b, _)| b.equipment.get(slot).map(str::to_owned));
            if current.as_deref() != id {
                failures.push(format!(
                    "#{at}: {slot:?} holds {current:?}, expected {id:?}"
                ));
            }
        }
    }
    if at == 1
        && let Some(set) = set.as_deref()
        && bench.layers_expected != set.held_layers_enabled()
    {
        failures.push(format!(
            "art: held layers enabled {}, expected {}",
            set.held_layers_enabled(),
            bench.layers_expected
        ));
    }
    bench.failures.extend(failures);
    if next {
        bench.index += 1;
    }
}

fn main() -> AppExit {
    let published = std::env::args().any(|a| a == "--published");
    let root = project_root();
    let loaded = GameData::load(&root)
        .map_err(|e| e.to_string())
        .and_then(|mut data| {
            dark_castle_rpg::app::load_world(&root, &mut data).map(|world| (data, world))
        });
    let (data, world) = match loaded {
        Ok(loaded) => loaded,
        Err(error) => {
            eprintln!("[equipment_visual_showcase] {error}");
            return AppExit::error();
        }
    };
    let stage = root.join(STAGE);
    let kind = if published { "published" } else { "candidate" };
    let out = root.join("target/equipment_visual_showcase").join(kind);
    if !published && !stage.join("animations/player_sockets.ron").is_file() {
        eprintln!(
            "[equipment_visual_showcase] no staged candidate in {}: run \
             art-src/player/scripts/post/game_sprites_v2_2.py first",
            stage.display()
        );
        return AppExit::error();
    }
    let saves = out.join("saves");
    let _ = std::fs::remove_dir_all(&saves);
    if let Err(error) = std::fs::create_dir_all(&saves) {
        eprintln!("[equipment_visual_showcase] {}: {error}", saves.display());
        return AppExit::error();
    }
    // What the art says about the held layers (published: read from its
    // manifest, never assumed).
    let layers_expected = if published {
        let path = root.join("assets").join(MANIFEST_PATH);
        match std::fs::read_to_string(&path)
            .map_err(|e| e.to_string())
            .and_then(|t| AnimationManifest::from_ron(&t).map_err(|e| e.to_string()))
        {
            Ok(manifest) => !manifest.weapon_in_body,
            Err(error) => {
                eprintln!("[equipment_visual_showcase] {}: {error}", path.display());
                return AppExit::error();
            }
        }
    } else {
        true
    };
    println!("[equipment_visual_showcase] art: {kind}, held layers expected: {layers_expected}");
    let mut app = if published {
        dark_castle_rpg::app::build_with_world(data, world)
    } else {
        dark_castle_rpg::app::build_with_candidate_art(data, world, &stage)
    };
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
            layers_expected,
            out,
        })
        .add_systems(PreUpdate, run_bench.after(bevy::input::InputSystems));
    let exit = app.run();
    match VERDICT.get() {
        Some(true) => exit,
        _ => AppExit::error(),
    }
}
