use std::{ffi::CString, marker::PhantomData};

use raylib_sys::{self as sys};

pub use raylib_sys::{Color, KeyboardKey, MouseButton, Rectangle, Vector2};

use crate::image::Image;
use crate::window::Window;

pub mod bytes;
pub mod image;
pub mod rand;
pub mod text;
pub mod window;

pub mod prelude {
    pub use crate::{
        Bounded, Color, DrawTarget, KeyboardKey, MouseButton, Rectangle, Texture2D, Vector2,
        image::{FileType, Image},
        rand::Random,
        window::{ConfigFlags, Window},
    };
}

pub trait Bounded {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn size(&self) -> (u32, u32) {
        (self.width(), self.height())
    }
    fn bounds(&self) -> sys::Rectangle {
        Rectangle::new(0., 0., self.width() as _, self.height() as _)
    }
}

pub trait DrawTarget {
    fn clear_background(&mut self, color: Color);

    fn draw_circle(&mut self, center: Vector2, radius: f32, color: Color);
    fn draw_line(&mut self, from: Vector2, to: Vector2, thick: f32, color: Color);
    fn draw_rectangle(&mut self, rect: Rectangle, color: Color);
    fn draw_rectangle_lines(&mut self, rect: Rectangle, line_thick: f32, color: Color);
    fn draw_text(&mut self, text: impl AsRef<str>, pos: Vector2, font_size: u32, color: Color);
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

    pub fn draw_texture(
        &mut self,
        texture: &Texture2D,
        position: Vector2,
        rotation: f32,
        scale: f32,
        tint: Color,
    ) {
        unsafe { sys::DrawTextureEx(texture.0, position, rotation, scale, tint) };
    }

    pub fn draw_texture_pro(
        &mut self,
        texture: &Texture2D,
        src: Rectangle,
        dst: Rectangle,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    ) {
        unsafe { sys::DrawTexturePro(texture.0, src, dst, origin, rotation, tint) };
    }

    pub fn draw_rectangle_pro(
        &mut self,
        rect: Rectangle,
        origin: Vector2,
        rotation: f32,
        color: Color,
    ) {
        unsafe { sys::DrawRectanglePro(rect, origin, rotation, color) };
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

    fn draw_circle(&mut self, center: Vector2, radius: f32, color: Color) {
        unsafe { sys::DrawCircle(center.x as _, center.y as _, radius, color) };
    }

    fn draw_line(&mut self, from: Vector2, to: Vector2, thick: f32, color: Color) {
        unsafe { sys::DrawLineEx(from, to, thick, color) };
    }

    fn draw_rectangle(&mut self, rect: Rectangle, color: Color) {
        unsafe { sys::DrawRectangleRec(rect, color) };
    }

    fn draw_rectangle_lines(&mut self, rect: Rectangle, line_thick: f32, color: Color) {
        unsafe { sys::DrawRectangleLinesEx(rect, line_thick, color) };
    }

    fn draw_text(&mut self, text: impl AsRef<str>, pos: Vector2, font_size: u32, color: Color) {
        let text = CString::new(text.as_ref()).expect("str has no null");
        unsafe { sys::DrawText(text.as_ptr(), pos.x as _, pos.y as _, font_size as _, color) };
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
