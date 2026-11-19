//! Bevy side of the data-driven sprite composition (N4-A).
//!
//! A composite character is an entity with a [`VisualPlayback`] (the one
//! animation state: clip, frame, facing) and a [`CompositeCharacter`] (the
//! composition and the outfit). It has one child sprite per layer slot
//! ([`CompositeLayer`]), not one per anatomical detail. Every frame,
//! [`sync_composite_layers`] resolves the outfit with
//! [`composition::resolve`] and points every layer at the **same** frame,
//! facing and pivot. There is no second clock.
//!
//! Nothing in the shipped game spawns a composite character yet: the N4-A
//! proof is `cargo run --example n4_sprite_machine`.

use std::collections::HashMap;
use std::sync::Arc;

use bevy::prelude::*;
use bevy::sprite::Anchor;

use super::VisualPlayback;
use super::composition::{CharacterComposition, Outfit, resolve_facing};
use super::manifest::{ROOT_CLIP, anchor_for};

/// Depth between two layer slots (all inside the character's own depth).
pub const LAYER_Z_STEP: f32 = 0.001;

/// Image and grid of one sheet of one clip.
#[derive(Debug, Clone)]
pub struct CompositeSheet {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

/// The composition, the outfit and the loaded sheets of a character.
#[derive(Component, Debug, Clone)]
pub struct CompositeCharacter {
    pub composition: Arc<CharacterComposition>,
    pub outfit: Outfit,
    /// `(clip, texture)` -> sheet handles.
    pub sheets: HashMap<(String, String), CompositeSheet>,
}

/// A layer slot child of a composite character.
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct CompositeLayer(pub String);

impl CompositeCharacter {
    /// Creates the image and layout handles of every sheet of every part
    /// (textures are asset paths, prefixed with `texture_prefix`).
    pub fn load(
        composition: Arc<CharacterComposition>,
        outfit: Outfit,
        server: &AssetServer,
        layouts: &mut Assets<TextureAtlasLayout>,
        texture_prefix: &str,
    ) -> Self {
        let mut sheets = HashMap::new();
        for part in composition.parts.values() {
            let pieces = part.pieces.iter().chain(part.variants.values().flatten());
            for piece in pieces {
                for (clip_id, sheet) in &piece.sheets {
                    let Some(clip) = composition.clips.get(clip_id) else {
                        continue;
                    };
                    let key = (clip_id.clone(), sheet.texture.clone());
                    if sheets.contains_key(&key) {
                        continue;
                    }
                    let layout = TextureAtlasLayout::from_grid(
                        UVec2::new(clip.cell.0, clip.cell.1),
                        sheet.columns,
                        sheet.rows,
                        None,
                        None,
                    );
                    sheets.insert(
                        key,
                        CompositeSheet {
                            image: server.load(format!("{texture_prefix}{}", sheet.texture)),
                            layout: layouts.add(layout),
                        },
                    );
                }
            }
        }
        Self {
            composition,
            outfit,
            sheets,
        }
    }

    /// One child sprite per layer slot, hidden until the first sync.
    pub fn spawn_layers(&self, commands: &mut Commands, root: Entity) {
        for layer in &self.composition.layers {
            commands.entity(root).with_child((
                CompositeLayer(layer.clone()),
                Sprite::default(),
                Anchor::default(),
                Transform::default(),
                Visibility::Hidden,
            ));
        }
    }
}

/// Points every layer at the shared clip, frame and facing.
pub fn sync_composite_layers(
    roots: Query<(&VisualPlayback, &CompositeCharacter, &Children)>,
    mut layers: Query<(
        &CompositeLayer,
        &mut Sprite,
        &mut Anchor,
        &mut Transform,
        &mut Visibility,
    )>,
) {
    for (playback, character, children) in &roots {
        let c = &character.composition;
        let clip_id = playback.clip.as_deref().unwrap_or(ROOT_CLIP);
        let frame = u32::try_from(playback.frame).unwrap_or(u32::MAX);
        let resolved = resolve_facing(c, &character.outfit, clip_id, frame, playback.flip);
        let clip = c.clips.get(clip_id);
        for child in children.iter() {
            let Ok((slot, mut sprite, mut anchor, mut transform, mut visibility)) =
                layers.get_mut(child)
            else {
                continue;
            };
            // The last visible piece on this slot wins (one piece per slot).
            let draw = resolved
                .as_ref()
                .ok()
                .and_then(|r| r.iter().rev().find(|d| d.layer == slot.0 && d.visible));
            let (Some(draw), Some(clip)) = (draw, clip) else {
                visibility.set_if_neq(Visibility::Hidden);
                continue;
            };
            let Some(sheet) = character
                .sheets
                .get(&(clip_id.to_owned(), draw.texture.clone()))
            else {
                visibility.set_if_neq(Visibility::Hidden);
                continue;
            };
            if sprite.image != sheet.image {
                sprite.image = sheet.image.clone();
            }
            let atlas = TextureAtlas {
                layout: sheet.layout.clone(),
                index: draw.index as usize,
            };
            if sprite.texture_atlas.as_ref() != Some(&atlas) {
                sprite.texture_atlas = Some(atlas);
            }
            let density = clip.density as f32;
            let size = Vec2::new(clip.cell.0 as f32 / density, clip.cell.1 as f32 / density);
            if sprite.custom_size != Some(size) {
                sprite.custom_size = Some(size);
            }
            // A hand-drawn mirrored sheet is already facing left.
            let flip_x = playback.flip && !draw.mirrored;
            if sprite.flip_x != flip_x {
                sprite.flip_x = flip_x;
            }
            let (ax, ay) = anchor_for(clip.pivot, clip.cell, playback.flip);
            anchor.set_if_neq(Anchor(Vec2::new(ax, ay)));
            let z = draw.z as f32 * LAYER_Z_STEP;
            if transform.translation.z != z {
                transform.translation.z = z;
            }
            visibility.set_if_neq(Visibility::Inherited);
        }
    }
}
