#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::window::{PrimaryWindow, WindowRef};
use bevy_easings::Lerp;
#[cfg(feature = "bevy_egui")]
use bevy_egui::EguiSet;
#[cfg(feature = "bevy_egui")]
use egui::EguiWantsFocus;
use util::approx_equal;
use std::f32::consts::{PI, TAU};

#[cfg(feature = "bevy_egui")]
mod egui;
mod util;

/// Bevy plugin that contains the systems for controlling `PanOrbitCamera` components.
/// # Example
/// ```no_run
/// # use bevy::prelude::*;
/// # use bevy_panorbit_camera::{PanOrbitCameraPlugin, PanOrbitCamera};
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_plugins(PanOrbitCameraPlugin)
///         .run();
/// }
/// ```
pub struct PanOrbitCameraPlugin;

impl Plugin for PanOrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActiveCameraData::default())
            .add_systems(
                Update,
                (active_viewport_data, pan_orbit_camera)
                    .chain()
                    .in_set(PanOrbitCameraSystemSet),
            );

        #[cfg(feature = "bevy_egui")]
        {
            app.init_resource::<EguiWantsFocus>()
                .add_systems(
                    Update,
                    egui::check_egui_wants_focus
                        .after(EguiSet::InitContexts)
                        .before(PanOrbitCameraSystemSet),
                )
                .configure_set(
                    Update,
                    PanOrbitCameraSystemSet.run_if(resource_equals(EguiWantsFocus {
                        prev: false,
                        curr: false,
                    })),
                );
        }
    }
}

