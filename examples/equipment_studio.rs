//! Dark Castle Equipment Studio — Modular Character Engine 03 (a tool,
//! separate from the game).
//!
//! Edits equipment designs (`art-src/equipment-studio/items/*.ron`, the
//! generic model of `player::visual::rig::design`) live on the approved
//! protagonist (N3.5 idle, N3.6 run) driven by the invisible skeleton of the
//! Modular Character Engine. Every item is drawn by the generic painter
//! (`rig::paint`) straight on the character's texel grid; the main view is
//! an integer zoom, the right-hand previews are 2x and 1x (native).
//!
//! ```text
//! cargo run --example equipment_studio                 # the editor
//! cargo run --example equipment_studio -- --tour       # scripted evidence run (real keys), then exit
//! cargo run --example equipment_studio -- --tour-reload  # reopens what --tour saved, then exit
//! ```
//!
//! | Key | Effect |
//! |---|---|
//! | PageUp / PageDown | select the design |
//! | Enter | equip / unequip the selected design (one per slot) |
//! | Ctrl+N / Ctrl+S / Ctrl+R | new design / save it / reload every design from disk |
//! | Ctrl+P | next slot for the selected design |
//! | Tab / Shift+Tab | next / previous part |
//! | N / I / Delete / Ctrl+D | add a polygon part / import the next PNG of `import/` / remove / duplicate |
//! | H / [ ] | hide or show the part / move it back or front |
//! | M | edit mode: part -> vertex -> pivot -> mount -> colour |
//! | Arrows (Shift = x4) | move the part / vertex / pivot / mount; colour: Left/Right ramp slot, Up/Down brightness |
//! | Q / E, - / = | rotate, scale (part or mount) |
//! | V / B, Y, Backspace | previous / next vertex, insert a vertex, delete it |
//! | K, R G B (Shift = -) | colour mode: cycle the part's material, edit the ramp slot |
//! | Space, . / , | play / pause, next / previous frame |
//! | T, F | idle / run, face left / right |
//! | Z / X or mouse wheel | zoom out / in (integer, no smoothing) |
//! | O, U, G | item isolated, character unequipped, skeleton and torso cage |
//! | Mouse (left drag) | vertex mode: drag the nearest vertex; part mode: drag the part; pivot mode: place the pivot |

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use bevy::asset::AssetApp;
use bevy::asset::io::AssetSourceBuilder;
use bevy::input::mouse::AccumulatedMouseScroll;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::sprite::Anchor;
use bevy::window::{PrimaryWindow, WindowResolution};
use dark_castle_rpg::core::pixel_art::PixelArt;
use dark_castle_rpg::core::pixel_image::pixel_image;
use dark_castle_rpg::core::resources::project_root;
use dark_castle_rpg::core::time::clamp_delta;
use dark_castle_rpg::player::visual::manifest::anchor_for;
use dark_castle_rpg::player::visual::playback::loop_frame;
use dark_castle_rpg::player::visual::rig::chest::TorsoCage;
use dark_castle_rpg::player::visual::rig::data::{RigClip, RigLab};
use dark_castle_rpg::player::visual::rig::design::{
    DESIGN_VERSION, DesignSpace, EquipmentDesign, Finish, Outline, PaintMaterial, Part, PartSource,
    PartTransform, Visibility as PartVisibility, chaikin,
};
use dark_castle_rpg::player::visual::rig::equipment::RigSlot;
use dark_castle_rpg::player::visual::rig::layers::{BodyLayer, DepthSlot};
use dark_castle_rpg::player::visual::rig::paint::{PaintJob, RgbaImage, SpaceMap, paint, pose_map};
use dark_castle_rpg::player::visual::rig::skeleton::{Bone, LocalPose, fit, forward};

const TAG: &str = "[equipment_studio]";
const STUDIO: &str = "art-src/equipment-studio";
const LAB_STAGE: &str = "art-src/modular-character-engine/stage";
const APPROVED: &str = "art-src/n3-8-production-run/stage";
const CAPTURES: &str = "target/equipment_studio";
const DARK: Color = Color::srgb(0.16, 0.15, 0.19);
/// Main view: centre x, sole line y (screen).
const MAIN: (f32, f32) = (190.0, -250.0);
/// Native previews (2x and 1x): centre x, sole y.
const PREVIEW_2X: (f32, f32) = (520.0, -250.0);
const PREVIEW_1X: (f32, f32) = (560.0, 120.0);
const SLOT_Z: f32 = 0.01;
/// Item sprites per view (equipped designs drawn at once).
const MAX_ITEMS: usize = 8;
const ZOOM_RANGE: (u32, u32) = (2, 8);
/// Window half size (the camera sits at the origin).
const HALF: (f32, f32) = (640.0, 360.0);

static VERDICT: OnceLock<bool> = OnceLock::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Part,
    Vertex,
    Pivot,
    Mount,
    Colour,
}

impl Mode {
    fn next(self) -> Self {
        match self {
            Self::Part => Self::Vertex,
            Self::Vertex => Self::Pivot,
            Self::Pivot => Self::Mount,
            Self::Mount => Self::Colour,
            Self::Colour => Self::Part,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Part => "PART (arrows move, Q/E rotate, -/= scale)",
            Self::Vertex => "VERTEX (V/B select, arrows move, Y insert, Backspace delete)",
            Self::Pivot => "PIVOT (arrows move the pivot; geometry stays)",
            Self::Mount => "MOUNT / SOCKET (arrows, Q/E, -/= move the whole item on its mount)",
            Self::Colour => {
                "COLOUR (K material, Left/Right ramp slot, R G B / Shift, Up/Down brightness)"
            }
        }
    }
}

/// Which view an entity belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum View {
    Main,
    X2,
    X1,
}

impl View {
    const ALL: [Self; 3] = [Self::Main, Self::X2, Self::X1];
}

#[derive(Component)]
struct ViewRoot(View);

#[derive(Component)]
struct BodySprite(BodyLayer);

#[derive(Component)]
struct ItemSprite(usize);

#[derive(Component)]
struct Hud;

/// One loaded design.
struct Entry {
    path: PathBuf,
    design: EquipmentDesign,
    rev: u64,
    saved_rev: u64,
    handle: Handle<Image>,
    painted: Option<(u64, String, usize)>,
    depth: DepthSlot,
}

/// A scripted step: chords tapped one per frame, then a capture.
struct Step {
    name: &'static str,
    chords: Vec<Vec<KeyCode>>,
    wait: u32,
    capture: bool,
    check: fn(&Studio) -> bool,
}

#[derive(Resource)]
struct Studio {
    root: PathBuf,
    rig: RigLab,
    rest: LocalPose,
    strips: HashMap<String, Handle<Image>>,
    body: HashMap<(String, usize, BodyLayer), Handle<Image>>,
    body_ready: bool,
    entries: Vec<Entry>,
    equipped: BTreeMap<RigSlot, usize>,
    selected: usize,
    part: usize,
    vertex: usize,
    mode: Mode,
    ramp_slot: usize,
    image_handles: HashMap<String, Handle<Image>>,
    decoded: HashMap<String, RgbaImage>,
    imports: Vec<String>,
    import_cursor: usize,
    clip: String,
    elapsed: f32,
    frame: usize,
    flip: bool,
    playing: bool,
    zoom: u32,
    isolated: bool,
    unequipped: bool,
    skeleton: bool,
    status: String,
    dragging: bool,
    tour: Option<Vec<Step>>,
    step: usize,
    wait: u32,
    warm: u32,
    out: PathBuf,
    failures: Vec<String>,
}

impl Studio {
    fn entry(&self) -> Option<&Entry> {
        self.entries.get(self.selected)
    }

    fn design(&self) -> Option<&EquipmentDesign> {
        self.entry().map(|e| &e.design)
    }

    /// Mutable design (marks it edited).
    fn edit(&mut self) -> Option<&mut EquipmentDesign> {
        let e = self.entries.get_mut(self.selected)?;
        e.rev += 1;
        Some(&mut e.design)
    }

    fn part(&self) -> Option<&Part> {
        self.design()?.parts.get(self.part)
    }

