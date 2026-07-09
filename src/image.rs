use std::{
    ffi::{CStr, CString},
    path::Path,
    str::FromStr,
};

use raylib_sys as sys;

use crate::{Bounded, Color, DrawTarget, Rectangle, Vector2, bytes::RlBytesOwned};

#[derive(Debug, Clone, Copy)]
pub enum FileType {
    Png,
    Bmp,
    Tga,
    Jpg,
    Gif,
    Pic,
    Ppm,
    Psd,
}

impl FileType {
    pub const fn as_str(self) -> &'static str {
        match self {
            FileType::Png => ".png",
            FileType::Bmp => ".bmp",
            FileType::Tga => ".tga",
            FileType::Jpg => ".jpg",
            FileType::Gif => ".gif",
            FileType::Pic => ".pic",
            FileType::Ppm => ".ppm",
            FileType::Psd => ".psd",
        }
    }

    pub(crate) const fn as_cstr(self) -> &'static CStr {
        match self {
            FileType::Png => c".png",
            FileType::Bmp => c".bmp",
            FileType::Tga => c".tga",
            FileType::Jpg => c".jpg",
            FileType::Gif => c".gif",
            FileType::Pic => c".pic",
            FileType::Ppm => c".ppm",
            FileType::Psd => c".psd",
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            ".png" | ".PNG" => Some(Self::Png),
            ".bmp" | ".BMP" => Some(Self::Bmp),
            ".tga" | ".TGA" => Some(Self::Tga),
            ".jpg" | ".JPG" | ".jpeg" | ".JPEG" => Some(Self::Jpg),
            ".gif" | ".GIF" => Some(Self::Gif),
            ".pic" | ".PIC" => Some(Self::Pic),
            ".ppm" | ".PPM" | ".pgm" | ".PGM" => Some(Self::Ppm),
            ".psd" | ".PSD" => Some(Self::Psd),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct InvalidFileType;

impl FromStr for FileType {
    type Err = InvalidFileType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s).ok_or(InvalidFileType)
    }
}

#[derive(Debug)]
pub struct Image(sys::Image);

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { sys::UnloadImage(self.0) };
    }
}

impl Image {
    pub(crate) fn from_sys(img: sys::Image) -> Option<Image> {
        if img.data.is_null() {
            None
        } else {
            Some(Self(img))
        }
    }

    pub(crate) fn inner(&self) -> sys::Image {
        self.0
    }
}

/// Load
impl Image {
    pub fn load(file: impl AsRef<Path>) -> std::io::Result<Self> {
        let file = file.as_ref();
        let Some(extension) = file.extension() else {
            return Err(std::io::Error::other("Missing File Extension"));
        };
        let contents = std::fs::read(file)?;
        let extension = {
            let mut vec = Vec::with_capacity(extension.as_encoded_bytes().len() + 1);
            vec.push(b'.');
            vec.extend_from_slice(extension.as_encoded_bytes());
            CString::new(vec).expect("Path can't contain null")
        };
        let image = Self::from_sys(unsafe {
            sys::LoadImageFromMemory(
                extension.as_ptr(),
                contents.as_ptr(),
                contents.len().try_into().unwrap(),
            )
        });

        if let Some(image) = image {
            Ok(image)
        } else {
            Err(std::io::Error::other("Unable to load image"))
        }
    }
}

/// Export
impl Image {
    pub fn export_to_memory(&self, file_type: FileType) -> impl std::ops::DerefMut<Target = [u8]> {
        let mut file_size: usize = 0;
        let data = unsafe {
            sys::ExportImageToMemory(
                self.0,
                file_type.as_cstr().as_ptr(),
                (&raw mut file_size).cast(),
            )
        };

        // SAFETY: data is from raylib and file_size is correct
        unsafe { RlBytesOwned::from_raw_parts(data, file_size) }
    }
}

/// Generate
impl Image {
    pub fn gen_color(width: u32, height: u32, color: Color) -> Self {
        Self(unsafe { sys::GenImageColor(width as _, height as _, color) })
    }

    /// Generate image: linear gradient, direction in degrees [0..360], 0=Vertical gradient
    pub fn gen_gradient_linear(
        width: u32,
        height: u32,
        direction: u32,
        start: Color,
        end: Color,
    ) -> Self {
        Self(unsafe {
            sys::GenImageGradientLinear(width as _, height as _, direction as _, start, end)
        })
    }

    /// Generate image: radial gradient
    pub fn gen_gradient_radial(
        width: u32,
        height: u32,
        density: f32,
        inner: Color,
        outer: Color,
    ) -> Self {
        Self(unsafe { sys::GenImageGradientRadial(width as _, height as _, density, inner, outer) })
    }

    /// Generate image: square gradient
    pub fn gen_gradient_square(
        width: u32,
        height: u32,
        density: f32,
        inner: Color,
        outer: Color,
    ) -> Self {
        Self(unsafe { sys::GenImageGradientSquare(width as _, height as _, density, inner, outer) })
    }

