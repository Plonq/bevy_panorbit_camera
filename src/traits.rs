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
