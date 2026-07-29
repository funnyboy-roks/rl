use std::ops::{Index, IndexMut};

use crate::math::{Angle, Vector3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<const M: usize, const N: usize> {
    values: [[f32; N]; M],
}

macro_rules! for_each {
    ($var: ident in $max: expr => $($tt: tt)*) => {
        let mut $var = 0;
        let max = $max;
        while $var < max {
            $($tt)*
            $var += 1;
        }
    };
    (($i: ident, $j: ident) in ($maxi: expr, $maxj: expr) => $($tt: tt)*) => {
        for_each! {
            $i in $maxi =>
            for_each! {
                $j in $maxj =>
                $($tt)*
            }
        }
    };
}

impl<const M: usize, const N: usize> Index<(usize, usize)> for Matrix<M, N> {
    type Output = f32;

    fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
        assert!(row < M && column < N);
        &self.values[row][column]
    }
}

impl<const M: usize, const N: usize> IndexMut<(usize, usize)> for Matrix<M, N> {
    fn index_mut(&mut self, (row, column): (usize, usize)) -> &mut Self::Output {
        assert!(row < M && column < N);
        &mut self.values[row][column]
    }
}

impl<const M: usize, const N: usize> Matrix<M, N> {
    pub const fn fill(fill: f32) -> Self {
        Self::from_values([[fill; N]; M])
    }

    pub const fn from_values(values: [[f32; N]; M]) -> Self {
        Self { values }
    }

    #[inline]
    pub const fn get(&self, row: usize, column: usize) -> f32 {
        debug_assert!(row < M && column < N);
        self.values[row][column]
    }

    #[inline]
    pub const fn try_get(&self, row: usize, column: usize) -> Option<f32> {
        if row > M || column > N {
            return None;
        }
        Some(self.get(row, column))
    }

    #[inline]
    pub const fn get_mut(&mut self, row: usize, column: usize) -> &mut f32 {
        debug_assert!(row < M && column < N);
        &mut self.values[row][column]
    }

    #[inline]
    pub const fn try_get_mut(&mut self, row: usize, column: usize) -> Option<&mut f32> {
        if row > M || column > N {
            return None;
        }
        Some(&mut self.values[row][column])
    }

    /// Resize this matrix to a different size
    ///
    /// All values outside of the new matrix bounds will be dropped.  If the new matrix bounds are
    /// larger than the existing bounds, then `fill` will be used to populate those values.
    pub const fn resize<const NR: usize, const NC: usize>(self, fill: f32) -> Matrix<NR, NC> {
        let mut mat = Matrix::fill(fill);

        for_each! { (i, j) in (NR, NC) =>
            if let Some(v) = self.try_get(i, j) {
                *mat.get_mut(i, j) = v;
            }
        }

        mat
    }

    pub const fn sub_matrix<const NR: usize, const NC: usize>(
        &self,
        row: usize,
        column: usize,
    ) -> Option<Matrix<NR, NC>> {
        const { assert!(NR <= M && NC <= N, "sub matrix must be smaller than matrix") };

        if row + NR > M || column + NC > N {
            return None;
        }

        // this would be cleaner with from_fn, but no const :(
        let mut mat = Matrix::fill(0.);

        for_each! { (i, j) in (NR, NC) =>
            *mat.get_mut(i,j) = self.values[row + i][column + j];
        }

        Some(mat)
    }

    pub const fn transpose(self) -> Matrix<N, M> {
        let mut mat = Matrix::fill(0.);

        for_each! { (i, j) in (M, N) =>
            *mat.get_mut(j, i) = self.get(i, j);
        }

        mat
    }

    pub const fn mul<const P: usize>(self, rhs: Matrix<N, P>) -> Matrix<M, P> {
        let mut mat = Matrix::fill(0.);

        for_each! { (i, j) in (M, P) =>
            let sum = mat.get_mut(i, j);
            for_each! { k in N =>
                *sum += self.get(i, k) * rhs.get(k, j);
            }
        }

        mat
    }

    pub const fn add(self, rhs: Matrix<M, N>) -> Matrix<M, N> {
        let mut out = self;
        for_each! { (i, j) in (M, N) => *out.get_mut(i, j) += rhs.get(i, j); }
        out
    }

    pub const fn sub(self, rhs: Matrix<M, N>) -> Matrix<M, N> {
        let mut out = self;
        for_each! { (i, j) in (M, N) => *out.get_mut(i, j) -= rhs.get(i, j); }
        out
    }

    pub const fn add_value(self, value: f32) -> Matrix<M, N> {
        let mut out = self;
        for_each! { (i, j) in (M, N) => *out.get_mut(i, j) += value; }
        out
    }

