//! Collapse every binding of one action into a single press, release or hold.
//!
//! Two keys for Jump must not produce two jumps. Releasing one of them while
//! the other stays down must not produce a release.

/// What the devices did for one action during this frame, before the edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeldState {
    /// A binding is still down from before this frame.
    pub steady: bool,
    /// A binding went down this frame, including a release followed by a press.
    pub retriggered: bool,
}

/// One frame of a logical action, after every binding was combined.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActionEdge {
    pub pressed: bool,
    pub just_pressed: bool,
    pub just_released: bool,
}

/// Combines every binding of one action.
///
/// A second binding going down while another stays down is not a new press.
/// Releasing one of two bindings is not a release. Releasing and pressing the
/// same binding again before the sample is a new press, which is what a
/// one-frame tap in the tests does.
pub fn combine(state: HeldState, was_pressed: bool) -> (ActionEdge, bool) {
    let pressed = state.steady || state.retriggered;
    let edge = ActionEdge {
        pressed,
        just_pressed: (pressed && !was_pressed) || (state.retriggered && !state.steady),
        just_released: !pressed && was_pressed,
    };
    (edge, pressed)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn held() -> HeldState {
        HeldState {
            steady: true,
            retriggered: false,
        }
    }

    fn up() -> HeldState {
        HeldState {
            steady: false,
            retriggered: false,
        }
    }

    #[test]
    fn a_second_binding_pressed_while_the_first_is_held_is_not_a_new_press() {
        let (first, was) = combine(held(), false);
        assert!(first.just_pressed);
        // The other key goes down. The action was already held, so this is
        // not a second jump and not a release.
        let (second, was) = combine(held(), was);
        assert!(second.pressed);
        assert!(!second.just_pressed);
        assert!(!second.just_released);
        let _ = was;
    }

    #[test]
    fn releasing_one_of_two_bindings_does_not_release_the_action() {
        let (_edge, was) = combine(held(), false);
        let (still, was) = combine(held(), was);
        assert!(!still.just_released);
        let (released, _) = combine(up(), was);
        assert!(released.just_released);
        assert!(!released.pressed);
    }

    #[test]
    fn two_bindings_arriving_on_the_same_frame_are_one_press() {
        let (edge, _) = combine(held(), false);
        assert!(edge.just_pressed);
        assert!(!edge.just_released);
    }

    #[test]
    fn a_binding_released_and_pressed_again_before_the_sample_is_a_new_press() {
        let (_edge, was) = combine(held(), false);
        let (again, _) = combine(
            HeldState {
                steady: false,
                retriggered: true,
            },
            was,
        );
        assert!(again.just_pressed);
        assert!(!again.just_released);
    }

    #[test]
    fn flickering_one_binding_while_another_stays_down_is_not_a_new_press() {
        let (_edge, was) = combine(held(), false);
        let (still, _) = combine(
            HeldState {
                steady: true,
                retriggered: true,
            },
            was,
        );
        assert!(still.pressed);
        assert!(!still.just_pressed);
        assert!(!still.just_released);
    }
}
