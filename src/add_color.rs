
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
    fn color(&'a self, color: [f32, ..4]) -> T {
        self.rgba(color[0], color[1], color[2], color[3])
    }
}

