use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::{close_on_esc, PrimaryWindow, WindowResolution},
};
use rand::prelude::*;

// == Settings ==
const INIT_WINDOW_WIDTH: f32 = 1600.0;
const INIT_WINDOW_HEIGHT: f32 = 1200.0;
const WINDOW_CLEAR_COLOUR: Color = Color::rgb(0.1, 0.1, 0.1);

const TOTAL_SPECIES: usize = 10;

const TOTAL_PARTICLES: usize = 1000;
const PARTICLE_RADIUS: f32 = 5.0;
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
        app.insert_resource(ClearColor(WINDOW_CLEAR_COLOUR))
            .init_resource::<AttractionMatrix>()
            .add_startup_system(spawn_camera)
            .add_startup_system(spawn_particles)
            .add_systems((update_velocities, update_positions).chain())
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
                let s = format!("{:.2} ", coefficients[i][j]);
                print!("{:>8}", s);
            }
            println!("");
        }

        Self(coefficients)
    }
}

// == Components ==
#[derive(Component)]
struct Species(u8);

impl Species {
    #[inline]
    #[must_use]
    fn colour(&self) -> Color {
        Color::Hsla {
            hue: self.0 as f32 * 360.0 / TOTAL_SPECIES as f32,
            saturation: 0.8,
            lightness: 0.5,
            alpha: 1.0,
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

    for _ in 0..TOTAL_PARTICLES {
        let x = (random::<f32>() * width * 0.5) + (width * 0.25);
        let y = (random::<f32>() * height * 0.5) + (height * 0.25);

        let species = Species(random::<u8>() % TOTAL_SPECIES as u8);
        let colour = species.colour();

        commands.spawn((
            species,
            Velocity { x: 0.0, y: 0.0 },
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Circle::new(PARTICLE_RADIUS).into())
                    .into(),
                material: materials.add(ColorMaterial::from(colour)),
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                ..default()
            },
        ));
    }
}

fn update_velocities(mut query: Query<(&mut Velocity, &Species, &Transform)>, time: Res<Time>) {}

fn update_positions(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    let dt = time.delta_seconds();
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * dt;
        transform.translation.y += velocity.y * dt;
    }
}
