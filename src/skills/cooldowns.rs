//! Per-skill cooldown tracking. Pure module: no Bevy types.

use std::collections::BTreeMap;

use super::definitions::SkillId;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Cooldowns {
    remaining: BTreeMap<SkillId, f32>,
}

impl Cooldowns {
    pub fn start(&mut self, id: &str, seconds: f32) {
        if seconds > 0.0 {
            self.remaining.insert(id.to_owned(), seconds);
        }
    }

    /// Seconds left, `0.0` when ready.
    pub fn remaining(&self, id: &str) -> f32 {
        self.remaining.get(id).copied().unwrap_or(0.0)
    }

    pub fn is_ready(&self, id: &str) -> bool {
        self.remaining(id) <= 0.0
    }

    pub fn tick(&mut self, dt: f32) {
        self.remaining.retain(|_, left| {
            *left -= dt;
            *left > 0.0
        });
    }

    pub fn active(&self) -> impl Iterator<Item = (&str, f32)> {
        self.remaining.iter().map(|(id, left)| (id.as_str(), *left))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cooldown_expires() {
        let mut cds = Cooldowns::default();
        cds.start("slash", 1.0);
        assert!(!cds.is_ready("slash"));
        cds.tick(0.6);
        assert!((cds.remaining("slash") - 0.4).abs() < 1e-6);
        cds.tick(0.5);
        assert!(cds.is_ready("slash"));
        assert_eq!(cds.active().count(), 0);
    }

    #[test]
    fn zero_cooldown_is_never_tracked() {
        let mut cds = Cooldowns::default();
        cds.start("jab", 0.0);
        assert!(cds.is_ready("jab"));
    }
}
