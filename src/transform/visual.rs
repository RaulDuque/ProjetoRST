//! Presentation of the transformation (V1.9). Reads gameplay state, never
//! writes it.
//!
//! No new asset: the Spectral Form recolours the existing protagonist
//! sprite (tint and transparency from the definition), adds a soft halo
//! behind it (a radial gradient generated in memory at startup, never a
//! file), rising motes, and a burst of motes when the form starts or ends. The look follows `Transformation`
//! in the same frame as the physics profiles, so the body never looks human
//! while it moves like a spectre (or the other way round): a form waiting
//! to end inside a veil still looks spectral. Veils of the room shimmer.

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::sprite::Anchor;

use super::definitions::{FormVisual, TransformationDefinition};
use super::systems::Transformation;
use crate::core::constants::PLAYER_SIZE;
use crate::core::events::{FormChange, FormChanged};
use crate::core::resources::GameData;
use crate::player::components::{Player, Vitals};
use crate::player::visual::PlayerVisual;
use crate::transform::state::EndReason;
use crate::world::maps::{RoomEntity, VeilSurface};

/// Soft radial gradient used by the halo (generated, white, alpha falloff).
#[derive(Resource, Debug, Clone)]
pub struct FormGlowImage(pub Handle<Image>);

const GLOW_TEXELS: u32 = 64;
/// Halo size around the body (world units).
const GLOW_SIZE: Vec2 = Vec2::new(78.0, 96.0);

/// Builds the halo texture once (presentation only; skipped in headless
/// apps without image assets).
pub fn create_glow_image(mut commands: Commands, images: Option<ResMut<Assets<Image>>>) {
    let Some(mut images) = images else {
        return;
    };
    let n = GLOW_TEXELS;
    let mut data = Vec::with_capacity((n * n * 4) as usize);
    for y in 0..n {
        for x in 0..n {
            let dx = (x as f32 + 0.5) / n as f32 * 2.0 - 1.0;
            let dy = (y as f32 + 0.5) / n as f32 * 2.0 - 1.0;
            let falloff = (1.0 - (dx * dx + dy * dy).sqrt()).clamp(0.0, 1.0);
            data.extend([255, 255, 255, (falloff * falloff * 255.0) as u8]);
        }
    }
    let image = Image::new(
        Extent3d {
            width: n,
            height: n,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    commands.insert_resource(FormGlowImage(images.add(image)));
}

/// Outline child of the player while a form is active.
#[derive(Component, Debug, Clone, Copy)]
pub struct FormOutline;

/// A particle of the form (world space, despawned with the room): the
/// motes around an active form rise; a burst marks its start and end.
#[derive(Component, Debug, Clone, Copy)]
pub struct FormMote {
    pub life: f32,
    pub total: f32,
    pub velocity: Vec2,
}

const OUTLINE_SCALE: f32 = 1.12;
/// Behind the sprite child.
const OUTLINE_BEHIND: f32 = 0.01;
const MOTE_LIFE: f32 = 0.7;
const MOTE_SIZE: f32 = 4.0;
const MOTE_RISE: f32 = 36.0;
const BURST_MOTES: u32 = 16;
const BURST_LIFE: f32 = 0.45;
const BURST_SPEED: f32 = 110.0;
const LOW_MP_PULSE_HZ: f32 = 4.0;
const VEIL_SHIMMER_HZ: f32 = 0.8;
const Z_EFFECTS: f32 = crate::core::constants::Z_PLAYER + 0.4;

fn active_def<'a>(
    data: &'a GameData,
    form: &Transformation,
) -> Option<&'a TransformationDefinition> {
    form.active().and_then(|f| data.transformations.get(f))
}

/// Alpha multiplier: a slow pulse when the upkeep has little time left.
fn low_mp_factor(def: &TransformationDefinition, mp: i32, seconds: f32) -> f32 {
    let rate = def.costs.mp_per_second;
    if rate <= 0.0 || mp as f32 / rate >= def.visual.low_mp_seconds {
        return 1.0;
    }
    0.6 + 0.4
        * (seconds * LOW_MP_PULSE_HZ * std::f32::consts::TAU)
            .sin()
            .abs()
}

fn tinted(visual: &FormVisual, base_alpha: f32, factor: f32) -> Color {
    let (r, g, b) = visual.tint;
    Color::srgba(r, g, b, base_alpha * visual.alpha * factor)
}

