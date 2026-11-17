//! D12 experimental modular protagonist lab (Bevy 0.20.1).
//!
//! Loads `assets/experimental/d12/` refined frames, uses sole-pivot
//! [`Anchor`], world height 64 (aligned with published locomotion), contact
//! shadow, and after-image B. Screenshots go to
//! `art-src/visual-prototypes/d12_correcao_escala_pixel/evidencias_bevy/`.
//!
//! Does **not** touch the published `PlayerVisualPlugin` path.
//!
//! ```text
//! cargo run --example modular_visual_lab
//! ```

use std::path::PathBuf;
use std::time::Duration;

use bevy::asset::RenderAssetUsages;
use bevy::camera::ScalingMode;
use bevy::image::ImageSampler;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::sprite::Anchor;
use bevy::time::TimeUpdateStrategy;
use bevy::window::WindowResolution;
use dark_castle_rpg::core::resources::project_root;
use dark_castle_rpg::player::visual::modular_lab::{
    AFTERIMAGE_TINT, AfterImageTrail, CELL_H, CELL_W, ModularClip, ModularLoadout, PIVOT_SOLE,
    anchor_for_pivot, contact_shadow_air_scale, contact_shadow_alpha, contact_shadow_size,
    frame_index, ghost_alpha, should_spawn_ghost, world_size,
};

const FRAME: Duration = Duration::from_millis(16);
const VIEW_H: f32 = 405.0;

#[derive(Resource)]
struct LabBank {
    classic_idle: Vec<Handle<Image>>,
    classic_run: Vec<Handle<Image>>,
    heavy_idle: Vec<Handle<Image>>,
    heavy_run: Vec<Handle<Image>>,
}

#[derive(Resource)]
struct LabState {
    loadout: ModularLoadout,
    clip: ModularClip,
    elapsed: f32,
    facing_left: bool,
    since_ghost: f32,
    trail: AfterImageTrail,
    ready: bool,
    script_t: f32,
    shots_done: u32,
    airborne: bool,
}

#[derive(Component)]
struct ProtagonistSprite;

#[derive(Component)]
struct ContactShadow;

#[derive(Component)]
struct Ghost {
    age: f32,
    lifetime: f32,
    peak_alpha: f32,
}

#[derive(Resource)]
struct ShotDir(PathBuf);

fn main() {
    let out =
        project_root().join("art-src/visual-prototypes/d12_correcao_escala_pixel/evidencias_bevy");
    if let Err(e) = std::fs::create_dir_all(&out) {
        eprintln!("could not create evidencias_bevy: {e}");
        std::process::exit(1);
    }

    App::new()
        .insert_resource(TimeUpdateStrategy::ManualDuration(FRAME))
        .insert_resource(ClearColor(Color::srgb(0.10, 0.09, 0.12)))
        .insert_resource(LabState {
            loadout: ModularLoadout::Classic,
            clip: ModularClip::Idle,
            elapsed: 0.0,
            facing_left: false,
            since_ghost: 0.0,
            trail: AfterImageTrail::default(),
            ready: false,
            script_t: 0.0,
            shots_done: 0,
            airborne: false,
        })
        .insert_resource(ShotDir(out))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "D12 modular visual lab".into(),
                        resolution: WindowResolution::new(1280, 720),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    file_path: project_root().join("assets").display().to_string(),
                    ..default()
                }),
        )
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                mark_ready,
                advance_time,
                apply_script,
                sync_sprite,
                sync_shadow,
                spawn_ghosts,
                tick_ghosts,
                take_scripted_shots,
            )
                .chain(),
        )
        .run();
}

