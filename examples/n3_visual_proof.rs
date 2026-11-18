//! Isolated N3 visual proof. It does not boot the game, the camera, or the
//! control profile.
//!
//! ```text
//! python3 art-src/n3-proof/scripts/build_n3_proof.py
//! cargo run --example n3_visual_proof
//! N3_INTERACTIVE=1 cargo run --example n3_visual_proof
//! ```
//!
//! The default run shows each required view and writes PNGs to
//! `docs/development/n3/evidence/`. Interactive keys, local to this window:
//!
//! | Key | Effect |
//! |---|---|
//! | 1 2 3 4 | idle, walk, jump, attack (starts the clip) |
//! | Z | sword / spear |
//! | X | light armor / heavy armor |
//! | C | cape on / off |
//! | M | mirror |
//! | B | N3 baseline / automatic pixel of the V2.2 attack / raw render |
//! | V | close view (360, integer 2×) / game framing (405) |
//! | Space | pause |
//! | . | one frame while paused |
//! | Esc | quit |
//!
//! Equipment changes do not restart the clip.

use std::path::PathBuf;
use std::time::Duration;

use bevy::camera::ScalingMode;
use bevy::image::ImagePlugin;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::sprite::Anchor;
use bevy::time::TimeUpdateStrategy;
use bevy::window::WindowResolution;
use dark_castle_rpg::core::resources::project_root;
use dark_castle_rpg::player::visual::manifest::{AnimationManifest, ClipMode, anchor_for};
use dark_castle_rpg::player::visual::sockets::{SocketTable, place};
use serde::Deserialize;

const CLOSE_VIEW: f32 = 360.0;
const GAME_VIEW: f32 = 405.0;

fn generated_dir() -> PathBuf {
    project_root().join("art-src/n3-proof/generated")
}

#[derive(Deserialize)]
struct Loadout {
    body_light: String,
    body_heavy: String,
    cape: String,
    sword: String,
    spear: String,
    cell: (u32, u32),
    columns: u32,
    rows: u32,
    pivot: (f32, f32),
    cape_cell: (u32, u32),
    cape_pivot: (f32, f32),
    sword_grip: (f32, f32),
    spear_grip: (f32, f32),
    sword_size: (u32, u32),
    spear_size: (u32, u32),
}

#[derive(Resource)]
struct ProofAssets {
    manifest: AnimationManifest,
    sockets: SocketTable,
    loadout: Loadout,
    light: Handle<Image>,
    heavy: Handle<Image>,
    cape: Handle<Image>,
    sword: Handle<Image>,
    spear: Handle<Image>,
    body_layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource)]
struct Proof {
    clip: String,
    frame: usize,
    /// Seconds accumulated toward the next frame while playing.
    elapsed: f32,
    armor_heavy: bool,
    spear: bool,
    cape: bool,
    mirror: bool,
    /// Close view is 640×360 world units on a 1280×720 window: exactly 2×.
    close: bool,
    paused: bool,
    /// 0 = N3 baseline, 1 = automatic pixel of the V2.2 attack, 2 = the raw render.
    source: u8,
}

#[derive(Component)]
struct BodyLayer;

#[derive(Component)]
struct CapeLayerMark;

#[derive(Component)]
struct WeaponLayer;

#[derive(Component)]
struct ProofHud;

struct Shot {
    clip: &'static str,
    frame: usize,
    heavy: bool,
    spear: bool,
    cape: bool,
    mirror: bool,
    close: bool,
    file: &'static str,
    source: u8,
    folder: &'static str,
}

#[derive(Resource)]
struct Script {
    interactive: bool,
    shots: Vec<Shot>,
    index: usize,
    /// 0 waiting for textures, 1 apply, 2 let the sprite render, 3 capture.
    phase: u8,
    ready_frames: u8,
}

fn main() {
    let interactive = std::env::var_os("N3_INTERACTIVE").is_some();
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "N3 visual proof".into(),
                        resolution: WindowResolution::new(1280, 720),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    file_path: project_root().display().to_string(),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::srgb(0.07, 0.05, 0.08)))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(
            16,
        )))
        .insert_resource(Proof {
            clip: "idle".into(),
            frame: 0,
            elapsed: 0.0,
            armor_heavy: false,
            spear: false,
            cape: true,
            mirror: false,
            close: true,
            paused: true,
            source: 0,
        })
        .insert_resource(Script {
            interactive,
            shots: shots(),
            index: 0,
            phase: 0,
            ready_frames: 0,
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (wait_or_drive, read_keys, sync, capture, finish).chain(),
        )
        .run();
}

