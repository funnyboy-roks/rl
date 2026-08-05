use std::{cell::OnceCell, path::Path};

use raylib_sys::{self as sys, Rectangle};

use crate::{math::Vector2, texture::Texture2D};

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct GlyphInfo(sys::GlyphInfo);

impl GlyphInfo {
    /// Character value (Unicode)
    pub fn value(self) -> char {
        char::from_u32(self.0.value as _).unwrap()
    }

    pub fn offset(self) -> Vector2 {
        Vector2::new(self.offset_x() as _, self.offset_y() as _)
    }

    /// Character offset X when drawing
    pub fn offset_x(self) -> i32 {
        self.0.offsetX
    }

    /// Character offset Y when drawing
    pub fn offset_y(self) -> i32 {
        self.0.offsetY
    }

    /// Character advance position X
    pub fn advance_x(self) -> i32 {
        self.0.advanceX
    }
}

#[derive(Debug)]
pub struct Font {
    base_size: u32,
    glyph_count: usize,
    glyph_padding: i32,
    texture: Texture2D,
    recs: *mut Rectangle,
    glyphs: *mut GlyphInfo,
}

thread_local! {
    // Make sure we only load the Rc<Texture2D> once
    static DEFAULT_FONT: OnceCell<Font> = const { OnceCell::new() };
}

impl Default for Font {
    fn default() -> Self {
        DEFAULT_FONT.with(|default| {
            default
                .get_or_init(|| {
                    let def = Self::from_sys(unsafe { sys::GetFontDefault() })
                        .expect("default font is valid");
                    // hack to make the drop never happen (Rc still always has one reference)
                    std::mem::forget(def.texture().inner());
                    def
                })
                .clone()
        })
    }
}

impl Font {
    pub(crate) fn from_sys(sys: sys::Font) -> Option<Self> {
        Self::is_valid(sys).then(|| Self {
            base_size: sys.baseSize as _,
            glyph_count: sys.glyphCount as _,
            glyph_padding: sys.glyphPadding,
            texture: Texture2D::from_sys(sys.texture).expect("Invalid font texture"),
            recs: sys.recs,
            glyphs: sys.glyphs.cast(),
        })
    }

    pub(crate) fn is_valid(font: sys::Font) -> bool {
        unsafe { sys::IsFontValid(font) }
    }

    pub(crate) fn to_sys(&self) -> sys::Font {
        sys::Font {
            baseSize: self.base_size as _,
            glyphCount: self.glyph_count as _,
            glyphPadding: self.glyph_padding,
            texture: *self.texture.inner(),
            recs: self.recs.cast(),
            glyphs: self.glyphs.cast(),
        }
    }

    // private clone
    pub(crate) fn clone(&self) -> Self {
        let Self {
            base_size,
            glyph_count,
            glyph_padding,
            ref texture,
            recs,
            glyphs,
        } = *self;
        Self {
            base_size,
            glyph_count,
            glyph_padding,
            texture: texture.clone(),
            recs,
            glyphs,
        }
    }

    pub fn measure_text(&self, text: impl AsRef<str>, font_size: f32, spacing: f32) -> Vector2 {
        // SAFETY: MesaureTextEx does not store this
        let text = unsafe { crate::util::allocate_cstring(text.as_ref()) };
        unsafe { sys::MeasureTextEx(self.to_sys(), text.as_ptr(), font_size, spacing) }.into()
    }

    pub fn base_size(&self) -> u32 {
        self.base_size
    }

    pub fn glyph_count(&self) -> usize {
        self.glyph_count
    }

    pub fn glyph_padding(&self) -> i32 {
        self.glyph_padding
    }

    pub fn texture(&self) -> &Texture2D {
        &self.texture
    }

    /// Get glyph index position in font for a codepoint (unicode character), fallback to '?' if not
    /// found
    pub fn get_glyph_index(&self, codepoint: char) -> usize {
        unsafe { sys::GetGlyphIndex(self.to_sys(), u32::from(codepoint) as _) }
            .try_into()
            .unwrap()
    }

    pub fn glyphs(&self) -> &[GlyphInfo] {
        unsafe { std::slice::from_raw_parts(self.glyphs.cast_const(), self.glyph_count()) }
    }

    pub fn recs(&self) -> &[Rectangle] {
        unsafe { std::slice::from_raw_parts(self.recs.cast_const(), self.glyph_count()) }
    }
}

/// Constructors
impl Font {
    pub fn load(file: impl AsRef<Path>) -> std::io::Result<Self> {
        // TODO: replace with native implemenation for io::result
        // SAFETY: LoadFont doesn't store this
        let file = unsafe {
            crate::util::allocate_cstring_bytes(file.as_ref().as_os_str().as_encoded_bytes())
        }
        .expect("path has no null");
        let font = Self::from_sys(unsafe { sys::LoadFont(file.as_ptr()) });
        if let Some(font) = font {
            Ok(font)
        } else {
            Err(std::io::Error::other("Unable to load font"))
        }
    }
}

pub fn measure(text: impl AsRef<str>, font_size: u32) -> u32 {
    // SAFETY: MesaureText does not store this
    let text = unsafe { crate::util::allocate_cstring(text.as_ref()) };
    unsafe { sys::MeasureText(text.as_ptr(), font_size as _) }
        .try_into()
        .unwrap()
}
