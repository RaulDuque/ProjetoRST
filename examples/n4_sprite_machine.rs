//! N4-A isolated proof of the data-driven sprite machine (not the game).
//!
//! Left: the approved N3.5 idle (one sprite, the original PNG). Right: the
//! same character composed by `player::visual::composite` from
//! `art-src/n4-sprite-machine/stage/protagonist.ron`: one child sprite per
//! layer slot, all following one `VisualPlayback`. Both at 4 screen pixels
//! per texel, nearest sampling, same pivot line.
//!
//! ```text
//! cargo run --example n4_sprite_machine                    # scripted tour + captures
//! N4_INTERACTIVE=1 cargo run --example n4_sprite_machine   # free inspection
//! ```
//!
//! | Key | Effect |
//! |---|---|
//! | 1 2 3 4 5 | remove / wear hair, shirt, trousers, boots, bracers |
//! | 0 | base body (remove every removable part) |
//! | 9 | original outfit |
//! | O | show / hide the original on the left |
//! | G | texel grid (every 8 texels), canvas and pivot |
//! | K | dark / light background |
//! | M | mirror (facing left) |
//! | Space | pause / resume the scripted tour |
//!
//! The tour writes `docs/development/n4/evidence/bevy_n4a_*.png` and exits;
//! exit code != 0 if the art does not load or a check fails.

use std::path::PathBuf;
use std::sync::{Arc, OnceLock};

use bevy::asset::AssetApp;
use bevy::asset::io::AssetSourceBuilder;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::sprite::Anchor;
use bevy::window::WindowResolution;
use dark_castle_rpg::core::resources::project_root;
use dark_castle_rpg::player::visual::VisualPlayback;
use dark_castle_rpg::player::visual::composite::{
    CompositeCharacter, CompositeLayer, sync_composite_layers,
};
use dark_castle_rpg::player::visual::composition::{CharacterComposition, Outfit};
use dark_castle_rpg::player::visual::manifest::anchor_for;

const STAGE: &str = "art-src/n4-sprite-machine/stage";
const ORIGINAL: &str = "art-src/n3-5-fidelity/generated";
const EVIDENCE: &str = "docs/development/n4/evidence";
const TAG: &str = "[n4_sprite_machine]";
/// Screen pixels per texel.
const SCALE: f32 = 4.0;
const CELL: (u32, u32) = (80, 128);
const PIVOT: (f32, f32) = (40.0, 118.0);
const LEFT_X: f32 = -220.0;
const RIGHT_X: f32 = 220.0;
const SOLE_Y: f32 = -230.0;
const DARK: Color = Color::srgb(0.19, 0.17, 0.22);
const LIGHT: Color = Color::srgb(0.84, 0.82, 0.78);
/// Keys 1..5 and the parts they toggle.
const TOGGLES: [(KeyCode, &str); 5] = [
    (KeyCode::Digit1, "hair"),
    (KeyCode::Digit2, "shirt"),
    (KeyCode::Digit3, "trousers"),
    (KeyCode::Digit4, "boots"),
    (KeyCode::Digit5, "bracers"),
];

/// Frames between "textures loaded" and the first capture.
const WARM_UP_FRAMES: u32 = 30;

static VERDICT: OnceLock<bool> = OnceLock::new();

