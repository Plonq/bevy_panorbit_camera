use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

use crate::{ActiveCameraData, PanOrbitCamera};

#[derive(Resource, Default, Debug)]
pub struct MouseKeyTracker {
    pub orbit: Vec2,
    pub pan: Vec2,
    pub scroll_line: f32,
    pub scroll_pixel: f32,
    pub orbit_button_changed: bool,
}

pub fn mouse_key_tracker(
    mut camera_movement: ResMut<MouseKeyTracker>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut scroll_events: EventReader<MouseWheel>,
    active_cam: Res<ActiveCameraData>,
    orbit_cameras: Query<&PanOrbitCamera>,
) {
    if let Some(active_entity) = active_cam.entity {
        if let Ok(pan_orbit) = orbit_cameras.get(active_entity) {
            let mut orbit = Vec2::ZERO;
            let mut pan = Vec2::ZERO;
            let mut scroll_line = 0.0;
            let mut scroll_pixel = 0.0;
            let mut orbit_button_changed = false;

            // Collect input deltas
            let mouse_delta = mouse_motion.read().map(|event| event.delta).sum::<Vec2>();
            let (scroll_line_delta, scroll_pixel_delta) = scroll_events
                .read()
                .map(|event| match event.unit {
                    MouseScrollUnit::Line => (event.y, 0.0),
                    MouseScrollUnit::Pixel => (0.0, event.y * 0.005),
                })
                .fold((0.0, 0.0), |acc, item| (acc.0 + item.0, acc.1 + item.1));

            // Orbit and pan
            if orbit_pressed(pan_orbit, &mouse_input, &key_input) {
                orbit += mouse_delta;
            } else if pan_pressed(pan_orbit, &mouse_input, &key_input) {
                // Pan only if we're not rotating at the moment
                pan += mouse_delta;
            }

            // Zoom
            scroll_line += scroll_line_delta;
            scroll_pixel += scroll_pixel_delta;

            // Other
            if orbit_just_pressed(pan_orbit, &mouse_input, &key_input)
                || orbit_just_released(pan_orbit, &mouse_input, &key_input)
            {
                orbit_button_changed = true;
            }

            camera_movement.orbit = orbit;
            camera_movement.pan = pan;
            camera_movement.scroll_line = scroll_line;
            camera_movement.scroll_pixel = scroll_pixel;
            camera_movement.orbit_button_changed = orbit_button_changed;
        }
    }
}

pub fn orbit_pressed(
    pan_orbit: &PanOrbitCamera,
    mouse_input: &Res<ButtonInput<MouseButton>>,
    key_input: &Res<ButtonInput<KeyCode>>,
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

pub fn orbit_just_pressed(
    pan_orbit: &PanOrbitCamera,
    mouse_input: &Res<ButtonInput<MouseButton>>,
    key_input: &Res<ButtonInput<KeyCode>>,
) -> bool {
    let just_pressed = pan_orbit
        .modifier_orbit
        .map_or(true, |modifier| key_input.pressed(modifier))
        && (mouse_input.just_pressed(pan_orbit.button_orbit));

    just_pressed
        && pan_orbit
            .modifier_pan
            .map_or(true, |modifier| !key_input.pressed(modifier))
}

pub fn orbit_just_released(
    pan_orbit: &PanOrbitCamera,
    mouse_input: &Res<ButtonInput<MouseButton>>,
    key_input: &Res<ButtonInput<KeyCode>>,
) -> bool {
    let just_released = pan_orbit
        .modifier_orbit
        .map_or(true, |modifier| key_input.pressed(modifier))
        && (mouse_input.just_released(pan_orbit.button_orbit));

    just_released
        && pan_orbit
            .modifier_pan
            .map_or(true, |modifier| !key_input.pressed(modifier))
}

pub fn pan_pressed(
    pan_orbit: &PanOrbitCamera,
    mouse_input: &Res<ButtonInput<MouseButton>>,
    key_input: &Res<ButtonInput<KeyCode>>,
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

pub fn pan_just_pressed(
    pan_orbit: &PanOrbitCamera,
    mouse_input: &Res<ButtonInput<MouseButton>>,
    key_input: &Res<ButtonInput<KeyCode>>,
) -> bool {
    let just_pressed = pan_orbit
        .modifier_pan
        .map_or(true, |modifier| key_input.pressed(modifier))
        && (mouse_input.just_pressed(pan_orbit.button_pan));

    just_pressed
        && pan_orbit
            .modifier_orbit
            .map_or(true, |modifier| !key_input.pressed(modifier))
}
