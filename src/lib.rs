#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use std::f32::consts::PI;

use bevy::input::gestures::PinchGesture;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::render::camera::{CameraUpdateSystem, RenderTarget};
use bevy::transform::TransformSystem;
use bevy::window::{PrimaryWindow, WindowRef};
#[cfg(feature = "bevy_egui")]
use bevy_egui::EguiPreUpdateSet;

#[cfg(feature = "bevy_egui")]
pub use crate::egui::{EguiFocusIncludesHover, EguiWantsFocus};
use crate::input::{mouse_key_tracker, MouseKeyTracker};
pub use crate::touch::TouchControls;
use crate::touch::{touch_tracker, TouchGestures, TouchTracker};
use crate::traits::OptionalClamp;

#[cfg(feature = "bevy_egui")]
mod egui;
mod input;
mod touch;
mod traits;
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
        app.init_resource::<ActiveCameraData>()
            .init_resource::<MouseKeyTracker>()
            .init_resource::<TouchTracker>()
            .add_systems(
                PostUpdate,
                (
                    (
                        active_viewport_data
                            .run_if(|active_cam: Res<ActiveCameraData>| !active_cam.manual),
                        mouse_key_tracker,
                        touch_tracker,
                    ),
                    pan_orbit_camera,
                )
                    .chain()
                    .in_set(PanOrbitCameraSystemSet)
                    .before(TransformSystem::TransformPropagate)
                    .before(CameraUpdateSystem),
            );

        #[cfg(feature = "bevy_egui")]
        {
            app.init_resource::<EguiWantsFocus>()
                .init_resource::<EguiFocusIncludesHover>()
                .add_systems(
                    PostUpdate,
                    egui::check_egui_wants_focus
                        .after(EguiPreUpdateSet::InitContexts)
                        .before(PanOrbitCameraSystemSet),
                );
        }
    }
}

/// Base system set to allow ordering of `PanOrbitCamera`
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PanOrbitCameraSystemSet;

