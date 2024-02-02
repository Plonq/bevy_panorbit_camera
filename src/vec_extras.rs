use bevy::math::Vec2;

pub trait Vec2Extras {
    fn midpoint(&self, other: Vec2) -> Vec2;
}

impl Vec2Extras for Vec2 {
    fn midpoint(&self, other: Vec2) -> Vec2 {
        let vec_to_other = other - *self;
        let half = vec_to_other / 2.0;
        *self + half
    }
}
