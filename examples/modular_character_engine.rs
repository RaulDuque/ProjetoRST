//! Dark Castle Modular Character Engine — prototypes 01 and 02 (isolated
//! lab, not the game).
//!
//! Three panels, all at 3 screen pixels per texel and on one sole line:
//!
//! - **left, original**: the approved frame (N3.5 idle / N3.6 run6), one
//!   sprite, untouched;
//! - **centre, hybrid** (the approved approach): the same approved frame cut
//!   into five depth layers (N4-C label maps or the 02 rules,
//!   `rig::layers`), with independent equipment bound to the skeleton
//!   (`rig::equipment`, `rig::grip`, `rig::chest`). It can walk around;
//! - **right, articulated** (kept as an experiment): pieces cut from the
//!   idle frame and moved by FK + two-bone IK (`rig::articulated`), wearing
//!   the same items.
//!
//! ```text
//! cargo run --example modular_character_engine          # scripted tour + checks, then exit
//! MCE_INTERACTIVE=1 cargo run --example modular_character_engine
//! ```
//!
//! | Key | Effect |
//! |---|---|
//! | Left / Right (or A / D) | walk: run while held, idle when released |
//! | C | idle / run in place |
//! | M | face left / right |
//! | Space | pause / play |
//! | . / , | next / previous frame (paused) |
//! | 1 | sword: A -> B -> none |
//! | 2 | pauldron: A -> B -> none |
//! | 3 | breastplate on / off |
//! | 4 | breastplate approach: projected -> rigid -> mesh warp |
//! | 9 / 0 | wear everything / remove everything |
//! | S | skeleton |
//! | P | sockets, pivots and item axes |
//! | T | tint the depth layers |
//! | L | layers one at a time (far arm, legs, torso, hair, near arm, equipment only) |
//! | X | exploded pieces |
//! | O | hide / show the original and articulated panels |
//!
//! The tour writes raw captures to `target/modular_character_engine/` (and
//! the isolated breastplate rasters in `raw/`), prints metrics and
//! `RESULT: PASS/FAIL`; `art-src/modular-character-engine/scripts/
//! make_evidence_02.py` builds `evidence/modular_character_engine/
//! 02_equipamentos/`.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::OnceLock;

use bevy::asset::io::AssetSourceBuilder;
use bevy::asset::{AssetApp, RenderAssetUsages};
use bevy::image::ImageSampler;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::sprite::Anchor;
use bevy::window::WindowResolution;
use dark_castle_rpg::core::resources::project_root;
use dark_castle_rpg::core::time::clamp_delta;
use dark_castle_rpg::player::visual::manifest::anchor_for;
use dark_castle_rpg::player::visual::playback::loop_frame;
use dark_castle_rpg::player::visual::rig::articulated::articulate;
use dark_castle_rpg::player::visual::rig::chest::{ChestMode, TorsoCage, warp};
use dark_castle_rpg::player::visual::rig::data::{RigClip, RigLab};
use dark_castle_rpg::player::visual::rig::equipment::{
    ItemArt, Mount, Placement, RigLoadout, RigSlot, SpriteArt, mirror, place_in_hand, place_on_bone,
};
use dark_castle_rpg::player::visual::rig::grip::hand_frame;
use dark_castle_rpg::player::visual::rig::layers::{
    BodyRegion, CUTOUT_BONES, DepthSlot, extract_cell, masked_rgba, material_of, partition_by_bone,
};
use dark_castle_rpg::player::visual::rig::skeleton::{
    Bone, Joint, JointPositions, LocalPose, WorldPose, fit, forward, retarget,
};

const TAG: &str = "[modular_character_engine]";
const STAGE: &str = "art-src/modular-character-engine/stage";
const APPROVED: &str = "art-src/n3-8-production-run/stage";
const FACTORY: &str = "art-src/n4-sprite-machine/stage/factory/isolated";
const CAPTURES: &str = "target/modular_character_engine";
/// Screen pixels per texel.
const SCALE: f32 = 3.0;
/// Screen y of the sole line.
const SOLE_Y: f32 = -250.0;
/// Panel centres (screen x).
const PANEL_X: [f32; 3] = [-420.0, 0.0, 420.0];
const DARK: Color = Color::srgb(0.19, 0.17, 0.22);
/// Z distance between two depth slots (inside a panel).
const SLOT_Z: f32 = 0.01;
/// Far pieces of the articulated panel are darkened (one shade back).
const FAR_SHADE: f32 = 0.6;
/// Lab walking speed, texels per second (the feet slide: the approved run
/// was measured at a faster ground speed, N3.7).
const WALK_SPEED: f32 = 120.0;
/// The walker wraps around past this distance from the centre (texels).
const WALK_WRAP: f32 = 200.0;
const WARM_UP_FRAMES: u32 = 20;
/// Frames between setting a shot and capturing it, and per shot.
const CAPTURE_AT: u32 = 6;
const SHOT_FRAMES: u32 = 10;

static VERDICT: OnceLock<bool> = OnceLock::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Panel {
    Original,
    Hybrid,
    Articulated,
}

impl Panel {
    const fn x(self) -> f32 {
        match self {
            Self::Original => PANEL_X[0],
            Self::Hybrid => PANEL_X[1],
            Self::Articulated => PANEL_X[2],
        }
    }
}

/// What the layer view shows (key L): every region, one fine body region
/// (Engine 04 split the near arm and the waist), or the equipment only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LayerView {
    All,
    Solo(BodyRegion),
    GearOnly,
}

impl LayerView {
    fn next(self) -> Self {
        let all = BodyRegion::ALL;
        match self {
            Self::All => Self::Solo(all[0]),
            Self::Solo(r) => match all.iter().position(|x| *x == r) {
                Some(i) if i + 1 < all.len() => Self::Solo(all[i + 1]),
                _ => Self::GearOnly,
            },
            Self::GearOnly => Self::All,
        }
    }

    fn shows_layer(self, layer: BodyRegion) -> bool {
        match self {
            Self::All => true,
            Self::Solo(l) => l == layer,
            Self::GearOnly => false,
        }
    }

    fn shows_gear(self) -> bool {
        matches!(self, Self::All | Self::GearOnly)
    }

    fn label(self) -> String {
        match self {
            Self::All => "all layers".into(),
            Self::Solo(l) => format!("layer {} only", l.key()),
            Self::GearOnly => "equipment only".into(),
        }
    }
}

#[derive(Component)]
struct OriginalSprite;

#[derive(Component)]
struct PanelRoot(Panel);

#[derive(Component)]
struct HybridLayer(BodyRegion);

#[derive(Component)]
struct PieceSprite(Bone);

#[derive(Component)]
struct ItemSprite {
    panel: Panel,
    slot: RigSlot,
}

#[derive(Component)]
struct Hud;

/// Which piece image a bone of the articulated panel uses (the far limbs
/// reuse the near ones: the side-view idle hides them).
const fn cutout_of(bone: Bone) -> Bone {
    match bone {
        Bone::UpperArmFar => Bone::UpperArmNear,
        Bone::ForearmFar => Bone::ForearmNear,
        Bone::ThighB => Bone::ThighA,
        Bone::ShinB => Bone::ShinA,
        Bone::FootB => Bone::FootA,
        other => other,
    }
}

const fn is_far(bone: Bone) -> bool {
    matches!(
        bone,
        Bone::UpperArmFar | Bone::ForearmFar | Bone::ThighB | Bone::ShinB | Bone::FootB
    )
}

