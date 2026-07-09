use std::{
    ffi::{CStr, CString},
    io::{Read, Seek},
    path::Path,
    str::FromStr,
};

use raylib_sys::{self as sys, PixelFormat};

use crate::{
    Bounded, Color, DrawTarget, Rectangle, Texture2D, Vector2, bytes::RlSlice, window::Window,
};

macro_rules! filetype {
    { $($ty: ident => $ext0: literal[$ext0c: literal] $(, $ext: literal)*;)* } => {
        #[derive(Debug, Clone, Copy)]
        pub enum FileType {
            $($ty),*
        }

        impl FileType {
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$ty => $ext0),*
                }
            }

            pub(crate) const fn as_cstr(self) -> &'static CStr {
                match self {
                    $(Self::$ty => $ext0c),*
                }
            }

            pub fn from_extension(s: &str) -> Option<Self> {
                match s {
                    $(
                    $ext0 $(|$ext)* $(|concat!(".", $ext))* => Some(Self::$ty),
                    )*
                    _ => None,
                }
            }
        }

    }
}

impl FileType {
    pub fn from_path(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let path = path.as_ref();

        let Some(extension) = path.extension().and_then(|s| s.to_str()) else {
            return Err(std::io::Error::other("Missing File Extension"));
        };

        Self::from_extension(extension)
            .ok_or_else(|| std::io::Error::other(format!("Unknown file extension: {}", extension)))
    }
}

filetype! {
    Png => "png"[c".png"], "PNG";
    Bmp => "bmp"[c".bmp"], "BMP";
    Tga => "tga"[c".tga"], "TGA";
    Jpg => "jpg"[c".jpg"], "JPG", "jpeg", "JPEG";
    Gif => "gif"[c".gif"], "GIF";
    Pic => "pic"[c".pic"], "PIC";
    Ppm => "ppm"[c".ppm"], "PPM", "pgm", ".GM";
    Psd => "psd"[c".psd"], "PSD";
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct InvalidFileType;

impl FromStr for FileType {
    type Err = InvalidFileType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_extension(s).ok_or(InvalidFileType)
    }
}

pub enum ImageResizeMode {
    Bicubic,
    NearestNeighbor,
}

#[derive(Debug)]
pub struct Image(sys::Image);

impl Clone for Image {
    fn clone(&self) -> Self {
        Self(unsafe { sys::ImageCopy(self.0) })
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { sys::UnloadImage(self.0) };
    }
}

impl Image {
    pub(crate) fn from_sys(img: sys::Image) -> Option<Image> {
        Self::is_valid(img).then_some(Self(img))
    }

    pub(crate) fn inner(&self) -> sys::Image {
        self.0
    }

    pub(crate) fn is_valid(image: sys::Image) -> bool {
        unsafe { sys::IsImageValid(image) }
    }

    /// Get image pixel color at (x, y) position
    pub fn get_color(&mut self, position: impl Into<Vector2>) -> Color {
        let position = position.into();
        unsafe { sys::GetImageColor(self.0, position.x as _, position.y as _) }
    }

    pub fn set_format(&mut self, format: PixelFormat) {
        unsafe { sys::ImageFormat(&raw mut self.0, format as _) };
    }

    /// Convert image to POT (power-of-two)
    pub fn to_pot(&mut self, fill: Color) {
        unsafe { sys::ImageToPOT(&raw mut self.0, fill) };
    }

    pub fn crop(&mut self, area: Rectangle) {
        unsafe { sys::ImageCrop(&raw mut self.0, area) };
    }

    /// Crop image depending on alpha value
    pub fn alpha_crop(&mut self, threshold: f32) {
        unsafe { sys::ImageAlphaCrop(&raw mut self.0, threshold) };
    }

    /// Clear alpha channel to desired color
    pub fn alpha_clear(&mut self, color: Color, threshold: f32) {
        unsafe { sys::ImageAlphaClear(&raw mut self.0, color, threshold) };
    }

    /// Apply alpha mask to image
    pub fn alpha_mask(&mut self, alpha_mask: &Image) {
        unsafe { sys::ImageAlphaMask(&raw mut self.0, alpha_mask.0) };
    }

    /// Premultiply alpha channel
    pub fn alpha_premultiply(&mut self) {
        unsafe { sys::ImageAlphaPremultiply(&raw mut self.0) };
    }

    /// Apply Gaussian blur using a box blur approximation
    pub fn blur_gaussian(&mut self, blur_size: u32) {
        unsafe { sys::ImageBlurGaussian(&raw mut self.0, blur_size as _) };
    }

