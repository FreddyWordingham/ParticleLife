use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::{close_on_esc, PrimaryWindow, WindowResolution},
};
use rand::prelude::*;

// == Settings ==
const INIT_WINDOW_WIDTH: f32 = 800.0;
const INIT_WINDOW_HEIGHT: f32 = 600.0;
const PARTICLE_SIZE: f32 = 5.0;
const PARTICLE_SPECIES: u8 = 6;
const PARTICLE_COUNT: usize = 100;

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
            .add_startup_system(spawn_particles)
            .add_system(move_particles)
            .add_system(confine_particles)
            .add_system(close_on_esc);
    }
}

// == Components ==
#[derive(Component)]
struct Species(u8);

impl Species {
    fn colour(&self) -> Color {
        match self.0 {
            0 => Color::RED,
            1 => Color::GREEN,
            2 => Color::BLUE,
            3 => Color::YELLOW,
            4 => Color::CYAN,
            5 => Color::PURPLE,
            _ => Color::WHITE,
        }
    }
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

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

fn spawn_particles(
    mut commands: Commands,
    query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let windows = query.get_single().unwrap();
    let width = windows.width();
    let height = windows.height();

    for _ in 0..PARTICLE_COUNT {
        let species = Species(random::<u8>() % PARTICLE_SPECIES);
        let colour = species.colour();

        commands.spawn((
            species,
            Velocity {
                x: random::<f32>() * 2.0 - 1.0,
                y: random::<f32>() * 2.0 - 1.0,
            },
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(PARTICLE_SIZE).into()).into(),
                material: materials.add(ColorMaterial::from(colour)),
                transform: Transform::from_translation(Vec3::new(
                    random::<f32>() * width,
                    random::<f32>() * height,
                    0.0,
                )),
                ..default()
            },
        ));
    }
}

fn move_particles(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}

fn confine_particles(
    mut query: Query<&mut Transform>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let width = window.width();
    let height = window.height();

    for mut transform in query.iter_mut() {
        if transform.translation.x < 0.0 {
            transform.translation.x = width;
        } else if transform.translation.x > width {
            transform.translation.x = 0.0;
        }

        if transform.translation.y < 0.0 {
            transform.translation.y = height;
        } else if transform.translation.y > height {
            transform.translation.y = 0.0;
        }
    }
}
