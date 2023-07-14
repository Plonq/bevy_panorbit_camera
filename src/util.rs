use crate::PanOrbitCamera;
use bevy::input::Input;
use bevy::math::{Mat3, Quat, Vec3};
use bevy::prelude::{KeyCode, MouseButton, Res, Transform};

pub fn calculate_from_translation_and_focus(translation: Vec3, focus: Vec3) -> (f32, f32, f32) {
    let comp_vec = translation - focus;
    let mut radius = comp_vec.length();
    if radius == 0.0 {
        radius = 0.05; // Radius 0 causes problems
    }
    let alpha = if comp_vec.x == 0.0 && comp_vec.z >= 0.0 {
        0.0
    } else {
        (comp_vec.z / (comp_vec.x.powi(2) + comp_vec.z.powi(2)).sqrt()).acos()
    };
    let beta = (comp_vec.y / radius).asin();
    (alpha, beta, radius)
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

/// Update `transform` based on alpha, beta, and the camera's focus and radius
pub fn update_orbit_transform(
    alpha: f32,
    beta: f32,
    pan_orbit: &PanOrbitCamera,
    transform: &mut Transform,
) {
    if let Some(radius) = pan_orbit.radius {
        let mut rotation = Quat::from_rotation_y(alpha);
        rotation *= Quat::from_rotation_x(-beta);

        transform.rotation = rotation;

        // Update the translation of the camera so we are always rotating 'around'
        // (orbiting) rather than rotating in place
        let rot_matrix = Mat3::from_quat(transform.rotation);
        transform.translation = pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, radius));
    }
}

pub fn apply_zoom_limits(value: f32, upper_limit: Option<f32>, lower_limit: Option<f32>) -> f32 {
    let mut new_val = value;
    if let Some(zoom_upper) = upper_limit {
        new_val = f32::min(new_val, zoom_upper);
    }
    if let Some(zoom_lower) = lower_limit {
        new_val = f32::max(new_val, zoom_lower);
    }
    // Prevent zoom to zero otherwise we can get stuck there because zoom
    // amount scales based on distance
    f32::max(new_val, 0.05)
}

#[cfg(test)]
mod calculate_from_translation_and_focus_tests {
    use super::*;
    use float_cmp::approx_eq;
    use std::f32::consts::PI;

    #[test]
    fn test_zero() {
        let translation = Vec3::new(0.0, 0.0, 0.0);
        let focus = Vec3::ZERO;
        let (alpha, beta, radius) = calculate_from_translation_and_focus(translation, focus);
        assert_eq!(alpha, 0.0);
        assert_eq!(beta, 0.0);
        assert_eq!(radius, 0.05);
    }

    #[test]
    fn test_in_front() {
        let translation = Vec3::new(0.0, 0.0, 5.0);
        let focus = Vec3::ZERO;
        let (alpha, beta, radius) = calculate_from_translation_and_focus(translation, focus);
        assert_eq!(alpha, 0.0);
        assert_eq!(beta, 0.0);
        assert_eq!(radius, 5.0);
    }

    #[test]
    fn test_to_the_side() {
        let translation = Vec3::new(5.0, 0.0, 0.0);
        let focus = Vec3::ZERO;
        let (alpha, beta, radius) = calculate_from_translation_and_focus(translation, focus);
        assert!(approx_eq!(f32, alpha, PI / 2.0));
        assert_eq!(beta, 0.0);
        assert_eq!(radius, 5.0);
    }

    #[test]
    fn test_above() {
        let translation = Vec3::new(0.0, 5.0, 0.0);
        let focus = Vec3::ZERO;
        let (alpha, beta, radius) = calculate_from_translation_and_focus(translation, focus);
        assert_eq!(alpha, 0.0);
        assert!(approx_eq!(f32, beta, PI / 2.0));
        assert_eq!(radius, 5.0);
    }

    #[test]
    fn test_arbitrary() {
        let translation = Vec3::new(0.92563736, 3.864204, -1.0105048);
        let focus = Vec3::ZERO;
        let (alpha, beta, radius) = calculate_from_translation_and_focus(translation, focus);
        assert!(approx_eq!(f32, alpha, 2.4));
        assert!(approx_eq!(f32, beta, 1.23));
        assert_eq!(radius, 4.1);
    }
}
