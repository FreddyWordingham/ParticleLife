use super::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Species(pub u8);

impl Species {
    #[inline]
    #[must_use]
    pub fn colour(&self) -> Color {
        Color::Hsla {
            hue: self.0 as f32 * 360.0 / TOTAL_SPECIES as f32,
            saturation: 0.8,
            lightness: 0.5,
            alpha: 1.0,
        }
    }
}
