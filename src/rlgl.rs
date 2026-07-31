use raylib_sys as sys;

use crate::{
    color::Color,
    math::{Angle, Vector2, Vector3, Vector4},
    sealed::Sealed,
    texture::Texture2D,
};

#[non_exhaustive]
pub struct RlMatrix;

pub fn with_matrix<F>(f: F)
where
    F: FnOnce(&mut RlMatrix),
{
    unsafe { sys::rlPushMatrix() };
    f(&mut RlMatrix);
    unsafe { sys::rlPopMatrix() };
}

/// Matrix operations
impl RlMatrix {
    /// Reset current matrix to identity matrix
    pub fn load_identity(&mut self) {
        unsafe { sys::rlLoadIdentity() };
    }

    /// Multiply the current matrix by a translation matrix
    pub fn translate(&mut self, v: Vector3) {
        unsafe { sys::rlTranslatef(v.x, v.y, v.z) };
    }

    /// Multiply the current matrix by a rotation matrix
    pub fn rotate(&mut self, angle: Angle, axis: Vector3) {
        unsafe { sys::rlRotatef(angle.to_degrees(), axis.x, axis.y, axis.z) };
    }

    /// Multiply the current matrix by a scaling matrix
    pub fn scale(&mut self, scale: Vector3) {
        unsafe { sys::rlScalef(scale.x, scale.y, scale.z) };
    }

    /// Multiply the current matrix by another matrix
    pub fn mult_matrix(&mut self, matrix: [f32; 16]) {
        unsafe { sys::rlMultMatrixf(matrix.as_ptr()) };
    }

    pub fn frustum(&mut self, left: f64, right: f64, bottom: f64, top: f64, znear: f64, zfar: f64) {
        unsafe { sys::rlFrustum(left, right, bottom, top, znear, zfar) };
    }

    pub fn ortho(&mut self, left: f64, right: f64, bottom: f64, top: f64, znear: f64, zfar: f64) {
        unsafe { sys::rlOrtho(left, right, bottom, top, znear, zfar) };
    }

    /// Set the viewport area
    pub fn viewport(&mut self, x: i32, y: i32, width: u32, height: u32) {
        unsafe { sys::rlViewport(x, y, width as _, height as _) };
    }

    /// Set clip planes distances
    pub fn set_clip_planes(&mut self, near: f64, far: f64) {
        unsafe { sys::rlSetClipPlanes(near, far) };
    }

    pub fn get_cull_distance_near(&self) -> f64 {
        unsafe { sys::rlGetCullDistanceNear() }
    }

    pub fn get_cull_distance_far(&self) -> f64 {
        unsafe { sys::rlGetCullDistanceNear() }
    }
}

/// Check internal buffer overflow for a given number of vertex
pub fn check_render_batch_limit(v_count: u32) -> bool {
    unsafe { sys::rlCheckRenderBatchLimit(v_count as _) }
}

pub fn set_texture(texture: &Texture2D) {
    let id = texture.inner_ref().id;
    assert!(id != 0);
    unsafe { sys::rlSetTexture(id) }
}

pub fn unset_texture() {
    unsafe { sys::rlSetTexture(0) }
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum DrawingMode {
    Lines = 0,
    Triangles = 4,
    Quads = 7,
}

#[non_exhaustive]
pub struct DrawingCtx;

/// Initialize drawing mode and finish vertex providing when done
pub fn drawing_mode<F>(mode: DrawingMode, f: F)
where
    F: FnOnce(&mut DrawingCtx),
{
    unsafe { sys::rlBegin(mode as _) };
    f(&mut DrawingCtx);
    unsafe { sys::rlEnd() };
}

impl Sealed for Color {}
impl Sealed for Vector2 {}
impl Sealed for (f32, f32) {}
impl Sealed for (i32, i32) {}
impl Sealed for Vector3 {}
impl Sealed for (f32, f32, f32) {}
impl Sealed for Vector4 {}

pub trait VertexArg: Sealed {
    fn apply(&self);
}

impl VertexArg for Vector2 {
    fn apply(&self) {
        unsafe { sys::rlVertex2f(self.x, self.y) };
    }
}

impl VertexArg for (f32, f32) {
    fn apply(&self) {
        VertexArg::apply(&Vector2::from(*self))
    }
}

impl VertexArg for (i32, i32) {
    fn apply(&self) {
        unsafe { sys::rlVertex2i(self.0, self.1) };
    }
}

impl VertexArg for Vector3 {
    fn apply(&self) {
        unsafe { sys::rlVertex3f(self.x, self.y, self.z) };
    }
}

impl VertexArg for (f32, f32, f32) {
    fn apply(&self) {
        VertexArg::apply(&Vector3::from(*self))
    }
}

pub trait ColorArg: Sealed {
    fn apply(&self);
}

impl ColorArg for Color {
    fn apply(&self) {
        unsafe { sys::rlColor4ub(self.r, self.g, self.b, self.a) };
    }
}
impl ColorArg for Vector3 {
    fn apply(&self) {
        unsafe { sys::rlColor3f(self.x, self.y, self.z) };
    }
}
impl ColorArg for Vector4 {
    fn apply(&self) {
        unsafe { sys::rlColor4f(self.x, self.y, self.z, self.w) };
    }
}

/// Vertex level operations
impl DrawingCtx {
    pub fn vertex(&mut self, vertex: impl VertexArg) -> &mut Self {
        vertex.apply();
        self
    }

    pub fn color(&mut self, color: impl ColorArg) -> &mut Self {
        color.apply();
        self
    }

    pub fn tex_coord(&mut self, coord: impl Into<Vector2>) -> &mut Self {
        let coord = coord.into();
        unsafe { sys::rlTexCoord2f(coord.x, coord.y) };
        self
    }

    pub fn normal(&mut self, normal: impl Into<Vector3>) -> &mut Self {
        let normal = normal.into();
        unsafe { sys::rlNormal3f(normal.x, normal.y, normal.z) };
        self
    }
}