    /// Generate image: checked
    pub fn gen_checked(
        width: u32,
        height: u32,
        checks_x: u32,
        checks_y: u32,
        col1: Color,
        col2: Color,
    ) -> Self {
        Self(unsafe {
            sys::GenImageChecked(
                width as _,
                height as _,
                checks_x as _,
                checks_y as _,
                col1,
                col2,
            )
        })
    }

    pub fn gen_white_noise(width: u32, height: u32, factor: f32) -> Self {
        Self(unsafe { sys::GenImageWhiteNoise(width as _, height as _, factor) })
    }

    pub fn gen_perlin_noise(
        width: u32,
        height: u32,
        offset_x: u32,
        offset_y: u32,
        scale: f32,
    ) -> Self {
        Self(unsafe {
            sys::GenImagePerlinNoise(width as _, height as _, offset_x as _, offset_y as _, scale)
        })
    }

    pub fn gen_cellular(width: u32, height: u32, tile_size: u32) -> Self {
        Self(unsafe { sys::GenImageCellular(width as _, height as _, tile_size as _) })
    }

    pub fn gen_text(width: u32, height: u32, text: impl AsRef<str>) -> Self {
        let text = CString::new(text.as_ref()).expect("str has no null");
        Self(unsafe { sys::GenImageText(width as _, height as _, text.as_ptr()) })
    }
}

/// Extra Drawing Functions
impl Image {
    pub fn draw_image(&mut self, image: &Image, src: Rectangle, dst: Rectangle, tint: Color) {
        unsafe { sys::ImageDraw(&raw mut self.0, image.0, src, dst, tint) };
    }

    pub fn draw_triangle_ex(
        &mut self,
        p1: (impl Into<Vector2>, Color),
        p2: (impl Into<Vector2>, Color),
        p3: (impl Into<Vector2>, Color),
    ) {
        unsafe {
            sys::ImageDrawTriangleEx(
                &raw mut self.0,
                p1.0.into(),
                p2.0.into(),
                p3.0.into(),
                p1.1,
                p2.1,
                p3.1,
            )
        };
    }
}

impl Bounded for Image {
    fn width(&self) -> u32 {
        self.0.width as _
    }

    fn height(&self) -> u32 {
        self.0.height as _
    }
}

impl DrawTarget for Image {
    fn clear_background(&mut self, color: Color) {
        unsafe { sys::ImageClearBackground(&raw mut self.0, color) };
    }

    fn draw_pixel(&mut self, position: impl Into<Vector2>, color: Color) {
        unsafe { sys::ImageDrawPixelV(&raw mut self.0, position.into(), color) };
    }

    fn draw_line(
        &mut self,
        from: impl Into<Vector2>,
        to: impl Into<Vector2>,
        thick: f32,
        color: Color,
    ) {
        unsafe { sys::ImageDrawLineEx(&raw mut self.0, from.into(), to.into(), thick as _, color) };
    }

    fn draw_circle(&mut self, center: impl Into<Vector2>, radius: f32, color: Color) {
        let center = center.into();
        unsafe {
            sys::ImageDrawCircle(
                &raw mut self.0,
                center.x as _,
                center.y as _,
                radius as _,
                color,
            )
        };
    }

    fn draw_circle_lines(&mut self, center: impl Into<Vector2>, radius: f32, color: Color) {
        unsafe { sys::ImageDrawCircleLinesV(&raw mut self.0, center.into(), radius as _, color) };
    }

    fn draw_rectangle(&mut self, rect: Rectangle, color: Color) {
        unsafe { sys::ImageDrawRectangleRec(&raw mut self.0, rect, color) };
    }

    fn draw_rectangle_lines(&mut self, rect: Rectangle, line_thick: f32, color: Color) {
        unsafe { sys::ImageDrawRectangleLines(&raw mut self.0, rect, line_thick as _, color) };
    }

    fn draw_triangle(
        &mut self,
        p1: impl Into<Vector2>,
        p2: impl Into<Vector2>,
        p3: impl Into<Vector2>,
        color: Color,
    ) {
        self.draw_triangle_ex((p1, color), (p2, color), (p3, color));
    }

    fn draw_triangle_lines(
        &mut self,
        p1: impl Into<Vector2>,
        p2: impl Into<Vector2>,
        p3: impl Into<Vector2>,
        color: Color,
    ) {
        unsafe {
            sys::ImageDrawTriangleLines(&raw mut self.0, p1.into(), p2.into(), p3.into(), color)
        };
    }

    fn draw_triangle_fan(&mut self, points: &[Vector2], color: Color) {
        unsafe {
            sys::ImageDrawTriangleFan(&raw mut self.0, points.as_ptr(), points.len() as _, color)
        };
    }

    fn draw_triangle_strip(&mut self, points: &[Vector2], color: Color) {
        unsafe {
            sys::ImageDrawTriangleStrip(&raw mut self.0, points.as_ptr(), points.len() as _, color)
        };
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
        unsafe {
            sys::ImageDrawText(
                &raw mut self.0,
                text.as_ptr(),
                pos.x as _,
                pos.y as _,
                font_size as _,
                color,
            )
        };
    }
}
