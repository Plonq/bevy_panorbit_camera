#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_easings::Lerp;
#[cfg(feature = "bevy_egui")]
use egui::EguiWantsFocus;
use std::f32::consts::{PI, TAU};

#[cfg(feature = "bevy_egui")]
mod egui;

/// Bevy plugin that contains the systems for controlling `PanOrbitCamera` components.
/// # Example
/// ```no_run
/// # use bevy::prelude::*;
/// # use bevy_panorbit_camera::{PanOrbitCameraPlugin, PanOrbitCamera};
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_plugin(PanOrbitCameraPlugin)
///         .run();
/// }
/// ```
pub struct PanOrbitCameraPlugin;

impl Plugin for PanOrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(pan_orbit_camera.in_set(PanOrbitCameraSystemSet));

        #[cfg(feature = "bevy_egui")]
        {
            app.init_resource::<EguiWantsFocus>()
                .add_system(egui::check_egui_wants_focus.before(PanOrbitCameraSystemSet))
                .configure_set(
                    PanOrbitCameraSystemSet.run_if(resource_equals(EguiWantsFocus {
                        prev: false,
                        curr: false,
                    })),
                );
        }
    }
}

/// System set to allow ordering of `PanOrbitCamera`
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PanOrbitCameraSystemSet;

/// Tags an entity as capable of panning and orbiting, and provides a way to configure the
/// camera's behaviour and controls.
/// The entity must have `Transform` and `Projection` components. Typically you would add a
/// `Camera3dBundle` which already contains these.
/// # Example
/// ```no_run
/// # use bevy::prelude::*;
/// # use bevy_panorbit_camera::{PanOrbitCameraPlugin, PanOrbitCamera};
/// # fn main() {
/// #     App::new()
/// #         .add_plugins(DefaultPlugins)
/// #         .add_plugin(PanOrbitCameraPlugin)
/// #         .add_startup_system(setup)
/// #         .run();
/// # }
/// fn setup(mut commands: Commands) {
///     commands
///         .spawn((
///             Camera3dBundle::default(),
///             PanOrbitCamera::default(),
///         ));
///  }
/// ```
#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct PanOrbitCamera {
    /// The point to orbit around. Automatically updated when panning the camera.
    /// Defaults to `Vec3::ZERO`.
    pub focus: Vec3,
    /// The radius of the orbit, or the distance from the `focus` point.
    /// For orthographic projection, this is ignored, and the projection's scale is used instead.
    /// Automatically updated (only for cameras with perspective projection).
    /// Defaults to `5.0`.
    pub radius: f32,
    /// Rotation in radians around the global Y axis (longitudinal). Updated automatically.
    /// If both `alpha` and `beta` are `0.0`, then the camera will be looking forward, i.e. in
    /// the `Vec3::NEG_Z` direction, with up being `Vec3::Y`.
    /// Defaults to `0.0`.
    pub alpha: f32,
    /// Rotation in radians around the local X axis (latitudinal). Updated automatically.
    /// If both `alpha` and `beta` are `0.0`, then the camera will be looking forward, i.e. in
    /// the `Vec3::NEG_Z` direction, with up being `Vec3::Y`.
    /// Defaults to `0.0`.
    pub beta: f32,
    /// The target alpha value. The camera will smoothly transition to this value. Used internally
    /// and typically you won't set this manually.
    pub target_alpha: f32,
    /// The target beta value. The camera will smoothly transition to this value. Used internally
    /// and typically you won't set this manually.
    pub target_beta: f32,
    /// Upper limit on the `alpha` value, in radians. Use this to restrict the maximum rotation
    /// around the global Y axis.
    pub alpha_upper_limit: Option<f32>,
    /// Lower limit on the `alpha` value, in radians. Use this to restrict the maximum rotation
    /// around the global Y axis.
    pub alpha_lower_limit: Option<f32>,
    /// Upper limit on the `beta` value, in radians. Use this to restrict the maximum rotation
    /// around the local X axis.
    pub beta_upper_limit: Option<f32>,
    /// Lower limit on the `beta` value, in radians. Use this to restrict the maximum rotation
    /// around the local X axis.
    pub beta_lower_limit: Option<f32>,
    /// The sensitivity of the orbiting motion. Defaults to `1.0`.
    pub orbit_sensitivity: f32,
    /// The sensitivity of the panning motion. Defaults to `1.0`.
    pub pan_sensitivity: f32,
    /// The sensitivity of moving the camera closer or further way using the scroll wheel. Defaults to `1.0`.
    pub zoom_sensitivity: f32,
    /// Button used to orbit the camera. Defaults to `Button::Left`.
    pub button_orbit: MouseButton,
    /// Button used to pan the camera. Defaults to `Button::Right`.
    pub button_pan: MouseButton,
    /// Key that must be pressed for `button_orbit` to work. Defaults to `None` (no modifier).
    pub modifier_orbit: Option<KeyCode>,
    /// Key that must be pressed for `button_pan` to work. Defaults to `None` (no modifier).
    pub modifier_pan: Option<KeyCode>,
    /// Whether to reverse the zoom direction. Defaults to `false`.
    pub reversed_zoom: bool,
    /// Whether the camera is currently upside down. Updated automatically. Should not be set manually.
    pub is_upside_down: bool,
    /// Whether to allow the camera to go upside down. Defaults to `false`.
    pub allow_upside_down: bool,
    /// If `false`, disable control of the camera. Defaults to `true`.
    pub enabled: bool,
    /// Whether `PanOrbitCamera` has been initialized with the initial config.
    /// Set to `true` if you want the camera to smoothly animate to its initial position.
    /// Defaults to `false`.
    pub initialized: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            is_upside_down: false,
            allow_upside_down: false,
            orbit_sensitivity: 1.0,
            pan_sensitivity: 1.0,
            zoom_sensitivity: 1.0,
            button_orbit: MouseButton::Left,
            button_pan: MouseButton::Right,
            modifier_orbit: None,
            modifier_pan: None,
            reversed_zoom: false,
            enabled: true,
            alpha: 0.0,
            beta: 0.0,
            target_alpha: 0.0,
            target_beta: 0.0,
            initialized: false,
            alpha_upper_limit: None,
            alpha_lower_limit: None,
            beta_upper_limit: None,
            beta_lower_limit: None,
        }
    }
}