#[derive(Debug, Clone, Copy, PartialEq)]
enum Look {
    Full,
    Without(&'static str),
    Base,
}

#[derive(Debug, Clone, Copy)]
struct Shot {
    name: &'static str,
    look: Look,
    light: bool,
    mirror: bool,
    grid: bool,
}

const fn shot(name: &'static str, look: Look) -> Shot {
    Shot {
        name,
        look,
        light: false,
        mirror: false,
        grid: false,
    }
}

const TOUR: [Shot; 11] = [
    shot("bevy_n4a_original_vs_modular", Look::Full),
    shot("bevy_n4a_no_hair", Look::Without("hair")),
    shot("bevy_n4a_no_shirt", Look::Without("shirt")),
    shot("bevy_n4a_no_trousers", Look::Without("trousers")),
    shot("bevy_n4a_no_boots", Look::Without("boots")),
    shot("bevy_n4a_no_bracers", Look::Without("bracers")),
    shot("bevy_n4a_base_body", Look::Base),
    Shot {
        light: true,
        ..shot("bevy_n4a_light_background", Look::Full)
    },
    Shot {
        mirror: true,
        ..shot("bevy_n4a_mirrored", Look::Full)
    },
    Shot {
        grid: true,
        ..shot("bevy_n4a_grid_pivot", Look::Full)
    },
    Shot {
        light: true,
        grid: true,
        ..shot("bevy_n4a_base_body_light_grid", Look::Base)
    },
];

#[derive(Resource)]
struct Proof {
    composition: Arc<CharacterComposition>,
    interactive: bool,
    paused: bool,
    show_original: bool,
    grid: bool,
    light: bool,
    mirror: bool,
    tour: usize,
    wait: u32,
    warm: u32,
    loaded: bool,
    out: PathBuf,
    failures: Vec<String>,
}

#[derive(Component)]
struct OriginalSprite;

#[derive(Component)]
struct ModularRoot;

#[derive(Component)]
struct Hud;

fn setup(
    mut commands: Commands,
    proof: Res<Proof>,
    server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);
    let (ax, ay) = anchor_for(PIVOT, CELL, false);
    commands.spawn((
        OriginalSprite,
        Sprite {
            image: server.load("original://candidate_106.png"),
            custom_size: Some(Vec2::new(CELL.0 as f32 * SCALE, CELL.1 as f32 * SCALE)),
            ..default()
        },
        Anchor(Vec2::new(ax, ay)),
        Transform::from_xyz(LEFT_X, SOLE_Y, 0.0),
    ));
    let c = proof.composition.clone();
    let character = CompositeCharacter::load(
        c.clone(),
        Outfit::default_for(&c),
        &server,
        &mut layouts,
        "n4://",
    );
    let density = c.clips["idle"].density as f32;
    let root = commands
        .spawn((
            ModularRoot,
            VisualPlayback {
                clip: Some("idle".into()),
                ..default()
            },
            // Layers are sized in world units (cell / density): scale them so
            // one texel is SCALE screen pixels, like the original.
            Transform::from_xyz(RIGHT_X, SOLE_Y, 0.0).with_scale(Vec3::splat(SCALE * density)),
            Visibility::default(),
        ))
        .id();
    character.spawn_layers(&mut commands, root);
    commands.entity(root).insert(character);
    commands.spawn((
        Hud,
        Text::new(""),
        TextFont {
            font_size: FontSize::Px(15.0),
            ..default()
        },
        TextColor(Color::srgb(0.95, 0.8, 0.45)),
        Node {
            position_type: PositionType::Absolute,
            left: px(12),
            top: px(10),
            ..default()
        },
    ));
}

fn apply_look(character: &mut CompositeCharacter, look: Look) {
    let c = character.composition.clone();
    character.outfit = Outfit::default_for(&c);
    match look {
        Look::Full => {}
        Look::Without(part) => {
            character.outfit.remove(&c, part);
        }
        Look::Base => {
            for id in &c.default_outfit {
                character.outfit.remove(&c, id);
            }
        }
    }
}

fn keys(
    input: Res<ButtonInput<KeyCode>>,
    mut proof: ResMut<Proof>,
    mut roots: Query<&mut CompositeCharacter, With<ModularRoot>>,
) {
    let Ok(mut character) = roots.single_mut() else {
        return;
    };
    let c = character.composition.clone();
    for (key, part) in TOGGLES {
        if input.just_pressed(key) {
            if character.outfit.parts.contains_key(part) {
                character.outfit.remove(&c, part);
            } else {
                character.outfit.wear(part, None);
            }
        }
    }
    if input.just_pressed(KeyCode::Digit0) {
        apply_look(&mut character, Look::Base);
    }
    if input.just_pressed(KeyCode::Digit9) {
        apply_look(&mut character, Look::Full);
    }
    if input.just_pressed(KeyCode::KeyO) {
        proof.show_original = !proof.show_original;
    }
    if input.just_pressed(KeyCode::KeyG) {
        proof.grid = !proof.grid;
    }
    if input.just_pressed(KeyCode::KeyK) {
        proof.light = !proof.light;
    }
    if input.just_pressed(KeyCode::KeyM) {
        proof.mirror = !proof.mirror;
    }
    if input.just_pressed(KeyCode::Space) {
        proof.paused = !proof.paused;
    }
}

