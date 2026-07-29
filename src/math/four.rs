use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use derive_more::{Add, AddAssign, Debug, Display, From, Into, Neg, Sub, SubAssign};

use crate::math::{Angle, Matrix, Vector2, Vector3};

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
#[display("Vector4({}, {}, {}, {})", x, y, z, w)]
#[from((f32, f32, f32, f32))]
#[repr(C)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

/// Constructors and constants
impl Vector4 {
    /// <0, 0, 0, 0>
    pub const ZERO: Self = Self::new(0., 0., 0., 0.);
    /// <1, 1, 1, 1>
    pub const ONE: Self = Self::new(1., 1., 1., 1.);
    /// <1, 0, 0, 0>
    pub const UNIT_X: Self = Self::new(1., 0., 0., 0.);
    /// <0, 1, 0, 0>
    pub const UNIT_Y: Self = Self::new(0., 1., 0., 0.);
    /// <0, 0, 1, 0>
    pub const UNIT_Z: Self = Self::new(0., 0., 1., 0.);
    /// <0, 0, 0, 1>
    pub const UNIT_W: Self = Self::new(0., 0., 0., 1.);

    /// Construct a new vector
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Construct a new vector from a single value as <value, value>
    #[must_use]
    pub const fn value(value: f32) -> Self {
        Self::new(value, value, value, value)
    }
}

impl From<[f32; 4]> for Vector4 {
    fn from([x, y, z, w]: [f32; 4]) -> Self {
        Self::new(x, y, z, w)
    }
}

impl From<raylib_sys::Vector4> for Vector4 {
    fn from(value: raylib_sys::Vector4) -> Self {
        Self::new(value.x, value.y, value.z, value.w)
    }
}

impl From<Vector4> for raylib_sys::Vector4 {
    fn from(value: Vector4) -> Self {
        Self::new(value.x, value.y, value.z, value.w)
    }
}

/// Swizzling
#[rustfmt::skip] // allow inline function defs
impl Vector4 {
    #[must_use] pub const fn x(self)    -> f32     { self.x                                       }
    #[must_use] pub const fn y(self)    -> f32     { self.y                                       }
    #[must_use] pub const fn z(self)    -> f32     { self.z                                       }
    #[must_use] pub const fn w(self)    -> f32     { self.w                                       }
    #[must_use] pub const fn xy(self)   -> Vector2 { Vector2::new(self.x, self.y)                 }
    #[must_use] pub const fn xz(self)   -> Vector2 { Vector2::new(self.x, self.z)                 }
    #[must_use] pub const fn xw(self)   -> Vector2 { Vector2::new(self.x, self.w)                 }
    #[must_use] pub const fn yz(self)   -> Vector2 { Vector2::new(self.y, self.z)                 }
    #[must_use] pub const fn yw(self)   -> Vector2 { Vector2::new(self.y, self.w)                 }
    #[must_use] pub const fn xyz(self)  -> Vector3 { Vector3::new(self.x, self.y, self.z)         }
    #[must_use] pub const fn xyw(self)  -> Vector3 { Vector3::new(self.x, self.y, self.w)         }
    #[must_use] pub const fn xzw(self)  -> Vector3 { Vector3::new(self.x, self.z, self.w)         }
    #[must_use] pub const fn yzw(self)  -> Vector3 { Vector3::new(self.y, self.z, self.w)         }
    #[must_use] pub const fn xyzw(self) -> Vector4 { Vector4::new(self.x, self.y, self.z, self.w) }
}

/// Const operators
#[rustfmt::skip]
impl Vector4 {
    #[must_use] pub const fn add(self, other: Self)          -> Self { Self::new(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w) }
    #[must_use] pub const fn add_value(self, value: f32)     -> Self { Self::new(self.x + value,   self.y + value,   self.z + value,   self.w + value)   }

    #[must_use] pub const fn sub(self, other: Self)          -> Self { Self::new(self.x - other.x, self.y - other.y, self.z - other.z, self.w - other.w) }
    #[must_use] pub const fn sub_value(self, value: f32)     -> Self { Self::new(self.x - value,   self.y - value,   self.z - value,   self.w - value)   }

    #[must_use] pub const fn mul_components(self, rhs: Self) -> Self { Self::new(self.x * rhs.x,   self.y * rhs.y,   self.z * rhs.z,   self.w * rhs.w)   }
    #[must_use] pub const fn mul_value(self, value: f32)     -> Self { Self::new(self.x * value,   self.y * value,   self.z * value,   self.w * value)   }
    #[must_use] pub const fn scale(self, value: f32)         -> Self { Self::mul_value(self, value)                                                    }