/// Tags an entity as capable of panning and orbiting, and provides a way to configure the
/// camera's behaviour and controls.
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
///             Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
///             PanOrbitCamera::default(),
///         ));
///  }
/// ```
#[derive(Component, Reflect, Copy, Clone, Debug, PartialEq)]
#[require(Camera3d)]
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
    /// If both `yaw` and `pitch` are `0.0`, then the camera will be looking forward, i.e. in
    /// the `Vec3::NEG_Z` direction, with up being `Vec3::Y`.
    /// If set to `None`, it will be calculated from the camera's current position during
    /// initialization.
    /// You should not update this after initialization - use `target_yaw` instead.
    /// Defaults to `None`.
    pub yaw: Option<f32>,
    /// Rotation in radians around the local X axis (latitudinal). Updated automatically.
    /// If both `yaw` and `pitch` are `0.0`, then the camera will be looking forward, i.e. in
    /// the `Vec3::NEG_Z` direction, with up being `Vec3::Y`.
    /// If set to `None`, it will be calculated from the camera's current position during
    /// initialization.
    /// You should not update this after initialization - use `target_pitch` instead.
    /// Defaults to `None`.
    pub pitch: Option<f32>,
    /// The target focus point. The camera will smoothly transition to this value. Updated
    /// automatically, but you can also update it manually to control the camera independently of
    /// the mouse controls, e.g. with the keyboard.
    /// Defaults to `Vec3::ZERO`.
    pub target_focus: Vec3,
    /// The target yaw value. The camera will smoothly transition to this value. Updated
    /// automatically, but you can also update it manually to control the camera independently of
    /// the mouse controls, e.g. with the keyboard.
    /// Defaults to `0.0`.
    pub target_yaw: f32,
    /// The target pitch value. The camera will smoothly transition to this value Updated
    /// automatically, but you can also update it manually to control the camera independently of
    /// the mouse controls, e.g. with the keyboard.
    /// Defaults to `0.0`.
    pub target_pitch: f32,
    /// The target radius value. The camera will smoothly transition to this value. Updated
    /// automatically, but you can also update it manually to control the camera independently of
    /// the mouse controls, e.g. with the keyboard.
    /// Defaults to `1.0`.
    pub target_radius: f32,
    /// Upper limit on the `yaw` value, in radians. Use this to restrict the maximum rotation
    /// around the global Y axis.
    /// Defaults to `None`.
    pub yaw_upper_limit: Option<f32>,
    /// Lower limit on the `yaw` value, in radians. Use this to restrict the maximum rotation
    /// around the global Y axis.
    /// Defaults to `None`.
    pub yaw_lower_limit: Option<f32>,
    /// Upper limit on the `pitch` value, in radians. Use this to restrict the maximum rotation
    /// around the local X axis.
    /// Defaults to `None`.
    pub pitch_upper_limit: Option<f32>,
    /// Lower limit on the `pitch` value, in radians. Use this to restrict the maximum rotation
    /// around the local X axis.
    /// Defaults to `None`.
    pub pitch_lower_limit: Option<f32>,
    /// The origin for a shape to restrict the cameras `focus` position.
    /// Defaults to `Vec3::ZERO`.
    pub focus_bounds_origin: Vec3,
    /// The shape (Sphere or Cuboid) that the `focus` is restricted by. Centered on the
    /// `focus_bounds_origin`.
    /// Defaults to `None`.
    pub focus_bounds_shape: Option<FocusBoundsShape>,
    /// Upper limit on the zoom. This applies to `radius`, in the case of using a perspective
    /// camera, or the projection's scale in the case of using an orthographic camera.
    /// Defaults to `None`.
    pub zoom_upper_limit: Option<f32>,
    /// Lower limit on the zoom. This applies to `radius`, in the case of using a perspective
    /// camera, or the projection's scale in the case of using an orthographic camera.
    /// Should always be >0 otherwise you'll get stuck at 0.
    /// Defaults to `0.05`.
    pub zoom_lower_limit: f32,
    /// The sensitivity of the orbiting motion. A value of `0.0` disables orbiting.
    /// Defaults to `1.0`.
    pub orbit_sensitivity: f32,
    /// How much smoothing is applied to the orbit motion. A value of `0.0` disables smoothing,
    /// so there's a 1:1 mapping of input to camera position. A value of `1.0` is infinite
    /// smoothing.
    /// Defaults to `0.8`.
    pub orbit_smoothness: f32,
    /// The sensitivity of the panning motion. A value of `0.0` disables panning.
    /// Defaults to `1.0`.
    pub pan_sensitivity: f32,
    /// How much smoothing is applied to the panning motion. A value of `0.0` disables smoothing,
    /// so there's a 1:1 mapping of input to camera position. A value of `1.0` is infinite
    /// smoothing.
    /// Defaults to `0.6`.
    pub pan_smoothness: f32,
    /// The sensitivity of moving the camera closer or further way using the scroll wheel.
    /// A value of `0.0` disables zooming.
    /// Defaults to `1.0`.
    pub zoom_sensitivity: f32,
    /// How much smoothing is applied to the zoom motion. A value of `0.0` disables smoothing,
    /// so there's a 1:1 mapping of input to camera position. A value of `1.0` is infinite
    /// smoothing.
    /// Defaults to `0.8`.
    /// Note that this setting does not apply to pixel-based scroll events, as they are typically
    /// already smooth. It only applies to line-based scroll events.
    pub zoom_smoothness: f32,
    /// Button used to orbit the camera.
    /// Defaults to `Button::Left`.
    pub button_orbit: MouseButton,
    /// Button used to pan the camera.
    /// Defaults to `Button::Right`.
    pub button_pan: MouseButton,
    /// Key that must be pressed for `button_orbit` to work.
    /// Defaults to `None` (no modifier).
    pub modifier_orbit: Option<KeyCode>,
    /// Key that must be pressed for `button_pan` to work.
    /// Defaults to `None` (no modifier).
    pub modifier_pan: Option<KeyCode>,
    /// Whether touch controls are enabled.
    /// Defaults to `true`.
    pub touch_enabled: bool,
    /// The control scheme for touch inputs.
    /// Defaults to `TouchControls::OneFingerOrbit`.
    pub touch_controls: TouchControls,
    /// The behavior for trackpad inputs.
    /// Defaults to `TrackpadBehavior::DefaultZoom`.
    /// To enable orbit behavior similar to Blender, change this to TrackpadBehavior::BlenderLike.
    /// For `BlenderLike` panning, add `ShiftLeft` to the `modifier_pan` field.
    /// For `BlenderLike` zooming, add `ControlLeft` in `modifier_zoom` field.
    pub trackpad_behavior: TrackpadBehavior,
    /// Whether to enable pinch-to-zoom functionality on trackpads.
    /// Defaults to `false`.
    pub trackpad_pinch_to_zoom_enabled: bool,
    /// The sensitivity of trackpad gestures when using `BlenderLike` behavior. A value of `0.0`
    /// effectively disables trackpad orbit/pan functionality. This applies to both orbit and pan.
    /// operations when using a trackpad with the `BlenderLike` behavior mode.
    /// Defaults to `1.0`.
    pub trackpad_sensitivity: f32,
    /// Whether to reverse the zoom direction.
    /// Defaults to `false`.
    pub reversed_zoom: bool,
    /// Whether the camera is currently upside down. Updated automatically.
    /// This is used to determine which way to orbit, because it's more intuitive to reverse the
    /// orbit direction when upside down.
    /// Should not be set manually unless you know what you're doing.
    /// Defaults to `false` (but will be updated immediately).
    pub is_upside_down: bool,
    /// Whether to allow the camera to go upside down.
    /// Defaults to `false`.
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
    /// Axis order definition. This can be used to e.g. define a different default
    /// up direction. The default up is Y, but if you want the camera rotated.
    /// The axis can be switched. Default is [Vec3::X, Vec3::Y, Vec3::Z]
    pub axis: [Vec3; 3],
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
            orbit_smoothness: 0.1,
            pan_sensitivity: 1.0,
            pan_smoothness: 0.02,
            zoom_sensitivity: 1.0,
            zoom_smoothness: 0.1,
            button_orbit: MouseButton::Left,
            button_pan: MouseButton::Right,
            modifier_orbit: None,
            modifier_pan: None,
            touch_enabled: true,
            touch_controls: TouchControls::OneFingerOrbit,
            trackpad_behavior: TrackpadBehavior::Default,
            trackpad_pinch_to_zoom_enabled: false,
            trackpad_sensitivity: 1.0,
            reversed_zoom: false,
            enabled: true,
            yaw: None,
            pitch: None,
            target_yaw: 0.0,
            target_pitch: 0.0,
            target_radius: 1.0,
            initialized: false,
            yaw_upper_limit: None,
            yaw_lower_limit: None,
            pitch_upper_limit: None,
            pitch_lower_limit: None,
            focus_bounds_origin: Vec3::ZERO,
            focus_bounds_shape: None,
            zoom_upper_limit: None,
            zoom_lower_limit: 0.05,
            force_update: false,
            axis: [Vec3::X, Vec3::Y, Vec3::Z],
        }
    }
}

