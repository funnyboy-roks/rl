use std::sync::atomic::Ordering;

use derive_more::{Deref, DerefMut};

use crate::camera::{Camera2D, Camera2DCanvas, Camera3D, Camera3DCanvas};
use crate::color::Color;
use crate::draw::{DrawTarget2D, DrawTarget2DFull};
use crate::globals::{DRAWING_TO_CAMERA, DRAWING_TO_TEXTURE, WINDOW_INITIALISED};
use crate::image::Image;
use crate::input::{Gamepad, Keyboard, Mouse};
use crate::math::{Angle, Vector2};
use crate::text::Font;
use crate::texture::Texture2D;
use crate::util::allocate_cstring;
use crate::window::Window;

pub use raylib_sys::{self as sys, Rectangle};

pub mod bytes;
pub mod camera;
pub mod color;
pub mod draw;
mod globals;
pub mod image;
pub mod input;
pub mod math;
pub mod rand;
pub mod rlgl;
pub mod shader;
pub mod text;
pub mod texture;
mod util;
pub mod window;

mod sealed {
    pub trait Sealed {}
}

pub mod prelude {
    pub use crate::{
        Bounded, Rectangle,
        camera::{Camera2D, Camera3D, CameraMode, CameraProjection},
        color::Color,
        draw::{DrawTarget2D, DrawTarget2DFull, DrawTarget3D},
        image::{FileType, Image, ImageResizeMode},
        input::{Gamepad, GamepadAxis, GamepadButton, Key, MouseButton},
        math::{Angle, Matrix, Ray, Vector2, Vector3, Vector4},
        rand::Random,
        shader,
        shader::Shader,
        text::Font,
        texture::{RenderTexture2D, Texture2D},
        window::{ConfigFlags, Window},
    };
}

pub trait Bounded {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn size(&self) -> Vector2 {
        Vector2::new(self.width() as _, self.height() as _)
    }
    fn bounds(&self) -> sys::Rectangle {
        Rectangle::new(0., 0., self.width() as _, self.height() as _)
    }
}

#[derive(Deref, DerefMut)]
pub struct Canvas<'window> {
    frame: Frame<'window>,
}

pub struct Frame<'window> {
    window: &'window mut Window,
}

impl<'w> Frame<'w> {
    /// # SAFETY
    ///
    /// This function should only be called if `WindowShouldClose` has been called already.
    pub(crate) unsafe fn new(arg: &'w mut Window) -> Self {
        Self { window: arg }
    }

    /// # SAFETY
    ///
    /// This is a placeholder value.  If it is used, things will break.
    unsafe fn empty() -> Frame<'static> {
        Frame {
            // SAFETY: NOT SAFE!!!! DO NOT DEREFERENCE!
            #[expect(invalid_value)]
            window: unsafe { std::mem::transmute::<usize, &'static mut Window>(0_usize) },
        }
    }

    pub fn window(&self) -> &Window {
        self.window
    }

    pub fn window_mut(&mut self) -> &mut Window {
        self.window
    }

    pub fn mouse(&self) -> Mouse {
        Mouse
    }

    pub fn keyboard(&self) -> Keyboard {
        Keyboard
    }

    pub fn gamepad(&self, gamepad: u32) -> Option<Gamepad> {
        Gamepad::new(gamepad)
    }

    pub fn get_time(&self) -> f32 {
        unsafe { sys::GetFrameTime() }
    }

    /// The number of frames that have run
    pub fn count(&self) -> u64 {
        // Because we increment at the start, we need to subtract one for an accurate count
        self.window().frame_count - 1
    }

    pub fn begin_drawing(self) -> Canvas<'w> {
        Canvas::assert_can_draw();
        unsafe { sys::BeginDrawing() };
        Canvas { frame: self }
    }

    pub fn with_canvas<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Canvas<'w>),
    {
        // SAFETY: we never use &mut self again.  (until we give it a valid value again
        let this = std::mem::replace(self, unsafe { Self::empty() });
        let mut canvas = this.begin_drawing();
        f(&mut canvas);
        canvas.end();
    }
}

