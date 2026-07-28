use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use derive_more::{Add, AddAssign, Debug, Display, From, Neg, Sub, SubAssign};

use crate::math::Vector2;

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
#[display("Vector3({}, {})", x, y)]
#[from((f32, f32, f32))]
#[repr(C)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Constructors and constants
impl Vector3 {
    /// <0, 0, 0>
    pub const ZERO: Self = Self::new(0., 0., 0.);
    /// <1, 1, 1>
    pub const ONE: Self = Self::new(1., 1., 1.);
    /// <1, 0, 0>
    pub const UNIT_X: Self = Self::new(1., 0., 0.);
    /// <0, 1, 0>
    pub const UNIT_Y: Self = Self::new(0., 1., 0.);
    /// <0, 0, 1>
    pub const UNIT_Z: Self = Self::new(0., 0., 1.);

    /// Construct a new vector
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Construct a new vector from a single value as <value, value>
    #[must_use]
    pub const fn value(value: f32) -> Self {
        Self::new(value, value, value)
    }
}

impl From<[f32; 3]> for Vector3 {
    fn from([x, y, z]: [f32; 3]) -> Self {
        Self::new(x, y, z)
    }
}

impl From<raylib_sys::Vector3> for Vector3 {
    fn from(value: raylib_sys::Vector3) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

impl From<Vector3> for raylib_sys::Vector3 {
    fn from(value: Vector3) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

/// Getters
#[rustfmt::skip] // allow inline function defs
impl Vector3 {
    #[must_use] pub const fn x(self) -> f32 { self.x }
    #[must_use] pub const fn y(self) -> f32 { self.y }
    #[must_use] pub const fn z(self) -> f32 { self.z }
    #[must_use] pub const fn xy(self) -> Vector2 { Vector2::new(self.x, self.y) }
    #[must_use] pub const fn xz(self) -> Vector2 { Vector2::new(self.x, self.z) }
    #[must_use] pub const fn yz(self) -> Vector2 { Vector2::new(self.y, self.z) }
    #[must_use] pub const fn xyz(self) -> Vector3 { self }
}

/// Const operators
#[rustfmt::skip]
impl Vector3 {
    #[must_use] pub const fn add(self, other: Self)          -> Self { Self::new(self.x + other.x, self.y + other.y, self.z + other.z) }
    #[must_use] pub const fn add_value(self, value: f32)     -> Self { Self::new(self.x + value,   self.y + value,   self.z + value)   }

    #[must_use] pub const fn sub(self, other: Self)          -> Self { Self::new(self.x - other.x, self.y - other.y, self.z - other.z) }
    #[must_use] pub const fn sub_value(self, value: f32)     -> Self { Self::new(self.x - value,   self.y - value,   self.z - value)   }

    #[must_use] pub const fn mul_components(self, rhs: Self) -> Self { Self::new(self.x * rhs.x,   self.y * rhs.y,   self.z * rhs.z)   }
    #[must_use] pub const fn mul_value(self, value: f32)     -> Self { Self::new(self.x * value,   self.y * value,   self.z * value)   }
    #[must_use] pub const fn scale(self, value: f32)         -> Self { Self::mul_value(self, value)                                    }