    fn clip(&self) -> Option<&RigClip> {
        self.rig.clips.get(&self.clip)
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

    /// The space map of the selected design on the current frame.
    fn map(&self) -> Option<SpaceMap> {
        let d = self.design()?;
        pose_map(&self.rig, &self.rest, d, &self.clip, self.frame).map(|(m, _, _)| m)
    }

    /// One texel in the design space of the selected item.
    fn texel(&self) -> (f32, f32) {
        self.map().map_or((1.0, 1.0), |m| m.texel())
    }

    /// Part-local point -> screen (main view).
    fn local_to_screen(&self, part: &Part, l: (f32, f32)) -> Option<Vec2> {
        let d = self.design()?;
        let s = d.mount.apply(part.transform.apply(l));
        Some(self.rig_to_screen(self.map()?.to_rig(s)))
    }

    fn rig_to_screen(&self, p: (f32, f32)) -> Vec2 {
        let sign = if self.flip { -1.0 } else { 1.0 };
        let z = self.zoom as f32;
        Vec2::new(MAIN.0 + p.0 * sign * z, MAIN.1 + p.1 * z)
    }

    fn screen_to_rig(&self, s: Vec2) -> (f32, f32) {
        let sign = if self.flip { -1.0 } else { 1.0 };
        let z = self.zoom as f32;
        ((s.x - MAIN.0) / z * sign, (s.y - MAIN.1) / z)
    }

    /// Screen -> part-local coordinates of the selected part.
    fn screen_to_local(&self, s: Vec2) -> Option<(f32, f32)> {
        let d = self.design()?;
        let part = d.parts.get(self.part)?;
        let space = self.map()?.to_space(self.screen_to_rig(s))?;
        part.transform.invert(d.mount.invert(space)?)
    }

    fn load_image(&mut self, path: &str, server: &AssetServer) {
        if !self.image_handles.contains_key(path) {
            self.image_handles
                .insert(path.to_owned(), server.load(format!("studio://{path}")));
        }
    }

    fn polygon_mut(&mut self) -> Option<&mut Vec<(f32, f32)>> {
        let part = self.part;
        match &mut self.edit()?.parts.get_mut(part)?.source {
            PartSource::Polygon { points, .. } => Some(points),
            PartSource::Image { .. } => None,
        }
    }
}

fn steel() -> PaintMaterial {
    PaintMaterial {
        id: "steel".into(),
        ramp: [
            [22, 22, 30],
            [52, 56, 70],
            [84, 90, 108],
            [122, 130, 148],
            [168, 176, 192],
            [220, 226, 236],
        ],
        finish: Finish::Metal,
    }
}

/// A new polygon part, a small quad near the middle of the design space.
fn new_part(d: &EquipmentDesign, id: &str) -> Part {
    let points = match d.space {
        DesignSpace::Torso => vec![(0.3, 0.3), (0.7, 0.3), (0.7, 0.5), (0.3, 0.5)],
        _ => vec![(-3.0, -3.0), (3.0, -3.0), (3.0, 3.0), (-3.0, 3.0)],
    };
    Part {
        id: d.free_id(id),
        source: PartSource::Polygon { points, smooth: 0 },
        transform: PartTransform::default(),
        material: d.materials.first().map_or("steel".into(), |m| m.id.clone()),
        layer: d.parts.iter().map(|p| p.layer).max().unwrap_or(0) + 1,
        outline: Outline::Selective,
        bevel: 1.0,
        regions: Vec::new(),
        ornaments: Vec::new(),
        visible: true,
        visibility: PartVisibility::default(),
    }
}

fn default_depth(slot: RigSlot) -> DepthSlot {
    match slot {
        RigSlot::MainHand => DepthSlot::MainHandGear,
        RigSlot::OffHand => DepthSlot::FarHandGear,
        RigSlot::Shoulder => DepthSlot::ShoulderGear,
        RigSlot::Chest => DepthSlot::ChestGear,
        RigSlot::Bracers => DepthSlot::BracerGear,
        RigSlot::Cape => DepthSlot::BackGear,
        RigSlot::Helmet => DepthSlot::HelmetGear,
        RigSlot::Belt => DepthSlot::BeltGear,
        RigSlot::Legs => DepthSlot::LegGear,
        RigSlot::Boots => DepthSlot::BootGear,
    }
}

fn next_slot(slot: RigSlot) -> RigSlot {
    let all = RigSlot::ALL;
    let i = all.iter().position(|s| *s == slot).unwrap_or(0);
    all[(i + 1) % all.len()]
}

fn load_designs(root: &Path, server: &AssetServer, images: &mut Assets<Image>) -> Vec<Entry> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(root.join("items"))
        .map(|d| d.filter_map(|e| e.ok().map(|e| e.path())).collect())
        .unwrap_or_default();
    paths.retain(|p| p.extension().is_some_and(|e| e == "ron"));
    paths.sort();
    let _ = server;
    paths
        .into_iter()
        .filter_map(|path| {
            let text = std::fs::read_to_string(&path).ok()?;
            match EquipmentDesign::from_ron(&text) {
                Ok(design) => Some(Entry {
                    depth: design.depth,
                    path,
                    design,
                    rev: 1,
                    saved_rev: 1,
                    handle: images.add(pixel_image(PixelArt::empty())),
                    painted: None,
                }),
                Err(e) => {
                    eprintln!("{TAG} {}: {e}", path.display());
                    None
                }
            }
        })
        .collect()
}

fn setup(
    mut commands: Commands,
    mut studio: ResMut<Studio>,
    server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    commands.spawn(Camera2d);
    let clips: Vec<(String, String)> = studio
        .rig
        .clips
        .iter()
        .map(|(k, c)| (k.clone(), c.texture.clone()))
        .collect();
    for (key, texture) in clips {
        studio.strips.insert(key, server.load(texture));
    }
    let root = studio.root.clone();
    studio.entries = load_designs(&root, &server, &mut images);
    let paths: Vec<String> = studio
        .entries
        .iter()
        .flat_map(|e| e.design.parts.iter())
        .filter_map(|p| match &p.source {
            PartSource::Image { path, .. } => Some(path.clone()),
            PartSource::Polygon { .. } => None,
        })
        .collect();
    for p in paths.iter().chain(studio.imports.clone().iter()) {
        studio.load_image(p, &server);
    }
    // Equip the first design of each slot.
    let mut equipped = BTreeMap::new();
    for (i, e) in studio.entries.iter().enumerate() {
        equipped.entry(e.design.slot).or_insert(i);
    }
    studio.equipped = equipped;
    for view in View::ALL {
        let (pos, scale) = match view {
            View::Main => (MAIN, studio.zoom as f32),
            View::X2 => (PREVIEW_2X, 2.0),
            View::X1 => (PREVIEW_1X, 1.0),
        };
        let root = commands
            .spawn((
                ViewRoot(view),
                Transform::from_xyz(pos.0, pos.1, 0.0).with_scale(Vec3::splat(scale)),
                Visibility::default(),
            ))
            .id();
        for layer in BodyLayer::ALL {
            commands.entity(root).with_child((
                BodySprite(layer),
                Sprite::default(),
                Anchor::default(),
                Transform::default(),
                Visibility::Hidden,
            ));
        }
        for i in 0..MAX_ITEMS {
            commands.entity(root).with_child((
                ItemSprite(i),
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
            font_size: FontSize::Px(12.0),
            ..default()
        },
        TextColor(Color::srgb(0.95, 0.85, 0.55)),
        Node {
            position_type: PositionType::Absolute,
            left: px(8),
            top: px(6),
            width: px(470),
            ..default()
        },
    ));
}

/// Builds the body layers (shared cut of the Modular Character Engine) and
/// decodes imported PNGs once they are loaded.
fn load_assets(mut studio: ResMut<Studio>, mut images: ResMut<Assets<Image>>) {
    // Decode PNGs for the painter.
    let pending: Vec<(String, Handle<Image>)> = studio
        .image_handles
        .iter()
        .filter(|(p, _)| !studio.decoded.contains_key(*p))
        .map(|(p, h)| (p.clone(), h.clone()))
        .collect();
    for (path, h) in pending {
        if let Some(img) = images.get(&h)
            && let Some(data) = img.data.as_ref()
        {
            let pixels = data.as_chunks::<4>().0.to_vec();
            studio.decoded.insert(
                path,
                RgbaImage {
                    width: img.width() as usize,
                    height: img.height() as usize,
                    pixels,
                },
            );
            for e in &mut studio.entries {
                e.painted = None;
            }
        }
    }
    if studio.body_ready || !studio.strips.values().all(|h| images.contains(h)) {
        return;
    }
    let stage = project_root().join(LAB_STAGE);
    let clips: Vec<(String, RigClip)> = studio
        .rig
        .clips
        .iter()
        .map(|(k, c)| (k.clone(), c.clone()))
        .collect();
    let mut failures = Vec::new();
    for (key, clip) in clips {
        let Some(img) = studio.strips.get(&key).and_then(|h| images.get(h)) else {
            continue;
        };
        let (w, data) = (img.width() as usize, img.data.clone().unwrap_or_default());
        match clip.cut_layers(&data, w, |p| std::fs::read_to_string(stage.join(p)).ok()) {
            Ok(cut) => {
                for set in cut {
                    if set.differing != 0 {
                        failures.push(format!(
                            "{key} #{}: {} texels differ",
                            set.source, set.differing
                        ));
                    }
                    println!(
                        "{TAG} body {key} #{}: {} texels differ",
                        set.source, set.differing
                    );
                    for (layer, bytes) in set.layers {
                        let art = PixelArt {
                            width: clip.cell.0,
                            height: clip.cell.1,
                            rgba: bytes,
                            grip: (0.0, 0.0),
                            density: 1.0,
                        };
                        let h = images.add(pixel_image(art));
                        studio.body.insert((key.clone(), set.source, layer), h);
                    }
                }
            }
            Err(e) => failures.push(format!("{key}: {e}")),
        }
    }
    studio.failures.extend(failures);
    studio.body_ready = true;
}

fn shift(input: &ButtonInput<KeyCode>) -> bool {
    input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight)
}

