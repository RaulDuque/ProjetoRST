//! Bevy side of the scenery (V2.3): spawns the backdrop, decoration, lights
//! and fog of the current room, moves the parallax layers with the camera
//! and animates the lights and the mist.
//!
//! Presentation only: every entity is a `RoomEntity` sprite without any
//! collision component, so the room's physics (`RoomGeometry`) never sees
//! it and the room change removes it with the room. Images are generated
//! once per distinct piece (`SceneryCache`) and shared. Effects can be
//! switched off in `assets/data/camera.ron` (`effects`).

use std::collections::HashMap;

use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;

use super::camera::{CameraConfig, MainCamera, view_size};
use super::exploration_systems::WorldGraph;
use super::maps::{CurrentRoom, RoomEntity};
use super::physics::RoomGeometry;
use super::scenery::{DecorLayer, parallax_centre, parallax_extent};
use super::scenery_art::{BackdropLayer, TILE, backdrop_layers, decor_art, light_of};
use crate::core::constants::{
    Z_BACKDROP_FAR, Z_BACKDROP_MID, Z_DECOR_BACK, Z_DECOR_FRONT, Z_FOG, Z_LIGHTS,
};
use crate::core::pixel_art::{PixelArt, radial_glow, vertical_fade};
use crate::core::pixel_image::pixel_image;

/// What a scenery entity is (tests and reports count them).
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SceneryPiece {
    Backdrop,
    Decor,
    Light,
    Fog,
}

/// A parallax layer (see `scenery::parallax_centre`).
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct ParallaxLayer {
    pub factor: f32,
    pub room: Vec2,
}

/// A light that breathes (torches, lanterns); windows stay steady.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Flicker {
    pub base_alpha: f32,
    pub phase: f32,
}

/// Low mist drifting slowly around its base position.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct FogDrift {
    pub base_x: f32,
}

/// Opacity of the backdrop layers, of the front decoration and of lights.
const FRONT_ALPHA: f32 = 0.8;
const LIGHT_ALPHA: f32 = 0.3;
const WINDOW_LIGHT_ALPHA: f32 = 0.22;
/// Flicker: share of the alpha that varies, and its speed (rad/s).
const FLICKER_DEPTH: f32 = 0.18;
const FLICKER_SPEED: f32 = 9.0;
/// Mist: band height, its bottom (the floor line), drift amplitude/speed.
const FOG_HEIGHT: f32 = 170.0;
const FOG_FLOOR: f32 = 40.0;
const FOG_DRIFT: f32 = 24.0;
const FOG_SPEED: f32 = 0.25;
/// Margin of a parallax layer beyond the view (world units).
const PARALLAX_MARGIN: f32 = 16.0;

/// Size of a backdrop layer: whole tiles, at least the parallax extent, so
/// the repeat never starts with a cut tile (the composition of each tile,
/// e.g. windows above the floor line, is kept).
pub fn layer_size(room: Vec2, view: Vec2, factor: f32) -> Vec2 {
    let whole = |extent: f32, tile: f32| (extent / tile).ceil().max(1.0) * tile;
    Vec2::new(
        whole(
            parallax_extent(room.x, view.x, factor, PARALLAX_MARGIN),
            TILE.0,
        ),
        whole(
            parallax_extent(room.y, view.y, factor, PARALLAX_MARGIN),
            TILE.1,
        ),
    )
}

/// Generated images, by piece (shared by every room that uses them).
#[derive(Resource, Debug, Default)]
pub struct SceneryCache {
    entries: HashMap<String, CachedArt>,
    layers: HashMap<String, (f32, bool)>,
}

#[derive(Debug, Clone)]
struct CachedArt {
    image: Option<Handle<Image>>,
    size: Vec2,
    anchor: Vec2,
}

impl SceneryCache {
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    fn contains(&self, key: &str) -> bool {
        self.entries.contains_key(key)
    }

    fn layer_params(&self, key: &str) -> Option<(f32, bool)> {
        self.layers.get(key).copied()
    }

    fn set_layer_params(&mut self, key: String, params: (f32, bool)) {
        self.layers.insert(key, params);
    }

    fn get(
        &mut self,
        key: String,
        images: Option<&mut Assets<Image>>,
        build: impl FnOnce() -> PixelArt,
    ) -> CachedArt {
        if let Some(c) = self.entries.get(&key) {
            return c.clone();
        }
        let art = build();
        let (w, h) = art.world_size();
        let (ax, ay) = art.anchor(false);
        let cached = CachedArt {
            image: images.map(|images| images.add(pixel_image(art))),
            size: Vec2::new(w, h),
            anchor: Vec2::new(ax, ay),
        };
        self.entries.insert(key, cached.clone());
        cached
    }
}

