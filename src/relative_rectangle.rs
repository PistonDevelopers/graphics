use {
    CanRectangle,
    HasRectangle,
    Rectangle,
};
use vecmath::{
    margin_rectangle,
    relative_rectangle,
};

/// Should be implemented by contexts that have rectangle information.
pub trait RelativeRectangle<'a> {
    /// Shrinks the current rectangle equally by all sides.
    fn margin(&'a self, m: f64) -> Self;

    /// Expands the current rectangle equally by all sides.
    #[inline(always)]
    fn expand(&'a self, m: f64) -> Self {
        self.margin(-m)
    }

    /// Moves to a relative rectangle using the current rectangle as tile.
    fn rel(&'a self, x: f64, y: f64) -> Self;

    /// Moves to the rectangle above using the current rectangle as tile.
    #[inline(always)]
    fn up(&'a self) -> Self {
        self.rel(0.0, 1.0)
    }

    /// Moves to the left rectangle using the current rectangle as tile.
    #[inline(always)]
    fn left(&'a self) -> Self {
        self.rel(-1.0, 0.0)
    }

    /// Moves to the right rectangle using the current rectangle as tile.
    #[inline(always)]
    fn right(&'a self) -> Self {
        self.rel(1.0, 0.0)
    }

    /// Moves to the rectangle below using the current rectangle as tile.
    #[inline(always)]
    fn down(&'a self) -> Self {
        self.rel(0.0, -1.0)
    }
}

impl<
    'a,
    T: HasRectangle<'a, Rectangle> + CanRectangle<'a, T, Rectangle>
> RelativeRectangle<'a> for T {
    #[inline(always)]
    fn margin(&'a self, m: f64) -> T {
        self.rectangle(margin_rectangle(self.get_rectangle(), m))
    }

    #[inline(always)]
    fn rel(&'a self, x: f64, y: f64) -> T {
        self.rectangle(relative_rectangle(self.get_rectangle(), x, y))
    }
}

