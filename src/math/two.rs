use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

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
    pub const ZERO: Self = Self::new(0., 0.);
    /// <1, 1>
    pub const ONE: Self = Self::new(1., 1.);
    /// <1, 0>
    pub const UNIT_X: Self = Self::new(1., 0.);
    /// <0, 1>
    pub const UNIT_Y: Self = Self::new(0., 1.);

    /// Construct a new vector
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Construct a new vector from a single value as <value, value>
    #[must_use]
    pub const fn value(value: f32) -> Self {
        Self::new(value, value)
    }
}

impl From<[f32; 2]> for Vector2 {
    fn from([x, y]: [f32; 2]) -> Self {
        Self::new(x, y)
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
    #[must_use] pub const fn x(self) -> f32 { self.x }
    #[must_use] pub const fn y(self) -> f32 { self.y }
    #[must_use] pub const fn xy(self) -> Vector2 { self }
}

/// Const operators
#[rustfmt::skip]
impl Vector2 {
    #[must_use] pub const fn add(self, other: Self)          -> Self { Self::new(self.x + other.x, self.y + other.y) }
    #[must_use] pub const fn add_value(self, value: f32)     -> Self { Self::new(self.x + value,   self.y + value)   }

    #[must_use] pub const fn sub(self, other: Self)          -> Self { Self::new(self.x - other.x, self.y - other.y) }
    #[must_use] pub const fn sub_value(self, value: f32)     -> Self { Self::new(self.x - value,   self.y - value)   }

    #[must_use] pub const fn mul_components(self, rhs: Self) -> Self { Self::new(self.x * rhs.x,   self.y * rhs.y)   }
    #[must_use] pub const fn mul_value(self, value: f32)     -> Self { Self::new(self.x * value,   self.y * value)   }
    #[must_use] pub const fn scale(self, value: f32)         -> Self { Self::mul_value(self, value)                  }

    #[must_use] pub const fn div_components(self, rhs: Self) -> Self { Self::new(self.x / rhs.x,   self.y / rhs.y)   }
    #[must_use] pub const fn div_value(self, value: f32)     -> Self { Self::new(self.x / value,   self.y / value)   }
}

/// Custom Operators
impl Vector2 {
    /// Length of this vector
    #[must_use]
    pub fn length(self) -> f32 {
        self.length_sq().sqrt()
    }

    /// Length of this vector squared
    #[must_use]
    pub const fn length_sq(self) -> f32 {
        self.dot(self)
    }

    /// Dot product of this vector an another
    #[must_use]
    pub const fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * self.y
    }

    /// Length of the cross product of this vector and another
    #[must_use]
    pub const fn cross(self, other: Self) -> f32 {
        self.x * other.y - self.y * other.x
    }

    /// Produce the vector from this one to another
    #[must_use]
    pub const fn to(self, other: Self) -> Self {
        other.sub(self)
    }

    /// Get the distance from this vector to another
    #[must_use]
    pub fn distance(self, other: Self) -> f32 {
        self.distance_sq(other).sqrt()
    }

    /// Get this distance from this vector to another squared
    #[must_use]
    pub const fn distance_sq(self, other: Self) -> f32 {
        self.to(other).length_sq()
    }

    /// Calculate the signed angle from `self` to `other`, relative to the origin (0, 0)
    ///
    /// # NOTE
    ///
    /// Coordinate system convention: positive X right, positive Y down positive angles appear
    /// clockwise, and negative angles appear counterclockwise
    #[must_use]
    pub fn angle(self, other: Self) -> f32 {
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
    #[must_use]
    pub fn line_angle(self, end: Self) -> f32 {
        -f32::atan2(end.y - self.y, end.x - self.x)
    }

    /// Normalise this vector to length 1
    #[must_use]
    pub fn normalize(self) -> Self {
        let len = self.length();
        if len > 0. { self / len } else { Self::ZERO }
    }

    /// Linear interpolate from `self` to `end` using `amount`
    #[must_use]
    pub const fn lerp(self, end: Self, amount: f32) -> Self {
        Self::new(
            self.x + amount * (end.x - self.x),
            self.y + amount * (end.y - self.y),
        )
    }

    /// Calculate reflected vector to normal
    #[must_use]
    pub const fn reflect(self, normal: Self) -> Self {
        let dot = self.dot(normal);

        self.sub(normal.mul_value(2. * dot))
    }

    /// Get min value for each pair of components
    #[must_use]
    pub const fn min(self, other: Self) -> Self {
        Self::new(self.x.min(other.x), self.y.min(other.y))
    }

    /// Get min value for each pair of components
    #[must_use]
    pub const fn max(self, other: Self) -> Self {
        Self::new(self.x.max(other.x), self.y.max(other.y))
    }

    /// Rotate vector by angle
    #[must_use]
    pub fn rotate(self, angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();

        Self::new(self.x * cos - self.y * sin, self.x * sin + self.y * cos)
    }

    /// Move this vector towards target
    #[must_use]
    pub fn move_towards(self, target: Self, max_distance: f32) -> Self {
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
    #[must_use]
    pub const fn invert(self) -> Self {
        Self::new(1. / self.x, 1. / self.y)
    }

    /// Clamp the components of the vector between min and max values specified by the given vectors
    #[must_use]
    pub const fn clamp(self, min: Self, max: Self) -> Self {
        assert!(min.x <= max.x && min.y <= max.y, "min > max");
        Self::new(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y))
    }

    /// Clamp the magnitude of the vector between two min and max values
    #[must_use]
    pub fn clamp_value(self, min: f32, max: f32) -> Self {
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
    #[must_use]
    pub const fn effectively_equal(self, other: Self) -> bool {
        const EPSILON: f32 = 0.000001;

        (self.x - other.x).abs() <= EPSILON * self.x.abs().max(other.x.abs()).max(1.)
            && (self.y - other.y).abs() <= EPSILON * self.y.abs().max(other.y.abs()).max(1.)
    }

    /// Compute the direction of a refracted ray
    /// self: normalized direction of the incoming ray
    /// normal: normalized normal vector of the interface of two optical media
    /// ratio: ratio of the refractive index of the medium from where the ray comes
    /// to the refractive index of the medium on the other side of the surface
    #[must_use]
    pub fn refract(self, normal: Self, ratio: f32) -> Self {
        let dot = self.dot(normal);
        let d = 1.0 - ratio * ratio * (1.0 - dot * dot);

        if d >= 0.0 {
            let d = d.sqrt();
            self * ratio - normal.scale(ratio * dot + d)
        } else {
            Self::ZERO
        }
    }
}

impl Add<f32> for Vector2 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        self.add_value(rhs)
    }
}

impl AddAssign<f32> for Vector2 {
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}

impl Sub<f32> for Vector2 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        self.sub_value(rhs)
    }
}

impl SubAssign<f32> for Vector2 {
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs;
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        self.mul_value(rhs)
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        self.div_value(rhs)
    }
}

impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}
