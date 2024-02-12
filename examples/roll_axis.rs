//! Demonstrates how to 'roll' the camera, thus control the 3rd axis of rotation

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use std::f32::consts::TAU;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, roll_controls)
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
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
            ..default()
        },
        PanOrbitCamera {
            // Changing the up vector (which rolling does) changes what 'up' means, so you likely
            // want to allow upside down when rolling.
            allow_upside_down: true,
            ..default()
        },
    ));
}

/// Use left/right arrow keys to roll the camera
fn roll_controls(
    mut pan_orbit_q: Query<(&mut PanOrbitCamera, &Transform)>,
    time: Res<Time>,
    key_input: Res<Input<KeyCode>>,
) {
    if let Ok((mut pan_orbit, transform)) = pan_orbit_q.get_single_mut() {
        let mut roll_angle = 0.0;
        let roll_amount = TAU / 3.0 * time.delta_seconds();
        if key_input.pressed(KeyCode::Left) {
            roll_angle -= roll_amount;
        }
        if key_input.pressed(KeyCode::Right) {
            roll_angle += roll_amount;
        }
        // Rotate the base transform by the roll amount around its current 'look' axis
        pan_orbit
            .base_transform
            .rotate_axis(transform.local_z(), roll_angle);
        // Whenever controlling the camera manually you must make it force update every frame
        pan_orbit.force_update = true;
    }
}