    #[must_use] pub const fn div_components(self, rhs: Self) -> Self { Self::new(self.x / rhs.x,   self.y / rhs.y,   self.z / rhs.z,   self.w / rhs.w)   }
    #[must_use] pub const fn div_value(self, value: f32)     -> Self { Self::new(self.x / value,   self.y / value,   self.z / value,   self.w / value)   }
}

/// Custom Operators
impl Vector4 {
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
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
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

    /// Get min value for each pair of components
    #[must_use]
    pub const fn min(self, other: Self) -> Self {
        Self::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
            self.w.min(other.w),
        )
    }

    /// Get min value for each pair of components
    #[must_use]
    pub const fn max(self, other: Self) -> Self {
        Self::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
            self.w.max(other.w),
        )
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
        Self::new(1. / self.x, 1. / self.y, 1. / self.z, 1. / self.z)
    }

    /// Clamp the components of the vector between min and max values specified by the given vectors
    #[must_use]
    pub const fn clamp(self, min: Self, max: Self) -> Self {
        assert!(
            min.x <= max.x && min.y <= max.y && min.z <= max.z && min.w <= max.w,
            "min > max"
        );
        Self::new(
            self.x.clamp(min.x, max.x),
            self.y.clamp(min.y, max.y),
            self.z.clamp(min.z, max.z),
            self.w.clamp(min.w, max.w),
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
            && (self.z - other.z).abs() <= EPSILON * self.z.abs().max(other.z.abs()).max(1.)
            && (self.w - other.w).abs() <= EPSILON * self.w.abs().max(other.w.abs()).max(1.)
    }
}

impl Add<f32> for Vector4 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        self.add_value(rhs)
    }
}

impl AddAssign<f32> for Vector4 {
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}

impl Sub<f32> for Vector4 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        self.sub_value(rhs)
    }
}

impl SubAssign<f32> for Vector4 {
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs;
    }
}

impl Mul<f32> for Vector4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        self.mul_value(rhs)
    }
}

impl MulAssign<f32> for Vector4 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div<f32> for Vector4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        self.div_value(rhs)
    }
}

impl DivAssign<f32> for Vector4 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

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
    Into,
)]
#[display("Quaternion({}, {}, {}, {})", _0.x, _0.y, _0.z, _0.w)]
#[repr(transparent)]
pub struct Quaternion(Vector4);

impl From<raylib_sys::Quaternion> for Quaternion {
    fn from(value: raylib_sys::Quaternion) -> Self {
        Self::new(value.x, value.y, value.z, value.w)
    }
}

impl From<Quaternion> for raylib_sys::Quaternion {
    fn from(value: Quaternion) -> Self {
        Self::new(value.0.x, value.0.y, value.0.z, value.0.w)
    }
}

/// Constructors
impl Quaternion {
    /// <0, 0, 0, 1>
    pub const IDENTITY: Self = Self(Vector4::UNIT_W);

    /// Construct a new vector
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self::from_vector(Vector4::new(x, y, z, w))
    }

    /// Construct a new vector from a single value as <value, value>
    #[must_use]
    pub const fn value(value: f32) -> Self {
        Self::from_vector(Vector4::value(value))
    }

    #[must_use]
    pub const fn from_vector(vec: Vector4) -> Self {
        Self(vec)
    }

    /// Calculate quaternion based on the rotation from one vector to another
    #[must_use]
    pub fn from_vectors(from: Vector3, to: Vector3) -> Self {
        // use the raylib function since we couldn't do const anyways
        unsafe { raylib_sys::QuaternionFromVector3ToVector3(from.into(), to.into()) }.into()
    }

    /// Get rotation quaternion for an angle and axis
    #[must_use]
    pub fn from_axis_angle(axis: Vector3, angle: Angle) -> Self {
        // use the raylib function since we couldn't do const anyways
        unsafe { raylib_sys::QuaternionFromAxisAngle(axis.into(), angle.to_radians()) }.into()
    }

    /// Get the quaternion equivalent to Euler angles
    ///
    /// # NOTE
    ///
    /// Rotation order is ZYX
    #[must_use]
    pub fn from_euler(pitch: Angle, yaw: Angle, roll: Angle) -> Self {
        // use the raylib function since we couldn't do const anyways
        unsafe {
            raylib_sys::QuaternionFromEuler(pitch.to_radians(), yaw.to_radians(), roll.to_radians())
        }
        .into()
    }
}