fn ctrl(input: &ButtonInput<KeyCode>) -> bool {
    input.pressed(KeyCode::ControlLeft) || input.pressed(KeyCode::ControlRight)
}

/// Keyboard: every edit of the studio.
fn keys(
    input: Res<ButtonInput<KeyCode>>,
    mut s: ResMut<Studio>,
    server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    let pressed = |k| input.just_pressed(k);
    let big = if shift(&input) { 4.0 } else { 1.0 };
    let frames = s.clip().map_or(1, |c| c.frames.len());
    // Animation and view.
    if pressed(KeyCode::Space) {
        s.playing = !s.playing;
    }
    if pressed(KeyCode::Period) && !s.playing {
        let f = (s.frame + 1) % frames;
        s.elapsed = s.frame_start(f) + 1e-4;
    }
    if pressed(KeyCode::Comma) && !s.playing {
        let f = (s.frame + frames - 1) % frames;
        s.elapsed = s.frame_start(f) + 1e-4;
    }
    if pressed(KeyCode::KeyT) {
        let next = if s.clip == "run" { "idle" } else { "run" };
        s.set_clip(next);
    }
    if pressed(KeyCode::KeyF) {
        s.flip = !s.flip;
    }
    if pressed(KeyCode::KeyZ) {
        s.zoom = (s.zoom - 1).max(ZOOM_RANGE.0);
    }
    if pressed(KeyCode::KeyX) {
        s.zoom = (s.zoom + 1).min(ZOOM_RANGE.1);
    }
    if pressed(KeyCode::KeyO) {
        s.isolated = !s.isolated;
    }
    if pressed(KeyCode::KeyU) {
        s.unequipped = !s.unequipped;
    }
    if pressed(KeyCode::KeyG) {
        s.skeleton = !s.skeleton;
    }
    // Designs.
    let n = s.entries.len().max(1);
    if pressed(KeyCode::PageDown) {
        s.selected = (s.selected + 1) % n;
        s.part = 0;
        s.vertex = 0;
    }
    if pressed(KeyCode::PageUp) {
        s.selected = (s.selected + n - 1) % n;
        s.part = 0;
        s.vertex = 0;
    }
    if pressed(KeyCode::Enter)
        && let Some(slot) = s.design().map(|d| d.slot)
    {
        let sel = s.selected;
        if s.equipped.get(&slot) == Some(&sel) {
            s.equipped.remove(&slot);
            s.status = format!("unequipped {slot:?}");
        } else {
            s.equipped.insert(slot, sel);
            s.status = format!("equipped {slot:?}");
        }
    }
    if ctrl(&input) && pressed(KeyCode::KeyN) {
        let mut k = 1;
        while s
            .entries
            .iter()
            .any(|e| e.design.id == format!("studio_item_{k}"))
        {
            k += 1;
        }
        let id = format!("studio_item_{k}");
        let mut d = EquipmentDesign {
            version: DESIGN_VERSION,
            id: id.clone(),
            name: format!("New item {k} (made in the studio)"),
            slot: RigSlot::Chest,
            space: DesignSpace::Torso,
            depth: DepthSlot::ChestGear,
            mount: PartTransform::default(),
            light: (0.45, 0.8),
            aspect_range: (0.67, 0.78),
            materials: vec![steel()],
            parts: Vec::new(),
            provisional: true,
            source: "created in the Equipment Studio".into(),
        };
        let part = new_part(&d, "plate");
        d.parts.push(part);
        let path = s.root.join("items").join(format!("{id}.ron"));
        let handle = images.add(pixel_image(PixelArt::empty()));
        s.entries.push(Entry {
            path,
            depth: d.depth,
            design: d,
            rev: 1,
            saved_rev: 0,
            handle,
            painted: None,
        });
        s.selected = s.entries.len() - 1;
        s.part = 0;
        let sel = s.selected;
        s.equipped.insert(RigSlot::Chest, sel);
        s.status = format!("new design {id} (not saved)");
    }
    if ctrl(&input) && pressed(KeyCode::KeyS) {
        let msg = match s.entry() {
            Some(e) => match e.design.to_ron() {
                Ok(text) => match std::fs::write(&e.path, text) {
                    Ok(()) => format!("saved {}", e.path.display()),
                    Err(err) => format!("save failed: {err}"),
                },
                Err(err) => format!("save failed: {err}"),
            },
            None => "nothing to save".into(),
        };
        let sel = s.selected;
        if let Some(e) = s.entries.get_mut(sel) {
            e.saved_rev = e.rev;
        }
        println!("{TAG} {msg}");
        s.status = msg;
    }
    if ctrl(&input) && pressed(KeyCode::KeyR) {
        let root = s.root.clone();
        let fresh = load_designs(&root, &server, &mut images);
        s.status = format!("reloaded {} designs from disk", fresh.len());
        s.entries = fresh;
        s.selected = s.selected.min(s.entries.len().saturating_sub(1));
        s.part = 0;
        let map: BTreeMap<RigSlot, usize> = s.equipped.clone();
        s.equipped = map
            .into_iter()
            .filter(|(_, i)| *i < s.entries.len())
            .collect();
    }
    if ctrl(&input)
        && pressed(KeyCode::KeyP)
        && let Some(d) = s.edit()
    {
        d.slot = next_slot(d.slot);
        d.depth = default_depth(d.slot);
        let (slot, sel) = (d.slot, s.selected);
        s.equipped.retain(|_, i| *i != sel);
        s.equipped.insert(slot, sel);
        s.status = format!("slot {slot:?}");
    }
    if ctrl(&input) {
        if pressed(KeyCode::KeyD)
            && let Some(mut p) = s.part().cloned()
            && let Some(d) = s.edit()
        {
            p.id = d.free_id(&p.id);
            p.layer += 1;
            d.parts.push(p);
            s.part = s.design().map_or(0, |d| d.parts.len() - 1);
        }
        return;
    }
    // Parts.
    let parts = s.design().map_or(0, |d| d.parts.len()).max(1);
    if pressed(KeyCode::Tab) {
        s.part = if shift(&input) {
            (s.part + parts - 1) % parts
        } else {
            (s.part + 1) % parts
        };
        s.vertex = 0;
    }
    if pressed(KeyCode::KeyN)
        && let Some(d) = s.edit()
    {
        let p = new_part(d, "part");
        d.parts.push(p);
        s.part = s.design().map_or(0, |d| d.parts.len() - 1);
        s.status = "added a polygon part".into();
    }
    if pressed(KeyCode::KeyI) && !s.imports.is_empty() {
        let path = s.imports[s.import_cursor % s.imports.len()].clone();
        s.import_cursor += 1;
        s.load_image(&path, &server);
        if let Some(d) = s.edit() {
            let mut p = new_part(d, "image");
            p.source = PartSource::Image {
                path: path.clone(),
                pivot: (2.0, 2.0),
            };
            p.outline = Outline::None;
            p.transform.position = match d.space {
                DesignSpace::Torso => (0.6, 0.6),
                _ => (0.0, 0.0),
            };
            d.parts.push(p);
        }
        s.part = s.design().map_or(0, |d| d.parts.len() - 1);
        s.status = format!("imported {path}");
    }
    if pressed(KeyCode::Delete) {
        let part = s.part;
        if let Some(d) = s.edit()
            && d.parts.len() > 1
        {
            d.parts.remove(part);
        }
        s.part = s.part.min(s.design().map_or(0, |d| d.parts.len() - 1));
    }
    if pressed(KeyCode::KeyH) {
        let part = s.part;
        if let Some(p) = s.edit().and_then(|d| d.parts.get_mut(part)) {
            p.visible = !p.visible;
        }
    }
    for (key, delta) in [(KeyCode::BracketLeft, -1), (KeyCode::BracketRight, 1)] {
        if pressed(key) {
            let part = s.part;
            if let Some(p) = s.edit().and_then(|d| d.parts.get_mut(part)) {
                p.layer += delta;
            }
        }
    }
    if pressed(KeyCode::KeyM) {
        s.mode = s.mode.next();
    }
    let arrow = [
        (KeyCode::ArrowLeft, (-1.0, 0.0)),
        (KeyCode::ArrowRight, (1.0, 0.0)),
        (KeyCode::ArrowUp, (0.0, 1.0)),
        (KeyCode::ArrowDown, (0.0, -1.0)),
    ]
    .into_iter()
    .find(|(k, _)| pressed(*k))
    .map(|(_, d)| d);
    let rotate = if pressed(KeyCode::KeyQ) {
        5.0
    } else if pressed(KeyCode::KeyE) {
        -5.0
    } else {
        0.0
    };
    let scale = if pressed(KeyCode::Equal) {
        1.05
    } else if pressed(KeyCode::Minus) {
        1.0 / 1.05
    } else {
        1.0
    };
    // One texel in design units (the torso cage is in u, v; y down the
    // cage is +v, so screen up is -v).
    let (tu, tv) = s.texel();
    let torso = s.design().is_some_and(|d| d.space == DesignSpace::Torso);
    let step = |d: (f32, f32)| {
        if torso {
            (d.0 * tu * big, -d.1 * tv * big)
        } else {
            (d.0 * big, d.1 * big)
        }
    };
    let part = s.part;
    // Only touch the design (its revision) when a key really edits it.
    let transform_key = arrow.is_some() || rotate != 0.0 || scale != 1.0;
    match s.mode {
        Mode::Part if transform_key => {
            if let Some(p) = s.edit().and_then(|d| d.parts.get_mut(part)) {
                if let Some(d) = arrow {
                    let (dx, dy) = step(d);
                    p.transform.position.0 += dx;
                    p.transform.position.1 += dy;
                }
                p.transform.rotation_deg += rotate;
                p.transform.scale.0 *= scale;
                p.transform.scale.1 *= scale;
            }
        }
        Mode::Vertex => {
            let count = match s.part().map(|p| &p.source) {
                Some(PartSource::Polygon { points, .. }) => points.len(),
                _ => 0,
            };
            if count > 0 {
                if pressed(KeyCode::KeyB) {
                    s.vertex = (s.vertex + 1) % count;
                }
                if pressed(KeyCode::KeyV) {
                    s.vertex = (s.vertex + count - 1) % count;
                }
                let v = s.vertex.min(count - 1);
                if let Some(d) = arrow {
                    let (dx, dy) = step(d);
                    // Arrows move in the design space; undo the part's
                    // rotation and scale so the vertex follows the arrow.
                    let t = s.part().map(|p| p.transform).unwrap_or_default();
                    let (sn, cs) = (-t.rotation_deg).to_radians().sin_cos();
                    let (lx, ly) = (
                        (cs * dx - sn * dy) / t.scale.0,
                        (sn * dx + cs * dy) / t.scale.1,
                    );
                    if let Some(points) = s.polygon_mut() {
                        points[v].0 += lx;
                        points[v].1 += ly;
                    }
                }
                if pressed(KeyCode::KeyY)
                    && let Some(points) = s.polygon_mut()
                {
                    let (a, b) = (points[v], points[(v + 1) % points.len()]);
                    points.insert(v + 1, ((a.0 + b.0) * 0.5, (a.1 + b.1) * 0.5));
                    s.vertex = v + 1;
                }
                if pressed(KeyCode::Backspace)
                    && let Some(points) = s.polygon_mut()
                    && points.len() > 3
                {
                    points.remove(v);
                    s.vertex = v.saturating_sub(1);
                }
            }
        }
        Mode::Pivot => {
            if let Some(d) = arrow {
                let (dx, dy) = step(d);
                if let Some(p) = s.edit().and_then(|d| d.parts.get_mut(part)) {
                    // Move the pivot without moving the geometry.
                    let t = p.transform;
                    let (sn, cs) = (-t.rotation_deg).to_radians().sin_cos();
                    let local = (
                        t.pivot.0 + (cs * dx - sn * dy) / t.scale.0,
                        t.pivot.1 + (sn * dx + cs * dy) / t.scale.1,
                    );
                    p.transform.position = t.apply(local);
                    p.transform.pivot = local;
                }
            }
        }
        Mode::Part => {}
        Mode::Mount if transform_key => {
            if let Some(d) = s.edit() {
                if let Some(a) = arrow {
                    let (dx, dy) = if d.space == DesignSpace::Torso {
                        (a.0 * tu * big, -a.1 * tv * big)
                    } else {
                        (a.0 * big, a.1 * big)
                    };
                    d.mount.position.0 += dx;
                    d.mount.position.1 += dy;
                }
                d.mount.rotation_deg += rotate;
                d.mount.scale.0 *= scale;
                d.mount.scale.1 *= scale;
            }
        }
        Mode::Mount => {}
        Mode::Colour => {
            if pressed(KeyCode::ArrowLeft) {
                s.ramp_slot = (s.ramp_slot + 5) % 6;
            }
            if pressed(KeyCode::ArrowRight) {
                s.ramp_slot = (s.ramp_slot + 1) % 6;
            }
            let sign: i16 = if shift(&input) { -8 } else { 8 };
            let mut delta = [0i16; 3];
            if pressed(KeyCode::KeyR) {
                delta[0] = sign;
            }
            if pressed(KeyCode::KeyG) {
                delta[1] = sign;
            }
            if pressed(KeyCode::KeyB) {
                delta[2] = sign;
            }
            if pressed(KeyCode::ArrowUp) {
                delta = [8; 3];
            }
            if pressed(KeyCode::ArrowDown) {
                delta = [-8; 3];
            }
            if pressed(KeyCode::KeyK) {
                let ids: Vec<String> = s
                    .design()
                    .map(|d| d.materials.iter().map(|m| m.id.clone()).collect())
                    .unwrap_or_default();
                if let Some(p) = s.edit().and_then(|d| d.parts.get_mut(part))
                    && !ids.is_empty()
                {
                    let i = ids.iter().position(|m| *m == p.material).unwrap_or(0);
                    p.material = ids[(i + 1) % ids.len()].clone();
                }
            }
            if delta != [0; 3] {
                let slot = s.ramp_slot;
                let material = s.part().map(|p| p.material.clone());
                if let Some(m) = material
                    && let Some(d) = s.edit()
                    && let Some(mat) = d.materials.iter_mut().find(|x| x.id == m)
                {
                    for (c, dv) in mat.ramp[slot].iter_mut().zip(delta) {
                        *c = (i16::from(*c) + dv).clamp(0, 255) as u8;
                    }
                }
            }
        }
    }
}

