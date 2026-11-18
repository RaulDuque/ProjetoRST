//! Controls screen. Opens with Escape while playing. The simulation is
//! paused because gameplay sets run only in `GameState::Playing`.
//!
//! Navigation uses the arrow keys and Enter, not the gameplay bindings, so
//! a broken profile can still be repaired. Escape cancels a capture; when
//! nothing is being captured it closes the screen and discards the draft.
//! Apply writes the local controls file and leaves the character save alone.

use bevy::input::mouse::MouseButton;
use bevy::prelude::*;

use crate::app::states::GameState;
use crate::input::action::{ACTIONS, Action};
use crate::input::binding::{Binding, PhysicalKey, PhysicalMouse};
use crate::input::presets::{classic, hybrid};
use crate::input::profile::{ControlProfile, Slot};
use crate::input::store::save_atomic;
use crate::input::{ControlsNotice, ControlsPath};
use crate::ui::theme::{FONT_SMALL, PANEL_BG, PANEL_BORDER, TEXT_DIM, TEXT_GOLD, text};

const FOOTER: [&str; 5] = [
    "Apply",
    "Hybrid preset",
    "Classic preset",
    "Restore saved",
    "Close",
];

#[derive(Component)]
pub struct ControlsRoot;

#[derive(Component)]
pub struct ControlsBody;

#[derive(Resource)]
pub struct ControlsDraft {
    profile: ControlProfile,
    row: usize,
    slot: Slot,
    listening: bool,
    /// The Enter that started a capture must not become the binding.
    armed: bool,
    message: String,
}

pub fn drive_controls(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
    path: Option<Res<ControlsPath>>,
    applied: Option<Res<ControlProfile>>,
    draft: Option<ResMut<ControlsDraft>>,
) {
    match state.get() {
        GameState::Playing if keys.just_pressed(KeyCode::Escape) => {
            next.set(GameState::Controls);
        }
        GameState::Controls => {
            let Some(mut draft) = draft else {
                return;
            };
            if keys.just_pressed(KeyCode::Escape) {
                if draft.listening {
                    draft.listening = false;
                    draft.armed = false;
                    draft.message = "Capture cancelled.".to_string();
                } else {
                    next.set(GameState::Playing);
                }
                return;
            }
            if draft.listening {
                capture(&mut draft, &keys, &mouse);
                return;
            }
            move_cursor(&mut draft, &keys);
            if keys.just_pressed(KeyCode::Backspace) {
                clear_slot(&mut draft);
            }
            if keys.just_pressed(KeyCode::Enter) {
                activate_row(
                    &mut commands,
                    &mut draft,
                    &mut next,
                    path.as_deref(),
                    applied.as_deref(),
                );
            }
        }
        _ => {}
    }
}

fn move_cursor(draft: &mut ControlsDraft, keys: &ButtonInput<KeyCode>) {
    let last = ACTIONS.len() + FOOTER.len() - 1;
    if keys.just_pressed(KeyCode::ArrowDown) {
        draft.row = (draft.row + 1).min(last);
    }
    if keys.just_pressed(KeyCode::ArrowUp) {
        draft.row = draft.row.saturating_sub(1);
    }
    if draft.row < ACTIONS.len() {
        if keys.just_pressed(KeyCode::ArrowRight) {
            draft.slot = Slot::Alternate;
        }
        if keys.just_pressed(KeyCode::ArrowLeft) {
            draft.slot = Slot::Primary;
        }
    }
}

fn clear_slot(draft: &mut ControlsDraft) {
    if draft.row >= ACTIONS.len() {
        return;
    }
    let action = ACTIONS[draft.row];
    match draft.profile.clear(action, draft.slot) {
        Ok(()) => draft.message = format!("Cleared {}.", action.label()),
        Err(error) => draft.message = error.message(),
    }
}

fn activate_row(
    commands: &mut Commands,
    draft: &mut ControlsDraft,
    next: &mut NextState<GameState>,
    path: Option<&ControlsPath>,
    applied: Option<&ControlProfile>,
) {
    if draft.row < ACTIONS.len() {
        draft.listening = true;
        draft.armed = true;
        draft.message = "Press a key or a mouse button. Escape cancels.".to_string();
        return;
    }
    match draft.row - ACTIONS.len() {
        0 => apply(commands, draft, next, path),
        1 => {
            draft.profile = hybrid();
            draft.message = "Hybrid preset loaded. Apply to keep it.".to_string();
        }
        2 => {
            draft.profile = classic();
            draft.message = "Classic preset loaded. Apply to keep it.".to_string();
        }
        3 => {
            if let Some(applied) = applied {
                draft.profile = applied.clone();
                draft.message = "Restored the controls that are saved.".to_string();
            }
        }
        _ => next.set(GameState::Playing),
    }
}

