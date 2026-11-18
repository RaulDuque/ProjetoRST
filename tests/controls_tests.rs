//! N2: logical controls. The shipped default is the hybrid preset. These
//! tests install it on purpose. The older route tests keep the classic
//! preset through `use_legacy_controls`, so they still press the old keys.

use std::fs;
use std::path::{Path, PathBuf};

use bevy::input::mouse::MouseButton;
use bevy::prelude::*;
use dark_castle_rpg::app::states::GameState;
use dark_castle_rpg::combat::command_buffer::{InputToken, Motion};
use dark_castle_rpg::input::action::Action;
use dark_castle_rpg::input::binding::{Binding, PhysicalKey};
use dark_castle_rpg::input::presets::{classic, hybrid};
use dark_castle_rpg::input::profile::{AssignError, Slot};
use dark_castle_rpg::input::{ActionEdges, PlayerInput, use_legacy_controls};
use dark_castle_rpg::player::components::{ActionState, CommandInput, Player};
use dark_castle_rpg::player::state::PlayerState;
use dark_castle_rpg::save::serialization::CURRENT_SAVE_VERSION;

#[path = "support/world_app.rs"]
mod world_app;

use world_app::{PendingPress, game, player_xy, run, storage, tap};

fn hybrid_game() -> App {
    let saves = storage("controls_hybrid");
    let mut app = game(&saves);
    app.insert_resource(hybrid());
    run(&mut app, 1);
    app
}

fn motions(app: &mut App) -> Vec<Motion> {
    let world = app.world_mut();
    world
        .query_filtered::<&CommandInput, With<Player>>()
        .single(world)
        .unwrap_or_else(|e| panic!("{e}"))
        .0
        .entries()
        .filter_map(|entry| match entry.token {
            InputToken::Motion(motion) => Some(motion),
            InputToken::Button(_) => None,
        })
        .collect()
}

fn hold(app: &mut App, keys: &[KeyCode], frames: usize) {
    for _ in 0..frames {
        app.world_mut()
            .resource_mut::<PendingPress>()
            .0
            .extend(keys.iter().copied());
        run(app, 1);
    }
}

#[test]
fn the_character_save_stays_on_version_3() {
    assert_eq!(CURRENT_SAVE_VERSION, 3);
}

#[test]
fn hybrid_rejects_a_gameplay_conflict_and_accepts_enter_on_a_menu_action() {
    let mut profile = hybrid();
    let err = match profile.assign(
        Action::Interact,
        Slot::Alternate,
        Binding::Key(PhysicalKey::D),
    ) {
        Err(err) => err,
        Ok(()) => panic!("D already moves right"),
    };
    assert_eq!(err, AssignError::Conflict(Action::MoveRight));
    assert_eq!(
        profile.binding(Action::Interact).primary,
        Some(Binding::Key(PhysicalKey::E))
    );
    profile
        .assign(
            Action::Jump,
            Slot::Primary,
            Binding::Key(PhysicalKey::Enter),
        )
        .unwrap_or_else(|error| panic!("Enter may also be a gameplay key: {error:?}"));
    assert!(matches!(
        profile.clear(Action::MoveLeft, Slot::Primary),
        Err(AssignError::Essential)
    ));
}

#[test]
fn switching_preset_does_not_touch_a_character_save_file() {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("controls_preset_switch");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap_or_else(|e| panic!("{e}"));
    let save = dir.join("slot_1.ron");
    fs::write(&save, "untouched").unwrap_or_else(|e| panic!("{e}"));
    let mut profile = hybrid();
    profile
        .assign(Action::Guard, Slot::Primary, Binding::Key(PhysicalKey::G))
        .unwrap_or_else(|e| panic!("{e:?}"));
    assert_eq!(
        fs::read_to_string(&save).unwrap_or_else(|e| panic!("{e}")),
        "untouched"
    );
    let restored = classic();
    assert_ne!(profile.slots, restored.slots);
}

#[test]
fn w_jumps_and_does_not_aim_up_while_the_arrow_aims_up() {
    let mut app = hybrid_game();
    hold(&mut app, &[KeyCode::KeyW], 1);
    let input = *app.world().resource::<PlayerInput>();
    assert!(input.jump_pressed, "W is jump");
    assert_eq!(input.axis_y, 0, "W is not aim up");
    hold(&mut app, &[KeyCode::ArrowUp], 1);
    let input = *app.world().resource::<PlayerInput>();
    assert!(!input.jump_pressed);
    assert_eq!(input.axis_y, 1);
}

#[test]
fn space_while_w_is_held_is_not_a_second_jump() {
    let mut app = hybrid_game();
    app.insert_resource(StickyKeys(vec![KeyCode::KeyW]))
        .add_systems(PreUpdate, press_sticky.after(bevy::input::InputSystems));
    run(&mut app, 1);
    assert!(
        app.world()
            .resource::<ActionEdges>()
            .just_pressed(Action::Jump)
    );
    app.world_mut()
        .resource_mut::<StickyKeys>()
        .0
        .push(KeyCode::Space);
    run(&mut app, 1);
    let edges = app.world().resource::<ActionEdges>();
    assert!(edges.pressed(Action::Jump));
    assert!(!edges.just_pressed(Action::Jump));
    assert!(!edges.just_released(Action::Jump));
}

#[derive(Resource, Default)]
struct StickyKeys(Vec<KeyCode>);

