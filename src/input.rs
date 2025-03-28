use bevy::input::gestures::PinchGesture;
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

use crate::{ActiveCameraData, PanOrbitCamera, TrackpadBehavior};

#[derive(Resource, Default, Debug)]
pub struct MouseKeyTracker {
    pub orbit: Vec2,
    pub pan: Vec2,
    pub scroll_line: f32,
    pub scroll_pixel: f32,
    pub orbit_button_changed: bool,
}

#[allow(clippy::too_many_arguments)]
pub fn mouse_key_tracker(
    mut camera_movement: ResMut<MouseKeyTracker>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut pinch_events: EventReader<PinchGesture>,
    mut scroll_events: EventReader<MouseWheel>,
    active_cam: Res<ActiveCameraData>,
    orbit_cameras: Query<&PanOrbitCamera>,
) {
    let active_entity = match active_cam.entity {
        Some(entity) => entity,
        None => return,
    };

    let pan_orbit = match orbit_cameras.get(active_entity) {
        Ok(camera) => camera,
        Err(_) => return,
    };

    // Collect input deltas
    let mouse_delta = mouse_motion.read().map(|event| event.delta).sum::<Vec2>();

    // Collect scroll events
    let scroll_events_vec: Vec<MouseWheel> = scroll_events.read().cloned().collect();

    // scroll processing needs to account for mouse and trackpad
    // and when it's the trackpad, if we're in BlenderLike mode, we get back trackpad_orbit and trackpad_pan
    // these two values are set to zero if we're in backwards compatible DefaultZoom mode
    let scroll_result = process_scroll_events(&scroll_events_vec, pan_orbit, &key_input);
    // Initialize orbit and pan with trackpad contributions
    let mut orbit = scroll_result.trackpad_orbit;
    let mut pan = scroll_result.trackpad_pan;

    // Handle pinch gestures separately
    // Process pinch events
    let pinch_zoom = process_pinch_events(&mut pinch_events, pan_orbit, &key_input);

    // Handle mouse movement for orbiting and panning
    if orbit_pressed(pan_orbit, &mouse_input, &key_input) {
        orbit += mouse_delta;
    } else if pan_pressed(pan_orbit, &mouse_input, &key_input) {
        pan += mouse_delta;
    }

    // Track button state changes
    let orbit_button_changed = orbit_just_pressed(pan_orbit, &mouse_input, &key_input)
        || orbit_just_released(pan_orbit, &mouse_input, &key_input);

    // Update the movement resource
    camera_movement.orbit = orbit;
    camera_movement.pan = pan;
    camera_movement.scroll_line = scroll_result.scroll_line;
    camera_movement.scroll_pixel = scroll_result.scroll_pixel + pinch_zoom;
    camera_movement.orbit_button_changed = orbit_button_changed;
}

#[derive(Default)]
struct ScrollProcessingResult {
    trackpad_orbit: Vec2,
    trackpad_pan: Vec2,
    scroll_line: f32,
    scroll_pixel: f32,
}

/// mimic how blender _doesn't_ handle pinch gestures when modifiers are pressed
fn process_scroll_events(
    scroll_events: &[MouseWheel],
    pan_orbit: &PanOrbitCamera,
    key_input: &Res<ButtonInput<KeyCode>>,
) -> ScrollProcessingResult {
    match pan_orbit.trackpad_behavior {
        TrackpadBehavior::BlenderLike {
            modifier_pan,
            modifier_zoom,
        } => {
            let is_zoom_modifier_pressed =
                modifier_zoom.is_none_or(|modifier| key_input.pressed(modifier));
            let is_pan_modifier_pressed =
                modifier_pan.is_none_or(|modifier| key_input.pressed(modifier));

            let mut result = ScrollProcessingResult::default();

            for event in scroll_events {
                match event.unit {
                    MouseScrollUnit::Line => {
                        result.scroll_line += event.y;
                    }
                    MouseScrollUnit::Pixel => {
                        if is_zoom_modifier_pressed {
                            result.scroll_pixel += event.y * 0.005;
                        } else if is_pan_modifier_pressed {
                            result.trackpad_pan +=
                                Vec2::new(event.x, event.y) * pan_orbit.trackpad_sensitivity;
                        } else {
                            result.trackpad_orbit +=
                                Vec2::new(event.x, event.y) * pan_orbit.trackpad_sensitivity;
                        }
                    }
                }
            }

            result
        }
        _ => {
            // Default behavior: all scroll events contribute to zoom
            let (scroll_line, scroll_pixel) = scroll_events
                .iter()
                .map(|event| match event.unit {
                    MouseScrollUnit::Line => (event.y, 0.0),
                    MouseScrollUnit::Pixel => (0.0, event.y * 0.005),
                })
                .fold((0.0, 0.0), |acc, item| (acc.0 + item.0, acc.1 + item.1));

            // DefaultZoom behavior - no trackpad involved
            ScrollProcessingResult {
                trackpad_orbit: Vec2::ZERO,
                trackpad_pan: Vec2::ZERO,
                scroll_line,
                scroll_pixel,
            }
        }
    }
}

