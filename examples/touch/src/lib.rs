// This lint usually gives bad advice in the context of Bevy -- hiding complex queries behind
// type aliases tends to obfuscate code while offering no improvement in code cleanliness.
#![allow(clippy::type_complexity)]

use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::{
    input::touch::TouchPhase,
    log::LogPlugin,
    prelude::*,
    window::{ApplicationLifetime, PrimaryWindow, WindowMode},
};
use bevy_panorbit_camera::{ActiveCameraData, PanOrbitCamera, PanOrbitCameraPlugin};

// the `bevy_main` proc_macro generates the required boilerplate for iOS and Android
#[bevy_main]
fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    mode: WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            })
            .set(LogPlugin {
                filter: "warn,winit=error,bevy_panorbit_camera=debug".into(),
                level: bevy::log::Level::DEBUG,
            }),
    )
    .add_plugins(PanOrbitCameraPlugin)
    .add_systems(Startup, setup_scene)
    .add_systems(Update, button_handler);

    // MSAA makes some Android devices panic, this is under investigation
    // https://github.com/bevyengine/bevy/issues/8229
    #[cfg(target_os = "android")]
    app.insert_resource(Msaa::Off);

    app.run();
}

/// set up a simple 3D scene
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut active_cam: ResMut<ActiveCameraData>,
    windows: Query<&Window, With<PrimaryWindow>>,
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
    // 2D camera for debugging
    commands.spawn(Camera2dBundle {
        camera: Camera {
            // Renders the minimap camera after the main camera, so it is rendered on top
            order: 1,
            ..default()
        },
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::None,
            ..default()
        },
        ..default()
    });
    // camera
    // let pan_orbit_id = commands
    //     .spawn((
    //         Camera3dBundle {
    //             transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    //             ..default()
    //         },
    //         PanOrbitCamera::default(),
    //     ))
    //     .id();

    // Test ui
    // commands
    //     .spawn(ButtonBundle {
    //         style: Style {
    //             justify_content: JustifyContent::Center,
    //             align_items: AlignItems::Center,
    //             position_type: PositionType::Absolute,
    //             left: Val::Px(50.0),
    //             right: Val::Px(50.0),
    //             bottom: Val::Px(50.0),
    //             ..default()
    //         },
    //         ..default()
    //     })
    //     .with_children(|b| {
    //         b.spawn(
    //             TextBundle::from_section(
    //                 "Test Button",
    //                 TextStyle {
    //                     font_size: 30.0,
    //                     color: Color::BLACK,
    //                     ..default()
    //                 },
    //             )
    //             .with_text_alignment(TextAlignment::Center),
    //         );
    //     });
    //
    // let primary_window = windows
    //     .get_single()
    //     .expect("There is only ever one primary window");
    // active_cam.set_if_neq(ActiveCameraData {
    //     entity: Some(pan_orbit_id),
    //     viewport_size: Some(Vec2::new(primary_window.width(), primary_window.height())),
    //     window_size: Some(Vec2::new(primary_window.width(), primary_window.height())),
    //     manual: true,
    // });
}

fn button_handler(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::BLUE.into();
            }
            Interaction::Hovered => {
                *color = Color::GRAY.into();
            }
            Interaction::None => {
                *color = Color::WHITE.into();
            }
        }
    }
}
