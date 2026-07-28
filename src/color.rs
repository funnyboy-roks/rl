use std::fmt::Display;

use crate::math::Vector3;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
#[repr(C)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl From<Color> for raylib_sys::Color {
    fn from(value: Color) -> Self {
        Self::new(value.r, value.g, value.b, value.a)
    }
}

impl From<raylib_sys::Color> for Color {
    fn from(value: raylib_sys::Color) -> Self {
        Self::new(value.r, value.g, value.b, value.a)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#{:02x}{:02x}{:02x}{:02x}",
            self.r, self.g, self.b, self.a
        )
    }
}

/// Constructors
impl Color {
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Color from integer in the format of `0xRRGGBBAA`
    #[must_use]
    pub const fn from_int(int: u32) -> Self {
        #[allow(clippy::identity_op, clippy::erasing_op, reason = "1*8 and 0*8")]
        Self {
            r: (int >> (3 * 8)) as u8 & 0xff,
            g: (int >> (2 * 8)) as u8 & 0xff,
            b: (int >> (1 * 8)) as u8 & 0xff,
            a: (int >> (0 * 8)) as u8 & 0xff,
        }
    }

    /// Create Color from normalized values (0..1)
    ///
    /// This is the inverse of [Self::to_normalized]
    #[must_use]
    pub const fn from_normalized([r, g, b, a]: [f32; 4]) -> Self {
        assert!(0. <= r && r <= 1., "r out of bounds");
        assert!(0. <= g && g <= 1., "g out of bounds");
        assert!(0. <= b && b <= 1., "b out of bounds");
        assert!(0. <= a && a <= 1., "a out of bounds");
        Color::new(
            (r * 255.) as u8,
            (g * 255.) as u8,
            (b * 255.) as u8,
            (a * 255.) as u8,
        )
    }

    /// Get a Color from HSV
    ///
    /// `hue` is provided in degrees: [0..360]
    /// `saturation`/`value` are provided normalized: [0..1]
    ///
    /// This is approximately the inverse of [Self::to_hsv]
    ///
    /// # NOTE
    ///
    /// Color->HSV->Color conversion will not yield exactly the same color due to rounding errors
    // Implementation reference: https://en.wikipedia.org/wiki/HSL_and_HSV#Alternative_HSV_conversion
    #[must_use]
    pub const fn from_hsv(hue: f32, saturation: f32, value: f32) -> Self {
        #[inline]
        const fn calc(x: f32, hue: f32, value: f32, saturation: f32) -> u8 {
            let k = (x + hue / 60.0) % 6.;
            let t = 4.0 - k;
            let k = if t < k { t } else { k };
            let k = if k < 1. { k } else { 1. };
            let k = if k > 0. { k } else { 0. };
            ((value - value * saturation * k) * 255.0) as u8
        }

        let r = calc(5., hue, value, saturation);
        let g = calc(3., hue, value, saturation);
        let b = calc(1., hue, value, saturation);

        Color::new(r, g, b, 255)
    }
}

/// Default Palette
impl Color {
    /// Light Gray
    pub const LIGHTGRAY: Self = Self::new(200, 200, 200, 255);
    /// Gray
    pub const GRAY: Self = Self::new(130, 130, 130, 255);
    /// Dark Gray
    pub const DARKGRAY: Self = Self::new(80, 80, 80, 255);
    /// Yellow
    pub const YELLOW: Self = Self::new(253, 249, 0, 255);
    /// Gold
    pub const GOLD: Self = Self::new(255, 203, 0, 255);
    /// Orange
    pub const ORANGE: Self = Self::new(255, 161, 0, 255);
    /// Pink
    pub const PINK: Self = Self::new(255, 109, 194, 255);
    /// Red
    pub const RED: Self = Self::new(230, 41, 55, 255);
    /// Maroon
    pub const MAROON: Self = Self::new(190, 33, 55, 255);
    /// Green
    pub const GREEN: Self = Self::new(0, 228, 48, 255);
    /// Lime
    pub const LIME: Self = Self::new(0, 158, 47, 255);
    /// Dark Green
    pub const DARKGREEN: Self = Self::new(0, 117, 44, 255);
    /// Sky Blue
    pub const SKYBLUE: Self = Self::new(102, 191, 255, 255);
    /// Blue
    pub const BLUE: Self = Self::new(0, 121, 241, 255);
    /// Dark Blue
    pub const DARKBLUE: Self = Self::new(0, 82, 172, 255);
    /// Purple
    pub const PURPLE: Self = Self::new(200, 122, 255, 255);
    /// Violet
    pub const VIOLET: Self = Self::new(135, 60, 190, 255);
    /// Dark Purple
    pub const DARKPURPLE: Self = Self::new(112, 31, 126, 255);
    /// Beige
    pub const BEIGE: Self = Self::new(211, 176, 131, 255);
    /// Brown
    pub const BROWN: Self = Self::new(127, 106, 79, 255);
    /// Dark Brown
    pub const DARKBROWN: Self = Self::new(76, 63, 47, 255);

