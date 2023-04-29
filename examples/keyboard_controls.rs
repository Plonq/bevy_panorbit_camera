//! Demonstrates how to control the camera using the keyboard

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PanOrbitCameraPlugin)
        .add_startup_system(setup)
        .add_system(keyboard_controls)
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
    commands.spawn((Camera3dBundle::default(), PanOrbitCamera::default()));
}

fn keyboard_controls(
    time: Res<Time>,
    key_input: Res<Input<KeyCode>>,
    mut pan_orbit_query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    for (mut pan_orbit, mut transform) in pan_orbit_query.iter_mut() {
        // Jump by 45 degrees using Left Ctrl + Arrows
        if key_input.pressed(KeyCode::LControl) {
            if key_input.just_pressed(KeyCode::Right) {
                pan_orbit.target_alpha += 45f32.to_radians();
            }
            if key_input.just_pressed(KeyCode::Left) {
                pan_orbit.target_alpha -= 45f32.to_radians();
            }
            if key_input.just_pressed(KeyCode::Up) {
                pan_orbit.target_beta += 45f32.to_radians();
            }
            if key_input.just_pressed(KeyCode::Down) {
                pan_orbit.target_beta -= 45f32.to_radians();
            }
        }
        // Pan using Left Shift + Arrows
        else if key_input.pressed(KeyCode::LShift) {
            let mut delta_translation = Vec3::ZERO;
            if key_input.pressed(KeyCode::Right) {
                delta_translation += transform.rotation * Vec3::X * time.delta_seconds();
            }
            if key_input.pressed(KeyCode::Left) {
                delta_translation += transform.rotation * Vec3::NEG_X * time.delta_seconds();
            }
            if key_input.pressed(KeyCode::Up) {
                delta_translation += transform.rotation * Vec3::Y * time.delta_seconds();
            }
            if key_input.pressed(KeyCode::Down) {
                delta_translation += transform.rotation * Vec3::NEG_Y * time.delta_seconds();
            }
            transform.translation += delta_translation;
            pan_orbit.focus += delta_translation;
        }
        // Smooth rotation using arrow keys without modifier
        else {
            if key_input.pressed(KeyCode::Right) {
                pan_orbit.target_alpha += 50f32.to_radians() * time.delta_seconds();
            }
            if key_input.pressed(KeyCode::Left) {
                pan_orbit.target_alpha -= 50f32.to_radians() * time.delta_seconds();
            }
            if key_input.pressed(KeyCode::Up) {
                pan_orbit.target_beta += 50f32.to_radians() * time.delta_seconds();
            }
            if key_input.pressed(KeyCode::Down) {
                pan_orbit.target_beta -= 50f32.to_radians() * time.delta_seconds();
            }

            // Zoom with Z and X
            if key_input.pressed(KeyCode::Z) {
                pan_orbit.radius -= 5.0 * time.delta_seconds();
            }
            if key_input.pressed(KeyCode::X) {
                pan_orbit.radius += 5.0 * time.delta_seconds();
            }
        }

        // Force camera to update its transform
        pan_orbit.force_update = true;
    }
}
