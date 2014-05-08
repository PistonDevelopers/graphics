use {
    CanColor,
    Color,
    HasColor,
};

/// Implemented by contexts that contains color.
pub trait RelativeColor<'a> {
    /// Multiplies with red, green, blue and alpha values.
    fn mul_rgba(&'a self, r: f32, g: f32, b: f32, a: f32) -> Self;

    /// Mixes the current color with white.
    ///
    /// 0 is black and 1 is white.
    #[inline(always)]
    fn tint(&'a self, f: f32) -> Self {
        self.mul_rgba(f, f, f, 1.0)
    }

    /// Mixes the current color with black.
    ///
    /// 0 is white and 1 is black.
    #[inline(always)]
    fn shade(&'a self, f: f32) -> Self {
        let f = 1.0 - f;
        self.mul_rgba(f, f, f, 1.0)
    }
}

impl<
    'a,
    T: HasColor<'a, Color> + CanColor<'a, T, Color>
> RelativeColor<'a> for T {
    #[inline(always)]
    fn mul_rgba(&'a self, r: f32, g: f32, b: f32, a: f32) -> T {
        let color = self.get_color();
        self.color([color[0] * r, color[1] * g, color[2] * b, color[3] * a])
    }
}