    /// White
    pub const WHITE: Self = Self::new(255, 255, 255, 255);
    /// Black
    pub const BLACK: Self = Self::new(0, 0, 0, 255);
    /// Blank (Transparent)
    pub const BLANK: Self = Self::new(0, 0, 0, 0);
    /// Magenta
    pub const MAGENTA: Self = Self::new(255, 0, 255, 255);
    /// My own White (raylib logo)
    pub const RAYWHITE: Self = Self::new(245, 245, 245, 255);
}

impl Color {
    /// Convert this colour into normalized form (0..1)
    ///
    /// This is the inverse of [Self::from_normalized]
    #[must_use]
    pub const fn to_normalized(self) -> [f32; 4] {
        [
            self.r as f32 / 255.,
            self.g as f32 / 255.,
            self.b as f32 / 255.,
            self.a as f32 / 255.,
        ]
    }

    /// Get a Color from HSV
    ///
    /// `hue` is provided in degrees: [0..360]
    /// `saturation`/`value` are provided normalized: [0..1]
    ///
    /// This is approximately the inverse of [Self::to_hsv]
    ///
    /// # NOTE
    ///
    /// Color->HSV->Color conversion will not yield exactly the same color due to rounding errors
    // NOTE: manually implemented rather than calling the raylib function so it can be const
    #[must_use]
    pub const fn to_hsv(self) -> Vector3 {
        let rgb = Vector3::new(
            self.r as f32 / 255.,
            self.g as f32 / 255.,
            self.b as f32 / 255.,
        );

        let min = if rgb.x < rgb.y { rgb.x } else { rgb.y };
        let min = if min < rgb.z { min } else { rgb.z };

        let max = if rgb.x > rgb.y { rgb.x } else { rgb.y };
        let max = if max > rgb.z { max } else { rgb.z };

        let v = max;

        let delta = max - min;

        if delta < 0.00001 {
            // Undefined, maybe NAN?
            return Vector3::ZERO;
        }

        let s = if max > 0. {
            // NOTE: If max is 0, this divide would cause a crash
            delta / max // Saturation
        } else {
            return Vector3::ZERO;
        };

        // NOTE: Comparing float values could not work properly
        let mut h = if rgb.x >= max {
            // Between yellow & magenta
            (rgb.y - rgb.z) / delta
        } else if rgb.y >= max {
            // Between cyan & yellow
            2. + (rgb.z - rgb.x) / delta
        } else {
            // Between magenta & cyan
            4. + (rgb.x - rgb.y) / delta
        };

        h *= 60.; // Convert to degrees

        if h < 0. {
            h += 360.;
        }

        Vector3::new(h, s, v)
    }
}

/// Operations
impl Color {
    /// Get color multiplied with another color
    #[must_use]
    pub const fn tint(self, other: Color) -> Color {
        Color::new(
            (self.r as u32 * other.r as u32 / 255) as u8,
            (self.g as u32 * other.g as u32 / 255) as u8,
            (self.b as u32 * other.b as u32 / 255) as u8,
            (self.a as u32 * other.a as u32 / 255) as u8,
        )
    }

    /// Get color with brightness correction
    ///
    /// `factor` must be in range -1..1
    #[must_use]
    pub const fn brightness(self, factor: f32) -> Color {
        assert!(-1. <= factor && factor <= 1.0, "factor out of range");

        let r = self.r as f32;
        let g = self.g as f32;
        let b = self.b as f32;

        let (r, g, b) = if factor < 0. {
            let factor = 1. + factor;
            (r * factor, g * factor, b * factor)
        } else {
            (
                (255. - r) * factor + r,
                (255. - g) * factor + g,
                (255. - b) * factor + b,
            )
        };

        Color::new(r as u8, g as u8, b as u8, self.a)
    }

