//! Physical attacks: the basic attack and melee techniques.

use bevy::prelude::*;

use super::damage::{DamageKind, HitSpec};
use super::hitboxes::{
    AttachedToOwner, CombatProfile, Hitbox, PendingStrike, RehitTimers, SideLock, Team,
};
use super::strike_timeline::{StrikePhase, StrikeTimeline};
use crate::core::constants::{HITBOX_ACTIVE_FRACTION, Z_EFFECTS};
use crate::core::events::{BasicAttackRequested, SkillStarted};
use crate::core::resources::GameData;
use crate::inventory::systems::EquipmentAppearance;
use crate::inventory::weapons::StrikeProfile;
use crate::player::action_rules::cancels_attack;
use crate::player::components::{ActionState, Belongings, CharacterSheet};
use crate::player::state::{PlayerAction, PlayerState};
use crate::skills::definitions::SkillEffect;

/// Alpha of the basic strike; its colour is the weapon's trail (V2.0,
/// white for the starting sword and unarmed, as before).
const SLASH_ALPHA: f32 = 0.35;
const TECHNIQUE_COLOR: Color = Color::srgba(1.0, 0.85, 0.4, 0.45);

/// Everything needed to spawn one melee hitbox.
struct MeleeStrike {
    owner: Entity,
    team: Team,
    size: Vec2,
    offset: Vec2,
    hit: HitSpec,
    lifetime: f32,
    color: Color,
}

fn spawn_melee_hitbox(
    commands: &mut Commands,
    strike: MeleeStrike,
    profile: &CombatProfile,
) -> Entity {
    let MeleeStrike {
        owner,
        team,
        size,
        offset,
        hit,
        lifetime,
        color,
    } = strike;
    commands
        .spawn((
            Hitbox {
                owner,
                team,
                size,
                attack: profile.attack(hit.kind).clone(),
                hit,
                lifetime,
                pierce: true,
                already_hit: Vec::new(),
            },
            AttachedToOwner { offset },
            Sprite::from_color(color, size),
            Transform::from_xyz(0.0, 0.0, Z_EFFECTS),
        ))
        .id()
}

/// Marks the hitbox of a basic attack with the weapon category that made it
/// (V2.0; `None` unarmed). Presentation and tests read it.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct WeaponStrike {
    pub category: Option<crate::inventory::items::ItemCategory>,
}

/// V2.4: timeline of the attacker's latest basic attack. Meaningful only
/// while its `ActionState` is `Attacking` (only the basic attack enters
/// that state); elapsed time comes from the action timer.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct ActiveStrike {
    pub timeline: StrikeTimeline,
    /// Share of horizontal control kept in the air (0 = none).
    pub air_control: f32,
}

impl ActiveStrike {
    pub fn from_profile(profile: &StrikeProfile) -> Self {
        Self {
            timeline: StrikeTimeline::from_profile(profile),
            air_control: profile.air_control,
        }
    }

    /// The strike and its elapsed time, while attacking.
    fn current<'a>(state: &ActionState, strike: Option<&'a Self>) -> Option<(&'a Self, f32)> {
        let strike = strike?;
        if state.state() != PlayerState::Attacking {
            return None;
        }
        Some((strike, strike.timeline.elapsed(state.remaining())))
    }

    /// Phase and cancel point now, or `None` when not attacking.
    pub fn read(state: &ActionState, strike: Option<&Self>) -> Option<(StrikePhase, bool)> {
        let (strike, elapsed) = Self::current(state, strike)?;
        Some((
            strike.timeline.phase(elapsed),
            strike.timeline.cancelable(elapsed),
        ))
    }

    /// Whether the attacking character may still turn around.
    pub fn may_turn(state: &ActionState, strike: Option<&Self>) -> bool {
        Self::current(state, strike).is_some_and(|(s, elapsed)| s.timeline.can_turn(elapsed))
    }

    /// Air control of the running basic attack (`None` when not attacking).
    pub fn air_control(state: &ActionState, strike: Option<&Self>) -> Option<f32> {
        Self::current(state, strike).map(|(s, _)| s.air_control)
    }

    /// Ends the basic attack when `action` may cancel it now
    /// ([`cancels_attack`]). Returns whether it was cancelled; the caller
    /// then starts `action` through the usual state machine checks.
    pub fn try_cancel(
        state: &mut ActionState,
        strike: Option<&Self>,
        action: PlayerAction,
    ) -> bool {
        let Some((phase, cancelable)) = Self::read(state, strike) else {
            return false;
        };
        cancels_attack(phase, cancelable, action) && state.cancel_attack()
    }
}

