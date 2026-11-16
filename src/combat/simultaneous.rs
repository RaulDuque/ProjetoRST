//! V2.4: choosing the hit that counts when several reach one target in the
//! same frame.
//!
//! Pure module: no Bevy types (targets are any comparable key). Used by
//! `SimultaneousHitPolicy::HighestDamagePerFrame`
//! (`combat::hitboxes`); the default policy never calls it.

use std::cmp::Reverse;

use super::reaction::KnockbackIntensity;

/// What makes one candidate stronger than another, in this order: more
/// effective damage, then a stronger knockback, then the lower `source`
/// (a stable id of the attacker). Equal on all three, the earlier
/// candidate wins.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HitRank {
    /// Damage the target would really take (after everything the target
    /// applies, e.g. the guard).
    pub damage: i32,
    pub knockback: KnockbackIntensity,
    pub source: u64,
}

impl HitRank {
    fn key(&self) -> (i32, KnockbackIntensity, Reverse<u64>) {
        (self.damage, self.knockback, Reverse(self.source))
    }
}

/// Indices (ascending) of the strongest candidate of each target. Only
/// candidates with damage > 0 compete: a target with nothing but misses
/// and zero-damage hits gets none.
pub fn strongest_per_target<T: PartialEq>(candidates: &[(T, HitRank)]) -> Vec<usize> {
    let mut chosen: Vec<usize> = Vec::new();
    for (index, (target, rank)) in candidates.iter().enumerate() {
        if rank.damage <= 0 {
            continue;
        }
        match chosen
            .iter_mut()
            .find(|best| candidates[**best].0 == *target)
        {
            Some(best) => {
                if rank.key() > candidates[*best].1.key() {
                    *best = index;
                }
            }
            None => chosen.push(index),
        }
    }
    chosen.sort_unstable();
    chosen
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rank(damage: i32) -> HitRank {
        HitRank {
            damage,
            knockback: KnockbackIntensity::Medium,
            source: 1,
        }
    }

    #[test]
    fn the_highest_damage_of_each_target_wins() {
        let candidates = [
            ('a', rank(5)),
            ('b', rank(9)),
            ('a', rank(30)),
            ('b', rank(2)),
        ];
        assert_eq!(strongest_per_target(&candidates), vec![1, 2]);
    }

    #[test]
    fn misses_and_zero_damage_never_compete() {
        let candidates = [('a', rank(0)), ('a', rank(-1)), ('b', rank(0))];
        assert!(strongest_per_target(&candidates).is_empty());
        let candidates = [('a', rank(0)), ('a', rank(1))];
        assert_eq!(strongest_per_target(&candidates), vec![1]);
    }

    #[test]
    fn ties_are_broken_by_knockback_then_source_then_order() {
        let heavy = HitRank {
            knockback: KnockbackIntensity::Heavy,
            ..rank(10)
        };
        assert_eq!(
            strongest_per_target(&[('a', rank(10)), ('a', heavy)]),
            vec![1]
        );
        let lower_source = HitRank {
            source: 0,
            ..rank(10)
        };
        assert_eq!(
            strongest_per_target(&[('a', rank(10)), ('a', lower_source)]),
            vec![1]
        );
        assert_eq!(
            strongest_per_target(&[('a', rank(10)), ('a', rank(10))]),
            vec![0]
        );
    }
}
