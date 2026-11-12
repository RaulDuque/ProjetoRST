//! Player lifecycle: spawning, timers, regeneration, receiving damage,
//! death/respawn, experience, drops and stat synchronisation.

use bevy::prelude::*;

use super::components::{
    ActionState, Belongings, CharacterSheet, CommandInput, JumpControl, Player, SkillBook, Vitals,
};
use super::progression::Progression;
use super::state::PlayerState;
use crate::combat::hitboxes::{CombatProfile, Hurtbox, Invulnerable, Team};
use crate::core::constants::{
    GUARD_DAMAGE_REDUCTION, PLAYER_HURT_KNOCKBACK, PLAYER_HURT_TIME, PLAYER_INVULNERABILITY,
    PLAYER_SIZE, Z_PLAYER,
};
use crate::core::events::{DamageDealt, EnemyDefeated, Notification};
use crate::core::resources::{GameData, GameRandom};
use crate::core::time::frame_delta;
use crate::input::PlayerInput;
use crate::inventory::items::drop_chance;
use crate::skills::definitions::Acquisition;
use crate::world::maps::CurrentRoom;
use crate::world::physics::{AffectedByGravity, Body, FacingDirection, Grounded, Velocity};

const PLAYER_COLOR: Color = Color::srgb(0.75, 0.2, 0.25);

/// Builds the starting character from `assets/data/character.ron`.
pub fn starting_character(data: &GameData) -> (CharacterSheet, Vitals, SkillBook, Belongings) {
    let config = &data.character;
    let mut belongings = Belongings::default();
    for (id, quantity) in &config.start.items {
        if let Err(error) = belongings.inventory.add(id, *quantity, &data.items) {
            warn!(%error, "invalid starting item");
        }
    }
    let attributes = config.start.attributes;
    let progression = Progression::default();
    let empty = std::collections::BTreeSet::new();
    for (slot, id) in &config.start.equipped {
        let Some(item) = data.items.get(id) else {
            continue;
        };
        let ctx = crate::skills::unlock::RequirementContext {
            level: progression.level(),
            attributes: &attributes,
            owned_items: &empty,
            progress_flags: &empty,
        };
        if let Err(error) = belongings.equipment.equip(*slot, item, &ctx) {
            warn!(%error, "invalid starting equipment");
        }
    }
    let bonus = belongings.equipment.total_modifiers(&data.items);
    let sheet = CharacterSheet::new(attributes, progression, bonus, &config.formulas);
    let vitals = Vitals::full(sheet.derived(), config.start.hearts);

    let mut book = SkillBook::default();
    for def in data
        .skills
        .iter()
        .filter(|d| d.acquisition == Acquisition::Starting)
    {
        book.learned.grant(def);
    }
    (sheet, vitals, book, belongings)
}

pub fn spawn_player(mut commands: Commands, data: Res<GameData>) {
    let Some(room) = data.rooms.get(&data.character.start.room) else {
        error!("start room missing; player not spawned");
        return;
    };
    let (sheet, vitals, book, belongings) = starting_character(&data);
    let size = Vec2::new(PLAYER_SIZE.0, PLAYER_SIZE.1);
    let profile = CombatProfile::from_derived(sheet.derived());
    commands.spawn((
        (
            Player,
            ActionState::default(),
            sheet,
            vitals,
            book,
            belongings,
            CommandInput::default(),
            JumpControl::default(),
            profile,
        ),
        (
            Body { size },
            Velocity::default(),
            Grounded::default(),
            AffectedByGravity,
            FacingDirection::default(),
            Hurtbox {
                team: Team::Player,
                size,
            },
            Invulnerable::default(),
            Sprite::from_color(PLAYER_COLOR, size),
            Transform::from_xyz(room.player_spawn.0, room.player_spawn.1, Z_PLAYER),
        ),
    ));
}

pub fn tick_action_state(time: Res<Time>, mut players: Query<&mut ActionState, With<Player>>) {
    let dt = frame_delta(&time);
    for mut state in &mut players {
        state.tick(dt);
    }
}

pub fn regenerate(time: Res<Time>, player: Single<(&mut Vitals, &CharacterSheet, &ActionState)>) {
    let dt = frame_delta(&time);
    let (mut vitals, sheet, state) = player.into_inner();
    if state.state() == PlayerState::Dead {
        return;
    }
    let derived = sheet.derived();
    let hp = vitals.hp_regen.tick(derived.hp_regen, dt);
    let mp = vitals.mp_regen.tick(derived.mp_regen, dt);
    vitals.hp.restore(hp);
    vitals.mp.restore(mp);
}

