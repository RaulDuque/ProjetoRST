//! V2.2: held equipment layers (weapon in the hand, shield) in the headless
//! game with an `AssetServer` and no renderer.
//!
//! Kinds of player art:
//! - the **published** art (V2.2 weaponless body + measured sockets): the
//!   layers are on and use the real sockets;
//! - a **legacy test stage** (the published manifest without the
//!   `weapon_in_body` line, as every manifest up to V1.5, so the sabre is
//!   taken as drawn in): the layers must stay off, so no second weapon is
//!   ever drawn, even with a socket table next to it;
//! - a **weaponless test stage** (`weapon_in_body: false` and synthetic
//!   sockets whose weapon x is `10 + frame`): the layers follow the equipped
//!   item, the frame shown, the facing, the two-hands rule and F5/F9.
//!
//! Stages are written by the test under the target dir and read through
//! `PlayerArtSource`.
//!
//! `assets/` and `saves/` are never written.

use std::path::{Path, PathBuf};
use std::time::Duration;

use bevy::asset::AssetPlugin;
use bevy::input::{InputPlugin as BevyInputPlugin, InputSystems};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::state::app::StatesPlugin;
use bevy::time::TimeUpdateStrategy;
use dark_castle_rpg::app::plugins::GamePlugin;
use dark_castle_rpg::combat::hitboxes::CombatProfile;
use dark_castle_rpg::core::events::{EquipOp, EquipRequest};
use dark_castle_rpg::core::resources::GameData;
use dark_castle_rpg::inventory::equipment::EquipmentSlot;
use dark_castle_rpg::inventory::held::{ShieldPlacement, WeaponForm};
use dark_castle_rpg::inventory::held_visual::{FRONT_Z, HeldEquipment, HeldLayer, SHIELD_Z};
use dark_castle_rpg::player::attributes::BaseAttributes;
use dark_castle_rpg::player::components::{Belongings, Player};
use dark_castle_rpg::player::visual::manifest::AnimationManifest;
use dark_castle_rpg::player::visual::sockets::{SOCKETS_PATH, SocketTable};
use dark_castle_rpg::player::visual::systems::VISUAL_OFFSET_Y;
use dark_castle_rpg::player::visual::{
    MANIFEST_PATH, PlayerArtSource, PlayerSpriteSet, PlayerVisual, VisualPlayback,
};
use dark_castle_rpg::save::save_manager::SaveStorage;
use dark_castle_rpg::save::systems::SaveSlots;
use dark_castle_rpg::skills::unlock::RequirementContext;

