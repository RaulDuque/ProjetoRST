//! GPU image of generated pixel art (V2.2). Bevy side of `core::pixel_art`.

use bevy::asset::RenderAssetUsages;
use bevy::image::ImageSampler;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use super::pixel_art::PixelArt;

/// An sRGB image of the art with nearest filtering (no blur when the
/// sprite is rotated or scaled).
pub fn pixel_image(art: PixelArt) -> Image {
    let mut image = Image::new(
        Extent3d {
            width: art.width,
            height: art.height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        art.rgba,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.sampler = ImageSampler::nearest();
    image
}