impl Bounded for Frame<'_> {
    fn width(&self) -> u32 {
        self.window().width()
    }

    fn height(&self) -> u32 {
        self.window().height()
    }
}

impl<'w> Canvas<'w> {
    /// End drawing the current frame
    ///
    /// This function should only be needed if you wish to use the frame again after drawing,
    /// otherwise, drop will make the appropriate call
    pub fn end(self) -> Frame<'w> {
        let mut this = self;
        std::mem::replace(
            &mut this.frame,
            // SAFETY: This is okay because the destructor does not attempt to access `self.frame`
            unsafe { Frame::empty() },
        )
    }

    #[inline]
    fn can_draw() -> bool {
        // window is initialised and we are not drawing to a texture
        WINDOW_INITIALISED.load(Ordering::Acquire)
            && !DRAWING_TO_TEXTURE.load(Ordering::Acquire)
            && !DRAWING_TO_CAMERA.load(Ordering::Acquire)
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
        assert!(Self::can_draw());
    }

    pub fn begin_camera_2d<'c>(&'c mut self, camera: Camera2D) -> Camera2DCanvas<'w, 'c> {
        Camera2DCanvas::new(self, camera)
    }

    pub fn with_camera_2d<'c, F>(&'c mut self, camera: Camera2D, f: F)
    where
        F: FnOnce(&mut Camera2DCanvas<'w, 'c>),
    {
        let mut cc = Camera2DCanvas::new(self, camera);
        f(&mut cc);
        drop(cc);
    }

    pub fn begin_camera_3d<'c>(&'c mut self, camera: Camera3D) -> Camera3DCanvas<'w, 'c> {
        Camera3DCanvas::new(self, camera)
    }

    pub fn with_camera_3d<'c, F>(&'c mut self, camera: Camera3D, f: F)
    where
        F: FnOnce(&mut Camera3DCanvas<'w, 'c>),
    {
        let mut cc = Camera3DCanvas::new(self, camera);
        f(&mut cc);
        drop(cc);
    }

    pub fn draw_fps(&mut self, x: i32, y: i32) {
        Self::assert_can_draw();
        unsafe { sys::DrawFPS(x, y) }
    }
}

impl Drop for Canvas<'_> {
    fn drop(&mut self) {
        // XXX: This function should not access `self.frame` as its value may not be safe to use
        // (see `Self::end`)

        // SAFETY: We started drawing when this struct was created and we are stopping now that it's
        // being destructed
        unsafe { sys::EndDrawing() };
    }
}

impl Bounded for Canvas<'_> {
    fn width(&self) -> u32 {
        self.frame.width()
    }

    fn height(&self) -> u32 {
        self.frame.height()
    }
}

