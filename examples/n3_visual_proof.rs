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
//! | B | N3 baseline / automatic pixel / raw render / N3.2 concept / N3.3 construction / N3.4 base / N3.5 fidelity / N3.6 run proof / N3.7 sync proof |
//! | R | N3.6 run proof: run6 / run10 / stand |
//! | F | N3.6 run proof: manifest timing / 8 / 10 / 12 fps |
//! | O | N3.6 and N3.7 proofs: pivot and bounding-box overlay on / off |
//! | T | N3.6 and N3.7 proofs: in place / moving across the view |
//! | G | N3.7 sync proof: scenario A original / B faster clip / C slower travel / D moderate |
//! | H | N3.7 sync proof: speed mode / idle -> start -> run -> idle transition |
//! | L | N3.7 sync proof: original / rim-light / lighter-boots frames |
//! | K | N3.7 sync proof: backdrop default / light / grey / near black / near black + back-light |
//! | V | close view (360, integer 2×) / game framing (405) / integer game framing (360, 1 texel = 1 px) |
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
const WINDOW_HEIGHT: f32 = 720.0;

/// Camera framing of the proof window. None of them touches `camera.ron`.
#[derive(Clone, Copy, PartialEq, Eq)]
enum View {
    /// 360 world units tall, sprites at density 1: one texel = 2 screen px.
    Close,
    /// The game's 405 framing, sprites at density 2.
    Game,
    /// N3.5 hypothesis: 360 framing, sprites at density 2, one texel = 1 px.
    GameInteger,
}

impl View {
    fn from_close(close: bool) -> Self {
        if close { Self::Close } else { Self::Game }
    }

    fn height(self) -> f32 {
        match self {
            Self::Close | Self::GameInteger => CLOSE_VIEW,
            Self::Game => GAME_VIEW,
        }
    }

    /// World units per texel (the inverse of the density).
    fn scale(self) -> f32 {
        match self {
            Self::Close => 1.0,
            Self::Game | Self::GameInteger => 0.5,
        }
    }

    fn screen_px_per_texel(self) -> f32 {
        WINDOW_HEIGHT / self.height() * self.scale()
    }

    fn label(self) -> &'static str {
        match self {
            Self::Close => "close-360-2x",
            Self::Game => "game-405",
            Self::GameInteger => "game-360-1x",
        }
    }

    fn next(self) -> Self {
        match self {
            Self::Close => Self::Game,
            Self::Game => Self::GameInteger,
            Self::GameInteger => Self::Close,
        }
    }
}

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
    view: View,
    paused: bool,
    /// 0 = N3 baseline, 1 = automatic pixel of the V2.2 attack, 2 = the raw
    /// render, 3 = N3.2 concept, 4 = N3.3 construction, 5 = N3.4 base body,
    /// 6 = N3.5 fidelity candidate, 7 = N3.6 run proof.
    source: u8,
    /// N3.6 run proof: index into `RUN_FPS_STEPS`.
    run_fps: usize,
    /// N3.6 run proof: pivot and bounding-box overlay.
    overlay: bool,
    /// N3.6 run proof: the body travels across the view instead of in place.
    moving: bool,
    /// N3.6 run proof: horizontal world offset while moving. N3.7: game
    /// units travelled (drawn at 2 texels per unit).
    travel: f32,
    /// N3.7 sync proof: index into `sync.ron` scenarios.
    sync_scenario: usize,
    /// N3.7 sync proof: index into `SYNC_VARIANTS`.
    sync_variant: usize,
    /// N3.7 sync proof: index into `SYNC_BACKDROPS`.
    sync_backdrop: usize,
    /// N3.7 sync proof: idle -> start -> run -> idle loop instead of the run alone.
    sync_transition: bool,
    sync_stage: Stage,
    /// Seconds spent in the idle stage.
    sync_stage_time: f32,
    /// Run frames shown since the run stage began.
    sync_run_frames: usize,
    /// One frame step requested with `.` while paused.
    sync_step: bool,
    /// Pivot and opaque box of the shown N3.7 frame, for the overlay.
    sync_overlay: Option<((f32, f32), Option<(u32, u32, u32, u32)>)>,
}

/// Stage of the N3.7 transition loop.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Stage {
    /// The N3.5 idle candidate.
    Idle,
    /// The real start of the video (`run_start`).
    Start,
    /// The approved run6 cycle.
    Run,
}

/// Source index of the N3.7 synchronization proof (no cape, no weapon).
const SYNC_SOURCE: u8 = 8;
const SYNC_DIR: &str = "art-src/n3-7-run-sync/generated";
const START_DIR: &str = "art-src/n3-7-run-sync/pipeline/run_start/generated";
const DEFAULT_CLEAR: Color = Color::srgb(0.07, 0.05, 0.08);
const NEAR_BLACK: Color = Color::srgb(0.071, 0.055, 0.086);
/// Run frames the L key cycles through (new files; run6 stays untouched).
const SYNC_VARIANTS: [&str; 3] = ["original", "rim-light", "lighter-boots"];
/// Backdrops the K key cycles through; the last adds the back-light plate.
const SYNC_BACKDROPS: [(&str, Color); 5] = [
    ("default", DEFAULT_CLEAR),
    ("light", Color::srgb(0.886, 0.878, 0.894)),
    ("grey", Color::srgb(0.588, 0.588, 0.612)),
    ("near-black", NEAR_BLACK),
    ("near-black+back-light", NEAR_BLACK),
];
const BACKLIGHT_BACKDROP: usize = 4;
/// Ground ticks every 20 game units, so a sliding foot is visible.
const TICK_SPACING_UNITS: f32 = 20.0;
const TICK_COUNT: i32 = 41;
/// Texels per game unit of the run art (density 2).
const RUN_DENSITY: f32 = 2.0;

/// Source index of the N3.6 run proof (no cape, no weapon).
const RUN_SOURCE: u8 = 7;
const RUN_DIR: &str = "art-src/n3-6-animation-proof/generated";
/// Frame-rate overrides the run proof cycles through (`None` = manifest).
const RUN_FPS_STEPS: [Option<f32>; 4] = [None, Some(8.0), Some(10.0), Some(12.0)];
/// Run clips the R key cycles through.
const RUN_CLIPS: [&str; 3] = ["run6", "run10", "idle"];
/// World units per second at which the planted sole slides back in the
/// video cycle (estimate from `metrics.json`); used by the moving mode.
const RUN_TRAVEL_SPEED: f32 = 78.0;
/// The moving mode wraps around inside this half-width (world units).
const RUN_TRAVEL_HALF_WIDTH: f32 = 260.0;

/// Source index of the N3.4 base body (no cape, no weapon).
const BASE_SOURCE: u8 = 5;
const BASE_SPRITE: &str = "art-src/n3-4-original-protagonist/generated/base_idle.png";
/// Source index of the N3.5 fidelity candidate (no cape, no weapon).
const FIDELITY_SOURCE: u8 = 6;
const FIDELITY_SPRITE: &str = "art-src/n3-5-fidelity/generated/candidate_106.png";
/// Opaque height of both single-frame bodies, in texels.
const BODY_FIGURE_TEXELS: u32 = 106;

/// The single-frame bodies without equipment (N3.4 and N3.5).
fn is_base(source: u8) -> bool {
    source == BASE_SOURCE || source == FIDELITY_SOURCE
}