/// Recolours the sprite (and the placeholder box) while a form is active.
/// Runs after the sprite visual, which rewrites the colour every frame.
pub fn tint_form(
    time: Res<Time>,
    data: Res<GameData>,
    mut players: Query<(&Transformation, &Vitals, &Children, &mut Sprite), With<Player>>,
    mut sprites: Query<
        &mut Sprite,
        (
            Or<(
                With<PlayerVisual>,
                With<crate::inventory::visual::CapeLayer>,
                With<crate::inventory::held_visual::HeldLayer>,
            )>,
            Without<Player>,
        ),
    >,
) {
    // V2.0: equipment layers (the cape; V2.2: the held weapon and shield)
    // turn spectral with the body, so no opaque human layer stays over the
    // translucent form.
    for (form, vitals, children, mut sprite) in &mut players {
        let Some(def) = active_def(&data, form) else {
            continue;
        };
        let factor = low_mp_factor(def, vitals.mp.current(), time.elapsed_secs());
        if sprite.color.alpha() > 0.0 {
            sprite.color = tinted(&def.visual, sprite.color.alpha(), factor);
        }
        for child in children.iter() {
            if let Ok(mut child_sprite) = sprites.get_mut(child) {
                let base = child_sprite.color.alpha();
                child_sprite.color = tinted(&def.visual, base, factor);
            }
        }
    }
}

/// Keeps the outline child in step with the form and the shown frame.
pub fn sync_outline(
    mut commands: Commands,
    data: Res<GameData>,
    players: Query<(Entity, &Transformation, &Children), With<Player>>,
    visuals: Query<
        (&Sprite, &Anchor, &Visibility, &Transform),
        (With<PlayerVisual>, Without<FormOutline>),
    >,
    mut outlines: Query<(Entity, &mut Sprite, &mut Anchor, &mut Transform), With<FormOutline>>,
) {
    for (player, form, children) in &players {
        let own: Vec<Entity> = children.iter().filter(|c| outlines.contains(*c)).collect();
        let Some(def) = active_def(&data, form) else {
            for outline in own {
                commands.entity(outline).try_despawn();
            }
            continue;
        };
        let (r, g, b) = def.visual.outline;
        let color = Color::srgba(r, g, b, def.visual.outline_alpha);
        let shown = children.iter().find_map(|c| {
            visuals
                .get(c)
                .ok()
                .filter(|(_, _, v, _)| **v != Visibility::Hidden)
        });
        let Some(&outline) = own.first() else {
            commands.entity(player).with_child((
                FormOutline,
                Sprite::from_color(
                    color,
                    Vec2::new(PLAYER_SIZE.0, PLAYER_SIZE.1) * OUTLINE_SCALE,
                ),
                Anchor::CENTER,
                Transform::from_xyz(0.0, 0.0, -OUTLINE_BEHIND),
            ));
            continue;
        };
        let Ok((_, mut sprite, mut anchor, mut transform)) = outlines.get_mut(outline) else {
            continue;
        };
        match shown {
            Some((frame, frame_anchor, _, frame_transform)) => {
                sprite.image = frame.image.clone();
                sprite.texture_atlas = frame.texture_atlas.clone();
                sprite.flip_x = frame.flip_x;
                sprite.custom_size = frame.custom_size.map(|s| s * OUTLINE_SCALE);
                *anchor = *frame_anchor;
                // Same place as the sprite child (its vertical offset), behind it.
                transform.translation = frame_transform.translation - Vec3::Z * OUTLINE_BEHIND;
            }
            None => {
                transform.translation = Vec3::new(0.0, 0.0, -OUTLINE_BEHIND);
                sprite.image = Handle::default();
                sprite.texture_atlas = None;
                sprite.custom_size = Some(Vec2::new(PLAYER_SIZE.0, PLAYER_SIZE.1) * OUTLINE_SCALE);
                *anchor = Anchor::CENTER;
            }
        }
        sprite.color = color;
    }
}

/// Halo child of the player while a form is active.
#[derive(Component, Debug, Clone, Copy)]
pub struct FormHalo;

/// Halo opacity (the low-MP pulse multiplies it).
const HALO_ALPHA: f32 = 0.42;
/// Behind the outline.
const HALO_BEHIND: f32 = 0.02;

/// A soft glow behind an active form (needs [`FormGlowImage`]).
pub fn sync_halo(
    mut commands: Commands,
    time: Res<Time>,
    data: Res<GameData>,
    glow: Option<Res<FormGlowImage>>,
    players: Query<(Entity, &Transformation, &Vitals, &Children), With<Player>>,
    mut halos: Query<(Entity, &mut Sprite), With<FormHalo>>,
) {
    let Some(glow) = glow else {
        return;
    };
    for (player, form, vitals, children) in &players {
        let own: Vec<Entity> = children.iter().filter(|c| halos.contains(*c)).collect();
        let Some(def) = active_def(&data, form) else {
            for halo in own {
                commands.entity(halo).try_despawn();
            }
            continue;
        };
        let (r, g, b) = def.visual.outline;
        let factor = low_mp_factor(def, vitals.mp.current(), time.elapsed_secs());
        let color = Color::srgba(r, g, b, HALO_ALPHA * factor);
        match own.first().and_then(|h| halos.get_mut(*h).ok()) {
            Some((_, mut sprite)) => sprite.color = color,
            None => {
                commands.entity(player).with_child((
                    FormHalo,
                    Sprite {
                        image: glow.0.clone(),
                        custom_size: Some(GLOW_SIZE),
                        color,
                        ..default()
                    },
                    Transform::from_xyz(0.0, 0.0, -HALO_BEHIND),
                ));
            }
        }
    }
}