const FRAME: Duration = Duration::from_millis(16);

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn tmp(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn frame_ron(i: usize) -> String {
    format!(
        "(weapon: (x: {}.0, y: 30.0, angle: 20.0, front: true), \
         hand_l: (x: -6.0, y: 28.0, front: false), head: (x: 1.0, y: 46.0), \
         chest: (x: 0.0, y: 36.0), back: (x: -2.0, y: 39.0))",
        10 + i
    )
}

/// A weaponless test stage: the published manifest with
/// `weapon_in_body: false` and synthetic sockets for every character clip
/// except `omit`.
fn test_stage(name: &str, omit: &[&str]) -> PathBuf {
    stage_with(name, omit, false)
}

/// A legacy stage: no `weapon_in_body` line (sabre drawn in by default),
/// with the same synthetic sockets next to it.
fn legacy_stage(name: &str) -> PathBuf {
    stage_with(name, &[], true)
}

fn stage_with(name: &str, omit: &[&str], legacy: bool) -> PathBuf {
    let dir = tmp(name);
    let published = std::fs::read_to_string(root().join("assets").join(MANIFEST_PATH))
        .unwrap_or_else(|e| panic!("{e}"));
    let lines: Vec<&str> = published
        .lines()
        .filter(|l| !l.trim_start().starts_with("weapon_in_body:"))
        .collect();
    let mut text = lines.join("\n");
    if !legacy {
        text = text.replacen(
            "    facing: Right,",
            "    facing: Right,\n    weapon_in_body: false,",
            1,
        );
    }
    let manifest = AnimationManifest::from_ron(&text).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(manifest.weapon_in_body, legacy);
    let mut clips = String::new();
    for (key, clip) in &manifest.clips {
        if key.starts_with("vfx_") || omit.contains(&key.as_str()) {
            continue;
        }
        let frames: Vec<String> = (0..clip.count as usize).map(frame_ron).collect();
        clips.push_str(&format!("\"{key}\": [{}],\n", frames.join(", ")));
    }
    let sockets = format!("(version: 1, clips: {{\n{clips}}})");
    SocketTable::from_ron(&sockets, &manifest).unwrap_or_else(|e| panic!("{e}"));
    std::fs::create_dir_all(dir.join("animations")).unwrap_or_else(|e| panic!("{e}"));
    std::fs::write(dir.join(MANIFEST_PATH), text).unwrap_or_else(|e| panic!("{e}"));
    std::fs::write(dir.join(SOCKETS_PATH), sockets).unwrap_or_else(|e| panic!("{e}"));
    dir
}

#[derive(Resource, Default)]
struct PendingKeys {
    press: Vec<KeyCode>,
    release: Vec<KeyCode>,
}

fn apply_pending_keys(mut pending: ResMut<PendingKeys>, mut keys: ResMut<ButtonInput<KeyCode>>) {
    for key in pending.release.drain(..) {
        keys.release(key);
    }
    for key in pending.press.drain(..) {
        keys.press(key);
    }
}

/// The game with the art read from `stage` (`None`: the published art).
fn app(stage: Option<&Path>, saves: &str) -> App {
    let data = GameData::load(root()).unwrap_or_else(|e| panic!("{e}"));
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatesPlugin, BevyInputPlugin))
        .add_plugins(AssetPlugin::default())
        .init_asset::<Image>()
        .init_asset::<TextureAtlasLayout>();
    if let Some(stage) = stage {
        app.insert_resource(PlayerArtSource {
            root: stage.to_path_buf(),
            texture_prefix: String::new(),
        });
    }
    app.insert_resource(TimeUpdateStrategy::ManualDuration(FRAME))
        .insert_resource(data)
        .insert_resource(SaveSlots(SaveStorage::new(tmp(saves))))
        .init_resource::<PendingKeys>()
        .add_systems(PreUpdate, apply_pending_keys.after(InputSystems))
        .add_plugins(GamePlugin);
    app.update();
    // The atlas PNGs never load here: put images under their handles.
    let handles: Vec<Handle<Image>> = app
        .world()
        .resource::<PlayerSpriteSet>()
        .atlases
        .values()
        .map(|a| a.image.clone())
        .collect();
    let mut images = app.world_mut().resource_mut::<Assets<Image>>();
    for handle in handles {
        images
            .insert(&handle, Image::default())
            .unwrap_or_else(|e| panic!("{e}"));
    }
    run(&mut app, 10);
    app
}

fn run(app: &mut App, frames: usize) {
    for _ in 0..frames {
        app.update();
    }
}

fn tap(app: &mut App, key: KeyCode) {
    app.world_mut()
        .resource_mut::<PendingKeys>()
        .press
        .push(key);
    run(app, 1);
    app.world_mut()
        .resource_mut::<PendingKeys>()
        .release
        .push(key);
    run(app, 1);
}

fn request(app: &mut App, slot: EquipmentSlot, op: EquipOp) {
    app.world_mut().write_message(EquipRequest { slot, op });
    run(app, 2);
}

fn give(app: &mut App, items: &[&str]) {
    let data = app.world().resource::<GameData>().clone();
    let world = app.world_mut();
    let mut q = world.query_filtered::<&mut Belongings, With<Player>>();
    let mut b = q.single_mut(world).unwrap_or_else(|e| panic!("{e}"));
    for id in items {
        b.inventory
            .add(id, 1, &data.items)
            .unwrap_or_else(|e| panic!("{e}"));
    }
}

#[derive(Debug, Clone)]
struct Layer {
    entity: Entity,
    visible: bool,
    translation: Vec3,
    flip: bool,
    anchor: Vec2,
    image: Handle<Image>,
}

