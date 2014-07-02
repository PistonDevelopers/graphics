
use vecmath::{
    margin_rectangle,
    relative_rectangle,
};
use internal::{
    CanRectangle,
    HasRectangle,
    Rectangle,
};

/// Should be implemented by contexts that have rectangle information.
pub trait RelativeRectangle<T> {
    /// Shrinks the current rectangle equally by all sides.
    fn margin(&self, m: f64) -> T;

    /// Expands the current rectangle equally by all sides.
    #[inline(always)]
    fn expand(&self, m: f64) -> T {
        self.margin(-m)
    }

    /// Moves to a relative rectangle using the current rectangle as tile.
    fn rel(&self, x: f64, y: f64) -> T;
}

impl<
    T: HasRectangle<Rectangle> + CanRectangle<U, Rectangle>,
    U
> RelativeRectangle<U> for T {
    #[inline(always)]
    fn margin(&self, m: f64) -> U {
        self.rectangle(margin_rectangle(self.get_rectangle(), m))
    }

    #[inline(always)]
    fn rel(&self, x: f64, y: f64) -> U {
        self.rectangle(relative_rectangle(self.get_rectangle(), x, y))
    }
}