    /// Apply custom square convolution kernel to image
    pub fn kernel_convolution<const KERNEL_SIZE: usize>(
        &mut self,
        kernel: [[f32; KERNEL_SIZE]; KERNEL_SIZE],
    ) {
        let kernel = kernel.as_flattened();
        debug_assert_eq!(kernel.len(), KERNEL_SIZE * KERNEL_SIZE);
        unsafe {
            sys::ImageKernelConvolution(
                &raw mut self.0,
                kernel.as_ptr(),
                (KERNEL_SIZE * KERNEL_SIZE) as _,
            )
        };
    }

    /// Resize image
    pub fn resize(&mut self, new_width: u32, new_height: u32, mode: ImageResizeMode) {
        match mode {
            ImageResizeMode::Bicubic => unsafe {
                sys::ImageResize(&raw mut self.0, new_width as _, new_height as _)
            },
            ImageResizeMode::NearestNeighbor => unsafe {
                sys::ImageResizeNN(&raw mut self.0, new_width as _, new_height as _)
            },
        }
    }

    /// Resize canvas and fill with color
    pub fn resize_canvas(
        &mut self,
        new_width: u32,
        new_height: u32,
        offset: (i32, i32),
        fill: Color,
    ) {
        unsafe {
            sys::ImageResizeCanvas(
                &raw mut self.0,
                new_width as _,
                new_height as _,
                offset.0,
                offset.1,
                fill,
            )
        };
    }

    /// Compute all mipmap levels for a provided image
    pub fn mipmaps(&mut self) {
        unsafe { sys::ImageMipmaps(&raw mut self.0) };
    }

    /// Dither image data to 16bpp or lower (Floyd-Steinberg dithering)
    pub fn dither(&mut self, r_bpp: u32, g_bpp: u32, b_bpp: u32, a_bpp: u32) {
        unsafe {
            sys::ImageDither(
                &raw mut self.0,
                r_bpp as _,
                g_bpp as _,
                b_bpp as _,
                a_bpp as _,
            )
        };
    }

    /// Flip image vertically
    pub fn flip_vertical(&mut self) {
        unsafe { sys::ImageFlipVertical(&raw mut self.0) };
    }

    /// Flip image horizontally
    pub fn flip_horizontal(&mut self) {
        unsafe { sys::ImageFlipHorizontal(&raw mut self.0) };
    }

    /// Rotate image by input angle in degrees (-359 to 359)
    pub fn rotate(&mut self, degrees: i32) {
        unsafe { sys::ImageRotate(&raw mut self.0, degrees) };
    }

    /// Rotate image clockwise 90deg
    pub fn rotate_cw(&mut self) {
        unsafe { sys::ImageRotateCW(&raw mut self.0) };
    }

    /// Rotate image counter-clockwise 90deg
    pub fn rotate_ccw(&mut self) {
        unsafe { sys::ImageRotateCCW(&raw mut self.0) };
    }

    /// Modify image color: tint
    pub fn tint(&mut self, tint: Color) {
        unsafe { sys::ImageColorTint(&raw mut self.0, tint) };
    }

    /// Modify image color: invert
    pub fn invert(&mut self) {
        unsafe { sys::ImageColorInvert(&raw mut self.0) };
    }

    /// Modify image color: grayscale
    pub fn grayscale(&mut self) {
        unsafe { sys::ImageColorGrayscale(&raw mut self.0) };
    }

    /// Modify image color: contrast
    pub fn contrast(&mut self, contrast: f32) {
        unsafe { sys::ImageColorContrast(&raw mut self.0, contrast) };
    }

    /// Modify image color: brightness
    pub fn brightness(&mut self, brightness: i32) {
        unsafe { sys::ImageColorBrightness(&raw mut self.0, brightness) };
    }

    /// Modify image color: replace color
    pub fn color_replace(&mut self, from: Color, to: Color) {
        unsafe { sys::ImageColorReplace(&raw mut self.0, from, to) };
    }

    /// Load color data from image as a Color array (RGBA - 32bit)
    pub fn load_colors(&mut self) -> RlSlice<Color> {
        let colors = unsafe { sys::LoadImageColors(self.0) };
        unsafe {
            RlSlice::from_raw_parts(colors, (self.width() * self.height()) as usize, |ptr| {
                sys::UnloadImageColors(ptr)
            })
        }
    }

    /// Load color palette from image as a Color array (RGBA - 32bit)
    pub fn load_palette(&mut self, max_palette_size: usize) -> RlSlice<Color> {
        let mut len = 0usize;
        let colors =
            unsafe { sys::LoadImagePalette(self.0, max_palette_size as _, (&raw mut len).cast()) };
        unsafe { RlSlice::from_raw_parts(colors, len, |ptr| sys::UnloadImagePalette(ptr)) }
    }

