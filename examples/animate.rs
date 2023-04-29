//! Demonstrates how you can animate the movement of the camera

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use std::f32::consts::TAU;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PanOrbitCameraPlugin)
        .add_startup_system(setup)
        .add_system(animate)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // Cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // Camera
    commands.spawn((
        Camera3dBundle::default(),
        PanOrbitCamera {
            // Disable smoothing, since the animation takes care of that
            orbit_smoothness: 0.0,
            // Might want to disable the controls
            enabled: false,
            ..default()
        },
    ));
}

// Animate the camera's position
fn animate(time: Res<Time>, mut pan_orbit_query: Query<&mut PanOrbitCamera>) {
    for mut pan_orbit in pan_orbit_query.iter_mut() {
        // Must set target values, not alpha/beta directly
        pan_orbit.target_alpha += 15f32.to_radians() * time.delta_seconds();
        pan_orbit.target_beta = time.elapsed_seconds_wrapped().sin() * TAU * 0.1;
        pan_orbit.radius = (((time.elapsed_seconds_wrapped() * 2.0).cos() + 1.0) * 0.5) * 2.0 + 4.0;

        // Force camera to update its transform
        pan_orbit.force_update = true;
    }
}