/// Depth of an articulated piece (sub-order inside the slot).
fn piece_depth(bone: Bone) -> (DepthSlot, f32) {
    match bone {
        Bone::UpperArmFar | Bone::ForearmFar => (DepthSlot::FarArm, 0.0),
        Bone::ThighB | Bone::ShinB | Bone::FootB => (DepthSlot::Legs, 0.0),
        Bone::ThighA | Bone::ShinA | Bone::FootA => (DepthSlot::Legs, 0.5),
        Bone::Spine => (DepthSlot::Torso, 0.0),
        Bone::Head => (DepthSlot::Hair, 0.0),
        Bone::UpperArmNear => (DepthSlot::NearUpperArm, 0.0),
        Bone::ForearmNear => (DepthSlot::NearForearm, 0.0),
    }
}

/// Exploded view: each articulated piece pushed away from the body.
fn explode_offset(bone: Bone) -> (f32, f32) {
    match bone {
        Bone::Spine => (0.0, 0.0),
        Bone::Head => (0.0, 14.0),
        Bone::UpperArmNear | Bone::UpperArmFar => (18.0, 2.0),
        Bone::ForearmNear | Bone::ForearmFar => (26.0, -6.0),
        Bone::ThighA | Bone::ThighB => (0.0, -8.0),
        Bone::ShinA | Bone::ShinB => (0.0, -16.0),
        Bone::FootA | Bone::FootB => (0.0, -22.0),
    }
}

/// Exploded view: where each slot's item is laid out beside the body.
fn exploded_item_spot(slot: RigSlot) -> (f32, f32) {
    match slot {
        RigSlot::MainHand => (48.0, 30.0),
        RigSlot::Shoulder => (-44.0, 80.0),
        RigSlot::Chest => (-46.0, 30.0),
        _ => (0.0, 110.0),
    }
}

/// Debug colours of the depth layers (tint mode).
fn layer_tint(layer: BodyRegion) -> Color {
    match layer {
        BodyRegion::FarArm => Color::srgb(0.35, 0.55, 1.0),
        BodyRegion::Legs => Color::srgb(0.55, 0.45, 0.35),
        BodyRegion::Waist => Color::srgb(0.45, 0.45, 0.5),
        BodyRegion::Torso => Color::srgb(0.6, 0.6, 0.6),
        BodyRegion::Hair => Color::srgb(1.0, 0.95, 0.4),
        BodyRegion::NearUpperArm => Color::srgb(1.0, 0.35, 0.35),
        BodyRegion::NearForearm => Color::srgb(1.0, 0.55, 0.3),
        BodyRegion::NearHand => Color::srgb(1.0, 0.75, 0.6),
    }
}

/// The equipment worn by a shot: one variant index per slot.
#[derive(Debug, Clone, Copy, Default)]
struct Wear {
    sword: Option<usize>,
    shoulder: Option<usize>,
    chest: bool,
}

const ALL: Wear = Wear {
    sword: Some(0),
    shoulder: Some(0),
    chest: true,
};
const SWORD: Wear = Wear {
    sword: Some(0),
    shoulder: None,
    chest: false,
};
const CHEST: Wear = Wear {
    sword: None,
    shoulder: None,
    chest: true,
};

/// One scripted capture. `State` shots set the lab directly; `Input` shots
/// press real keys through `ButtonInput` (the interactive controls).
#[derive(Debug, Clone)]
enum Shot {
    State(StateShot),
    Input(InputShot),
}

#[derive(Debug, Clone)]
struct StateShot {
    name: String,
    clip: &'static str,
    /// `Some(f)`: jump to the start of frame f; `None`: advance the clock by
    /// one run frame (1/15 s) without resetting it.
    frame: Option<usize>,
    flip: bool,
    wear: Wear,
    chest_mode: ChestMode,
    skeleton: bool,
    sockets: bool,
    tint: bool,
    exploded: bool,
    layer_view: LayerView,
    /// The frame the clock must show (continuity checks).
    expect_frame: Option<usize>,
}

#[derive(Debug, Clone)]
struct InputShot {
    name: String,
    /// Keys tapped when the shot starts.
    taps: Vec<KeyCode>,
    /// Key held during the shot (walking).
    hold: Option<KeyCode>,
    /// Real-time frames before the capture.
    wait: u32,
    /// What the controls must have done (checked at the capture).
    expect: fn(&Lab) -> bool,
}

fn state(name: impl Into<String>, clip: &'static str, frame: Option<usize>) -> StateShot {
    StateShot {
        name: name.into(),
        clip,
        frame,
        flip: false,
        wear: Wear::default(),
        chest_mode: ChestMode::Projected,
        skeleton: false,
        sockets: false,
        tint: false,
        exploded: false,
        layer_view: LayerView::All,
        expect_frame: None,
    }
}

fn input(
    name: &str,
    taps: &[KeyCode],
    hold: Option<KeyCode>,
    wait: u32,
    expect: fn(&Lab) -> bool,
) -> Shot {
    Shot::Input(InputShot {
        name: name.to_owned(),
        taps: taps.to_vec(),
        hold,
        wait,
        expect,
    })
}

fn worn_is(lab: &Lab, slot: RigSlot, id: &str) -> bool {
    lab.loadout.worn(slot) == Some(id)
}