/// Swizzling
#[rustfmt::skip] // allow inline function defs
impl Quaternion {
    #[must_use] pub const fn x(self)    -> f32     { self.0.x() }
    #[must_use] pub const fn y(self)    -> f32     { self.0.y() }
    #[must_use] pub const fn z(self)    -> f32     { self.0.z() }
    #[must_use] pub const fn w(self)    -> f32     { self.0.w() }
    #[must_use] pub const fn xy(self)   -> Vector2 { self.0.xy() }
    #[must_use] pub const fn xz(self)   -> Vector2 { self.0.xz() }
    #[must_use] pub const fn xw(self)   -> Vector2 { self.0.xw() }
    #[must_use] pub const fn yz(self)   -> Vector2 { self.0.yz() }
    #[must_use] pub const fn yw(self)   -> Vector2 { self.0.yw() }
    #[must_use] pub const fn xyz(self)  -> Vector3 { self.0.xyz() }
    #[must_use] pub const fn xyw(self)  -> Vector3 { self.0.xyw() }
    #[must_use] pub const fn xzw(self)  -> Vector3 { self.0.xzw() }
    #[must_use] pub const fn yzw(self)  -> Vector3 { self.0.yzw() }
    #[must_use] pub const fn xyzw(self) -> Vector4 { self.0.xyzw() }
}

/// Const operators
#[rustfmt::skip]
impl Quaternion {
    #[must_use] pub const fn add(self, other: Self)          -> Self { Self::from_vector(self.0.add(other.0))          }
    #[must_use] pub const fn add_value(self, value: f32)     -> Self { Self::from_vector(self.0.add_value(value))      }

    #[must_use] pub const fn sub(self, other: Self)          -> Self { Self::from_vector(self.0.sub(other.0))          }
    #[must_use] pub const fn sub_value(self, value: f32)     -> Self { Self::from_vector(self.0.sub_value(value))      }

    #[must_use] pub const fn mul_value(self, value: f32)     -> Self { Self::from_vector(self.0.mul_value(value))      }
    #[must_use] pub const fn mul_components(self, rhs: Self) -> Self { Self::from_vector(self.0.mul_components(rhs.0)) }
    #[must_use] pub const fn scale(self, value: f32)         -> Self { Self::from_vector(self.0.scale(value))          }

    #[must_use] pub const fn div_components(self, rhs: Self) -> Self { Self::from_vector(self.0.div_components(rhs.0)) }
    #[must_use] pub const fn div_value(self, value: f32)     -> Self { Self::from_vector(self.0.div_value(value))      }
}

/// Quaternion Operations
impl Quaternion {
    // in here because it breaks the alignment of const ops impl
    #[must_use]
    pub const fn mul(self, rhs: Self) -> Self {
        // shorter names
        let a = self.0;
        let b = rhs.0;

        Self::new(
            a.x * b.w + a.w * b.x + a.y * b.z - a.z * b.y,
            a.y * b.w + a.w * b.y + a.z * b.x - a.x * b.z,
            a.z * b.w + a.w * b.z + a.x * b.y - a.y * b.x,
            a.w * b.w - a.x * b.x - a.y * b.y - a.z * b.z,
        )
    }

    #[must_use]
    pub const fn to_vector(self) -> Vector4 {
        self.0
    }

    /// Computes the length of this quaternion
    #[must_use]
    pub fn length(self) -> f32 {
        self.0.length()
    }

    /// Computes the length squared of this quaternion
    #[must_use]
    pub fn length_sq(self) -> f32 {
        self.0.length_sq()
    }

    /// Normalize the this quaterion (set the length to 1)
    #[must_use]
    pub fn normalize(self) -> Self {
        Self::from_vector(self.0.normalize())
    }

    /// Invert provided quaternion
    #[must_use]
    pub const fn invert(self) -> Quaternion {
        let len_sq = self.0.length_sq();

        if len_sq != 0. {
            let inv_length = 1. / len_sq;

            Self::new(
                self.0.x * -inv_length,
                self.0.y * -inv_length,
                self.0.w * -inv_length,
                self.0.z * inv_length,
            )
        } else {
            self
        }
    }

    /// Calculate linear interpolation between this quaternion and another
    #[must_use]
    pub fn lerp(self, end: Self, amount: f32) -> Self {
        Self::from_vector(self.0.lerp(end.0, amount))
    }

    /// Calculate slerp-optimized interpolation between this quaternion and another
    #[must_use]
    pub fn nlerp(self, end: Self, amount: f32) -> Self {
        // use the raylib function since we couldn't do const anyways
        unsafe { raylib_sys::QuaternionNlerp(self.into(), end.into(), amount) }.into()
    }

