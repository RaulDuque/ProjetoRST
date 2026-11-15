//! Bevy systems of the player's sprite visual (presentation only).

use std::collections::HashMap;

use bevy::prelude::*;
use bevy::sprite::Anchor;

use super::manifest::{AnimationManifest, anchor_for};
use super::playback::{action_frame, action_progress, locomotion_rate, loop_frame, once_frame};
use super::selection::{self, SelectionContext, Timing, Tint, select};
use super::{AtlasHandles, MANIFEST_PATH, PlayerSpriteSet, PlayerVisual, VisualPlayback};
use crate::app::states::GameState;
use crate::combat::hitboxes::Invulnerable;
use crate::core::constants::PLAYER_SIZE;
use crate::core::events::SkillStarted;
use crate::core::resources::{GameData, project_root};
use crate::core::time::frame_delta;
use crate::player::animation::{BLINK_ALPHA, BLINK_RATE};
use crate::player::components::{ActionState, Player};
use crate::player::state::{Facing, PlayerState};
use crate::world::physics::{FacingDirection, Grounded, Velocity};

/// The sole line (sprite pivot) sits at the bottom of the collision body.
pub(super) const VISUAL_OFFSET_Y: f32 = -PLAYER_SIZE.1 / 2.0;
/// Slightly in front of the placeholder box and its children.
pub(super) const VISUAL_Z: f32 = 0.05;

/// Reads the manifest and creates the image and layout handles once.
///
/// A missing manifest keeps the placeholder visuals (info log); an invalid
/// one is reported as an error and also keeps them, so a broken art export
/// never stops the game. `data_tests` checks the shipped manifest.
pub fn load_player_sprites(
    mut commands: Commands,
    asset_server: Option<Res<AssetServer>>,
    layouts: Option<ResMut<Assets<TextureAtlasLayout>>>,
) {
    let path = project_root().join("assets").join(MANIFEST_PATH);
    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) => {
            info!(%error, path = %path.display(), "no player sprite manifest; placeholder visuals");
            return;
        }
    };
    let manifest = match AnimationManifest::from_ron(&text) {
        Ok(manifest) => manifest,
        Err(error) => {
            error!(%error, "invalid player sprite manifest; placeholder visuals");
            return;
        }
    };
    let (Some(server), Some(mut layouts)) = (asset_server, layouts) else {
        debug!("no asset server (headless app); placeholder visuals");
        return;
    };
    let atlases: HashMap<String, AtlasHandles> = manifest
        .atlases
        .iter()
        .map(|(id, def)| {
            let layout = TextureAtlasLayout::from_grid(
                UVec2::new(def.cell.0, def.cell.1),
                def.columns,
                def.rows,
                Some(UVec2::splat(def.padding)),
                None,
            );
            let (w, h) = def.world_size();
            let handles = AtlasHandles {
                image: server.load(def.texture.clone()),
                layout: layouts.add(layout),
                cell: def.cell,
                world_size: Vec2::new(w, h),
            };
            (id.clone(), handles)
        })
        .collect();
    commands.insert_resource(PlayerSpriteSet { manifest, atlases });
}

/// Gives every player one sprite child (none in headless apps).
pub fn attach_player_visual(
    mut commands: Commands,
    set: Option<Res<PlayerSpriteSet>>,
    players: Query<Entity, With<Player>>,
    visuals: Query<&ChildOf, With<PlayerVisual>>,
) {
    let Some(set) = set else {
        return;
    };
    let Some((_, clip)) = set.manifest.resolve(super::selection::IDLE) else {
        return;
    };
    let Some(atlas) = set.atlases.get(&clip.atlas) else {
        return;
    };
    for player in &players {
        if visuals.iter().any(|parent| parent.parent() == player) {
            continue;
        }
        let (ax, ay) = anchor_for(clip.pivot, atlas.cell, false);
        commands.entity(player).with_child((
            PlayerVisual,
            VisualPlayback::default(),
            Sprite {
                image: atlas.image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: atlas.layout.clone(),
                    index: clip.first as usize,
                }),
                custom_size: Some(atlas.world_size),
                ..default()
            },
            Anchor(Vec2::new(ax, ay)),
            Transform::from_xyz(0.0, VISUAL_OFFSET_Y, VISUAL_Z),
            // Shown once the textures are loaded (the box covers until then).
            Visibility::Hidden,
        ));
    }
}

