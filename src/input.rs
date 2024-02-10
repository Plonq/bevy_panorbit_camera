use crate::{ActiveCameraData, PanOrbitCamera};
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::input::touch::Touch;
use bevy::prelude::*;
use std::f32::consts::TAU;

use crate::traits::Midpoint;

#[derive(Resource, Default, Debug)]
pub struct MouseKeyTracker {
    pub orbit: Vec2,
    pub pan: Vec2,
    pub scroll_line: f32,
    pub scroll_pixel: f32,
    pub roll_angle: f32,
    pub orbit_button_changed: bool,
}

pub fn mouse_key_tracker(
    mut camera_movement: ResMut<MouseKeyTracker>,
    mouse_input: Res<Input<MouseButton>>,
    key_input: Res<Input<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut scroll_events: EventReader<MouseWheel>,
    active_cam: Res<ActiveCameraData>,
    orbit_cameras: Query<&PanOrbitCamera>,
) {
    if let Some(active_entity) = active_cam.entity {
        if let Ok(pan_orbit) = orbit_cameras.get(active_entity) {
            let mouse_delta = mouse_motion.read().map(|event| event.delta).sum::<Vec2>();

            let mut orbit = Vec2::ZERO;
            let mut pan = Vec2::ZERO;
            let mut scroll_line = 0.0;
            let mut scroll_pixel = 0.0;
            let mut roll_angle = 0.0;
            let mut orbit_button_changed = false;

            // Orbit and pan
            if orbit_pressed(pan_orbit, &mouse_input, &key_input) {
                orbit += mouse_delta;
            } else if pan_pressed(pan_orbit, &mouse_input, &key_input) {
                // Pan only if we're not rotating at the moment
                pan += mouse_delta;
            }

            // Zoom
            for ev in scroll_events.read() {
                let delta_scroll = ev.y;
                match ev.unit {
                    MouseScrollUnit::Line => {
                        scroll_line += delta_scroll;
                    }
                    MouseScrollUnit::Pixel => {
                        scroll_pixel += delta_scroll * 0.005;
                    }
                };
            }

            // Roll
            let roll_amount = TAU * 0.003;

            if let Some(roll_left_key) = pan_orbit.key_roll_left {
                if key_input.pressed(roll_left_key) {
                    roll_angle -= roll_amount;
                }
            }
            if let Some(roll_right_key) = pan_orbit.key_roll_right {
                if key_input.pressed(roll_right_key) {
                    roll_angle += roll_amount;
                }
            }

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
            camera_movement.roll_angle = roll_angle;
            camera_movement.orbit_button_changed = orbit_button_changed;
        }
    }
}

/// Store current and previous frame touch data
#[derive(Resource, Default, Debug)]
pub struct TouchTracker {
    pub curr_pressed: (Option<Touch>, Option<Touch>),
    pub prev_pressed: (Option<Touch>, Option<Touch>),
}

impl TouchTracker {
    /// Return orbit, pan, and zoom values based on touch data
    pub fn calculate_movement(&self) -> (Vec2, Vec2, f32) {
        let mut orbit = Vec2::ZERO;
        let mut pan = Vec2::ZERO;
        let mut zoom_pixel = 0.0;

        // Only match when curr and prev have same number of touches, for simplicity.
        // I did not notice any adverse behaviour as a result.
        match (self.curr_pressed, self.prev_pressed) {
            ((Some(curr), None), (Some(prev), None)) => {
                let curr_pos = curr.position();
                let prev_pos = prev.position();

                orbit += curr_pos - prev_pos;
            }
            ((Some(curr1), Some(curr2)), (Some(prev1), Some(prev2))) => {
                let curr1_pos = curr1.position();
                let curr2_pos = curr2.position();
                let prev1_pos = prev1.position();
                let prev2_pos = prev2.position();

                let curr_midpoint = curr1_pos.midpoint(curr2_pos);
                let prev_midpoint = prev1_pos.midpoint(prev2_pos);
                pan += curr_midpoint - prev_midpoint;

                let curr_dist = curr1_pos.distance(curr2_pos);
                let prev_dist = prev1_pos.distance(prev2_pos);
                zoom_pixel += (curr_dist - prev_dist) * 0.015;
            }
            _ => {}
        }

        (orbit, pan, zoom_pixel)
    }
}

/// Read touch input and save it in TouchTracker resource for easy consumption by the main system
pub fn touch_tracker(touches: Res<Touches>, mut touch_tracker: ResMut<TouchTracker>) {
    let pressed: Vec<&Touch> = touches.iter().collect();

    match pressed.len() {
        0 => {
            touch_tracker.curr_pressed = (None, None);
            touch_tracker.prev_pressed = (None, None);
        }
        1 => {
            let touch: &Touch = pressed.first().unwrap();
            touch_tracker.prev_pressed = touch_tracker.curr_pressed;
            touch_tracker.curr_pressed = (Some(*touch), None);
        }
        2 => {
            let touch1: &Touch = pressed.first().unwrap();
            let touch2: &Touch = pressed.last().unwrap();
            touch_tracker.prev_pressed = touch_tracker.curr_pressed;
            touch_tracker.curr_pressed = (Some(*touch1), Some(*touch2));
        }
        _ => {}
    }
}

pub fn orbit_pressed(
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

pub fn orbit_just_pressed(
    pan_orbit: &PanOrbitCamera,
    mouse_input: &Res<Input<MouseButton>>,
    key_input: &Res<Input<KeyCode>>,
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
    mouse_input: &Res<Input<MouseButton>>,
    key_input: &Res<Input<KeyCode>>,
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

pub fn pan_just_pressed(
    pan_orbit: &PanOrbitCamera,
    mouse_input: &Res<Input<MouseButton>>,
    key_input: &Res<Input<KeyCode>>,
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
