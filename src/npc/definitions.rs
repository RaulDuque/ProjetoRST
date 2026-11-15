//! NPCs and their dialogues, as data (V2.1). Pure module: no Bevy types.
//!
//! - NPCs: `assets/data/npcs/*.ron` (a list of [`NpcDefinition`] per file);
//! - dialogues: `assets/data/dialogues/*.ron` (a list of [`DialogueNode`]).
//!
//! A dialogue is a small graph of nodes: each node has lines said in order,
//! then either choices or a `next` node (or the end). Conditions are a fixed,
//! typed set ([`DialogueCondition`]) evaluated by matching: nothing in these
//! files is ever executed. Details: docs/NPC_COMMERCE_V2_1.md.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::world::graph::{Ability, Capabilities};

/// What kind of character an NPC is (presentation and future services).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NpcKind {
    Villager,
    Merchant,
    Smith,
    Informant,
    /// Reserved for future quests (no quest system yet).
    QuestGiver,
}

/// A condition of a greeting or a choice. All conditions of a list must
/// hold. Only facts the game already keeps are used: no new flags.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DialogueCondition {
    /// A relic ability was obtained (e.g. `VeilStep`, the Relic of the Veil).
    Ability(Ability),
    /// A world flag was set (lever, sigil).
    Flag(String),
    /// The player carries (bag) or wears this item.
    Item(String),
    /// This room was visited.
    Visited(String),
    /// Some room of this region was visited.
    RegionVisited(String),
}

/// What the game knows when a condition is checked.
#[derive(Debug, Clone, Default)]
pub struct DialogueFacts {
    pub capabilities: Capabilities,
    pub items: BTreeSet<String>,
    pub visited_rooms: BTreeSet<String>,
    pub visited_regions: BTreeSet<String>,
}

impl DialogueCondition {
    pub fn holds(&self, facts: &DialogueFacts) -> bool {
        match self {
            DialogueCondition::Ability(a) => facts.capabilities.abilities.contains(a),
            DialogueCondition::Flag(f) => facts.capabilities.flags.contains(f),
            DialogueCondition::Item(i) => facts.items.contains(i),
            DialogueCondition::Visited(r) => facts.visited_rooms.contains(r),
            DialogueCondition::RegionVisited(r) => facts.visited_regions.contains(r),
        }
    }
}

pub fn all_hold(conditions: &[DialogueCondition], facts: &DialogueFacts) -> bool {
    conditions.iter().all(|c| c.holds(facts))
}

/// The node an NPC starts with when its conditions hold. Greetings are
/// checked in order; the first that holds is used (put the specific ones
/// first and a greeting without conditions last).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Greeting {
    #[serde(default)]
    pub when: Vec<DialogueCondition>,
    pub node: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NpcDefinition {
    /// Unique in the game.
    pub id: String,
    pub name: String,
    pub kind: NpcKind,
    /// Room id of the world graph.
    pub room: String,
    /// Body centre in room coordinates (feet on a floor at y 40: y = 40 + h/2).
    pub position: (f32, f32),
    /// Placeholder body (world units) and colour (sRGB 0..1).
    pub size: (f32, f32),
    pub color: (f32, f32, f32),
    pub greetings: Vec<Greeting>,
    /// Shop this NPC runs (`assets/data/shops/`), if any.
    #[serde(default)]
    pub shop: Option<String>,
    /// V2.2: provisional figure (outfit, headwear, prop, accent). Absent:
    /// a plain figure of `color` (`npc::art`).
    #[serde(default)]
    pub look: Option<super::art::NpcLook>,
}

/// What the map shows for an NPC (V2.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpcMapMarker {
    /// Runs a shop (buy and sell).
    Shop,
    /// Anyone else worth finding again.
    Npc,
}

impl NpcDefinition {
    /// The marker of this NPC on the map: a shop when it runs one.
    pub fn map_marker(&self) -> NpcMapMarker {
        if self.shop.is_some() {
            NpcMapMarker::Shop
        } else {
            NpcMapMarker::Npc
        }
    }
}