/// Tracks which `PanOrbitCamera` is active (should handle input events), along with the window
/// and viewport dimensions, which are used for scaling mouse motion.
/// `PanOrbitCameraPlugin` manages this resource automatically, in order to support multiple
/// viewports/windows. However, if this doesn't work for you, you can take over and manage it
/// yourself, e.g. when you want to control a camera that is rendering to a texture.
#[derive(Resource, Default, Debug, PartialEq)]
pub struct ActiveCameraData {
    /// ID of the entity with `PanOrbitCamera` that will handle user input. In other words, this
    /// is the camera that will move when you orbit/pan/zoom.
    pub entity: Option<Entity>,
    /// The viewport size. This is only used to scale the panning mouse motion. I recommend setting
    /// this to the actual render target dimensions (e.g. the image or viewport), and changing
    /// `PanOrbitCamera::pan_sensitivity` to adjust the sensitivity if required.
    pub viewport_size: Option<Vec2>,
    /// The size of the window. This is only used to scale the orbit mouse motion. I recommend
    /// setting this to actual dimensions of the window that you want to control the camera from,
    /// and changing `PanOrbitCamera::orbit_sensitivity` to adjust the sensitivity if required.
    pub window_size: Option<Vec2>,
    /// Indicates to `PanOrbitCameraPlugin` that it should not update/overwrite this resource.
    /// If you are manually updating this resource you should set this to `true`.
    /// Note that setting this to `true` will effectively break multiple viewport/window support
    /// unless you manually reimplement it.
    pub manual: bool,
}

