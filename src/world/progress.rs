//! Persistent world progress (doors, chests, save points, story flags).
//! Pure module: no Bevy types.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct WorldProgress {
    pub unlocked_doors: BTreeSet<String>,
    pub opened_chests: BTreeSet<String>,
    pub discovered_save_points: BTreeSet<String>,
    pub defeated_bosses: BTreeSet<String>,
    /// Free-form story/progression flags (checked by skill requirements).
    pub flags: BTreeSet<String>,
    pub last_save_point: Option<String>,
}

impl WorldProgress {
    pub fn set_flag(&mut self, flag: &str) -> bool {
        self.flags.insert(flag.to_owned())
    }

    pub fn has_flag(&self, flag: &str) -> bool {
        self.flags.contains(flag)
    }
}