fn sprite_for(art: &CachedArt, color: Color) -> Sprite {
    let mut sprite = match &art.image {
        Some(image) => Sprite::from_image(image.clone()),
        None => Sprite::from_color(Color::WHITE, art.size),
    };
    sprite.custom_size = Some(art.size);
    sprite.color = color;
    sprite
}

fn rgb(c: (f32, f32, f32)) -> Color {
    Color::srgb(c.0, c.1, c.2)
}

/// Spawns the scenery of the room just loaded (with the NPCs, in
/// `RoomStep::React`).
pub fn spawn_scenery(
    mut commands: Commands,
    world: Option<Res<WorldGraph>>,
    current: Option<Res<CurrentRoom>>,
    config: Option<Res<CameraConfig>>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut cache: ResMut<SceneryCache>,
    mut images: Option<ResMut<Assets<Image>>>,
    mut prepared: Local<Option<String>>,
) {
    let (Some(world), Some(current)) = (world, current) else {
        return;
    };
    if prepared.as_deref() == Some(current.0.as_str()) && !current.is_changed() {
        return;
    }
    *prepared = Some(current.0.clone());
    let Some(room) = world.0.room(&current.0) else {
        return;
    };
    let Some(visual) = world.0.regions.get(&room.region).and_then(|r| r.visual) else {
        return;
    };
    let settings = config.map(|c| c.0).unwrap_or_default();
    let effects = settings.effects;
    let (w, h) = room.size();
    let room_size = Vec2::new(w, h);
    let view = view_size(&settings, window.single().ok());
    let key = format!("{visual:?}");
    if effects.parallax {
        // Generate the layers only when they are not cached yet.
        let keys: Vec<String> = (0..2).map(|i| format!("backdrop{i}:{key}")).collect();
        let mut fresh: Vec<Option<BackdropLayer>> = if keys.iter().all(|k| cache.contains(k)) {
            vec![None, None]
        } else {
            backdrop_layers(&visual).into_iter().map(Some).collect()
        };
        for (i, key) in keys.into_iter().enumerate() {
            let built = fresh.get_mut(i).and_then(Option::take);
            let (factor, tile_y) = built
                .as_ref()
                .map(|l| (l.factor, l.tile_y))
                .or_else(|| cache.layer_params(&key))
                .unwrap_or((1.0, false));
            let cached = cache.get(key.clone(), images.as_deref_mut(), || {
                built.map_or_else(PixelArt::empty, |l| l.art)
            });
            cache.set_layer_params(key, (factor, tile_y));
            let size = layer_size(room_size, view, factor);
            let mut sprite = sprite_for(&cached, Color::WHITE);
            sprite.custom_size = Some(size);
            sprite.image_mode = SpriteImageMode::Tiled {
                tile_x: true,
                tile_y,
                stretch_value: 1.0,
            };
            let z = if i == 0 {
                Z_BACKDROP_FAR
            } else {
                Z_BACKDROP_MID
            };
            commands.spawn((
                RoomEntity,
                SceneryPiece::Backdrop,
                ParallaxLayer {
                    factor,
                    room: room_size,
                },
                sprite,
                Transform::from_xyz(w / 2.0, h / 2.0, z),
            ));
        }
    }
    if effects.decor || effects.lights {
        for (i, d) in room.decor.iter().enumerate() {
            if effects.decor {
                let cached = cache.get(
                    format!("decor:{:?}:{key}", d.kind),
                    images.as_deref_mut(),
                    || decor_art(d.kind, &visual),
                );
                let (front, alpha) = match d.layer {
                    DecorLayer::Back => (false, 1.0),
                    DecorLayer::Front => (true, FRONT_ALPHA),
                };
                let mut sprite = sprite_for(&cached, Color::srgba(1.0, 1.0, 1.0, alpha));
                sprite.flip_x = d.flip;
                let anchor = Vec2::new(
                    if d.flip {
                        -cached.anchor.x
                    } else {
                        cached.anchor.x
                    },
                    cached.anchor.y,
                );
                commands.spawn((
                    RoomEntity,
                    SceneryPiece::Decor,
                    sprite,
                    Anchor(anchor),
                    Transform::from_xyz(
                        d.at.0,
                        d.at.1,
                        if front { Z_DECOR_FRONT } else { Z_DECOR_BACK },
                    ),
                ));
            }
            if effects.lights
                && let Some((radius, offset)) = light_of(d.kind)
            {
                let cached = cache.get(format!("glow:{radius}"), images.as_deref_mut(), || {
                    radial_glow(radius, 0.5)
                });
                let steady = matches!(d.kind, super::scenery::DecorKind::TallWindow { .. });
                let base = if steady {
                    WINDOW_LIGHT_ALPHA
                } else {
                    LIGHT_ALPHA
                };
                let (r, g, b) = visual.light;
                let mut entity = commands.spawn((
                    RoomEntity,
                    SceneryPiece::Light,
                    sprite_for(&cached, Color::srgba(r, g, b, base)),
                    Transform::from_xyz(d.at.0 + offset.0, d.at.1 + offset.1, Z_LIGHTS),
                ));
                if !steady {
                    entity.insert(Flicker {
                        base_alpha: base,
                        phase: i as f32 * 1.7,
                    });
                }
            }
        }
    }
    if effects.fog
        && let Some(fog) = visual.fog
    {
        let cached = cache.get("fog".into(), images.as_deref_mut(), || {
            vertical_fade(64.0, FOG_HEIGHT, 0.25)
        });
        let mut sprite = sprite_for(&cached, rgb(fog.color).with_alpha(fog.density));
        // The fade is transparent at the top, dense at the bottom: on the floor.
        sprite.custom_size = Some(Vec2::new(w + 2.0 * FOG_DRIFT, FOG_HEIGHT));
        commands.spawn((
            RoomEntity,
            SceneryPiece::Fog,
            FogDrift { base_x: w / 2.0 },
            sprite,
            Transform::from_xyz(w / 2.0, FOG_FLOOR + FOG_HEIGHT / 2.0, Z_FOG),
        ));
    }
}

