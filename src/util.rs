use bevy::math::Vec3;

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

#[cfg(test)]
mod tests {
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