impl PanOrbitCamera {
    /// Creates a `PanOrbitCamera` from a translation and focus point. Values of `alpha`, `beta`,
    /// and `radius` will be automatically calculated.
    /// # Example
    /// ```no_run
    /// # use bevy::prelude::*;
    /// # use bevy_panorbit_camera::{PanOrbitCameraPlugin, PanOrbitCamera};
    /// # fn main() {
    /// #     App::new()
    /// #         .add_plugins(DefaultPlugins)
    /// #         .add_plugin(PanOrbitCameraPlugin)
    /// #         .add_startup_system(setup)
    /// #         .run();
    /// # }
    /// fn setup(mut commands: Commands) {
    ///     let position = Vec3::new(4.0, 2.0, 3.0);
    ///     let focus = Vec3::new(1.0, 1.0, 1.0);
    ///     commands
    ///         .spawn((
    ///             Camera3dBundle::default(),
    ///             PanOrbitCamera::from_translation_and_focus(position, focus),
    ///         ));
    ///  }
    /// ```
    pub fn from_translation_and_focus(translation: Vec3, focus: Vec3) -> Self {
        let comp_vec = translation - focus;
        let radius = comp_vec.length();
        let mut alpha = if comp_vec.x == 0.0 && comp_vec.z == 0.0 {
            PI / 2.0
        } else {
            (comp_vec.x / (comp_vec.x.powi(2) + comp_vec.z.powi(2)).sqrt()).acos()
        };
        if comp_vec.z > 0.0 {
            alpha = 2.0 * PI - alpha;
        }
        let beta = (comp_vec.y / radius).acos();
        PanOrbitCamera {
            focus,
            radius,
            alpha,
            beta,
            ..default()
        }
    }
}

