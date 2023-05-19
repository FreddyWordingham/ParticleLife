use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::{close_on_esc, PrimaryWindow, WindowResolution},
};
use rand::prelude::*;

// == Settings ==
const INIT_WINDOW_WIDTH: f32 = 1400.0;
const INIT_WINDOW_HEIGHT: f32 = 1400.0;
const WINDOW_CLEAR_COLOUR: Color = Color::rgb(0.1, 0.1, 0.1);

const TOTAL_SPECIES: usize = 11;

const TOTAL_PARTICLES: usize = 3000;
const PARTICLE_RADIUS: f32 = 2.0;
const PARTICLE_MASS: f32 = 1.0;

const FRICTION_HALF_LIFE: f32 = 0.04;
const R_MAX: f32 = 100.0;
const BETA: f32 = 0.3;

// == Main ==
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

// == Plugins ==
struct SimulationPlugin;
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

// == Resources ==
#[derive(Resource)]
struct AttractionMatrix([[f32; TOTAL_SPECIES]; TOTAL_SPECIES]);
impl Default for AttractionMatrix {
    fn default() -> Self {
        let mut rng = rand::rngs::SmallRng::from_seed(generate_rng_seed());

        let mut coefficients = [[0.0; TOTAL_SPECIES]; TOTAL_SPECIES];
        for i in 0..TOTAL_SPECIES {
            for j in 0..TOTAL_SPECIES {
                // coefficients[i][j] = rng.gen_range(-1.0..1.0);

                if i == j {
                    coefficients[i][j] = 1.0;
                } else if j == (i + 1) {
                    coefficients[i][j] = 0.4;
                } else {
                    coefficients[i][j] = 0.0;
                }
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
struct Velocity(Vec2);

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

    let f = 0.5;
    for i in 0..TOTAL_PARTICLES {
        let x = (random::<f32>() + 0.5) * (width * f);
        let y = (random::<f32>() + 0.5) * (height * f);

        let species = Species(random::<u8>() % TOTAL_SPECIES as u8);
        let colour = species.colour();

        commands.spawn((
            species,
            Velocity(Vec2::new(0.0, 0.0)),
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Circle::new(PARTICLE_RADIUS).into())
                    .into(),
                material: materials.add(ColorMaterial::from(colour)),
                transform: Transform::from_translation(Vec3::new(x, y, -(i as f32))),
                ..default()
            },
        ));
    }
}

fn update_velocities(
    mut query: Query<(&Transform, &mut Velocity, &Species)>,
    time: Res<Time>,
    attraction_matrix: Res<AttractionMatrix>,
) {
    let mut total_forces = Vec::with_capacity(query.iter().len());
    for (i, (transform_i, _velocity_i, species_i)) in query.iter().enumerate() {
        let mut total_force = Vec2::new(0.0, 0.0);

        for (j, (transform_j, _velocity_j, species_j)) in query.iter().enumerate() {
            if i == j {
                continue;
            }

            let dx = transform_j.translation.x - transform_i.translation.x;
            let dy = transform_j.translation.y - transform_i.translation.y;
            let r = (dx * dx + dy * dy).sqrt();

            if r <= 0.0 || r >= R_MAX {
                continue;
            }

            let k = attraction_matrix.0[species_i.0 as usize][species_j.0 as usize];
            let f = force(r / R_MAX, k);
            total_force.x += f * dx / r;
            total_force.y += f * dy / r;
        }

        total_force.x *= R_MAX;
        total_force.y *= R_MAX;
        total_forces.push(total_force);
    }

    let dt = time.delta_seconds();
    let friction_coefficient: f32 = 2.0f32.powf(-dt / FRICTION_HALF_LIFE);
    for ((_transform, mut velocity, _species), total_force) in query.iter_mut().zip(total_forces) {
        velocity.0.x *= friction_coefficient;
        velocity.0.y *= friction_coefficient;

        velocity.0.x += total_force.x * dt / PARTICLE_MASS;
        velocity.0.y += total_force.y * dt / PARTICLE_MASS;
    }
}

fn force(r: f32, k: f32) -> f32 {
    if r < BETA {
        return r / BETA - 1.0;
    }
    return k * (1.0 - (2.0 * r - 1.0 - BETA).abs() / (1.0 - BETA));
}

fn update_positions(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    let dt = time.delta_seconds();
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.0.x * dt;
        transform.translation.y += velocity.0.y * dt;
    }
}

fn confine_particles(
    mut transform_query: Query<&mut Transform>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let windows = window_query.get_single().unwrap();
    let width = windows.width();
    let height = windows.height();

    let f = 0.125;
    for mut transform in transform_query.iter_mut() {
        if transform.translation.x < 0.0 {
            transform.translation.x = ((random::<f32>() * 2.0 * f) + (0.5 - f)) * width;
            transform.translation.y = ((random::<f32>() * 2.0 * f) + (0.5 - f)) * height;
        } else if transform.translation.x > width {
            transform.translation.x = ((random::<f32>() * 2.0 * f) + (0.5 - f)) * width;
            transform.translation.y = ((random::<f32>() * 2.0 * f) + (0.5 - f)) * height;
        }

        if transform.translation.y < 0.0 {
            transform.translation.x = ((random::<f32>() * 2.0 * f) + (0.5 - f)) * width;
            transform.translation.y = ((random::<f32>() * 2.0 * f) + (0.5 - f)) * height;
        } else if transform.translation.y > height {
            transform.translation.x = ((random::<f32>() * 2.0 * f) + (0.5 - f)) * width;
            transform.translation.y = ((random::<f32>() * 2.0 * f) + (0.5 - f)) * height;
        }
    }
}

fn restrain_particles(
    mut velocity_transform_query: Query<(&mut Velocity, &Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let windows = window_query.get_single().unwrap();
    let width = windows.width();
    let height = windows.height();

    let dt = time.delta_seconds();
    let d = 250.0;
    for (mut velocity, transform) in velocity_transform_query.iter_mut() {
        if transform.translation.x < d {
            velocity.0 -= 100.0 * dt;
        } else if transform.translation.x > (width - d) {
            velocity.0 += 100.0 * dt;
        }

        if transform.translation.y < d {
            velocity.0 -= 100.0 * dt;
        } else if transform.translation.y > (height - d) {
            velocity.0 += 100.0 * dt;
        }
    }
}

fn generate_rng_seed() -> [u8; 32] {
    let args = std::env::args().collect::<Vec<_>>();

    format!("{:_>32}", args[1])[0..32]
        .chars()
        .map(|c| c as u8)
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap_or_else(|v: Vec<u8>| {
            panic!("Expected a Vec of length {} but it was {}", 32, v.len())
        })
}