fn layers(app: &mut App, kind: HeldLayer) -> Vec<Layer> {
    let world = app.world_mut();
    let mut q = world.query::<(
        Entity,
        &HeldLayer,
        &Sprite,
        &Anchor,
        &Transform,
        &Visibility,
    )>();
    q.iter(world)
        .filter(|(_, k, ..)| **k == kind)
        .map(|(entity, _, sprite, anchor, transform, visibility)| Layer {
            entity,
            visible: *visibility != Visibility::Hidden,
            translation: transform.translation,
            flip: sprite.flip_x,
            anchor: anchor.0,
            image: sprite.image.clone(),
        })
        .collect()
}

fn one(app: &mut App, kind: HeldLayer) -> Layer {
    let all = layers(app, kind);
    assert_eq!(all.len(), 1, "{kind:?}: {} layers", all.len());
    all[0].clone()
}

fn held(app: &mut App) -> HeldEquipment {
    let world = app.world_mut();
    world
        .query_filtered::<&HeldEquipment, With<Player>>()
        .single(world)
        .copied()
        .unwrap_or_else(|e| panic!("{e}"))
}

/// Frame of the clip the body shows now.
fn body_frame(app: &mut App) -> (String, usize) {
    let set = app.world().resource::<PlayerSpriteSet>().clone();
    let world = app.world_mut();
    let playback = world
        .query_filtered::<&VisualPlayback, With<PlayerVisual>>()
        .single(world)
        .cloned()
        .unwrap_or_else(|e| panic!("{e}"));
    let key = playback.clip.unwrap_or_default();
    let first = set.manifest.clips.get(&key).map_or(0, |c| c.first as usize);
    (key, playback.index - first)
}

#[test]
fn the_published_art_is_weaponless_and_its_sockets_are_used() {
    let mut app = app(None, "held_published");
    let set = app.world().resource::<PlayerSpriteSet>().clone();
    assert!(!set.manifest.weapon_in_body, "V2.2 atlases have no sabre");
    assert!(
        set.held_layers_enabled(),
        "the published socket table loads"
    );
    let table = set.sockets.clone().unwrap_or_else(|| panic!("no sockets"));
    // Every character clip has a socket entry per frame (effects have none).
    for (key, clip) in &set.manifest.clips {
        if key.starts_with("vfx_") {
            continue;
        }
        let frames = table.clips.get(key).map_or(0, Vec::len);
        assert_eq!(frames, clip.count as usize, "{key}");
    }
    // The starting sword is drawn on the real socket of the frame shown.
    run(&mut app, 5);
    let (clip, frame) = body_frame(&mut app);
    let socket = table
        .frame(&clip, frame)
        .unwrap_or_else(|| panic!("{clip} {frame}"));
    let layer = one(&mut app, HeldLayer::Weapon);
    assert!(layer.visible);
    assert!((layer.translation.x - socket.weapon.x).abs() < 1e-4);
    assert!((layer.translation.y - (VISUAL_OFFSET_Y + socket.weapon.y)).abs() < 1e-4);
}

#[test]
fn art_with_the_baked_sabre_keeps_the_layers_off() {
    let stage = legacy_stage("held_stage_legacy");
    let mut app = app(Some(&stage), "held_legacy");
    let set = app.world().resource::<PlayerSpriteSet>().clone();
    assert!(
        set.manifest.weapon_in_body,
        "a manifest without the line has the sabre drawn in"
    );
    assert!(set.sockets.is_some(), "even with sockets next to it");
    assert!(!set.held_layers_enabled());
    // The equipment still resolves (the inventory and the next art read it)...
    assert_eq!(
        held(&mut app).weapon.map(|w| w.form),
        Some(WeaponForm::Sword)
    );
    // ...but no second weapon is drawn over the baked one.
    give(&mut app, &["hearthsplitter", "watch_buckler"]);
    request(
        &mut app,
        EquipmentSlot::Weapon,
        EquipOp::Equip("hearthsplitter".into()),
    );
    request(
        &mut app,
        EquipmentSlot::Offhand,
        EquipOp::Equip("watch_buckler".into()),
    );
    assert!(layers(&mut app, HeldLayer::Weapon).is_empty());
    assert!(layers(&mut app, HeldLayer::Shield).is_empty());
}

