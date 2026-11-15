//! Held equipment layers (V2.2): the weapon in the right hand and the
//! shield on the left arm (or on the back), as child sprites of the player
//! that follow the frame shown by the body sprite.
//!
//! Presentation only: reads `Belongings`, the body sprite and its playback;
//! never writes gameplay state. Every frame the layers take the sockets of
//! the frame the body shows (`player::visual::sockets`), mirrored with the
//! facing, in front of or behind the body as measured in Blender. Entities
//! are spawned once per layer and updated in place; they are removed only
//! when the item is unequipped or the art cannot show them.
//!
//! The layers are off while the body art still has the sabre drawn in
//! (`weapon_in_body`, every atlas up to V1.5) or without sockets, so a
//! second weapon is never drawn over the baked one. Details:
//! docs/VISUAL_EQUIPMENT_V2_2.md.

use std::collections::HashMap;

use bevy::prelude::*;
use bevy::sprite::Anchor;

use super::appearance::Rgb;
use super::held::{HeldLook, ShieldPlacement, WeaponForm, held_angle};
use super::held_art::{shield_art, weapon_art};
use crate::core::resources::GameData;
use crate::player::components::{Belongings, Player};
use crate::player::visual::sockets::place;
use crate::player::visual::systems::{VISUAL_OFFSET_Y, VISUAL_Z};
use crate::player::visual::{PlayerSpriteSet, PlayerVisual, VisualPlayback};
use crate::world::physics::FacingDirection;

/// The resolved held equipment of the player.
#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Deref)]
pub struct HeldEquipment(pub HeldLook);

/// A held equipment layer (child of the player).
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeldLayer {
    Weapon,
    Shield,
}

/// Depth offsets around the body sprite (`VISUAL_Z`): a hand towards the
/// camera draws over the body, the far hand and the back behind it (but in
/// front of the cape, 0.03).
pub const FRONT_Z: f32 = VISUAL_Z + 0.01;
pub const BEHIND_Z: f32 = VISUAL_Z - 0.01;
/// A shield in the hand reads in front of the body (profile games draw it
/// on the visible side), but behind the weapon.
pub const SHIELD_Z: f32 = VISUAL_Z + 0.005;
/// Atlas of the strike/skill clips: their frames always follow the sabre.
const STRIKE_ATLAS: &str = "combat";
/// Tilt of a shield slung on the back (radians, towards the back).
const BACK_TILT: f32 = -0.35;

/// Which generated image a layer uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArtKey {
    Weapon(WeaponForm, [u8; 3]),
    Shield([u8; 3]),
}

fn rgb_key(c: Rgb) -> [u8; 3] {
    [c.0, c.1, c.2].map(|v| (v.clamp(0.0, 1.0) * 255.0).round() as u8)
}

/// Generated art, built on first use (a handful of images in total).
#[derive(Resource, Debug, Default)]
pub struct HeldArtCache {
    entries: HashMap<ArtKey, (Option<Handle<Image>>, ArtInfo)>,
}

/// Size and anchor data kept with each image.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArtInfo {
    pub world_size: Vec2,
    pub anchor: (f32, f32),
}

impl HeldArtCache {
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    fn get(
        &mut self,
        key: ArtKey,
        images: Option<&mut Assets<Image>>,
    ) -> (Option<Handle<Image>>, ArtInfo) {
        if let Some(entry) = self.entries.get(&key) {
            return entry.clone();
        }
        let art = match key {
            ArtKey::Weapon(form, c) => weapon_art(form, from_key(c)),
            ArtKey::Shield(c) => shield_art(from_key(c)),
        };
        let (w, h) = art.world_size();
        let info = ArtInfo {
            world_size: Vec2::new(w, h),
            anchor: art.anchor(false),
        };
        let handle = images.map(|images| images.add(crate::core::pixel_image::pixel_image(art)));
        self.entries.insert(key, (handle.clone(), info));
        (handle, info)
    }
}

fn from_key(c: [u8; 3]) -> Rgb {
    Rgb(
        f32::from(c[0]) / 255.0,
        f32::from(c[1]) / 255.0,
        f32::from(c[2]) / 255.0,
    )
}

