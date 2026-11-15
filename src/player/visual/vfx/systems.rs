//! Bevy systems of the player's visual effects (presentation only).
//!
//! Every effect is a child sprite of the player, so it follows the player
//! (the Crescent Slash lunge, a respawn) and is removed with it. Effects read
//! messages and components; they write only their own entities.

use std::collections::HashMap;

use bevy::prelude::*;
use bevy::sprite::Anchor;

use super::super::manifest::anchor_for;
use super::super::systems::VISUAL_OFFSET_Y;
use super::super::{PlayerSpriteSet, VISUAL_Z_EFFECTS};
use super::PlayerVfx;
use super::rules::{
    GUARD_SPARK, REVIVE, effect_frame, interrupted, revived, skill_clip, time_scale,
};
use crate::app::states::GameState;
use crate::combat::damage::DamageOutcome;
use crate::core::events::{DamageDealt, SkillStarted};
use crate::core::resources::GameData;
use crate::core::time::frame_delta;
use crate::player::components::{ActionState, Player};
use crate::player::state::{Facing, PlayerState};
use crate::world::physics::FacingDirection;

/// What a new effect needs, besides the sprite set.
struct NewEffect<'a> {
    owner: Entity,
    clip: &'a str,
    facing: Facing,
    /// Gameplay action it accompanies (seconds), 0 for none.
    action_seconds: f32,
    follows_action: bool,
}

/// Spawns one effect as a child of its owner; replaces a running copy of the
/// same clip on the same owner, so effects never pile up. Returns whether
/// the manifest had the clip.
fn spawn_effect(
    commands: &mut Commands,
    set: &PlayerSpriteSet,
    running: &Query<(Entity, &ChildOf, &PlayerVfx)>,
    effect: NewEffect<'_>,
) -> bool {
    let Some(clip) = set.manifest.clips.get(effect.clip) else {
        return false;
    };
    let Some(atlas) = set.atlases.get(&clip.atlas) else {
        return false;
    };
    for (entity, parent, vfx) in running {
        if parent.parent() == effect.owner && vfx.clip == effect.clip {
            commands.entity(entity).try_despawn();
        }
    }
    let flip = effect.facing == Facing::Left;
    let (ax, ay) = anchor_for(clip.pivot, atlas.cell, flip);
    commands.entity(effect.owner).with_child((
        PlayerVfx {
            clip: effect.clip.to_owned(),
            elapsed: 0.0,
            time_scale: time_scale(effect.action_seconds, clip.total_duration()),
            follows_action: effect.follows_action,
        },
        Sprite {
            image: atlas.image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: atlas.layout.clone(),
                index: clip.first as usize,
            }),
            custom_size: Some(atlas.world_size),
            flip_x: flip,
            ..default()
        },
        Anchor(Vec2::new(ax, ay)),
        Transform::from_xyz(0.0, VISUAL_OFFSET_Y, VISUAL_Z_EFFECTS),
        Visibility::Inherited,
    ));
    true
}

fn sprites_ready(set: Option<&PlayerSpriteSet>, images: Option<&Assets<Image>>) -> bool {
    matches!((set, images), (Some(set), Some(images)) if set.ready(images))
}

/// Effects of confirmed skills (`SkillStarted`, the same message the combat
/// and magic executors react to; this reader has its own cursor). The `vfx`
/// key of the skill data names the clip; a key without a clip (the Ember
/// Bolt projectile is its own visual) shows nothing.
pub fn spawn_skill_vfx(
    mut commands: Commands,
    data: Option<Res<GameData>>,
    set: Option<Res<PlayerSpriteSet>>,
    images: Option<Res<Assets<Image>>>,
    mut started: MessageReader<SkillStarted>,
    players: Query<(&ActionState, &FacingDirection), With<Player>>,
    running: Query<(Entity, &ChildOf, &PlayerVfx)>,
) {
    let messages: Vec<SkillStarted> = started.read().cloned().collect();
    let (Some(data), Some(set)) = (data, set) else {
        return;
    };
    if !sprites_ready(Some(&set), images.as_deref()) {
        return;
    }
    let mut done: Vec<(Entity, String)> = Vec::new();
    for message in &messages {
        let Ok((action, facing)) = players.get(message.caster) else {
            continue;
        };
        let Some(def) = data.skills.get(&message.skill_id) else {
            continue;
        };
        let Some(clip) = skill_clip(&def.vfx) else {
            continue;
        };
        // One effect per caster and clip per frame.
        if done.iter().any(|(c, k)| *c == message.caster && *k == clip) {
            continue;
        }
        let effect = NewEffect {
            owner: message.caster,
            clip: &clip,
            facing: facing.0,
            action_seconds: action.remaining(),
            follows_action: !def.category.is_spell(),
        };
        if spawn_effect(&mut commands, &set, &running, effect) {
            done.push((message.caster, clip));
        }
    }
}

