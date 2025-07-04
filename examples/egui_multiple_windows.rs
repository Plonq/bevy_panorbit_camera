//! Demonstrates the bevy_egui feature together with multiple windows.
//! This is a combination of the egui and multiple_windows examples, and doesn't show anything new,
//! it's primarily here for easy e2e testing.

use bevy::render::camera::RenderTarget;
use bevy::window::WindowRef;
use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use bevy_egui::{
    egui, EguiContext, EguiMultipassSchedule, EguiPlugin, EguiPrimaryContextPass,
    PrimaryEguiContext,
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SecondWindowContextPass;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(EguiPrimaryContextPass, ui_example_system_first_window)
        .add_systems(SecondWindowContextPass, ui_example_system_second_window);

    app.run();
}

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
        Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
        PanOrbitCamera::default(),
        PrimaryEguiContext,
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
        Camera {
            target: RenderTarget::Window(WindowRef::Entity(second_window)),
            ..default()
        },
        Transform::from_translation(Vec3::new(5.0, 1.5, 7.0)),
        PanOrbitCamera::default(),
        EguiMultipassSchedule::new(SecondWindowContextPass),
    ));
}

fn ui_example_system_first_window(
    mut egui_ctx: Single<&mut EguiContext, With<PrimaryEguiContext>>,
) -> Result {
    egui::Window::new("Hello").show(egui_ctx.get_mut(), |ui| {
        ui.label("world");
    });
    Ok(())
}

fn ui_example_system_second_window(
    mut egui_ctx: Single<&mut EguiContext, Without<PrimaryEguiContext>>,
) -> Result {
    egui::Window::new("Hello").show(egui_ctx.get_mut(), |ui| {
        ui.label("world2");
    });
    Ok(())
}