    /// Get color with contrast correction
    ///
    /// `factor` must be in range -1..1
    #[must_use]
    pub const fn contrast(self, factor: f32) -> Color {
        assert!(-1. <= factor && factor <= 1.0, "factor out of range");

        let factor = 1. + factor;
        let factor = factor * factor;

        const fn calc(comp: u8, factor: f32) -> u8 {
            let p = (((comp as f32 / 255.) - 0.5) * factor + 0.5) * 255.;
            p.clamp(0., 255.) as u8
        }

        Color::new(
            calc(self.r, factor),
            calc(self.g, factor),
            calc(self.b, factor),
            self.a,
        )
    }

    /// Get color with alpha applied
    ///
    /// `alpha` must be in range 0..1
    #[must_use]
    pub const fn alpha(self, alpha: f32) -> Color {
        assert!(0. <= alpha && alpha <= 1.0, "alpha out of range");
        Color::new(self.r, self.g, self.b, (alpha * 255.) as u8)
    }

    /// Get `self` alpha-blended into `dest` with `tint`
    #[must_use]
    pub const fn alpha_blend(self, dest: Color, tint: Color) -> Color {
        let tinted = Color::new(
            ((self.r as u32 * (tint.r as u32 + 1)) >> 8) as u8,
            ((self.g as u32 * (tint.g as u32 + 1)) >> 8) as u8,
            ((self.b as u32 * (tint.b as u32 + 1)) >> 8) as u8,
            ((self.a as u32 * (tint.a as u32 + 1)) >> 8) as u8,
        );

        if tinted.a == 0 {
            dest
        } else if tinted.a == 255 {
            tinted
        } else {
            let alpha = tinted.a as u32 + 1; // Shifting by 8 (dividing by 256), so need to take that excess into account
            let result_a = (alpha * 256 + dest.a as u32 * (256 - alpha)) >> 8;

            if result_a == 0 {
                Self::WHITE
            } else {
                // NOTE: macro, not closure, because const
                macro_rules! calc {
                    ($comp: ident) => {
                        (((tinted.$comp as u32 * alpha * 256
                            + dest.$comp as u32 * dest.a as u32 * (256 - alpha))
                            / result_a)
                            >> 8) as u8
                    };
                }
                Color::new(calc!(r), calc!(g), calc!(b), result_a as u8)
            }
        }
    }

    /// Get color lerp interpolation between two colors, factor [0..1]
    ///
    /// # NOTE
    ///
    /// If factor > 1 or < 0, then the result colour may not be between `self` and `end`.
    #[must_use]
    pub const fn lerp(self, end: Color, amount: f32) -> Color {
        Color::new(
            ((1. - amount) * self.r as f32 + amount * end.r as f32) as u8,
            ((1. - amount) * self.g as f32 + amount * end.g as f32) as u8,
            ((1. - amount) * self.b as f32 + amount * end.b as f32) as u8,
            ((1. - amount) * self.a as f32 + amount * end.a as f32) as u8,
        )
    }
}

#[cfg(test)]
mod test {
    use crate::color::Color;

    #[test]
    fn color_to_hsv_to_color() {
        // 10 random colours
        let colors = [
            Color::from_int(0x480de6ff),
            Color::from_int(0xe9bfa2ff),
            Color::from_int(0x71718dff),
            Color::from_int(0x950534ff),
            Color::from_int(0x979bbdff),
            Color::from_int(0xe9f7a9ff),
            Color::from_int(0xa31ba0ff),
            Color::from_int(0x2807c5ff),
            Color::from_int(0xa37876ff),
            Color::from_int(0xfbefe5ff),
        ];

        for c in colors {
            let hsv = c.to_hsv();
            let c2 = Color::from_hsv(hsv.x, hsv.y, hsv.z);

            // A little fuzzy because of rounding errors
            let dr = c.r.abs_diff(c2.r);
            let dg = c.g.abs_diff(c2.g);
            let db = c.b.abs_diff(c2.b);

            assert!(dr < 2);
            assert!(dg < 2);
            assert!(db < 2);
        }
    }
}
