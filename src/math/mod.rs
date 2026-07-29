//! Reimplementations of the maths items from Raylib (raymath)

mod four;
mod matrix;
mod three;
mod two;

use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign},
};

pub use four::{Quaternion, Vector4};
pub use matrix::Matrix;
pub use three::Vector3;
pub use two::Vector2;

// NOTE: Not using a generic for the type because that can't be used at const
#[derive(Debug, Clone, Copy)]
pub enum Angle {
    Degrees(f32),
    Radians(f32),
}

impl Angle {
    pub const ZERO: Self = Self::degrees(0.);

    pub const fn degrees(degrees: f32) -> Self {
        Self::Degrees(degrees)
    }

    pub const fn radians(radians: f32) -> Self {
        Self::Radians(radians)
    }

    pub const fn to_degrees(self) -> f32 {
        match self {
            Angle::Degrees(d) => d,
            Angle::Radians(r) => r.to_degrees(),
        }
    }

    pub const fn to_radians(self) -> f32 {
        match self {
            Angle::Degrees(d) => d.to_radians(),
            Angle::Radians(r) => r,
        }
    }

    /// Proxy method for [`f32::sin_cos`] on radians
    pub fn sin_cos(self) -> (f32, f32) {
        self.to_radians().sin_cos()
    }

    /// Proxy method for [`f32::sin`] on radians
    pub fn sin(self) -> f32 {
        self.to_radians().sin()
    }

    /// Proxy method for [`f32::cos`] on radians
    pub fn cos(self) -> f32 {
        self.to_radians().cos()
    }

    /// Proxy method for [`f32::tan`] on radians
    pub fn tan(self) -> f32 {
        self.to_radians().tan()
    }

    /// Proxy method for [`f32::asin`] on radians
    pub fn asin(self) -> f32 {
        self.to_radians().asin()
    }

    /// Proxy method for [`f32::acos`] on radians
    pub fn acos(self) -> f32 {
        self.to_radians().acos()
    }

    /// Proxy method for [`f32::atan`] on radians
    pub fn atan(self) -> f32 {
        self.to_radians().atan()
    }
}

impl PartialEq for Angle {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Degrees(l), Self::Degrees(r)) => l == r,
            (Self::Radians(l), Self::Radians(r)) => l == r,
            (Self::Radians(l), Self::Degrees(r)) => l.to_degrees() == *r,
            (Self::Degrees(l), Self::Radians(r)) => l.to_radians() == *r,
        }
    }
}

impl PartialOrd for Angle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.to_degrees().partial_cmp(&other.to_degrees())
    }
}

impl Add for Angle {
    type Output = Angle;

    /// Add two angles together, conforming to the unit of the left argument
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Angle::Degrees(l), Angle::Degrees(r)) => Angle::Degrees(l + r),
            (Angle::Degrees(l), Angle::Radians(r)) => Angle::Degrees(l + r.to_degrees()),
            (Angle::Radians(l), Angle::Degrees(r)) => Angle::Radians(l + r.to_radians()),
            (Angle::Radians(l), Angle::Radians(r)) => Angle::Radians(l + r),
        }
    }
}

impl AddAssign for Angle {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Mul<f32> for Angle {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        match self {
            Angle::Degrees(d) => Angle::Degrees(d * rhs),
            Angle::Radians(r) => Angle::Radians(r * rhs),
        }
    }
}

impl MulAssign<f32> for Angle {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div<f32> for Angle {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        match self {
            Angle::Degrees(d) => Angle::Degrees(d / rhs),
            Angle::Radians(r) => Angle::Radians(r / rhs),
        }
    }
}

impl DivAssign<f32> for Angle {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Angle::Degrees(d) => write!(f, "{}°", d),
            Angle::Radians(r) => write!(f, "{}π radians", r / std::f32::consts::PI),
        }
    }
}
