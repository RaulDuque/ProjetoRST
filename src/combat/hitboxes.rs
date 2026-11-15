//! Hitboxes, hurtboxes and the combat profile shared by every fighter.
//!
//! The overlap test produces [`DamageDealt`] messages; applying damage is
//! the responsibility of the target's own module (player or enemies).

use bevy::prelude::*;

use super::damage::{
    AttackProfile, DamageKind, DamageRolls, DefenseProfile, HitSpec, calculate_damage,
};
use crate::core::events::{DamageDealt, RoomChanged};
use crate::core::resources::GameRandom;
use crate::core::time::frame_delta;
use crate::player::attributes::DerivedStats;
use crate::player::components::ActionState;
use crate::player::state::PlayerState;
use crate::world::collision::Aabb;
use crate::world::physics::FacingDirection;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Team {
    Player,
    Enemy,
}

/// Offensive and defensive numbers used when a hit connects.
#[derive(Component, Debug, Clone)]
pub struct CombatProfile {
    pub physical: AttackProfile,
    pub magical: AttackProfile,
    pub defense: DefenseProfile,
}

impl CombatProfile {
    pub fn from_derived(d: &DerivedStats) -> Self {
        let profile = |power| AttackProfile {
            power,
            hit: d.hit,
            critical_chance: d.critical_chance,
            critical_damage: d.critical_damage,
        };
        Self {
            physical: profile(d.attack),
            magical: profile(d.magic_attack),
            defense: DefenseProfile {
                defense: d.defense,
                magic_defense: d.magic_defense,
                dodge: d.dodge,
                resistances: d.resistances.clone(),
            },
        }
    }

    pub fn attack(&self, kind: DamageKind) -> &AttackProfile {
        match kind {
            DamageKind::Physical => &self.physical,
            DamageKind::Magical => &self.magical,
        }
    }
}

/// Area that can be hit, centred on the entity.
#[derive(Component, Debug, Clone, Copy)]
pub struct Hurtbox {
    pub team: Team,
    pub size: Vec2,
}

/// Remaining invulnerability (seconds). Hits are ignored while positive.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Invulnerable {
    pub remaining: f32,
}

/// Damaging area. Each target is hit at most once per hitbox.
#[derive(Component, Debug, Clone)]
pub struct Hitbox {
    pub owner: Entity,
    pub team: Team,
    pub size: Vec2,
    pub attack: AttackProfile,
    pub hit: HitSpec,
    pub lifetime: f32,
    /// Keeps hitting further targets instead of disappearing on first hit.
    pub pierce: bool,
    pub already_hit: Vec<Entity>,
}

/// Keeps a hitbox at `offset` in front of its owner (mirrored by facing).
#[derive(Component, Debug, Clone, Copy)]
pub struct AttachedToOwner {
    pub offset: Vec2,
}

/// V2.4: a strike still in its anticipation (`startup`). Until `delay`
/// runs out the hitbox follows its owner hidden, cannot hit and its
/// lifetime does not run. Interruption and room changes remove it like any
/// other hitbox.
#[derive(Component, Debug, Clone, Copy)]
pub struct PendingStrike {
    pub delay: f32,
}

impl PendingStrike {
    pub fn armed(&self) -> bool {
        self.delay <= 0.0
    }
}

/// V2.4: an attached hitbox that keeps the side (facing sign) it had when
/// it became able to hit, so turning around never flips a live strike.
/// While pending it still follows its owner's facing. Attached hitboxes
/// without it mirror their owner every frame, as before V2.4 (enemy
/// strikes, techniques).
#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub struct SideLock {
    /// `None` until the hitbox is armed.
    pub sign: Option<f32>,
}

/// V2.4: lets one hitbox hit the same target again `interval` seconds
/// after its last hit on it. Without it a hitbox hits each target once
/// (`Hitbox::already_hit`, D-009). Entries end when their time runs out
/// and the whole record goes away with the hitbox.
#[derive(Component, Debug, Clone, PartialEq)]
pub struct RehitTimers {
    interval: f32,
    recent: Vec<(Entity, f32)>,
}

impl RehitTimers {
    pub fn new(interval: f32) -> Self {
        Self {
            interval,
            recent: Vec::new(),
        }
    }

    pub fn interval(&self) -> f32 {
        self.interval
    }

    /// Runs the timers and forgets the targets whose wait is over.
    pub fn tick(&mut self, dt: f32) {
        for (_, remaining) in &mut self.recent {
            *remaining -= dt;
        }
        self.recent.retain(|(_, remaining)| *remaining > 0.0);
    }

