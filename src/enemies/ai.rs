//! Enemy decision making.
//!
//! Pure module: no Bevy types. [`decide`] picks a behaviour from what the
//! enemy perceives; the Bevy layer (`enemies::behavior`) executes it. Enemy
//! AI never reads player input or player control state.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnemyBehavior {
    Patrol,
    Chase,
    Attack,
    Defend,
    Flee,
    Wait,
    Retreat,
    UseSkill,
}

/// Tuning of one enemy archetype (part of its data file).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AiProfile {
    /// Distance at which the enemy notices the player.
    pub aggro_range: f32,
    /// Distance at which it gives up chasing.
    pub leash_range: f32,
    /// Distance at which it attacks.
    pub attack_range: f32,
    /// HP ratio under which it flees (`0` = never).
    pub flee_below_hp: f32,
}

impl Default for AiProfile {
    fn default() -> Self {
        Self {
            aggro_range: 260.0,
            leash_range: 420.0,
            attack_range: 46.0,
            flee_below_hp: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Perception {
    /// Horizontal distance to the player, `None` if the player is absent/dead.
    pub distance_to_player: Option<f32>,
    pub hp_ratio: f32,
    pub currently: EnemyBehavior,
    /// The attack is still recovering.
    pub attack_on_cooldown: bool,
}

pub fn decide(profile: &AiProfile, p: &Perception) -> EnemyBehavior {
    let Some(distance) = p.distance_to_player else {
        return EnemyBehavior::Patrol;
    };
    if p.hp_ratio < profile.flee_below_hp && distance < profile.aggro_range {
        return EnemyBehavior::Flee;
    }
    let engaged = matches!(
        p.currently,
        EnemyBehavior::Chase | EnemyBehavior::Attack | EnemyBehavior::Wait
    );
    let range = if engaged {
        profile.leash_range
    } else {
        profile.aggro_range
    };
    if distance > range {
        return EnemyBehavior::Patrol;
    }
    if distance <= profile.attack_range {
        if p.attack_on_cooldown {
            EnemyBehavior::Wait
        } else {
            EnemyBehavior::Attack
        }
    } else {
        EnemyBehavior::Chase
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn perceive(distance: Option<f32>, currently: EnemyBehavior) -> Perception {
        Perception {
            distance_to_player: distance,
            hp_ratio: 1.0,
            currently,
            attack_on_cooldown: false,
        }
    }

    #[test]
    fn patrols_until_player_is_close() {
        let profile = AiProfile::default();
        assert_eq!(
            decide(&profile, &perceive(None, EnemyBehavior::Patrol)),
            EnemyBehavior::Patrol
        );
        assert_eq!(
            decide(&profile, &perceive(Some(500.0), EnemyBehavior::Patrol)),
            EnemyBehavior::Patrol
        );
        assert_eq!(
            decide(&profile, &perceive(Some(200.0), EnemyBehavior::Patrol)),
            EnemyBehavior::Chase
        );
        assert_eq!(
            decide(&profile, &perceive(Some(30.0), EnemyBehavior::Chase)),
            EnemyBehavior::Attack
        );
    }

    #[test]
    fn keeps_chasing_inside_leash() {
        let profile = AiProfile::default();
        assert_eq!(
            decide(&profile, &perceive(Some(350.0), EnemyBehavior::Chase)),
            EnemyBehavior::Chase
        );
        assert_eq!(
            decide(&profile, &perceive(Some(350.0), EnemyBehavior::Patrol)),
            EnemyBehavior::Patrol
        );
    }

    #[test]
    fn flees_when_weak() {
        let profile = AiProfile {
            flee_below_hp: 0.3,
            ..Default::default()
        };
        let p = Perception {
            hp_ratio: 0.1,
            ..perceive(Some(100.0), EnemyBehavior::Chase)
        };
        assert_eq!(decide(&profile, &p), EnemyBehavior::Flee);
    }
}