impl DrawTarget2D for Canvas<'_> {
    fn clear_background(&mut self, color: Color) {
        Self::assert_can_draw();
        unsafe { sys::ClearBackground(color.into()) }
    }

    fn draw_pixel(&mut self, positon: impl Into<Vector2>, color: Color) {
        Self::assert_can_draw();
        unsafe { sys::DrawPixelV(positon.into().into(), color.into()) }
    }

    fn draw_line(
        &mut self,
        from: impl Into<Vector2>,
        to: impl Into<Vector2>,
        thick: f32,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe { sys::DrawLineEx(from.into().into(), to.into().into(), thick, color.into()) };
    }

    fn draw_circle(&mut self, center: impl Into<Vector2>, radius: f32, color: Color) {
        Self::assert_can_draw();
        let center = center.into();
        unsafe { sys::DrawCircle(center.x as _, center.y as _, radius, color.into()) };
    }

    fn draw_circle_lines(&mut self, center: impl Into<Vector2>, radius: f32, color: Color) {
        Self::assert_can_draw();
        unsafe { sys::DrawCircleLinesV(center.into().into(), radius, color.into()) }
    }

    fn draw_rectangle(&mut self, rect: Rectangle, color: Color) {
        Self::assert_can_draw();
        unsafe { sys::DrawRectangleRec(rect, color.into()) };
    }

    fn draw_rectangle_lines(&mut self, rect: Rectangle, line_thick: f32, color: Color) {
        Self::assert_can_draw();
        unsafe { sys::DrawRectangleLinesEx(rect, line_thick, color.into()) };
    }

    fn draw_triangle(
        &mut self,
        p1: impl Into<Vector2>,
        p2: impl Into<Vector2>,
        p3: impl Into<Vector2>,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawTriangle(
                p1.into().into(),
                p2.into().into(),
                p3.into().into(),
                color.into(),
            )
        };
    }

    fn draw_triangle_lines(
        &mut self,
        p1: impl Into<Vector2>,
        p2: impl Into<Vector2>,
        p3: impl Into<Vector2>,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawTriangleLines(
                p1.into().into(),
                p2.into().into(),
                p3.into().into(),
                color.into(),
            )
        };
    }

    fn draw_triangle_fan(&mut self, points: &[Vector2], color: Color) {
        Self::assert_can_draw();
        // cast here is fine because both Vector2s have the same layout
        unsafe { sys::DrawTriangleFan(points.as_ptr().cast(), points.len() as _, color.into()) };
    }

    fn draw_triangle_strip(&mut self, points: &[Vector2], color: Color) {
        Self::assert_can_draw();
        // cast here is fine because both Vector2s have the same layout
        unsafe { sys::DrawTriangleStrip(points.as_ptr().cast(), points.len() as _, color.into()) };
    }

    fn draw_text(
        &mut self,
        text: impl AsRef<str>,
        pos: impl Into<Vector2>,
        font_size: u32,
        color: Color,
    ) {
        Self::assert_can_draw();
        // SAFETY: DrawText does not store this
        let text = unsafe { allocate_cstring(text.as_ref()) };
        let pos = pos.into();
        unsafe {
            sys::DrawText(
                text.as_ptr(),
                pos.x as _,
                pos.y as _,
                font_size as _,
                color.into(),
            )
        };
    }
}

