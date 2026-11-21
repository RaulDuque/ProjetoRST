//! Dark Castle Equipment Studio V2 — Modular Character Engine 04 (a tool,
//! separate from the game).
//!
//! Panels (Bevy UI): a top bar (new, open, save, save as, import, undo,
//! redo), a left panel (equipment library by category, part hierarchy and
//! part buttons), the canvas (the approved protagonist wearing the items), a
//! right panel (properties of the selected part and design) and a bottom bar
//! (animation timeline and view switches). The canvas and the 2x and 1x
//! previews show one and the same composite (`rig::paint::compose_frame`):
//! only the zoom and the optional guides differ.
//!
//! ```text
//! cargo run --example equipment_studio                    # edits art-src/equipment-studio/items
//! cargo run --example equipment_studio -- --items DIR     # edits another folder
//! cargo run --example equipment_studio -- --tour          # evidence run on a fresh workspace copy, then exit
//! cargo run --example equipment_studio -- --tour-reload   # reopens that workspace, then exit
//! ```
//!
//! The tours work on a copy in `target/equipment_studio/workspace/`: the
//! owner's saved items are never written and the runs repeat.
//!
//! | Key | Effect (the panels have the same actions) |
//! |---|---|
//! | Ctrl+Z / Ctrl+Y or Ctrl+Shift+Z | undo / redo |
//! | Ctrl+N / Ctrl+O / Ctrl+S / Ctrl+Shift+S | new / open (rescan) / save / save as |
//! | Ctrl+R | revert every design to disk (twice when there are unsaved changes) |
//! | PageUp / PageDown, Enter | select design, equip / unequip |
//! | Tab / Shift+Tab | next / previous part |
//! | N / I / Delete / Ctrl+D / Ctrl+G / H | add part / import PNG / remove / duplicate / group / hide |
//! | [ ] | send back / bring forward |
//! | M | mode: part, vertex, pivot, mount, colour, paint |
//! | Arrows (Shift x4), Q / E, - / = | move, rotate, scale (part or mount) |
//! | V / B, Y, Backspace | vertex: previous / next, insert, delete |
//! | Space, . / , | play / pause, next / previous frame |
//! | T, F, Z / X | idle / run, mirror, zoom |
//! | O, U, G, K | isolated item, unequipped, guides on / off, skeleton |
//!
//! Mouse: click a part on the canvas to select it; part mode: drag moves,
//! right-drag rotates around the pivot, Ctrl+wheel scales; vertex mode:
//! drag a vertex; pivot mode: place the pivot; paint mode: brush, eraser,
//! fill or picker on a pixel part; wheel zooms. One drag is one undo step.

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use bevy::asset::AssetApp;
use bevy::asset::io::AssetSourceBuilder;
use bevy::input::mouse::AccumulatedMouseScroll;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::sprite::Anchor;
use bevy::ui::{ComputedNode, UiGlobalTransform};
use bevy::window::{PrimaryWindow, WindowCloseRequested, WindowResolution};
use dark_castle_rpg::core::pixel_art::PixelArt;
use dark_castle_rpg::core::pixel_image::pixel_image;
use dark_castle_rpg::core::resources::project_root;
use dark_castle_rpg::core::time::clamp_delta;
use dark_castle_rpg::player::visual::manifest::anchor_for;
use dark_castle_rpg::player::visual::playback::loop_frame;
use dark_castle_rpg::player::visual::rig::chest::TorsoCage;
use dark_castle_rpg::player::visual::rig::data::{RigClip, RigLab};
use dark_castle_rpg::player::visual::rig::design::{
    DESIGN_VERSION, DesignSpace, EquipmentDesign, Finish, Ornament, Outline, PaintMaterial, Part,
    PartSource, PartTransform, Region, Visibility as PartVisibility, chaikin, pixel_char,
    pixel_index,
};
use dark_castle_rpg::player::visual::rig::equipment::{BoneEnd, RigSlot};
use dark_castle_rpg::player::visual::rig::grip::HandSide;
use dark_castle_rpg::player::visual::rig::history::History;
use dark_castle_rpg::player::visual::rig::layers::{BodyRegion, DepthSlot};
use dark_castle_rpg::player::visual::rig::paint::{
    Painted, RgbaImage, SpaceMap, WornItem, compose_frame, paint_on_frame, pixel_cell_at,
    space_map, split_by_depth,
};
use dark_castle_rpg::player::visual::rig::skeleton::{Bone, LocalPose, fit, forward};

const TAG: &str = "[equipment_studio]";
const STUDIO: &str = "art-src/equipment-studio";
const LAB_STAGE: &str = "art-src/modular-character-engine/stage";
const APPROVED: &str = "art-src/n3-8-production-run/stage";
const CAPTURES: &str = "target/equipment_studio/v2";
const WORKSPACE: &str = "target/equipment_studio/workspace";
const BG: Color = Color::srgb(0.14, 0.13, 0.17);
const PANEL: Color = Color::srgb(0.09, 0.09, 0.12);
const BUTTON: Color = Color::srgb(0.22, 0.21, 0.27);
const BUTTON_ON: Color = Color::srgb(0.47, 0.33, 0.12);
const INK: Color = Color::srgb(0.96, 0.88, 0.62);
const DIM: Color = Color::srgb(0.68, 0.66, 0.74);
const FONT: f32 = 11.0;
/// Layout (logical px, from the top-left corner).
const WIN: (f32, f32) = (1280.0, 720.0);
const TOP_H: f32 = 30.0;
const LEFT_W: f32 = 250.0;
const RIGHT_W: f32 = 300.0;
const BOTTOM_H: f32 = 54.0;
/// Views: centre x and sole y (logical, from the top-left).
const MAIN_VIEW: (f32, f32) = (500.0, 630.0);
const VIEW_2X: (f32, f32) = (870.0, 630.0);
const VIEW_1X: (f32, f32) = (870.0, 330.0);
const ZOOM_RANGE: (u32, u32) = (2, 8);
const SPEEDS: [f32; 4] = [0.25, 0.5, 1.0, 2.0];
/// Mouse: how close (screen px) a vertex must be to be grabbed.
const GRAB_RADIUS: f32 = 24.0;
/// Seconds a second Ctrl+R / close request has to confirm a discard.
const CONFIRM_SECONDS: f32 = 5.0;

static VERDICT: OnceLock<bool> = OnceLock::new();

// ------------------------------------------------------------------ state

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Part,
    Vertex,
    Pivot,
    Mount,
    Colour,
    Paint,
}

impl Mode {
    const ALL: [Self; 6] = [
        Self::Part,
        Self::Vertex,
        Self::Pivot,
        Self::Mount,
        Self::Colour,
        Self::Paint,
    ];

    fn next(self) -> Self {
        next_in(&Self::ALL, self)
    }

    fn name(self) -> &'static str {
        match self {
            Self::Part => "part",
            Self::Vertex => "vertex",
            Self::Pivot => "pivot",
            Self::Mount => "mount",
            Self::Colour => "colour",
            Self::Paint => "paint",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tool {
    Brush,
    Eraser,
    Fill,
    Picker,
}

/// What vertex editing edits: the part's polygon or one of its regions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Target {
    Shape,
    Region(usize),
}

/// Numeric fields of the properties panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    PosX,
    PosY,
    Rot,
    ScaleX,
    ScaleY,
    PivotX,
    PivotY,
    Layer,
    Bevel,
    Smooth,
}

/// Every button of the panels (the keyboard and the tour use the same).
#[derive(Debug, Clone, PartialEq)]
enum Action {
    New,
    Open,
    Save,
    SaveAs,
    Import,
    Undo,
    Redo,
    Category(Option<RigSlot>),
    SelectDesign(usize),
    ToggleEquip(usize),
    SelectPart(usize),
    AddPart,
    AddPixels,
    Duplicate,
    Group,
    Remove,
    ToggleHide,
    ParentNext,
    ParentTo(Option<usize>),
    KeepGlobal,
    Nudge(Field, f32),
    DesignSpace,
    PartBind,
    PartDepth,
    DesignDepth,
    MaterialNext,
    NewMaterial,
    FinishNext,
    RampSlot(usize),
    Channel(usize, i16),
    OutlineNext,
    AddRegion,
    DeleteRegion,
    TargetNext,
    AddRivet,
    AddStitch,
    DeleteOrnament,
    ClipsNext,
    TurnNext,
    HidesNext,
    SetMode(Mode),
    SetTool(Tool),
    PaletteIndex(usize),
    Frame(usize),
    Clip(&'static str),
    Play,
    Step(i32),
    Speed(i32),
    Mirror,
    Zoom(i32),
    Grid,
    Guides,
    Isolated,
    Unequipped,
    Skeleton,
}

struct Entry {
    path: PathBuf,
    design: EquipmentDesign,
    saved: Option<EquipmentDesign>,
    rev: u64,
}

impl Entry {
    fn dirty(&self) -> bool {
        self.saved.as_ref() != Some(&self.design)
    }
}

/// What undo restores.
#[derive(Clone, PartialEq)]
struct Snapshot {
    designs: Vec<(PathBuf, EquipmentDesign)>,
    selected: usize,
    part: usize,
}

/// One frame of the tour's synthetic mouse.
#[derive(Debug, Clone, Copy)]
struct PointerFrame {
    pos: Vec2,
    left: bool,
    right: bool,
}

impl PointerFrame {
    fn up(pos: Vec2) -> Self {
        Self {
            pos,
            left: false,
            right: false,
        }
    }

    fn left(pos: Vec2) -> Self {
        Self {
            pos,
            left: true,
            right: false,
        }
    }