fn shots() -> Vec<Shot> {
    let mut list = vec![
        shot("idle", 1, false, false, true, false, true, "bevy_idle.png"),
        shot(
            "walk",
            0,
            false,
            false,
            true,
            false,
            true,
            "bevy_walk_00.png",
        ),
        shot(
            "walk",
            1,
            false,
            false,
            true,
            false,
            true,
            "bevy_walk_01.png",
        ),
        shot(
            "walk",
            2,
            false,
            false,
            true,
            false,
            true,
            "bevy_walk_02.png",
        ),
        shot(
            "walk",
            3,
            false,
            false,
            true,
            false,
            true,
            "bevy_walk_03.png",
        ),
        shot(
            "walk",
            4,
            false,
            false,
            true,
            false,
            true,
            "bevy_walk_04.png",
        ),
        shot(
            "walk",
            5,
            false,
            false,
            true,
            false,
            true,
            "bevy_walk_05.png",
        ),
        shot("jump", 1, false, false, true, false, true, "bevy_jump.png"),
        shot(
            "attack",
            0,
            false,
            false,
            true,
            false,
            true,
            "bevy_attack_windup.png",
        ),
        shot(
            "attack",
            2,
            false,
            false,
            true,
            false,
            true,
            "bevy_attack.png",
        ),
        shot(
            "attack",
            2,
            true,
            false,
            true,
            false,
            true,
            "bevy_armor_heavy.png",
        ),
        shot(
            "attack",
            2,
            false,
            true,
            true,
            false,
            true,
            "bevy_weapon_spear.png",
        ),
        shot(
            "attack",
            2,
            false,
            false,
            false,
            false,
            true,
            "bevy_cape_off.png",
        ),
        shot(
            "attack",
            2,
            false,
            false,
            true,
            true,
            true,
            "bevy_mirror.png",
        ),
        shot(
            "attack",
            2,
            false,
            false,
            true,
            false,
            false,
            "bevy_framing_game.png",
        ),
        shot(
            "attack",
            2,
            true,
            true,
            true,
            false,
            true,
            "bevy_framing_close.png",
        ),
        shot("idle", 1, false, false, true, false, true, "bevy_idle.png"),
    ];
    if project_root()
        .join("art-src/n3-proof/n3_1/attack_pixel.png")
        .is_file()
    {
        list.extend([
            render_shot(0, 1, true, "bevy_attack_pixel.png"),
            render_shot(0, 2, true, "bevy_attack_raw.png"),
            render_shot(3, 1, true, "bevy_attack_follow.png"),
            render_shot(6, 1, true, "bevy_attack_recover.png"),
            render_shot(0, 1, false, "bevy_attack_game_frame.png"),
        ]);
    }
    list
}

fn render_shot(frame: usize, source: u8, close: bool, file: &'static str) -> Shot {
    Shot {
        clip: "attack",
        frame,
        heavy: false,
        spear: false,
        cape: true,
        mirror: false,
        close,
        file,
        source,
        folder: "docs/development/n3_1/evidence",
    }
}

