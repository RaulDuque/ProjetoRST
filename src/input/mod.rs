//! Keyboard bindings and the per-frame [`PlayerInput`] snapshot.
//!
//! Gameplay systems read `PlayerInput` instead of raw keys, so rebinding
//! (or adding gamepad support) only touches this module.

use bevy::prelude::*;

use crate::app::states::GameSet;

/// Key bindings. Defaults are documented in `docs/INPUT_SYSTEM.md`.
#[derive(Resource, Debug, Clone)]
pub struct InputBindings {
    pub left: KeyCode,
    pub right: KeyCode,
    pub up: KeyCode,
    pub down: KeyCode,
    pub run: KeyCode,
    pub jump: KeyCode,
    pub attack: KeyCode,
    pub magic: KeyCode,
    pub guard: KeyCode,
    pub dodge: KeyCode,
    pub status: KeyCode,
    pub respawn: KeyCode,
    pub quick_save: KeyCode,
    pub quick_load: KeyCode,
    /// Opens and closes the map panel (V1.7).
    pub map: KeyCode,
    /// Uses a chest, lever or hidden switch the player stands on (V1.8).
    /// A free key of its own: `↑` is part of commands (`↓ ↑ + ATTACK`), and
    /// in that sequence the `↑` frame is no longer a crouch, so sharing it
    /// would use objects by accident.
    pub interact: KeyCode,
    /// Enters and leaves the transformation (V1.9: Spectral Form). `T` was
    /// free: no command, menu or debug key uses it.
    pub transform: KeyCode,
    /// Opens and closes the equipment/inventory screen (V2.0). `I` was free:
    /// no command, menu or debug key uses it. Inside the screen (simulation
    /// paused) `↑`/`↓` select and `Enter` equips or removes; gameplay keys
    /// are not read there, so nothing conflicts with combat.
    pub inventory: KeyCode,
    /// Confirms in the inventory screen (V2.0).
    pub confirm: KeyCode,
}

impl Default for InputBindings {
    fn default() -> Self {
        Self {
            left: KeyCode::ArrowLeft,
            right: KeyCode::ArrowRight,
            up: KeyCode::ArrowUp,
            down: KeyCode::ArrowDown,
            run: KeyCode::ShiftLeft,
            jump: KeyCode::KeyZ,
            attack: KeyCode::KeyX,
            magic: KeyCode::KeyC,
            guard: KeyCode::KeyA,
            dodge: KeyCode::KeyS,
            status: KeyCode::Tab,
            respawn: KeyCode::KeyR,
            quick_save: KeyCode::F5,
            quick_load: KeyCode::F9,
            map: KeyCode::KeyM,
            interact: KeyCode::KeyD,
            transform: KeyCode::KeyT,
            inventory: KeyCode::KeyI,
            confirm: KeyCode::Enter,
        }
    }
}

/// What the player asked for this frame.
#[derive(Resource, Debug, Clone, Copy, Default, PartialEq)]
pub struct PlayerInput {
    /// -1 left, 0, +1 right.
    pub axis_x: i8,
    /// -1 down, 0, +1 up.
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

fn axis(keys: &ButtonInput<KeyCode>, negative: KeyCode, positive: KeyCode) -> i8 {
    i8::from(keys.pressed(positive)) - i8::from(keys.pressed(negative))
}

pub fn read_keyboard(
    keys: Res<ButtonInput<KeyCode>>,
    bindings: Res<InputBindings>,
    mut input: ResMut<PlayerInput>,
) {
    *input = PlayerInput {
        axis_x: axis(&keys, bindings.left, bindings.right),
        axis_y: axis(&keys, bindings.down, bindings.up),
        run_held: keys.pressed(bindings.run),
        jump_pressed: keys.just_pressed(bindings.jump),
        jump_held: keys.pressed(bindings.jump),
        attack_pressed: keys.just_pressed(bindings.attack),
        magic_pressed: keys.just_pressed(bindings.magic),
        guard_pressed: keys.just_pressed(bindings.guard),
        guard_held: keys.pressed(bindings.guard),
        dodge_pressed: keys.just_pressed(bindings.dodge),
        respawn_pressed: keys.just_pressed(bindings.respawn),
        interact_pressed: keys.just_pressed(bindings.interact),
        transform_pressed: keys.just_pressed(bindings.transform),
    };
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputBindings>()
            .init_resource::<PlayerInput>()
            .add_systems(Update, read_keyboard.in_set(GameSet::Input));
    }
}
