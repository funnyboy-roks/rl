//! Reimplementations of the maths items from Raylib (raymath)

use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

use derive_more::{Add, AddAssign, Debug, Display, From, Neg, Sub, SubAssign};

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    PartialOrd,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Neg,
    Display,
    From,
)]
#[display("Vector2({}, {})", x, y)]
#[from((f32, f32))]
#[repr(C)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

/// Constructors and constants
impl Vector2 {
    /// <0, 0>
    pub const ZERO: Vector2 = Vector2::new(0., 0.);
    /// <1, 1>
    pub const ONE: Vector2 = Vector2::new(1., 1.);
    /// <1, 0>
    pub const UNIT_X: Vector2 = Vector2::new(1., 0.);
    /// <0, 1>
    pub const UNIT_Y: Vector2 = Vector2::new(0., 1.);

    /// Construct a new vector
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Construct a new vector from a single value as <value, value>
    pub const fn value(value: f32) -> Self {
        Self::new(value, value)
    }
}

impl From<raylib_sys::Vector2> for Vector2 {
    fn from(value: raylib_sys::Vector2) -> Self {
        Self::new(value.x, value.y)
    }
}

impl From<Vector2> for raylib_sys::Vector2 {
    fn from(value: Vector2) -> Self {
        Self::new(value.x, value.y)
    }
}

/// Getters
#[rustfmt::skip] // allow inline function defs
impl Vector2 {
    pub const fn x(self) -> f32 { self.x }
    pub const fn y(self) -> f32 { self.y }
    pub const fn xy(self) -> Vector2 { self }
    pub const fn yx(self) -> Vector2 { Vector2::new(self.y, self.x) }
}

/// Const operators
#[rustfmt::skip]
impl Vector2 {
    pub const fn add(self, other: Vector2)   -> Vector2 { Self::new(self.x + other.x, self.y + other.y) }
    pub const fn add_value(self, value: f32) -> Vector2 { Self::new(self.x + value, self.y + value) }

    pub const fn sub(self, other: Vector2)   -> Vector2 { Self::new(self.x - other.x, self.y - other.y) }
    pub const fn sub_value(self, value: f32) -> Vector2 { Self::new(self.x - value, self.y - value) }

    pub const fn mul_components(self, rhs: Vector2) -> Vector2 { Self::new(self.x * rhs.x, self.y * rhs.y) }
    pub const fn mul_value(self, value: f32) -> Vector2 { Self::new(self.x * value, self.y * value) }
    pub const fn scale(self, value: f32)     -> Vector2 { Self::mul_value(self, value) }

    pub const fn div_value(self, value: f32) -> Vector2 { Self::new(self.x / value, self.y / value) }
}

/// Custom Operators
impl Vector2 {
    /// Length of this vector
    pub fn length(self) -> f32 {
        self.length_sq().sqrt()
    }