/// Base system set to allow ordering of `PanOrbitCamera`
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
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
/// #         .add_plugins(PanOrbitCameraPlugin)
/// #         .add_systems(Startup, setup)
/// #         .run();
/// # }
/// fn setup(mut commands: Commands) {
///     commands
///         .spawn((
///             Camera3dBundle {
///                 transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
///                 ..default()
///             },
///             PanOrbitCamera::default(),
///         ));
///  }
/// ```
#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct PanOrbitCamera {
    /// The point to orbit around, and what the camera looks at. Updated automatically.
    /// If you want to change the focus programmatically after initialization, set `target_focus`
    /// instead.
    /// Defaults to `Vec3::ZERO`.
    pub focus: Vec3,
    /// The radius of the orbit, or the distance from the `focus` point.
    /// For orthographic projection, this is ignored, and the projection's `scale` is used instead.
    /// If set to `None`, it will be calculated from the camera's current position during
    /// initialization.
    /// Automatically updated.
    /// Defaults to `None`.
    pub radius: Option<f32>,
    /// Rotation in radians around the global Y axis (longitudinal). Updated automatically.
    /// If both `alpha` and `beta` are `0.0`, then the camera will be looking forward, i.e. in
    /// the `Vec3::NEG_Z` direction, with up being `Vec3::Y`.
    /// If set to `None`, it will be calculated from the camera's current position during
    /// initialization.
    /// You should not update this after initialization - use `target_alpha` instead.
    /// Defaults to `None`.
    pub alpha: Option<f32>,
    /// Rotation in radians around the local X axis (latitudinal). Updated automatically.
    /// If both `alpha` and `beta` are `0.0`, then the camera will be looking forward, i.e. in
    /// the `Vec3::NEG_Z` direction, with up being `Vec3::Y`.
    /// If set to `None`, it will be calculated from the camera's current position during
    /// initialization.
    /// You should not update this after initialization - use `target_beta` instead.
    /// Defaults to `None`.
    pub beta: Option<f32>,
    /// The target focus point. The camera will smoothly transition to this value. Updated
    /// automatically, but you can also update it manually to control the camera independently of
    /// the mouse controls, e.g. with the keyboard.
    /// Defaults to `Vec3::ZERO`.
    pub target_focus: Vec3,
    /// The target alpha value. The camera will smoothly transition to this value. Updated
    /// automatically, but you can also update it manually to control the camera independently of
    /// the mouse controls, e.g. with the keyboard.
    /// Defaults to `0.0`.
    pub target_alpha: f32,
    /// The target beta value. The camera will smoothly transition to this value Updated
    /// automatically, but you can also update it manually to control the camera independently of
    /// the mouse controls, e.g. with the keyboard.
    /// Defaults to `0.0`.
    pub target_beta: f32,
    /// The target radius value. The camera will smoothly transition to this value. Updated
    /// automatically, but you can also update it manually to control the camera independently of
    /// the mouse controls, e.g. with the keyboard.
    /// Defaults to `1.0`.
    pub target_radius: f32,
    /// Upper limit on the `alpha` value, in radians. Use this to restrict the maximum rotation
    /// around the global Y axis.
    /// Defaults to `None`.
    pub alpha_upper_limit: Option<f32>,
    /// Lower limit on the `alpha` value, in radians. Use this to restrict the maximum rotation
    /// around the global Y axis.
    /// Defaults to `None`.
    pub alpha_lower_limit: Option<f32>,
    /// Upper limit on the `beta` value, in radians. Use this to restrict the maximum rotation
    /// around the local X axis.
    /// Defaults to `None`.
    pub beta_upper_limit: Option<f32>,
    /// Lower limit on the `beta` value, in radians. Use this to restrict the maximum rotation
    /// around the local X axis.
    /// Defaults to `None`.
    pub beta_lower_limit: Option<f32>,
    /// Upper limit on the zoom. This applies to `radius`, in the case of using a perspective
    /// camera, or the projection scale in the case of using an orthographic
    /// camera. Note that the zoom value (radius or scale) will never go below `0.02`.
    /// Defaults to `None`.
    pub zoom_upper_limit: Option<f32>,
    /// Lower limit on the zoom. This applies to `radius`, in the case of using a perspective
    /// camera, or the projection scale in the case of using an orthographic
    /// camera. Note that the zoom value (radius or scale) will never go below `0.02`.
    /// Defaults to `None`.
    pub zoom_lower_limit: Option<f32>,
    /// The sensitivity of the orbiting motion. Defaults to `1.0`.
    pub orbit_sensitivity: f32,
    /// How much smoothing is applied to the orbit motion. A value of `0.0` disables smoothing,
    /// so there's a 1:1 mapping of input to camera position. A value of `1.0` is infinite
    /// smoothing. Defaults to `0.8`.
    pub orbit_smoothness: f32,
    /// The sensitivity of the panning motion. Defaults to `1.0`.
    pub pan_sensitivity: f32,
    /// How much smoothing is applied to the panning motion. A value of `0.0` disables smoothing,
    /// so there's a 1:1 mapping of input to camera position. A value of `1.0` is infinite
    /// smoothing. Defaults to `0.6`.
    pub pan_smoothness: f32,
    /// The sensitivity of moving the camera closer or further way using the scroll wheel. Defaults to `1.0`.
    pub zoom_sensitivity: f32,
    /// How much smoothing is applied to the zoom motion. A value of `0.0` disables smoothing,
    /// so there's a 1:1 mapping of input to camera position. A value of `1.0` is infinite
    /// smoothing. Defaults to `0.8`.
    /// Note that this setting does not apply to pixel-based scroll events, as they are typically 
    /// already smooth. It only applies to line-based scroll events.
    pub zoom_smoothness: f32,
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
    /// Whether to update the camera's transform regardless of whether there are any changes/input.
    /// Set this to `true` if you want to modify values directly.
    /// This will be automatically set back to `false` after one frame.
    /// Defaults to `false`.
    pub force_update: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            target_focus: Vec3::ZERO,
            radius: None,
            is_upside_down: false,
            allow_upside_down: false,
            orbit_sensitivity: 1.0,
            orbit_smoothness: 0.8,
            pan_sensitivity: 1.0,
            pan_smoothness: 0.6,
            zoom_sensitivity: 1.0,
            zoom_smoothness: 0.8,
            button_orbit: MouseButton::Left,
            button_pan: MouseButton::Right,
            modifier_orbit: None,
            modifier_pan: None,
            reversed_zoom: false,
            enabled: true,
            alpha: None,
            beta: None,
            target_alpha: 0.0,
            target_beta: 0.0,
            target_radius: 1.0,
            initialized: false,
            alpha_upper_limit: None,
            alpha_lower_limit: None,
            beta_upper_limit: None,
            beta_lower_limit: None,
            zoom_upper_limit: None,
            zoom_lower_limit: None,
            force_update: false,
        }
    }
}

