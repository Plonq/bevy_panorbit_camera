use bevy::math::Vec2;
use std::ops::{Add, Sub};

pub trait Midpoint {
    type V: Add + Sub;

    /// Return the value exact halfway between two values
    fn midpoint(&self, other: Self::V) -> Self::V;
}

impl Midpoint for Vec2 {
    type V = Vec2;

    /// Return the vector exact halfway between two vectors
    fn midpoint(&self, other: Self::V) -> Self::V {
        let vec_to_other = other - *self;
        let half = vec_to_other / 2.0;
        *self + half
    }
}

pub trait OptionalClamp {
    type N: PartialOrd;

    /// Clamp a value between two other values. The other values are optional. If both
    /// `min` and `max` are `None`, then the return value is equal to `self`.
    fn clamp_optional(&self, min: Option<Self::N>, max: Option<Self::N>) -> Self::N;
}

impl OptionalClamp for f32 {
    type N = f32;

    fn clamp_optional(&self, min: Option<Self::N>, max: Option<Self::N>) -> Self::N {
        let mut new_val = *self;
        if let Some(min) = min {
            new_val = f32::max(new_val, min);
        }
        if let Some(max) = max {
            new_val = f32::min(new_val, max);
        }
        new_val
    }
}
