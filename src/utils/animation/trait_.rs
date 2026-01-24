//! Interpolatable trait and implementations

/// Trait for types that can be interpolated
pub trait Interpolatable: Clone {
    /// Interpolate between two values
    fn lerp(&self, other: &Self, t: f64) -> Self;
}

impl Interpolatable for f32 {
    fn lerp(&self, other: &Self, t: f64) -> Self {
        *self + (*other - *self) * t as f32
    }
}

impl Interpolatable for f64 {
    fn lerp(&self, other: &Self, t: f64) -> Self {
        *self + (*other - *self) * t
    }
}

impl Interpolatable for i32 {
    fn lerp(&self, other: &Self, t: f64) -> Self {
        (*self as f64 + (*other - *self) as f64 * t).round() as i32
    }
}

impl Interpolatable for u8 {
    fn lerp(&self, other: &Self, t: f64) -> Self {
        (*self as f64 + (*other as f64 - *self as f64) * t).round() as u8
    }
}

impl Interpolatable for u16 {
    fn lerp(&self, other: &Self, t: f64) -> Self {
        (*self as f64 + (*other as f64 - *self as f64) * t).round() as u16
    }
}

impl Interpolatable for (f64, f64) {
    fn lerp(&self, other: &Self, t: f64) -> Self {
        (self.0.lerp(&other.0, t), self.1.lerp(&other.1, t))
    }
}

impl Interpolatable for (f64, f64, f64) {
    fn lerp(&self, other: &Self, t: f64) -> Self {
        (
            self.0.lerp(&other.0, t),
            self.1.lerp(&other.1, t),
            self.2.lerp(&other.2, t),
        )
    }
}
