//! Controls. Physical keys become logical actions before any gameplay system
//! sees them. A future gamepad would call the same edge combiner; none is
//! implemented here.

pub mod action;
pub mod binding;
pub mod edges;
pub mod presets;
pub mod profile;
pub mod store;

use bevy::input::mouse::MouseButton;
use bevy::prelude::*;

use crate::app::states::GameSet;

use action::{ACTIONS, Action};
use binding::Binding;
use edges::{HeldState, combine};
use presets::{classic, hybrid};
use profile::ControlProfile;
use store::{LoadError, load, user_path};

pub use action::Action as GameAction;
pub use profile::{AssignError, ControlProfile as Controls, ProfileOrigin, Slot};

/// Where `controls.ron` is read and written. Tests set this so they never
/// touch the developer's real file.
#[derive(Resource, Clone, Debug)]
pub struct ControlsPath(pub std::path::PathBuf);

/// Shown when the file on disk could not be used. The file itself is kept.
#[derive(Resource, Clone, Debug)]
pub struct ControlsNotice(pub String);

/// Combined press state for every action, updated even while a menu is open
/// so a key that stayed held does not look newly pressed when play resumes.
#[derive(Resource, Clone, Debug, Default)]
pub struct ActionEdges {
    pressed: [bool; Action::COUNT],
    just_pressed: [bool; Action::COUNT],
    just_released: [bool; Action::COUNT],
}

impl ActionEdges {
    pub fn pressed(&self, action: Action) -> bool {
        self.pressed[action.index()]
    }

    pub fn just_pressed(&self, action: Action) -> bool {
        self.just_pressed[action.index()]
    }

    pub fn just_released(&self, action: Action) -> bool {
        self.just_released[action.index()]
    }
}

#[derive(Resource, Default)]
struct HeldLatch {
    was: [bool; Action::COUNT],
}

/// What the player is doing this frame, in gameplay terms.
///
/// `axis_x` is -1/0/1 (left/none/right). `axis_y` is aim, not jump: +1 is
/// aim up, -1 is aim down. Jump is `jump_pressed` / `jump_held`.
#[derive(Resource, Default, Debug, Clone, Copy)]
pub struct PlayerInput {
    pub axis_x: i8,
    pub axis_y: i8,
    pub run_held: bool,
    pub jump_pressed: bool,
    pub jump_held: bool,
    pub attack_pressed: bool,
    pub magic_pressed: bool,
    pub guard_pressed: bool,
    pub guard_held: bool,
    pub dodge_pressed: bool,
    pub respawn_pressed: bool,
    pub interact_pressed: bool,
    pub transform_pressed: bool,
}

/// Reads devices into [`ActionEdges`] every frame, including while a menu
/// pauses the simulation. Gameplay copies those edges in [`GameSet::Input`].
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ControlReadSet;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerInput>()
            .init_resource::<ActionEdges>()
            .init_resource::<HeldLatch>()
            .configure_sets(Update, ControlReadSet.before(GameSet::Input))
            .add_systems(Startup, install_controls)
            .add_systems(Update, read_controls.in_set(ControlReadSet))
            .add_systems(Update, publish_player_input.in_set(GameSet::Input));
    }
}