    pub const fn sub_value(self, value: f32) -> Matrix<M, N> {
        self.add_value(-value)
    }

    pub const fn mul_value(self, value: f32) -> Matrix<M, N> {
        let mut out = self;
        for_each! { (i, j) in (M, N) => *out.get_mut(i, j) *= value; }
        out
    }

    /// Present the contents of this matrix as a slice (row-wise)
    pub const fn as_slice(&self) -> &[f32] {
        self.values.as_flattened()
    }

    pub const fn as_array(self) -> [[f32; N]; M] {
        self.values
    }

    const fn assert_size<const AM: usize, const AN: usize>(self) -> Matrix<AM, AN> {
        assert!(AM == M);
        assert!(AN == N);
        // compiler: please optimise this
        let mut out = Matrix::<AM, AN>::fill(0.);
        out.values
            .as_flattened_mut()
            .copy_from_slice(self.values.as_flattened());
        out
    }
}

/// Square matricies
impl<const N: usize> Matrix<N, N> {
    pub const fn identity() -> Self {
        let mut mat = Self::fill(0.);
        for_each! { i in N =>
            mat.values[i][i] = 1.;
        }
        mat
    }

    pub const fn det(self) -> f32 {
        match N {
            0 => panic!("Invalid matrix size"),
            1 => self.get(0, 0),
            2 => {
                let [[a, b], [c, d]] = self.assert_size::<2, 2>().values;
                a * d - b * c
            }
            3 => {
                let [[a, b, c], [d, e, f], [g, h, i]] = self.assert_size::<3, 3>().values;

                // https://en.wikipedia.org/wiki/Determinant#Leibniz_formula
                a * e * i + b * f * g + c * d * h - c * e * g - b * d * i - a * f * h
            }
            4 => {
                // Using Laplace expansion (https://en.wikipedia.org/wiki/Laplace_expansion),
                // previous operation can be simplified to 40 multiplications, decreasing matrix
                // size from 4x4 to 2x2 using minors

                let [
                    [m0, m4, m8, m12],
                    [m1, m5, m9, m13],
                    [m2, m6, m10, m14],
                    [m3, m7, m11, m15],
                ] = self.assert_size::<4, 4>().values;

                m0 * (m5 * (m10 * m15 - m11 * m14) - m9 * (m6 * m15 - m7 * m14)
                    + m13 * (m6 * m11 - m7 * m10))
                    - m4 * (m1 * (m10 * m15 - m11 * m14) - m9 * (m2 * m15 - m3 * m14)
                        + m13 * (m2 * m11 - m3 * m10))
                    + m8 * (m1 * (m6 * m15 - m7 * m14) - m5 * (m2 * m15 - m3 * m14)
                        + m13 * (m2 * m7 - m3 * m6))
                    - m12
                        * (m1 * (m6 * m11 - m7 * m10) - m5 * (m2 * m11 - m3 * m10)
                            + m9 * (m2 * m7 - m3 * m6))
            }
            _ => panic!("determinant is not implemented for matricies of size"), // for this library's purpose, up to four is enough
        }
    }

    pub const fn trace(self) -> f32 {
        let mut sum = 0.;
        for_each! { i in N =>
            sum += self.get(i, i);
        }
        sum
    }
}

impl Matrix<4, 4> {
    const fn from_sys(sys: raylib_sys::Matrix) -> Self {
        Self::from_values([
            [sys.m0, sys.m4, sys.m8, sys.m12],
            [sys.m1, sys.m5, sys.m9, sys.m13],
            [sys.m2, sys.m6, sys.m10, sys.m14],
            [sys.m3, sys.m7, sys.m11, sys.m15],
        ])
    }

    pub fn from_rotation(axis: Vector3, angle: Angle) -> Self {
        let Vector3 { x, y, z } = axis.normalize();

        let (sin, cos) = angle.to_radians().sin_cos();
        let t = 1. - cos;

        Self::from_values([
            [
                x * x * t + cos,
                y * x * t + z * sin,
                z * x * t - y * sin,
                0.,
            ],
            [
                x * y * t - z * sin,
                y * y * t + cos,
                z * y * t + x * sin,
                0.,
            ],
            [
                x * z * t + y * sin,
                y * z * t - x * sin,
                z * z * t + cos,
                0.,
            ],
            [0., 0., 0., 1.],
        ])
    }