    fn right(pos: Vec2) -> Self {
        Self {
            pos,
            left: false,
            right: true,
        }
    }
}

/// A tour operation.
#[derive(Debug, Clone)]
enum Op {
    /// A key chord through `ButtonInput<KeyCode>`.
    Keys(Vec<KeyCode>),
    /// A mouse click on the panel button of this action.
    Click(Action),
    /// A click on the library button of a design.
    ClickDesign(&'static str),
    /// A click on the equip button of a design.
    ClickEquip(&'static str),
    /// A click on the hierarchy row of a part (selected design).
    ClickRow(&'static str),
    /// A click on the "parent" button of a hierarchy row.
    ClickParent(&'static str),
    /// A click on the canvas where this part is on top.
    ClickPart(&'static str),
    DragPart(f32, f32),
    RotateDrag(f32),
    DragVertex(usize, f32, f32),
    /// One brush stroke over these cells of the selected pixel part.
    PaintCells(Vec<(usize, usize)>),
    Wait(u32),
}

struct Step {
    name: &'static str,
    ops: Vec<Op>,
    capture: bool,
    check: fn(&Studio) -> bool,
}

#[derive(Resource)]
struct Studio {
    items_dir: PathBuf,
    rig: RigLab,
    rest: LocalPose,
    strips: HashMap<String, Handle<Image>>,
    body: HashMap<(String, usize), Vec<(BodyRegion, Vec<u8>)>>,
    body_ready: bool,
    entries: Vec<Entry>,
    equipped: BTreeMap<RigSlot, usize>,
    selected: usize,
    part: usize,
    vertex: usize,
    target: Target,
    mode: Mode,
    tool: Tool,
    palette_index: usize,
    ramp_slot: usize,
    keep_global: bool,
    category: Option<RigSlot>,
    image_handles: HashMap<String, Handle<Image>>,
    decoded: HashMap<String, RgbaImage>,
    imports: Vec<String>,
    import_cursor: usize,
    clip: String,
    elapsed: f32,
    frame: usize,
    flip: bool,
    playing: bool,
    speed: usize,
    zoom: u32,
    grid: bool,
    guides: bool,
    isolated: bool,
    unequipped: bool,
    skeleton: bool,
    status: String,
    history: History<Snapshot>,
    dragging: Option<(MouseButton, Vec2)>,
    drag_before: Option<(&'static str, Snapshot)>,
    composite: Handle<Image>,
    composite_key: Option<String>,
    pick: Vec<Option<(usize, usize)>>,
    last_composite: Vec<[u8; 4]>,
    painted: HashMap<(usize, u64, String, usize), (Painted, DepthSlot)>,
    ui_key: String,
    close_armed: f32,
    revert_armed: f32,
    pointer: Option<PointerFrame>,
    last_buttons: (bool, bool),
    tour: Option<Vec<Step>>,
    step: usize,
    op: usize,
    op_frame: u32,
    queue: Vec<PointerFrame>,
    keys_held: Vec<KeyCode>,
    warm: u32,
    out: PathBuf,
    failures: Vec<String>,
    exit_now: bool,
}

impl Studio {
    fn design(&self) -> Option<&EquipmentDesign> {
        self.entries.get(self.selected).map(|e| &e.design)
    }

    fn part(&self) -> Option<&Part> {
        self.design()?.parts.get(self.part)
    }

    fn clip(&self) -> Option<&RigClip> {
        self.rig.clips.get(&self.clip)
    }

    fn snapshot(&self) -> Snapshot {
        Snapshot {
            designs: self
                .entries
                .iter()
                .map(|e| (e.path.clone(), e.design.clone()))
                .collect(),
            selected: self.selected,
            part: self.part,
        }
    }

    /// Records the state before an edit (one undo step).
    fn record(&mut self, label: &str) {
        let s = self.snapshot();
        self.history.record(label, &s);
    }

    fn restore(&mut self, s: Snapshot) {
        let saved: HashMap<PathBuf, Option<EquipmentDesign>> = self
            .entries
            .iter()
            .map(|e| (e.path.clone(), e.saved.clone()))
            .collect();
        let rev = self.entries.iter().map(|e| e.rev).max().unwrap_or(0) + 1;
        self.entries = s
            .designs
            .into_iter()
            .map(|(path, design)| Entry {
                saved: saved.get(&path).cloned().flatten(),
                path,
                design,
                rev,
            })
            .collect();
        self.selected = s.selected.min(self.entries.len().saturating_sub(1));
        self.part = s.part;
        self.clamp_selection();
    }

    fn clamp_selection(&mut self) {
        let n = self.design().map_or(0, |d| d.parts.len());
        self.part = self.part.min(n.saturating_sub(1));
        let len = self.entries.len();
        self.equipped.retain(|_, i| *i < len);
        if let Target::Region(k) = self.target
            && self.part().is_none_or(|p| k >= p.regions.len())
        {
            self.target = Target::Shape;
        }
    }

    /// The selected design, mutable (bumps its revision).
    fn edit(&mut self) -> Option<&mut EquipmentDesign> {
        let e = self.entries.get_mut(self.selected)?;
        e.rev += 1;
        Some(&mut e.design)
    }

    fn edit_part(&mut self) -> Option<&mut Part> {
        let part = self.part;
        self.edit()?.parts.get_mut(part)
    }

    /// The space map of the selected part on the current frame.
    fn part_map(&self) -> Option<SpaceMap> {
        let d = self.design()?;
        let space = d.effective_space(self.part);
        space_map(&self.rig, &self.rest, &space, &self.clip, self.frame).map(|(m, _)| m)
    }

    fn part_in_torso(&self) -> bool {
        self.design()
            .is_some_and(|d| d.effective_space(self.part) == DesignSpace::Torso)
    }

    /// One texel in the selected part's space.
    fn texel(&self) -> (f32, f32) {
        self.part_map().map_or((1.0, 1.0), |m| m.texel())
    }

    fn rig_to_screen(&self, p: (f32, f32)) -> Vec2 {
        let sign = if self.flip { -1.0 } else { 1.0 };
        let z = self.zoom as f32;
        Vec2::new(MAIN_VIEW.0 + p.0 * sign * z, MAIN_VIEW.1 - p.1 * z)
    }

    fn screen_to_rig(&self, s: Vec2) -> (f32, f32) {
        let sign = if self.flip { -1.0 } else { 1.0 };
        let z = self.zoom as f32;
        ((s.x - MAIN_VIEW.0) / z * sign, (MAIN_VIEW.1 - s.y) / z)
    }

    /// Part-local point of the selected part -> screen.
    fn local_to_screen(&self, l: (f32, f32)) -> Option<Vec2> {
        let d = self.design()?;
        let w = d.mount.affine().then(&d.world_affine(self.part));
        Some(self.rig_to_screen(self.part_map()?.to_rig(w.apply(l))))
    }

    /// Screen -> local coordinates of the selected part.
    fn screen_to_local(&self, s: Vec2) -> Option<(f32, f32)> {
        let d = self.design()?;
        let w = d.mount.affine().then(&d.world_affine(self.part));
        let space = self.part_map()?.to_space(self.screen_to_rig(s))?;
        Some(w.inverse()?.apply(space))
    }

    /// Screen -> the parent space of the selected part (moving the part).
    fn screen_to_parent(&self, s: Vec2) -> Option<(f32, f32)> {
        let d = self.design()?;
        let base = match d.parent_of(self.part) {
            Some(p) => d.mount.affine().then(&d.world_affine(p)),
            None => d.mount.affine(),
        };
        let space = self.part_map()?.to_space(self.screen_to_rig(s))?;
        Some(base.inverse()?.apply(space))
    }

    fn texel_screen(&self, t: usize) -> Option<Vec2> {
        let g = self.clip()?.geometry();
        Some(self.rig_to_screen(g.texel_centre(t % g.width, t / g.width)))
    }

    fn screen_texel(&self, s: Vec2) -> Option<usize> {
        let g = self.clip()?.geometry();
        let (x, y) = g.to_texel(self.screen_to_rig(s));
        let inside = x >= 0.0 && y >= 0.0 && (x as usize) < g.width && (y as usize) < g.height;
        inside.then(|| y as usize * g.width + x as usize)
    }

    fn load_image(&mut self, path: &str, server: &AssetServer) {
        if !self.image_handles.contains_key(path) {
            self.image_handles
                .insert(path.to_owned(), server.load(format!("studio://{path}")));
        }
    }

    /// The points vertex editing works on.
    fn target_points(&self) -> Option<Vec<(f32, f32)>> {
        let p = self.part()?;
        match self.target {
            Target::Shape => match &p.source {
                PartSource::Polygon { points, .. } => Some(points.clone()),
                _ => None,
            },
            Target::Region(k) => p.regions.get(k).map(|r| r.points.clone()),
        }
    }

    fn target_points_mut(&mut self) -> Option<&mut Vec<(f32, f32)>> {
        let target = self.target;
        let p = self.edit_part()?;
        match target {
            Target::Shape => match &mut p.source {
                PartSource::Polygon { points, .. } => Some(points),
                _ => None,
            },
            Target::Region(k) => p.regions.get_mut(k).map(|r| &mut r.points),
        }
    }

    fn any_dirty(&self) -> bool {
        self.entries.iter().any(Entry::dirty)
    }

    /// Starts a drag: the state before it becomes one undo step when the
    /// drag ends, if anything changed.
    fn begin_drag(&mut self, label: &'static str, button: MouseButton, pos: Vec2) {
        self.drag_before = Some((label, self.snapshot()));
        self.dragging = Some((button, pos));
    }

    fn end_drag(&mut self) {
        self.dragging = None;
        if let Some((label, before)) = self.drag_before.take()
            && before.designs != self.snapshot().designs
        {
            self.history.record(label, &before);
        }
    }

    fn design_index(&self, id: &str) -> Option<usize> {
        self.entries.iter().position(|e| e.design.id == id)
    }
}

// ------------------------------------------------------------ components

#[derive(Component)]
struct ViewSprite(u8);

#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum PanelKind {
    Top,
    Left,
    Right,
    Bottom,
}

#[derive(Component)]
struct UiButton(Action);

// ------------------------------------------------------------ helpers

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

fn blank_part(d: &EquipmentDesign, id: &str, source: PartSource) -> Part {
    Part {
        id: d.free_id(id),
        parent: None,
        bind: None,
        depth: None,
        hides: Vec::new(),
        source,
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

/// A small quad near the middle of a space.
fn quad_for(space: &DesignSpace) -> Vec<(f32, f32)> {
    match space {
        DesignSpace::Torso => vec![(0.3, 0.3), (0.7, 0.3), (0.7, 0.5), (0.3, 0.5)],
        _ => vec![(-3.0, -3.0), (3.0, -3.0), (3.0, 3.0), (-3.0, 3.0)],
    }
}

/// Where a new image or pixel part goes in a space.
fn centre_for(space: &DesignSpace) -> (f32, f32) {
    match space {
        DesignSpace::Torso => (0.62, 0.42),
        _ => (0.0, 0.0),
    }
}

fn bbox(points: &[(f32, f32)]) -> ((f32, f32), (f32, f32)) {
    let mut lo = (f32::MAX, f32::MAX);
    let mut hi = (f32::MIN, f32::MIN);
    for p in points {
        lo = (lo.0.min(p.0), lo.1.min(p.1));
        hi = (hi.0.max(p.0), hi.1.max(p.1));
    }
    (lo, hi)
}

/// Spaces offered by the "cycle" buttons.
const SPACES: [DesignSpace; 8] = [
    DesignSpace::Torso,
    DesignSpace::Hand {
        side: HandSide::Near,
    },
    DesignSpace::Hand {
        side: HandSide::Far,
    },
    DesignSpace::Bone {
        bone: Bone::UpperArmNear,
        at: BoneEnd::Start,
        follow: 0.5,
    },
    DesignSpace::Bone {
        bone: Bone::ForearmNear,
        at: BoneEnd::Start,
        follow: 1.0,
    },
    DesignSpace::Bone {
        bone: Bone::ForearmFar,
        at: BoneEnd::Start,
        follow: 1.0,
    },
    DesignSpace::Bone {
        bone: Bone::Head,
        at: BoneEnd::Start,
        follow: 1.0,
    },
    DesignSpace::Bone {
        bone: Bone::ThighA,
        at: BoneEnd::Start,
        follow: 1.0,
    },
];

const GEAR_DEPTHS: [DepthSlot; 11] = [
    DepthSlot::ChestGear,
    DepthSlot::WaistGear,
    DepthSlot::BeltGear,
    DepthSlot::ShoulderGear,
    DepthSlot::BracerGear,
    DepthSlot::FarHandGear,
    DepthSlot::HelmetGear,
    DepthSlot::MainHandGear,
    DepthSlot::LegGear,
    DepthSlot::BootGear,
    DepthSlot::BackGear,
];

/// Body regions a part may replace (cycled by the "hides" button).
const HIDE_OPTIONS: [&[BodyRegion]; 7] = [
    &[],
    &[BodyRegion::NearUpperArm],
    &[BodyRegion::NearForearm],
    &[BodyRegion::NearHand],
    &[BodyRegion::Torso],
    &[BodyRegion::Waist],
    &[BodyRegion::Hair],
];

const FINISHES: [Finish; 4] = [Finish::Metal, Finish::Leather, Finish::Cloth, Finish::Flat];
const OUTLINES: [Outline; 3] = [Outline::Selective, Outline::Ink, Outline::None];

fn space_name(s: &DesignSpace) -> String {
    match s {
        DesignSpace::Torso => "torso cage".into(),
        DesignSpace::Hand { side } => format!("hand {side:?}"),
        DesignSpace::Bone { bone, follow, .. } => format!("bone {bone:?} {follow:.1}"),
    }
}

fn next_in<T: PartialEq + Copy>(list: &[T], cur: T) -> T {
    let i = list
        .iter()
        .position(|x| *x == cur)
        .map_or(0, |i| (i + 1) % list.len());
    list[i]
}

/// The next element of a list after `cur`, `None` after the last one.
fn cycle_opt<T: PartialEq + Copy>(list: &[T], cur: Option<T>) -> Option<T> {
    match cur {
        None => list.first().copied(),
        Some(c) => {
            let i = list.iter().position(|x| *x == c).map_or(0, |i| i + 1);
            list.get(i).copied()
        }
    }
}

fn polygon_points(p: Option<&Part>) -> Option<Vec<(f32, f32)>> {
    match p.map(|p| &p.source) {
        Some(PartSource::Polygon { points, .. }) => Some(points.clone()),
        _ => None,
    }
}

// ------------------------------------------------------------ loading

fn load_designs(dir: &Path) -> Vec<Entry> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(dir)
        .map(|d| d.filter_map(|e| e.ok().map(|e| e.path())).collect())
        .unwrap_or_default();
    paths.retain(|p| p.extension().is_some_and(|e| e == "ron"));
    paths.sort();
    paths
        .into_iter()
        .filter_map(|path| {
            let text = std::fs::read_to_string(&path).ok()?;
            match EquipmentDesign::from_ron(&text) {
                Ok(design) => Some(Entry {
                    saved: Some(design.clone()),
                    path,
                    design,
                    rev: 1,
                }),
                Err(e) => {
                    eprintln!("{TAG} {}: {e}", path.display());
                    None
                }
            }
        })
        .collect()
}

fn panel_node(left: f32, top: f32, width: f32, height: f32) -> Node {
    Node {
        position_type: PositionType::Absolute,
        left: px(left),
        top: px(top),
        width: px(width),
        height: px(height),
        flex_direction: FlexDirection::Row,
        flex_wrap: FlexWrap::Wrap,
        align_content: AlignContent::FlexStart,
        align_items: AlignItems::Center,
        padding: UiRect::all(px(4)),
        column_gap: px(3),
        row_gap: px(2),
        overflow: Overflow::clip(),
        ..default()
    }
}

fn setup(
    mut commands: Commands,
    mut res: ResMut<Studio>,
    server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    let s = &mut *res;
    commands.spawn(Camera2d);
    for (key, clip) in &s.rig.clips {
        s.strips
            .insert(key.clone(), server.load(clip.texture.clone()));
    }
    s.entries = load_designs(&s.items_dir);
    let mut paths: Vec<String> = s
        .entries
        .iter()
        .flat_map(|e| e.design.parts.iter())
        .filter_map(|p| match &p.source {
            PartSource::Image { path, .. } => Some(path.clone()),
            _ => None,
        })
        .collect();
    paths.extend(s.imports.iter().cloned());
    for p in paths {
        s.load_image(&p, &server);
    }
    // Wear the gothic set when it is there, and a sword.
    let mut eq = BTreeMap::new();
    for (i, e) in s.entries.iter().enumerate() {
        if e.design.id.starts_with("gothic_set_") || e.design.slot == RigSlot::MainHand {
            eq.entry(e.design.slot).or_insert(i);
        }
    }
    s.equipped = eq;
    s.selected = s.design_index("gothic_set_cuirass").unwrap_or(0);
    s.composite = images.add(pixel_image(PixelArt::empty()));
    let views = [(MAIN_VIEW, s.zoom as f32), (VIEW_2X, 2.0), (VIEW_1X, 1.0)];
    for (i, ((x, y), scale)) in views.into_iter().enumerate() {
        commands.spawn((
            ViewSprite(i as u8),
            Sprite::default(),
            Anchor::default(),
            Transform::from_xyz(x - WIN.0 * 0.5, WIN.1 * 0.5 - y, 0.0)
                .with_scale(Vec3::splat(scale)),
            Visibility::Hidden,
        ));
    }
    let middle = WIN.1 - TOP_H - BOTTOM_H;
    let panels = [
        (PanelKind::Top, panel_node(0.0, 0.0, WIN.0, TOP_H)),
        (PanelKind::Left, panel_node(0.0, TOP_H, LEFT_W, middle)),
        (
            PanelKind::Right,
            panel_node(WIN.0 - RIGHT_W, TOP_H, RIGHT_W, middle),
        ),
        (
            PanelKind::Bottom,
            panel_node(0.0, WIN.1 - BOTTOM_H, WIN.0, BOTTOM_H),
        ),
    ];
    for (kind, node) in panels {
        commands.spawn((kind, node, BackgroundColor(PANEL)));
    }
}

/// Body regions (the shared cut of the engine) and the decoded PNGs.
fn load_assets(mut res: ResMut<Studio>, images: Res<Assets<Image>>) {
    let s = &mut *res;
    let pending: Vec<(String, Handle<Image>)> = s
        .image_handles
        .iter()
        .filter(|(p, _)| !s.decoded.contains_key(*p))
        .map(|(p, h)| (p.clone(), h.clone()))
        .collect();
    for (path, h) in pending {
        if let Some(img) = images.get(&h)
            && let Some(data) = img.data.as_ref()
        {
            s.decoded.insert(
                path,
                RgbaImage {
                    width: img.width() as usize,
                    height: img.height() as usize,
                    pixels: data.as_chunks::<4>().0.to_vec(),
                },
            );
            s.painted.clear();
            s.composite_key = None;
        }
    }
    if s.body_ready || !s.strips.values().all(|h| images.contains(h)) {
        return;
    }
    let stage = project_root().join(LAB_STAGE);
    for (key, clip) in &s.rig.clips {
        let Some(img) = s.strips.get(key).and_then(|h| images.get(h)) else {
            continue;
        };
        let (w, data) = (img.width() as usize, img.data.clone().unwrap_or_default());
        match clip.cut_layers(&data, w, |p| std::fs::read_to_string(stage.join(p)).ok()) {
            Ok(cut) => {
                for set in cut {
                    println!(
                        "{TAG} body {key} #{}: {} texels differ",
                        set.source, set.differing
                    );
                    if set.differing != 0 {
                        s.failures
                            .push(format!("{key} #{}: the cut lost texels", set.source));
                    }
                    s.body.insert((key.clone(), set.source), set.layers);
                }
            }
            Err(e) => s.failures.push(format!("{key}: {e}")),
        }
    }
    s.body_ready = true;
}

// ------------------------------------------------------------ actions

fn new_design(s: &mut Studio) {
    s.record("new design");
    let mut k = 1;
    while s.entries.iter().any(|e| e.design.id == format!("item_{k}")) {
        k += 1;
    }
    let id = format!("item_{k}");
    let mut d = EquipmentDesign {
        version: DESIGN_VERSION,
        id: id.clone(),
        name: format!("New item {k}"),
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
    let p = blank_part(
        &d,
        "plate",
        PartSource::Polygon {
            points: quad_for(&d.space),
            smooth: 0,
        },
    );
    d.parts.push(p);
    let path = s.items_dir.join(format!("{id}.ron"));
    s.entries.push(Entry {
        path,
        design: d,
        saved: None,
        rev: 1,
    });
    s.selected = s.entries.len() - 1;
    s.part = 0;
    s.equipped.insert(RigSlot::Chest, s.selected);
    s.status = format!("new design {id}");
}

fn save(s: &mut Studio) {
    let msg = match s.entries.get_mut(s.selected) {
        Some(e) => {
            let written = e
                .design
                .to_ron()
                .map_err(|x| x.to_string())
                .and_then(|t| std::fs::write(&e.path, t).map_err(|x| x.to_string()));
            match written {
                Ok(()) => {
                    e.saved = Some(e.design.clone());
                    let name = e.path.file_name().and_then(|n| n.to_str()).unwrap_or("?");
                    format!("saved {name}")
                }
                Err(x) => format!("save failed: {x}"),
            }
        }
        None => "nothing to save".into(),
    };
    println!("{TAG} {msg}");
    s.status = msg;
}

fn save_as(s: &mut Studio) {
    let Some(d) = s.design().cloned() else {
        return;
    };
    let mut id = format!("{}_copy", d.id);
    let mut k = 2;
    while s.entries.iter().any(|e| e.design.id == id) {
        id = format!("{}_copy{k}", d.id);
        k += 1;
    }
    s.record("save as");
    let mut copy = d;
    copy.id = id.clone();
    copy.name = format!("{} (copy)", copy.name);
    let path = s.items_dir.join(format!("{id}.ron"));
    let ok = copy
        .to_ron()
        .ok()
        .is_some_and(|t| std::fs::write(&path, t).is_ok());
    s.entries.push(Entry {
        path,
        saved: ok.then(|| copy.clone()),
        design: copy,
        rev: 1,
    });
    s.selected = s.entries.len() - 1;
    s.status = if ok {
        format!("saved as {id}.ron")
    } else {
        "save as failed".into()
    };
    println!("{TAG} {}", s.status);
}

fn revert(s: &mut Studio) {
    if s.any_dirty() && s.revert_armed <= 0.0 {
        s.revert_armed = CONFIRM_SECONDS;
        s.status = "unsaved changes: Ctrl+R again to discard them".into();
        return;
    }
    s.record("revert to disk");
    s.entries = load_designs(&s.items_dir);
    s.clamp_selection();
    s.revert_armed = 0.0;
    s.status = format!("reverted {} designs from disk", s.entries.len());
}

/// The selected part's material, mutable.
fn edit_material(s: &mut Studio) -> Option<&mut PaintMaterial> {
    let part = s.part;
    let d = s.edit()?;
    let id = d.parts.get(part)?.material.clone();
    d.materials.iter_mut().find(|m| m.id == id)
}

fn apply(a: &Action, s: &mut Studio, server: &AssetServer) {
    match a {
        Action::New => new_design(s),
        Action::Open => {
            let mut added = 0;
            for e in load_designs(&s.items_dir) {
                if !s.entries.iter().any(|x| x.path == e.path) {
                    s.entries.push(e);
                    added += 1;
                }
            }
            s.status = format!("library rescanned: {added} new designs");
        }
        Action::Save => save(s),
        Action::SaveAs => save_as(s),
        Action::Import => import(s, server),
        Action::Undo => {
            let cur = s.snapshot();
            s.status = match s.history.undo(&cur) {
                Some(e) => {
                    s.restore(e.state);
                    format!("undo: {}", e.label)
                }
                None => "nothing to undo".into(),
            };
        }
        Action::Redo => {
            let cur = s.snapshot();
            s.status = match s.history.redo(&cur) {
                Some(e) => {
                    s.restore(e.state);
                    format!("redo: {}", e.label)
                }
                None => "nothing to redo".into(),
            };
        }
        Action::Category(c) => s.category = *c,
        Action::SelectDesign(i) => {
            if *i < s.entries.len() {
                s.selected = *i;
                s.part = 0;
                s.vertex = 0;
                s.target = Target::Shape;
            }
        }
        Action::ToggleEquip(i) => {
            if let Some(slot) = s.entries.get(*i).map(|e| e.design.slot) {
                if s.equipped.get(&slot) == Some(i) {
                    s.equipped.remove(&slot);
                    s.status = format!("unequipped {slot:?}");
                } else {
                    s.equipped.insert(slot, *i);
                    s.status = format!("equipped {slot:?}");
                }
            }
        }
        Action::SelectPart(i) => {
            s.part = *i;
            s.vertex = 0;
            s.target = Target::Shape;
            s.clamp_selection();
        }
        Action::AddPart | Action::AddPixels => add_part(s, *a == Action::AddPixels),
        Action::Duplicate => {
            s.record("duplicate");
            let part = s.part;
            if let Some(c) = s.edit().map(|d| d.duplicate_subtree(part)) {
                s.part = c;
            }
        }
        Action::Group => {
            s.record("group");
            let part = s.part;
            if let Some(g) = s.edit().map(|d| d.group(&[part], "group")) {
                s.part = g;
            }
        }
        Action::Remove => {
            let keeps = s
                .design()
                .is_some_and(|d| d.parts.len() > d.subtree(s.part).len());
            if keeps {
                s.record("remove part");
                let part = s.part;
                if let Some(d) = s.edit() {
                    d.remove_subtree(part);
                }
                s.clamp_selection();
            } else {
                s.status = "a design keeps at least one part".into();
            }
        }
        Action::ToggleHide => {
            s.record("hide part");
            if let Some(p) = s.edit_part() {
                p.visible = !p.visible;
            }
        }
        Action::ParentNext => {
            let Some(d) = s.design() else { return };
            let sub = d.subtree(s.part);
            let mut cands: Vec<Option<usize>> = vec![None];
            cands.extend((0..d.parts.len()).filter(|i| !sub.contains(i)).map(Some));
            let cur = d.parent_of(s.part);
            let next = cands[cands
                .iter()
                .position(|c| *c == cur)
                .map_or(0, |i| (i + 1) % cands.len())];
            set_parent(s, next);
        }
        Action::ParentTo(p) => set_parent(s, *p),
        Action::KeepGlobal => s.keep_global = !s.keep_global,
        Action::Nudge(field, delta) => nudge(s, *field, *delta),
        Action::DesignSpace => {
            s.record("design space");
            if let Some(d) = s.edit() {
                d.space = next_in(&SPACES, d.space);
            }
        }
        Action::PartBind => {
            if s.design().is_some_and(|d| d.parent_of(s.part).is_some()) {
                s.status = "only a root part binds to another space".into();
                return;
            }
            s.record("part binding");
            if let Some(p) = s.edit_part() {
                p.bind = cycle_opt(&SPACES, p.bind);
            }
        }
        Action::PartDepth => {
            s.record("part depth");
            if let Some(p) = s.edit_part() {
                p.depth = cycle_opt(&GEAR_DEPTHS, p.depth);
            }
        }
        Action::DesignDepth => {
            s.record("design depth");
            if let Some(d) = s.edit() {
                d.depth = next_in(&GEAR_DEPTHS, d.depth);
            }
        }
        Action::MaterialNext => {
            s.record("material");
            let part = s.part;
            if let Some(d) = s.edit() {
                let ids: Vec<String> = d.materials.iter().map(|m| m.id.clone()).collect();
                if let Some(p) = d.parts.get_mut(part)
                    && !ids.is_empty()
                {
                    let i = ids
                        .iter()
                        .position(|m| *m == p.material)
                        .map_or(0, |i| (i + 1) % ids.len());
                    p.material = ids[i].clone();
                }
            }
        }
        Action::NewMaterial => {
            s.record("new material");
            let part = s.part;
            if let Some(d) = s.edit() {
                let base = d
                    .parts
                    .get(part)
                    .and_then(|p| d.material(&p.material))
                    .cloned()
                    .unwrap_or_else(steel);
                let mut k = 1;
                while d.material(&format!("material_{k}")).is_some() {
                    k += 1;
                }
                let id = format!("material_{k}");
                d.materials.push(PaintMaterial {
                    id: id.clone(),
                    ..base
                });
                if let Some(p) = d.parts.get_mut(part) {
                    p.material = id.clone();
                }
                s.status = format!("new material {id} (copy of the part's material)");
            }
        }
        Action::FinishNext => {
            s.record("finish");
            if let Some(m) = edit_material(s) {
                m.finish = next_in(&FINISHES, m.finish);
            }
        }
        Action::RampSlot(i) => s.ramp_slot = (*i).min(5),
        Action::Channel(ch, dv) => {
            s.record("colour");
            let slot = s.ramp_slot;
            if let Some(m) = edit_material(s) {
                let c = &mut m.ramp[slot];
                let shift = |v: u8| (i16::from(v) + dv).clamp(0, 255) as u8;
                if *ch < 3 {
                    c[*ch] = shift(c[*ch]);
                } else {
                    for v in c.iter_mut() {
                        *v = shift(*v);
                    }
                }
            }
        }
        Action::OutlineNext => {
            s.record("outline");
            if let Some(p) = s.edit_part() {
                p.outline = next_in(&OUTLINES, p.outline);
            }
        }
        Action::AddRegion => {
            let Some(points) = polygon_points(s.part()) else {
                s.status = "regions belong to polygon parts".into();
                return;
            };
            s.record("add region");
            let ((x0, y0), (x1, y1)) = bbox(&points);
            let (cx, cy) = ((x0 + x1) * 0.5, (y0 + y1) * 0.5);
            let (hw, hh) = ((x1 - x0) * 0.25, (y1 - y0) * 0.2);
            if let Some(p) = s.edit_part() {
                p.regions.push(Region {
                    points: vec![
                        (cx - hw, cy - hh),
                        (cx + hw, cy - hh),
                        (cx + hw, cy + hh),
                        (cx - hw, cy + hh),
                    ],
                    material: None,
                    tone: -1,
                });
                let k = p.regions.len() - 1;
                s.target = Target::Region(k);
            }
        }
        Action::DeleteRegion => {
            s.record("delete region");
            let target = s.target;
            if let Some(p) = s.edit_part() {
                match target {
                    Target::Region(k) if k < p.regions.len() => {
                        p.regions.remove(k);
                    }
                    _ => {
                        p.regions.pop();
                    }
                }
            }
            s.target = Target::Shape;
        }
        Action::TargetNext => {
            let n = s.part().map_or(0, |p| p.regions.len());
            s.target = match s.target {
                Target::Shape if n > 0 => Target::Region(0),
                Target::Region(k) if k + 1 < n => Target::Region(k + 1),
                _ => Target::Shape,
            };
            s.vertex = 0;
        }
        Action::AddRivet | Action::AddStitch => {
            let Some(points) = polygon_points(s.part()) else {
                s.status = "ornaments belong to polygon parts".into();
                return;
            };
            s.record("add ornament");
            let ((x0, y0), (x1, y1)) = bbox(&points);
            let (cx, cy) = ((x0 + x1) * 0.5, (y0 + y1) * 0.5);
            let stitch = *a == Action::AddStitch;
            if let Some(p) = s.edit_part() {
                p.ornaments.push(if stitch {
                    let inset = (x1 - x0) * 0.1;
                    Ornament::Line {
                        points: vec![(x0 + inset, cy), (x1 - inset, cy)],
                        tone: 5,
                        dashed: true,
                    }
                } else {
                    Ornament::Rivet { at: (cx, cy) }
                });
            }
        }
        Action::DeleteOrnament => {
            s.record("delete ornament");
            if let Some(p) = s.edit_part() {
                p.ornaments.pop();
            }
        }
        Action::ClipsNext => {
            s.record("visibility");
            if let Some(p) = s.edit_part() {
                p.visibility.clips = match p.visibility.clips.first().map(String::as_str) {
                    None => vec!["idle".into()],
                    Some("idle") => vec!["run".into()],
                    _ => Vec::new(),
                };
            }
        }
        Action::TurnNext => {
            s.record("perspective");
            if let Some(p) = s.edit_part() {
                p.visibility.turn = match p.visibility.turn {
                    None => Some((0.0, 0.5)),
                    Some((lo, _)) if lo < 0.25 => Some((0.5, 1.0)),
                    _ => None,
                };
            }
        }
        Action::HidesNext => {
            s.record("hides");
            if let Some(p) = s.edit_part() {
                let i = HIDE_OPTIONS
                    .iter()
                    .position(|o| *o == p.hides.as_slice())
                    .map_or(1, |i| (i + 1) % HIDE_OPTIONS.len());
                p.hides = HIDE_OPTIONS[i].to_vec();
            }
        }
        Action::SetMode(m) => s.mode = *m,
        Action::SetTool(t) => s.tool = *t,
        Action::PaletteIndex(i) => s.palette_index = *i,
        Action::Frame(f) => {
            s.playing = false;
            s.elapsed = frame_start(s, *f);
        }
        Action::Clip(c) => {
            if s.clip != *c {
                s.clip = (*c).to_owned();
                s.elapsed = frame_start(s, 0);
            }
        }
        Action::Play => s.playing = !s.playing,
        Action::Step(d) => {
            s.playing = false;
            let n = s.clip().map_or(1, |c| c.frames.len()) as i32;
            let f = (s.frame as i32 + d).rem_euclid(n.max(1)) as usize;
            s.elapsed = frame_start(s, f);
        }
        Action::Speed(d) => {
            s.speed = (s.speed as i32 + d).clamp(0, SPEEDS.len() as i32 - 1) as usize;
        }
        Action::Mirror => s.flip = !s.flip,
        Action::Zoom(d) => {
            s.zoom = (s.zoom as i32 + d).clamp(ZOOM_RANGE.0 as i32, ZOOM_RANGE.1 as i32) as u32;
        }
        Action::Grid => s.grid = !s.grid,
        Action::Guides => s.guides = !s.guides,
        Action::Isolated => s.isolated = !s.isolated,
        Action::Unequipped => s.unequipped = !s.unequipped,
        Action::Skeleton => s.skeleton = !s.skeleton,
    }
}

fn import(s: &mut Studio, server: &AssetServer) {
    if s.imports.is_empty() {
        s.status = "no PNG in import/".into();
        return;
    }
    let path = s.imports[s.import_cursor % s.imports.len()].clone();
    s.import_cursor += 1;
    s.load_image(&path, server);
    s.record("import image");
    let parent = s
        .part()
        .filter(|p| matches!(p.source, PartSource::Group))
        .map(|p| p.id.clone());
    let space = s
        .design()
        .map_or(DesignSpace::Torso, |d| d.effective_space(s.part));
    if let Some(d) = s.edit() {
        let mut p = blank_part(
            d,
            "image",
            PartSource::Image {
                path: path.clone(),
                pivot: (2.0, 2.0),
            },
        );
        p.outline = Outline::None;
        p.parent = parent;
        p.transform.position = centre_for(&space);
        d.parts.push(p);
        s.part = d.parts.len() - 1;
    }
    s.status = format!("imported {path}");
}

fn add_part(s: &mut Studio, pixels: bool) {
    s.record(if pixels { "add pixel part" } else { "add part" });
    let parent = s
        .part()
        .filter(|p| matches!(p.source, PartSource::Group))
        .map(|p| p.id.clone());
    let space = s
        .design()
        .map_or(DesignSpace::Torso, |d| d.effective_space(s.part));
    let ramp = s
        .part()
        .and_then(|p| s.design()?.material(&p.material).map(|m| m.ramp))
        .unwrap_or(steel().ramp);
    if let Some(d) = s.edit() {
        let mut p = if pixels {
            let size = (12, 10);
            let mut p = blank_part(
                d,
                "pixels",
                PartSource::Pixels {
                    size,
                    pivot: (size.0 as f32 * 0.5, size.1 as f32 * 0.5),
                    palette: ramp.iter().map(|c| [c[0], c[1], c[2], 255]).collect(),
                    rows: vec![".".repeat(size.0 as usize); size.1 as usize],
                },
            );
            p.outline = Outline::None;
            p
        } else {
            blank_part(
                d,
                "part",
                PartSource::Polygon {
                    points: quad_for(&space),
                    smooth: 0,
                },
            )
        };
        if pixels {
            p.transform.position = centre_for(&space);
        }
        p.parent = parent;
        d.parts.push(p);
        s.part = d.parts.len() - 1;
    }
    if pixels {
        s.mode = Mode::Paint;
        s.tool = Tool::Brush;
        s.palette_index = 3;
    }
}

fn set_parent(s: &mut Studio, parent: Option<usize>) {
    s.record("change parent");
    let (part, keep) = (s.part, s.keep_global);
    let result = s.edit().map(|d| d.reparent(part, parent, keep));
    let name = parent
        .and_then(|p| s.design().map(|d| d.parts[p].id.clone()))
        .unwrap_or_else(|| "none".into());
    s.status = match result {
        Some(Ok(())) => format!(
            "parent: {name} ({} kept)",
            if keep { "global" } else { "local" }
        ),
        Some(Err(e)) => {
            // Nothing changed: drop the undo step just recorded.
            let cur = s.snapshot();
            let _ = s.history.undo(&cur);
            format!("refused: {e}")
        }
        None => String::new(),
    };
}

fn nudge(s: &mut Studio, field: Field, delta: f32) {
    let (tu, tv) = s.texel();
    let (dx, dy) = if s.part_in_torso() {
        (tu, tv)
    } else {
        (1.0, 1.0)
    };
    s.record("transform");
    let Some(p) = s.edit_part() else { return };
    let t = &mut p.transform;
    match field {
        Field::PosX => t.position.0 += delta * dx,
        Field::PosY => t.position.1 += delta * dy,
        Field::Rot => t.rotation_deg += delta,
        Field::ScaleX => t.scale.0 *= 1.0 + delta,
        Field::ScaleY => t.scale.1 *= 1.0 + delta,
        Field::PivotX | Field::PivotY => {
            // The geometry stays: the position follows the pivot.
            let old = *t;
            let pivot = if field == Field::PivotX {
                (old.pivot.0 + delta * dx, old.pivot.1)
            } else {
                (old.pivot.0, old.pivot.1 + delta * dy)
            };
            t.position = old.apply(pivot);
            t.pivot = pivot;
        }
        Field::Layer => p.layer += delta as i32,
        Field::Bevel => p.bevel = (p.bevel + delta).max(0.0),
        Field::Smooth => {
            if let PartSource::Polygon { smooth, .. } = &mut p.source {
                *smooth = (i32::from(*smooth) + delta as i32).clamp(0, 3) as u8;
            }
        }
    }
}

fn frame_start(s: &Studio, frame: usize) -> f32 {
    // A hair past the frame start, so float sums land inside it.
    s.clip().map_or(0.0, |c| {
        c.frames.iter().take(frame).map(|f| f.duration).sum::<f32>()
    }) + 1e-4
}

// ------------------------------------------------------------ input

fn ctrl(i: &ButtonInput<KeyCode>) -> bool {
    i.pressed(KeyCode::ControlLeft) || i.pressed(KeyCode::ControlRight)
}

fn shift(i: &ButtonInput<KeyCode>) -> bool {
    i.pressed(KeyCode::ShiftLeft) || i.pressed(KeyCode::ShiftRight)
}

fn keys(input: Res<ButtonInput<KeyCode>>, mut res: ResMut<Studio>, server: Res<AssetServer>) {
    let s = &mut *res;
    let p = |k| input.just_pressed(k);
    let big = if shift(&input) { 4.0 } else { 1.0 };
    let run = |s: &mut Studio, a: Action| apply(&a, s, &server);
    if ctrl(&input) {
        if (p(KeyCode::KeyZ) && shift(&input)) || p(KeyCode::KeyY) {
            run(s, Action::Redo);
        } else if p(KeyCode::KeyZ) {
            run(s, Action::Undo);
        }
        let chords = [
            (KeyCode::KeyN, Action::New),
            (KeyCode::KeyO, Action::Open),
            (KeyCode::KeyD, Action::Duplicate),
            (KeyCode::KeyG, Action::Group),
        ];
        for (k, a) in chords {
            if p(k) {
                run(s, a);
            }
        }
        if p(KeyCode::KeyS) {
            run(
                s,
                if shift(&input) {
                    Action::SaveAs
                } else {
                    Action::Save
                },
            );
        }
        if p(KeyCode::KeyR) {
            revert(s);
        }
        return;
    }
    let taps = [
        (KeyCode::Space, Action::Play),
        (KeyCode::Period, Action::Step(1)),
        (KeyCode::Comma, Action::Step(-1)),
        (KeyCode::KeyF, Action::Mirror),
        (KeyCode::KeyZ, Action::Zoom(-1)),
        (KeyCode::KeyX, Action::Zoom(1)),
        (KeyCode::KeyO, Action::Isolated),
        (KeyCode::KeyU, Action::Unequipped),
        (KeyCode::KeyG, Action::Guides),
        (KeyCode::KeyK, Action::Skeleton),
        (KeyCode::KeyN, Action::AddPart),
        (KeyCode::KeyI, Action::Import),
        (KeyCode::Delete, Action::Remove),
        (KeyCode::KeyH, Action::ToggleHide),
        (KeyCode::BracketLeft, Action::Nudge(Field::Layer, -1.0)),
        (KeyCode::BracketRight, Action::Nudge(Field::Layer, 1.0)),
    ];
    for (k, a) in taps {
        if p(k) {
            run(s, a);
        }
    }
    if p(KeyCode::KeyT) {
        let c = if s.clip == "run" { "idle" } else { "run" };
        run(s, Action::Clip(c));
    }
    let n = s.entries.len().max(1);
    if p(KeyCode::PageDown) {
        let i = (s.selected + 1) % n;
        run(s, Action::SelectDesign(i));
    }
    if p(KeyCode::PageUp) {
        let i = (s.selected + n - 1) % n;
        run(s, Action::SelectDesign(i));
    }
    if p(KeyCode::Enter) {
        let i = s.selected;
        run(s, Action::ToggleEquip(i));
    }
    let parts = s.design().map_or(1, |d| d.parts.len()).max(1);
    if p(KeyCode::Tab) {
        let i = if shift(&input) {
            (s.part + parts - 1) % parts
        } else {
            (s.part + 1) % parts
        };
        run(s, Action::SelectPart(i));
    }
    if p(KeyCode::KeyM) {
        s.mode = s.mode.next();
    }
    let arrow = [
        (KeyCode::ArrowLeft, (-1.0, 0.0)),
        (KeyCode::ArrowRight, (1.0, 0.0)),
        (KeyCode::ArrowUp, (0.0, 1.0)),
        (KeyCode::ArrowDown, (0.0, -1.0)),
    ]
    .into_iter()
    .find(|(k, _)| p(*k))
    .map(|(_, d)| (d.0 * big, d.1 * big));
    let rot = if p(KeyCode::KeyQ) {
        5.0
    } else if p(KeyCode::KeyE) {
        -5.0
    } else {
        0.0
    };
    let scale = if p(KeyCode::Equal) {
        0.05
    } else if p(KeyCode::Minus) {
        -0.05
    } else {
        0.0
    };
    // Screen up is -v in the torso cage.
    let up = if s.part_in_torso() { -1.0 } else { 1.0 };
    match s.mode {
        Mode::Part => {
            if let Some(d) = arrow {
                // One arrow press is one undo step.
                s.record("move part");
                s.history.begin("move part", &s.snapshot());
                run(s, Action::Nudge(Field::PosX, d.0));
                run(s, Action::Nudge(Field::PosY, d.1 * up));
                s.history.end();
            }
            if rot != 0.0 {
                run(s, Action::Nudge(Field::Rot, rot));
            }
            if scale != 0.0 {
                s.history.begin("scale part", &s.snapshot());
                run(s, Action::Nudge(Field::ScaleX, scale));
                run(s, Action::Nudge(Field::ScaleY, scale));
                s.history.end();
            }
        }
        Mode::Pivot => {
            if let Some(d) = arrow {
                s.history.begin("move pivot", &s.snapshot());
                run(s, Action::Nudge(Field::PivotX, d.0));
                run(s, Action::Nudge(Field::PivotY, d.1 * up));
                s.history.end();
            }
        }
        Mode::Mount => {
            if arrow.is_some() || rot != 0.0 || scale != 0.0 {
                s.record("mount");
                let (tu, tv) = s.texel();
                if let Some(d) = s.edit() {
                    if let Some(a) = arrow {
                        let torso = d.space == DesignSpace::Torso;
                        let (dx, dy) = if torso { (a.0 * tu, -a.1 * tv) } else { a };
                        d.mount.position.0 += dx;
                        d.mount.position.1 += dy;
                    }
                    d.mount.rotation_deg += rot;
                    d.mount.scale.0 *= 1.0 + scale;
                    d.mount.scale.1 *= 1.0 + scale;
                }
            }
        }
        Mode::Vertex => vertex_keys(s, &p, arrow, up),
        Mode::Colour | Mode::Paint => {}
    }
}

fn vertex_keys(s: &mut Studio, p: &dyn Fn(KeyCode) -> bool, arrow: Option<(f32, f32)>, up: f32) {
    let count = s.target_points().map_or(0, |v| v.len());
    if count == 0 {
        return;
    }
    if p(KeyCode::KeyB) {
        s.vertex = (s.vertex + 1) % count;
    }
    if p(KeyCode::KeyV) {
        s.vertex = (s.vertex + count - 1) % count;
    }
    let v = s.vertex.min(count - 1);
    if let Some(a) = arrow {
        s.record("move vertex");
        let (tu, tv) = if s.part_in_torso() {
            s.texel()
        } else {
            (1.0, 1.0)
        };
        if let Some(pts) = s.target_points_mut() {
            pts[v].0 += a.0 * tu;
            pts[v].1 += a.1 * up * tv;
        }
    }
    if p(KeyCode::KeyY) {
        s.record("insert vertex");
        if let Some(pts) = s.target_points_mut() {
            let (a, b) = (pts[v], pts[(v + 1) % pts.len()]);
            pts.insert(v + 1, ((a.0 + b.0) * 0.5, (a.1 + b.1) * 0.5));
        }
        s.vertex = v + 1;
    }
    if p(KeyCode::Backspace) && count > 3 {
        s.record("delete vertex");
        if let Some(pts) = s.target_points_mut() {
            pts.remove(v);
        }
        s.vertex = v.saturating_sub(1);
    }
}

/// Mouse on the panels (buttons) and on the canvas. The tour drives the
/// same code through a synthetic pointer.
fn mouse(
    buttons: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    scroll: Res<AccumulatedMouseScroll>,
    window: Single<&Window, With<PrimaryWindow>>,
    ui: Query<(&UiButton, &ComputedNode, &UiGlobalTransform)>,
    mut res: ResMut<Studio>,
    server: Res<AssetServer>,
) {
    let s = &mut *res;
    let (pos, left, right) = match s.pointer {
        Some(p) => (Some(p.pos), p.left, p.right),
        None => (
            window.cursor_position(),
            buttons.pressed(MouseButton::Left),
            buttons.pressed(MouseButton::Right),
        ),
    };
    let left_down = left && !s.last_buttons.0;
    let right_down = right && !s.last_buttons.1;
    s.last_buttons = (left, right);
    if s.dragging.is_some() && !left && !right {
        s.end_drag();
    }
    let Some(pos) = pos else {
        return;
    };
    if left_down {
        let phys = pos * window.scale_factor();
        let hit = ui
            .iter()
            .find(|(_, node, t)| node.contains_point(**t, phys))
            .map(|(b, _, _)| b.0.clone());
        if let Some(a) = hit {
            apply(&a, s, &server);
            return;
        }
    }
    let in_canvas =
        pos.x > LEFT_W && pos.x < WIN.0 - RIGHT_W && pos.y > TOP_H && pos.y < WIN.1 - BOTTOM_H;
    if in_canvas && s.pointer.is_none() && scroll.delta.y != 0.0 {
        let up = scroll.delta.y > 0.0;
        if ctrl(&keyboard) {
            let d = if up { 0.05 } else { -0.05 };
            s.history.begin("scale part", &s.snapshot());
            apply(&Action::Nudge(Field::ScaleX, d), s, &server);
            apply(&Action::Nudge(Field::ScaleY, d), s, &server);
            s.history.end();
        } else {
            apply(&Action::Zoom(if up { 1 } else { -1 }), s, &server);
        }
    }
    if in_canvas && left_down {
        canvas_press(s, pos);
    }
    if in_canvas && right_down && s.mode == Mode::Part {
        s.begin_drag("rotate part", MouseButton::Right, pos);
    }
    let Some((button, last)) = s.dragging else {
        return;
    };
    match (s.mode, button) {
        (Mode::Part, MouseButton::Left) => {
            // The part follows the cursor delta in its parent's space.
            if let (Some(a), Some(b)) = (s.screen_to_parent(last), s.screen_to_parent(pos)) {
                if let Some(p) = s.edit_part() {
                    p.transform.position.0 += b.0 - a.0;
                    p.transform.position.1 += b.1 - a.1;
                }
                s.dragging = Some((button, pos));
            }
        }
        (Mode::Part, MouseButton::Right) => {
            let dx = pos.x - last.x;
            if dx.abs() >= 1.0 {
                if let Some(p) = s.edit_part() {
                    p.transform.rotation_deg -= dx * 0.5;
                }
                s.dragging = Some((button, pos));
            }
        }
        (Mode::Vertex, _) => {
            let v = s.vertex;
            if let Some(l) = s.screen_to_local(pos)
                && let Some(pts) = s.target_points_mut()
                && v < pts.len()
            {
                pts[v] = l;
            }
        }
        (Mode::Pivot, _) => {
            if let Some(l) = s.screen_to_local(pos)
                && let Some(p) = s.edit_part()
            {
                let t = p.transform;
                p.transform.position = t.apply(l);
                p.transform.pivot = l;
            }
        }
        (Mode::Paint, _) => paint_at(s, pos, false),
        _ => {}
    }
}

fn canvas_press(s: &mut Studio, pos: Vec2) {
    match s.mode {
        Mode::Part => {
            if let Some(t) = s.screen_texel(pos)
                && let Some(Some((d, p))) = s.pick.get(t).copied()
            {
                s.selected = d;
                s.part = p;
                s.target = Target::Shape;
                let id = s.part().map(|p| p.id.clone()).unwrap_or_default();
                s.status = format!("picked {id}");
            }
            s.begin_drag("drag part", MouseButton::Left, pos);
        }
        Mode::Vertex => {
            let Some(points) = s.target_points() else {
                return;
            };
            let best = points
                .iter()
                .enumerate()
                .filter_map(|(i, p)| Some((i, s.local_to_screen(*p)?.distance(pos))))
                .min_by(|a, b| a.1.total_cmp(&b.1));
            if let Some((i, d)) = best
                && d < GRAB_RADIUS
            {
                s.vertex = i;
                s.begin_drag("drag vertex", MouseButton::Left, pos);
            }
        }
        Mode::Pivot => s.begin_drag("place pivot", MouseButton::Left, pos),
        Mode::Paint => {
            s.begin_drag("paint", MouseButton::Left, pos);
            paint_at(s, pos, true);
        }
        Mode::Mount | Mode::Colour => {}
    }
}

/// Brush, eraser, fill or picker on the selected pixel part.
fn paint_at(s: &mut Studio, pos: Vec2, first: bool) {
    let cell = match (s.design(), s.part_map(), s.clip()) {
        (Some(d), Some(map), Some(c)) => {
            pixel_cell_at(d, s.part, map, c.geometry(), s.screen_to_rig(pos))
        }
        _ => None,
    };
    let Some((x, y)) = cell else {
        return;
    };
    let (tool, index) = (s.tool, s.palette_index);
    if tool == Tool::Picker {
        if let Some(PartSource::Pixels { rows, .. }) = s.part().map(|p| &p.source)
            && let Some(i) = rows
                .get(y)
                .and_then(|r| r.chars().nth(x))
                .and_then(pixel_index)
        {
            s.palette_index = i;
        }
        return;
    }
    if tool == Tool::Fill && !first {
        return;
    }
    let Some(PartSource::Pixels { rows, size, .. }) = s.edit_part().map(|p| &mut p.source) else {
        return;
    };
    let mut grid: Vec<Vec<char>> = rows.iter().map(|r| r.chars().collect()).collect();
    let (w, h) = (size.0 as usize, size.1 as usize);
    if grid.len() != h || grid.iter().any(|r| r.len() != w) {
        return;
    }
    let ink = if tool == Tool::Eraser {
        '.'
    } else {
        pixel_char(index)
    };
    if tool == Tool::Fill {
        let from = grid[y][x];
        if from != ink {
            let mut stack = vec![(x, y)];
            while let Some((cx, cy)) = stack.pop() {
                if grid[cy][cx] != from {
                    continue;
                }
                grid[cy][cx] = ink;
                if cx > 0 {
                    stack.push((cx - 1, cy));
                }
                if cy > 0 {
                    stack.push((cx, cy - 1));
                }
                if cx + 1 < w {
                    stack.push((cx + 1, cy));
                }
                if cy + 1 < h {
                    stack.push((cx, cy + 1));
                }
            }
        }
    } else {
        grid[y][x] = ink;
    }
    *rows = grid.into_iter().map(|r| r.into_iter().collect()).collect();
}

/// Closing with unsaved changes asks twice.
fn guard_close(
    mut requests: MessageReader<WindowCloseRequested>,
    mut s: ResMut<Studio>,
    time: Res<Time>,
    mut exit: MessageWriter<AppExit>,
) {
    let dt = time.delta_secs();
    s.close_armed = (s.close_armed - dt).max(0.0);
    s.revert_armed = (s.revert_armed - dt).max(0.0);
    for _ in requests.read() {
        if s.any_dirty() && s.close_armed <= 0.0 {
            s.close_armed = CONFIRM_SECONDS;
            let names: Vec<String> = s
                .entries
                .iter()
                .filter(|e| e.dirty())
                .map(|e| e.design.id.clone())
                .collect();
            s.status = format!(
                "UNSAVED: {}. Close again to discard, or Ctrl+S",
                names.join(", ")
            );
        } else {
            exit.write(AppExit::Success);
        }
    }
    if s.exit_now {
        exit.write(AppExit::Success);
    }
}

fn advance(time: Res<Time>, mut s: ResMut<Studio>) {
    if s.playing {
        s.elapsed += clamp_delta(time.delta_secs()) * SPEEDS[s.speed];
    }
    let durations = s.clip().map(RigClip::durations).unwrap_or_default();
    s.frame = loop_frame(&durations, s.elapsed);
}

// ------------------------------------------------------------ render

/// Builds the one final composite (and the pick map) when anything changed.
fn render(mut res: ResMut<Studio>, mut images: ResMut<Assets<Image>>) {
    let s = &mut *res;
    if !s.body_ready {
        return;
    }
    let Some(c) = s.clip().cloned() else {
        return;
    };
    let Some(f) = c.frames.get(s.frame).copied() else {
        return;
    };
    let worn: Vec<usize> = if s.isolated {
        vec![s.selected]
    } else if s.unequipped {
        Vec::new()
    } else {
        s.equipped.values().copied().collect()
    };
    let revs: Vec<u64> = worn
        .iter()
        .map(|i| s.entries.get(*i).map_or(0, |e| e.rev))
        .collect();
    let key = format!(
        "{}|{}|{}|{worn:?}|{revs:?}|{}",
        s.clip,
        s.frame,
        s.isolated,
        s.decoded.len()
    );
    if s.composite_key.as_deref() == Some(key.as_str()) {
        return;
    }
    for &i in &worn {
        let Some(e) = s.entries.get(i) else { continue };
        let pk = (i, e.rev, s.clip.clone(), s.frame);
        if s.painted.contains_key(&pk) {
            continue;
        }
        let decoded = &s.decoded;
        let lookup = |p: &str| decoded.get(p);
        if let Some(r) = paint_on_frame(&s.rig, &s.rest, &e.design, &s.clip, s.frame, &lookup) {
            s.painted.insert(pk, r);
        }
    }
    let n = c.cell.0 as usize * c.cell.1 as usize;
    let empty: Vec<(BodyRegion, Vec<u8>)> = BodyRegion::ALL
        .iter()
        .map(|r| (*r, vec![0u8; n * 4]))
        .collect();
    let body = if s.isolated {
        &empty
    } else {
        s.body.get(&(s.clip.clone(), f.source)).unwrap_or(&empty)
    };
    let items: Vec<(usize, WornItem<'_>)> = worn
        .iter()
        .filter_map(|i| {
            let e = s.entries.get(*i)?;
            let (p, d) = s.painted.get(&(*i, e.rev, s.clip.clone(), s.frame))?;
            Some((
                *i,
                WornItem {
                    design: &e.design,
                    painted: p,
                    depth: *d,
                },
            ))
        })
        .collect();
    let worn_items: Vec<WornItem<'_>> = items
        .iter()
        .map(|(_, w)| WornItem {
            design: w.design,
            painted: w.painted,
            depth: w.depth,
        })
        .collect();
    let drop = if s.isolated { 0 } else { f.drop as usize };
    let composite = compose_frame(body, drop, c.cell.0 as usize, &worn_items);
    // Pick map: the top item part of each texel, unless a body region is in
    // front of it.
    let mut top: Vec<(Option<DepthSlot>, Option<(usize, usize)>)> = vec![(None, None); n];
    for (wi, it) in &items {
        for (depth, layer) in split_by_depth(it.design, it.painted, it.depth) {
            for t in 0..n.min(layer.len()) {
                if layer[t][3] > 0 && top[t].0.is_none_or(|d| depth >= d) {
                    top[t] = (Some(depth), it.painted.owner[t].map(|p| (*wi, p)));
                }
            }
        }
    }
    for (region, bytes) in body {
        for t in 0..n {
            if bytes.get(t * 4 + 3).is_some_and(|a| *a > 0)
                && top[t].0.is_some_and(|d| region.depth() > d)
                && composite[t][3] > 0
            {
                top[t] = (Some(region.depth()), None);
            }
        }
    }
    s.pick = top.into_iter().map(|(_, o)| o).collect();
    let art = PixelArt {
        width: c.cell.0,
        height: c.cell.1,
        rgba: composite.iter().flatten().copied().collect(),
        grip: (0.0, 0.0),
        density: 1.0,
    };
    let _ = images.insert(&s.composite, pixel_image(art));
    s.last_composite = composite;
    s.composite_key = Some(key);
    if s.painted.len() > 400 {
        s.painted.clear();
    }
}

fn sync_views(
    s: Res<Studio>,
    mut q: Query<(
        &ViewSprite,
        &mut Sprite,
        &mut Anchor,
        &mut Transform,
        &mut Visibility,
    )>,
) {
    let Some(c) = s.clip() else { return };
    let (ax, ay) = anchor_for(c.pivot, c.cell, s.flip);
    for (v, mut sprite, mut anchor, mut t, mut vis) in &mut q {
        if sprite.image != s.composite {
            sprite.image = s.composite.clone();
        }
        sprite.custom_size = Some(Vec2::new(c.cell.0 as f32, c.cell.1 as f32));
        sprite.flip_x = s.flip;
        anchor.set_if_neq(Anchor(Vec2::new(ax, ay)));
        if v.0 == 0 {
            t.scale = Vec3::splat(s.zoom as f32);
        }
        vis.set_if_neq(if s.composite_key.is_some() {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        });
    }
}

fn world(p: Vec2) -> Vec2 {
    Vec2::new(p.x - WIN.0 * 0.5, WIN.1 * 0.5 - p.y)
}

/// Pixel grid, skeleton, torso cage and the selected part's guides (all off
/// with "guides").
fn overlays(s: Res<Studio>, mut gizmos: Gizmos) {
    if !s.guides {
        return;
    }
    let Some(c) = s.clip() else { return };
    let g = c.geometry();
    if s.grid && s.zoom >= 3 {
        let grey = Color::srgba(1.0, 1.0, 1.0, 0.03);
        for x in 0..=g.width {
            let a = s.rig_to_screen(g.to_rig((x as f32, 0.0)));
            let b = s.rig_to_screen(g.to_rig((x as f32, g.height as f32)));
            gizmos.line_2d(world(a), world(b), grey);
        }
        for y in 0..=g.height {
            let a = s.rig_to_screen(g.to_rig((0.0, y as f32)));
            let b = s.rig_to_screen(g.to_rig((g.width as f32, y as f32)));
            gizmos.line_2d(world(a), world(b), grey);
        }
    }
    if s.skeleton
        && let Some(j) = c.frame_joints(s.frame)
    {
        let pose = forward(&fit(&j));
        for bone in Bone::ALL {
            let (a, b) = pose.segment(bone);
            gizmos.line_2d(
                world(s.rig_to_screen(a)),
                world(s.rig_to_screen(b)),
                Color::srgba(1.0, 1.0, 1.0, 0.7),
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
                world(s.rig_to_screen(q[i])),
                world(s.rig_to_screen(q[(i + 1) % 4])),
                Color::srgb(0.5, 1.0, 0.3),
            );
        }
    }
    let cyan = Color::srgb(0.0, 1.0, 1.0);
    let yellow = Color::srgb(1.0, 0.9, 0.2);
    let magenta = Color::srgb(1.0, 0.25, 1.0);
    let Some(part) = s.part().cloned() else {
        return;
    };
    if let PartSource::Polygon { points, smooth } = &part.source {
        let pts: Vec<Vec2> = chaikin(points, *smooth)
            .iter()
            .filter_map(|p| s.local_to_screen(*p))
            .map(world)
            .collect();
        for i in 0..pts.len() {
            gizmos.line_2d(pts[i], pts[(i + 1) % pts.len()], cyan);
        }
    }
    if s.mode == Mode::Vertex
        && let Some(points) = s.target_points()
    {
        for (i, p) in points.iter().enumerate() {
            if let Some(v) = s.local_to_screen(*p) {
                let sel = i == s.vertex;
                let (r, col) = if sel { (6.0, yellow) } else { (3.0, cyan) };
                gizmos.circle_2d(world(v), r, col);
            }
        }
    }
    if let Some(p) = s.local_to_screen(part.transform.pivot) {
        gizmos.cross_2d(Isometry2d::from_translation(world(p)), 9.0, magenta);
    }
    // The socket (mount origin) of the whole item.
    if let (Some(d), Some(map)) = (s.design(), s.part_map()) {
        let o = s.rig_to_screen(map.to_rig(d.mount.position));
        gizmos.circle_2d(world(o), 7.0, yellow);
    }
}

// ------------------------------------------------------------ panels

fn text(content: String, color: Color) -> impl Bundle {
    (
        Text::new(content),
        TextFont {
            font_size: FontSize::Px(FONT),
            ..default()
        },
        TextColor(color),
    )
}

fn label(p: &mut ChildSpawnerCommands<'_>, content: impl Into<String>, color: Color, width: f32) {
    p.spawn((
        Node {
            width: px(width),
            ..default()
        },
        children![text(content.into(), color)],
    ));
}

fn button(p: &mut ChildSpawnerCommands<'_>, content: impl Into<String>, action: Action, on: bool) {
    p.spawn((
        UiButton(action),
        Node {
            padding: UiRect::axes(px(5), px(2)),
            ..default()
        },
        BackgroundColor(if on { BUTTON_ON } else { BUTTON }),
        children![text(content.into(), INK)],
    ));
}

fn swatch(p: &mut ChildSpawnerCommands<'_>, c: [u8; 4], action: Action, on: bool) {
    p.spawn((
        UiButton(action),
        Node {
            width: px(22),
            height: px(16),
            border: UiRect::all(px(if on { 2 } else { 0 })),
            ..default()
        },
        BackgroundColor(Color::srgb_u8(c[0], c[1], c[2])),
        BorderColor::all(Color::WHITE),
    ));
}

fn break_row(p: &mut ChildSpawnerCommands<'_>) {
    p.spawn(Node {
        width: percent(100),
        height: px(1),
        ..default()
    });
}

/// A stamp of everything the panels show (they are rebuilt when it changes).
fn ui_stamp(s: &Studio) -> String {
    let revs: u64 = s.entries.iter().map(|e| e.rev).sum();
    let dirty = s.entries.iter().filter(|e| e.dirty()).count();
    format!(
        "{}|{}|{revs}|{dirty}|{}|{:?}|{:?}|{:?}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{:?}|{}|{}|{}|{:?}|{}|{}",
        s.entries.len(),
        s.selected,
        s.part,
        s.mode,
        s.tool,
        s.target,
        s.palette_index,
        s.ramp_slot,
        s.keep_global,
        s.clip,
        s.frame,
        s.playing,
        s.speed,
        s.zoom,
        s.grid,
        s.guides,
        s.isolated,
        s.unequipped,
        s.category,
        s.status,
        s.history.len(),
        s.history.can_redo(),
        s.equipped,
        s.flip,
        s.skeleton,
    )
}

fn panels(mut commands: Commands, mut res: ResMut<Studio>, roots: Query<(Entity, &PanelKind)>) {
    let stamp = ui_stamp(&res);
    if stamp == res.ui_key {
        return;
    }
    res.ui_key = stamp;
    let s = &*res;
    for (e, kind) in &roots {
        commands.entity(e).despawn_children();
        commands.entity(e).with_children(|p| match kind {
            PanelKind::Top => top_panel(p, s),
            PanelKind::Left => left_panel(p, s),
            PanelKind::Right => right_panel(p, s),
            PanelKind::Bottom => bottom_panel(p, s),
        });
    }
}

fn top_panel(p: &mut ChildSpawnerCommands<'_>, s: &Studio) {
    label(p, "EQUIPMENT STUDIO V2", INK, 130.0);
    button(p, "New", Action::New, false);
    button(p, "Open", Action::Open, false);
    let dirty = s.entries.get(s.selected).is_some_and(Entry::dirty);
    button(
        p,
        if dirty { "Save *" } else { "Save" },
        Action::Save,
        dirty,
    );
    button(p, "Save as", Action::SaveAs, false);
    button(p, "Import PNG", Action::Import, false);
    let undo = s
        .history
        .next_undo()
        .map_or("Undo".to_owned(), |l| format!("Undo: {l}"));
    button(p, undo, Action::Undo, false);
    button(p, "Redo", Action::Redo, s.history.can_redo());
    label(p, format!("  {}", s.status), DIM, 620.0);
}

/// Parts in depth-first order with their depth in the tree.
fn tree_order(d: &EquipmentDesign) -> Vec<(usize, usize)> {
    fn walk(d: &EquipmentDesign, i: usize, depth: usize, out: &mut Vec<(usize, usize)>) {
        out.push((i, depth));
        let mut c = d.children(i);
        c.sort_by_key(|j| -d.parts[*j].layer);
        for j in c {
            walk(d, j, depth + 1, out);
        }
    }
    let mut roots: Vec<usize> = (0..d.parts.len())
        .filter(|i| d.parent_of(*i).is_none())
        .collect();
    roots.sort_by_key(|i| -d.parts[*i].layer);
    let mut out = Vec::new();
    for r in roots {
        walk(d, r, 0, &mut out);
    }
    out
}

fn left_panel(p: &mut ChildSpawnerCommands<'_>, s: &Studio) {
    label(p, "LIBRARY", INK, 60.0);
    button(p, "all", Action::Category(None), s.category.is_none());
    for c in [
        RigSlot::Chest,
        RigSlot::Shoulder,
        RigSlot::Bracers,
        RigSlot::Belt,
        RigSlot::MainHand,
    ] {
        button(
            p,
            format!("{c:?}"),
            Action::Category(Some(c)),
            s.category == Some(c),
        );
    }
    break_row(p);
    for (i, e) in s.entries.iter().enumerate() {
        if s.category.is_some_and(|c| c != e.design.slot) {
            continue;
        }
        let worn = s.equipped.get(&e.design.slot) == Some(&i);
        button(
            p,
            if worn { "on" } else { "--" },
            Action::ToggleEquip(i),
            worn,
        );
        let name = format!("{}{}", e.design.id, if e.dirty() { " *" } else { "" });
        button(p, name, Action::SelectDesign(i), i == s.selected);
        break_row(p);
    }
    let Some(d) = s.design() else { return };
    label(p, format!("HIERARCHY  {}", d.id), INK, 230.0);
    let sub = d.subtree(s.part);
    for (i, depth) in tree_order(d).into_iter().take(20) {
        let part = &d.parts[i];
        let kind = match part.source {
            PartSource::Group => " [group]",
            PartSource::Polygon { .. } => "",
            PartSource::Image { .. } => " [png]",
            PartSource::Pixels { .. } => " [px]",
        };
        let hidden = if d.effective_visible(i) {
            ""
        } else {
            " (hidden)"
        };
        let can_parent = !sub.contains(&i);
        if can_parent {
            button(p, "p", Action::ParentTo(Some(i)), false);
        } else {
            label(p, "", DIM, 12.0);
        }
        let row = format!("{}{}{kind}{hidden}", "  ".repeat(depth), part.id);
        button(p, row, Action::SelectPart(i), i == s.part);
        break_row(p);
    }
    label(p, "p = make it the parent of the selected part", DIM, 240.0);
    button(p, "+part", Action::AddPart, false);
    button(p, "+pixels", Action::AddPixels, false);
    button(p, "dup", Action::Duplicate, false);
    button(p, "group", Action::Group, false);
    button(p, "remove", Action::Remove, false);
    button(p, "hide", Action::ToggleHide, false);
}

fn field(p: &mut ChildSpawnerCommands<'_>, name: &str, value: String, f: Field, step: f32) {
    label(p, format!("{name} {value}"), DIM, 78.0);
    button(p, "-", Action::Nudge(f, -step), false);
    button(p, "+", Action::Nudge(f, step), false);
}

fn right_panel(p: &mut ChildSpawnerCommands<'_>, s: &Studio) {
    let Some(d) = s.design() else { return };
    label(p, format!("DESIGN {} ({:?})", d.id, d.slot), INK, 290.0);
    label(p, format!("space {}", space_name(&d.space)), DIM, 150.0);
    button(p, "cycle", Action::DesignSpace, false);
    break_row(p);
    label(p, format!("depth {:?}", d.depth), DIM, 150.0);
    button(p, "cycle", Action::DesignDepth, false);
    break_row(p);
    let Some(part) = d.parts.get(s.part) else {
        return;
    };
    let kind = match &part.source {
        PartSource::Polygon { points, smooth } => {
            format!("polygon {} pts, curve {smooth}", points.len())
        }
        PartSource::Image { path, .. } => {
            format!("png {}", path.rsplit('/').next().unwrap_or(path))
        }
        PartSource::Group => "group".into(),
        PartSource::Pixels { size, .. } => format!("pixels {}x{}", size.0, size.1),
    };
    label(p, format!("PART {}  ({kind})", part.id), INK, 290.0);
    let parent = part.parent.clone().unwrap_or_else(|| "none".into());
    label(p, format!("parent {parent}"), DIM, 110.0);
    button(p, "next", Action::ParentNext, false);
    button(p, "none", Action::ParentTo(None), false);
    button(p, "keep global", Action::KeepGlobal, s.keep_global);
    break_row(p);
    let t = part.transform;
    field(p, "x", format!("{:.2}", t.position.0), Field::PosX, 1.0);
    field(p, "y", format!("{:.2}", t.position.1), Field::PosY, 1.0);
    break_row(p);
    field(p, "rot", format!("{:.0}", t.rotation_deg), Field::Rot, 5.0);
    field(p, "layer", format!("{}", part.layer), Field::Layer, 1.0);
    break_row(p);
    field(p, "sx", format!("{:.2}", t.scale.0), Field::ScaleX, 0.05);
    field(p, "sy", format!("{:.2}", t.scale.1), Field::ScaleY, 0.05);
    break_row(p);
    field(p, "pivx", format!("{:.2}", t.pivot.0), Field::PivotX, 1.0);
    field(p, "pivy", format!("{:.2}", t.pivot.1), Field::PivotY, 1.0);
    break_row(p);
    let bind = part
        .bind
        .map_or_else(|| "(design space)".into(), |b| space_name(&b));
    label(p, format!("bone bind {bind}"), DIM, 200.0);
    button(p, "cycle", Action::PartBind, false);
    break_row(p);
    let depth = part.depth.map_or_else(
        || format!("{:?} (inherited)", d.effective_depth(s.part, d.depth)),
        |x| format!("{x:?}"),
    );
    label(p, format!("depth {depth}"), DIM, 200.0);
    button(p, "cycle", Action::PartDepth, false);
    break_row(p);
    if let Some(m) = d.material(&part.material) {
        label(p, format!("material {} {:?}", m.id, m.finish), DIM, 140.0);
        button(p, "next", Action::MaterialNext, false);
        button(p, "new", Action::NewMaterial, false);
        button(p, "finish", Action::FinishNext, false);
        break_row(p);
        for (i, c) in m.ramp.iter().enumerate() {
            swatch(
                p,
                [c[0], c[1], c[2], 255],
                Action::RampSlot(i),
                i == s.ramp_slot,
            );
        }
        let c = m.ramp[s.ramp_slot];
        label(p, format!(" {} {} {}", c[0], c[1], c[2]), DIM, 90.0);
        break_row(p);
        for (i, name) in ["R", "G", "B", "all"].iter().enumerate() {
            button(p, format!("{name}-"), Action::Channel(i, -8), false);
            button(p, format!("{name}+"), Action::Channel(i, 8), false);
        }
        break_row(p);
    }
    label(p, format!("outline {:?}", part.outline), DIM, 100.0);
    button(p, "cycle", Action::OutlineNext, false);
    field(p, "bevel", format!("{:.1}", part.bevel), Field::Bevel, 0.5);
    break_row(p);
    if matches!(part.source, PartSource::Polygon { .. }) {
        field(p, "curve", String::new(), Field::Smooth, 1.0);
        break_row(p);
    }
    let target = match s.target {
        Target::Shape => "shape".to_owned(),
        Target::Region(k) => format!("region {k}"),
    };
    label(
        p,
        format!("regions {} (edit {target})", part.regions.len()),
        DIM,
        130.0,
    );
    button(p, "+", Action::AddRegion, false);
    button(p, "-", Action::DeleteRegion, false);
    button(p, "edit", Action::TargetNext, false);
    break_row(p);
    label(p, format!("ornaments {}", part.ornaments.len()), DIM, 90.0);
    button(p, "+rivet", Action::AddRivet, false);
    button(p, "+stitch", Action::AddStitch, false);
    button(p, "-last", Action::DeleteOrnament, false);
    break_row(p);
    let clips = if part.visibility.clips.is_empty() {
        "all clips".to_owned()
    } else {
        part.visibility.clips.join(",")
    };
    let turn = part
        .visibility
        .turn
        .map_or("any view".to_owned(), |(a, b)| {
            format!("turn {a:.1}-{b:.1}")
        });
    let shown = if part.visible { "" } else { "HIDDEN, " };
    label(p, format!("visible: {shown}{clips}, {turn}"), DIM, 170.0);
    button(p, "clips", Action::ClipsNext, false);
    button(p, "view", Action::TurnNext, false);
    break_row(p);
    label(p, format!("replaces {:?}", part.hides), DIM, 200.0);
    button(p, "cycle", Action::HidesNext, false);
    break_row(p);
    label(p, "MODE", INK, 34.0);
    for m in Mode::ALL {
        button(p, m.name(), Action::SetMode(m), s.mode == m);
    }
    if let PartSource::Pixels { palette, .. } = &part.source {
        break_row(p);
        let tools = [
            (Tool::Brush, "brush"),
            (Tool::Eraser, "eraser"),
            (Tool::Fill, "fill"),
            (Tool::Picker, "picker"),
        ];
        for (t, name) in tools {
            button(p, name, Action::SetTool(t), s.tool == t);
        }
        break_row(p);
        for (i, c) in palette.iter().enumerate() {
            swatch(p, *c, Action::PaletteIndex(i), i == s.palette_index);
        }
    }
}

fn bottom_panel(p: &mut ChildSpawnerCommands<'_>, s: &Studio) {
    let n = s.clip().map_or(0, |c| c.frames.len());
    label(p, "TIMELINE", INK, 60.0);
    button(p, "idle", Action::Clip("idle"), s.clip == "idle");
    button(p, "run", Action::Clip("run"), s.clip == "run");
    button(p, "<", Action::Step(-1), false);
    for f in 0..n {
        button(p, format!(" {f} "), Action::Frame(f), f == s.frame);
    }
    button(p, ">", Action::Step(1), false);
    let play = if s.playing { "pause" } else { "play" };
    button(p, play, Action::Play, s.playing);
    button(p, "slower", Action::Speed(-1), false);
    label(p, format!("{}x", SPEEDS[s.speed]), DIM, 32.0);
    button(p, "faster", Action::Speed(1), false);
    button(p, "mirror", Action::Mirror, s.flip);
    break_row(p);
    label(p, "VIEW", INK, 60.0);
    button(p, "zoom-", Action::Zoom(-1), false);
    label(p, format!("{}x", s.zoom), DIM, 22.0);
    button(p, "zoom+", Action::Zoom(1), false);
    button(p, "grid", Action::Grid, s.grid);
    button(p, "guides", Action::Guides, s.guides);
    button(p, "skeleton", Action::Skeleton, s.skeleton);
    button(p, "isolated", Action::Isolated, s.isolated);
    button(p, "unequipped", Action::Unequipped, s.unequipped);
    label(p, "   right: same composite, 2x and 1x", DIM, 230.0);
}

// ------------------------------------------------------------ tour

fn step(name: &'static str, ops: Vec<Op>, capture: bool, check: fn(&Studio) -> bool) -> Step {
    Step {
        name,
        ops,
        capture,
        check,
    }
}

fn ok(_: &Studio) -> bool {
    true
}

fn sel(s: &Studio, id: &str) -> bool {
    s.design().is_some_and(|d| d.id == id)
}

fn part_is(s: &Studio, id: &str) -> bool {
    s.part().is_some_and(|p| p.id == id)
}

fn parent_is(s: &Studio, id: &str) -> bool {
    s.part().is_some_and(|p| p.parent.as_deref() == Some(id))
}

fn clean(s: &Studio, id: &str) -> bool {
    s.entries
        .iter()
        .find(|e| e.design.id == id)
        .is_some_and(|e| !e.dirty())
}

fn worn(s: &Studio, slot: RigSlot, id: &str) -> bool {
    s.equipped
        .get(&slot)
        .and_then(|i| s.entries.get(*i))
        .is_some_and(|e| e.design.id == id)
}

fn pixel_count(s: &Studio, c: char) -> usize {
    match s.part().map(|p| &p.source) {
        Some(PartSource::Pixels { rows, .. }) => rows
            .iter()
            .map(|r| r.chars().filter(|x| *x == c).count())
            .sum(),
        _ => 0,
    }
}

fn undo_keys(n: usize) -> Vec<Op> {
    (0..n)
        .map(|_| Op::Keys(vec![KeyCode::ControlLeft, KeyCode::KeyZ]))
        .collect()
}

fn with_wait(mut ops: Vec<Op>, n: u32) -> Vec<Op> {
    ops.push(Op::Wait(n));
    ops
}

fn tour_steps() -> Vec<Step> {
    let mut v = Vec::new();
    v.extend(interface_steps());
    v.extend(hierarchy_steps());
    v.extend(undo_steps());
    v.extend(material_steps());
    v.extend(mouse_steps());
    v.extend(pose_steps());
    v.extend(mix_steps());
    v.extend(save_steps());
    v
}

fn interface_steps() -> Vec<Step> {
    use Op::*;
    vec![step(
        "01_interface",
        vec![
            Click(Action::Clip("run")),
            Click(Action::Frame(2)),
            Click(Action::Grid),
            ClickRow("breastplate"),
            Wait(6),
        ],
        true,
        |s| s.clip == "run" && !s.playing && s.grid && part_is(s, "breastplate"),
    )]
}

fn hierarchy_steps() -> Vec<Step> {
    use KeyCode::*;
    use Op::*;
    vec![
        // The breastplate carries the keel and the emblem.
        step(
            "02_h0_breastplate",
            vec![Click(Action::Grid), ClickPart("breastplate"), Wait(4)],
            true,
            |s| part_is(s, "breastplate"),
        ),
        step(
            "02_h1_rotate",
            vec![Keys(vec![KeyQ]), Wait(1), Keys(vec![KeyQ]), Wait(4)],
            true,
            |s| s.part().is_some_and(|p| p.transform.rotation_deg > 9.0),
        ),
        step("02_h2_move", vec![DragPart(16.0, -8.0), Wait(4)], true, ok),
        step("02_h3_undo", with_wait(undo_keys(3), 4), true, |s| {
            clean(s, "gothic_set_cuirass")
        }),
        // The sword: the ruby becomes a child of the blade (global kept);
        // rotating the blade then carries the ruby.
        step(
            "02_s0_sword",
            vec![
                ClickDesign("arming_sword"),
                ClickRow("pommel_ruby"),
                Wait(4),
            ],
            true,
            |s| sel(s, "arming_sword") && part_is(s, "pommel_ruby"),
        ),
        step(
            "02_s1_parent",
            vec![ClickParent("blade"), Wait(4)],
            true,
            |s| parent_is(s, "blade"),
        ),
        step(
            "02_s2_blade",
            vec![ClickRow("blade"), Wait(3)],
            false,
            |s| part_is(s, "blade"),
        ),
        step(
            "02_s3_rotate",
            vec![
                Click(Action::Nudge(Field::Rot, 5.0)),
                Click(Action::Nudge(Field::Rot, 5.0)),
                Click(Action::Nudge(Field::Rot, 5.0)),
                Click(Action::Nudge(Field::Rot, 5.0)),
                Wait(4),
            ],
            true,
            ok,
        ),
        step(
            "02_s4_rotate",
            vec![
                Click(Action::Nudge(Field::Rot, 5.0)),
                Click(Action::Nudge(Field::Rot, 5.0)),
                Click(Action::Nudge(Field::Rot, 5.0)),
                Click(Action::Nudge(Field::Rot, 5.0)),
                Wait(4),
            ],
            true,
            |s| s.part().is_some_and(|p| p.transform.rotation_deg > -60.0),
        ),
    ]
}

fn undo_steps() -> Vec<Step> {
    use KeyCode::*;
    use Op::*;
    vec![
        step("03_u0_before", vec![Wait(2)], true, ok),
        step("03_u1_undo", with_wait(undo_keys(1), 4), true, ok),
        step("03_u2_undo", with_wait(undo_keys(3), 4), true, ok),
        step("03_u3_undo_all", with_wait(undo_keys(5), 4), true, |s| {
            clean(s, "arming_sword")
        }),
        step(
            "03_u4_redo",
            vec![Keys(vec![ControlLeft, KeyY]), Wait(4)],
            true,
            |s| {
                s.design()
                    .is_some_and(|d| d.parts.iter().any(|p| p.parent.as_deref() == Some("blade")))
            },
        ),
        step(
            "03_u5_redo",
            vec![Keys(vec![ControlLeft, ShiftLeft, KeyZ]), Wait(4)],
            true,
            |s| s.history.can_redo(),
        ),
        step("03_u6_back", with_wait(undo_keys(2), 3), false, |s| {
            clean(s, "arming_sword")
        }),
    ]
}

fn material_steps() -> Vec<Step> {
    use KeyCode::*;
    use Op::*;
    let red = |slot: usize, r: usize, b: usize| {
        let mut ops = vec![Click(Action::RampSlot(slot))];
        ops.extend((0..r).map(|_| Click(Action::Channel(0, 8))));
        ops.extend((0..b).map(|_| Click(Action::Channel(2, -8))));
        ops.push(Wait(3));
        ops
    };
    vec![
        step(
            "04_m0_select",
            vec![
                ClickDesign("gothic_set_pauldron"),
                ClickRow("top_plate"),
                Click(Action::SetMode(Mode::Colour)),
                Wait(4),
            ],
            true,
            |s| part_is(s, "top_plate"),
        ),
        step(
            "04_m1_new_material",
            vec![Click(Action::NewMaterial), Wait(4)],
            true,
            |s| {
                s.part()
                    .is_some_and(|p| p.material.starts_with("material_"))
            },
        ),
        step("04_m2_base", red(3, 4, 2), true, ok),
        step("04_m3_light", red(4, 4, 2), true, ok),
        step("04_m4_shadow", red(2, 3, 1), true, ok),
        step(
            "04_m5_finish",
            vec![
                Click(Action::FinishNext),
                Click(Action::FinishNext),
                Wait(3),
            ],
            true,
            ok,
        ),
        step(
            "04_m6_region",
            vec![Click(Action::AddRegion), Wait(3)],
            true,
            |s| s.part().is_some_and(|p| !p.regions.is_empty()),
        ),
        step(
            "04_m7_rivet",
            vec![Click(Action::AddRivet), Click(Action::AddStitch), Wait(3)],
            true,
            |s| s.part().is_some_and(|p| p.ornaments.len() >= 2),
        ),
        // Unsaved changes: the first Ctrl+R only warns; the second reverts.
        step(
            "04_m8_revert_warns",
            vec![Keys(vec![ControlLeft, KeyR]), Wait(3)],
            true,
            |s| s.any_dirty() && s.status.starts_with("unsaved"),
        ),
        step(
            "04_m9_revert",
            vec![Keys(vec![ControlLeft, KeyR]), Wait(3)],
            false,
            |s| !s.any_dirty(),
        ),
    ]
}

fn mouse_steps() -> Vec<Step> {
    use Op::*;
    vec![
        step(
            "08_k0_select",
            vec![
                ClickDesign("gothic_set_cuirass"),
                Click(Action::SetMode(Mode::Part)),
                ClickPart("gorget"),
                Wait(4),
            ],
            true,
            |s| part_is(s, "gorget") && s.guides,
        ),
        step(
            "08_k1_drag",
            vec![DragPart(0.0, -14.0), Wait(3)],
            true,
            |s| s.history.next_undo() == Some("drag part"),
        ),
        step(
            "08_k2_rotate",
            vec![RotateDrag(-24.0), Wait(3)],
            true,
            |s| s.history.next_undo() == Some("rotate part"),
        ),
        step(
            "08_k3_vertex_mode",
            vec![Click(Action::SetMode(Mode::Vertex)), Wait(3)],
            true,
            |s| s.mode == Mode::Vertex,
        ),
        step(
            "08_k4_vertex",
            vec![DragVertex(2, 20.0, -6.0), Wait(3)],
            true,
            |s| s.history.next_undo() == Some("drag vertex"),
        ),
        step(
            "08_k5_vertex",
            vec![DragVertex(3, 16.0, 10.0), Wait(3)],
            true,
            ok,
        ),
        step(
            "08_k6_pixels",
            vec![Click(Action::Isolated), Click(Action::AddPixels), Wait(4)],
            true,
            |s| s.isolated && s.mode == Mode::Paint && pixel_count(s, '.') == 120,
        ),
        step(
            "08_k7_brush",
            vec![
                Click(Action::PaletteIndex(5)),
                PaintCells((2..10).map(|x| (x, 2)).collect()),
                PaintCells([1, 3, 4, 5, 6, 7].iter().map(|y| (5, *y)).collect()),
                Wait(3),
            ],
            true,
            |s| pixel_count(s, '5') >= 12,
        ),
        step(
            "08_k8_fill",
            vec![
                Click(Action::SetTool(Tool::Fill)),
                Click(Action::PaletteIndex(1)),
                PaintCells(vec![(0, 9)]),
                Wait(3),
            ],
            true,
            |s| pixel_count(s, '1') > 60,
        ),
        // drag, rotate, 2 vertices, pixel part, 2 strokes, fill.
        step(
            "08_k9_undo_all",
            {
                let mut ops = vec![Click(Action::Isolated)];
                ops.extend(undo_keys(8));
                with_wait(ops, 4)
            },
            true,
            |s| !s.isolated && clean(s, "gothic_set_cuirass"),
        ),
    ]
}

fn pose_steps() -> Vec<Step> {
    use Op::*;
    let mut v = vec![step(
        "06_p_idle_0",
        vec![
            Click(Action::Guides),
            Click(Action::SetMode(Mode::Part)),
            Click(Action::Clip("idle")),
            Click(Action::Frame(0)),
            Wait(4),
        ],
        true,
        |s| !s.guides && s.clip == "idle",
    )];
    v.push(step(
        "06_p_idle_1",
        vec![Click(Action::Frame(1)), Wait(4)],
        true,
        |s| s.frame == 1,
    ));
    let names = [
        "06_p_run_0",
        "06_p_run_1",
        "06_p_run_2",
        "06_p_run_3",
        "06_p_run_4",
        "06_p_run_5",
    ];
    for (f, n) in names.into_iter().enumerate() {
        v.push(step(
            n,
            vec![Click(Action::Clip("run")), Click(Action::Frame(f)), Wait(4)],
            true,
            ok,
        ));
    }
    v.push(step(
        "05_c_unequipped",
        vec![Click(Action::Frame(2)), Click(Action::Unequipped), Wait(4)],
        true,
        |s| s.unequipped,
    ));
    v.push(step(
        "05_c_back",
        vec![Click(Action::Unequipped), Wait(2)],
        false,
        |s| !s.unequipped,
    ));
    let pieces = [
        ("gothic_set_cuirass", "05_c_isolated_cuirass"),
        ("gothic_set_pauldron", "05_c_isolated_pauldron"),
        ("gothic_set_bracers", "05_c_isolated_bracers"),
        ("gothic_set_faulds", "05_c_isolated_faulds"),
    ];
    for (id, name) in pieces {
        v.push(step(
            name,
            vec![ClickDesign(id), Click(Action::Isolated), Wait(4)],
            true,
            |s| s.isolated,
        ));
        v.push(step(
            "05_c_off",
            vec![Click(Action::Isolated), Wait(2)],
            false,
            |s| !s.isolated,
        ));
    }
    v
}

fn mix_steps() -> Vec<Step> {
    use Op::*;
    vec![
        step(
            "07_x0_play",
            vec![
                Click(Action::Speed(-1)),
                Click(Action::Speed(-1)),
                Click(Action::Play),
                Wait(6),
            ],
            true,
            |s| s.playing,
        ),
        step(
            "07_x1_jerkin",
            vec![ClickEquip("leather_jerkin"), Wait(6)],
            true,
            |s| worn(s, RigSlot::Chest, "leather_jerkin") && s.playing,
        ),
        step("07_x2", vec![Wait(8)], true, ok),
        step(
            "07_x3_no_pauldron",
            vec![ClickEquip("gothic_set_pauldron"), Wait(6)],
            true,
            |s| !s.equipped.contains_key(&RigSlot::Shoulder),
        ),
        step("07_x4", vec![Wait(8)], true, ok),
        step(
            "07_x5_cuirass_pauldron",
            vec![
                ClickEquip("gothic_set_pauldron"),
                ClickEquip("gothic_set_cuirass"),
                Wait(6),
            ],
            true,
            |s| worn(s, RigSlot::Chest, "gothic_set_cuirass"),
        ),
        step("07_x6", vec![Wait(8)], true, ok),
        step(
            "07_x7_no_bracers",
            vec![ClickEquip("gothic_set_bracers"), Wait(6)],
            true,
            |s| !s.equipped.contains_key(&RigSlot::Bracers),
        ),
        step(
            "07_x8_no_faulds",
            vec![ClickEquip("gothic_set_faulds"), Wait(6)],
            true,
            |s| !s.equipped.contains_key(&RigSlot::Belt),
        ),
        step(
            "07_x9_all",
            vec![
                ClickEquip("gothic_set_bracers"),
                ClickEquip("gothic_set_faulds"),
                Wait(6),
            ],
            true,
            |s| worn(s, RigSlot::Belt, "gothic_set_faulds"),
        ),
        step(
            "07_x10_stop",
            vec![
                Click(Action::Play),
                Click(Action::Speed(1)),
                Click(Action::Speed(1)),
                Wait(3),
            ],
            false,
            |s| !s.playing,
        ),
    ]
}

fn save_steps() -> Vec<Step> {
    use Op::*;
    vec![
        // A hierarchical item saved: the sword with the ruby under the blade.
        step(
            "09_v0_sword",
            vec![
                Click(Action::Frame(2)),
                ClickDesign("arming_sword"),
                ClickRow("pommel_ruby"),
                ClickParent("blade"),
                Wait(3),
            ],
            false,
            |s| parent_is(s, "blade"),
        ),
        step(
            "09_v1_saved",
            vec![Click(Action::Save), Wait(4)],
            true,
            |s| s.status.starts_with("saved") && clean(s, "arming_sword"),
        ),
        // A new material on the pauldron, saved and saved as a copy.
        step(
            "09_v2_pauldron",
            vec![
                ClickDesign("gothic_set_pauldron"),
                ClickRow("top_plate"),
                Click(Action::NewMaterial),
                Click(Action::RampSlot(3)),
                Click(Action::Channel(1, 8)),
                Click(Action::Channel(1, 8)),
                Click(Action::Channel(1, 8)),
                Click(Action::Save),
                Wait(4),
            ],
            true,
            |s| s.status.starts_with("saved") && clean(s, "gothic_set_pauldron"),
        ),
        step(
            "09_v3_save_as",
            vec![Click(Action::SaveAs), Wait(4)],
            true,
            |s| s.status.starts_with("saved as"),
        ),
        // A new item made only with the panels: a plate with a brass buckle.
        step(
            "09_v4_new_item",
            vec![
                Click(Action::New),
                Click(Action::Import),
                Click(Action::Save),
                Wait(4),
            ],
            true,
            |s| sel(s, "item_1") && clean(s, "item_1"),
        ),
        step(
            "09_v5_back",
            vec![ClickEquip("gothic_set_cuirass"), Wait(3)],
            false,
            |s| worn(s, RigSlot::Chest, "gothic_set_cuirass"),
        ),
    ]
}

fn reload_steps() -> Vec<Step> {
    use Op::*;
    vec![
        step(
            "09_r0_reopened",
            vec![
                Click(Action::Clip("run")),
                Click(Action::Frame(2)),
                ClickDesign("arming_sword"),
                ClickRow("pommel_ruby"),
                Wait(8),
            ],
            true,
            |s| parent_is(s, "blade"),
        ),
        step(
            "09_r1_material",
            vec![
                ClickDesign("gothic_set_pauldron"),
                ClickRow("top_plate"),
                Wait(4),
            ],
            true,
            |s| {
                s.part()
                    .is_some_and(|p| p.material.starts_with("material_"))
                    && s.design_index("gothic_set_pauldron_copy").is_some()
                    && s.design_index("item_1").is_some()
            },
        ),
    ]
}

/// Logical screen centre of the panel button of an action.
fn button_pos(
    ui: &Query<(&UiButton, &ComputedNode, &UiGlobalTransform)>,
    a: &Action,
    scale: f32,
) -> Option<Vec2> {
    ui.iter()
        .find(|(b, _, _)| b.0 == *a)
        .map(|(_, _, t)| t.translation / scale)
}

fn click_at(p: Vec2) -> Vec<PointerFrame> {
    vec![
        PointerFrame::up(p),
        PointerFrame::left(p),
        PointerFrame::up(p),
        PointerFrame::up(p),
    ]
}

fn drag_frames(p: Vec2, delta: Vec2, right: bool) -> Vec<PointerFrame> {
    let held = |q| {
        if right {
            PointerFrame::right(q)
        } else {
            PointerFrame::left(q)
        }
    };
    let mut q = vec![PointerFrame::up(p), held(p)];
    for i in 1..=6 {
        q.push(held(p + delta * (i as f32 / 6.0)));
    }
    q.push(PointerFrame::up(p + delta));
    q.push(PointerFrame::up(p + delta));
    q
}

/// Turns one tour op into pointer frames or key presses. `Err` names what
/// was not found.
fn start_op(
    s: &mut Studio,
    op: &Op,
    keyboard: &mut ButtonInput<KeyCode>,
    ui: &Query<(&UiButton, &ComputedNode, &UiGlobalTransform)>,
    scale: f32,
) -> Result<(), String> {
    let click = |a: Action| {
        button_pos(ui, &a, scale)
            .map(click_at)
            .ok_or(format!("no button {a:?}"))
    };
    let row = |s: &Studio, id: &str| {
        s.design()
            .and_then(|d| d.index_of(id))
            .ok_or(format!("no part {id}"))
    };
    match op {
        Op::Keys(k) => {
            for key in k {
                keyboard.press(*key);
            }
            s.keys_held = k.clone();
        }
        Op::Click(a) => s.queue = click(a.clone())?,
        Op::ClickDesign(id) => {
            let i = s.design_index(id).ok_or(format!("no design {id}"))?;
            s.queue = click(Action::SelectDesign(i))?;
        }
        Op::ClickEquip(id) => {
            let i = s.design_index(id).ok_or(format!("no design {id}"))?;
            s.queue = click(Action::ToggleEquip(i))?;
        }
        Op::ClickRow(id) => s.queue = click(Action::SelectPart(row(s, id)?))?,
        Op::ClickParent(id) => s.queue = click(Action::ParentTo(Some(row(s, id)?)))?,
        Op::ClickPart(id) => {
            let target = (s.selected, row(s, id)?);
            let t = s
                .pick
                .iter()
                .position(|o| *o == Some(target))
                .and_then(|t| s.texel_screen(t))
                .ok_or(format!("part {id} not on top anywhere"))?;
            s.queue = click_at(t);
        }
        Op::DragPart(dx, dy) => {
            let p = s
                .part()
                .and_then(|p| s.local_to_screen(p.transform.pivot))
                .ok_or("no part to drag")?;
            s.queue = drag_frames(p, Vec2::new(*dx, *dy), false);
        }
        Op::RotateDrag(dx) => {
            let p = s
                .part()
                .and_then(|p| s.local_to_screen(p.transform.pivot))
                .ok_or("no part to rotate")?;
            s.queue = drag_frames(p, Vec2::new(*dx, 0.0), true);
        }
        Op::DragVertex(v, dx, dy) => {
            let p = s
                .target_points()
                .and_then(|pts| pts.get(*v).copied())
                .and_then(|l| s.local_to_screen(l))
                .ok_or(format!("no vertex {v}"))?;
            s.queue = drag_frames(p, Vec2::new(*dx, *dy), false);
        }
        Op::PaintCells(cells) => {
            // A canvas texel that maps to each cell of the pixel part.
            let (Some(d), Some(map), Some(c)) = (s.design(), s.part_map(), s.clip()) else {
                return Err("no pixel part".into());
            };
            let g = c.geometry();
            let mut q = Vec::new();
            for cell in cells {
                let hit = (0..g.width * g.height).find(|t| {
                    let p = g.texel_centre(t % g.width, t / g.width);
                    pixel_cell_at(d, s.part, map, g, p) == Some(*cell)
                });
                let t = hit
                    .and_then(|t| s.texel_screen(t))
                    .ok_or(format!("cell {cell:?} not on screen"))?;
                if q.is_empty() {
                    q.push(PointerFrame::up(t));
                }
                q.push(PointerFrame::left(t));
            }
            if let Some(last) = q.last().copied() {
                q.push(PointerFrame::up(last.pos));
                q.push(PointerFrame::up(last.pos));
            }
            s.queue = q;
        }
        Op::Wait(_) => {}
    }
    Ok(())
}

fn run_tour(
    mut commands: Commands,
    mut res: ResMut<Studio>,
    mut keyboard: ResMut<ButtonInput<KeyCode>>,
    ui: Query<(&UiButton, &ComputedNode, &UiGlobalTransform)>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let s = &mut *res;
    if s.tour.is_none() {
        return;
    }
    if !s.body_ready || s.composite_key.is_none() || s.warm < 40 {
        s.warm += 1;
        return;
    }
    for k in std::mem::take(&mut s.keys_held) {
        keyboard.release(k);
    }
    if !s.queue.is_empty() {
        let f = s.queue.remove(0);
        s.pointer = Some(f);
        return;
    }
    let total = s.tour.as_ref().map_or(0, Vec::len);
    if s.step >= total {
        s.op_frame += 1;
        if s.op_frame > 40 && VERDICT.get().is_none() {
            for f in &s.failures {
                println!("{TAG} FAIL {f}");
            }
            let pass = s.failures.is_empty();
            println!("{TAG} RESULT: {}", if pass { "PASS" } else { "FAIL" });
            let _ = VERDICT.set(pass);
            s.exit_now = true;
        }
        return;
    }
    let Some((ops, capture, check, name)) = s
        .tour
        .as_ref()
        .and_then(|t| t.get(s.step))
        .map(|t| (t.ops.clone(), t.capture, t.check, t.name))
    else {
        return;
    };
    if let Some(op) = ops.get(s.op) {
        if let Op::Wait(n) = op {
            s.op_frame += 1;
            if s.op_frame < *n {
                return;
            }
            s.op_frame = 0;
        } else if let Err(e) = start_op(s, op, &mut keyboard, &ui, window.scale_factor()) {
            s.failures.push(format!("{name}: {e}"));
        }
        s.op += 1;
        return;
    }
    if !check(s) {
        s.failures.push(format!(
            "{name}: the studio did not reach the expected state"
        ));
    }
    if capture {
        let path = s.out.join(format!("{name}.png"));
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
        if let Some(c) = s.clip() {
            let raw = s.out.join(format!("{name}_{}x{}.rgba", c.cell.0, c.cell.1));
            let bytes: Vec<u8> = s.last_composite.iter().flatten().copied().collect();
            if let Err(e) = std::fs::write(raw, bytes) {
                s.failures.push(format!("{name}: raw composite: {e}"));
            }
        }
    }
    let worn: Vec<(String, String)> = s
        .equipped
        .iter()
        .map(|(k, i)| {
            let id = s.entries.get(*i).map(|e| e.design.id.clone());
            (format!("{k:?}"), id.unwrap_or_default())
        })
        .collect();
    println!(
        "{TAG} {name} design={} part={} mode={} clip={} frame={} worn={worn:?} undo={} status={}",
        s.design().map_or("-", |d| d.id.as_str()),
        s.part().map_or("-", |p| p.id.as_str()),
        s.mode.name(),
        s.clip,
        s.frame,
        s.history.len(),
        s.status
    );
    s.pointer = None;
    s.op = 0;
    s.op_frame = 0;
    s.step += 1;
}

/// A fresh copy of the items in the workspace (only `.ron` files there are
/// replaced; the owner's folder is only read).
fn prepare_workspace(root: &Path) -> std::io::Result<PathBuf> {
    let ws = root.join(WORKSPACE);
    std::fs::create_dir_all(&ws)?;
    for e in std::fs::read_dir(&ws)? {
        let p = e?.path();
        if p.extension().is_some_and(|x| x == "ron") {
            std::fs::remove_file(p)?;
        }
    }
    for e in std::fs::read_dir(root.join(STUDIO).join("items"))? {
        let p = e?.path();
        if p.extension().is_some_and(|x| x == "ron")
            && let Some(name) = p.file_name()
        {
            std::fs::copy(&p, ws.join(name))?;
        }
    }
    Ok(ws)
}

fn main() -> AppExit {
    let root = project_root();
    let args: Vec<String> = std::env::args().collect();
    let tour_mode = args.iter().any(|a| a == "--tour");
    let reload_mode = args.iter().any(|a| a == "--tour-reload");
    let items_dir = if tour_mode {
        match prepare_workspace(&root) {
            Ok(ws) => ws,
            Err(e) => {
                eprintln!("{TAG} workspace: {e}");
                return AppExit::error();
            }
        }
    } else if reload_mode {
        root.join(WORKSPACE)
    } else {
        args.iter()
            .position(|a| a == "--items")
            .and_then(|i| args.get(i + 1))
            .map_or_else(|| root.join(STUDIO).join("items"), PathBuf::from)
    };
    println!("{TAG} items: {}", items_dir.display());
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
    let tour = if tour_mode {
        Some(tour_steps())
    } else if reload_mode {
        Some(reload_steps())
    } else {
        None
    };
    let scripted = tour.is_some();
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
                    title: "Dark Castle Equipment Studio V2".into(),
                    resolution: WindowResolution::new(WIN.0 as u32, WIN.1 as u32)
                        .with_scale_factor_override(1.0),
                    resizable: false,
                    ..default()
                }),
                close_when_requested: false,
                ..default()
            }),
    )
    .insert_resource(ClearColor(BG))
    .insert_resource(Studio {
        items_dir,
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
        target: Target::Shape,
        mode: Mode::Part,
        tool: Tool::Brush,
        palette_index: 3,
        ramp_slot: 3,
        keep_global: true,
        category: None,
        image_handles: HashMap::new(),
        decoded: HashMap::new(),
        imports,
        import_cursor: 0,
        clip: "idle".into(),
        elapsed: 0.0,
        frame: 0,
        flip: false,
        playing: !scripted,
        speed: 2,
        zoom: 4,
        grid: false,
        guides: true,
        isolated: false,
        unequipped: false,
        skeleton: false,
        status: "ready".into(),
        history: History::new(200),
        dragging: None,
        drag_before: None,
        composite: Handle::default(),
        composite_key: None,
        pick: Vec::new(),
        last_composite: Vec::new(),
        painted: HashMap::new(),
        ui_key: String::new(),
        close_armed: 0.0,
        revert_armed: 0.0,
        pointer: None,
        last_buttons: (false, false),
        tour,
        step: 0,
        op: 0,
        op_frame: 0,
        queue: Vec::new(),
        keys_held: Vec::new(),
        warm: 0,
        out,
        failures: Vec::new(),
        exit_now: false,
    })
    .add_systems(Startup, setup)
    .add_systems(
        Update,
        (
            load_assets,
            run_tour,
            keys,
            mouse,
            guard_close,
            advance,
            render,
            sync_views,
            overlays,
            panels,
        )
            .chain(),
    );
    let exit = app.run();
    if !scripted {
        return exit;
    }
    match VERDICT.get() {
        Some(true) => exit,
        _ => AppExit::error(),
    }
}