/// Mouse: drag a vertex, the part or place the pivot; wheel zooms.
fn mouse(
    buttons: Res<ButtonInput<MouseButton>>,
    scroll: Res<AccumulatedMouseScroll>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut s: ResMut<Studio>,
) {
    if scroll.delta.y > 0.0 {
        s.zoom = (s.zoom + 1).min(ZOOM_RANGE.1);
    } else if scroll.delta.y < 0.0 {
        s.zoom = (s.zoom - 1).max(ZOOM_RANGE.0);
    }
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let screen = Vec2::new(cursor.x - HALF.0, HALF.1 - cursor.y);
    if buttons.just_released(MouseButton::Left) {
        s.dragging = false;
    }
    if buttons.just_pressed(MouseButton::Left) && s.mode == Mode::Vertex {
        // Pick the nearest vertex of the selected part.
        if let Some(part) = s.part().cloned()
            && let PartSource::Polygon { points, .. } = &part.source
        {
            let best = points
                .iter()
                .enumerate()
                .filter_map(|(i, p)| Some((i, s.local_to_screen(&part, *p)?.distance(screen))))
                .min_by(|a, b| a.1.total_cmp(&b.1));
            if let Some((i, d)) = best
                && d < 24.0
            {
                s.vertex = i;
                s.dragging = true;
            }
        }
    }
    if buttons.just_pressed(MouseButton::Left) && matches!(s.mode, Mode::Part | Mode::Pivot) {
        s.dragging = true;
    }
    if !s.dragging || !buttons.pressed(MouseButton::Left) {
        return;
    }
    let Some(local) = s.screen_to_local(screen) else {
        return;
    };
    let part = s.part;
    match s.mode {
        Mode::Vertex => {
            let v = s.vertex;
            if let Some(points) = s.polygon_mut()
                && v < points.len()
            {
                points[v] = local;
            }
        }
        Mode::Part => {
            let t = s.part().map(|p| p.transform).unwrap_or_default();
            if let Some(p) = s.edit().and_then(|d| d.parts.get_mut(part)) {
                p.transform.position = t.apply(local);
                p.transform.pivot = local;
            }
        }
        Mode::Pivot => {
            if let Some(p) = s.edit().and_then(|d| d.parts.get_mut(part)) {
                let t = p.transform;
                p.transform.position = t.apply(local);
                p.transform.pivot = local;
            }
        }
        _ => {}
    }
}