/// Sources drawn without cape or weapon.
fn without_equipment(source: u8) -> bool {
    is_base(source) || source == RUN_SOURCE || source == SYNC_SOURCE
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
    view: View,
    file: &'static str,
    source: u8,
    folder: &'static str,
    /// N3.7 settings of the capture (`None` for the older sources).
    sync: Option<SyncShot>,
}

#[derive(Clone, Copy)]
struct SyncShot {
    scenario: usize,
    variant: usize,
    backdrop: usize,
    /// `Some` captures the transition loop at that stage.
    stage: Option<Stage>,
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
        .insert_resource(ClearColor(DEFAULT_CLEAR))
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
            view: View::Close,
            paused: true,
            source: 0,
            run_fps: 0,
            overlay: true,
            moving: false,
            travel: 0.0,
            sync_scenario: 0,
            sync_variant: 0,
            sync_backdrop: 0,
            sync_transition: false,
            sync_stage: Stage::Run,
            sync_stage_time: 0.0,
            sync_run_frames: 0,
            sync_step: false,
            sync_overlay: None,
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
            (
                wait_or_drive,
                read_keys,
                sync,
                sync_n37,
                draw_run_overlay,
                capture,
                finish,
            )
                .chain(),
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
    if project_root()
        .join("art-src/n3-2-art-direction/generated/concept_b_sprite.png")
        .is_file()
    {
        list.extend([
            concept_shot(true, "bevy_concept_b.png"),
            concept_shot(false, "bevy_concept_b_game.png"),
        ]);
    }
    if project_root()
        .join("art-src/n3-3-native-pixel/generated/body_idle.png")
        .is_file()
    {
        list.push(native_shot());
    }
    if project_root().join(BASE_SPRITE).is_file() {
        list.push(base_shot(true, "bevy_n34.png"));
        list.push(base_shot(false, "bevy_n34_game.png"));
    }
    if project_root().join(FIDELITY_SPRITE).is_file() {
        list.push(fidelity_shot(FIDELITY_SOURCE, View::Close, "bevy_n35.png"));
        list.push(fidelity_shot(
            FIDELITY_SOURCE,
            View::Game,
            "bevy_n35_405.png",
        ));
        list.push(fidelity_shot(
            FIDELITY_SOURCE,
            View::GameInteger,
            "bevy_n35_360.png",
        ));
        list.push(fidelity_shot(BASE_SOURCE, View::Game, "bevy_n34_405.png"));
        list.push(fidelity_shot(
            BASE_SOURCE,
            View::GameInteger,
            "bevy_n34_360.png",
        ));
    }
    if project_root().join(RUN_DIR).join("run.ron").is_file() {
        list.push(run_shot(View::Close, "bevy_n36.png"));
        list.push(run_shot(View::Game, "bevy_n36_405.png"));
        list.push(run_shot(View::GameInteger, "bevy_n36_360.png"));
        // Every run6 frame on the integer view, for the frame-by-frame sheet
        // (not committed; `build_run.py` cuts them into one evidence image).
        for (frame, file) in RUN_FRAME_SHOTS.iter().enumerate() {
            list.push(Shot {
                frame,
                folder: "target/n3_6_bevy_frames",
                ..run_shot(View::GameInteger, file)
            });
        }
    }
    if project_root().join(SYNC_DIR).join("sync.ron").is_file() {
        let base = SyncShot {
            scenario: 0,
            variant: 0,
            backdrop: 0,
            stage: None,
        };
        // Scenario D (moderate), stride frame, overlay on.
        list.push(sync_shot(
            View::Close,
            "bevy_n37.png",
            SyncShot {
                scenario: 3,
                ..base
            },
            2,
        ));
        list.push(sync_shot(View::Game, "camera_405.png", base, 2));
        list.push(sync_shot(View::GameInteger, "camera_360.png", base, 2));
        list.push(sync_shot(
            View::GameInteger,
            "bevy_n37_rim.png",
            SyncShot {
                variant: 1,
                backdrop: 3,
                ..base
            },
            2,
        ));
        list.push(sync_shot(
            View::GameInteger,
            "bevy_n37_backlight.png",
            SyncShot {
                backdrop: BACKLIGHT_BACKDROP,
                ..base
            },
            2,
        ));
        list.push(sync_shot(
            View::Close,
            "bevy_n37_transition.png",
            SyncShot {
                stage: Some(Stage::Start),
                ..base
            },
            2,
        ));
    }
    list
}

/// N3.7 captures: paused, the body at the origin.
fn sync_shot(view: View, file: &'static str, sync: SyncShot, frame: usize) -> Shot {
    Shot {
        clip: "run6",
        frame,
        heavy: false,
        spear: false,
        cape: false,
        mirror: false,
        view,
        file,
        source: SYNC_SOURCE,
        folder: "docs/development/n3_7/evidence",
        sync: Some(sync),
    }
}

const RUN_FRAME_SHOTS: [&str; 6] = [
    "run6_0.png",
    "run6_1.png",
    "run6_2.png",
    "run6_3.png",
    "run6_4.png",
    "run6_5.png",
];

/// N3.6 captures: run6 frame 2 (stride), paused, under each view.
fn run_shot(view: View, file: &'static str) -> Shot {
    Shot {
        clip: "run6",
        frame: 2,
        heavy: false,
        spear: false,
        cape: false,
        mirror: false,
        view,
        file,
        source: RUN_SOURCE,
        folder: "docs/development/n3_6/evidence",
        sync: None,
    }
}

/// N3.5 captures: the candidate and the N3.4 body under the same views.
fn fidelity_shot(source: u8, view: View, file: &'static str) -> Shot {
    Shot {
        clip: "idle",
        frame: 0,
        heavy: false,
        spear: false,
        cape: false,
        mirror: false,
        view,
        file,
        source,
        folder: "docs/development/n3_5/evidence",
        sync: None,
    }
}

fn base_shot(close: bool, file: &'static str) -> Shot {
    Shot {
        clip: "idle",
        frame: 0,
        heavy: false,
        spear: false,
        cape: false,
        mirror: false,
        view: View::from_close(close),
        file,
        source: BASE_SOURCE,
        folder: "docs/development/n3_4/evidence",
        sync: None,
    }
}

fn native_shot() -> Shot {
    Shot {
        clip: "idle",
        frame: 0,
        heavy: false,
        spear: false,
        cape: true,
        mirror: false,
        view: View::Close,
        file: "bevy_n33.png",
        source: 4,
        folder: "docs/development/n3_3/evidence",
        sync: None,
    }
}

fn concept_shot(close: bool, file: &'static str) -> Shot {
    Shot {
        clip: "idle",
        frame: 0,
        heavy: false,
        spear: false,
        cape: false,
        mirror: false,
        view: View::from_close(close),
        file,
        source: 3,
        folder: "docs/development/n3_2/evidence",
        sync: None,
    }
}

