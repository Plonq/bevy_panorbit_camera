//! Demonstrates how to change the 'up' vector of the camera

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
    // Ground
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..default()
    });
    // Cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
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
        PanOrbitCamera {
            base_transform: Transform::from_rotation(Quat::from_rotation_arc(
                Vec3::NEG_Y,
                // This is the new 'up' vector, and it must be normalised before passing to
                // `from_rotation_arc`. Alternatively you can set `base_transform` any way you want,
                // e.g. from a rotation or an existing transform.
                Vec3::new(0.2, -1.2, 0.2).normalize(),
            )),
            // When changing the 'up' vector, you probably also want to allow upside down, because
            // 'upside down' is based upon the world 'up' vector (via alpha/beta values), and so
            // doesn't make any sense when the up vector is some arbitrary direction.
            allow_upside_down: true,
            ..default()
        },
    ));
}