/// Spark of a hit blocked by the guard. The signal is the gameplay's own:
/// a hit (`DamageDealt`, not a miss) on a player who is guarding and faces
/// it (`Facing::faces_hit`, the rule `receive_damage` applies). Never
/// inferred from enemy proximity.
pub fn spawn_guard_vfx(
    mut commands: Commands,
    set: Option<Res<PlayerSpriteSet>>,
    images: Option<Res<Assets<Image>>>,
    mut hits: MessageReader<DamageDealt>,
    players: Query<(&ActionState, &FacingDirection), With<Player>>,
    running: Query<(Entity, &ChildOf, &PlayerVfx)>,
) {
    let hits: Vec<DamageDealt> = hits.read().copied().collect();
    let Some(set) = set else {
        return;
    };
    if !sprites_ready(Some(&set), images.as_deref()) {
        return;
    }
    let mut sparked: Vec<Entity> = Vec::new();
    for hit in &hits {
        if sparked.contains(&hit.target) || matches!(hit.outcome, DamageOutcome::Miss) {
            continue;
        }
        let Ok((action, facing)) = players.get(hit.target) else {
            continue;
        };
        if action.state() != PlayerState::Guarding || !facing.0.faces_hit(hit.direction) {
            continue;
        }
        let effect = NewEffect {
            owner: hit.target,
            clip: GUARD_SPARK,
            facing: facing.0,
            action_seconds: 0.0,
            follows_action: false,
        };
        if spawn_effect(&mut commands, &set, &running, effect) {
            sparked.push(hit.target);
        }
    }
}

/// Light of the revive: the state leaves `Dead` (respawn or quick load).
/// The player entity is updated in place by both, so the effect is a child
/// of the same, single player and appears where the player now is.
pub fn spawn_revive_vfx(
    mut commands: Commands,
    mut previous: Local<HashMap<Entity, PlayerState>>,
    set: Option<Res<PlayerSpriteSet>>,
    images: Option<Res<Assets<Image>>>,
    players: Query<(Entity, &ActionState, &FacingDirection), With<Player>>,
    running: Query<(Entity, &ChildOf, &PlayerVfx)>,
) {
    let ready = sprites_ready(set.as_deref(), images.as_deref());
    previous.retain(|entity, _| players.contains(*entity));
    for (entity, action, facing) in &players {
        let now = action.state();
        let before = previous.insert(entity, now);
        if !ready || !revived(before, now) {
            continue;
        }
        let Some(set) = set.as_deref() else {
            continue;
        };
        let effect = NewEffect {
            owner: entity,
            clip: REVIVE,
            facing: facing.0,
            action_seconds: 0.0,
            follows_action: false,
        };
        spawn_effect(&mut commands, set, &running, effect);
    }
}

/// Plays every effect once and removes it at its end, or when the technique
/// it belongs to stops (hurt, death, load). Paused with the gameplay.
pub fn animate_player_vfx(
    mut commands: Commands,
    time: Res<Time>,
    game_state: Option<Res<State<GameState>>>,
    set: Option<Res<PlayerSpriteSet>>,
    players: Query<&ActionState, With<Player>>,
    mut effects: Query<(Entity, &ChildOf, &mut PlayerVfx, &mut Sprite)>,
) {
    let playing = game_state.is_none_or(|s| *s.get() == GameState::Playing);
    let dt = if playing { frame_delta(&time) } else { 0.0 };
    for (entity, parent, mut vfx, mut sprite) in &mut effects {
        let state = players.get(parent.parent()).map(|a| a.state());
        let clip = set.as_ref().and_then(|s| s.manifest.clips.get(&vfx.clip));
        let (Ok(state), Some(clip)) = (state, clip) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        if interrupted(vfx.follows_action, state) {
            commands.entity(entity).try_despawn();
            continue;
        }
        // The frame of this moment, then advance: a new effect shows frame 0
        // in the frame it was spawned (the strike and the effect start together).
        let Some(frame) = effect_frame(&clip.durations, vfx.elapsed, vfx.time_scale) else {
            commands.entity(entity).try_despawn();
            continue;
        };
        vfx.elapsed += dt;
        let index = clip.first as usize + frame;
        if let Some(atlas) = sprite.texture_atlas.as_mut()
            && atlas.index != index
        {
            atlas.index = index;
        }
    }
}