/// The one clock (the frame always comes from `elapsed`).
fn advance(time: Res<Time>, mut s: ResMut<Studio>) {
    if s.playing {
        s.elapsed += clamp_delta(time.delta_secs());
    }
    let durations = s.clip().map(RigClip::durations).unwrap_or_default();
    s.frame = loop_frame(&durations, s.elapsed);
}

/// Repaints every equipped design whose design, clip or frame changed.
fn repaint(mut s: ResMut<Studio>, mut images: ResMut<Assets<Image>>) {
    if !s.body_ready {
        return;
    }
    let (clip, frame) = (s.clip.clone(), s.frame);
    let Some(c) = s.clip().cloned() else {
        return;
    };
    let mut shown: Vec<usize> = s.equipped.values().copied().collect();
    shown.push(s.selected);
    for i in shown {
        let Some(e) = s.entries.get(i) else {
            continue;
        };
        let key = (e.rev, clip.clone(), frame);
        if e.painted.as_ref() == Some(&key) {
            continue;
        }
        let Some((map, depth, turn)) = pose_map(&s.rig, &s.rest, &e.design, &clip, frame) else {
            continue;
        };
        let decoded = &s.decoded;
        let lookup = |p: &str| decoded.get(p);
        let job = PaintJob {
            design: &e.design,
            map,
            cell: c.geometry(),
            clip: &clip,
            turn,
            images: &lookup,
        };
        let painted = paint(&job);
        let art = PixelArt {
            width: c.cell.0,
            height: c.cell.1,
            rgba: painted.rgba.iter().flatten().copied().collect(),
            grip: (0.0, 0.0),
            density: 1.0,
        };
        let handle = e.handle.clone();
        let _ = images.insert(&handle, pixel_image(art));
        if let Some(e) = s.entries.get_mut(i) {
            e.painted = Some(key);
            e.depth = depth;
        }
    }
}

fn sync_views(
    s: Res<Studio>,
    mut roots: Query<(&ViewRoot, &mut Transform)>,
    mut bodies: Query<
        (
            &ChildOf,
            &BodySprite,
            &mut Sprite,
            &mut Anchor,
            &mut Transform,
            &mut Visibility,
        ),
        Without<ViewRoot>,
    >,
    mut items: Query<
        (
            &ChildOf,
            &ItemSprite,
            &mut Sprite,
            &mut Anchor,
            &mut Transform,
            &mut Visibility,
        ),
        (Without<ViewRoot>, Without<BodySprite>),
    >,
    views: Query<&ViewRoot>,
) {
    let Some(c) = s.clip() else {
        return;
    };
    let Some(f) = c.frames.get(s.frame) else {
        return;
    };
    for (root, mut t) in &mut roots {
        if root.0 == View::Main {
            t.scale = Vec3::splat(s.zoom as f32);
        }
    }
    let size = Vec2::new(c.cell.0 as f32, c.cell.1 as f32);
    let (ax, ay) = anchor_for(c.pivot, c.cell, s.flip);
    for (parent, layer, mut sprite, mut anchor, mut t, mut vis) in &mut bodies {
        let isolated = s.isolated && views.get(parent.parent()).is_ok_and(|v| v.0 == View::Main);
        let Some(h) = s.body.get(&(s.clip.clone(), f.source, layer.0)) else {
            vis.set_if_neq(Visibility::Hidden);
            continue;
        };
        if isolated {
            vis.set_if_neq(Visibility::Hidden);
            continue;
        }
        sprite.image = h.clone();
        sprite.custom_size = Some(size);
        sprite.flip_x = s.flip;
        anchor.set_if_neq(Anchor(Vec2::new(ax, ay)));
        let drop = if layer.0.is_upper_body() {
            f.drop as f32
        } else {
            0.0
        };
        t.translation = Vec3::new(0.0, -drop, f32::from(layer.0.depth().order()) * SLOT_Z);
        vis.set_if_neq(Visibility::Inherited);
    }
    let mut shown: Vec<usize> = if s.unequipped {
        Vec::new()
    } else {
        s.equipped.values().copied().collect()
    };
    if s.isolated && !shown.contains(&s.selected) {
        shown.push(s.selected);
    }
    for (_, item, mut sprite, mut anchor, mut t, mut vis) in &mut items {
        let Some(e) = shown.get(item.0).and_then(|i| s.entries.get(*i)) else {
            vis.set_if_neq(Visibility::Hidden);
            continue;
        };
        if e.painted.is_none() {
            vis.set_if_neq(Visibility::Hidden);
            continue;
        }
        sprite.image = e.handle.clone();
        sprite.custom_size = Some(size);
        sprite.flip_x = s.flip;
        anchor.set_if_neq(Anchor(Vec2::new(ax, ay)));
        t.translation = Vec3::new(0.0, 0.0, f32::from(e.depth.order()) * SLOT_Z + 0.001);
        vis.set_if_neq(Visibility::Inherited);
    }
}

