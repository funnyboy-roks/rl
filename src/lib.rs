use std::{ffi::CString, marker::PhantomData};

use raylib_sys::{self as sys};

pub use raylib_sys::{Color, KeyboardKey, MouseButton, Rectangle, Vector2};

use crate::draw::{DrawTarget, DrawTargetFull};
use crate::image::Image;
use crate::window::Window;

pub mod bytes;
pub mod draw;
pub mod image;
pub mod rand;
pub mod text;
pub mod window;

pub mod prelude {
    pub use crate::{
        Bounded, Color, KeyboardKey, MouseButton, Rectangle, Texture2D, Vector2,
        draw::{DrawTarget, DrawTargetFull},
        image::{FileType, Image},
        rand::Random,
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

#[derive(Debug)]
pub struct Mouse<'frame>(PhantomData<&'frame ()>);

impl Mouse<'_> {
    pub fn position(&self) -> Vector2 {
        unsafe { sys::GetMousePosition() }
    }

    pub fn delta(&self) -> Vector2 {
        unsafe { sys::GetMouseDelta() }
    }

    pub fn wheel_move(&self) -> f32 {
        unsafe { sys::GetMouseWheelMove() }
    }

    pub fn wheel_move_v(&self) -> Vector2 {
        unsafe { sys::GetMouseWheelMoveV() }
    }

    pub fn show_cursor(&mut self) {
        unsafe { sys::ShowCursor() }
    }

    pub fn hide_cursor(&mut self) {
        unsafe { sys::HideCursor() }
    }

    pub fn is_cursor_hidden(&self) -> bool {
        unsafe { sys::IsCursorHidden() }
    }

    pub fn enable_cursor(&mut self) {
        unsafe { sys::EnableCursor() }
    }

    pub fn disable_cursor(&mut self) {
        unsafe { sys::DisableCursor() }
    }

    pub fn is_cursor_on_screen(&self) -> bool {
        unsafe { sys::IsCursorOnScreen() }
    }

    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        unsafe { sys::IsMouseButtonPressed(button as _) }
    }

    pub fn is_button_down(&self, button: MouseButton) -> bool {
        unsafe { sys::IsMouseButtonDown(button as _) }
    }

    pub fn is_button_released(&self, button: MouseButton) -> bool {
        unsafe { sys::IsMouseButtonReleased(button as _) }
    }

    pub fn is_button_up(&self, button: MouseButton) -> bool {
        unsafe { sys::IsMouseButtonUp(button as _) }
    }
}

#[derive(Debug)]
pub struct Frame<'window> {
    window: &'window mut Window,
}

impl Frame<'_> {
    pub fn window(&self) -> &Window {
        self.window
    }

    pub fn window_mut(&mut self) -> &mut Window {
        self.window
    }

    pub fn mouse<'f>(&'f self) -> &'f Mouse<'f> {
        &Mouse(PhantomData)
    }

    pub fn mouse_mut<'f>(&'f mut self) -> &'f mut Mouse<'f> {
        // SAFETY: This is wildly unsafe, but since Mouse is zero-sized and we never use the value
        // of the reference itself, it's fine
        #[allow(mutable_transmutes)]
        unsafe {
            std::mem::transmute(&Mouse(PhantomData))
        }
    }

    pub fn get_time(&self) -> f32 {
        unsafe { sys::GetFrameTime() }
    }

    /// The number of frames that have run
    pub fn count(&self) -> u64 {
        // Because we increment at the start, we need to subtract one for an accurate count
        self.window().frame_count - 1
    }

    pub fn draw_fps(&mut self, x: i32, y: i32) {
        unsafe { sys::DrawFPS(x, y) }
    }

    pub fn is_key_pressed(&self, key: KeyboardKey) -> bool {
        unsafe { sys::IsKeyPressed(key as _) }
    }

    pub fn is_key_pressed_repeat(&self, key: KeyboardKey) -> bool {
        unsafe { sys::IsKeyPressedRepeat(key as _) }
    }

    pub fn is_key_down(&self, key: KeyboardKey) -> bool {
        unsafe { sys::IsKeyDown(key as _) }
    }

    pub fn is_key_released(&self, key: KeyboardKey) -> bool {
        unsafe { sys::IsKeyReleased(key as _) }
    }

    pub fn is_key_up(&self, key: KeyboardKey) -> bool {
        unsafe { sys::IsKeyUp(key as _) }
    }

    // TODO
    // pub fn get_key_pressed(&self, key: KeyboardKey) -> KeyboardKey {
    //     unsafe { sys::GetKeyPressed() }
    // }
}

