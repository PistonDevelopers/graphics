//! Helper methods for colors

use types::{Color, ColorComponent};

/// White color.
pub const WHITE: Color = [1.0; 4];
/// Black color.
pub const BLACK: Color = [0.0, 0.0, 0.0, 1.0];
/// Transparent color.
pub const TRANSPARENT: Color = [0.0; 4];

/// Returns a grey color
pub fn grey(f: ColorComponent) -> Color {
    [f, f, f, 1.0]
}

/// Returns a semi-transparent white color
pub fn alpha(f: ColorComponent) -> Color {
    [1.0, 1.0, 1.0, f]
}

/// Converts from hexadecimal color format
pub fn hex(hex: &str) -> Color {
    use read_color::rgb_maybe_a;

    let (rgb, a) = rgb_maybe_a(&mut hex.chars()).unwrap();
    let color = match a {
        None => [rgb[0], rgb[1], rgb[2], 255],
        Some(a) => [rgb[0], rgb[1], rgb[2], a],
    };
    let inv_255 = 1.0f32 / 255.0f32;
    [color[0] as f32 * inv_255,
     color[1] as f32 * inv_255,
     color[2] as f32 * inv_255,
     color[3] as f32 * inv_255]
}

#[inline(always)]
fn component_srgb_to_linear(f: ColorComponent) -> ColorComponent {
    if f <= 0.04045 {
        f / 12.92
    } else {
        ((f + 0.055) / 1.055).powf(2.4)
    }
}

/// Converts gamma (brightness) from sRGB to linear color space.
///
/// sRGB is the default color space for image editors, pictures, internet etc.
/// Linear gamma yields better results when doing math with colors.
pub fn gamma_srgb_to_linear(c: Color) -> Color {
    [component_srgb_to_linear(c[0]),
     component_srgb_to_linear(c[1]),
     component_srgb_to_linear(c[2]),
     c[3]]
}

#[inline(always)]
fn component_linear_to_srgb(f: ColorComponent) -> ColorComponent {
    if f <= 0.0031308 {
        f * 12.92
    } else {
        1.055 * f.powf(1.0 / 2.4) - 0.055
    }
}

/// Converts gamma (brightness) of a color from linear color space to sRGB.
///
/// sRGB is the default color space for image editors, pictures, internet etc.
/// Linear gamma yields better results when doing math with colors.
pub fn gamma_linear_to_srgb(c: Color) -> Color {
    [component_linear_to_srgb(c[0]),
     component_linear_to_srgb(c[1]),
     component_linear_to_srgb(c[2]),
     c[3]]
}