/// Main system for processing input and converting to transformations
fn pan_orbit_camera(
    windows_query: Query<&Window, With<PrimaryWindow>>,
    mouse_input: Res<Input<MouseButton>>,
    key_input: Res<Input<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut scroll_events: EventReader<MouseWheel>,
    mut camera_query: Query<(&mut PanOrbitCamera, &mut Transform, &mut Projection)>,
) {
    for (mut pan_orbit, mut transform, mut projection) in camera_query.iter_mut() {
        if !pan_orbit.initialized {
            if let Some(upper_alpha) = pan_orbit.alpha_upper_limit {
                if pan_orbit.alpha > upper_alpha {
                    pan_orbit.alpha = upper_alpha;
                }
            }
            if let Some(lower_alpha) = pan_orbit.alpha_lower_limit {
                if pan_orbit.alpha < lower_alpha {
                    pan_orbit.alpha = lower_alpha;
                }
            }
            if let Some(upper_beta) = pan_orbit.beta_upper_limit {
                if pan_orbit.beta > upper_beta {
                    pan_orbit.beta = upper_beta;
                }
            }
            if let Some(lower_beta) = pan_orbit.beta_lower_limit {
                if pan_orbit.beta < lower_beta {
                    pan_orbit.beta = lower_beta;
                }
            }
            update_orbit_transform(pan_orbit.alpha, pan_orbit.beta, &pan_orbit, &mut transform);
            pan_orbit.target_alpha = pan_orbit.alpha;
            pan_orbit.target_beta = pan_orbit.beta;

            pan_orbit.initialized = true;
            return;
        }

        // 1 - Get Input

        let mut pan = Vec2::ZERO;
        let mut rotation_move = Vec2::ZERO;
        let mut scroll = 0.0;
        let mut orbit_button_changed = false;

        if pan_orbit.enabled {
            if orbit_pressed(&pan_orbit, &mouse_input, &key_input) {
                for ev in mouse_motion_events.iter() {
                    rotation_move += ev.delta * pan_orbit.orbit_sensitivity;
                }
            } else if pan_pressed(&pan_orbit, &mouse_input, &key_input) {
                // Pan only if we're not rotating at the moment
                for ev in mouse_motion_events.iter() {
                    pan += ev.delta * pan_orbit.pan_sensitivity;
                }
            }

            for ev in scroll_events.iter() {
                let direction = match pan_orbit.reversed_zoom {
                    true => -1.0,
                    false => 1.0,
                };
                let multiplier = match ev.unit {
                    MouseScrollUnit::Line => 1.0,
                    MouseScrollUnit::Pixel => 0.005,
                };
                scroll += ev.y * multiplier * direction * pan_orbit.zoom_sensitivity;
            }

            if orbit_just_pressed_or_released(&pan_orbit, &mouse_input, &key_input) {
                orbit_button_changed = true;
            }
        }

        // 2 - Process input into target alpha/beta, or focus, radius

        if orbit_button_changed {
            // Only check for upside down when orbiting started or ended this frame,
            // so we don't reverse the horizontal direction while the user is still dragging
            let wrapped_beta = (pan_orbit.target_beta % TAU).abs();
            pan_orbit.is_upside_down = wrapped_beta > TAU / 4.0 && wrapped_beta < 3.0 * TAU / 4.0;
        }

        let mut has_moved = false;
        if rotation_move.length_squared() > 0.0 {
            let window = get_primary_window_size(&windows_query);
            let delta_x = {
                let delta = rotation_move.x / window.x * PI * 2.0;
                if pan_orbit.is_upside_down {
                    -delta
                } else {
                    delta
                }
            };
            let delta_y = rotation_move.y / window.y * PI;
            pan_orbit.target_alpha -= delta_x;
            pan_orbit.target_beta += delta_y;

            if let Some(upper_alpha) = pan_orbit.alpha_upper_limit {
                if pan_orbit.target_alpha > upper_alpha {
                    pan_orbit.target_alpha = upper_alpha;
                }
            }
            if let Some(lower_alpha) = pan_orbit.alpha_lower_limit {
                if pan_orbit.target_alpha < lower_alpha {
                    pan_orbit.target_alpha = lower_alpha;
                }
            }
            if let Some(upper_beta) = pan_orbit.beta_upper_limit {
                if pan_orbit.target_beta > upper_beta {
                    pan_orbit.target_beta = upper_beta;
                }
            }
            if let Some(lower_beta) = pan_orbit.beta_lower_limit {
                if pan_orbit.target_beta < lower_beta {
                    pan_orbit.target_beta = lower_beta;
                }
            }

            if !pan_orbit.allow_upside_down {
                if pan_orbit.target_beta < -PI / 2.0 {
                    pan_orbit.target_beta = -PI / 2.0;
                }
                if pan_orbit.target_beta > PI / 2.0 {
                    pan_orbit.target_beta = PI / 2.0;
                }
            }
            has_moved = true;
        } else if pan.length_squared() > 0.0 {
            // Make panning distance independent of resolution and FOV,
            let window = get_primary_window_size(&windows_query);
            let mut multiplier = 1.0;
            match *projection {
                Projection::Perspective(ref p) => {
                    pan *= Vec2::new(p.fov * p.aspect_ratio, p.fov) / window;
                    // Make panning proportional to distance away from focus point
                    multiplier = pan_orbit.radius;
                }
                Projection::Orthographic(ref p) => {
                    pan *= Vec2::new(p.area.width(), p.area.height()) / window;
                }
            }
            // Translate by local axes
            let right = transform.rotation * Vec3::X * -pan.x;
            let up = transform.rotation * Vec3::Y * pan.y;
            let translation = (right + up) * multiplier;
            pan_orbit.focus += translation;
            has_moved = true;
        } else if scroll.abs() > 0.0 {
            match *projection {
                Projection::Perspective(_) => {
                    pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
                    // Prevent zoom to zero otherwise we can get stuck there
                    pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
                }
                Projection::Orthographic(ref mut p) => {
                    p.scale -= scroll * p.scale * 0.2;
                    // Prevent zoom to zero otherwise we can get stuck there
                    p.scale = f32::max(p.scale, 0.05);
                }
            }
            has_moved = true;
        }

        // 3 - Apply orbit rotation based on target alpha/beta

        if has_moved
            || pan_orbit.target_alpha != pan_orbit.alpha
            || pan_orbit.target_beta != pan_orbit.beta
        {
            // Otherwise, interpolate our way there
            let mut target_alpha = pan_orbit.alpha.lerp(&pan_orbit.target_alpha, &0.2);
            let mut target_beta = pan_orbit.beta.lerp(&pan_orbit.target_beta, &0.2);

            // If we're super close, then just snap to target rotation to save cycles
            if (target_alpha - pan_orbit.target_alpha).abs() < 0.001 {
                target_alpha = pan_orbit.target_alpha;
            }
            if (target_beta - pan_orbit.target_beta).abs() < 0.001 {
                target_beta = pan_orbit.target_beta;
            }

            update_orbit_transform(target_alpha, target_beta, &pan_orbit, &mut transform);

            // Update current alpha and beta values
            pan_orbit.alpha = target_alpha;
            pan_orbit.beta = target_beta;
        }
    }
}

