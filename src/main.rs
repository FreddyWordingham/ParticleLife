use bevy::{prelude::*, window::WindowResolution};
use particle_life::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Freddy's Particle Life".to_string(),
                resolution: WindowResolution::new(INIT_WINDOW_WIDTH, INIT_WINDOW_HEIGHT),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(SimulationPlugin)
        .run();
}
