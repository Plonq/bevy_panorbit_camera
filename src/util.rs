use bevy::prelude::*;

const EPSILON: f32 = 0.001;

pub fn calculate_from_translation_and_focus(
    translation: Vec3,
    focus: Vec3,
    axis: [Vec3; 3],
) -> (f32, f32, f32) {
    let axis = Mat3::from_cols(axis[0], axis[1], axis[2]);
    let comp_vec = translation - focus;
    let mut radius = comp_vec.length();
    if radius == 0.0 {
        radius = 0.05; // Radius 0 causes problems
    }
    let comp_vec = axis * comp_vec;
    let yaw = comp_vec.x.atan2(comp_vec.z);
    let pitch = (comp_vec.y / radius).asin();
    (yaw, pitch, radius)
}

/// Update `transform` based on yaw, pitch, and the camera's focus and radius
pub fn update_orbit_transform(
    yaw: f32,
    pitch: f32,
    mut radius: f32,
    focus: Vec3,
    transform: &mut Transform,
    projection: &mut Projection,
    axis: [Vec3; 3],
) {
    let mut new_transform = Transform::IDENTITY;
    if let Projection::Orthographic(ref mut p) = *projection {
        p.scale = radius;
        // (near + far) / 2.0 ensures that objects near `focus` are not clipped
        radius = (p.near + p.far) / 2.0;
    }
    let yaw_rot = Quat::from_axis_angle(axis[1], yaw);
    let pitch_rot = Quat::from_axis_angle(axis[0], -pitch);
    new_transform.rotation *= yaw_rot * pitch_rot;
    new_transform.translation += focus + new_transform.rotation * Vec3::new(0.0, 0.0, radius);
    *transform = new_transform;
}

