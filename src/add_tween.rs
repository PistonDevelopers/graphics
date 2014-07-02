
/// Implemented by contexts who can give an animated inbetweening context.
pub trait AddTween<T> {
    /// Do linear interpolation.
    fn lerp(&self, tween_factor: f64) -> T;
}

