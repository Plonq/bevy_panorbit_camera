use crate::traits::Midpoint;
use bevy::input::touch::Touch;
use bevy::math::Vec2;
use bevy::prelude::{Res, ResMut, Resource, Touches};
use bevy::utils::HashMap;

/// Store current and previous frame touch data
#[derive(Resource, Default, Debug)]
pub struct TouchTracker {
    pub current_pressed: HashMap<u64, Touch>,
    pub previous_pressed: HashMap<u64, Touch>,
    pub curr_pressed: (Option<Touch>, Option<Touch>),
    pub prev_pressed: (Option<Touch>, Option<Touch>),
}

impl TouchTracker {
    /// Return orbit, pan, and zoom values based on touch data
    pub fn calculate_movement(&self) -> (Vec2, Vec2, f32) {
        let mut orbit = Vec2::ZERO;
        let mut pan = Vec2::ZERO;
        let mut zoom = 0.0;

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
                zoom += curr_dist - prev_dist;
            }
            _ => {}
        }

        (orbit, pan, zoom)
    }
}

/// Read touch input and save it in TouchTracker resource for easy consumption by the main system
pub fn touch_tracker(touches: Res<Touches>, mut touch_tracker: ResMut<TouchTracker>) {
    let pressed: Vec<&Touch> = touches.iter().collect();

    match pressed.len() {
        0 => {
            touch_tracker.current_pressed.clear();
            touch_tracker.previous_pressed.clear();

            touch_tracker.curr_pressed = (None, None);
            touch_tracker.prev_pressed = (None, None);
        }
        1 => {
            let touch: &Touch = pressed.first().unwrap();
            touch_tracker.previous_pressed = touch_tracker.current_pressed.clone();
            touch_tracker.current_pressed.clear();
            touch_tracker.current_pressed.insert(touch.id(), *touch);

            touch_tracker.prev_pressed = touch_tracker.curr_pressed;
            touch_tracker.curr_pressed = (Some(*touch), None);
        }
        2 => {
            let touch1: &Touch = pressed.first().unwrap();
            let touch2: &Touch = pressed.last().unwrap();
            touch_tracker.previous_pressed = touch_tracker.current_pressed.clone();
            touch_tracker.current_pressed.clear();
            touch_tracker.current_pressed.insert(touch1.id(), *touch1);
            touch_tracker.current_pressed.insert(touch2.id(), *touch2);

            touch_tracker.prev_pressed = touch_tracker.curr_pressed;
            touch_tracker.curr_pressed = (Some(*touch1), Some(*touch2));
        }
        _ => {}
    }
}