fn shot(
    clip: &'static str,
    frame: usize,
    heavy: bool,
    spear: bool,
    cape: bool,
    mirror: bool,
    close: bool,
    file: &'static str,
) -> Shot {
    Shot {
        clip,
        frame,
        heavy,
        spear,
        cape,
        mirror,
        close,
        file,
        source: 0,
        folder: "docs/development/n3/evidence",
    }
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let dir = generated_dir();
    let manifest_text = std::fs::read_to_string(dir.join("proof.ron"))
        .unwrap_or_else(|err| panic!("n3 manifest: {err}"));
    let manifest = AnimationManifest::from_ron(&manifest_text)
        .unwrap_or_else(|err| panic!("n3 manifest: {err}"));
    let socket_text = std::fs::read_to_string(dir.join("sockets.ron"))
        .unwrap_or_else(|err| panic!("n3 sockets: {err}"));
    let sockets = SocketTable::from_ron(&socket_text, &manifest)
        .unwrap_or_else(|err| panic!("n3 sockets: {err}"));
    let loadout: Loadout = ron::from_str(
        &std::fs::read_to_string(dir.join("loadout.ron"))
            .unwrap_or_else(|err| panic!("n3 loadout: {err}")),
    )
    .unwrap_or_else(|err| panic!("n3 loadout: {err}"));

    let body_layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(loadout.cell.0, loadout.cell.1),
        loadout.columns,
        loadout.rows,
        Some(UVec2::ZERO),
        None,
    ));
    let cape_layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(loadout.cape_cell.0, loadout.cape_cell.1),
        loadout.columns,
        loadout.rows,
        Some(UVec2::ZERO),
        None,
    ));
    let light = assets.load(format!("art-src/n3-proof/generated/{}", loadout.body_light));
    let heavy = assets.load(format!("art-src/n3-proof/generated/{}", loadout.body_heavy));
    let cape = assets.load(format!("art-src/n3-proof/generated/{}", loadout.cape));
    let sword = assets.load(format!("art-src/n3-proof/generated/{}", loadout.sword));
    let spear = assets.load(format!("art-src/n3-proof/generated/{}", loadout.spear));
    load_render(&mut commands, &assets, &mut layouts);

    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: CLOSE_VIEW,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    let (ax, ay) = anchor_for(loadout.pivot, loadout.cell, false);
    commands.spawn((
        BodyLayer,
        Sprite {
            image: light.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: body_layout.clone(),
                index: 0,
            }),
            custom_size: Some(Vec2::new(loadout.cell.0 as f32, loadout.cell.1 as f32)),
            ..default()
        },
        Anchor(Vec2::new(ax, ay)),
        Transform::from_xyz(0.0, 0.0, 2.0),
    ));
    let (cx, cy) = anchor_for(loadout.cape_pivot, loadout.cape_cell, false);
    commands.spawn((
        CapeLayerMark,
        Sprite {
            image: cape.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: cape_layout.clone(),
                index: 0,
            }),
            custom_size: Some(Vec2::new(
                loadout.cape_cell.0 as f32,
                loadout.cape_cell.1 as f32,
            )),
            ..default()
        },
        Anchor(Vec2::new(cx, cy)),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
    commands.spawn((
        WeaponLayer,
        Sprite {
            image: sword.clone(),
            custom_size: Some(Vec2::new(
                loadout.sword_size.0 as f32,
                loadout.sword_size.1 as f32,
            )),
            ..default()
        },
        Anchor(Vec2::ZERO),
        Transform::from_xyz(0.0, 0.0, 3.0),
    ));
    commands.spawn((
        ProofHud,
        Text::new("n3"),
        TextFont {
            font_size: FontSize::Px(18.0),
            ..default()
        },
        TextColor(Color::srgb(0.93, 0.9, 0.84)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));

    commands.insert_resource(ProofAssets {
        manifest,
        sockets,
        loadout,
        light,
        heavy,
        cape,
        sword,
        spear,
        body_layout,
    });
}

#[derive(Resource)]
struct RenderBank {
    manifest: AnimationManifest,
    sockets: SocketTable,
    pixel: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
    raw: Vec<Handle<Image>>,
}

fn load_render(
    commands: &mut Commands,
    assets: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
) {
    let dir = project_root().join("art-src/n3-proof/n3_1");
    let manifest_path = dir.join("attack.ron");
    if !manifest_path.is_file() {
        return;
    }
    let manifest = AnimationManifest::from_ron(
        &std::fs::read_to_string(&manifest_path)
            .unwrap_or_else(|err| panic!("n3.1 manifest: {err}")),
    )
    .unwrap_or_else(|err| panic!("n3.1 manifest: {err}"));
    let sockets = SocketTable::from_ron(
        &std::fs::read_to_string(dir.join("sockets.ron"))
            .unwrap_or_else(|err| panic!("n3.1 sockets: {err}")),
        &manifest,
    )
    .unwrap_or_else(|err| panic!("n3.1 sockets: {err}"));
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(256, 192),
        7,
        1,
        Some(UVec2::ZERO),
        None,
    ));
    let pixel = assets.load("art-src/n3-proof/n3_1/attack_pixel.png");
    let raw = (0..7)
        .map(|index| assets.load(format!("art-src/n3-proof/n3_1/raw/attack_{index:02}.png")))
        .collect();
    commands.insert_resource(RenderBank {
        manifest,
        sockets,
        pixel,
        layout,
        raw,
    });
}

