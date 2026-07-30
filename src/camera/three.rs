use std::sync::atomic::Ordering;

use crate::{
    Canvas,
    color::Color,
    draw::DrawTarget3D,
    globals::{DRAWING_TO_CAMERA, DRAWING_TO_TEXTURE, WINDOW_INITIALISED},
    math::{Angle, Ray, Vector3},
};

use raylib_sys as sys;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CameraProjection {
    Perspective = sys::CameraProjection::CAMERA_PERSPECTIVE as u32,
    Orthographic = sys::CameraProjection::CAMERA_ORTHOGRAPHIC as u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CameraMode {
    Free,        // Camera free mode
    Orbital,     // Camera orbital, around target, zoom supported
    FirstPerson, // Camera first person
    ThirdPerson, // Camera third person
    Custom {
        movement: Vector3,
        rotation: Vector3,
        zoom: f32,
    },
}

impl CameraMode {
    const fn to_sys_mode(self) -> Result<sys::CameraMode, (Vector3, Vector3, f32)> {
        match self {
            CameraMode::Free => Ok(sys::CameraMode::CAMERA_FREE),
            CameraMode::Orbital => Ok(sys::CameraMode::CAMERA_ORBITAL),
            CameraMode::FirstPerson => Ok(sys::CameraMode::CAMERA_FIRST_PERSON),
            CameraMode::ThirdPerson => Ok(sys::CameraMode::CAMERA_THIRD_PERSON),
            CameraMode::Custom {
                movement,
                rotation,
                zoom,
            } => Err((movement, rotation, zoom)),
        }
    }
}

#[derive(bauer::Builder, Clone, Copy, Debug)]
pub struct Camera3D {
    /// Camera position
    ///
    /// default = `<10, 10, 10>`
    #[builder(default = "Vector3::value(10.)", into)]
    pub position: Vector3,
    /// Camera target it looks-at
    ///
    /// default = `Vector3::ZERO`
    #[builder(default = "Vector3::ZERO", into)]
    pub target: Vector3,
    /// Camera up vector (rotation over its axis)
    ///
    /// default = `Vector3::UNIT_Y`
    #[builder(default = "Vector3::UNIT_Y", into)]
    pub up: Vector3,
    /// Camera field-of-view aperture in Y in perspective, used as near plane height in
    /// world units in orthographic
    ///
    /// default = `45°`
    #[builder(default = "Angle::degrees(45.)", into)]
    pub fovy: Angle,
    /// Camera projection
    ///
    /// default = `CameraProjection::Perspective`
    #[builder(default = "CameraProjection::Perspective")]
    pub projection: CameraProjection,
}

impl Default for Camera3D {
    /// Generate a default camera with:
    ///
    /// - `offset` = [`Vector2::ZERO`]
    /// - `target` = [`Vector2::ZERO`]
    /// - `rotation` = `0.0`
    /// - `zoom` = `1.0`
    fn default() -> Self {
        Self::builder().build()
    }
}

impl Camera3D {
    fn to_sys(self) -> sys::Camera3D {
        sys::Camera3D {
            position: self.position.into(),
            target: self.target.into(),
            up: self.up.into(),
            fovy: self.fovy.to_degrees(),
            projection: self.projection as _,
        }
    }

    fn from_sys(camera: sys::Camera3D) -> Self {
        const {
            assert!(sys::CameraProjection::CAMERA_PERSPECTIVE as u32 == 0);
            assert!(sys::CameraProjection::CAMERA_ORTHOGRAPHIC as u32 == 1);
        };
        Self {
            position: camera.position.into(),
            target: camera.target.into(),
            up: camera.up.into(),
            fovy: Angle::degrees(camera.fovy),
            projection: match camera.projection {
                // asserted values above
                0 => CameraProjection::Perspective,
                1 => CameraProjection::Orthographic,
                _ => unreachable!("Invalid sys camera projection"),
            },
        }
    }

    pub fn update(&mut self, mode: CameraMode) {
        let mut sys_cam = self.to_sys();
        match mode.to_sys_mode() {
            Ok(mode) => unsafe { sys::UpdateCamera(&raw mut sys_cam, mode as _) },
            Err((movement, rotation, zoom)) => unsafe {
                sys::UpdateCameraPro(&raw mut sys_cam, movement.into(), rotation.into(), zoom)
            },
        }
        *self = Self::from_sys(sys_cam);
    }
}

pub struct Camera3DCanvas<'window, 'canvas> {
    canvas: &'canvas mut Canvas<'window>,
    _camera: Camera3D,
}

impl<'w, 'c> Camera3DCanvas<'w, 'c> {
    pub(crate) fn new(canvas: &'c mut Canvas<'w>, camera: Camera3D) -> Self {
        if DRAWING_TO_CAMERA
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire)
            .is_err()
        {
            panic!("Only one camera may be drawn to at a time.");
        }
        unsafe { sys::BeginMode3D(camera.to_sys()) }
        Self {
            canvas,
            _camera: camera,
        }
    }

    #[inline]
    fn can_draw() -> bool {
        // window is initialised and we are not drawing to a texture
        WINDOW_INITIALISED.load(Ordering::Acquire)
            && !DRAWING_TO_TEXTURE.load(Ordering::Acquire)
            && DRAWING_TO_CAMERA.load(Ordering::Acquire)
    }

    #[inline]
    fn assert_can_draw() {
        assert!(
            WINDOW_INITIALISED.load(Ordering::Acquire),
            "Attempting to draw without a window initialised"
        );
        assert!(
            !DRAWING_TO_TEXTURE.load(Ordering::Acquire),
            "Cannot draw to frame while drawing to texture"
        );
        assert!(
            DRAWING_TO_CAMERA.load(Ordering::Acquire),
            "Attempting to draw to uninitialsed camera"
        );
        assert!(Self::can_draw());
    }

    // A bit more ergonmic way to drop self
    pub fn end(self) {
        drop(self);
    }
}