// Tracks the camera entity that should be handling input events.
// This enables having multiple cameras with different viewports or windows.
#[derive(Resource, Default, Debug, PartialEq)]
struct ActiveCameraData {
    entity: Option<Entity>,
    viewport_size: Option<Vec2>,
    window_size: Option<Vec2>,
}

// Gathers data about the active viewport, i.e. the viewport the user is interacting with. This
// enables multiple viewports/windows.
fn active_viewport_data(
    mut active_cam: ResMut<ActiveCameraData>,
    mouse_input: Res<Input<MouseButton>>,
    key_input: Res<Input<KeyCode>>,
    scroll_events: EventReader<MouseWheel>,
    primary_windows: Query<&Window, With<PrimaryWindow>>,
    other_windows: Query<&Window, Without<PrimaryWindow>>,
    orbit_cameras: Query<(Entity, &Camera, &PanOrbitCamera)>,
) {
    // let mut new_resource: Option<ActiveCameraData> = None;
    let mut new_resource = ActiveCameraData {
        entity: None,
        viewport_size: None,
        window_size: None,
    };
    let mut max_cam_order = 0;

    let mut has_input = false;
    for (entity, camera, pan_orbit) in orbit_cameras.iter() {
        let input_just_activated = util::orbit_just_pressed(pan_orbit, &mouse_input, &key_input)
            || util::pan_just_pressed(pan_orbit, &mouse_input, &key_input)
            || !scroll_events.is_empty();

        if input_just_activated {
            has_input = true;
            // First check if cursor is in the same window as this camera
            if let RenderTarget::Window(win_ref) = camera.target {
                let window = match win_ref {
                    WindowRef::Primary => primary_windows
                        .get_single()
                        .expect("Must exist, since the camera is referencing it"),
                    WindowRef::Entity(entity) => other_windows
                        .get(entity)
                        .expect("Must exist, since the camera is referencing it"),
                };
                if let Some(cursor_pos) = window.cursor_position() {
                    // Now check if cursor is within this camera's viewport
                    if let Some(Rect { min, max }) = camera.logical_viewport_rect() {
                        // Window coordinates have Y starting at the bottom, so we need to reverse
                        // the y component before comparing with the viewport rect
                        let cursor_in_vp = cursor_pos.x > min.x
                            && cursor_pos.x < max.x
                            && cursor_pos.y > min.y
                            && cursor_pos.y < max.y;

                        // Only set if camera order is higher. This may overwrite a previous value
                        // in the case the viewport is overlapping another viewport.
                        if cursor_in_vp && camera.order >= max_cam_order {
                            new_resource = ActiveCameraData {
                                entity: Some(entity),
                                viewport_size: camera.logical_viewport_size(),
                                window_size: Some(Vec2::new(window.width(), window.height())),
                            };
                            max_cam_order = camera.order;
                        }
                    }
                }
            }
        }
    }

    if has_input {
        active_cam.set_if_neq(new_resource);
    }
}

