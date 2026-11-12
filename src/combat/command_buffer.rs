//! Input vocabulary and the timed buffer of recent player inputs.
//!
//! Pure module: no Bevy types. Directions are stored *relative to the
//! facing at the moment they were pressed* (forward/back), so the same
//! command works whichever way the character looks.

use std::collections::VecDeque;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::player::state::Facing;

/// Maximum number of inputs remembered.
pub const DEFAULT_BUFFER_CAPACITY: usize = 32;
/// Inputs older than this (seconds) are discarded.
pub const DEFAULT_BUFFER_MAX_AGE: f64 = 1.5;

/// A stick/d-pad position relative to the facing direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Motion {
    Neutral,
    Up,
    Down,
    Forward,
    Back,
    UpForward,
    UpBack,
    DownForward,
    DownBack,
}

impl Motion {
    /// Builds a relative motion from absolute axes (`x`: -1 left / +1 right,
    /// `y`: -1 down / +1 up) and the current facing.
    pub fn from_axes(x: i8, y: i8, facing: Facing) -> Motion {
        let forward = match facing {
            Facing::Right => x.signum(),
            Facing::Left => -x.signum(),
        };
        match (forward, y.signum()) {
            (0, 0) => Motion::Neutral,
            (0, 1) => Motion::Up,
            (0, -1) => Motion::Down,
            (1, 0) => Motion::Forward,
            (-1, 0) => Motion::Back,
            (1, 1) => Motion::UpForward,
            (-1, 1) => Motion::UpBack,
            (1, -1) => Motion::DownForward,
            _ => Motion::DownBack,
        }
    }

    /// Arrow glyph as if the character faced right.
    pub fn glyph(self) -> &'static str {
        match self {
            Motion::Neutral => "N",
            Motion::Up => "↑",
            Motion::Down => "↓",
            Motion::Forward => "→",
            Motion::Back => "←",
            Motion::UpForward => "↗",
            Motion::UpBack => "↖",
            Motion::DownForward => "↘",
            Motion::DownBack => "↙",
        }
    }
}

/// Action buttons that can finish a command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CommandButton {
    Attack,
    Magic,
    Jump,
    Guard,
}

impl CommandButton {
    pub fn label(self) -> &'static str {
        match self {
            CommandButton::Attack => "ATTACK",
            CommandButton::Magic => "MAGIC",
            CommandButton::Jump => "JUMP",
            CommandButton::Guard => "GUARD",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InputToken {
    Motion(Motion),
    Button(CommandButton),
}

impl fmt::Display for InputToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputToken::Motion(m) => f.write_str(m.glyph()),
            InputToken::Button(b) => f.write_str(b.label()),
        }
    }
}

/// One recorded input and the instant (seconds) it happened.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimedInput {
    pub token: InputToken,
    pub time: f64,
}

/// Rolling buffer of recent inputs.
///
/// Motions are recorded only when the stick *changes* position, so holding
/// a direction produces a single entry.
#[derive(Debug, Clone)]
pub struct CommandBuffer {
    entries: VecDeque<TimedInput>,
    capacity: usize,
    max_age: f64,
    current_motion: Motion,
}

impl Default for CommandBuffer {
    fn default() -> Self {
        Self::new(DEFAULT_BUFFER_CAPACITY, DEFAULT_BUFFER_MAX_AGE)
    }
}

impl CommandBuffer {
    pub fn new(capacity: usize, max_age: f64) -> Self {
        Self {
            entries: VecDeque::with_capacity(capacity),
            capacity: capacity.max(1),
            max_age,
            current_motion: Motion::Neutral,
        }
    }

    pub fn entries(&self) -> impl DoubleEndedIterator<Item = &TimedInput> + ExactSizeIterator {
        self.entries.iter()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Records the stick position. Returns whether a new entry was added.
    pub fn record_motion(&mut self, motion: Motion, time: f64) -> bool {
        if motion == self.current_motion {
            return false;
        }
        self.current_motion = motion;
        self.push(InputToken::Motion(motion), time);
        true
    }

    pub fn record_button(&mut self, button: CommandButton, time: f64) {
        self.push(InputToken::Button(button), time);
    }

    fn push(&mut self, token: InputToken, time: f64) {
        if self.entries.len() == self.capacity {
            self.entries.pop_front();
        }
        self.entries.push_back(TimedInput { token, time });
    }

    /// Drops entries older than the configured maximum age.
    pub fn prune(&mut self, now: f64) {
        while let Some(front) = self.entries.front() {
            if now - front.time > self.max_age {
                self.entries.pop_front();
            } else {
                break;
            }
        }
    }

    /// Forgets every recorded input (used after a command is consumed or
    /// cancelled, so the same inputs can never trigger twice). The current
    /// stick position is kept, so a still-held direction is not re-recorded.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