    /// Length of this vector squared
    pub const fn length_sq(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// Dot product of this vector an another
    pub const fn dot(self, other: Vector2) -> f32 {
        self.x * other.x + self.y * self.y
    }

    /// Length of the cross product of this vector and another
    pub const fn cross(self, other: Vector2) -> f32 {
        self.x * other.y - self.y * self.x
    }

    /// Produce the vector from this one to another
    pub const fn to(self, other: Vector2) -> Vector2 {
        other.sub(self)
    }

    /// Get the distance from this vector to another
    pub fn distance(self, other: Vector2) -> f32 {
        self.distance_sq(other).sqrt()
    }

    /// Get this distance from this vector to another squared
    pub const fn distance_sq(self, other: Vector2) -> f32 {
        self.to(other).length_sq()
    }

    /// Calculate the signed angle from `self` to `other`, relative to the origin (0, 0)
    ///
    /// # NOTE
    ///
    /// Coordinate system convention: positive X right, positive Y down positive angles appear
    /// clockwise, and negative angles appear counterclockwise
    pub fn angle(self, other: Vector2) -> f32 {
        let dot = self.x * other.x + self.y * other.y;
        let det = self.x * other.y - self.y * other.x;

        f32::atan2(det, dot)
    }

    /// Calculate angle defined by a two vectors line
    ///
    /// # NOTE
    ///
    /// - Parameters need to be normalized
    /// - Current implementation should be aligned with glm::angle
    pub fn line_angle(self, end: Vector2) -> f32 {
        -f32::atan2(end.y - self.y, end.x - self.x)
    }

    /// Normalise this vector to length 1
    pub fn normalize(self) -> Vector2 {
        let len = self.length();
        if len > 0. { self / len } else { Vector2::ZERO }
    }

    /// Linear interpolate from `self` to `end` using `amount`
    pub const fn lerp(self, end: Vector2, amount: f32) -> Vector2 {
        Self::new(
            self.x + amount * (end.x - self.x),
            self.y + amount * (end.y - self.y),
        )
    }

    /// Calculate reflected vector to normal
    pub const fn reflect(self, normal: Vector2) -> Vector2 {
        let dot = self.dot(normal);

        Self::new(
            self.x - (2. * normal.x) * dot,
            self.y - (2. * normal.y) * dot,
        )
    }

    /// Get min value for each pair of components
    pub const fn min(self, other: Vector2) -> Vector2 {
        Self::new(self.x.min(other.x), self.y.min(other.y))
    }

    /// Get min value for each pair of components
    pub const fn max(self, other: Vector2) -> Vector2 {
        Self::new(self.x.max(other.x), self.y.max(other.y))
    }

    /// Rotate vector by angle
    pub fn rotate(self, angle: f32) -> Vector2 {
        let (sin, cos) = angle.sin_cos();

        Self::new(self.x * cos - self.y * sin, self.x * sin + self.y * cos)
    }

    /// Move this vector towards target
    pub fn move_towards(self, target: Vector2, max_distance: f32) -> Vector2 {
        let to = self.to(target);
        let to_len_sq = to.length_sq();

        if to_len_sq == 0. || to_len_sq <= max_distance * max_distance {
            return target;
        }

        self + to.normalize() * max_distance
    }

    /// Invert the vector
    ///
    /// `<x, y> -> <1/x, 1/y>`
    pub const fn invert(self) -> Vector2 {
        Self::new(1. / self.x, 1. / self.y)
    }

    /// Clamp the components of the vector between min and max values specified by the given vectors
    pub const fn clamp(self, min: Vector2, max: Vector2) -> Vector2 {
        assert!(min.x <= max.x && min.y <= max.y, "min > max");
        Self::new(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y))
    }

    /// Clamp the magnitude of the vector between two min and max values
    pub fn clamp_value(self, min: f32, max: f32) -> Vector2 {
        assert!(min <= max, "min > max");
        let len_sq = self.length_sq();
        if len_sq > 0. {
            let len = len_sq.sqrt();

            let scale = if len < min {
                min / len
            } else if len > max {
                max / len
            } else {
                1.
            };

            self * scale
        } else {
            self
        }
    }

    /// Check whether two vectors are effectively equal
    pub const fn effectively_equal(self, other: Vector2) -> bool {
        const EPSILON: f32 = 0.000001;

        (self.x - other.x).abs() <= EPSILON * self.x.abs().max(other.x.abs()).max(1.)
            && (self.y - other.y).abs() <= EPSILON * self.y.abs().max(other.y.abs()).max(1.)
    }

    /// Compute the direction of a refracted ray
    /// self: normalized direction of the incoming ray
    /// normal: normalized normal vector of the interface of two optical media
    /// ratio: ratio of the refractive index of the medium from where the ray comes
    /// to the refractive index of the medium on the other side of the surface
    pub fn refract(self, normal: Vector2, ratio: f32) -> Vector2 {
        let dot = self.dot(normal);
        let d = 1.0 - ratio * ratio * (1.0 - dot * dot);

        if d >= 0.0 {
            let d = d.sqrt();
            (self * ratio - (ratio * dot + d)).mul_components(normal)
        } else {
            Vector2::ZERO
        }
    }
}

impl Add<f32> for Vector2 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Vector2::new(self.x + rhs, self.y + rhs)
    }
}

impl AddAssign<f32> for Vector2 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl Sub<f32> for Vector2 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Vector2::new(self.x - rhs, self.y - rhs)
    }
}

impl SubAssign<f32> for Vector2 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Vector2::new(self.x / rhs, self.y / rhs)
    }
}