/// Main system for processing input and converting to transformations
fn pan_orbit_camera(
    active_cam: Res<ActiveCameraData>,
    mouse_input: Res<Input<MouseButton>>,
    key_input: Res<Input<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut scroll_events: EventReader<MouseWheel>,
    mut orbit_cameras: Query<(Entity, &mut PanOrbitCamera, &mut Transform, &mut Projection)>,
) {
    let mouse_delta = mouse_motion.iter().map(|event| event.delta).sum::<Vec2>();

    for (entity, mut pan_orbit, mut transform, mut projection) in orbit_cameras.iter_mut() {
        if !pan_orbit.initialized {
            // Calculate alpha, beta, and radius from the camera's position. If user sets all
            // these explicitly, this calculation is wasted, but that's okay since it will only run
            // once on init.
            let (alpha, beta, radius) =
                util::calculate_from_translation_and_focus(transform.translation, pan_orbit.focus);
            let &mut mut alpha = pan_orbit.alpha.get_or_insert(alpha);
            let &mut mut beta = pan_orbit.beta.get_or_insert(beta);
            let &mut mut radius = pan_orbit.radius.get_or_insert(radius);

            // Apply limits
            if let Some(upper_alpha) = pan_orbit.alpha_upper_limit {
                if alpha > upper_alpha {
                    alpha = upper_alpha;
                }
            }
            if let Some(lower_alpha) = pan_orbit.alpha_lower_limit {
                if alpha < lower_alpha {
                    alpha = lower_alpha;
                }
            }
            if let Some(upper_beta) = pan_orbit.beta_upper_limit {
                if beta > upper_beta {
                    beta = upper_beta;
                }
            }
            if let Some(lower_beta) = pan_orbit.beta_lower_limit {
                if beta < lower_beta {
                    beta = lower_beta;
                }
            }
            radius = util::apply_zoom_limits(
                radius,
                pan_orbit.zoom_upper_limit,
                pan_orbit.zoom_lower_limit,
            );

            pan_orbit.alpha = Some(alpha);
            pan_orbit.beta = Some(beta);
            pan_orbit.radius = Some(radius);
            pan_orbit.target_radius = radius;
            pan_orbit.target_alpha = alpha;
            pan_orbit.target_beta = beta;
            pan_orbit.target_focus = pan_orbit.focus;

            if let Projection::Orthographic(ref mut p) = *projection {
                p.scale = radius;
            }

            util::update_orbit_transform(alpha, beta, &pan_orbit, &mut transform);

            pan_orbit.initialized = true;
        }

        // 1 - Get Input

        let mut pan = Vec2::ZERO;
        let mut rotation_move = Vec2::ZERO;
        let mut scroll_line = 0.0;
        let mut scroll_pixel = 0.0;
        let mut orbit_button_changed = false;

        if pan_orbit.enabled && active_cam.entity == Some(entity) {
            if util::orbit_pressed(&pan_orbit, &mouse_input, &key_input) {
                rotation_move += mouse_delta * pan_orbit.orbit_sensitivity;
            } else if util::pan_pressed(&pan_orbit, &mouse_input, &key_input) {
                // Pan only if we're not rotating at the moment
                pan += mouse_delta * pan_orbit.pan_sensitivity;
            }

            for ev in scroll_events.iter() {
                let direction = match pan_orbit.reversed_zoom {
                    true => -1.0,
                    false => 1.0,
                };
                let delta_scroll = ev.y * direction * pan_orbit.zoom_sensitivity;
                match ev.unit {
                    MouseScrollUnit::Line => {
                        scroll_line += delta_scroll;
                    }
                    MouseScrollUnit::Pixel => {
                        scroll_pixel += delta_scroll * 0.005;
                    }
                };
            }

            if util::orbit_just_pressed(&pan_orbit, &mouse_input, &key_input)
                || util::orbit_just_released(&pan_orbit, &mouse_input, &key_input)
            {
                orbit_button_changed = true;
            }
        }

        // 2 - Process input into target alpha/beta, or focus, radius

        if orbit_button_changed {
            // Only check for upside down when orbiting started or ended this frame,
            // so we don't reverse the alpha direction while the user is still dragging
            let wrapped_beta = (pan_orbit.target_beta % TAU).abs();
            pan_orbit.is_upside_down = wrapped_beta > TAU / 4.0 && wrapped_beta < 3.0 * TAU / 4.0;
        }

        let mut has_moved = false;
        if rotation_move.length_squared() > 0.0 {
            // Use window size for rotation otherwise the sensitivity
            // is far too high for small viewports
            if let Some(win_size) = active_cam.window_size {
                let delta_x = {
                    let delta = rotation_move.x / win_size.x * PI * 2.0;
                    if pan_orbit.is_upside_down {
                        -delta
                    } else {
                        delta
                    }
                };
                let delta_y = rotation_move.y / win_size.y * PI;
                pan_orbit.target_alpha -= delta_x;
                pan_orbit.target_beta += delta_y;

                has_moved = true;
            }
        } else if pan.length_squared() > 0.0 {
            // Make panning distance independent of resolution and FOV,
            if let Some(vp_size) = active_cam.viewport_size {
                let mut multiplier = 1.0;
                match *projection {
                    Projection::Perspective(ref p) => {
                        pan *= Vec2::new(p.fov * p.aspect_ratio, p.fov) / vp_size;
                        // Make panning proportional to distance away from focus point
                        if let Some(radius) = pan_orbit.radius {
                            multiplier = radius;
                        }
                    }
                    Projection::Orthographic(ref p) => {
                        pan *= Vec2::new(p.area.width(), p.area.height()) / vp_size;
                    }
                }
                // Translate by local axes
                let right = transform.rotation * Vec3::X * -pan.x;
                let up = transform.rotation * Vec3::Y * pan.y;
                let translation = (right + up) * multiplier;
                pan_orbit.target_focus += translation;
                has_moved = true;
            }
        } else if (scroll_line + scroll_pixel).abs() > 0.0 {
            // Calculate the amount of change based on line scroll and pixel scroll
            let line_delta = -scroll_line * pan_orbit.target_radius * 0.2;
            let pixel_delta = -scroll_pixel * pan_orbit.target_radius * 0.2;
            // Add the calculated deltas to the target radius
            pan_orbit.target_radius = util::apply_zoom_limits(
                pan_orbit.target_radius + line_delta + pixel_delta,
                pan_orbit.zoom_upper_limit,
                pan_orbit.zoom_lower_limit,
            );
            // If it is pixel-based scrolling, it is added directly to the target
            pan_orbit.radius = pan_orbit.radius.map(|radius| radius + pixel_delta);
            has_moved = true;
        }

        // 3 - Apply rotation constraints

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

        // 4 - Update the camera's transform based on current values

        if let (Some(alpha), Some(beta), Some(radius)) = (pan_orbit.alpha, pan_orbit.beta, pan_orbit.radius) {
            if has_moved
                || pan_orbit.target_alpha != alpha
                || pan_orbit.target_beta != beta
                || pan_orbit.target_radius != radius
                || pan_orbit.target_focus != pan_orbit.focus
                || pan_orbit.force_update
            {
                // Interpolate towards the target radius
                let t = 1.0 - pan_orbit.zoom_smoothness;
                let mut new_radius = radius.lerp(&pan_orbit.target_radius, &t);
                if approx_equal(new_radius, pan_orbit.target_radius) {
                    new_radius = pan_orbit.target_radius
                }
                if let Projection::Orthographic(ref mut p) = *projection {
                    p.scale = new_radius;
                }

                pan_orbit.radius = Some(new_radius);

                // Interpolate towards the target focus
                let t = 1.0 - pan_orbit.pan_smoothness;
                pan_orbit.focus = pan_orbit.focus.lerp(pan_orbit.target_focus, t);

                // Interpolate towards the target rotation
                let t = 1.0 - pan_orbit.orbit_smoothness;
                let mut new_alpha = alpha.lerp(&pan_orbit.target_alpha, &t);
                let mut new_beta = beta.lerp(&pan_orbit.target_beta, &t);

                // If we're super close, then just snap to target rotation to save cycles
                if approx_equal(new_alpha, pan_orbit.target_alpha) {
                    new_alpha = pan_orbit.target_alpha;
                }
                if approx_equal(new_beta, pan_orbit.target_beta) {
                    new_beta = pan_orbit.target_beta;
                }

                util::update_orbit_transform(new_alpha, new_beta, &pan_orbit, &mut transform);

                // Update current alpha and beta values
                pan_orbit.alpha = Some(new_alpha);
                pan_orbit.beta = Some(new_beta);

                if pan_orbit.force_update {
                    pan_orbit.force_update = false;
                }
            }
        }
    }
}