pub fn approx_equal(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

pub fn lerp_and_snap_f32(from: f32, to: f32, smoothness: f32, dt: f32) -> f32 {
    let t = smoothness.powi(7);
    let mut new_value = from.lerp(to, 1.0 - t.powf(dt));
    if smoothness < 1.0 && approx_equal(new_value, to) {
        new_value = to;
    }
    new_value
}

pub fn lerp_and_snap_vec3(from: Vec3, to: Vec3, smoothness: f32, dt: f32) -> Vec3 {
    let t = smoothness.powi(7);
    let mut new_value = from.lerp(to, 1.0 - t.powf(dt));
    if smoothness < 1.0 && approx_equal((new_value - to).length(), 0.0) {
        new_value.x = to.x;
    }
    new_value
}

#[cfg(test)]
mod calculate_from_translation_and_focus_tests {
    use super::*;
    use float_cmp::approx_eq;
    use std::f32::consts::PI;
    const AXIS: [Vec3; 3] = [Vec3::X, Vec3::Y, Vec3::Z];
    const AXIS_Z_UP: [Vec3; 3] = [Vec3::X, Vec3::Z, Vec3::Y];

    #[test]
    fn zero() {
        let translation = Vec3::new(0.0, 0.0, 0.0);
        let focus = Vec3::ZERO;
        let (yaw, pitch, radius) = calculate_from_translation_and_focus(translation, focus, AXIS);
        assert_eq!(yaw, 0.0);
        assert_eq!(pitch, 0.0);
        assert_eq!(radius, 0.05);
    }

    #[test]
    fn zero_z_up_axis() {
        let translation = Vec3::new(0.0, 0.0, 0.0);
        let focus = Vec3::ZERO;
        let (yaw, pitch, radius) =
            calculate_from_translation_and_focus(translation, focus, AXIS_Z_UP);
        assert_eq!(yaw, 0.0);
        assert_eq!(pitch, 0.0);
        assert_eq!(radius, 0.05);
    }

    #[test]
    fn in_front() {
        let translation = Vec3::new(0.0, 0.0, 5.0);
        let focus = Vec3::ZERO;
        let (yaw, pitch, radius) = calculate_from_translation_and_focus(translation, focus, AXIS);
        assert_eq!(yaw, 0.0);
        assert_eq!(pitch, 0.0);
        assert_eq!(radius, 5.0);
    }

    #[test]
    fn in_front_z_up_axis() {
        let translation = Vec3::new(0.0, 5.0, 0.0);
        let axis = [Vec3::X, Vec3::Z, Vec3::Y];
        let focus = Vec3::ZERO;
        let (yaw, pitch, radius) = calculate_from_translation_and_focus(translation, focus, axis);
        assert_eq!(yaw, 0.0);
        assert_eq!(pitch, 0.0);
        assert_eq!(radius, 5.0);
    }

    #[test]
    fn to_the_side() {
        let translation = Vec3::new(5.0, 0.0, 0.0);
        let focus = Vec3::ZERO;
        let (yaw, pitch, radius) = calculate_from_translation_and_focus(translation, focus, AXIS);
        assert!(approx_eq!(f32, yaw, PI / 2.0));
        assert_eq!(pitch, 0.0);
        assert_eq!(radius, 5.0);
    }

    #[test]
    fn above() {
        let translation = Vec3::new(0.0, 5.0, 0.0);
        let focus = Vec3::ZERO;
        let (yaw, pitch, radius) = calculate_from_translation_and_focus(translation, focus, AXIS);
        assert_eq!(yaw, 0.0);
        assert!(approx_eq!(f32, pitch, PI / 2.0));
        assert_eq!(radius, 5.0);
    }

    #[test]
    fn above_z_as_up_axis() {
        let translation = Vec3::new(0.0, 0.0, 5.0);
        let focus = Vec3::ZERO;
        let (yaw, pitch, radius) =
            calculate_from_translation_and_focus(translation, focus, AXIS_Z_UP);
        assert_eq!(yaw, 0.0);
        assert!(approx_eq!(f32, pitch, PI / 2.0));
        assert_eq!(radius, 5.0);
    }

    #[test]
    fn arbitrary() {
        let translation = Vec3::new(0.92563736, 3.864204, -1.0105048);
        let focus = Vec3::ZERO;
        let (yaw, pitch, radius) = calculate_from_translation_and_focus(translation, focus, AXIS);
        assert!(approx_eq!(f32, yaw, 2.4));
        assert!(approx_eq!(f32, pitch, 1.23));
        assert_eq!(radius, 4.1);
    }

    #[test]
    fn negative_x() {
        let translation = Vec3::new(-5.0, 5.0, 9.0);
        let focus = Vec3::ZERO;
        let (yaw, pitch, radius) = calculate_from_translation_and_focus(translation, focus, AXIS);
        assert!(approx_eq!(f32, yaw, -0.5070985));
        assert!(approx_eq!(f32, pitch, 0.45209613));
        assert!(approx_eq!(f32, radius, 11.445523));
    }
}

#[cfg(test)]
mod approx_equal_tests {
    use super::*;

    #[test]
    fn same_value_is_approx_equal() {
        assert!(approx_equal(1.0, 1.0));
    }

    #[test]
    fn value_within_threshold_is_approx_equal() {
        assert!(approx_equal(1.0, 1.0000001));
    }

    #[test]
    fn value_outside_threshold_is_not_approx_equal() {
        assert!(!approx_equal(1.0, 1.01));
    }
}

#[cfg(test)]
mod lerp_and_snap_f32_tests {
    use super::*;

    #[test]
    fn lerps_when_output_outside_snap_threshold() {
        let out = lerp_and_snap_f32(1.0, 2.0, 0.5, 1.0);
        // Due to the frame rate independence, this value is not easily predictable
        assert_eq!(out, 1.9921875);
    }

    #[test]
    fn snaps_to_target_when_inside_threshold() {
        let out = lerp_and_snap_f32(1.9991, 2.0, 0.5, 1.0);
        assert_eq!(out, 2.0);
        let out = lerp_and_snap_f32(1.9991, 2.0, 0.1, 1.0);
        assert_eq!(out, 2.0);
        let out = lerp_and_snap_f32(1.9991, 2.0, 0.9, 1.0);
        assert_eq!(out, 2.0);
    }

    #[test]
    fn does_not_snap_if_smoothness_is_one() {
        // Smoothness of one results in the value not changing, so it doesn't make sense to snap
        let out = lerp_and_snap_f32(1.9991, 2.0, 1.0, 1.0);
        assert_eq!(out, 1.9991);
    }
}

#[cfg(test)]
mod lerp_and_snap_vec3_tests {
    use super::*;

    #[test]
    fn lerps_when_output_outside_snap_threshold() {
        let out = lerp_and_snap_vec3(Vec3::ZERO, Vec3::X, 0.5, 1.0);
        // Due to the frame rate independence, this value is not easily predictable
        assert_eq!(out, Vec3::new(0.9921875, 0.0, 0.0));
    }

    #[test]
    fn snaps_to_target_when_inside_threshold() {
        let out = lerp_and_snap_vec3(Vec3::X * 0.9991, Vec3::X, 0.5, 1.0);
        assert_eq!(out, Vec3::X);
        let out = lerp_and_snap_vec3(Vec3::X * 0.9991, Vec3::X, 0.1, 1.0);
        assert_eq!(out, Vec3::X);
        let out = lerp_and_snap_vec3(Vec3::X * 0.9991, Vec3::X, 0.9, 1.0);
        assert_eq!(out, Vec3::X);
    }

    #[test]
    fn does_not_snap_if_smoothness_is_one() {
        // Smoothness of one results in the value not changing, so it doesn't make sense to snap
        let out = lerp_and_snap_vec3(Vec3::X * 0.9991, Vec3::X, 1.0, 1.0);
        assert_eq!(out, Vec3::X * 0.9991);
    }
}
