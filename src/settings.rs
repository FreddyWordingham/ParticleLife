use bevy::prelude::*;

pub const INIT_WINDOW_WIDTH: f32 = 1125.0 * 0.5;
pub const INIT_WINDOW_HEIGHT: f32 = 2436.0 * 0.5;
pub const WINDOW_CLEAR_COLOUR: Color = Color::rgb(0.1, 0.1, 0.1);

pub const TOTAL_SPECIES: usize = 7;

pub const TOTAL_PARTICLES: usize = 2000;
pub const PARTICLE_RADIUS: f32 = 1.0;
pub const PARTICLE_MASS: f32 = 1.0;

pub const FRICTION_HALF_LIFE: f32 = 0.04;
pub const R_MAX: f32 = 50.0;
pub const BETA: f32 = 0.3;