fn render_shot(frame: usize, source: u8, close: bool, file: &'static str) -> Shot {
    Shot {
        clip: "attack",
        frame,
        heavy: false,
        spear: false,
        cape: true,
        mirror: false,
        view: View::from_close(close),
        file,
        source,
        folder: "docs/development/n3_1/evidence",
        sync: None,
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
        view: View::from_close(close),
        file,
        source: 0,
        folder: "docs/development/n3/evidence",
        sync: None,
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
    load_concept(&mut commands, &assets);
    load_native(&mut commands, &assets);
    load_base(&mut commands, &assets);
    load_run(&mut commands, &assets, &mut layouts);
    load_sync(&mut commands, &assets, &mut layouts);

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

#[derive(Resource)]
struct ConceptView {
    manifest: AnimationManifest,
    image: Handle<Image>,
}

fn load_concept(commands: &mut Commands, assets: &AssetServer) {
    let path = project_root().join("art-src/n3-2-art-direction/generated/concept_b.ron");
    if !path.is_file() {
        return;
    }
    let manifest = AnimationManifest::from_ron(
        &std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("n3.2 manifest: {err}")),
    )
    .unwrap_or_else(|err| panic!("n3.2 manifest: {err}"));
    let image = assets.load("art-src/n3-2-art-direction/generated/concept_b_sprite.png");
    commands.insert_resource(ConceptView { manifest, image });
}

#[derive(Resource)]
struct NativeBank {
    manifest: AnimationManifest,
    sockets: SocketTable,
    body: Handle<Image>,
    cape: Handle<Image>,
    sword: Handle<Image>,
}

fn load_native(commands: &mut Commands, assets: &AssetServer) {
    let dir = project_root().join("art-src/n3-3-native-pixel/generated");
    let path = dir.join("native.ron");
    if !path.is_file() {
        return;
    }
    let manifest = AnimationManifest::from_ron(
        &std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("n3.3 manifest: {err}")),
    )
    .unwrap_or_else(|err| panic!("n3.3 manifest: {err}"));
    let sockets = SocketTable::from_ron(
        &std::fs::read_to_string(dir.join("sockets.ron"))
            .unwrap_or_else(|err| panic!("n3.3 sockets: {err}")),
        &manifest,
    )
    .unwrap_or_else(|err| panic!("n3.3 sockets: {err}"));
    commands.insert_resource(NativeBank {
        manifest,
        sockets,
        body: assets.load("art-src/n3-3-native-pixel/generated/body_idle.png"),
        cape: assets.load("art-src/n3-3-native-pixel/generated/cape_idle.png"),
        sword: assets.load("art-src/n3-3-native-pixel/generated/sword.png"),
    });
}

struct BaseEntry {
    source: u8,
    name: &'static str,
    manifest: AnimationManifest,
    body: Handle<Image>,
}

/// Single-frame bodies without equipment, one entry per source.
#[derive(Resource)]
struct BaseBank {
    entries: Vec<BaseEntry>,
}

impl BaseBank {
    fn get(&self, source: u8) -> Option<&BaseEntry> {
        self.entries.iter().find(|entry| entry.source == source)
    }
}

fn load_base(commands: &mut Commands, assets: &AssetServer) {
    let candidates = [
        (
            BASE_SOURCE,
            "n3.4-base",
            "art-src/n3-4-original-protagonist/generated/base.ron",
            BASE_SPRITE,
        ),
        (
            FIDELITY_SOURCE,
            "n3.5-fidelity",
            "art-src/n3-5-fidelity/generated/candidate.ron",
            FIDELITY_SPRITE,
        ),
    ];
    let mut entries = Vec::new();
    for (source, name, manifest_path, sprite) in candidates {
        let path = project_root().join(manifest_path);
        if !path.is_file() {
            continue;
        }
        let manifest = AnimationManifest::from_ron(
            &std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("{name} manifest: {err}")),
        )
        .unwrap_or_else(|err| panic!("{name} manifest: {err}"));
        entries.push(BaseEntry {
            source,
            name,
            manifest,
            body: assets.load(sprite),
        });
    }
    if !entries.is_empty() {
        commands.insert_resource(BaseBank { entries });
    }
}

/// The N3.6 run proof: one manifest, one texture and grid layout per atlas,
/// and the opaque box of every frame for the overlay.
#[derive(Resource)]
struct RunBank {
    manifest: AnimationManifest,
    boxes: std::collections::BTreeMap<String, Vec<(u32, u32, u32, u32)>>,
    images: std::collections::BTreeMap<String, Handle<Image>>,
    layouts: std::collections::BTreeMap<String, Handle<TextureAtlasLayout>>,
}

fn load_run(
    commands: &mut Commands,
    assets: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
) {
    let dir = project_root().join(RUN_DIR);
    let path = dir.join("run.ron");
    if !path.is_file() {
        return;
    }
    let manifest = AnimationManifest::from_ron(
        &std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("n3.6 manifest: {err}")),
    )
    .unwrap_or_else(|err| panic!("n3.6 manifest: {err}"));
    let boxes = ron::from_str(
        &std::fs::read_to_string(dir.join("boxes.ron"))
            .unwrap_or_else(|err| panic!("n3.6 boxes: {err}")),
    )
    .unwrap_or_else(|err| panic!("n3.6 boxes: {err}"));
    let mut images = std::collections::BTreeMap::new();
    let mut grids = std::collections::BTreeMap::new();
    for (name, atlas) in &manifest.atlases {
        images.insert(
            name.clone(),
            assets.load(format!("{RUN_DIR}/{}", atlas.texture)),
        );
        grids.insert(
            name.clone(),
            layouts.add(TextureAtlasLayout::from_grid(
                UVec2::new(atlas.cell.0, atlas.cell.1),
                atlas.columns,
                atlas.rows,
                Some(UVec2::splat(atlas.padding)),
                None,
            )),
        );
    }
    spawn_overlay(commands);
    commands.insert_resource(RunBank {
        manifest,
        boxes,
        images,
        layouts: grids,
    });
}

/// One speed scenario of `sync.ron` (written by `sync_study.py`).
#[derive(Deserialize)]
struct SyncScenario {
    id: String,
    name: String,
    /// Playback factor over the authored 12 fps.
    rate: f32,
    /// Horizontal speed, game units per second.
    speed: f32,
}

#[derive(Deserialize)]
struct SyncPlan {
    /// Measured backward speed of the planted foot at the video's timing.
    reference_speed: f32,
    /// `PLAYER_RUN_SPEED` of the game (read only, never changed here).
    game_speed: f32,
    base_fps: f32,
    scenarios: Vec<SyncScenario>,
    run6_phase: Vec<String>,
}

/// `transition.ron` (written by `transition_study.py`).
#[derive(Deserialize)]
struct TransitionPlan {
    idle_hold: f32,
    /// Run frames shown before the stop may happen.
    run_frames: usize,
    /// run6 frame closest to the idle silhouette: the cut happens after it.
    exit_frame: usize,
    start_clip: String,
}

/// The N3.7 synchronization proof: scenarios, transition plan, the real
/// start of the video and the experimental run6 variants (new files).
#[derive(Resource)]
struct SyncBank {
    plan: SyncPlan,
    transition: TransitionPlan,
    start: AnimationManifest,
    start_boxes: std::collections::BTreeMap<String, Vec<(u32, u32, u32, u32)>>,
    start_image: Handle<Image>,
    start_layout: Handle<TextureAtlasLayout>,
    rim: Handle<Image>,
    boots: Handle<Image>,
    backlight: Handle<Image>,
}

impl SyncBank {
    fn handles(&self) -> [&Handle<Image>; 4] {
        [&self.start_image, &self.rim, &self.boots, &self.backlight]
    }
}

#[derive(Component)]
struct GroundTick(i32);

#[derive(Component)]
struct Backdrop;

