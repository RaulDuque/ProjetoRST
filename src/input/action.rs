//! Logical actions. Gameplay and menus read these, never a physical key.

use serde::{Deserialize, Serialize};

/// Where an action is live. The same physical control may be used in two
/// different contexts (Enter confirms a menu and can also be a gameplay key).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InputContext {
    /// Movement, combat and world use while the simulation is running.
    Gameplay,
    /// Menus, dialogue and shops.
    Menu,
}

/// One thing the player can ask the game to do.
///
/// The order is the on-screen order and the index into the edge arrays.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Action {
    MoveLeft,
    MoveRight,
    /// Command "up". This is not jump.
    AimUp,
    /// Command "down" and crouch while grounded.
    AimDown,
    Jump,
    Run,
    Attack,
    Magic,
    Guard,
    Dodge,
    Interact,
    Transform,
    Inventory,
    Map,
    Status,
    Respawn,
    QuickSave,
    QuickLoad,
    Confirm,
}

pub const ACTIONS: [Action; Action::COUNT] = [
    Action::MoveLeft,
    Action::MoveRight,
    Action::AimUp,
    Action::AimDown,
    Action::Jump,
    Action::Run,
    Action::Attack,
    Action::Magic,
    Action::Guard,
    Action::Dodge,
    Action::Interact,
    Action::Transform,
    Action::Inventory,
    Action::Map,
    Action::Status,
    Action::Respawn,
    Action::QuickSave,
    Action::QuickLoad,
    Action::Confirm,
];

impl Action {
    pub const COUNT: usize = 19;

    pub fn index(self) -> usize {
        match self {
            Action::MoveLeft => 0,
            Action::MoveRight => 1,
            Action::AimUp => 2,
            Action::AimDown => 3,
            Action::Jump => 4,
            Action::Run => 5,
            Action::Attack => 6,
            Action::Magic => 7,
            Action::Guard => 8,
            Action::Dodge => 9,
            Action::Interact => 10,
            Action::Transform => 11,
            Action::Inventory => 12,
            Action::Map => 13,
            Action::Status => 14,
            Action::Respawn => 15,
            Action::QuickSave => 16,
            Action::QuickLoad => 17,
            Action::Confirm => 18,
        }
    }

    pub fn context(self) -> InputContext {
        match self {
            Action::Confirm => InputContext::Menu,
            _ => InputContext::Gameplay,
        }
    }

    /// An essential action must keep at least one binding.
    pub fn essential(self) -> bool {
        matches!(
            self,
            Action::MoveLeft
                | Action::MoveRight
                | Action::AimUp
                | Action::AimDown
                | Action::Jump
                | Action::Attack
                | Action::Interact
                | Action::Confirm
        )
    }

    pub fn label(self) -> &'static str {
        match self {
            Action::MoveLeft => "Move left",
            Action::MoveRight => "Move right",
            Action::AimUp => "Aim up",
            Action::AimDown => "Aim down / crouch",
            Action::Jump => "Jump",
            Action::Run => "Run",
            Action::Attack => "Attack",
            Action::Magic => "Magic",
            Action::Guard => "Guard",
            Action::Dodge => "Dodge",
            Action::Interact => "Interact",
            Action::Transform => "Spectral form",
            Action::Inventory => "Inventory",
            Action::Map => "Map",
            Action::Status => "Status",
            Action::Respawn => "Respawn",
            Action::QuickSave => "Quick save",
            Action::QuickLoad => "Quick load",
            Action::Confirm => "Confirm",
        }
    }
}