/// Largest distance the player can travel in one frame by moving (max fall
/// speed 900 u/s over the 1/30 s frame clamp is 30 u). A bigger jump is a
/// teleport (quick load, respawn): it never counts as a landing.
const TELEPORT_DISTANCE: f32 = 48.0;
/// A landing needs a real fall. Right after a quick load the state reads
/// `Falling` for one frame (the ground contact is only refreshed by the
/// physics step) and must not count as one.
const MIN_FALL_FOR_LANDING: f32 = 0.1;

/// Picks the clip of the gameplay state and the frame of this moment.
pub fn animate_player_visual(
    time: Res<Time>,
    game_state: Option<Res<State<GameState>>>,
    data: Option<Res<GameData>>,
    set: Option<Res<PlayerSpriteSet>>,
    images: Option<Res<Assets<Image>>>,
    mut started: MessageReader<SkillStarted>,
    players: Query<
        (
            &ActionState,
            &FacingDirection,
            &Velocity,
            &Invulnerable,
            &Transform,
            Option<&Grounded>,
            Option<&crate::inventory::systems::EquipmentAppearance>,
        ),
        With<Player>,
    >,
    mut visuals: Query<
        (
            &ChildOf,
            &mut VisualPlayback,
            &mut Sprite,
            &mut Anchor,
            &mut Visibility,
        ),
        With<PlayerVisual>,
    >,
) {
    // Skills confirmed this frame (the visual only reads the message).
    let skills: Vec<(Entity, String)> = started
        .read()
        .filter_map(|s| {
            let def = data.as_ref()?.skills.get(&s.skill_id)?;
            Some((s.caster, def.animation.clone()))
        })
        .collect();
    let Some(set) = set else {
        return;
    };
    let ready = images.is_some_and(|images| set.ready(&images));
    // Gameplay time stops on the status screen: so does the animation.
    let playing = game_state.is_none_or(|s| *s.get() == GameState::Playing);
    let dt = if playing { frame_delta(&time) } else { 0.0 };
    let blink_on = (time.elapsed_secs() * BLINK_RATE).sin() > 0.0;
    for (parent, mut playback, mut sprite, mut anchor, mut visibility) in &mut visuals {
        let Ok((action, facing, velocity, invulnerable, transform, grounded, look)) =
            players.get(parent.parent())
        else {
            continue;
        };
        let state = action.state();
        let position = transform.translation.truncate();
        let moved = playback
            .last_position
            .map_or(f32::INFINITY, |p| p.distance(position));
        playback.last_position = Some(position);
        if let Some((_, animation)) = skills.iter().rev().find(|(c, _)| *c == parent.parent()) {
            playback.skill_clip = Some(animation.clone());
        } else if state != PlayerState::Casting {
            playback.skill_clip = None;
        }
        // Landing: falling last frame, standing this frame, without a teleport.
        let land_total = set
            .manifest
            .clips
            .get(selection::LAND)
            .map_or(0.0, |c| c.total_duration());
        if state == PlayerState::Falling {
            playback.falling_time += dt;
        }
        if playback.state == Some(PlayerState::Falling)
            && state == PlayerState::Idle
            && moved < TELEPORT_DISTANCE
            && playback.falling_time >= MIN_FALL_FOR_LANDING
        {
            playback.landing_left = land_total;
        } else if state != PlayerState::Idle {
            playback.landing_left = 0.0;
        }
        if state != PlayerState::Falling {
            playback.falling_time = 0.0;
        }
        if !ready {
            visibility.set_if_neq(Visibility::Hidden);
            playback.state = Some(state);
            continue;
        }
        let (key, clip, timing, tint) = {
            // V1.5: an attack off the ground shows the air variant (read-only
            // ground contact; only when the manifest has the clip).
            let airborne_attack = state == PlayerState::Attacking
                && grounded.is_some_and(|g| !g.on_ground)
                && set.manifest.clips.contains_key(selection::ATTACK_BASIC_AIR);
            let ctx = SelectionContext {
                skill_animation: playback.skill_clip.as_deref(),
                landing: playback.landing_left > 0.0,
                airborne_attack,
            };
            let plan = select(state, &ctx);
            let Some((key, clip)) = set.manifest.resolve(plan.clip) else {
                continue;
            };
            // The manifest lacks the wanted clip: hold the idle frame, tinted.
            if key == plan.clip {
                (key, clip, plan.timing, Tint::None)
            } else {
                (key, clip, Timing::Hold, plan.missing_tint)
            }
        };
        let Some(atlas) = set.atlases.get(&clip.atlas) else {
            continue;
        };
        let remaining = action.remaining();
        let new_action = timing == Timing::Action && remaining > playback.last_remaining + 1e-6;
        if playback.clip.as_deref() != Some(key) || playback.state != Some(state) || new_action {
            // Same action, other variant (an air attack that lands): keep the
            // action's length so the frame continues from the same progress.
            let variant_swap =
                timing == Timing::Action && playback.state == Some(state) && !new_action;
            playback.clip = Some(key.to_owned());
            playback.elapsed = 0.0;
            if !variant_swap {
                playback.action_total = remaining;
            }
        }
        let frame = match timing {
            Timing::Loop { speed_scaled } => {
                let rate = if speed_scaled {
                    locomotion_rate(velocity.x, clip.speed_reference)
                } else {
                    1.0
                };
                playback.elapsed += dt * rate;
                loop_frame(&clip.durations, playback.elapsed)
            }
            Timing::Action => {
                // Duration and progress come from the gameplay timer, so a
                // faster attack (attack_speed) plays the same frames faster.
                let progress = action_progress(playback.action_total, remaining);
                action_frame(&clip.durations, progress)
            }
            Timing::Once => {
                let frame = once_frame(&clip.durations, playback.elapsed);
                playback.elapsed += dt;
                frame
            }
            Timing::Hold => 0,
        };
        if key == selection::LAND {
            playback.landing_left -= dt;
        }
        playback.state = Some(state);
        playback.last_remaining = remaining;
        let index = clip.first as usize + frame;
        playback.index = index;

        let flip = facing.0 == Facing::Left;
        let (r, g, b) = tint.rgb();
        let alpha = if invulnerable.remaining > 0.0 && state != PlayerState::Dead && blink_on {
            BLINK_ALPHA
        } else {
            1.0
        };
        // V2.0: the equipment's discreet body tint (white without one).
        let equip = look.map_or(crate::inventory::appearance::Rgb::WHITE, |l| l.tint);
        let color = Color::srgba(r * equip.0, g * equip.1, b * equip.2, alpha);
        if sprite.image != atlas.image {
            sprite.image = atlas.image.clone();
        }
        let wanted_atlas = TextureAtlas {
            layout: atlas.layout.clone(),
            index,
        };
        if sprite.texture_atlas.as_ref() != Some(&wanted_atlas) {
            sprite.texture_atlas = Some(wanted_atlas);
        }
        if sprite.custom_size != Some(atlas.world_size) {
            sprite.custom_size = Some(atlas.world_size);
        }
        if sprite.flip_x != flip {
            sprite.flip_x = flip;
        }
        if sprite.color != color {
            sprite.color = color;
        }
        let (ax, ay) = anchor_for(clip.pivot, atlas.cell, flip);
        anchor.set_if_neq(Anchor(Vec2::new(ax, ay)));
        visibility.set_if_neq(Visibility::Inherited);
    }
}
