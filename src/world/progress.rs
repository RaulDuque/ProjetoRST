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

    /// Records a save point the first time it is used. Returns whether it
    /// was newly discovered.
    pub fn discover_save_point(&mut self, id: &str) -> bool {
        self.discovered_save_points.insert(id.to_owned())
    }

    /// Marks where the last rest/save happened (stable object id).
    pub fn set_last_save_point(&mut self, id: &str) {
        self.last_save_point = Some(id.to_owned());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discover_save_point_is_idempotent() {
        let mut progress = WorldProgress::default();
        assert!(progress.discover_save_point("altar_a"));
        assert!(!progress.discover_save_point("altar_a"));
        assert_eq!(progress.discovered_save_points.len(), 1);
        progress.set_last_save_point("altar_a");
        assert_eq!(progress.last_save_point.as_deref(), Some("altar_a"));
    }
}
