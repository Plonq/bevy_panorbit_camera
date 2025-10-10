//! Demonstrates how to pause time without affecting the camera

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (pause_game_system, cube_rotator_system))
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
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));
    // Cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Cube,
    ));
    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // Camera
    commands.spawn((
        Transform::from_xyz(0.0, 1.5, 5.0),
        PanOrbitCamera {
            use_real_time: true,
            ..default()
        },
    ));
    // Help text
    commands.spawn(Text::new(
        "\
Press Space to pause the 'game'",
    ));
}

// Pauses the game (i.e. virtual time)
fn pause_game_system(key_input: Res<ButtonInput<KeyCode>>, mut time: ResMut<Time<Virtual>>) {
    if key_input.just_pressed(KeyCode::Space) {
        if time.is_paused() {
            time.unpause()
        } else {
            time.pause()
        }
    }
}

// Rotates the cube so you can see the effect of pausing time
// Note the default time for the Update schedule is `Time<Virtual>`
fn cube_rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Cube>>) {
    for mut transform in &mut query {
        transform.rotate_y(1.0 * time.delta_secs());
    }
}
