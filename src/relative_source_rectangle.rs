use vecmath::{
    margin_source_rectangle,
    relative_source_rectangle,
};
use internal::{
    CanSourceRectangle,
    HasSourceRectangle,
    PixelRectangle,
};

/// Should be implemented by contexts that have source rectangle information.
pub trait RelativeSourceRectangle<'a, T> {
    /// Shrinks the current source rectangle equally by all sides.
    fn src_margin(&'a self, m: i64) -> T;

    /// Expands the current source rectangle equally by all sides.
    #[inline(always)]
    fn src_expand(&'a self, m: i64) -> T {
        self.src_margin(-m)
    }

    /// Moves to a relative source rectangle using the current source rectangle as tile.
    fn src_rel(&'a self, x: i64, y: i64) -> T;

    /// Moves to the source rectangle above using the current source rectangle as tile.
    #[inline(always)]
    fn src_up(&'a self) -> T {
        self.src_rel(0, 1)
    }

    /// Moves to the left source rectangle using the current source rectangle as tile.
    #[inline(always)]
    fn src_left(&'a self) -> T {
        self.src_rel(-1, 0)
    }

    /// Moves to the right source rectangle using the current source rectangle as tile.
    #[inline(always)]
    fn src_right(&'a self) -> T {
        self.src_rel(1, 0)
    }

    /// Moves to the rectangle below using the current source rectangle as tile.
    #[inline(always)]
    fn src_down(&'a self) -> T {
        self.src_rel(0, -1)
    }
}

impl<
    'a,
    T: HasSourceRectangle<'a, PixelRectangle> + CanSourceRectangle<'a, U, PixelRectangle>,
    U
> RelativeSourceRectangle<'a, U> for T {
    #[inline(always)]
    fn src_margin(&'a self, m: i64) -> U {
        self.source_rectangle(margin_source_rectangle(*self.get_source_rectangle(), m))
    }

    #[inline(always)]
    fn src_rel(&'a self, x: i64, y: i64) -> U {
        self.source_rectangle(relative_source_rectangle(*self.get_source_rectangle(), x, y))
    }
}

