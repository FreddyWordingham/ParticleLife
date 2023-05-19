use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::{close_on_esc, PrimaryWindow, WindowResolution},
};
use rand::prelude::*;

// == Settings ==
const INIT_WINDOW_WIDTH: f32 = 1600.0;
const INIT_WINDOW_HEIGHT: f32 = 1200.0;

// == Main ==
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(INIT_WINDOW_WIDTH, INIT_WINDOW_HEIGHT),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(SimulationPlugin)
        .run();
}

// == Plugins ==
struct SimulationPlugin;
impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_system(close_on_esc);
    }
}

// == Components ==

// == Systems ==
fn spawn_camera(mut commands: Commands, query: Query<&Window, With<PrimaryWindow>>) {
    let windows = query.get_single().unwrap();
    let width = windows.width();
    let height = windows.height();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(width * 0.5, height * 0.5, 0.0),
        ..default()
    });
}
