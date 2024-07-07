//! Demonstrates how to control the camera using the keyboard
//! Controls:
//!     Orbit/rotate smoothly: Arrows
//!     Orbit/rotate in 45deg increments: Ctrl+Arrows
//!     Pan smoothly: Shift+Arrows
//!     Pan in 1m increments: Ctrl+Shift+Arrows
//!     Zoom in/out: Z/X

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_controls)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
        material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
        ..default()
    });
    // Cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}

fn keyboard_controls(
    time: Res<Time>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut pan_orbit_query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    for (mut pan_orbit, mut transform) in pan_orbit_query.iter_mut() {
        if key_input.pressed(KeyCode::ControlLeft) {
            // Jump focus point 1m using Ctrl+Shift + Arrows
            if key_input.pressed(KeyCode::ShiftLeft) {
                if key_input.just_pressed(KeyCode::ArrowRight) {
                    pan_orbit.target_focus += Vec3::X;
                }
                if key_input.just_pressed(KeyCode::ArrowLeft) {
                    pan_orbit.target_focus -= Vec3::X;
                }
                if key_input.just_pressed(KeyCode::ArrowUp) {
                    pan_orbit.target_focus += Vec3::Y;
                }
                if key_input.just_pressed(KeyCode::ArrowDown) {
                    pan_orbit.target_focus -= Vec3::Y;
                }
            } else {
                // Jump by 45 degrees using Left Ctrl + Arrows
                if key_input.just_pressed(KeyCode::ArrowRight) {
                    pan_orbit.target_yaw += 45f32.to_radians();
                }
                if key_input.just_pressed(KeyCode::ArrowLeft) {
                    pan_orbit.target_yaw -= 45f32.to_radians();
                }
                if key_input.just_pressed(KeyCode::ArrowUp) {
                    pan_orbit.target_pitch += 45f32.to_radians();
                }
                if key_input.just_pressed(KeyCode::ArrowDown) {
                    pan_orbit.target_pitch -= 45f32.to_radians();
                }
            }
        }
        // Pan using Left Shift + Arrows
        else if key_input.pressed(KeyCode::ShiftLeft) {
            let mut delta_translation = Vec3::ZERO;
            if key_input.pressed(KeyCode::ArrowRight) {
                delta_translation += transform.rotation * Vec3::X * time.delta_seconds();
            }
            if key_input.pressed(KeyCode::ArrowLeft) {
                delta_translation += transform.rotation * Vec3::NEG_X * time.delta_seconds();
            }
            if key_input.pressed(KeyCode::ArrowUp) {
                delta_translation += transform.rotation * Vec3::Y * time.delta_seconds();
            }
            if key_input.pressed(KeyCode::ArrowDown) {
                delta_translation += transform.rotation * Vec3::NEG_Y * time.delta_seconds();
            }
            transform.translation += delta_translation;
            pan_orbit.target_focus += delta_translation;
        }
        // Smooth rotation using arrow keys without modifier
        else {
            if key_input.pressed(KeyCode::ArrowRight) {
                pan_orbit.target_yaw += 50f32.to_radians() * time.delta_seconds();
            }
            if key_input.pressed(KeyCode::ArrowLeft) {
                pan_orbit.target_yaw -= 50f32.to_radians() * time.delta_seconds();
            }
            if key_input.pressed(KeyCode::ArrowUp) {
                pan_orbit.target_pitch += 50f32.to_radians() * time.delta_seconds();
            }
            if key_input.pressed(KeyCode::ArrowDown) {
                pan_orbit.target_pitch -= 50f32.to_radians() * time.delta_seconds();
            }

            // Zoom with Z and X
            if key_input.pressed(KeyCode::KeyZ) {
                pan_orbit.target_radius -= 5.0 * time.delta_seconds();
            }
            if key_input.pressed(KeyCode::KeyX) {
                pan_orbit.target_radius += 5.0 * time.delta_seconds();
            }
        }

        // Force camera to update its transform
        pan_orbit.force_update = true;
    }
}