    /// Whether `target` may be hit now.
    pub fn ready(&self, target: Entity) -> bool {
        !self.recent.iter().any(|(hit, _)| *hit == target)
    }

    /// Starts the wait for `target` after a hit.
    pub fn record(&mut self, target: Entity) {
        self.recent.push((target, self.interval));
    }

    /// Targets still waiting.
    pub fn tracked(&self) -> usize {
        self.recent.len()
    }
}

/// True for every hitbox that can hit now (pending strikes cannot).
pub fn is_armed(pending: Option<&PendingStrike>) -> bool {
    pending.is_none_or(PendingStrike::armed)
}

/// Runs the anticipation of pending strikes and shows them once armed.
pub fn arm_pending_strikes(
    time: Res<Time>,
    mut pending: Query<(&mut PendingStrike, &mut Visibility)>,
) {
    let dt = frame_delta(&time);
    for (mut strike, mut visibility) in &mut pending {
        if strike.armed() {
            continue;
        }
        strike.delay -= dt;
        if strike.armed() {
            *visibility = Visibility::Inherited;
        }
    }
}

pub fn tick_invulnerability(time: Res<Time>, mut query: Query<&mut Invulnerable>) {
    let dt = frame_delta(&time);
    for mut inv in &mut query {
        if inv.remaining > 0.0 {
            inv.remaining = (inv.remaining - dt).max(0.0);
        }
    }
}

pub fn follow_owners(
    mut commands: Commands,
    mut hitboxes: Query<(
        Entity,
        &Hitbox,
        &AttachedToOwner,
        &mut Transform,
        Option<&mut SideLock>,
        Option<&PendingStrike>,
    )>,
    owners: Query<(&Transform, &FacingDirection), Without<Hitbox>>,
) {
    for (entity, hitbox, attached, mut transform, lock, pending) in &mut hitboxes {
        match owners.get(hitbox.owner) {
            Ok((owner, facing)) => {
                let sign = match lock {
                    Some(mut lock) => {
                        if lock.sign.is_none() && is_armed(pending) {
                            lock.sign = Some(facing.sign());
                        }
                        lock.sign.unwrap_or_else(|| facing.sign())
                    }
                    None => facing.sign(),
                };
                transform.translation.x = owner.translation.x + attached.offset.x * sign;
                transform.translation.y = owner.translation.y + attached.offset.y;
            }
            Err(_) => commands.entity(entity).try_despawn(),
        }
    }
}

pub fn expire_hitboxes(
    mut commands: Commands,
    time: Res<Time>,
    mut hitboxes: Query<(Entity, &mut Hitbox, Option<&PendingStrike>)>,
) {
    let dt = frame_delta(&time);
    for (entity, mut hitbox, pending) in &mut hitboxes {
        if !is_armed(pending) {
            continue;
        }
        hitbox.lifetime -= dt;
        if hitbox.lifetime <= 0.0 {
            commands.entity(entity).try_despawn();
        }
    }
}

/// Cancels the melee hitboxes of an action interrupted by damage or death.
///
/// Every hitbox attached to its owner ([`AttachedToOwner`]) whose owner has
/// an [`ActionState`] in [`PlayerState::Hurt`] or [`PlayerState::Dead`] is
/// despawned, so a staggered or fallen character stops hitting and no
/// hitbox is carried to the spawn point by the respawn.
///
/// Ordering: runs in `GameSet::Resolution` right after the player's
/// `receive_damage`, in the same frame the damage is applied (before the
/// next frame's respawn in `GameSet::Actions`). Hits resolved earlier in the
/// frame by `detect_hits` (`GameSet::Combat`) stand: trading blows in the
/// same frame is legitimate and is never cancelled retroactively.
///
/// Untouched: owners without an `ActionState` (enemies), detached hitboxes
/// (projectiles already launched) and owners whose damage was blocked
/// (invulnerability, misses, zero damage), which never enter `Hurt` or
/// `Dead`. Guarding prevents `Hurt` but a lethal hit still kills (and
/// cancels). The rule is state-based: a technique hitbox that outlives its
/// `Casting` (high attack speed) is also removed if the owner gets hurt.
pub fn cancel_interrupted_hitboxes(
    mut commands: Commands,
    hitboxes: Query<(Entity, &Hitbox), With<AttachedToOwner>>,
    owners: Query<&ActionState>,
) {
    for (entity, hitbox) in &hitboxes {
        let Ok(owner) = owners.get(hitbox.owner) else {
            continue;
        };
        if matches!(owner.state(), PlayerState::Hurt | PlayerState::Dead) {
            commands.entity(entity).try_despawn();
        }
    }
}