fn wait_or_drive(
    assets: Res<AssetServer>,
    bank: Option<Res<ProofAssets>>,
    render: Option<Res<RenderBank>>,
    mut proof: ResMut<Proof>,
    mut script: ResMut<Script>,
    mut exit: MessageWriter<AppExit>,
) {
    let Some(bank) = bank else {
        return;
    };
    let handles = [
        &bank.light,
        &bank.heavy,
        &bank.cape,
        &bank.sword,
        &bank.spear,
    ];
    if handles.iter().any(|handle| {
        assets
            .get_load_state(*handle)
            .is_some_and(|state| state.is_failed())
    }) {
        eprintln!("[n3_visual_proof] a texture failed to load");
        exit.write(AppExit::error());
        return;
    }
    if script.interactive || script.phase > 0 {
        return;
    }
    if !handles.iter().all(|handle| assets.is_loaded(*handle)) {
        return;
    }
    if let Some(render) = render.as_ref() {
        let pending = !assets.is_loaded(&render.pixel)
            || render.raw.iter().any(|handle| !assets.is_loaded(handle));
        let failed = assets
            .get_load_state(&render.pixel)
            .is_some_and(|state| state.is_failed())
            || render.raw.iter().any(|handle| {
                assets
                    .get_load_state(handle)
                    .is_some_and(|state| state.is_failed())
            });
        if failed {
            eprintln!("[n3_visual_proof] an n3.1 texture failed to load");
            exit.write(AppExit::error());
            return;
        }
        if pending {
            return;
        }
    }
    script.ready_frames = script.ready_frames.saturating_add(1);
    if script.ready_frames < 12 {
        return;
    }
    apply_shot(&mut proof, &script.shots[0]);
    script.phase = 1;
}

fn apply_shot(proof: &mut Proof, shot: &Shot) {
    // The frame is chosen. Swapping the pieces below does not rewind it.
    proof.clip = shot.clip.to_string();
    proof.frame = shot.frame;
    proof.elapsed = 0.0;
    proof.armor_heavy = shot.heavy;
    proof.spear = shot.spear;
    proof.cape = shot.cape;
    proof.mirror = shot.mirror;
    proof.close = shot.close;
    proof.paused = true;
    proof.source = shot.source;
}

fn read_keys(
    keys: Res<ButtonInput<KeyCode>>,
    mut proof: ResMut<Proof>,
    script: Res<Script>,
    mut exit: MessageWriter<AppExit>,
) {
    if !script.interactive {
        return;
    }
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
        return;
    }
    let clip = if keys.just_pressed(KeyCode::Digit1) {
        Some("idle")
    } else if keys.just_pressed(KeyCode::Digit2) {
        Some("walk")
    } else if keys.just_pressed(KeyCode::Digit3) {
        Some("jump")
    } else if keys.just_pressed(KeyCode::Digit4) {
        Some("attack")
    } else {
        None
    };
    if let Some(clip) = clip {
        proof.clip = clip.to_string();
        proof.frame = 0;
        proof.elapsed = 0.0;
        proof.source = 0;
    }
    if keys.just_pressed(KeyCode::KeyZ) {
        proof.spear = !proof.spear;
    }
    if keys.just_pressed(KeyCode::KeyX) {
        proof.armor_heavy = !proof.armor_heavy;
    }
    if keys.just_pressed(KeyCode::KeyC) {
        proof.cape = !proof.cape;
    }
    if keys.just_pressed(KeyCode::KeyM) {
        proof.mirror = !proof.mirror;
    }
    if keys.just_pressed(KeyCode::KeyV) {
        proof.close = !proof.close;
    }
    if keys.just_pressed(KeyCode::KeyB) {
        proof.source = (proof.source + 1) % 3;
        proof.clip = "attack".into();
        proof.frame = 0;
        proof.elapsed = 0.0;
    }
    if keys.just_pressed(KeyCode::Space) {
        proof.paused = !proof.paused;
    }
    if keys.just_pressed(KeyCode::Period) && proof.paused {
        proof.frame = proof.frame.saturating_add(1);
    }
}