fn orbit_pressed(
    pan_orbit: &PanOrbitCamera,
    mouse_input: &Res<Input<MouseButton>>,
    key_input: &Res<Input<KeyCode>>,
) -> bool {
    let is_pressed = pan_orbit
        .modifier_orbit
        .map_or(true, |modifier| key_input.pressed(modifier))
        && mouse_input.pressed(pan_orbit.button_orbit);

    is_pressed
        && pan_orbit
            .modifier_pan
            .map_or(true, |modifier| !key_input.pressed(modifier))
}

fn orbit_just_pressed_or_released(
    pan_orbit: &PanOrbitCamera,
    mouse_input: &Res<Input<MouseButton>>,
    key_input: &Res<Input<KeyCode>>,
) -> bool {
    let just_pressed = pan_orbit
        .modifier_orbit
        .map_or(true, |modifier| key_input.pressed(modifier))
        && (mouse_input.just_pressed(pan_orbit.button_orbit)
            || mouse_input.just_released(pan_orbit.button_orbit));

    just_pressed
        && pan_orbit
            .modifier_pan
            .map_or(true, |modifier| !key_input.pressed(modifier))
}

fn pan_pressed(
    pan_orbit: &PanOrbitCamera,
    mouse_input: &Res<Input<MouseButton>>,
    key_input: &Res<Input<KeyCode>>,
) -> bool {
    let is_pressed = pan_orbit
        .modifier_pan
        .map_or(true, |modifier| key_input.pressed(modifier))
        && mouse_input.pressed(pan_orbit.button_pan);

    is_pressed
        && pan_orbit
            .modifier_orbit
            .map_or(true, |modifier| !key_input.pressed(modifier))
}

fn get_primary_window_size(windows_query: &Query<&Window, With<PrimaryWindow>>) -> Vec2 {
    let Ok(primary) = windows_query.get_single() else {
        // No primary window? Dunno how we can be controlling a camera, but let's return ONE
        // so when dividing by this value nothing explodes
        return Vec2::ONE;
    };
    Vec2::new(primary.width(), primary.height())
}

/// Update `transform` based on alpha, beta, and the camera's focus and radius
fn update_orbit_transform(
    alpha: f32,
    beta: f32,
    pan_orbit: &PanOrbitCamera,
    transform: &mut Transform,
) {
    let mut rotation = Quat::from_rotation_y(alpha);
    rotation *= Quat::from_rotation_x(-beta);

    transform.rotation = rotation;

    // Update the translation of the camera so we are always rotating 'around'
    // (orbiting) rather than rotating in place
    let rot_matrix = Mat3::from_quat(transform.rotation);
    transform.translation =
        pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
}