/// Room lifecycle (V1.7): every hitbox belongs to the room it was created
/// in. A room change (door or load) despawns all of them in the same frame
/// (`RoomStep::React`, before the next `GameSet::Combat`), whatever their
/// remaining lifetime: enemy strikes, the player's slashes and techniques,
/// projectiles. Inside a room nothing changes (lifetime, size, damage).
pub fn clear_on_room_change(
    mut commands: Commands,
    mut changes: MessageReader<RoomChanged>,
    hitboxes: Query<Entity, With<Hitbox>>,
) {
    if changes.read().count() == 0 {
        return;
    }
    for entity in &hitboxes {
        commands.entity(entity).try_despawn();
    }
}

/// Draws the three random values of a damage roll.
pub fn roll_damage(rng: &mut GameRandom) -> DamageRolls {
    DamageRolls {
        hit: rng.0.next_f32(),
        critical: rng.0.next_f32(),
        variance: rng.0.next_f32(),
    }
}

/// Overlap test. Each hitbox hits a target once (`already_hit`), or, with
/// [`RehitTimers`], again after its interval; a target is never hit twice
/// by the same hitbox in one frame.
pub fn detect_hits(
    mut commands: Commands,
    time: Res<Time>,
    mut rng: ResMut<GameRandom>,
    mut hitboxes: Query<(
        Entity,
        &Transform,
        &mut Hitbox,
        Option<&PendingStrike>,
        Option<&mut RehitTimers>,
    )>,
    hurtboxes: Query<(
        Entity,
        &Transform,
        &Hurtbox,
        &CombatProfile,
        Option<&Invulnerable>,
    )>,
    mut damage: MessageWriter<DamageDealt>,
) {
    let dt = frame_delta(&time);
    for (hitbox_entity, hitbox_transform, mut hitbox, pending, mut rehit) in &mut hitboxes {
        if !is_armed(pending) {
            continue;
        }
        if let Some(rehit) = rehit.as_mut() {
            rehit.tick(dt);
        }
        let hit_area = Aabb::new(
            (
                hitbox_transform.translation.x,
                hitbox_transform.translation.y,
            ),
            (hitbox.size.x, hitbox.size.y),
        );
        for (target, target_transform, hurtbox, profile, invulnerable) in &hurtboxes {
            let waiting = match rehit.as_ref() {
                Some(rehit) => !rehit.ready(target),
                None => hitbox.already_hit.contains(&target),
            };
            if hurtbox.team == hitbox.team
                || waiting
                || invulnerable.is_some_and(|i| i.remaining > 0.0)
            {
                continue;
            }
            let target_area = Aabb::new(
                (
                    target_transform.translation.x,
                    target_transform.translation.y,
                ),
                (hurtbox.size.x, hurtbox.size.y),
            );
            if !hit_area.intersects(&target_area) {
                continue;
            }
            let outcome = calculate_damage(
                &hitbox.attack,
                &profile.defense,
                &hitbox.hit,
                roll_damage(&mut rng),
            );
            let direction =
                (target_transform.translation.x - hitbox_transform.translation.x).signum();
            damage.write(DamageDealt {
                attacker: hitbox.owner,
                target,
                outcome,
                direction: if direction == 0.0 { 1.0 } else { direction },
            });
            match rehit.as_mut() {
                Some(rehit) => rehit.record(target),
                None => hitbox.already_hit.push(target),
            }
            if !hitbox.pierce {
                commands.entity(hitbox_entity).try_despawn();
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn two_targets() -> (Entity, Entity) {
        let mut world = World::new();
        (world.spawn_empty().id(), world.spawn_empty().id())
    }

    #[test]
    fn rehit_waits_per_target_and_forgets_finished_waits() {
        let (a, b) = two_targets();
        let mut rehit = RehitTimers::new(0.2);
        assert!(rehit.ready(a) && rehit.ready(b));
        rehit.record(a);
        assert!(!rehit.ready(a), "no second hit right away");
        assert!(rehit.ready(b), "another target is independent");
        rehit.tick(0.1);
        assert!(!rehit.ready(a));
        rehit.record(b);
        rehit.tick(0.1);
        assert!(rehit.ready(a), "wait over");
        assert!(!rehit.ready(b));
        assert_eq!(rehit.tracked(), 1, "finished waits are dropped");
        rehit.tick(0.1);
        assert_eq!(rehit.tracked(), 0);
    }

    #[test]
    fn pending_strikes_are_not_armed_until_their_delay_ends() {
        assert!(is_armed(None));
        assert!(!is_armed(Some(&PendingStrike { delay: 0.01 })));
        assert!(is_armed(Some(&PendingStrike { delay: 0.0 })));
    }
}