    /// Get x-rotation matrix
    pub fn rotate_x(angle: Angle) -> Self {
        let (sin, cos) = angle.sin_cos();

        #[rustfmt::skip]
        let x = Self::from_values([
            [1.,   0.,    0.,  0.],
            [0.,  cos,  -sin,  0.],
            [0.,  sin,   cos,  0.],
            [0.,   0.,    0.,  1.],
        ]);
        x
    }

    /// Get y-rotation matrix
    pub fn rotate_y(angle: Angle) -> Self {
        let (sin, cos) = angle.sin_cos();

        #[rustfmt::skip]
        let x = Self::from_values([
            [ cos,  0.,  sin,  0.],
            [  0.,  1.,   0.,  0.],
            [-sin,  0.,  cos,  0.],
            [  0.,  0.,   0.,  1.],
        ]);
        x
    }

    /// Get z-rotation matrix
    pub fn rotate_z(angle: Angle) -> Self {
        let (sin, cos) = angle.sin_cos();

        #[rustfmt::skip]
        let x = Self::from_values([
            [cos,  -sin,  0.,  0.],
            [sin,   cos,  0.,  0.],
            [ 0.,    0.,  1.,  0.],
            [ 0.,    0.,  0.,  1.],
        ]);
        x
    }

    /// Get xyz-rotation matrix
    pub fn rotate_xyz([x, y, z]: [Angle; 3]) -> Self {
        let (sx, cx) = x.sin_cos();
        let (sy, cy) = y.sin_cos();
        let (sz, cz) = z.sin_cos();

        #[rustfmt::skip]
        let x = Self::from_values([
            [cz*cy,                sz*cy,                -sy,   0.],
            [(cz*sy*sx) - (sz*cx), (sz*sy*sx) + (cz*cx), cy*sx, 0.],
            [(cz*sy*cx) + (sz*sx), (sz*sy*cx) - (cz*sx), cy*cx, 0.],
            [ 0.,                  0.,                   0.,    1.],
        ]);
        x
    }

    /// Get zyx-rotation matrix
    pub fn rotate_zyx([x, y, z]: [Angle; 3]) -> Self {
        let (sx, cx) = x.sin_cos();
        let (sy, cy) = y.sin_cos();
        let (sz, cz) = z.sin_cos();

        #[rustfmt::skip]
        let x = Self::from_values([
            [cz*cy,   cz * sy * sx - cx * sz, sz * sx + cz * cx * sy, 0.],
            [cy * sz, cz * cx + sz * sy * sx, cx * sz * sy - cz * sx, 0.],
            [-sy,     cy * sx,                cy * cx,                0.],
            [ 0.,     0.,                     0.,                     1.],
        ]);
        x
    }

    pub const fn scaling(x: f32, y: f32, z: f32) -> Self {
        Self::from_values([
            [x, 0., 0., 0.],
            [0., y, 0., 0.],
            [0., 0., z, 0.],
            [0., 0., 0., 1.],
        ])
    }

    pub const fn frustrum(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near_plane: f32,
        far_plane: f32,
    ) -> Self {
        let rl = right - left;
        let tb = top - bottom;
        let fmn = far_plane - near_plane;

        Self::from_values([
            [(near_plane * 2.) / rl, 0., (right + left) / rl, 0.],
            [0., (near_plane * 2.) / tb, (top + bottom) / tb, 0.],
            [
                0.,
                0.,
                -(far_plane + near_plane) / fmn,
                -(far_plane * near_plane * 2.) / fmn,
            ],
            [0., 0., -1., 0.],
        ])
    }

    /// Get perspective projection matrix
    pub fn perspective(fov_y: Angle, aspect: f32, near_plane: f32, far_plane: f32) -> Self {
        Self::from_sys(raylib_sys::Matrix::perspective(
            fov_y.to_radians() as _,
            aspect as _,
            near_plane as _,
            far_plane as _,
        ))
    }

    /// Get orthographic projection matrix
    pub const fn ortho(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near_plane: f32,
        far_plane: f32,
    ) -> Self {
        let rl = right - left;
        let tb = top - bottom;
        let f_n = far_plane - near_plane;

        let m0 = 2. / rl;
        let m4 = 0.;
        let m8 = 0.;
        let m12 = -(left + right) / rl;

        let m1 = 0.;
        let m5 = 2. / tb;
        let m9 = 0.;
        let m13 = -(top + bottom) / tb;

        let m2 = 0.;
        let m6 = 0.;
        let m10 = -2. / f_n;
        let m14 = -(far_plane + near_plane) / f_n;

        let m3 = 0.;
        let m7 = 0.;
        let m11 = 0.;
        let m15 = 1.;

        #[rustfmt::skip]
        let x = Self::from_values([
            [m0, m4, m8, m12],
            [m1, m5, m9, m13],
            [m2, m6, m10, m14],
            [m3, m7, m11, m15],
        ]);
        x
    }

