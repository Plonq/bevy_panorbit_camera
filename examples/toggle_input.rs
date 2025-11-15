//! Demonstrates simplified ui toggling controlled by a public resource

use bevy::prelude::*;
use bevy_panorbit_camera::{
  PanOrbitCameraIgnoreInput,
  PanOrbitCameraPlugin,
  PanOrbitCamera
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup_3d) // setup stuff to pann over/orbit around
        .add_systems(Startup, setup_ui) // setup the fancy ui
        .add_systems(Update, interact_with_ui)
        .run();
}

fn setup_3d(
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
        Transform::from_xyz(0.0, 1.5, 5.0),
        PanOrbitCamera::default(),
    ));
}

fn interact_with_ui (
    query: Query<
        (
            Entity,
            &Button,
            &Interaction,
            &mut BackgroundColor,
        ),
        Changed<Interaction>,
    >,
    mut allow_input: ResMut<PanOrbitCameraIgnoreInput>
) {

    let mut ignore_input_now: Option<bool> = None;
    // guessing there's a "i know there's only one of these" type query syntax
    // but idk whta it is.
    // I also don't really know if this ui interaction system would actually be
    // reliable for this purpose IRL, it's just the absolute simplest demo I
    // could come up with
    for (_id, _node, interaction, mut color) in query {
        match *interaction {
            Interaction::Pressed => {
              // make it red when pushing on the button... we make it red and
              // don't give the camera any inputs
              ignore_input_now = Some(true);
              *color = BackgroundColor(Color::srgba(0.8, 0.2, 0.15, 0.5));
            }
            Interaction::Hovered => {
              // just hovering, make it blue and but dont control the camera
              ignore_input_now = Some(true);
              *color = BackgroundColor(Color::srgba(0.5, 0.9, 1.0, 0.25));
            }
            Interaction::None => {
              // mouse has exited ui: make it orange and now the camera works
              ignore_input_now = Some(false);
              *color = BackgroundColor(Color::srgba(0.15, 0.3, 0.6, 0.25));
            }
        }
    }

    match ignore_input_now {
      None => {},
      Some(value) => {
        allow_input.set_if_neq(PanOrbitCameraIgnoreInput(value));
      }
    };
}

fn setup_ui(mut commands: Commands) {
    // ui camera
    commands.spawn((
      Camera2d,
      Camera { order: 1, ..default() }
    ));
    // the ui itself: a big in the corner that will steal input from the camera.
    commands.spawn((
        Node {
            width: percent(25.0),
            height: percent(25.0),
            top: px(100.0),
            left: px(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        Button,
        BackgroundColor(Color::srgba(0.2, 0.4, 0.8, 0.5)),
    ));
}
