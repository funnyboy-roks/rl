use std::sync::atomic::Ordering;

use crate::{
    Canvas, Rectangle,
    color::Color,
    draw::{DrawTarget2D, DrawTarget2DFull},
    globals::{DRAWING_TO_CAMERA, DRAWING_TO_TEXTURE, WINDOW_INITIALISED},
    math::{Angle, Vector2},
    text::Font,
    texture::Texture2D,
    util::allocate_cstring,
};

use raylib_sys as sys;

#[derive(bauer::Builder, Clone, Copy, Debug)]
#[builder(
    const,
    build_fn {
        map = |c| -> Camera2D { assert!(c.zoom > 0.); c }
    },
)]
pub struct Camera2D {
    /// Camera offset (screen space offset from window origin)
    ///
    /// default = `Vector2::ZERO`
    #[builder(default = "Vector2::ZERO")]
    pub offset: Vector2,
    /// Camera target (world space target point that is mapped to screen space offset)
    ///
    /// default = `Vector2::ZERO`
    #[builder(default = "Vector2::ZERO")]
    pub target: Vector2,
    /// Camera rotation in degrees (pivots around target)
    ///
    /// default = `0.0`
    #[builder(default = "0.")]
    pub rotation: f32,
    /// Camera zoom (scaling around target)
    ///
    /// **Must not be set to 0**
    ///
    /// default = `1.0`
    #[builder(default = "1.")]
    pub zoom: f32,
}

impl Default for Camera2D {
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

impl Camera2D {
    fn to_sys(self) -> sys::Camera2D {
        sys::Camera2D {
            offset: self.offset.into(),
            target: self.target.into(),
            rotation: self.rotation,
            zoom: self.zoom,
        }
    }
}

pub struct Camera2DCanvas<'window, 'canvas> {
    canvas: &'canvas mut Canvas<'window>,
    _camera: Camera2D,
}

impl<'w, 'c> Camera2DCanvas<'w, 'c> {
    pub(crate) fn new(canvas: &'c mut Canvas<'w>, camera: Camera2D) -> Self {
        if DRAWING_TO_CAMERA
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire)
            .is_err()
        {
            panic!("Only one camera may be drawn to at a time.");
        }
        unsafe { sys::BeginMode2D(camera.to_sys()) }
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

impl<'w, 'c> Drop for Camera2DCanvas<'w, 'c> {
    fn drop(&mut self) {
        if DRAWING_TO_CAMERA
            .compare_exchange(true, false, Ordering::Acquire, Ordering::Acquire)
            .is_err()
        {
            panic!("Attempted to end camera drawing without calling BeginCamera2D");
        }
        unsafe { sys::EndMode2D() }
    }
}

impl DrawTarget2D for Camera2DCanvas<'_, '_> {
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

impl DrawTarget2DFull for Camera2DCanvas<'_, '_> {
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
        self.canvas
            .frame
            .window
            .resources
            .push(Box::new(texture.clone()));
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
        self.canvas
            .frame
            .window
            .resources
            .push(Box::new(texture.clone()));
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
        self.canvas
            .frame
            .window
            .resources
            .push(Box::new(font.clone()));
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