pub fn receive_damage(
    mut hits: MessageReader<DamageDealt>,
    player: Single<
        (
            Entity,
            &mut Vitals,
            &mut ActionState,
            &mut Velocity,
            &mut Invulnerable,
            &FacingDirection,
        ),
        With<Player>,
    >,
    mut notifications: MessageWriter<Notification>,
) {
    let (entity, mut vitals, mut state, mut velocity, mut invulnerable, facing) =
        player.into_inner();
    for hit in hits.read().filter(|h| h.target == entity) {
        // Keep reading instead of `break`, so no hit stays queued for the
        // next frame (defence in depth: do not rely on the respawn i-frames).
        if state.state() == PlayerState::Dead {
            continue;
        }
        // Hits are only generated while the player is vulnerable, but several
        // can arrive in the same frame: the first one that deals damage starts
        // the invulnerability and the rest must be ignored.
        if invulnerable.remaining > 0.0 {
            continue;
        }
        let mut amount = hit.outcome.amount();
        // Guard blocks hits coming from the front.
        let from_front = hit.direction == -facing.sign();
        if state.state() == PlayerState::Guarding && from_front {
            amount = (amount as f32 * (1.0 - GUARD_DAMAGE_REDUCTION)).round() as i32;
        }
        if amount <= 0 {
            continue;
        }
        vitals.hp.deplete(amount);
        invulnerable.remaining = PLAYER_INVULNERABILITY;
        if vitals.hp.is_empty() {
            state.die();
            velocity.x = 0.0;
            notifications.write(Notification("You have fallen. Press R to rise.".into()));
        } else if state.hurt(PLAYER_HURT_TIME) {
            velocity.x = hit.direction * PLAYER_HURT_KNOCKBACK.0;
            velocity.y = PLAYER_HURT_KNOCKBACK.1;
        }
    }
}

/// Revives at the room's spawn point with full HP/MP. Progress is kept.
pub fn respawn(
    input: Res<PlayerInput>,
    data: Res<GameData>,
    room: Res<CurrentRoom>,
    player: Single<
        (
            &mut ActionState,
            &mut Vitals,
            &mut Transform,
            &mut Velocity,
            &mut Invulnerable,
        ),
        With<Player>,
    >,
) {
    let (mut state, mut vitals, mut transform, mut velocity, mut invulnerable) =
        player.into_inner();
    if state.state() != PlayerState::Dead || !input.respawn_pressed {
        return;
    }
    let Some(room) = data.rooms.get(&room.0) else {
        return;
    };
    state.revive();
    vitals.hp.refill();
    vitals.mp.refill();
    transform.translation.x = room.player_spawn.0;
    transform.translation.y = room.player_spawn.1;
    velocity.0 = Vec2::ZERO;
    invulnerable.remaining = PLAYER_INVULNERABILITY;
}

pub fn collect_rewards(
    data: Res<GameData>,
    mut rng: ResMut<GameRandom>,
    mut defeated: MessageReader<EnemyDefeated>,
    player: Single<(&mut CharacterSheet, &mut Belongings), With<Player>>,
    mut notifications: MessageWriter<Notification>,
) {
    let (mut sheet, mut belongings) = player.into_inner();
    let config = &data.character;
    for reward in defeated.read() {
        let summary =
            sheet.gain_experience(reward.experience, &config.progression, &config.formulas);
        if summary.levels_gained > 0 {
            notifications.write(Notification(format!(
                "Level {}! +{} attribute points",
                summary.new_level, summary.points_gained
            )));
        }
        let luck = sheet.derived().effective.luck;
        for drop in &reward.drops {
            if rng.0.next_f32() >= drop_chance(drop.chance, luck) {
                continue;
            }
            match belongings.inventory.add(&drop.item, 1, &data.items) {
                Ok(()) => {
                    let name = data
                        .items
                        .get(&drop.item)
                        .map_or("item", |i| i.name.as_str());
                    notifications.write(Notification(format!("Found {name}")));
                }
                Err(error) => warn!(%error, "invalid drop"),
            }
        }
    }
}

/// Propagates derived-stat changes to HP/MP maxima and the combat profile.
pub fn sync_derived_stats(
    mut players: Query<
        (&CharacterSheet, &mut Vitals, &mut CombatProfile),
        (With<Player>, Changed<CharacterSheet>),
    >,
) {
    for (sheet, mut vitals, mut profile) in &mut players {
        vitals.sync_max(sheet.derived());
        *profile = CombatProfile::from_derived(sheet.derived());
    }
}
