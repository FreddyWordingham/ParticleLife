use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::{close_on_esc, PrimaryWindow, WindowResolution},
};
use rand::prelude::*;

// == Settings ==
const INIT_WINDOW_WIDTH: f32 = 1600.0;
const INIT_WINDOW_HEIGHT: f32 = 1200.0;
const PARTICLE_SIZE: f32 = 5.0;
const PARTICLE_SPECIES: u8 = 6;
const PARTICLE_COUNT: usize = 1000;
const PARTICLE_MASS: f32 = 1.0e3;

const FORCE_MATRIX: [[f32; 6]; 6] = [
    [1.0, 1.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 1.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 1.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 1.0, 1.0],
    [1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
];

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
            .add_system(accelerate_particles)
            .add_system(move_particles)
            .add_system(confine_particles)
            .add_system(accelerate_particles)
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
            1 => Color::YELLOW,
            2 => Color::GREEN,
            3 => Color::CYAN,
            4 => Color::BLUE,
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
        let x = (random::<f32>() * width * 0.5) + (width * 0.25);
        let y = (random::<f32>() * height * 0.5) + (height * 0.25);

        // let species = Species((x / (width / PARTICLE_SPECIES as f32)) as u8);
        let species = Species(random::<u8>() % PARTICLE_SPECIES);
        let colour = species.colour();

        commands.spawn((
            species,
            Velocity {
                // x: random::<f32>() * 2.0 - 1.0,
                // y: random::<f32>() * 2.0 - 1.0,
                x: 0.0,
                y: 0.0,
            },
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(PARTICLE_SIZE).into()).into(),
                material: materials.add(ColorMaterial::from(colour)),
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                ..default()
            },
        ));
    }
}

const R3: f32 = 200.0;
const R1: f32 = R3 / 3.0;
const R2: f32 = R1 + R1;

fn accelerate_particles(mut query: Query<(&mut Velocity, &Species, &Transform)>) {
    let mut forces = Vec::with_capacity(PARTICLE_COUNT);
    for (n, (_velocity0, species0, transform0)) in query.iter().enumerate() {
        let mut force = (0.0, 0.0);
        for (m, (_velocity1, species1, transform1)) in query.iter().enumerate() {
            if n == m {
                continue;
            }

            let dx = transform0.translation.x - transform1.translation.x;
            let dy = transform0.translation.y - transform1.translation.y;
            let distance = dx.hypot(dy);

            if distance > R3 {
                continue;
            }

            let angle = dy.atan2(dx);

            if distance < R1 {
                let f = (distance / R1) - 1.0;
                force.0 += f * angle.cos();
                force.1 += f * angle.sin();
            } else if distance < R2 {
                let k = FORCE_MATRIX[species0.0 as usize][species1.0 as usize];
                let f = (((distance - R1) / R1) - 1.0) * k;
                force.0 += f * angle.cos();
                force.1 += f * angle.sin();
            } else {
                let k = FORCE_MATRIX[species0.0 as usize][species1.0 as usize];
                let f = (1.0 - ((distance - R2) / R1)) * k;
                force.0 += f * angle.cos();
                force.1 += f * angle.sin();
            }

            // if distance < R0 {
            //     let force = ((distance / R0) - 1.0) * FORCE_CONSTANT;
            //     f.0 -= force * angle.cos();
            //     f.1 -= force * angle.sin();
            // }

            // if distance < R1 {}

            // let force = k / distance;
            // f.0 += force * angle.cos();
            // f.1 += force * angle.sin();
        }
        forces.push(force);
    }

    for (n, (mut velocity, _species, _transform)) in query.iter_mut().enumerate() {
        let force = forces[n];
        velocity.x -= force.0 / PARTICLE_MASS;
        velocity.y -= force.1 / PARTICLE_MASS;

        velocity.x = velocity.x.clamp(-0.5, 0.5);
        velocity.y = velocity.y.clamp(-0.5, 0.5);
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