/// Places the parallax layers for the camera of this frame.
pub fn update_parallax(
    config: Option<Res<CameraConfig>>,
    window: Query<&Window, With<PrimaryWindow>>,
    geometry: Option<Res<RoomGeometry>>,
    camera: Query<&Transform, (With<MainCamera>, Without<ParallaxLayer>)>,
    mut layers: Query<(&ParallaxLayer, &mut Transform, &mut Sprite)>,
) {
    let Ok(cam) = camera.single() else {
        return;
    };
    let settings = config.map(|c| c.0).unwrap_or_default();
    let view = view_size(&settings, window.single().ok());
    for (layer, mut t, mut sprite) in &mut layers {
        // Only layers of the loaded room (a despawned room's layer may live
        // one more frame).
        if geometry.as_ref().is_some_and(|g| g.size != layer.room) {
            continue;
        }
        t.translation.x = parallax_centre(cam.translation.x, layer.room.x, layer.factor);
        t.translation.y = parallax_centre(cam.translation.y, layer.room.y, layer.factor);
        let size = layer_size(layer.room, view, layer.factor);
        if sprite.custom_size != Some(size) {
            sprite.custom_size = Some(size);
        }
    }
}

/// Torches and lanterns breathe; the mist drifts.
pub fn animate_scenery(
    time: Res<Time>,
    mut lights: Query<(&Flicker, &mut Sprite), Without<FogDrift>>,
    mut fog: Query<(&FogDrift, &mut Transform), Without<Flicker>>,
) {
    let t = time.elapsed_secs();
    for (f, mut sprite) in &mut lights {
        let wave = (t * FLICKER_SPEED + f.phase).sin() * 0.6
            + (t * FLICKER_SPEED * 2.3 + f.phase * 0.5).sin() * 0.4;
        let alpha = f.base_alpha * (1.0 - FLICKER_DEPTH * 0.5 + FLICKER_DEPTH * 0.5 * wave);
        sprite.color = sprite.color.with_alpha(alpha);
    }
    for (drift, mut transform) in &mut fog {
        transform.translation.x = drift.base_x + FOG_DRIFT * (t * FOG_SPEED).sin();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layers_are_whole_tiles_covering_the_extent() {
        for (room, view) in [
            (Vec2::new(2560.0, 720.0), Vec2::new(720.0, 405.0)),
            (Vec2::new(1280.0, 2160.0), Vec2::new(720.0, 405.0)),
            (Vec2::new(1280.0, 720.0), Vec2::new(1280.0, 720.0)),
        ] {
            for factor in [0.12, 0.4] {
                let s = layer_size(room, view, factor);
                assert_eq!((s.x % TILE.0, s.y % TILE.1), (0.0, 0.0));
                assert!(s.x >= parallax_extent(room.x, view.x, factor, PARALLAX_MARGIN));
                assert!(s.y >= parallax_extent(room.y, view.y, factor, PARALLAX_MARGIN));
            }
        }
    }
}