fn tour() -> Vec<Shot> {
    let mut out: Vec<Shot> = Vec::new();
    let mut push = |s: StateShot| out.push(Shot::State(s));
    // Bare: the hybrid must match the original pixel for pixel.
    for f in 0..2 {
        push(state(format!("bare_idle_f{f}"), "idle", Some(f)));
    }
    for f in 0..6 {
        push(state(format!("bare_run_f{f}"), "run", Some(f)));
    }
    // Skeleton over the character (refined marks).
    push(StateShot {
        skeleton: true,
        ..state("skeleton_idle_f0", "idle", Some(0))
    });
    for f in 0..6 {
        push(StateShot {
            skeleton: true,
            sockets: true,
            wear: SWORD,
            ..state(format!("skeleton_run_f{f}"), "run", Some(f))
        });
    }
    // Sword grip: idle (breath) and run.
    for f in 0..2 {
        push(StateShot {
            wear: SWORD,
            ..state(format!("sword_idle_f{f}"), "idle", Some(f))
        });
    }
    for f in 0..6 {
        push(StateShot {
            wear: SWORD,
            ..state(format!("sword_run_f{f}"), "run", Some(f))
        });
    }
    // Breastplate alone, in the three approaches, every pose.
    for mode in ChestMode::ALL {
        let tag = mode_tag(mode);
        for f in 0..2 {
            push(StateShot {
                wear: CHEST,
                chest_mode: mode,
                ..state(format!("chest_{tag}_idle_f{f}"), "idle", Some(f))
            });
        }
        for f in 0..6 {
            push(StateShot {
                wear: CHEST,
                chest_mode: mode,
                ..state(format!("chest_{tag}_run_f{f}"), "run", Some(f))
            });
        }
    }
    // Sword + pauldron only (the prototype 01 loadout, for the comparison).
    for f in 0..6 {
        push(StateShot {
            wear: Wear {
                sword: Some(0),
                shoulder: Some(0),
                chest: false,
            },
            ..state(format!("sp_run_f{f}"), "run", Some(f))
        });
    }
    // Everything together, and its depth layers.
    for f in 0..6 {
        push(StateShot {
            wear: ALL,
            ..state(format!("all_run_f{f}"), "run", Some(f))
        });
        push(StateShot {
            wear: ALL,
            tint: true,
            ..state(format!("depth_run_f{f}"), "run", Some(f))
        });
    }
    for f in 0..2 {
        push(StateShot {
            wear: ALL,
            ..state(format!("all_idle_f{f}"), "idle", Some(f))
        });
    }
    // Layers shown one at a time.
    let mut view = LayerView::All;
    for i in 0..10 {
        push(StateShot {
            wear: ALL,
            layer_view: view,
            ..state(format!("layers_run_f2_{i}"), "run", Some(2))
        });
        view = view.next();
    }
    push(StateShot {
        wear: ALL,
        exploded: true,
        sockets: true,
        ..state("exploded_idle_f0", "idle", Some(0))
    });
    push(StateShot {
        wear: ALL,
        flip: true,
        skeleton: true,
        sockets: true,
        ..state("mirror_run_f4", "run", Some(4))
    });
    // Swapping during the run: the clock only advances, never resets.
    let swap: [Wear; 6] = [
        Wear {
            sword: Some(0),
            shoulder: None,
            chest: false,
        },
        Wear {
            sword: Some(0),
            shoulder: Some(0),
            chest: false,
        },
        Wear {
            sword: Some(1),
            shoulder: Some(0),
            chest: true,
        },
        Wear {
            sword: Some(1),
            shoulder: Some(1),
            chest: true,
        },
        Wear {
            sword: None,
            shoulder: Some(1),
            chest: true,
        },
        Wear::default(),
    ];
    push(StateShot {
        wear: swap[0],
        expect_frame: Some(0),
        ..state("swap_s00", "run", Some(0))
    });
    for step in 1..24 {
        push(StateShot {
            wear: swap[step / 4],
            expect_frame: Some(step % 6),
            ..state(format!("swap_s{step:02}"), "run", None)
        });
    }
    // Turning around mid-run with everything worn.
    push(StateShot {
        wear: ALL,
        expect_frame: Some(0),
        ..state("turn_t00", "run", Some(0))
    });
    for step in 1..18 {
        push(StateShot {
            wear: ALL,
            flip: (5..12).contains(&step),
            expect_frame: Some(step % 6),
            ..state(format!("turn_t{step:02}"), "run", None)
        });
    }
    // The interactive controls, driven by real key presses.
    out.extend([
        input(
            "ui_00_start",
            &[KeyCode::Digit0, KeyCode::KeyC],
            None,
            8,
            |l| l.loadout.items().count() == 0 && l.clip == "idle",
        ),
        input("ui_01_sword", &[KeyCode::Digit1], None, 8, |l| {
            worn_is(l, RigSlot::MainHand, "sword_arming_cross")
        }),
        input("ui_02_pauldron", &[KeyCode::Digit2], None, 8, |l| {
            worn_is(l, RigSlot::Shoulder, "pauldron_steel")
        }),
        input("ui_03_chest", &[KeyCode::Digit3], None, 8, |l| {
            worn_is(l, RigSlot::Chest, "plate_cuirass") && l.loadout.items().count() == 3
        }),
        input(
            "ui_04_walk_right",
            &[],
            Some(KeyCode::ArrowRight),
            20,
            |l| l.clip == "run" && !l.flip && l.x > 0.0,
        ),
        input("ui_05_walk_left", &[], Some(KeyCode::ArrowLeft), 20, |l| {
            l.clip == "run" && l.flip
        }),
        input(
            "ui_06_swap_while_walking",
            &[KeyCode::Digit1, KeyCode::Digit2],
            Some(KeyCode::ArrowLeft),
            12,
            |l| {
                l.clip == "run"
                    && worn_is(l, RigSlot::MainHand, "sword_estoc_ring")
                    && worn_is(l, RigSlot::Shoulder, "pauldron_bronze")
            },
        ),
        input("ui_07_released_idle", &[], None, 10, |l| {
            l.clip == "idle" && !l.walking
        }),
        input(
            "ui_08_paused_skeleton",
            &[KeyCode::KeyC, KeyCode::Space, KeyCode::KeyS, KeyCode::KeyP],
            None,
            6,
            |l| !l.playing && l.skeleton && l.sockets && l.clip == "run",
        ),
        input("ui_09_step_frame", &[KeyCode::Period], None, 6, |l| {
            !l.playing
        }),
        input(
            "ui_10_layer_view",
            &[
                KeyCode::KeyS,
                KeyCode::KeyP,
                KeyCode::KeyL,
                KeyCode::KeyL,
                KeyCode::KeyL,
                KeyCode::KeyL,
                KeyCode::KeyL,
                KeyCode::KeyL,
            ],
            None,
            6,
            |l| l.layer_view == LayerView::Solo(BodyRegion::NearUpperArm),
        ),
        input(
            "ui_11_tint_mesh_warp",
            &[
                KeyCode::KeyL,
                KeyCode::KeyL,
                KeyCode::KeyL,
                KeyCode::KeyL,
                KeyCode::KeyT,
                KeyCode::Digit4,
                KeyCode::Digit4,
            ],
            None,
            6,
            |l| l.layer_view == LayerView::All && l.tint && l.chest_mode == ChestMode::MeshWarp,
        ),
        input(
            "ui_12_comparison_hidden",
            &[
                KeyCode::KeyT,
                KeyCode::Digit4,
                KeyCode::KeyO,
                KeyCode::Space,
            ],
            None,
            10,
            |l| !l.comparison && l.playing && l.chest_mode == ChestMode::Projected,
        ),
    ]);
    out
}

fn mode_tag(mode: ChestMode) -> &'static str {
    match mode {
        ChestMode::Rigid => "rigid",
        ChestMode::MeshWarp => "warp",
        ChestMode::Projected => "projected",
    }
}

/// Images built from the approved frames once they are loaded.
struct Built {
    /// (clip, source, layer) -> layer image.
    layers: HashMap<(String, usize, BodyRegion), Handle<Image>>,
    /// Cutout pieces of the idle (cell-sized images).
    pieces: HashMap<Bone, Handle<Image>>,
    /// (clip, frame, mode) -> breastplate image (cell-sized; the rigid mode
    /// is the rest raster, placed as a sprite).
    chest: HashMap<(String, usize, ChestMode), Handle<Image>>,
}

#[derive(Resource)]
struct Lab {
    rig: RigLab,
    rest: LocalPose,
    rest_world: WorldPose,
    rest_cage: TorsoCage,
    strips: HashMap<String, (Handle<Image>, Handle<TextureAtlasLayout>)>,
    items: HashMap<String, Handle<Image>>,
    built: Option<Built>,
    clip: String,
    elapsed: f32,
    frame: usize,
    flip: bool,
    playing: bool,
    /// Hybrid walker offset from its panel centre (texels).
    x: f32,
    walking: bool,
    loadout: RigLoadout,
    chest_mode: ChestMode,
    skeleton: bool,
    sockets: bool,
    tint: bool,
    exploded: bool,
    layer_view: LayerView,
    comparison: bool,
    interactive: bool,
    /// The tour is in its key-driven part (real-time clock).
    realtime: bool,
    shots: Vec<Shot>,
    tour: usize,
    wait: u32,
    warm: u32,
    held: Option<KeyCode>,
    out: PathBuf,
    failures: Vec<String>,
}

impl Lab {
    fn clip(&self) -> Option<&RigClip> {
        self.rig.clips.get(&self.clip)
    }

    fn joints(&self) -> Option<JointPositions> {
        self.clip()?.frame_joints(self.frame)
    }

    /// Cycles a slot through its variants, then none.
    fn cycle(&mut self, slot: RigSlot) {
        let variants = self.rig.variants.get(&slot).cloned().unwrap_or_default();
        let next = match self.loadout.worn(slot) {
            None => variants.first().cloned(),
            Some(id) => {
                let i = variants.iter().position(|v| v == id);
                i.and_then(|i| variants.get(i + 1)).cloned()
            }
        };
        match next {
            Some(id) => {
                self.loadout.equip(slot, &id);
            }
            None => {
                self.loadout.unequip(slot);
            }
        }
    }

    fn wear_variant(&mut self, slot: RigSlot, variant: Option<usize>) {
        let id = variant.and_then(|v| self.rig.variants.get(&slot)?.get(v).cloned());
        match id {
            Some(id) => {
                self.loadout.equip(slot, &id);
            }
            None => {
                self.loadout.unequip(slot);
            }
        }
    }

    fn wear(&mut self, w: Wear) {
        self.wear_variant(RigSlot::MainHand, w.sword);
        self.wear_variant(RigSlot::Shoulder, w.shoulder);
        self.wear_variant(RigSlot::Chest, w.chest.then_some(0));
    }

