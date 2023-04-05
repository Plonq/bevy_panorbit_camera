use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::f32::consts::{PI, TAU};

pub struct OrbitCameraPlugin;

impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RollingMouseMovement(vec![]))
            .add_system(pan_orbit_camera);
    }
}

// Holds the last X mouse movement deltas
#[derive(Resource)]
struct RollingMouseMovement(Vec<Vec2>);

/// Tags an entity as capable of panning and orbiting.
/// The entity must have `Transform` and `Projection` components (these are added automatically if
/// you use `Camera3dBundle`).
#[derive(Component)]
pub struct PanOrbitCamera {
    /// The point to orbit around. Automatically updated when panning the camera
    pub focus: Vec3,
    /// The radius of the orbit, or the distance from the `focus` point.
    /// For orthographic projection, this is ignored, and the projection's scale is used instead.
    /// Automatically updated when zooming in and out (for perspective projection).
    pub radius: f32,
    /// Rotation in radians around the global Y axis. Updated automatically.
    pub alpha: f32,
    /// Rotation in radians around the local X axis (i.e. applied after the alpha rotation is applied). Updated automatically.
    pub beta: f32,
    /// The sensitivity of the orbiting motion. Defaults to `1.0`.
    pub orbit_sensitivity: f32,
    /// The sensitivity of the panning motion. Defaults to `1.0`.
    pub pan_sensitivity: f32,
    /// The sensitivity of moving the camera closer or further way using the scroll wheel. Defaults to `1.0`.
    pub zoom_sensitivity: f32,
    /// How smooth the orbital movement is. Should be a value between `0.0` and `0.9`, where `0.0`
    /// means no smoothness, and `0.9` is very smooth.
    /// Defaults to `0.8`
    pub orbit_smoothness: f32,
    /// Button used to orbit the camera. Defaults to <mouse>Left</mouse>.
    pub button_orbit: MouseButton,
    /// Button used to pan the camera. Defaults to <mouse>Right</mouse>.
    pub button_pan: MouseButton,
    pub modifier_orbit: Option<KeyCode>,
    pub modifier_pan: Option<KeyCode>,
    /// Whether the camera is currently upside down. Updated automatically.
    pub is_upside_down: bool,
    /// Whether to allow the camera to go upside down.
    pub allow_upside_down: bool,
    /// If `false`, disable control of the camera. Defaults to `true`.
    pub enabled: bool,
    /// Whether the initial camera translation has been set based on `focus`, `alpha`, `beta`, and `radius`.
    /// If set to `false`, the camera's transform will be updated in the next tick even if there is no user input.
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
            orbit_smoothness: 0.8,
            button_orbit: MouseButton::Left,
            button_pan: MouseButton::Right,
            modifier_orbit: None,
            modifier_pan: None,
            enabled: true,
            alpha: 0.0,
            beta: 0.0,
            initialized: false,
        }
    }
}

impl PanOrbitCamera {
    /// Creates a `PanOrbitCamera` from a translation and focus point. Values of `alpha`, `beta`,
    /// and `radius` will be automatically calculated.
    pub fn from_translation(translation: Vec3, focus: Vec3) -> Self {
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

/// Pan the camera with middle mouse click, zoom with scroll wheel, orbit with right mouse click.
fn pan_orbit_camera(
    windows_query: Query<&Window, With<PrimaryWindow>>,
    mouse_input: Res<Input<MouseButton>>,
    key_input: Res<Input<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut scroll_events: EventReader<MouseWheel>,
    mut camera_query: Query<(&mut PanOrbitCamera, &mut Transform, &mut Projection)>,
    mut rolling_movement: ResMut<RollingMouseMovement>,
) {
    for (mut pan_orbit, mut transform, mut projection) in camera_query.iter_mut() {
        let mut pan = Vec2::ZERO;
        let mut rotation_move = Vec2::ZERO;
        let mut scroll = 0.0;
        let mut orbit_button_changed = false;

        if pan_orbit.enabled {
            if orbit_pressed(&pan_orbit, &mouse_input, &key_input) {
                let mut motion = Vec2::ZERO;
                for ev in mouse_motion_events.iter() {
                    motion += ev.delta * pan_orbit.orbit_sensitivity;
                }
                rotation_move += motion;
                // Record last 3 motions so we can use for inertia
                rolling_movement.0.insert(0, motion);
                rolling_movement.0.truncate(3);
            } else if pan_pressed(&pan_orbit, &mouse_input, &key_input) {
                // Pan only if we're not rotating at the moment
                for ev in mouse_motion_events.iter() {
                    pan += ev.delta * pan_orbit.pan_sensitivity;
                }
            }

            for ev in scroll_events.iter() {
                scroll +=
                    ev.y * match ev.unit {
                        MouseScrollUnit::Line => 1.0,
                        MouseScrollUnit::Pixel => 0.005,
                    } * pan_orbit.zoom_sensitivity;
            }

            if orbit_just_pressed_or_released(&pan_orbit, &mouse_input, &key_input) {
                orbit_button_changed = true;
            }
        }

        if orbit_button_changed {
            // Only check for upside down when orbiting started or ended this frame,
            // so we don't reverse the horizontal direction while the user is still dragging
            pan_orbit.is_upside_down = pan_orbit.beta < -PI / 2.0 || pan_orbit.beta > PI / 2.0;
        }

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
            pan_orbit.alpha -= delta_x;
            pan_orbit.beta += delta_y;
            // Ensure values between 0 and TAU (one full rotation)
            pan_orbit.alpha %= TAU;
            pan_orbit.beta %= TAU;

            if !pan_orbit.allow_upside_down {
                if pan_orbit.beta < -PI / 2.0 {
                    pan_orbit.beta = -PI / 2.0;
                }
                if pan_orbit.beta > PI / 2.0 {
                    pan_orbit.beta = PI / 2.0;
                }
            }
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
        }

        // Calculate target location based on alpha/beta
        let mut rotation = Quat::from_rotation_y(pan_orbit.alpha);
        rotation *= Quat::from_rotation_x(-pan_orbit.beta);

        // Return early if there's nothing to do
        if transform.rotation == rotation {
            return;
        }

        if !pan_orbit.initialized {
            // If not initialized, snap to correct rotation
            transform.rotation = rotation;
            pan_orbit.initialized = true;
        } else if transform.rotation.angle_between(rotation) < 0.01 {
            // If we're very close to the target rotation, snap to it so we aren't getting infinitely
            // closer forever
            transform.rotation = rotation;
        } else {
            // Otherwise, slerp there for smoothing effect, unless that's disabled
            let smoothness = f32::max(f32::min(pan_orbit.orbit_smoothness, 0.9), 0.0);
            transform.rotation = transform.rotation.lerp(rotation, 1.0 - smoothness);
        }

        // Update the translation of the camera so we are always rotating 'around'
        // (orbiting) rather than rotating in place
        let rot_matrix = Mat3::from_quat(transform.rotation);
        transform.translation =
            pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));

        // Prevent camera roll due to lerping the rotation
        transform.look_at(pan_orbit.focus, Vec3::Y);
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
        && mouse_input.just_pressed(pan_orbit.button_orbit);

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
