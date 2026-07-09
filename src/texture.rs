use std::{ffi::CString, path::Path, sync::atomic::Ordering};

use raylib_sys::{self as sys};

use crate::{
    Bounded, Color, Rectangle, Vector2,
    draw::{DrawTarget, DrawTargetFull},
    globals::DRAWING_TO_TEXTURE,
    image::Image,
};

#[derive(Debug)]
#[repr(transparent)]
pub struct Texture2D(sys::Texture2D);

impl Drop for Texture2D {
    fn drop(&mut self) {
        unsafe { sys::UnloadTexture(self.0) };
    }
}

impl Bounded for Texture2D {
    fn width(&self) -> u32 {
        self.0.width as _
    }

    fn height(&self) -> u32 {
        self.0.height as _
    }
}

impl AsRef<Texture2D> for &Texture2D {
    fn as_ref(&self) -> &Texture2D {
        self
    }
}

impl Texture2D {
    pub(crate) fn from_ref_sys(texture: &sys::Texture2D) -> &Self {
        assert!(Self::is_valid(*texture));
        // SAFETY: Texture2D is a transparent wrapper around the raylib one
        unsafe { std::mem::transmute(texture) }
    }

    pub(crate) fn from_sys(texture: sys::Texture2D) -> Option<Self> {
        Self::is_valid(texture).then_some(Self(texture))
    }

    pub(crate) fn inner(&self) -> sys::Texture2D {
        self.0
    }

    pub(crate) fn is_valid(texture: sys::Texture2D) -> bool {
        unsafe { sys::IsTextureValid(texture) }
    }

    // https://github.com/raysan5/raylib/blob/master/src/rtextures.c#L4229
    pub fn load(file: impl AsRef<Path>) -> std::io::Result<Self> {
        let image = Image::load(file)?;
        Ok(Self::from_image(&image))
    }

    pub fn from_image(image: &Image) -> Self {
        Self::from_sys(unsafe { sys::LoadTextureFromImage(image.inner()) })
            .expect("Texture is valid if the image is valid")
    }
}

#[derive(Debug)]
pub struct RenderTexture2D(sys::RenderTexture2D);

impl Drop for RenderTexture2D {
    fn drop(&mut self) {
        unsafe { sys::UnloadRenderTexture(self.0) };
    }
}

impl Bounded for RenderTexture2D {
    fn width(&self) -> u32 {
        self.0.texture.width as _
    }

    fn height(&self) -> u32 {
        self.0.texture.height as _
    }
}

impl RenderTexture2D {
    pub(crate) fn from_sys(texture: sys::RenderTexture2D) -> Option<Self> {
        Self::is_valid(texture).then_some(Self(texture))
    }

    pub(crate) fn inner(&self) -> sys::RenderTexture2D {
        self.0
    }

    pub(crate) fn is_valid(texture: sys::RenderTexture2D) -> bool {
        unsafe { sys::IsRenderTextureValid(texture) }
    }

    /// Create a new render texture
    ///
    /// # Panics
    ///
    /// If failed to be created
    pub fn new(width: u32, height: u32) -> Self {
        Self::try_new(width, height).expect("Failed to create render texture")
    }

    /// Attempt to create a new render texture and return None if it can't be created
    pub fn try_new(width: u32, height: u32) -> Option<Self> {
        Self::from_sys(unsafe { sys::LoadRenderTexture(width as _, height as _) })
    }

    /// Color buffer attachment texture
    pub fn texture(&self) -> &Texture2D {
        Texture2D::from_ref_sys(&self.0.texture)
    }

    /// Depth buffer attachment texture
    pub fn depth(&self) -> &Texture2D {
        Texture2D::from_ref_sys(&self.0.texture)
    }

    /// OpenGL framebuffer object id
    pub fn id(&self) -> u32 {
        self.0.id
    }