    #[must_use] pub const fn div_components(self, rhs: Self) -> Self { Self::new(self.x / rhs.x,   self.y / rhs.y,   self.z / rhs.z)   }
    #[must_use] pub const fn div_value(self, value: f32)     -> Self { Self::new(self.x / value,   self.y / value,   self.z / value)   }
}

/// Custom Operators
impl Vector3 {
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
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Length of the cross product of this vector and another
    #[must_use]
    pub const fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Calculate one vector perpendicular vector
    #[must_use]
    pub const fn perpendicular(self) -> Self {
        let mut min = self.x.abs();
        let mut cardinal_axis = Self::UNIT_X;

        if self.y.abs() < min {
            min = self.y.abs();
            cardinal_axis = Self::UNIT_Y;
        }

        if self.z.abs() < min {
            cardinal_axis = Self::UNIT_Z;
        }

        self.cross(cardinal_axis)
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

    /// Calculate the signed angle from `self` to `other`, relative to the origin (0, 0, 0)
    #[must_use]
    pub fn angle(self, other: Self) -> f32 {
        let len = self.cross(other).length();
        let dot = self.dot(other);

        f32::atan2(len, dot)
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
        self.add_value(amount).mul_components(end.sub(self))
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
        Self::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    /// Get min value for each pair of components
    #[must_use]
    pub const fn max(self, other: Self) -> Self {
        Self::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }

    /// Calculate the projection of `self` onto `other`
    #[must_use]
    pub const fn project(self, other: Self) -> Self {
        let v1dv2 = self.dot(other);
        let v2dv2 = other.length_sq();

        other.mul_value(v1dv2 / v2dv2)
    }

    /// Calculate the rejection of the vector `self` onto `other`
    #[must_use]
    pub const fn reject(self, other: Self) -> Self {
        let v1dv2 = self.dot(other);
        let v2dv2 = other.length_sq();

        self.sub(other.mul_value(v1dv2 / v2dv2))
    }

    /// Orthonormalize provided vectors
    ///
    /// First return value is `self` normalised, second return is orthornormalized vector.
    ///
    /// Makes vectors normalized and orthogonal to each other
    /// Gram-Schmidt function implementation
    #[must_use]
    pub fn orthonormalize(self, other: Self) -> (Self, Self) {
        let v1 = self.normalize();
        let vn1 = v1.cross(other);
        let vn1 = vn1.normalize();
        let vn2 = vn1.cross(v1);

        (v1, vn2)
    }

    #[must_use]
    pub fn rotate_by_axis_angle(self, axis: Self, angle: f32) -> Self {
        // Using Euler-Rodrigues Formula
        // Ref.: https://en.wikipedia.org/w/index.php?title=Euler%E2%80%93Rodrigues_formula

        let result = self;

        // SelfNormalize(axis);
        let axis = axis.normalize();

        let angle = angle / 2.;
        let (sin, cos) = angle.sin_cos();
        let b = axis.x * sin;
        let c = axis.y * sin;
        let d = axis.z * sin;
        let w = Self::new(b, c, d);

        let wv = w.cross(self);
        let wwv = w.cross(wv);
        let wv = wv.scale(2. * cos);
        let wwv = wwv.scale(2.);

        result + wv + wwv
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
    /// `<x, y> -> <1/x, 1/y, 1/z>`
    #[must_use]
    pub const fn invert(self) -> Self {
        Self::new(1. / self.x, 1. / self.y, 1. / self.z)
    }

    /// Clamp the components of the vector between min and max values specified by the given vectors
    #[must_use]
    pub const fn clamp(self, min: Self, max: Self) -> Self {
        assert!(
            min.x <= max.x && min.y <= max.y && min.z <= max.z,
            "min > max"
        );
        Self::new(
            self.x.clamp(min.x, max.x),
            self.y.clamp(min.y, max.y),
            self.z.clamp(min.z, max.z),
        )
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
            let d = d.sqrt(); // can't be const
            self * ratio - normal.scale(ratio * dot + d)
        } else {
            Self::ZERO
        }
    }

    /// Calculate cubic hermite interpolation between two vectors and their tangents as described in the
    /// GLTF 2.0 specification:
    /// <https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#interpolation-cubic>
    #[must_use]
    pub const fn cubic_hermite(
        self,
        tangent1: Self,
        v2: Self,
        tangent2: Self,
        amount: f32,
    ) -> Self {
        let p2 = amount * amount;
        let p3 = amount * amount * amount;

        self.scale(2. * p3 - 3. * p2 + 1.)
            .add(tangent1.scale(p3 - 2. * p2 + amount))
            .add(v2.scale(-2. * p3 + 3. * p2))
            .add(tangent2.scale(p3 - p2))
    }

    /// Compute barycenter coordinates (u, v, w) for point p with respect to triangle (a, b, c)
    /// NOTE: Assumes self is on the plane of the triangle
    #[must_use]
    pub const fn barycenter(self, triangle: (Self, Self, Self)) -> Self {
        let (a, b, c) = triangle;
        let v0 = b.sub(a);
        let v1 = c.sub(a);
        let v2 = self.sub(a);
        let d00 = v0.dot(v0);
        let d01 = v0.dot(v1);
        let d11 = v1.dot(v1);
        let d20 = v2.dot(v0);
        let d21 = v2.dot(v1);

        let denom = d00 * d11 - d01 * d01;

        let y = (d11 * d20 - d01 * d21) / denom;
        let z = (d00 * d21 - d01 * d20) / denom;
        let x = 1. - (z + y);

        Self::new(x, y, z)
    }
}

impl Add<f32> for Vector3 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        self.add_value(rhs)
    }
}

impl AddAssign<f32> for Vector3 {
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}

impl Sub<f32> for Vector3 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        self.sub_value(rhs)
    }
}

impl SubAssign<f32> for Vector3 {
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs;
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        self.mul_value(rhs)
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div<f32> for Vector3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        self.div_value(rhs)
    }
}

impl DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}
