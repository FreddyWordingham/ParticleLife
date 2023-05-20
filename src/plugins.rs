use super::*;
use bevy::{prelude::*, window::close_on_esc};

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(WINDOW_CLEAR_COLOUR))
            .init_resource::<AttractionMatrix>()
            .add_startup_system(spawn_camera)
            .add_startup_system(spawn_particles)
            .add_systems((update_velocities, update_positions).chain())
            .add_system(confine_particles)
            .add_system(restrain_particles)
            .add_system(close_on_esc);
    }
}