    pub fn draw_with<'t, F>(&'t mut self, f: F)
    where
        F: FnOnce(&mut DrawRenderTexture2D<'t>),
    {
        let mut ctx = self.draw();
        f(&mut ctx);
        drop(ctx);
    }

    // TODO: link to frame in some way?
    fn draw<'t>(&'t mut self) -> DrawRenderTexture2D<'t> {
        DrawRenderTexture2D::new(self)
    }
}

pub struct DrawRenderTexture2D<'texture> {
    texture: &'texture mut RenderTexture2D,
}

impl DrawRenderTexture2D<'_> {
    #[inline]
    fn assert_can_draw(&self) {
        assert!(
            DRAWING_TO_TEXTURE.load(Ordering::Acquire),
            "Attempting to draw to texture without calling BeginTextureMode"
        );
    }
}

impl Drop for DrawRenderTexture2D<'_> {
    fn drop(&mut self) {
        if DRAWING_TO_TEXTURE
            .compare_exchange(true, false, Ordering::Acquire, Ordering::Acquire)
            .is_err()
        {
            panic!("Attempted to end texture drawing without calling BeginTextureMode");
        }
        // SAFETY: We call this in the constructor
        unsafe { sys::EndTextureMode() };
    }
}

impl<'t> DrawRenderTexture2D<'t> {
    fn new(texture: &'t mut RenderTexture2D) -> Self {
        if DRAWING_TO_TEXTURE
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire)
            .is_err()
        {
            panic!("Only one texture may be drawn to at a time.");
        }
        unsafe { sys::BeginTextureMode(texture.inner()) };
        Self { texture }
    }
}

impl Bounded for DrawRenderTexture2D<'_> {
    fn width(&self) -> u32 {
        self.texture.width()
    }

    fn height(&self) -> u32 {
        self.texture.height()
    }
}

impl DrawTarget for DrawRenderTexture2D<'_> {
    fn clear_background(&mut self, color: Color) {
        self.assert_can_draw();
        unsafe { sys::ClearBackground(color) }
    }

    fn draw_pixel(&mut self, positon: impl Into<Vector2>, color: Color) {
        self.assert_can_draw();
        unsafe { sys::DrawPixelV(positon.into(), color) }
    }

    fn draw_line(
        &mut self,
        from: impl Into<Vector2>,
        to: impl Into<Vector2>,
        thick: f32,
        color: Color,
    ) {
        self.assert_can_draw();
        unsafe { sys::DrawLineEx(from.into(), to.into(), thick, color) };
    }

    fn draw_circle(&mut self, center: impl Into<Vector2>, radius: f32, color: Color) {
        self.assert_can_draw();
        let center = center.into();
        unsafe { sys::DrawCircle(center.x as _, center.y as _, radius, color) };
    }

    fn draw_circle_lines(&mut self, center: impl Into<Vector2>, radius: f32, color: Color) {
        self.assert_can_draw();
        unsafe { sys::DrawCircleLinesV(center.into(), radius, color) }
    }

    fn draw_rectangle(&mut self, rect: Rectangle, color: Color) {
        self.assert_can_draw();
        unsafe { sys::DrawRectangleRec(rect, color) };
    }

    fn draw_rectangle_lines(&mut self, rect: Rectangle, line_thick: f32, color: Color) {
        self.assert_can_draw();
        unsafe { sys::DrawRectangleLinesEx(rect, line_thick, color) };
    }

    fn draw_triangle(
        &mut self,
        p1: impl Into<Vector2>,
        p2: impl Into<Vector2>,
        p3: impl Into<Vector2>,
        color: Color,
    ) {
        self.assert_can_draw();
        unsafe { sys::DrawTriangle(p1.into(), p2.into(), p3.into(), color) };
    }

    fn draw_triangle_lines(
        &mut self,
        p1: impl Into<Vector2>,
        p2: impl Into<Vector2>,
        p3: impl Into<Vector2>,
        color: Color,
    ) {
        self.assert_can_draw();
        unsafe { sys::DrawTriangleLines(p1.into(), p2.into(), p3.into(), color) };
    }

    fn draw_triangle_fan(&mut self, points: &[Vector2], color: Color) {
        self.assert_can_draw();
        unsafe { sys::DrawTriangleFan(points.as_ptr(), points.len() as _, color) };
    }

    fn draw_triangle_strip(&mut self, points: &[Vector2], color: Color) {
        self.assert_can_draw();
        unsafe { sys::DrawTriangleStrip(points.as_ptr(), points.len() as _, color) };
    }

    fn draw_text(
        &mut self,
        text: impl AsRef<str>,
        pos: impl Into<Vector2>,
        font_size: u32,
        color: Color,
    ) {
        self.assert_can_draw();
        let text = CString::new(text.as_ref()).expect("str has no null");
        let pos = pos.into();
        unsafe { sys::DrawText(text.as_ptr(), pos.x as _, pos.y as _, font_size as _, color) };
    }
}

