
/// Implemented by contexts that contains color.
pub trait RelativeColor<'a> {
    /// Multiplies with red, green, blue and alpha values.
    fn mul_rgba(&'a self, r: f32, g: f32, b: f32, a: f32) -> Self;
}