#[test]
fn the_weapon_layer_follows_the_equipped_item_and_the_frame() {
    let stage = test_stage("held_stage_follow", &[]);
    let mut app = app(Some(&stage), "held_follow");
    assert!(
        app.world()
            .resource::<PlayerSpriteSet>()
            .held_layers_enabled()
    );
    // The starting Iron Sword is in the hand.
    assert_eq!(
        held(&mut app).weapon.map(|w| w.form),
        Some(WeaponForm::Sword)
    );
    let sword = one(&mut app, HeldLayer::Weapon);
    assert!(sword.visible && !sword.flip);
    // Placed on the socket of the frame shown (x = 10 + frame, y = 30).
    for _ in 0..12 {
        run(&mut app, 3);
        let (_, frame) = body_frame(&mut app);
        let layer = one(&mut app, HeldLayer::Weapon);
        assert_eq!(layer.translation.x, 10.0 + frame as f32);
        assert_eq!(layer.translation.y, VISUAL_OFFSET_Y + 30.0);
        assert_eq!(layer.translation.z, FRONT_Z);
        assert_eq!(
            layer.entity, sword.entity,
            "updated in place, never respawned"
        );
    }
    // Swap: the same entity changes image; every form has its own art.
    give(
        &mut app,
        &["gutter_knife", "hearthsplitter", "watch_pike", "bell_maul"],
    );
    let mut images = vec![sword.image.clone()];
    for (id, form) in [
        ("gutter_knife", WeaponForm::Dagger),
        ("hearthsplitter", WeaponForm::Axe),
        ("watch_pike", WeaponForm::Spear),
        ("bell_maul", WeaponForm::Heavy),
    ] {
        request(&mut app, EquipmentSlot::Weapon, EquipOp::Equip(id.into()));
        assert_eq!(held(&mut app).weapon.map(|w| w.form), Some(form), "{id}");
        let layer = one(&mut app, HeldLayer::Weapon);
        assert_eq!(layer.entity, sword.entity, "{id}");
        assert!(layer.visible, "{id}");
        assert!(!images.contains(&layer.image), "{id}: art of another form");
        images.push(layer.image);
    }
    // Unequip: no weapon in the hand at all.
    request(&mut app, EquipmentSlot::Weapon, EquipOp::Unequip);
    assert_eq!(held(&mut app).weapon, None);
    run(&mut app, 2);
    assert!(layers(&mut app, HeldLayer::Weapon).is_empty());
}

#[test]
fn facing_left_mirrors_the_weapon() {
    let stage = test_stage("held_stage_facing", &[]);
    let mut app = app(Some(&stage), "held_facing");
    let right = one(&mut app, HeldLayer::Weapon);
    app.world_mut()
        .resource_mut::<PendingKeys>()
        .press
        .push(KeyCode::ArrowLeft);
    run(&mut app, 4);
    app.world_mut()
        .resource_mut::<PendingKeys>()
        .release
        .push(KeyCode::ArrowLeft);
    run(&mut app, 20);
    let (_, frame) = body_frame(&mut app);
    let left = one(&mut app, HeldLayer::Weapon);
    assert!(left.flip && !right.flip);
    assert_eq!(left.translation.x, -(10.0 + frame as f32));
    assert_eq!(left.anchor.x, -right.anchor.x, "grip anchor mirrored");
    assert_eq!(left.anchor.y, right.anchor.y);
}