    /// Calculates spherical linear interpolation between this quaternion and another
    #[must_use]
    pub fn slerp(self, end: Self, amount: f32) -> Self {
        // use the raylib function since we couldn't do const anyways
        unsafe { raylib_sys::QuaternionSlerp(self.into(), end.into(), amount) }.into()
    }

    /// Calculate quaternion cubic spline interpolation using Cubic Hermite Spline algorithm
    /// as described in the GLTF 2.0 specification: https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#interpolation-cubic
    #[must_use]
    pub fn cubic_hermite_spline(
        self,
        out_tangent1: Self,
        q2: Self,
        in_tangent2: Self,
        t: f32,
    ) -> Self {
        // use the raylib function since we couldn't do const anyways
        unsafe {
            raylib_sys::QuaternionCubicHermiteSpline(
                self.into(),
                out_tangent1.into(),
                q2.into(),
                in_tangent2.into(),
                t,
            )
        }
        .into()
    }

    /// Get the rotation angle and axis for a given quaternion
    #[must_use]
    pub fn to_axis_angle(self) -> (Vector3, Angle) {
        let mut axis = Vector3::ZERO;
        let mut angle = 0.;

        // use the raylib function since we couldn't do const anyways
        unsafe {
            raylib_sys::QuaternionToAxisAngle(
                self.into(),
                (&raw mut axis).cast(), // okay because layout is the same
                &raw mut angle,
            )
        }

        (axis, Angle::radians(angle))
    }

    /// Get the Euler angles equivalent to quaternion (roll, pitch, yaw)
    #[must_use]
    pub fn to_euler(self) -> (Angle, Angle, Angle) {
        // use the raylib function since we couldn't do const anyways
        let raylib_sys::Vector3 { x, y, z } = unsafe { raylib_sys::QuaternionToEuler(self.into()) };

        (Angle::radians(x), Angle::radians(y), Angle::radians(z))
    }

    /// Check whether two quaternions are effectively equal
    #[must_use]
    pub const fn effectively_equal(self, other: Self) -> bool {
        const EPSILON: f32 = 0.000001;

        let p = self.0;
        let q = other.0;

        // yoinked from raylib, no idea what this does...
        ((p.x - q.x).abs() <= EPSILON * p.x.abs().max(q.x.abs()).max(1.)
            && (p.y - q.y).abs() <= EPSILON * p.y.abs().max(q.y.abs()).max(1.)
            && (p.z - q.z).abs() <= EPSILON * p.z.abs().max(q.z.abs()).max(1.)
            && (p.w - q.w).abs() <= EPSILON * p.w.abs().max(q.w.abs()).max(1.))
            || ((p.x + q.x).abs() <= EPSILON * p.x.abs().max(q.x.abs()).max(1.)
                && (p.y + q.y).abs() <= EPSILON * p.y.abs().max(q.y.abs()).max(1.)
                && (p.z + q.z).abs() <= EPSILON * p.z.abs().max(q.z.abs()).max(1.)
                && (p.w + q.w).abs() <= EPSILON * p.w.abs().max(q.w.abs()).max(1.))
    }

    /// Transform a quaternion given a transformation matrix
    #[must_use]
    pub const fn transform(self, matrix: Matrix<4, 4>) -> Self {
        let Vector4 { x, y, z, w } = self.0;

        let m = matrix;

        Self::new(
            m.get(0, 0) * x + m.get(0, 1) * y + m.get(0, 2) * z + m.get(0, 3) * w,
            m.get(1, 0) * x + m.get(1, 1) * y + m.get(1, 2) * z + m.get(1, 3) * w,
            m.get(2, 0) * x + m.get(2, 1) * y + m.get(2, 2) * z + m.get(2, 3) * w,
            m.get(3, 0) * x + m.get(3, 1) * y + m.get(3, 2) * z + m.get(3, 3) * w,
        )
    }
}

impl Add<f32> for Quaternion {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        self.add_value(rhs)
    }
}

impl AddAssign<f32> for Quaternion {
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}

impl Sub<f32> for Quaternion {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        self.sub_value(rhs)
    }
}

impl SubAssign<f32> for Quaternion {
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs;
    }
}

impl Mul<f32> for Quaternion {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        self.mul_value(rhs)
    }
}

impl MulAssign<f32> for Quaternion {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div<f32> for Quaternion {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        self.div_value(rhs)
    }
}

impl DivAssign<f32> for Quaternion {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl Mul for Quaternion {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul(rhs)
    }
}
