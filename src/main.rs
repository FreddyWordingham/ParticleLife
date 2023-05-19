use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::{close_on_esc, PrimaryWindow, WindowResolution},
};
use rand::prelude::*;

// == Settings ==
const INIT_WINDOW_WIDTH: f32 = 1600.0;
const INIT_WINDOW_HEIGHT: f32 = 1200.0;

const TOTAL_SPECIES: usize = 6;

// const TOTAL_PARTICLES: usize = 1000;
// const PARTICLE_SIZE: f32 = 5.0;
// const PARTICLE_MASS: f32 = 6.0;

// const FRICTION_HALF_LIFE: f32 = 0.04;
// // const friction_coefficient: f32 = 2.0f32.powf(-delta_time / FRICTION_HALF_LIFE);
// const R_MAX: f32 = 100.0;

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
        app.init_resource::<AttractionMatrix>()
            .add_startup_system(spawn_camera)
            .add_system(close_on_esc);
    }
}

// == Resources ==
#[derive(Resource)]
struct AttractionMatrix([[f32; TOTAL_SPECIES]; TOTAL_SPECIES]);
impl Default for AttractionMatrix {
    fn default() -> Self {
        let mut coefficients = [[0.0; TOTAL_SPECIES]; TOTAL_SPECIES];
        for i in 0..TOTAL_SPECIES {
            for j in 0..TOTAL_SPECIES {
                coefficients[i][j] = (random::<f32>() * 2.0) - 1.0;
                print!("{:.2} ", coefficients[i][j]);
            }
            println!("");
        }

        Self(coefficients)
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