    fn frame_start(&self, frame: usize) -> f32 {
        self.clip()
            .map(|c| c.frames.iter().take(frame).map(|f| f.duration).sum())
            .unwrap_or(0.0)
    }

    fn set_clip(&mut self, clip: &str) {
        if self.clip != clip {
            self.clip = clip.to_owned();
            self.elapsed = 0.0;
        }
    }
}

fn setup(
    mut commands: Commands,
    mut lab: ResMut<Lab>,
    server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);
    let clips: Vec<(String, RigClip)> = lab
        .rig
        .clips
        .iter()
        .map(|(k, c)| (k.clone(), c.clone()))
        .collect();
    for (key, clip) in clips {
        let layout = layouts.add(TextureAtlasLayout::from_grid(
            UVec2::new(clip.cell.0, clip.cell.1),
            clip.columns,
            1,
            None,
            None,
        ));
        lab.strips
            .insert(key, (server.load(clip.texture.clone()), layout));
    }
    let items: Vec<(String, String)> = lab
        .rig
        .items
        .iter()
        .filter_map(|(k, i)| Some((k.clone(), i.sprite()?.texture.clone())))
        .collect();
    for (id, texture) in items {
        lab.items.insert(id, server.load(texture));
    }
    commands.spawn((
        OriginalSprite,
        Sprite::default(),
        Anchor::default(),
        Transform::from_xyz(Panel::Original.x(), SOLE_Y, 0.0).with_scale(Vec3::splat(SCALE)),
        Visibility::Hidden,
    ));
    for panel in [Panel::Hybrid, Panel::Articulated] {
        let root = commands
            .spawn((
                PanelRoot(panel),
                Transform::from_xyz(panel.x(), SOLE_Y, 0.0).with_scale(Vec3::splat(SCALE)),
                Visibility::default(),
            ))
            .id();
        if panel == Panel::Hybrid {
            for layer in BodyRegion::ALL {
                commands.entity(root).with_child((
                    HybridLayer(layer),
                    Sprite::default(),
                    Anchor::default(),
                    Transform::default(),
                    Visibility::Hidden,
                ));
            }
        } else {
            for bone in Bone::ALL {
                commands.entity(root).with_child((
                    PieceSprite(bone),
                    Sprite::default(),
                    Anchor::default(),
                    Transform::default(),
                    Visibility::Hidden,
                ));
            }
        }
        for slot in [RigSlot::MainHand, RigSlot::Shoulder, RigSlot::Chest] {
            commands.entity(root).with_child((
                ItemSprite { panel, slot },
                Sprite::default(),
                Anchor::default(),
                Transform::default(),
                Visibility::Hidden,
            ));
        }
    }
    commands.spawn((
        Hud,
        Text::new(""),
        TextFont {
            font_size: FontSize::Px(13.0),
            ..default()
        },
        TextColor(Color::srgb(0.95, 0.8, 0.45)),
        Node {
            position_type: PositionType::Absolute,
            left: px(12),
            top: px(8),
            ..default()
        },
    ));
}

fn rgba_image(width: u32, height: u32, data: Vec<u8>) -> Image {
    let mut image = Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.sampler = ImageSampler::nearest();
    image
}

fn flat(px: &[[u8; 4]]) -> Vec<u8> {
    px.iter().flatten().copied().collect()
}

/// Once the approved strips are in memory: cut every source into its depth
/// layers (authored N4-C maps or the rules), the idle into pieces, and draw
/// the breastplate on every frame in the three approaches.
fn build_layers(mut lab: ResMut<Lab>, mut images: ResMut<Assets<Image>>) {
    if lab.built.is_some() {
        return;
    }
    let ready = lab.strips.values().all(|(h, _)| images.contains(h))
        && lab.items.values().all(|h| images.contains(h));
    if !ready {
        return;
    }
    let root = project_root().join(STAGE);
    let mut layers = HashMap::new();
    let mut pieces = HashMap::new();
    let mut chest = HashMap::new();
    let mut failures = Vec::new();
    let clips: Vec<(String, RigClip)> = lab
        .rig
        .clips
        .iter()
        .map(|(k, c)| (k.clone(), c.clone()))
        .collect();
    for (key, clip) in &clips {
        let Some((handle, _)) = lab.strips.get(key) else {
            continue;
        };
        let Some(image) = images.get(handle) else {
            continue;
        };
        let strip_width = image.width() as usize;
        let Some(data) = image.data.clone() else {
            failures.push(format!("{key}: strip has no CPU data"));
            continue;
        };
        let cell = clip.geometry();
        // The shared cut (rig::data::RigClip::cut_layers): N4-C maps or rules.
        match clip.cut_layers(&data, strip_width, |path| {
            std::fs::read_to_string(root.join(path)).ok()
        }) {
            Ok(cut) => {
                for set in cut {
                    for (layer, bytes) in set.layers {
                        let h =
                            images.add(rgba_image(cell.width as u32, cell.height as u32, bytes));
                        layers.insert((key.clone(), set.source, layer), h);
                    }
                    println!(
                        "{TAG} partition {key} #{} ({}): {} texels differ",
                        set.source,
                        if set.authored { "N4-C map" } else { "02 rules" },
                        set.differing
                    );
                    if set.differing != 0 {
                        failures.push(format!(
                            "{key} #{}: {} texels differ",
                            set.source, set.differing
                        ));
                    }
                }
            }
            Err(e) => failures.push(format!("{key}: {e}")),
        }
        // The articulated comparison's pieces, cut from the rest frame.
        if *key == lab.rig.rest_clip
            && let (Some(texels), Some(joints)) = (
                extract_cell(&data, strip_width, 0, cell),
                clip.source_joints(0),
            )
        {
            let materials: Vec<_> = texels.iter().map(|t| material_of(*t)).collect();
            let bones = partition_by_bone(&materials, cell, &joints);
            for bone in CUTOUT_BONES {
                let bytes = masked_rgba(&texels, &bones, bone);
                let h = images.add(rgba_image(cell.width as u32, cell.height as u32, bytes));
                pieces.insert(bone, h);
            }
        }
    }
    // The breastplate: one model, every frame, three approaches.
    let raw = lab.out.join("raw");
    let _ = std::fs::create_dir_all(&raw);
    let chest_model = lab.rig.items.values().find_map(|i| match &i.art {
        ItemArt::Model(m) => lab.rig.models.get(m).cloned(),
        ItemArt::Sprite(_) => None,
    });
    if let (Some(model), Some(rest_clip)) = (chest_model, lab.rig.rest().cloned()) {
        let rest_cell = rest_clip.geometry();
        let rest_raster = model.rasterize(&lab.rest_cage, rest_cell);
        let rest_handle = images.add(rgba_image(
            rest_cell.width as u32,
            rest_cell.height as u32,
            flat(&rest_raster),
        ));
        let _ = std::fs::write(
            raw.join(format!(
                "chest_rigid_{}x{}.rgba",
                rest_cell.width, rest_cell.height
            )),
            flat(&rest_raster),
        );
        for (key, clip) in &clips {
            let cell = clip.geometry();
            for frame in 0..clip.frames.len() {
                let Some(j) = clip.frame_joints(frame) else {
                    continue;
                };
                let cage = TorsoCage::from_joints(&j);
                let projected = model.rasterize(&cage, cell);
                let warped = warp(&rest_raster, &lab.rest_cage, rest_cell, &cage, cell);
                for (mode, px) in [
                    (ChestMode::Projected, &projected),
                    (ChestMode::MeshWarp, &warped),
                ] {
                    let _ = std::fs::write(
                        raw.join(format!(
                            "chest_{}_{key}_f{frame}_{}x{}.rgba",
                            mode_tag(mode),
                            cell.width,
                            cell.height
                        )),
                        flat(px),
                    );
                    let h = images.add(rgba_image(cell.width as u32, cell.height as u32, flat(px)));
                    chest.insert((key.clone(), frame, mode), h);
                }
                chest.insert((key.clone(), frame, ChestMode::Rigid), rest_handle.clone());
            }
        }
    } else {
        failures.push("no breastplate model".into());
    }
    lab.failures.extend(failures);
    lab.built = Some(Built {
        layers,
        pieces,
        chest,
    });
}