fn press_sticky(sticky: Res<StickyKeys>, mut keys: ResMut<ButtonInput<KeyCode>>) {
    for key in &sticky.0 {
        keys.press(*key);
    }
}

#[derive(Resource, Default)]
struct HoldLeft(bool);

fn press_left_after_input(hold: Res<HoldLeft>, mut mouse: ResMut<ButtonInput<MouseButton>>) {
    if hold.0 {
        mouse.press(MouseButton::Left);
    }
}

#[test]
fn mouse_and_the_attack_key_in_one_frame_are_one_attack() {
    let mut app = hybrid_game();
    app.insert_resource(HoldLeft(true)).add_systems(
        PreUpdate,
        press_left_after_input.after(bevy::input::InputSystems),
    );
    hold(&mut app, &[KeyCode::KeyX], 1);
    let edges = app.world().resource::<ActionEdges>();
    assert!(edges.just_pressed(Action::Attack));
    assert!(app.world().resource::<PlayerInput>().attack_pressed);
    hold(&mut app, &[KeyCode::KeyX], 1);
    assert!(
        !app.world()
            .resource::<ActionEdges>()
            .just_pressed(Action::Attack)
    );
}

#[test]
fn d_moves_right_and_e_interacts() {
    let mut app = hybrid_game();
    let before = player_xy(&mut app).x;
    hold(&mut app, &[KeyCode::KeyD], 4);
    assert!(player_xy(&mut app).x > before);
    assert!(!app.world().resource::<PlayerInput>().interact_pressed);

    let mut app = hybrid_game();
    hold(&mut app, &[KeyCode::KeyE], 1);
    assert!(app.world().resource::<PlayerInput>().interact_pressed);
    assert_eq!(app.world().resource::<PlayerInput>().axis_x, 0);
}

#[test]
fn aim_down_reaches_the_command_buffer_and_jump_does_not() {
    let mut app = hybrid_game();
    hold(&mut app, &[KeyCode::KeyS], 2);
    assert!(motions(&mut app).contains(&Motion::Down));

    let mut app = hybrid_game();
    hold(&mut app, &[KeyCode::KeyW], 2);
    let seen = motions(&mut app);
    assert!(
        !seen.contains(&Motion::Up),
        "jump must not be aim up: {seen:?}"
    );
    hold(&mut app, &[KeyCode::ArrowUp], 2);
    assert!(motions(&mut app).contains(&Motion::Up));
}

#[test]
fn s_crouches_on_the_ground() {
    let mut app = hybrid_game();
    hold(&mut app, &[KeyCode::KeyS], 2);
    let world = app.world_mut();
    let state = world
        .query_filtered::<&ActionState, With<Player>>()
        .single(world)
        .unwrap_or_else(|e| panic!("{e}"))
        .0
        .state();
    assert_eq!(state, PlayerState::Crouching);
}

#[test]
fn the_controls_screen_pauses_movement() {
    let mut app = hybrid_game();
    tap(&mut app, KeyCode::Escape);
    run(&mut app, 1);
    assert_eq!(
        *app.world().resource::<State<GameState>>().get(),
        GameState::Controls
    );
    let x = player_xy(&mut app).x;
    hold(&mut app, &[KeyCode::KeyD], 6);
    assert_eq!(player_xy(&mut app).x, x);
    assert_eq!(
        *app.world().resource::<State<GameState>>().get(),
        GameState::Controls
    );
}

#[test]
fn classic_still_jumps_with_z_and_moves_with_arrows() {
    let saves = storage("controls_classic");
    let mut app = game(&saves);
    use_legacy_controls(&mut app);
    let before = player_xy(&mut app).x;
    hold(&mut app, &[KeyCode::ArrowRight], 4);
    assert!(player_xy(&mut app).x > before);
    hold(&mut app, &[KeyCode::KeyZ], 1);
    assert!(app.world().resource::<PlayerInput>().jump_pressed);
    assert!(!app.world().resource::<PlayerInput>().interact_pressed);
}

#[test]
fn a_corrupt_file_boots_the_hybrid_preset_and_is_not_overwritten() {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("controls_corrupt_boot");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap_or_else(|e| panic!("{e}"));
    let path = dir.join("controls.ron");
    fs::write(&path, "not a profile").unwrap_or_else(|e| panic!("{e}"));
    let before = fs::read(&path).unwrap_or_else(|e| panic!("{e}"));

    let data =
        dark_castle_rpg::core::resources::GameData::load(Path::new(env!("CARGO_MANIFEST_DIR")))
            .unwrap_or_else(|e| panic!("{e}"));
    let mut app = App::new();
    app.insert_resource(dark_castle_rpg::input::ControlsPath(path.clone()))
        .add_plugins((
            MinimalPlugins,
            bevy::state::app::StatesPlugin,
            bevy::input::InputPlugin,
        ))
        .insert_resource(data)
        .add_plugins(dark_castle_rpg::app::plugins::GamePlugin);
    app.update();

    let profile = app
        .world()
        .resource::<dark_castle_rpg::input::profile::ControlProfile>();
    assert!(profile.same_bindings(&hybrid()));
    assert!(
        app.world()
            .get_resource::<dark_castle_rpg::input::ControlsNotice>()
            .is_some()
    );
    assert_eq!(fs::read(&path).unwrap_or_else(|e| panic!("{e}")), before);
}