/// Resolves what the hands hold whenever the belongings change.
pub fn sync_held_look(
    mut commands: Commands,
    data: Res<GameData>,
    players: Query<(Entity, Ref<Belongings>, Option<&HeldEquipment>), With<Player>>,
) {
    for (entity, belongings, current) in &players {
        if current.is_some() && !belongings.is_changed() {
            continue;
        }
        let look = HeldEquipment(HeldLook::resolve(&belongings.equipment, &data.items));
        if current != Some(&look) {
            commands.entity(entity).insert(look);
        }
    }
}

/// Where one layer goes this frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayerPose {
    pub translation: Vec3,
    pub rotation: f32,
    pub flip: bool,
}

/// Pure placement of a layer from a socket: mirrored with the facing, the
/// sole line at the bottom of the collision body, front/behind the body.
pub fn layer_pose(x: f32, y: f32, angle: f32, front: bool, facing_sign: f32) -> LayerPose {
    let (x, y, rotation) = place(x, y, angle, facing_sign);
    LayerPose {
        translation: Vec3::new(
            x,
            VISUAL_OFFSET_Y + y,
            if front { FRONT_Z } else { BEHIND_Z },
        ),
        rotation,
        flip: facing_sign < 0.0,
    }
}

type BodyQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static VisualPlayback,
        &'static Sprite,
        &'static Visibility,
    ),
    (With<PlayerVisual>, Without<HeldLayer>),
>;