fn read_ron<T: serde::de::DeserializeOwned>(path: &std::path::Path) -> T {
    ron::from_str(
        &std::fs::read_to_string(path).unwrap_or_else(|err| panic!("{}: {err}", path.display())),
    )
    .unwrap_or_else(|err| panic!("{}: {err}", path.display()))
}

fn load_sync(
    commands: &mut Commands,
    assets: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
) {
    let dir = project_root().join(SYNC_DIR);
    let start_dir = project_root().join(START_DIR);
    if !dir.join("sync.ron").is_file()
        || !dir.join("transition.ron").is_file()
        || !start_dir.join("run_start.ron").is_file()
    {
        return;
    }
    let plan: SyncPlan = read_ron(&dir.join("sync.ron"));
    let transition: TransitionPlan = read_ron(&dir.join("transition.ron"));
    let start = AnimationManifest::from_ron(
        &std::fs::read_to_string(start_dir.join("run_start.ron"))
            .unwrap_or_else(|err| panic!("run_start manifest: {err}")),
    )
    .unwrap_or_else(|err| panic!("run_start manifest: {err}"));
    let start_boxes = read_ron(&start_dir.join("boxes.ron"));
    let Some(atlas) = start
        .clips
        .get(&transition.start_clip)
        .and_then(|clip| start.atlases.get(&clip.atlas))
    else {
        panic!("run_start manifest: no {} clip", transition.start_clip);
    };
    let start_image = assets.load(format!("{START_DIR}/{}", atlas.texture));
    let start_layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(atlas.cell.0, atlas.cell.1),
        atlas.columns,
        atlas.rows,
        Some(UVec2::splat(atlas.padding)),
        None,
    ));
    let backlight = assets.load(format!("{SYNC_DIR}/backlight.png"));
    for k in 0..TICK_COUNT {
        commands.spawn((
            GroundTick(k - TICK_COUNT / 2),
            Sprite {
                color: Color::srgb(0.45, 0.42, 0.48),
                custom_size: Some(Vec2::ONE),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 4.0),
            Visibility::Hidden,
        ));
    }
    commands.spawn((
        Backdrop,
        Sprite {
            image: backlight.clone(),
            ..default()
        },
        Anchor(Vec2::ZERO),
        Transform::from_xyz(0.0, 0.0, 0.2),
        Visibility::Hidden,
    ));
    commands.insert_resource(SyncBank {
        plan,
        transition,
        start,
        start_boxes,
        start_image,
        start_layout,
        rim: assets.load(format!("{SYNC_DIR}/run6_rim.png")),
        boots: assets.load(format!("{SYNC_DIR}/run6_boots.png")),
        backlight,
    });
}

/// One bar of the run-proof overlay. The project builds Bevy without the
/// gizmo renderer, so the overlay is drawn with thin plain sprites.
#[derive(Component, Clone, Copy)]
enum OverlayBar {
    BoxTop,
    BoxBottom,
    BoxLeft,
    BoxRight,
    PivotHorizontal,
    PivotVertical,
    Ground,
}

const OVERLAY_BARS: [OverlayBar; 7] = [
    OverlayBar::BoxTop,
    OverlayBar::BoxBottom,
    OverlayBar::BoxLeft,
    OverlayBar::BoxRight,
    OverlayBar::PivotHorizontal,
    OverlayBar::PivotVertical,
    OverlayBar::Ground,
];

fn spawn_overlay(commands: &mut Commands) {
    for bar in OVERLAY_BARS {
        let color = match bar {
            OverlayBar::PivotHorizontal | OverlayBar::PivotVertical => Color::srgb(1.0, 0.2, 0.25),
            OverlayBar::Ground => Color::srgba(1.0, 0.2, 0.25, 0.45),
            _ => Color::srgb(0.3, 0.55, 1.0),
        };
        commands.spawn((
            bar,
            Sprite {
                color,
                custom_size: Some(Vec2::ONE),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 5.0),
            Visibility::Hidden,
        ));
    }
}

/// Pivot cross, ground line and opaque box of the shown run frame.
fn draw_run_overlay(
    proof: Res<Proof>,
    run: Option<Res<RunBank>>,
    body: Query<&Transform, (With<BodyLayer>, Without<OverlayBar>)>,
    mut bars: Query<(&OverlayBar, &mut Transform, &mut Sprite, &mut Visibility)>,
) {
    let shown = if proof.source == SYNC_SOURCE {
        proof.sync_overlay.filter(|_| proof.overlay)
    } else {
        (proof.source == RUN_SOURCE && proof.overlay)
            .then_some(())
            .and(run.as_ref())
            .and_then(|run| {
                let clip = run.manifest.clips.get(&proof.clip)?;
                let frame_box = *run.boxes.get(&proof.clip)?.get(proof.frame)?;
                Some((clip.pivot, Some(frame_box)))
            })
    };
    let (Some((pivot, frame_box)), Ok(body)) = (shown, body.single()) else {
        for (_, _, _, mut visibility) in &mut bars {
            *visibility = Visibility::Hidden;
        }
        return;
    };
    // The N3.7 idle stage has no recorded box: pivot and ground only.
    let (x0, y0, x1, y1) = frame_box.unwrap_or((0, 0, 0, 0));
    let scale = proof.view.scale();
    let sign = if proof.mirror { -1.0 } else { 1.0 };
    let origin = body.translation.truncate();
    let to_world =
        |x: f32, y: f32| origin + Vec2::new(sign * (x - pivot.0) * scale, (pivot.1 - y) * scale);
    let a = to_world(x0 as f32, y0 as f32);
    let b = to_world(x1 as f32 + 1.0, y1 as f32 + 1.0);
    let (left, right) = (a.x.min(b.x), a.x.max(b.x));
    let (bottom, top) = (a.y.min(b.y), a.y.max(b.y));
    let ground = to_world(pivot.0, pivot.1 + 1.0);
    let thin = scale;
    let arm = 6.0 * scale;
    for (bar, mut transform, mut sprite, mut visibility) in &mut bars {
        let (centre, size) = match bar {
            OverlayBar::BoxTop => (
                Vec2::new((left + right) / 2.0, top),
                Vec2::new(right - left, thin),
            ),
            OverlayBar::BoxBottom => (
                Vec2::new((left + right) / 2.0, bottom),
                Vec2::new(right - left, thin),
            ),
            OverlayBar::BoxLeft => (
                Vec2::new(left, (top + bottom) / 2.0),
                Vec2::new(thin, top - bottom),
            ),
            OverlayBar::BoxRight => (
                Vec2::new(right, (top + bottom) / 2.0),
                Vec2::new(thin, top - bottom),
            ),
            OverlayBar::PivotHorizontal => (ground, Vec2::new(2.0 * arm, thin)),
            OverlayBar::PivotVertical => (ground, Vec2::new(thin, 2.0 * arm)),
            OverlayBar::Ground => (
                Vec2::new(0.0, ground.y),
                Vec2::new(2.0 * (RUN_TRAVEL_HALF_WIDTH + 60.0), thin),
            ),
        };
        let is_box = matches!(
            bar,
            OverlayBar::BoxTop | OverlayBar::BoxBottom | OverlayBar::BoxLeft | OverlayBar::BoxRight
        );
        transform.translation = centre.extend(5.0);
        sprite.custom_size = Some(size);
        *visibility = if is_box && frame_box.is_none() {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        };
    }
}