/// What a choice does.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChoiceAction {
    /// Go to another node of the dialogue.
    Goto(String),
    /// Open the shop of the NPC being talked to.
    OpenShop,
    /// End the conversation.
    End,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Choice {
    pub text: String,
    /// Shown only when these hold.
    #[serde(default)]
    pub when: Vec<DialogueCondition>,
    pub action: ChoiceAction,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DialogueNode {
    /// Unique in the game.
    pub id: String,
    /// Id of the NPC who speaks it.
    pub speaker: String,
    /// Said one after the other (Enter/D advances).
    pub lines: Vec<String>,
    /// Offered after the last line. Empty: `next` or the end.
    #[serde(default)]
    pub choices: Vec<Choice>,
    /// Node that follows when there are no choices (`None` ends).
    #[serde(default)]
    pub next: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum NpcError {
    #[error("duplicate id `{0}`")]
    DuplicateId(String),
    #[error("NPC `{npc}`: {problem}")]
    Npc { npc: String, problem: String },
    #[error("dialogue node `{node}`: {problem}")]
    Node { node: String, problem: String },
}

/// Every NPC and dialogue node, validated against each other.
#[derive(Debug, Clone, Default)]
pub struct NpcLibrary {
    npcs: BTreeMap<String, NpcDefinition>,
    nodes: BTreeMap<String, DialogueNode>,
}

fn npc_err(npc: &str, problem: impl Into<String>) -> NpcError {
    NpcError::Npc {
        npc: npc.to_owned(),
        problem: problem.into(),
    }
}

fn node_err(node: &str, problem: impl Into<String>) -> NpcError {
    NpcError::Node {
        node: node.to_owned(),
        problem: problem.into(),
    }
}

impl NpcLibrary {
    /// Builds and checks the library: unique ids, every referenced node
    /// exists and belongs to the right speaker, a default greeting, no
    /// empty node, `OpenShop` only for NPCs with a shop, sane sizes.
    pub fn new(
        npcs: impl IntoIterator<Item = NpcDefinition>,
        nodes: impl IntoIterator<Item = DialogueNode>,
    ) -> Result<Self, NpcError> {
        let mut library = NpcLibrary::default();
        for npc in npcs {
            if library.npcs.contains_key(&npc.id) {
                return Err(NpcError::DuplicateId(npc.id));
            }
            library.npcs.insert(npc.id.clone(), npc);
        }
        for node in nodes {
            if library.nodes.contains_key(&node.id) || library.npcs.contains_key(&node.id) {
                return Err(NpcError::DuplicateId(node.id));
            }
            library.nodes.insert(node.id.clone(), node);
        }
        library.check()?;
        Ok(library)
    }

    fn check(&self) -> Result<(), NpcError> {
        for npc in self.npcs.values() {
            let (w, h) = npc.size;
            if !(w > 0.0 && h > 0.0 && w <= 200.0 && h <= 200.0) {
                return Err(npc_err(&npc.id, "size out of range"));
            }
            if npc.look.is_some_and(|l| !l.is_valid()) {
                return Err(npc_err(&npc.id, "look accent outside 0..1"));
            }
            let (x, y) = npc.position;
            if !(x.is_finite() && y.is_finite()) {
                return Err(npc_err(&npc.id, "position is not finite"));
            }
            if npc.greetings.is_empty() {
                return Err(npc_err(&npc.id, "no greeting"));
            }
            if !npc.greetings.iter().any(|g| g.when.is_empty()) {
                return Err(npc_err(
                    &npc.id,
                    "no greeting without conditions (the default)",
                ));
            }
            for greeting in &npc.greetings {
                self.check_target(&npc.id, &greeting.node)?;
            }
        }
        for node in self.nodes.values() {
            let Some(npc) = self.npcs.get(&node.speaker) else {
                return Err(node_err(
                    &node.id,
                    format!("unknown speaker `{}`", node.speaker),
                ));
            };
            if node.lines.is_empty() || node.lines.iter().any(|l| l.trim().is_empty()) {
                return Err(node_err(&node.id, "empty line"));
            }
            if !node.choices.is_empty() && node.next.is_some() {
                return Err(node_err(&node.id, "has both choices and next"));
            }
            if let Some(next) = &node.next {
                self.check_target(&npc.id, next)?;
            }
            if !node.choices.is_empty() && !node.choices.iter().any(|c| c.when.is_empty()) {
                return Err(node_err(
                    &node.id,
                    "every choice is conditional (the player could be stuck)",
                ));
            }
            for choice in &node.choices {
                match &choice.action {
                    ChoiceAction::Goto(target) => self.check_target(&npc.id, target)?,
                    ChoiceAction::OpenShop if npc.shop.is_none() => {
                        return Err(node_err(&node.id, "OpenShop for an NPC without a shop"));
                    }
                    ChoiceAction::OpenShop | ChoiceAction::End => {}
                }
            }
        }
        Ok(())
    }

    fn check_target(&self, npc: &str, node: &str) -> Result<(), NpcError> {
        match self.nodes.get(node) {
            None => Err(npc_err(npc, format!("unknown dialogue node `{node}`"))),
            Some(n) if n.speaker != npc => Err(npc_err(
                npc,
                format!("node `{node}` is spoken by `{}`", n.speaker),
            )),
            Some(_) => Ok(()),
        }
    }

    pub fn npc(&self, id: &str) -> Option<&NpcDefinition> {
        self.npcs.get(id)
    }

    pub fn node(&self, id: &str) -> Option<&DialogueNode> {
        self.nodes.get(id)
    }

    pub fn npcs(&self) -> impl Iterator<Item = &NpcDefinition> {
        self.npcs.values()
    }

    pub fn in_room<'a>(&'a self, room: &'a str) -> impl Iterator<Item = &'a NpcDefinition> {
        self.npcs.values().filter(move |n| n.room == room)
    }

    /// The first greeting whose conditions hold.
    pub fn greeting(&self, npc: &str, facts: &DialogueFacts) -> Option<&DialogueNode> {
        let def = self.npcs.get(npc)?;
        def.greetings
            .iter()
            .find(|g| all_hold(&g.when, facts))
            .and_then(|g| self.nodes.get(&g.node))
    }
}
