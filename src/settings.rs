use bevy::prelude::*;

pub const INIT_WINDOW_WIDTH: f32 = 1000.0;
pub const INIT_WINDOW_HEIGHT: f32 = 1000.0;
pub const WINDOW_CLEAR_COLOUR: Color = Color::rgb(0.1, 0.1, 0.1);

pub const TOTAL_SPECIES: usize = 7;

pub const TOTAL_PARTICLES: usize = 100;
pub const PARTICLE_RADIUS: f32 = 2.0;
pub const PARTICLE_MASS: f32 = 1.0;

pub const FRICTION_HALF_LIFE: f32 = 0.04;
pub const R_MAX: f32 = 100.0;
pub const BETA: f32 = 0.3;
