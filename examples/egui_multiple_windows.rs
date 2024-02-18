//! Demonstrates the bevy_egui feature together with multiple windows.
//! This is a combination of the egui and multiple_windows examples, and doesn't show anything new,
//! it's primarily here for easy e2e testing.

use bevy::ecs::event::EventUpdateSignal;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::window::WindowRef;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, ui_example_system);

    // Make Bevy drop unconsumed events every frame to prevent weird behaviour when moving mouse
    // out of an egui window right after scrolling (zooming)
    // See: https://bevyengine.org/news/bevy-0-13/#events-live-longer
    app.world.remove_resource::<EventUpdateSignal>();

    app.run();
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
        PanOrbitCamera::default(),
    ));

    // Spawn a second window
    let second_window = commands
        .spawn(Window {
            title: "Second window".to_owned(),
            ..default()
        })
        .id();

    // second window camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(5.0, 1.5, 7.0)),
            camera: Camera {
                target: RenderTarget::Window(WindowRef::Entity(second_window)),
                ..default()
            },
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}

fn ui_example_system(mut contexts: EguiContexts, windows: Query<Entity, With<Window>>) {
    for window in windows.iter() {
        egui::Window::new("Hello").show(contexts.ctx_for_window_mut(window), |ui| {
            ui.label("world");
        });
    }
}
