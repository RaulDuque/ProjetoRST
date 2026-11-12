//! Gothic placeholder palette and text helpers shared by every screen.

use bevy::prelude::*;

pub const PANEL_BG: Color = Color::srgba(0.05, 0.03, 0.06, 0.85);
pub const PANEL_BORDER: Color = Color::srgb(0.45, 0.35, 0.2);
pub const TEXT_MAIN: Color = Color::srgb(0.92, 0.88, 0.8);
pub const TEXT_DIM: Color = Color::srgb(0.6, 0.56, 0.5);
pub const TEXT_GOLD: Color = Color::srgb(0.95, 0.78, 0.35);
pub const BAR_BG: Color = Color::srgb(0.12, 0.08, 0.1);
pub const HP_COLOR: Color = Color::srgb(0.75, 0.12, 0.15);
pub const MP_COLOR: Color = Color::srgb(0.2, 0.35, 0.85);
pub const EXP_COLOR: Color = Color::srgb(0.85, 0.7, 0.25);

pub const FONT_SMALL: f32 = 14.0;
pub const FONT_NORMAL: f32 = 17.0;
pub const FONT_LARGE: f32 = 24.0;

pub fn text(value: impl Into<String>, size: f32, color: Color) -> impl Bundle {
    (
        Text::new(value),
        TextFont {
            font_size: FontSize::Px(size),
            ..default()
        },
        TextColor(color),
    )
}