impl Drop for Frame<'_> {
    fn drop(&mut self) {
        self.window.prev_mouse = Some(self.mouse().position());
        unsafe { sys::EndDrawing() };
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

impl Bounded for Window {
    fn width(&self) -> u32 {
        unsafe { sys::GetRenderWidth() }.try_into().unwrap()
    }

    fn height(&self) -> u32 {
        unsafe { sys::GetRenderHeight() }.try_into().unwrap()
    }
}

impl DrawTarget for Frame<'_> {
    fn clear_background(&mut self, color: Color) {
        unsafe { sys::ClearBackground(color) }
    }

    fn draw_pixel(&mut self, positon: impl Into<Vector2>, color: Color) {
        unsafe { sys::DrawPixelV(positon.into(), color) }
    }

    fn draw_line(
        &mut self,
        from: impl Into<Vector2>,
        to: impl Into<Vector2>,
        thick: f32,
        color: Color,
    ) {
        unsafe { sys::DrawLineEx(from.into(), to.into(), thick, color) };
    }

    fn draw_circle(&mut self, center: impl Into<Vector2>, radius: f32, color: Color) {
        let center = center.into();
        unsafe { sys::DrawCircle(center.x as _, center.y as _, radius, color) };
    }

    fn draw_circle_lines(&mut self, center: impl Into<Vector2>, radius: f32, color: Color) {
        unsafe { sys::DrawCircleLinesV(center.into(), radius, color) }
    }

    fn draw_rectangle(&mut self, rect: Rectangle, color: Color) {
        unsafe { sys::DrawRectangleRec(rect, color) };
    }

    fn draw_rectangle_lines(&mut self, rect: Rectangle, line_thick: f32, color: Color) {
        unsafe { sys::DrawRectangleLinesEx(rect, line_thick, color) };
    }

    fn draw_triangle(
        &mut self,
        p1: impl Into<Vector2>,
        p2: impl Into<Vector2>,
        p3: impl Into<Vector2>,
        color: Color,
    ) {
        unsafe { sys::DrawTriangle(p1.into(), p2.into(), p3.into(), color) };
    }

    fn draw_triangle_lines(
        &mut self,
        p1: impl Into<Vector2>,
        p2: impl Into<Vector2>,
        p3: impl Into<Vector2>,
        color: Color,
    ) {
        unsafe { sys::DrawTriangleLines(p1.into(), p2.into(), p3.into(), color) };
    }

    fn draw_triangle_fan(&mut self, points: &[Vector2], color: Color) {
        unsafe { sys::DrawTriangleFan(points.as_ptr(), points.len() as _, color) };
    }

    fn draw_triangle_strip(&mut self, points: &[Vector2], color: Color) {
        unsafe { sys::DrawTriangleStrip(points.as_ptr(), points.len() as _, color) };
    }

    fn draw_text(
        &mut self,
        text: impl AsRef<str>,
        pos: impl Into<Vector2>,
        font_size: u32,
        color: Color,
    ) {
        let text = CString::new(text.as_ref()).expect("str has no null");
        let pos = pos.into();
        unsafe { sys::DrawText(text.as_ptr(), pos.x as _, pos.y as _, font_size as _, color) };
    }
}

impl DrawTargetFull for Frame<'_> {
    fn draw_line_strip(&mut self, points: &[Vector2], color: Color) {
        unsafe { sys::DrawLineStrip(points.as_ptr(), points.len() as _, color) };
    }

    fn draw_line_bezier(
        &mut self,
        start: impl Into<Vector2>,
        end: impl Into<Vector2>,
        thick: f32,
        color: Color,
    ) {
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
        let radius = radius.into();
        unsafe { sys::DrawEllipseV(center.into(), radius.x, radius.y, color) };
    }

    fn draw_ellipse_lines(
        &mut self,
        center: impl Into<Vector2>,
        radius: impl Into<Vector2>,
        color: Color,
    ) {
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
        unsafe { sys::DrawRectanglePro(rect, origin.into(), rotation, color) };
    }

    fn draw_rectangle_rounded(
        &mut self,
        rect: Rectangle,
        roundness: f32,
        segments: u32,
        color: Color,
    ) {
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
        unsafe { sys::DrawPolyLinesEx(center.into(), sides as _, radius, rotation, thick, color) };
    }

    fn draw_spline_linear(&mut self, points: &[Vector2], thick: f32, color: Color) {
        unsafe { sys::DrawSplineLinear(points.as_ptr(), points.len() as _, thick, color) };
    }

    fn draw_spline_basis(&mut self, points: &[Vector2], thick: f32, color: Color) {
        unsafe { sys::DrawSplineBasis(points.as_ptr(), points.len() as _, thick, color) };
    }

    fn draw_spline_catmull_rom(&mut self, points: &[Vector2], thick: f32, color: Color) {
        unsafe { sys::DrawSplineCatmullRom(points.as_ptr(), points.len() as _, thick, color) };
    }

    fn draw_spline_bezier_quadratic(&mut self, points: &[Vector2], thick: f32, color: Color) {
        unsafe { sys::DrawSplineBezierQuadratic(points.as_ptr(), points.len() as _, thick, color) };
    }

    fn draw_spline_bezier_cubic(&mut self, points: &[Vector2], thick: f32, color: Color) {
        unsafe { sys::DrawSplineBezierCubic(points.as_ptr(), points.len() as _, thick, color) };
    }

    fn draw_spline_segment_linear(
        &mut self,
        p1: impl Into<Vector2>,
        p2: impl Into<Vector2>,
        thick: f32,
        color: Color,
    ) {
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
        unsafe { sys::DrawTextureEx(texture.0, position.into(), rotation, scale, tint) };
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
        unsafe { sys::DrawTexturePro(texture.0, src, dst, origin.into(), rotation, tint) };
    }
}

#[derive(Debug)]
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

impl Texture2D {
    pub fn from_image(image: &Image) -> Self {
        Self(unsafe { sys::LoadTextureFromImage(image.inner()) })
    }
}
