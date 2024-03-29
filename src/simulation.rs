use super::*;
use bevy::{prelude::*, window::close_on_esc};

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(WINDOW_CLEAR_COLOUR))
            .insert_resource(AttractionMatrix::random())
            .insert_resource(RateOfChange(DEFAULT_RATE_OF_CHANGE))
            .add_startup_system(spawn_camera_with_bloom)
            .add_startup_system(spawn_particles)
            .add_startup_system(spawn_particles_circle)
            // .add_systems((update_velocities, update_positions).chain())
            .add_systems((update_velocities_with_grid, update_positions).chain())
            // .add_systems((update_velocities_with_rasterisation, update_positions).chain())
            .add_system(confine_particles_by_wrap)
            .add_system(close_on_esc)
            .add_system(set_rate_of_change);
    }
}