/// Small deterministic hash for particle offsets (the gameplay RNG is not
/// touched by presentation).
fn scatter(n: u32) -> f32 {
    let x = n.wrapping_mul(2_654_435_761).rotate_left(13) ^ 0x9e37_79b9;
    (x % 1000) as f32 / 1000.0
}

/// Spawns motes around an active form and a burst on start and end.
pub fn spawn_form_effects(
    mut commands: Commands,
    time: Res<Time>,
    data: Res<GameData>,
    mut changed: MessageReader<FormChanged>,
    players: Query<(&Transformation, &Transform), With<Player>>,
    mut clock: Local<(f32, u32)>,
) {
    let Ok((form, transform)) = players.single() else {
        return;
    };
    let at = transform.translation.truncate();
    for event in changed.read() {
        let Some(def) = data.transformations.get(event.form) else {
            continue;
        };
        let color = match event.change {
            FormChange::Entered => {
                let (r, g, b) = def.visual.outline;
                Color::srgba(r, g, b, 0.9)
            }
            FormChange::Left(EndReason::Death) => Color::srgba(0.5, 0.5, 0.55, 0.8),
            FormChange::Left(_) => Color::srgba(1.0, 0.93, 0.8, 0.85),
            FormChange::Ending(_) | FormChange::Refused(_) => continue,
        };
        for i in 0..BURST_MOTES {
            let angle = i as f32 / BURST_MOTES as f32 * std::f32::consts::TAU;
            let direction = Vec2::new(angle.cos(), angle.sin());
            commands.spawn((
                RoomEntity,
                FormMote {
                    life: BURST_LIFE,
                    total: BURST_LIFE,
                    velocity: direction * BURST_SPEED,
                },
                Sprite::from_color(color, Vec2::splat(MOTE_SIZE + 1.0)),
                Transform::from_xyz(
                    at.x + direction.x * PLAYER_SIZE.0 * 0.5,
                    at.y + direction.y * PLAYER_SIZE.1 * 0.5,
                    Z_EFFECTS,
                ),
            ));
        }
    }
    let Some(def) = active_def(&data, form) else {
        clock.0 = 0.0;
        return;
    };
    clock.0 += time.delta_secs();
    while clock.0 >= def.visual.mote_interval {
        clock.0 -= def.visual.mote_interval;
        clock.1 = clock.1.wrapping_add(1);
        let dx = (scatter(clock.1) - 0.5) * PLAYER_SIZE.0 * 1.4;
        let dy = (scatter(clock.1.wrapping_mul(7)) - 0.5) * PLAYER_SIZE.1;
        let (r, g, b) = def.visual.outline;
        commands.spawn((
            RoomEntity,
            FormMote {
                life: MOTE_LIFE,
                total: MOTE_LIFE,
                velocity: Vec2::new(0.0, MOTE_RISE),
            },
            Sprite::from_color(Color::srgba(r, g, b, 0.8), Vec2::splat(MOTE_SIZE)),
            Transform::from_xyz(at.x + dx, at.y + dy, Z_EFFECTS),
        ));
    }
}

/// Motes move and fade, then disappear.
pub fn animate_form_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut motes: Query<(Entity, &mut FormMote, &mut Transform, &mut Sprite)>,
) {
    let dt = time.delta_secs();
    for (entity, mut mote, mut transform, mut sprite) in &mut motes {
        mote.life -= dt;
        if mote.life <= 0.0 {
            commands.entity(entity).try_despawn();
            continue;
        }
        transform.translation += (mote.velocity * dt).extend(0.0);
        let alpha = 0.85 * mote.life / mote.total;
        sprite.color = sprite.color.with_alpha(alpha);
    }
}

/// Veils shimmer gently so they read as something other than masonry.
pub fn shimmer_veils(time: Res<Time>, mut veils: Query<(&VeilSurface, &mut Sprite)>) {
    let wave = (time.elapsed_secs() * VEIL_SHIMMER_HZ * std::f32::consts::TAU).sin();
    for (veil, mut sprite) in &mut veils {
        sprite.color = veil
            .color
            .with_alpha(veil.color.alpha() * (0.8 + 0.2 * wave));
    }
}
