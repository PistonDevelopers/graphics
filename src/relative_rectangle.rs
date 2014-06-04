
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
pub trait RelativeRectangle<'a, T> {
    /// Shrinks the current rectangle equally by all sides.
    fn margin(&'a self, m: f64) -> T;

    /// Expands the current rectangle equally by all sides.
    #[inline(always)]
    fn expand(&'a self, m: f64) -> T {
        self.margin(-m)
    }

    /// Moves to a relative rectangle using the current rectangle as tile.
    fn rel(&'a self, x: f64, y: f64) -> T;
}

impl<
    'a,
    T: HasRectangle<'a, Rectangle> + CanRectangle<'a, U, Rectangle>,
    U
> RelativeRectangle<'a, U> for T {
    #[inline(always)]
    fn margin(&'a self, m: f64) -> U {
        self.rectangle(margin_rectangle(*self.get_rectangle(), m))
    }

    #[inline(always)]
    fn rel(&'a self, x: f64, y: f64) -> U {
        self.rectangle(relative_rectangle(*self.get_rectangle(), x, y))
    }
}