fn apply(
    commands: &mut Commands,
    draft: &mut ControlsDraft,
    next: &mut NextState<GameState>,
    path: Option<&ControlsPath>,
) {
    if let Err(error) = draft.profile.validate() {
        draft.message = error;
        return;
    }
    let Some(path) = path else {
        draft.message = "No controls file path is set.".to_string();
        return;
    };
    if let Err(error) = save_atomic(&path.0, &draft.profile) {
        draft.message = format!("Could not save controls: {error}");
        return;
    }
    commands.insert_resource(draft.profile.clone());
    commands.remove_resource::<ControlsNotice>();
    next.set(GameState::Playing);
}

fn capture(
    draft: &mut ControlsDraft,
    keys: &ButtonInput<KeyCode>,
    mouse: &ButtonInput<MouseButton>,
) {
    if draft.armed {
        draft.armed = false;
        return;
    }
    let action = ACTIONS[draft.row];
    let slot = draft.slot;
    if let Some(button) = mouse.get_just_pressed().copied().next() {
        match PhysicalMouse::from_button(button) {
            Some(button) => finish(draft, action, slot, Binding::Mouse(button)),
            None => draft.message = "That mouse button cannot be bound.".to_string(),
        }
        return;
    }
    let Some(code) = keys.get_just_pressed().copied().next() else {
        return;
    };
    if code == KeyCode::Escape {
        return;
    }
    match PhysicalKey::from_keycode(code) {
        Some(key) => finish(draft, action, slot, Binding::Key(key)),
        None => {
            draft.listening = false;
            draft.message = "That key cannot be bound.".to_string();
        }
    }
}

fn finish(draft: &mut ControlsDraft, action: Action, slot: Slot, binding: Binding) {
    draft.listening = false;
    match draft.profile.assign(action, slot, binding) {
        Ok(()) => {
            draft.message = format!("{} set to {}.", action.label(), binding.label());
        }
        Err(error) => draft.message = error.message(),
    }
}

pub fn spawn_controls_screen(mut commands: Commands, applied: Res<ControlProfile>) {
    commands.insert_resource(ControlsDraft {
        profile: applied.clone(),
        row: 0,
        slot: Slot::Primary,
        listening: false,
        armed: false,
        message: "Arrows move. Left/Right picks a slot. Enter captures. Backspace clears."
            .to_string(),
    });
    commands
        .spawn((
            ControlsRoot,
            DespawnOnExit(GameState::Controls),
            Node {
                position_type: PositionType::Absolute,
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.72)),
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: px(6),
                        padding: UiRect::all(px(16)),
                        border: UiRect::all(px(2)),
                        width: px(760),
                        ..default()
                    },
                    BackgroundColor(PANEL_BG),
                    BorderColor::all(PANEL_BORDER),
                ))
                .with_children(|panel| {
                    panel.spawn(text("CONTROLS", 24.0, TEXT_GOLD));
                    panel.spawn((ControlsBody, text("", FONT_SMALL, TEXT_DIM)));
                });
        });
}

pub fn update_controls_screen(
    draft: Res<ControlsDraft>,
    mut body: Query<&mut Text, With<ControlsBody>>,
) {
    let Ok(mut body) = body.single_mut() else {
        return;
    };
    let mut lines = Vec::new();
    lines.push(format!("Preset: {:?}", draft.profile.origin));
    lines.push(String::new());
    for (index, action) in ACTIONS.iter().enumerate() {
        let entry = draft.profile.binding(*action);
        let mark = if index == draft.row { ">" } else { " " };
        let primary = highlight(
            entry.primary,
            draft.slot == Slot::Primary && index == draft.row,
        );
        let alternate = highlight(
            entry.alternate,
            draft.slot == Slot::Alternate && index == draft.row,
        );
        lines.push(format!(
            "{mark} {:<22} {primary:<14} {alternate}",
            action.label()
        ));
    }
    lines.push(String::new());
    for (index, label) in FOOTER.iter().enumerate() {
        let mark = if draft.row == ACTIONS.len() + index {
            ">"
        } else {
            " "
        };
        lines.push(format!("{mark} {label}"));
    }
    lines.push(String::new());
    if draft.listening {
        lines.push("Listening… Escape cancels this capture.".to_string());
    }
    lines.push(draft.message.clone());
    body.0 = lines.join("\n");
}

pub fn close_controls_screen(mut commands: Commands) {
    commands.remove_resource::<ControlsDraft>();
}

fn highlight(binding: Option<Binding>, selected: bool) -> String {
    let label = binding.map(|binding| binding.label()).unwrap_or("—");
    if selected {
        format!("[{label}]")
    } else {
        label.to_string()
    }
}