/// Basic attack (the attack button without a recognised command). V2.0:
/// shape, timing, power and element come from the equipped weapon
/// ([`StrikeProfile`]); without a weapon (or without belongings) it is the
/// pre-V2.0 strike, whose numbers the starting sword keeps.
///
/// V2.4: past the weapon's cancel point a new request cancels the running
/// strike. Whenever a strike starts, any earlier strike hitbox of the same
/// attacker is removed, so only one exists at a time.
pub fn basic_attack(
    mut commands: Commands,
    data: Res<GameData>,
    mut requests: MessageReader<BasicAttackRequested>,
    mut attackers: Query<(
        &mut ActionState,
        &CombatProfile,
        &CharacterSheet,
        Option<&Belongings>,
        Option<&EquipmentAppearance>,
        Option<&ActiveStrike>,
    )>,
    strikes: Query<(Entity, &Hitbox), With<WeaponStrike>>,
) {
    for request in requests.read() {
        let Ok((mut state, profile, sheet, belongings, look, active)) =
            attackers.get_mut(request.attacker)
        else {
            continue;
        };
        ActiveStrike::try_cancel(&mut state, active, PlayerAction::Attack);
        let speed = sheet.derived().attack_speed;
        let strike = belongings.map_or_else(
            || StrikeProfile::unarmed(speed),
            |b| data.items.strike_profile(&b.equipment, speed),
        );
        if state
            .start_action(PlayerState::Attacking, strike.action_time)
            .is_err()
        {
            continue;
        }
        // At most one strike per attacker: normally the previous one has
        // already expired, but a timing that ends both on the same frame
        // could leave it alive for one more frame.
        for (entity, hitbox) in &strikes {
            if hitbox.owner == request.attacker {
                commands.entity(entity).try_despawn();
            }
        }
        let trail = look.map_or(crate::inventory::appearance::DEFAULT_TRAIL, |l| l.trail);
        let size = Vec2::new(strike.size.0, strike.size.1);
        let melee = MeleeStrike {
            owner: request.attacker,
            team: Team::Player,
            size,
            offset: Vec2::new(strike.offset.0, strike.offset.1),
            hit: HitSpec {
                kind: DamageKind::Physical,
                element: strike.element,
                base_damage: 0,
                multiplier: strike.multiplier,
            },
            lifetime: strike.active_time,
            color: Color::srgba(trail.0, trail.1, trail.2, SLASH_ALPHA),
        };
        let hitbox = spawn_melee_hitbox(&mut commands, melee, profile);
        let mut hitbox = commands.entity(hitbox);
        hitbox.insert((
            WeaponStrike {
                category: strike.category,
            },
            SideLock::default(),
        ));
        if strike.startup > 0.0 {
            hitbox.insert((
                PendingStrike {
                    delay: strike.startup,
                },
                Visibility::Hidden,
            ));
        }
        if let Some(interval) = strike.rehit_interval {
            hitbox.insert(RehitTimers::new(interval));
        }
        commands
            .entity(request.attacker)
            .insert(ActiveStrike::from_profile(&strike));
    }
}

/// V2.4: what happens to the hitbox of a basic attack when a skill
/// (a technique, or a spell whose `usable_in` lists `Attacking`) cancels
/// that attack. One rule for the whole game, not per weapon or skill.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StrikeCancelPolicy {
    /// The strike's hitbox lives on until its own lifetime ends, so strike
    /// and skill may both hit (pre-V2.4 behaviour).
    #[default]
    Preserve,
    /// The strike's hitbox is removed in the frame the skill starts, before
    /// `GameSet::Combat`; hits it already landed stay applied.
    RemoveOnCancel,
}

/// Applies [`StrikeCancelPolicy`] to every skill that just started.
///
/// A [`WeaponStrike`] hitbox of the caster that is still alive when a skill
/// starts belongs to the basic attack the skill interrupted: every other way
/// out of `Attacking` leaves none (it expires before the attack ends and
/// before any cancel point, and a new attack removes the previous one).
/// Technique hitboxes, projectiles and other fighters' hitboxes never carry
/// `WeaponStrike`, so they are never touched.
pub fn apply_strike_cancel_policy(
    mut commands: Commands,
    policy: Res<StrikeCancelPolicy>,
    mut started: MessageReader<SkillStarted>,
    strikes: Query<(Entity, &Hitbox), With<WeaponStrike>>,
) {
    for skill in started.read() {
        if *policy == StrikeCancelPolicy::Preserve {
            continue;
        }
        for (entity, hitbox) in &strikes {
            if hitbox.owner == skill.caster {
                commands.entity(entity).try_despawn();
            }
        }
    }
}

/// Melee techniques (`SkillEffect::MeleeStrike`). Reacts only to
/// `SkillStarted`, i.e. activations already confirmed by `begin_skill`.
pub fn melee_techniques(
    mut commands: Commands,
    data: Res<GameData>,
    mut activations: MessageReader<SkillStarted>,
    casters: Query<&CombatProfile>,
) {
    for activation in activations.read() {
        let Some(def) = data.skills.get(&activation.skill_id) else {
            continue;
        };
        let SkillEffect::MeleeStrike { width, height, .. } = def.effect else {
            continue;
        };
        let Ok(profile) = casters.get(activation.caster) else {
            continue;
        };
        let size = Vec2::new(width, height);
        let strike = MeleeStrike {
            owner: activation.caster,
            team: Team::Player,
            size,
            offset: Vec2::new(size.x / 2.0 + 6.0, height / 2.0 - 20.0),
            hit: HitSpec {
                kind: def.damage_kind,
                element: def.element,
                base_damage: def.base_damage,
                multiplier: def.damage_multiplier,
            },
            lifetime: def.execution_time * HITBOX_ACTIVE_FRACTION,
            color: TECHNIQUE_COLOR,
        };
        spawn_melee_hitbox(&mut commands, strike, profile);
    }
}
