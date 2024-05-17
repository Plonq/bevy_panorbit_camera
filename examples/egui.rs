//! Demonstrates the bevy_egui feature which allows bevy_panorbit_camera to ignore input events in
//! egui windows

use bevy::ecs::event::EventUpdateSignal;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_panorbit_camera::{EguiWantsFocus, PanOrbitCamera, PanOrbitCameraPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        // .insert_resource(EguiWantsFocus {
        //     include_hover: true,
        //     ..default()
        // })
        .add_plugins(EguiPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (draw_file_tree, draw_title_bar, ui_example_system).chain(),
        );

    // Make Bevy drop unconsumed events every frame to prevent weird behaviour when moving mouse
    // out of an egui window immediately after scrolling (zooming)
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
        PanOrbitCamera::default(),
    ));
}

fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}

pub fn draw_title_bar(mut contexts: EguiContexts) {
    egui::TopBottomPanel::top("title_bar").show(contexts.ctx_mut(), |ui| {
        ui.visuals_mut().button_frame = false;
        ui.horizontal(|ui| {
            ui.menu_button("File", |ui| {
                let _ = ui.button("Open").clicked();
            });
            ui.separator();
            ui.menu_button("View", |ui| {
                let mut test = false;
                ui.checkbox(&mut test, "Checkbox A");
                ui.checkbox(&mut test, "Checkbox B");
            });
        });
    });
}

pub fn draw_file_tree(mut contexts: EguiContexts) {
    egui::SidePanel::left("file_tree")
        .default_width(300.0)
        .max_width(500.0)
        .show(contexts.ctx_mut(), |ui| {
            egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
                egui::CollapsingHeader::new("Very Long Folder Name To Force Scroll A").show(
                    ui,
                    |ui| {
                        ui.add(
                            egui::Label::new("Very Long File Name To Force Scroll A")
                                .selectable(false)
                                .sense(egui::Sense::click()),
                        );
                        ui.add(
                            egui::Label::new("Very Long File Name To Force Scroll B")
                                .selectable(false)
                                .sense(egui::Sense::click()),
                        );
                    },
                );
                ui.add(
                    egui::Label::new("Very Long File Name To Force Scroll C")
                        .selectable(false)
                        .sense(egui::Sense::click()),
                );
            });
        });
}