impl DrawTargetFull for DrawRenderTexture2D<'_> {
    fn draw_line_strip(&mut self, points: &[Vector2], color: Color) {
        self.assert_can_draw();
        unsafe { sys::DrawLineStrip(points.as_ptr(), points.len() as _, color) };
    }

    fn draw_line_bezier(
        &mut self,
        start: impl Into<Vector2>,
        end: impl Into<Vector2>,
        thick: f32,
        color: Color,
    ) {
        self.assert_can_draw();
        unsafe { sys::DrawLineBezier(start.into(), end.into(), thick, color) };
    }

    fn draw_line_dashed(
        &mut self,
        start: impl Into<Vector2>,
        end: impl Into<Vector2>,
        dash_size: u32,
        space_size: u32,
        color: Color,
    ) {
        self.assert_can_draw();
        unsafe {
            sys::DrawLineDashed(
                start.into(),
                end.into(),
                dash_size as _,
                space_size as _,
                color,
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
        self.assert_can_draw();
        unsafe { sys::DrawCircleGradient(center.into(), radius, inner, outer) };
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
        self.assert_can_draw();
        unsafe {
            sys::DrawCircleSector(
                center.into(),
                radius,
                start_angle,
                end_angle,
                segments as _,
                color,
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
        self.assert_can_draw();
        unsafe {
            sys::DrawCircleSectorLines(
                center.into(),
                radius,
                start_angle,
                end_angle,
                segments as _,
                color,
            )
        };
    }

    fn draw_ellipse(
        &mut self,
        center: impl Into<Vector2>,
        radius: impl Into<Vector2>,
        color: Color,
    ) {
        self.assert_can_draw();
        let radius = radius.into();
        unsafe { sys::DrawEllipseV(center.into(), radius.x, radius.y, color) };
    }

    fn draw_ellipse_lines(
        &mut self,
        center: impl Into<Vector2>,
        radius: impl Into<Vector2>,
        color: Color,
    ) {
        self.assert_can_draw();
        let radius = radius.into();
        unsafe { sys::DrawEllipseLinesV(center.into(), radius.x, radius.y, color) };
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
        self.assert_can_draw();
        unsafe {
            sys::DrawRing(
                center.into(),
                inner_radius,
                outer_radius,
                start_angle,
                end_angle,
                segments as _,
                color,
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
        self.assert_can_draw();
        unsafe {
            sys::DrawRingLines(
                center.into(),
                inner_radius,
                outer_radius,
                start_angle,
                end_angle,
                segments as _,
                color,
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
        self.assert_can_draw();
        unsafe {
            sys::DrawRectangleGradientEx(rect, top_left, top_right, bottom_left, bottom_right)
        };
    }

    fn draw_rectangle_pro(
        &mut self,
        rect: Rectangle,
        origin: impl Into<Vector2>,
        rotation: f32,
        color: Color,
    ) {
        self.assert_can_draw();
        unsafe { sys::DrawRectanglePro(rect, origin.into(), rotation, color) };
    }

    fn draw_rectangle_rounded(
        &mut self,
        rect: Rectangle,
        roundness: f32,
        segments: u32,
        color: Color,
    ) {
        self.assert_can_draw();
        unsafe { sys::DrawRectangleRounded(rect, roundness, segments as _, color) };
    }

    fn draw_rectangle_rounded_lines(
        &mut self,
        rect: Rectangle,
        roundness: f32,
        segments: u32,
        thick: f32,
        color: Color,
    ) {
        self.assert_can_draw();
        unsafe { sys::DrawRectangleRoundedLinesEx(rect, roundness, segments as _, thick, color) };
    }

    fn draw_poly(
        &mut self,
        center: impl Into<Vector2>,
        sides: u32,
        radius: f32,
        rotation: f32,
        color: Color,
    ) {
        self.assert_can_draw();
        unsafe { sys::DrawPoly(center.into(), sides as _, radius, rotation, color) };
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
        self.assert_can_draw();
        unsafe { sys::DrawPolyLinesEx(center.into(), sides as _, radius, rotation, thick, color) };
    }

    fn draw_spline_linear(&mut self, points: &[Vector2], thick: f32, color: Color) {
        self.assert_can_draw();
        unsafe { sys::DrawSplineLinear(points.as_ptr(), points.len() as _, thick, color) };
    }

    fn draw_spline_basis(&mut self, points: &[Vector2], thick: f32, color: Color) {
        self.assert_can_draw();
        unsafe { sys::DrawSplineBasis(points.as_ptr(), points.len() as _, thick, color) };
    }

    fn draw_spline_catmull_rom(&mut self, points: &[Vector2], thick: f32, color: Color) {
        self.assert_can_draw();
        unsafe { sys::DrawSplineCatmullRom(points.as_ptr(), points.len() as _, thick, color) };
    }

    fn draw_spline_bezier_quadratic(&mut self, points: &[Vector2], thick: f32, color: Color) {
        self.assert_can_draw();
        unsafe { sys::DrawSplineBezierQuadratic(points.as_ptr(), points.len() as _, thick, color) };
    }

    fn draw_spline_bezier_cubic(&mut self, points: &[Vector2], thick: f32, color: Color) {
        self.assert_can_draw();
        unsafe { sys::DrawSplineBezierCubic(points.as_ptr(), points.len() as _, thick, color) };
    }

    fn draw_spline_segment_linear(
        &mut self,
        p1: impl Into<Vector2>,
        p2: impl Into<Vector2>,
        thick: f32,
        color: Color,
    ) {
        self.assert_can_draw();
        unsafe { sys::DrawSplineSegmentLinear(p1.into(), p2.into(), thick, color) };
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
        self.assert_can_draw();
        unsafe {
            sys::DrawSplineSegmentBasis(p1.into(), p2.into(), p3.into(), p4.into(), thick, color)
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
        self.assert_can_draw();
        unsafe {
            sys::DrawSplineSegmentCatmullRom(
                p1.into(),
                p2.into(),
                p3.into(),
                p4.into(),
                thick,
                color,
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
        self.assert_can_draw();
        unsafe {
            sys::DrawSplineSegmentBezierQuadratic(p1.into(), p2.into(), p3.into(), thick, color)
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
        self.assert_can_draw();
        unsafe {
            sys::DrawSplineSegmentBezierCubic(
                p1.into(),
                p2.into(),
                p3.into(),
                p4.into(),
                thick,
                color,
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
        self.assert_can_draw();
        unsafe { sys::DrawTextureEx(texture.inner(), position.into(), rotation, scale, tint) };
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
        self.assert_can_draw();
        unsafe {
            sys::DrawTexturePro(
                texture.as_ref().inner(),
                src,
                dst,
                origin.into(),
                rotation,
                tint,
            )
        };
    }
}
