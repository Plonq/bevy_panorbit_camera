use bevy::input::touch::Touch;
use bevy::math::Vec2;
use bevy::prelude::*;

/// The control scheme to use for touch input. Given that some touch gestures don't make sense
/// being changed (e.g. pinch to zoom), there is just a set if different schemes rather than
/// full customization.
#[derive(Reflect, Default, Debug, Copy, Clone, PartialEq)]
pub enum TouchControls {
    /// Touch controls where single finger orbits:
    ///  - One finger move: orbit
    ///  - Two finger move: pan
    ///  - Two finger pinch: zoom
    #[default]
    OneFingerOrbit,
    /// Touch controls where single finger pans:
    ///  - One finger move: pan
    ///  - Two finger move: orbit
    ///  - Two finger pinch: zoom
    TwoFingerOrbit,
}

/// Holds information about current mobile gestures
#[derive(Debug, Clone)]
pub enum TouchGestures {
    /// No mobile gestures
    None,
    /// One finger mobile gestures
    OneFinger(OneFingerGestures),
    /// Two finger mobile gestures
    TwoFinger(TwoFingerGestures),
}

/// Holds information pertaining to one finger gestures
#[derive(Debug, Clone, Copy)]
pub struct OneFingerGestures {
    /// The delta movement of the mobile
    pub motion: Vec2,
}

/// Holds information pertaining to two finger gestures
#[derive(Debug, Clone, Copy)]
pub struct TwoFingerGestures {
    /// The delta movement of both touches.
    /// Uses the midpoint between the touches to calculate movement. Thus, if the midpoint doesn't
    /// move then this will be zero (or close to zero), like when pinching.
    pub motion: Vec2,
    /// The delta distance between both touches.
    /// Use this to implement pinch gestures.
    pub pinch: f32,
    /// The delta angle of the two touches.
    /// Positive values correspond to rotating clockwise.
    #[allow(dead_code)]
    pub rotation: f32,
}

/// Stores current and previous frame mobile data, and provides a method to get mobile gestures
#[derive(Resource, Default, Debug)]
pub struct TouchTracker {
    curr_pressed: (Option<Touch>, Option<Touch>),
    prev_pressed: (Option<Touch>, Option<Touch>),
}

impl TouchTracker {
    /// Calculate and return mobile gesture data for this frame
    pub fn get_touch_gestures(&self) -> TouchGestures {
        // The below matches only match when the previous and current frames have the same number
        // of touches. This means that when the number of touches changes, there's one frame
        // where this will return `TouchGestures::None`. From my testing, this does not result
        // in any adverse effects.
        match (self.curr_pressed, self.prev_pressed) {
            // Zero fingers
            ((None, None), (None, None)) => TouchGestures::None,
            // One finger
            ((Some(curr), None), (Some(prev), None)) => {
                let curr_pos = curr.position();
                let prev_pos = prev.position();

                let motion = curr_pos - prev_pos;

                TouchGestures::OneFinger(OneFingerGestures { motion })
            }
            // Two fingers
            ((Some(curr1), Some(curr2)), (Some(prev1), Some(prev2))) => {
                let curr1_pos = curr1.position();
                let curr2_pos = curr2.position();
                let prev1_pos = prev1.position();
                let prev2_pos = prev2.position();

                // Move
                let curr_midpoint = curr1_pos.midpoint(curr2_pos);
                let prev_midpoint = prev1_pos.midpoint(prev2_pos);
                let motion = curr_midpoint - prev_midpoint;

                // Pinch
                let curr_dist = curr1_pos.distance(curr2_pos);
                let prev_dist = prev1_pos.distance(prev2_pos);
                let pinch = curr_dist - prev_dist;

                // Rotate
                let prev_vec = prev2_pos - prev1_pos;
                let curr_vec = curr2_pos - curr1_pos;
                let prev_angle_negy = prev_vec.angle_to(Vec2::NEG_Y);
                let curr_angle_negy = curr_vec.angle_to(Vec2::NEG_Y);
                let prev_angle_posy = prev_vec.angle_to(Vec2::Y);
                let curr_angle_posy = curr_vec.angle_to(Vec2::Y);
                let rotate_angle_negy = curr_angle_negy - prev_angle_negy;
                let rotate_angle_posy = curr_angle_posy - prev_angle_posy;
                // The angle between -1deg and +1deg is 358deg according to Vec2::angle_between,
                // but we want the answer to be +2deg (or -2deg if swapped). Therefore, we calculate
                // two angles - one from UP and one from DOWN, and use the one with the smallest
                // absolute value. This is necessary to get a predictable result when the two touches
                // swap sides (i.e. mobile 1's X position being less than the other, to the other way
                // round).
                let rotation = if rotate_angle_negy.abs() < rotate_angle_posy.abs() {
                    rotate_angle_negy
                } else {
                    rotate_angle_posy
                };

                TouchGestures::TwoFinger(TwoFingerGestures {
                    motion,
                    pinch,
                    rotation,
                })
            }
            // Three fingers and more not currently supported
            _ => TouchGestures::None,
        }
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
