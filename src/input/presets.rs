//! The two immutable presets. Editing one copies it into a custom profile.

use std::collections::BTreeMap;

use super::action::{ACTIONS, Action};
use super::binding::{Binding, PhysicalKey, PhysicalMouse};
use super::profile::{ActionBinding, ControlProfile, PROFILE_VERSION, ProfileOrigin};

fn key(key: PhysicalKey) -> Binding {
    Binding::Key(key)
}

fn slots(entries: [(Action, ActionBinding); Action::COUNT]) -> BTreeMap<Action, ActionBinding> {
    let map: BTreeMap<_, _> = entries.into_iter().collect();
    debug_assert_eq!(map.len(), ACTIONS.len());
    map
}

/// Keyboard and mouse. This is what a new install uses.
///
/// Guard moved from A to Q, and dodge from S to F, because A and S are now
/// movement. Aim up stays on the up arrow so command motions still have an
/// "up" that is not jump.
pub fn hybrid() -> ControlProfile {
    ControlProfile {
        version: PROFILE_VERSION,
        origin: ProfileOrigin::Hybrid,
        slots: slots([
            (Action::MoveLeft, ActionBinding::only(key(PhysicalKey::A))),
            (Action::MoveRight, ActionBinding::only(key(PhysicalKey::D))),
            (
                Action::AimUp,
                ActionBinding::only(key(PhysicalKey::ArrowUp)),
            ),
            (Action::AimDown, ActionBinding::only(key(PhysicalKey::S))),
            (
                Action::Jump,
                ActionBinding::pair(key(PhysicalKey::W), key(PhysicalKey::Space)),
            ),
            (
                Action::Run,
                ActionBinding::only(key(PhysicalKey::ShiftLeft)),
            ),
            (
                Action::Attack,
                ActionBinding::pair(Binding::Mouse(PhysicalMouse::Left), key(PhysicalKey::X)),
            ),
            (Action::Magic, ActionBinding::only(key(PhysicalKey::C))),
            (Action::Guard, ActionBinding::only(key(PhysicalKey::Q))),
            (Action::Dodge, ActionBinding::only(key(PhysicalKey::F))),
            (Action::Interact, ActionBinding::only(key(PhysicalKey::E))),
            (Action::Transform, ActionBinding::only(key(PhysicalKey::T))),
            (Action::Inventory, ActionBinding::only(key(PhysicalKey::I))),
            (Action::Map, ActionBinding::only(key(PhysicalKey::M))),
            (Action::Status, ActionBinding::only(key(PhysicalKey::Tab))),
            (Action::Respawn, ActionBinding::only(key(PhysicalKey::R))),
            (Action::QuickSave, ActionBinding::only(key(PhysicalKey::F5))),
            (Action::QuickLoad, ActionBinding::only(key(PhysicalKey::F9))),
            (
                Action::Confirm,
                ActionBinding::only(key(PhysicalKey::Enter)),
            ),
        ]),
    }
}

/// The scheme the game used before N2: arrows, Z jump, X attack, D interact.
pub fn classic() -> ControlProfile {
    ControlProfile {
        version: PROFILE_VERSION,
        origin: ProfileOrigin::Classic,
        slots: slots([
            (
                Action::MoveLeft,
                ActionBinding::only(key(PhysicalKey::ArrowLeft)),
            ),
            (
                Action::MoveRight,
                ActionBinding::only(key(PhysicalKey::ArrowRight)),
            ),
            (
                Action::AimUp,
                ActionBinding::only(key(PhysicalKey::ArrowUp)),
            ),
            (
                Action::AimDown,
                ActionBinding::only(key(PhysicalKey::ArrowDown)),
            ),
            (Action::Jump, ActionBinding::only(key(PhysicalKey::Z))),
            (
                Action::Run,
                ActionBinding::only(key(PhysicalKey::ShiftLeft)),
            ),
            (Action::Attack, ActionBinding::only(key(PhysicalKey::X))),
            (Action::Magic, ActionBinding::only(key(PhysicalKey::C))),
            (Action::Guard, ActionBinding::only(key(PhysicalKey::A))),
            (Action::Dodge, ActionBinding::only(key(PhysicalKey::S))),
            (Action::Interact, ActionBinding::only(key(PhysicalKey::D))),
            (Action::Transform, ActionBinding::only(key(PhysicalKey::T))),
            (Action::Inventory, ActionBinding::only(key(PhysicalKey::I))),
            (Action::Map, ActionBinding::only(key(PhysicalKey::M))),
            (Action::Status, ActionBinding::only(key(PhysicalKey::Tab))),
            (Action::Respawn, ActionBinding::only(key(PhysicalKey::R))),
            (Action::QuickSave, ActionBinding::only(key(PhysicalKey::F5))),
            (Action::QuickLoad, ActionBinding::only(key(PhysicalKey::F9))),
            (
                Action::Confirm,
                ActionBinding::only(key(PhysicalKey::Enter)),
            ),
        ]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_presets_are_complete_and_free_of_same_context_conflicts() {
        hybrid().validate().unwrap_or_else(|e| panic!("{e}"));
        classic().validate().unwrap_or_else(|e| panic!("{e}"));
    }

    #[test]
    fn classic_keeps_the_old_gameplay_keys() {
        let classic = classic();
        assert_eq!(
            classic.binding(Action::Jump).primary,
            Some(key(PhysicalKey::Z))
        );
        assert_eq!(
            classic.binding(Action::Attack).primary,
            Some(key(PhysicalKey::X))
        );
        assert_eq!(
            classic.binding(Action::Interact).primary,
            Some(key(PhysicalKey::D))
        );
        assert_eq!(
            classic.binding(Action::Guard).primary,
            Some(key(PhysicalKey::A))
        );
        assert_eq!(
            classic.binding(Action::Dodge).primary,
            Some(key(PhysicalKey::S))
        );
    }

    #[test]
    fn hybrid_jump_is_not_aim_up_and_interact_is_not_move_right() {
        let hybrid = hybrid();
        assert_eq!(
            hybrid.binding(Action::Jump).primary,
            Some(key(PhysicalKey::W))
        );
        assert_eq!(
            hybrid.binding(Action::Jump).alternate,
            Some(key(PhysicalKey::Space))
        );
        assert_eq!(
            hybrid.binding(Action::AimUp).primary,
            Some(key(PhysicalKey::ArrowUp))
        );
        assert_eq!(
            hybrid.binding(Action::MoveRight).primary,
            Some(key(PhysicalKey::D))
        );
        assert_eq!(
            hybrid.binding(Action::Interact).primary,
            Some(key(PhysicalKey::E))
        );
        assert_eq!(
            hybrid.binding(Action::Attack).primary,
            Some(Binding::Mouse(PhysicalMouse::Left))
        );
    }
}
