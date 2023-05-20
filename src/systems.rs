use super::*;
use bevy::{math::Vec3Swizzles, prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};
use ndarray::prelude::*;
use rand::prelude::*;

pub fn spawn_camera(mut commands: Commands, query: Query<&Window, With<PrimaryWindow>>) {
    let windows = query.get_single().unwrap();
    let width = windows.width();
    let height = windows.height();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(width * 0.5, height * 0.5, 0.0),
        ..default()
    });
}

pub fn spawn_particles(
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

pub fn update_velocities(
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

pub fn update_velocities_with_grid(
    mut query: Query<(&Transform, &mut Velocity, &Species)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    attraction_matrix: Res<AttractionMatrix>,
) {
    let windows = window_query.get_single().unwrap();
    let width = windows.width();
    let height = windows.height();

    let rx = (width / R_MAX).ceil() as usize;
    let dx = width / rx as f32;
    let ry = (height / R_MAX).ceil() as usize;
    let dy = height / ry as f32;

    let mut grid = Array2::from_elem((rx, ry), Vec::new());
    for (transform, _velocity, species) in query.iter() {
        let x = ((transform.translation.x / dx).floor() as usize).clamp(0, rx - 1);
        let y = ((transform.translation.y / dy).floor() as usize).clamp(0, ry - 1);
        grid[[x, y]].push((transform.translation.xy(), species.0));
    }

    // for yi in 0..ry {
    //     for xi in 0..rx {
    //         print!(" {}", grid[[xi, yi]].len());
    //     }
    //     println!();
    // }
    // println!();

    let mut total_forces = Vec::with_capacity(query.iter().len());
    for (transform, _velocity, species) in query.iter_mut() {
        let mut total_force = Vec2::new(0.0, 0.0);

        let x = ((transform.translation.x / dx).floor() as usize).clamp(0, rx - 1);
        let y = ((transform.translation.y / dy).floor() as usize).clamp(0, ry - 1);

        let x_prev = (x + rx - 1) % (rx - 1);
        let x_next = (x + 1) % (rx - 1);
        let y_prev = (y + ry - 1) % (ry - 1);
        let y_next = (y + 1) % (ry - 1);

        let neighbors = vec![
            (x_prev, y_prev),
            (x, y_prev),
            (x_next, y_prev),
            (x_prev, y),
            (x, y),
            (x_next, y),
            (x_prev, y_next),
            (x, y_next),
            (x_next, y_next),
        ];
        for (sx, sy) in neighbors {
            for (position, other_species) in grid[[sx, sy]].iter() {
                let dx = position.x - transform.translation.x;
                let dy = position.y - transform.translation.y;
                let r = (dx * dx + dy * dy).sqrt();

                if r <= 0.0 || r >= R_MAX {
                    continue;
                }

                let k = attraction_matrix.0[species.0 as usize][*other_species as usize];
                let f = force(r / R_MAX, k);
                total_force.x += f * dx / r;
                total_force.y += f * dy / r;
            }
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

pub fn force(r: f32, k: f32) -> f32 {
    if r < BETA {
        return r / BETA - 1.0;
    }
    return k * (1.0 - (2.0 * r - 1.0 - BETA).abs() / (1.0 - BETA));
}

pub fn update_positions(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    let dt = time.delta_seconds();
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.0.x * dt;
        transform.translation.y += velocity.0.y * dt;
    }
}

pub fn confine_particles(
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

pub fn restrain_particles(
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
            velocity.0.x -= 1000.0 * dt;
        } else if transform.translation.x > (width - d) {
            velocity.0.x += 1000.0 * dt;
        }

        if transform.translation.y < d {
            velocity.0.y -= 1000.0 * dt;
        } else if transform.translation.y > (height - d) {
            velocity.0.y += 1000.0 * dt;
        }
    }
}