#[test]
fn shield_and_two_hands_rules_show_up_in_the_layers() {
    let stage = test_stage("held_stage_shield", &[]);
    let mut app = app(Some(&stage), "held_shield");
    give(&mut app, &["watch_buckler", "watch_pike", "hearthsplitter"]);
    // Sword + shield: shield on the left arm.
    request(
        &mut app,
        EquipmentSlot::Offhand,
        EquipOp::Equip("watch_buckler".into()),
    );
    let shield = held(&mut app).shield.unwrap_or_else(|| panic!("no shield"));
    assert_eq!(shield.placement, ShieldPlacement::LeftHand);
    let layer = one(&mut app, HeldLayer::Shield);
    assert!(layer.visible);
    assert_eq!(
        layer.translation.truncate(),
        Vec2::new(-6.0, VISUAL_OFFSET_Y + 28.0)
    );
    // In front of the body, behind the weapon.
    assert_eq!(layer.translation.z, SHIELD_Z);
    const { assert!(SHIELD_Z < FRONT_Z) };
    // Axe (one-handed) keeps the shield.
    request(
        &mut app,
        EquipmentSlot::Weapon,
        EquipOp::Equip("hearthsplitter".into()),
    );
    assert_eq!(held(&mut app).weapon.map(|w| w.form), Some(WeaponForm::Axe));
    assert_eq!(layers(&mut app, HeldLayer::Shield).len(), 1);
    // The spear needs both hands: refused while the shield is held.
    request(
        &mut app,
        EquipmentSlot::Weapon,
        EquipOp::Equip("watch_pike".into()),
    );
    assert_eq!(held(&mut app).weapon.map(|w| w.form), Some(WeaponForm::Axe));
    // Without the shield the spear goes in; the shield is refused then.
    request(&mut app, EquipmentSlot::Offhand, EquipOp::Unequip);
    request(
        &mut app,
        EquipmentSlot::Weapon,
        EquipOp::Equip("watch_pike".into()),
    );
    request(
        &mut app,
        EquipmentSlot::Offhand,
        EquipOp::Equip("watch_buckler".into()),
    );
    let look = held(&mut app);
    assert_eq!(
        look.weapon.map(|w| (w.form, w.two_handed)),
        Some((WeaponForm::Spear, true))
    );
    assert_eq!(look.shield, None);
    run(&mut app, 2);
    assert!(layers(&mut app, HeldLayer::Shield).is_empty());
    // An inconsistent loadout (an edited save) is not hidden: the shield
    // shows, slung on the back, never in the hand holding the spear.
    {
        let world = app.world_mut();
        let data = world.resource::<GameData>().clone();
        let mut q = world.query_filtered::<&mut Belongings, With<Player>>();
        let mut b = q.single_mut(world).unwrap_or_else(|e| panic!("{e}"));
        let def = data
            .items
            .get("watch_buckler")
            .unwrap_or_else(|| panic!("buckler"));
        let attributes = BaseAttributes {
            strength: 99,
            constitution: 99,
            intelligence: 99,
            luck: 99,
        };
        let none = std::collections::BTreeSet::new();
        let ctx = RequirementContext {
            level: 99,
            attributes: &attributes,
            owned_items: &none,
            progress_flags: &none,
        };
        b.equipment
            .equip(EquipmentSlot::Offhand, def, &ctx)
            .unwrap_or_else(|e| panic!("{e}"));
    }
    run(&mut app, 2);
    let shield = held(&mut app).shield.unwrap_or_else(|| panic!("no shield"));
    assert_eq!(shield.placement, ShieldPlacement::Back);
    let layer = one(&mut app, HeldLayer::Shield);
    assert!(layer.visible && layer.translation.z < FRONT_Z);
}

#[test]
fn a_clip_without_sockets_hides_the_layer_instead_of_floating() {
    let stage = test_stage("held_stage_missing", &["dodge"]);
    let mut app = app(Some(&stage), "held_missing");
    assert!(one(&mut app, HeldLayer::Weapon).visible);
    tap(&mut app, KeyCode::KeyS);
    run(&mut app, 2);
    let (clip, _) = body_frame(&mut app);
    assert_eq!(clip, "dodge");
    assert!(!one(&mut app, HeldLayer::Weapon).visible);
    run(&mut app, 60);
    assert!(
        one(&mut app, HeldLayer::Weapon).visible,
        "back after the dodge"
    );
}

