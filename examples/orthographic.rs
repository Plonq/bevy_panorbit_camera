//! Demonstrates usage with an orthographic camera

use bevy::render::camera::ScalingMode;
use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, switch_projection)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // help
    commands.spawn(TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: "Press R to switch projection".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        },
        ..default()
    });
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
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 6.0)),
            projection: Projection::Orthographic(OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical(1.0),
                ..default()
            }),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}

fn switch_projection(
    mut next_projection: Local<Projection>,
    mut keyboard_events: EventReader<KeyboardInput>,
    mut camera_query: Query<(&mut PanOrbitCamera, &mut Projection)>,
) {
    for event in keyboard_events.read() {
        if event.key_code == KeyCode::KeyR && event.state == ButtonState::Pressed {
            let Ok((mut camera, mut projection)) = camera_query.get_single_mut() else {
                return;
            };
            std::mem::swap(&mut *next_projection, &mut *projection);
            camera.force_update = true;
        }
    }
}
