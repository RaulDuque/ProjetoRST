//! Command notation parser and sequence recogniser.
//!
//! Pure module: no Bevy types. It only answers "which command did the
//! player input?"; it never applies effects, spends resources or checks
//! cooldowns (that is `skills::execution`).
//!
//! # Notation
//!
//! A command is a list of motions followed by one button, written as if the
//! character faced right. Tokens are separated by spaces, commas or `+`:
//!
//! ```text
//! "↓ ↘ → + ATTACK"      "D DF F + ATTACK"      "F B F ATTACK"
//! ```
//!
//! Motions: `↑ U`, `↓ D`, `→ F`, `← B`, `↗ UF`, `↖ UB`, `↘ DF`, `↙ DB`, `N` (neutral).
//! Buttons: `ATTACK`/`ATK`, `MAGIC`/`MAG`, `JUMP`, `GUARD`.

use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::command_buffer::{CommandButton, InputToken, Motion, TimedInput};

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum NotationError {
    #[error("empty command notation")]
    Empty,
    #[error("unknown token `{0}`")]
    UnknownToken(String),
    #[error("command must end with a button")]
    MissingButton,
    #[error("button `{0}` must be the last token")]
    ButtonNotLast(String),
}

/// A parsed command: motions then a finishing button.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CommandSequence {
    motions: Vec<Motion>,
    button: CommandButton,
}

impl CommandSequence {
    pub fn new(motions: Vec<Motion>, button: CommandButton) -> Self {
        Self { motions, button }
    }

    pub fn motions(&self) -> &[Motion] {
        &self.motions
    }

    pub fn button(&self) -> CommandButton {
        self.button
    }

    /// Total number of inputs, used to prefer more specific commands.
    pub fn len(&self) -> usize {
        self.motions.len() + 1
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    /// Parses the notation described in the module docs.
    pub fn parse(notation: &str) -> Result<Self, NotationError> {
        let tokens: Vec<&str> = notation
            .split(|c: char| c.is_whitespace() || c == '+' || c == ',')
            .filter(|t| !t.is_empty())
            .collect();
        if tokens.is_empty() {
            return Err(NotationError::Empty);
        }

        let mut motions = Vec::new();
        let mut button = None;
        for raw in tokens {
            if let Some(b) = button {
                let b: CommandButton = b;
                return Err(NotationError::ButtonNotLast(b.label().to_owned()));
            }
            match parse_token(raw)? {
                InputToken::Motion(m) => motions.push(m),
                InputToken::Button(b) => button = Some(b),
            }
        }
        let button = button.ok_or(NotationError::MissingButton)?;
        Ok(Self { motions, button })
    }
}

fn parse_token(raw: &str) -> Result<InputToken, NotationError> {
    let upper = raw.to_uppercase();
    let token = match upper.as_str() {
        "↑" | "U" | "UP" => InputToken::Motion(Motion::Up),
        "↓" | "D" | "DOWN" => InputToken::Motion(Motion::Down),
        "→" | "F" | "FORWARD" => InputToken::Motion(Motion::Forward),
        "←" | "B" | "BACK" => InputToken::Motion(Motion::Back),
        "↗" | "UF" | "UP_FORWARD" => InputToken::Motion(Motion::UpForward),
        "↖" | "UB" | "UP_BACK" => InputToken::Motion(Motion::UpBack),
        "↘" | "DF" | "DOWN_FORWARD" => InputToken::Motion(Motion::DownForward),
        "↙" | "DB" | "DOWN_BACK" => InputToken::Motion(Motion::DownBack),
        "N" | "NEUTRAL" => InputToken::Motion(Motion::Neutral),
        "ATTACK" | "ATK" => InputToken::Button(CommandButton::Attack),
        "MAGIC" | "MAG" => InputToken::Button(CommandButton::Magic),
        "JUMP" => InputToken::Button(CommandButton::Jump),
        "GUARD" => InputToken::Button(CommandButton::Guard),
        _ => return Err(NotationError::UnknownToken(raw.to_owned())),
    };
    Ok(token)
}

impl CommandSequence {
    /// ASCII rendering (`"D DF F + ATTACK"`), for fonts without arrow glyphs.
    pub fn ascii(&self) -> String {
        let mut out = String::new();
        for m in &self.motions {
            out.push_str(m.ascii());
            out.push(' ');
        }
        out.push_str("+ ");
        out.push_str(self.button.label());
        out
    }
}

impl TryFrom<String> for CommandSequence {
    type Error = NotationError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        CommandSequence::parse(&value)
    }
}

impl From<CommandSequence> for String {
    fn from(value: CommandSequence) -> Self {
        value.to_string()
    }
}

impl fmt::Display for CommandSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for m in &self.motions {
            write!(f, "{} ", m.glyph())?;
        }
        write!(f, "+ {}", self.button.label())
    }
}

/// Timing tolerance of one command.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CommandTiming {
    /// Max seconds from the first motion to the button.
    pub total_window: f64,
    /// Max seconds between two consecutive inputs of the command.
    pub step_window: f64,
    /// Unrelated inputs tolerated in the middle of the command (small
    /// execution variations such as an accidental diagonal).
    pub max_extra_inputs: usize,
}

