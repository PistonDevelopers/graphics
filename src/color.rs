//! Helper methods for colors

use internal;

pub const WHITE: internal::Color = [1.0; 4];
pub const BLACK: internal::Color = [0.0, 0.0, 0.0, 1.0];
pub const TRANSPARENT: internal::Color = [0.0; 4];

/// Returns a grey color
pub fn grey(f: internal::ColorComponent) -> internal::Color {
    [f, f, f, 1.0]
}

/// Returns a semi-transparent white color
pub fn alpha(f: internal::ColorComponent) -> internal::Color {
    [1.0, 1.0, 1.0, f]
}

/// Converts from hexadecimal color format
pub fn hex(hex: &str) -> internal::Color {
    use read_color::rgb_maybe_a;

    let (rgb, a) = rgb_maybe_a(&mut hex.chars()).unwrap();
    let color = match a {
        None => [rgb[0], rgb[1], rgb[2], 255],
        Some(a) => [rgb[0], rgb[1], rgb[2], a]
    };
    let inv_255 = 1.0f32 / 255.0f32;
    [
        color[0] as f32 * inv_255,
        color[1] as f32 * inv_255,
        color[2] as f32 * inv_255,
        color[3] as f32 * inv_255
    ]
}
