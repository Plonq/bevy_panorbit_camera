//! Demonstrates all common configuration options

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use std::f32::consts::TAU;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PanOrbitCameraPlugin)
        .add_startup_system(setup)
        .add_system(toggle_camera_controls_system)
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
            // Set focal point
            focus: Vec3::new(0.0, 1.0, 0.0),
            // Set the starting position
            alpha: TAU / 8.0,
            beta: TAU / 8.0,
            radius: 5.0,
            // Set limits on the position
            alpha_upper_limit: Some(TAU / 4.0),
            alpha_lower_limit: Some(-TAU / 4.0),
            beta_upper_limit: Some(TAU / 3.0),
            beta_lower_limit: Some(-TAU / 3.0),
            // Adjust sensitivity of controls
            orbit_sensitivity: 1.5,
            pan_sensitivity: 0.5,
            zoom_sensitivity: 0.5,
            // Allow the camera to go upside down
            allow_upside_down: true,
            // Blender-like key bindings
            button_orbit: MouseButton::Middle,
            button_pan: MouseButton::Middle,
            modifier_pan: Some(KeyCode::LShift),
            // Reverse the zoom direction
            reversed_zoom: true,
            // Makes sure it's enabled (this is default)
            enabled: true,
            ..default()
        },
    ));
}

// Press 'T' to toggle the camera controls
fn toggle_camera_controls_system(
    key_input: Res<Input<KeyCode>>,
    mut pan_orbit_query: Query<&mut PanOrbitCamera>,
) {
    if key_input.just_pressed(KeyCode::T) {
        for mut pan_orbit in pan_orbit_query.iter_mut() {
            pan_orbit.enabled = !pan_orbit.enabled;
        }
    }
}
