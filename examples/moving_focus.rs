//! Demonstrates orbiting around a moving focal point

use std::f32::consts::TAU;

use bevy::prelude::*;

use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin, PanOrbitCameraSystemSet};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (move_cube_system, update_focus_system)
                .chain()
                .before(PanOrbitCameraSystemSet),
        )
        .run();
}

#[derive(Component)]
struct Cube;

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
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Cube,
    ));
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
            transform: Transform::from_translation(Vec3::new(0.0, 4.5, 7.0)),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}

fn move_cube_system(time: Res<Time>, mut query: Query<&mut Transform, With<Cube>>) {
    for mut tfm in query.iter_mut() {
        tfm.translation = Vec3::new(
            time.elapsed_seconds_wrapped().sin() * TAU * 0.5,
            0.5,
            time.elapsed_seconds_wrapped().cos() * TAU * 0.5,
        );
    }
}

fn update_focus_system(
    mut cam_query: Query<&mut PanOrbitCamera, Without<Cube>>,
    cube_query: Query<&Transform, With<Cube>>,
) {
    for mut orbit_cam in cam_query.iter_mut() {
        let cube_location = cube_query.get_single().unwrap().translation;
        orbit_cam.focus = cube_location;
        orbit_cam.target_focus = cube_location;
        orbit_cam.force_update = true;
    }
}