/// Scripted examples and the existing key-tap tests keep the old scheme.
/// The real game does not call this.
pub fn use_legacy_controls(app: &mut App) {
    app.insert_resource(classic());
    let dir = std::env::var("CARGO_TARGET_TMPDIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::env::temp_dir());
    app.insert_resource(ControlsPath(dir.join("dark_castle_controls_isolated.ron")));
}

fn install_controls(
    mut commands: Commands,
    profile: Option<Res<ControlProfile>>,
    path: Option<Res<ControlsPath>>,
) {
    let file = path
        .as_ref()
        .map(|path| path.0.clone())
        .unwrap_or_else(user_path);
    if path.is_none() {
        commands.insert_resource(ControlsPath(file.clone()));
    }
    if profile.is_some() {
        return;
    }
    match load(&file) {
        Ok(loaded) => {
            commands.insert_resource(loaded);
        }
        Err(LoadError::Missing) => {
            commands.insert_resource(hybrid());
        }
        Err(LoadError::Invalid(reason)) => {
            tracing::warn!(
                path = %file.display(),
                %reason,
                "controls file was not used; the file was left as it is"
            );
            commands.insert_resource(hybrid());
            commands.insert_resource(ControlsNotice(format!(
                "Controls file could not be read ({reason}). Using the hybrid preset. The file was not overwritten."
            )));
        }
    }
}

fn binding_flags(
    binding: Binding,
    keys: &ButtonInput<KeyCode>,
    mouse: &ButtonInput<MouseButton>,
) -> (bool, bool, bool) {
    match binding {
        Binding::Key(key) => {
            let code = key.to_keycode();
            (
                keys.pressed(code),
                keys.just_pressed(code),
                keys.just_released(code),
            )
        }
        Binding::Mouse(button) => {
            let button = button.to_button();
            (
                mouse.pressed(button),
                mouse.just_pressed(button),
                mouse.just_released(button),
            )
        }
    }
}

/// The only place a device becomes an action. Keyboard and mouse are
/// combined here; a later device would feed the same `combine` call.
fn read_controls(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    profile: Option<Res<ControlProfile>>,
    mut latch: ResMut<HeldLatch>,
    mut edges: ResMut<ActionEdges>,
) {
    let Some(profile) = profile else {
        return;
    };
    for action in ACTIONS {
        let mut state = HeldState {
            steady: false,
            retriggered: false,
        };
        for binding in profile.binding(action).iter() {
            let (pressed, just_pressed, just_released) = binding_flags(binding, &keys, &mouse);
            if pressed && !just_pressed && !just_released {
                state.steady = true;
            }
            if just_pressed {
                state.retriggered = true;
            }
        }
        let (edge, was) = combine(state, latch.was[action.index()]);
        latch.was[action.index()] = was;
        edges.pressed[action.index()] = edge.pressed;
        edges.just_pressed[action.index()] = edge.just_pressed;
        edges.just_released[action.index()] = edge.just_released;
    }
}

pub fn publish_player_input(edges: Res<ActionEdges>, mut input: ResMut<PlayerInput>) {
    let left = edges.pressed(Action::MoveLeft);
    let right = edges.pressed(Action::MoveRight);
    let up = edges.pressed(Action::AimUp);
    let down = edges.pressed(Action::AimDown);
    input.axis_x = match (left, right) {
        (true, false) => -1,
        (false, true) => 1,
        _ => 0,
    };
    input.axis_y = match (up, down) {
        (true, false) => 1,
        (false, true) => -1,
        _ => 0,
    };
    input.run_held = edges.pressed(Action::Run);
    input.jump_pressed = edges.just_pressed(Action::Jump);
    input.jump_held = edges.pressed(Action::Jump);
    input.attack_pressed = edges.just_pressed(Action::Attack);
    input.magic_pressed = edges.just_pressed(Action::Magic);
    input.guard_pressed = edges.just_pressed(Action::Guard);
    input.guard_held = edges.pressed(Action::Guard);
    input.dodge_pressed = edges.just_pressed(Action::Dodge);
    input.respawn_pressed = edges.just_pressed(Action::Respawn);
    input.interact_pressed = edges.just_pressed(Action::Interact);
    input.transform_pressed = edges.just_pressed(Action::Transform);
}

/// One line for the HUD, taken from the active profile.
pub fn controls_hint(profile: &ControlProfile) -> String {
    let jump = profile.binding(Action::Jump);
    let jump_label = match (jump.primary, jump.alternate) {
        (Some(primary), Some(alternate)) => format!("{}/{}", primary.label(), alternate.label()),
        (Some(primary), None) => primary.label().to_string(),
        (None, Some(alternate)) => alternate.label().to_string(),
        (None, None) => "—".to_string(),
    };
    let attack = profile
        .binding(Action::Attack)
        .primary
        .map(|binding| binding.label())
        .unwrap_or("—");
    format!(
        "{} left  {} right  {} jump  {} attack  {} interact  Esc controls",
        profile
            .binding(Action::MoveLeft)
            .primary
            .map(|binding| binding.label())
            .unwrap_or("—"),
        profile
            .binding(Action::MoveRight)
            .primary
            .map(|binding| binding.label())
            .unwrap_or("—"),
        jump_label,
        attack,
        profile
            .binding(Action::Interact)
            .primary
            .map(|binding| binding.label())
            .unwrap_or("—"),
    )
}