fn process_pinch_events(
    pinch_events: &mut EventReader<PinchGesture>,
    pan_orbit: &PanOrbitCamera,
    key_input: &Res<ButtonInput<KeyCode>>,
) -> f32 {
    if !pan_orbit.trackpad_pinch_to_zoom_enabled {
        return 0.0;
    }

    // Check if no modifiers are pressed (including BlenderLike modifiers if applicable)
    let no_modifiers_pressed = match pan_orbit.trackpad_behavior {
        TrackpadBehavior::BlenderLike {
            modifier_pan,
            modifier_zoom,
        } => {
            // Check regular modifiers and BlenderLike modifiers
            pan_orbit
                .modifier_orbit
                .is_none_or(|modifier| !key_input.pressed(modifier))
                && pan_orbit
                    .modifier_pan
                    .is_none_or(|modifier| !key_input.pressed(modifier))
                && modifier_pan.is_none_or(|modifier| !key_input.pressed(modifier))
                && modifier_zoom.is_none_or(|modifier| !key_input.pressed(modifier))
        }
        _ => {
            // Just check regular modifiers
            pan_orbit
                .modifier_orbit
                .is_none_or(|modifier| !key_input.pressed(modifier))
                && pan_orbit
                    .modifier_pan
                    .is_none_or(|modifier| !key_input.pressed(modifier))
        }
    };

    if no_modifiers_pressed {
        pinch_events
            .read()
            .map(|event| event.0 * 10.0 * pan_orbit.trackpad_sensitivity)
            .sum()
    } else {
        0.0
    }
}

pub fn orbit_pressed(
    pan_orbit: &PanOrbitCamera,
    mouse_input: &Res<ButtonInput<MouseButton>>,
    key_input: &Res<ButtonInput<KeyCode>>,
) -> bool {
    let is_pressed = pan_orbit
        .modifier_orbit
        .is_none_or(|modifier| key_input.pressed(modifier))
        && mouse_input.pressed(pan_orbit.button_orbit);

    is_pressed
        && pan_orbit
            .modifier_pan
            .is_none_or(|modifier| !key_input.pressed(modifier))
}

pub fn orbit_just_pressed(
    pan_orbit: &PanOrbitCamera,
    mouse_input: &Res<ButtonInput<MouseButton>>,
    key_input: &Res<ButtonInput<KeyCode>>,
) -> bool {
    let just_pressed = pan_orbit
        .modifier_orbit
        .is_none_or(|modifier| key_input.pressed(modifier))
        && (mouse_input.just_pressed(pan_orbit.button_orbit));

    just_pressed
        && pan_orbit
            .modifier_pan
            .is_none_or(|modifier| !key_input.pressed(modifier))
}

pub fn orbit_just_released(
    pan_orbit: &PanOrbitCamera,
    mouse_input: &Res<ButtonInput<MouseButton>>,
    key_input: &Res<ButtonInput<KeyCode>>,
) -> bool {
    let just_released = pan_orbit
        .modifier_orbit
        .is_none_or(|modifier| key_input.pressed(modifier))
        && (mouse_input.just_released(pan_orbit.button_orbit));

    just_released
        && pan_orbit
            .modifier_pan
            .is_none_or(|modifier| !key_input.pressed(modifier))
}

pub fn pan_pressed(
    pan_orbit: &PanOrbitCamera,
    mouse_input: &Res<ButtonInput<MouseButton>>,
    key_input: &Res<ButtonInput<KeyCode>>,
) -> bool {
    let is_pressed = pan_orbit
        .modifier_pan
        .is_none_or(|modifier| key_input.pressed(modifier))
        && mouse_input.pressed(pan_orbit.button_pan);

    is_pressed
        && pan_orbit
            .modifier_orbit
            .is_none_or(|modifier| !key_input.pressed(modifier))
}

pub fn pan_just_pressed(
    pan_orbit: &PanOrbitCamera,
    mouse_input: &Res<ButtonInput<MouseButton>>,
    key_input: &Res<ButtonInput<KeyCode>>,
) -> bool {
    let just_pressed = pan_orbit
        .modifier_pan
        .is_none_or(|modifier| key_input.pressed(modifier))
        && (mouse_input.just_pressed(pan_orbit.button_pan));

    just_pressed
        && pan_orbit
            .modifier_orbit
            .is_none_or(|modifier| !key_input.pressed(modifier))
}
