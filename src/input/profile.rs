//! A control profile: which physical controls fire each logical action.
//!
//! Conflict rule: a new binding is rejected when another action in the same
//! context already uses it. The previous binding stays. Escape cannot be
//! bound. An essential action cannot be left with both slots empty.

use std::collections::BTreeMap;

use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

use super::action::{ACTIONS, Action};
use super::binding::Binding;

pub const PROFILE_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProfileOrigin {
    Hybrid,
    Classic,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Slot {
    Primary,
    Alternate,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionBinding {
    pub primary: Option<Binding>,
    pub alternate: Option<Binding>,
}

impl ActionBinding {
    pub fn empty() -> Self {
        Self {
            primary: None,
            alternate: None,
        }
    }

    pub fn only(binding: Binding) -> Self {
        Self {
            primary: Some(binding),
            alternate: None,
        }
    }

    pub fn pair(primary: Binding, alternate: Binding) -> Self {
        Self {
            primary: Some(primary),
            alternate: Some(alternate),
        }
    }

    pub fn get(&self, slot: Slot) -> Option<Binding> {
        match slot {
            Slot::Primary => self.primary,
            Slot::Alternate => self.alternate,
        }
    }

    pub fn set(&mut self, slot: Slot, binding: Option<Binding>) {
        match slot {
            Slot::Primary => self.primary = binding,
            Slot::Alternate => self.alternate = binding,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Binding> + '_ {
        self.primary.into_iter().chain(self.alternate)
    }
}

/// The player's controls. Not part of the character save.
#[derive(Debug, Clone, PartialEq, Resource, Serialize, Deserialize)]
pub struct ControlProfile {
    pub version: u32,
    pub origin: ProfileOrigin,
    pub slots: BTreeMap<Action, ActionBinding>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignError {
    /// The other slot of this same action already uses it.
    Duplicate,
    /// Another action in the same context already uses it.
    Conflict(Action),
    /// This was the last binding of an action the player must be able to do.
    Essential,
}

impl ControlProfile {
    pub fn binding(&self, action: Action) -> ActionBinding {
        self.slots
            .get(&action)
            .cloned()
            .unwrap_or_else(ActionBinding::empty)
    }

    pub fn same_bindings(&self, other: &Self) -> bool {
        self.slots == other.slots
    }

    /// Put `binding` in `slot`. On failure the profile is unchanged.
    pub fn assign(
        &mut self,
        action: Action,
        slot: Slot,
        binding: Binding,
    ) -> Result<(), AssignError> {
        for (other_action, other) in &self.slots {
            if *other_action == action {
                continue;
            }
            if other_action.context() != action.context() {
                continue;
            }
            if other.iter().any(|held| held == binding) {
                return Err(AssignError::Conflict(*other_action));
            }
        }
        let entry = self
            .slots
            .entry(action)
            .or_insert_with(ActionBinding::empty);
        let other_slot = match slot {
            Slot::Primary => entry.alternate,
            Slot::Alternate => entry.primary,
        };
        if other_slot == Some(binding) {
            return Err(AssignError::Duplicate);
        }
        entry.set(slot, Some(binding));
        self.refresh_origin();
        Ok(())
    }

    /// Clear a slot. Essential actions must keep one binding.
    pub fn clear(&mut self, action: Action, slot: Slot) -> Result<(), AssignError> {
        let Some(entry) = self.slots.get(&action) else {
            return Err(AssignError::Essential);
        };
        let other = match slot {
            Slot::Primary => entry.alternate,
            Slot::Alternate => entry.primary,
        };
        if other.is_none() && action.essential() {
            return Err(AssignError::Essential);
        }
        let entry = self.slots.get_mut(&action).ok_or(AssignError::Essential)?;
        entry.set(slot, None);
        self.refresh_origin();
        Ok(())
    }

    /// A loaded file is usable only when it is complete and conflict-free.
    pub fn validate(&self) -> Result<(), String> {
        if self.version != PROFILE_VERSION {
            return Err(format!(
                "controls file version is {}, expected {PROFILE_VERSION}",
                self.version
            ));
        }
        for action in ACTIONS {
            let Some(entry) = self.slots.get(&action) else {
                return Err(format!("controls file is missing {action:?}"));
            };
            if action.essential() && entry.primary.is_none() && entry.alternate.is_none() {
                return Err(format!("{action:?} has no binding"));
            }
            if let (Some(primary), Some(alternate)) = (entry.primary, entry.alternate)
                && primary == alternate
            {
                return Err(format!("{action:?} uses the same control twice"));
            }
        }
        let actions: Vec<Action> = self.slots.keys().copied().collect();
        for (index, action) in actions.iter().enumerate() {
            for other in actions.iter().skip(index + 1) {
                if action.context() != other.context() {
                    continue;
                }
                let left = &self.slots[action];
                let right = &self.slots[other];
                for binding in left.iter() {
                    if right.iter().any(|candidate| candidate == binding) {
                        return Err(format!(
                            "{action:?} and {other:?} share {}",
                            binding.label()
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    fn refresh_origin(&mut self) {
        if self.same_bindings(&super::presets::hybrid()) {
            self.origin = ProfileOrigin::Hybrid;
        } else if self.same_bindings(&super::presets::classic()) {
            self.origin = ProfileOrigin::Classic;
        } else {
            self.origin = ProfileOrigin::Custom;
        }
    }
}

impl AssignError {
    pub fn message(self) -> String {
        match self {
            AssignError::Duplicate => "That control is already the other slot.".to_string(),
            AssignError::Conflict(action) => {
                format!("Already used by {} in the same context.", action.label())
            }
            AssignError::Essential => "This action needs at least one control.".to_string(),
        }
    }
}