impl Default for CommandTiming {
    fn default() -> Self {
        Self {
            total_window: 0.6,
            step_window: 0.3,
            max_extra_inputs: 1,
        }
    }
}

/// How well the buffer matches one sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SequenceMatch {
    /// Every input present, in order and on time.
    Complete,
    /// Every input present and in order, but too slow.
    TooSlow,
    /// The last `matched` motions were input, the earlier ones missing.
    Partial { matched: usize },
    /// Wrong button or nothing in common.
    NoMatch,
}

/// Matches the newest entries of `inputs` (oldest first) against `sequence`.
///
/// The newest entry must be the sequence's button. Motions are then matched
/// backwards as a subsequence, tolerating up to `max_extra_inputs` unrelated
/// entries. Neutral positions are ignored unless the sequence uses them, and
/// matching never looks past an earlier button press.
pub fn match_sequence(
    inputs: &[TimedInput],
    sequence: &CommandSequence,
    timing: &CommandTiming,
) -> SequenceMatch {
    let Some((last, history)) = inputs.split_last() else {
        return SequenceMatch::NoMatch;
    };
    if last.token != InputToken::Button(sequence.button) {
        return SequenceMatch::NoMatch;
    }

    let motions = sequence.motions();
    let uses_neutral = motions.contains(&Motion::Neutral);
    let mut matched_times: Vec<f64> = Vec::with_capacity(motions.len());
    let mut remaining = motions.len();
    let mut extras = 0usize;

    for entry in history.iter().rev() {
        if remaining == 0 {
            break;
        }
        let motion = match entry.token {
            InputToken::Motion(m) => m,
            InputToken::Button(_) => break,
        };
        if motion == Motion::Neutral && !uses_neutral {
            continue;
        }
        if motion == motions[remaining - 1] {
            matched_times.push(entry.time);
            remaining -= 1;
        } else {
            extras += 1;
            if extras > timing.max_extra_inputs {
                break;
            }
        }
    }

    let matched = motions.len() - remaining;
    if remaining > 0 {
        // Half of the motions (rounded up) must be present to call it an
        // incomplete attempt rather than an unrelated input.
        let threshold = motions.len().div_ceil(2);
        return if matched > 0 && matched >= threshold {
            SequenceMatch::Partial { matched }
        } else {
            SequenceMatch::NoMatch
        };
    }

    // matched_times is newest-first; add the button and check the gaps.
    let mut times = Vec::with_capacity(matched_times.len() + 1);
    times.push(last.time);
    times.extend(matched_times);
    let first = times.last().copied().unwrap_or(last.time);
    let total_ok = last.time - first <= timing.total_window;
    let steps_ok = times.windows(2).all(|w| w[0] - w[1] <= timing.step_window);
    if total_ok && steps_ok {
        SequenceMatch::Complete
    } else {
        SequenceMatch::TooSlow
    }
}

/// A command the recogniser can detect.
#[derive(Debug, Clone, PartialEq)]
pub struct CommandPattern {
    pub id: String,
    pub sequence: CommandSequence,
    pub timing: CommandTiming,
    /// Higher wins when several commands match.
    pub priority: i32,
}

/// Result of [`recognize`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Recognition {
    /// All complete matches, best first (priority, then longer sequence).
    Matched(Vec<String>),
    /// A command was attempted but inputs were missing.
    Incomplete {
        id: String,
        matched: usize,
        required: usize,
    },
    /// A command was input in order but outside its time window.
    TooSlow { id: String },
    /// The button press does not correspond to any command.
    Invalid,
}

/// Recognises the command finished by the newest input of `inputs`.
pub fn recognize(inputs: &[TimedInput], patterns: &[CommandPattern]) -> Recognition {
    let mut ranked: Vec<&CommandPattern> = patterns.iter().collect();
    ranked.sort_by(|a, b| {
        b.priority
            .cmp(&a.priority)
            .then(b.sequence.len().cmp(&a.sequence.len()))
            .then(a.id.cmp(&b.id))
    });

    let mut complete = Vec::new();
    let mut too_slow: Option<&CommandPattern> = None;
    let mut best_partial: Option<(&CommandPattern, usize)> = None;

    for pattern in ranked {
        match match_sequence(inputs, &pattern.sequence, &pattern.timing) {
            SequenceMatch::Complete => complete.push(pattern.id.clone()),
            SequenceMatch::TooSlow => {
                too_slow.get_or_insert(pattern);
            }
            SequenceMatch::Partial { matched } => {
                let better = best_partial.is_none_or(|(p, m)| {
                    // Prefer the attempt closest to completion.
                    matched * p.sequence.motions().len() > m * pattern.sequence.motions().len()
                });
                if better {
                    best_partial = Some((pattern, matched));
                }
            }
            SequenceMatch::NoMatch => {}
        }
    }

    if !complete.is_empty() {
        Recognition::Matched(complete)
    } else if let Some(p) = too_slow {
        Recognition::TooSlow { id: p.id.clone() }
    } else if let Some((p, matched)) = best_partial {
        Recognition::Incomplete {
            id: p.id.clone(),
            matched,
            required: p.sequence.motions().len(),
        }
    } else {
        Recognition::Invalid
    }
}