fn wait_or_drive(
    assets: Res<AssetServer>,
    bank: Option<Res<ProofAssets>>,
    render: Option<Res<RenderBank>>,
    concept: Option<Res<ConceptView>>,
    native: Option<Res<NativeBank>>,
    base: Option<Res<BaseBank>>,
    run: Option<Res<RunBank>>,
    sync_bank: Option<Res<SyncBank>>,
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
    if let Some(concept) = concept.as_ref() {
        if assets
            .get_load_state(&concept.image)
            .is_some_and(|state| state.is_failed())
        {
            eprintln!("[n3_visual_proof] the n3.2 concept failed to load");
            exit.write(AppExit::error());
            return;
        }
        if !assets.is_loaded(&concept.image) {
            return;
        }
    }
    if let Some(native) = native.as_ref() {
        let handles = [&native.body, &native.cape, &native.sword];
        if handles.iter().any(|handle| {
            assets
                .get_load_state(*handle)
                .is_some_and(|state| state.is_failed())
        }) {
            eprintln!("[n3_visual_proof] the n3.3 construction failed to load");
            exit.write(AppExit::error());
            return;
        }
        if handles.iter().any(|handle| !assets.is_loaded(*handle)) {
            return;
        }
    }
    if let Some(base) = base.as_ref() {
        for entry in &base.entries {
            if assets
                .get_load_state(&entry.body)
                .is_some_and(|state| state.is_failed())
            {
                eprintln!("[n3_visual_proof] the {} body failed to load", entry.name);
                exit.write(AppExit::error());
                return;
            }
            if !assets.is_loaded(&entry.body) {
                return;
            }
        }
    }
    if let Some(run) = run.as_ref() {
        for handle in run.images.values() {
            if assets
                .get_load_state(handle)
                .is_some_and(|state| state.is_failed())
            {
                eprintln!("[n3_visual_proof] an n3.6 run texture failed to load");
                exit.write(AppExit::error());
                return;
            }
            if !assets.is_loaded(handle) {
                return;
            }
        }
    }
    if let Some(sync_bank) = sync_bank.as_ref() {
        for handle in sync_bank.handles() {
            if assets
                .get_load_state(handle)
                .is_some_and(|state| state.is_failed())
            {
                eprintln!("[n3_visual_proof] an n3.7 texture failed to load");
                exit.write(AppExit::error());
                return;
            }
            if !assets.is_loaded(handle) {
                return;
            }
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
    proof.view = shot.view;
    proof.paused = true;
    proof.source = shot.source;
    // Scripted run captures: overlay only in the close view, never moving.
    proof.overlay = shot.view == View::Close;
    proof.moving = false;
    proof.travel = 0.0;
    proof.run_fps = 0;
    let sync = shot.sync.unwrap_or(SyncShot {
        scenario: 0,
        variant: 0,
        backdrop: 0,
        stage: None,
    });
    proof.sync_scenario = sync.scenario;
    proof.sync_variant = sync.variant;
    proof.sync_backdrop = sync.backdrop;
    proof.sync_transition = sync.stage.is_some();
    proof.sync_stage = sync.stage.unwrap_or(Stage::Run);
    proof.sync_stage_time = 0.0;
    proof.sync_run_frames = 0;
    proof.sync_step = false;
    // N3.7 speed captures report the scenario's slide, so they count as moving.
    proof.moving = shot.sync.is_some_and(|sync| sync.stage.is_none());
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
        proof.view = proof.view.next();
    }
    if keys.just_pressed(KeyCode::KeyB) {
        let span = if project_root().join(SYNC_DIR).join("sync.ron").is_file() {
            9
        } else if project_root().join(RUN_DIR).join("run.ron").is_file() {
            8
        } else if project_root().join(FIDELITY_SPRITE).is_file() {
            7
        } else if project_root().join(BASE_SPRITE).is_file() {
            6
        } else if project_root()
            .join("art-src/n3-3-native-pixel/generated/body_idle.png")
            .is_file()
        {
            5
        } else if project_root()
            .join("art-src/n3-2-art-direction/generated/concept_b_sprite.png")
            .is_file()
        {
            4
        } else {
            3
        };
        proof.source = (proof.source + 1) % span;
        if proof.source == 1 || proof.source == 2 {
            proof.clip = "attack".into();
        } else if proof.source == RUN_SOURCE {
            proof.clip = RUN_CLIPS[0].into();
        } else if proof.source == SYNC_SOURCE {
            proof.clip = RUN_CLIPS[0].into();
            proof.moving = true;
            proof.travel = 0.0;
            proof.sync_transition = false;
            proof.sync_stage = Stage::Run;
            proof.sync_run_frames = 0;
        } else if proof.source == 0 {
            proof.clip = "idle".into();
        }
        proof.frame = 0;
        proof.elapsed = 0.0;
    }
    if proof.source == RUN_SOURCE {
        if keys.just_pressed(KeyCode::KeyR) {
            let next = RUN_CLIPS
                .iter()
                .position(|clip| *clip == proof.clip)
                .map_or(0, |i| (i + 1) % RUN_CLIPS.len());
            proof.clip = RUN_CLIPS[next].into();
            proof.frame = 0;
            proof.elapsed = 0.0;
        }
        if keys.just_pressed(KeyCode::KeyF) {
            proof.run_fps = (proof.run_fps + 1) % RUN_FPS_STEPS.len();
        }
        if keys.just_pressed(KeyCode::KeyO) {
            proof.overlay = !proof.overlay;
        }
        if keys.just_pressed(KeyCode::KeyT) {
            proof.moving = !proof.moving;
            proof.travel = 0.0;
        }
    }
    if proof.source == SYNC_SOURCE {
        if keys.just_pressed(KeyCode::KeyG) {
            proof.sync_scenario = proof.sync_scenario.saturating_add(1);
            proof.elapsed = 0.0;
        }
        if keys.just_pressed(KeyCode::KeyH) {
            proof.sync_transition = !proof.sync_transition;
            proof.sync_stage = if proof.sync_transition {
                Stage::Idle
            } else {
                Stage::Run
            };
            proof.sync_stage_time = 0.0;
            proof.sync_run_frames = 0;
            proof.frame = 0;
            proof.elapsed = 0.0;
            proof.travel = 0.0;
        }
        if keys.just_pressed(KeyCode::KeyL) {
            proof.sync_variant = (proof.sync_variant + 1) % SYNC_VARIANTS.len();
        }
        if keys.just_pressed(KeyCode::KeyK) {
            proof.sync_backdrop = (proof.sync_backdrop + 1) % SYNC_BACKDROPS.len();
        }
        if keys.just_pressed(KeyCode::KeyO) {
            proof.overlay = !proof.overlay;
        }
        if keys.just_pressed(KeyCode::KeyT) {
            proof.moving = !proof.moving;
            proof.travel = 0.0;
        }
    }
    if keys.just_pressed(KeyCode::Space) {
        proof.paused = !proof.paused;
    }
    if keys.just_pressed(KeyCode::Period) && proof.paused {
        if proof.source == SYNC_SOURCE {
            proof.sync_step = true;
        } else {
            proof.frame = proof.frame.saturating_add(1);
        }
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
    concept: Option<Res<ConceptView>>,
    native: Option<Res<NativeBank>>,
    base: Option<Res<BaseBank>>,
    run: Option<Res<RunBank>>,
) {
    let Some(bank) = bank else {
        return;
    };
    // The N3.7 proof is drawn by `sync_n37`.
    if proof.source == SYNC_SOURCE {
        return;
    }
    if matches!(proof.source, 1 | 2) && render.is_none() {
        proof.source = 0;
    }
    if proof.source == 3 && concept.is_none() {
        proof.source = 0;
    }
    if proof.source == 4 && native.is_none() {
        proof.source = 0;
    }
    let entry = base.as_ref().and_then(|base| base.get(proof.source));
    if is_base(proof.source) && entry.is_none() {
        proof.source = 0;
    }
    if proof.source == RUN_SOURCE && run.is_none() {
        proof.source = 0;
    }
    let run_clip = run
        .as_ref()
        .filter(|_| proof.source == RUN_SOURCE)
        .and_then(|run| run.manifest.clips.get(&proof.clip));
    if proof.source == RUN_SOURCE && run_clip.is_none() {
        proof.clip = RUN_CLIPS[0].into();
        return;
    }
    let shown = if let Some(clip) = run_clip {
        // Per-frame timing from the manifest, unless an fps override is set.
        let count = clip.count.max(1) as usize;
        let step = RUN_FPS_STEPS[proof.run_fps]
            .map_or_else(|| clip.durations[proof.frame % count], |fps| 1.0 / fps);
        (count, clip.first as usize, clip.pivot, step, true)
    } else if is_base(proof.source) {
        let Some(entry) = entry else {
            return;
        };
        let Some(clip) = entry.manifest.clips.get("idle") else {
            return;
        };
        (
            clip.count as usize,
            clip.first as usize,
            clip.pivot,
            1.0,
            true,
        )
    } else if proof.source == 4 {
        let Some(native) = native.as_ref() else {
            return;
        };
        let Some(clip) = native.manifest.clips.get("idle") else {
            return;
        };
        (
            clip.count as usize,
            clip.first as usize,
            clip.pivot,
            1.0,
            true,
        )
    } else if proof.source == 3 {
        let Some(concept) = concept.as_ref() else {
            return;
        };
        let Some(clip) = concept.manifest.clips.get("idle") else {
            return;
        };
        (
            clip.count as usize,
            clip.first as usize,
            clip.pivot,
            1.0,
            true,
        )
    } else if proof.source != 0 {
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
    let scale = proof.view.scale();
    let height = proof.view.height();
    for mut projection in &mut cameras {
        if let Projection::Orthographic(ortho) = projection.as_mut() {
            ortho.scaling_mode = ScalingMode::FixedVertical {
                viewport_height: height,
            };
        }
    }
    let sign = if proof.mirror { -1.0 } else { 1.0 };
    let index = first + proof.frame;
    if proof.source == RUN_SOURCE && proof.moving && !proof.paused {
        proof.travel += RUN_TRAVEL_SPEED * time.delta_secs();
        if proof.travel > RUN_TRAVEL_HALF_WIDTH {
            proof.travel -= 2.0 * RUN_TRAVEL_HALF_WIDTH;
        }
    }
    let travel = if proof.source == RUN_SOURCE && proof.moving {
        proof.travel * sign
    } else {
        0.0
    };
    let run_atlas = run_clip.map(|clip| clip.atlas.clone());
    if let Ok((mut sprite, mut anchor, mut transform)) = body.single_mut() {
        transform.translation.x = travel;
        let cell = if let Some(atlas) = run_atlas
            .as_ref()
            .and_then(|name| run.as_ref()?.manifest.atlases.get(name))
        {
            atlas.cell
        } else if proof.source == 0 {
            bank.loadout.cell
        } else if proof.source == 3 {
            concept
                .as_ref()
                .and_then(|concept| concept.manifest.atlases.get("concept"))
                .map(|atlas| atlas.cell)
                .unwrap_or((31, 100))
        } else if proof.source == 4 {
            native
                .as_ref()
                .and_then(|native| native.manifest.atlases.get("body"))
                .map(|atlas| atlas.cell)
                .unwrap_or((80, 128))
        } else if is_base(proof.source) {
            entry
                .and_then(|entry| entry.manifest.atlases.get("body"))
                .map(|atlas| atlas.cell)
                .unwrap_or((80, 128))
        } else {
            (256, 192)
        };
        if let Some((image, layout)) = run_atlas.as_ref().and_then(|name| {
            let run = run.as_ref()?;
            Some((
                run.images.get(name)?.clone(),
                run.layouts.get(name)?.clone(),
            ))
        }) {
            sprite.image = image;
            sprite.texture_atlas = Some(TextureAtlas { layout, index });
        } else if proof.source == 3 {
            if let Some(concept) = concept.as_ref() {
                sprite.texture_atlas = None;
                sprite.image = concept.image.clone();
            }
        } else if proof.source == 4 {
            if let Some(native) = native.as_ref() {
                sprite.texture_atlas = None;
                sprite.image = native.body.clone();
            }
        } else if is_base(proof.source) {
            if let Some(entry) = entry {
                sprite.texture_atlas = None;
                sprite.image = entry.body.clone();
            }
        } else if proof.source == 0 {
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
        *visibility = if proof.cape && (proof.source == 0 || proof.source == 4) {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if proof.source == 4 {
            if let Some(native) = native.as_ref() {
                let cell = native
                    .manifest
                    .atlases
                    .get("body")
                    .map(|atlas| atlas.cell)
                    .unwrap_or((80, 128));
                sprite.texture_atlas = None;
                sprite.image = native.cape.clone();
                sprite.flip_x = proof.mirror;
                sprite.custom_size = Some(Vec2::new(cell.0 as f32 * scale, cell.1 as f32 * scale));
                let (x, y) = anchor_for(pivot, cell, proof.mirror);
                anchor.0 = Vec2::new(x, y);
                transform.translation = Vec3::new(0.0, 0.0, 1.0);
            }
        } else {
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
    }
    if let Ok((mut sprite, mut anchor, mut transform)) = weapon.single_mut() {
        let (grip, size, image) = if proof.source == 4 {
            (
                (6.0, 5.0),
                (36, 10),
                native
                    .as_ref()
                    .map(|native| native.sword.clone())
                    .unwrap_or_else(|| bank.sword.clone()),
            )
        } else if proof.spear {
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
        // The N3.2 concept and the N3.4/N3.5 base bodies carry no weapon.
        sprite.custom_size = if proof.source == 3 || without_equipment(proof.source) {
            Some(Vec2::ZERO)
        } else {
            Some(Vec2::new(size.0 as f32 * scale, size.1 as f32 * scale))
        };
        let x = grip.0 / size.0 as f32 - 0.5;
        let y = 0.5 - grip.1 / size.1 as f32;
        anchor.0 = Vec2::new(if proof.mirror { -x } else { x }, y);
        let socket_frame = if proof.source == 4 {
            native
                .as_ref()
                .and_then(|native| native.sockets.frame("idle", proof.frame))
        } else if proof.source == 0 {
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
            (3, _) => "not a separate layer",
            (4, _) => "separate sword",
            (source, _) if without_equipment(source) => "none (base body)",
            _ => "N3 sword sprite on the measured socket",
        };
        let cape = if without_equipment(proof.source) {
            "none (base body)"
        } else if proof.source == 4 {
            if proof.cape { "separate layer" } else { "off" }
        } else if proof.source == 3 {
            "not a separate layer"
        } else if proof.source != 0 {
            "inside the render"
        } else if proof.cape {
            "on"
        } else {
            "off"
        };
        let facing = if proof.mirror { "left" } else { "right" };
        let view = proof.view.label();
        let paused = if proof.paused { "paused" } else { "playing" };
        let source = match proof.source {
            1 => "auto-pixel",
            2 => "raw-render",
            3 => "n3.2-concept-b",
            4 => "n3.3-construction",
            other if is_base(other) => entry.map_or("base", |entry| entry.name),
            RUN_SOURCE => "n3.6-run-proof",
            _ => "n3-baseline",
        };
        text.0 = format!(
            "{source}  clip {}  frame {}/{}  armor {}  weapon {}  cape {}  facing {}  view {}  {}",
            proof.clip, proof.frame, count, armor, weapon_name, cape, facing, view, paused
        );
        // Only the N3.5 source carries the art line, so the published N3.4
        // captures keep their HUD.
        if let Some(entry) = entry.filter(|_| proof.source == FIDELITY_SOURCE) {
            let cell = entry
                .manifest
                .atlases
                .get("body")
                .map_or((0, 0), |atlas| atlas.cell);
            text.0.push_str(&format!(
                "\nart {}x{}  figure {} texels  world {:.0} units  {:.0} texels/unit  camera {:.0}  {:.3} screen px/texel  NOT APPROVED",
                cell.0,
                cell.1,
                BODY_FIGURE_TEXELS,
                BODY_FIGURE_TEXELS as f32 * scale,
                1.0 / scale,
                height,
                proof.view.screen_px_per_texel(),
            ));
        }
        if let Some(clip) = run_clip {
            let cell = run_atlas
                .as_ref()
                .and_then(|name| run.as_ref()?.manifest.atlases.get(name))
                .map_or((0, 0), |atlas| atlas.cell);
            let timing = RUN_FPS_STEPS[proof.run_fps].map_or_else(
                || format!("manifest ({:.3} s cycle)", clip.total_duration()),
                |fps| format!("{fps:.0} fps"),
            );
            text.0.push_str(&format!(
                "\nart {}x{}  figure {} texels  {:.0} texels/unit  camera {:.0}  {:.3} screen px/texel  timing {timing}  {}  overlay {}  NOT APPROVED",
                cell.0,
                cell.1,
                BODY_FIGURE_TEXELS,
                1.0 / scale,
                height,
                proof.view.screen_px_per_texel(),
                if proof.moving { "moving" } else { "in place" },
                if proof.overlay { "on" } else { "off" },
            ));
        }
    }
}

/// One frame step of the N3.7 proof. In the transition loop the stop is a
/// direct cut after the exit frame (the video has no stop frames).
fn sync_advance(proof: &mut Proof, start_count: usize, run_count: usize, plan: &TransitionPlan) {
    match proof.sync_stage {
        Stage::Idle => {
            proof.sync_stage = Stage::Start;
            proof.frame = 0;
        }
        Stage::Start => {
            proof.frame += 1;
            if proof.frame >= start_count {
                proof.sync_stage = Stage::Run;
                proof.frame = 0;
                proof.sync_run_frames = 0;
            }
        }
        Stage::Run => {
            let leaving = proof.frame % run_count.max(1);
            proof.sync_run_frames = proof.sync_run_frames.saturating_add(1);
            if proof.sync_transition
                && proof.sync_run_frames >= plan.run_frames
                && leaving == plan.exit_frame
            {
                proof.sync_stage = Stage::Idle;
                proof.sync_stage_time = 0.0;
                proof.frame = 0;
            } else {
                proof.frame = (leaving + 1) % run_count.max(1);
            }
        }
    }
}

/// The N3.7 sync proof: speed scenarios, transition loop, contrast variants
/// and backdrops. Every other source is left to `sync`.
fn sync_n37(
    mut proof: ResMut<Proof>,
    sync_bank: Option<Res<SyncBank>>,
    run: Option<Res<RunBank>>,
    base: Option<Res<BaseBank>>,
    time: Res<Time>,
    mut clear: ResMut<ClearColor>,
    mut cameras: Query<&mut Projection, With<Camera2d>>,
    mut body: Query<
        (&mut Sprite, &mut Anchor, &mut Transform),
        (With<BodyLayer>, Without<WeaponLayer>, Without<Backdrop>),
    >,
    mut weapon: Query<&mut Sprite, (With<WeaponLayer>, Without<BodyLayer>, Without<Backdrop>)>,
    mut cape: Query<&mut Visibility, (With<CapeLayerMark>, Without<GroundTick>, Without<Backdrop>)>,
    mut ticks: Query<
        (&GroundTick, &mut Transform, &mut Sprite, &mut Visibility),
        (
            Without<BodyLayer>,
            Without<WeaponLayer>,
            Without<CapeLayerMark>,
            Without<Backdrop>,
        ),
    >,
    mut backdrop: Query<
        (&mut Sprite, &mut Anchor, &mut Transform, &mut Visibility),
        (
            With<Backdrop>,
            Without<BodyLayer>,
            Without<WeaponLayer>,
            Without<CapeLayerMark>,
            Without<GroundTick>,
        ),
    >,
    mut hud: Query<&mut Text, With<ProofHud>>,
) {
    if proof.source != SYNC_SOURCE {
        if clear.0 != DEFAULT_CLEAR {
            clear.0 = DEFAULT_CLEAR;
        }
        proof.sync_overlay = None;
        for (_, _, _, mut visibility) in &mut ticks {
            *visibility = Visibility::Hidden;
        }
        for (_, _, _, mut visibility) in &mut backdrop {
            *visibility = Visibility::Hidden;
        }
        return;
    }
    let (Some(bank), Some(run)) = (sync_bank, run) else {
        proof.source = 0;
        return;
    };
    let (Some(run6), Some(start)) = (
        run.manifest.clips.get("run6"),
        bank.start.clips.get(&bank.transition.start_clip),
    ) else {
        proof.source = 0;
        return;
    };
    let scenario_count = bank.plan.scenarios.len().max(1);
    proof.sync_scenario %= scenario_count;
    let Some(scenario) = bank.plan.scenarios.get(proof.sync_scenario) else {
        return;
    };
    let rate = scenario.rate.max(0.01);
    let run_count = run6.count.max(1) as usize;
    let start_count = start.count.max(1) as usize;
    if !proof.sync_transition {
        proof.sync_stage = Stage::Run;
    }

    // --- timing ---------------------------------------------------------
    let dt = time.delta_secs();
    if !proof.paused {
        if proof.sync_stage == Stage::Idle {
            proof.sync_stage_time += dt;
            if proof.sync_stage_time >= bank.transition.idle_hold {
                sync_advance(&mut proof, start_count, run_count, &bank.transition);
                proof.elapsed = 0.0;
            }
        } else {
            proof.elapsed += dt;
            loop {
                let authored = match proof.sync_stage {
                    Stage::Start => start.durations[proof.frame % start_count],
                    Stage::Run => run6.durations[proof.frame % run_count],
                    Stage::Idle => break,
                };
                let step = authored / rate;
                if step <= 0.0 || proof.elapsed < step {
                    break;
                }
                proof.elapsed -= step;
                sync_advance(&mut proof, start_count, run_count, &bank.transition);
            }
        }
    }
    if proof.sync_step {
        proof.sync_step = false;
        sync_advance(&mut proof, start_count, run_count, &bank.transition);
    }
    match proof.sync_stage {
        Stage::Start => proof.frame %= start_count,
        Stage::Run => proof.frame %= run_count,
        Stage::Idle => proof.frame = 0,
    }

    // --- camera and travel (game units, drawn at 2 texels per unit) -------
    let scale = proof.view.scale();
    let height = proof.view.height();
    for mut projection in &mut cameras {
        if let Projection::Orthographic(ortho) = projection.as_mut() {
            ortho.scaling_mode = ScalingMode::FixedVertical {
                viewport_height: height,
            };
        }
    }
    let world_per_unit = RUN_DENSITY * scale;
    let half_view = WINDOW_HEIGHT.recip() * 1280.0 * height / 2.0;
    let wrap = (half_view - 60.0) / world_per_unit;
    if proof.moving && !proof.paused && proof.sync_stage != Stage::Idle {
        proof.travel += scenario.speed * dt;
        if proof.travel > wrap {
            proof.travel -= 2.0 * wrap;
        }
    }
    if !proof.moving {
        proof.travel = 0.0;
    }
    let sign = if proof.mirror { -1.0 } else { 1.0 };
    let origin_x = proof.travel * world_per_unit * sign;

    // --- body -------------------------------------------------------------
    let idle_entry = base.as_ref().and_then(|base| base.get(FIDELITY_SOURCE));
    let shown = match proof.sync_stage {
        Stage::Idle => idle_entry.and_then(|entry| {
            let clip = entry.manifest.clips.get("idle")?;
            let atlas = entry.manifest.atlases.get(&clip.atlas)?;
            Some((entry.body.clone(), None, atlas.cell, clip.pivot, None))
        }),
        Stage::Start => bank.start.atlases.get(&start.atlas).map(|atlas| {
            let frame_box = bank
                .start_boxes
                .get(&bank.transition.start_clip)
                .and_then(|boxes| boxes.get(proof.frame).copied());
            (
                bank.start_image.clone(),
                Some((
                    bank.start_layout.clone(),
                    start.first as usize + proof.frame,
                )),
                atlas.cell,
                start.pivot,
                frame_box,
            )
        }),
        Stage::Run => {
            let image = match proof.sync_variant {
                1 => Some(bank.rim.clone()),
                2 => Some(bank.boots.clone()),
                _ => run.images.get(&run6.atlas).cloned(),
            };
            let frame_box = run
                .boxes
                .get("run6")
                .and_then(|boxes| boxes.get(proof.frame).copied());
            match (
                image,
                run.layouts.get(&run6.atlas),
                run.manifest.atlases.get(&run6.atlas),
            ) {
                (Some(image), Some(layout), Some(atlas)) => Some((
                    image,
                    Some((layout.clone(), run6.first as usize + proof.frame)),
                    atlas.cell,
                    run6.pivot,
                    frame_box,
                )),
                _ => None,
            }
        }
    };
    let Some((image, atlas, cell, pivot, frame_box)) = shown else {
        return;
    };
    proof.sync_overlay = Some((pivot, frame_box));
    if let Ok((mut sprite, mut anchor, mut transform)) = body.single_mut() {
        sprite.image = image;
        sprite.texture_atlas = atlas.map(|(layout, index)| TextureAtlas { layout, index });
        sprite.flip_x = proof.mirror;
        sprite.custom_size = Some(Vec2::new(cell.0 as f32 * scale, cell.1 as f32 * scale));
        let (x, y) = anchor_for(pivot, cell, proof.mirror);
        anchor.0 = Vec2::new(x, y);
        transform.translation.x = origin_x;
        transform.translation.y = 0.0;
    }
    for mut sprite in &mut weapon {
        sprite.custom_size = Some(Vec2::ZERO);
    }
    for mut visibility in &mut cape {
        *visibility = Visibility::Hidden;
    }

    // --- backdrop, back-light and ground ticks ----------------------------
    let (backdrop_name, colour) = SYNC_BACKDROPS[proof.sync_backdrop % SYNC_BACKDROPS.len()];
    if clear.0 != colour {
        clear.0 = colour;
    }
    let lit = proof.sync_backdrop % SYNC_BACKDROPS.len() == BACKLIGHT_BACKDROP;
    for (mut sprite, mut anchor, mut transform, mut visibility) in &mut backdrop {
        let plate = (112.0, 128.0);
        sprite.custom_size = Some(Vec2::new(plate.0 * scale, plate.1 * scale));
        let (x, y) = anchor_for((56.0, 118.0), (112, 128), false);
        anchor.0 = Vec2::new(x, y);
        transform.translation.x = origin_x;
        *visibility = if lit {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
    for (tick, mut transform, mut sprite, mut visibility) in &mut ticks {
        let x = tick.0 as f32 * TICK_SPACING_UNITS * world_per_unit;
        transform.translation = Vec3::new(x, -3.0 * scale, 4.0);
        sprite.custom_size = Some(Vec2::new(scale, 4.0 * scale));
        *visibility = Visibility::Inherited;
    }

    // --- HUD --------------------------------------------------------------
    if let Ok(mut text) = hud.single_mut() {
        let (stage, clip_name, count) = match proof.sync_stage {
            Stage::Idle => ("idle (N3.5)", "idle", 1),
            Stage::Start => (
                "start (video 3-13)",
                bank.transition.start_clip.as_str(),
                start_count,
            ),
            Stage::Run => ("run", "run6", run_count),
        };
        let phase = match proof.sync_stage {
            Stage::Run => bank
                .plan
                .run6_phase
                .get(proof.frame)
                .map_or("?", String::as_str),
            Stage::Start => "start",
            Stage::Idle => "idle",
        };
        let slide = (scenario.speed - rate * bank.plan.reference_speed).abs();
        let slide_text = if proof.moving {
            format!(
                "slide {slide:.0} u/s ({:.0}%)",
                slide / scenario.speed.max(1.0) * 100.0
            )
        } else {
            format!(
                "in place: planted foot moves back {:.0} u/s",
                rate * bank.plan.reference_speed
            )
        };
        text.0 = format!(
            "n3.7-sync-proof  scenario {} {}  mode {}  stage {stage}  clip {clip_name}  frame {}/{}  view {}  {}",
            scenario.id,
            scenario.name,
            if proof.sync_transition {
                "transition"
            } else {
                "speed"
            },
            proof.frame,
            count,
            proof.view.label(),
            if proof.paused { "paused" } else { "playing" },
        );
        text.0.push_str(&format!(
            "\nfps {:.1}  rate {:.2}  speed {:.0} u/s (game {:.0})  x {:.1} u  phase {phase}  {slide_text}",
            bank.plan.base_fps * rate,
            rate,
            scenario.speed,
            bank.plan.game_speed,
            proof.travel,
        ));
        text.0.push_str(&format!(
            "\nframes {}  backdrop {backdrop_name}  camera {:.0}  {:.3} screen px/texel  overlay {}  {}  NOT APPROVED",
            SYNC_VARIANTS[proof.sync_variant % SYNC_VARIANTS.len()],
            height,
            proof.view.screen_px_per_texel(),
            if proof.overlay { "on" } else { "off" },
            if proof.moving { "moving" } else { "in place" },
        ));
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
    if let Some(folder) = path.parent()
        && let Err(err) = std::fs::create_dir_all(folder)
    {
        eprintln!(
            "[n3_visual_proof] cannot create {}: {err}",
            folder.display()
        );
    }
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