/// The selected part's geometry, vertices, pivot and the mount origin.
fn overlays(s: Res<Studio>, mut gizmos: Gizmos) {
    let cyan = Color::srgb(0.0, 1.0, 1.0);
    let yellow = Color::srgb(1.0, 0.9, 0.2);
    let magenta = Color::srgb(1.0, 0.2, 1.0);
    if s.skeleton
        && let Some(j) = s.clip().and_then(|c| c.frame_joints(s.frame))
    {
        let pose = forward(&fit(&j));
        for bone in Bone::ALL {
            let (a, b) = pose.segment(bone);
            gizmos.line_2d(
                s.rig_to_screen(a),
                s.rig_to_screen(b),
                Color::srgba(1.0, 1.0, 1.0, 0.6),
            );
        }
        let cage = TorsoCage::from_joints(&j);
        let q = [
            cage.chest_back,
            cage.chest_front,
            cage.waist_front,
            cage.waist_back,
        ];
        for i in 0..4 {
            gizmos.line_2d(
                s.rig_to_screen(q[i]),
                s.rig_to_screen(q[(i + 1) % 4]),
                Color::srgb(0.5, 1.0, 0.3),
            );
        }
    }
    let Some(part) = s.part().cloned() else {
        return;
    };
    match &part.source {
        PartSource::Polygon { points, smooth } => {
            let curve = chaikin(points, *smooth);
            let pts: Vec<Vec2> = curve
                .iter()
                .filter_map(|p| s.local_to_screen(&part, *p))
                .collect();
            for i in 0..pts.len() {
                gizmos.line_2d(pts[i], pts[(i + 1) % pts.len()], cyan);
            }
            if s.mode == Mode::Vertex {
                for (i, p) in points.iter().enumerate() {
                    if let Some(v) = s.local_to_screen(&part, *p) {
                        let r = if i == s.vertex { 6.0 } else { 3.0 };
                        gizmos.circle_2d(v, r, if i == s.vertex { yellow } else { cyan });
                    }
                }
            }
        }
        PartSource::Image { .. } => {
            if let Some(c) = s.local_to_screen(&part, part.transform.pivot) {
                gizmos.rect_2d(Isometry2d::from_translation(c), Vec2::splat(10.0), cyan);
            }
        }
    }
    if let Some(p) = s.local_to_screen(&part, part.transform.pivot) {
        gizmos.cross_2d(Isometry2d::from_translation(p), 9.0, magenta);
    }
    // The mount (socket) origin of the whole item.
    if let (Some(d), Some(map)) = (s.design(), s.map()) {
        let o = s.rig_to_screen(map.to_rig(d.mount.position));
        gizmos.circle_2d(o, 8.0, yellow);
        let dir = d.mount.apply((
            if d.space == DesignSpace::Torso {
                0.2
            } else {
                10.0
            },
            0.0,
        ));
        gizmos.arrow_2d(o, s.rig_to_screen(map.to_rig(dir)), yellow);
    }
    // Panel frames for the native previews.
    gizmos.line_2d(
        Vec2::new(-150.0, -250.0),
        Vec2::new(640.0, -250.0),
        Color::srgba(1.0, 1.0, 1.0, 0.12),
    );
}

fn hud(s: Res<Studio>, mut q: Query<&mut Text, With<Hud>>) {
    let Ok(mut text) = q.single_mut() else {
        return;
    };
    let mut out =
        String::from("DARK CASTLE EQUIPMENT STUDIO  (Engine 03 prototype, not the game)\n");
    out += &format!(
        "mode: {}\n\nDESIGNS  (PgUp/PgDn select, Enter equip, Ctrl+N new, Ctrl+S save, Ctrl+R reload)\n",
        s.mode.name()
    );
    for (i, e) in s.entries.iter().enumerate() {
        let eq = s.equipped.get(&e.design.slot) == Some(&i);
        out += &format!(
            " {} {:<18} {:<8} {:<6} {}{}\n",
            if i == s.selected { ">" } else { " " },
            e.design.id,
            format!("{:?}", e.design.slot),
            match e.design.space {
                DesignSpace::Torso => "torso",
                DesignSpace::Hand { .. } => "hand",
                DesignSpace::Bone { .. } => "bone",
            },
            if eq { "[equipped]" } else { "" },
            if e.rev != e.saved_rev {
                " *unsaved*"
            } else {
                ""
            },
        );
    }
    if let Some(d) = s.design() {
        out += &format!(
            "\nPARTS of {}  (Tab select, N add, I import PNG, Del remove, H hide, [ ] layer)\n",
            d.id
        );
        let mut idx: Vec<usize> = (0..d.parts.len()).collect();
        idx.sort_by_key(|i| -d.parts[*i].layer);
        for i in idx {
            let p = &d.parts[i];
            let kind = match &p.source {
                PartSource::Polygon { points, smooth } => format!(
                    "poly {}pt{}",
                    points.len(),
                    if *smooth > 0 { " curve" } else { "" }
                ),
                PartSource::Image { path, .. } => {
                    format!("png {}", path.rsplit('/').next().unwrap_or(path))
                }
            };
            out += &format!(
                " {} {:<14} L{:<2} {:<12} {:<16} {}\n",
                if i == s.part { ">" } else { " " },
                p.id,
                p.layer,
                p.material,
                kind,
                if p.visible { "" } else { "(hidden)" }
            );
        }
        if let Some(p) = d.parts.get(s.part) {
            let t = p.transform;
            out += &format!(
                "\nSELECTED {}: pos ({:.2}, {:.2}) rot {:.0} scale {:.2}x{:.2} pivot ({:.2}, {:.2})\n  bevel {:.1} outline {:?} regions {} ornaments {}",
                p.id,
                t.position.0,
                t.position.1,
                t.rotation_deg,
                t.scale.0,
                t.scale.1,
                t.pivot.0,
                t.pivot.1,
                p.bevel,
                p.outline,
                p.regions.len(),
                p.ornaments.len()
            );
            if let PartSource::Polygon { points, .. } = &p.source
                && let Some(v) = points.get(s.vertex)
            {
                out += &format!("  vertex {} ({:.2}, {:.2})", s.vertex, v.0, v.1);
            }
            if let Some(m) = d.material(&p.material) {
                let c = m.ramp[s.ramp_slot];
                out += &format!(
                    "\n  material {} ({:?}) ramp slot {} = ({}, {}, {})",
                    m.id,
                    m.finish,
                    ["ink", "deep", "shadow", "base", "light", "highlight"][s.ramp_slot],
                    c[0],
                    c[1],
                    c[2]
                );
            }
        }
        out += &format!(
            "\nMOUNT pos ({:.2}, {:.2}) rot {:.0} scale {:.2}",
            d.mount.position.0, d.mount.position.1, d.mount.rotation_deg, d.mount.scale.0
        );
    }
    out += &format!(
        "\n\nclip {} frame {}{}  facing {}  zoom {}x (previews 2x, 1x)  {}{}\nT idle/run  Space play  . , frame  F flip  Z X zoom  O isolated  U unequipped  G skeleton  M mode\nstatus: {}",
        s.clip,
        s.frame,
        if s.playing { "" } else { " [paused]" },
        if s.flip { "left" } else { "right" },
        s.zoom,
        if s.isolated { "[isolated] " } else { "" },
        if s.unequipped { "[unequipped]" } else { "" },
        s.status
    );
    text.0 = out;
}

// ------------------------------------------------------------------ tour

fn tap(k: KeyCode) -> Vec<KeyCode> {
    vec![k]
}

fn chord(a: KeyCode, b: KeyCode) -> Vec<KeyCode> {
    vec![a, b]
}

fn step(
    name: &'static str,
    chords: Vec<Vec<KeyCode>>,
    wait: u32,
    capture: bool,
    check: fn(&Studio) -> bool,
) -> Step {
    Step {
        name,
        chords,
        wait,
        capture,
        check,
    }
}

fn ok(_: &Studio) -> bool {
    true
}

fn selected_is(s: &Studio, id: &str) -> bool {
    s.design().is_some_and(|d| d.id == id)
}

fn part_is(s: &Studio, id: &str) -> bool {
    s.part().is_some_and(|p| p.id == id)
}