fn sync(
    mut proof: ResMut<Proof>,
    bank: Option<Res<ProofAssets>>,
    mut cameras: Query<&mut Projection, With<Camera2d>>,
    mut body: Query<
        (&mut Sprite, &mut Anchor, &mut Transform),
        (
            With<BodyLayer>,
            Without<CapeLayerMark>,
            Without<WeaponLayer>,
        ),
    >,
    mut cape: Query<
        (&mut Sprite, &mut Anchor, &mut Transform, &mut Visibility),
        (
            With<CapeLayerMark>,
            Without<BodyLayer>,
            Without<WeaponLayer>,
        ),
    >,
    mut weapon: Query<
        (&mut Sprite, &mut Anchor, &mut Transform),
        (
            With<WeaponLayer>,
            Without<BodyLayer>,
            Without<CapeLayerMark>,
        ),
    >,
    mut hud: Query<&mut Text, With<ProofHud>>,
    time: Res<Time>,
    render: Option<Res<RenderBank>>,
) {
    let Some(bank) = bank else {
        return;
    };
    if proof.source != 0 && render.is_none() {
        proof.source = 0;
    }
    let shown = if proof.source != 0 {
        let Some(render) = render.as_ref() else {
            return;
        };
        let Some(clip) = render.manifest.clips.get("attack") else {
            return;
        };
        (
            clip.count as usize,
            clip.first as usize,
            clip.pivot,
            clip.durations.first().copied().unwrap_or(0.1),
            false,
        )
    } else {
        let Some(clip) = bank.manifest.clips.get(&proof.clip) else {
            return;
        };
        (
            clip.count as usize,
            clip.first as usize,
            clip.pivot,
            clip.durations.first().copied().unwrap_or(0.1),
            matches!(clip.mode, ClipMode::Loop { .. }),
        )
    };
    let (count, first, pivot, step, looped) = shown;
    if !proof.paused && step > 0.0 {
        proof.elapsed += time.delta_secs();
        while proof.elapsed >= step {
            proof.elapsed -= step;
            proof.frame = proof.frame.saturating_add(1);
        }
    }
    if looped {
        if count > 0 {
            proof.frame %= count;
        }
    } else {
        proof.frame = proof.frame.min(count.saturating_sub(1));
    }
    let scale = if proof.close { 1.0 } else { 0.5 };
    let height = if proof.close { CLOSE_VIEW } else { GAME_VIEW };
    for mut projection in &mut cameras {
        if let Projection::Orthographic(ortho) = projection.as_mut() {
            ortho.scaling_mode = ScalingMode::FixedVertical {
                viewport_height: height,
            };
        }
    }
    let sign = if proof.mirror { -1.0 } else { 1.0 };
    let index = first + proof.frame;
    if let Ok((mut sprite, mut anchor, _transform)) = body.single_mut() {
        let cell = if proof.source == 0 {
            bank.loadout.cell
        } else {
            (256, 192)
        };
        if proof.source == 0 {
            sprite.image = if proof.armor_heavy {
                bank.heavy.clone()
            } else {
                bank.light.clone()
            };
            if let Some(atlas) = sprite.texture_atlas.as_mut() {
                atlas.layout = bank.body_layout.clone();
                atlas.index = index;
            }
        } else if let Some(render) = render.as_ref() {
            if proof.source == 2 {
                sprite.texture_atlas = None;
                let raw_index = proof.frame.min(render.raw.len().saturating_sub(1));
                sprite.image = render.raw[raw_index].clone();
            } else {
                sprite.image = render.pixel.clone();
                sprite.texture_atlas = Some(TextureAtlas {
                    layout: render.layout.clone(),
                    index,
                });
            }
        }
        sprite.flip_x = proof.mirror;
        sprite.custom_size = Some(Vec2::new(cell.0 as f32 * scale, cell.1 as f32 * scale));
        let (x, y) = anchor_for(pivot, cell, proof.mirror);
        anchor.0 = Vec2::new(x, y);
    }
    if let Ok((mut sprite, mut anchor, mut transform, mut visibility)) = cape.single_mut() {
        *visibility = if proof.cape && proof.source == 0 {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            atlas.index = index;
        }
        sprite.flip_x = proof.mirror;
        sprite.custom_size = Some(Vec2::new(
            bank.loadout.cape_cell.0 as f32 * scale,
            bank.loadout.cape_cell.1 as f32 * scale,
        ));
        let (x, y) = anchor_for(
            bank.loadout.cape_pivot,
            bank.loadout.cape_cell,
            proof.mirror,
        );
        anchor.0 = Vec2::new(x, y);
        if let Some(frame) = bank.sockets.frame(&proof.clip, proof.frame) {
            let (sx, sy, _) = place(frame.back.x, frame.back.y, 0.0, sign);
            transform.translation = Vec3::new(sx * scale, sy * scale, 1.0);
        }
    }
    if let Ok((mut sprite, mut anchor, mut transform)) = weapon.single_mut() {
        let (grip, size, image) = if proof.spear {
            (
                bank.loadout.spear_grip,
                bank.loadout.spear_size,
                bank.spear.clone(),
            )
        } else {
            (
                bank.loadout.sword_grip,
                bank.loadout.sword_size,
                bank.sword.clone(),
            )
        };
        sprite.image = image;
        sprite.flip_x = proof.mirror;
        sprite.custom_size = Some(Vec2::new(size.0 as f32 * scale, size.1 as f32 * scale));
        let x = grip.0 / size.0 as f32 - 0.5;
        let y = 0.5 - grip.1 / size.1 as f32;
        anchor.0 = Vec2::new(if proof.mirror { -x } else { x }, y);
        let socket_frame = if proof.source == 0 {
            bank.sockets.frame(&proof.clip, proof.frame)
        } else {
            render
                .as_ref()
                .and_then(|render| render.sockets.frame("attack", proof.frame))
        };
        if let Some(frame) = socket_frame {
            let (sx, sy, rotation) =
                place(frame.weapon.x, frame.weapon.y, frame.weapon.angle, sign);
            transform.translation = Vec3::new(
                sx * scale,
                sy * scale,
                if frame.weapon.front { 3.0 } else { 0.5 },
            );
            transform.rotation = Quat::from_rotation_z(rotation);
        }
    }
    if let Ok(mut text) = hud.single_mut() {
        let armor = if proof.armor_heavy { "heavy" } else { "light" };
        let weapon_name = match (proof.source, proof.spear) {
            (0, true) => "spear",
            (0, false) => "sword",
            _ => "N3 sword sprite on the measured socket",
        };
        let cape = if proof.source != 0 {
            "inside the render"
        } else if proof.cape {
            "on"
        } else {
            "off"
        };
        let facing = if proof.mirror { "left" } else { "right" };
        let view = if proof.close {
            "close-360-2x"
        } else {
            "game-405"
        };
        let paused = if proof.paused { "paused" } else { "playing" };
        let source = match proof.source {
            1 => "auto-pixel",
            2 => "raw-render",
            _ => "n3-baseline",
        };
        text.0 = format!(
            "{source}  clip {}  frame {}/{}  armor {}  weapon {}  cape {}  facing {}  view {}  {}",
            proof.clip, proof.frame, count, armor, weapon_name, cape, facing, view, paused
        );
    }
}

fn capture(mut script: ResMut<Script>, mut proof: ResMut<Proof>, mut commands: Commands) {
    if script.interactive || script.phase == 0 || script.phase >= 4 {
        return;
    }
    if script.phase == 1 {
        script.phase = 2;
        return;
    }
    if script.phase == 2 {
        script.phase = 3;
        return;
    }
    let shot = &script.shots[script.index];
    let path = project_root().join(shot.folder).join(shot.file);
    println!("[n3_visual_proof] {}", shot.file);
    commands
        .spawn(Screenshot::primary_window())
        .observe(save_to_disk(path));
    script.index += 1;
    if script.index >= script.shots.len() {
        // The window has to present once more or the last PNG is dropped.
        script.phase = 4;
        script.ready_frames = 0;
        return;
    }
    apply_shot(&mut proof, &script.shots[script.index]);
    script.phase = 1;
}

fn finish(mut script: ResMut<Script>, mut exit: MessageWriter<AppExit>) {
    if script.phase != 4 {
        return;
    }
    script.ready_frames = script.ready_frames.saturating_add(1);
    if script.ready_frames >= 4 {
        exit.write(AppExit::Success);
    }
}