    /// Get image alpha border rectangle
    pub fn get_alpha_border(&mut self, threshold: f32) -> Rectangle {
        unsafe { sys::GetImageAlphaBorder(self.0, threshold) }
    }
}

/// Load
impl Image {
    pub fn load(file: impl AsRef<Path>) -> std::io::Result<Self> {
        // NOTE: We read the file manually so we can use the std::io error
        let file = file.as_ref();
        let ft = FileType::from_path(file)?;
        let contents = std::fs::read(file)?;

        let image = Self::load_from_memory(ft, &contents);

        if let Some(image) = image {
            Ok(image)
        } else {
            Err(std::io::Error::other("Unable to load image"))
        }
    }

    pub fn load_from_memory(kind: FileType, data: &[u8]) -> Option<Self> {
        Self::from_sys(unsafe {
            sys::LoadImageFromMemory(
                kind.as_cstr().as_ptr(),
                data.as_ptr(),
                data.len().try_into().unwrap(),
            )
        })
    }

    /// Load image from RAW file data
    // this is pretty much the same implementation as in RayLib, but we redo it to use std::io
    // errors
    pub fn load_raw(
        file: impl AsRef<Path>,
        width: u32,
        height: u32,
        format: PixelFormat,
        header_size: u64,
    ) -> std::io::Result<Self> {
        let file = file.as_ref();
        let mut file = std::fs::File::open(file)?;

        let len = file.seek(std::io::SeekFrom::End(0))?;

        if header_size > len {
            return Err(std::io::Error::other(
                "header_len is larger than file length",
            ));
        }

        file.seek(std::io::SeekFrom::Start(header_size))?;

        let mut buf = Vec::with_capacity((len - header_size) as usize);
        file.read_to_end(&mut buf)?;

        let image = sys::Image {
            data: unsafe { sys::MemAlloc(buf.len() as _) },
            width: width as _,
            height: height as _,
            mipmaps: 1,
            format: format as _,
        };

        // SAFETY: We've allocated sizeof<u8> * buf.len bytes in image.data
        unsafe {
            std::ptr::copy_nonoverlapping(buf.as_ptr(), image.data.cast(), buf.len());
        }

        Ok(Self(image))
    }

    /// Load image sequence from file (frames appended to image.data)
    pub fn load_animation(file: impl AsRef<Path>) -> std::io::Result<(Self, u32)> {
        // NOTE: We read the file manually so we can use the std::io error
        let file = file.as_ref();
        let ft = FileType::from_path(file)?;
        let contents = std::fs::read(file)?;

        let anim = Self::load_animation_from_memory(ft, &contents);

        if let Some(anim) = anim {
            Ok(anim)
        } else {
            Err(std::io::Error::other("Unable to load image"))
        }
    }

    /// Load image sequence from memory buffer
    pub fn load_animation_from_memory(kind: FileType, data: &[u8]) -> Option<(Self, u32)> {
        let mut frames = 0u32;
        let img = Self::from_sys(unsafe {
            sys::LoadImageAnimFromMemory(
                kind.as_cstr().as_ptr(),
                data.as_ptr(),
                data.len().try_into().unwrap(),
                (&raw mut frames).cast(),
            )
        });

        img.map(|i| (i, frames))
    }

    /// Load image from GPU texture data
    pub fn from_texture(texture: &Texture2D) -> Self {
        Self::from_sys(unsafe { sys::LoadImageFromTexture(texture.inner()) })
            .expect("LoadImageFromTexture is infallible")
    }

    /// Load image from GPU texture data
    pub fn from_screen(window: &Window) -> Self {
        let _ = window;
        Self::from_sys(unsafe { sys::LoadImageFromScreen() })
            .expect("LoadImageFromScreen is infallible")
    }

    pub fn from_image(image: &Image, rect: Rectangle) -> Self {
        Self::from_sys(unsafe { sys::ImageFromImage(image.0, rect) })
            .expect("ImageFromImage is infallible")
    }

    pub fn from_channel(image: &Image, channel: u32) -> Self {
        Self::from_sys(unsafe { sys::ImageFromChannel(image.0, channel as _) })
            .expect("ImageFromChannel is infallible")
    }

    pub fn from_text(text: impl AsRef<str>, font_size: u32, color: Color) -> Self {
        let text = CString::new(text.as_ref()).expect("str has no null");
        Self::from_sys(unsafe { sys::ImageText(text.as_ptr(), font_size as _, color) })
            .expect("ImageText is infallible")
    }

    // TODO
    // pub fn from_text_ex() -> Self {}
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
        unsafe { RlSlice::from_raw_parts(data, file_size, |ptr| sys::MemFree(ptr.cast())) }
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