/// The evidence run, entirely through the studio's keys.
fn tour_steps() -> Vec<Step> {
    use KeyCode::*;
    let mut v = vec![
        // Start: the run, paused on frame 2, gothic cuirass selected.
        step("t00_start", vec![tap(KeyT), tap(Space)], 10, true, |s| {
            s.clip == "run" && !s.playing
        }),
    ];
    // Both breastplates in the six run poses (designs are sorted:
    // arming_sword, gothic_cuirass, leather_jerkin; gothic is equipped).
    v.push(step("t01_select_a", vec![tap(PageDown)], 6, false, |s| {
        selected_is(s, "gothic_cuirass")
    }));
    let names_a = [
        "pose_a_0", "pose_a_1", "pose_a_2", "pose_a_3", "pose_a_4", "pose_a_5",
    ];
    let names_b = [
        "pose_b_0", "pose_b_1", "pose_b_2", "pose_b_3", "pose_b_4", "pose_b_5",
    ];
    for (f, name) in names_a.into_iter().enumerate() {
        let chords = if f == 0 { vec![] } else { vec![tap(Period)] };
        v.push(step(name, chords, 6, true, ok));
    }
    v.push(step(
        "t01_select_b",
        vec![tap(Period), tap(PageDown), tap(Enter)],
        6,
        false,
        |s| selected_is(s, "leather_jerkin") && s.frame == 0,
    ));
    for (f, name) in names_b.into_iter().enumerate() {
        let chords = if f == 0 { vec![] } else { vec![tap(Period)] };
        v.push(step(name, chords, 6, true, ok));
    }
    // Idle with both swords etc.: show the idle with everything equipped.
    v.extend([
        step("idle_equipped", vec![tap(KeyT)], 8, true, |s| {
            s.clip == "idle"
        }),
        step("idle_unequipped", vec![tap(KeyU)], 6, true, |s| {
            s.unequipped
        }),
        step("idle_isolated", vec![tap(KeyU), tap(KeyO)], 6, true, |s| {
            s.isolated && !s.unequipped
        }),
        step("run_isolated", vec![tap(KeyT)], 6, true, |s| {
            s.clip == "run"
        }),
        step(
            "zoom_in",
            vec![tap(KeyO), tap(KeyX), tap(KeyX)],
            6,
            true,
            |s| s.zoom == 6,
        ),
        step("zoom_back", vec![tap(KeyZ), tap(KeyZ)], 4, false, |s| {
            s.zoom == 4
        }),
        step("flip_left", vec![tap(KeyF)], 6, true, |s| s.flip),
        step("flip_right", vec![tap(KeyF)], 4, false, |s| !s.flip),
    ]);
    // GIF 1: visual change of a part (gothic breastplate: vertex then colour).
    v.push(step(
        "edit_select_part",
        vec![
            tap(PageDown),
            tap(PageDown),
            tap(Tab),
            tap(Tab),
            tap(Tab),
            tap(Tab),
            tap(Tab),
            tap(KeyM),
            tap(KeyB),
            tap(KeyB),
            tap(KeyB),
        ],
        6,
        true,
        |s| selected_is(s, "gothic_cuirass") && part_is(s, "breastplate") && s.mode == Mode::Vertex,
    ));
    for name in [
        "edit_vertex_1",
        "edit_vertex_2",
        "edit_vertex_3",
        "edit_vertex_4",
    ] {
        v.push(step(name, vec![chord(ShiftLeft, ArrowRight)], 5, true, ok));
    }
    v.push(step(
        "edit_colour_mode",
        vec![tap(KeyM), tap(KeyM), tap(KeyM)],
        5,
        true,
        |s| s.mode == Mode::Colour && s.ramp_slot == 3,
    ));
    for name in ["edit_colour_1", "edit_colour_2", "edit_colour_3"] {
        v.push(step(
            name,
            vec![
                tap(KeyR),
                tap(KeyR),
                chord(ShiftLeft, KeyB),
                chord(ShiftLeft, KeyB),
            ],
            5,
            true,
            ok,
        ));
    }
    // Undo the experiments: reload from disk (nothing was saved).
    v.push(step(
        "edit_reload_discard",
        vec![chord(ControlLeft, KeyR)],
        6,
        true,
        |s| s.status.starts_with("reloaded"),
    ));
    // GIF 2: pivot and socket of the imported sword.
    v.extend([
        step(
            "pivot_select_sword",
            vec![tap(PageUp), tap(KeyM), tap(KeyM), tap(KeyM)],
            6,
            true,
            |s| selected_is(s, "arming_sword") && s.mode == Mode::Pivot,
        ),
        step(
            "pivot_move_1",
            vec![chord(ShiftLeft, ArrowRight)],
            5,
            true,
            ok,
        ),
        step(
            "pivot_move_2",
            vec![chord(ShiftLeft, ArrowRight), chord(ShiftLeft, ArrowRight)],
            5,
            true,
            ok,
        ),
        step(
            "pivot_rotate_back",
            vec![
                tap(KeyM),
                tap(KeyM),
                tap(KeyM),
                tap(KeyQ),
                tap(KeyQ),
                tap(KeyQ),
            ],
            5,
            true,
            |s| s.mode == Mode::Part,
        ),
        step(
            "pivot_rotate_more",
            vec![tap(KeyQ), tap(KeyQ), tap(KeyQ)],
            5,
            true,
            ok,
        ),
        step(
            "mount_mode",
            vec![tap(KeyM), tap(KeyM), tap(KeyM)],
            5,
            true,
            |s| s.mode == Mode::Mount,
        ),
        step(
            "mount_move",
            vec![chord(ShiftLeft, ArrowUp), chord(ShiftLeft, ArrowRight)],
            5,
            true,
            ok,
        ),
        step(
            "mount_rotate",
            vec![tap(KeyE), tap(KeyE), tap(KeyE), tap(KeyE)],
            5,
            true,
            ok,
        ),
        step(
            "mount_reload",
            vec![chord(ControlLeft, KeyR)],
            6,
            true,
            |s| s.status.starts_with("reloaded"),
        ),
    ]);
    // GIF 3 / third item: a belt made only with the studio, then saved.
    v.extend([
        step("new_design", vec![chord(ControlLeft, KeyN)], 6, true, |s| {
            selected_is(s, "studio_item_1")
        }),
        // Chest -> Bracers -> Cape -> Helmet -> Belt.
        step(
            "new_slot_belt",
            vec![
                chord(ControlLeft, KeyP),
                chord(ControlLeft, KeyP),
                chord(ControlLeft, KeyP),
                chord(ControlLeft, KeyP),
            ],
            6,
            true,
            |s| s.design().is_some_and(|d| d.slot == RigSlot::Belt),
        ),
        // Vertex mode: turn the quad into a band at the waist.
        // Mount -> Colour -> Part -> Vertex.
        step(
            "new_vertex_mode",
            vec![tap(KeyM), tap(KeyM), tap(KeyM)],
            4,
            false,
            |s| s.mode == Mode::Vertex,
        ),
    ]);
    // Vertices of the template quad: 0 (0.3, 0.3), 1 (0.7, 0.3), 2 (0.7, 0.5), 3 (0.3, 0.5).
    // One texel is about 1/16 u and 1/24 v on run frame 0; Shift = 4 texels.
    let mut belt = Vec::new();
    let mut nudge = |down_big: usize, down: usize, side: KeyCode, side_big: usize, next: bool| {
        for _ in 0..down_big {
            belt.push(chord(ShiftLeft, ArrowDown));
        }
        for _ in 0..down {
            belt.push(tap(ArrowDown));
        }
        for _ in 0..side_big {
            belt.push(chord(ShiftLeft, side));
        }
        if next {
            belt.push(tap(KeyB));
        }
    };
    nudge(3, 2, ArrowLeft, 2, true); // vertex 0 -> back of the waist, top edge
    nudge(3, 1, ArrowRight, 2, true); // vertex 1 -> front, top edge
    nudge(3, 0, ArrowRight, 2, true); // vertex 2 -> front, bottom edge
    nudge(3, 0, ArrowLeft, 2, false); // vertex 3 -> back, bottom edge
    v.push(Step {
        name: "new_belt_shape",
        chords: belt,
        wait: 6,
        capture: true,
        check: ok,
    });
    v.extend([
        // Colour: a dark brown leather band.
        // Vertex -> Pivot -> Mount -> Colour; base, light and shadow slots
        // turned from steel grey to brown leather.
        step("new_belt_colour", vec![
            tap(KeyM), tap(KeyM), tap(KeyM),
            tap(ArrowDown), tap(ArrowDown), tap(ArrowDown), tap(ArrowDown), tap(KeyR), tap(KeyR), chord(ShiftLeft, KeyB), chord(ShiftLeft, KeyB), chord(ShiftLeft, KeyB), chord(ShiftLeft, KeyB),
            tap(ArrowRight), tap(ArrowDown), tap(ArrowDown), tap(ArrowDown), tap(KeyR), tap(KeyR), chord(ShiftLeft, KeyB), chord(ShiftLeft, KeyB), chord(ShiftLeft, KeyB), chord(ShiftLeft, KeyB),
            tap(ArrowLeft), tap(ArrowLeft), tap(ArrowDown), tap(ArrowDown), tap(KeyR), tap(KeyR), chord(ShiftLeft, KeyB), chord(ShiftLeft, KeyB), chord(ShiftLeft, KeyB),
        ], 6, true, |s| s.mode == Mode::Colour && s.ramp_slot == 2),
        // A buckle: import PNGs until the brass buckle comes up.
        step("new_belt_buckle", vec![tap(KeyI)], 8, true, |s| s.part().is_some_and(|p| matches!(&p.source, PartSource::Image { path, .. } if path.ends_with("buckle_brass.png")))),
        step("new_belt_buckle_place", vec![tap(KeyM), chord(ShiftLeft, ArrowDown), chord(ShiftLeft, ArrowDown), chord(ShiftLeft, ArrowRight)], 6, true, |s| s.mode == Mode::Part),
        step("new_belt_saved", vec![chord(ControlLeft, KeyS)], 8, true, |s| s.status.starts_with("saved")),
    ]);
    // Swapping during the run, the clock never resets.
    v.extend([
        step("swap_play", vec![tap(Space)], 6, true, |s| s.playing),
        step(
            "swap_1_jerkin",
            vec![tap(PageUp), tap(Enter)],
            4,
            true,
            |s| selected_is(s, "leather_jerkin"),
        ),
        step("swap_2", vec![], 3, true, ok),
        step(
            "swap_3_gothic",
            vec![tap(PageUp), tap(Enter)],
            4,
            true,
            |s| selected_is(s, "gothic_cuirass"),
        ),
        step("swap_4", vec![], 3, true, ok),
        step(
            "swap_5_sword_off",
            vec![tap(PageUp), tap(Enter)],
            4,
            true,
            |s| !s.equipped.contains_key(&RigSlot::MainHand),
        ),
        step("swap_6", vec![], 3, true, ok),
        step("swap_7_sword_on", vec![tap(Enter)], 4, true, |s| {
            s.equipped.contains_key(&RigSlot::MainHand)
        }),
        step(
            "swap_8_belt_off",
            vec![tap(PageUp), tap(Enter)],
            4,
            true,
            |s| !s.equipped.contains_key(&RigSlot::Belt),
        ),
        step("swap_9_belt_on", vec![tap(Enter)], 4, true, |s| {
            s.equipped.contains_key(&RigSlot::Belt)
        }),
        step("final_all", vec![tap(Space), tap(KeyG)], 6, true, |s| {
            !s.playing && s.skeleton
        }),
    ]);
    v
}

