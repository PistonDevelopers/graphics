use {
    CanColor,
    Color,
    HasColor,
};

/// Implemented by contexts that contains color.
pub trait RelativeColor<'a> {
    /// Multiplies with red, green, blue and alpha values.
    fn mul_rgba(&'a self, r: f32, g: f32, b: f32, a: f32) -> Self;
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

