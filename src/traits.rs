use bevy::math::Vec2;
use std::ops::{Add, Sub};

pub trait Midpoint {
    type V: Add + Sub;

    fn midpoint(&self, other: Self::V) -> Self::V;
}

impl Midpoint for Vec2 {
    type V = Vec2;

    fn midpoint(&self, other: Vec2) -> Vec2 {
        let vec_to_other = other - *self;
        let half = vec_to_other / 2.0;
        *self + half
    }
}
