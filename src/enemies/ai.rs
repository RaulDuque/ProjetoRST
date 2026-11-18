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

/// How an enemy uses distance (D20).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiStyle {
    /// Close in and melee (Bone Sentry).
    #[default]
    Melee,
    /// Hold a band; fire projectiles (`projectile_speed` > 0).
    Ranged,
    /// Back off when the player crowds; strike in a mid band.
    KeepDistance,
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
    /// Combat role (D20).
    pub style: AiStyle,
    /// Ideal stand-off for [`AiStyle::Ranged`] / [`AiStyle::KeepDistance`].
    /// `0` falls back to `attack_range`.
    pub preferred_range: f32,
    /// Below this, ranged/keep-distance enemies retreat. `0` disables.
    pub min_range: f32,
    /// Horizontal projectile speed. `0` = melee hitbox on attack.
    pub projectile_speed: f32,
    /// Telegraph before the attack lands (seconds).
    pub windup: f32,
}

impl Default for AiProfile {
    fn default() -> Self {
        Self {
            aggro_range: 260.0,
            leash_range: 420.0,
            attack_range: 46.0,
            flee_below_hp: 0.0,
            style: AiStyle::Melee,
            preferred_range: 0.0,
            min_range: 0.0,
            projectile_speed: 0.0,
            windup: 0.35,
        }
    }
}

impl AiProfile {
    pub fn preferred(&self) -> f32 {
        if self.preferred_range > 0.0 {
            self.preferred_range
        } else {
            self.attack_range
        }
    }

    pub fn uses_projectile(&self) -> bool {
        self.projectile_speed > 0.0
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

fn engagement_range(profile: &AiProfile, currently: EnemyBehavior) -> f32 {
    let engaged = matches!(
        currently,
        EnemyBehavior::Chase
            | EnemyBehavior::Attack
            | EnemyBehavior::Wait
            | EnemyBehavior::Defend
            | EnemyBehavior::Retreat
    );
    if engaged {
        profile.leash_range
    } else {
        profile.aggro_range
    }
}

fn attack_or_wait(on_cooldown: bool) -> EnemyBehavior {
    if on_cooldown {
        EnemyBehavior::Wait
    } else {
        EnemyBehavior::Attack
    }
}

pub fn decide(profile: &AiProfile, p: &Perception) -> EnemyBehavior {
    let Some(distance) = p.distance_to_player else {
        return EnemyBehavior::Patrol;
    };
    if p.hp_ratio < profile.flee_below_hp && distance < profile.aggro_range {
        return EnemyBehavior::Flee;
    }
    let range = engagement_range(profile, p.currently);
    if distance > range {
        return EnemyBehavior::Patrol;
    }

    match profile.style {
        AiStyle::Melee => {
            if distance <= profile.attack_range {
                attack_or_wait(p.attack_on_cooldown)
            } else {
                EnemyBehavior::Chase
            }
        }
        AiStyle::Ranged | AiStyle::KeepDistance => {
            if profile.min_range > 0.0 && distance < profile.min_range {
                return EnemyBehavior::Retreat;
            }
            if distance <= profile.attack_range {
                return attack_or_wait(p.attack_on_cooldown);
            }
            let preferred = profile.preferred();
            if distance > preferred {
                EnemyBehavior::Chase
            } else {
                // In the stand-off band: hold and wait for an opening.
                EnemyBehavior::Defend
            }
        }
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
    fn melee_patrols_until_player_is_close() {
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
    fn melee_keeps_chasing_inside_leash() {
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

    #[test]
    fn ranged_retreats_when_crowded_and_chases_outside_preferred() {
        let profile = AiProfile {
            style: AiStyle::Ranged,
            aggro_range: 400.0,
            leash_range: 520.0,
            attack_range: 320.0,
            preferred_range: 260.0,
            min_range: 90.0,
            projectile_speed: 220.0,
            ..Default::default()
        };
        assert_eq!(
            decide(&profile, &perceive(Some(50.0), EnemyBehavior::Defend)),
            EnemyBehavior::Retreat
        );
        // Inside fire range: shoot.
        assert_eq!(
            decide(&profile, &perceive(Some(200.0), EnemyBehavior::Chase)),
            EnemyBehavior::Attack
        );
        // Beyond preferred but still in fire range: still shoot.
        assert_eq!(
            decide(&profile, &perceive(Some(300.0), EnemyBehavior::Defend)),
            EnemyBehavior::Attack
        );
        // Outside fire range, inside aggro: close the gap.
        assert_eq!(
            decide(&profile, &perceive(Some(360.0), EnemyBehavior::Defend)),
            EnemyBehavior::Chase
        );
    }

    #[test]
    fn keep_distance_waits_on_cooldown_inside_attack_band() {
        let profile = AiProfile {
            style: AiStyle::KeepDistance,
            attack_range: 80.0,
            preferred_range: 140.0,
            min_range: 50.0,
            ..Default::default()
        };
        let p = Perception {
            attack_on_cooldown: true,
            ..perceive(Some(70.0), EnemyBehavior::Attack)
        };
        assert_eq!(decide(&profile, &p), EnemyBehavior::Wait);
    }
}