/// The scripted tour: wait for the textures, set each look, capture.
fn tour(
    mut commands: Commands,
    mut proof: ResMut<Proof>,
    images: Res<Assets<Image>>,
    original: Query<&Sprite, With<OriginalSprite>>,
    mut roots: Query<&mut CompositeCharacter, With<ModularRoot>>,
    layers: Query<(&CompositeLayer, &Visibility)>,
    mut exit: MessageWriter<AppExit>,
) {
    if proof.interactive || proof.paused {
        return;
    }
    let Ok(mut character) = roots.single_mut() else {
        return;
    };
    if !proof.loaded {
        let ready = original.iter().all(|s| images.contains(&s.image))
            && character.sheets.values().all(|s| images.contains(&s.image));
        proof.wait += 1;
        if ready {
            proof.loaded = true;
            proof.wait = 0;
            proof.warm = WARM_UP_FRAMES;
        } else if proof.wait > 600 {
            proof.failures.push("textures did not load".into());
            finish(&mut proof, &mut exit);
        }
        return;
    }
    // Loaded textures still need a few frames to reach the GPU.
    if proof.warm > 0 {
        proof.warm -= 1;
        return;
    }
    let Some(shot) = TOUR.get(proof.tour).copied() else {
        proof.wait += 1;
        if proof.wait > 20 {
            finish(&mut proof, &mut exit);
        }
        return;
    };
    match proof.wait {
        0 => {
            apply_look(&mut character, shot.look);
            proof.light = shot.light;
            proof.mirror = shot.mirror;
            proof.grid = shot.grid;
            proof.show_original = true;
        }
        6 => {
            let visible = layers
                .iter()
                .filter(|(_, v)| **v != Visibility::Hidden)
                .count();
            let expected = match shot.look {
                Look::Full => 8,
                Look::Without("hair") => 6,
                Look::Without(_) => 7,
                Look::Base => 2,
            };
            if visible != expected {
                proof.failures.push(format!(
                    "{}: {visible} visible layers, expected {expected}",
                    shot.name
                ));
            }
            let path = proof.out.join(format!("{}.png", shot.name));
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(path));
            println!("{TAG} {}", shot.name);
        }
        _ => {}
    }
    proof.wait += 1;
    if proof.wait > 12 {
        proof.wait = 0;
        proof.tour += 1;
    }
}

fn finish(proof: &mut Proof, exit: &mut MessageWriter<AppExit>) {
    if VERDICT.get().is_some() {
        return;
    }
    for f in &proof.failures {
        println!("{TAG} FAIL {f}");
    }
    let pass = proof.failures.is_empty();
    println!("{TAG} RESULT: {}", if pass { "PASS" } else { "FAIL" });
    let _ = VERDICT.set(pass);
    exit.write(AppExit::Success);
}

/// Background, facing, original visibility and HUD from the proof state.
fn present(
    proof: Res<Proof>,
    mut clear: ResMut<ClearColor>,
    mut original: Query<(&mut Sprite, &mut Anchor, &mut Visibility), With<OriginalSprite>>,
    mut roots: Query<(&mut VisualPlayback, &CompositeCharacter), With<ModularRoot>>,
    mut hud: Query<&mut Text, With<Hud>>,
) {
    clear.0 = if proof.light { LIGHT } else { DARK };
    let flip = proof.mirror;
    if let Ok((mut sprite, mut anchor, mut visibility)) = original.single_mut() {
        sprite.flip_x = flip;
        let (ax, ay) = anchor_for(PIVOT, CELL, flip);
        anchor.set_if_neq(Anchor(Vec2::new(ax, ay)));
        visibility.set_if_neq(if proof.show_original {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        });
    }
    let mut worn = String::new();
    if let Ok((mut playback, character)) = roots.single_mut() {
        // The one animation state: the proof only sets the facing (the idle
        // has one frame), the layers read it.
        if playback.flip != flip {
            playback.flip = flip;
        }
        worn = character
            .outfit
            .parts
            .keys()
            .cloned()
            .collect::<Vec<_>>()
            .join(", ");
    }
    if let Ok(mut text) = hud.single_mut() {
        text.0 = format!(
            "N4-A SPRITE MACHINE PROOF (NOT APPROVED)  left: N3.5 original  right: modular layers\n\
             worn: {worn}\n\
             1-5 hair/shirt/trousers/boots/bracers  0 base  9 outfit  O original  G grid  K background  M mirror  Space pause{}",
            if proof.paused { "  [PAUSED]" } else { "" }
        );
    }
}

