//! A conversation in progress (V2.1). Pure module: no Bevy types.
//!
//! [`Conversation`] walks the node graph of [`NpcLibrary`]: lines first, then
//! the choices that hold (or `next`, or the end). It never touches the game:
//! the caller acts on [`Step`] (open a shop, close the box).

use super::definitions::{Choice, ChoiceAction, DialogueFacts, NpcLibrary, all_hold};

/// What the player sees now.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conversation {
    pub npc: String,
    pub node: String,
    /// Line of the node being shown.
    pub line: usize,
    /// Selected choice (index into the visible choices).
    pub cursor: usize,
}

/// Result of confirming.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    /// Still talking (next line or another node).
    Continue,
    OpenShop,
    End,
}

impl Conversation {
    /// Starts with the NPC's first greeting that holds.
    pub fn start(npc: &str, library: &NpcLibrary, facts: &DialogueFacts) -> Option<Self> {
        let node = library.greeting(npc, facts)?;
        Some(Self {
            npc: npc.to_owned(),
            node: node.id.clone(),
            line: 0,
            cursor: 0,
        })
    }

    pub fn text<'a>(&self, library: &'a NpcLibrary) -> Option<&'a str> {
        library
            .node(&self.node)
            .and_then(|n| n.lines.get(self.line))
            .map(String::as_str)
    }

    /// Whether the last line of the node is shown.
    pub fn at_last_line(&self, library: &NpcLibrary) -> bool {
        library
            .node(&self.node)
            .is_none_or(|n| self.line + 1 >= n.lines.len())
    }

    /// Choices visible now (only after the last line).
    pub fn choices<'a>(&self, library: &'a NpcLibrary, facts: &DialogueFacts) -> Vec<&'a Choice> {
        if !self.at_last_line(library) {
            return Vec::new();
        }
        library.node(&self.node).map_or_else(Vec::new, |n| {
            n.choices
                .iter()
                .filter(|c| all_hold(&c.when, facts))
                .collect()
        })
    }

    pub fn move_cursor(&mut self, delta: i32, library: &NpcLibrary, facts: &DialogueFacts) {
        let count = self.choices(library, facts).len();
        if count == 0 {
            self.cursor = 0;
            return;
        }
        let next = (self.cursor as i32 + delta).clamp(0, count as i32 - 1);
        self.cursor = next as usize;
    }

    /// Enter/D: next line, or the selected choice, or `next`, or the end.
    pub fn confirm(&mut self, library: &NpcLibrary, facts: &DialogueFacts) -> Step {
        let Some(node) = library.node(&self.node) else {
            return Step::End;
        };
        if !self.at_last_line(library) {
            self.line += 1;
            return Step::Continue;
        }
        let choices = self.choices(library, facts);
        if let Some(choice) = choices.get(self.cursor.min(choices.len().saturating_sub(1))) {
            return match &choice.action {
                ChoiceAction::Goto(target) => self.go(target),
                ChoiceAction::OpenShop => Step::OpenShop,
                ChoiceAction::End => Step::End,
            };
        }
        match &node.next {
            Some(next) => {
                let next = next.clone();
                self.go(&next)
            }
            None => Step::End,
        }
    }

    fn go(&mut self, node: &str) -> Step {
        self.node = node.to_owned();
        self.line = 0;
        self.cursor = 0;
        Step::Continue
    }
}
