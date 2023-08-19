//! Demonstrates the ability to manually override which instance of PanOrbitCamera receives input,
//! and how that input is scaled.
//! This effectively disables automatic handling of multiple viewport/windows, but may be suitable
//! if you have a unique situation, for example when the camera you want to control is rendering to
//! a texture instead of a window.

use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy::window::{PrimaryWindow, WindowResized};
use bevy_panorbit_camera::{ActiveCameraData, PanOrbitCamera, PanOrbitCameraPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_active_cam, set_camera_viewports))
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
    // Main Camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.5, 5.0)),
        ..default()
    },));
    // Minimap Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(1.0, 1.5, 4.0)),
            camera: Camera {
                // Renders the minimap camera after the main camera, so it is rendered on top
                order: 1,
                ..default()
            },
            camera_3d: Camera3d {
                // Don't clear on the second camera because the first camera already cleared the window
                clear_color: ClearColorConfig::None,
                ..default()
            },
            ..default()
        },
        PanOrbitCamera::default(),
        MinimapCamera,
    ));
}

#[derive(Component)]
struct MinimapCamera;

fn update_active_cam(
    mut active_cam: ResMut<ActiveCameraData>,
    minimap_camera_q: Query<(Entity, &Camera), With<MinimapCamera>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let primary_window = windows
        .get_single()
        .expect("There is only ever one primary camera");
    let (minimap_camera_id, minimap_camera) =
        minimap_camera_q.get_single().expect("We only added one");
    active_cam.set_if_neq(ActiveCameraData {
        // Set the entity to the entity ID of the camera you want to control
        entity: Some(minimap_camera_id),
        // Set the viewport and window size based on how you want the mouse motion to be scaled.
        // The viewport size is used to scale pan motion in order to get 1:1 motion (at default
        // settings), and window size is used for scaling rotation (because it feels better than
        // using the viewport size). Here, we set the values to the actual values of the minimap
        // camera, which is what PanOrbitCameraPlugin would do.
        viewport_size: minimap_camera.logical_viewport_size(),
        window_size: Some(Vec2::new(primary_window.width(), primary_window.height())),
        // Setting manual to true ensures PanOrbitCameraPlugin will not overwrite this resource
        manual: true,
    });
}

fn set_camera_viewports(
    windows: Query<&Window>,
    mut resize_events: EventReader<WindowResized>,
    mut right_camera: Query<&mut Camera, With<MinimapCamera>>,
) {
    for resize_event in resize_events.iter() {
        let window = windows.get(resize_event.window).unwrap();
        let mut right_camera = right_camera.single_mut();
        let size = window.resolution.physical_width() / 5;
        right_camera.viewport = Some(Viewport {
            physical_position: UVec2::new(window.resolution.physical_width() - size, 0),
            physical_size: UVec2::new(size, size),
            ..default()
        });
    }
}
