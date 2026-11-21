//! Dark Castle Modular Character Engine — prototype 01 (isolated lab, not
//! the game).
//!
//! Three panels, all at 3 screen pixels per texel and on one sole line:
//!
//! - **left, original**: the approved frame (N3.5 idle / N3.6 run6), one
//!   sprite, untouched;
//! - **centre, hybrid**: the same approved frame cut by its marked skeleton
//!   into five depth layers (`rig::layers`), with equipment drawn once and
//!   bound to the bones (`rig::equipment`);
//! - **right, articulated**: pieces cut from the idle frame and moved by
//!   forward kinematics plus two-bone IK (`rig::articulated`) to the same
//!   poses, wearing the same items: the honest comparison.
//!
//! ```text
//! cargo run --example modular_character_engine          # scripted tour + checks, then exit
//! MCE_INTERACTIVE=1 cargo run --example modular_character_engine
//! ```
//!
//! | Key | Effect |
//! |---|---|
//! | Space | play / pause |
//! | Left Right | previous / next frame (paused) |
//! | C | idle / run |
//! | M | facing left / right |
//! | 1 | main hand: sword A -> sword B -> none |
//! | 2 | shoulder: pauldron A -> pauldron B -> none |
//! | S | skeleton overlay |
//! | P | sockets and pivots |
//! | T | tint the depth layers |
//! | X | exploded pieces |
//!
//! The tour writes raw captures to `target/modular_character_engine/` and
//! prints `RESULT: PASS/FAIL`; `art-src/modular-character-engine/scripts/
//! make_evidence.py` turns them into `evidence/modular_character_engine/
//! 01_prototipo/`.

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
use dark_castle_rpg::player::visual::rig::data::{RigClip, RigLab};
use dark_castle_rpg::player::visual::rig::equipment::{RigLoadout, RigSlot, mirror, place};
use dark_castle_rpg::player::visual::rig::layers::{
    BodyLayer, CUTOUT_BONES, DepthSlot, extract_cell, masked_rgba, material_of, partition,
    partition_by_bone,
};
use dark_castle_rpg::player::visual::rig::skeleton::{
    Bone, JointPositions, LocalPose, WorldPose, fit, forward,
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

#[derive(Component)]
struct OriginalSprite;

#[derive(Component)]
struct PanelRoot;

#[derive(Component)]
struct HybridLayer(BodyLayer);

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
        Bone::UpperArmNear => (DepthSlot::NearArm, 0.0),
        Bone::ForearmNear => (DepthSlot::NearArm, 0.5),
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

/// Debug colours of the depth layers (tint mode).
fn layer_tint(layer: BodyLayer) -> Color {
    match layer {
        BodyLayer::FarArm => Color::srgb(0.35, 0.55, 1.0),
        BodyLayer::Legs => Color::srgb(0.55, 0.45, 0.35),
        BodyLayer::Torso => Color::srgb(0.6, 0.6, 0.6),
        BodyLayer::Hair => Color::srgb(1.0, 0.95, 0.4),
        BodyLayer::NearArm => Color::srgb(1.0, 0.35, 0.35),
    }
}

/// One scripted capture.
#[derive(Debug, Clone)]
struct Shot {
    name: String,
    clip: &'static str,
    /// `Some(f)`: jump to the start of frame f; `None`: advance the clock by
    /// one run frame (1/15 s) without resetting it.
    frame: Option<usize>,
    flip: bool,
    weapon: Option<usize>,
    shoulder: Option<usize>,
    skeleton: bool,
    sockets: bool,
    tint: bool,
    exploded: bool,
    /// The frame the clock must show (continuity checks).
    expect_frame: Option<usize>,
}

fn shot(name: impl Into<String>, clip: &'static str, frame: Option<usize>) -> Shot {
    Shot {
        name: name.into(),
        clip,
        frame,
        flip: false,
        weapon: None,
        shoulder: None,
        skeleton: false,
        sockets: false,
        tint: false,
        exploded: false,
        expect_frame: None,
    }
}

fn tour() -> Vec<Shot> {
    let mut out = Vec::new();
    // Bare: the hybrid must match the original pixel for pixel.
    for f in 0..2 {
        out.push(shot(format!("bare_idle_f{f}"), "idle", Some(f)));
    }
    for f in 0..6 {
        out.push(shot(format!("bare_run_f{f}"), "run", Some(f)));
    }
    // Skeleton over the character, idle and every run frame.
    out.push(Shot {
        skeleton: true,
        ..shot("skeleton_idle_f0", "idle", Some(0))
    });
    for f in 0..6 {
        out.push(Shot {
            skeleton: true,
            ..shot(format!("skeleton_run_f{f}"), "run", Some(f))
        });
    }
    // Equipped: sword A + pauldron A.
    for f in 0..2 {
        out.push(Shot {
            weapon: Some(0),
            shoulder: Some(0),
            ..shot(format!("equipped_idle_f{f}"), "idle", Some(f))
        });
    }
    for f in 0..6 {
        out.push(Shot {
            weapon: Some(0),
            shoulder: Some(0),
            ..shot(format!("equipped_run_f{f}"), "run", Some(f))
        });
    }
    // Depth layers tinted, and sockets/pivots.
    for f in 0..6 {
        out.push(Shot {
            weapon: Some(0),
            shoulder: Some(0),
            tint: true,
            ..shot(format!("depth_run_f{f}"), "run", Some(f))
        });
    }
    for (clip, f) in [("idle", 0), ("run", 1), ("run", 4)] {
        out.push(Shot {
            weapon: Some(0),
            shoulder: Some(0),
            sockets: true,
            skeleton: true,
            ..shot(format!("sockets_{clip}_f{f}"), clip, Some(f))
        });
    }
    out.push(Shot {
        weapon: Some(0),
        shoulder: Some(0),
        exploded: true,
        sockets: true,
        ..shot("exploded_idle_f0", "idle", Some(0))
    });
    // Swap during the run: the clock only advances, never resets.
    let swap: [(Option<usize>, Option<usize>); 6] = [
        (Some(0), Some(0)),
        (Some(1), Some(0)),
        (Some(1), Some(1)),
        (None, Some(1)),
        (None, None),
        (Some(0), Some(0)),
    ];
    out.push(Shot {
        weapon: Some(0),
        shoulder: Some(0),
        expect_frame: Some(0),
        ..shot("swap_s00", "run", Some(0))
    });
    for step in 1..24 {
        let (weapon, shoulder) = swap[step / 4];
        out.push(Shot {
            weapon,
            shoulder,
            expect_frame: Some(step % 6),
            ..shot(format!("swap_s{step:02}"), "run", None)
        });
    }
    // Turning around mid-run (equipment stays bound).
    out.push(Shot {
        weapon: Some(0),
        shoulder: Some(0),
        expect_frame: Some(0),
        ..shot("turn_t00", "run", Some(0))
    });
    for step in 1..18 {
        out.push(Shot {
            weapon: Some(0),
            shoulder: Some(0),
            flip: (5..12).contains(&step),
            expect_frame: Some(step % 6),
            ..shot(format!("turn_t{step:02}"), "run", None)
        });
    }
    // Mirrored, still.
    out.push(Shot {
        weapon: Some(1),
        shoulder: Some(1),
        flip: true,
        skeleton: true,
        ..shot("mirror_run_f2", "run", Some(2))
    });
    out
}

/// Images built from the approved frames once they are loaded.
struct Built {
    /// (clip, source, layer) -> layer image.
    layers: HashMap<(String, usize, BodyLayer), Handle<Image>>,
    /// Cutout pieces of the idle (cell-sized images).
    pieces: HashMap<Bone, Handle<Image>>,
}

#[derive(Resource)]
struct Lab {
    rig: RigLab,
    rest: LocalPose,
    rest_world: WorldPose,
    strips: HashMap<String, (Handle<Image>, Handle<TextureAtlasLayout>)>,
    items: HashMap<String, Handle<Image>>,
    built: Option<Built>,
    clip: String,
    elapsed: f32,
    frame: usize,
    flip: bool,
    playing: bool,
    loadout: RigLoadout,
    skeleton: bool,
    sockets: bool,
    tint: bool,
    exploded: bool,
    interactive: bool,
    shots: Vec<Shot>,
    tour: usize,
    wait: u32,
    warm: u32,
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

    fn frame_start(&self, frame: usize) -> f32 {
        self.clip()
            .map(|c| c.frames.iter().take(frame).map(|f| f.duration).sum())
            .unwrap_or(0.0)
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
        .map(|(k, i)| (k.clone(), i.texture.clone()))
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
                PanelRoot,
                Transform::from_xyz(panel.x(), SOLE_Y, 0.0).with_scale(Vec3::splat(SCALE)),
                Visibility::default(),
            ))
            .id();
        if panel == Panel::Hybrid {
            for layer in BodyLayer::ALL {
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
        for slot in [RigSlot::MainHand, RigSlot::Shoulder] {
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
            font_size: FontSize::Px(14.0),
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

/// Once the approved strips are in memory: cut every source into its
/// depth layers (and the idle into pieces), and check the partition.
fn build_layers(mut lab: ResMut<Lab>, mut images: ResMut<Assets<Image>>) {
    if lab.built.is_some() {
        return;
    }
    let ready = lab.strips.values().all(|(h, _)| images.contains(h))
        && lab.items.values().all(|h| images.contains(h));
    if !ready {
        return;
    }
    let mut layers = HashMap::new();
    let mut pieces = HashMap::new();
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
        for source in 0..clip.sources.len() {
            let (Some(texels), Some(joints)) = (
                extract_cell(&data, strip_width, source, cell),
                clip.source_joints(source),
            ) else {
                failures.push(format!("{key} #{source}: cannot cut the cell"));
                continue;
            };
            let materials: Vec<_> = texels.iter().map(|t| material_of(*t)).collect();
            let labels = partition(&materials, cell, &joints);
            // Partition check: every opaque texel in exactly one layer and
            // the union identical to the approved frame.
            let mut union = vec![[0u8; 4]; texels.len()];
            for layer in BodyLayer::ALL {
                let bytes = masked_rgba(&texels, &labels, layer);
                for (i, px) in bytes.as_chunks::<4>().0.iter().enumerate() {
                    if px[3] > 0 {
                        union[i] = [px[0], px[1], px[2], px[3]];
                    }
                }
                let h = images.add(rgba_image(cell.width as u32, cell.height as u32, bytes));
                layers.insert((key.clone(), source, layer), h);
            }
            let differing = union.iter().zip(&texels).filter(|(a, b)| a != b).count();
            println!("{TAG} partition {key} #{source}: {differing} texels differ");
            if differing != 0 {
                failures.push(format!("{key} #{source}: {differing} texels differ"));
            }
            if *key == lab.rig.rest_clip && source == 0 {
                let bones = partition_by_bone(&materials, cell, &joints);
                for bone in CUTOUT_BONES {
                    let bytes = masked_rgba(&texels, &bones, bone);
                    let h = images.add(rgba_image(cell.width as u32, cell.height as u32, bytes));
                    pieces.insert(bone, h);
                }
            }
        }
    }
    lab.failures.extend(failures);
    lab.built = Some(Built { layers, pieces });
}

fn keys(input: Res<ButtonInput<KeyCode>>, mut lab: ResMut<Lab>) {
    if !lab.interactive {
        return;
    }
    let frames = lab.clip().map_or(1, |c| c.frames.len());
    if input.just_pressed(KeyCode::Space) {
        lab.playing = !lab.playing;
    }
    if input.just_pressed(KeyCode::ArrowRight) && !lab.playing {
        lab.frame = (lab.frame + 1) % frames;
        lab.elapsed = lab.frame_start(lab.frame);
    }
    if input.just_pressed(KeyCode::ArrowLeft) && !lab.playing {
        lab.frame = (lab.frame + frames - 1) % frames;
        lab.elapsed = lab.frame_start(lab.frame);
    }
    if input.just_pressed(KeyCode::KeyC) {
        lab.clip = if lab.clip == "run" { "idle" } else { "run" }.to_owned();
        lab.frame = 0;
        lab.elapsed = 0.0;
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
    if input.just_pressed(KeyCode::KeyS) {
        lab.skeleton = !lab.skeleton;
    }
    if input.just_pressed(KeyCode::KeyP) {
        lab.sockets = !lab.sockets;
    }
    if input.just_pressed(KeyCode::KeyT) {
        lab.tint = !lab.tint;
    }
    if input.just_pressed(KeyCode::KeyX) {
        lab.exploded = !lab.exploded;
    }
}

/// The one clock: the frame always comes from `elapsed` through the same
/// loop function as the player animation. Equipment never touches it.
fn advance(time: Res<Time>, mut lab: ResMut<Lab>) {
    if lab.interactive && lab.playing {
        lab.elapsed += clamp_delta(time.delta_secs());
    }
    let durations = lab.clip().map(RigClip::durations).unwrap_or_default();
    lab.frame = loop_frame(&durations, lab.elapsed);
}

fn run_tour(mut commands: Commands, mut lab: ResMut<Lab>, mut exit: MessageWriter<AppExit>) {
    if lab.interactive || lab.built.is_none() {
        return;
    }
    if lab.warm < WARM_UP_FRAMES {
        lab.warm += 1;
        return;
    }
    let Some(s) = lab.shots.get(lab.tour).cloned() else {
        finish(&mut lab, &mut exit);
        return;
    };
    if lab.wait == 0 {
        if lab.clip != s.clip {
            lab.clip = s.clip.to_owned();
            lab.elapsed = 0.0;
        }
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
        lab.wear_variant(RigSlot::MainHand, s.weapon);
        lab.wear_variant(RigSlot::Shoulder, s.shoulder);
        lab.skeleton = s.skeleton;
        lab.sockets = s.sockets;
        lab.tint = s.tint;
        lab.exploded = s.exploded;
    }
    if lab.wait == CAPTURE_AT {
        if let Some(expected) = s.expect_frame
            && lab.frame != expected
        {
            let msg = format!("{}: frame {} expected {expected}", s.name, lab.frame);
            lab.failures.push(msg);
        }
        let path = lab.out.join(format!("{}.png", s.name));
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
        println!(
            "{TAG} {} clip={} frame={} flip={} worn={:?}",
            s.name,
            lab.clip,
            lab.frame,
            lab.flip,
            lab.loadout.items().collect::<Vec<_>>()
        );
    }
    lab.wait += 1;
    if lab.wait >= SHOT_FRAMES {
        lab.wait = 0;
        lab.tour += 1;
    }
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

fn sync_original(
    lab: Res<Lab>,
    mut q: Query<(&mut Sprite, &mut Anchor, &mut Visibility), With<OriginalSprite>>,
) {
    let (Some(clip), Some((image, layout))) = (lab.clip(), lab.strips.get(&lab.clip)) else {
        return;
    };
    let Ok((mut sprite, mut anchor, mut vis)) = q.single_mut() else {
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
    vis.set_if_neq(if lab.built.is_some() {
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
        let Some(image) = built.layers.get(&(lab.clip.clone(), frame.source, layer.0)) else {
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

/// The pose a panel shows this frame.
fn panel_pose(lab: &Lab, panel: Panel) -> Option<WorldPose> {
    let joints = lab.joints()?;
    Some(match panel {
        Panel::Articulated => articulate(&lab.rest, &joints),
        _ => forward(&fit(&joints)),
    })
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
    let built = lab.built.is_some();
    for (item_sprite, mut sprite, mut anchor, mut transform, mut vis) in &mut q {
        let worn = lab.loadout.worn(item_sprite.slot);
        let item = worn.and_then(|id| Some((lab.rig.items.get(id)?, lab.items.get(id)?)));
        let pose = panel_pose(&lab, item_sprite.panel);
        let (Some((item, image)), Some(pose), true) = (item, pose, built) else {
            vis.set_if_neq(Visibility::Hidden);
            continue;
        };
        let (position, rotation, depth) = if lab.exploded && item_sprite.panel == Panel::Hybrid {
            // Laid out beside the body, upright, to show the piece alone.
            let x = match item_sprite.slot {
                RigSlot::MainHand => 46.0,
                RigSlot::Shoulder => -46.0,
            };
            ((x, 40.0), 0.0, DepthSlot::ShoulderGear)
        } else {
            let p = mirror(place(item, &pose, &lab.rest), lab.flip);
            (p.position, p.rotation, p.depth)
        };
        if sprite.image != *image {
            sprite.image = image.clone();
        }
        sprite.custom_size = Some(Vec2::new(item.size.0 as f32, item.size.1 as f32));
        sprite.flip_x = lab.flip;
        let (ax, ay) = anchor_for(item.pivot, item.size, lab.flip);
        anchor.set_if_neq(Anchor(Vec2::new(ax, ay)));
        transform.translation = Vec3::new(position.0, position.1, slot_z(depth, 0.0));
        transform.rotation = Quat::from_rotation_z(rotation);
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
    let (Some(built), Some(rest_clip), Some(pose)) = (
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
        // The piece turns around its first joint in the rest frame.
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
fn screen(panel: Panel, p: (f32, f32), flip: bool) -> Vec2 {
    let sign = if flip { -1.0 } else { 1.0 };
    Vec2::new(panel.x() + p.0 * sign * SCALE, SOLE_Y + p.1 * SCALE)
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
    if lab.built.is_none() || lab.exploded && !lab.sockets {
        return;
    }
    for panel in [Panel::Hybrid, Panel::Articulated] {
        let Some(pose) = panel_pose(&lab, panel) else {
            continue;
        };
        if lab.skeleton && !lab.exploded {
            for bone in Bone::ALL {
                let (a, b) = pose.segment(bone);
                gizmos.line_2d(
                    screen(panel, a, lab.flip),
                    screen(panel, b, lab.flip),
                    bone_colour(bone),
                );
                gizmos.circle_2d(screen(panel, a, lab.flip), 2.5, Color::WHITE);
            }
            if panel == Panel::Hybrid {
                // Also over the original (the marked joints are the same).
                for bone in Bone::ALL {
                    let (a, b) = pose.segment(bone);
                    gizmos.line_2d(
                        screen(Panel::Original, a, lab.flip),
                        screen(Panel::Original, b, lab.flip),
                        bone_colour(bone),
                    );
                }
            }
        }
        if lab.sockets {
            for (_, id) in lab.loadout.items() {
                let Some(item) = lab.rig.items.get(id) else {
                    continue;
                };
                if lab.exploded && panel == Panel::Hybrid {
                    let x = match item.slot {
                        RigSlot::MainHand => 46.0,
                        RigSlot::Shoulder => -46.0,
                    };
                    let c = screen(panel, (x, 40.0), lab.flip);
                    gizmos.cross_2d(
                        Isometry2d::from_translation(c),
                        7.0,
                        Color::srgb(0.0, 1.0, 1.0),
                    );
                    continue;
                }
                let p = mirror(place(item, &pose, &lab.rest), lab.flip);
                let c = screen(panel, p.position, false);
                gizmos.cross_2d(
                    Isometry2d::from_translation(c),
                    7.0,
                    Color::srgb(0.0, 1.0, 1.0),
                );
                // The item's "up" axis.
                let up = Vec2::new(-p.rotation.sin(), p.rotation.cos()) * 30.0;
                gizmos.arrow_2d(c, c + up, Color::srgb(1.0, 0.0, 1.0));
                // The bone it is bound to.
                let (a, b) = pose.segment(item.bone);
                gizmos.line_2d(
                    screen(panel, a, lab.flip),
                    screen(panel, b, lab.flip),
                    Color::srgb(0.0, 1.0, 1.0),
                );
            }
        }
    }
    // Sole line.
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
        "MODULAR CHARACTER ENGINE 01 (prototype, not approved)   clip {} frame {} {}\n\
         ORIGINAL (approved frame)        HYBRID (approved frame + rig-bound items)        ARTICULATED (idle pieces + FK/IK)\n\
         worn: {}{}{}{}",
        lab.clip,
        lab.frame,
        if lab.flip {
            "facing left"
        } else {
            "facing right"
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
        if lab.interactive {
            "\nSpace play  Left/Right frame  C clip  M mirror  1 weapon  2 shoulder  S skeleton  P sockets  T tint  X exploded"
        } else {
            ""
        },
    );
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
    let out = root.join(CAPTURES);
    if let Err(error) = std::fs::create_dir_all(&out) {
        eprintln!("{TAG} {}: {error}", out.display());
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
                    title: "Modular Character Engine 01".into(),
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
        strips: HashMap::new(),
        items: HashMap::new(),
        built: None,
        clip: "run".to_owned(),
        elapsed: 0.0,
        frame: 0,
        flip: false,
        playing: true,
        loadout: RigLoadout::default(),
        skeleton: false,
        sockets: false,
        tint: false,
        exploded: false,
        interactive,
        shots: tour(),
        tour: 0,
        wait: 0,
        warm: 0,
        out,
        failures: Vec::new(),
    })
    .add_systems(Startup, setup)
    .add_systems(
        Update,
        (
            build_layers,
            keys,
            run_tour,
            advance,
            sync_original,
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