/// The shape to restrict the camera's focus inside.
#[derive(Clone, PartialEq, Debug, Reflect, Copy)]
pub enum FocusBoundsShape {
    /// Limit the camera's focus to a sphere centered on `focus_bounds_origin`.
    Sphere(Sphere),
    /// Limit the camera's focus to a cuboid centered on `focus_bounds_origin`.
    Cuboid(Cuboid),
}

impl From<Sphere> for FocusBoundsShape {
    fn from(value: Sphere) -> Self {
        Self::Sphere(value)
    }
}

impl From<Cuboid> for FocusBoundsShape {
    fn from(value: Cuboid) -> Self {
        Self::Cuboid(value)
    }
}

/// Allows for changing the `TrackpadBehavior` from default to the way it works in Blender.
/// In Blender the trackpad orbits when scrolling. If you hold down the `ShiftLeft`, it Pans and
/// holding down `ControlLeft` will Zoom.
#[derive(Clone, PartialEq, Debug, Reflect, Copy)]
pub enum TrackpadBehavior {
    /// Default touchpad behavior. I.e., no special gesture support, scrolling on the touchpad (vertically) will zoom, as it does with a mouse.
    Default,
    /// Blender-like touchpad behavior. Scrolling on the touchpad will orbit, and you can pinch to zoom. Optionally you can pan, or switch scroll to zoom, by holding down a modifier.
    BlenderLike {
        /// Modifier key that enables panning while scrolling
        modifier_pan: Option<KeyCode>,

        /// Modifier key that enables panning while scrolling
        modifier_zoom: Option<KeyCode>,
    },
}

impl TrackpadBehavior {
    /// Creates a `BlenderLike` variant with default modifiers (Shift for pan, Ctrl for zoom)
    pub fn blender_default() -> Self {
        Self::BlenderLike {
            modifier_pan: Some(KeyCode::ShiftLeft),
            modifier_zoom: Some(KeyCode::ControlLeft),
        }
    }
}