fn keys(input: Res<ButtonInput<KeyCode>>, mut lab: ResMut<Lab>) {
    if !lab.interactive && !lab.realtime {
        return;
    }
    let frames = lab.clip().map_or(1, |c| c.frames.len());
    if input.just_pressed(KeyCode::Space) {
        lab.playing = !lab.playing;
    }
    if input.just_pressed(KeyCode::Period) && !lab.playing {
        lab.frame = (lab.frame + 1) % frames;
        lab.elapsed = lab.frame_start(lab.frame) + 1e-4;
    }
    if input.just_pressed(KeyCode::Comma) && !lab.playing {
        lab.frame = (lab.frame + frames - 1) % frames;
        lab.elapsed = lab.frame_start(lab.frame) + 1e-4;
    }
    let left = input.pressed(KeyCode::ArrowLeft) || input.pressed(KeyCode::KeyA);
    let right = input.pressed(KeyCode::ArrowRight) || input.pressed(KeyCode::KeyD);
    let walking = left != right;
    if walking {
        lab.flip = left;
        lab.set_clip("run");
    } else if lab.walking {
        lab.set_clip("idle");
    }
    lab.walking = walking;
    if input.just_pressed(KeyCode::KeyC) {
        let next = if lab.clip == "run" { "idle" } else { "run" };
        lab.set_clip(next);
    }
    if input.just_pressed(KeyCode::KeyM) {
        lab.flip = !lab.flip;
    }
    if input.just_pressed(KeyCode::Digit1) {
        lab.cycle(RigSlot::MainHand);
    }
    if input.just_pressed(KeyCode::Digit2) {
        lab.cycle(RigSlot::Shoulder);
    }
    if input.just_pressed(KeyCode::Digit3) {
        lab.cycle(RigSlot::Chest);
    }
    if input.just_pressed(KeyCode::Digit4) {
        lab.chest_mode = match lab.chest_mode {
            ChestMode::Projected => ChestMode::Rigid,
            ChestMode::Rigid => ChestMode::MeshWarp,
            ChestMode::MeshWarp => ChestMode::Projected,
        };
    }
    if input.just_pressed(KeyCode::Digit9) {
        lab.wear(ALL);
    }
    if input.just_pressed(KeyCode::Digit0) {
        lab.wear(Wear::default());
    }
    if input.just_pressed(KeyCode::KeyS) {
        lab.skeleton = !lab.skeleton;
    }
    if input.just_pressed(KeyCode::KeyP) {
        lab.sockets = !lab.sockets;
    }
    if input.just_pressed(KeyCode::KeyT) {
        lab.tint = !lab.tint;
    }
    if input.just_pressed(KeyCode::KeyL) {
        lab.layer_view = lab.layer_view.next();
    }
    if input.just_pressed(KeyCode::KeyX) {
        lab.exploded = !lab.exploded;
    }
    if input.just_pressed(KeyCode::KeyO) {
        lab.comparison = !lab.comparison;
    }
}

/// The one clock: the frame always comes from `elapsed` through the same
/// loop function as the player animation. Equipment never touches it.
fn advance(time: Res<Time>, mut lab: ResMut<Lab>) {
    let dt = clamp_delta(time.delta_secs());
    if (lab.interactive || lab.realtime) && lab.playing {
        lab.elapsed += dt;
        if lab.walking {
            let dir = if lab.flip { -1.0 } else { 1.0 };
            lab.x += dir * WALK_SPEED * dt;
            if lab.x.abs() > WALK_WRAP {
                lab.x = -lab.x.signum() * WALK_WRAP;
            }
        }
    }
    let durations = lab.clip().map(RigClip::durations).unwrap_or_default();
    lab.frame = loop_frame(&durations, lab.elapsed);
}

fn run_tour(
    mut commands: Commands,
    mut lab: ResMut<Lab>,
    mut keyboard: ResMut<ButtonInput<KeyCode>>,
    mut exit: MessageWriter<AppExit>,
) {
    if lab.interactive || lab.built.is_none() {
        return;
    }
    if lab.warm < WARM_UP_FRAMES {
        lab.warm += 1;
        return;
    }
    let Some(shot) = lab.shots.get(lab.tour).cloned() else {
        if let Some(k) = lab.held.take() {
            keyboard.release(k);
        }
        // Let the last screenshots reach the disk before exiting.
        lab.wait += 1;
        if lab.wait > 30 {
            finish(&mut lab, &mut exit);
        }
        return;
    };
    let (name, capture_at, total) = match &shot {
        Shot::State(s) => (s.name.clone(), CAPTURE_AT, SHOT_FRAMES),
        // One key per frame (pressed on even frames, released on odd ones),
        // so the same key can be tapped several times.
        Shot::Input(s) => {
            let taps = 2 * s.taps.len() as u32;
            (s.name.clone(), taps + s.wait, taps + s.wait + 2)
        }
    };
    if lab.wait == 0 {
        match &shot {
            Shot::State(s) => apply_state(&mut lab, s),
            Shot::Input(s) => {
                if !lab.realtime {
                    // Hand over to the real controls, from a clean start.
                    lab.realtime = true;
                    lab.playing = true;
                    lab.x = 0.0;
                    lab.flip = false;
                    lab.skeleton = false;
                    lab.sockets = false;
                    lab.tint = false;
                    lab.exploded = false;
                    lab.layer_view = LayerView::All;
                    lab.chest_mode = ChestMode::Projected;
                }
                if let Some(old) = lab.held.take()
                    && Some(old) != s.hold
                {
                    keyboard.release(old);
                }
                if let Some(h) = s.hold {
                    keyboard.press(h);
                    lab.held = Some(h);
                }
            }
        }
    }
    if let Shot::Input(s) = &shot {
        let i = (lab.wait / 2) as usize;
        if let Some(k) = s.taps.get(i) {
            if lab.wait.is_multiple_of(2) {
                keyboard.press(*k);
            } else {
                keyboard.release(*k);
            }
        }
    }
    if lab.wait == capture_at {
        if let Shot::Input(s) = &shot
            && !(s.expect)(&lab)
        {
            let msg = format!("{}: the controls did not reach the expected state", s.name);
            lab.failures.push(msg);
        }
        if let Shot::State(s) = &shot
            && let Some(expected) = s.expect_frame
            && lab.frame != expected
        {
            let msg = format!("{}: frame {} expected {expected}", s.name, lab.frame);
            lab.failures.push(msg);
        }
        let path = lab.out.join(format!("{name}.png"));
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
        println!(
            "{TAG} {name} clip={} frame={} flip={} x={:.0} chest={} worn={:?}",
            lab.clip,
            lab.frame,
            lab.flip,
            lab.x,
            mode_tag(lab.chest_mode),
            lab.loadout.items().collect::<Vec<_>>()
        );
    }
    lab.wait += 1;
    if lab.wait >= total {
        lab.wait = 0;
        lab.tour += 1;
    }
}

fn apply_state(lab: &mut Lab, s: &StateShot) {
    lab.set_clip(s.clip);
    match s.frame {
        Some(f) => lab.elapsed = lab.frame_start(f) + 1e-4,
        None => {
            let step = lab
                .clip()
                .and_then(|c| c.frames.first())
                .map_or(0.0, |f| f.duration);
            lab.elapsed += step;
        }
    }
    lab.flip = s.flip;
    lab.wear(s.wear);
    lab.chest_mode = s.chest_mode;
    lab.skeleton = s.skeleton;
    lab.sockets = s.sockets;
    lab.tint = s.tint;
    lab.exploded = s.exploded;
    lab.layer_view = s.layer_view;
}

