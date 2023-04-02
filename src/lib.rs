use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::f32::consts::{PI, TAU};

pub struct OrbitCameraPlugin;

impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(pan_orbit_camera);
    }
}

/// Tags an entity as capable of panning and orbiting.
#[derive(Component)]
pub struct PanOrbitCamera {
    /// The point to orbit around. Automatically updated when panning the camera
    pub focus: Vec3,
    /// The radius of the orbit, or the distance from the `focus` point. Automatically updated when zooming in and out
    pub radius: f32,
    /// Rotation in radians around the global Y axis. Updated automatically
    pub alpha: f32,
    /// Rotation in radians around the local X axis (i.e. applied after the alpha rotation is applied). Updated automatically
    pub beta: f32,
    /// The sensitivity of the orbiting motion. Defaults to `1.0`
    pub orbit_sensitivity: f32,
    /// The sensitivity of the panning motion. Defaults to `1.0`
    pub pan_sensitivity: f32,
    /// The sensitivity of moving the camera closer or further way using the scroll wheel. Defaults to `1.0`
    pub zoom_sensitivity: f32,
    /// The amount of deceleration to apply to the camera's rotation after you let go. Defaults to `1.0`
    pub damping: f32,
    /// Button used to orbit the camera. Defaults to <mouse>Left</mouse>
    pub button_orbit: MouseButton,
    /// Button used to pan the camera. Defaults to <mouse>Right</mouse>
    pub button_pan: MouseButton,
    /// Whether the camera is currently upside down. Automatically updated
    pub is_upside_down: bool,
    /// Whether to allow the camera to go upside down
    pub allow_upside_down: bool,
    /// If `false`, disable control of the camera. Defaults to `true`
    pub enabled: bool,
    /// Whether the initial camera translation has been set based on `focus`, `alpha`, `beta`, and `radius`.
    /// If set to `false`, the camera's position and rotation will be updated in the next tick even if there is no user input
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
            damping: 1.0,
            button_orbit: MouseButton::Left,
            button_pan: MouseButton::Right,
            enabled: true,
            alpha: 0.0,
            beta: 0.0,
            initialized: false,
        }
    }
}

/// Pan the camera with middle mouse click, zoom with scroll wheel, orbit with right mouse click.
fn pan_orbit_camera(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut scroll_events: EventReader<MouseWheel>,
    mouse_input: Res<Input<MouseButton>>,
    mut camera_query: Query<(&mut PanOrbitCamera, &mut Transform, &mut Projection)>,
) {
    for (mut pan_orbit, mut transform, mut projection) in camera_query.iter_mut() {
        if !pan_orbit.enabled {
            return;
        }

        let mut pan = Vec2::ZERO;
        let mut rotation_move = Vec2::ZERO;
        let mut scroll = 0.0;
        let mut orbit_button_changed = false;

        if mouse_input.pressed(pan_orbit.button_orbit) {
            for ev in mouse_motion_events.iter() {
                rotation_move += ev.delta * pan_orbit.orbit_sensitivity;
            }
        } else if mouse_input.pressed(pan_orbit.button_pan) {
            // Pan only if we're not rotating at the moment
            for ev in mouse_motion_events.iter() {
                pan += ev.delta * pan_orbit.pan_sensitivity;
            }
        }

        for ev in scroll_events.iter() {
            scroll +=
                ev.y * match ev.unit {
                    MouseScrollUnit::Line => 1.0,
                    MouseScrollUnit::Pixel => 0.01,
                } * pan_orbit.zoom_sensitivity;
        }

        if mouse_input.just_released(pan_orbit.button_orbit)
            || mouse_input.just_pressed(pan_orbit.button_orbit)
        {
            orbit_button_changed = true;
        }

        if orbit_button_changed {
            // Only check for upside down when orbiting started or ended this frame,
            // so we don't reverse the horizontal direction while the user is still dragging
            pan_orbit.is_upside_down = pan_orbit.beta < -PI / 2.0 || pan_orbit.beta > PI / 2.0;
        }

        let mut has_moved = false;
        if rotation_move.length_squared() > 0.0 {
            has_moved = true;
            let window = get_primary_window_size(&windows);
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
            pan_orbit.beta -= delta_y;
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
            has_moved = true;
            // Make panning distance independent of resolution and FOV,
            let window = get_primary_window_size(&windows);
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
            has_moved = true;
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

        if has_moved || !pan_orbit.initialized {
            // Yaw is in global space (rotate around global y-axis), but pitch is in
            // local space (rotate around the camera's x-axis)
            let mut rotation = Quat::from_rotation_y(pan_orbit.alpha);
            rotation *= Quat::from_rotation_x(pan_orbit.beta);
            transform.rotation = rotation;

            // Update the translation of the camera so we are always rotating 'around'
            // (orbiting) rather than rotating in place
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation =
                pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));

            if !pan_orbit.initialized {
                pan_orbit.initialized = true;
            }
        }
    }
}

fn get_primary_window_size(windows: &Query<&Window, With<PrimaryWindow>>) -> Vec2 {
    let Ok(primary) = windows.get_single() else {
        // No primary window? Dunno how we can be controlling a camera, but let's return ONE
        // so when dividing by this value nothing explodes
        return Vec2::ONE;
    };
    Vec2::new(primary.width(), primary.height())
}
