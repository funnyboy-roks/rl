use std::ffi::{CStr, CString};

use raylib_sys as sys;

use crate::{Frame, Image, Vector2};

#[derive(Debug)]
pub struct Window {
    pub(crate) prev_mouse: Option<Vector2>,
    // 1 + Number of frames that have been run since this window was started.  Value is intialised
    // on call to begin_drawing
    pub(crate) frame_count: u64,
}

impl Drop for Window {
    fn drop(&mut self) {
        // SAFETY: the only way to get a window is to have called InitWindow
        unsafe { sys::CloseWindow() };
    }
}

bitflags::bitflags! {
    pub struct ConfigFlags: u32 {
        const VSYNC_HINT = 64;
        const FULLSCREEN_MODE = 2;
        const WINDOW_RESIZABLE = 4;
        const WINDOW_UNDECORATED = 8;
        const WINDOW_HIDDEN = 128;
        const WINDOW_MINIMIZED = 512;
        const WINDOW_MAXIMIZED = 1_024;
        const WINDOW_UNFOCUSED = 2_048;
        const WINDOW_TOPMOST = 4_096;
        const WINDOW_ALWAYS_RUN = 256;
        const WINDOW_TRANSPARENT = 16;
        const WINDOW_HIGHDPI = 8_192;
        const WINDOW_MOUSE_PASSTHROUGH = 16_384;
        const BORDERLESS_WINDOWED_MODE = 32_768;
        const MSAA_4X_HINT = 32;
        const INTERLACED_HINT = 65_536;
    }
}

macro_rules! gen_getter {
    ($($name: ident => $fn: ident -> $ret: ty;)*) => {
        $(
            pub fn $name(&self) -> $ret {
                unsafe { sys::$fn() }.try_into().unwrap()
            }
        )*
    }
}

impl Window {
    pub fn init(width: u32, height: u32, title: impl AsRef<str>) -> Window {
        let title = CString::new(title.as_ref()).expect("str can't contain null");
        // SAFETY: title is heap allocated c-string
        unsafe { sys::InitWindow(width as _, height as _, title.as_ptr()) };

        Window {
            prev_mouse: None,
            frame_count: 0,
        }
    }

    pub fn init_with_flags(
        width: u32,
        height: u32,
        title: impl AsRef<str>,
        flags: ConfigFlags,
    ) -> Window {
        unsafe { sys::SetConfigFlags(flags.bits()) };
        Self::init(width, height, title)
    }

    pub fn close(self) {
        drop(self);
    }

    pub fn set_target_fps(&mut self, target: u32) {
        unsafe { sys::SetTargetFPS(target as _) }
    }