fn finish(lab: &mut Lab, exit: &mut MessageWriter<AppExit>) {
    if VERDICT.get().is_some() {
        return;
    }
    for f in &lab.failures {
        println!("{TAG} FAIL {f}");
    }
    let pass = lab.failures.is_empty();
    println!("{TAG} RESULT: {}", if pass { "PASS" } else { "FAIL" });
    let _ = VERDICT.set(pass);
    exit.write(AppExit::Success);
}

fn sync_panels(
    lab: Res<Lab>,
    mut roots: Query<(&PanelRoot, &mut Transform, &mut Visibility)>,
    mut original: Query<
        (&mut Sprite, &mut Anchor, &mut Visibility),
        (With<OriginalSprite>, Without<PanelRoot>),
    >,
) {
    for (root, mut t, mut v) in &mut roots {
        match root.0 {
            Panel::Hybrid => t.translation.x = Panel::Hybrid.x() + lab.x * SCALE,
            _ => {
                v.set_if_neq(if lab.comparison {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                });
            }
        }
    }
    let (Some(clip), Some((image, layout))) = (lab.clip(), lab.strips.get(&lab.clip)) else {
        return;
    };
    let Ok((mut sprite, mut anchor, mut vis)) = original.single_mut() else {
        return;
    };
    let source = clip.frames.get(lab.frame).map_or(0, |f| f.source);
    sprite.image = image.clone();
    sprite.texture_atlas = Some(TextureAtlas {
        layout: layout.clone(),
        index: source,
    });
    sprite.custom_size = Some(Vec2::new(clip.cell.0 as f32, clip.cell.1 as f32));
    sprite.flip_x = lab.flip;
    let (ax, ay) = anchor_for(clip.pivot, clip.cell, lab.flip);
    anchor.set_if_neq(Anchor(Vec2::new(ax, ay)));
    vis.set_if_neq(if lab.built.is_some() && lab.comparison {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    });
}

fn slot_z(slot: DepthSlot, sub: f32) -> f32 {
    (f32::from(slot.order()) + sub * 0.5) * SLOT_Z
}

fn sync_hybrid(
    lab: Res<Lab>,
    mut q: Query<(
        &HybridLayer,
        &mut Sprite,
        &mut Anchor,
        &mut Transform,
        &mut Visibility,
    )>,
) {
    let (Some(clip), Some(built)) = (lab.clip(), lab.built.as_ref()) else {
        return;
    };
    let Some(frame) = clip.frames.get(lab.frame) else {
        return;
    };
    for (layer, mut sprite, mut anchor, mut transform, mut vis) in &mut q {
        let image = built.layers.get(&(lab.clip.clone(), frame.source, layer.0));
        let (Some(image), true) = (image, lab.layer_view.shows_layer(layer.0)) else {
            vis.set_if_neq(Visibility::Hidden);
            continue;
        };
        if sprite.image != *image {
            sprite.image = image.clone();
        }
        sprite.custom_size = Some(Vec2::new(clip.cell.0 as f32, clip.cell.1 as f32));
        sprite.flip_x = lab.flip;
        sprite.color = if lab.tint {
            layer_tint(layer.0)
        } else {
            Color::WHITE
        };
        let (ax, ay) = anchor_for(clip.pivot, clip.cell, lab.flip);
        anchor.set_if_neq(Anchor(Vec2::new(ax, ay)));
        let drop = if layer.0.is_upper_body() {
            frame.drop as f32
        } else {
            0.0
        };
        transform.translation = Vec3::new(0.0, -drop, slot_z(layer.0.depth(), 0.0));
        vis.set_if_neq(Visibility::Inherited);
    }
}

/// The joints and pose a panel shows this frame (the articulated panel's
/// fist and torso box follow its own pose).
fn panel_pose(lab: &Lab, panel: Panel) -> Option<(JointPositions, WorldPose)> {
    let joints = lab.joints()?;
    Some(match panel {
        Panel::Articulated => {
            let pose = articulate(&lab.rest, &joints);
            let mut j = joints;
            j.set(Joint::HandNear, pose.segment(Bone::ForearmNear).1);
            j.set(Joint::HandFar, pose.segment(Bone::ForearmFar).1);
            (j, pose)
        }
        _ => {
            let pose = forward(&fit(&joints));
            (joints, pose)
        }
    })
}

/// Where a worn item goes in a panel (before mirroring), with its sprite.
fn item_placement(lab: &Lab, panel: Panel, id: &str) -> Option<(Placement, ItemDraw)> {
    let item = lab.rig.items.get(id)?;
    let (joints, pose) = panel_pose(lab, panel)?;
    match (&item.art, &item.mount) {
        (ItemArt::Sprite(art), Mount::Hand { side, up_degrees }) => {
            let grip = lab.rig.grip(&lab.clip)?;
            let hand = hand_frame(*side, &joints, &pose, &lab.rest, grip);
            let p = place_in_hand(hand, *up_degrees, grip.depth_at(lab.frame, item.depth));
            Some((p, ItemDraw::Sprite(art.clone())))
        }
        (ItemArt::Sprite(art), mount @ Mount::Bone { .. }) => {
            let p = place_on_bone(mount, item.depth, &pose, &lab.rest)?;
            Some((p, ItemDraw::Sprite(art.clone())))
        }
        (ItemArt::Model(_), Mount::Torso) => {
            let mode = if panel == Panel::Articulated {
                ChestMode::Rigid
            } else {
                lab.chest_mode
            };
            Some(match mode {
                ChestMode::Rigid => {
                    // The rest raster turned with the spine about the cage
                    // centre, moved to the frame's cage centre.
                    let lean =
                        pose.bone(Bone::Spine).angle - lab.rest_world.bone(Bone::Spine).angle;
                    let centre = match panel {
                        Panel::Articulated => {
                            let spine = pose.bone(Bone::Spine);
                            let rest = lab.rest_world.bone(Bone::Spine);
                            let local = rest.inverse().apply(lab.rest_cage.centre());
                            spine.apply(local)
                        }
                        _ => TorsoCage::from_joints(&joints).centre(),
                    };
                    (
                        Placement {
                            position: centre,
                            rotation: lean,
                            depth: item.depth,
                        },
                        ItemDraw::RigidChest,
                    )
                }
                other => (
                    Placement {
                        position: (0.0, 0.0),
                        rotation: 0.0,
                        depth: item.depth,
                    },
                    ItemDraw::CellChest(other),
                ),
            })
        }
        _ => None,
    }
}

/// How an item is drawn.
#[derive(Debug, Clone)]
enum ItemDraw {
    Sprite(SpriteArt),
    /// The rest raster as a sprite anchored on the rest cage centre.
    RigidChest,
    /// A cell-sized raster on the pivot (warp / projected).
    CellChest(ChestMode),
}