    /// Get camera look-at matrix (view matrix)
    pub fn look_at(eye: Vector3, target: Vector3, up: Vector3) -> Self {
        Self::from_sys(raylib_sys::Matrix::look_at(
            eye.into(),
            target.into(),
            up.into(),
        ))
    }

    pub const fn invert(self) -> Self {
        // taken from raylib, so only on 4x4
        let [
            [a00, a01, a02, a03],
            [a10, a11, a12, a13],
            [a20, a21, a22, a23],
            [a30, a31, a32, a33],
        ] = self.values;

        let b00 = a00 * a11 - a01 * a10;
        let b01 = a00 * a12 - a02 * a10;
        let b02 = a00 * a13 - a03 * a10;
        let b03 = a01 * a12 - a02 * a11;
        let b04 = a01 * a13 - a03 * a11;
        let b05 = a02 * a13 - a03 * a12;
        let b06 = a20 * a31 - a21 * a30;
        let b07 = a20 * a32 - a22 * a30;
        let b08 = a20 * a33 - a23 * a30;
        let b09 = a21 * a32 - a22 * a31;
        let b10 = a21 * a33 - a23 * a31;
        let b11 = a22 * a33 - a23 * a32;

        // Calculate the invert determinant (inlined to avoid double-caching)
        let inv_det = 1. / (b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06);

        Self::from_values([
            [
                (a11 * b11 - a12 * b10 + a13 * b09) * inv_det,
                (-a10 * b11 + a12 * b08 - a13 * b07) * inv_det,
                (a10 * b10 - a11 * b08 + a13 * b06) * inv_det,
                (-a10 * b09 + a11 * b07 - a12 * b06) * inv_det,
            ],
            [
                (-a01 * b11 + a02 * b10 - a03 * b09) * inv_det,
                (a00 * b11 - a02 * b08 + a03 * b07) * inv_det,
                (-a00 * b10 + a01 * b08 - a03 * b06) * inv_det,
                (a00 * b09 - a01 * b07 + a02 * b06) * inv_det,
            ],
            [
                (a31 * b05 - a32 * b04 + a33 * b03) * inv_det,
                (-a30 * b05 + a32 * b02 - a33 * b01) * inv_det,
                (a30 * b04 - a31 * b02 + a33 * b00) * inv_det,
                (-a30 * b03 + a31 * b01 - a32 * b00) * inv_det,
            ],
            [
                (-a21 * b05 + a22 * b04 - a23 * b03) * inv_det,
                (a20 * b05 - a22 * b02 + a23 * b01) * inv_det,
                (-a20 * b04 + a21 * b02 - a23 * b00) * inv_det,
                (a20 * b03 - a21 * b01 + a22 * b00) * inv_det,
            ],
        ])
    }
}

#[cfg(test)]
mod test {
    use crate::math::matrix::Matrix;

    #[test]
    fn test() {
        let mat = Matrix::from_values([
            [0., 4., 8., 12.],
            [1., 5., 9., 13.],
            [2., 6., 10., 14.],
            [3., 7., 11., 15.],
        ]);
        dbg!(mat);
        dbg!(mat.transpose());
        let mat2 = dbg!(mat.sub_matrix::<2, 3>(1, 1)).unwrap();
        dbg!(mat2.transpose());
    }

    #[test]
    fn multiplication() {
        #[rustfmt::skip]
        let a = Matrix::from_values([
            [2., 3., 4.],
            [1., 0., 0.],
        ]);
        #[rustfmt::skip]
        let b = Matrix::from_values([
            [0., 1000.],
            [1., 100.],
            [0., 10.],
        ]);

        assert_eq!(a.mul(b), Matrix::from_values([[3., 2340.], [0., 1000.]]))
    }

    #[test]
    fn determinant() {
        let a = Matrix::from_values([[69.]]);
        assert_eq!(a.det(), 69.);
        let a = Matrix::from_values([[3., 8.], [4., 6.]]);
        assert_eq!(a.det(), -14.);
        let a = Matrix::from_values([[6., 1., 1.], [4., -2., 5.], [2., 8., 7.]]);
        assert_eq!(a.det(), -306.);
        let a = Matrix::from_values([
            [3., 2., 1., 6.],
            [6., 3., 3., 5.],
            [1., 4., 5., 9.],
            [6., 3., 7., 9.],
        ]);
        assert_eq!(a.det(), -292.);
    }
}
