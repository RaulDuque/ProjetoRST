//! Presentation of the equipment (V2.0): the cape layer. Reads gameplay
//! state, never writes it.
//!
//! The cape is a child sprite of the player, behind the protagonist sprite,
//! hung from the shoulders on the side the character turns its back to.
//! Its texture is generated in memory once (white, shaded, ragged hem) and
//! tinted with the item's colour, so any number of capes costs one image.
//! It sways with the horizontal speed and lifts while falling; it is hidden
//! while dodging (the roll) and when fallen. During the Spectral Form the
//! transformation recolours it like the body (`transform::visual::tint_form`).
//! The body tint is applied by the sprite visual and the strike trail by the
//! basic attack. Details: docs/EQUIPMENT_EXPANSION_V2_0.md §14-15.

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::sprite::Anchor;

use super::systems::EquipmentAppearance;
use crate::player::components::{ActionState, Player};
use crate::player::state::PlayerState;
use crate::world::physics::{FacingDirection, Velocity};

/// Generated cape texture (white; tinted per item).
#[derive(Resource, Debug, Clone)]
pub struct CapeImage(pub Handle<Image>);

/// The cape child of the player.
#[derive(Component, Debug, Clone, Copy)]
pub struct CapeLayer;

const CAPE_TEXELS: (u32, u32) = (24, 64);
/// Width of the cape at the hem (world units).
pub const CAPE_WIDTH: f32 = 20.0;
/// Shoulder line above the body centre, and how far behind it hangs.
const SHOULDER_Y: f32 = 13.0;
const BEHIND_X: f32 = 5.0;
/// The crouch lowers the shoulders.
const CROUCH_DROP: f32 = 12.0;
/// Behind the protagonist sprite (`player::visual::systems::VISUAL_Z` 0.05)
/// and in front of the placeholder box.
const CAPE_Z: f32 = 0.03;
/// Sway: radians per unit of speed, and its limit.
const SWAY_PER_SPEED: f32 = 0.0016;
const MAX_SWAY: f32 = 0.7;
const IDLE_SWAY: f32 = 0.04;
/// Hem line (fraction of the texture height) around which the points wave.
const HEM: f32 = 0.92;

/// Builds the cape texture once (skipped in headless apps without image
/// assets; the cape is then a plain rectangle, which tests can still see).
pub fn create_cape_image(mut commands: Commands, images: Option<ResMut<Assets<Image>>>) {
    let Some(mut images) = images else {
        return;
    };
    let (w, h) = CAPE_TEXELS;
    let mut data = Vec::with_capacity((w * h * 4) as usize);
    for y in 0..h {
        let t = y as f32 / (h - 1) as f32; // 0 at the shoulders
        // Narrow at the shoulders, flaring towards the hem.
        let half = (0.3 + 0.2 * t) * w as f32;
        for x in 0..w {
            let dx = (x as f32 + 0.5) - w as f32 / 2.0;
            let phase = (x as f32 / w as f32 * 3.0 * std::f32::consts::TAU).cos();
            // Ragged hem: three points.
            let bottom = HEM + 0.05 * phase;
            let inside = dx.abs() <= half && t <= bottom;
            if !inside {
                data.extend([0, 0, 0, 0]);
                continue;
            }
            // Folds: vertical shading, darker at the edges and the top.
            let fold = 0.82 + 0.18 * (dx / 2.5).cos().abs();
            let edge = 1.0 - 0.35 * (dx.abs() / half).powi(2);
            let shade = (fold * edge * (0.75 + 0.25 * t)).clamp(0.0, 1.0);
            let v = (shade * 255.0) as u8;
            data.extend([v, v, v, 255]);
        }
    }
    let image = Image::new(
        Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    commands.insert_resource(CapeImage(images.add(image)));
}

fn cape_sprite(image: Option<&CapeImage>, color: Color, length: f32) -> Sprite {
    let size = Vec2::new(CAPE_WIDTH, length);
    match image {
        Some(image) => Sprite {
            image: image.0.clone(),
            custom_size: Some(size),
            color,
            ..default()
        },
        None => Sprite::from_color(color, size),
    }
}

/// Spawns, updates and removes the cape child from the appearance.
pub fn sync_cape_layer(
    mut commands: Commands,
    time: Res<Time>,
    image: Option<Res<CapeImage>>,
    players: Query<
        (
            Entity,
            Option<&EquipmentAppearance>,
            &ActionState,
            &FacingDirection,
            &Velocity,
            Option<&Children>,
        ),
        With<Player>,
    >,
    mut capes: Query<(&mut Sprite, &mut Transform, &mut Visibility), With<CapeLayer>>,
) {
    for (player, look, action, facing, velocity, children) in &players {
        let cape = look.and_then(|l| l.cape);
        let existing = children
            .into_iter()
            .flatten()
            .copied()
            .find(|c| capes.contains(*c));
        let Some(cape) = cape else {
            if let Some(child) = existing {
                commands.entity(child).try_despawn();
            }
            continue;
        };
        let color = Color::srgb(cape.color.0, cape.color.1, cape.color.2);
        let Some(child) = existing else {
            commands.entity(player).with_child((
                CapeLayer,
                cape_sprite(image.as_deref(), color, cape.length),
                Anchor::TOP_CENTER,
                Transform::from_xyz(-facing.sign() * BEHIND_X, SHOULDER_Y, CAPE_Z),
            ));
            continue;
        };
        let Ok((mut sprite, mut transform, mut visibility)) = capes.get_mut(child) else {
            continue;
        };
        let state = action.state();
        let hidden = matches!(state, PlayerState::Dead | PlayerState::Dodging);
        visibility.set_if_neq(if hidden {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        });
        let sign = facing.sign();
        let drop = if state == PlayerState::Crouching {
            CROUCH_DROP
        } else {
            0.0
        };
        transform.translation.x = -sign * BEHIND_X;
        transform.translation.y = SHOULDER_Y - drop;
        // Running pushes the hem back; falling lifts it; a slow breath idle.
        let speed = velocity.x.abs() + (-velocity.y).max(0.0) * 0.5;
        let idle = IDLE_SWAY * (time.elapsed_secs() * 2.0).sin();
        let sway = (speed * SWAY_PER_SPEED).min(MAX_SWAY) + idle;
        transform.rotation = Quat::from_rotation_z(-sign * sway);
        sprite.flip_x = sign < 0.0;
        sprite.custom_size = Some(Vec2::new(CAPE_WIDTH, cape.length));
        sprite.color = color;
    }
}