fn sync_items(
    lab: Res<Lab>,
    mut q: Query<(
        &ItemSprite,
        &mut Sprite,
        &mut Anchor,
        &mut Transform,
        &mut Visibility,
    )>,
) {
    let Some(built) = lab.built.as_ref() else {
        return;
    };
    let (Some(clip), Some(rest_clip)) = (lab.clip(), lab.rig.rest()) else {
        return;
    };
    for (item_sprite, mut sprite, mut anchor, mut transform, mut vis) in &mut q {
        let worn = lab.loadout.worn(item_sprite.slot).map(str::to_owned);
        let placed = worn
            .as_deref()
            .and_then(|id| Some((id, item_placement(&lab, item_sprite.panel, id)?)));
        let (Some((id, (placement, draw))), true) = (placed, lab.layer_view.shows_gear()) else {
            vis.set_if_neq(Visibility::Hidden);
            continue;
        };
        let (image, size, pivot, cell) = match &draw {
            ItemDraw::Sprite(art) => {
                let Some(h) = lab.items.get(id) else {
                    vis.set_if_neq(Visibility::Hidden);
                    continue;
                };
                (h.clone(), art.size, art.pivot, art.size)
            }
            ItemDraw::RigidChest => {
                let Some(h) = built
                    .chest
                    .get(&(lab.clip.clone(), lab.frame, ChestMode::Rigid))
                else {
                    vis.set_if_neq(Visibility::Hidden);
                    continue;
                };
                let g = rest_clip.geometry();
                (
                    h.clone(),
                    rest_clip.cell,
                    g.to_texel(lab.rest_cage.centre()),
                    rest_clip.cell,
                )
            }
            ItemDraw::CellChest(mode) => {
                let Some(h) = built.chest.get(&(lab.clip.clone(), lab.frame, *mode)) else {
                    vis.set_if_neq(Visibility::Hidden);
                    continue;
                };
                (h.clone(), clip.cell, clip.pivot, clip.cell)
            }
        };
        let exploded = lab.exploded && item_sprite.panel == Panel::Hybrid;
        let p = if exploded {
            let spot = exploded_item_spot(item_sprite.slot);
            let anchor_pos = match draw {
                // Cell rasters: shift so the plate lands at the spot.
                ItemDraw::CellChest(_) | ItemDraw::RigidChest => (spot.0, 0.0),
                ItemDraw::Sprite(_) => spot,
            };
            Placement {
                position: anchor_pos,
                rotation: 0.0,
                depth: DepthSlot::ShoulderGear,
            }
        } else {
            mirror(placement, lab.flip)
        };
        if sprite.image != image {
            sprite.image = image;
        }
        sprite.custom_size = Some(Vec2::new(size.0 as f32, size.1 as f32));
        sprite.flip_x = lab.flip;
        sprite.color = if lab.tint {
            Color::srgb(0.3, 1.0, 0.9)
        } else {
            Color::WHITE
        };
        let (ax, ay) = anchor_for(pivot, cell, lab.flip);
        anchor.set_if_neq(Anchor(Vec2::new(ax, ay)));
        transform.translation = Vec3::new(p.position.0, p.position.1, slot_z(p.depth, 0.0));
        transform.rotation = Quat::from_rotation_z(p.rotation);
        vis.set_if_neq(Visibility::Inherited);
    }
}

fn sync_articulated(
    lab: Res<Lab>,
    mut q: Query<(
        &PieceSprite,
        &mut Sprite,
        &mut Anchor,
        &mut Transform,
        &mut Visibility,
    )>,
) {
    let (Some(built), Some(rest_clip), Some((_, pose))) = (
        lab.built.as_ref(),
        lab.rig.rest(),
        panel_pose(&lab, Panel::Articulated),
    ) else {
        return;
    };
    let cell = rest_clip.geometry();
    let sign = if lab.flip { -1.0 } else { 1.0 };
    for (piece, mut sprite, mut anchor, mut transform, mut vis) in &mut q {
        let source = cutout_of(piece.0);
        let Some(image) = built.pieces.get(&source) else {
            vis.set_if_neq(Visibility::Hidden);
            continue;
        };
        if sprite.image != *image {
            sprite.image = image.clone();
        }
        sprite.custom_size = Some(Vec2::new(cell.width as f32, cell.height as f32));
        sprite.flip_x = lab.flip;
        let shade = if is_far(piece.0) { FAR_SHADE } else { 1.0 };
        sprite.color = Color::srgb(shade, shade, shade);
        let rest_bone = lab.rest_world.bone(source);
        let pivot_texel = cell.to_texel(rest_bone.origin());
        let (ax, ay) = anchor_for(pivot_texel, rest_clip.cell, lab.flip);
        anchor.set_if_neq(Anchor(Vec2::new(ax, ay)));
        let (origin, rotation) = if lab.exploded {
            let o = rest_bone.origin();
            let d = explode_offset(piece.0);
            ((o.0 + d.0, o.1 + d.1), 0.0)
        } else {
            let bone = pose.bone(piece.0);
            (bone.origin(), bone.angle - rest_bone.angle)
        };
        let (slot, sub) = piece_depth(piece.0);
        transform.translation = Vec3::new(origin.0 * sign, origin.1, slot_z(slot, sub));
        transform.rotation = Quat::from_rotation_z(rotation * sign);
        vis.set_if_neq(Visibility::Inherited);
    }
}

/// Screen point of a rig point in a panel.
fn screen(lab: &Lab, panel: Panel, p: (f32, f32)) -> Vec2 {
    let sign = if lab.flip { -1.0 } else { 1.0 };
    let x0 = panel.x()
        + if panel == Panel::Hybrid {
            lab.x * SCALE
        } else {
            0.0
        };
    Vec2::new(x0 + p.0 * sign * SCALE, SOLE_Y + p.1 * SCALE)
}

fn bone_colour(bone: Bone) -> Color {
    match bone {
        Bone::Spine | Bone::Head => Color::srgb(1.0, 1.0, 1.0),
        Bone::UpperArmNear | Bone::ForearmNear => Color::srgb(1.0, 0.3, 0.3),
        Bone::UpperArmFar | Bone::ForearmFar => Color::srgb(0.3, 0.6, 1.0),
        Bone::ThighA | Bone::ShinA | Bone::FootA => Color::srgb(1.0, 0.85, 0.1),
        Bone::ThighB | Bone::ShinB | Bone::FootB => Color::srgb(0.1, 1.0, 0.5),
    }
}

fn overlays(lab: Res<Lab>, mut gizmos: Gizmos) {
    if lab.built.is_none() {
        return;
    }
    let panels: &[Panel] = if lab.comparison {
        &[Panel::Hybrid, Panel::Articulated]
    } else {
        &[Panel::Hybrid]
    };
    let cyan = Color::srgb(0.0, 1.0, 1.0);
    let magenta = Color::srgb(1.0, 0.0, 1.0);
    for &panel in panels {
        let Some((joints, pose)) = panel_pose(&lab, panel) else {
            continue;
        };
        if lab.skeleton && !lab.exploded {
            for bone in Bone::ALL {
                let (a, b) = pose.segment(bone);
                gizmos.line_2d(
                    screen(&lab, panel, a),
                    screen(&lab, panel, b),
                    bone_colour(bone),
                );
                gizmos.circle_2d(screen(&lab, panel, a), 2.5, Color::WHITE);
                if panel == Panel::Hybrid && lab.comparison {
                    gizmos.line_2d(
                        screen(&lab, Panel::Original, a),
                        screen(&lab, Panel::Original, b),
                        bone_colour(bone),
                    );
                }
            }
            if panel == Panel::Hybrid {
                // The torso box (cage of the breastplate).
                let c = TorsoCage::from_joints(&joints);
                let q = [c.chest_back, c.chest_front, c.waist_front, c.waist_back];
                for i in 0..4 {
                    gizmos.line_2d(
                        screen(&lab, panel, q[i]),
                        screen(&lab, panel, q[(i + 1) % 4]),
                        Color::srgb(0.6, 1.0, 0.3),
                    );
                }
            }
        }
        if lab.sockets && !(lab.exploded && panel == Panel::Hybrid) {
            for (_, id) in lab.loadout.items() {
                let Some((p, draw)) = item_placement(&lab, panel, id) else {
                    continue;
                };
                if matches!(draw, ItemDraw::CellChest(_)) {
                    continue;
                }
                let p = mirror(p, lab.flip);
                let sign = if lab.flip { -1.0 } else { 1.0 };
                let c = screen(&lab, panel, (p.position.0 * sign, p.position.1));
                gizmos.cross_2d(Isometry2d::from_translation(c), 7.0, cyan);
                let up = Vec2::new(-p.rotation.sin(), p.rotation.cos()) * 30.0;
                gizmos.arrow_2d(c, c + up, magenta);
            }
        }
        if lab.sockets && lab.exploded && panel == Panel::Hybrid {
            for (slot, _) in lab.loadout.items() {
                let spot = exploded_item_spot(slot);
                let c = screen(&lab, panel, spot);
                gizmos.cross_2d(Isometry2d::from_translation(c), 7.0, cyan);
            }
        }
    }
    gizmos.line_2d(
        Vec2::new(-640.0, SOLE_Y),
        Vec2::new(640.0, SOLE_Y),
        Color::srgba(1.0, 1.0, 1.0, 0.15),
    );
}