#[test]
fn quick_load_restores_the_held_look_and_gameplay_is_unchanged() {
    let stage = test_stage("held_stage_save", &[]);
    let mut with_layers = app(Some(&stage), "held_save");
    let legacy = legacy_stage("held_stage_save_legacy");
    let mut without = app(Some(&legacy), "held_save_legacy");
    give(&mut with_layers, &["hearthsplitter", "watch_buckler"]);
    request(
        &mut with_layers,
        EquipmentSlot::Offhand,
        EquipOp::Equip("watch_buckler".into()),
    );
    tap(&mut with_layers, KeyCode::F5);
    run(&mut with_layers, 5);
    request(&mut with_layers, EquipmentSlot::Offhand, EquipOp::Unequip);
    request(
        &mut with_layers,
        EquipmentSlot::Weapon,
        EquipOp::Equip("hearthsplitter".into()),
    );
    assert_eq!(
        held(&mut with_layers).weapon.map(|w| w.form),
        Some(WeaponForm::Axe)
    );
    tap(&mut with_layers, KeyCode::F9);
    run(&mut with_layers, 5);
    let look = held(&mut with_layers);
    assert_eq!(look.weapon.map(|w| w.form), Some(WeaponForm::Sword));
    assert_eq!(
        look.shield.map(|s| s.placement),
        Some(ShieldPlacement::LeftHand)
    );
    assert!(one(&mut with_layers, HeldLayer::Weapon).visible);
    assert!(one(&mut with_layers, HeldLayer::Shield).visible);
    // The layers are presentation only: same combat numbers without them.
    give(&mut without, &["watch_buckler"]);
    request(
        &mut without,
        EquipmentSlot::Offhand,
        EquipOp::Equip("watch_buckler".into()),
    );
    let profile = |app: &mut App| {
        let world = app.world_mut();
        world
            .query_filtered::<&CombatProfile, With<Player>>()
            .single(world)
            .map(|p| format!("{p:?}"))
            .unwrap_or_else(|e| panic!("{e}"))
    };
    assert_eq!(profile(&mut with_layers), profile(&mut without));
}

#[test]
fn held_layers_cost_is_measured_against_the_same_scene_without_them() {
    use dark_castle_rpg::inventory::held_visual::HeldArtCache;
    use std::time::Instant;

    fn measure(app: &mut App) -> (f64, usize) {
        give(app, &["watch_buckler", "emberlined_cloak"]);
        request(
            app,
            EquipmentSlot::Offhand,
            EquipOp::Equip("watch_buckler".into()),
        );
        request(
            app,
            EquipmentSlot::Body,
            EquipOp::Equip("emberlined_cloak".into()),
        );
        // Let transient entities (notifications, effects) settle first.
        run(app, 240);
        let frames = 300;
        let t = Instant::now();
        run(app, frames);
        let ms = t.elapsed().as_secs_f64() * 1000.0 / frames as f64;
        (ms, live(app))
    }
    fn live(app: &mut App) -> usize {
        let world = app.world_mut();
        world.query::<Entity>().iter(world).count()
    }
    let stage = test_stage("held_stage_perf", &[]);
    let mut with_layers = app(Some(&stage), "held_perf");
    let legacy = legacy_stage("held_stage_perf_legacy");
    let mut without = app(Some(&legacy), "held_perf_legacy");
    let (ms_with, entities_with) = measure(&mut with_layers);
    let (ms_without, entities_without) = measure(&mut without);
    let images = with_layers.world().resource::<HeldArtCache>().len();
    // Exactly two layer entities with the weapon-free art, none with the
    // legacy (sabre drawn in) one; the scene does not grow over time (layers are updated
    // in place, never respawned).
    let held =
        |app: &mut App| layers(app, HeldLayer::Weapon).len() + layers(app, HeldLayer::Shield).len();
    assert_eq!(held(&mut with_layers), 2);
    assert_eq!(held(&mut without), 0);
    let ids = |app: &mut App| {
        let mut v: Vec<Entity> = layers(app, HeldLayer::Weapon)
            .into_iter()
            .chain(layers(app, HeldLayer::Shield))
            .map(|l| l.entity)
            .collect();
        v.sort();
        v
    };
    let before = ids(&mut with_layers);
    run(&mut with_layers, 120);
    assert_eq!(ids(&mut with_layers), before, "layers are never respawned");
    assert_eq!(live(&mut with_layers), entities_with);
    assert_eq!(images, 2, "one weapon image and one shield image");
    let report = format!(
        "held layers on (weapon + shield + cape) {ms_with:.4} ms/frame, {entities_with} entities\n\
         held layers off (legacy art with the sabre drawn in, same scene) {ms_without:.4} ms/frame, {entities_without} entities\n\
         generated held images {images}\n"
    );
    let dir = root().join("target/tmp");
    let _ = std::fs::create_dir_all(&dir);
    let _ = std::fs::write(dir.join("held_v22_perf.txt"), &report);
    println!("{report}");
}