fn setup(mut commands: Commands, assets: Res<AssetServer>, mut images: ResMut<Assets<Image>>) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: VIEW_H,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    let ground = images.add(solid(1280, 4, [48, 44, 56, 255]));
    commands.spawn((
        Sprite {
            image: ground,
            custom_size: Some(Vec2::new(640.0, 2.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    let bank = LabBank {
        classic_idle: load_clip(&assets, "classico", "idle", 6),
        classic_run: load_clip(&assets, "classico", "run", 8),
        heavy_idle: load_clip(&assets, "pesado", "idle", 6),
        heavy_run: load_clip(&assets, "pesado", "run", 8),
    };

    let (ww, wh) = world_size();
    let (ax, ay) = anchor_for_pivot(PIVOT_SOLE, (CELL_W, CELL_H), false);
    let (sw, sh) = contact_shadow_size(ww);
    let shadow_img = images.add(ellipse_shadow(64, 16));

    commands.spawn((
        ContactShadow,
        Sprite {
            image: shadow_img,
            custom_size: Some(Vec2::new(sw, sh)),
            color: Color::srgba(0.02, 0.015, 0.03, contact_shadow_alpha(false)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.5),
        Anchor(Vec2::new(0.0, 0.0)),
    ));

    commands.spawn((
        ProtagonistSprite,
        Sprite {
            image: bank.classic_idle[0].clone(),
            custom_size: Some(Vec2::new(ww, wh)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        Anchor(Vec2::new(ax, ay)),
    ));

    commands.insert_resource(bank);
}

fn load_clip(assets: &AssetServer, loadout: &str, clip: &str, n: usize) -> Vec<Handle<Image>> {
    (0..n)
        .map(|i| {
            let path = format!("experimental/d12/frames/{loadout}/{clip}/f{i:02}.png");
            assets.load(path)
        })
        .collect()
}

fn solid(w: u32, h: u32, rgba: [u8; 4]) -> Image {
    let mut img = Image::new(
        Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        rgba.iter()
            .cycle()
            .take((w * h * 4) as usize)
            .copied()
            .collect(),
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    img.sampler = ImageSampler::nearest();
    img
}

fn ellipse_shadow(w: u32, h: u32) -> Image {
    let mut rgba = vec![0u8; (w * h * 4) as usize];
    let cx = (w as f32) / 2.0;
    let cy = (h as f32) / 2.0;
    for y in 0..h {
        for x in 0..w {
            let nx = (x as f32 - cx) / (w as f32 / 2.0);
            let ny = (y as f32 - cy) / (h as f32 / 2.0);
            let d = nx * nx + ny * ny;
            if d <= 1.0 {
                let a = ((1.0 - d).powf(1.4) * 255.0) as u8;
                let i = ((y * w + x) * 4) as usize;
                rgba[i] = 8;
                rgba[i + 1] = 6;
                rgba[i + 2] = 10;
                rgba[i + 3] = a;
            }
        }
    }
    let mut img = Image::new(
        Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        rgba,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    img.sampler = ImageSampler::nearest();
    img
}

fn mark_ready(bank: Res<LabBank>, mut images: ResMut<Assets<Image>>, mut state: ResMut<LabState>) {
    if state.ready {
        return;
    }
    let handles: Vec<_> = bank
        .classic_idle
        .iter()
        .chain(bank.classic_run.iter())
        .chain(bank.heavy_idle.iter())
        .chain(bank.heavy_run.iter())
        .cloned()
        .collect();
    if !handles.iter().all(|h| images.contains(h)) {
        return;
    }
    for h in &handles {
        if let Some(mut img) = images.get_mut(h) {
            img.sampler = ImageSampler::nearest();
        }
    }
    state.ready = true;
}

fn advance_time(time: Res<Time>, mut state: ResMut<LabState>) {
    if !state.ready {
        return;
    }
    let dt = time.delta_secs();
    state.elapsed += dt;
    state.since_ghost += dt;
    state.script_t += dt;
}

fn apply_script(mut state: ResMut<LabState>) {
    if !state.ready {
        return;
    }
    let t = state.script_t;
    state.clip = if t < 1.2 || t >= 4.6 {
        ModularClip::Idle
    } else {
        ModularClip::Run
    };
    state.loadout = if (2.6..5.4).contains(&t) {
        ModularLoadout::Heavy
    } else {
        ModularLoadout::Classic
    };
    state.facing_left = (3.8..5.4).contains(&t);
    // Brief airborne window to show shadow shrink (visual only).
    state.airborne = (5.0..5.35).contains(&t);
}

fn pick_handle(bank: &LabBank, loadout: ModularLoadout, clip: ModularClip) -> &[Handle<Image>] {
    match (loadout, clip) {
        (ModularLoadout::Classic, ModularClip::Idle) => &bank.classic_idle,
        (ModularLoadout::Classic, ModularClip::Run) => &bank.classic_run,
        (ModularLoadout::Heavy, ModularClip::Idle) => &bank.heavy_idle,
        (ModularLoadout::Heavy, ModularClip::Run) => &bank.heavy_run,
    }
}

fn sync_sprite(
    time: Res<Time>,
    state: Res<LabState>,
    bank: Res<LabBank>,
    mut q: Query<(&mut Sprite, &mut Transform, &mut Anchor), With<ProtagonistSprite>>,
) {
    if !state.ready {
        return;
    }
    let Ok((mut sprite, mut tf, mut anchor)) = q.single_mut() else {
        return;
    };
    let frames = pick_handle(&bank, state.loadout, state.clip);
    let idx = frame_index(state.clip, state.elapsed);
    sprite.image = frames[idx].clone();
    sprite.flip_x = state.facing_left;
    let (ax, ay) = anchor_for_pivot(PIVOT_SOLE, (CELL_W, CELL_H), state.facing_left);
    *anchor = Anchor(Vec2::new(ax, ay));
    if state.clip == ModularClip::Run {
        let dir = if state.facing_left { -1.0 } else { 1.0 };
        tf.translation.x += dir * 28.0 * time.delta_secs();
    }
    tf.translation.y = if state.airborne { 28.0 } else { 0.0 };
}

fn sync_shadow(
    state: Res<LabState>,
    player: Query<&Transform, (With<ProtagonistSprite>, Without<ContactShadow>)>,
    mut shadow: Query<
        (&mut Transform, &mut Sprite),
        (With<ContactShadow>, Without<ProtagonistSprite>),
    >,
) {
    if !state.ready {
        return;
    }
    let Ok(pt) = player.single() else {
        return;
    };
    let Ok((mut tf, mut sprite)) = shadow.single_mut() else {
        return;
    };
    let (ww, _) = world_size();
    let (sw, sh) = contact_shadow_size(ww);
    let scale = contact_shadow_air_scale(state.airborne);
    tf.translation.x = pt.translation.x;
    tf.translation.y = 0.0; // stays on ground while player jumps
    tf.translation.z = 0.5;
    sprite.custom_size = Some(Vec2::new(sw * scale, sh * scale));
    sprite.color.set_alpha(contact_shadow_alpha(state.airborne));
}

fn spawn_ghosts(
    mut commands: Commands,
    mut state: ResMut<LabState>,
    bank: Res<LabBank>,
    q: Query<&Transform, With<ProtagonistSprite>>,
    ghosts: Query<(), With<Ghost>>,
) {
    if !state.ready {
        return;
    }
    let moving = state.clip == ModularClip::Run;
    let active = ghosts.iter().count();
    if !should_spawn_ghost(moving, state.since_ghost, active, state.trail) {
        return;
    }
    let Ok(tf) = q.single() else {
        return;
    };
    let frames = pick_handle(&bank, state.loadout, state.clip);
    let idx = frame_index(state.clip, state.elapsed);
    let (ww, wh) = world_size();
    let behind = if state.facing_left { 6.0 } else { -6.0 };
    let (ax, ay) = anchor_for_pivot(PIVOT_SOLE, (CELL_W, CELL_H), state.facing_left);
    let color = Color::srgba(
        AFTERIMAGE_TINT[0],
        AFTERIMAGE_TINT[1],
        AFTERIMAGE_TINT[2],
        AFTERIMAGE_TINT[3],
    );
    commands.spawn((
        Ghost {
            age: 0.0,
            lifetime: state.trail.lifetime,
            peak_alpha: AFTERIMAGE_TINT[3],
        },
        Sprite {
            image: frames[idx].clone(),
            custom_size: Some(Vec2::new(ww, wh)),
            color,
            flip_x: state.facing_left,
            ..default()
        },
        Transform::from_translation(tf.translation + Vec3::new(behind, 0.0, -0.1)),
        Anchor(Vec2::new(ax, ay)),
    ));
    state.since_ghost = 0.0;
}

fn tick_ghosts(
    mut commands: Commands,
    time: Res<Time>,
    mut q: Query<(Entity, &mut Ghost, &mut Sprite)>,
) {
    let dt = time.delta_secs();
    for (e, mut g, mut sprite) in &mut q {
        g.age += dt;
        let a = ghost_alpha(g.age, g.lifetime, g.peak_alpha);
        if a <= 0.0 {
            commands.entity(e).despawn();
            continue;
        }
        sprite.color.set_alpha(a);
    }
}

fn take_scripted_shots(
    mut commands: Commands,
    mut state: ResMut<LabState>,
    shot_dir: Res<ShotDir>,
    mut exit: MessageWriter<AppExit>,
) {
    if !state.ready {
        return;
    }
    let shots: &[(f32, &str)] = &[
        (0.80, "01_idle_classic"),
        (1.80, "02_run_classic_trail"),
        (3.20, "03_run_heavy_kept_clock"),
        (4.20, "04_heavy_facing_left"),
        (5.00, "05_idle_heavy_shadow"),
        (5.20, "06_airborne_shadow"),
        (5.80, "07_idle_classic_right"),
    ];
    for (i, (at, name)) in shots.iter().enumerate() {
        let bit = 1u32 << i;
        if state.script_t >= *at && state.shots_done & bit == 0 {
            let path = shot_dir.0.join(format!("{name}.png"));
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(path));
            state.shots_done |= bit;
        }
    }
    if state.script_t >= 6.40 {
        exit.write(AppExit::Success);
    }
}