/// Texel grid every 8 texels, canvas border and pivot cross.
fn grid(proof: Res<Proof>, mut gizmos: Gizmos) {
    if !proof.grid {
        return;
    }
    let line = Color::srgba(0.3, 0.7, 1.0, 0.45);
    let border = Color::srgb(0.3, 0.7, 1.0);
    let pivot = Color::srgb(1.0, 0.2, 0.3);
    for x0 in [LEFT_X, RIGHT_X] {
        let left = x0 - PIVOT.0 * SCALE;
        let top = SOLE_Y + PIVOT.1 * SCALE;
        let w = CELL.0 as f32 * SCALE;
        let h = CELL.1 as f32 * SCALE;
        for i in (0..=CELL.0).step_by(8) {
            let x = left + i as f32 * SCALE;
            gizmos.line_2d(Vec2::new(x, top), Vec2::new(x, top - h), line);
        }
        for j in (0..=CELL.1).step_by(8) {
            let y = top - j as f32 * SCALE;
            gizmos.line_2d(Vec2::new(left, y), Vec2::new(left + w, y), line);
        }
        let corners = [
            Vec2::new(left, top),
            Vec2::new(left + w, top),
            Vec2::new(left + w, top - h),
            Vec2::new(left, top - h),
        ];
        for i in 0..4 {
            gizmos.line_2d(corners[i], corners[(i + 1) % 4], border);
        }
        gizmos.line_2d(
            Vec2::new(x0 - 24.0, SOLE_Y),
            Vec2::new(x0 + 24.0, SOLE_Y),
            pivot,
        );
        gizmos.line_2d(
            Vec2::new(x0, SOLE_Y - 24.0),
            Vec2::new(x0, SOLE_Y + 24.0),
            pivot,
        );
    }
}

fn main() -> AppExit {
    let root = project_root();
    let interactive = std::env::var_os("N4_INTERACTIVE").is_some();
    let text = match std::fs::read_to_string(root.join(STAGE).join("protagonist.ron")) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("{TAG} composition missing: {error}");
            return AppExit::error();
        }
    };
    let composition = match CharacterComposition::from_ron(&text) {
        Ok(c) => Arc::new(c),
        Err(error) => {
            eprintln!("{TAG} invalid composition: {error}");
            return AppExit::error();
        }
    };
    let out = root.join(EVIDENCE);
    if let Err(error) = std::fs::create_dir_all(&out) {
        eprintln!("{TAG} {}: {error}", out.display());
        return AppExit::error();
    }
    let mut app = App::new();
    let stage = root.join(STAGE);
    let original = root.join(ORIGINAL);
    app.register_asset_source(
        "n4",
        AssetSourceBuilder::platform_default(&stage.to_string_lossy(), None),
    )
    .register_asset_source(
        "original",
        AssetSourceBuilder::platform_default(&original.to_string_lossy(), None),
    )
    .add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "N4-A sprite machine proof".into(),
                    resolution: WindowResolution::new(1280, 720),
                    ..default()
                }),
                ..default()
            }),
    )
    .insert_resource(ClearColor(DARK))
    .insert_resource(Proof {
        composition,
        interactive,
        paused: false,
        show_original: true,
        grid: false,
        light: false,
        mirror: false,
        tour: 0,
        wait: 0,
        warm: 0,
        loaded: false,
        out,
        failures: Vec::new(),
    })
    .add_systems(Startup, setup)
    .add_systems(
        Update,
        (keys, tour, present, sync_composite_layers, grid).chain(),
    );
    let exit = app.run();
    if interactive {
        return exit;
    }
    match VERDICT.get() {
        Some(true) => exit,
        _ => AppExit::error(),
    }
}
