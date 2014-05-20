use internal::{
    Color,
};

/// Implemented by contexts who can add color.
pub trait AddColor<'a, T> {
    /// Add color with alpha channel.
    fn rgba(&'a self, r: f32, g: f32, b: f32, a: f32) -> T;

    /// Adds color with alpha channel set to 1.0.
    #[inline(always)]
    fn rgb(&'a self, r: f32, g: f32, b: f32) -> T {
        self.rgba(r, g, b, 1.0)
    }

    /// Add color [r, g, b, a].
    #[inline(always)]
    fn color(&'a self, color: Color) -> T {
        self.rgba(color[0], color[1], color[2], color[3])
    }

    /// Adds a gray color.
    ///
    /// `0.0` is black and `1.0` is white.
    #[inline(always)]
    fn grey(&'a self, f: f32) -> T {
        self.rgba(f, f, f, 1.0)
    }

    /// Adds a white semi-transparent color.
    ///
    /// `0.0` is fully transparent and `1.0` is fully opaque.
    #[inline(always)]
    fn alpha(&'a self, f: f32) -> T {
        self.rgba(1.0, 1.0, 1.0, f)
    }
}

