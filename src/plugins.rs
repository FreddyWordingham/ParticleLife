use super::*;
use bevy::{prelude::*, window::close_on_esc};

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(WINDOW_CLEAR_COLOUR))
            .insert_resource(AttractionMatrix::random())
            .add_startup_system(spawn_camera)
            .add_startup_system(spawn_particles)
            .add_startup_system(spawn_particles_circle)
            .add_systems((update_velocities, update_positions).chain())
            // .add_systems((update_velocities_with_grid, update_positions).chain())
            .add_system(confine_particles_by_wrap)
            .add_system(close_on_esc);
    }
}