/// Spawns, places and removes the held layers.
pub fn sync_held_layers(
    mut commands: Commands,
    set: Option<Res<PlayerSpriteSet>>,
    mut cache: ResMut<HeldArtCache>,
    mut images: Option<ResMut<Assets<Image>>>,
    players: Query<(Entity, &HeldEquipment, &FacingDirection, &Children), With<Player>>,
    bodies: BodyQuery,
    mut layers: Query<
        (
            &HeldLayer,
            &mut Sprite,
            &mut Anchor,
            &mut Transform,
            &mut Visibility,
        ),
        Without<PlayerVisual>,
    >,
) {
    for (player, held, facing, children) in &players {
        let body = children.iter().find_map(|c| bodies.get(c).ok());
        let enabled = set.as_deref().is_some_and(|s| s.held_layers_enabled());
        let mut wanted: Vec<(HeldLayer, ArtKey)> = Vec::new();
        if enabled && body.is_some() {
            if let Some(w) = held.weapon {
                wanted.push((HeldLayer::Weapon, ArtKey::Weapon(w.form, rgb_key(w.accent))));
            }
            if let Some(s) = held.shield {
                wanted.push((HeldLayer::Shield, ArtKey::Shield(rgb_key(s.color))));
            }
        }
        // Remove layers that are no longer wanted.
        let mut present: Vec<(Entity, HeldLayer)> = Vec::new();
        for child in children.iter() {
            if let Ok((kind, ..)) = layers.get(child) {
                if wanted.iter().any(|(k, _)| k == kind) {
                    present.push((child, *kind));
                } else {
                    commands.entity(child).try_despawn();
                }
            }
        }
        let (Some(set), Some((playback, body_sprite, body_visibility))) = (set.as_deref(), body)
        else {
            continue;
        };
        let clip = playback
            .clip
            .as_deref()
            .and_then(|key| Some((key, set.manifest.clips.get(key)?)));
        let striking = clip.is_some_and(|(_, c)| c.atlas == STRIKE_ATLAS);
        let frame = clip.and_then(|(key, clip)| {
            let index = playback.index.checked_sub(clip.first as usize)?;
            set.sockets.as_ref()?.frame(key, index)
        });
        let sign = facing.sign();
        let alpha = body_sprite.color.alpha();
        for (kind, key) in wanted {
            let (handle, info) = cache.get(key, images.as_deref_mut());
            let pose = frame.map(|f| match kind {
                HeldLayer::Weapon => {
                    let angle = held.weapon.map_or(f.weapon.angle, |w| {
                        held_angle(w.form, f.weapon.angle, striking)
                    });
                    layer_pose(f.weapon.x, f.weapon.y, angle, f.weapon.front, sign)
                }
                HeldLayer::Shield => match held.shield.map(|s| s.placement) {
                    Some(ShieldPlacement::Back) => {
                        let mut pose = layer_pose(f.back.x, f.back.y - 8.0, 90.0, false, sign);
                        pose.rotation += BACK_TILT * sign;
                        pose
                    }
                    _ => {
                        let mut pose = layer_pose(f.hand_l.x, f.hand_l.y, 0.0, true, sign);
                        pose.translation.z = SHIELD_Z;
                        pose
                    }
                },
            });
            let visible = pose.is_some() && *body_visibility != Visibility::Hidden;
            let pose = pose.unwrap_or(LayerPose {
                translation: Vec3::new(0.0, 0.0, BEHIND_Z),
                rotation: 0.0,
                flip: false,
            });
            let (ax, ay) = info.anchor;
            let anchor = Anchor(Vec2::new(if pose.flip { -ax } else { ax }, ay));
            let transform = Transform::from_translation(pose.translation)
                .with_rotation(Quat::from_rotation_z(pose.rotation));
            let visibility = if visible {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
            let existing = present.iter().find(|(_, k)| *k == kind).map(|(e, _)| *e);
            let Some(entity) = existing else {
                let mut sprite = match handle.clone() {
                    Some(image) => Sprite::from_image(image),
                    None => Sprite::from_color(Color::WHITE, info.world_size),
                };
                sprite.custom_size = Some(info.world_size);
                sprite.flip_x = pose.flip;
                commands
                    .entity(player)
                    .with_child((kind, sprite, anchor, transform, visibility));
                continue;
            };
            let Ok((_, mut sprite, mut current_anchor, mut current, mut vis)) =
                layers.get_mut(entity)
            else {
                continue;
            };
            if let Some(image) = handle
                && sprite.image != image
            {
                sprite.image = image;
            }
            if sprite.custom_size != Some(info.world_size) {
                sprite.custom_size = Some(info.world_size);
            }
            if sprite.flip_x != pose.flip {
                sprite.flip_x = pose.flip;
            }
            // Blink with the body (invulnerability); the form tint, when
            // active, is applied afterwards by `transform::visual::tint_form`.
            let color = Color::srgba(1.0, 1.0, 1.0, alpha);
            if sprite.color != color {
                sprite.color = color;
            }
            current_anchor.set_if_neq(anchor);
            if *current != transform {
                *current = transform;
            }
            vis.set_if_neq(visibility);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn facing_mirrors_the_layer_and_keeps_the_height() {
        let right = layer_pose(10.0, 30.0, 40.0, true, 1.0);
        let left = layer_pose(10.0, 30.0, 40.0, true, -1.0);
        assert_eq!(right.translation.x, -left.translation.x);
        assert_eq!(right.translation.y, left.translation.y);
        assert!((right.rotation + left.rotation).abs() < 1e-6);
        assert!(!right.flip && left.flip);
        assert_eq!(right.translation.y, VISUAL_OFFSET_Y + 30.0);
    }

    #[test]
    fn depth_puts_the_far_hand_behind_the_body() {
        assert!(layer_pose(0.0, 0.0, 0.0, true, 1.0).translation.z > VISUAL_Z);
        assert!(layer_pose(0.0, 0.0, 0.0, false, 1.0).translation.z < VISUAL_Z);
        // ...but in front of the cape (0.03).
        const { assert!(BEHIND_Z > 0.03) };
    }

    #[test]
    fn the_cache_builds_each_image_once() {
        let mut cache = HeldArtCache::default();
        let key = ArtKey::Weapon(WeaponForm::Axe, [10, 20, 30]);
        let (_, a) = cache.get(key, None);
        let (_, b) = cache.get(key, None);
        assert_eq!(a, b);
        cache.get(ArtKey::Shield([1, 2, 3]), None);
        assert_eq!(cache.len(), 2);
    }
}