/// What `--tour-reload` checks: the studio reopened the saved item.
fn reload_steps() -> Vec<Step> {
    use KeyCode::*;
    vec![
        step("reload_list", vec![tap(KeyT), tap(Space)], 12, true, |s| {
            s.entries.iter().any(|e| e.design.id == "studio_item_1")
        }),
        step("reload_selected", vec![tap(PageUp)], 8, true, |s| {
            selected_is(s, "studio_item_1")
                && s.design()
                    .is_some_and(|d| d.slot == RigSlot::Belt && d.parts.len() == 2)
        }),
        step("reload_frames", vec![tap(Period), tap(Period)], 6, true, ok),
    ]
}

fn run_tour(
    mut commands: Commands,
    mut s: ResMut<Studio>,
    mut keyboard: ResMut<ButtonInput<KeyCode>>,
    mut exit: MessageWriter<AppExit>,
) {
    if s.tour.is_none()
        || !s.body_ready
        || s.entries.iter().any(|e| {
            e.painted.is_none()
                && s.equipped
                    .values()
                    .any(|i| s.entries.get(*i).is_some_and(|x| x.path == e.path))
        }) && s.warm < 200
    {
        s.warm += 1;
        return;
    }
    if s.warm < 30 {
        s.warm += 1;
        return;
    }
    let total = s.tour.as_ref().map_or(0, Vec::len);
    if s.step >= total {
        s.wait += 1;
        if s.wait > 30 && VERDICT.get().is_none() {
            for f in &s.failures {
                println!("{TAG} FAIL {f}");
            }
            let pass = s.failures.is_empty();
            println!("{TAG} RESULT: {}", if pass { "PASS" } else { "FAIL" });
            let _ = VERDICT.set(pass);
            exit.write(AppExit::Success);
        }
        return;
    }
    let (chords, wait, capture, check, name) = {
        let Some(t) = s.tour.as_ref().and_then(|t| t.get(s.step)) else {
            return;
        };
        (t.chords.clone(), t.wait, t.capture, t.check, t.name)
    };
    let i = (s.wait / 2) as usize;
    if let Some(c) = chords.get(i) {
        // Modifiers go down first and come up last.
        if s.wait.is_multiple_of(2) {
            for k in c {
                keyboard.press(*k);
            }
        } else {
            for k in c.iter().rev() {
                keyboard.release(*k);
            }
        }
    }
    let at = 2 * chords.len() as u32 + wait;
    if s.wait == at {
        if !check(&s) {
            let msg = format!("{name}: the studio did not reach the expected state");
            s.failures.push(msg);
        }
        if capture {
            let path = s.out.join(format!("{name}.png"));
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(path));
        }
        println!(
            "{TAG} {name} design={} part={} mode={:?} clip={} frame={} equipped={:?} status={}",
            s.design().map_or("-", |d| d.id.as_str()),
            s.part().map_or("-", |p| p.id.as_str()),
            s.mode,
            s.clip,
            s.frame,
            s.equipped
                .iter()
                .map(|(k, i)| (
                    format!("{k:?}"),
                    s.entries
                        .get(*i)
                        .map(|e| e.design.id.clone())
                        .unwrap_or_default()
                ))
                .collect::<Vec<_>>(),
            s.status
        );
    }
    s.wait += 1;
    if s.wait > at + 1 {
        s.wait = 0;
        s.step += 1;
    }
}

fn main() -> AppExit {
    let root = project_root();
    let args: Vec<String> = std::env::args().collect();
    let tour = if args.iter().any(|a| a == "--tour") {
        Some(tour_steps())
    } else if args.iter().any(|a| a == "--tour-reload") {
        Some(reload_steps())
    } else {
        None
    };
    let lab_path = root.join(LAB_STAGE).join("rig_lab.ron");
    let rig = match std::fs::read_to_string(&lab_path)
        .map_err(|e| e.to_string())
        .and_then(|t| RigLab::from_ron(&t).map_err(|e| e.to_string()))
    {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{TAG} {}: {e}", lab_path.display());
            return AppExit::error();
        }
    };
    let Some(rest_joints) = rig.rest().and_then(|c| c.frame_joints(0)) else {
        eprintln!("{TAG} rest pose missing");
        return AppExit::error();
    };
    let studio_root = root.join(STUDIO);
    let mut imports: Vec<String> = std::fs::read_dir(studio_root.join("import"))
        .map(|d| {
            d.filter_map(|e| e.ok())
                .filter_map(|e| e.file_name().into_string().ok())
                .filter(|n| n.ends_with(".png"))
                .map(|n| format!("import/{n}"))
                .collect()
        })
        .unwrap_or_default();
    imports.sort();
    let out = root.join(CAPTURES);
    let _ = std::fs::create_dir_all(&out);
    let mut app = App::new();
    app.register_asset_source(
        "approved",
        AssetSourceBuilder::platform_default(&root.join(APPROVED).to_string_lossy(), None),
    )
    .register_asset_source(
        "studio",
        AssetSourceBuilder::platform_default(&studio_root.to_string_lossy(), None),
    )
    .add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Dark Castle Equipment Studio".into(),
                    resolution: WindowResolution::new(1280, 720),
                    ..default()
                }),
                ..default()
            }),
    )
    .insert_resource(ClearColor(DARK))
    .insert_resource(Studio {
        root: studio_root,
        rest: fit(&rest_joints),
        rig,
        strips: HashMap::new(),
        body: HashMap::new(),
        body_ready: false,
        entries: Vec::new(),
        equipped: BTreeMap::new(),
        selected: 0,
        part: 0,
        vertex: 0,
        mode: Mode::Part,
        ramp_slot: 3,
        image_handles: HashMap::new(),
        decoded: HashMap::new(),
        imports,
        import_cursor: 0,
        clip: "idle".into(),
        elapsed: 0.0,
        frame: 0,
        flip: false,
        playing: true,
        zoom: 4,
        isolated: false,
        unequipped: false,
        skeleton: false,
        status: "ready".into(),
        dragging: false,
        tour,
        step: 0,
        wait: 0,
        warm: 0,
        out,
        failures: Vec::new(),
    })
    .add_systems(Startup, setup)
    .add_systems(
        Update,
        (
            load_assets,
            run_tour,
            keys,
            mouse,
            advance,
            repaint,
            sync_views,
            overlays,
            hud,
        )
            .chain(),
    );
    let tour_mode = args.iter().any(|a| a.starts_with("--tour"));
    let exit = app.run();
    if !tour_mode {
        return exit;
    }
    match VERDICT.get() {
        Some(true) => exit,
        _ => AppExit::error(),
    }
}
