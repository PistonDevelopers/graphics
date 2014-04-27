
/// Implemented by contexts who can give an animated inbetweening context.
pub trait AddTween<'a, T> {
    /// Do linear interpolation.
    fn lerp(&'a self, tween_factor: f64) -> T;
}

