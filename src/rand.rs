use std::f32::consts::TAU;

use raylib_sys as sys;

use crate::Vector2;

/// Get a random value between min and max (both included)
pub fn random_value(min: i32, max: i32) -> i32 {
    unsafe { sys::GetRandomValue(min, max) }
}

pub trait Random {
    fn random() -> Self;
}

impl Random for f32 {
    /// Random value in [0, 1)
    fn random() -> Self {
        random_value(0, 100000000 - 1) as f32 / 100000000.
    }
}

impl Random for Vector2 {
    /// Unit vector pointing in random direction
    fn random() -> Self {
        Self::new(1., 0.).rotate(f32::random() * TAU)
    }
}

impl Random for bool {
    fn random() -> Self {
        random_value(0, 1) == 1
    }
}