fn hud(lab: Res<Lab>, mut q: Query<&mut Text, With<Hud>>) {
    let Ok(mut text) = q.single_mut() else {
        return;
    };
    let worn: Vec<String> = lab
        .loadout
        .items()
        .map(|(s, id)| format!("{s:?}={id}"))
        .collect();
    text.0 = format!(
        "MODULAR CHARACTER ENGINE 02 (lab, not approved)   clip {} frame {} {}{}   breastplate: {}   view: {}\n\
         {}\n\
         worn: {}{}{}\n\
         arrows/A D walk  C idle/run  M turn  Space pause  . , frame  1 sword  2 pauldron  3 breastplate  4 approach  9 all  0 none  S skeleton  P sockets  T tint  L layers  X exploded  O panels",
        lab.clip,
        lab.frame,
        if lab.flip {
            "facing left"
        } else {
            "facing right"
        },
        if lab.playing { "" } else { "  [PAUSED]" },
        mode_tag(lab.chest_mode),
        lab.layer_view.label(),
        if lab.comparison {
            "ORIGINAL (approved frame)        HYBRID (approved frame + independent equipment)        ARTICULATED (experiment)"
        } else {
            "HYBRID only (O shows the comparison panels)"
        },
        if worn.is_empty() {
            "nothing".to_owned()
        } else {
            worn.join("  ")
        },
        if lab.tint {
            "   [depth layers tinted]"
        } else {
            ""
        },
        if lab.exploded { "   [exploded]" } else { "" },
    );
}

/// Writes `rig_export.json`: the rest bones and, per run frame, the local
/// rotations (degrees clockwise on screen, Agent Sprite Forge rig_anim.v1
/// convention) and our forward-kinematics result on the rest proportions,
/// in run-canvas texels (y down). `scripts/run_external_tools.py` renders
/// the same poses with ASF `rig_animate.py` and compares the joints.
fn export_rig(rig: &RigLab, rest: &LocalPose, out: &std::path::Path) -> std::io::Result<()> {
    let Some(run) = rig.clips.get("run") else {
        return Ok(());
    };
    let g = run.geometry();
    let tex = |p: (f32, f32)| {
        let t = g.to_texel(p);
        format!("[{:.3},{:.3}]", t.0, t.1)
    };
    let seg = |w: &WorldPose, b: Bone| {
        let (a, e) = w.segment(b);
        format!("[{},{}]", tex(a), tex(e))
    };
    let name = |b: Bone| format!("{b:?}");
    let rest_world = forward(rest);
    let bones: Vec<String> = Bone::ALL
        .iter()
        .map(|b| {
            let parent = b
                .parent()
                .map_or("null".to_owned(), |p| format!("\"{}\"", name(p)));
            format!(
                "\"{}\": {{\"parent\": {parent}, \"seg\": {}}}",
                name(*b),
                seg(&rest_world, *b)
            )
        })
        .collect();
    let mut frames = Vec::new();
    for f in 0..run.frames.len() {
        let Some(j) = run.frame_joints(f) else {
            continue;
        };
        let pose = retarget(rest, &fit(&j));
        let world = forward(&pose);
        let rot: Vec<String> = Bone::ALL
            .iter()
            .map(|b| {
                let d = (pose.local[b.index()].angle - rest.local[b.index()].angle).to_degrees();
                format!("\"{}\": {:.4}", name(*b), -d)
            })
            .collect();
        let p0 = rest_world.bone(Bone::Spine).origin();
        let p1 = world.bone(Bone::Spine).origin();
        let fk: Vec<String> = Bone::ALL
            .iter()
            .map(|b| format!("\"{}\": {}", name(*b), seg(&world, *b)))
            .collect();
        frames.push(format!(
            "{{\"translate\": [{:.4},{:.4}], \"rotate\": {{{}}}, \"fk\": {{{}}}}}",
            p1.0 - p0.0,
            -(p1.1 - p0.1),
            rot.join(", "),
            fk.join(", ")
        ));
    }
    let text = format!(
        "{{\"canvas\": [{}, {}], \"pivot\": [{}, {}], \"bones\": {{{}}}, \"frames\": [{}]}}\n",
        g.width,
        g.height,
        g.pivot.0,
        g.pivot.1,
        bones.join(", "),
        frames.join(", ")
    );
    std::fs::write(out.join("rig_export.json"), text)
}

fn main() -> AppExit {
    let root = project_root();
    let interactive = std::env::var_os("MCE_INTERACTIVE").is_some();
    let path = root.join(STAGE).join("rig_lab.ron");
    let rig = match std::fs::read_to_string(&path)
        .map_err(|e| e.to_string())
        .and_then(|t| RigLab::from_ron(&t).map_err(|e| e.to_string()))
    {
        Ok(rig) => rig,
        Err(error) => {
            eprintln!("{TAG} {}: {error}", path.display());
            return AppExit::error();
        }
    };
    let Some(rest_joints) = rig.rest().and_then(|c| c.frame_joints(0)) else {
        eprintln!("{TAG} rest pose missing");
        return AppExit::error();
    };
    let rest = fit(&rest_joints);
    let rest_world = forward(&rest);
    let rest_cage = TorsoCage::from_joints(&rest_joints);
    let out = root.join(CAPTURES);
    if let Err(error) = std::fs::create_dir_all(&out) {
        eprintln!("{TAG} {}: {error}", out.display());
        return AppExit::error();
    }
    if let Err(error) = export_rig(&rig, &rest, &out) {
        eprintln!("{TAG} rig export: {error}");
        return AppExit::error();
    }
    let mut app = App::new();
    app.register_asset_source(
        "approved",
        AssetSourceBuilder::platform_default(&root.join(APPROVED).to_string_lossy(), None),
    )
    .register_asset_source(
        "factory",
        AssetSourceBuilder::platform_default(&root.join(FACTORY).to_string_lossy(), None),
    )
    .register_asset_source(
        "mce",
        AssetSourceBuilder::platform_default(&root.join(STAGE).to_string_lossy(), None),
    )
    .add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Modular Character Engine 02".into(),
                    resolution: WindowResolution::new(1280, 720),
                    ..default()
                }),
                ..default()
            }),
    )
    .insert_resource(ClearColor(DARK))
    .insert_resource(Lab {
        rig,
        rest,
        rest_world,
        rest_cage,
        strips: HashMap::new(),
        items: HashMap::new(),
        built: None,
        clip: "idle".to_owned(),
        elapsed: 0.0,
        frame: 0,
        flip: false,
        playing: true,
        x: 0.0,
        walking: false,
        loadout: RigLoadout::default(),
        chest_mode: ChestMode::Projected,
        skeleton: false,
        sockets: false,
        tint: false,
        exploded: false,
        layer_view: LayerView::All,
        comparison: true,
        interactive,
        realtime: false,
        shots: tour(),
        tour: 0,
        wait: 0,
        warm: 0,
        held: None,
        out,
        failures: Vec::new(),
    })
    .add_systems(Startup, setup)
    .add_systems(
        Update,
        (
            build_layers,
            run_tour,
            keys,
            advance,
            sync_panels,
            sync_hybrid,
            sync_items,
            sync_articulated,
            overlays,
            hud,
        )
            .chain(),
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