impl<'w, 'c> Drop for Camera3DCanvas<'w, 'c> {
    fn drop(&mut self) {
        if DRAWING_TO_CAMERA
            .compare_exchange(true, false, Ordering::Acquire, Ordering::Acquire)
            .is_err()
        {
            panic!("Attempted to end camera drawing without calling BeginCamera2D");
        }
        unsafe { sys::EndMode3D() }
    }
}

impl DrawTarget3D for Camera3DCanvas<'_, '_> {
    fn draw_line(&mut self, start: impl Into<Vector3>, end: impl Into<Vector3>, color: Color) {
        Self::assert_can_draw();
        unsafe { sys::DrawLine3D(start.into().into(), end.into().into(), color.into()) };
    }

    fn draw_point(&mut self, point: impl Into<Vector3>, color: Color) {
        Self::assert_can_draw();
        unsafe { sys::DrawPoint3D(point.into().into(), color.into()) };
    }

    fn draw_circle(
        &mut self,
        center: impl Into<Vector3>,
        radius: f32,
        rotation_axis: impl Into<Vector3>,
        rotation_angle: Angle,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawCircle3D(
                center.into().into(),
                radius,
                rotation_axis.into().into(),
                rotation_angle.to_radians(), // TODO: Double check
                color.into(),
            )
        };
    }

    fn draw_triangle(
        &mut self,
        p1: impl Into<Vector3>,
        p2: impl Into<Vector3>,
        p3: impl Into<Vector3>,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawTriangle3D(
                p1.into().into(),
                p2.into().into(),
                p3.into().into(),
                color.into(),
            )
        };
    }

    fn draw_triangle_strip(&mut self, points: &[Vector3], color: Color) {
        Self::assert_can_draw();
        // cast here is fine because both Vector2s have the same layout
        unsafe {
            sys::DrawTriangleStrip3D(points.as_ptr().cast(), points.len() as _, color.into())
        };
    }

    fn draw_cube(&mut self, position: impl Into<Vector3>, size: impl Into<Vector3>, color: Color) {
        Self::assert_can_draw();
        unsafe { sys::DrawCubeV(position.into().into(), size.into().into(), color.into()) };
    }

    fn draw_cube_wires(
        &mut self,
        position: impl Into<Vector3>,
        size: impl Into<Vector3>,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe { sys::DrawCubeWiresV(position.into().into(), size.into().into(), color.into()) };
    }

    fn draw_sphere_ex(
        &mut self,
        center: impl Into<Vector3>,
        radius: f32,
        rings: u32,
        slices: u32,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawSphereEx(
                center.into().into(),
                radius,
                rings as _,
                slices as _,
                color.into(),
            )
        };
    }

    fn draw_sphere_wires_ex(
        &mut self,
        center: impl Into<Vector3>,
        radius: f32,
        rings: u32,
        slices: u32,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawSphereWires(
                center.into().into(),
                radius,
                rings as _,
                slices as _,
                color.into(),
            )
        };
    }

    fn draw_cylinder(
        &mut self,
        start: impl Into<Vector3>,
        end: impl Into<Vector3>,
        radius_start: f32,
        radius_end: f32,
        sides: u32,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawCylinderEx(
                start.into().into(),
                end.into().into(),
                radius_start,
                radius_end,
                sides as _,
                color.into(),
            )
        };
    }

    fn draw_cylinder_wires(
        &mut self,
        start: impl Into<Vector3>,
        end: impl Into<Vector3>,
        radius_start: f32,
        radius_end: f32,
        sides: u32,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawCylinderWiresEx(
                start.into().into(),
                end.into().into(),
                radius_start,
                radius_end,
                sides as _,
                color.into(),
            )
        };
    }

    fn draw_capsule(
        &mut self,
        start: impl Into<Vector3>,
        end: impl Into<Vector3>,
        radius: f32,
        slices: u32,
        rings: u32,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawCapsule(
                start.into().into(),
                end.into().into(),
                radius,
                slices as _,
                rings as _,
                color.into(),
            )
        };
    }

    fn draw_capsule_wires(
        &mut self,
        start: impl Into<Vector3>,
        end: impl Into<Vector3>,
        radius: f32,
        slices: u32,
        rings: u32,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawCapsuleWires(
                start.into().into(),
                end.into().into(),
                radius,
                slices as _,
                rings as _,
                color.into(),
            )
        };
    }

    fn draw_plane(
        &mut self,
        center: impl Into<Vector3>,
        size: impl Into<crate::prelude::Vector2>,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe { sys::DrawPlane(center.into().into(), size.into().into(), color.into()) };
    }

    fn draw_ray(&mut self, ray: Ray, color: Color) {
        Self::assert_can_draw();
        unsafe { sys::DrawRay(ray.into(), color.into()) };
    }

    fn draw_grid(&mut self, slices: u32, spacing: f32) {
        Self::assert_can_draw();
        unsafe { sys::DrawGrid(slices as _, spacing) };
    }
}
