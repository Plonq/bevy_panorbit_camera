//! Demonstrates the simplest usage

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Rotation to mimic a rotated world.
    let rotate = Transform::from_rotation(Quat::from_rotation_x(90.0f32.to_radians()));
    // Ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        rotate,
    ));
    // Cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        rotate * Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        rotate * Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // Camera
    // Swaps the axis of the camera to use Z as up instead of Y as up which is the default.
    let swapped_axis = [Vec3::X, Vec3::Z, Vec3::Y];
    let camera = PanOrbitCamera {
        axis: swapped_axis,
        pitch: Some(-45f32.to_radians()),
        ..default()
    };
    commands.spawn((Transform::from_xyz(0.0, 1.5, 5.0), camera));
}