    pub fn begin_drawing<'a>(&'a mut self) -> Frame<'a> {
        unsafe { sys::BeginDrawing() };
        self.frame_count += 1;
        Frame { window: self }
    }

    /// Helper method that calls WindowShouldClose and BeginDrawing
    pub fn next_frame<'w>(&'w mut self) -> Option<Frame<'w>> {
        if self.should_close() {
            None
        } else {
            Some(self.begin_drawing())
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Monitor(u32);

impl Monitor {
    pub fn id(&self) -> u32 {
        self.0
    }

    pub fn width(&self) -> u32 {
        unsafe { sys::GetMonitorWidth(self.0.try_into().unwrap()) }
            .try_into()
            .unwrap()
    }

    pub fn height(&self) -> u32 {
        unsafe { sys::GetMonitorHeight(self.0.try_into().unwrap()) }
            .try_into()
            .unwrap()
    }

    pub fn physical_width(&self) -> u32 {
        unsafe { sys::GetMonitorPhysicalWidth(self.0.try_into().unwrap()) }
            .try_into()
            .unwrap()
    }

    pub fn physical_height(&self) -> u32 {
        unsafe { sys::GetMonitorPhysicalHeight(self.0.try_into().unwrap()) }
            .try_into()
            .unwrap()
    }

    pub fn refresh_rate(&self) -> u32 {
        unsafe { sys::GetMonitorRefreshRate(self.0.try_into().unwrap()) }
            .try_into()
            .unwrap()
    }

    pub fn position(&self) -> Vector2 {
        unsafe { sys::GetMonitorPosition(self.0.try_into().unwrap()) }
    }

    pub fn name(&self) -> Option<String> {
        let name = unsafe { sys::GetMonitorName(self.0.try_into().unwrap()) };
        if name.is_null() {
            None
        } else {
            let name = unsafe { CStr::from_ptr(name) };
            Some(name.to_string_lossy().into_owned())
        }
    }
}

/// Getters
impl Window {
    gen_getter! {
        should_close => WindowShouldClose -> bool;
        is_ready => IsWindowReady -> bool;
        is_fullscreen => IsWindowFullscreen -> bool;
        is_hidden => IsWindowHidden -> bool;
        is_minimized => IsWindowMinimized -> bool;
        is_maximized => IsWindowMaximized -> bool;
        is_focused => IsWindowFocused -> bool;
        is_window_resized => IsWindowResized -> bool;
        get_screen_width => GetScreenWidth -> u32;
        get_screen_height => GetScreenHeight -> u32;
        get_render_width => GetRenderWidth -> u32;
        get_render_height => GetRenderHeight -> u32;
        get_monitor_count => GetMonitorCount -> u32;
        get_position => GetWindowPosition -> Vector2;
        get_scale_dpi => GetWindowScaleDPI -> Vector2;
        get_fps => GetFPS -> u32;
    }

    pub fn is_state(&self, flag: ConfigFlags) -> bool {
        unsafe { sys::IsWindowState(flag.bits()) }
    }

    pub fn get_monitor_position(&self, monitor: u32) -> Vector2 {
        unsafe { sys::GetMonitorPosition(monitor.try_into().unwrap()) }
    }

    pub fn get_current_monitor(&self) -> Monitor {
        Monitor(unsafe { sys::GetCurrentMonitor() }.try_into().unwrap())
    }

    pub fn get_monitor(&self, monitor: u32) -> Monitor {
        Monitor(monitor)
    }

    pub fn get_monitors(&self) -> impl Iterator<Item = Monitor> {
        (0..self.get_monitor_count()).map(|m| self.get_monitor(m))
    }

    pub fn get_clipboard_text(&self) -> Option<String> {
        let text = unsafe { sys::GetClipboardText() };
        if text.is_null() {
            None
        } else {
            let text = unsafe { CStr::from_ptr(text) };
            Some(text.to_string_lossy().into_owned())
        }
    }

    pub fn get_clipboard_image(&self) -> Option<Image> {
        let img = unsafe { sys::GetClipboardImage() };
        Image::from_sys(img)
    }
}

/// Setters
impl Window {
    /// Set window configuration state using flags
    pub fn set_state(&mut self, flags: ConfigFlags) {
        unsafe { sys::SetWindowState(flags.bits()) }
    }

    /// Clear window configuration state flags
    pub fn clear_state(&mut self, flags: ConfigFlags) {
        unsafe { sys::ClearWindowState(flags.bits()) }
    }

    pub fn toggle_fullscreen(&mut self) {
        unsafe { sys::ToggleFullscreen() }
    }

    pub fn toggle_bordeless_windowed(&mut self) {
        unsafe { sys::ToggleBorderlessWindowed() }
    }

    pub fn maximize(&mut self) {
        unsafe { sys::MaximizeWindow() }
    }

    pub fn minimize(&mut self) {
        unsafe { sys::MinimizeWindow() }
    }

    pub fn restore(&mut self) {
        unsafe { sys::RestoreWindow() }
    }

    pub fn set_icon(&mut self, image: &Image) {
        unsafe { sys::SetWindowIcon(image.inner()) }
    }

    pub fn set_icons(&mut self, images: &[Image]) {
        if images.len() == 1 {
            self.set_icon(&images[0]);
            return;
        }
        let mut images = images.iter().map(Image::inner).collect::<Vec<_>>();
        unsafe { sys::SetWindowIcons(images.as_mut_ptr(), images.len() as _) }
    }

    pub fn set_title(&mut self, title: impl AsRef<str>) {
        let title = CString::new(title.as_ref()).expect("str can't contain null");
        unsafe { sys::SetWindowTitle(title.as_ptr()) }
    }

    pub fn set_position(&mut self, x: u32, y: u32) {
        unsafe { sys::SetWindowPosition(x as _, y as _) }
    }

    pub fn set_monitor(&mut self, monitor: u32) {
        unsafe { sys::SetWindowMonitor(monitor as _) }
    }

    pub fn set_min_size(&mut self, width: u32, height: u32) {
        unsafe { sys::SetWindowMinSize(width as _, height as _) }
    }

    pub fn set_max_size(&mut self, width: u32, height: u32) {
        unsafe { sys::SetWindowMaxSize(width as _, height as _) }
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        unsafe { sys::SetWindowSize(width as _, height as _) }
    }

    pub fn set_opacity(&mut self, opacity: f32) {
        unsafe { sys::SetWindowOpacity(opacity) }
    }

    pub fn set_focused(&mut self) {
        unsafe { sys::SetWindowFocused() }
    }

    pub fn enable_event_listening(&mut self) {
        unsafe { sys::EnableEventWaiting() }
    }

    pub fn disable_event_listening(&mut self) {
        unsafe { sys::DisableEventWaiting() }
    }
}