impl DrawTarget2DFull for Canvas<'_> {
    fn draw_line_strip(&mut self, points: &[Vector2], color: Color) {
        Self::assert_can_draw();
        // cast here is fine because both Vector2s have the same layout
        unsafe { sys::DrawLineStrip(points.as_ptr().cast(), points.len() as _, color.into()) };
    }

    fn draw_line_bezier(
        &mut self,
        start: impl Into<Vector2>,
        end: impl Into<Vector2>,
        thick: f32,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe { sys::DrawLineBezier(start.into().into(), end.into().into(), thick, color.into()) };
    }

    fn draw_line_dashed(
        &mut self,
        start: impl Into<Vector2>,
        end: impl Into<Vector2>,
        dash_size: u32,
        space_size: u32,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawLineDashed(
                start.into().into(),
                end.into().into(),
                dash_size as _,
                space_size as _,
                color.into(),
            )
        };
    }

    fn draw_circle_gradient(
        &mut self,
        center: impl Into<Vector2>,
        radius: f32,
        inner: Color,
        outer: Color,
    ) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawCircleGradient(center.into().into(), radius, inner.into(), outer.into())
        };
    }

    fn draw_circle_sector(
        &mut self,
        center: impl Into<Vector2>,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: u32,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawCircleSector(
                center.into().into(),
                radius,
                start_angle,
                end_angle,
                segments as _,
                color.into(),
            )
        };
    }

    fn draw_circle_sector_lines(
        &mut self,
        center: impl Into<Vector2>,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: u32,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawCircleSectorLines(
                center.into().into(),
                radius,
                start_angle,
                end_angle,
                segments as _,
                color.into(),
            )
        };
    }

    fn draw_ellipse(
        &mut self,
        center: impl Into<Vector2>,
        radius: impl Into<Vector2>,
        color: Color,
    ) {
        Self::assert_can_draw();
        let radius = radius.into();
        unsafe { sys::DrawEllipseV(center.into().into(), radius.x, radius.y, color.into()) };
    }

    fn draw_ellipse_lines(
        &mut self,
        center: impl Into<Vector2>,
        radius: impl Into<Vector2>,
        color: Color,
    ) {
        Self::assert_can_draw();
        let radius = radius.into();
        unsafe { sys::DrawEllipseLinesV(center.into().into(), radius.x, radius.y, color.into()) };
    }

    fn draw_ring(
        &mut self,
        center: impl Into<Vector2>,
        inner_radius: f32,
        outer_radius: f32,
        start_angle: f32, // TODO: Range?
        end_angle: f32,
        segments: u32,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawRing(
                center.into().into(),
                inner_radius,
                outer_radius,
                start_angle,
                end_angle,
                segments as _,
                color.into(),
            )
        };
    }

    fn draw_ring_lines(
        &mut self,
        center: impl Into<Vector2>,
        inner_radius: f32,
        outer_radius: f32,
        start_angle: f32, // TODO: Range?
        end_angle: f32,
        segments: u32,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawRingLines(
                center.into().into(),
                inner_radius,
                outer_radius,
                start_angle,
                end_angle,
                segments as _,
                color.into(),
            )
        };
    }

    fn draw_rectangle_gradient(
        &mut self,
        rect: Rectangle,
        top_left: Color,
        top_right: Color,
        bottom_left: Color,
        bottom_right: Color,
    ) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawRectangleGradientEx(
                rect,
                top_left.into(),
                top_right.into(),
                bottom_left.into(),
                bottom_right.into(),
            )
        };
    }

    fn draw_rectangle_pro(
        &mut self,
        rect: Rectangle,
        origin: impl Into<Vector2>,
        rotation: f32,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe { sys::DrawRectanglePro(rect, origin.into().into(), rotation, color.into()) };
    }

    fn draw_rectangle_rounded(
        &mut self,
        rect: Rectangle,
        roundness: f32,
        segments: u32,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe { sys::DrawRectangleRounded(rect, roundness, segments as _, color.into()) };
    }

    fn draw_rectangle_rounded_lines(
        &mut self,
        rect: Rectangle,
        roundness: f32,
        segments: u32,
        thick: f32,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawRectangleRoundedLinesEx(rect, roundness, segments as _, thick, color.into())
        };
    }

    fn draw_poly(
        &mut self,
        center: impl Into<Vector2>,
        sides: u32,
        radius: f32,
        rotation: f32,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawPoly(
                center.into().into(),
                sides as _,
                radius,
                rotation,
                color.into(),
            )
        };
    }

    fn draw_poly_lines(
        &mut self,
        center: impl Into<Vector2>,
        sides: u32,
        radius: f32,
        rotation: f32,
        thick: f32,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawPolyLinesEx(
                center.into().into(),
                sides as _,
                radius,
                rotation,
                thick,
                color.into(),
            )
        };
    }

    fn draw_spline_linear(&mut self, points: &[Vector2], thick: f32, color: Color) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawSplineLinear(
                points.as_ptr().cast(),
                points.len() as _,
                thick,
                color.into(),
            )
        };
    }

    fn draw_spline_basis(&mut self, points: &[Vector2], thick: f32, color: Color) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawSplineBasis(
                points.as_ptr().cast(),
                points.len() as _,
                thick,
                color.into(),
            )
        };
    }

    fn draw_spline_catmull_rom(&mut self, points: &[Vector2], thick: f32, color: Color) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawSplineCatmullRom(
                points.as_ptr().cast(),
                points.len() as _,
                thick,
                color.into(),
            )
        };
    }

    fn draw_spline_bezier_quadratic(&mut self, points: &[Vector2], thick: f32, color: Color) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawSplineBezierQuadratic(
                points.as_ptr().cast(),
                points.len() as _,
                thick,
                color.into(),
            )
        };
    }

    fn draw_spline_bezier_cubic(&mut self, points: &[Vector2], thick: f32, color: Color) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawSplineBezierCubic(
                points.as_ptr().cast(),
                points.len() as _,
                thick,
                color.into(),
            )
        };
    }

    fn draw_spline_segment_linear(
        &mut self,
        p1: impl Into<Vector2>,
        p2: impl Into<Vector2>,
        thick: f32,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawSplineSegmentLinear(p1.into().into(), p2.into().into(), thick, color.into())
        };
    }

    fn draw_spline_segment_basis(
        &mut self,
        p1: impl Into<Vector2>,
        p2: impl Into<Vector2>,
        p3: impl Into<Vector2>,
        p4: impl Into<Vector2>,
        thick: f32,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawSplineSegmentBasis(
                p1.into().into(),
                p2.into().into(),
                p3.into().into(),
                p4.into().into(),
                thick,
                color.into(),
            )
        };
    }

    fn draw_spline_segment_catmull_rom(
        &mut self,
        p1: impl Into<Vector2>,
        p2: impl Into<Vector2>,
        p3: impl Into<Vector2>,
        p4: impl Into<Vector2>,
        thick: f32,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawSplineSegmentCatmullRom(
                p1.into().into(),
                p2.into().into(),
                p3.into().into(),
                p4.into().into(),
                thick,
                color.into(),
            )
        };
    }

    fn draw_spline_segment_bezier_quadratic(
        &mut self,
        p1: impl Into<Vector2>,
        p2: impl Into<Vector2>,
        p3: impl Into<Vector2>,
        thick: f32,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawSplineSegmentBezierQuadratic(
                p1.into().into(),
                p2.into().into(),
                p3.into().into(),
                thick,
                color.into(),
            )
        };
    }

    fn draw_spline_segment_bezier_cubic(
        &mut self,
        p1: impl Into<Vector2>,
        p2: impl Into<Vector2>,
        p3: impl Into<Vector2>,
        p4: impl Into<Vector2>,
        thick: f32,
        color: Color,
    ) {
        Self::assert_can_draw();
        unsafe {
            sys::DrawSplineSegmentBezierCubic(
                p1.into().into(),
                p2.into().into(),
                p3.into().into(),
                p4.into().into(),
                thick,
                color.into(),
            )
        };
    }

    fn draw_texture(
        &mut self,
        texture: &Texture2D,
        position: impl Into<Vector2>,
        rotation: f32,
        scale: f32,
        tint: Color,
    ) {
        Self::assert_can_draw();
        self.frame.window.resources.push(Box::new(texture.clone()));
        unsafe {
            sys::DrawTextureEx(
                *texture.inner(),
                position.into().into(),
                rotation,
                scale,
                tint.into(),
            )
        };
    }

    fn draw_texture_pro(
        &mut self,
        texture: &Texture2D,
        src: Rectangle,
        dst: Rectangle,
        origin: impl Into<Vector2>,
        rotation: f32,
        tint: Color,
    ) {
        Self::assert_can_draw();
        self.frame.window.resources.push(Box::new(texture.clone()));
        unsafe {
            sys::DrawTexturePro(
                *texture.inner(),
                src,
                dst,
                origin.into().into(),
                rotation,
                tint.into(),
            )
        };
    }

    fn draw_text_pro(
        &mut self,
        font: &Font,
        text: impl AsRef<str>,
        pos: impl Into<Vector2>,
        origin: impl Into<Vector2>,
        rotation: Angle,
        font_size: f32,
        spacing: f32,
        color: Color,
    ) {
        Self::assert_can_draw();
        // SAFETY: DrawTextPro does not store this
        let text = unsafe { allocate_cstring(text.as_ref()) };
        let pos = pos.into();
        self.frame.window.resources.push(Box::new(font.clone()));
        unsafe {
            sys::DrawTextPro(
                font.to_sys(),
                text.as_ptr(),
                pos.into(),
                origin.into().into(),
                rotation.to_radians(),
                font_size,
                spacing,
                color.into(),
            )
        };
    }
}