/// Gather data about the active viewport, i.e. the viewport the user is interacting with.
/// Enables multiple viewports/windows.
#[allow(clippy::too_many_arguments)]
fn active_viewport_data(
    mut active_cam: ResMut<ActiveCameraData>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    pinch_events: EventReader<PinchGesture>,
    scroll_events: EventReader<MouseWheel>,
    touches: Res<Touches>,
    primary_windows: Query<&Window, With<PrimaryWindow>>,
    other_windows: Query<&Window, Without<PrimaryWindow>>,
    orbit_cameras: Query<(Entity, &Camera, &PanOrbitCamera)>,
    #[cfg(feature = "bevy_egui")] egui_wants_focus: Res<EguiWantsFocus>,
) {
    let mut new_resource = ActiveCameraData::default();
    let mut max_cam_order = 0;

    let mut has_input = false;
    for (entity, camera, pan_orbit) in orbit_cameras.iter() {
        let input_just_activated = input::orbit_just_pressed(pan_orbit, &mouse_input, &key_input)
            || input::pan_just_pressed(pan_orbit, &mouse_input, &key_input)
            || !pinch_events.is_empty()
            || !scroll_events.is_empty()
            || (touches.iter_just_pressed().count() > 0
                && touches.iter_just_pressed().count() == touches.iter().count());

        if input_just_activated {
            has_input = true;
            #[allow(unused_mut, unused_assignments)]
            let mut should_get_input = true;
            #[cfg(feature = "bevy_egui")]
            {
                should_get_input = !egui_wants_focus.prev && !egui_wants_focus.curr;
            }
            if should_get_input {
                // First check if cursor is in the same window as this camera
                if let RenderTarget::Window(win_ref) = camera.target {
                    let Some(window) = (match win_ref {
                        WindowRef::Primary => primary_windows.single().ok(),
                        WindowRef::Entity(entity) => other_windows.get(entity).ok(),
                    }) else {
                        // Window does not exist - maybe it was closed and the camera not cleaned up
                        continue;
                    };

                    // Is the cursor/touch in this window?
                    // Note: there's a bug in winit that causes `window.cursor_position()` to return
                    // a `Some` value even if the cursor is not in this window, in very specific cases.
                    // See: https://github.com/Plonq/bevy_panorbit_camera/issues/22
                    if let Some(input_position) = window.cursor_position().or(touches
                        .iter_just_pressed()
                        .collect::<Vec<_>>()
                        .first()
                        .map(|touch| touch.position()))
                    {
                        // Now check if cursor is within this camera's viewport
                        if let Some(Rect { min, max }) = camera.logical_viewport_rect() {
                            // Window coordinates have Y starting at the bottom, so we need to reverse
                            // the y component before comparing with the viewport rect
                            let cursor_in_vp = input_position.x > min.x
                                && input_position.x < max.x
                                && input_position.y > min.y
                                && input_position.y < max.y;

                            // Only set if camera order is higher. This may overwrite a previous value
                            // in the case the viewport is overlapping another viewport.
                            if cursor_in_vp && camera.order >= max_cam_order {
                                new_resource = ActiveCameraData {
                                    entity: Some(entity),
                                    viewport_size: camera.logical_viewport_size(),
                                    window_size: Some(Vec2::new(window.width(), window.height())),
                                    manual: false,
                                };
                                max_cam_order = camera.order;
                            }
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
    mouse_key_tracker: Res<MouseKeyTracker>,
    touch_tracker: Res<TouchTracker>,
    mut orbit_cameras: Query<(Entity, &mut PanOrbitCamera, &mut Transform, &mut Projection)>,
    time: Res<Time>,
) {
    for (entity, mut pan_orbit, mut transform, mut projection) in orbit_cameras.iter_mut() {
        // Closures that apply limits to the yaw, pitch, and zoom values
        let apply_zoom_limits = {
            let zoom_upper_limit = pan_orbit.zoom_upper_limit;
            let zoom_lower_limit = pan_orbit.zoom_lower_limit;
            move |zoom: f32| zoom.clamp_optional(Some(zoom_lower_limit), zoom_upper_limit)
        };

        let apply_yaw_limits = {
            let yaw_upper_limit = pan_orbit.yaw_upper_limit;
            let yaw_lower_limit = pan_orbit.yaw_lower_limit;
            move |yaw: f32| yaw.clamp_optional(yaw_lower_limit, yaw_upper_limit)
        };

        let apply_pitch_limits = {
            let pitch_upper_limit = pan_orbit.pitch_upper_limit;
            let pitch_lower_limit = pan_orbit.pitch_lower_limit;
            move |pitch: f32| pitch.clamp_optional(pitch_lower_limit, pitch_upper_limit)
        };

        let apply_focus_limits = {
            let origin = pan_orbit.focus_bounds_origin;
            let shape = pan_orbit.focus_bounds_shape;

            move |focus: Vec3| {
                let Some(shape) = shape else {
                    return focus;
                };

                match shape {
                    FocusBoundsShape::Cuboid(shape) => shape.closest_point(focus - origin) + origin,
                    FocusBoundsShape::Sphere(shape) => shape.closest_point(focus - origin) + origin,
                }
            }
        };

        if !pan_orbit.initialized {
            // Calculate yaw, pitch, and radius from the camera's position. If user sets all
            // these explicitly, this calculation is wasted, but that's okay since it will only run
            // once on init.
            let (yaw, pitch, radius) = util::calculate_from_translation_and_focus(
                transform.translation,
                pan_orbit.focus,
                pan_orbit.axis,
            );
            let &mut mut yaw = pan_orbit.yaw.get_or_insert(yaw);
            let &mut mut pitch = pan_orbit.pitch.get_or_insert(pitch);
            let &mut mut radius = pan_orbit.radius.get_or_insert(radius);
            let mut focus = pan_orbit.focus;

            // Apply limits
            yaw = apply_yaw_limits(yaw);
            pitch = apply_pitch_limits(pitch);
            radius = apply_zoom_limits(radius);
            focus = apply_focus_limits(focus);

            // Set initial values
            pan_orbit.yaw = Some(yaw);
            pan_orbit.pitch = Some(pitch);
            pan_orbit.radius = Some(radius);
            pan_orbit.target_yaw = yaw;
            pan_orbit.target_pitch = pitch;
            pan_orbit.target_radius = radius;
            pan_orbit.target_focus = focus;

            util::update_orbit_transform(
                yaw,
                pitch,
                radius,
                focus,
                &mut transform,
                &mut projection,
                pan_orbit.axis,
            );

            pan_orbit.initialized = true;
        }

        // 1 - Get Input

        let mut orbit = Vec2::ZERO;
        let mut pan = Vec2::ZERO;
        let mut scroll_line = 0.0;
        let mut scroll_pixel = 0.0;
        let mut orbit_button_changed = false;

        // The reason we only skip getting input if the camera is inactive/disabled is because
        // it might still be moving (lerping towards target values) when the user is not
        // actively controlling it.
        if pan_orbit.enabled && active_cam.entity == Some(entity) {
            let zoom_direction = match pan_orbit.reversed_zoom {
                true => -1.0,
                false => 1.0,
            };

            orbit = mouse_key_tracker.orbit * pan_orbit.orbit_sensitivity;
            pan = mouse_key_tracker.pan * pan_orbit.pan_sensitivity;
            scroll_line =
                mouse_key_tracker.scroll_line * zoom_direction * pan_orbit.zoom_sensitivity;
            scroll_pixel =
                mouse_key_tracker.scroll_pixel * zoom_direction * pan_orbit.zoom_sensitivity;
            orbit_button_changed = mouse_key_tracker.orbit_button_changed;

            if pan_orbit.touch_enabled {
                let (touch_orbit, touch_pan, touch_zoom_pixel) = match pan_orbit.touch_controls {
                    TouchControls::OneFingerOrbit => match touch_tracker.get_touch_gestures() {
                        TouchGestures::None => (Vec2::ZERO, Vec2::ZERO, 0.0),
                        TouchGestures::OneFinger(one_finger_gestures) => {
                            (one_finger_gestures.motion, Vec2::ZERO, 0.0)
                        }
                        TouchGestures::TwoFinger(two_finger_gestures) => (
                            Vec2::ZERO,
                            two_finger_gestures.motion,
                            two_finger_gestures.pinch * 0.015,
                        ),
                    },
                    TouchControls::TwoFingerOrbit => match touch_tracker.get_touch_gestures() {
                        TouchGestures::None => (Vec2::ZERO, Vec2::ZERO, 0.0),
                        TouchGestures::OneFinger(one_finger_gestures) => {
                            (Vec2::ZERO, one_finger_gestures.motion, 0.0)
                        }
                        TouchGestures::TwoFinger(two_finger_gestures) => (
                            two_finger_gestures.motion,
                            Vec2::ZERO,
                            two_finger_gestures.pinch * 0.015,
                        ),
                    },
                };

                orbit += touch_orbit * pan_orbit.orbit_sensitivity;
                pan += touch_pan * pan_orbit.pan_sensitivity;
                scroll_pixel += touch_zoom_pixel * zoom_direction * pan_orbit.zoom_sensitivity;
            }
        }

        // 2 - Process input into target yaw/pitch, or focus, radius

        // Only check for upside down when orbiting started or ended this frame,
        // so we don't reverse the yaw direction while the user is still dragging
        if orbit_button_changed {
            let world_up = pan_orbit.axis[1];
            pan_orbit.is_upside_down = transform.up().dot(world_up) < 0.0;
        }

        let mut has_moved = false;
        if orbit.length_squared() > 0.0 {
            // Use window size for rotation otherwise the sensitivity
            // is far too high for small viewports
            if let Some(win_size) = active_cam.window_size {
                let delta_x = {
                    let delta = orbit.x / win_size.x * PI * 2.0;
                    if pan_orbit.is_upside_down {
                        -delta
                    } else {
                        delta
                    }
                };
                let delta_y = orbit.y / win_size.y * PI;
                pan_orbit.target_yaw -= delta_x;
                pan_orbit.target_pitch += delta_y;

                has_moved = true;
            }
        }
        if pan.length_squared() > 0.0 {
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
                    Projection::Custom(_) => todo!(),
                }
                // Translate by local axes
                let right = transform.rotation * pan_orbit.axis[0] * -pan.x;
                let up = transform.rotation * pan_orbit.axis[1] * pan.y;
                let translation = (right + up) * multiplier;
                pan_orbit.target_focus += translation;
                has_moved = true;
            }
        }
        if (scroll_line + scroll_pixel).abs() > 0.0 {
            // Calculate the impact of scrolling on the reference value
            let line_delta = -scroll_line * (pan_orbit.target_radius) * 0.2;
            let pixel_delta = -scroll_pixel * (pan_orbit.target_radius) * 0.2;

            // Update the target value
            pan_orbit.target_radius += line_delta + pixel_delta;

            // If it is pixel-based scrolling, add it directly to the current value
            pan_orbit.radius = pan_orbit
                .radius
                .map(|value| apply_zoom_limits(value + pixel_delta));

            has_moved = true;
        }

        // 3 - Apply constraints

        pan_orbit.target_yaw = apply_yaw_limits(pan_orbit.target_yaw);
        pan_orbit.target_pitch = apply_pitch_limits(pan_orbit.target_pitch);
        pan_orbit.target_radius = apply_zoom_limits(pan_orbit.target_radius);
        pan_orbit.target_focus = apply_focus_limits(pan_orbit.target_focus);

        if !pan_orbit.allow_upside_down {
            pan_orbit.target_pitch = pan_orbit.target_pitch.clamp(-PI / 2.0, PI / 2.0);
        }

        // 4 - Update the camera's transform based on current values

        if let (Some(yaw), Some(pitch), Some(radius)) =
            (pan_orbit.yaw, pan_orbit.pitch, pan_orbit.radius)
        {
            if has_moved
                // For smoothed values, we must check whether current value is different from target
                // value. If we only checked whether the values were non-zero this frame, then
                // the camera would instantly stop moving as soon as you stopped moving it, instead
                // of smoothly stopping
                || pan_orbit.target_yaw != yaw
                || pan_orbit.target_pitch != pitch
                || pan_orbit.target_radius != radius
                || pan_orbit.target_focus != pan_orbit.focus
                || pan_orbit.force_update
            {
                // Interpolate towards the target values
                let new_yaw = util::lerp_and_snap_f32(
                    yaw,
                    pan_orbit.target_yaw,
                    pan_orbit.orbit_smoothness,
                    time.delta_secs(),
                );
                let new_pitch = util::lerp_and_snap_f32(
                    pitch,
                    pan_orbit.target_pitch,
                    pan_orbit.orbit_smoothness,
                    time.delta_secs(),
                );
                let new_radius = util::lerp_and_snap_f32(
                    radius,
                    pan_orbit.target_radius,
                    pan_orbit.zoom_smoothness,
                    time.delta_secs(),
                );
                let new_focus = util::lerp_and_snap_vec3(
                    pan_orbit.focus,
                    pan_orbit.target_focus,
                    pan_orbit.pan_smoothness,
                    time.delta_secs(),
                );

                util::update_orbit_transform(
                    new_yaw,
                    new_pitch,
                    new_radius,
                    new_focus,
                    &mut transform,
                    &mut projection,
                    pan_orbit.axis,
                );

                // Update the current values
                pan_orbit.yaw = Some(new_yaw);
                pan_orbit.pitch = Some(new_pitch);
                pan_orbit.radius = Some(new_radius);
                pan_orbit.focus = new_focus;
                pan_orbit.force_update = false;
            }
        }
    }
}
